window.onload = async function() {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);

    let team_id = urlParams.get("team_id");
    let season_id = urlParams.get("season_id");
    let car_class_id = urlParams.get("car_class_id");

    let body = document.body;
    let tbl = document.createElement("table");

    tbl.style.width = '100px';
    tbl.style.border = '1px solid black';

    let url = `/api/v1/season-team-standings?team_id=${team_id}&season_id=${season_id}&car_class_id=${car_class_id}`;
    let response = await (await fetch(url)).json();
    
    for (let weekIdx = 0; weekIdx < response.length; ++weekIdx) {
        let row = tbl.insertRow();
        let cell1 = row.insertCell();
        let cell2 = row.insertCell();
        let points = response[weekIdx];
        cell1.appendChild(document.createTextNode(`W${weekIdx + 1}`));
        cell2.appendChild(document.createTextNode(`${points}`));
    }

    body.appendChild(tbl);
}
