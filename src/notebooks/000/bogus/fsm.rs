use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 1. Define your FSM States declaratively
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FsmState {
    Vacant,        // Empty vacuum state
    Crystallizing, // Computing processing frames
    Terminal,      // Handoff to external EXE / cleared
}

// 2. Map your YAML structure directly to strict, typed memory layout
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub workflow_id: String,
    pub fsm_initial: FsmState,
    pub dag_edges: Vec<(String, String)>, // (Parent node, Child node)
    pub frame_projections: HashMap<String, String>,
}

fn main() {
    // Simulated input from your distributed notebook network
    let yaml_state = "
workflow_id: 'tai_yin_wuji_01'
fsm_initial: 'Vacant'
dag_edges:
  - ['node_alpha', 'node_beta']
  - ['node_beta', 'node_gamma']
frame_projections:
  node_alpha: 'frame_001'
";

    // Rust parses and validates the state topology instantly.
    // If the YAML structure violates the types, it throws a compile-safe error immediately.
    let config: PipelineConfig = serde_yaml::from_str(yaml_state).unwrap();
    
    println!("Loaded State Machine tracking for: {}", config.workflow_id);
    println!("Initial FSM Phase: {:?}", config.fsm_initial);
}
