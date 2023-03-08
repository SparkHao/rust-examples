use rand::{thread_rng, Rng, distributions::Alphanumeric};
use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);


    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p|{
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(&Alphanumeric) as char).collect()
    });
    vec.par_sort_unstable();
    println!("vec: {:?}", vec)
}