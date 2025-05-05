import { Runtime, Inspector } from "@observablehq/runtime";
import notebook from "./notebooks/issue-graph"; // Replace with your .js notebook module
import d3 from "d3";

export function renderNotebook(targetElement: HTMLElement, data: any) {
  const runtime = new Runtime();
  const main = runtime.module(notebook, (name: any) => {
    if (name === "chart") return new Inspector(targetElement);
  });

  // Send data to Observable notebook's defined inputs
  main.redefine("data", data);
}

export function renderVisualizer(data: any[]) {
  const container = document.getElementById("app")!;
  container.innerHTML = '<svg width="800" height="600"></svg>';
  const svg = d3.select("svg");

  svg
    .selectAll("circle")
    .data(data)
    .enter()
    .append("circle")
    .attr("cx", (_, i) => 100 + i * 20)
    .attr("cy", 300)
    .attr("r", 10)
    .style("fill", "steelblue");
}
