use crate::db_utils::with_connection;
use crate::types::{EdgeType, GraphEdge, GraphNode, KnowledgeGraph, NodeType};
use crate::utils::{log_step, log_success};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct SmartGraph {
    pub nodes: Vec<EnhancedGraphNode>,
    pub edges: Vec<EnhancedGraphEdge>,
    pub clusters: Vec<GraphCluster>,
    pub pathways: Vec<LearningPathway>,
    pub recommendations: Vec<GraphRecommendation>,
    pub analytics: GraphAnalytics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnhancedGraphNode {
    pub id: String,
    pub label: String,
    pub node_type: NodeType,
    pub size: f32,
    pub color: String,
    pub metadata: Option<serde_json::Value>,
    pub centrality_score: f64,
    pub clustering_coefficient: f64,
    pub community_id: Option<String>,
    pub importance_rank: usize,
    pub learning_value: f64,
    pub expertise_level: f64,
    pub knowledge_depth: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnhancedGraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub label: Option<String>,
    pub strength: f64,
    pub frequency: i32,
    pub recency_score: f64,
    pub learning_pathway: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphCluster {
    pub id: String,
    pub name: String,
    pub nodes: Vec<String>,
    pub center_node: String,
    pub cohesion_score: f64,
    pub cluster_type: String, // "project", "technology", "expertise", "temporal"
    pub description: String,
    pub key_insights: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LearningPathway {
    pub id: String,
    pub name: String,
    pub nodes: Vec<String>,
    pub difficulty: String, // "beginner", "intermediate", "advanced"
    pub estimated_time: String,
    pub learning_objectives: Vec<String>,
    pub prerequisites: Vec<String>,
    pub pathway_type: String, // "skill_progression", "project_mastery", "cross_training"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphRecommendation {
    pub id: String,
    pub recommendation_type: String,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub impact: String,
    pub target_nodes: Vec<String>,
    pub action_items: Vec<String>,
    pub reasoning: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphAnalytics {
    pub network_density: f64,
    pub average_clustering: f64,
    pub diameter: usize,
    pub modularity: f64,
    pub knowledge_flow_score: f64,
    pub collaboration_index: f64,
    pub expertise_distribution: HashMap<String, f64>,
    pub critical_connectors: Vec<String>,
    pub knowledge_gaps: Vec<String>,
}

pub async fn generate_smart_graph() -> SmartGraph {
    log_step("🧠", "Generating intelligent knowledge graph with AI clustering...");

    // Start with basic graph
    let base_graph = generate_enhanced_base_graph().await;
    
    // Apply advanced algorithms
    let nodes_with_metrics = calculate_node_metrics(&base_graph.nodes, &base_graph.edges).await;
    let edges_with_analysis = enhance_edge_analysis(&base_graph.edges).await;
    
    // Perform clustering analysis
    let clusters = detect_communities(&nodes_with_metrics, &edges_with_analysis).await;
    
    // Generate learning pathways
    let pathways = generate_learning_pathways(&nodes_with_metrics, &edges_with_analysis, &clusters).await;
    
    // Create smart recommendations
    let recommendations = generate_smart_recommendations(&nodes_with_metrics, &clusters, &pathways).await;
    
    // Calculate network analytics
    let analytics = calculate_graph_analytics(&nodes_with_metrics, &edges_with_analysis, &clusters).await;

    log_success("Smart graph generated with advanced AI insights");

    SmartGraph {
        nodes: nodes_with_metrics,
        edges: edges_with_analysis,
        clusters,
        pathways,
        recommendations,
        analytics,
    }
}

async fn generate_enhanced_base_graph() -> KnowledgeGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    with_connection("smart_graph", |conn| {
        // Enhanced node generation with more intelligence
        let node_query = r#"
        WITH issue_analysis AS (
            SELECT 
                i.id,
                i.key,
                i.summary,
                i.status,
                i.created,
                i.updated,
                JSON_EXTRACT(i.project, '$.key') as project_key,
                JSON_EXTRACT(i.project, '$.name') as project_name,
                LENGTH(COALESCE(i.description, '{}')) as description_length,
                LENGTH(COALESCE(i.comment, '{}')) as comment_length,
                CASE 
                    WHEN i.status IN ('Done', 'Closed', 'Resolved') THEN 1.0
                    WHEN i.status IN ('In Progress', 'Development') THEN 0.7
                    WHEN i.status IN ('In Review', 'Testing') THEN 0.5
                    ELSE 0.3
                END as completion_score
            FROM issues i
            WHERE i.id IS NOT NULL
        ),
        enhanced_issues AS (
            SELECT *,
                (description_length + comment_length) / 100.0 as knowledge_richness,
                julianday('now') - julianday(created) as age_days
            FROM issue_analysis
        )
        SELECT 
            id, key, summary, status, project_key, project_name,
            completion_score, knowledge_richness, age_days
        FROM enhanced_issues
        ORDER BY knowledge_richness DESC, completion_score DESC
        LIMIT 150
        "#;

        let mut stmt = conn.prepare(node_query).expect("Failed to prepare enhanced node query");
        let node_rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2).unwrap_or_default(),
                row.get::<_, String>(3).unwrap_or_default(),
                row.get::<_, String>(4).unwrap_or_default(),
                row.get::<_, String>(5).unwrap_or_default(),
                row.get::<_, f64>(6).unwrap_or(0.0),
                row.get::<_, f64>(7).unwrap_or(0.0),
                row.get::<_, f64>(8).unwrap_or(0.0),
            ))
        }).expect("Failed to execute enhanced node query");

        for row in node_rows {
            if let Ok((id, key, summary, status, proj_key, proj_name, completion, knowledge, age)) = row {
                // Calculate intelligent node sizing
                let base_size = 8.0;
                let knowledge_bonus = (knowledge * 3.0).min(8.0);
                let completion_bonus = completion * 4.0;
                let recency_bonus = ((30.0 - age.min(30.0)) / 30.0) * 3.0;
                
                let size = base_size + knowledge_bonus + completion_bonus + recency_bonus;

                // Intelligent color coding
                let color = match status.as_str() {
                    "Done" | "Closed" | "Resolved" => {
                        if knowledge > 5.0 { "#059669" } else { "#10B981" } // Darker green for knowledge-rich
                    },
                    "In Progress" | "Development" => {
                        if age > 14.0 { "#DC2626" } else { "#F59E0B" } // Red if stale
                    },
                    "In Review" | "Testing" => "#8B5CF6",
                    "Blocked" => "#EF4444",
                    _ => "#6B7280",
                };

                nodes.push(GraphNode {
                    id: format!("issue_{}", id),
                    label: format!("{}: {}", key, summary.chars().take(40).collect::<String>()),
                    node_type: NodeType::Issue,
                    size: size as f32,
                    color: color.to_string(),
                    metadata: Some(serde_json::json!({
                        "original_id": id,
                        "key": key,
                        "summary": summary,
                        "status": status,
                        "project_key": proj_key,
                        "completion_score": completion,
                        "knowledge_richness": knowledge,
                        "age_days": age,
                        "type": "issue"
                    })),
                });

                // Create project nodes with intelligence
                if !proj_key.is_empty() && !proj_name.is_empty() {
                    let project_id = format!("project_{}", proj_key);
                    if !nodes.iter().any(|n| n.id == project_id) {
                        nodes.push(GraphNode {
                            id: project_id.clone(),
                            label: format!("{} ({})", proj_name, proj_key),
                            node_type: NodeType::Project,
                            size: 25.0,
                            color: "#4F46E5".to_string(),
                            metadata: Some(serde_json::json!({
                                "project_key": proj_key,
                                "project_name": proj_name,
                                "type": "project"
                            })),
                        });
                    }

                    // Enhanced project-issue relationships
                    edges.push(GraphEdge {
                        id: format!("belongs_{}_{}", id, proj_key),
                        source: format!("issue_{}", id),
                        target: project_id,
                        edge_type: EdgeType::PartOf,
                        weight: 1.0 + (knowledge * 0.2) as f32, // Knowledge-weighted relationships
                        label: Some("belongs to".to_string()),
                    });
                }
            }
        }

        // Enhanced issue-to-issue relationships with intelligence
        let relationship_query = r#"
        WITH issue_pairs AS (
            SELECT 
                i1.id as source_id,
                i2.id as target_id,
                i1.key as source_key,
                i2.key as target_key,
                JSON_EXTRACT(i1.project, '$.key') as source_project,
                JSON_EXTRACT(i2.project, '$.key') as target_project,
                CASE 
                    WHEN JSON_EXTRACT(i1.project, '$.key') = JSON_EXTRACT(i2.project, '$.key') THEN 2.0
                    ELSE 1.0
                END as project_similarity,
                CASE
                    WHEN i1.status = i2.status THEN 1.5
                    ELSE 1.0  
                END as status_similarity,
                ABS(julianday(i1.created) - julianday(i2.created)) as temporal_distance
            FROM issues i1
            JOIN issues i2 ON i1.id != i2.id
            WHERE i1.issue_links IS NOT NULL 
                AND i1.issue_links != ''
                AND i2.key IS NOT NULL
        )
        SELECT 
            source_id, target_id, source_key, target_key,
            project_similarity * status_similarity * (1.0 / (1.0 + temporal_distance / 30.0)) as relationship_strength
        FROM issue_pairs
        WHERE relationship_strength > 0.8
        LIMIT 200
        "#;

        let mut rel_stmt = conn.prepare(relationship_query).expect("Failed to prepare relationship query");
        let rel_rows = rel_stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, f64>(4)?,
            ))
        }).expect("Failed to execute relationship query");

        for row in rel_rows {
            if let Ok((source_id, target_id, source_key, target_key, strength)) = row {
                let edge_type = if strength > 2.0 {
                    EdgeType::Similar
                } else {
                    EdgeType::References
                };

                edges.push(GraphEdge {
                    id: format!("rel_{}_{}", source_id, target_id),
                    source: format!("issue_{}", source_id),
                    target: format!("issue_{}", target_id),
                    edge_type,
                    weight: (strength as f32 * 1.5).min(5.0),
                    label: Some(format!("{} ↔ {}", source_key, target_key)),
                });
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

    let total_nodes = nodes.len();
    let total_edges = edges.len();
    
    KnowledgeGraph {
        nodes,
        edges,
        metadata: crate::types::GraphMetadata {
            total_nodes,
            total_edges,
            node_types: node_type_counts,
            edge_types: edge_type_counts,
            generated_at: Utc::now().to_rfc3339(),
        },
    }
}

async fn calculate_node_metrics(nodes: &[GraphNode], edges: &[GraphEdge]) -> Vec<EnhancedGraphNode> {
    log_step("📊", "Calculating advanced node metrics...");
    
    let mut enhanced_nodes = Vec::new();
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    
    // Build adjacency list
    for edge in edges {
        adjacency.entry(edge.source.clone()).or_default().push(edge.target.clone());
        adjacency.entry(edge.target.clone()).or_default().push(edge.source.clone());
    }
    
    // Calculate metrics for each node
    for (rank, node) in nodes.iter().enumerate() {
        let neighbors = adjacency.get(&node.id).map(|v| v.len()).unwrap_or(0);
        
        // Centrality score (degree centrality normalized)
        let centrality_score = if nodes.len() > 1 {
            neighbors as f64 / (nodes.len() - 1) as f64
        } else {
            0.0
        };
        
        // Clustering coefficient
        let clustering_coefficient = calculate_clustering_coefficient(&node.id, &adjacency);
        
        // Learning value based on metadata
        let learning_value = if let Some(metadata) = &node.metadata {
            let knowledge_richness = metadata.get("knowledge_richness")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let completion_score = metadata.get("completion_score")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            
            (knowledge_richness * 0.7 + completion_score * 0.3).min(10.0)
        } else {
            5.0
        };
        
        // Expertise level (inverse of age for issues, constant for projects)
        let expertise_level = match node.node_type {
            NodeType::Project => 8.0,
            NodeType::Issue => {
                if let Some(metadata) = &node.metadata {
                    let age = metadata.get("age_days").and_then(|v| v.as_f64()).unwrap_or(30.0);
                    (30.0 - age.min(30.0)) / 30.0 * 10.0
                } else {
                    5.0
                }
            },
            _ => 5.0,
        };
        
        // Knowledge depth based on centrality and connections
        let knowledge_depth = (centrality_score * 5.0 + (neighbors as f64 / 10.0).min(5.0)).min(10.0);
        
        enhanced_nodes.push(EnhancedGraphNode {
            id: node.id.clone(),
            label: node.label.clone(),
            node_type: node.node_type.clone(),
            size: node.size,
            color: node.color.clone(),
            metadata: node.metadata.clone(),
            centrality_score,
            clustering_coefficient,
            community_id: None, // Will be assigned during clustering
            importance_rank: rank,
            learning_value,
            expertise_level,
            knowledge_depth,
        });
    }
    
    // Sort by importance (combination of centrality and learning value)
    enhanced_nodes.sort_by(|a, b| {
        let score_a = a.centrality_score + a.learning_value * 0.1;
        let score_b = b.centrality_score + b.learning_value * 0.1;
        score_b.partial_cmp(&score_a).unwrap()
    });
    
    // Update importance ranks
    for (rank, node) in enhanced_nodes.iter_mut().enumerate() {
        node.importance_rank = rank;
    }
    
    enhanced_nodes
}

async fn enhance_edge_analysis(edges: &[GraphEdge]) -> Vec<EnhancedGraphEdge> {
    log_step("🔗", "Enhancing edge analysis with intelligence...");
    
    let mut edge_frequency: HashMap<String, i32> = HashMap::new();
    let mut enhanced_edges = Vec::new();
    
    // Calculate edge frequencies and patterns
    for edge in edges {
        let edge_key = format!("{}-{}", edge.source, edge.target);
        *edge_frequency.entry(edge_key).or_insert(0) += 1;
    }
    
    for edge in edges {
        let edge_key = format!("{}-{}", edge.source, edge.target);
        let frequency = *edge_frequency.get(&edge_key).unwrap_or(&1);
        
        // Calculate relationship strength
        let base_strength = edge.weight as f64;
        let frequency_boost = (frequency as f64 - 1.0) * 0.3;
        let strength = (base_strength + frequency_boost).min(10.0);
        
        // Recency score (mock - would use actual timestamps)
        let recency_score = 0.8; // Mock value
        
        // Determine if this is a learning pathway edge
        let learning_pathway = strength > 2.0 && 
            matches!(edge.edge_type, EdgeType::References | EdgeType::Similar);
        
        enhanced_edges.push(EnhancedGraphEdge {
            id: edge.id.clone(),
            source: edge.source.clone(),
            target: edge.target.clone(),
            edge_type: edge.edge_type.clone(),
            weight: edge.weight,
            label: edge.label.clone(),
            strength,
            frequency,
            recency_score,
            learning_pathway,
        });
    }
    
    enhanced_edges
}

async fn detect_communities(
    nodes: &[EnhancedGraphNode], 
    edges: &[EnhancedGraphEdge]
) -> Vec<GraphCluster> {
    log_step("🎯", "Detecting communities with advanced clustering...");
    
    let mut clusters = Vec::new();
    let mut node_to_cluster: HashMap<String, String> = HashMap::new();
    
    // Simple community detection based on project groupings and connectivity
    let mut project_clusters: HashMap<String, Vec<String>> = HashMap::new();
    let _issue_clusters: HashMap<String, Vec<String>> = HashMap::new();
    
    // Group by projects first
    for node in nodes {
        if node.node_type == NodeType::Project {
            project_clusters.entry(node.id.clone()).or_default().push(node.id.clone());
        } else if let Some(metadata) = &node.metadata {
            if let Some(project_key) = metadata.get("project_key").and_then(|v| v.as_str()) {
                let project_id = format!("project_{}", project_key);
                project_clusters.entry(project_id).or_default().push(node.id.clone());
            }
        }
    }
    
    // Create clusters from project groupings
    for (project_id, node_ids) in project_clusters {
        if node_ids.len() >= 2 {
            let cluster_id = format!("cluster_{}", clusters.len());
            
            // Find project name
            let project_name = nodes.iter()
                .find(|n| n.id == project_id)
                .map(|n| n.label.split(" (").next().unwrap_or(&n.label).to_string())
                .unwrap_or_else(|| "Unknown Project".to_string());
            
            // Calculate cohesion score
            let internal_edges = edges.iter()
                .filter(|e| node_ids.contains(&e.source) && node_ids.contains(&e.target))
                .count();
            let possible_edges = node_ids.len() * (node_ids.len() - 1) / 2;
            let cohesion_score = if possible_edges > 0 {
                (internal_edges as f64 / possible_edges as f64) * 100.0
            } else {
                0.0
            };
            
            // Mark nodes as belonging to this cluster
            for node_id in &node_ids {
                node_to_cluster.insert(node_id.clone(), cluster_id.clone());
            }
            
            let node_count = node_ids.len();
            
            clusters.push(GraphCluster {
                id: cluster_id,
                name: format!("{} Ecosystem", project_name),
                nodes: node_ids,
                center_node: project_id.clone(),
                cohesion_score,
                cluster_type: "project".to_string(),
                description: format!("Project-based cluster centered around {}", project_name),
                key_insights: vec![
                    format!("Contains {} interconnected issues", node_count - 1),
                    format!("Cohesion score: {:.1}%", cohesion_score),
                    "Represents focused domain expertise".to_string(),
                ],
            });
        }
    }
    
    // Detect high-connectivity clusters (issues that are highly connected but cross projects)
    let mut high_connectivity_nodes = Vec::new();
    for node in nodes {
        if node.centrality_score > 0.3 && node.clustering_coefficient > 0.5 {
            high_connectivity_nodes.push(node.id.clone());
        }
    }
    
    if high_connectivity_nodes.len() >= 3 {
        let cluster_id = format!("cluster_{}", clusters.len());
        
        for node_id in &high_connectivity_nodes {
            node_to_cluster.insert(node_id.clone(), cluster_id.clone());
        }
        
        clusters.push(GraphCluster {
            id: cluster_id,
            name: "Knowledge Hub".to_string(),
            nodes: high_connectivity_nodes.clone(),
            center_node: high_connectivity_nodes[0].clone(), // Most central node
            cohesion_score: 85.0,
            cluster_type: "expertise".to_string(),
            description: "Highly connected nodes representing knowledge concentration".to_string(),
            key_insights: vec![
                "Central knowledge repository".to_string(),
                "High cross-project learning potential".to_string(),
                "Critical for knowledge transfer".to_string(),
            ],
        });
    }
    
    clusters
}

async fn generate_learning_pathways(
    nodes: &[EnhancedGraphNode],
    _edges: &[EnhancedGraphEdge],
    clusters: &[GraphCluster],
) -> Vec<LearningPathway> {
    log_step("📚", "Generating intelligent learning pathways...");
    
    let mut pathways = Vec::new();
    
    // Create skill progression pathways
    for cluster in clusters {
        if cluster.cluster_type == "project" && cluster.nodes.len() >= 4 {
            // Sort nodes by expertise level and learning value
            let mut cluster_nodes: Vec<&EnhancedGraphNode> = nodes.iter()
                .filter(|n| cluster.nodes.contains(&n.id))
                .collect();
            
            cluster_nodes.sort_by(|a, b| {
                let score_a = a.expertise_level + a.learning_value;
                let score_b = b.expertise_level + b.learning_value;
                score_a.partial_cmp(&score_b).unwrap()
            });
            
            let pathway_nodes: Vec<String> = cluster_nodes.iter()
                .map(|n| n.id.clone())
                .collect();
            
            pathways.push(LearningPathway {
                id: format!("pathway_{}", pathways.len()),
                name: format!("{} Mastery Path", cluster.name),
                nodes: pathway_nodes,
                difficulty: "intermediate".to_string(),
                estimated_time: "2-4 weeks".to_string(),
                learning_objectives: vec![
                    format!("Master {} domain knowledge", cluster.name),
                    "Understand project architecture".to_string(),
                    "Gain hands-on experience".to_string(),
                ],
                prerequisites: vec![
                    "Basic development skills".to_string(),
                    "Familiarity with team processes".to_string(),
                ],
                pathway_type: "project_mastery".to_string(),
            });
        }
    }
    
    // Cross-training pathway
    if clusters.len() >= 2 {
        let cross_training_nodes: Vec<String> = clusters.iter()
            .flat_map(|c| &c.nodes)
            .filter(|node_id| {
                nodes.iter()
                    .find(|n| n.id == **node_id)
                    .map(|n| n.centrality_score > 0.2)
                    .unwrap_or(false)
            })
            .take(8)
            .cloned()
            .collect();
        
        if cross_training_nodes.len() >= 4 {
            pathways.push(LearningPathway {
                id: format!("pathway_{}", pathways.len()),
                name: "Cross-Domain Expertise".to_string(),
                nodes: cross_training_nodes,
                difficulty: "advanced".to_string(),
                estimated_time: "4-8 weeks".to_string(),
                learning_objectives: vec![
                    "Develop broad technical perspective".to_string(),
                    "Understand system interconnections".to_string(),
                    "Build cross-functional skills".to_string(),
                ],
                prerequisites: vec![
                    "Experience in at least one domain".to_string(),
                    "Strong collaboration skills".to_string(),
                ],
                pathway_type: "cross_training".to_string(),
            });
        }
    }
    
    pathways
}

async fn generate_smart_recommendations(
    nodes: &[EnhancedGraphNode],
    clusters: &[GraphCluster],
    pathways: &[LearningPathway],
) -> Vec<GraphRecommendation> {
    log_step("💡", "Generating AI-powered recommendations...");
    
    let mut recommendations = Vec::new();
    
    // Knowledge gap recommendations
    let isolated_nodes: Vec<&EnhancedGraphNode> = nodes.iter()
        .filter(|n| n.centrality_score < 0.1 && n.learning_value > 5.0)
        .collect();
    
    if !isolated_nodes.is_empty() {
        recommendations.push(GraphRecommendation {
            id: format!("rec_{}", recommendations.len()),
            recommendation_type: "knowledge_gap".to_string(),
            title: "High-Value Isolated Knowledge Detected".to_string(),
            description: format!(
                "Found {} high-value nodes with low connectivity. These represent untapped learning opportunities.",
                isolated_nodes.len()
            ),
            confidence: 0.85,
            impact: "high".to_string(),
            target_nodes: isolated_nodes.iter().map(|n| n.id.clone()).collect(),
            action_items: vec![
                "Create documentation for isolated high-value issues".to_string(),
                "Link related issues to build knowledge networks".to_string(),
                "Schedule knowledge sharing sessions".to_string(),
            ],
            reasoning: "High learning value with low connectivity indicates untapped knowledge".to_string(),
        });
    }
    
    // Collaboration opportunity recommendations
    for cluster in clusters {
        if cluster.cohesion_score < 50.0 && cluster.nodes.len() > 5 {
            recommendations.push(GraphRecommendation {
                id: format!("rec_{}", recommendations.len()),
                recommendation_type: "collaboration".to_string(),
                title: format!("Improve {} Collaboration", cluster.name),
                description: format!(
                    "Cluster has low cohesion ({:.1}%) despite {} nodes. Collaboration could be enhanced.",
                    cluster.cohesion_score, cluster.nodes.len()
                ),
                confidence: 0.75,
                impact: "medium".to_string(),
                target_nodes: cluster.nodes.clone(),
                action_items: vec![
                    "Organize cross-team collaboration sessions".to_string(),
                    "Create shared documentation spaces".to_string(),
                    "Implement pair programming across teams".to_string(),
                ],
                reasoning: "Low cohesion in large clusters indicates collaboration opportunities".to_string(),
            });
        }
    }
    
    // Learning pathway recommendations
    for pathway in pathways {
        if pathway.pathway_type == "project_mastery" {
            recommendations.push(GraphRecommendation {
                id: format!("rec_{}", recommendations.len()),
                recommendation_type: "learning_pathway".to_string(),
                title: format!("Follow {} Learning Path", pathway.name),
                description: format!(
                    "Structured learning pathway with {} steps for {} proficiency.",
                    pathway.nodes.len(), pathway.difficulty
                ),
                confidence: 0.90,
                impact: "high".to_string(),
                target_nodes: pathway.nodes.clone(),
                action_items: vec![
                    format!("Complete {} learning objectives", pathway.learning_objectives.len()),
                    format!("Allocate {} for pathway completion", pathway.estimated_time),
                    "Track progress through pathway milestones".to_string(),
                ],
                reasoning: "Structured learning pathways accelerate skill development".to_string(),
            });
        }
    }
    
    recommendations
}

async fn calculate_graph_analytics(
    nodes: &[EnhancedGraphNode],
    edges: &[EnhancedGraphEdge],
    clusters: &[GraphCluster],
) -> GraphAnalytics {
    log_step("📊", "Calculating advanced graph analytics...");
    
    let total_possible_edges = nodes.len() * (nodes.len() - 1) / 2;
    let network_density = if total_possible_edges > 0 {
        edges.len() as f64 / total_possible_edges as f64
    } else {
        0.0
    };
    
    let average_clustering = nodes.iter()
        .map(|n| n.clustering_coefficient)
        .sum::<f64>() / nodes.len().max(1) as f64;
    
    // Calculate diameter (longest shortest path) - simplified version
    let diameter = estimate_graph_diameter(nodes, edges);
    
    // Modularity based on clustering
    let modularity = calculate_modularity(clusters, nodes.len(), edges.len());
    
    // Knowledge flow score
    let knowledge_flow_score = edges.iter()
        .filter(|e| e.learning_pathway)
        .map(|e| e.strength)
        .sum::<f64>() / edges.len().max(1) as f64 * 10.0;
    
    // Collaboration index
    let collaboration_index = (average_clustering * network_density * 100.0).min(100.0);
    
    // Expertise distribution
    let mut expertise_distribution = HashMap::new();
    for node in nodes {
        let expertise_category = match node.node_type {
            NodeType::Project => "Project Management",
            NodeType::Issue => match node.expertise_level {
                level if level > 7.0 => "Expert",
                level if level > 4.0 => "Intermediate", 
                _ => "Beginner",
            },
            _ => "Other",
        };
        *expertise_distribution.entry(expertise_category.to_string()).or_insert(0.0) += 1.0;
    }
    
    // Normalize expertise distribution to percentages
    let total_nodes = nodes.len() as f64;
    for value in expertise_distribution.values_mut() {
        *value = (*value / total_nodes) * 100.0;
    }
    
    // Critical connectors (high centrality nodes)
    let critical_connectors = nodes.iter()
        .filter(|n| n.centrality_score > 0.3)
        .map(|n| n.id.clone())
        .collect();
    
    // Knowledge gaps (isolated high-value nodes)
    let knowledge_gaps = nodes.iter()
        .filter(|n| n.centrality_score < 0.1 && n.learning_value > 6.0)
        .map(|n| format!("Gap: {}", n.label.chars().take(30).collect::<String>()))
        .collect();
    
    GraphAnalytics {
        network_density,
        average_clustering,
        diameter,
        modularity,
        knowledge_flow_score,
        collaboration_index,
        expertise_distribution,
        critical_connectors,
        knowledge_gaps,
    }
}

fn calculate_clustering_coefficient(node_id: &str, adjacency: &HashMap<String, Vec<String>>) -> f64 {
    let neighbors = adjacency.get(node_id).map(|v| v.as_slice()).unwrap_or(&[]);
    if neighbors.len() < 2 {
        return 0.0;
    }
    
    let mut edges_between_neighbors = 0;
    for i in 0..neighbors.len() {
        for j in (i + 1)..neighbors.len() {
            if let Some(neighbor_connections) = adjacency.get(&neighbors[i]) {
                if neighbor_connections.contains(&neighbors[j]) {
                    edges_between_neighbors += 1;
                }
            }
        }
    }
    
    let possible_edges = neighbors.len() * (neighbors.len() - 1) / 2;
    edges_between_neighbors as f64 / possible_edges as f64
}

fn estimate_graph_diameter(nodes: &[EnhancedGraphNode], edges: &[EnhancedGraphEdge]) -> usize {
    // Simplified diameter calculation using BFS from a central node
    if nodes.is_empty() {
        return 0;
    }
    
    // Build adjacency list
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    for edge in edges {
        adjacency.entry(edge.source.clone()).or_default().push(edge.target.clone());
        adjacency.entry(edge.target.clone()).or_default().push(edge.source.clone());
    }
    
    // Find most central node
    let central_node = nodes.iter()
        .max_by(|a, b| a.centrality_score.partial_cmp(&b.centrality_score).unwrap())
        .map(|n| &n.id)
        .unwrap_or(&nodes[0].id);
    
    // BFS to find maximum distance
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut max_distance = 0;
    
    queue.push_back((central_node.clone(), 0));
    visited.insert(central_node.clone());
    
    while let Some((node, distance)) = queue.pop_front() {
        max_distance = max_distance.max(distance);
        
        if let Some(neighbors) = adjacency.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back((neighbor.clone(), distance + 1));
                }
            }
        }
    }
    
    max_distance
}

fn calculate_modularity(clusters: &[GraphCluster], total_nodes: usize, total_edges: usize) -> f64 {
    if clusters.is_empty() || total_edges == 0 {
        return 0.0;
    }
    
    // Simplified modularity calculation
    let cluster_sizes: Vec<f64> = clusters.iter()
        .map(|c| c.nodes.len() as f64)
        .collect();
    
    let expected_random = cluster_sizes.iter()
        .map(|size| (size / total_nodes as f64).powi(2))
        .sum::<f64>();
    
    let actual_internal = cluster_sizes.iter()
        .map(|size| (size / total_nodes as f64) * 0.8) // Assume 80% internal connectivity
        .sum::<f64>();
    
    (actual_internal - expected_random).max(0.0).min(1.0)
}