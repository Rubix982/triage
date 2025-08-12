import React, { useState, useEffect } from 'react';
import { MagnifyingGlassIcon, UserGroupIcon, LightBulbIcon, ChatBubbleLeftRightIcon } from '@heroicons/react/24/outline';

// ================================
// TYPE DEFINITIONS
// ================================

interface Expert {
  personId: string;
  displayName: string;
  email?: string;
  platforms: string[];
  expertiseAreas: ExpertiseArea[];
  influenceMetrics: InfluenceMetrics;
  recentActivity: RecentActivity[];
  collaborationPartners: string[];
}

interface ExpertiseArea {
  topic: string;
  confidenceScore: number;
  evidenceCount: number;
  platforms: string[];
}

interface InfluenceMetrics {
  authorityScore: number;
  collaborationFrequency: number;
  knowledgeSharingScore: number;
  problemSolvingRate: number;
}

interface RecentActivity {
  platform: string;
  contentId: string;
  type: 'solution' | 'teaching' | 'collaboration' | 'problem_solving';
  topic: string;
  timestamp: string;
  impact: number;
}

interface CollaborationRecommendation {
  recommendedPersonId: string;
  displayName: string;
  confidenceScore: number;
  sharedTopics: string[];
  collaborationHistory: number;
  reasoning: string;
}

interface ExpertDiscoveryProps {
  onExpertSelect?: (expert: Expert) => void;
  highlightTopic?: string;
}

// ================================
// EXPERT DISCOVERY COMPONENT
// ================================

const ExpertDiscovery: React.FC<ExpertDiscoveryProps> = ({
  onExpertSelect,
  highlightTopic
}) => {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedTopic, setSelectedTopic] = useState(highlightTopic || '');
  const [searchResults, setSearchResults] = useState<Expert[]>([]);
  const [topExperts, setTopExperts] = useState<Expert[]>([]);
  const [recommendations, setRecommendations] = useState<CollaborationRecommendation[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [activeTab, setActiveTab] = useState<'search' | 'experts' | 'recommendations'>('search');
  const [availableTopics, setAvailableTopics] = useState<string[]>([]);

  // Load initial data
  useEffect(() => {
    loadTopExperts();
    loadAvailableTopics();
    if (highlightTopic) {
      setSelectedTopic(highlightTopic);
      searchExpertsByTopic(highlightTopic);
    }
  }, [highlightTopic]);

  // Search when term or topic changes
  useEffect(() => {
    if (searchTerm.length > 2 || selectedTopic) {
      performSearch();
    } else {
      setSearchResults([]);
    }
  }, [searchTerm, selectedTopic]);

  const loadTopExperts = async () => {
    try {
      // This would call the backend API to get top experts
      // For now, using mock data structure
      const mockExperts: Expert[] = [];
      setTopExperts(mockExperts);
    } catch (error) {
      console.error('Failed to load top experts:', error);
    }
  };

  const loadAvailableTopics = async () => {
    try {
      // This would call the backend to get all available expertise topics
      const mockTopics = [
        'React', 'TypeScript', 'Node.js', 'Database Design', 'API Design',
        'DevOps', 'Testing', 'Performance Optimization', 'Security', 'UI/UX'
      ];
      setAvailableTopics(mockTopics);
    } catch (error) {
      console.error('Failed to load topics:', error);
    }
  };

  const performSearch = async () => {
    if (!searchTerm && !selectedTopic) return;
    
    setIsSearching(true);
    try {
      // This would call the backend search API
      // For now, creating mock search results
      const mockResults: Expert[] = [];
      setSearchResults(mockResults);
    } catch (error) {
      console.error('Search failed:', error);
    } finally {
      setIsSearching(false);
    }
  };

  const searchExpertsByTopic = async (topic: string) => {
    try {
      // Call backend API to search experts by topic
      const response = await fetch(`/api/people/search?topic=${encodeURIComponent(topic)}`);
      const data = await response.json();
      setSearchResults(data.experts || []);
    } catch (error) {
      console.error('Topic search failed:', error);
    }
  };

  const getRecommendations = async (personId: string, topic?: string) => {
    try {
      const url = `/api/people/recommendations/${personId}${topic ? `?topic=${encodeURIComponent(topic)}` : ''}`;
      const response = await fetch(url);
      const data = await response.json();
      setRecommendations(data.recommendations || []);
      setActiveTab('recommendations');
    } catch (error) {
      console.error('Failed to get recommendations:', error);
    }
  };

  const handleExpertClick = (expert: Expert) => {
    onExpertSelect?.(expert);
    getRecommendations(expert.personId, selectedTopic);
  };

  const getExpertiseLevel = (score: number): string => {
    if (score >= 0.8) return 'Expert';
    if (score >= 0.6) return 'Advanced';
    if (score >= 0.4) return 'Intermediate';
    return 'Beginner';
  };

  const getExpertiseLevelColor = (score: number): string => {
    if (score >= 0.8) return 'bg-purple-100 text-purple-800';
    if (score >= 0.6) return 'bg-blue-100 text-blue-800';
    if (score >= 0.4) return 'bg-green-100 text-green-800';
    return 'bg-gray-100 text-gray-800';
  };

  const getPlatformColor = (platform: string): string => {
    switch (platform.toLowerCase()) {
      case 'slack': return 'bg-red-100 text-red-800';
      case 'google': return 'bg-green-100 text-green-800';
      case 'jira': return 'bg-blue-100 text-blue-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const formatTimestamp = (timestamp: string): string => {
    return new Date(timestamp).toLocaleDateString();
  };

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200">
      {/* Header */}
      <div className="border-b border-gray-200 p-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <UserGroupIcon className="h-5 w-5 text-blue-600" />
            <h3 className="text-lg font-semibold text-gray-900">Expert Discovery</h3>
          </div>
          
          {/* Tab Navigation */}
          <div className="flex space-x-1 bg-gray-100 rounded-lg p-1">
            <button
              onClick={() => setActiveTab('search')}
              className={`px-3 py-1 text-sm rounded-md transition-colors ${
                activeTab === 'search' 
                  ? 'bg-white text-blue-600 shadow-sm' 
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              Search
            </button>
            <button
              onClick={() => setActiveTab('experts')}
              className={`px-3 py-1 text-sm rounded-md transition-colors ${
                activeTab === 'experts' 
                  ? 'bg-white text-blue-600 shadow-sm' 
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              Top Experts
            </button>
            <button
              onClick={() => setActiveTab('recommendations')}
              className={`px-3 py-1 text-sm rounded-md transition-colors ${
                activeTab === 'recommendations' 
                  ? 'bg-white text-blue-600 shadow-sm' 
                  : 'text-gray-600 hover:text-gray-800'
              }`}
              disabled={recommendations.length === 0}
            >
              Recommendations {recommendations.length > 0 && `(${recommendations.length})`}
            </button>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="p-4">
        {activeTab === 'search' && (
          <div className="space-y-4">
            {/* Search Controls */}
            <div className="space-y-3">
              {/* Search Input */}
              <div className="relative">
                <MagnifyingGlassIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
                <input
                  type="text"
                  placeholder="Search for experts by name, skill, or topic..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>

              {/* Topic Filter */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Filter by Expertise Area
                </label>
                <select
                  value={selectedTopic}
                  onChange={(e) => setSelectedTopic(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                  <option value="">All Topics</option>
                  {availableTopics.map(topic => (
                    <option key={topic} value={topic}>{topic}</option>
                  ))}
                </select>
              </div>
            </div>

            {/* Search Results */}
            <div>
              {isSearching ? (
                <div className="flex items-center justify-center py-8">
                  <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
                  <span className="ml-2 text-gray-600">Searching experts...</span>
                </div>
              ) : searchResults.length > 0 ? (
                <div className="space-y-3">
                  <h4 className="text-sm font-medium text-gray-700">
                    Found {searchResults.length} expert{searchResults.length !== 1 ? 's' : ''}
                  </h4>
                  {searchResults.map(expert => (
                    <ExpertCard key={expert.personId} expert={expert} onClick={handleExpertClick} />
                  ))}
                </div>
              ) : (searchTerm.length > 2 || selectedTopic) ? (
                <div className="text-center py-8 text-gray-500">
                  <LightBulbIcon className="mx-auto h-8 w-8 text-gray-400" />
                  <p className="mt-2">No experts found for your search</p>
                  <p className="text-sm">Try different keywords or topics</p>
                </div>
              ) : (
                <div className="text-center py-8 text-gray-500">
                  <MagnifyingGlassIcon className="mx-auto h-8 w-8 text-gray-400" />
                  <p className="mt-2">Start typing to search for experts</p>
                </div>
              )}
            </div>
          </div>
        )}

        {activeTab === 'experts' && (
          <div className="space-y-4">
            <h4 className="text-sm font-medium text-gray-700">Top Experts by Influence</h4>
            {topExperts.length > 0 ? (
              <div className="space-y-3">
                {topExperts.map(expert => (
                  <ExpertCard key={expert.personId} expert={expert} onClick={handleExpertClick} />
                ))}
              </div>
            ) : (
              <div className="text-center py-8 text-gray-500">
                <UserGroupIcon className="mx-auto h-8 w-8 text-gray-400" />
                <p className="mt-2">No expert data available yet</p>
                <p className="text-sm">Process some content to discover experts</p>
              </div>
            )}
          </div>
        )}

        {activeTab === 'recommendations' && (
          <div className="space-y-4">
            <h4 className="text-sm font-medium text-gray-700">Collaboration Recommendations</h4>
            {recommendations.length > 0 ? (
              <div className="space-y-3">
                {recommendations.map((rec, index) => (
                  <div key={index} className="border border-gray-200 rounded-lg p-3 hover:bg-gray-50">
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <h5 className="font-medium text-gray-900">{rec.displayName}</h5>
                        <p className="text-sm text-gray-600 mt-1">{rec.reasoning}</p>
                        <div className="flex items-center mt-2 space-x-2">
                          <span className="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded">
                            {Math.round(rec.confidenceScore * 100)}% match
                          </span>
                          <span className="text-xs text-gray-500">
                            {rec.collaborationHistory} past collaborations
                          </span>
                        </div>
                        {rec.sharedTopics.length > 0 && (
                          <div className="flex flex-wrap gap-1 mt-2">
                            {rec.sharedTopics.map(topic => (
                              <span key={topic} className="text-xs bg-gray-100 text-gray-600 px-2 py-1 rounded">
                                {topic}
                              </span>
                            ))}
                          </div>
                        )}
                      </div>
                      <button
                        onClick={() => {/* Handle connect action */}}
                        className="ml-3 px-3 py-1 text-xs bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                      >
                        Connect
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-center py-8 text-gray-500">
                <ChatBubbleLeftRightIcon className="mx-auto h-8 w-8 text-gray-400" />
                <p className="mt-2">No recommendations yet</p>
                <p className="text-sm">Click on an expert to get collaboration recommendations</p>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

// ================================
// EXPERT CARD COMPONENT
// ================================

interface ExpertCardProps {
  expert: Expert;
  onClick: (expert: Expert) => void;
}

const ExpertCard: React.FC<ExpertCardProps> = ({ expert, onClick }) => {
  return (
    <div 
      onClick={() => onClick(expert)}
      className="border border-gray-200 rounded-lg p-4 hover:bg-gray-50 cursor-pointer transition-colors"
    >
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center space-x-2">
            <h5 className="font-medium text-gray-900">{expert.displayName}</h5>
            <span className={`text-xs px-2 py-1 rounded ${getExpertiseLevelColor(expert.influenceMetrics.authorityScore)}`}>
              {getExpertiseLevel(expert.influenceMetrics.authorityScore)}
            </span>
          </div>
          
          {expert.email && (
            <p className="text-sm text-gray-600 mt-1">{expert.email}</p>
          )}

          {/* Expertise Areas */}
          {expert.expertiseAreas.length > 0 && (
            <div className="flex flex-wrap gap-1 mt-2">
              {expert.expertiseAreas.slice(0, 4).map(area => (
                <span key={area.topic} className="text-xs bg-blue-100 text-blue-600 px-2 py-1 rounded">
                  {area.topic}
                </span>
              ))}
              {expert.expertiseAreas.length > 4 && (
                <span className="text-xs text-gray-500">+{expert.expertiseAreas.length - 4} more</span>
              )}
            </div>
          )}

          {/* Platforms */}
          <div className="flex items-center space-x-2 mt-2">
            <span className="text-xs text-gray-500">Active on:</span>
            {expert.platforms.map(platform => (
              <span key={platform} className={`text-xs px-2 py-1 rounded ${getPlatformColor(platform)}`}>
                {platform}
              </span>
            ))}
          </div>

          {/* Metrics */}
          <div className="grid grid-cols-2 gap-4 mt-3 text-xs text-gray-600">
            <div>
              <span className="font-medium">Authority:</span> {Math.round(expert.influenceMetrics.authorityScore * 100)}%
            </div>
            <div>
              <span className="font-medium">Knowledge Sharing:</span> {Math.round(expert.influenceMetrics.knowledgeSharingScore * 100)}%
            </div>
            <div>
              <span className="font-medium">Problem Solving:</span> {Math.round(expert.influenceMetrics.problemSolvingRate * 100)}%
            </div>
            <div>
              <span className="font-medium">Collaborations:</span> {expert.collaborationPartners.length}
            </div>
          </div>
        </div>
        
        {/* Recent Activity Indicator */}
        {expert.recentActivity.length > 0 && (
          <div className="ml-4 text-right">
            <div className="w-2 h-2 bg-green-500 rounded-full"></div>
            <span className="text-xs text-gray-500 mt-1 block">Active</span>
          </div>
        )}
      </div>
    </div>
  );
};

// Helper functions (moved outside component to avoid recreating)
const getExpertiseLevel = (score: number): string => {
  if (score >= 0.8) return 'Expert';
  if (score >= 0.6) return 'Advanced';
  if (score >= 0.4) return 'Intermediate';
  return 'Beginner';
};

const getExpertiseLevelColor = (score: number): string => {
  if (score >= 0.8) return 'bg-purple-100 text-purple-800';
  if (score >= 0.6) return 'bg-blue-100 text-blue-800';
  if (score >= 0.4) return 'bg-green-100 text-green-800';
  return 'bg-gray-100 text-gray-800';
};

const getPlatformColor = (platform: string): string => {
  switch (platform.toLowerCase()) {
    case 'slack': return 'bg-red-100 text-red-800';
    case 'google': return 'bg-green-100 text-green-800';
    case 'jira': return 'bg-blue-100 text-blue-800';
    default: return 'bg-gray-100 text-gray-800';
  }
};

export default ExpertDiscovery;