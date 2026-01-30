//! Diagram syntax parser

use crate::core::Error;
use crate::core::Result;
use crate::content::rst::diagrams::models::*;
use chrono::NaiveDate;

/// Diagram parser
pub struct DiagramParser;

impl DiagramParser {
    /// Create a new diagram parser
    pub fn new() -> Self {
        Self
    }

    /// Parse diagram from text
    pub fn parse(&self, diagram_type: &str, content: &str) -> Result<Diagram> {
        let diagram_type = DiagramType::from_str(diagram_type)
            .ok_or_else(|| Error::content(format!("Unknown diagram type: {}", diagram_type)))?;

        match diagram_type {
            DiagramType::Flowchart => {
                let diagram = self.parse_flowchart(content)?;
                Ok(Diagram::Flowchart(diagram))
            }
            DiagramType::Gantt => {
                let diagram = self.parse_gantt(content)?;
                Ok(Diagram::Gantt(diagram))
            }
            DiagramType::Sequence => {
                let diagram = self.parse_sequence(content)?;
                Ok(Diagram::Sequence(diagram))
            }
            DiagramType::ClassDiagram => {
                let diagram = self.parse_class(content)?;
                Ok(Diagram::Class(diagram))
            }
            DiagramType::State => {
                let diagram = self.parse_state(content)?;
                Ok(Diagram::State(diagram))
            }
        }
    }

    /// Parse flowchart
    fn parse_flowchart(&self, content: &str) -> Result<FlowchartDiagram> {
        let mut nodes: Vec<FlowchartNode> = Vec::new();
        let mut edges: Vec<FlowchartEdge> = Vec::new();
        let mut node_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse edge: "Node1 -> Node2 -> Node3"
            if line.contains("->") {
                let parts: Vec<&str> = line.split("->").collect();
                if parts.len() >= 2 {
                    // Create all nodes mentioned in this line
                    for part in &parts {
                        let node_id = part.trim();
                        if !node_id.is_empty() && !node_map.contains_key(node_id) {
                            let node_index = nodes.len();
                            node_map.insert(node_id.to_string(), node_index);
                            
                            let node_type = if node_id == "Start" || node_id == "End" {
                                FlowchartNodeType::StartEnd
                            } else if node_id == "Decision" {
                                FlowchartNodeType::Decision
                            } else {
                                FlowchartNodeType::Process
                            };

                            nodes.push(FlowchartNode {
                                id: node_id.to_string(),
                                label: node_id.to_string(),
                                node_type,
                                x: 0.0, // Will be calculated during layout
                                y: 0.0,
                                width: 100.0,
                                height: 40.0,
                            });
                        }
                    }

                    // Create edges between consecutive nodes
                    for i in 0..parts.len() - 1 {
                        let from = parts[i].trim();
                        let to = parts[i + 1].trim();
                        if !from.is_empty() && !to.is_empty() {
                            edges.push(FlowchartEdge {
                                from: from.to_string(),
                                to: to.to_string(),
                                label: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(FlowchartDiagram { nodes, edges })
    }

    /// Parse gantt chart
    fn parse_gantt(&self, content: &str) -> Result<GanttDiagram> {
        let mut tasks: Vec<GanttTask> = Vec::new();
        let mut start_date = String::from("2024-01-01");
        let mut end_date = String::from("2024-01-31");

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse task: "Task1 [2024-01-01] : 5d"
            if line.contains('[') && line.contains(']') && line.contains(':') {
                let task_name = line.split('[').next().unwrap_or("").trim();
                let date_str = line
                    .split('[')
                    .nth(1)
                    .and_then(|s| s.split(']').next())
                    .unwrap_or("2024-01-01")
                    .trim();
                
                let duration_str = line
                    .split(':')
                    .nth(1)
                    .unwrap_or("5d")
                    .trim();
                
                let duration = duration_str
                    .trim_end_matches('d')
                    .parse::<u32>()
                    .unwrap_or(5);

                tasks.push(GanttTask {
                    id: task_name.to_string(),
                    label: task_name.to_string(),
                    start_date: date_str.to_string(),
                    duration_days: duration,
                    dependencies: Vec::new(),
                });

                // Update start/end dates
                if let Ok(parsed_date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    if start_date.as_str() > date_str {
                        start_date = date_str.to_string();
                    }
                    
                    let task_end = parsed_date + chrono::Duration::days(duration as i64);
                    let task_end_str = task_end.format("%Y-%m-%d").to_string();
                    if end_date.as_str() < task_end_str.as_str() {
                        end_date = task_end_str;
                    }
                }
            }
        }

        Ok(GanttDiagram {
            tasks,
            start_date,
            end_date,
        })
    }

    /// Parse sequence diagram
    fn parse_sequence(&self, content: &str) -> Result<SequenceDiagram> {
        let mut actors: Vec<SequenceActor> = Vec::new();
        let mut messages: Vec<SequenceMessage> = Vec::new();
        let mut actor_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (idx, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse message: "Alice -> Bob: Hello"
            if line.contains("->") && line.contains(':') {
                let parts: Vec<&str> = line.split("->").collect();
                if parts.len() == 2 {
                    let from = parts[0].trim();
                    let rest = parts[1].trim();
                    let to_label: Vec<&str> = rest.split(':').collect();
                    let to = to_label[0].trim();
                    let label = if to_label.len() > 1 {
                        to_label[1].trim().to_string()
                    } else {
                        String::new()
                    };

                    // Create actors if they don't exist
                    for (_actor_idx, actor_id) in [from, to].iter().enumerate() {
                        if !actor_map.contains_key(*actor_id) {
                            let actor_index = actors.len();
                            actor_map.insert(actor_id.to_string(), actor_index);
                            
                            actors.push(SequenceActor {
                                id: actor_id.to_string(),
                                label: actor_id.to_string(),
                                x: 0.0, // Will be calculated during layout
                            });
                        }
                    }

                    messages.push(SequenceMessage {
                        from: from.to_string(),
                        to: to.to_string(),
                        label,
                        message_type: SequenceMessageType::Sync,
                        y: idx as f64 * 50.0 + 80.0,
                    });
                }
            }
        }

        Ok(SequenceDiagram { actors, messages })
    }

    /// Parse class diagram
    fn parse_class(&self, content: &str) -> Result<ClassDiagram> {
        let mut classes: Vec<ClassNode> = Vec::new();
        let mut relationships: Vec<ClassRelationship> = Vec::new();
        let mut class_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse relationship: "User |+| Database"
            if line.contains('|') {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 3 {
                    let from = parts[0].trim();
                    let rel_type_str = parts[1].trim();
                    let to = parts[2].trim();

                    // Create classes if they don't exist
                    for (_idx, class_id) in [from, to].iter().enumerate() {
                        if !class_map.contains_key(*class_id) {
                            let class_index = classes.len();
                            class_map.insert(class_id.to_string(), class_index);
                            
                            classes.push(ClassNode {
                                id: class_id.to_string(),
                                label: class_id.to_string(),
                                attributes: Vec::new(),
                                methods: Vec::new(),
                                x: 0.0, // Will be calculated during layout
                                y: 0.0,
                                width: 120.0,
                                height: 80.0,
                            });
                        }
                    }

                    let relationship_type = match rel_type_str {
                        "+|+" => ClassRelationshipType::Composition,
                        "-|>" => ClassRelationshipType::Inheritance,
                        "<->" => ClassRelationshipType::Association,
                        "-|o" => ClassRelationshipType::Aggregation,
                        "..>" => ClassRelationshipType::Dependency,
                        _ => ClassRelationshipType::Association,
                    };

                    relationships.push(ClassRelationship {
                        from: from.to_string(),
                        to: to.to_string(),
                        relationship_type,
                        label: None,
                    });
                }
            }
        }

        Ok(ClassDiagram {
            classes,
            relationships,
        })
    }

    /// Parse state diagram
    fn parse_state(&self, content: &str) -> Result<StateDiagram> {
        let mut states: Vec<StateNode> = Vec::new();
        let mut transitions: Vec<StateTransition> = Vec::new();
        let mut state_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Parse transition: "Idle -> Running : start"
            if line.contains("->") && line.contains(':') {
                let parts: Vec<&str> = line.split("->").collect();
                if parts.len() == 2 {
                    let from = parts[0].trim();
                    let rest = parts[1].trim();
                    let to_label: Vec<&str> = rest.split(':').collect();
                    let to = to_label[0].trim();
                    let label = if to_label.len() > 1 {
                        to_label[1].trim().to_string()
                    } else {
                        String::new()
                    };

                    // Create states if they don't exist
                    for (_idx, state_id) in [from, to].iter().enumerate() {
                        if !state_map.contains_key(*state_id) {
                            let state_index = states.len();
                            state_map.insert(state_id.to_string(), state_index);
                            
                            let state_type = if *state_id == "Idle" || *state_id == "Start" {
                                StateType::Initial
                            } else if *state_id == "End" {
                                StateType::Final
                            } else {
                                StateType::Normal
                            };

                            states.push(StateNode {
                                id: state_id.to_string(),
                                label: state_id.to_string(),
                                state_type,
                                x: 0.0, // Will be calculated during layout
                                y: 0.0,
                                width: 100.0,
                                height: 40.0,
                            });
                        }
                    }

                    transitions.push(StateTransition {
                        from: from.to_string(),
                        to: to.to_string(),
                        label,
                    });
                }
            }
        }

        Ok(StateDiagram { states, transitions })
    }
}

impl Default for DiagramParser {
    fn default() -> Self {
        Self::new()
    }
}