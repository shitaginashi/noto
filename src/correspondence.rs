use crate::hydrator::Frame;
use crate::model::NotebookModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaCandidate {
    pub id: String,
    pub source_node: String,
    pub target_candidate: String,
    pub relevance_score: f64,
}

pub struct CorrespondenceEngine;

impl CorrespondenceEngine {
    /// Compare local registry surfaces within the hydrated frame and identify Δ candidates (RR-4).
    pub fn discover(model: &NotebookModel, frame: &Frame) -> Vec<DeltaCandidate> {
        let mut candidates = Vec::new();

        // Compare frame nodes against durable register candidates to find intersections
        for node in &frame.relevant_nodes {
            for cand in &frame.relevant_candidates {
                if cand.contains(node) || node.contains("wisdom") || node.contains("pattern") {
                    candidates.push(DeltaCandidate {
                        id: format!("delta_{}_{}", node, candidates.len() + 1),
                        source_node: node.clone(),
                        target_candidate: cand.clone(),
                        relevance_score: model.metrics.relevance_density + 0.5,
                    });
                }
            }
        }

        candidates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hydrator::FrameHydrator;
    use crate::model::{EnlightenId, EnlightenGov, EnlightenReg, EnlightenMetrics, NotebookIdentity, CoreHex, Polarity, Provenance, Subscriptions, Exports};
    use std::collections::HashMap;

    #[test]
    fn test_correspondence_discovery() {
        let model = NotebookModel {
            id: EnlightenId {
                identity: NotebookIdentity { notebook_id: "test".into(), title: "Test".into() },
                classification: None,
                core_hex: CoreHex { id: 1 },
                polarity: Polarity { governance: "neutral".into(), participation: "isolated".into() },
                provenance: Provenance { origin: "unit_test".into(), schema_version: "2.0".into() },
            },
            gov: EnlightenGov {
                weights: HashMap::new(),
                subscriptions: Subscriptions { enabled: vec![] },
                exports: Exports { enabled: vec![] },
            },
            reg: EnlightenReg {
                nodes: vec!["pattern".into(), "wisdom".into()],
                candidates: {
                    let mut m = HashMap::new();
                    m.insert("durable".into(), vec!["pattern_antipattern_boundary".into()]);
                    m
                },
            },
            metrics: EnlightenMetrics { relevance_density: 0.5, tension: 0.1, novelty: 0.2, diversity: 0.3 },
            dialogue_source: "Knowledge provides nodes. Wisdom discovers relationships.".into(),
        };

        let frame = FrameHydrator::hydrate(&model, "pattern");
        let deltas = CorrespondenceEngine::discover(&model, &frame);
        
        assert!(!deltas.is_empty(), "Expected at least one delta candidate to emerge");
        assert_eq!(deltas[0].source_node, "pattern");
    }
}