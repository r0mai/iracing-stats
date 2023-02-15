import * as d3 from 'd3';
import { curveNatural } from 'd3';
import { svgTranslate, svgPx } from './Utility.js'

export function linePlot(
    div,
    data,
    xFunc, // e => e["start_time"]
    yFunc, // e => e["new_irating"]
) {
    let margin = {top: 10, right: 30, bottom: 30, left: 60},
        width = 800 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    // append the svg object to the body of the page
    let svg = d3.select(div)
        .append("svg")
            .attr("width", width + margin.left + margin.right)
            .attr("height", height + margin.top + margin.bottom)
        .append("g")
            .attr("transform", svgTranslate(margin.left, margin.top));
    
    // let x = d3.scaleLinear()
    //     .domain([0, data.length])
    //     .range([0, width]);
    let xExtent = d3.extent(data, xFunc);
    xExtent[1] = Math.max(xExtent[1], new Date(performance.timing.domLoading));
    let x = d3.scaleTime()
        .domain(xExtent)
        .range([0, width]);

    let y = d3.scaleLinear()
        .domain(d3.extent(data, yFunc))
        .range([height, 0]);
    
    svg.append("g")
        .attr("transform", svgTranslate(0, height))
        .call(d3.axisBottom(x));

    svg.append("g")
        .call(d3.axisLeft(y));

    let line = d3.line()
        .curve(d3.curveStepAfter)
        .x(d => x(xFunc(d)))
        .y(d => y(yFunc(d)));

    svg.append("path")
        .datum(data)
        .attr("fill", "none")
        .attr("stroke", "red")
        .attr("stroke-width", 1.5)
        .attr("d", line);

    if (0) {
        // Tooltip
        let tooltip = d3.select(div)
            .append("div")
            .style("visibility", "hidden")
            .attr("class", "tooltip")

        let marker_id = "marker-cirle";
        let getMarkerFromEvent = function(event) {
            return event.currentTarget.parentElement.querySelector(`#${marker_id}`);
        }

        let mouseover = function(event, d) {
            let marker = getMarkerFromEvent(event);
            marker.setAttribute("opacity", 1)
            tooltip
                .html("IRating: " + xFunc(d))
                .style("left", svgPx(x(xFunc(d))+10))
                .style("top", svgPx(y(yFunc(d))))
                .style("visibility", "visible");
        }
        let mouseleave = function(event, d) {
            let marker = getMarkerFromEvent(event);
            marker.setAttribute("opacity", 0)
            tooltip.style("visibility", "hidden");
        }

        let points = svg.append("g")
            .selectAll("rects")
            .data(data)
            .enter()
            .append("g");
        
        points.append("rect")
            .attr("x", function(d) { return x(xFunc(d)); })
            .attr("y", 0)
            .attr("width", function(d) { return x(1) - x(0); })
            .attr("height", height)
            .attr("opacity", 0)
            .attr("stroke", "none")
            .on("mouseover", mouseover)
            .on("mouseleave", mouseleave);

        points.append("circle")
            .attr("id", marker_id)
            .attr("cx", function(d) { return x(xFunc(d)); })
            .attr("cy", function(d) { return y(yFunc(d)); })
            .attr("r", 2)
            .attr("opacity", 0)
            .attr("stroke", "none")
            .attr("fill", "black");
    }
}