import React, { useState, useEffect } from 'react';
import {
  UserGroupIcon,
  ChartBarIcon,
  MagnifyingGlassIcon,
  CalendarIcon,
  Cog6ToothIcon,
  ArrowPathIcon
} from '@heroicons/react/24/outline';

import PeopleNetworkGraph from './PeopleNetworkGraph';
import ExpertDiscovery from './ExpertDiscovery';
import CollaborationTimeline from './CollaborationTimeline';
import TeamInsightsDashboard from './TeamInsightsDashboard';

// ================================
// TYPE DEFINITIONS
// ================================

interface PeopleIntelligenceProps {
  className?: string;
}

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

interface CollaborationEvent {
  id: string;
  timestamp: string;
  type: 'comment' | 'document_edit' | 'slack_message' | 'knowledge_transfer' | 'problem_resolution';
  platform: 'jira' | 'google' | 'slack';
  participants: Array<{ personId: string; displayName: string; role: string; contributionLevel: number; }>;
  contentId: string;
  title: string;
  description: string;
  impact: number;
  topics: string[];
}

type ActiveTab = 'dashboard' | 'network' | 'discovery' | 'timeline' | 'settings';

// ================================
// PEOPLE INTELLIGENCE MAIN COMPONENT
// ================================

const PeopleIntelligence: React.FC<PeopleIntelligenceProps> = ({ className = '' }) => {
  const [activeTab, setActiveTab] = useState<ActiveTab>('dashboard');
  const [selectedPerson, setSelectedPerson] = useState<string | null>(null);
  const [selectedTopic, setSelectedTopic] = useState<string>('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [lastSync, setLastSync] = useState<Date | null>(null);
  const [networkData, setNetworkData] = useState(null);
  const [recentInsights, setRecentInsights] = useState<string[]>([]);

  // Load initial data
  useEffect(() => {
    loadPeopleIntelligence();
  }, []);

  const loadPeopleIntelligence = async () => {
    try {
      // Load network overview data
      const response = await fetch('/api/people/overview');
      const data = await response.json();
      
      // Update state with real data
      setLastSync(new Date());
      
      // Mock some recent insights
      setRecentInsights([
        "Sarah has become the top React expert this month",
        "Cross-platform collaboration up 25%",
        "New knowledge gap identified in Security",
        "DevOps team showing strong mentoring patterns"
      ]);
    } catch (error) {
      console.error('Failed to load people intelligence:', error);
    }
  };

  const handlePersonSelect = (personId: string) => {
    setSelectedPerson(personId);
    // Could switch to a detailed person view or highlight in network
  };

  const handleTopicSelect = (topic: string) => {
    setSelectedTopic(topic);
    // Could filter all views by this topic
  };

  const handleExpertSelect = (expert: Expert) => {
    setSelectedPerson(expert.personId);
    setSelectedTopic(expert.expertiseAreas[0]?.topic || '');
  };

  const handleEventSelect = (event: CollaborationEvent) => {
    // Could show detailed event information or related content
    console.log('Selected event:', event);
  };

  const handleProcessContent = async (platform: string, contentId: string) => {
    setIsProcessing(true);
    try {
      const response = await fetch('/api/people/analyze', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          platform,
          content_id: contentId,
          channel_id: platform === 'slack' ? 'general' : undefined
        })
      });
      
      const result = await response.json();
      
      if (result.success) {
        // Refresh data
        await loadPeopleIntelligence();
        setRecentInsights(prev => [
          `New insights from ${platform} content: ${result.insights.participants.length} people involved`,
          ...prev.slice(0, 3)
        ]);
      }
    } catch (error) {
      console.error('Failed to process content:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleDrillDown = (insight: string, data: any) => {
    // Handle drill-down actions from dashboard insights
    console.log('Drill down:', insight, data);
    
    if (insight === 'knowledge_gap') {
      setActiveTab('discovery');
      setSelectedTopic(data.topic);
    }
  };

  const tabs = [
    { id: 'dashboard' as const, name: 'Dashboard', icon: ChartBarIcon, description: 'Team insights and metrics' },
    { id: 'network' as const, name: 'Network', icon: UserGroupIcon, description: 'People collaboration graph' },
    { id: 'discovery' as const, name: 'Experts', icon: MagnifyingGlassIcon, description: 'Find and discover experts' },
    { id: 'timeline' as const, name: 'Timeline', icon: CalendarIcon, description: 'Collaboration history' },
    { id: 'settings' as const, name: 'Settings', icon: Cog6ToothIcon, description: 'Configure and process' },
  ];

  return (
    <div className={`bg-gray-50 min-h-screen ${className}`}>
      {/* Header */}
      <div className="bg-white border-b border-gray-200 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center space-x-3">
              <UserGroupIcon className="h-8 w-8 text-blue-600" />
              <div>
                <h1 className="text-xl font-bold text-gray-900">People Intelligence</h1>
                <p className="text-sm text-gray-600">Collaboration insights and team analytics</p>
              </div>
            </div>
            
            <div className="flex items-center space-x-4">
              {/* Sync Status */}
              {lastSync && (
                <div className="text-sm text-gray-600">
                  <span>Last sync: {lastSync.toLocaleTimeString()}</span>
                </div>
              )}
              
              {/* Refresh Button */}
              <button
                onClick={loadPeopleIntelligence}
                disabled={isProcessing}
                className="inline-flex items-center px-3 py-1 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50"
              >
                <ArrowPathIcon className={`h-4 w-4 mr-1 ${isProcessing ? 'animate-spin' : ''}`} />
                Refresh
              </button>
            </div>
          </div>

          {/* Recent Insights Banner */}
          {recentInsights.length > 0 && (
            <div className="pb-4">
              <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
                <div className="flex items-center">
                  <div className="flex-shrink-0">
                    <svg className="h-4 w-4 text-blue-400" fill="currentColor" viewBox="0 0 20 20">
                      <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
                    </svg>
                  </div>
                  <div className="ml-2">
                    <h4 className="text-sm font-medium text-blue-800">Recent Insights</h4>
                    <div className="text-sm text-blue-700">
                      {recentInsights[0]}
                      {recentInsights.length > 1 && (
                        <span className="text-blue-600"> +{recentInsights.length - 1} more</span>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* Navigation Tabs */}
          <div className="flex space-x-8">
            {tabs.map(tab => {
              const Icon = tab.icon;
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex items-center space-x-2 py-4 px-1 border-b-2 font-medium text-sm transition-colors ${
                    activeTab === tab.id
                      ? 'border-blue-500 text-blue-600'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  <Icon className="h-5 w-5" />
                  <span>{tab.name}</span>
                </button>
              );
            })}
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {activeTab === 'dashboard' && (
          <TeamInsightsDashboard
            timeRange="month"
            onDrillDown={handleDrillDown}
          />
        )}

        {activeTab === 'network' && (
          <div className="space-y-6">
            <div className="flex items-center justify-between">
              <div>
                <h2 className="text-lg font-semibold text-gray-900">People Network Graph</h2>
                <p className="text-gray-600">Interactive visualization of collaboration relationships</p>
              </div>
              
              {selectedTopic && (
                <div className="bg-blue-100 text-blue-800 px-3 py-1 rounded-lg text-sm">
                  Highlighting: {selectedTopic}
                </div>
              )}
            </div>
            
            <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
              <PeopleNetworkGraph
                data={networkData}
                selectedPersonId={selectedPerson}
                onPersonSelect={handlePersonSelect}
                highlightTopic={selectedTopic}
                width={800}
                height={600}
              />
            </div>
          </div>
        )}

        {activeTab === 'discovery' && (
          <div className="space-y-6">
            <div>
              <h2 className="text-lg font-semibold text-gray-900">Expert Discovery</h2>
              <p className="text-gray-600">Find experts and get collaboration recommendations</p>
            </div>
            
            <ExpertDiscovery
              onExpertSelect={handleExpertSelect}
              highlightTopic={selectedTopic}
            />
          </div>
        )}

        {activeTab === 'timeline' && (
          <div className="space-y-6">
            <div>
              <h2 className="text-lg font-semibold text-gray-900">Collaboration Timeline</h2>
              <p className="text-gray-600">Track collaboration events and knowledge transfer over time</p>
            </div>
            
            <CollaborationTimeline
              personId={selectedPerson}
              topicFilter={selectedTopic}
              onEventSelect={handleEventSelect}
              height={500}
            />
          </div>
        )}

        {activeTab === 'settings' && (
          <div className="space-y-6">
            <div>
              <h2 className="text-lg font-semibold text-gray-900">Settings & Processing</h2>
              <p className="text-gray-600">Configure people intelligence and process content</p>
            </div>

            {/* Content Processing */}
            <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
              <h3 className="text-lg font-semibold text-gray-900 mb-4">Process Content for People Insights</h3>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="text-center p-6 border border-gray-200 rounded-lg">
                  <div className="w-12 h-12 bg-blue-100 rounded-lg mx-auto mb-3 flex items-center justify-center">
                    <svg className="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                  </div>
                  <h4 className="font-medium text-gray-900">Jira Issues</h4>
                  <p className="text-sm text-gray-600 mt-1 mb-3">Extract collaboration data from issue comments and transitions</p>
                  <button
                    onClick={() => handleProcessContent('jira', 'PROJ-123')}
                    disabled={isProcessing}
                    className="w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
                  >
                    Process Jira Content
                  </button>
                </div>

                <div className="text-center p-6 border border-gray-200 rounded-lg">
                  <div className="w-12 h-12 bg-green-100 rounded-lg mx-auto mb-3 flex items-center justify-center">
                    <svg className="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                  </div>
                  <h4 className="font-medium text-gray-900">Google Docs</h4>
                  <p className="text-sm text-gray-600 mt-1 mb-3">Analyze document collaboration, comments, and suggestions</p>
                  <button
                    onClick={() => handleProcessContent('google', 'doc-id-123')}
                    disabled={isProcessing}
                    className="w-full px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:opacity-50"
                  >
                    Process Google Docs
                  </button>
                </div>

                <div className="text-center p-6 border border-gray-200 rounded-lg">
                  <div className="w-12 h-12 bg-red-100 rounded-lg mx-auto mb-3 flex items-center justify-center">
                    <svg className="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                    </svg>
                  </div>
                  <h4 className="font-medium text-gray-900">Slack Threads</h4>
                  <p className="text-sm text-gray-600 mt-1 mb-3">Extract thread dynamics and knowledge transfer events</p>
                  <button
                    onClick={() => handleProcessContent('slack', '1234567890.123456')}
                    disabled={isProcessing}
                    className="w-full px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50"
                  >
                    Process Slack Threads
                  </button>
                </div>
              </div>

              {isProcessing && (
                <div className="mt-4 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                  <div className="flex items-center">
                    <svg className="animate-spin h-5 w-5 text-yellow-600 mr-2" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    <span className="text-yellow-800">Processing content for people insights...</span>
                  </div>
                </div>
              )}
            </div>

            {/* System Status */}
            <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
              <h3 className="text-lg font-semibold text-gray-900 mb-4">System Status</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 text-sm">
                <div className="flex items-center justify-between p-3 bg-green-50 rounded-lg">
                  <span className="text-green-800">People Tracked</span>
                  <span className="font-medium text-green-900">24</span>
                </div>
                <div className="flex items-center justify-between p-3 bg-blue-50 rounded-lg">
                  <span className="text-blue-800">Interactions Analyzed</span>
                  <span className="font-medium text-blue-900">1,247</span>
                </div>
                <div className="flex items-center justify-between p-3 bg-purple-50 rounded-lg">
                  <span className="text-purple-800">Knowledge Transfers</span>
                  <span className="font-medium text-purple-900">156</span>
                </div>
                <div className="flex items-center justify-between p-3 bg-orange-50 rounded-lg">
                  <span className="text-orange-800">Active Platforms</span>
                  <span className="font-medium text-orange-900">3</span>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default PeopleIntelligence;