import React, { useState, useEffect, useRef } from 'react';
import * as d3 from 'd3';
import { CalendarIcon, UserGroupIcon, LightBulbIcon, ChatBubbleLeftRightIcon, DocumentTextIcon } from '@heroicons/react/24/outline';

// ================================
// TYPE DEFINITIONS
// ================================

interface CollaborationEvent {
  id: string;
  timestamp: string;
  type: 'comment' | 'document_edit' | 'slack_message' | 'knowledge_transfer' | 'problem_resolution' | 'meeting' | 'review';
  platform: 'jira' | 'google' | 'slack' | 'other';
  participants: Participant[];
  contentId: string;
  title: string;
  description: string;
  impact: number; // 0-1 scale
  topics: string[];
  outcome?: 'resolved' | 'in_progress' | 'blocked' | 'deferred';
}

interface Participant {
  personId: string;
  displayName: string;
  role: 'leader' | 'contributor' | 'reviewer' | 'observer';
  contributionLevel: number; // 0-1 scale
}

interface TimelineFilter {
  dateRange: {
    start: Date;
    end: Date;
  };
  platforms: string[];
  eventTypes: string[];
  participants: string[];
  topics: string[];
  minImpact: number;
}

interface CollaborationTimelineProps {
  personId?: string;
  projectId?: string;
  topicFilter?: string;
  height?: number;
  onEventSelect?: (event: CollaborationEvent) => void;
}

// ================================
// COLLABORATION TIMELINE COMPONENT
// ================================

const CollaborationTimeline: React.FC<CollaborationTimelineProps> = ({
  personId,
  projectId,
  topicFilter,
  height = 400,
  onEventSelect
}) => {
  const svgRef = useRef<SVGSVGElement>(null);
  const [events, setEvents] = useState<CollaborationEvent[]>([]);
  const [filteredEvents, setFilteredEvents] = useState<CollaborationEvent[]>([]);
  const [filters, setFilters] = useState<TimelineFilter>({
    dateRange: {
      start: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000), // 30 days ago
      end: new Date()
    },
    platforms: ['jira', 'google', 'slack'],
    eventTypes: ['comment', 'document_edit', 'slack_message', 'knowledge_transfer', 'problem_resolution'],
    participants: [],
    topics: [],
    minImpact: 0
  });
  const [selectedEvent, setSelectedEvent] = useState<CollaborationEvent | null>(null);
  const [viewMode, setViewMode] = useState<'timeline' | 'calendar' | 'flow'>('timeline');
  const [isLoading, setIsLoading] = useState(true);
  const [zoomLevel, setZoomLevel] = useState(1);

  // Load collaboration events
  useEffect(() => {
    loadCollaborationEvents();
  }, [personId, projectId, topicFilter]);

  // Apply filters when they change
  useEffect(() => {
    applyFilters();
  }, [events, filters]);

  const loadCollaborationEvents = async () => {
    setIsLoading(true);
    try {
      // This would call the backend API to get collaboration events
      // For now, creating mock data
      const mockEvents: CollaborationEvent[] = generateMockEvents();
      setEvents(mockEvents);
    } catch (error) {
      console.error('Failed to load collaboration events:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const generateMockEvents = (): CollaborationEvent[] => {
    const now = new Date();
    const events: CollaborationEvent[] = [];
    
    // Generate sample events over the last 30 days
    for (let i = 0; i < 50; i++) {
      const daysAgo = Math.floor(Math.random() * 30);
      const timestamp = new Date(now.getTime() - daysAgo * 24 * 60 * 60 * 1000);
      
      events.push({
        id: `event-${i}`,
        timestamp: timestamp.toISOString(),
        type: ['comment', 'document_edit', 'slack_message', 'knowledge_transfer', 'problem_resolution'][Math.floor(Math.random() * 5)] as any,
        platform: ['jira', 'google', 'slack'][Math.floor(Math.random() * 3)] as any,
        participants: [
          {
            personId: `person-${Math.floor(Math.random() * 10)}`,
            displayName: ['Alice', 'Bob', 'Charlie', 'Diana', 'Eve'][Math.floor(Math.random() * 5)],
            role: ['leader', 'contributor', 'reviewer'][Math.floor(Math.random() * 3)] as any,
            contributionLevel: Math.random()
          }
        ],
        contentId: `content-${i}`,
        title: `Collaboration Event ${i + 1}`,
        description: `Description for event ${i + 1}`,
        impact: Math.random(),
        topics: ['React', 'API', 'Database', 'UI/UX', 'Performance'][Math.floor(Math.random() * 5)] ? ['React'] : [],
        outcome: Math.random() > 0.3 ? 'resolved' : 'in_progress'
      });
    }
    
    return events.sort((a, b) => new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime());
  };

  const applyFilters = () => {
    let filtered = events.filter(event => {
      const eventDate = new Date(event.timestamp);
      
      // Date range filter
      if (eventDate < filters.dateRange.start || eventDate > filters.dateRange.end) {
        return false;
      }
      
      // Platform filter
      if (filters.platforms.length > 0 && !filters.platforms.includes(event.platform)) {
        return false;
      }
      
      // Event type filter
      if (filters.eventTypes.length > 0 && !filters.eventTypes.includes(event.type)) {
        return false;
      }
      
      // Impact filter
      if (event.impact < filters.minImpact) {
        return false;
      }
      
      // Topic filter
      if (topicFilter && !event.topics.some(topic => 
        topic.toLowerCase().includes(topicFilter.toLowerCase())
      )) {
        return false;
      }
      
      return true;
    });
    
    setFilteredEvents(filtered);
  };

  // D3 Timeline Visualization
  useEffect(() => {
    if (!filteredEvents.length || !svgRef.current) return;

    const svg = d3.select(svgRef.current);
    const margin = { top: 20, right: 30, bottom: 40, left: 100 };
    const width = 800 - margin.left - margin.right;
    const timelineHeight = height - margin.top - margin.bottom;

    svg.selectAll('*').remove();

    const container = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    // Create scales
    const timeExtent = d3.extent(filteredEvents, d => new Date(d.timestamp)) as [Date, Date];
    const xScale = d3.scaleTime()
      .domain(timeExtent)
      .range([0, width]);

    const yScale = d3.scaleBand()
      .domain(filteredEvents.map(d => d.id))
      .range([0, timelineHeight])
      .padding(0.1);

    // Create axes
    const xAxis = d3.axisBottom(xScale)
      .tickFormat(d3.timeFormat('%m/%d'));
    
    container
      .append('g')
      .attr('transform', `translate(0,${timelineHeight})`)
      .call(xAxis);

    // Create timeline lanes for different platforms
    const platformLanes = d3.group(filteredEvents, d => d.platform);
    const platforms = Array.from(platformLanes.keys());
    
    const laneHeight = timelineHeight / platforms.length;
    const laneScale = d3.scaleBand()
      .domain(platforms)
      .range([0, timelineHeight])
      .padding(0.1);

    // Draw platform lanes
    platforms.forEach(platform => {
      container
        .append('rect')
        .attr('x', 0)
        .attr('y', laneScale(platform)!)
        .attr('width', width)
        .attr('height', laneScale.bandwidth())
        .attr('fill', getPlatformColor(platform))
        .attr('opacity', 0.1);

      container
        .append('text')
        .attr('x', -10)
        .attr('y', laneScale(platform)! + laneScale.bandwidth() / 2)
        .attr('text-anchor', 'end')
        .attr('dominant-baseline', 'middle')
        .attr('font-size', '12px')
        .attr('font-weight', 'bold')
        .attr('fill', getPlatformColor(platform))
        .text(platform.toUpperCase());
    });

    // Draw events
    const eventGroups = container
      .selectAll('.event')
      .data(filteredEvents)
      .join('g')
      .attr('class', 'event')
      .attr('transform', d => `translate(${xScale(new Date(d.timestamp))},${laneScale(d.platform)! + Math.random() * laneScale.bandwidth() * 0.8})`);

    // Event circles
    eventGroups
      .append('circle')
      .attr('r', d => 4 + d.impact * 6)
      .attr('fill', d => getEventColor(d.type))
      .attr('stroke', '#fff')
      .attr('stroke-width', 2)
      .attr('opacity', 0.8)
      .style('cursor', 'pointer');

    // Event tooltips and interactions
    eventGroups
      .on('mouseenter', function(event, d) {
        d3.select(this).select('circle')
          .transition()
          .duration(200)
          .attr('r', 4 + d.impact * 8)
          .attr('stroke-width', 3);

        showTooltip(event, d);
      })
      .on('mouseleave', function(event, d) {
        d3.select(this).select('circle')
          .transition()
          .duration(200)
          .attr('r', 4 + d.impact * 6)
          .attr('stroke-width', 2);

        hideTooltip();
      })
      .on('click', (event, d) => {
        setSelectedEvent(d);
        onEventSelect?.(d);
      });

    // Draw connections between related events
    drawEventConnections(container, filteredEvents, xScale, laneScale);

  }, [filteredEvents, height, viewMode]);

  const drawEventConnections = (container: any, events: CollaborationEvent[], xScale: any, laneScale: any) => {
    // Find related events (same participants, topics, or content)
    const connections: Array<[CollaborationEvent, CollaborationEvent]> = [];
    
    events.forEach((event1, i) => {
      events.slice(i + 1).forEach(event2 => {
        const hasCommonParticipants = event1.participants.some(p1 => 
          event2.participants.some(p2 => p1.personId === p2.personId)
        );
        const hasCommonTopics = event1.topics.some(t1 => 
          event2.topics.some(t2 => t1 === t2)
        );
        const isRelatedContent = event1.contentId === event2.contentId;
        
        if (hasCommonParticipants || hasCommonTopics || isRelatedContent) {
          connections.push([event1, event2]);
        }
      });
    });

    // Draw connection lines
    connections.forEach(([event1, event2]) => {
      const x1 = xScale(new Date(event1.timestamp));
      const y1 = laneScale(event1.platform)! + laneScale.bandwidth() / 2;
      const x2 = xScale(new Date(event2.timestamp));
      const y2 = laneScale(event2.platform)! + laneScale.bandwidth() / 2;

      container
        .append('line')
        .attr('x1', x1)
        .attr('y1', y1)
        .attr('x2', x2)
        .attr('y2', y2)
        .attr('stroke', '#e5e7eb')
        .attr('stroke-width', 1)
        .attr('stroke-dasharray', '2,2')
        .attr('opacity', 0.5);
    });
  };

  const showTooltip = (event: any, d: CollaborationEvent) => {
    // Implementation would show a tooltip with event details
    console.log('Show tooltip for event:', d);
  };

  const hideTooltip = () => {
    // Implementation would hide the tooltip
  };

  const getEventColor = (type: string): string => {
    switch (type) {
      case 'comment': return '#3b82f6';
      case 'document_edit': return '#10b981';
      case 'slack_message': return '#e11d48';
      case 'knowledge_transfer': return '#8b5cf6';
      case 'problem_resolution': return '#f59e0b';
      case 'meeting': return '#6b7280';
      case 'review': return '#06b6d4';
      default: return '#9ca3af';
    }
  };

  const getPlatformColor = (platform: string): string => {
    switch (platform) {
      case 'jira': return '#0052cc';
      case 'google': return '#0f9549';
      case 'slack': return '#e11d48';
      default: return '#6b7280';
    }
  };

  const getEventIcon = (type: string) => {
    switch (type) {
      case 'comment':
        return <ChatBubbleLeftRightIcon className="h-4 w-4" />;
      case 'document_edit':
        return <DocumentTextIcon className="h-4 w-4" />;
      case 'slack_message':
        return <ChatBubbleLeftRightIcon className="h-4 w-4" />;
      case 'knowledge_transfer':
        return <LightBulbIcon className="h-4 w-4" />;
      case 'problem_resolution':
        return <UserGroupIcon className="h-4 w-4" />;
      default:
        return <CalendarIcon className="h-4 w-4" />;
    }
  };

  const formatDate = (dateString: string): string => {
    return new Date(dateString).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <span className="ml-2 text-gray-600">Loading collaboration timeline...</span>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200">
      {/* Header */}
      <div className="border-b border-gray-200 p-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <CalendarIcon className="h-5 w-5 text-blue-600" />
            <h3 className="text-lg font-semibold text-gray-900">Collaboration Timeline</h3>
            <span className="text-sm text-gray-500">({filteredEvents.length} events)</span>
          </div>
          
          {/* View Mode Toggle */}
          <div className="flex space-x-1 bg-gray-100 rounded-lg p-1">
            <button
              onClick={() => setViewMode('timeline')}
              className={`px-3 py-1 text-sm rounded-md transition-colors ${
                viewMode === 'timeline' 
                  ? 'bg-white text-blue-600 shadow-sm' 
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              Timeline
            </button>
            <button
              onClick={() => setViewMode('calendar')}
              className={`px-3 py-1 text-sm rounded-md transition-colors ${
                viewMode === 'calendar' 
                  ? 'bg-white text-blue-600 shadow-sm' 
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              Calendar
            </button>
            <button
              onClick={() => setViewMode('flow')}
              className={`px-3 py-1 text-sm rounded-md transition-colors ${
                viewMode === 'flow' 
                  ? 'bg-white text-blue-600 shadow-sm' 
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              Flow
            </button>
          </div>
        </div>

        {/* Filter Controls */}
        <div className="flex items-center space-x-4 mt-3">
          <div className="flex items-center space-x-2">
            <label className="text-sm font-medium text-gray-700">Platforms:</label>
            {['jira', 'google', 'slack'].map(platform => (
              <label key={platform} className="flex items-center">
                <input
                  type="checkbox"
                  checked={filters.platforms.includes(platform)}
                  onChange={(e) => {
                    if (e.target.checked) {
                      setFilters(prev => ({
                        ...prev,
                        platforms: [...prev.platforms, platform]
                      }));
                    } else {
                      setFilters(prev => ({
                        ...prev,
                        platforms: prev.platforms.filter(p => p !== platform)
                      }));
                    }
                  }}
                  className="mr-1"
                />
                <span className="text-sm text-gray-600 capitalize">{platform}</span>
              </label>
            ))}
          </div>

          <div className="flex items-center space-x-2">
            <label className="text-sm font-medium text-gray-700">Min Impact:</label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.1"
              value={filters.minImpact}
              onChange={(e) => setFilters(prev => ({
                ...prev,
                minImpact: parseFloat(e.target.value)
              }))}
              className="w-20"
            />
            <span className="text-sm text-gray-600">{Math.round(filters.minImpact * 100)}%</span>
          </div>
        </div>
      </div>

      {/* Timeline Visualization */}
      <div className="p-4">
        {viewMode === 'timeline' && (
          <div className="relative">
            <svg
              ref={svgRef}
              width="800"
              height={height}
              className="border border-gray-100 rounded"
            />
            
            {/* Legend */}
            <div className="absolute top-4 right-4 bg-white border border-gray-200 rounded-lg p-3 shadow-sm">
              <h5 className="text-sm font-semibold text-gray-700 mb-2">Event Types</h5>
              <div className="space-y-1 text-xs">
                {['comment', 'document_edit', 'slack_message', 'knowledge_transfer', 'problem_resolution'].map(type => (
                  <div key={type} className="flex items-center">
                    <div
                      className="w-3 h-3 rounded-full mr-2"
                      style={{ backgroundColor: getEventColor(type) }}
                    ></div>
                    <span className="capitalize">{type.replace('_', ' ')}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* Event List View */}
        {viewMode === 'calendar' && (
          <div className="space-y-2 max-h-96 overflow-y-auto">
            {filteredEvents.map(event => (
              <div
                key={event.id}
                onClick={() => {
                  setSelectedEvent(event);
                  onEventSelect?.(event);
                }}
                className="flex items-start space-x-3 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer"
              >
                <div className="flex-shrink-0 mt-1">
                  <div
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: getEventColor(event.type) }}
                  ></div>
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center justify-between">
                    <h5 className="text-sm font-medium text-gray-900 truncate">{event.title}</h5>
                    <span className="text-xs text-gray-500">{formatDate(event.timestamp)}</span>
                  </div>
                  <p className="text-sm text-gray-600 mt-1">{event.description}</p>
                  <div className="flex items-center mt-2 space-x-2">
                    <span className={`text-xs px-2 py-1 rounded ${getPlatformColor(event.platform)} text-white`}>
                      {event.platform}
                    </span>
                    <span className="text-xs text-gray-500">
                      {event.participants.length} participant{event.participants.length !== 1 ? 's' : ''}
                    </span>
                    {event.impact > 0.7 && (
                      <span className="text-xs bg-yellow-100 text-yellow-800 px-2 py-1 rounded">
                        High Impact
                      </span>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Flow View - Simplified for now */}
        {viewMode === 'flow' && (
          <div className="text-center py-12 text-gray-500">
            <CalendarIcon className="mx-auto h-12 w-12 text-gray-400" />
            <p className="mt-4">Flow view coming soon</p>
            <p className="text-sm">Will show collaboration patterns and knowledge flow</p>
          </div>
        )}
      </div>

      {/* Selected Event Details */}
      {selectedEvent && (
        <div className="border-t border-gray-200 p-4 bg-gray-50">
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <h4 className="text-lg font-semibold text-gray-900">{selectedEvent.title}</h4>
              <p className="text-sm text-gray-600 mt-1">{selectedEvent.description}</p>
              <div className="flex items-center mt-3 space-x-4 text-sm text-gray-600">
                <span>Platform: <strong className="capitalize">{selectedEvent.platform}</strong></span>
                <span>Type: <strong className="capitalize">{selectedEvent.type.replace('_', ' ')}</strong></span>
                <span>Impact: <strong>{Math.round(selectedEvent.impact * 100)}%</strong></span>
                <span>Date: <strong>{formatDate(selectedEvent.timestamp)}</strong></span>
              </div>
              {selectedEvent.topics.length > 0 && (
                <div className="flex flex-wrap gap-1 mt-2">
                  {selectedEvent.topics.map(topic => (
                    <span key={topic} className="text-xs bg-blue-100 text-blue-600 px-2 py-1 rounded">
                      {topic}
                    </span>
                  ))}
                </div>
              )}
            </div>
            <button
              onClick={() => setSelectedEvent(null)}
              className="text-gray-400 hover:text-gray-600"
            >
              <svg className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default CollaborationTimeline;