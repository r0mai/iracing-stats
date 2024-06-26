let i = 0;
export const kSummary = i++;
export const kIRatingHistory = i++;
export const kCPIHistory = i++;
export const kTrackUsage = i++;
export const kCarUsage = i++;
export const kSessionList = i++;
export const kActivityHistory = i++;
export const kCarTrackMatrix = i++;
export const kReportTypeCount = i++;

const kNames = [
    "summary",
    "irating-history",
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
        return kIRatingHistory; // revert this to kSummary
    } else {
        return idx;
    }
}

export function findName(reportIdx) {
    if (reportIdx < 0 || reportIdx >= kReportTypeCount) {
        return kNames[1];// revert this to 0
    } else {
        return kNames[reportIdx]; 
    }
}