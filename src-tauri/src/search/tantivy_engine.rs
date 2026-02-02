use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, Value as _, STORED, TEXT};
use tantivy::{doc, Index, IndexReader, IndexWriter, Searcher, TantivyDocument};

use crate::core::document::Document;
use crate::core::errors::{AxiomError, Result};

pub struct TantivyEngine {
    index: Index,
    reader: IndexReader,
    writer: IndexWriter,
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

        let index = Index::create_in_dir(path, schema.clone())
            .map_err(|e| AxiomError::Search(e.to_string()))?;

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

        let query = query_parser
            .parse_query(query)
            .map_err(|e| AxiomError::Search(e.to_string()))?;

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .map_err(|e| AxiomError::Search(e.to_string()))?;

        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher
                .doc(doc_address)
                .map_err(|e| AxiomError::Search(e.to_string()))?;

            // Get document ID from the retrieved doc
            if let Some(id_value) = retrieved_doc.get_first(self.fields.id) {
                let id_str: &str = id_value.as_str().unwrap_or("");
                if !id_str.is_empty() {
                    results.push(SearchResult {
                        doc_id: id_str.to_string(),
                        score: _score,
                    });
                }
            }
        }

        Ok(results)
    }
}

pub struct SearchResult {
    pub doc_id: String,
    pub score: f32,
}
