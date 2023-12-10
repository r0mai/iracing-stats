function createSVGElement(name) {
    return document.createElementNS('http://www.w3.org/2000/svg', name);
}

async function downloadTrackAssetData() {
    let assetData = await fetch("track-asset-data.json");
    return await assetData.json();
}

async function generateSVG(container, assetData, trackID) {
    let trackData = assetData[trackID];
    let prefix = trackData["track_map"];
    let layerSuffixes = trackData["track_map_layers"];

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

    rootSVG.setAttribute("width", 1280);
    rootSVG.setAttribute("height", 720);
    rootSVG.setAttribute("viewbox", "0, 0, 1920, 1080");

    for (let layer of layers) {
        let layerURL = prefix + layerSuffixes[layer.name];
        let svgResponse = await fetch(layerURL);
        let svgText = await svgResponse.text();
        let layerGroup = createSVGElement("g");
        layerGroup.setAttribute("id", layer.name);
        layerGroup.innerHTML = svgText;

        rootSVG.appendChild(layerGroup);
    }
}

window.onload = async function() {
    let container = document.querySelector("#svg-container");
    let assetData = await downloadTrackAssetData();
    await generateSVG(container, assetData, "1");
}