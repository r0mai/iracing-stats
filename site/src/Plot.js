import * as d3 from 'd3';
import { svgTranslate, svgPx } from './Utility.js';
import { theme } from './Theme.js';
import { svgRotate } from './Utility.js';
import { attachTooltipToElement, createTooltip } from './Tooltip.js';

function multiDimensionalExtent(marray, func) {
    let min = Infinity;
    let max = -Infinity;
    marray.forEach((arr) => {
        if (arr.length > 0) {
            let extent = d3.extent(arr, func);
            min = Math.min(min, extent[0]);
            max = Math.max(max, extent[1]);
        }
    });
    return [min, max];
}

export function linePlot(
    div,
    data,  // array of values
    xFunc, // e => e["start_time"]
    yFunc, // e => e["new_irating"]
    // {
    //    horizontalLanes: [{min: X, max: Y, color: C}, ...]
    //    lineColors: [color1, color2, ...]
    //    showHorizontalGridLines: bool,
    //    showVerticalGridLines: bool,
    //    legendLabels: ["label1", "label2", ...]
    // }
    style, 
) {

    let horizontalLanes = style.horizontalLanes || [];
    let showHorizontalGridLines = style.showHorizontalGridLines || false;
    let showVerticalGridLines = style.showVerticalGridLines || false;

    let lineColors = style.lineColors || [];
    for (let i = 0; i < data.length; ++i) {
        lineColors[i] = lineColors[i] || "red";
    }

    let margin = {top: 10, right: 30, bottom: 30, left: 60},
        width = 1200 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    let fullWidth = width + margin.left + margin.right;
    let fullHeight = height + margin.top + margin.bottom + 100; // + 100 for the labels

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
    let xExtent = multiDimensionalExtent(data, xFunc);
    xExtent[1] = Math.max(xExtent[1], new Date(performance.timing.domLoading));
    let x = d3.scaleTime()
        .domain(xExtent)
        .range([0, width]);

    let yExtent = multiDimensionalExtent(data, yFunc);
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

    // Grid lines:
    // https://www.essycode.com/posts/adding-gridlines-chart-d3/
    if (showHorizontalGridLines) {
        svg.append("g")
            .call(d3.axisLeft(y)
                .tickSize(-width)
                .tickFormat(''))
            .call(g => g.selectAll(".tick line")
                .attr("stroke-opacity", 0.3))
            .call(g => g.selectAll(".domain")
                .remove())
            ;
    }

    if (showVerticalGridLines) {
        svg.append("g")
            .call(d3.axisBottom(x)
                .tickSize(height)
                .tickFormat(''))
            .call(g => g.selectAll(".tick line")
                .attr("stroke-opacity", 0.3))
            .call(g => g.selectAll(".domain")
                .remove())
            ;
    }


    let line = d3.line()
        .curve(d3.curveStepAfter)
        .x(d => x(xFunc(d)))
        .y(d => y(yFunc(d)));

    for (let i = 0; i < data.length; ++i) {
        svg.append("path")
            .datum(data[i])
            .attr("fill", "none")
            .attr("stroke", lineColors[i])
            .attr("stroke-width", 1.5)
            .attr("d", line);
    }

    if (style.legendLabels !== undefined) {
        let legendG = svg.append("g")
            .attr("transform", svgTranslate(0, height + 25))
        let y = 0;
        for (let i = 0; i < data.length; ++i) {
            let labelText = style.legendLabels[i] || "???";
            legendG.append("line")
                .attr("x1", 0)
                .attr("y1", y)
                .attr("x2", 12)
                .attr("y2", y)
                .style("stroke", lineColors[i])
                .style("stroke-width", 1.5);
            legendG.append("text")
                .attr("x", 14)
                .attr("y", y + 7)
                .text(labelText)
                .style("font-size", "14px")
                .attr("alignment-baseline", "middle")
                .attr("fill", theme.palette.text.primary)
            y += 20;
        }
    }

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

const oneJanLookUpTable = (() => {
    let table = [];
    for (let y = 2000; y <= 2030; ++y) {
        table[y - 2000] = new Date(Date.UTC(y, 0, 1));
    }
    return table;
})();

function lookupOneJan(year) {
    return oneJanLookUpTable[year - 2000];
}

// Makes Monday the 0th day
function getDay(date) {
    let idx = date.getUTCDay();
    return [6, 0, 1, 2, 3, 4, 5][idx];
}

// Adapted from https://stackoverflow.com/questions/6117814/get-week-of-year-in-javascript-like-in-php
function getWeekNumber(date) {
    let onejan = lookupOneJan(date.getUTCFullYear());
    let dayIndex = (date.getTime() - onejan.getTime()) / 86400000;
    let week = Math.ceil((dayIndex + getDay(onejan) + 1) / 7);
    return week - 1;
}

function dateToYMDKey(date) {
    let year = date.getUTCFullYear();
    let month = date.getUTCMonth();
    let day = date.getUTCDate();
    return day + 100 * month + 10000 * year;
}

function lerp(min, max, t) {
    return (1-t) * min + t * max;
}

function createColorScale(minValue, maxValue) {
    return d3.scaleLinear()
        .domain([
            minValue,
            lerp(minValue, maxValue, 0.5),
            maxValue])
        .range(["#d1cef1", "#443cc6", "#25226d"])
        ;
}

function appendColorScaleLegend(parent, colorScale, width, height, maxValue, formatValue) {
    let legendG = parent.append("g");
    let defs = legendG.append("defs");
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
        .attr("offset", "50%")
        .attr("stop-color", colorScale(maxValue * 0.5));
    scaleGradient.append("stop")
        .attr("offset", "100%")
        .attr("stop-color", colorScale(maxValue));

    legendG.append("rect")
        .attr("x", 0)
        .attr("y", 0)
        .attr("width", width)
        .attr("height", height)
        .attr("rx", width * 0.2)
        .attr("fill", "url(#scaleGradient)")
        ;
    legendG.append("text")
        .attr("x", width * 2)
        .attr("y", 0)
        .attr("dy", "0.5em")
        .attr("fill", theme.palette.text.primary)
        .text(formatValue(maxValue))
        ;
    legendG.append("text")
        .attr("x", width * 2)
        .attr("y", height)
        .attr("dy", "0.5em")
        .attr("fill", theme.palette.text.primary)
        .text(formatValue(0))
        ;
    return legendG;
}

export function yearlyFrequencyMap(
    div,
    data,
    dateFunc,
    valueFunc,
    formatValue)
{
    let dateExtent = d3.extent(data, dateFunc);
    let startYear = dateExtent[0].getUTCFullYear();
    let endYear = dateExtent[1].getUTCFullYear();

    let frequencyData = [];
    let currentDate = new Date(lookupOneJan(startYear).getTime());
    let lastKey = dateToYMDKey(new Date(Date.UTC(endYear, 11, 31)));
    let sessionIdx = 0;
    let maxValue = 0;
    while (true) {
        let currentKey = dateToYMDKey(currentDate);

        let value = undefined;
        while (sessionIdx < data.length) {
            let session = data[sessionIdx];
            let sessionDate = dateFunc(session);
            let sessionKey = dateToYMDKey(sessionDate);
            if (sessionKey !== currentKey) {
                break;
            }

            value = value || 0;
            value += valueFunc(session);
            sessionIdx += 1;
        }

        maxValue = Math.max(value || 0, maxValue);
        frequencyData.push({
            date: new Date(currentDate.getTime()),
            value: value
        });

        currentDate.setUTCDate(currentDate.getUTCDate() + 1);
        if (currentKey === lastKey) {
            break;
        }
    }

    let rectW = 10;
    let rectH = 10;
    let gapX = 2;
    let gapY = 2;
    let offsetX = rectW + gapX;
    let offsetY = rectH + gapY;
    let yearOffsetY = 7 * offsetY + 6;
    let leftMargin = 80;
    let rightMargin = 100;

    let svg = d3.select(div)
        .append('svg')
        .attr("width", 55 * offsetX + leftMargin + rightMargin)
        .attr("height", (endYear - startYear + 1) * yearOffsetY)
        ;

    let colorScale = createColorScale(0, maxValue);

    // legend
    appendColorScaleLegend(svg, colorScale, rectW, offsetY * 5, maxValue, formatValue)
        .attr("transform", svgTranslate(55 * offsetX + leftMargin + offsetX, offsetY));

    // tooltip
    let tooltipWidth = 150;
    let tooltip = createTooltip(div, tooltipWidth);

    let dataIdx = 0;
    for (let y = startYear; y <= endYear; ++y) {
        let originY = (endYear - y) * yearOffsetY;

        let yearG = svg.append("g")
            .attr("transform", svgTranslate(leftMargin, originY));

        // year text
        yearG.append("text")
            .attr("x", -75)
            .attr("y", 0.5 * yearOffsetY)
            .attr("fill", theme.palette.text.primary)
            .text(`${y}`);

        // week day names
        {
            let names = ["Mon", "Thu", "Sun"];
            let offsets = [0, 3, 6];

            for (let i = 0; i < names.length; ++i) {
                yearG.append("text")
                    .attr("x", -25)
                    .attr("y", offsetY * offsets[i])
                    .attr("dy", "0.9em")
                    .attr("font-size", rectH)
                    .attr("fill", theme.palette.text.primary)
                    .text(names[i]);
            }
        }

        let firstDay = lookupOneJan(y);
        let lastDay = new Date(Date.UTC(y, 11, 31));

        let firstDayDayIdx = getDay(firstDay);
        let lastDayDayIdx = getDay(lastDay);
        let lastWeekIdx = getWeekNumber(lastDay);

        for (let w = 0; w <= lastWeekIdx; ++w) {
            let startD = w === 0 ? firstDayDayIdx : 0;
            let lastD = w === lastWeekIdx ? lastDayDayIdx : 6;
            for (let d = startD; d <= lastD; ++d) {
                let dayData = frequencyData[dataIdx];

                let color = undefined;
                if (dayData.value === undefined) {
                    color = "#444";
                } else {
                    color = colorScale(dayData.value);
                }

                let year = dayData.date.getUTCFullYear();
                let month = dayData.date.getUTCMonth();
                let day = dayData.date.getUTCDate();
                let dayName = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'][dayData.date.getUTCDay()];
                let tooltipHtml =
                    `${year}/${month + 1}/${day} ${dayName}<br/><b>${formatValue(dayData.value)}</b>`;

                let rect = yearG.append("rect")
                    .attr("x", offsetX * w)
                    .attr("y", offsetY * d)
                    .attr("width", rectW)
                    .attr("height", rectH)
                    .attr("rx", rectW * 0.2)
                    .attr("fill", color);

                attachTooltipToElement(rect, tooltip, tooltipWidth, tooltipHtml);

                // month separators
                {
                    let strokeWidth = 1;
                    let strokeColor = "#F77";
                    // right
                    if (dataIdx + 7 < frequencyData.length) {
                        let rightData = frequencyData[dataIdx + 7];
                        let rightYear = rightData.date.getUTCFullYear();
                        let rightMonth = rightData.date.getUTCMonth();
                        if (year === rightYear && month !== rightMonth) {
                            yearG.append("line")
                                .attr("x1", offsetX * w + rectW + gapX/2)
                                .attr("y1", offsetY * d - gapY/2)
                                .attr("x2", offsetX * w + rectW + gapX/2)
                                .attr("y2", offsetY * d + rectH + gapY/2)
                                .attr("stroke-width", strokeWidth)
                                .attr("stroke", strokeColor);
                        }

                    }
                    // bottom
                    if (d !== lastD && dataIdx + 1 < frequencyData.length) {
                        let bottomData = frequencyData[dataIdx + 1];
                        let bottomYear = bottomData.date.getUTCFullYear();
                        let bottomMonth = bottomData.date.getUTCMonth();
                        if (year === bottomYear && month !== bottomMonth) {
                            yearG.append("line")
                                .attr("x1", offsetX * w - gapX/2)
                                .attr("y1", offsetY * d + rectH + gapY/2)
                                .attr("x2", offsetX * w + rectW + gapX/2)
                                .attr("y2", offsetY * d + rectH + gapY/2)
                                .attr("stroke-width", strokeWidth)
                                .attr("stroke", strokeColor);
                        }

                    }
                }

                dataIdx += 1;
            }
        }
    }
}

export function heatMap(
    div,
    matrix,
    xLabels,
    yLabels,
    formatValue,
    style)
{
    let width = matrix.length;
    if (width === 0) {
        return;
    }

    let height = matrix[0].length;

    // TODO make these global styles or something
    let rectW = 10;
    let rectH = 10;
    let gapX = 2;
    let gapY = 2;
    let offsetX = rectW + gapX;
    let offsetY = rectH + gapY;
    let leftMargin = 200;
    let rightMargin = 100;
    let topMargin = 200;

    let maxValue = 0;
    for (let x = 0; x < width; ++x) {
        for (let y = 0; y < height; ++y) {
            let value = matrix[x][y];
            if (value !== undefined) {
                maxValue = Math.max(maxValue, value);
            }
        }
    }

    let svg = d3.select(div)
        .append('svg')
        .attr("width", width * offsetX + leftMargin + rightMargin)
        .attr("height", height * offsetY + topMargin)
        ;

    let colorScale = createColorScale(0, maxValue);

    appendColorScaleLegend(svg, colorScale, rectW, offsetY * 5, maxValue, formatValue)
        .attr("transform", svgTranslate(width * offsetX + leftMargin + offsetX, topMargin));

    let matrixG = svg.append("g")
        .attr("transform", svgTranslate(leftMargin, topMargin))
        ;

    let tooltipWidth = 300;
    let tooltip = createTooltip(div, tooltipWidth);

    for (let y = 0; y < height; ++y) {
        for (let x = 0; x < width; ++x) {
            let value = matrix[x][y];

            let xLabel = xLabels[x];
            let yLabel = yLabels[y];

            let tooltipHtml = `${xLabel}<br/>${yLabel}<br/><b>${formatValue(value)}</b>`;

            let color;
            if (value === undefined) {
                color = "#444";
            } else {
                color = colorScale(value);
            }
            let rect = matrixG.append("rect")
                .attr("x", x * offsetX)
                .attr("y", y * offsetY)
                .attr("width", rectW)
                .attr("height", rectH)
                .attr("rx", rectW * 0.2)
                .attr("fill", color)
                ;

            attachTooltipToElement(rect, tooltip, tooltipWidth, tooltipHtml);
        }
    }

    let createLabel = (group, y, textAnchor, text) => {
        group.append("text")
            .attr("x", 0)
            .attr("y", y)
            .attr("dy", "0.9em")
            .attr("text-anchor", textAnchor)
            .attr("font-size", rectH)
            .attr("fill", theme.palette.text.primary)
            .text(text);
    };

    let yLabelsG = svg.append("g")
        .attr("transform", svgTranslate(leftMargin - gapY, topMargin));
    for (let y = 0; y < height; ++y) {
        createLabel(yLabelsG, y * offsetY, "end", yLabels[y]);
    }

    let xLabelsG = svg.append("g")
        .attr("transform", svgTranslate(leftMargin - gapY, topMargin - gapY) + svgRotate(-90));
    for (let x = 0; x < width; ++x) {
        createLabel(xLabelsG, x * offsetX, "start", xLabels[x]);
    }
}