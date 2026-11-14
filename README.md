# noto ノート (nōto)

**noto** is a research proof of concept for a local, question-driven notebook runtime written in Rust. It explores how a question can select relevant material from a notebook, create a temporary frame, compare it with candidate structures, and report exploratory metrics about relevance, novelty, and diversity.

This is experimental software—not a production notebook application, database, or general-purpose AI system.

## Research idea

The project explores a simple premise: knowledge may emerge not just by accumulating notes, but by finding useful relationships between them.

In the prototype, a question is used to hydrate a frame from notebook-local registry entries and text. The runtime compares relevant nodes with candidate structures, reports exploratory metrics, and appends discovered candidates to `enlighten_delta.yml`.

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
cargo run -- src/notebooks/000/teaser "What is wisdom?"
```

If no directory is supplied, the CLI defaults to `notebooks`. If no question is supplied, it defaults to `What is wisdom?`.

The loader requires the notebook files to live directly inside the supplied directory; it does not recurse into subdirectories. In this repository, the sample notebook workspace is `src/notebooks/000/teaser`, which is a valid single-notebook directory because it contains the metadata files directly under that folder.

Example:

```sh
cargo run -- src/notebooks/000/teaser "What is wisdom?"
```

If you want to use the built-in default path instead, create a local `notebooks/` directory with the same flat layout, then run:

```sh
mkdir -p notebooks
# copy or symlink the notebook metadata files into notebooks/
cargo run -- notebooks "What is wisdom?"
```

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
