export function twoSum(nums: number[], target: number): number[] {
    const dict = new Map<number, number>();
    for (let i = 0; i < nums.length; ++i) {
        const remainIdx = dict.get(nums[i]);
        if (typeof remainIdx === 'number')
            return [remainIdx, i];
        else
            dict.set(target - nums[i], i);
    }

    throw new Error();
};