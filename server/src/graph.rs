use crate::db_utils::with_connection;
use crate::types::{EdgeType, GraphEdge, GraphMetadata, GraphNode, KnowledgeGraph, NodeType};
use crate::utils::{log_step, log_success};
use chrono::Utc;
use std::collections::HashMap;

pub async fn generate_knowledge_graph() -> KnowledgeGraph {
    log_step("🕸️", "Generating knowledge graph from database...");

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    // Generate nodes from issues and projects
    with_connection("generate_graph", |conn| {
        // Create project nodes
        let mut project_stmt = conn
            .prepare("SELECT id, key, name FROM projects")
            .expect("Failed to prepare project query");

        let project_rows = project_stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let key: String = row.get(1)?;
                let name: String = row.get(2)?;
                Ok((id, key, name))
            })
            .expect("Failed to query projects");

        for row in project_rows {
            let (id, key, name) = row.expect("Failed to read project row");
            nodes.push(GraphNode {
                id: format!("project_{}", id),
                label: format!("{} ({})", name, key),
                node_type: NodeType::Project,
                size: 20.0,
                color: "#4F46E5".to_string(), // Indigo for projects
                metadata: Some(serde_json::json!({
                    "original_id": id,
                    "key": key,
                    "name": name,
                    "type": "project"
                })),
            });
        }

        // Create issue nodes
        let mut issue_stmt = conn
            .prepare("SELECT id, key, summary, status, project FROM issues LIMIT 100")
            .expect("Failed to prepare issue query");

        let issue_rows = issue_stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let key: String = row.get(1)?;
                let summary: String = row.get(2).unwrap_or_default();
                let status: String = row.get(3).unwrap_or_default();
                let project_json: String = row.get(4).unwrap_or_default();
                Ok((id, key, summary, status, project_json))
            })
            .expect("Failed to query issues");

        for row in issue_rows {
            let (id, key, summary, status, project_json) = row.expect("Failed to read issue row");
            
            // Determine node size based on status or other factors
            let size = match status.as_str() {
                "Done" | "Closed" => 8.0,
                "In Progress" | "In Review" => 12.0,
                _ => 10.0,
            };

            // Color coding by status
            let color = match status.as_str() {
                "Done" | "Closed" => "#10B981", // Green
                "In Progress" => "#F59E0B", // Yellow
                "In Review" => "#8B5CF6", // Purple
                "Open" | "To Do" => "#EF4444", // Red
                _ => "#6B7280", // Gray
            };

            nodes.push(GraphNode {
                id: format!("issue_{}", id),
                label: format!("{}: {}", key, summary.chars().take(30).collect::<String>()),
                node_type: NodeType::Issue,
                size,
                color: color.to_string(),
                metadata: Some(serde_json::json!({
                    "original_id": id,
                    "key": key,
                    "summary": summary,
                    "status": status,
                    "type": "issue"
                })),
            });

            // Create edge from issue to project if project data exists
            if !project_json.is_empty() {
                if let Ok(project_value) = serde_json::from_str::<serde_json::Value>(&project_json) {
                    if let Some(project_id) = project_value.get("id").and_then(|v| v.as_str()) {
                        edges.push(GraphEdge {
                            id: format!("issue_{}_to_project_{}", id, project_id),
                            source: format!("issue_{}", id),
                            target: format!("project_{}", project_id),
                            edge_type: EdgeType::PartOf,
                            weight: 1.0,
                            label: Some("belongs to".to_string()),
                        });
                    }
                }
            }
        }

        // Create edges between issues that reference each other
        let mut link_stmt = conn
            .prepare("SELECT id, issue_links FROM issues WHERE issue_links IS NOT NULL AND issue_links != '' LIMIT 100")
            .expect("Failed to prepare issue links query");

        let link_rows = link_stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let links_json: String = row.get(1)?;
                Ok((id, links_json))
            })
            .expect("Failed to query issue links");

        for row in link_rows {
            let (source_id, links_json) = row.expect("Failed to read issue link row");
            
            if let Ok(links_value) = serde_json::from_str::<serde_json::Value>(&links_json) {
                if let Some(links_array) = links_value.as_array() {
                    for link in links_array {
                        // Extract linked issue information
                        if let (Some(outward_issue), Some(inward_issue)) = (
                            link.get("outwardIssue"),
                            link.get("inwardIssue"),
                        ) {
                            // Determine target issue ID
                            let target_key = outward_issue
                                .get("key")
                                .or_else(|| inward_issue.get("key"))
                                .and_then(|v| v.as_str());
                                
                            if let Some(target_key) = target_key {
                                // Find target issue ID by key
                                let mut target_stmt = conn
                                    .prepare("SELECT id FROM issues WHERE key = ?")
                                    .expect("Failed to prepare target query");
                                    
                                if let Ok(mut target_rows) = target_stmt.query_map([target_key], |row| {
                                    Ok(row.get::<_, String>(0)?)
                                }) {
                                    if let Some(Ok(target_id)) = target_rows.next() {
                                        edges.push(GraphEdge {
                                            id: format!("link_{}_{}", source_id, target_id),
                                            source: format!("issue_{}", source_id),
                                            target: format!("issue_{}", target_id),
                                            edge_type: EdgeType::References,
                                            weight: 2.0,
                                            label: Some("references".to_string()),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    // Generate metadata
    let mut node_type_counts = HashMap::new();
    let mut edge_type_counts = HashMap::new();

    for node in &nodes {
        *node_type_counts.entry(node.node_type.clone()).or_insert(0) += 1;
    }

    for edge in &edges {
        *edge_type_counts.entry(edge.edge_type.clone()).or_insert(0) += 1;
    }

    let metadata = GraphMetadata {
        total_nodes: nodes.len(),
        total_edges: edges.len(),
        node_types: node_type_counts,
        edge_types: edge_type_counts,
        generated_at: Utc::now().to_rfc3339(),
    };

    log_success(&format!(
        "Knowledge graph generated: {} nodes, {} edges",
        nodes.len(),
        edges.len()
    ));

    KnowledgeGraph {
        nodes,
        edges,
        metadata,
    }
}

// Add more sophisticated analysis functions
pub async fn analyze_graph_patterns(graph: &KnowledgeGraph) -> serde_json::Value {
    log_step("📊", "Analyzing graph patterns...");

    let mut analysis = serde_json::Map::new();
    
    // Calculate basic statistics
    analysis.insert("total_nodes".to_string(), serde_json::Value::Number(graph.nodes.len().into()));
    analysis.insert("total_edges".to_string(), serde_json::Value::Number(graph.edges.len().into()));
    
    // Calculate node degree distribution
    let mut degree_counts: HashMap<String, usize> = HashMap::new();
    for edge in &graph.edges {
        *degree_counts.entry(edge.source.clone()).or_insert(0) += 1;
        *degree_counts.entry(edge.target.clone()).or_insert(0) += 1;
    }
    
    let max_degree = degree_counts.values().max().unwrap_or(&0);
    let avg_degree: f64 = degree_counts.values().sum::<usize>() as f64 / graph.nodes.len() as f64;
    
    analysis.insert("max_degree".to_string(), serde_json::Value::Number((*max_degree).into()));
    analysis.insert("avg_degree".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(avg_degree).unwrap_or(serde_json::Number::from(0))));
    
    // Find most connected nodes
    let mut degree_vec: Vec<(String, usize)> = degree_counts.into_iter().collect();
    degree_vec.sort_by(|a, b| b.1.cmp(&a.1));
    
    let top_nodes: Vec<serde_json::Value> = degree_vec.into_iter().take(5)
        .map(|(node_id, degree)| {
            serde_json::json!({
                "node_id": node_id,
                "degree": degree,
                "label": graph.nodes.iter()
                    .find(|n| n.id == node_id)
                    .map(|n| n.label.clone())
                    .unwrap_or_else(|| "Unknown".to_string())
            })
        })
        .collect();
    
    analysis.insert("most_connected_nodes".to_string(), serde_json::Value::Array(top_nodes));
    
    log_success("Graph analysis complete");
    serde_json::Value::Object(analysis)
}