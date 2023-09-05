function minNumber(nums1: number[], nums2: number[]): number {
    let min1 = +Infinity;
    let min2 = +Infinity;
    const m = {};
    let minc = +Infinity;

    for(let i = 0; i < nums1.length;++i) {
        const item = nums1[i];
        min1 = Math.min(min1, item);

        m[item] = true;
    }
    for(let j = 0; j < nums2.length;++j) {
        const item = nums2[j];
        min2 = Math.min(min2, item);
        if(m[item]) minc = Math.min(minc, item);
    }

    if(minc < +Infinity) return minc;

    if(min1 < min2) return min1 * 10 + min2
    else return min2 * 10 + min1
};