let textarea = null;
let responseTextarea = null;
let clipboardButton = null;
let textToCopy = "";

function replaceAll(str, search, replacement) {
    var result = str;
    return result.split(search).join(replacement);
}

function parseDateStr(str) {
    let nums = str.split(".");
    if (nums.length != 3 && nums.length != 4) {
        throw new Error("Date is invalid: " + str);
    }
    let date = new Date();
    date.setUTCFullYear(Number(nums[0]), Number(nums[1]) - 1, Number(nums[2]));
    return date;
}

function parseFinishedStr(str) {
    if (str == "DNF" ||
        str == "DSQ" ||
        str == "DQS" ||
        str == "N/A")
    {
        return str;
    }
    if (str.length == 0) {
        throw new Error("Finished is invalid (empty): " + str);
    }
    if (str[0] != "P") {
        throw new Error("Finished is invalid (doesn't start with P): " + str);
    }
    return Number(str.slice(1));
}

function parseSessionIDStr(str) {
    const regexp = /https:\/\/members.iracing.com\/membersite\/member\/EventResult\.do\?&?subsessionid=([0-9]+)/g;
    let matches = [...str.matchAll(regexp)];
    let ids = matches.map(e => e[1]);
    return ids[0];
}

function parseString(input) {
    let lines = input.split('\n');
    let parsedResults = [];
    for (let line of lines) {
        try {
            let result = {};
            let columns = line.split("\t");
            result.id = columns[0];
            result.eventName = columns[3];
            result.date = parseDateStr(columns[4]);
            result.driver = columns[7];
            result.finished = parseFinishedStr(columns[10]);
            result.sessionID = parseSessionIDStr(columns[11]);
            result.car = columns[12];
            result.track = columns[13];
            result.trackConfig = columns[14];
            parsedResults.push(result);
        } catch (x) {
            console.error("Error on line \"" + line + "\"");
            throw x;
        }
    }
    return parsedResults;
}

async function refreshText() {
    let results = parseString(textarea.value);
    console.log(results);

    let output = "";

    let prefix = 
        '<div class="table-2">\n' +
        '<table width="100%">\n' +
        '<thead>\n' +
        '<tr>\n' +
        '<th align="left">Esemény</th>\n' +
        '<th align="left">Dátum</th>\n' +
        '<th align="left">Pilóta</th>\n' +
        '<th align="left">Helyzés</th>\n' +
        '<th align="left">Pálya</th>\n' +
        '</tr>\n' +
        '</thead>\n' +
        '<tbody></tbody>\n'
        ;

    let suffix =
        '</tbody>\n' +
        '</table>\n' +
        '</div>\n'
        ;

    output += prefix;

    let currentResult = null;
    let currentDrivers = [];

    for (let result of results) {
        if (currentResult === null || currentResult.id != result.id) {
            if (currentResult !== null) {
                let dateStr = `${currentResult.date.getUTCFullYear()}.${currentResult.date.getUTCMonth() + 1}.${currentResult.date.getUTCDate()}`;
                output += '<tr>\n';
                output += `<td align="left"><a href="https://members.iracing.com/membersite/member/EventResult.do?subsessionid=${currentResult.sessionID}">${currentResult.eventName}</a></td>\n`;
                output += `<td align="left">${dateStr}</td>\n`;
                output += `<td align="left">${currentDrivers.join(', ')}</td>\n`;
                output += `<td align="left">${currentResult.finished}</td>\n`;
                output += `<td align="left">${currentResult.track}</td>\n`;
                output += '</tr>\n';
                output += '\n';
            }

            currentDrivers = [];
            currentResult = result;
        }
        currentDrivers.push(result.driver);
    }

    output += suffix;

    textToCopy = output;
    clipboardButton.style.visibility = "visible";
    responseTextarea.value = output;
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
