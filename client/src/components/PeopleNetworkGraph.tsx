import React, { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';

// ================================
// TYPE DEFINITIONS
// ================================

interface Person {
  id: string;
  displayName: string;
  email?: string;
  platforms: string[];
  expertiseAreas: ExpertiseArea[];
  influenceMetrics: InfluenceMetrics;
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

interface CollaborationEdge {
  source: string;
  target: string;
  weight: number;
  platforms: string[];
  interactions: number;
  topics: string[];
}

interface NetworkData {
  nodes: Person[];
  links: CollaborationEdge[];
}

interface PeopleNetworkGraphProps {
  data?: NetworkData;
  selectedPersonId?: string;
  onPersonSelect?: (personId: string) => void;
  onPersonHover?: (person: Person | null) => void;
  width?: number;
  height?: number;
  highlightTopic?: string;
}

// ================================
// PEOPLE NETWORK GRAPH COMPONENT
// ================================

const PeopleNetworkGraph: React.FC<PeopleNetworkGraphProps> = ({
  data,
  selectedPersonId,
  onPersonSelect,
  onPersonHover,
  width = 800,
  height = 600,
  highlightTopic
}) => {
  const svgRef = useRef<SVGSVGElement>(null);
  const [hoveredPerson, setHoveredPerson] = useState<Person | null>(null);
  const [isLoading, setIsLoading] = useState(!data);

  // Load network data if not provided
  useEffect(() => {
    if (!data) {
      loadNetworkData();
    }
  }, [data]);

  const loadNetworkData = async () => {
    try {
      setIsLoading(true);
      const response = await fetch('/api/people/overview');
      const networkData = await response.json();
      // Transform backend data to our format
      // This is a placeholder - would need actual data transformation
    } catch (error) {
      console.error('Failed to load network data:', error);
    } finally {
      setIsLoading(false);
    }
  };

  // Initialize D3 force simulation
  useEffect(() => {
    if (!data || !svgRef.current) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll('*').remove();

    // Create container groups
    const container = svg.append('g').attr('class', 'network-container');
    const linksGroup = container.append('g').attr('class', 'links');
    const nodesGroup = container.append('g').attr('class', 'nodes');

    // Set up zoom behavior
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => {
        container.attr('transform', event.transform);
      });

    svg.call(zoom);

    // Create force simulation
    const simulation = d3.forceSimulation<Person>(data.nodes)
      .force('link', d3.forceLink<Person, CollaborationEdge>(data.links)
        .id(d => d.id)
        .distance(d => Math.max(50, 200 - d.weight * 10))
      )
      .force('charge', d3.forceManyBody()
        .strength(d => -300 - (d.influenceMetrics.authorityScore * 200))
      )
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide()
        .radius(d => getNodeRadius(d) + 5)
      );

    // Draw links
    const links = linksGroup
      .selectAll('line')
      .data(data.links)
      .join('line')
      .attr('class', 'collaboration-link')
      .attr('stroke', d => getLinkColor(d))
      .attr('stroke-width', d => Math.max(1, d.weight * 3))
      .attr('stroke-opacity', d => Math.min(1, d.weight * 2))
      .style('cursor', 'pointer');

    // Draw nodes
    const nodes = nodesGroup
      .selectAll('g')
      .data(data.nodes)
      .join('g')
      .attr('class', 'person-node')
      .style('cursor', 'pointer');

    // Node circles
    nodes.append('circle')
      .attr('r', getNodeRadius)
      .attr('fill', d => getPersonColor(d))
      .attr('stroke', d => selectedPersonId === d.id ? '#2563eb' : '#fff')
      .attr('stroke-width', d => selectedPersonId === d.id ? 3 : 1.5)
      .attr('opacity', d => getPersonOpacity(d));

    // Node labels
    nodes.append('text')
      .attr('dx', 0)
      .attr('dy', d => getNodeRadius(d) + 16)
      .attr('text-anchor', 'middle')
      .attr('font-size', '11px')
      .attr('font-weight', '500')
      .attr('fill', '#374151')
      .text(d => d.displayName || d.email?.split('@')[0] || d.id);

    // Platform indicators
    nodes.each(function(d) {
      const node = d3.select(this);
      const radius = getNodeRadius(d);
      
      d.platforms.forEach((platform, i) => {
        const angle = (i / d.platforms.length) * 2 * Math.PI;
        const badgeX = Math.cos(angle) * (radius + 8);
        const badgeY = Math.sin(angle) * (radius + 8);
        
        node.append('circle')
          .attr('cx', badgeX)
          .attr('cy', badgeY)
          .attr('r', 4)
          .attr('fill', getPlatformColor(platform))
          .attr('stroke', '#fff')
          .attr('stroke-width', 1);
      });
    });

    // Event handlers
    nodes
      .on('click', (event, d) => {
        event.stopPropagation();
        onPersonSelect?.(d.id);
      })
      .on('mouseenter', (event, d) => {
        setHoveredPerson(d);
        onPersonHover?.(d);
        
        // Highlight connected nodes and links
        highlightConnections(d.id, true);
      })
      .on('mouseleave', () => {
        setHoveredPerson(null);
        onPersonHover?.(null);
        
        // Remove highlights
        highlightConnections('', false);
      });

    // Drag behavior
    const drag = d3.drag<SVGGElement, Person>()
      .on('start', (event, d) => {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        d.fx = d.x;
        d.fy = d.y;
      })
      .on('drag', (event, d) => {
        d.fx = event.x;
        d.fy = event.y;
      })
      .on('end', (event, d) => {
        if (!event.active) simulation.alphaTarget(0);
        d.fx = null;
        d.fy = null;
      });

    nodes.call(drag);

    // Update positions on simulation tick
    simulation.on('tick', () => {
      links
        .attr('x1', d => (d.source as Person).x!)
        .attr('y1', d => (d.source as Person).y!)
        .attr('x2', d => (d.target as Person).x!)
        .attr('y2', d => (d.target as Person).y!);

      nodes
        .attr('transform', d => `translate(${d.x},${d.y})`);
    });

    // Highlight connections function
    function highlightConnections(personId: string, highlight: boolean) {
      const opacity = highlight ? 0.2 : 1;
      const highlightOpacity = highlight ? 1 : 1;

      nodes
        .attr('opacity', d => {
          if (!highlight) return 1;
          if (d.id === personId) return highlightOpacity;
          const isConnected = data.links.some(link => 
            (link.source === personId && link.target === d.id) ||
            (link.target === personId && link.source === d.id)
          );
          return isConnected ? highlightOpacity : opacity;
        });

      links
        .attr('opacity', d => {
          if (!highlight) return Math.min(1, d.weight * 2);
          const isConnected = d.source === personId || d.target === personId;
          return isConnected ? highlightOpacity : opacity;
        });
    }

    // Center the graph initially
    const bounds = container.node()?.getBBox();
    if (bounds) {
      const fullWidth = bounds.width;
      const fullHeight = bounds.height;
      const widthScale = width / fullWidth;
      const heightScale = height / fullHeight;
      const scale = Math.min(widthScale, heightScale) * 0.8;
      
      const translate = [
        (width - fullWidth * scale) / 2,
        (height - fullHeight * scale) / 2
      ];

      svg.call(zoom.transform, d3.zoomIdentity
        .translate(translate[0], translate[1])
        .scale(scale)
      );
    }

  }, [data, selectedPersonId, highlightTopic, width, height]);

  // Helper functions
  const getNodeRadius = (person: Person): number => {
    const baseRadius = 12;
    const authorityBonus = person.influenceMetrics.authorityScore * 15;
    return Math.min(30, baseRadius + authorityBonus);
  };

  const getPersonColor = (person: Person): string => {
    if (highlightTopic) {
      const hasExpertise = person.expertiseAreas.some(area => 
        area.topic.toLowerCase().includes(highlightTopic.toLowerCase())
      );
      if (hasExpertise) return '#10b981'; // Green for topic experts
    }
    
    // Color by primary platform or influence level
    const influenceLevel = person.influenceMetrics.authorityScore;
    if (influenceLevel > 0.8) return '#8b5cf6'; // Purple for high influence
    if (influenceLevel > 0.6) return '#3b82f6'; // Blue for medium influence
    if (influenceLevel > 0.4) return '#06b6d4'; // Cyan for some influence
    return '#6b7280'; // Gray for low influence
  };

  const getPersonOpacity = (person: Person): number => {
    if (selectedPersonId && selectedPersonId !== person.id) {
      // Check if connected to selected person
      const isConnected = data?.links.some(link => 
        (link.source === selectedPersonId && link.target === person.id) ||
        (link.target === selectedPersonId && link.source === person.id)
      );
      return isConnected ? 1 : 0.3;
    }
    return 1;
  };

  const getLinkColor = (link: CollaborationEdge): string => {
    // Color by collaboration strength or primary platform
    if (link.platforms.includes('slack')) return '#e11d48';
    if (link.platforms.includes('google')) return '#0f9549';
    if (link.platforms.includes('jira')) return '#0052cc';
    return '#6b7280';
  };

  const getPlatformColor = (platform: string): string => {
    switch (platform.toLowerCase()) {
      case 'slack': return '#e11d48';
      case 'google': return '#0f9549';
      case 'jira': return '#0052cc';
      default: return '#6b7280';
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
        <span className="ml-2 text-gray-600">Loading people network...</span>
      </div>
    );
  }

  if (!data || data.nodes.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-gray-500">
        <div className="text-center">
          <svg className="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z" />
          </svg>
          <p className="mt-2">No people network data available</p>
          <p className="text-sm text-gray-400">Process some Jira, Google, or Slack content first</p>
        </div>
      </div>
    );
  }

  return (
    <div className="relative">
      <svg
        ref={svgRef}
        width={width}
        height={height}
        className="border border-gray-200 rounded-lg bg-gray-50"
      />
      
      {/* Legend */}
      <div className="absolute top-4 left-4 bg-white border border-gray-200 rounded-lg p-3 shadow-sm">
        <h4 className="text-sm font-semibold text-gray-700 mb-2">Legend</h4>
        <div className="space-y-1 text-xs">
          <div className="flex items-center">
            <div className="w-3 h-3 rounded-full bg-purple-500 mr-2"></div>
            <span>High Influence</span>
          </div>
          <div className="flex items-center">
            <div className="w-3 h-3 rounded-full bg-blue-500 mr-2"></div>
            <span>Medium Influence</span>
          </div>
          <div className="flex items-center">
            <div className="w-3 h-3 rounded-full bg-gray-400 mr-2"></div>
            <span>Low Influence</span>
          </div>
        </div>
        <div className="mt-2 pt-2 border-t border-gray-200">
          <div className="space-y-1 text-xs">
            <div className="flex items-center">
              <div className="w-2 h-2 rounded-full bg-red-500 mr-2"></div>
              <span>Slack</span>
            </div>
            <div className="flex items-center">
              <div className="w-2 h-2 rounded-full bg-green-500 mr-2"></div>
              <span>Google</span>
            </div>
            <div className="flex items-center">
              <div className="w-2 h-2 rounded-full bg-blue-600 mr-2"></div>
              <span>Jira</span>
            </div>
          </div>
        </div>
      </div>

      {/* Hover tooltip */}
      {hoveredPerson && (
        <div className="absolute bottom-4 left-4 bg-white border border-gray-200 rounded-lg p-3 shadow-lg max-w-xs">
          <h5 className="font-semibold text-gray-800">{hoveredPerson.displayName}</h5>
          <p className="text-sm text-gray-600">{hoveredPerson.email}</p>
          <div className="mt-2 space-y-1 text-xs">
            <div>Authority: {Math.round(hoveredPerson.influenceMetrics.authorityScore * 100)}%</div>
            <div>Collaborations: {hoveredPerson.collaborationPartners.length}</div>
            <div>Platforms: {hoveredPerson.platforms.join(', ')}</div>
          </div>
          {hoveredPerson.expertiseAreas.length > 0 && (
            <div className="mt-2">
              <div className="text-xs font-medium text-gray-700">Top Expertise:</div>
              <div className="text-xs text-gray-600">
                {hoveredPerson.expertiseAreas.slice(0, 3).map(area => area.topic).join(', ')}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default PeopleNetworkGraph;