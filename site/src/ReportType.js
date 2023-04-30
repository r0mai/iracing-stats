let i = 0;
export const kSummary = i++;
export const kIRacingHistory = i++;
export const kCPIHistory = i++;
export const kTrackUsage = i++;
export const kCarUsage = i++;
export const kSessionList = i++;
export const kActivityHistory = i++;
export const kCarTrackMatrix = i++;
export const kReportTypeCount = i++;

const kNames = [
    "summary",
    "iracing-history",
    "cpi-history",
    "track-usage",
    "car-usage",
    "session-list",
    "activity-history",
    "car-track-matrix",
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