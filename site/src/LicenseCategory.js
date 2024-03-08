export const kOval = 1;
export const kRoad = 2;
export const kDirtOval = 3;
export const kDirtRoad = 4;
export const kSportsCar = 5;
export const kFormulaCar = 6;

const kNames = [
    "oval",
    "road",
    "dirt-oval",
    "dirt-road",
    "sports-car",
    "formula-car",
];

const kNiceNames = [
    "Oval",
    "Road",
    "Dirt Oval",
    "Dirt Road",
    "Sports Car",
    "Formula Car",
];

export function findIndex(licenseName) {
    // 1 based index
    let idx = kNames.indexOf(licenseName);
    if (idx === -1) {
        return kRoad;
    } else {
        return idx + 1;
    }
}

export function findName(licenseIdx) {
    // 1 based index
    if (licenseIdx <= 0 || licenseIdx > kNames.length) {
        return kNames[kRoad - 1];
    } else {
        return kNames[licenseIdx - 1];
    }
}

export function toNiceName(licenseIdx) {
    // 1 based index
    if (licenseIdx < 0 || licenseIdx > 4) {
        return kNiceNames[kRoad - 1];
    } else {
        return kNiceNames[licenseIdx - 1];
    }
}