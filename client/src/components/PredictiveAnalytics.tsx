import React, { useEffect, useRef } from 'react'
import * as d3 from 'd3'

interface PredictiveForecast {
  metric: string
  period: string
  predicted_value: number
  confidence_interval: [number, number]
  trend_strength: number
  key_factors: string[]
}

interface ThroughputAnalysis {
  daily_avg: number
  weekly_avg: number
  peak_throughput: number
  throughput_variance: number
  efficiency_opportunities: string[]
}

interface PredictiveAnalyticsProps {
  forecasts: PredictiveForecast[]
  throughput: ThroughputAnalysis
}

export default function PredictiveAnalytics({ forecasts, throughput }: PredictiveAnalyticsProps) {
  const forecastChartRef = useRef<SVGSVGElement>(null)
  const confidenceChartRef = useRef<SVGSVGElement>(null)

  // Draw forecast visualization
  useEffect(() => {
    if (!forecastChartRef.current || forecasts.length === 0) return

    const svg = d3.select(forecastChartRef.current)
    svg.selectAll('*').remove()

    const margin = { top: 20, right: 30, bottom: 40, left: 60 }
    const width = 600 - margin.left - margin.right
    const height = 300 - margin.top - margin.bottom

    const container = svg
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`)

    // Prepare data
    const data = forecasts.map((f, i) => ({
      ...f,
      x: i,
      label: `${f.metric} (${f.period})`
    }))

    // Scales
    const xScale = d3.scaleBand()
      .domain(data.map(d => d.label))
      .range([0, width])
      .padding(0.3)

    const yScale = d3.scaleLinear()
      .domain([0, d3.max(data, d => Math.max(d.predicted_value, d.confidence_interval[1])) || 100])
      .range([height, 0])
      .nice()

    // Axes
    container
      .append('g')
      .attr('transform', `translate(0,${height})`)
      .call(d3.axisBottom(xScale))
      .selectAll('text')
      .style('fill', '#9CA3AF')
      .style('font-size', '10px')
      .attr('transform', 'rotate(-15)')
      .style('text-anchor', 'end')

    container
      .append('g')
      .call(d3.axisLeft(yScale))
      .selectAll('text')
      .style('fill', '#9CA3AF')

    // Confidence intervals (error bars)
    container
      .selectAll('.confidence-interval')
      .data(data)
      .enter()
      .append('line')
      .attr('class', 'confidence-interval')
      .attr('x1', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2)
      .attr('x2', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2)
      .attr('y1', d => yScale(d.confidence_interval[0]))
      .attr('y2', d => yScale(d.confidence_interval[1]))
      .attr('stroke', '#6B7280')
      .attr('stroke-width', 2)

    // Confidence interval caps
    container
      .selectAll('.confidence-cap-low')
      .data(data)
      .enter()
      .append('line')
      .attr('class', 'confidence-cap-low')
      .attr('x1', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2 - 5)
      .attr('x2', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2 + 5)
      .attr('y1', d => yScale(d.confidence_interval[0]))
      .attr('y2', d => yScale(d.confidence_interval[0]))
      .attr('stroke', '#6B7280')
      .attr('stroke-width', 2)

    container
      .selectAll('.confidence-cap-high')
      .data(data)
      .enter()
      .append('line')
      .attr('class', 'confidence-cap-high')
      .attr('x1', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2 - 5)
      .attr('x2', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2 + 5)
      .attr('y1', d => yScale(d.confidence_interval[1]))
      .attr('y2', d => yScale(d.confidence_interval[1]))
      .attr('stroke', '#6B7280')
      .attr('stroke-width', 2)

    // Predicted values (dots)
    container
      .selectAll('.prediction-dot')
      .data(data)
      .enter()
      .append('circle')
      .attr('class', 'prediction-dot')
      .attr('cx', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2)
      .attr('cy', d => yScale(d.predicted_value))
      .attr('r', d => 4 + d.trend_strength * 4)
      .attr('fill', d => {
        if (d.trend_strength > 0.7) return '#10B981' // Strong positive trend
        if (d.trend_strength > 0.3) return '#F59E0B' // Moderate trend
        return '#EF4444' // Weak/negative trend
      })
      .attr('stroke', '#1F2937')
      .attr('stroke-width', 2)

    // Value labels
    container
      .selectAll('.value-label')
      .data(data)
      .enter()
      .append('text')
      .attr('class', 'value-label')
      .attr('x', d => (xScale(d.label) || 0) + xScale.bandwidth() / 2)
      .attr('y', d => yScale(d.predicted_value) - 10)
      .attr('text-anchor', 'middle')
      .style('fill', '#E5E7EB')
      .style('font-size', '12px')
      .style('font-weight', 'bold')
      .text(d => d.predicted_value.toFixed(1))

  }, [forecasts])

  // Draw trend strength chart
  useEffect(() => {
    if (!confidenceChartRef.current || forecasts.length === 0) return

    const svg = d3.select(confidenceChartRef.current)
    svg.selectAll('*').remove()

    const width = 400
    const height = 200
    const radius = Math.min(width, height) / 2 - 20

    const container = svg
      .append('g')
      .attr('transform', `translate(${width / 2}, ${height / 2})`)

    const data = forecasts.map(f => ({
      label: f.metric,
      value: f.trend_strength,
    }))

    const color = d3.scaleOrdinal()
      .domain(data.map(d => d.label))
      .range(['#10B981', '#3B82F6', '#8B5CF6', '#F59E0B'])

    const pie = d3.pie<any>()
      .value(d => d.value)
      .sort(null)

    const arc = d3.arc<any>()
      .innerRadius(radius * 0.5)
      .outerRadius(radius)

    const arcs = container
      .selectAll('.arc')
      .data(pie(data))
      .enter()
      .append('g')
      .attr('class', 'arc')

    arcs
      .append('path')
      .attr('d', arc)
      .attr('fill', d => color(d.data.label) as string)
      .attr('stroke', '#1F2937')
      .attr('stroke-width', 2)

    // Labels
    arcs
      .append('text')
      .attr('transform', d => `translate(${arc.centroid(d)})`)
      .attr('text-anchor', 'middle')
      .style('fill', 'white')
      .style('font-size', '10px')
      .style('font-weight', 'bold')
      .text(d => d.data.label)

  }, [forecasts])

  const getForecastAccuracy = (forecast: PredictiveForecast) => {
    const range = forecast.confidence_interval[1] - forecast.confidence_interval[0]
    const accuracy = Math.max(0, 100 - (range / forecast.predicted_value * 100))
    return accuracy.toFixed(0)
  }

  const getTrendStrengthLabel = (strength: number) => {
    if (strength > 0.7) return { label: 'Strong', color: 'text-green-400' }
    if (strength > 0.4) return { label: 'Moderate', color: 'text-yellow-400' }
    return { label: 'Weak', color: 'text-red-400' }
  }

  return (
    <div className="space-y-6">
      <h3 className="text-2xl font-bold text-white">🔮 Predictive Analytics & Forecasting</h3>

      {/* Throughput Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-blue-900/20 p-4 rounded-lg border border-blue-700">
          <div className="text-2xl font-bold text-blue-400">{throughput.daily_avg.toFixed(1)}</div>
          <div className="text-sm text-blue-300">Daily Avg Throughput</div>
        </div>
        <div className="bg-green-900/20 p-4 rounded-lg border border-green-700">
          <div className="text-2xl font-bold text-green-400">{throughput.weekly_avg.toFixed(0)}</div>
          <div className="text-sm text-green-300">Weekly Avg Issues</div>
        </div>
        <div className="bg-purple-900/20 p-4 rounded-lg border border-purple-700">
          <div className="text-2xl font-bold text-purple-400">{throughput.peak_throughput.toFixed(0)}</div>
          <div className="text-sm text-purple-300">Peak Throughput</div>
        </div>
        <div className="bg-yellow-900/20 p-4 rounded-lg border border-yellow-700">
          <div className="text-2xl font-bold text-yellow-400">{throughput.throughput_variance.toFixed(0)}%</div>
          <div className="text-sm text-yellow-300">Variance</div>
        </div>
      </div>

      {/* Forecast Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
          <h4 className="text-lg font-medium text-white mb-4">📈 Predictions with Confidence Intervals</h4>
          <svg ref={forecastChartRef} width="600" height="300" className="w-full h-auto" />
          
          <div className="mt-4 text-sm text-gray-400">
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full bg-green-500"></div>
                <span>Strong Trend (70%+ confidence)</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
                <span>Moderate Trend</span>
              </div>
              <div className="flex items-center space-x-2">
                <div className="w-3 h-3 rounded-full bg-red-500"></div>
                <span>Weak Trend</span>
              </div>
            </div>
          </div>
        </div>

        <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
          <h4 className="text-lg font-medium text-white mb-4">🎯 Trend Strength Distribution</h4>
          <svg ref={confidenceChartRef} width="400" height="200" className="w-full h-auto" />
          
          <div className="mt-4 space-y-2">
            {forecasts.map((forecast, index) => {
              const { label, color } = getTrendStrengthLabel(forecast.trend_strength)
              return (
                <div key={index} className="flex items-center justify-between text-sm">
                  <span className="text-gray-300">{forecast.metric}</span>
                  <span className={color}>{label} ({(forecast.trend_strength * 100).toFixed(0)}%)</span>
                </div>
              )
            })}
          </div>
        </div>
      </div>

      {/* Detailed Forecasts */}
      <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
        <h4 className="text-lg font-medium text-white mb-4">📊 Detailed Predictions</h4>
        <div className="space-y-4">
          {forecasts.map((forecast, index) => {
            const { label: trendLabel, color: trendColor } = getTrendStrengthLabel(forecast.trend_strength)
            
            return (
              <div key={index} className="bg-gray-900/50 p-4 rounded-lg">
                <div className="flex items-center justify-between mb-3">
                  <h5 className="font-semibold text-white capitalize">
                    {forecast.metric.replace('_', ' ')} - {forecast.period.replace('_', ' ')}
                  </h5>
                  <div className="flex items-center space-x-3">
                    <span className={`text-sm font-medium ${trendColor}`}>
                      {trendLabel} Trend
                    </span>
                    <span className="text-sm text-gray-400">
                      {getForecastAccuracy(forecast)}% Accuracy
                    </span>
                  </div>
                </div>
                
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div>
                    <div className="text-2xl font-bold text-blue-400">
                      {forecast.predicted_value.toFixed(1)}
                      {forecast.metric === 'velocity' ? '%' : ''}
                    </div>
                    <div className="text-sm text-gray-400">Predicted Value</div>
                  </div>
                  
                  <div>
                    <div className="text-lg font-medium text-gray-300">
                      {forecast.confidence_interval[0].toFixed(1)} - {forecast.confidence_interval[1].toFixed(1)}
                      {forecast.metric === 'velocity' ? '%' : ''}
                    </div>
                    <div className="text-sm text-gray-400">Confidence Range</div>
                  </div>
                  
                  <div>
                    <div className="text-lg font-medium text-purple-400">
                      {(forecast.trend_strength * 100).toFixed(0)}%
                    </div>
                    <div className="text-sm text-gray-400">Trend Strength</div>
                  </div>
                </div>
                
                {forecast.key_factors.length > 0 && (
                  <div className="mt-3">
                    <div className="text-sm text-gray-400 mb-2">Key Influencing Factors:</div>
                    <div className="flex flex-wrap gap-2">
                      {forecast.key_factors.map((factor, factorIndex) => (
                        <span
                          key={factorIndex}
                          className="text-xs bg-blue-900/30 text-blue-300 px-2 py-1 rounded"
                        >
                          {factor}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )
          })}
        </div>
      </div>

      {/* Efficiency Opportunities */}
      {throughput.efficiency_opportunities.length > 0 && (
        <div className="bg-gray-800 p-6 rounded-lg border border-gray-700">
          <h4 className="text-lg font-medium text-white mb-4">💡 Efficiency Opportunities</h4>
          <div className="space-y-3">
            {throughput.efficiency_opportunities.map((opportunity, index) => (
              <div key={index} className="flex items-start space-x-3">
                <span className="text-yellow-400 mt-1">•</span>
                <span className="text-gray-300">{opportunity}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}