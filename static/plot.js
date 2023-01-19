

function verticalBarChart(div, data, xFunc, yFunc, format) {
    let xTickFormat = format?.xTickFormat ?? (e => e);
    let yTickFormat = format?.yTickFormat ?? (e => e);
    let barFill = format?.barFill ?? "red";

    // Bar charts with few lanes are a bit crowded. Something wrong with the math here
    let coreHeight = data.length * 20;
    let margin = {top: 20, right: 80, bottom: 40, left: 250},
        width = 1000 - margin.left - margin.right,
        height = coreHeight - margin.top - margin.bottom;

    let svg = d3.select(div)
        .append("svg")
            .attr("width", width + margin.left + margin.right)
            .attr("height", height + margin.top + margin.bottom)
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
                .text(d => xTickFormat(xFunc(d)))
        });
            
}
