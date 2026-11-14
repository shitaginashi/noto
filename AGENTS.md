# AGENTS.md

This repository is a Rust-based research prototype for a local, question-driven notebook runtime called `noto`.

## Purpose

`noto` is designed to help a model or agent work with notebook-like knowledge objects by:

- loading notebook metadata from a directory,
- selecting relevant nodes based on a question,
- hydrating a temporary "frame" of context,
- discovering candidate correspondences between local nodes and durable structures,
- reporting exploratory metrics,
- persisting new candidate deltas as YAML.

The goal is not production-grade notebook software or a general-purpose AI platform. It is an experimental runtime for exploring how local knowledge can be surfaced and related around a question.

## Repository shape

- `README.md` — user-facing overview and runtime notes.
- `Cargo.toml` — Rust crate definition and dependencies.
- `LICENSE` — MIT license.
- `src/` — Rust source files.
- `src/notebooks/` — template and sample notebook artifacts.
- `rc` — repository diff artifact; not part of the runtime.

## Key runtime modules

- `src/main.rs` — CLI entry point.
- `src/lib.rs` — exported modules and notebook document model.
- `src/loader.rs` — reads notebook metadata files from a directory.
- `src/hydrator.rs` — builds a relevant frame from the question and notebook contents.
- `src/correspondence.rs` — discovers delta candidates.
- `src/delta.rs` — persists deltas to `enlighten_delta.yml`.
- `src/metrics.rs` — computes exploratory observation metrics.
- `src/model.rs` — TOML schema structs for notebook identity, registry, metrics, and governance.

## Runtime contract

The CLI accepts:

```sh
cargo run -- <notebook-directory> "<question>"
```

Behavior:

1. Load notebook files directly from the target directory.
2. Require the directory to contain flat files such as:
   - `_id.toml`
   - `_reg.toml`
   - optional `_gov.toml`
   - optional `_metrics.toml`
   - optional `_dialogue.md`, `_notes.md`, or `_atlas.md`
3. Build a question-driven frame from matching node names and dialogue snippets.
4. Discover candidate correspondences between relevant nodes and durable candidates.
5. Report metrics: relevance density, tension, novel correspondence, diversity.
6. Persist delta candidates to `enlighten_delta.yml` in the same notebook directory.

Important: the loader does not recurse into subdirectories. The target notebook directory must contain the metadata files directly at its root.

## Notebook conventions

Notebook files use suffix conventions:

- `_id.toml`
- `_reg.toml`
- `_gov.toml`
- `_metrics.toml`
- `_dialogue.md`
- `_notes.md`
- `_atlas.md`

The project also uses templates under `src/notebooks/` such as:

- `template_id.toml`
- `template_reg.toml`
- `template_gov.toml`
- `template_metrics.toml`
- `template_notes.md`
- `template_atlas.md`
- `template_lineage.yml`
- `template_delta.yml`

The sample notebook currently used by the repo is:

```sh
src/notebooks/000/teaser
```

## Model intentions and design vocabulary

This codebase uses a deliberately research-heavy vocabulary. Concepts include:

- `nodes`
- `candidates`
- `durable_candidates`
- `delta`
- `frame`
- `relevance_density`
- `tension`
- `novel_correspondence`
- `diversity`
- `lineage`
- `governance`

These are not fixed domain standards; they are part of the prototype’s experimental ontology. When editing or extending the project, preserve the language where possible unless the change explicitly refactors the underlying domain model.

## Agent guidance

Agents working in this repo should:

- prefer editing the code and templates that embody the runtime model rather than changing README-only assumptions,
- validate runtime assumptions against actual files in `src/notebooks`,
- keep output deterministic and explain why some files are optional,
- avoid silently changing notebook conventions or schema names unless the schema is intentionally being versioned,
- prefer preserving the existing “research prototype” shape over introducing production assumptions.

## Operational safety rules for agents

- Do not add production-grade abstractions without a clear reason.
- Do not silently make the loader recursive unless the existing contract and templates are updated together.
- Do not change schema field names casually; these are part of the notebook data model.
- Do not treat generated metrics or deltas as authoritative facts.
- Preserve the distinction between template artifacts, sample notebooks, and concrete runtime outputs.

## Helpful paths for agents

- `src/notebooks/template_reg.toml`
- `src/notebooks/template_id.toml`
- `src/notebooks/template_metrics.toml`
- `src/notebooks/000/teaser/teaser_id.toml`
- `src/notebooks/000/teaser/teaser_reg.toml`
- `src/notebooks/000/teaser/teaser_notes.md`
- `src/main.rs`
- `src/loader.rs`
- `src/hydrator.rs`
- `src/correspondence.rs`
- `src/metrics.rs`

## Suggested agent challenge

A frontier agent should be able to read this repo and answer questions such as:

- what files are required to load a notebook directory?
- what does the sample notebook contain?
- how does a question select relevant nodes?
- where are delta candidates written and why?
- what is the meaning of the experimental metrics?

These questions should be answerable without reading the entire history of the project; the repo’s current structure and file naming are intended to make that possible.
