/*
 * @lc app=leetcode.cn id=1046 lang=typescript
 *
 * [1046] 最后一块石头的重量
 */

// @lc code=start
function lastStoneWeight(stones: number[]): number {
    const maxHeap = new MaxHeap(stones);
    while (maxHeap.numbers.length > 1) {
        let a = maxHeap.popMax();
        let b = maxHeap.popMax();
        let left = Math.abs(a - b);
        if (left > 0) maxHeap.insert(left);
    }
    return maxHeap.max || 0;
};
class MaxHeap {
    constructor(public numbers: number[]) {
        for (let i = this.numbers.length >> 1; i >= 0; --i)
            this.maxify(i);
    }

    public insert(num: number) {
        this.numbers.push(num);
        let index = this.numbers.length - 1;
        while (index > 0) {
            let parentIndex = this.parentIndex(index);
            if (this.numbers[index] > this.numbers[parentIndex]) {
                this.swap(index, parentIndex);
                index = parentIndex;
            } else break;
        }
    }

    public get max(): number {
        return this.numbers[0];
    }

    public popMax(): number {
        if (this.numbers.length <= 0) return -1;
        let max = this.numbers[0];
        this.numbers[0] = this.numbers[this.numbers.length - 1];
        this.numbers.pop();
        this.maxify(0);
        return max;
    }

    public maxify(index: number) {
        if (index >= this.numbers.length) return;

        const leftIndex = this.leftIndex(index);
        const rightIndex = this.rightIndex(index);
        const rootVal = this.numbers[index];
        let smallest = index;

        if (leftIndex < this.numbers.length && rootVal < this.numbers[leftIndex])
            smallest = leftIndex;
        if (rightIndex < this.numbers.length && this.numbers[smallest] < this.numbers[rightIndex])
            smallest = rightIndex;

        if (smallest !== index) {
            this.swap(index, smallest);
            this.maxify(smallest);
        }
    }
    public parentIndex(index: number): number {
        return index >> 2;
    }
    public leftIndex(index: number): number {
        return index * 2 + 1;
    }
    public rightIndex(index: number): number {
        return index * 2 + 2;
    }
    public swap(from: number, to: number) {
        [this.numbers[from], this.numbers[to]] = [this.numbers[to], this.numbers[from]];
    }
}
// @lc code=end

