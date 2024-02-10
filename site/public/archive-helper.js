let textarea = null;
let responseTextarea = null;
let clipboardButton = null;
let archiveRadio = null;
let websiteRadio = null;
let textToCopy = "";

function replaceAll(str, search, replacement) {
    var result = str;
    return result.split(search).join(replacement);
}

async function refreshText() {
    const regexp = /https:\/\/members.iracing.com\/membersite\/member\/EventResult\.do\?&?subsessionid=([0-9]+)/g;
    let matches = [...textarea.value.matchAll(regexp)];
    let subsession_ids = matches.map(e => e[1]);
    if (subsession_ids.length === 0) {
        return;
    }

    let query_type = document.querySelector('input[name="response-type"]:checked').value;

    let unique_subsession_ids = [...new Set(subsession_ids)];

    let url = `/api/v1/session-result?team=rsmr&output_type=${query_type}&subsession_ids=` + unique_subsession_ids.join(";");
    let response = await fetch(url);
    let responseText = await response.text();
    textToCopy = responseText;
    clipboardButton.style.visibility = "visible";
    responseTextarea.value = responseText;
}

window.onload = function() {
    let textToCopy = "";

    textarea = document.querySelector("#input-area");
    responseTextarea = document.querySelector("#response");
    clipboardButton = document.querySelector("#clipboard-button");
    archiveRadio = document.querySelector("#radio-archive");
    archiveWebsite = document.querySelector("#radio-website");

    clipboardButton.addEventListener("click", function() {
        navigator.clipboard.writeText(textToCopy);
    });

    textarea.value = "";
    textarea.addEventListener("input", async function() {
        refreshText();
    }, false);

    archiveRadio.addEventListener("input", async function() {
        refreshText();
    }, false);

    archiveWebsite.addEventListener("input", async function() {
        refreshText();
    }, false);
}
