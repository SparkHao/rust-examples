#![feature(test)]

extern crate rand;
extern crate test;

fn sum_for(x: &[f64]) -> f64 {
    let mut result = 0f64;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[cfg(test)]
mod bench {
    use test::Bencher;
    use rand::{Rng, thread_rng};
    use super::*;

    const LEN: usize = 1024 * 1024;

    fn rand_array(cnt: u32) -> Vec<f64> {
        let mut rng = thread_rng();
        (0..cnt).map(|_| rng.gen::<f64>()).collect()
    }

    #[bench]
    fn bench_for(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(||{
            sum_for(&samples)
        })
    }

    #[bench]
    fn bench_iter(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_iter(&samples)
        })
    }
}

//test bench::bench_for  ... bench:   1,236,688 ns/iter (+/- 27,503)
//test bench::bench_iter ... bench:   1,178,692 ns/iter (+/- 28,816)