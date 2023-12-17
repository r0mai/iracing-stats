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

function addStyle(rootSVG) {
    let styleElement = createSVGElement("style");

    styleElement.innerHTML = `
         @font-face {
            font-family: "Neuropolitical";
            src: url("neuropolitical.otf");
        }
    `;

    rootSVG.appendChild(styleElement);
}

function createLineElement(x1, y1, x2, y2) {
    let line = createSVGElement("line");
    line.setAttribute("x1", x1);
    line.setAttribute("y1", y1);
    line.setAttribute("x2", x2);
    line.setAttribute("y2", y2);
    line.setAttribute("stroke", "black");
    line.setAttribute("stroke-width", 0.5);
    return line;
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

    addStyle(rootSVG);

    rootSVG.setAttribute("width", "297mm");
    rootSVG.setAttribute("height", "210mm");
    rootSVG.setAttribute("viewBox", "0 0 297 210");

    // background
    {
        let background = createSVGElement("rect");
        background.setAttribute("width", "100%");
        background.setAttribute("height", "100%");
        background.setAttribute("fill", "#FFF");

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

            const fontFamilyRegex = /font-family:'.*'/;
            const fontSizeRegex = /font-size:[^;]*;/;
            const fillRegex = /fill:[^;]*;/;

            let styleTag = subSVG.querySelector("style");
            let css = styleTag.innerHTML;

            css = css.replace(fillRegex, "fill:#000");

            if (layer.name == "turns") {
                css = css.replace(fontFamilyRegex, "font-family:'Neuropolitical'");
                css = css.replace(fontSizeRegex, "font-size:24px");
                styleTag.innerHTML = css;
            }

            trackMapG.appendChild(layerGroup);
        }

        rootSVG.appendChild(trackMapG);
    }

    // lines
    {
        let linesG = createSVGElement("g");
        for (let i = 0; i < 12; ++i) {
            let y = 50 + i * 10;
            let lineLeft = createLineElement(10, y, 70, y);
            let lineRight = createLineElement(227, y, 287, y);
            linesG.appendChild(lineLeft);
            linesG.appendChild(lineRight);
        }
        rootSVG.appendChild(linesG);
    }

    // track data
    {
        function createDataText(fontSize) {
            fontSize = fontSize || 4;
            let text = createSVGElement("text");
            text.setAttribute("font-size", fontSize);
            text.setAttribute("font-family", "Neuropolitical");
            return text;
        }

        let trackDataG = createSVGElement("g");
        let cursorY = 20;
        let cursorAdvanceY = 6;
        {
            let text = createDataText();
            text.setAttribute("transform", `translate(227 ${cursorY})`)
            text.innerHTML = `- My car:`;
            trackDataG.appendChild(text);
            let line = createLineElement(250, cursorY + 0.75, 287, cursorY + 0.75);
            trackDataG.appendChild(line);
            
            cursorY += cursorAdvanceY;
        }

        {
            let cornersPerLap = trackData?.corners_per_lap || 99;
            let text = createDataText();
            text.setAttribute("transform", `translate(227 ${cursorY})`)
            text.innerHTML = `- Corners per lap: ${cornersPerLap}`;
            trackDataG.appendChild(text);
            cursorY += cursorAdvanceY;
        }

        {
            let trackLength = trackData?.track_config_length || 3.14;
            let text = createDataText();
            text.setAttribute("transform", `translate(227 ${cursorY})`)
            text.innerHTML = `- Track length: ${round(trackLength, 2)}km`;
            trackDataG.appendChild(text);
            cursorY += cursorAdvanceY;
        }
        rootSVG.appendChild(trackDataG);
    }

    // my notes text
    {
        let text = createDataText();
        text.setAttribute("transform", `translate(27 32)`);
        text.innerHTML = `My notes`;
        rootSVG.appendChild(text);
    }

    // rsm logo
    {
        let image = createSVGElement("image");
        image.setAttribute("x", 3);
        image.setAttribute("y", 3);
        image.setAttribute("width", 30.4);
        image.setAttribute("height", 10);
        image.setAttribute("href", "rsm.png");
        rootSVG.appendChild(image);
    }

    // tracknotes text
    {
        let text = createDataText(3);
        text.setAttribute("transform", `translate(6 18)`);
        text.innerHTML = `TRACKNOTES`;
        rootSVG.appendChild(text);
    }

    // url
    {
        let text = createDataText(2);
        text.setAttribute("transform", `translate(4 208)`);
        text.innerHTML = `WWW.RSMRACING.HU`;
        rootSVG.appendChild(text);
    }

    // track name
    {
        let track_name = trackData?.track_name || "Sebring International Raceway";
        let config_name = trackData?.config_name || "International";

        {
            let text = createDataText(8);
            text.setAttribute("x", "50%");
            text.setAttribute("y", "10");
            text.setAttribute("text-anchor", "middle");
            text.innerHTML = track_name;
            rootSVG.appendChild(text);
        }

        {
            let text = createDataText(6);
            text.setAttribute("x", "50%");
            text.setAttribute("y", "18");
            text.setAttribute("text-anchor", "middle");
            text.innerHTML = `${config_name} <${trackID}>`;
            rootSVG.appendChild(text);
        }
    }
}

window.onload = async function() {
    let trackID = new URLSearchParams(location.search).get("track_id") || "1";
    let container = document.querySelector("#svg-container");
    await generateSVG(container, trackID);
}