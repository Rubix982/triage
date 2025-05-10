import React, { useEffect, useRef } from 'react'
import * as d3 from 'd3'

export default function MyChart() {
  const ref = useRef<SVGSVGElement | null>(null)

  useEffect(() => {
    if (!ref.current) return

    const data = [25, 40, 15, 60, 20]

    const svg = d3.select(ref.current)
    svg.selectAll('*').remove() // clear previous renders

    const width = 300
    const height = 100
    const barWidth = width / data.length

    svg.attr('width', width).attr('height', height)

    svg
      .selectAll('rect')
      .data(data)
      .enter()
      .append('rect')
      .attr('x', (_, i) => i * barWidth)
      .attr('y', (d) => height - d)
      .attr('width', barWidth - 4)
      .attr('height', (d) => d)
      .attr('fill', '#60a5fa')
  }, [])

  return (
    <div className="p-4">
      <svg ref={ref} className="rounded border border-gray-500" />
    </div>
  )
}
