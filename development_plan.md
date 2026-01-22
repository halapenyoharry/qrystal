The Final Qrystal Prompt (Strict Terminology)
Here is the updated prompt. I have scrubbed the words "File" and "Directory" from the code structure and replaced them with Object and Cluster.

Paste this into Jules to begin.

Markdown
# Agent Role: Lead Rust Engineer
# Mission: Initialize "Qrystal" OS Interface

## 1. Context & Philosophy
We are building **Qrystal**, a negentropic OS interface.
* **The Terminology:**
    * **Object:** (formerly "file"). A distinct unit of information (text, code, image).
    * **Cluster:** (formerly "directory"). A gravitational center that binds Objects together.
    * **Lens:** (formerly "app"). The method of viewing an Object.
* **The Physics:** Objects are nodes in a force-directed graph. Clusters create gravity wells.
* **The Tech:** Rust, Iced (GUI), `fdg-sim` (Physics).

## 2. Environment Setup (Execute First)
*Target System: Ubuntu Linux (Jules VM)*

Run this script to prepare the build environment:
```bash
# System Dependencies for Iced
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev libfreetype6-dev libexpat1-dev libxcb-composite0-dev libfontconfig1-dev cmake

# Confirm Toolchain
rustc --version
cargo --version
3. Phase 1: Repository Initialization
Initialize Project: cargo new qrystal

Git Setup:

Run git init.

Create .gitignore.

Documentation (README.md):

Qrystal
A negentropic interface for the post-file era.

Clusters: Gravitational nodes that organize context.

Objects: Data points with semantic mass.

Lenses: Context-aware rendering of information.

4. Phase 2: Dependencies (Cargo.toml)
Ini, TOML
[dependencies]
# GUI
iced = { version = "0.12", features = ["canvas", "tokio", "debug"] }
cosmic-text = "0.11" 

# Physics & Math
fdg-sim = "0.6"
glam = "0.24"

# System
walkdir = "2.4"
notify = "6.1"
kamadak-exif = "0.5"
5. Phase 3: The Data Model (src/main.rs)
Strictly adhere to this terminology in your Structs:

Rust
// The Atom of Qrystal
enum NodeType {
    Cluster,       // A gravity well (Folder)
    Object(String), // A data unit (File + Extension)
}

struct Node {
    path: std::path::PathBuf,
    node_type: NodeType,
    mass: f32, // Calculated by semantic density
    // Physics state handled by fdg-sim
}

struct QrystalApp {
    graph: GraphState,
    // The Cluster currently providing the main gravity
    active_cluster: std::path::PathBuf, 
}
6. Phase 4: Implementation Plan
Stage A: The "Pulse" (Physics Loop)
Create the GraphState struct.

Implement the physics tick (60fps).

Validation: Print coordinates of a test Object orbiting a Cluster.

Stage B: The "Viewport" (Rendering)
Draw Clusters as large, heavy circles (Blue).

Draw Objects as smaller satellites (White).

Implement iced::widget::canvas::Program.

Stage C: The "Lens" (Interaction)
Clicking a Cluster: It becomes the new center of the graph (re-scans children).

Clicking an Object: It triggers the "Lens" (renders text content in a center panel).

Execution Order
Run Environment Setup.

Initialize Repo.

Add Dependencies.

Implement Stage A.
