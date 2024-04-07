import { Bench } from 'tinybench';
import { twoSum } from './n1';

(async function benchmark() {
    const bench = new Bench({ time: 100 });
    bench.add('twoSum', () => {
        twoSum([2, 7, 11, 15], 9);
    });

    await bench.warmup();
    await bench.run()

    console.table(bench.table());
})();


