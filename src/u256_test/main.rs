fn main(){
    println!("start");
    let a = U256{low: 100, high: 0};

    let v = (a.high << 128).to_string() + &a.low.to_string();
    println!("v: {:?}", v);
}

#[derive(Debug)]
struct U256 {
    low: u128,
    high: u128,
}

