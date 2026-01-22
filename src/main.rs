use iced::{Element, Sandbox, Settings, widget::text};
use std::path::PathBuf;

fn main() -> iced::Result {
    QrystalApp::run(Settings::default())
}

// The Atom of Qrystal
#[derive(Debug, Clone)]
enum NodeType {
    Cluster,       // A gravity well (Folder)
    Object(String), // A data unit (File + Extension)
}

#[allow(dead_code)]
struct Node {
    path: PathBuf,
    node_type: NodeType,
    mass: f32, // Calculated by semantic density
    // Physics state handled by fdg-sim
}

// Placeholder for Phase 4
struct GraphState;

struct QrystalApp {
    graph: GraphState,
    // The Cluster currently providing the main gravity
    active_cluster: PathBuf,
}

impl Sandbox for QrystalApp {
    type Message = ();

    fn new() -> Self {
        QrystalApp {
            graph: GraphState,
            active_cluster: PathBuf::from("."),
        }
    }

    fn title(&self) -> String {
        String::from("Qrystal")
    }

    fn update(&mut self, _message: Self::Message) {
        // Physics tick will go here
    }

    fn view(&self) -> Element<'_, Self::Message> {
        text("Qrystal: A negentropic interface").into()
    }
}
