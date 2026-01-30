//! Diagram data models

use serde::{Deserialize, Serialize};

/// Diagram type enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiagramType {
    Flowchart,
    Gantt,
    Sequence,
    ClassDiagram,
    State,
}

impl DiagramType {
    /// Parse diagram type from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "flowchart" => Some(DiagramType::Flowchart),
            "gantt" => Some(DiagramType::Gantt),
            "sequence" => Some(DiagramType::Sequence),
            "class-diagram" | "class" => Some(DiagramType::ClassDiagram),
            "state" => Some(DiagramType::State),
            _ => None,
        }
    }
}

/// Main diagram enum that can hold any diagram type
#[derive(Debug, Clone)]
pub enum Diagram {
    Flowchart(FlowchartDiagram),
    Gantt(GanttDiagram),
    Sequence(SequenceDiagram),
    Class(ClassDiagram),
    State(StateDiagram),
}

// ==================== Flowchart ====================

/// Flowchart diagram
#[derive(Debug, Clone)]
pub struct FlowchartDiagram {
    pub nodes: Vec<FlowchartNode>,
    pub edges: Vec<FlowchartEdge>,
}

/// Flowchart node
#[derive(Debug, Clone)]
pub struct FlowchartNode {
    pub id: String,
    pub label: String,
    pub node_type: FlowchartNodeType,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Flowchart node type
#[derive(Debug, Clone, PartialEq)]
pub enum FlowchartNodeType {
    StartEnd,
    Process,
    Decision,
    Data,
    Document,
}

/// Flowchart edge
#[derive(Debug, Clone)]
pub struct FlowchartEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
}

// ==================== Gantt ====================

/// Gantt chart diagram
#[derive(Debug, Clone)]
pub struct GanttDiagram {
    pub tasks: Vec<GanttTask>,
    pub start_date: String,
    pub end_date: String,
}

/// Gantt task
#[derive(Debug, Clone)]
pub struct GanttTask {
    pub id: String,
    pub label: String,
    pub start_date: String,
    pub duration_days: u32,
    pub dependencies: Vec<String>,
}

// ==================== Sequence ====================

/// Sequence diagram
#[derive(Debug, Clone)]
pub struct SequenceDiagram {
    pub actors: Vec<SequenceActor>,
    pub messages: Vec<SequenceMessage>,
}

/// Sequence actor
#[derive(Debug, Clone)]
pub struct SequenceActor {
    pub id: String,
    pub label: String,
    pub x: f64,
}

/// Sequence message
#[derive(Debug, Clone)]
pub struct SequenceMessage {
    pub from: String,
    pub to: String,
    pub label: String,
    pub message_type: SequenceMessageType,
    pub y: f64,
}

/// Sequence message type
#[derive(Debug, Clone, PartialEq)]
pub enum SequenceMessageType {
    Sync,
    Async,
    Reply,
    SelfCall,
}

// ==================== Class Diagram ====================

/// Class diagram
#[derive(Debug, Clone)]
pub struct ClassDiagram {
    pub classes: Vec<ClassNode>,
    pub relationships: Vec<ClassRelationship>,
}

/// Class node
#[derive(Debug, Clone)]
pub struct ClassNode {
    pub id: String,
    pub label: String,
    pub attributes: Vec<String>,
    pub methods: Vec<String>,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Class relationship
#[derive(Debug, Clone)]
pub struct ClassRelationship {
    pub from: String,
    pub to: String,
    pub relationship_type: ClassRelationshipType,
    pub label: Option<String>,
}

/// Class relationship type
#[derive(Debug, Clone, PartialEq)]
pub enum ClassRelationshipType {
    Association,
    Aggregation,
    Composition,
    Inheritance,
    Dependency,
}

// ==================== State Diagram ====================

/// State diagram
#[derive(Debug, Clone)]
pub struct StateDiagram {
    pub states: Vec<StateNode>,
    pub transitions: Vec<StateTransition>,
}

/// State node
#[derive(Debug, Clone)]
pub struct StateNode {
    pub id: String,
    pub label: String,
    pub state_type: StateType,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// State type
#[derive(Debug, Clone, PartialEq)]
pub enum StateType {
    Initial,
    Normal,
    Final,
}

/// State transition
#[derive(Debug, Clone)]
pub struct StateTransition {
    pub from: String,
    pub to: String,
    pub label: String,
}