import * as d3 from 'd3';
import { svgPx } from './Utility.js';

export function createTooltip(parentDiv, width) {
    return d3.select(parentDiv)
        .append("div")
        .style("visibility", "hidden")
        .style("position", "absolute")
        .style("width", svgPx(width))
        .style("max-width", svgPx(width))
        .style("background-color", "gray")
        .style("text-align", "center")
        ;
}

export function attachTooltipToElement(element, tooltip, width, html) {
    let mouseover = function(event) {
        tooltip
            .html(html)
            .style("left", svgPx(event.pageX - width * 0.5))
            .style("top", svgPx(event.pageY + 10))
            .style("visibility", "visible");
    };
    let mouseleave = function() {
        tooltip.style("visibility", "hidden");
    };

    element
        .on("mousemove", mouseover)
        .on("mouseout", mouseleave);
}