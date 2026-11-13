use std::env;
use std::path::Path;
use notebook_runtime::loader::NotebookLoader;
use notebook_runtime::hydrator::FrameHydrator;
use notebook_runtime::correspondence::CorrespondenceEngine;
use notebook_runtime::delta::DeltaPersistence;
use notebook_runtime::metrics::MetricsObserver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    let notebook_dir = if args.len() > 1 {
        &args[1]
    } else {
        "notebooks"
    };

    let question = if args.len() > 2 {
        args[2].clone()
    } else {
        "What is wisdom?".to_string()
    };

    println!("[noto] Loading notebook workspace from: {}", notebook_dir);
    let model = NotebookLoader::load_from_dir(notebook_dir)?;

    println!("[noto] Hydrating frame for question: \"{}\"", question);
    let frame = FrameHydrator::hydrate(&model, &question);
    println!("[noto] Hydrated Frame:\n{:#?}", frame);

    println!("[noto] Realizing ephemeral graph and discovering correspondence...");
    let deltas = CorrespondenceEngine::discover(&model, &frame);

    println!("[noto] Observing runtime metrics (RR-6)...");
    let report = MetricsObserver::observe(&model, &frame, &deltas);
    println!("  - Relevance Density (ρ): {}", report.relevance_density);
    println!("  - Tension (τ): {}", report.tension);
    println!("  - Novel Correspondence (ΔE): {}", report.novel_correspondence);
    println!("  - Diversity (D): {}", report.diversity);

    let delta_path = Path::new(notebook_dir).join("enlighten_delta.yml");
    if deltas.is_empty() {
        println!("[noto] No new delta candidates discovered (ΔE = 0).");
    } else {
        for candidate in &deltas {
            println!("[noto] Discovered Δ Candidate: {} -> Persisting to {:?}", candidate.id, delta_path);
            DeltaPersistence::append(&delta_path, candidate)?;
        }
        
        // Resolve and print canonical absolute path
        match std::fs::canonicalize(&delta_path) {
            Ok(canonical) => println!("[noto] Successfully persisted deltas to canonical absolute path: {:?}", canonical),
            Err(_) => println!("[noto] Successfully persisted deltas to path: {:?}", delta_path),
        }
    }

    println!("[noto] PoC execution cycle completed successfully.");
    Ok(())
}
