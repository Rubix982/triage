import React, { useEffect, useState } from 'react'
import SmartKnowledgeGraph from './SmartKnowledgeGraph'
import AIInsightsPanel from './AIInsightsPanel'
import PredictiveAnalytics from './PredictiveAnalytics'
import PerformanceScoreCard from './PerformanceScoreCard'

interface AdvancedMetrics {
  velocity_insights: {
    current_velocity: number
    velocity_trend: string
    velocity_consistency: number
    peak_performance_factors: string[]
    seasonal_patterns: Array<{
      period: string
      performance_multiplier: number
      confidence: number
    }>
    capacity_utilization: number
  }
  bottleneck_analysis: {
    critical_bottlenecks: Array<{
      stage: string
      severity: number
      avg_wait_time: number
      impact_score: number
      suggested_actions: string[]
    }>
    flow_efficiency: number
    wait_time_analysis: Record<string, number>
    throughput_analysis: {
      daily_avg: number
      weekly_avg: number
      peak_throughput: number
      throughput_variance: number
      efficiency_opportunities: string[]
    }
    recommendations: string[]
  }
  predictive_forecasts: Array<{
    metric: string
    period: string
    predicted_value: number
    confidence_interval: [number, number]
    trend_strength: number
    key_factors: string[]
  }>
  team_dynamics: {
    collaboration_score: number
    knowledge_distribution: Record<string, number>
    bus_factor_risk: number
    cross_training_opportunities: string[]
    expertise_gaps: string[]
  }
  quality_metrics: {
    defect_rate: number
    rework_percentage: number
    first_time_right: number
    quality_trend: string
    quality_predictors: Array<{
      factor: string
      correlation: number
      impact: string
    }>
  }
  ai_insights: Array<{
    category: string
    insight: string
    confidence: number
    impact: string
    action_items: string[]
    data_points: string[]
  }>
  performance_score: number
}

export default function AdvancedDashboard() {
  const [data, setData] = useState<AdvancedMetrics | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [activeView, setActiveView] = useState<'overview' | 'insights' | 'predictions' | 'graph'>('overview')

  // Fetch advanced analytics data
  useEffect(() => {
    const fetchAdvancedData = async () => {
      try {
        setLoading(true)
        const response = await fetch('http://127.0.0.1:3001/api/analytics/advanced')
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        const advancedData: AdvancedMetrics = await response.json()
        setData(advancedData)
        setError(null)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch advanced analytics')
        console.error('Failed to fetch advanced analytics:', err)
      } finally {
        setLoading(false)
      }
    }

    fetchAdvancedData()
  }, [])

  if (loading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-white">🧠 Loading advanced AI analytics...</div>
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

  if (!data) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-gray-400">No advanced analytics data available</div>
      </div>
    )
  }

  return (
    <div className="w-full space-y-6">
      <div className="text-white">
        <h2 className="text-2xl font-bold mb-2">🧠 AI-Powered Advanced Analytics</h2>
        <p className="text-gray-300 mb-6">
          Intelligent insights, predictive forecasts, and performance optimization powered by advanced algorithms
        </p>
        
        {/* Navigation */}
        <div className="flex space-x-1 mb-6 bg-gray-800 p-1 rounded-lg">
          {[
            { key: 'overview', label: '🎯 Performance Overview', icon: '🎯' },
            { key: 'insights', label: '🧠 AI Insights', icon: '🧠' },
            { key: 'predictions', label: '🔮 Predictions', icon: '🔮' },
            { key: 'graph', label: '🕸️ Smart Graph', icon: '🕸️' },
          ].map(({ key, label, icon }) => (
            <button
              key={key}
              onClick={() => setActiveView(key as any)}
              className={`px-4 py-2 text-sm font-medium rounded-lg transition-all ${
                activeView === key
                  ? 'bg-blue-600 text-white shadow-lg'
                  : 'text-gray-300 hover:text-white hover:bg-gray-700'
              }`}
            >
              {label}
            </button>
          ))}
        </div>

        {/* Performance Score Hero */}
        <PerformanceScoreCard 
          score={data.performance_score}
          trend={data.velocity_insights.velocity_trend}
          className="mb-6"
        />

        {/* Dynamic Content Based on Active View */}
        {activeView === 'overview' && (
          <div className="space-y-6">
            {/* Key Metrics Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <MetricCard
                title="Current Velocity"
                value={`${data.velocity_insights.current_velocity.toFixed(1)}%`}
                trend={data.velocity_insights.velocity_trend}
                icon="⚡"
                color="blue"
              />
              <MetricCard
                title="Flow Efficiency"
                value={`${data.bottleneck_analysis.flow_efficiency.toFixed(1)}%`}
                trend={data.bottleneck_analysis.flow_efficiency > 60 ? 'good' : 'needs_improvement'}
                icon="🌊"
                color="green"
              />
              <MetricCard
                title="Team Collaboration"
                value={`${data.team_dynamics.collaboration_score.toFixed(1)}%`}
                trend={data.team_dynamics.collaboration_score > 70 ? 'good' : 'needs_improvement'}
                icon="👥"
                color="purple"
              />
              <MetricCard
                title="Quality Score"
                value={`${data.quality_metrics.first_time_right.toFixed(1)}%`}
                trend={data.quality_metrics.quality_trend}
                icon="🎯"
                color="yellow"
              />
            </div>

            {/* Bottleneck Analysis */}
            {data.bottleneck_analysis.critical_bottlenecks.length > 0 && (
              <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                <h3 className="text-xl font-bold mb-4 text-red-400">🚨 Critical Bottlenecks Detected</h3>
                <div className="space-y-4">
                  {data.bottleneck_analysis.critical_bottlenecks.map((bottleneck, index) => (
                    <div key={index} className="bg-red-900/20 p-4 rounded-lg border border-red-700/50">
                      <div className="flex items-center justify-between mb-2">
                        <h4 className="font-semibold text-red-300">{bottleneck.stage}</h4>
                        <div className="flex items-center space-x-2">
                          <span className="text-sm text-gray-400">Severity:</span>
                          <div className="w-16 h-2 bg-gray-700 rounded-full overflow-hidden">
                            <div
                              className="h-full bg-red-500 transition-all duration-300"
                              style={{ width: `${bottleneck.severity}%` }}
                            />
                          </div>
                          <span className="text-sm font-medium text-red-400">
                            {bottleneck.severity.toFixed(0)}%
                          </span>
                        </div>
                      </div>
                      <p className="text-sm text-gray-300 mb-3">
                        Average wait time: {bottleneck.avg_wait_time.toFixed(1)} days
                        • Impact score: {bottleneck.impact_score.toFixed(1)}
                      </p>
                      <div className="space-y-1">
                        <p className="text-xs font-medium text-gray-400">Suggested Actions:</p>
                        {bottleneck.suggested_actions.slice(0, 2).map((action, actionIndex) => (
                          <p key={actionIndex} className="text-xs text-blue-300">• {action}</p>
                        ))}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Seasonal Patterns */}
            {data.velocity_insights.seasonal_patterns.length > 0 && (
              <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                <h3 className="text-xl font-bold mb-4 text-blue-400">📅 Performance Patterns</h3>
                <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
                  {data.velocity_insights.seasonal_patterns.map((pattern, index) => (
                    <div key={index} className="text-center">
                      <div className="text-sm font-medium text-gray-300">{pattern.period}</div>
                      <div className={`text-lg font-bold ${
                        pattern.performance_multiplier > 1.1 ? 'text-green-400' :
                        pattern.performance_multiplier < 0.9 ? 'text-red-400' : 'text-gray-400'
                      }`}>
                        {(pattern.performance_multiplier * 100).toFixed(0)}%
                      </div>
                      <div className="text-xs text-gray-500">
                        {(pattern.confidence * 100).toFixed(0)}% confidence
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {activeView === 'insights' && (
          <AIInsightsPanel insights={data.ai_insights} />
        )}

        {activeView === 'predictions' && (
          <PredictiveAnalytics
            forecasts={data.predictive_forecasts}
            throughput={data.bottleneck_analysis.throughput_analysis}
          />
        )}

        {activeView === 'graph' && (
          <SmartKnowledgeGraph />
        )}
      </div>
    </div>
  )
}

interface MetricCardProps {
  title: string
  value: string
  trend: string
  icon: string
  color: 'blue' | 'green' | 'purple' | 'yellow'
}

function MetricCard({ title, value, trend, icon, color }: MetricCardProps) {
  const colorClasses = {
    blue: 'border-blue-700 bg-blue-900/20',
    green: 'border-green-700 bg-green-900/20',
    purple: 'border-purple-700 bg-purple-900/20',
    yellow: 'border-yellow-700 bg-yellow-900/20',
  }

  const valueColorClasses = {
    blue: 'text-blue-400',
    green: 'text-green-400',
    purple: 'text-purple-400',
    yellow: 'text-yellow-400',
  }

  const getTrendIcon = (trend: string) => {
    switch (trend) {
      case 'accelerating':
      case 'improving':
      case 'good':
        return '↗️'
      case 'declining':
      case 'needs_improvement':
        return '↘️'
      case 'stable':
        return '➡️'
      default:
        return '❓'
    }
  }

  const getTrendColor = (trend: string) => {
    switch (trend) {
      case 'accelerating':
      case 'improving':
      case 'good':
        return 'text-green-400'
      case 'declining':
      case 'needs_improvement':
        return 'text-red-400'
      case 'stable':
        return 'text-gray-400'
      default:
        return 'text-gray-400'
    }
  }

  return (
    <div className={`p-4 rounded-lg border ${colorClasses[color]}`}>
      <div className="flex items-center justify-between mb-2">
        <span className="text-2xl">{icon}</span>
        <span className={`text-sm ${getTrendColor(trend)}`}>
          {getTrendIcon(trend)}
        </span>
      </div>
      <h4 className="text-sm font-medium text-gray-400 mb-1">{title}</h4>
      <p className={`text-2xl font-bold ${valueColorClasses[color]}`}>{value}</p>
      <p className={`text-xs capitalize ${getTrendColor(trend)}`}>
        {trend.replace('_', ' ')}
      </p>
    </div>
  )
}