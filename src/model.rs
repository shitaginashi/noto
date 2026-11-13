use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotebookIdentity {
    pub notebook_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoreHex {
    #[serde(default)]
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Provenance {
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub schema_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Polarity {
    #[serde(default)]
    pub governance: String,
    #[serde(default)]
    pub participation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Classification {
    #[serde(rename = "type", alias = "artifact_type", default)]
    pub notebook_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnlightenId {
    #[serde(default)]
    pub identity: NotebookIdentity,
    #[serde(default)]
    pub classification: Option<Classification>,
    #[serde(default)]
    pub core_hex: CoreHex,
    #[serde(default)]
    pub polarity: Polarity,
    #[serde(default)]
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UtilityMeta {
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContributionMeta {
    #[serde(default)]
    pub inbound: i32,
    #[serde(default)]
    pub outbound: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Subscriptions {
    #[serde(default)]
    pub enabled: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Exports {
    #[serde(default)]
    pub enabled: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnlightenGov {
    #[serde(default)]
    pub weights: HashMap<String, f64>,
    #[serde(default)]
    pub subscriptions: Subscriptions,
    #[serde(default)]
    pub exports: Exports,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JurisdictionTable {
    #[serde(default)]
    pub owns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatternsTable {
    #[serde(default)]
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConceptsTable {
    #[serde(default)]
    pub concepts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DurableTable {
    #[serde(default)]
    pub structures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodesTable {
    #[serde(default)]
    pub local: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandidatesTable {
    #[serde(default)]
    pub durable: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentsTable {
    #[serde(default)]
    pub candidates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegIdentityBlock {
    #[serde(default)]
    pub notebook_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub artifact_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegInput {
    #[serde(default)]
    pub schema_version: Option<String>,
    #[serde(default)]
    pub registry_version: Option<String>,
    #[serde(default)]
    pub identity: Option<RegIdentityBlock>,
    #[serde(default)]
    pub jurisdiction: Option<JurisdictionTable>,
    #[serde(default)]
    pub key_patterns: Option<PatternsTable>,
    #[serde(default)]
    pub key_concepts: Option<ConceptsTable>,
    #[serde(default)]
    pub durable_candidates: Option<DurableTable>,
    #[serde(default)]
    pub nodes: Option<NodesTable>,
    #[serde(default)]
    pub candidates: Option<CandidatesTable>,
    #[serde(default)]
    pub attachments: Option<AttachmentsTable>,
    #[serde(default)]
    pub discoverability: Option<HashMap<String, Vec<String>>>,
    #[serde(default)]
    pub registry: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub raw_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnlightenReg {
    pub nodes: Vec<String>,
    pub candidates: HashMap<String, Vec<String>>,
}

impl<'de> Deserialize<'de> for EnlightenReg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = RegInput::deserialize(deserializer)?;
        
        let mut nodes = input.raw_nodes;
        if let Some(n) = input.nodes {
            nodes.extend(n.local);
        }
        if let Some(j) = input.jurisdiction {
            nodes.extend(j.owns);
        }
        if let Some(p) = input.key_patterns {
            nodes.extend(p.patterns);
        }
        if let Some(c) = input.key_concepts {
            nodes.extend(c.concepts);
        }

        let mut candidates = HashMap::new();
        if let Some(c) = input.candidates {
            candidates.insert("durable".to_string(), c.durable);
        }
        if let Some(a) = input.attachments {
            candidates.insert("attachments".to_string(), a.candidates);
        }
        if let Some(d) = input.durable_candidates {
            candidates.entry("durable".to_string()).or_default().extend(d.structures);
        }

        Ok(EnlightenReg {
            nodes,
            candidates,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsInput {
    #[serde(default)]
    pub metrics: Option<EnlightenMetricsData>,
    #[serde(flatten)]
    pub flat: Option<EnlightenMetricsData>,
    #[serde(default)]
    pub utility: Option<UtilityMeta>,
    #[serde(default)]
    pub contribution: Option<ContributionMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnlightenMetricsData {
    #[serde(default)]
    pub relevance_density: f64,
    #[serde(default)]
    pub tension: f64,
    #[serde(default)]
    pub novelty: f64,
    #[serde(default)]
    pub diversity: f64,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct EnlightenMetrics {
    pub relevance_density: f64,
    pub tension: f64,
    pub novelty: f64,
    pub diversity: f64,
}

impl<'de> Deserialize<'de> for EnlightenMetrics {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let input = MetricsInput::deserialize(deserializer)?;
        let data = input.metrics.or(input.flat).unwrap_or_default();
        Ok(EnlightenMetrics {
            relevance_density: data.relevance_density,
            tension: data.tension,
            novelty: data.novelty,
            diversity: data.diversity,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookModel {
    pub id: EnlightenId,
    pub gov: EnlightenGov,
    pub reg: EnlightenReg,
    pub metrics: EnlightenMetrics,
    pub dialogue_source: String,
}
