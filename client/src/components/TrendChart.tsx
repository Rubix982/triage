import React, { useEffect, useRef, useState } from 'react'
import * as d3 from 'd3'

interface TrendData {
  period: string
  metric: string
  value: number
  change_from_previous?: number
}

interface TrendChartProps {
  metric: string
  period: string
  title: string
  color?: string
  width?: number
  height?: number
}

export default function TrendChart({ 
  metric, 
  period, 
  title, 
  color = '#60A5FA',
  width = 600,
  height = 200 
}: TrendChartProps) {
  const svgRef = useRef<SVGSVGElement>(null)
  const [data, setData] = useState<TrendData[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  // Fetch time series data
  useEffect(() => {
    const fetchTrendData = async () => {
      try {
        setLoading(true)
        const response = await fetch(`http://127.0.0.1:3001/api/analytics/timeseries?metric=${metric}&period=${period}`)
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        const trendData: TrendData[] = await response.json()
        setData(trendData)
        setError(null)
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to fetch trend data')
        console.error('Failed to fetch trend data:', err)
      } finally {
        setLoading(false)
      }
    }

    fetchTrendData()
  }, [metric, period])

  // Draw line chart
  useEffect(() => {
    if (!data.length || !svgRef.current) return

    const svg = d3.select(svgRef.current)
    svg.selectAll('*').remove()

    const margin = { top: 20, right: 30, bottom: 40, left: 50 }
    const chartWidth = width - margin.left - margin.right
    const chartHeight = height - margin.top - margin.bottom

    const container = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`)

    // Scales
    const xScale = d3
      .scalePoint()
      .domain(data.map(d => d.period))
      .range([0, chartWidth])
      .padding(0.1)

    const yScale = d3
      .scaleLinear()
      .domain(d3.extent(data, d => d.value) as [number, number])
      .nice()
      .range([chartHeight, 0])

    // Line generator
    const line = d3
      .line<TrendData>()
      .x(d => xScale(d.period) || 0)
      .y(d => yScale(d.value))
      .curve(d3.curveMonotoneX)

    // Add axes
    container
      .append('g')
      .attr('transform', `translate(0,${chartHeight})`)
      .call(d3.axisBottom(xScale))
      .selectAll('text')
      .style('fill', '#9CA3AF')
      .style('font-size', '10px')

    container
      .append('g')
      .call(d3.axisLeft(yScale))
      .selectAll('text')
      .style('fill', '#9CA3AF')
      .style('font-size', '10px')

    // Add grid lines
    container
      .append('g')
      .attr('class', 'grid')
      .attr('transform', `translate(0,${chartHeight})`)
      .call(d3.axisBottom(xScale)
        .tickSize(-chartHeight)
        .tickFormat(() => '')
      )
      .style('stroke-dasharray', '3,3')
      .style('opacity', 0.3)
      .style('stroke', '#6B7280')

    container
      .append('g')
      .attr('class', 'grid')
      .call(d3.axisLeft(yScale)
        .tickSize(-chartWidth)
        .tickFormat(() => '')
      )
      .style('stroke-dasharray', '3,3')
      .style('opacity', 0.3)
      .style('stroke', '#6B7280')

    // Add the line
    container
      .append('path')
      .datum(data)
      .attr('fill', 'none')
      .attr('stroke', color)
      .attr('stroke-width', 2)
      .attr('d', line)

    // Add dots
    container
      .selectAll('.dot')
      .data(data)
      .enter()
      .append('circle')
      .attr('class', 'dot')
      .attr('cx', d => xScale(d.period) || 0)
      .attr('cy', d => yScale(d.value))
      .attr('r', 4)
      .attr('fill', color)
      .attr('stroke', '#1F2937')
      .attr('stroke-width', 2)
      .on('mouseover', function(event, d) {
        // Show tooltip
        const tooltip = d3
          .select('body')
          .append('div')
          .attr('class', 'trend-tooltip')
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
          .html(`
            <strong>${d.period}</strong><br/>
            ${title}: ${d.value.toFixed(metric === 'velocity' ? 1 : 0)}${metric === 'velocity' ? '%' : ''}<br/>
            ${d.change_from_previous ? `Change: ${d.change_from_previous > 0 ? '+' : ''}${d.change_from_previous.toFixed(1)}%` : ''}
          `)
          .style('left', (event.pageX + 10) + 'px')
          .style('top', (event.pageY - 10) + 'px')
          .transition()
          .duration(200)
          .style('opacity', 1)

        // Highlight dot
        d3.select(this)
          .transition()
          .duration(200)
          .attr('r', 6)
          .attr('stroke-width', 3)
      })
      .on('mouseout', function() {
        d3.selectAll('.trend-tooltip').remove()
        
        d3.select(this)
          .transition()
          .duration(200)
          .attr('r', 4)
          .attr('stroke-width', 2)
      })

    // Add value labels
    container
      .selectAll('.value-label')
      .data(data)
      .enter()
      .append('text')
      .attr('class', 'value-label')
      .attr('x', d => xScale(d.period) || 0)
      .attr('y', d => yScale(d.value) - 10)
      .attr('text-anchor', 'middle')
      .style('fill', '#E5E7EB')
      .style('font-size', '10px')
      .style('font-weight', 'bold')
      .text(d => d.value.toFixed(metric === 'velocity' ? 1 : 0))

  }, [data, color, width, height, metric, title])

  if (loading) {
    return (
      <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
        <h4 className="text-lg font-medium text-white mb-4">{title}</h4>
        <div className="flex items-center justify-center h-32">
          <div className="text-gray-400 text-sm">Loading...</div>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
        <h4 className="text-lg font-medium text-white mb-4">{title}</h4>
        <div className="flex items-center justify-center h-32">
          <div className="text-red-400 text-sm">Error: {error}</div>
        </div>
      </div>
    )
  }

  if (!data.length) {
    return (
      <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
        <h4 className="text-lg font-medium text-white mb-4">{title}</h4>
        <div className="flex items-center justify-center h-32">
          <div className="text-gray-400 text-sm">No trend data available</div>
        </div>
      </div>
    )
  }

  // Calculate overall trend
  const firstValue = data[0]?.value || 0
  const lastValue = data[data.length - 1]?.value || 0
  const overallChange = firstValue !== 0 ? ((lastValue - firstValue) / firstValue * 100) : 0
  const trendDirection = overallChange > 0 ? '↗️' : overallChange < 0 ? '↘️' : '➡️'
  const trendColor = overallChange > 0 ? 'text-green-400' : overallChange < 0 ? 'text-red-400' : 'text-gray-400'

  return (
    <div className="bg-gray-800 p-4 rounded-lg border border-gray-700">
      <div className="flex items-center justify-between mb-4">
        <h4 className="text-lg font-medium text-white">{title}</h4>
        <div className="flex items-center space-x-2">
          <span className="text-2xl">{trendDirection}</span>
          <span className={`text-sm font-medium ${trendColor}`}>
            {overallChange > 0 ? '+' : ''}{overallChange.toFixed(1)}%
          </span>
        </div>
      </div>
      <svg ref={svgRef} width={width} height={height} className="w-full h-auto" />
      <div className="text-xs text-gray-400 mt-2">
        Last {data.length} {period}s • Latest: {lastValue.toFixed(metric === 'velocity' ? 1 : 0)}{metric === 'velocity' ? '%' : ''}
      </div>
    </div>
  )
}