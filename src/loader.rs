use crate::model::{NotebookModel, EnlightenId, EnlightenGov, EnlightenReg, EnlightenMetrics};
use std::fs;
use std::path::Path;

pub struct NotebookLoader;

impl NotebookLoader {
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<NotebookModel, Box<dyn std::error::Error>> {
        let dir = dir.as_ref();
        let mut id_content = None;
        let mut gov_content = None;
        let mut reg_content = None;
        let mut metrics_content = None;
        let mut dialogue_source = String::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with("_id.toml") {
                    id_content = Some(fs::read_to_string(&path)?);
                } else if name.ends_with("_gov.toml") {
                    gov_content = Some(fs::read_to_string(&path)?);
                } else if name.ends_with("_reg.toml") {
                    reg_content = Some(fs::read_to_string(&path)?);
                } else if name.ends_with("_metrics.toml") {
                    metrics_content = Some(fs::read_to_string(&path)?);
                } else if (name.ends_with("_dialogue.md") || name.ends_with("_notes.md") || name.ends_with("_atlas.md")) && dialogue_source.is_empty() {
                    dialogue_source = fs::read_to_string(&path).unwrap_or_default();
                }
            }
        }

        let id: EnlightenId = if let Some(content) = id_content {
            toml::from_str(&content)?
        } else {
            return Err("Missing required ID TOML file".into());
        };

        let gov: EnlightenGov = if let Some(content) = gov_content {
            toml::from_str(&content).unwrap_or_default()
        } else {
            EnlightenGov::default()
        };

        let reg: EnlightenReg = if let Some(content) = reg_content {
            toml::from_str(&content)?
        } else {
            return Err("Missing required Registry TOML file".into());
        };

        let metrics: EnlightenMetrics = if let Some(content) = metrics_content {
            toml::from_str(&content).unwrap_or_default()
        } else {
            EnlightenMetrics {
                relevance_density: 0.5,
                tension: 0.1,
                novelty: 0.2,
                diversity: 0.3,
            }
        };

        Ok(NotebookModel {
            id,
            gov,
            reg,
            metrics,
            dialogue_source,
        })
    }
}