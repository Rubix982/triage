import React, { useEffect, useRef, useState } from 'react'
import * as d3 from 'd3'
import TrendChart from './TrendChart'

interface TeamVelocity {
  period: string
  issues_created: number
  issues_resolved: number
  velocity_score: number
}

interface StatusFlowAnalysis {
  status: string
  count: number
  percentage: number
  avg_time_in_status?: number
}

interface ProjectProductivity {
  project_key: string
  project_name: string
  total_issues: number
  resolved_issues: number
  resolution_rate: number
  avg_resolution_time?: number
}

interface DashboardSummary {
  total_issues: number
  total_projects: number
  resolution_rate: number
  avg_resolution_time?: number
  most_productive_period: string
  generated_at: string
}

interface AnalyticsData {
  team_velocity: TeamVelocity[]
  status_distribution: StatusFlowAnalysis[]
  project_productivity: ProjectProductivity[]
  summary_stats: DashboardSummary
}

export default function AnalyticsDashboard() {
  const [data, setData] = useState<AnalyticsData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  
  const velocityChartRef = useRef<SVGSVGElement>(null)
  const statusChartRef = useRef<SVGSVGElement>(null)
  const productivityChartRef = useRef<SVGSVGElement>(null)

  // Fetch analytics data
  useEffect(() => {
    const fetchAnalytics = async () => {
      try {
        setLoading(true)
        const response = await fetch('http://127.0.0.1:3001/api/analytics')
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        const analyticsData: AnalyticsData = await response.json()
        setData(analyticsData)
        setError(null)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch analytics data')
        console.error('Failed to fetch analytics:', err)
      } finally {
        setLoading(false)
      }
    }

    fetchAnalytics()
  }, [])

  // Team Velocity Chart
  useEffect(() => {
    if (!data?.team_velocity || !velocityChartRef.current) return

    const svg = d3.select(velocityChartRef.current)
    svg.selectAll('*').remove()

    const margin = { top: 20, right: 30, bottom: 40, left: 50 }
    const width = 600 - margin.left - margin.right
    const height = 300 - margin.bottom - margin.top

    const container = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`)

    // Scales
    const xScale = d3
      .scaleBand()
      .domain(data.team_velocity.map(d => d.period))
      .range([0, width])
      .padding(0.2)

    const yScale = d3
      .scaleLinear()
      .domain([0, d3.max(data.team_velocity, d => Math.max(d.issues_created, d.issues_resolved)) || 0])
      .range([height, 0])

    // Axes
    container
      .append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(xScale))
      .selectAll('text')
      .style('fill', '#9CA3AF')

    container
      .append('g')
      .call(d3.axisLeft(yScale))
      .selectAll('text')
      .style('fill', '#9CA3AF')

    // Bars for issues created
    container
      .selectAll('.bar-created')
      .data(data.team_velocity)
      .enter()
      .append('rect')
      .attr('class', 'bar-created')
      .attr('x', d => (xScale(d.period) || 0) + xScale.bandwidth() / 4)
      .attr('y', d => yScale(d.issues_created))
      .attr('width', xScale.bandwidth() / 2 - 2)
      .attr('height', d => height - yScale(d.issues_created))
      .attr('fill', '#60A5FA')
      .attr('opacity', 0.8)

    // Bars for issues resolved
    container
      .selectAll('.bar-resolved')
      .data(data.team_velocity)
      .enter()
      .append('rect')
      .attr('class', 'bar-resolved')
      .attr('x', d => (xScale(d.period) || 0) + (3 * xScale.bandwidth()) / 4)
      .attr('y', d => yScale(d.issues_resolved))
      .attr('width', xScale.bandwidth() / 2 - 2)
      .attr('height', d => height - yScale(d.issues_resolved))
      .attr('fill', '#10B981')
      .attr('opacity', 0.8)

    // Legend
    const legend = container
      .append('g')
      .attr('transform', `translate(${width - 120}, 20)`)

    legend
      .append('rect')
      .attr('x', 0)
      .attr('y', 0)
      .attr('width', 15)
      .attr('height', 15)
      .attr('fill', '#60A5FA')

    legend
      .append('text')
      .attr('x', 20)
      .attr('y', 12)
      .text('Created')
      .style('fill', '#E5E7EB')
      .style('font-size', '12px')

    legend
      .append('rect')
      .attr('x', 0)
      .attr('y', 20)
      .attr('width', 15)
      .attr('height', 15)
      .attr('fill', '#10B981')

    legend
      .append('text')
      .attr('x', 20)
      .attr('y', 32)
      .text('Resolved')
      .style('fill', '#E5E7EB')
      .style('font-size', '12px')

  }, [data])

  // Status Distribution Pie Chart
  useEffect(() => {
    if (!data?.status_distribution || !statusChartRef.current) return

    const svg = d3.select(statusChartRef.current)
    svg.selectAll('*').remove()

    const width = 400
    const height = 300
    const radius = Math.min(width, height) / 2 - 10

    const container = svg
      .append('g')
      .attr('transform', `translate(${width / 2},${height / 2})`)

    const color = d3
      .scaleOrdinal()
      .domain(data.status_distribution.map(d => d.status))
      .range(['#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#6B7280', '#EC4899'])

    const pie = d3
      .pie<StatusFlowAnalysis>()
      .value(d => d.count)
      .sort(null)

    const arc = d3
      .arc<d3.PieArcDatum<StatusFlowAnalysis>>()
      .innerRadius(0)
      .outerRadius(radius)

    const arcs = container
      .selectAll('.arc')
      .data(pie(data.status_distribution))
      .enter()
      .append('g')
      .attr('class', 'arc')

    arcs
      .append('path')
      .attr('d', arc)
      .attr('fill', d => color(d.data.status) as string)
      .attr('stroke', '#1F2937')
      .attr('stroke-width', 2)

    // Labels
    arcs
      .append('text')
      .attr('transform', d => `translate(${arc.centroid(d)})`)
      .attr('text-anchor', 'middle')
      .style('fill', 'white')
      .style('font-size', '12px')
      .style('font-weight', 'bold')
      .text(d => d.data.count > 2 ? d.data.status : '') // Only show label if count > 2

  }, [data])

  // Project Productivity Horizontal Bar Chart
  useEffect(() => {
    if (!data?.project_productivity || !productivityChartRef.current) return

    const svg = d3.select(productivityChartRef.current)
    svg.selectAll('*').remove()

    const margin = { top: 20, right: 30, bottom: 40, left: 120 }
    const width = 600 - margin.left - margin.right
    const height = Math.max(300, data.project_productivity.length * 30) - margin.top - margin.bottom

    // Update SVG height
    svg.attr('height', height + margin.top + margin.bottom)

    const container = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`)

    // Sort data by resolution rate
    const sortedData = [...data.project_productivity].sort((a, b) => b.resolution_rate - a.resolution_rate)

    // Scales
    const xScale = d3
      .scaleLinear()
      .domain([0, 100])
      .range([0, width])

    const yScale = d3
      .scaleBand()
      .domain(sortedData.map(d => d.project_key))
      .range([0, height])
      .padding(0.2)

    // Axes
    container
      .append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(xScale).tickFormat(d => d + '%'))
      .selectAll('text')
      .style('fill', '#9CA3AF')

    container
      .append('g')
      .call(d3.axisLeft(yScale))
      .selectAll('text')
      .style('fill', '#9CA3AF')

    // Bars
    container
      .selectAll('.productivity-bar')
      .data(sortedData)
      .enter()
      .append('rect')
      .attr('class', 'productivity-bar')
      .attr('x', 0)
      .attr('y', d => yScale(d.project_key) || 0)
      .attr('width', d => xScale(d.resolution_rate))
      .attr('height', yScale.bandwidth())
      .attr('fill', d => {
        if (d.resolution_rate >= 80) return '#10B981'
        if (d.resolution_rate >= 60) return '#F59E0B'
        return '#EF4444'
      })
      .attr('opacity', 0.8)

    // Value labels
    container
      .selectAll('.productivity-label')
      .data(sortedData)
      .enter()
      .append('text')
      .attr('class', 'productivity-label')
      .attr('x', d => xScale(d.resolution_rate) + 5)
      .attr('y', d => (yScale(d.project_key) || 0) + yScale.bandwidth() / 2)
      .attr('dy', '.35em')
      .style('fill', '#E5E7EB')
      .style('font-size', '12px')
      .text(d => `${d.resolution_rate.toFixed(1)}% (${d.resolved_issues}/${d.total_issues})`)

  }, [data])

  if (loading) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-white">Loading analytics dashboard...</div>
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
        <div className="text-gray-400">No analytics data available</div>
      </div>
    )
  }

  return (
    <div className="w-full space-y-6">
      <div className="text-white">
        <h3 className="text-lg font-bold mb-4">📊 Analytics Dashboard</h3>
        
        {/* Summary Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
          <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
            <h4 className="text-sm font-medium text-gray-400">Total Issues</h4>
            <p className="text-2xl font-bold text-blue-400">{data.summary_stats.total_issues}</p>
          </div>
          
          <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
            <h4 className="text-sm font-medium text-gray-400">Total Projects</h4>
            <p className="text-2xl font-bold text-green-400">{data.summary_stats.total_projects}</p>
          </div>
          
          <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
            <h4 className="text-sm font-medium text-gray-400">Resolution Rate</h4>
            <p className="text-2xl font-bold text-purple-400">{data.summary_stats.resolution_rate.toFixed(1)}%</p>
          </div>
          
          <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
            <h4 className="text-sm font-medium text-gray-400">Most Productive</h4>
            <p className="text-2xl font-bold text-yellow-400">{data.summary_stats.most_productive_period}</p>
          </div>
        </div>

        {/* Charts Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Team Velocity Chart */}
          <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
            <h4 className="text-lg font-medium text-white mb-4">Team Velocity (Monthly)</h4>
            <svg ref={velocityChartRef} width="600" height="300" className="w-full h-auto" />
          </div>

          {/* Status Distribution Chart */}
          <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
            <h4 className="text-lg font-medium text-white mb-4">Status Distribution</h4>
            <div className="flex items-center">
              <svg ref={statusChartRef} width="400" height="300" className="flex-shrink-0" />
              <div className="ml-4 space-y-2">
                {data.status_distribution.map((status, index) => (
                  <div key={status.status} className="flex items-center space-x-2 text-sm">
                    <div 
                      className="w-3 h-3 rounded-full"
                      style={{ backgroundColor: ['#10B981', '#F59E0B', '#EF4444', '#8B5CF6', '#6B7280', '#EC4899'][index % 6] }}
                    />
                    <span className="text-gray-300">{status.status}: {status.count} ({status.percentage.toFixed(1)}%)</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Project Productivity Chart */}
        <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
          <h4 className="text-lg font-medium text-white mb-4">Project Productivity (Resolution Rate)</h4>
          <svg ref={productivityChartRef} width="600" className="w-full h-auto" />
        </div>

        {/* Trend Analysis */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
          <TrendChart 
            metric="issues_created" 
            period="week" 
            title="Issue Creation Trend (Weekly)"
            color="#60A5FA"
            width={600}
            height={250}
          />
          <TrendChart 
            metric="issues_resolved" 
            period="week" 
            title="Issue Resolution Trend (Weekly)"
            color="#10B981"
            width={600}
            height={250}
          />
        </div>
        
        <div className="mt-6">
          <TrendChart 
            metric="velocity" 
            period="month" 
            title="Team Velocity Trend (Monthly %)"
            color="#8B5CF6"
            width={1200}
            height={300}
          />
        </div>

        {/* Footer */}
        <div className="text-sm text-gray-400 mt-6">
          Generated: {new Date(data.summary_stats.generated_at).toLocaleString()}
        </div>
      </div>
    </div>
  )
}