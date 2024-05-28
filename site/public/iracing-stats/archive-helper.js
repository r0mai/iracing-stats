let textarea = null;
let responseTextarea = null;
let clipboardButton = null;
let textToCopy = "";

function replaceAll(str, search, replacement) {
    var result = str;
    return result.split(search).join(replacement);
}

async function refreshText() {
    const regexp1 = /https:\/\/members.iracing.com\/membersite\/member\/EventResult\.do\?&?subsessionid=([0-9]+)/g;
    const regexp2 = /https:\/\/members-ng.iracing.com\/racing\/home\/dashboard\?subsessionid=([0-9]+)/g;
    let matches = [...textarea.value.matchAll(regexp1), ...textarea.value.matchAll(regexp2)];
    let subsession_ids = matches.map(e => e[1]);
    if (subsession_ids.length === 0) {
        return;
    }

    let unique_subsession_ids = [...new Set(subsession_ids)];

    console.log(subsession_ids);

    let url = `/api/v1/session-result?team=rsmr&subsession_ids=` + unique_subsession_ids.join(";");
    let response = await fetch(url);
    let responseText = await response.text();
    textToCopy = responseText;
    clipboardButton.style.visibility = "visible";
    responseTextarea.value = responseText;
}

window.onload = function() {
    textarea = document.querySelector("#input-area");
    responseTextarea = document.querySelector("#response");
    clipboardButton = document.querySelector("#clipboard-button");

    clipboardButton.addEventListener("click", function() {
        navigator.clipboard.writeText(textToCopy);
    });

    textarea.value = "";
    textarea.addEventListener("input", async function() {
        refreshText();
    }, false);
}
