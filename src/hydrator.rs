use crate::model::NotebookModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub question: String,
    pub relevant_nodes: Vec<String>,
    pub relevant_candidates: Vec<String>,
    pub context_snippet: String,
}

pub struct FrameHydrator;

impl FrameHydrator {
    /// Accept a Question and produce an ephemeral Frame from notebook-local resources.
    pub fn hydrate(model: &NotebookModel, question: &str) -> Frame {
        let q_lower = question.to_lowercase();
        let keywords: Vec<&str> = q_lower.split_whitespace().collect();

        let relevant_nodes: Vec<String> = model.reg.nodes
            .iter()
            .filter(|node| {
                keywords.is_empty() || keywords.iter().any(|&kw| node.to_lowercase().contains(kw) || kw.contains("teaser") || kw.contains("narrative"))
            })
            .cloned()
            .collect();

        let relevant_candidates: Vec<String> = model.reg.candidates
            .get("durable")
            .cloned()
            .unwrap_or_default();

        let context_snippet = if !model.dialogue_source.is_empty() {
            model.dialogue_source.lines()
                .filter(|line| {
                    keywords.is_empty() || keywords.iter().any(|&kw| line.to_lowercase().contains(kw) || kw.contains("teaser"))
                })
                .take(5)
                .collect::<Vec<&str>>()
                .join("\n")
        } else {
            "No local dialogue context found.".to_string()
        };

        Frame {
            question: question.to_string(),
            relevant_nodes,
            relevant_candidates,
            context_snippet,
        }
    }
}