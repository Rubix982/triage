import React, { useState, useEffect } from 'react';
import {
  ChartBarIcon,
  UserGroupIcon,
  LightBulbIcon,
  TrendingUpIcon,
  ClockIcon,
  FireIcon,
  UsersIcon,
  ChatBubbleLeftRightIcon,
  DocumentTextIcon,
  ExclamationTriangleIcon
} from '@heroicons/react/24/outline';

// ================================
// TYPE DEFINITIONS
// ================================

interface TeamMetrics {
  totalPeople: number;
  activeCollaborators: number;
  crossPlatformUsers: number;
  knowledgeTransferEvents: number;
  avgResponseTime: number;
  collaborationScore: number;
  expertiseDistribution: ExpertiseDistribution[];
  platformActivity: PlatformActivity[];
  collaborationTrends: CollaborationTrend[];
  topPerformers: TopPerformer[];
  knowledgeGaps: KnowledgeGap[];
  teamNetworkMetrics: NetworkMetrics;
}

interface ExpertiseDistribution {
  topic: string;
  expertCount: number;
  totalContributions: number;
  avgExpertiseLevel: number;
  platforms: string[];
}

interface PlatformActivity {
  platform: string;
  activeUsers: number;
  interactions: number;
  knowledgeTransfers: number;
  responseTime: number;
  growth: number; // percentage change
}

interface CollaborationTrend {
  period: string;
  collaborations: number;
  knowledgeTransfers: number;
  problemResolutions: number;
  newConnections: number;
}

interface TopPerformer {
  personId: string;
  displayName: string;
  email: string;
  category: 'knowledge_sharing' | 'problem_solving' | 'collaboration' | 'mentoring';
  score: number;
  metrics: {
    contributions: number;
    helpedPeople: number;
    responsiveness: number;
    expertise: string[];
  };
  trend: 'up' | 'down' | 'stable';
}

interface KnowledgeGap {
  topic: string;
  demand: number; // how often it's asked about
  supply: number; // how many experts we have
  gapSeverity: 'low' | 'medium' | 'high' | 'critical';
  suggestedActions: string[];
  affectedProjects: string[];
}

interface NetworkMetrics {
  density: number; // how connected the network is
  centralizedNodes: number; // key people who connect many others
  isolatedNodes: number; // people with few connections
  averagePathLength: number; // degrees of separation
  clusteringCoefficient: number; // how clustered the network is
}

interface TeamInsightsDashboardProps {
  teamId?: string;
  projectId?: string;
  timeRange?: 'week' | 'month' | 'quarter' | 'year';
  onDrillDown?: (insight: string, data: any) => void;
}

// ================================
// TEAM INSIGHTS DASHBOARD COMPONENT
// ================================

const TeamInsightsDashboard: React.FC<TeamInsightsDashboardProps> = ({
  teamId,
  projectId,
  timeRange = 'month',
  onDrillDown
}) => {
  const [metrics, setMetrics] = useState<TeamMetrics | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedInsight, setSelectedInsight] = useState<string>('overview');
  const [refreshInterval, setRefreshInterval] = useState<number | null>(null);

  useEffect(() => {
    loadTeamMetrics();
    
    // Set up auto-refresh
    if (refreshInterval) {
      const interval = setInterval(loadTeamMetrics, refreshInterval * 1000);
      return () => clearInterval(interval);
    }
  }, [teamId, projectId, timeRange, refreshInterval]);

  const loadTeamMetrics = async () => {
    setIsLoading(true);
    try {
      // This would call the backend API to get team insights
      const response = await fetch(`/api/people/overview?timeRange=${timeRange}`);
      const data = await response.json();
      
      // For now, using mock data
      const mockMetrics: TeamMetrics = generateMockMetrics();
      setMetrics(mockMetrics);
    } catch (error) {
      console.error('Failed to load team metrics:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const generateMockMetrics = (): TeamMetrics => {
    return {
      totalPeople: 24,
      activeCollaborators: 18,
      crossPlatformUsers: 12,
      knowledgeTransferEvents: 156,
      avgResponseTime: 3.2,
      collaborationScore: 0.78,
      expertiseDistribution: [
        { topic: 'React', expertCount: 5, totalContributions: 89, avgExpertiseLevel: 0.8, platforms: ['jira', 'slack'] },
        { topic: 'API Design', expertCount: 3, totalContributions: 45, avgExpertiseLevel: 0.9, platforms: ['jira', 'google'] },
        { topic: 'Database', expertCount: 2, totalContributions: 23, avgExpertiseLevel: 0.7, platforms: ['slack', 'google'] },
        { topic: 'DevOps', expertCount: 4, totalContributions: 67, avgExpertiseLevel: 0.85, platforms: ['jira', 'slack', 'google'] },
      ],
      platformActivity: [
        { platform: 'slack', activeUsers: 18, interactions: 234, knowledgeTransfers: 45, responseTime: 2.1, growth: 15 },
        { platform: 'jira', activeUsers: 15, interactions: 123, knowledgeTransfers: 67, responseTime: 4.2, growth: -5 },
        { platform: 'google', activeUsers: 12, interactions: 89, knowledgeTransfers: 44, responseTime: 5.1, growth: 8 },
      ],
      collaborationTrends: [
        { period: 'Week 1', collaborations: 45, knowledgeTransfers: 12, problemResolutions: 8, newConnections: 3 },
        { period: 'Week 2', collaborations: 52, knowledgeTransfers: 18, problemResolutions: 11, newConnections: 5 },
        { period: 'Week 3', collaborations: 48, knowledgeTransfers: 15, problemResolutions: 9, newConnections: 2 },
        { period: 'Week 4', collaborations: 61, knowledgeTransfers: 22, problemResolutions: 14, newConnections: 7 },
      ],
      topPerformers: [
        {
          personId: 'sarah-smith',
          displayName: 'Sarah Smith',
          email: 'sarah.smith@company.com',
          category: 'knowledge_sharing',
          score: 0.92,
          metrics: { contributions: 34, helpedPeople: 12, responsiveness: 0.95, expertise: ['React', 'TypeScript'] },
          trend: 'up'
        },
        {
          personId: 'mike-johnson',
          displayName: 'Mike Johnson',
          email: 'mike.johnson@company.com',
          category: 'problem_solving',
          score: 0.88,
          metrics: { contributions: 28, helpedPeople: 15, responsiveness: 0.87, expertise: ['DevOps', 'API'] },
          trend: 'stable'
        }
      ],
      knowledgeGaps: [
        {
          topic: 'Security',
          demand: 23,
          supply: 1,
          gapSeverity: 'critical',
          suggestedActions: ['Hire security expert', 'Security training program'],
          affectedProjects: ['Project A', 'Project B']
        },
        {
          topic: 'Mobile Development',
          demand: 12,
          supply: 2,
          gapSeverity: 'medium',
          suggestedActions: ['Mobile development workshop', 'External contractor'],
          affectedProjects: ['Mobile App']
        }
      ],
      teamNetworkMetrics: {
        density: 0.67,
        centralizedNodes: 5,
        isolatedNodes: 2,
        averagePathLength: 2.3,
        clusteringCoefficient: 0.74
      }
    };
  };

  const getGapSeverityColor = (severity: string): string => {
    switch (severity) {
      case 'critical': return 'bg-red-100 text-red-800';
      case 'high': return 'bg-orange-100 text-orange-800';
      case 'medium': return 'bg-yellow-100 text-yellow-800';
      case 'low': return 'bg-green-100 text-green-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getTrendIcon = (trend: string) => {
    switch (trend) {
      case 'up': return <TrendingUpIcon className="h-4 w-4 text-green-500" />;
      case 'down': return <TrendingUpIcon className="h-4 w-4 text-red-500 rotate-180" />;
      case 'stable': return <div className="h-4 w-4 bg-gray-400 rounded-full" />;
      default: return null;
    }
  };

  const getCategoryColor = (category: string): string => {
    switch (category) {
      case 'knowledge_sharing': return 'bg-purple-100 text-purple-800';
      case 'problem_solving': return 'bg-blue-100 text-blue-800';
      case 'collaboration': return 'bg-green-100 text-green-800';
      case 'mentoring': return 'bg-yellow-100 text-yellow-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <span className="ml-2 text-gray-600">Loading team insights...</span>
      </div>
    );
  }

  if (!metrics) {
    return (
      <div className="text-center py-12 text-gray-500">
        <ChartBarIcon className="mx-auto h-12 w-12 text-gray-400" />
        <p className="mt-4">No team data available</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <ChartBarIcon className="h-8 w-8 text-blue-600" />
          <div>
            <h2 className="text-2xl font-bold text-gray-900">Team Insights Dashboard</h2>
            <p className="text-gray-600">Collaboration intelligence and team performance analytics</p>
          </div>
        </div>
        
        {/* Time Range Selector */}
        <div className="flex space-x-2">
          {(['week', 'month', 'quarter', 'year'] as const).map(range => (
            <button
              key={range}
              onClick={() => {/* Update time range */}}
              className={`px-3 py-1 text-sm rounded-md capitalize transition-colors ${
                timeRange === range 
                  ? 'bg-blue-600 text-white' 
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
            >
              {range}
            </button>
          ))}
        </div>
      </div>

      {/* Key Metrics Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <MetricCard
          title="Total People"
          value={metrics.totalPeople}
          icon={<UsersIcon className="h-6 w-6" />}
          trend={12}
          color="blue"
        />
        <MetricCard
          title="Active Collaborators"
          value={metrics.activeCollaborators}
          icon={<UserGroupIcon className="h-6 w-6" />}
          trend={8}
          color="green"
          subtitle={`${Math.round((metrics.activeCollaborators / metrics.totalPeople) * 100)}% of team`}
        />
        <MetricCard
          title="Knowledge Transfers"
          value={metrics.knowledgeTransferEvents}
          icon={<LightBulbIcon className="h-6 w-6" />}
          trend={23}
          color="purple"
        />
        <MetricCard
          title="Avg Response Time"
          value={`${metrics.avgResponseTime}h`}
          icon={<ClockIcon className="h-6 w-6" />}
          trend={-15}
          color="orange"
          subtitle="Faster this month"
        />
      </div>

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Platform Activity */}
        <div className="lg:col-span-2">
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Platform Activity</h3>
            <div className="space-y-4">
              {metrics.platformActivity.map(platform => (
                <div key={platform.platform} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className={`w-3 h-3 rounded-full ${getPlatformIndicatorColor(platform.platform)}`} />
                    <div>
                      <h4 className="font-medium text-gray-900 capitalize">{platform.platform}</h4>
                      <p className="text-sm text-gray-600">{platform.activeUsers} active users</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="flex items-center space-x-2">
                      <span className="text-sm font-medium">{platform.interactions} interactions</span>
                      <span className={`text-xs px-2 py-1 rounded ${
                        platform.growth > 0 ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                      }`}>
                        {platform.growth > 0 ? '+' : ''}{platform.growth}%
                      </span>
                    </div>
                    <p className="text-xs text-gray-500 mt-1">{platform.responseTime}h avg response</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Network Health */}
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">Network Health</h3>
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <span className="text-sm text-gray-600">Network Density</span>
              <span className="font-medium">{Math.round(metrics.teamNetworkMetrics.density * 100)}%</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div 
                className="bg-blue-600 h-2 rounded-full" 
                style={{ width: `${metrics.teamNetworkMetrics.density * 100}%` }}
              />
            </div>
            
            <div className="grid grid-cols-2 gap-4 pt-2 text-sm">
              <div>
                <span className="text-gray-600">Key Connectors</span>
                <p className="font-medium">{metrics.teamNetworkMetrics.centralizedNodes}</p>
              </div>
              <div>
                <span className="text-gray-600">Isolated Members</span>
                <p className="font-medium">{metrics.teamNetworkMetrics.isolatedNodes}</p>
              </div>
              <div>
                <span className="text-gray-600">Avg Path Length</span>
                <p className="font-medium">{metrics.teamNetworkMetrics.averagePathLength}</p>
              </div>
              <div>
                <span className="text-gray-600">Clustering</span>
                <p className="font-medium">{Math.round(metrics.teamNetworkMetrics.clusteringCoefficient * 100)}%</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Top Performers */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Top Performers</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {metrics.topPerformers.map(performer => (
            <div key={performer.personId} className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-2">
                    <h4 className="font-medium text-gray-900">{performer.displayName}</h4>
                    {getTrendIcon(performer.trend)}
                  </div>
                  <p className="text-sm text-gray-600">{performer.email}</p>
                  <span className={`text-xs px-2 py-1 rounded mt-2 inline-block ${getCategoryColor(performer.category)}`}>
                    {performer.category.replace('_', ' ')}
                  </span>
                </div>
                <div className="text-right">
                  <span className="text-2xl font-bold text-blue-600">{Math.round(performer.score * 100)}</span>
                  <p className="text-xs text-gray-500">Performance Score</p>
                </div>
              </div>
              <div className="grid grid-cols-2 gap-2 mt-3 text-xs text-gray-600">
                <div>Contributions: <strong>{performer.metrics.contributions}</strong></div>
                <div>Helped: <strong>{performer.metrics.helpedPeople} people</strong></div>
                <div>Response Rate: <strong>{Math.round(performer.metrics.responsiveness * 100)}%</strong></div>
                <div>Expertise: <strong>{performer.metrics.expertise.length} areas</strong></div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Knowledge Gaps */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-gray-900">Knowledge Gaps</h3>
          <ExclamationTriangleIcon className="h-5 w-5 text-orange-500" />
        </div>
        <div className="space-y-4">
          {metrics.knowledgeGaps.map((gap, index) => (
            <div key={index} className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-2">
                    <h4 className="font-medium text-gray-900">{gap.topic}</h4>
                    <span className={`text-xs px-2 py-1 rounded ${getGapSeverityColor(gap.gapSeverity)}`}>
                      {gap.gapSeverity}
                    </span>
                  </div>
                  <div className="flex items-center mt-2 text-sm text-gray-600">
                    <span>Demand: <strong>{gap.demand}</strong></span>
                    <span className="mx-2">•</span>
                    <span>Supply: <strong>{gap.supply} expert{gap.supply !== 1 ? 's' : ''}</strong></span>
                  </div>
                  <div className="mt-2">
                    <p className="text-xs text-gray-500 mb-1">Suggested Actions:</p>
                    <ul className="list-disc list-inside text-xs text-gray-600">
                      {gap.suggestedActions.map((action, i) => (
                        <li key={i}>{action}</li>
                      ))}
                    </ul>
                  </div>
                </div>
                <button
                  onClick={() => onDrillDown?.('knowledge_gap', gap)}
                  className="px-3 py-1 text-xs bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  Address
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Expertise Distribution */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">Expertise Distribution</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {metrics.expertiseDistribution.map(expertise => (
            <div key={expertise.topic} className="text-center p-4 bg-gray-50 rounded-lg">
              <h4 className="font-medium text-gray-900">{expertise.topic}</h4>
              <div className="mt-2">
                <span className="text-2xl font-bold text-blue-600">{expertise.expertCount}</span>
                <p className="text-xs text-gray-500">experts</p>
              </div>
              <div className="mt-2 text-xs text-gray-600">
                <div>{expertise.totalContributions} contributions</div>
                <div>{Math.round(expertise.avgExpertiseLevel * 100)}% avg level</div>
              </div>
              <div className="flex justify-center space-x-1 mt-2">
                {expertise.platforms.map(platform => (
                  <div
                    key={platform}
                    className={`w-2 h-2 rounded-full ${getPlatformIndicatorColor(platform)}`}
                    title={platform}
                  />
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

// ================================
// HELPER COMPONENTS
// ================================

interface MetricCardProps {
  title: string;
  value: string | number;
  icon: React.ReactNode;
  trend?: number;
  color: 'blue' | 'green' | 'purple' | 'orange' | 'red';
  subtitle?: string;
}

const MetricCard: React.FC<MetricCardProps> = ({ title, value, icon, trend, color, subtitle }) => {
  const colorClasses = {
    blue: 'bg-blue-50 text-blue-600',
    green: 'bg-green-50 text-green-600',
    purple: 'bg-purple-50 text-purple-600',
    orange: 'bg-orange-50 text-orange-600',
    red: 'bg-red-50 text-red-600'
  };

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
      <div className="flex items-center justify-between">
        <div className={`p-2 rounded-lg ${colorClasses[color]}`}>
          {icon}
        </div>
        {trend !== undefined && (
          <span className={`text-xs px-2 py-1 rounded ${
            trend > 0 ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
          }`}>
            {trend > 0 ? '+' : ''}{trend}%
          </span>
        )}
      </div>
      <div className="mt-4">
        <p className="text-2xl font-bold text-gray-900">{value}</p>
        <p className="text-sm text-gray-600">{title}</p>
        {subtitle && <p className="text-xs text-gray-500 mt-1">{subtitle}</p>}
      </div>
    </div>
  );
};

// Helper functions
const getPlatformIndicatorColor = (platform: string): string => {
  switch (platform) {
    case 'slack': return 'bg-red-500';
    case 'google': return 'bg-green-500';
    case 'jira': return 'bg-blue-500';
    default: return 'bg-gray-500';
  }
};

export default TeamInsightsDashboard;