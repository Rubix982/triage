import React, { useEffect, useState } from 'react'

interface KnowledgeConcept {
  id: string
  name: string
  category: string
  description: string
  confidence_score: number
  frequency: number
  related_issues: string[]
  context_examples: string[]
  learning_difficulty: string
  prerequisites: string[]
  related_concepts: string[]
  tags: string[]
}

interface TechnologyKnowledge {
  id: string
  name: string
  category: string
  version_info?: string
  usage_patterns: Array<{
    context: string
    frequency: number
    success_rate: number
    common_mistakes: string[]
  }>
  common_issues: Array<{
    issue: string
    solutions: string[]
    prevention: string[]
    frequency: number
  }>
  best_practices: Array<{
    practice: string
    rationale: string
    examples: string[]
    benefits: string[]
  }>
  learning_resources: string[]
  skill_level_required: string
  team_expertise_level: number
  adoption_trend: string
  related_technologies: string[]
}

interface KnowledgePattern {
  id: string
  pattern_type: string
  name: string
  description: string
  examples: Array<{
    title: string
    description: string
    code_example?: string
    outcome: string
    lessons_learned: string[]
  }>
  effectiveness_score: number
  usage_frequency: number
  success_rate: number
  anti_patterns: string[]
  when_to_use: string[]
  when_not_to_use: string[]
}

interface LearningMaterial {
  id: string
  title: string
  content_type: string
  source_issues: string[]
  extracted_content: string
  key_learnings: string[]
  difficulty_level: string
  estimated_reading_time: number
  prerequisites: string[]
  related_materials: string[]
  quality_score: number
  last_updated: string
}

interface KnowledgeGap {
  id: string
  gap_type: string
  title: string
  description: string
  severity: number
  affected_areas: string[]
  potential_impact: string
  suggested_actions: string[]
  learning_priority: number
  estimated_effort: string
}

interface KnowledgeInsight {
  id: string
  insight_type: string
  title: string
  description: string
  confidence: number
  impact: string
  action_items: string[]
  supporting_evidence: string[]
  generated_at: string
}

interface KnowledgeBaseData {
  concepts: KnowledgeConcept[]
  technologies: TechnologyKnowledge[]
  patterns: KnowledgePattern[]
  learning_materials: LearningMaterial[]
  knowledge_gaps: KnowledgeGap[]
  expertise_map: Record<string, any[]>
  knowledge_graph: any
  insights: KnowledgeInsight[]
}

export default function KnowledgeBase() {
  const [data, setData] = useState<KnowledgeBaseData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [activeTab, setActiveTab] = useState<'concepts' | 'technologies' | 'patterns' | 'materials' | 'gaps' | 'insights'>('concepts')
  const [searchQuery, setSearchQuery] = useState('')

  useEffect(() => {
    const fetchKnowledgeBase = async () => {
      try {
        setLoading(true)
        const response = await fetch('http://127.0.0.1:3001/api/knowledge')
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        const knowledgeData: KnowledgeBaseData = await response.json()
        setData(knowledgeData)
        setError(null)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch knowledge base')
        console.error('Failed to fetch knowledge base:', err)
      } finally {
        setLoading(false)
      }
    }

    fetchKnowledgeBase()
  }, [])

  const filterData = <T extends { name?: string; title?: string }>(items: T[]): T[] => {
    if (!searchQuery) return items
    return items.filter(item => 
      (item.name?.toLowerCase().includes(searchQuery.toLowerCase())) ||
      (item.title?.toLowerCase().includes(searchQuery.toLowerCase()))
    )
  }

  const getCategoryColor = (category: string) => {
    const colors: Record<string, string> = {
      'Technical': 'bg-blue-900/20 border-blue-700 text-blue-300',
      'Business': 'bg-green-900/20 border-green-700 text-green-300',
      'Process': 'bg-purple-900/20 border-purple-700 text-purple-300',
      'Architecture': 'bg-yellow-900/20 border-yellow-700 text-yellow-300',
      'Security': 'bg-red-900/20 border-red-700 text-red-300',
      'Performance': 'bg-orange-900/20 border-orange-700 text-orange-300',
      'Testing': 'bg-indigo-900/20 border-indigo-700 text-indigo-300',
      'DevOps': 'bg-pink-900/20 border-pink-700 text-pink-300',
    }
    return colors[category] || 'bg-gray-900/20 border-gray-700 text-gray-300'
  }

  const getDifficultyColor = (difficulty: string) => {
    const colors: Record<string, string> = {
      'beginner': 'text-green-400',
      'intermediate': 'text-yellow-400',
      'advanced': 'text-red-400',
    }
    return colors[difficulty] || 'text-gray-400'
  }

  const getSeverityColor = (severity: number) => {
    if (severity >= 8) return 'text-red-400'
    if (severity >= 6) return 'text-yellow-400'
    return 'text-green-400'
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-white">🧠 Building intelligent knowledge base...</div>
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
        <div className="text-gray-400">No knowledge base data available</div>
      </div>
    )
  }

  return (
    <div className="w-full space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-3xl font-bold text-white">🧠 Intelligent Knowledge Base</h2>
        <div className="flex items-center space-x-4">
          <input
            type="text"
            placeholder="Search knowledge..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="bg-gray-700 border border-gray-600 rounded px-4 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-blue-500"
          />
        </div>
      </div>

      {/* Navigation Tabs */}
      <div className="border-b border-gray-700">
        <nav className="flex space-x-1">
          {[
            { id: 'concepts', label: '🔍 Concepts', count: data.concepts.length },
            { id: 'technologies', label: '💻 Technologies', count: data.technologies.length },
            { id: 'patterns', label: '🔄 Patterns', count: data.patterns.length },
            { id: 'materials', label: '📚 Learning', count: data.learning_materials.length },
            { id: 'gaps', label: '⚠️ Gaps', count: data.knowledge_gaps.length },
            { id: 'insights', label: '💡 Insights', count: data.insights.length },
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

      {/* Content */}
      <div className="space-y-6">
        {activeTab === 'concepts' && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {filterData(data.concepts).map((concept) => (
              <div key={concept.id} className={`p-6 rounded-lg border ${getCategoryColor(concept.category)}`}>
                <div className="flex items-start justify-between mb-4">
                  <h3 className="text-lg font-semibold text-white">{concept.name}</h3>
                  <div className="flex items-center space-x-2">
                    <span className={`text-sm font-medium ${getDifficultyColor(concept.learning_difficulty)}`}>
                      {concept.learning_difficulty}
                    </span>
                    <span className="text-sm text-gray-400">
                      {(concept.confidence_score * 100).toFixed(0)}%
                    </span>
                  </div>
                </div>
                
                <p className="text-gray-300 text-sm mb-4">{concept.description}</p>
                
                <div className="space-y-3">
                  <div>
                    <span className="text-xs text-gray-400">Frequency:</span>
                    <span className="text-sm text-white ml-2">{concept.frequency} mentions</span>
                  </div>
                  
                  {concept.prerequisites.length > 0 && (
                    <div>
                      <span className="text-xs text-gray-400">Prerequisites:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {concept.prerequisites.slice(0, 3).map((prereq, idx) => (
                          <span key={idx} className="text-xs bg-gray-800 text-gray-300 px-2 py-1 rounded">
                            {prereq}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                  
                  {concept.tags.length > 0 && (
                    <div>
                      <span className="text-xs text-gray-400">Tags:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {concept.tags.slice(0, 4).map((tag, idx) => (
                          <span key={idx} className="text-xs bg-blue-900/30 text-blue-300 px-2 py-1 rounded">
                            {tag}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}

        {activeTab === 'technologies' && (
          <div className="space-y-6">
            {filterData(data.technologies).map((tech) => (
              <div key={tech.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-4">
                  <div>
                    <h3 className="text-xl font-semibold text-white">{tech.name}</h3>
                    <div className="flex items-center space-x-4 mt-2">
                      <span className="text-sm text-gray-400">{tech.category}</span>
                      {tech.version_info && (
                        <span className="text-sm text-blue-400">{tech.version_info}</span>
                      )}
                      <span className={`text-sm font-medium ${getDifficultyColor(tech.skill_level_required)}`}>
                        {tech.skill_level_required}
                      </span>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-2xl font-bold text-green-400">
                      {tech.team_expertise_level.toFixed(1)}/10
                    </div>
                    <div className="text-xs text-gray-400">Team Expertise</div>
                  </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  {tech.usage_patterns.length > 0 && (
                    <div>
                      <h4 className="font-medium text-white mb-3">Usage Patterns</h4>
                      {tech.usage_patterns.map((pattern, idx) => (
                        <div key={idx} className="bg-gray-900/50 p-3 rounded mb-3">
                          <div className="text-sm text-gray-300 mb-2">{pattern.context}</div>
                          <div className="flex items-center justify-between text-xs">
                            <span className="text-gray-400">Frequency: {pattern.frequency}</span>
                            <span className="text-green-400">Success: {(pattern.success_rate * 100).toFixed(0)}%</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}

                  {tech.best_practices.length > 0 && (
                    <div>
                      <h4 className="font-medium text-white mb-3">Best Practices</h4>
                      {tech.best_practices.slice(0, 2).map((practice, idx) => (
                        <div key={idx} className="bg-gray-900/50 p-3 rounded mb-3">
                          <div className="text-sm font-medium text-blue-400 mb-1">{practice.practice}</div>
                          <div className="text-xs text-gray-400 mb-2">{practice.rationale}</div>
                          <div className="flex flex-wrap gap-1">
                            {practice.benefits.map((benefit, bidx) => (
                              <span key={bidx} className="text-xs bg-green-900/30 text-green-300 px-2 py-1 rounded">
                                {benefit}
                              </span>
                            ))}
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>

                {tech.related_technologies.length > 0 && (
                  <div className="mt-4 pt-4 border-t border-gray-700">
                    <span className="text-sm text-gray-400">Related Technologies:</span>
                    <div className="flex flex-wrap gap-2 mt-2">
                      {tech.related_technologies.map((relTech, idx) => (
                        <span key={idx} className="text-sm bg-purple-900/30 text-purple-300 px-3 py-1 rounded">
                          {relTech}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {activeTab === 'patterns' && (
          <div className="space-y-6">
            {filterData(data.patterns).map((pattern) => (
              <div key={pattern.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-4">
                  <div>
                    <h3 className="text-xl font-semibold text-white">{pattern.name}</h3>
                    <div className="flex items-center space-x-4 mt-2">
                      <span className="text-sm text-gray-400">{pattern.pattern_type}</span>
                      <span className="text-sm text-yellow-400">
                        Used {pattern.usage_frequency} times
                      </span>
                      <span className="text-sm text-green-400">
                        {(pattern.success_rate * 100).toFixed(0)}% success
                      </span>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-2xl font-bold text-blue-400">
                      {(pattern.effectiveness_score * 100).toFixed(0)}%
                    </div>
                    <div className="text-xs text-gray-400">Effectiveness</div>
                  </div>
                </div>

                <p className="text-gray-300 mb-4">{pattern.description}</p>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div>
                    <h4 className="font-medium text-white mb-3">When to Use</h4>
                    <ul className="space-y-1">
                      {pattern.when_to_use.map((use, idx) => (
                        <li key={idx} className="text-sm text-gray-300 flex items-start">
                          <span className="text-green-400 mr-2">✓</span>
                          {use}
                        </li>
                      ))}
                    </ul>
                  </div>

                  <div>
                    <h4 className="font-medium text-white mb-3">When NOT to Use</h4>
                    <ul className="space-y-1">
                      {pattern.when_not_to_use.map((notUse, idx) => (
                        <li key={idx} className="text-sm text-gray-300 flex items-start">
                          <span className="text-red-400 mr-2">✗</span>
                          {notUse}
                        </li>
                      ))}
                    </ul>
                  </div>
                </div>

                {pattern.examples.length > 0 && (
                  <div className="mt-6 pt-4 border-t border-gray-700">
                    <h4 className="font-medium text-white mb-3">Examples</h4>
                    <div className="space-y-3">
                      {pattern.examples.slice(0, 2).map((example, idx) => (
                        <div key={idx} className="bg-gray-900/50 p-3 rounded">
                          <div className="text-sm font-medium text-blue-400 mb-1">{example.title}</div>
                          <div className="text-xs text-gray-300 mb-2">{example.description}</div>
                          {example.code_example && (
                            <code className="text-xs bg-black/50 text-green-400 p-2 rounded block">
                              {example.code_example}
                            </code>
                          )}
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {activeTab === 'materials' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            {filterData(data.learning_materials).map((material) => (
              <div key={material.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-4">
                  <h3 className="text-lg font-semibold text-white">{material.title}</h3>
                  <div className="text-right">
                    <div className="text-sm text-blue-400">{material.content_type}</div>
                    <div className="text-xs text-gray-400">{material.estimated_reading_time} min read</div>
                  </div>
                </div>

                <div className="flex items-center space-x-4 mb-4">
                  <span className={`text-sm font-medium ${getDifficultyColor(material.difficulty_level)}`}>
                    {material.difficulty_level}
                  </span>
                  <div className="flex items-center">
                    <span className="text-sm text-yellow-400">★</span>
                    <span className="text-sm text-gray-400 ml-1">
                      {(material.quality_score * 5).toFixed(1)}/5
                    </span>
                  </div>
                  <span className="text-sm text-gray-400">
                    {material.source_issues.length} sources
                  </span>
                </div>

                <div className="text-sm text-gray-300 mb-4 line-clamp-3">
                  {material.extracted_content.substring(0, 200)}...
                </div>

                {material.key_learnings.length > 0 && (
                  <div className="mb-4">
                    <h4 className="text-sm font-medium text-white mb-2">Key Learnings:</h4>
                    <ul className="space-y-1">
                      {material.key_learnings.slice(0, 3).map((learning, idx) => (
                        <li key={idx} className="text-xs text-gray-400 flex items-start">
                          <span className="text-blue-400 mr-2">•</span>
                          {learning}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                {material.prerequisites.length > 0 && (
                  <div className="pt-3 border-t border-gray-700">
                    <span className="text-xs text-gray-400">Prerequisites:</span>
                    <div className="flex flex-wrap gap-1 mt-1">
                      {material.prerequisites.map((prereq, idx) => (
                        <span key={idx} className="text-xs bg-orange-900/30 text-orange-300 px-2 py-1 rounded">
                          {prereq}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {activeTab === 'gaps' && (
          <div className="space-y-6">
            {data.knowledge_gaps.map((gap) => (
              <div key={gap.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-4">
                  <h3 className="text-lg font-semibold text-white">{gap.title}</h3>
                  <div className="text-right">
                    <div className={`text-2xl font-bold ${getSeverityColor(gap.severity)}`}>
                      {gap.severity.toFixed(1)}/10
                    </div>
                    <div className="text-xs text-gray-400">Severity</div>
                  </div>
                </div>

                <div className="flex items-center space-x-4 mb-4">
                  <span className="text-sm text-gray-400">{gap.gap_type}</span>
                  <span className="text-sm text-blue-400">Priority {gap.learning_priority}</span>
                  <span className="text-sm text-yellow-400">{gap.estimated_effort}</span>
                </div>

                <p className="text-gray-300 mb-4">{gap.description}</p>

                <div className="mb-4">
                  <h4 className="text-sm font-medium text-white mb-2">Potential Impact:</h4>
                  <p className="text-sm text-red-300">{gap.potential_impact}</p>
                </div>

                <div className="mb-4">
                  <h4 className="text-sm font-medium text-white mb-2">Suggested Actions:</h4>
                  <ul className="space-y-1">
                    {gap.suggested_actions.map((action, idx) => (
                      <li key={idx} className="text-sm text-gray-300 flex items-start">
                        <span className="text-green-400 mr-2">•</span>
                        {action}
                      </li>
                    ))}
                  </ul>
                </div>

                {gap.affected_areas.length > 0 && (
                  <div className="pt-3 border-t border-gray-700">
                    <span className="text-xs text-gray-400">Affected Areas:</span>
                    <div className="flex flex-wrap gap-1 mt-1">
                      {gap.affected_areas.map((area, idx) => (
                        <span key={idx} className="text-xs bg-red-900/30 text-red-300 px-2 py-1 rounded">
                          {area}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {activeTab === 'insights' && (
          <div className="space-y-6">
            {data.insights.map((insight) => (
              <div key={insight.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                <div className="flex items-start justify-between mb-4">
                  <div>
                    <h3 className="text-lg font-semibold text-white">{insight.title}</h3>
                    <div className="flex items-center space-x-4 mt-2">
                      <span className="text-sm text-gray-400">{insight.insight_type}</span>
                      <span className="text-sm text-blue-400">{insight.impact} impact</span>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-2xl font-bold text-green-400">
                      {(insight.confidence * 100).toFixed(0)}%
                    </div>
                    <div className="text-xs text-gray-400">Confidence</div>
                  </div>
                </div>

                <p className="text-gray-300 mb-4">{insight.description}</p>

                <div className="mb-4">
                  <h4 className="text-sm font-medium text-white mb-2">Action Items:</h4>
                  <ul className="space-y-1">
                    {insight.action_items.map((item, idx) => (
                      <li key={idx} className="text-sm text-gray-300 flex items-start">
                        <span className="text-blue-400 mr-2">•</span>
                        {item}
                      </li>
                    ))}
                  </ul>
                </div>

                {insight.supporting_evidence.length > 0 && (
                  <div className="pt-3 border-t border-gray-700">
                    <h4 className="text-sm font-medium text-white mb-2">Supporting Evidence:</h4>
                    <ul className="space-y-1">
                      {insight.supporting_evidence.map((evidence, idx) => (
                        <li key={idx} className="text-xs text-gray-400 flex items-start">
                          <span className="text-yellow-400 mr-2">•</span>
                          {evidence}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                <div className="text-xs text-gray-500 mt-4">
                  Generated: {new Date(insight.generated_at).toLocaleDateString()}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}