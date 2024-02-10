function replaceAll(str, search, replacement) {
    var result = str;
    return result.split(search).join(replacement);
}

window.onload = function() {
    let textToCopy = "";

    let textarea = document.querySelector("#input-area");
    let responseDiv = document.querySelector("#response");
    let clipboardButton = document.querySelector("#clipboard-button");

    clipboardButton.addEventListener("click", function() {
        navigator.clipboard.writeText(textToCopy);
    });

    textarea.value = "";
    textarea.addEventListener("input", async function() {
        const regexp = /https:\/\/members.iracing.com\/membersite\/member\/EventResult\.do\?&?subsessionid=([0-9]+)/g;
        let matches = [...textarea.value.matchAll(regexp)];
        let subsession_ids = matches.map(e => e[1]);
        if (subsession_ids.length === 0) {
            return;
        }

        let unique_subsession_ids = [...new Set(subsession_ids)];

        let url = "/api/v1/session-result?team=rsmr&subsession_ids=" + unique_subsession_ids.join(";");
        let response = await fetch(url);
        let responseText = await response.text();
        textToCopy = responseText;
        clipboardButton.style.visibility = "visible";
        responseDiv.innerHTML = replaceAll(responseText, "\n", "<br/>");
    }, false);
}
