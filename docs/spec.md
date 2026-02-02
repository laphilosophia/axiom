# Document Orchestrator

> This document describes a **simple, practical document orchestrator**.
> It does **not** define a cognitive system, epistemic theory, or memory model.
> The goal is clarity, not ambition.

---

## 0. Scope (Hard Boundaries)

This system **is**:

* a local document organization tool
* a document lifecycle orchestrator
* a lightweight web UI over filesystem data

This system **is not**:

* a cognitive runtime
* a knowledge graph
* a learning system
* a memory model
* an enterprise engine

Any concept outside basic document management is out of scope.

---

## 1. Problem Statement (Practical)

The problem is operational:

* Many documents exist
* Some are outdated
* Some are related
* Some are forgotten and rewritten
* Plain search is insufficient to prevent duplication

This is a **document hygiene and organization problem**.

---

## 2. Core Objective

Provide a tool that:

* stores documents on the filesystem
* tracks document status (active, outdated, archived)
* surfaces potentially related existing documents
* helps the user avoid rewriting existing material

Nothing more.

---

## 3. Document Model

A document consists of:

### 3.1 Content

* Arbitrary format (markdown, text, etc.)
* Treated as opaque by the system

### 3.2 Metadata

Minimal required metadata:

* id (stable, filesystem‑independent)
* title
* status: draft | active | superseded | archived
* created_at
* updated_at
* tags (optional)

Metadata exists solely to support organization and filtering.

---

## 4. Document Lifecycle

Documents move through simple states:

* draft → active
* active → superseded
* active → archived

Rules:

* superseded documents are read‑only
* deletion is discouraged but allowed

Lifecycle exists to answer: *"Is this document still relevant?"*

---

## 5. Relationships (Simple and Explicit)

Relationships exist to avoid duplication, not to model knowledge.

### 5.1 Explicit Relationships (Stored in Metadata)

Supported types:

* supersedes
* references

Properties:

* user‑defined
* explicit
* stable

Example:

* Document A supersedes Document B

---

### 5.2 Suggested Relationships (Computed, Optional)

The system **may suggest** related documents based on:

* title similarity
* tag overlap
* recent edit proximity
* optional LLM similarity

These suggestions:

* are not stored unless accepted
* do not decay
* have no weights
* are recomputed on demand

They exist only to assist the user.

---

## 6. Search

Search provides:

* full‑text search
* filtering by status
* filtering by tags

Search is deterministic and transparent.

---

## 7. Orchestrator

The orchestrator coordinates:

* metadata updates
* lifecycle transitions
* relationship validation
* indexing

It does **not**:

* infer truth
* rank importance
* maintain long‑term behavioral state

---

## 8. LLM Integration (Optional, Limited)

LLM usage is strictly optional.

Allowed uses:

* "Do similar documents already exist?"
* "Suggest possible related documents"

LLM output:

* is advisory
* never stored automatically
* never changes document state

The system works fully without an LLM.

---

## 9. Architecture

* filesystem = source of truth
* simple metadata store (JSON/YAML)
* local web server
* browser‑based UI

No background intelligence required.

---

## 10. Success Criteria

The system is successful if:

* the user stops rewriting documents they already have
* outdated documents are clearly marked
* related documents are easy to find

No cognitive claims are made beyond this.

---

End of document.
