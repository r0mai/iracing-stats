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