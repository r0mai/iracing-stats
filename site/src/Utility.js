export function toHours(interval) {
    return interval / 10000 / 60 / 60;
}

// https://stackoverflow.com/a/7343013
export function round(value, precision) {
    var multiplier = Math.pow(10, precision || 0);
    return Math.round(value * multiplier) / multiplier;
}