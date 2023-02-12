import { useD3 } from './hooks/useD3.js';
import * as d3 from 'd3';
import { svgTranslate, svgPx } from './Utility.js'

function populateIratingHistoryRaceD3JSDiv(div, sessions) {
    let result = sessions.filter((session) => {
        return (
            session["new_irating"] !== -1 &&
            session["event_type"] === 5 && // race
            session["simsession_number"] === 0 &&
            session["license_category"] === 2 // road
        );
    }).map((d, idx) => ({...d, index: idx})); // add index field
    // TODO sort

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
    
    let x = d3.scaleLinear()
        .domain([0, result.length])
        .range([0, width]);

    let y = d3.scaleLinear()
        .domain(d3.extent(result, e => e["new_irating"]))
        .range([height, 0]);
    
    svg.append("g")
        .attr("transform", svgTranslate(0, height))
        .call(d3.axisBottom(x));

    svg.append("g")
        .call(d3.axisLeft(y));

    let line = d3.line()
        .x(d => x(d["index"]))
        .y(d => y(d["new_irating"]));

    svg.append("path")
        .datum(result)
        .attr("fill", "none")
        .attr("stroke", "red")
        .attr("stroke-width", 1.5)
        .attr("d", line);

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
            .html("IRating: " + d["new_irating"])
            .style("left", svgPx(x(d["index"])+10))
            .style("top", svgPx(y(d["new_irating"])))
            .style("visibility", "visible");
    }
    let mouseleave = function(event, d) {
        let marker = getMarkerFromEvent(event);
        marker.setAttribute("opacity", 0)
        tooltip.style("visibility", "hidden");
    }

    let points = svg.append("g")
        .selectAll("rects")
        .data(result)
        .enter()
        .append("g");
    
    points.append("rect")
        .attr("x", function(d) { return x(d["index"]); })
        .attr("y", 0)
        .attr("width", function(d) { return x(1) - x(0); })
        .attr("height", height)
        .attr("opacity", 0)
        .attr("stroke", "none")
        .on("mouseover", mouseover)
        .on("mouseleave", mouseleave);

    points.append("circle")
        .attr("id", marker_id)
        .attr("cx", function(d) { return x(d["index"]); })
        .attr("cy", function(d) { return y(d["new_irating"]); })
        .attr("r", 2)
        .attr("opacity", 0)
        .attr("stroke", "none")
        .attr("fill", "black");
}

function IRatingHistory({driverSessions}) {
    const ref = useD3(
        (root) => {
            populateIratingHistoryRaceD3JSDiv(root, driverSessions);
        },
        [driverSessions]
    );
    
    return <div ref={ref}/>;
}

export default IRatingHistory;