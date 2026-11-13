use crate::correspondence::DeltaCandidate;
use std::fs;
use std::path::Path;

pub struct DeltaPersistence;

impl DeltaPersistence {
    /// Write discovered Delta candidates directly to enlighten_delta.yml.
    pub fn append<P: AsRef<Path>>(path: P, candidate: &DeltaCandidate) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        
        // Read existing content or initialize valid YAML root
        let mut existing_content = if path.exists() {
            fs::read_to_string(path)?
        } else {
            String::from("deltas:\n")
        };

        // If stale templates exist, reset the file content
        if existing_content.contains("notebook_001") || existing_content.contains("previous_position") {
            existing_content = String::from("deltas:\n");
        }

        // Avoid duplicate entry insertion
        if existing_content.contains(&candidate.id) {
            return Ok(());
        }

        // Append structured entry
        let entry = format!(
            "  - id: \"{}\"\n    source_node: \"{}\"\n    target_candidate: \"{}\"\n    relevance_score: {}\n",
            candidate.id, candidate.source_node, candidate.target_candidate, candidate.relevance_score
        );

        existing_content.push_str(&entry);
        fs::write(path, existing_content)?;
        Ok(())
    }
}