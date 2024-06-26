import { useD3 } from './hooks/useD3.js';
import { colorsFromThresholds, heatMap, plotColorInterpolator } from './Plot.js';
import { fromMinutes, fromHours, formatTime, getTimeInSession } from './Utility.js';

function createIndexMapping(data, keyFunc, labelFunc) {
    let indexMap = new Map();
    let labelArray = [];
    let nextIdx = 0;

    for (let session of data) {
        let key = keyFunc(session);
        if (indexMap.has(key)) {
            continue;
        }

        indexMap.set(key, nextIdx);
        labelArray[nextIdx] = labelFunc(session);
        nextIdx += 1;
    }
    return {
        indexMap: indexMap,
        labelArray: labelArray
    };
}

function createUsageMatrix(sessions, carMap, trackMap) {
    // key to idx; key to label
    let {indexMap: xIdxMap, labelArray: xLabels} = createIndexMapping(
        sessions,
        s => s["car_id"],
        s => carMap[s["car_id"]]["car_name"]);
    let {indexMap: yIdxMap, labelArray: yLabels} = createIndexMapping(
        sessions,
        s => s["package_id"],
        s => trackMap[s["track_id"]]["track_name"]);

    let width = xIdxMap.size;
    let height = yIdxMap.size;

    let matrix = Array.from(Array(width), () => new Array(yIdxMap.size));

    let xSums = Array(width).fill(0);
    let ySums = Array(height).fill(0);

    for (let session of sessions) {
        let xKey = session["car_id"];
        let yKey = session["package_id"];
        let xIdx = xIdxMap.get(xKey);
        let yIdx = yIdxMap.get(yKey);
        let value = getTimeInSession(session);

        xSums[xIdx] += value;
        ySums[yIdx] += value;

        let sum = matrix[xIdx][yIdx];

        if (sum === undefined) {
            sum = 0;
        }

        sum += value;
        matrix[xIdx][yIdx] = sum;
    }

    let sortedXIndices = Array.from(Array(width).keys());
    let sortedYIndices = Array.from(Array(height).keys());

    sortedXIndices.sort((lhs, rhs) => xSums[rhs] - xSums[lhs]);
    sortedYIndices.sort((lhs, rhs) => ySums[rhs] - ySums[lhs]);

    let sortedMatrix = Array.from(Array(width), () => new Array(height));
    let sortedXLabels = Array(width);
    let sortedYLabels = Array(height);

    for (let x = 0; x < width; ++x) {
        for (let y = 0; y < height; ++y) {
            sortedMatrix[x][y] = matrix[sortedXIndices[x]][sortedYIndices[y]];
        }
    }

    for (let x = 0; x < width; ++x) {
        sortedXLabels[x] = xLabels[sortedXIndices[x]];
    }
    for (let y = 0; y < height; ++y) {
        sortedYLabels[y] = yLabels[sortedYIndices[y]];
    }

    return {
        matrix: sortedMatrix,
        xLabels: sortedXLabels,
        yLabels: sortedYLabels
    };
}

function plotCarTrackMatrix(div, matrix, xLabels, yLabels) {
    if (matrix.length === 0 || matrix[0].length === 0) {
        div.innerHTML = "No data";
    } else {
        let thresholds = [
            fromMinutes(1),
            fromHours(1),
            fromHours(10),
            fromHours(25),
            fromHours(50)
        ];
        heatMap(
            div,
            matrix,
            xLabels,
            yLabels,
            e => e === undefined ? "No Activity" : formatTime(e),
            {
                thresholds: thresholds,
                // https://colorbrewer2.org/#type=sequential&scheme=OrRd&n=6
                thresholdColors: ['#fef0d9','#fdd49e','#fdbb84','#fc8d59','#e34a33','#b30000']
            }
        );
    }
}

function CarTrackMatrix({driverSessions, carMap, trackMap}) {
    let {matrix, xLabels, yLabels} = createUsageMatrix(driverSessions, carMap, trackMap);
    const ref = useD3(
        (root) => {
            plotCarTrackMatrix(root, matrix, xLabels, yLabels);
        },
        [driverSessions]
    );
    
    return (
        <div ref={ref}/>
    );
}

export default CarTrackMatrix;