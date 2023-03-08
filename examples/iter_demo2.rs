use std::collections::HashMap;

fn main() {
    let values = vec![1, 2, 3];
    for v in values.into_iter(){
        println!("{}", v);
    }

    // println!("{:?}", values);
    let values = vec![1, 2, 3];
    let _values_iter = values.iter();
    println!("{:?}", values);


    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);


    //zip to HashMap
    let names = ["Spark", "Free"];
    let ages = [23, 23];
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();
    println!("{:?}", folks);
}