pub mod model;
pub mod loader;
pub mod hydrator;
pub mod correspondence;
pub mod delta;
pub mod metrics;

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CellStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookCell {
    pub id: String,
    pub language: String,
    pub source: String,
    pub status: CellStatus,
    pub output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookDocument {
    pub version: String,
    pub cells: Vec<NotebookCell>,
}

impl NotebookDocument {
    pub fn new() -> Self {
        Self {
            version: "0.1.0".to_string(),
            cells: Vec::new(),
        }
    }

    /// Save the notebook document to a TOML file at the specified path.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }

    /// Load a notebook document from a TOML file at the specified path.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let document: NotebookDocument = toml::from_str(&contents)?;
        Ok(document)
    }
}

impl Default for NotebookDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_notebook_save_and_load() {
        let temp_file = "test_notebook.toml";
        
        let mut doc = NotebookDocument::new();
        doc.cells.push(NotebookCell {
            id: "cell_1".to_string(),
            language: "rust".to_string(),
            source: "println!(\"Hello, noto!\");".to_string(),
            status: CellStatus::Completed,
            output: Some("Hello, noto!\n".to_string()),
        });

        let save_result = doc.save_to_file(temp_file);
        assert!(save_result.is_ok(), "Failed to save notebook to file");

        let loaded_doc = NotebookDocument::load_from_file(temp_file);
        assert!(loaded_doc.is_ok(), "Failed to load notebook from file");

        let loaded_doc = loaded_doc.unwrap();
        assert_eq!(loaded_doc.version, "0.1.0");
        assert_eq!(loaded_doc.cells.len(), 1);
        assert_eq!(loaded_doc.cells[0].id, "cell_1");
        assert_eq!(loaded_doc.cells[0].source, "println!(\"Hello, noto!\");");
        assert_eq!(loaded_doc.cells[0].status, CellStatus::Completed);

        let _ = fs::remove_file(temp_file);
    }
}