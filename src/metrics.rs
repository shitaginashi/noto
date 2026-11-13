use crate::correspondence::DeltaCandidate;
use crate::hydrator::Frame;
use crate::model::NotebookModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationReport {
    pub relevance_density: f64,
    pub tension: f64,
    pub novel_correspondence: f64,
    pub diversity: f64,
}

pub struct MetricsObserver;

impl MetricsObserver {
    /// Calculate observation metrics strictly from model, frame, and discovered deltas (RR-6).
    pub fn observe(model: &NotebookModel, frame: &Frame, deltas: &[DeltaCandidate]) -> ObservationReport {
        let base_rho = model.metrics.relevance_density;
        let node_count = frame.relevant_nodes.len() as f64;
        
        // Calculate relevance density based on frame node ratio and base density
        let relevance_density = if node_count > 0.0 {
            (base_rho + (node_count * 0.1)).min(1.0)
        } else {
            base_rho
        };

        // Tension derived from candidate overlap / dissonance
        let tension = model.metrics.tension + (deltas.len() as f64 * 0.05);

        // Novel correspondence (ΔE) derived from yielded delta relevance scores
        let novel_correspondence: f64 = deltas.iter().map(|d| d.relevance_score).sum();

        // Diversity from total unique candidates vs nodes
        let diversity = model.metrics.diversity + (frame.relevant_candidates.len() as f64 * 0.05);

        ObservationReport {
            relevance_density,
            tension,
            novel_correspondence,
            diversity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::correspondence::DeltaCandidate;
    use crate::hydrator::Frame;
    use crate::model::{EnlightenId, EnlightenGov, EnlightenReg, EnlightenMetrics, NotebookIdentity, CoreHex, Polarity, Provenance, Subscriptions, Exports};
    use std::collections::HashMap;

    #[test]
    fn test_metrics_observation() {
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
                nodes: vec!["pattern".into()],
                candidates: HashMap::new(),
            },
            metrics: EnlightenMetrics { relevance_density: 0.4, tension: 0.1, novelty: 0.0, diversity: 0.2 },
            dialogue_source: "".into(),
        };

        let frame = Frame {
            question: "pattern".into(),
            relevant_nodes: vec!["pattern".into()],
            relevant_candidates: vec!["cand_1".into()],
            context_snippet: "".into(),
        };

        let deltas = vec![DeltaCandidate {
            id: "d1".into(),
            source_node: "pattern".into(),
            target_candidate: "cand_1".into(),
            relevance_score: 0.9,
        }];

        let report = MetricsObserver::observe(&model, &frame, &deltas);
        assert!(report.relevance_density > 0.4);
        assert_eq!(report.novel_correspondence, 0.9);
    }
}