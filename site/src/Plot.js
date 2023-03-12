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

    let lineColor = style.lineColor || "red";
    let horizontalLanes = style.horizontalLanes || [];

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

    horizontalLanes.forEach(lane => {
        let min = Math.max(lane.min, yExtent[0]);
        let max = Math.min(lane.max, yExtent[1]);
        svg.append("rect")
            .attr("fill", lane.color)
            .attr("x", x(xExtent[0]))
            .attr("width", x(xExtent[1]) - x(xExtent[0]))
            .attr("y", y(max))
            .attr("height", y(min) - y(max));
    });
    
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

function range(start, end) {
    let res = [];
    for (let i = start; i <= end; ++i) {
        res.push(i);
    }
    return res;
}

const oneJanLookUpTable = (() => {
    let table = [];
    for (let y = 2000; y <= 2030; ++y) {
        table[y - 2000] = new Date(y, 0, 1);
    }
    return table;
})();

function lookupOneJan(year) {
    return oneJanLookUpTable[year - 2000];
}

// Adapted from https://stackoverflow.com/questions/6117814/get-week-of-year-in-javascript-like-in-php
function getWeekNumber(date) {
    let onejan = lookupOneJan(date.getFullYear());
    let dayIndex = (date.getTime() - onejan.getTime()) / 86400000;
    let week = Math.ceil((dayIndex + onejan.getDay() + 1) / 7);
    return week - 1;
}

function ywdToKey(year, week, day) {
    return day + 10 * week + 1000 * year;
}

function dateToKey(date) {
    let year = date.getFullYear();
    let week = getWeekNumber(date);
    let day = date.getDay();
    return ywdToKey(year, week, day);
}

export function yearlyFrequencyMap(
    div,
    data,
    dateFunc,
    valueFunc,
    formatValue)
{
    let dateExtent = d3.extent(data, dateFunc);
    let startYear = dateExtent[0].getFullYear();
    let endYear = dateExtent[1].getFullYear();

    let frequencyMap = new Map();

    let maxValue = 0;
    for (let session of data) {
        let date = dateFunc(session);
        let key = dateToKey(date);
        let value = frequencyMap.get(key);
        if (value === undefined) {
            value = {
                date: new Date(date.getFullYear(), date.getMonth(), date.getDate()),
                value: 0
            };
            frequencyMap.set(key, value);
        }
        value.value += valueFunc(session);
        maxValue = Math.max(value.value, maxValue);
    }

    let rectW = 10;
    let rectH = 10;
    let offsetX = rectW + 2;
    let offsetY = rectH + 2;
    let yearOffsetY = 7 * offsetY + 6;
    let leftMargin = 50;
    let rightMargin = 100;

    let svg = d3.select(div)
        .append('svg')
        .attr("width", 55 * offsetX + leftMargin + rightMargin)
        .attr("height", (endYear - startYear + 1) * yearOffsetY)
        ;

    let colorScale = d3.scaleLinear()
        .domain([0, maxValue])
        .range(["#aaa", "blue"])
        ;

    // legend
    {
        let defs = svg.append("defs");
        let scaleGradient = defs.append("linearGradient")
            .attr("id", "scaleGradient")
            .attr("x1", "0%")
            .attr("y1", "100%")
            .attr("x2", "0%")
            .attr("y2", "0%");
        
        scaleGradient.append("stop")
            .attr("offset", "0%")
            .attr("stop-color", colorScale(0));
        scaleGradient.append("stop")
            .attr("offset", "100%")
            .attr("stop-color", colorScale(maxValue));

        let legendG = svg.append("g")
            .attr("transform", svgTranslate(55 * offsetX + leftMargin + offsetX, offsetY));
        legendG.append("rect")
            .attr("x", 0)
            .attr("y", 0)
            .attr("width", rectW * 1)
            .attr("height", offsetY * 5)
            .attr("rx", rectW * 0.2)
            .attr("fill", "url(#scaleGradient)")
            ;
        legendG.append("text")
            .attr("x", rectW * 2)
            .attr("y", 0)
            .attr("dy", "0.5em")
            .attr("fill", theme.palette.text.primary)
            .text(formatValue(maxValue))
            ;
        legendG.append("text")
            .attr("x", rectW * 2)
            .attr("y", offsetY * 5)
            .attr("dy", "0.5em")
            .attr("fill", theme.palette.text.primary)
            .text(formatValue(0))
            ;
    }

    // tooltip
    let tooltipWidth = 150;
    let tooltip = d3.select(div)
        .append("div")
        .style("visibility", "hidden")
        .style("position", "absolute")
        .style("width", svgPx(tooltipWidth))
        .style("max-width", svgPx(tooltipWidth))
        .style("background-color", "gray")
        .style("text-align", "center")
        ;

    for (let y = endYear; y >= startYear; --y) {
        let originY = (endYear - y) * yearOffsetY;

        svg.append("text")
            .attr("x", 5)
            .attr("y", originY + 0.5 * yearOffsetY)
            .attr("fill", theme.palette.text.primary)
            .text(`${y}`)
            ;

        let yearG = svg.append("g")
            .attr("transform", svgTranslate(leftMargin, originY))
            ;

        let firstDay = lookupOneJan(y);
        let lastDay = new Date(y, 11, 31);

        let firstDayDayIdx = firstDay.getDay();
        let lastDayDayIdx = lastDay.getDay();
        let lastWeekIdx = getWeekNumber(lastDay);

        for (let w = 0; w <= lastWeekIdx; ++w) {
            let startD = w == 0 ? firstDayDayIdx : 0;
            let lastD = w == lastWeekIdx ? lastDayDayIdx: 6;
            for (let d = startD; d <= lastD; ++d) {
                let key = ywdToKey(y, w, d);
                let color = undefined;
                let value = frequencyMap.get(key);
                if (value === undefined) {
                    color = "#444";
                } else {
                    color = colorScale(value.value);
                }

                let rect = yearG.append("rect")
                    .attr("x", offsetX * w)
                    .attr("y", offsetY * d)
                    .attr("width", rectW)
                    .attr("height", rectH)
                    .attr("rx", rectW * 0.2)
                    .attr("fill", color)

                if (value !== undefined) {
                    let mouseover = function(event) {
                        tooltip
                            .html(value.date.toDateString() + "<br/>" + formatValue(value.value))
                            .style("left", svgPx(event.pageX - tooltipWidth * 0.5))
                            .style("top", svgPx(event.pageY + 10))
                            .style("visibility", "visible");
                    };
                    let mouseleave = function(event) {
                        tooltip.style("visibility", "hidden");
                    };

                    rect
                        .on("mousemove", mouseover)
                        .on("mouseout", mouseleave);
                        ;
                }
            }
        }
    }
}