import * as d3 from 'd3';
import { svgTranslate, svgPx } from './Utility.js';
import { theme } from './Theme.js';

export function linePlot(
    div,
    data,
    xFunc, // e => e["start_time"]
    yFunc, // e => e["new_irating"]
    // {
    //    horizontalLanes: [{min: X, max: Y, color: C}, ...]
    //    lineColor: color
    // }
    style, 
) {
    let margin = {top: 10, right: 30, bottom: 30, left: 60},
        width = 1200 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    let fullWidth = width + margin.left + margin.right;
    let fullHeight = height + margin.top + margin.bottom;

    // append the svg object to the body of the page
    let svg = d3.select(div)
        .append("svg")
            .attr("preserveAspectRatio", "xMinYMin meet")
            .attr("viewBox", `0 0 ${fullWidth} ${fullHeight}`)
            // .attr("width", width + margin.left + margin.right)
            // .attr("height", height + margin.top + margin.bottom)
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

    let yExtent = d3.extent(data, yFunc);
    let y = d3.scaleLinear()
        .domain(yExtent)
        .range([height, 0]);

    if (style.horizontalLanes) {
        style.horizontalLanes.forEach(lane => {
            let min = Math.max(lane.min, yExtent[0]);
            let max = Math.min(lane.max, yExtent[1]);
            svg.append("rect")
                .attr("fill", lane.color)
                .attr("x", x(xExtent[0]))
                .attr("width", x(xExtent[1]) - x(xExtent[0]))
                .attr("y", y(max))
                .attr("height", y(min) - y(max));
        });
    }
    
    svg.append("g")
        .attr("transform", svgTranslate(0, height))
        .call(d3.axisBottom(x));

    svg.append("g")
        .call(d3.axisLeft(y));

    let line = d3.line()
        .curve(d3.curveStepAfter)
        .x(d => x(xFunc(d)))
        .y(d => y(yFunc(d)));

    let lineColor = style.lineColor || "red";

    svg.append("path")
        .datum(data)
        .attr("fill", "none")
        .attr("stroke", lineColor)
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

export function verticalBarChart(
    div,
    data,
    xFunc,
    yFunc,
    format)
{
    let xTickFormat = format?.xTickFormat ?? (e => e);
    let yTickFormat = format?.yTickFormat ?? (e => e);
    let barFill = format?.barFill ?? "red";

    // Bar charts with few lanes are a bit crowded. Something wrong with the math here
    let coreHeight = data.length * 20;
    let margin = {top: 20, right: 80, bottom: 40, left: 250},
        width = 1000 - margin.left - margin.right,
        height = coreHeight - margin.top - margin.bottom;

    let fullWidth = width + margin.left + margin.right;
    let fullHeight = height + margin.top + margin.bottom;

    // append the svg object to the body of the page
    let svg = d3.select(div)
        .append("svg")
            .attr("preserveAspectRatio", "xMinYMin meet")
            .attr("viewBox", `0 0 ${fullWidth} ${fullHeight}`)
            // .attr("width", width + margin.left + margin.right)
            // .attr("height", height + margin.top + margin.bottom)
        .append("g")
            .attr("transform", svgTranslate(margin.left, margin.top));

    let x = d3.scaleLinear()
        .domain([0, d3.max(data, xFunc)])
        .range([0, width]);

    let y = d3.scaleBand()
        .domain(data.map(yFunc))
        .range([0, height])
        .padding(0.1);

    svg.append("g")
        .attr("transform", svgTranslate(0, height))
        .call(d3.axisBottom(x).tickFormat(xTickFormat));

    svg.append("g")
        .call(d3.axisLeft(y).tickFormat(yTickFormat));

    svg.selectAll("bars")
        .data(data)
        .join((enter) => {
            enter.append("rect")
                .attr("x", x(0))
                .attr("y", d => y(yFunc(d)))
                .attr("width", d => x(xFunc(d)))
                .attr("height", y.bandwidth())
                .attr("fill", barFill);
            enter.append("text")
                .attr("x", d => x(xFunc(d)))
                .attr("y", d => y(yFunc(d)) + 0*y.bandwidth() / 2)
                .attr("text-anchor", "left")
                .attr("font-size", y.bandwidth() * 0.8)
                .attr("dx", "0.5em")
                .attr("dy", "0.9em")
                .attr("fill", theme.palette.text.primary)
                // .attr("fill-opacity", 0.7)
                .text(d => xTickFormat(xFunc(d)))
        });
            
}
