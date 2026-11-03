notebook_os_srd.md
I think we're finally at the point where requirements stop and implementation begins.
And the most important requirement we've discovered is actually not a Rust requirement at all:
The runtime is not responsible for inventing correspondence.
The runtime is responsible for creating the conditions under which correspondence may emerge.
That is a radically smaller and more testable problem.
PoC Runtime Requirements
RR-1 Notebook Persistence
The runtime shall load a notebook from:
Plain Text
TIAN
*.toml
 
DI
payloads
 
RAN
delta pool
without requiring:
Plain Text
Subscriptions
Basins
Agents
Other notebooks
RR-2 Frame Hydration
The runtime shall accept:
Plain Text
Question
and produce:
Plain Text
Frame
from notebook-local resources.
No external dependencies.
RR-3 Graph Realization
The runtime shall create:
Plain Text
Graph
as a temporary answer artifact.
Invariant:
Plain Text
Graphs are not persisted.
Persist only:
Plain Text
TIAN
REN
DI
RAN
components.
RR-4 Correspondence Discovery
The runtime shall compare local registry surfaces and identify:
Plain Text
Potential Correspondence
Output:
Plain Text
Δ Candidate
 
No ontology solving required.
RR-5 Delta Persistence
The runtime shall append discovered candidates to:
Plain Text
enlighten_delta.yml
 
General rule:
Plain Text
Persist Δ
 
Not Graph
RR-6 Metrics Observation
The runtime shall calculate:
Plain Text
ρ
relevance density
 
τ
tension
 
ΔE
novel correspondence
 
D
diversity
and record them in metrics.
No forecasting.
Only observation.
Runtime Invariants
These are the important ones.
I-1
Plain Text
Graph = Answer
Not storage.
Not database.
Not model.
Hydrated answer artifact.
I-2
Plain Text
Notebook = KG Jurisdiction
Persistent.
Everything else may emerge or dissolve.
I-3
Plain Text
TIAN governs
 
REN binds
 
DI realizes
 
RAN remembers
 
I-4
Plain Text
Assets = Payloads
Assets are not graph primitives.
Payloads participate through nodes.
I-5
Plain Text
Knowledge emerges through correspondence.
Not accumulation.
This follows directly from:
Knowledge provides nodes; wisdom discovers relationships.
I-6
Plain Text
Questions create local relevance.
Questions hydrate frames.
Frames hydrate graphs.
Graphs answer questions.
Component Inventory
Only after requirements:
Plain Text
Notebook Runtime
 
TIAN Loader
 
Registry Surface Engine
 
Frame Hydrator
 
Correspondence Engine
 
Delta Pool
No agent.
No promotion engine.
No migration engine.
No basin runtime.
Not yet.
The Big One
What I think we've finally learned:
Earlier we thought:
Plain Text
Hydrate Graph
was the objective.
Now it appears:
Plain Text
Generate ΔE
 
is the objective.
The graph is only the mechanism.
The proof condition remains very small:
Plain Text
1 Notebook
1 Payload
1 Question
↓
Graph
↓
ΔE > 0
Any success proves the architecture.
More success only proves scale.