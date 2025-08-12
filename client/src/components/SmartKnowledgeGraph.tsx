import React, { useEffect, useRef, useState } from 'react'
import * as d3 from 'd3'

interface SmartGraphData {
  nodes: Array<{
    id: string
    label: string
    node_type: string
    size: number
    color: string
    centrality_score: number
    clustering_coefficient: number
    community_id?: string
    importance_rank: number
    learning_value: number
    expertise_level: number
    knowledge_depth: number
    metadata?: any
  }>
  edges: Array<{
    id: string
    source: string
    target: string
    edge_type: string
    weight: number
    strength: number
    frequency: number
    learning_pathway: boolean
    label?: string
  }>
  clusters: Array<{
    id: string
    name: string
    nodes: string[]
    center_node: string
    cohesion_score: number
    cluster_type: string
    description: string
    key_insights: string[]
  }>
  pathways: Array<{
    id: string
    name: string
    nodes: string[]
    difficulty: string
    estimated_time: string
    learning_objectives: string[]
    pathway_type: string
  }>
  recommendations: Array<{
    id: string
    recommendation_type: string
    title: string
    description: string
    confidence: number
    impact: string
    target_nodes: string[]
    action_items: string[]
  }>
  analytics: {
    network_density: number
    average_clustering: number
    diameter: number
    modularity: number
    knowledge_flow_score: number
    collaboration_index: number
    expertise_distribution: Record<string, number>
    critical_connectors: string[]
    knowledge_gaps: string[]
  }
}

export default function SmartKnowledgeGraph() {
  const svgRef = useRef<SVGSVGElement>(null)
  const [data, setData] = useState<SmartGraphData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [selectedCluster, setSelectedCluster] = useState<string | null>(null)
  const [showPathways, setShowPathways] = useState(false)
  const [activePathway, setActivePathway] = useState<string | null>(null)
  const [viewMode, setViewMode] = useState<'normal' | 'clusters' | 'pathways'>('normal')

  // Fetch smart graph data
  useEffect(() => {
    const fetchSmartGraph = async () => {
      try {
        setLoading(true)
        const response = await fetch('http://127.0.0.1:3001/api/graph/smart')
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        const smartData: SmartGraphData = await response.json()
        setData(smartData)
        setError(null)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch smart graph')
        console.error('Failed to fetch smart graph:', err)
      } finally {
        setLoading(false)
      }
    }

    fetchSmartGraph()
  }, [])

  // Render smart graph
  useEffect(() => {
    if (!data || !svgRef.current) return

    const svg = d3.select(svgRef.current)
    svg.selectAll('*').remove()

    const width = 1200
    const height = 800

    // Create zoom behavior
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 5])
      .on('zoom', (event) => {
        container.attr('transform', event.transform)
      })

    svg.call(zoom)

    const container = svg.append('g').attr('class', 'graph-container')

    // Filter nodes and edges based on view mode
    let displayNodes = [...data.nodes]
    let displayEdges = [...data.edges]

    if (selectedCluster) {
      const cluster = data.clusters.find(c => c.id === selectedCluster)
      if (cluster) {
        displayNodes = data.nodes.filter(n => cluster.nodes.includes(n.id))
        displayEdges = data.edges.filter(e => 
          cluster.nodes.includes(e.source) && cluster.nodes.includes(e.target)
        )
      }
    }

    if (activePathway) {
      const pathway = data.pathways.find(p => p.id === activePathway)
      if (pathway) {
        displayNodes = data.nodes.filter(n => pathway.nodes.includes(n.id))
        displayEdges = data.edges.filter(e => 
          pathway.nodes.includes(e.source) && pathway.nodes.includes(e.target)
        )
      }
    }

    // Cluster background circles (if in cluster mode)
    if (viewMode === 'clusters') {
      data.clusters.forEach(cluster => {
        const clusterNodes = data.nodes.filter(n => cluster.nodes.includes(n.id))
        if (clusterNodes.length > 0) {
          const centerX = d3.mean(clusterNodes, n => n.metadata?.x || width / 2) || width / 2
          const centerY = d3.mean(clusterNodes, n => n.metadata?.y || height / 2) || height / 2
          const radius = Math.max(80, clusterNodes.length * 15)

          container
            .append('circle')
            .attr('cx', centerX)
            .attr('cy', centerY)
            .attr('r', radius)
            .attr('fill', d3.schemeCategory10[data.clusters.indexOf(cluster) % 10])
            .attr('fill-opacity', 0.1)
            .attr('stroke', d3.schemeCategory10[data.clusters.indexOf(cluster) % 10])
            .attr('stroke-width', 2)
            .attr('stroke-dasharray', '5,5')

          container
            .append('text')
            .attr('x', centerX)
            .attr('y', centerY - radius - 10)
            .attr('text-anchor', 'middle')
            .style('fill', '#E5E7EB')
            .style('font-size', '14px')
            .style('font-weight', 'bold')
            .text(cluster.name)
        }
      })
    }

    // Create force simulation with enhanced forces
    const simulation = d3.forceSimulation(displayNodes as any)
      .force('link', d3.forceLink(displayEdges)
        .id(d => (d as any).id)
        .distance(d => {
          const edge = d as any
          const baseDistance = 50
          const strengthMultiplier = edge.learning_pathway ? 0.5 : 1
          return baseDistance + (edge.weight * 10) * strengthMultiplier
        })
        .strength(d => {
          const edge = d as any
          return edge.learning_pathway ? 0.8 : 0.3
        })
      )
      .force('charge', d3.forceManyBody()
        .strength(d => {
          const node = d as any
          const baseStrength = -200
          const centralityBoost = node.centrality_score * -300
          const importanceBoost = (1 - node.importance_rank / data.nodes.length) * -100
          return baseStrength + centralityBoost + importanceBoost
        })
      )
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force('collision', d3.forceCollide()
        .radius(d => (d as any).size + 5)
        .strength(0.7)
      )

    // Enhanced edge rendering with pathway highlighting
    const linkGroup = container.append('g').attr('class', 'links')
    
    const links = linkGroup.selectAll('.link')
      .data(displayEdges)
      .enter()
      .append('line')
      .attr('class', 'link')
      .attr('stroke', d => {
        if (d.learning_pathway && showPathways) return '#10B981'
        if (d.strength > 5) return '#F59E0B'
        return '#6B7280'
      })
      .attr('stroke-width', d => Math.sqrt(d.strength) + 1)
      .attr('stroke-opacity', d => d.learning_pathway && showPathways ? 0.9 : 0.6)
      .style('stroke-dasharray', d => d.learning_pathway && showPathways ? '0' : '3,3')

    // Enhanced node rendering with intelligence indicators
    const nodeGroup = container.append('g').attr('class', 'nodes')
    
    const nodeGroups = nodeGroup.selectAll('.node-group')
      .data(displayNodes)
      .enter()
      .append('g')
      .attr('class', 'node-group')
      .style('cursor', 'pointer')

    // Main node circles with gradient for importance
    const defs = svg.append('defs')
    
    displayNodes.forEach((node, i) => {
      const gradient = defs.append('radialGradient')
        .attr('id', `gradient-${i}`)
      
      gradient.append('stop')
        .attr('offset', '0%')
        .attr('stop-color', node.color)
        .attr('stop-opacity', 0.9)
      
      gradient.append('stop')
        .attr('offset', '100%')
        .attr('stop-color', d3.color(node.color)?.darker(1).toString() || node.color)
        .attr('stop-opacity', 0.7)
    })

    const nodes = nodeGroups
      .append('circle')
      .attr('class', 'node')
      .attr('r', d => d.size * (1 + d.centrality_score * 0.5))
      .attr('fill', (d, i) => `url(#gradient-${i})`)
      .attr('stroke', '#fff')
      .attr('stroke-width', d => 2 + d.importance_rank <= 5 ? 2 : 0) // Highlight top 5 important nodes
      .on('click', handleNodeClick)
      .on('mouseover', showNodeTooltip)
      .on('mouseout', hideTooltip)

    // Importance indicators (small rings for top nodes)
    nodeGroups
      .filter(d => d.importance_rank <= 3)
      .append('circle')
      .attr('class', 'importance-ring')
      .attr('r', d => d.size * 1.3)
      .attr('fill', 'none')
      .attr('stroke', '#FFD700')
      .attr('stroke-width', 2)
      .attr('stroke-dasharray', '2,2')
      .style('opacity', 0.8)

    // Knowledge depth indicators
    nodeGroups
      .filter(d => d.knowledge_depth > 7)
      .append('circle')
      .attr('class', 'knowledge-ring')
      .attr('r', d => d.size * 1.5)
      .attr('fill', 'none')
      .attr('stroke', '#8B5CF6')
      .attr('stroke-width', 1)
      .style('opacity', 0.6)

    // Node labels with intelligent sizing
    const labels = nodeGroups
      .append('text')
      .attr('class', 'node-label')
      .attr('text-anchor', 'middle')
      .attr('dy', '.35em')
      .attr('font-size', d => Math.max(8, Math.min(12, d.size / 2 + d.centrality_score * 3)))
      .attr('font-weight', d => d.importance_rank <= 5 ? 'bold' : 'normal')
      .attr('fill', 'white')
      .attr('stroke', 'black')
      .attr('stroke-width', 0.5)
      .attr('paint-order', 'stroke')
      .style('pointer-events', 'none')
      .text(d => {
        const maxLength = Math.max(8, Math.floor(d.size / 2))
        return d.label.length > maxLength 
          ? d.label.substring(0, maxLength) + '...' 
          : d.label
      })

    // Drag behavior with enhanced physics
    const drag = d3.drag<SVGGElement, any>()
      .on('start', (event, d) => {
        if (!event.active) simulation.alphaTarget(0.3).restart()
        d.fx = d.x
        d.fy = d.y
      })
      .on('drag', (event, d) => {
        d.fx = event.x
        d.fy = event.y
      })
      .on('end', (event, d) => {
        if (!event.active) simulation.alphaTarget(0)
        d.fx = null
        d.fy = null
      })

    nodeGroups.call(drag)

    // Simulation tick with smooth animations
    simulation.on('tick', () => {
      links
        .attr('x1', d => (d.source as any).x)
        .attr('y1', d => (d.source as any).y)
        .attr('x2', d => (d.target as any).x)
        .attr('y2', d => (d.target as any).y)

      nodeGroups
        .attr('transform', d => `translate(${d.x},${d.y})`)
    })

    // Event handlers
    function handleNodeClick(event: any, d: any) {
      event.stopPropagation()
      console.log('Node clicked:', d)
      
      // Highlight connected nodes
      const connectedEdges = displayEdges.filter(e => e.source === d.id || e.target === d.id)
      const connectedNodes = new Set<string>()
      
      connectedEdges.forEach(e => {
        connectedNodes.add(typeof e.source === 'string' ? e.source : e.source.id)
        connectedNodes.add(typeof e.target === 'string' ? e.target : e.target.id)
      })

      // Dim non-connected nodes
      nodes.style('opacity', node => connectedNodes.has(node.id) ? 1 : 0.3)
      links.style('opacity', edge => 
        connectedEdges.some(ce => ce.id === edge.id) ? 1 : 0.1
      )
    }

    function showNodeTooltip(event: any, d: any) {
      const tooltip = d3.select('body').append('div')
        .attr('class', 'smart-graph-tooltip')
        .style('opacity', 0)
        .style('position', 'absolute')
        .style('background', 'rgba(0, 0, 0, 0.9)')
        .style('color', 'white')
        .style('padding', '12px')
        .style('border-radius', '8px')
        .style('font-size', '12px')
        .style('pointer-events', 'none')
        .style('z-index', '1000')
        .style('max-width', '300px')

      tooltip.html(`
        <div style="font-weight: bold; margin-bottom: 8px;">${d.label}</div>
        <div style="margin-bottom: 4px;"><strong>Type:</strong> ${d.node_type}</div>
        <div style="margin-bottom: 4px;"><strong>Importance:</strong> #${d.importance_rank + 1}</div>
        <div style="margin-bottom: 4px;"><strong>Centrality:</strong> ${(d.centrality_score * 100).toFixed(0)}%</div>
        <div style="margin-bottom: 4px;"><strong>Learning Value:</strong> ${d.learning_value.toFixed(1)}/10</div>
        <div style="margin-bottom: 4px;"><strong>Knowledge Depth:</strong> ${d.knowledge_depth.toFixed(1)}/10</div>
        <div style="margin-bottom: 4px;"><strong>Expertise Level:</strong> ${d.expertise_level.toFixed(1)}/10</div>
        ${d.clustering_coefficient > 0 ? `<div><strong>Clustering:</strong> ${(d.clustering_coefficient * 100).toFixed(0)}%</div>` : ''}
      `)
      .style('left', (event.pageX + 10) + 'px')
      .style('top', (event.pageY - 10) + 'px')
      .transition()
      .duration(200)
      .style('opacity', 1)
    }

    function hideTooltip() {
      d3.selectAll('.smart-graph-tooltip').remove()
      
      // Reset node and link opacity
      nodes.style('opacity', 1)
      links.style('opacity', d => d.learning_pathway && showPathways ? 0.9 : 0.6)
    }

    // Clear selection when clicking on background
    svg.on('click', () => {
      hideTooltip()
    })

    return () => {
      simulation.stop()
    }
  }, [data, selectedCluster, showPathways, activePathway, viewMode])

  if (loading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-white">🧠 Loading intelligent graph analysis...</div>
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
        <div className="text-gray-400">No smart graph data available</div>
      </div>
    )
  }

  return (
    <div className="w-full space-y-4">
      {/* Controls */}
      <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
        <div className="flex flex-wrap items-center justify-between gap-4">
          <div className="flex items-center space-x-4">
            <div className="flex items-center space-x-2">
              <label className="text-sm text-gray-300">View Mode:</label>
              <select
                value={viewMode}
                onChange={(e) => setViewMode(e.target.value as any)}
                className="bg-gray-700 border border-gray-600 rounded px-3 py-1 text-white text-sm focus:outline-none focus:border-blue-500"
              >
                <option value="normal">All Nodes</option>
                <option value="clusters">Cluster View</option>
                <option value="pathways">Learning Pathways</option>
              </select>
            </div>

            {viewMode === 'clusters' && (
              <div className="flex items-center space-x-2">
                <label className="text-sm text-gray-300">Cluster:</label>
                <select
                  value={selectedCluster || ''}
                  onChange={(e) => setSelectedCluster(e.target.value || null)}
                  className="bg-gray-700 border border-gray-600 rounded px-3 py-1 text-white text-sm focus:outline-none focus:border-blue-500"
                >
                  <option value="">All Clusters</option>
                  {data.clusters.map(cluster => (
                    <option key={cluster.id} value={cluster.id}>
                      {cluster.name} ({cluster.nodes.length} nodes)
                    </option>
                  ))}
                </select>
              </div>
            )}

            {viewMode === 'pathways' && (
              <div className="flex items-center space-x-2">
                <label className="text-sm text-gray-300">Pathway:</label>
                <select
                  value={activePathway || ''}
                  onChange={(e) => setActivePathway(e.target.value || null)}
                  className="bg-gray-700 border border-gray-600 rounded px-3 py-1 text-white text-sm focus:outline-none focus:border-blue-500"
                >
                  <option value="">All Pathways</option>
                  {data.pathways.map(pathway => (
                    <option key={pathway.id} value={pathway.id}>
                      {pathway.name} ({pathway.difficulty})
                    </option>
                  ))}
                </select>
              </div>
            )}
          </div>

          <div className="flex items-center space-x-4">
            <label className="flex items-center space-x-2">
              <input
                type="checkbox"
                checked={showPathways}
                onChange={(e) => setShowPathways(e.target.checked)}
                className="rounded"
              />
              <span className="text-sm text-gray-300">Show Learning Pathways</span>
            </label>

            <button
              onClick={() => {
                setSelectedCluster(null)
                setActivePathway(null)
                setShowPathways(false)
                setViewMode('normal')
              }}
              className="px-3 py-1 bg-red-600 hover:bg-red-700 text-white text-sm rounded font-medium"
            >
              Reset View
            </button>
          </div>
        </div>
      </div>

      {/* Graph Analytics Summary */}
      <div className="grid grid-cols-2 md:grid-cols-6 gap-4">
        <div className="bg-gray-800 p-3 rounded-lg text-center">
          <div className="text-lg font-bold text-blue-400">{data.analytics.network_density.toFixed(2)}</div>
          <div className="text-xs text-gray-400">Network Density</div>
        </div>
        <div className="bg-gray-800 p-3 rounded-lg text-center">
          <div className="text-lg font-bold text-green-400">{data.analytics.average_clustering.toFixed(2)}</div>
          <div className="text-xs text-gray-400">Avg Clustering</div>
        </div>
        <div className="bg-gray-800 p-3 rounded-lg text-center">
          <div className="text-lg font-bold text-purple-400">{data.analytics.diameter}</div>
          <div className="text-xs text-gray-400">Graph Diameter</div>
        </div>
        <div className="bg-gray-800 p-3 rounded-lg text-center">
          <div className="text-lg font-bold text-yellow-400">{data.analytics.modularity.toFixed(2)}</div>
          <div className="text-xs text-gray-400">Modularity</div>
        </div>
        <div className="bg-gray-800 p-3 rounded-lg text-center">
          <div className="text-lg font-bold text-red-400">{data.analytics.knowledge_flow_score.toFixed(1)}</div>
          <div className="text-xs text-gray-400">Knowledge Flow</div>
        </div>
        <div className="bg-gray-800 p-3 rounded-lg text-center">
          <div className="text-lg font-bold text-indigo-400">{data.analytics.collaboration_index.toFixed(0)}%</div>
          <div className="text-xs text-gray-400">Collaboration</div>
        </div>
      </div>

      {/* Main Graph */}
      <div className="bg-gray-800 rounded-lg border border-gray-700 overflow-hidden">
        <svg
          ref={svgRef}
          width="1200"
          height="800"
          className="w-full h-auto bg-gray-900"
          style={{ minHeight: '600px' }}
        />
      </div>

      {/* Legend */}
      <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
        <h4 className="font-semibold text-white mb-3">🗂️ Graph Legend</h4>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div>
            <h5 className="font-medium text-gray-300 mb-2">Node Types</h5>
            <div className="space-y-1">
              <div className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full bg-blue-500"></div>
                <span className="text-gray-400">Issues</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full bg-indigo-500"></div>
                <span className="text-gray-400">Projects</span>
              </div>
            </div>
          </div>
          
          <div>
            <h5 className="font-medium text-gray-300 mb-2">Special Indicators</h5>
            <div className="space-y-1">
              <div className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full border-2 border-yellow-400"></div>
                <span className="text-gray-400">Top 3 Important</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full border border-purple-400"></div>
                <span className="text-gray-400">High Knowledge Depth</span>
              </div>
            </div>
          </div>
          
          <div>
            <h5 className="font-medium text-gray-300 mb-2">Connections</h5>
            <div className="space-y-1">
              <div className="flex items-center space-x-2">
                <div className="w-6 h-0.5 bg-green-500"></div>
                <span className="text-gray-400">Learning Pathways</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-6 h-0.5 bg-gray-500" style={{ strokeDasharray: '3,3' }}></div>
                <span className="text-gray-400">Regular Connections</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}