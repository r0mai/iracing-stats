import React from 'react';

export const useD3 = (renderChartFn, dependencies) => {
    const ref = React.useRef();

    React.useEffect(() => {
        // Delete any previous svgs
        ref.current.innerHTML = '';
        renderChartFn(ref.current);
        return () => {};
    }, dependencies);
    return ref;
}