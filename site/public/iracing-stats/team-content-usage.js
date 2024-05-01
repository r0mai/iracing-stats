function buildTable(container, json, contentKey) {
    let table = document.createElement("table");
    table.style.border = "1px solid #000";
    container.appendChild(table);

    let contentSet = new Set();
    let drivers = [];
    for (let driverName in json["driver_map"]) {
        drivers.push(driverName);
        for (let contentName in json["driver_map"][driverName][contentKey]) {
            contentSet.add(contentName)
        }
    }

    let contents = Array.from(contentSet).sort();

    let topRow = table.insertRow();
    topRow.insertCell(); // skip top left corner
    for (let driver of drivers) {
        let cell = topRow.insertCell();
        cell.style.writingMode = "tb-rl";
        cell.style.transform = "rotate(-180deg)";
        cell.style.width = "16px";
        cell.appendChild(document.createTextNode(driver));
    }

    for (let content of contents) {
        let row = table.insertRow();
        let leftCell = row.insertCell();
        leftCell.style.textAlign = "right";

        let count = 0;
        for (let driver of drivers) {
            let contentUsage = json["driver_map"][driver][contentKey];
            let cell = row.insertCell();
            cell.style.border = "1px solid #000";
            if (contentUsage[content] !== undefined) {
                cell.style.backgroundColor = "green";
                ++count;
            }
        }
        if (count == drivers.length) {
            leftCell.style.fontWeight = "bold";
        }

        leftCell.appendChild(document.createTextNode(`${content} [${count}/${drivers.length}]`));
    }
}

async function buildSite(teamName) {
    let response = await fetch(`/api/v1/site-team-content-usage?site_team=${teamName}`);
    let json = await response.json();

    let trackContainer = document.querySelector("#track_container");
    let carContainer = document.querySelector("#car_container");

    buildTable(trackContainer, json, "track_map");
    buildTable(carContainer, json, "car_map");
}

window.onload = async function() {
    let teamName = new URLSearchParams(location.search).get("team") || "rsmr";
    await buildSite(teamName);
}