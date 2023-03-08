use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};



struct Person{
    age: u32,
}

fn main() {
    let v: Vec<Person> = vec![
        Person {age: 32},
        Person {age: 18},
        Person {age: 20},
        Person {age: 25},
        Person {age: 17},
        Person {age: 14},
        Person {age: 30},
        Person {age: 42},
        Person {age: 21},
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    println!("num over 30: {}", num_over_30);

    let sum_over_30 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).reduce(|| 0, |x, y| x + y);
    println!("sum over 30: {}", sum_over_30);

    let alt_sum_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    println!("alt sum 30: {}", alt_sum_30);

}