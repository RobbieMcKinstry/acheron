import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { JSDOM } from "jsdom";
import * as d3 from "d3";

const __dirname = dirname(fileURLToPath(import.meta.url));
const resultsPath = join(__dirname, "results.json");
const outputPath = join(__dirname, "throughput.svg");

// ─── Load data ──────────────────────────────────────────────────────────────
const results = JSON.parse(readFileSync(resultsPath, "utf-8"));
if (!Array.isArray(results) || results.length === 0) {
  console.error("results.json is empty or not an array.");
  process.exit(1);
}

// ─── Chart dimensions ───────────────────────────────────────────────────────
const margin = { top: 50, right: 30, bottom: 50, left: 70 };
const barWidth = 80;
const gap = 40;
const chartWidth = results.length * (barWidth + gap) - gap;
const width = margin.left + chartWidth + margin.right;
const height = 400;
const innerHeight = height - margin.top - margin.bottom;

// ─── Virtual DOM ────────────────────────────────────────────────────────────
const dom = new JSDOM("<!DOCTYPE html><html><body></body></html>");
const document = dom.window.document;
const body = d3.select(document.body);

const svg = body
  .append("svg")
  .attr("xmlns", "http://www.w3.org/2000/svg")
  .attr("width", width)
  .attr("height", height)
  .attr("viewBox", `0 0 ${width} ${height}`)
  .attr("font-family", "system-ui, sans-serif");

// Background
svg
  .append("rect")
  .attr("width", width)
  .attr("height", height)
  .attr("fill", "#fafafa");

const g = svg
  .append("g")
  .attr("transform", `translate(${margin.left},${margin.top})`);

// ─── Scales ─────────────────────────────────────────────────────────────────
const x = d3
  .scaleBand()
  .domain(results.map((d) => d.tag))
  .range([0, chartWidth])
  .padding(0.3);

const maxSolved = d3.max(results, (d) => d.solved);
const y = d3
  .scaleLinear()
  .domain([0, maxSolved * 1.15])
  .nice()
  .range([innerHeight, 0]);

// ─── Axes ───────────────────────────────────────────────────────────────────
// X axis
g.append("g")
  .attr("transform", `translate(0,${innerHeight})`)
  .call(d3.axisBottom(x))
  .selectAll("text")
  .attr("font-size", "13px");

// Y axis
g.append("g").call(d3.axisLeft(y).ticks(6)).selectAll("text").attr("font-size", "12px");

// Y axis label
g.append("text")
  .attr("transform", "rotate(-90)")
  .attr("x", -innerHeight / 2)
  .attr("y", -50)
  .attr("text-anchor", "middle")
  .attr("font-size", "14px")
  .attr("fill", "#333")
  .text("Problems Solved");

// ─── Bars ───────────────────────────────────────────────────────────────────
const color = d3
  .scaleOrdinal()
  .domain(results.map((d) => d.tag))
  .range(d3.schemeTableau10);

g.selectAll(".bar")
  .data(results)
  .enter()
  .append("rect")
  .attr("class", "bar")
  .attr("x", (d) => x(d.tag))
  .attr("y", (d) => y(d.solved))
  .attr("width", x.bandwidth())
  .attr("height", (d) => innerHeight - y(d.solved))
  .attr("fill", (d) => color(d.tag))
  .attr("rx", 3);

// ─── Bar labels ─────────────────────────────────────────────────────────────
g.selectAll(".label")
  .data(results)
  .enter()
  .append("text")
  .attr("class", "label")
  .attr("x", (d) => x(d.tag) + x.bandwidth() / 2)
  .attr("y", (d) => y(d.solved) - 8)
  .attr("text-anchor", "middle")
  .attr("font-size", "13px")
  .attr("font-weight", "600")
  .attr("fill", "#333")
  .text((d) => d.solved);

// ─── Title ──────────────────────────────────────────────────────────────────
svg
  .append("text")
  .attr("x", width / 2)
  .attr("y", 30)
  .attr("text-anchor", "middle")
  .attr("font-size", "18px")
  .attr("font-weight", "bold")
  .attr("fill", "#222")
  .text("Throughput: Problems Solved in 60s");

// ─── Serialize ──────────────────────────────────────────────────────────────
const svgHtml = body.select("svg").node().outerHTML;
writeFileSync(outputPath, svgHtml, "utf-8");
console.log(`SVG written to ${outputPath}`);
