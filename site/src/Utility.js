export function toHours(interval) {
    return interval / 10000 / 60 / 60;
}

// https://stackoverflow.com/a/7343013
export function round(value, precision) {
    var multiplier = Math.pow(10, precision || 0);
    return Math.round(value * multiplier) / multiplier;
}

export function svgTranslate(w, h) {
    return "translate(" + w + "," + h + ")";
}

export function svgRotate(angle) {
    return "rotate(" + angle + ")";
}

export function svgPx(v) {
    return `${v}px`;
}

export function isDriverCustomerID(driver) {
    return driver.startsWith("$");
}

export function extractCustomerID(driver) {
    return driver.slice(1);
}

export function driverToQueryParam(driver) {
    if (isDriverCustomerID(driver)) {
        return "cust_id=" + extractCustomerID(driver);
    } else {
        return "driver_name=" + driver;
    }
}

export function formatTime(time) {
    let hours = toHours(time);
    if (hours < 1) {
        return round(hours * 60, 0) + " min";
    } else {
        return round(hours, 1) + "h";
    }
}

export function mapifyTrackData(tracks) {
    let trackMap = {};
    tracks.forEach(track => {
        trackMap[track["track_id"]] = track;
    });
    return trackMap;
}

export function mapifyCarData(cars) {
    let carMap = {};
    cars.forEach(car => {
        carMap[car["car_id"]] = car;
    });
    return carMap;
}

export function isRookie(session) {
    return session["new_irating"] === -1;
}

export function isRace(session) {
    return session["event_type"] === 5;
}

export function isMainEvent(session) {
    return session["simsession_number"] === 0;
}

export function isCategory(session, category) {
    return session["license_category"] === category ;
}

export function isTrackCategory(session, trackMap, category) {
    return trackMap[session["track_id"]]["category"] == category;
}

export function getHighestIRating(driverSessions, category) {
    let filteredIRating = driverSessions.filter(session => isCategory(session, category)).map(session => session["new_irating"]);
    return Math.max.apply(null, filteredIRating);
}

export function getTimeInSession(session) {
    return session["average_lap"] * session["laps_complete"];
}

export function calcSessionCPI(driverSession, trackMap) {
    return driverSession["laps_complete"] * trackMap[driverSession["track_id"]]["corners_per_lap"] / driverSession["incidents"];
}