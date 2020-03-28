#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::Rng;
    use test::Bencher;

    /* Size 60 hashset vs. vector, looking up median. */

    #[bench]
    fn bench_hash_int(b: &mut Bencher) {
        let hashtable = (0..60).collect::<HashSet<i32>>();
        b.iter(|| {
            hashtable.contains(&30)
        });
    }

    #[bench]
    fn bench_linear_int(b: &mut Bencher) {
        let vec = (0..60).collect::<Vec<i32>>();
        b.iter(|| {
            vec.contains(&30)
        });
    }

    /* What if we look up a random item, instead of the median? */

    #[bench]
    fn bench_rng_hash_int(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let hashtable = (0..60).collect::<HashSet<i32>>();
        b.iter(|| {
            hashtable.contains(&rng.gen_range(0, 30))
        });
    }

    #[bench]
    fn bench_rng_linear_int(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let vec = (0..60).collect::<Vec<i32>>();
        b.iter(|| {
            vec.contains(&rng.gen_range(0, 30))
        });
    }

    /* What about a non-existant item? */

    #[bench]
    fn bench_404_hash_int(b: &mut Bencher) {
        let hashtable = (0..60).collect::<HashSet<i32>>();
        b.iter(|| {
            hashtable.contains(&404)
        });
    }

    #[bench]
    fn bench_404_linear_int(b: &mut Bencher) {
        let vec = (0..60).collect::<Vec<i32>>();
        b.iter(|| {
            vec.contains(&404)
        });
    }
}
