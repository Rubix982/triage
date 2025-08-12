import React, { useEffect, useState } from 'react'

interface SyncStatus {
  last_sync_time: string
  sync_statistics: {
    total_projects: number
    total_issues: number
    new_issues_since_last_sync: number
    updated_issues_since_last_sync: number
    escl_count: number
    issues_with_comments: number
    average_comments_per_issue: number
    sync_duration_seconds: number
  }
  recent_issues: Array<{
    id: string
    key: string
    summary: string
    status: string
    created: string
    project_name: string
    is_escl: boolean
    priority: string
    has_comments: boolean
    comment_count: number
    description_preview: string
  }>
  updated_issues: Array<{
    id: string
    key: string
    summary: string
    status: string
    updated: string
    project_name: string
    is_escl: boolean
    update_type: string
    changes_summary: string
    new_comment_count: number
  }>
  comment_rich_issues: Array<{
    id: string
    key: string
    summary: string
    status: string
    project_name: string
    is_escl: boolean
    comment_count: number
    comment_quality_score: number
    has_solution_indicators: boolean
    solution_keywords: string[]
    last_comment_date: string
    participant_count: number
  }>
  escl_insights: {
    total_escls: number
    new_escls: number
    resolved_escls: number
    escls_with_rich_comments: number
    top_escl_categories: Array<{
      category: string
      count: number
      avg_comments: number
      resolution_rate: number
    }>
    resolution_time_avg_days: number
    comment_engagement_score: number
  }
  knowledge_impact: {
    new_concepts_discovered: number
    new_technologies_identified: number
    new_solution_patterns: number
    knowledge_value_score: number
    recommended_actions: string[]
  }
}

export default function SyncStatusDashboard() {
  const [syncStatus, setSyncStatus] = useState<SyncStatus | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<'recent' | 'updated' | 'comments' | 'escls' | 'insights'>('recent')

  useEffect(() => {
    const fetchSyncStatus = async () => {
      try {
        setLoading(true)
        const response = await fetch('http://127.0.0.1:3001/api/sync/status')
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        const data: SyncStatus = await response.json()
        setSyncStatus(data)
        setError(null)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch sync status')
        console.error('Failed to fetch sync status:', err)
      } finally {
        setLoading(false)
      }
    }

    fetchSyncStatus()
    
    // Refresh every 5 minutes
    const interval = setInterval(fetchSyncStatus, 5 * 60 * 1000)
    return () => clearInterval(interval)
  }, [])

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString() + ' ' + new Date(dateString).toLocaleTimeString()
  }

  const getUpdateTypeIcon = (type: string) => {
    const icons: Record<string, string> = {
      'StatusChange': '🔄',
      'NewComments': '💬',
      'DescriptionUpdate': '📝',
      'Resolution': '✅',
      'Assignment': '👤',
      'Multiple': '🔀'
    }
    return icons[type] || '📄'
  }

  const getUpdateTypeColor = (type: string) => {
    const colors: Record<string, string> = {
      'StatusChange': 'text-blue-400',
      'NewComments': 'text-green-400',
      'DescriptionUpdate': 'text-yellow-400',
      'Resolution': 'text-green-500',
      'Assignment': 'text-purple-400',
      'Multiple': 'text-orange-400'
    }
    return colors[type] || 'text-gray-400'
  }

  const getPriorityColor = (priority: string) => {
    const colors: Record<string, string> = {
      'High': 'text-red-400',
      'Medium': 'text-yellow-400',
      'Low': 'text-green-400'
    }
    return colors[priority] || 'text-gray-400'
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-white">📊 Loading sync status...</div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-red-400">Error: {error}</div>
      </div>
    )
  }

  if (!syncStatus) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-gray-400">No sync status data available</div>
      </div>
    )
  }

  return (
    <div className="w-full space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold text-white">📊 Sync Status Dashboard</h2>
          <p className="text-gray-300 mt-1">
            Track recent ESCLs, updates, and knowledge discoveries from Jira sync
          </p>
        </div>
        <div className="text-right">
          <div className="text-sm text-gray-400">Last Sync:</div>
          <div className="text-lg font-medium text-blue-400">
            {formatDate(syncStatus.last_sync_time)}
          </div>
        </div>
      </div>

      {/* Sync Statistics Overview */}
      <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-7 gap-4">
        <div className="bg-blue-900/20 p-4 rounded-lg border border-blue-700">
          <div className="text-2xl font-bold text-blue-400">{syncStatus.sync_statistics.total_issues}</div>
          <div className="text-sm text-blue-300">Total Issues</div>
        </div>
        <div className="bg-green-900/20 p-4 rounded-lg border border-green-700">
          <div className="text-2xl font-bold text-green-400">{syncStatus.sync_statistics.new_issues_since_last_sync}</div>
          <div className="text-sm text-green-300">New Issues</div>
        </div>
        <div className="bg-purple-900/20 p-4 rounded-lg border border-purple-700">
          <div className="text-2xl font-bold text-purple-400">{syncStatus.sync_statistics.updated_issues_since_last_sync}</div>
          <div className="text-sm text-purple-300">Updated Issues</div>
        </div>
        <div className="bg-red-900/20 p-4 rounded-lg border border-red-700">
          <div className="text-2xl font-bold text-red-400">{syncStatus.sync_statistics.escl_count}</div>
          <div className="text-sm text-red-300">Total ESCLs</div>
        </div>
        <div className="bg-yellow-900/20 p-4 rounded-lg border border-yellow-700">
          <div className="text-2xl font-bold text-yellow-400">{syncStatus.sync_statistics.issues_with_comments}</div>
          <div className="text-sm text-yellow-300">With Comments</div>
        </div>
        <div className="bg-indigo-900/20 p-4 rounded-lg border border-indigo-700">
          <div className="text-2xl font-bold text-indigo-400">{syncStatus.sync_statistics.average_comments_per_issue.toFixed(1)}</div>
          <div className="text-sm text-indigo-300">Avg Comments</div>
        </div>
        <div className="bg-gray-900/20 p-4 rounded-lg border border-gray-700">
          <div className="text-2xl font-bold text-gray-400">{syncStatus.sync_statistics.total_projects}</div>
          <div className="text-sm text-gray-300">Projects</div>
        </div>
      </div>

      {/* ESCL Insights */}
      <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
        <h3 className="text-xl font-semibold text-white mb-4">🎫 ESCL Insights</h3>
        <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-green-400">{syncStatus.escl_insights.new_escls}</div>
            <div className="text-sm text-gray-400">New ESCLs</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-400">{syncStatus.escl_insights.resolved_escls}</div>
            <div className="text-sm text-gray-400">Resolved</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-400">{syncStatus.escl_insights.escls_with_rich_comments}</div>
            <div className="text-sm text-gray-400">Comment Rich</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-yellow-400">{(syncStatus.escl_insights.comment_engagement_score * 100).toFixed(0)}%</div>
            <div className="text-sm text-gray-400">Engagement</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-orange-400">{syncStatus.escl_insights.resolution_time_avg_days.toFixed(1)}</div>
            <div className="text-sm text-gray-400">Avg Days</div>
          </div>
        </div>

        {syncStatus.escl_insights.top_escl_categories.length > 0 && (
          <div className="mt-6">
            <h4 className="text-lg font-medium text-white mb-3">Top ESCL Categories</h4>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {syncStatus.escl_insights.top_escl_categories.map((category, idx) => (
                <div key={idx} className="bg-gray-900/50 p-3 rounded">
                  <div className="flex items-center justify-between mb-2">
                    <span className="font-medium text-blue-400">{category.category}</span>
                    <span className="text-sm text-gray-400">{category.count} issues</span>
                  </div>
                  <div className="flex items-center justify-between text-sm text-gray-300">
                    <span>Avg Comments: {category.avg_comments.toFixed(1)}</span>
                    <span className="text-green-400">{(category.resolution_rate * 100).toFixed(0)}% resolved</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Knowledge Impact */}
      <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
        <h3 className="text-xl font-semibold text-white mb-4">🧠 Knowledge Impact</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-400">{syncStatus.knowledge_impact.new_concepts_discovered}</div>
            <div className="text-sm text-gray-400">New Concepts</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-400">{syncStatus.knowledge_impact.new_technologies_identified}</div>
            <div className="text-sm text-gray-400">Technologies</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-400">{syncStatus.knowledge_impact.new_solution_patterns}</div>
            <div className="text-sm text-gray-400">Solution Patterns</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-yellow-400">{syncStatus.knowledge_impact.knowledge_value_score.toFixed(1)}/10</div>
            <div className="text-sm text-gray-400">Value Score</div>
          </div>
        </div>

        {syncStatus.knowledge_impact.recommended_actions.length > 0 && (
          <div>
            <h4 className="text-lg font-medium text-white mb-3">🎯 Recommended Actions</h4>
            <div className="space-y-2">
              {syncStatus.knowledge_impact.recommended_actions.map((action, idx) => (
                <div key={idx} className="flex items-start space-x-3 bg-gray-900/50 p-3 rounded">
                  <span className="text-green-400 mt-1">•</span>
                  <span className="text-gray-300">{action}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Navigation Tabs */}
      <div className="border-b border-gray-700">
        <nav className="flex space-x-1">
          {[
            { id: 'recent', label: '🆕 Recent Issues', count: syncStatus.recent_issues.length },
            { id: 'updated', label: '🔄 Updated Issues', count: syncStatus.updated_issues.length },
            { id: 'comments', label: '💬 Comment Rich', count: syncStatus.comment_rich_issues.length },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === tab.id
                  ? 'text-blue-400 border-blue-400'
                  : 'text-gray-400 border-transparent hover:text-gray-300 hover:border-gray-300'
              }`}
            >
              {tab.label} ({tab.count})
            </button>
          ))}
        </nav>
      </div>

      {/* Tab Content */}
      <div className="space-y-4">
        {activeTab === 'recent' && (
          <div className="space-y-4">
            <h3 className="text-lg font-semibold text-white">🆕 Recently Created Issues</h3>
            {syncStatus.recent_issues.map((issue) => (
              <div key={issue.id} className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-start space-x-3">
                    <div>
                      <div className="flex items-center space-x-3 mb-2">
                        <span className={`font-medium ${issue.is_escl ? 'text-red-400' : 'text-blue-400'}`}>
                          {issue.is_escl ? '🎫' : '📋'} {issue.key}
                        </span>
                        <span className={`text-xs px-2 py-1 rounded ${getPriorityColor(issue.priority)} bg-gray-900/50`}>
                          {issue.priority}
                        </span>
                        <span className="text-xs text-gray-400">{issue.status}</span>
                      </div>
                      <h4 className="font-medium text-white mb-2">{issue.summary}</h4>
                      <p className="text-sm text-gray-300 mb-2">{issue.description_preview}</p>
                    </div>
                  </div>
                  <div className="text-right ml-4">
                    <div className="text-sm text-gray-400">{formatDate(issue.created)}</div>
                    {issue.has_comments && (
                      <div className="text-sm text-green-400 mt-1">
                        💬 {issue.comment_count} comments
                      </div>
                    )}
                  </div>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">Project: {issue.project_name}</span>
                  {issue.is_escl && <span className="text-red-300 font-medium">ESCL</span>}
                </div>
              </div>
            ))}
          </div>
        )}

        {activeTab === 'updated' && (
          <div className="space-y-4">
            <h3 className="text-lg font-semibold text-white">🔄 Recently Updated Issues</h3>
            {syncStatus.updated_issues.map((issue) => (
              <div key={issue.id} className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex items-start space-x-3">
                    <div className="flex items-center space-x-2 mt-1">
                      <span className="text-lg">{getUpdateTypeIcon(issue.update_type)}</span>
                    </div>
                    <div>
                      <div className="flex items-center space-x-3 mb-2">
                        <span className={`font-medium ${issue.is_escl ? 'text-red-400' : 'text-blue-400'}`}>
                          {issue.is_escl ? '🎫' : '📋'} {issue.key}
                        </span>
                        <span className={`text-xs px-2 py-1 rounded ${getUpdateTypeColor(issue.update_type)} bg-gray-900/50`}>
                          {issue.update_type}
                        </span>
                        <span className="text-xs text-gray-400">{issue.status}</span>
                      </div>
                      <h4 className="font-medium text-white mb-2">{issue.summary}</h4>
                      <p className="text-sm text-gray-300">{issue.changes_summary}</p>
                    </div>
                  </div>
                  <div className="text-right ml-4">
                    <div className="text-sm text-gray-400">{formatDate(issue.updated)}</div>
                    {issue.new_comment_count > 0 && (
                      <div className="text-sm text-green-400 mt-1">
                        💬 +{issue.new_comment_count} comments
                      </div>
                    )}
                  </div>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">Project: {issue.project_name}</span>
                  {issue.is_escl && <span className="text-red-300 font-medium">ESCL</span>}
                </div>
              </div>
            ))}
          </div>
        )}

        {activeTab === 'comments' && (
          <div className="space-y-4">
            <h3 className="text-lg font-semibold text-white">💬 Issues with Rich Comments</h3>
            {syncStatus.comment_rich_issues.map((issue) => (
              <div key={issue.id} className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-3">
                  <div>
                    <div className="flex items-center space-x-3 mb-2">
                      <span className={`font-medium ${issue.is_escl ? 'text-red-400' : 'text-blue-400'}`}>
                        {issue.is_escl ? '🎫' : '📋'} {issue.key}
                      </span>
                      <span className="text-xs text-gray-400">{issue.status}</span>
                      {issue.has_solution_indicators && (
                        <span className="text-xs px-2 py-1 rounded bg-green-900/50 text-green-300">
                          ✅ Has Solutions
                        </span>
                      )}
                    </div>
                    <h4 className="font-medium text-white mb-3">{issue.summary}</h4>
                    
                    {issue.solution_keywords.length > 0 && (
                      <div className="mb-3">
                        <span className="text-sm text-gray-400 mb-2 block">Solution Keywords:</span>
                        <div className="flex flex-wrap gap-1">
                          {issue.solution_keywords.map((keyword, idx) => (
                            <span key={idx} className="text-xs bg-green-900/30 text-green-300 px-2 py-1 rounded">
                              {keyword}
                            </span>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                  <div className="text-right ml-4">
                    <div className="text-lg font-bold text-green-400">{issue.comment_count}</div>
                    <div className="text-xs text-gray-400">comments</div>
                    <div className="text-sm text-purple-400 mt-1">
                      Quality: {issue.comment_quality_score.toFixed(1)}/10
                    </div>
                    <div className="text-xs text-gray-400 mt-1">
                      {issue.participant_count} participants
                    </div>
                  </div>
                </div>
                
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">Project: {issue.project_name}</span>
                  <span className="text-gray-400">Last: {formatDate(issue.last_comment_date)}</span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}