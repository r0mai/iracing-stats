function generateRaceChart(div, data, xFunc, yFunc) {
    let width = 600;
    let height = 600;

    let rawSVG = document.createElement('svg');

    // svg.node();

    let svg = d3.select(div)
        .append(() => rawSVG)
        // .attr('width', width)
        // .attr('height', height)
        .attr("viewBox", `0 0 ${width} ${height}`)
        .attr("preserveAspectRatio", "xMinYMin meet")
        ;
    svg.append('rect')
        .attr('width', '599')
        .attr('height', '599')
        .attr('fill', 'red')
        ;

    return rawSVG;
}

function testRaceChart(div) {
    return generateRaceChart(div);
}