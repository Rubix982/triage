import React, { useState } from 'react'

interface AIInsight {
  category: string
  insight: string
  confidence: number
  impact: string
  action_items: string[]
  data_points: string[]
}

interface AIInsightsPanelProps {
  insights: AIInsight[]
}

export default function AIInsightsPanel({ insights }: AIInsightsPanelProps) {
  const [selectedInsight, setSelectedInsight] = useState<number | null>(null)
  const [filterByImpact, setFilterByImpact] = useState<string>('all')

  const filteredInsights = insights.filter(insight => 
    filterByImpact === 'all' || insight.impact === filterByImpact
  )

  const getImpactColor = (impact: string) => {
    switch (impact.toLowerCase()) {
      case 'high': return 'text-red-400 bg-red-900/20 border-red-700'
      case 'medium': return 'text-yellow-400 bg-yellow-900/20 border-yellow-700'
      case 'low': return 'text-blue-400 bg-blue-900/20 border-blue-700'
      default: return 'text-gray-400 bg-gray-900/20 border-gray-700'
    }
  }

  const getConfidenceColor = (confidence: number) => {
    if (confidence >= 0.8) return 'text-green-400'
    if (confidence >= 0.6) return 'text-yellow-400'
    return 'text-red-400'
  }

  const getCategoryIcon = (category: string) => {
    switch (category.toLowerCase()) {
      case 'performance': return '⚡'
      case 'process optimization': return '🔧'
      case 'knowledge management': return '📚'
      case 'workflow efficiency': return '🌊'
      case 'collaboration': return '👥'
      case 'quality': return '🎯'
      default: return '💡'
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h3 className="text-2xl font-bold text-white">🧠 AI-Powered Insights</h3>
        
        {/* Impact Filter */}
        <div className="flex items-center space-x-2">
          <label className="text-sm text-gray-400">Filter by impact:</label>
          <select
            value={filterByImpact}
            onChange={(e) => setFilterByImpact(e.target.value)}
            className="bg-gray-800 border border-gray-600 rounded px-3 py-1 text-white text-sm focus:outline-none focus:border-blue-500"
          >
            <option value="all">All Impacts</option>
            <option value="high">High Impact</option>
            <option value="medium">Medium Impact</option>
            <option value="low">Low Impact</option>
          </select>
        </div>
      </div>

      {/* Insights Overview */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
        <div className="bg-red-900/20 p-4 rounded-lg border border-red-700/50">
          <div className="text-2xl font-bold text-red-400">
            {insights.filter(i => i.impact === 'high').length}
          </div>
          <div className="text-sm text-red-300">High Impact Issues</div>
        </div>
        <div className="bg-yellow-900/20 p-4 rounded-lg border border-yellow-700/50">
          <div className="text-2xl font-bold text-yellow-400">
            {insights.filter(i => i.impact === 'medium').length}
          </div>
          <div className="text-sm text-yellow-300">Medium Impact Items</div>
        </div>
        <div className="bg-green-900/20 p-4 rounded-lg border border-green-700/50">
          <div className="text-2xl font-bold text-green-400">
            {(insights.reduce((sum, i) => sum + i.confidence, 0) / insights.length * 100).toFixed(0)}%
          </div>
          <div className="text-sm text-green-300">Avg. Confidence</div>
        </div>
      </div>

      {/* Insights List */}
      <div className="space-y-4">
        {filteredInsights.length === 0 ? (
          <div className="text-center py-8 text-gray-400">
            No insights match the selected filter
          </div>
        ) : (
          filteredInsights.map((insight, index) => (
            <div
              key={index}
              className={`p-6 rounded-lg border transition-all cursor-pointer ${
                selectedInsight === index
                  ? 'bg-gray-700 border-blue-500 shadow-lg'
                  : 'bg-gray-800 border-gray-700 hover:border-gray-600'
              }`}
              onClick={() => setSelectedInsight(selectedInsight === index ? null : index)}
            >
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-center space-x-3">
                  <span className="text-2xl">{getCategoryIcon(insight.category)}</span>
                  <div>
                    <h4 className="font-semibold text-white">{insight.category}</h4>
                    <div className="flex items-center space-x-3 mt-1">
                      <span className={`text-xs px-2 py-1 rounded border ${getImpactColor(insight.impact)}`}>
                        {insight.impact.toUpperCase()} IMPACT
                      </span>
                      <span className={`text-xs font-medium ${getConfidenceColor(insight.confidence)}`}>
                        {(insight.confidence * 100).toFixed(0)}% confidence
                      </span>
                    </div>
                  </div>
                </div>
                <button className="text-gray-400 hover:text-white transition-colors">
                  {selectedInsight === index ? '▼' : '▶'}
                </button>
              </div>

              <p className="text-gray-300 mb-4 leading-relaxed">{insight.insight}</p>

              {/* Expanded Details */}
              {selectedInsight === index && (
                <div className="space-y-4 border-t border-gray-700 pt-4 mt-4">
                  {/* Action Items */}
                  <div>
                    <h5 className="font-medium text-white mb-2">🎯 Recommended Actions</h5>
                    <ul className="space-y-1">
                      {insight.action_items.map((action, actionIndex) => (
                        <li key={actionIndex} className="text-sm text-blue-300 flex items-start">
                          <span className="mr-2">•</span>
                          <span>{action}</span>
                        </li>
                      ))}
                    </ul>
                  </div>

                  {/* Data Points */}
                  {insight.data_points.length > 0 && (
                    <div>
                      <h5 className="font-medium text-white mb-2">📊 Supporting Data</h5>
                      <div className="space-y-1">
                        {insight.data_points.map((point, pointIndex) => (
                          <div key={pointIndex} className="text-sm text-gray-400 bg-gray-900/50 px-3 py-1 rounded">
                            {point}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}

                  {/* Implementation Priority */}
                  <div className="flex items-center justify-between pt-3 border-t border-gray-700">
                    <div className="text-sm text-gray-400">
                      Implementation Priority: 
                      <span className={`ml-2 font-medium ${getImpactColor(insight.impact).split(' ')[0]}`}>
                        {insight.impact.toUpperCase()}
                      </span>
                    </div>
                    <div className="text-sm text-gray-400">
                      Confidence Level: 
                      <span className={`ml-2 font-medium ${getConfidenceColor(insight.confidence)}`}>
                        {(insight.confidence * 100).toFixed(0)}%
                      </span>
                    </div>
                  </div>
                </div>
              )}
            </div>
          ))
        )}
      </div>

      {/* Summary Section */}
      {filteredInsights.length > 0 && (
        <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
          <h4 className="font-semibold text-white mb-3">📋 Executive Summary</h4>
          <div className="space-y-2 text-sm text-gray-300">
            <p>
              • <strong>{insights.filter(i => i.impact === 'high').length} critical issues</strong> requiring immediate attention
            </p>
            <p>
              • <strong>{insights.filter(i => i.confidence >= 0.8).length} high-confidence insights</strong> ready for implementation
            </p>
            <p>
              • Average confidence level of <strong>{(insights.reduce((sum, i) => sum + i.confidence, 0) / insights.length * 100).toFixed(0)}%</strong> across all insights
            </p>
            <p>
              • Primary focus areas: <strong>
                {Array.from(new Set(insights.map(i => i.category))).slice(0, 3).join(', ')}
              </strong>
            </p>
          </div>
        </div>
      )}
    </div>
  )
}