import React, { useEffect, useRef } from 'react'
import * as d3 from 'd3'

interface PerformanceScoreCardProps {
  score: number
  trend: string
  className?: string
}

export default function PerformanceScoreCard({ score, trend, className = '' }: PerformanceScoreCardProps) {
  const gaugeRef = useRef<SVGSVGElement>(null)

  useEffect(() => {
    if (!gaugeRef.current) return

    const svg = d3.select(gaugeRef.current)
    svg.selectAll('*').remove()

    const width = 200
    const height = 120
    const radius = 80

    const container = svg
      .append('g')
      .attr('transform', `translate(${width / 2}, ${height - 10})`)

    // Background arc
    const backgroundArc = d3.arc()
      .innerRadius(radius - 15)
      .outerRadius(radius)
      .startAngle(-Math.PI / 2)
      .endAngle(Math.PI / 2)

    container
      .append('path')
      .attr('d', backgroundArc as any)
      .attr('fill', '#374151')

    // Score arc
    const scoreAngle = -Math.PI / 2 + (score / 100) * Math.PI
    const scoreArc = d3.arc()
      .innerRadius(radius - 15)
      .outerRadius(radius)
      .startAngle(-Math.PI / 2)
      .endAngle(scoreAngle)

    // Color based on score
    const getScoreColor = (score: number) => {
      if (score >= 80) return '#10B981' // Green
      if (score >= 60) return '#F59E0B' // Yellow
      if (score >= 40) return '#EF4444' // Red
      return '#6B7280' // Gray
    }

    container
      .append('path')
      .attr('d', scoreArc as any)
      .attr('fill', getScoreColor(score))
      .style('filter', 'drop-shadow(0 0 6px rgba(59, 130, 246, 0.5))')

    // Score text
    container
      .append('text')
      .attr('text-anchor', 'middle')
      .attr('y', -10)
      .style('font-size', '28px')
      .style('font-weight', 'bold')
      .style('fill', getScoreColor(score))
      .text(Math.round(score))

    container
      .append('text')
      .attr('text-anchor', 'middle')
      .attr('y', 8)
      .style('font-size', '10px')
      .style('fill', '#9CA3AF')
      .text('PERFORMANCE SCORE')

    // Trend indicator
    const getTrendSymbol = (trend: string) => {
      switch (trend) {
        case 'accelerating':
        case 'improving':
          return '↗'
        case 'declining':
          return '↘'
        case 'stable':
          return '→'
        default:
          return '?'
      }
    }

    container
      .append('text')
      .attr('text-anchor', 'middle')
      .attr('y', 25)
      .style('font-size', '16px')
      .style('fill', getScoreColor(score))
      .text(getTrendSymbol(trend))

  }, [score, trend])

  const getScoreDescription = (score: number) => {
    if (score >= 80) return { text: 'Excellent Performance', color: 'text-green-400' }
    if (score >= 60) return { text: 'Good Performance', color: 'text-yellow-400' }
    if (score >= 40) return { text: 'Needs Improvement', color: 'text-red-400' }
    return { text: 'Critical Issues', color: 'text-red-500' }
  }

  const { text: description, color: descriptionColor } = getScoreDescription(score)

  return (
    <div className={`bg-gray-800 p-6 rounded-xl border border-gray-700 ${className}`}>
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-xl font-bold text-white mb-2">🎯 Team Performance Score</h3>
          <p className={`text-lg font-semibold ${descriptionColor}`}>{description}</p>
          <p className="text-sm text-gray-400 mt-2">
            Composite score based on velocity, flow efficiency, quality, and consistency
          </p>
          
          {/* Key Factors */}
          <div className="mt-4 space-y-1">
            <div className="text-xs text-gray-500">Key Factors:</div>
            <div className="flex space-x-4 text-xs">
              <span className="text-blue-400">• Velocity (30%)</span>
              <span className="text-green-400">• Flow (25%)</span>
              <span className="text-purple-400">• Quality (30%)</span>
              <span className="text-yellow-400">• Consistency (15%)</span>
            </div>
          </div>
        </div>
        
        <div className="flex-shrink-0">
          <svg ref={gaugeRef} width="200" height="120" />
        </div>
      </div>
    </div>
  )
}