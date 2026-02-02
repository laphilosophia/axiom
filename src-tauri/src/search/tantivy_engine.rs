use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, Value as _, STORED, TEXT};
use tantivy::snippet::SnippetGenerator;
use tantivy::{doc, Index, IndexReader, IndexWriter, Searcher, TantivyDocument};

use crate::core::document::Document;
use crate::core::errors::{AxiomError, Result};

pub struct TantivyEngine {
    index: Index,
    reader: IndexReader,
    writer: IndexWriter,
    #[allow(dead_code)]
    schema: Schema,
    fields: TantivyFields,
}

struct TantivyFields {
    id: Field,
    title: Field,
    content: Field,
    status: Field,
    tags: Field,
}

impl TantivyEngine {
    pub fn new(path: &Path) -> Result<Self> {
        std::fs::create_dir_all(path)?;

        let mut schema_builder = Schema::builder();
        let id = schema_builder.add_text_field("id", TEXT | STORED);
        let title = schema_builder.add_text_field("title", TEXT | STORED);
        let content = schema_builder.add_text_field("content", TEXT);
        let status = schema_builder.add_text_field("status", TEXT | STORED);
        let tags = schema_builder.add_text_field("tags", TEXT);
        let schema = schema_builder.build();

        // Check if index already exists by looking for meta.json
        let meta_path = path.join("meta.json");
        let index = if meta_path.exists() {
            // Open existing index
            Index::open_in_dir(path)
                .map_err(|e| AxiomError::Search(format!("Failed to open index: {}", e)))?
        } else {
            // Create new index
            Index::create_in_dir(path, schema.clone())
                .map_err(|e| AxiomError::Search(format!("Failed to create index: {}", e)))?
        };

        let reader = index
            .reader_builder()
            .try_into()
            .map_err(|e| AxiomError::Search(e.to_string()))?;

        let writer = index
            .writer(50_000_000)
            .map_err(|e| AxiomError::Search(e.to_string()))?;

        let fields = TantivyFields {
            id,
            title,
            content,
            status,
            tags,
        };

        Ok(Self {
            index,
            reader,
            writer,
            schema,
            fields,
        })
    }

    pub fn add_document(&mut self, doc: &Document) -> Result<()> {
        let tantivy_doc = doc!(
            self.fields.id => doc.id.clone(),
            self.fields.title => doc.title.clone(),
            self.fields.content => doc.content.clone(),
            self.fields.status => doc.status.to_string(),
            self.fields.tags => doc.tags.join(" "),
        );

        self.writer
            .add_document(tantivy_doc)
            .map_err(|e| AxiomError::Search(e.to_string()))?;

        Ok(())
    }

    pub fn update_document(&mut self, doc: &Document) -> Result<()> {
        // Delete old document
        let term = tantivy::Term::from_field_text(self.fields.id, &doc.id);
        self.writer.delete_term(term);

        // Add updated document
        self.add_document(doc)?;

        Ok(())
    }

    pub fn delete_document(&mut self, doc_id: &str) -> Result<()> {
        let term = tantivy::Term::from_field_text(self.fields.id, doc_id);
        self.writer.delete_term(term);
        Ok(())
    }

    pub fn commit(&mut self) -> Result<()> {
        self.writer
            .commit()
            .map_err(|e| AxiomError::Search(e.to_string()))?;
        // Reload the reader to pick up new commits
        self.reader
            .reload()
            .map_err(|e| AxiomError::Search(e.to_string()))?;
        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher: Searcher = self.reader.searcher();

        let query_parser = QueryParser::for_index(
            &self.index,
            vec![self.fields.title, self.fields.content, self.fields.tags],
        );

        let parsed_query = query_parser
            .parse_query(query)
            .map_err(|e| AxiomError::Search(e.to_string()))?;

        let top_docs = searcher
            .search(&parsed_query, &TopDocs::with_limit(limit))
            .map_err(|e| AxiomError::Search(e.to_string()))?;

        // Setup snippet generator for highlighting
        let snippet_generator: SnippetGenerator =
            SnippetGenerator::create(&searcher, &parsed_query, self.fields.content)
                .map_err(|e: tantivy::TantivyError| AxiomError::Search(e.to_string()))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| AxiomError::Search(e.to_string()))?;

            // Get document ID
            let id = retrieved_doc
                .get_first(self.fields.id)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if id.is_empty() {
                continue;
            }

            // Get title
            let title = retrieved_doc
                .get_first(self.fields.title)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Get status
            let status = retrieved_doc
                .get_first(self.fields.status)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Generate snippet with highlighting
            let snippet = snippet_generator.snippet_from_doc(&retrieved_doc);
            let snippet_html = snippet.to_html();

            results.push(SearchResult {
                doc_id: id,
                title,
                status,
                snippet: if snippet_html.is_empty() {
                    None
                } else {
                    Some(snippet_html)
                },
                score,
            });
        }

        Ok(results)
    }

    /// Search with status filter
    pub fn search_with_status(
        &self,
        query: &str,
        status_filter: Option<&str>,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let full_query = match status_filter {
            Some(status) => format!("({}) AND status:{}", query, status),
            None => query.to_string(),
        };
        self.search(&full_query, limit)
    }

    /// Search with tag filter
    pub fn search_with_tag(
        &self,
        query: &str,
        tag: Option<&str>,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let full_query = match tag {
            Some(t) => format!("({}) AND tags:{}", query, t),
            None => query.to_string(),
        };
        self.search(&full_query, limit)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub doc_id: String,
    pub title: String,
    pub status: String,
    pub snippet: Option<String>,
    pub score: f32,
}
