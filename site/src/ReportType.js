let i = 0;
export const kSummary = i++;
export const kIRacingHistory = i++;
export const kCPIHistory = i++;
export const kTrackUsage = i++;
export const kCarUsage = i++;
export const kSessionList = i++;
export const kHistoryChart = i++;
export const kReportTypeCount = i++;

export const kNames = [
    "summary",
    "iracing-history",
    "track-usage",
    "cpi-history",
    "track-usage",
    "car-usage",
    "session-list",
    "history-chart"
];

export function findIndex(reportName) {
    let idx = kNames.indexOf(reportName);
    if (idx === -1) {
        return kSummary;
    } else {
        return idx;
    }
}

export function findName(reportIdx) {
    if (reportIdx < 0 || reportIdx >= kReportTypeCount) {
        return kNames[0];
    } else {
        return kNames[reportIdx];
    }
}