/*
 * @lc app=leetcode.cn id=146 lang=typescript
 *
 * [146] LRU 缓存
 */

// @lc code=start
class Dln {
    prev: Dln | null;
    next: Dln | null;
    constructor(public key: number, public value: number) {
        this.prev = null;
        this.next = null;
    }
}
class LRUCache {
    size: number;
    cap: number;
    cache: Record<number, Dln>;
    head: Dln;
    tail: Dln;
    constructor(cap: number) {
        this.size = 0;
        this.cap = cap;
        this.cache = {};
        this.head = new Dln(0, 0);
        this.tail = new Dln(0, 0);
        this.head.next = this.tail;
        this.tail.prev = this.head;
    }
    get(key: number): number {
        const node = this.cache[key];
        if(!node) return -1;
        this.moveToHead(node);
        return node.value;
    }
    put(key: number, value: number): void {
        if (!this.cache[key]) {
            const node = new Dln(key, value);
            this.cache[key] = node;
            this.addTohead(node);
            this.size++;
            if (this.size > this.cap) {
                const removed = this.removeTail();
                delete this.cache[removed.key];
                this.size--;
            }
        } else {
            const node = this.cache[key];
            node.value = value;
            this.moveToHead(node);
        }
    }
    addTohead(node: Dln): void {
        node.prev = this.head;
        node.next = this.head.next;
        this.head.next!.prev = node;
        this.head.next = node;
    }
    removeNode(node: Dln): void {
        node.prev!.next = node.next;
        node.next!.prev = node.prev;
    }
    removeTail(): Dln {
        const node = this.tail.prev!;
        this.removeNode(node);
        return node;
    }
    moveToHead(node: Dln): void {
        this.removeNode(node);
        this.addTohead(node);
    }

}

/**
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */
// @lc code=end

