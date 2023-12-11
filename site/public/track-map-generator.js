function createSVGElement(name) {
    return document.createElementNS('http://www.w3.org/2000/svg', name);
}

async function downloadTrackAssetData() {
    let response = await fetch("track-asset-data.json");
    return await response.json();
}

async function downloadTrackData(trackID) {
    let response = await fetch("/api/v1/track-data");
    try  {
        let result = await response.json();
        if (!result) {
            return null;
        }
        for (let track of result["tracks"]) {
            if (track["track_id"] == trackID) {
                return track;
            }
        }
    } catch (x) {}
    return null;
}

function round(value, precision) {
    var multiplier = Math.pow(10, precision || 0);
    return Math.round(value * multiplier) / multiplier;
}

async function generateSVG(container, trackID) {
    let trackAssetData = (await downloadTrackAssetData())[trackID];
    let trackData = await downloadTrackData(trackID);
    let layerPrefix = trackAssetData["track_map"];
    let layerSuffixes = trackAssetData["track_map_layers"];

    let layers = [
        // { name: "background" },
        { name: "active" },
        // { name: "inactive" },
        // { name: "pitroad" },
        { name: "start-finish" },
        { name: "turns" },
    ];

    let rootSVG = createSVGElement("svg");
    container.appendChild(rootSVG);

    rootSVG.setAttribute("width", "297mm");
    rootSVG.setAttribute("height", "210mm");
    rootSVG.setAttribute("viewBox", "0 0 297 210");

    // background
    {
        let background = createSVGElement("rect");
        background.setAttribute("width", "100%");
        background.setAttribute("height", "100%");
        background.setAttribute("fill", "#444");

        rootSVG.appendChild(background);
    }

    // track map
    {
        let trackMapG = createSVGElement("g");

        // track SVG is 1920 * 1080
        let trackScale = (297/1920) * (5/9);
        let trackTranslateX = 297 * (2/9);
        let trackTranslateY = 210 * (1/4);
        trackMapG.setAttribute("transform", `translate(${trackTranslateX}, ${trackTranslateY}) scale(${trackScale})`)

        for (let layer of layers) {
            let layerURL = layerPrefix + layerSuffixes[layer.name];
            let svgResponse = await fetch(layerURL);
            let svgText = await svgResponse.text();
            let layerGroup = createSVGElement("g");
            layerGroup.setAttribute("id", layer.name);
            layerGroup.innerHTML = svgText;
            let subSVG = layerGroup.children[0];
            subSVG.setAttribute("width", 1920);
            subSVG.setAttribute("height", 1080);

            trackMapG.appendChild(layerGroup);
        }

        rootSVG.appendChild(trackMapG);
    }

    // lines
    {
        let linesG = createSVGElement("g");
        for (let i = 0; i < 12; ++i) {
            let lineLeft = createSVGElement("line");
            let lineRight = createSVGElement("line");
            let y = 50 + i * 10;
            lineLeft.setAttribute("x1", 10);
            lineLeft.setAttribute("y1", y);
            lineLeft.setAttribute("x2", 70);
            lineLeft.setAttribute("y2", y);
            lineLeft.setAttribute("stroke", "black");
            lineLeft.setAttribute("stroke-width", 0.5);

            lineRight.setAttribute("x1", 227);
            lineRight.setAttribute("y1", y);
            lineRight.setAttribute("x2", 287);
            lineRight.setAttribute("y2", y);
            lineRight.setAttribute("stroke", "black");
            lineRight.setAttribute("stroke-width", 0.5);
            linesG.appendChild(lineLeft);
            linesG.appendChild(lineRight);
        }
        rootSVG.appendChild(linesG);
    }

    // track data
    {
        function createDataText() {
            let text = createSVGElement("text");
            text.setAttribute("font-size", "6");
            return text;
        }

        let trackDataG = createSVGElement("g");
        {
            let cornersPerLap = trackData?.corners_per_lap || 99;
            let cornersPerLapText = createDataText();
            cornersPerLapText.setAttribute("transform", "translate(227 20)")
            cornersPerLapText.innerHTML = `- Corners per lap: ${cornersPerLap}`;
            trackDataG.appendChild(cornersPerLapText);
        }

        {
            let trackLength = trackData?.track_config_length || 3.14;
            let trackLengthText = createDataText();
            trackLengthText.setAttribute("transform", "translate(227 26)")
            trackLengthText.innerHTML = `- Track length: ${round(trackLength, 2)}km`;
            trackDataG.appendChild(trackLengthText);
        }
        rootSVG.appendChild(trackDataG);
    }
}

window.onload = async function() {
    let trackID = new URLSearchParams(location.search).get("track_id") || "1";
    let container = document.querySelector("#svg-container");
    await generateSVG(container, trackID);
}