import React, { useState, useEffect, useRef } from 'react'

interface SearchResult {
  id: string
  title: string
  content: string
  result_type: string
  similarity_score: number
  context: {
    project: string
    category: string
    difficulty: string
    expertise_level: number
    usage_frequency: number
    related_concepts: string[]
  }
  related_items: Array<{
    id: string
    title: string
    relationship_type: string
    similarity_score: number
  }>
  tags: string[]
  created_date: string
  last_updated: string
}

interface ConceptCluster {
  id: string
  name: string
  center_concept: string
  related_concepts: string[]
  cohesion_score: number
  cluster_size: number
  dominant_category: string
  average_difficulty: string
  key_technologies: string[]
  representative_issues: string[]
}

interface SmartRecommendation {
  id: string
  recommendation_type: string
  title: string
  description: string
  target_items: string[]
  confidence_score: number
  reasoning: string[]
  action_items: string[]
  estimated_value: string
}

interface SearchResponse {
  results: SearchResult[]
  clusters: ConceptCluster[]
  recommendations: SmartRecommendation[]
  query_analysis: {
    detected_concepts: string[]
    detected_technologies: string[]
    query_intent: string
    difficulty_level: string
    suggested_refinements: string[]
  }
  total_results: number
  search_time_ms: number
}

interface UserNote {
  id: string
  title: string
  content: string
  note_type: string
  tags: string[]
  linked_items: Array<{
    item_id: string
    item_type: string
    title: string
    relationship: string
  }>
  created_at: string
  updated_at: string
  is_private: boolean
  metadata: {
    color?: string
    priority: number
    completion_status: string
    estimated_time?: number
    difficulty_level: string
    source_context?: string
  }
}

interface SavedView {
  id: string
  name: string
  description: string
  view_type: string
  configuration: any
  created_at: string
  is_favorite: boolean
  usage_count: number
  last_accessed: string
}

export default function SmartSearch() {
  const [query, setQuery] = useState('')
  const [searchResults, setSearchResults] = useState<SearchResponse | null>(null)
  const [userNotes, setUserNotes] = useState<UserNote[]>([])
  const [savedViews, setSavedViews] = useState<SavedView[]>([])
  const [isSearching, setIsSearching] = useState(false)
  const [activeTab, setActiveTab] = useState<'results' | 'clusters' | 'recommendations' | 'notes' | 'views'>('results')
  const [selectedItems, setSelectedItems] = useState<Set<string>>(new Set())
  const [showNoteModal, setShowNoteModal] = useState(false)
  const [showSaveViewModal, setShowSaveViewModal] = useState(false)
  const [noteContent, setNoteContent] = useState('')
  const [noteTitle, setNoteTitle] = useState('')
  const [viewName, setViewName] = useState('')
  const [viewDescription, setViewDescription] = useState('')

  const searchInputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    loadUserNotes()
    loadSavedViews()
  }, [])

  const loadUserNotes = async () => {
    try {
      const response = await fetch('http://127.0.0.1:3001/api/notes')
      if (response.ok) {
        const data = await response.json()
        setUserNotes(data.notes)
      }
    } catch (error) {
      console.error('Failed to load notes:', error)
    }
  }

  const loadSavedViews = async () => {
    try {
      const response = await fetch('http://127.0.0.1:3001/api/views')
      if (response.ok) {
        const data = await response.json()
        setSavedViews(data.views)
      }
    } catch (error) {
      console.error('Failed to load views:', error)
    }
  }

  const performSearch = async () => {
    if (!query.trim()) return

    setIsSearching(true)
    try {
      const searchParams = new URLSearchParams({
        q: query,
        threshold: '0.3',
        limit: '20',
        include_related: 'true'
      })

      const response = await fetch(`http://127.0.0.1:3001/api/search?${searchParams}`)
      if (response.ok) {
        const data: SearchResponse = await response.json()
        setSearchResults(data)
        setActiveTab('results')
      }
    } catch (error) {
      console.error('Search failed:', error)
    } finally {
      setIsSearching(false)
    }
  }

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      performSearch()
    }
  }

  const toggleItemSelection = (itemId: string) => {
    const newSelected = new Set(selectedItems)
    if (newSelected.has(itemId)) {
      newSelected.delete(itemId)
    } else {
      newSelected.add(itemId)
    }
    setSelectedItems(newSelected)
  }

  const createNoteFromSelected = async () => {
    if (selectedItems.size === 0) return

    const selectedResults = searchResults?.results.filter(r => selectedItems.has(r.id)) || []
    const linkedItems = selectedResults.map(r => ({
      item_id: r.id,
      item_type: r.result_type,
      title: r.title,
      relationship: 'referenced_in_note'
    }))

    try {
      const response = await fetch('http://127.0.0.1:3001/api/notes', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          title: noteTitle || `Search Notes: ${query}`,
          content: noteContent,
          note_type: 'PersonalNote',
          tags: searchResults?.query_analysis.detected_concepts || [],
          linked_items: linkedItems,
          is_private: false,
          metadata: {
            priority: 3,
            completion_status: 'NotStarted',
            difficulty_level: searchResults?.query_analysis.difficulty_level || 'intermediate',
            source_context: `Search: ${query}`
          }
        })
      })

      if (response.ok) {
        setShowNoteModal(false)
        setNoteContent('')
        setNoteTitle('')
        setSelectedItems(new Set())
        loadUserNotes()
      }
    } catch (error) {
      console.error('Failed to create note:', error)
    }
  }

  const saveCurrentView = async () => {
    if (!searchResults || !viewName.trim()) return

    try {
      const response = await fetch('http://127.0.0.1:3001/api/views', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: viewName,
          description: viewDescription,
          view_type: 'Search',
          configuration: {
            search_query: query,
            filters: {},
            display_options: {
              layout: 'list',
              items_per_page: 20,
              show_previews: true,
              group_by: null,
              color_coding: null
            },
            sort_preferences: {
              primary_sort: 'similarity_score',
              secondary_sort: 'updated_at',
              sort_order: 'desc',
              custom_weights: {}
            },
            custom_fields: []
          }
        })
      })

      if (response.ok) {
        setShowSaveViewModal(false)
        setViewName('')
        setViewDescription('')
        loadSavedViews()
      }
    } catch (error) {
      console.error('Failed to save view:', error)
    }
  }

  const loadSavedView = async (view: SavedView) => {
    // Update view usage
    await fetch(`http://127.0.0.1:3001/api/views/${view.id}/use`, {
      method: 'PUT'
    })

    // Load the saved search
    if (view.configuration.search_query) {
      setQuery(view.configuration.search_query)
      // Trigger search after setting query
      setTimeout(() => {
        performSearch()
      }, 100)
    }

    loadSavedViews() // Refresh to update usage count
  }

  const toggleViewFavorite = async (viewId: string) => {
    try {
      const response = await fetch(`http://127.0.0.1:3001/api/views/${viewId}/favorite`, {
        method: 'PUT'
      })
      if (response.ok) {
        loadSavedViews()
      }
    } catch (error) {
      console.error('Failed to toggle favorite:', error)
    }
  }

  const getResultTypeIcon = (type: string) => {
    const icons: Record<string, string> = {
      'Issue': '🎫',
      'Concept': '💡',
      'Technology': '⚙️',
      'Pattern': '🔄',
      'LearningMaterial': '📚',
      'UserNote': '📝'
    }
    return icons[type] || '📄'
  }

  const getDifficultyColor = (difficulty: string) => {
    const colors: Record<string, string> = {
      'beginner': 'text-green-400',
      'intermediate': 'text-yellow-400',
      'advanced': 'text-red-400'
    }
    return colors[difficulty] || 'text-gray-400'
  }

  const getSimilarityColor = (score: number) => {
    if (score > 0.8) return 'text-green-400'
    if (score > 0.6) return 'text-yellow-400'
    if (score > 0.4) return 'text-orange-400'
    return 'text-red-400'
  }

  return (
    <div className="w-full space-y-6">
      {/* Header */}
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-white">🔍 Smart Knowledge Search</h1>
        <p className="text-gray-300 max-w-2xl mx-auto">
          Discover connections, save insights, and build your personal knowledge base with AI-powered semantic search
        </p>
      </div>

      {/* Search Interface */}
      <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
        <div className="flex items-center space-x-4">
          <div className="flex-1 relative">
            <input
              ref={searchInputRef}
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Search for issues, concepts, technologies, patterns... (e.g., 'React authentication issues' or 'Docker deployment problems')"
              className="w-full bg-gray-700 border border-gray-600 rounded-lg px-4 py-3 text-white placeholder-gray-400 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/20"
            />
            {isSearching && (
              <div className="absolute right-3 top-3.5">
                <div className="animate-spin h-5 w-5 border-2 border-blue-500 border-t-transparent rounded-full"></div>
              </div>
            )}
          </div>
          <button
            onClick={performSearch}
            disabled={isSearching || !query.trim()}
            className="px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white rounded-lg font-medium transition-colors"
          >
            {isSearching ? 'Searching...' : 'Search'}
          </button>
        </div>

        {/* Quick Actions */}
        {searchResults && (
          <div className="flex items-center justify-between mt-4 pt-4 border-t border-gray-700">
            <div className="flex items-center space-x-4">
              <span className="text-sm text-gray-400">
                {searchResults.total_results} results in {searchResults.search_time_ms}ms
              </span>
              {selectedItems.size > 0 && (
                <span className="text-sm text-blue-400">
                  {selectedItems.size} items selected
                </span>
              )}
            </div>
            <div className="flex items-center space-x-2">
              {selectedItems.size > 0 && (
                <button
                  onClick={() => setShowNoteModal(true)}
                  className="px-3 py-1 bg-green-600 hover:bg-green-700 text-white text-sm rounded font-medium"
                >
                  📝 Create Note
                </button>
              )}
              <button
                onClick={() => setShowSaveViewModal(true)}
                className="px-3 py-1 bg-purple-600 hover:bg-purple-700 text-white text-sm rounded font-medium"
              >
                💾 Save View
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Search Results */}
      {searchResults && (
        <div className="space-y-6">
          {/* Query Analysis */}
          {searchResults.query_analysis && (
            <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
              <h3 className="font-medium text-white mb-3">🧠 Query Analysis</h3>
              <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                <div>
                  <span className="text-xs text-gray-400">Intent:</span>
                  <div className="text-sm text-blue-400 font-medium">
                    {searchResults.query_analysis.query_intent.replace(/([A-Z])/g, ' $1').trim()}
                  </div>
                </div>
                <div>
                  <span className="text-xs text-gray-400">Difficulty:</span>
                  <div className={`text-sm font-medium ${getDifficultyColor(searchResults.query_analysis.difficulty_level)}`}>
                    {searchResults.query_analysis.difficulty_level}
                  </div>
                </div>
                <div>
                  <span className="text-xs text-gray-400">Technologies:</span>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {searchResults.query_analysis.detected_technologies.slice(0, 3).map((tech, idx) => (
                      <span key={idx} className="text-xs bg-blue-900/30 text-blue-300 px-2 py-1 rounded">
                        {tech}
                      </span>
                    ))}
                  </div>
                </div>
                <div>
                  <span className="text-xs text-gray-400">Concepts:</span>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {searchResults.query_analysis.detected_concepts.slice(0, 3).map((concept, idx) => (
                      <span key={idx} className="text-xs bg-purple-900/30 text-purple-300 px-2 py-1 rounded">
                        {concept}
                      </span>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* Navigation Tabs */}
          <div className="border-b border-gray-700">
            <nav className="flex space-x-1">
              {[
                { id: 'results', label: '🔍 Results', count: searchResults.results.length },
                { id: 'clusters', label: '🕸️ Clusters', count: searchResults.clusters.length },
                { id: 'recommendations', label: '💡 Recommendations', count: searchResults.recommendations.length },
                { id: 'notes', label: '📝 My Notes', count: userNotes.length },
                { id: 'views', label: '👁️ Saved Views', count: savedViews.length },
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
          <div className="space-y-6">
            {activeTab === 'results' && (
              <div className="space-y-4">
                {searchResults.results.map((result) => (
                  <div key={result.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700 hover:border-gray-600 transition-colors">
                    <div className="flex items-start justify-between mb-4">
                      <div className="flex items-start space-x-4 flex-1">
                        <input
                          type="checkbox"
                          checked={selectedItems.has(result.id)}
                          onChange={() => toggleItemSelection(result.id)}
                          className="mt-1 rounded"
                        />
                        <div className="flex-1">
                          <div className="flex items-center space-x-3 mb-2">
                            <span className="text-lg">{getResultTypeIcon(result.result_type)}</span>
                            <h3 className="text-lg font-semibold text-white">{result.title}</h3>
                            <span className="text-xs text-gray-400">{result.result_type}</span>
                          </div>
                          <p className="text-gray-300 text-sm line-clamp-2 mb-3">{result.content}</p>
                        </div>
                      </div>
                      <div className="text-right ml-4">
                        <div className={`text-lg font-bold ${getSimilarityColor(result.similarity_score)}`}>
                          {(result.similarity_score * 100).toFixed(0)}%
                        </div>
                        <div className="text-xs text-gray-400">similarity</div>
                      </div>
                    </div>

                    <div className="flex items-center justify-between">
                      <div className="flex items-center space-x-4">
                        <div className="flex items-center space-x-2">
                          <span className="text-xs text-gray-400">Project:</span>
                          <span className="text-sm text-blue-400">{result.context.project}</span>
                        </div>
                        <div className="flex items-center space-x-2">
                          <span className="text-xs text-gray-400">Level:</span>
                          <span className={`text-sm font-medium ${getDifficultyColor(result.context.difficulty)}`}>
                            {result.context.difficulty}
                          </span>
                        </div>
                      </div>

                      {result.tags.length > 0 && (
                        <div className="flex flex-wrap gap-1">
                          {result.tags.slice(0, 3).map((tag, idx) => (
                            <span key={idx} className="text-xs bg-gray-700 text-gray-300 px-2 py-1 rounded">
                              {tag}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>

                    {result.related_items.length > 0 && (
                      <div className="mt-4 pt-4 border-t border-gray-700">
                        <span className="text-sm text-gray-400 mb-2 block">Related Items:</span>
                        <div className="space-y-1">
                          {result.related_items.slice(0, 2).map((item, idx) => (
                            <div key={idx} className="text-sm text-gray-300 flex items-center justify-between">
                              <span>{item.title}</span>
                              <span className="text-xs text-blue-400">{(item.similarity_score * 100).toFixed(0)}%</span>
                            </div>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                ))}
              </div>
            )}

            {activeTab === 'clusters' && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {searchResults.clusters.map((cluster) => (
                  <div key={cluster.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                    <div className="flex items-start justify-between mb-4">
                      <h3 className="text-lg font-semibold text-white">{cluster.name}</h3>
                      <div className="text-right">
                        <div className="text-lg font-bold text-green-400">{cluster.cluster_size}</div>
                        <div className="text-xs text-gray-400">items</div>
                      </div>
                    </div>

                    <div className="space-y-3">
                      <div>
                        <span className="text-xs text-gray-400">Center Concept:</span>
                        <div className="text-sm text-blue-400 font-medium">{cluster.center_concept}</div>
                      </div>

                      <div>
                        <span className="text-xs text-gray-400">Related Concepts:</span>
                        <div className="flex flex-wrap gap-1 mt-1">
                          {cluster.related_concepts.slice(0, 4).map((concept, idx) => (
                            <span key={idx} className="text-xs bg-purple-900/30 text-purple-300 px-2 py-1 rounded">
                              {concept}
                            </span>
                          ))}
                        </div>
                      </div>

                      <div>
                        <span className="text-xs text-gray-400">Technologies:</span>
                        <div className="flex flex-wrap gap-1 mt-1">
                          {cluster.key_technologies.map((tech, idx) => (
                            <span key={idx} className="text-xs bg-blue-900/30 text-blue-300 px-2 py-1 rounded">
                              {tech}
                            </span>
                          ))}
                        </div>
                      </div>

                      <div className="flex items-center justify-between text-sm">
                        <span className="text-gray-400">Cohesion: {(cluster.cohesion_score * 100).toFixed(0)}%</span>
                        <span className={`font-medium ${getDifficultyColor(cluster.average_difficulty)}`}>
                          {cluster.average_difficulty}
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}

            {activeTab === 'recommendations' && (
              <div className="space-y-4">
                {searchResults.recommendations.map((rec) => (
                  <div key={rec.id} className="bg-gray-800 p-6 rounded-lg border border-gray-700">
                    <div className="flex items-start justify-between mb-4">
                      <h3 className="text-lg font-semibold text-white">{rec.title}</h3>
                      <div className="text-right">
                        <div className="text-lg font-bold text-green-400">
                          {(rec.confidence_score * 100).toFixed(0)}%
                        </div>
                        <div className="text-xs text-gray-400">confidence</div>
                      </div>
                    </div>

                    <p className="text-gray-300 mb-4">{rec.description}</p>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <div>
                        <span className="text-sm font-medium text-white mb-2 block">Reasoning:</span>
                        <ul className="space-y-1">
                          {rec.reasoning.map((reason, idx) => (
                            <li key={idx} className="text-sm text-gray-400 flex items-start">
                              <span className="text-yellow-400 mr-2">•</span>
                              {reason}
                            </li>
                          ))}
                        </ul>
                      </div>

                      <div>
                        <span className="text-sm font-medium text-white mb-2 block">Action Items:</span>
                        <ul className="space-y-1">
                          {rec.action_items.map((action, idx) => (
                            <li key={idx} className="text-sm text-gray-300 flex items-start">
                              <span className="text-green-400 mr-2">→</span>
                              {action}
                            </li>
                          ))}
                        </ul>
                      </div>
                    </div>

                    <div className="mt-4 pt-4 border-t border-gray-700 flex items-center justify-between">
                      <span className="text-sm text-gray-400">Type: {rec.recommendation_type}</span>
                      <span className="text-sm text-blue-400">Value: {rec.estimated_value}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}

            {activeTab === 'notes' && (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {userNotes.map((note) => (
                  <div key={note.id} className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <div className="flex items-start justify-between mb-3">
                      <h3 className="text-sm font-semibold text-white line-clamp-1">{note.title}</h3>
                      <span className="text-xs text-gray-400">{note.note_type}</span>
                    </div>

                    <p className="text-sm text-gray-300 line-clamp-3 mb-3">{note.content}</p>

                    <div className="space-y-2">
                      {note.tags.length > 0 && (
                        <div className="flex flex-wrap gap-1">
                          {note.tags.slice(0, 3).map((tag, idx) => (
                            <span key={idx} className="text-xs bg-blue-900/30 text-blue-300 px-2 py-1 rounded">
                              {tag}
                            </span>
                          ))}
                        </div>
                      )}

                      <div className="flex items-center justify-between text-xs text-gray-400">
                        <span>Priority: {note.metadata.priority}/5</span>
                        <span>{new Date(note.created_at).toLocaleDateString()}</span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}

            {activeTab === 'views' && (
              <div className="space-y-4">
                {savedViews.map((view) => (
                  <div key={view.id} className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <div className="flex items-start justify-between mb-3">
                      <div className="flex-1">
                        <div className="flex items-center space-x-3">
                          <h3 className="text-lg font-semibold text-white">{view.name}</h3>
                          {view.is_favorite && <span className="text-yellow-400">⭐</span>}
                        </div>
                        <p className="text-sm text-gray-300 mt-1">{view.description}</p>
                      </div>
                      <div className="flex items-center space-x-2 ml-4">
                        <button
                          onClick={() => toggleViewFavorite(view.id)}
                          className="p-1 text-gray-400 hover:text-yellow-400 transition-colors"
                        >
                          {view.is_favorite ? '⭐' : '☆'}
                        </button>
                        <button
                          onClick={() => loadSavedView(view)}
                          className="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-sm rounded"
                        >
                          Load
                        </button>
                      </div>
                    </div>

                    <div className="flex items-center justify-between text-sm text-gray-400">
                      <span>Type: {view.view_type}</span>
                      <span>Used {view.usage_count} times</span>
                      <span>{new Date(view.last_accessed).toLocaleDateString()}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      )}

      {/* Create Note Modal */}
      {showNoteModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-gray-800 p-6 rounded-lg border border-gray-700 w-full max-w-md">
            <h2 className="text-lg font-semibold text-white mb-4">📝 Create Note from Selected Items</h2>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-gray-300 mb-2">Note Title</label>
                <input
                  type="text"
                  value={noteTitle}
                  onChange={(e) => setNoteTitle(e.target.value)}
                  placeholder={`Search Notes: ${query}`}
                  className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm text-gray-300 mb-2">Content</label>
                <textarea
                  value={noteContent}
                  onChange={(e) => setNoteContent(e.target.value)}
                  rows={4}
                  placeholder="Add your insights, thoughts, or summaries here..."
                  className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-blue-500"
                />
              </div>

              <div className="text-sm text-gray-400">
                Will link to {selectedItems.size} selected items
              </div>
            </div>

            <div className="flex justify-end space-x-3 mt-6">
              <button
                onClick={() => setShowNoteModal(false)}
                className="px-4 py-2 text-gray-300 hover:text-white transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={createNoteFromSelected}
                className="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded font-medium"
              >
                Create Note
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Save View Modal */}
      {showSaveViewModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-gray-800 p-6 rounded-lg border border-gray-700 w-full max-w-md">
            <h2 className="text-lg font-semibold text-white mb-4">💾 Save Current View</h2>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-gray-300 mb-2">View Name</label>
                <input
                  type="text"
                  value={viewName}
                  onChange={(e) => setViewName(e.target.value)}
                  placeholder="My Search View"
                  className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm text-gray-300 mb-2">Description</label>
                <textarea
                  value={viewDescription}
                  onChange={(e) => setViewDescription(e.target.value)}
                  rows={3}
                  placeholder="Describe what this view is for..."
                  className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-400 focus:outline-none focus:border-blue-500"
                />
              </div>

              <div className="text-sm text-gray-400">
                Query: "{query}"
              </div>
            </div>

            <div className="flex justify-end space-x-3 mt-6">
              <button
                onClick={() => setShowSaveViewModal(false)}
                className="px-4 py-2 text-gray-300 hover:text-white transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={saveCurrentView}
                disabled={!viewName.trim()}
                className="px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 text-white rounded font-medium"
              >
                Save View
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}