import React, { useEffect, useRef, useState } from 'react'
import * as d3 from 'd3'

interface GraphNode {
  id: string
  label: string
  node_type: string
  size: number
  color: string
  metadata?: any
  x?: number
  y?: number
  fx?: number | null
  fy?: number | null
}

interface GraphEdge {
  id: string
  source: string | GraphNode
  target: string | GraphNode
  edge_type: string
  weight: number
  label?: string
}

interface KnowledgeGraphData {
  nodes: GraphNode[]
  edges: GraphEdge[]
  metadata: {
    total_nodes: number
    total_edges: number
    generated_at: string
  }
}

interface KnowledgeGraphProps {
  width?: number
  height?: number
}

export default function KnowledgeGraph({
  width = 800,
  height = 600,
}: KnowledgeGraphProps) {
  const svgRef = useRef<SVGSVGElement>(null)
  const [data, setData] = useState<KnowledgeGraphData | null>(null)
  const [filteredData, setFilteredData] = useState<KnowledgeGraphData | null>(
    null
  )
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null)

  // Filter controls
  const [nodeTypeFilter, setNodeTypeFilter] = useState<string>('all')
  const [searchTerm, setSearchTerm] = useState('')
  const [maxNodes, setMaxNodes] = useState(50)

  // Fetch graph data
  useEffect(() => {
    const fetchGraphData = async () => {
      try {
        setLoading(true)
        const response = await fetch(
          `http://127.0.0.1:3001/api/graph?limit=${maxNodes}`
        )
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        const graphData: KnowledgeGraphData = await response.json()
        setData(graphData)
        setError(null)
      } catch (err) {
        setError(
          err instanceof Error ? err.message : 'Failed to fetch graph data'
        )
        console.error('Failed to fetch graph data:', err)
      } finally {
        setLoading(false)
      }
    }

    fetchGraphData()
  }, [maxNodes])

  // Apply filters to data
  useEffect(() => {
    if (!data) {
      setFilteredData(null)
      return
    }

    let filteredNodes = [...data.nodes]
    let filteredEdges = [...data.edges]

    // Filter by node type
    if (nodeTypeFilter !== 'all') {
      filteredNodes = filteredNodes.filter(
        (node) => node.node_type.toLowerCase() === nodeTypeFilter.toLowerCase()
      )
    }

    // Filter by search term
    if (searchTerm) {
      filteredNodes = filteredNodes.filter(
        (node) =>
          node.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
          node.node_type.toLowerCase().includes(searchTerm.toLowerCase()) ||
          (node.metadata?.key &&
            node.metadata.key.toLowerCase().includes(searchTerm.toLowerCase()))
      )
    }

    // Filter edges to only include those with both nodes present
    const nodeIds = new Set(filteredNodes.map((n) => n.id))
    filteredEdges = filteredEdges.filter(
      (edge) =>
        nodeIds.has(edge.source as string) && nodeIds.has(edge.target as string)
    )

    setFilteredData({
      nodes: filteredNodes,
      edges: filteredEdges,
      metadata: {
        ...data.metadata,
        total_nodes: filteredNodes.length,
        total_edges: filteredEdges.length,
      },
    })
  }, [data, nodeTypeFilter, searchTerm])

  // Initialize D3 visualization
  useEffect(() => {
    if (!filteredData || !svgRef.current) return

    const displayData = filteredData

    const svg = d3.select(svgRef.current)
    svg.selectAll('*').remove()

    // Create zoom behavior
    const zoom = d3
      .zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 10])
      .on('zoom', (event) => {
        container.attr('transform', event.transform)
      })

    svg.call(zoom)

    // Create container group for zooming/panning
    const container = svg.append('g').attr('class', 'graph-container')

    // Create simulation
    const simulation = d3
      .forceSimulation<GraphNode>(displayData.nodes)
      .force(
        'link',
        d3
          .forceLink<GraphNode, GraphEdge>(displayData.edges)
          .id((d) => d.id)
          .distance((d) => 50 + d.weight * 20)
          .strength(0.1)
      )
      .force(
        'charge',
        d3.forceManyBody().strength((d) => -100 - d.size * 10)
      )
      .force('center', d3.forceCenter(width / 2, height / 2))
      .force(
        'collision',
        d3.forceCollide<GraphNode>().radius((d) => d.size + 2)
      )

    // Create edges
    const links = container
      .selectAll<SVGLineElement, GraphEdge>('.link')
      .data(displayData.edges)
      .enter()
      .append('line')
      .attr('class', 'link')
      .attr('stroke', '#999')
      .attr('stroke-opacity', 0.6)
      .attr('stroke-width', (d) => Math.sqrt(d.weight))

    // Create edge labels
    const linkLabels = container
      .selectAll<SVGTextElement, GraphEdge>('.link-label')
      .data(displayData.edges.filter((d) => d.label))
      .enter()
      .append('text')
      .attr('class', 'link-label')
      .attr('text-anchor', 'middle')
      .attr('font-size', '8px')
      .attr('fill', '#666')
      .text((d) => d.label || '')

    // Create node groups
    const nodeGroups = container
      .selectAll<SVGGElement, GraphNode>('.node-group')
      .data(displayData.nodes)
      .enter()
      .append('g')
      .attr('class', 'node-group')
      .style('cursor', 'pointer')

    // Create nodes
    const nodes = nodeGroups
      .append('circle')
      .attr('class', 'node')
      .attr('r', (d) => d.size)
      .attr('fill', (d) => d.color)
      .attr('stroke', '#fff')
      .attr('stroke-width', 2)
      .on('click', (event, d) => {
        event.stopPropagation()
        setSelectedNode(d)
      })
      .on('mouseover', function (event, d) {
        d3.select(this).attr('stroke-width', 4).attr('stroke', '#000')

        // Show tooltip
        const tooltip = d3
          .select('body')
          .append('div')
          .attr('class', 'tooltip')
          .style('opacity', 0)
          .style('position', 'absolute')
          .style('background', 'rgba(0, 0, 0, 0.8)')
          .style('color', 'white')
          .style('padding', '8px')
          .style('border-radius', '4px')
          .style('font-size', '12px')
          .style('pointer-events', 'none')
          .style('z-index', '1000')

        tooltip
          .html(
            `
          <strong>${d.label}</strong><br/>
          Type: ${d.node_type}<br/>
          ${d.metadata?.status ? `Status: ${d.metadata.status}<br/>` : ''}
          ${d.metadata?.key ? `Key: ${d.metadata.key}` : ''}
        `
          )
          .style('left', event.pageX + 10 + 'px')
          .style('top', event.pageY - 10 + 'px')
          .transition()
          .duration(200)
          .style('opacity', 1)
      })
      .on('mouseout', function () {
        d3.select(this).attr('stroke-width', 2).attr('stroke', '#fff')

        d3.selectAll('.tooltip').remove()
      })

    // Add node labels
    const labels = nodeGroups
      .append('text')
      .attr('class', 'node-label')
      .attr('text-anchor', 'middle')
      .attr('dy', '.35em')
      .attr('font-size', (d) => Math.max(8, d.size / 3))
      .attr('font-weight', 'bold')
      .attr('fill', 'white')
      .attr('stroke', 'black')
      .attr('stroke-width', 0.5)
      .attr('paint-order', 'stroke')
      .style('pointer-events', 'none')
      .text((d) => {
        const maxLength = Math.max(5, Math.floor(d.size / 2))
        return d.label.length > maxLength
          ? d.label.substring(0, maxLength) + '...'
          : d.label
      })

    // Add drag behavior
    const drag = d3
      .drag<SVGGElement, GraphNode>()
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

    // Update positions on simulation tick
    simulation.on('tick', () => {
      links
        .attr('x1', (d) => (d.source as GraphNode).x!)
        .attr('y1', (d) => (d.source as GraphNode).y!)
        .attr('x2', (d) => (d.target as GraphNode).x!)
        .attr('y2', (d) => (d.target as GraphNode).y!)

      linkLabels
        .attr(
          'x',
          (d) => ((d.source as GraphNode).x! + (d.target as GraphNode).x!) / 2
        )
        .attr(
          'y',
          (d) => ((d.source as GraphNode).y! + (d.target as GraphNode).y!) / 2
        )

      nodeGroups.attr('transform', (d) => `translate(${d.x},${d.y})`)
    })

    // Clear selection when clicking on background
    svg.on('click', () => setSelectedNode(null))

    // Cleanup
    return () => {
      simulation.stop()
    }
  }, [filteredData, width, height])

  if (loading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-white">Loading knowledge graph...</div>
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
        <div className="text-gray-400">No graph data available</div>
      </div>
    )
  }

  // Get unique node types for filter dropdown
  const nodeTypes = ['all', ...new Set(data.nodes.map((n) => n.node_type))]

  return (
    <div className="w-full">
      <div className="mb-4 text-white">
        <h3 className="text-lg font-bold mb-2">Knowledge Graph</h3>

        {/* Controls */}
        <div className="flex flex-wrap gap-4 mb-4 p-4 bg-gray-800 rounded-lg">
          <div className="flex items-center gap-2">
            <label
              htmlFor="search"
              className="text-sm font-medium text-gray-300"
            >
              Search:
            </label>
            <input
              id="search"
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder="Search nodes..."
              className="px-3 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm focus:outline-none focus:border-blue-500"
            />
          </div>

          <div className="flex items-center gap-2">
            <label
              htmlFor="nodeType"
              className="text-sm font-medium text-gray-300"
            >
              Node Type:
            </label>
            <select
              id="nodeType"
              value={nodeTypeFilter}
              onChange={(e) => setNodeTypeFilter(e.target.value)}
              className="px-3 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm focus:outline-none focus:border-blue-500"
            >
              {nodeTypes.map((type) => (
                <option key={type} value={type}>
                  {type === 'all' ? 'All Types' : type}
                </option>
              ))}
            </select>
          </div>

          <div className="flex items-center gap-2">
            <label
              htmlFor="maxNodes"
              className="text-sm font-medium text-gray-300"
            >
              Max Nodes:
            </label>
            <select
              id="maxNodes"
              value={maxNodes}
              onChange={(e) => setMaxNodes(Number(e.target.value))}
              className="px-3 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm focus:outline-none focus:border-blue-500"
            >
              <option value={25}>25</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
              <option value={200}>200</option>
            </select>
          </div>

          <button
            onClick={() => {
              setSearchTerm('')
              setNodeTypeFilter('all')
              setSelectedNode(null)
            }}
            className="px-3 py-1 bg-red-600 hover:bg-red-700 text-white text-sm rounded font-medium"
          >
            Clear Filters
          </button>
        </div>

        {/* Stats */}
        <div className="flex gap-4 text-sm text-gray-300 mb-4">
          <span>Total Nodes: {data.metadata.total_nodes}</span>
          <span>Total Edges: {data.metadata.total_edges}</span>
          {filteredData && (
            <>
              <span className="text-blue-400">
                Showing: {filteredData.metadata.total_nodes} nodes,{' '}
                {filteredData.metadata.total_edges} edges
              </span>
            </>
          )}
          <span>
            Generated: {new Date(data.metadata.generated_at).toLocaleString()}
          </span>
        </div>
      </div>

      <div className="flex gap-4">
        <div className="flex-1">
          <svg
            ref={svgRef}
            width={width}
            height={height}
            className="border border-gray-600 rounded bg-gray-800"
          />
        </div>

        {selectedNode && (
          <div className="w-80 p-4 bg-gray-800 border border-gray-600 rounded">
            <h4 className="text-white font-bold mb-2">Node Details</h4>
            <div className="text-sm text-gray-300 space-y-1">
              <div>
                <strong>Label:</strong> {selectedNode.label}
              </div>
              <div>
                <strong>Type:</strong> {selectedNode.node_type}
              </div>
              <div>
                <strong>Size:</strong> {selectedNode.size}
              </div>
              {selectedNode.metadata && (
                <div>
                  <strong>Metadata:</strong>
                  <pre className="mt-1 p-2 bg-gray-900 rounded text-xs overflow-auto max-h-40">
                    {JSON.stringify(selectedNode.metadata, null, 2)}
                  </pre>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
