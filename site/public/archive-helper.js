window.onload = function() {
    let textarea = document.querySelector("#input-area");
    let responseDiv = document.querySelector("#response");

    textarea.value = "";
    textarea.addEventListener("input", async function() {
        const regexp = /https:\/\/members.iracing.com\/membersite\/member\/EventResult\.do\?&subsessionid=([0-9]+)/g;
        let matches = [...textarea.value.matchAll(regexp)];
        let subsession_ids = matches.map(e => e[1]);
        if (subsession_ids.length === 0) {
            return;
        }

        let url = "/api/v1/session-result?team=rsmr&subsession_ids=" + subsession_ids.join(";");
        let response = await fetch(url);
        let responseText = await response.text();
        responseDiv.innerHTML = responseText;
    }, false);
}
