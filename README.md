# noto ノート (nōto)

**noto** is a research proof of concept for a local, question-driven notebook runtime written in Rust. It explores how a question can select relevant material from a notebook, create a temporary frame, and surface candidate correspondences (“deltas”) for inspection.

This is experimental software—not a production notebook application, database, or general-purpose AI system.

## Research idea

The project explores a simple premise: knowledge may emerge not just by accumulating notes, but by finding useful relationships between them.

In the prototype, a question is used to hydrate a frame from notebook-local registry entries and text. The runtime compares relevant nodes with candidate structures, reports exploratory metrics, and can persist discovered delta candidates. It does not call an external language model.

## What the prototype does

- Loads notebook identity and registry data from TOML files.
- Optionally loads governance, metrics, and Markdown dialogue or notes.
- Selects registry nodes using basic keyword matching against a question.
- Compares selected nodes with durable candidates to produce delta candidates.
- Reports prototype metrics for relevance density, tension, novel correspondence, and diversity.
- Appends discovered candidates to `enlighten_delta.yml`.

These behaviors are intentionally small and experimental. In particular, frame selection and correspondence discovery are simple heuristics, not semantic search or validated measures of knowledge.

## Requirements

- Rust and Cargo

## Build and test

```sh
cargo build
cargo test
```

## Run

Pass the path to a directory containing a notebook’s metadata files, followed by a question:

```sh
cargo run -- <notebook-directory> "What is wisdom?"
```

If no directory is supplied, the CLI defaults to `notebooks`. If no question is supplied, it defaults to `What is wisdom?`.

The loader expects the relevant files directly inside the supplied directory (it does not recursively search subdirectories). It requires an `_id.toml` file and an `_reg.toml` file; governance and metrics files are optional.

## Notebook file conventions

The loader looks for files ending in:

| Suffix | Purpose |
| --- | --- |
| `_id.toml` | Notebook identity and provenance |
| `_reg.toml` | Registry nodes and candidates |
| `_gov.toml` | Optional governance metadata |
| `_metrics.toml` | Optional starting metrics |
| `_dialogue.md`, `_notes.md`, `_atlas.md` | Optional text context |

See [`src/notebooks`](src/notebooks) for the project’s notebook materials and templates.

## Project status

This is an evolving research/PoC. Behavior, data formats, and terminology may change. Treat generated metrics and delta candidates as exploratory outputs, not authoritative results.

## License

MIT License. See [`LICENSE`](LICENSE).
