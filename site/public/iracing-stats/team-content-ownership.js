async function buildSite(container, teamName) {
    let response = await fetch(`/api/v1/site-team-track-usage?site_team=${teamName}`);
    let json = await response.json();

    let table = document.createElement("table");
    table.style.border = "1px solid #000";
    container.appendChild(table);

    let trackSet = new Set();
    let drivers = [];
    for (let driverName in json["driver_map"]) {
        drivers.push(driverName);
        for (let trackName in json["driver_map"][driverName]["track_map"]) {
            trackSet.add(trackName)
        }
    }

    let tracks = Array.from(trackSet).sort();

    let topRow = table.insertRow();
    topRow.insertCell(); // skip top left corner
    for (let driver of drivers) {
        let cell = topRow.insertCell();
        cell.style.writingMode = "tb-rl";
        cell.style.transform = "rotate(-180deg)";
        cell.style.width = "16px";
        cell.appendChild(document.createTextNode(driver));
    }

    for (let track of tracks) {
        let row = table.insertRow();
        let leftCell = row.insertCell();
        leftCell.style.textAlign = "right";
        leftCell.appendChild(document.createTextNode(track));

        for (let driver of drivers) {
            let trackUsage = json["driver_map"][driver]["track_map"];
            let cell = row.insertCell();
            cell.style.border = "1px solid #000";
            if (trackUsage[track] !== undefined) {
                cell.style.backgroundColor = "green";
            }
        }
    }
}

window.onload = async function() {
    let teamName = new URLSearchParams(location.search).get("team") || "rsmr";
    let container = document.querySelector("#container");
    await buildSite(container, teamName);
}