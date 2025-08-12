use crate::db_utils::with_connection;
use crate::utils::{log_step, log_success};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamVelocity {
    pub period: String,
    pub issues_created: i32,
    pub issues_resolved: i32,
    pub velocity_score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusFlowAnalysis {
    pub status: String,
    pub count: i32,
    pub percentage: f64,
    pub avg_time_in_status: Option<f64>, // days
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectProductivity {
    pub project_key: String,
    pub project_name: String,
    pub total_issues: i32,
    pub resolved_issues: i32,
    pub resolution_rate: f64,
    pub avg_resolution_time: Option<f64>, // days
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrendAnalysis {
    pub period: String,
    pub metric: String,
    pub value: f64,
    pub change_from_previous: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalyticsDashboard {
    pub team_velocity: Vec<TeamVelocity>,
    pub status_distribution: Vec<StatusFlowAnalysis>,
    pub project_productivity: Vec<ProjectProductivity>,
    pub trends: Vec<TrendAnalysis>,
    pub summary_stats: DashboardSummary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardSummary {
    pub total_issues: i32,
    pub total_projects: i32,
    pub resolution_rate: f64,
    pub avg_resolution_time: Option<f64>,
    pub most_productive_period: String,
    pub generated_at: String,
}

pub async fn generate_analytics_dashboard() -> AnalyticsDashboard {
    log_step("📊", "Generating analytics dashboard...");

    let team_velocity = calculate_team_velocity().await;
    let status_distribution = analyze_status_flow().await;
    let project_productivity = calculate_project_productivity().await;
    let trends = analyze_trends().await;
    let summary_stats = generate_summary_stats().await;

    log_success("Analytics dashboard generated successfully");

    AnalyticsDashboard {
        team_velocity,
        status_distribution,
        project_productivity,
        trends,
        summary_stats,
    }
}

async fn calculate_team_velocity() -> Vec<TeamVelocity> {
    log_step("⚡", "Calculating team velocity metrics...");
    
    let mut velocity_data = Vec::new();

    with_connection("calculate_velocity", |conn| {
        // Get velocity data by month for the last 6 months
        let query = r#"
        WITH monthly_stats AS (
            SELECT 
                strftime('%Y-%m', created) as month,
                COUNT(*) as created_count,
                COUNT(CASE WHEN status IN ('Done', 'Closed', 'Resolved') THEN 1 END) as resolved_count
            FROM issues 
            WHERE created IS NOT NULL 
                AND created != '' 
                AND datetime(created) >= datetime('now', '-6 months')
            GROUP BY strftime('%Y-%m', created)
            ORDER BY month DESC
        )
        SELECT 
            month,
            created_count,
            resolved_count,
            CAST(resolved_count AS REAL) / CAST(created_count AS REAL) * 100 as velocity_score
        FROM monthly_stats
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare velocity query");
        let rows = stmt.query_map([], |row| {
            Ok(TeamVelocity {
                period: row.get(0)?,
                issues_created: row.get(1)?,
                issues_resolved: row.get(2)?,
                velocity_score: row.get(3)?,
            })
        }).expect("Failed to execute velocity query");

        for row in rows {
            if let Ok(velocity) = row {
                velocity_data.push(velocity);
            }
        }
    });

    velocity_data
}

async fn analyze_status_flow() -> Vec<StatusFlowAnalysis> {
    log_step("🔄", "Analyzing status flow patterns...");
    
    let mut status_data = Vec::new();
    
    with_connection("analyze_status", |conn| {
        // Get status distribution
        let query = r#"
        WITH status_counts AS (
            SELECT 
                COALESCE(status, 'Unknown') as status,
                COUNT(*) as count
            FROM issues 
            GROUP BY status
        ),
        total_issues AS (
            SELECT COUNT(*) as total FROM issues
        )
        SELECT 
            sc.status,
            sc.count,
            CAST(sc.count AS REAL) / CAST(ti.total AS REAL) * 100 as percentage
        FROM status_counts sc
        CROSS JOIN total_issues ti
        ORDER BY sc.count DESC
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare status query");
        let rows = stmt.query_map([], |row| {
            Ok(StatusFlowAnalysis {
                status: row.get(0)?,
                count: row.get(1)?,
                percentage: row.get(2)?,
                avg_time_in_status: None, // TODO: Calculate based on changelog data
            })
        }).expect("Failed to execute status query");

        for row in rows {
            if let Ok(status_flow) = row {
                status_data.push(status_flow);
            }
        }
    });

    status_data
}

async fn calculate_project_productivity() -> Vec<ProjectProductivity> {
    log_step("🎯", "Calculating project productivity metrics...");
    
    let mut productivity_data = Vec::new();
    
    with_connection("calculate_productivity", |conn| {
        let query = r#"
        SELECT 
            p.key as project_key,
            p.name as project_name,
            COUNT(i.id) as total_issues,
            COUNT(CASE WHEN i.status IN ('Done', 'Closed', 'Resolved') THEN 1 END) as resolved_issues,
            CASE 
                WHEN COUNT(i.id) > 0 THEN 
                    CAST(COUNT(CASE WHEN i.status IN ('Done', 'Closed', 'Resolved') THEN 1 END) AS REAL) / CAST(COUNT(i.id) AS REAL) * 100
                ELSE 0 
            END as resolution_rate
        FROM projects p
        LEFT JOIN issues i ON JSON_EXTRACT(i.project, '$.key') = p.key
        GROUP BY p.id, p.key, p.name
        HAVING COUNT(i.id) > 0
        ORDER BY resolution_rate DESC, total_issues DESC
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare productivity query");
        let rows = stmt.query_map([], |row| {
            Ok(ProjectProductivity {
                project_key: row.get(0)?,
                project_name: row.get(1)?,
                total_issues: row.get(2)?,
                resolved_issues: row.get(3)?,
                resolution_rate: row.get(4)?,
                avg_resolution_time: None, // TODO: Calculate based on created/updated dates
            })
        }).expect("Failed to execute productivity query");

        for row in rows {
            if let Ok(productivity) = row {
                productivity_data.push(productivity);
            }
        }
    });

    productivity_data
}

async fn analyze_trends() -> Vec<TrendAnalysis> {
    log_step("📈", "Analyzing trends over time...");
    
    let mut trends = Vec::new();
    
    with_connection("analyze_trends", |conn| {
        // Weekly issue creation trend
        let query = r#"
        WITH weekly_creation AS (
            SELECT 
                strftime('%Y-W%W', created) as week,
                COUNT(*) as issues_created
            FROM issues 
            WHERE created IS NOT NULL 
                AND created != '' 
                AND datetime(created) >= datetime('now', '-12 weeks')
            GROUP BY strftime('%Y-W%W', created)
            ORDER BY week
        )
        SELECT 
            week,
            'issues_created' as metric,
            CAST(issues_created AS REAL) as value
        FROM weekly_creation
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare trends query");
        let rows = stmt.query_map([], |row| {
            Ok(TrendAnalysis {
                period: row.get(0)?,
                metric: row.get(1)?,
                value: row.get(2)?,
                change_from_previous: None, // TODO: Calculate percentage change
            })
        }).expect("Failed to execute trends query");

        for row in rows {
            if let Ok(trend) = row {
                trends.push(trend);
            }
        }
    });

    trends
}

async fn generate_summary_stats() -> DashboardSummary {
    log_step("📋", "Generating summary statistics...");
    
    let mut summary = DashboardSummary {
        total_issues: 0,
        total_projects: 0,
        resolution_rate: 0.0,
        avg_resolution_time: None,
        most_productive_period: "N/A".to_string(),
        generated_at: Utc::now().to_rfc3339(),
    };
    
    with_connection("generate_summary", |conn| {
        // Get total counts and resolution rate
        let query = r#"
        SELECT 
            (SELECT COUNT(*) FROM issues) as total_issues,
            (SELECT COUNT(*) FROM projects) as total_projects,
            CASE 
                WHEN (SELECT COUNT(*) FROM issues) > 0 THEN
                    CAST((SELECT COUNT(*) FROM issues WHERE status IN ('Done', 'Closed', 'Resolved')) AS REAL) / 
                    CAST((SELECT COUNT(*) FROM issues) AS REAL) * 100
                ELSE 0 
            END as resolution_rate
        "#;

        let mut stmt = conn.prepare(query).expect("Failed to prepare summary query");
        let mut rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, f64>(2)?,
            ))
        }).expect("Failed to execute summary query");

        if let Some(Ok((total_issues, total_projects, resolution_rate))) = rows.next() {
            summary.total_issues = total_issues;
            summary.total_projects = total_projects;
            summary.resolution_rate = resolution_rate;
        }

        // Find most productive period (month with highest resolution rate)
        let productivity_query = r#"
        WITH monthly_productivity AS (
            SELECT 
                strftime('%Y-%m', created) as month,
                COUNT(*) as total,
                COUNT(CASE WHEN status IN ('Done', 'Closed', 'Resolved') THEN 1 END) as resolved,
                CASE 
                    WHEN COUNT(*) > 0 THEN
                        CAST(COUNT(CASE WHEN status IN ('Done', 'Closed', 'Resolved') THEN 1 END) AS REAL) / CAST(COUNT(*) AS REAL) * 100
                    ELSE 0 
                END as rate
            FROM issues 
            WHERE created IS NOT NULL AND created != ''
            GROUP BY strftime('%Y-%m', created)
            HAVING total >= 3  -- Only consider months with at least 3 issues
        )
        SELECT month
        FROM monthly_productivity 
        ORDER BY rate DESC, resolved DESC 
        LIMIT 1
        "#;

        let mut prod_stmt = conn.prepare(productivity_query).expect("Failed to prepare productivity query");
        let mut prod_rows = prod_stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        }).expect("Failed to execute productivity query");

        if let Some(Ok(most_productive)) = prod_rows.next() {
            summary.most_productive_period = most_productive;
        }
    });

    summary
}

pub async fn get_time_series_data(metric: &str, period: &str) -> Vec<TrendAnalysis> {
    log_step("📊", &format!("Getting time series data for {} by {}", metric, period));
    
    let mut data = Vec::new();
    
    with_connection("get_time_series", |conn| {
        let query = match (metric, period) {
            ("issues_created", "week") => r#"
                SELECT 
                    strftime('%Y-W%W', created) as period,
                    'issues_created' as metric,
                    CAST(COUNT(*) AS REAL) as value
                FROM issues 
                WHERE created IS NOT NULL 
                    AND created != '' 
                    AND datetime(created) >= datetime('now', '-12 weeks')
                GROUP BY strftime('%Y-W%W', created)
                ORDER BY period
            "#,
            ("issues_resolved", "week") => r#"
                SELECT 
                    strftime('%Y-W%W', updated) as period,
                    'issues_resolved' as metric,
                    CAST(COUNT(*) AS REAL) as value
                FROM issues 
                WHERE updated IS NOT NULL 
                    AND updated != '' 
                    AND status IN ('Done', 'Closed', 'Resolved')
                    AND datetime(updated) >= datetime('now', '-12 weeks')
                GROUP BY strftime('%Y-W%W', updated)
                ORDER BY period
            "#,
            ("velocity", "month") => r#"
                WITH monthly_velocity AS (
                    SELECT 
                        strftime('%Y-%m', created) as period,
                        COUNT(*) as created,
                        COUNT(CASE WHEN status IN ('Done', 'Closed', 'Resolved') THEN 1 END) as resolved
                    FROM issues 
                    WHERE created IS NOT NULL 
                        AND created != '' 
                        AND datetime(created) >= datetime('now', '-6 months')
                    GROUP BY strftime('%Y-%m', created)
                )
                SELECT 
                    period,
                    'velocity' as metric,
                    CASE WHEN created > 0 THEN CAST(resolved AS REAL) / CAST(created AS REAL) * 100 ELSE 0 END as value
                FROM monthly_velocity
                ORDER BY period
            "#,
            _ => return data, // Unknown metric/period combination
        };

        let mut stmt = conn.prepare(query).expect("Failed to prepare time series query");
        let rows = stmt.query_map([], |row| {
            Ok(TrendAnalysis {
                period: row.get(0)?,
                metric: row.get(1)?,
                value: row.get(2)?,
                change_from_previous: None,
            })
        }).expect("Failed to execute time series query");

        for row in rows {
            if let Ok(trend) = row {
                data.push(trend);
            }
        }
    });

    // Calculate percentage change from previous period
    for i in 1..data.len() {
        let current = data[i].value;
        let previous = data[i-1].value;
        if previous != 0.0 {
            data[i].change_from_previous = Some((current - previous) / previous * 100.0);
        }
    }

    data
}