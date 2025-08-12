import React, { useState, useEffect, useMemo } from 'react';
import { Search, Filter, Globe, FileText, MessageSquare, GitBranch, Users, Clock, TrendingUp, ExternalLink, Lock, Unlock, Star } from 'lucide-react';
import { useAuth } from '../contexts/AuthContext';

// Types for the unified search system
interface UnifiedSearchRequest {
  query: string;
  content_types: string[];
  platforms: string[];
  search_mode: 'Relevance' | 'Recent' | 'Popular' | 'Comprehensive' | 'Semantic';
  include_relationships: boolean;
  max_results?: number;
  user_id?: string;
}

interface UnifiedSearchResult {
  query: string;
  total_results: number;
  search_time_ms: number;
  results: SearchResultItem[];
  facets: SearchFacets;
  suggestions: string[];
  related_queries: string[];
}

interface SearchResultItem {
  id: string;
  title: string;
  content: string;
  content_preview: string;
  result_type: string;
  platform: string;
  similarity_score: number;
  relevance_score: number;
  context: SearchContext;
  related_items: RelatedItem[];
  tags: string[];
  concepts: string[];
  technologies: string[];
  created_date: string;
  last_updated: string;
  author?: string;
  engagement_metrics: EngagementMetrics;
  access_info: AccessInfo;
}

interface SearchContext {
  project: string;
  platform: string;
  content_type: string;
  category: string;
  source_url: string;
  parent_ticket?: string;
  knowledge_impact_score: number;
  usage_frequency: number;
  related_concepts: string[];
}

interface RelatedItem {
  id: string;
  title: string;
  relationship_type: string;
  similarity_score: number;
}

interface EngagementMetrics {
  view_count: number;
  comment_count: number;
  share_count: number;
  reaction_count: number;
  search_hits: number;
  knowledge_score: number;
}

interface AccessInfo {
  is_accessible: boolean;
  requires_auth: boolean;
  platform_auth_required: string[];
  sharing_level: string;
}

interface SearchFacets {
  platforms: Record<string, number>;
  content_types: Record<string, number>;
  authors: Record<string, number>;
  projects: Record<string, number>;
  concepts: Record<string, number>;
  technologies: Record<string, number>;
}

const UniversalSearch: React.FC = () => {
  const { google, slack, github, confluence, initiateAuth, isLoading: authLoading, error: authError } = useAuth();
  
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<UnifiedSearchResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Search configuration
  const [searchMode, setSearchMode] = useState<'Relevance' | 'Recent' | 'Popular' | 'Comprehensive' | 'Semantic'>('Comprehensive');
  const [selectedPlatforms, setSelectedPlatforms] = useState<string[]>(['jira', 'google', 'slack']);
  const [selectedContentTypes, setSelectedContentTypes] = useState<string[]>(['All']);
  const [includeRelationships, setIncludeRelationships] = useState(true);
  
  // UI state
  const [activeTab, setActiveTab] = useState<'all' | 'jira' | 'google' | 'slack'>('all');
  const [showFilters, setShowFilters] = useState(false);
  const [expandedResults, setExpandedResults] = useState<Set<string>>(new Set());

  // Get authentication status from context
  const authStatus = {
    google: google.isAuthenticated,
    slack: slack.isAuthenticated,
    github: github.isAuthenticated,
    confluence: confluence.isAuthenticated,
  };

  const performSearch = async () => {
    if (!query.trim()) return;
    
    setLoading(true);
    setError(null);
    
    try {
      const searchRequest: UnifiedSearchRequest = {
        query: query.trim(),
        content_types: selectedContentTypes,
        platforms: selectedPlatforms,
        search_mode: searchMode,
        include_relationships: includeRelationships,
        max_results: 50,
        user_id: 'current_user', // Would come from auth context
      };

      const response = await fetch('http://127.0.0.1:3001/api/search/unified', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(searchRequest),
      });

      if (!response.ok) {
        throw new Error(`Search failed: ${response.status}`);
      }

      const searchResults: UnifiedSearchResult = await response.json();
      setResults(searchResults);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Search failed');
      console.error('Search error:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      performSearch();
    }
  };

  const filteredResults = useMemo(() => {
    if (!results) return [];
    if (activeTab === 'all') return results.results;
    return results.results.filter(result => result.platform === activeTab);
  }, [results, activeTab]);

  const getPlatformIcon = (platform: string) => {
    switch (platform) {
      case 'jira': return '📋';
      case 'google': return '📄';
      case 'slack': return '💬';
      case 'github': return '🔧';
      case 'confluence': return '📖';
      default: return '📄';
    }
  };

  const getPlatformColor = (platform: string) => {
    switch (platform) {
      case 'jira': return 'text-blue-400 bg-blue-900/20 border-blue-700';
      case 'google': return 'text-green-400 bg-green-900/20 border-green-700';
      case 'slack': return 'text-purple-400 bg-purple-900/20 border-purple-700';
      case 'github': return 'text-orange-400 bg-orange-900/20 border-orange-700';
      case 'confluence': return 'text-cyan-400 bg-cyan-900/20 border-cyan-700';
      default: return 'text-gray-400 bg-gray-900/20 border-gray-700';
    }
  };

  const getContentTypeIcon = (contentType: string) => {
    switch (contentType.toLowerCase()) {
      case 'jira_issue': return <FileText className="w-4 h-4" />;
      case 'google_doc': case 'google_sheet': case 'google_slide': return <FileText className="w-4 h-4" />;
      case 'slack_thread': case 'slack_message': return <MessageSquare className="w-4 h-4" />;
      case 'github_pr': case 'github_issue': return <GitBranch className="w-4 h-4" />;
      default: return <FileText className="w-4 h-4" />;
    }
  };

  const toggleResultExpansion = (resultId: string) => {
    const newExpanded = new Set(expandedResults);
    if (newExpanded.has(resultId)) {
      newExpanded.delete(resultId);
    } else {
      newExpanded.add(resultId);
    }
    setExpandedResults(newExpanded);
  };

  const handleAuthClick = async (platform: string) => {
    if (!authStatus[platform as keyof typeof authStatus]) {
      try {
        await initiateAuth(platform);
      } catch (err) {
        console.error(`Failed to initiate ${platform} auth:`, err);
      }
    }
  };

  return (
    <div className="w-full space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold text-white flex items-center gap-3">
            <Globe className="w-8 h-8 text-blue-400" />
            Universal Knowledge Search
          </h2>
          <p className="text-gray-300 mt-2">
            Search across Jira, Google Docs, Slack conversations, and more - all in one place
          </p>
        </div>
        
        {/* Authentication Status */}
        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-400">Connected:</span>
          {Object.entries(authStatus).map(([platform, connected]) => (
            <button
              key={platform}
              onClick={() => handleAuthClick(platform)}
              disabled={authLoading}
              className={`flex items-center gap-1 px-2 py-1 rounded-md text-xs border ${
                connected 
                  ? 'text-green-400 bg-green-900/20 border-green-700 cursor-default' 
                  : 'text-gray-400 bg-gray-900/20 border-gray-700 hover:border-gray-600 cursor-pointer disabled:opacity-50'
              }`}
            >
              {connected ? <Unlock className="w-3 h-3" /> : <Lock className="w-3 h-3" />}
              {platform} {authLoading && '...'}
            </button>
          ))}
        </div>
      </div>

      {/* Search Bar */}
      <div className="relative">
        <div className="flex items-center gap-3">
          <div className="relative flex-1">
            <Search className="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyPress={handleKeyPress}
              placeholder="Search across all platforms... (e.g., 'authentication timeout', 'payment service design')"
              className="w-full pl-12 pr-4 py-4 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-400 focus:border-blue-500 focus:outline-none text-lg"
            />
          </div>
          <button
            onClick={performSearch}
            disabled={loading || !query.trim()}
            className="px-6 py-4 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white rounded-lg font-medium transition-colors"
          >
            {loading ? 'Searching...' : 'Search'}
          </button>
          <button
            onClick={() => setShowFilters(!showFilters)}
            className={`p-4 border border-gray-700 rounded-lg transition-colors ${
              showFilters ? 'bg-blue-900/20 border-blue-600 text-blue-400' : 'text-gray-400 hover:border-gray-600'
            }`}
          >
            <Filter className="w-5 h-5" />
          </button>
        </div>

        {/* Advanced Filters */}
        {showFilters && (
          <div className="mt-4 p-4 bg-gray-800 border border-gray-700 rounded-lg space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {/* Search Mode */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Search Mode</label>
                <select
                  value={searchMode}
                  onChange={(e) => setSearchMode(e.target.value as any)}
                  className="w-full bg-gray-700 border border-gray-600 rounded-md px-3 py-2 text-white"
                >
                  <option value="Comprehensive">Comprehensive</option>
                  <option value="Relevance">Most Relevant</option>
                  <option value="Recent">Most Recent</option>
                  <option value="Popular">Most Popular</option>
                  <option value="Semantic">Semantic AI</option>
                </select>
              </div>

              {/* Platforms */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Platforms</label>
                <div className="flex flex-wrap gap-2">
                  {['jira', 'google', 'slack', 'github'].map(platform => (
                    <button
                      key={platform}
                      onClick={() => {
                        if (selectedPlatforms.includes(platform)) {
                          setSelectedPlatforms(prev => prev.filter(p => p !== platform));
                        } else {
                          setSelectedPlatforms(prev => [...prev, platform]);
                        }
                      }}
                      className={`px-3 py-1 rounded-md text-sm border transition-colors ${
                        selectedPlatforms.includes(platform)
                          ? 'text-blue-400 bg-blue-900/20 border-blue-600'
                          : 'text-gray-400 bg-gray-900/20 border-gray-700 hover:border-gray-600'
                      }`}
                    >
                      {getPlatformIcon(platform)} {platform}
                    </button>
                  ))}
                </div>
              </div>

              {/* Content Types */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Content Types</label>
                <div className="flex flex-wrap gap-2">
                  {['All', 'JiraIssue', 'GoogleDoc', 'SlackThread'].map(type => (
                    <button
                      key={type}
                      onClick={() => {
                        if (type === 'All') {
                          setSelectedContentTypes(['All']);
                        } else {
                          setSelectedContentTypes(prev => 
                            prev.includes('All') ? [type] : 
                            prev.includes(type) ? prev.filter(t => t !== type) : [...prev, type]
                          );
                        }
                      }}
                      className={`px-3 py-1 rounded-md text-sm border transition-colors ${
                        selectedContentTypes.includes(type)
                          ? 'text-blue-400 bg-blue-900/20 border-blue-600'
                          : 'text-gray-400 bg-gray-900/20 border-gray-700 hover:border-gray-600'
                      }`}
                    >
                      {type}
                    </button>
                  ))}
                </div>
              </div>
            </div>

            {/* Include Relationships */}
            <div className="flex items-center gap-3">
              <input
                type="checkbox"
                id="relationships"
                checked={includeRelationships}
                onChange={(e) => setIncludeRelationships(e.target.checked)}
                className="w-4 h-4 text-blue-600 bg-gray-700 border-gray-600 rounded focus:ring-blue-500"
              />
              <label htmlFor="relationships" className="text-gray-300">
                Include related content and cross-platform relationships
              </label>
            </div>
          </div>
        )}
      </div>

      {/* Error Display */}
      {error && (
        <div className="p-4 bg-red-900/20 border border-red-700 rounded-lg">
          <p className="text-red-400">❌ {error}</p>
        </div>
      )}

      {/* Results */}
      {results && (
        <div className="space-y-6">
          {/* Results Header */}
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              <span className="text-lg font-medium text-white">
                {results.total_results.toLocaleString()} results 
                <span className="text-gray-400 text-sm ml-2">
                  ({results.search_time_ms}ms)
                </span>
              </span>
              
              {/* Platform Tabs */}
              <div className="flex gap-1">
                {(['all', 'jira', 'google', 'slack'] as const).map(tab => {
                  const count = tab === 'all' 
                    ? results.total_results 
                    : results.facets?.platforms?.[tab] || 0;
                  
                  return (
                    <button
                      key={tab}
                      onClick={() => setActiveTab(tab)}
                      className={`px-3 py-1 rounded-md text-sm transition-colors ${
                        activeTab === tab
                          ? 'bg-blue-600 text-white'
                          : 'text-gray-400 hover:text-gray-300 hover:bg-gray-800'
                      }`}
                    >
                      {getPlatformIcon(tab)} {tab} ({count})
                    </button>
                  );
                })}
              </div>
            </div>

            {/* Search Suggestions */}
            {results.suggestions.length > 0 && (
              <div className="flex items-center gap-2">
                <span className="text-sm text-gray-400">Try:</span>
                {results.suggestions.slice(0, 3).map((suggestion, idx) => (
                  <button
                    key={idx}
                    onClick={() => setQuery(suggestion)}
                    className="text-xs px-2 py-1 bg-gray-800 border border-gray-700 rounded-md text-gray-300 hover:border-gray-600"
                  >
                    {suggestion}
                  </button>
                ))}
              </div>
            )}
          </div>

          {/* Results List */}
          <div className="space-y-4">
            {filteredResults.map((result) => (
              <div
                key={result.id}
                className="bg-gray-800 border border-gray-700 rounded-lg p-6 hover:border-gray-600 transition-colors"
              >
                {/* Result Header */}
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-start gap-3 flex-1">
                    <div className={`p-2 rounded-md border ${getPlatformColor(result.platform)}`}>
                      {getContentTypeIcon(result.result_type)}
                    </div>
                    
                    <div className="flex-1">
                      <div className="flex items-center gap-3 mb-2">
                        <h3 className="text-lg font-semibold text-white hover:text-blue-400 cursor-pointer">
                          {result.title}
                        </h3>
                        <div className={`px-2 py-1 rounded-md text-xs border ${getPlatformColor(result.platform)}`}>
                          {getPlatformIcon(result.platform)} {result.platform}
                        </div>
                        {result.context.parent_ticket && (
                          <span className="px-2 py-1 bg-blue-900/20 text-blue-400 text-xs rounded-md border border-blue-700">
                            📋 {result.context.parent_ticket}
                          </span>
                        )}
                      </div>
                      
                      <p className="text-gray-300 mb-3" 
                         dangerouslySetInnerHTML={{ __html: result.content_preview.replace(new RegExp(`(${query})`, 'gi'), '<mark class="bg-yellow-400 text-black px-1 rounded">$1</mark>') }} 
                      />
                      
                      <div className="flex items-center gap-4 text-sm text-gray-400">
                        <span className="flex items-center gap-1">
                          <Users className="w-4 h-4" />
                          {result.author || 'Unknown'}
                        </span>
                        <span className="flex items-center gap-1">
                          <Clock className="w-4 h-4" />
                          {new Date(result.last_updated).toLocaleDateString()}
                        </span>
                        <span className="flex items-center gap-1">
                          <TrendingUp className="w-4 h-4" />
                          {result.engagement_metrics.knowledge_score.toFixed(1)}/10
                        </span>
                        <span className="flex items-center gap-1">
                          <Star className="w-4 h-4" />
                          {Math.round(result.relevance_score * 100)}% match
                        </span>
                      </div>
                    </div>
                  </div>

                  {/* Access Status */}
                  <div className="flex items-center gap-2">
                    {result.access_info.requires_auth ? (
                      <div className="flex items-center gap-1 px-2 py-1 bg-yellow-900/20 text-yellow-400 text-xs rounded-md border border-yellow-700">
                        <Lock className="w-3 h-3" />
                        Auth Required
                      </div>
                    ) : (
                      <div className="flex items-center gap-1 px-2 py-1 bg-green-900/20 text-green-400 text-xs rounded-md border border-green-700">
                        <Unlock className="w-3 h-3" />
                        Accessible
                      </div>
                    )}
                    
                    <a
                      href={result.context.source_url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="p-2 text-gray-400 hover:text-gray-300 transition-colors"
                    >
                      <ExternalLink className="w-4 h-4" />
                    </a>
                  </div>
                </div>

                {/* Tags and Concepts */}
                {(result.tags.length > 0 || result.concepts.length > 0 || result.technologies.length > 0) && (
                  <div className="flex flex-wrap gap-2 mb-4">
                    {result.tags.map(tag => (
                      <span key={tag} className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded-md">
                        #{tag}
                      </span>
                    ))}
                    {result.concepts.map(concept => (
                      <span key={concept} className="px-2 py-1 bg-purple-900/20 text-purple-400 text-xs rounded-md border border-purple-700">
                        🧠 {concept}
                      </span>
                    ))}
                    {result.technologies.map(tech => (
                      <span key={tech} className="px-2 py-1 bg-orange-900/20 text-orange-400 text-xs rounded-md border border-orange-700">
                        ⚙️ {tech}
                      </span>
                    ))}
                  </div>
                )}

                {/* Engagement Metrics */}
                <div className="flex items-center gap-6 mb-4 text-sm">
                  <div className="flex items-center gap-1 text-gray-400">
                    <MessageSquare className="w-4 h-4" />
                    {result.engagement_metrics.comment_count} comments
                  </div>
                  <div className="flex items-center gap-1 text-gray-400">
                    <Users className="w-4 h-4" />
                    {result.engagement_metrics.view_count} views
                  </div>
                  <div className="flex items-center gap-1 text-gray-400">
                    <TrendingUp className="w-4 h-4" />
                    {result.engagement_metrics.reaction_count} reactions
                  </div>
                  <div className="flex items-center gap-1 text-gray-400">
                    <Search className="w-4 h-4" />
                    {result.engagement_metrics.search_hits} searches
                  </div>
                </div>

                {/* Related Items */}
                {result.related_items.length > 0 && (
                  <div className="border-t border-gray-700 pt-4">
                    <button
                      onClick={() => toggleResultExpansion(result.id)}
                      className="flex items-center gap-2 text-sm text-blue-400 hover:text-blue-300 mb-3"
                    >
                      <span>Related Content ({result.related_items.length})</span>
                      <span className={`transform transition-transform ${expandedResults.has(result.id) ? 'rotate-90' : ''}`}>
                        ▶
                      </span>
                    </button>
                    
                    {expandedResults.has(result.id) && (
                      <div className="space-y-2">
                        {result.related_items.map(relatedItem => (
                          <div key={relatedItem.id} className="flex items-center gap-3 p-2 bg-gray-900/50 rounded-md">
                            <span className="text-xs px-2 py-1 bg-blue-900/20 text-blue-400 rounded border border-blue-700">
                              {relatedItem.relationship_type}
                            </span>
                            <span className="text-gray-300 flex-1">{relatedItem.title}</span>
                            <span className="text-xs text-gray-500">
                              {Math.round(relatedItem.similarity_score * 100)}% match
                            </span>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>

          {/* Faceted Search Sidebar */}
          {results.facets && (
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mt-8">
              <div className="lg:col-span-3">
                {/* Results content is above */}
              </div>
              
              <div className="space-y-4">
                <h3 className="text-lg font-semibold text-white">Refine Search</h3>
                
                {/* Authors */}
                {Object.keys(results.facets.authors).length > 0 && (
                  <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h4 className="font-medium text-gray-300 mb-3">Authors</h4>
                    <div className="space-y-2">
                      {Object.entries(results.facets.authors).slice(0, 5).map(([author, count]) => (
                        <button
                          key={author}
                          className="flex items-center justify-between w-full text-left p-2 text-sm text-gray-300 hover:bg-gray-700 rounded-md"
                        >
                          <span>{author}</span>
                          <span className="text-gray-500">({count})</span>
                        </button>
                      ))}
                    </div>
                  </div>
                )}

                {/* Concepts */}
                {Object.keys(results.facets.concepts).length > 0 && (
                  <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h4 className="font-medium text-gray-300 mb-3">Concepts</h4>
                    <div className="space-y-2">
                      {Object.entries(results.facets.concepts).slice(0, 8).map(([concept, count]) => (
                        <button
                          key={concept}
                          className="flex items-center justify-between w-full text-left p-2 text-sm text-gray-300 hover:bg-gray-700 rounded-md"
                        >
                          <span>🧠 {concept}</span>
                          <span className="text-gray-500">({count})</span>
                        </button>
                      ))}
                    </div>
                  </div>
                )}

                {/* Technologies */}
                {Object.keys(results.facets.technologies).length > 0 && (
                  <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
                    <h4 className="font-medium text-gray-300 mb-3">Technologies</h4>
                    <div className="space-y-2">
                      {Object.entries(results.facets.technologies).slice(0, 8).map(([tech, count]) => (
                        <button
                          key={tech}
                          className="flex items-center justify-between w-full text-left p-2 text-sm text-gray-300 hover:bg-gray-700 rounded-md"
                        >
                          <span>⚙️ {tech}</span>
                          <span className="text-gray-500">({count})</span>
                        </button>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Empty State */}
      {!loading && !error && !results && (
        <div className="text-center py-12">
          <Globe className="w-16 h-16 text-gray-400 mx-auto mb-4" />
          <h3 className="text-xl font-semibold text-gray-300 mb-2">
            Search Across All Your Knowledge
          </h3>
          <p className="text-gray-400 mb-6">
            Find information from Jira tickets, Google Docs, Slack conversations, and more - all in one search
          </p>
          <div className="text-sm text-gray-500 space-y-1">
            <p>🔍 Try: "authentication service timeout"</p>
            <p>💬 Try: "payment processing discussion"</p>
            <p>📄 Try: "API design documentation"</p>
          </div>
        </div>
      )}
    </div>
  );
};

export default UniversalSearch;