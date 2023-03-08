struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
       if self.count < 5 {
        self.count += 1;
        Some(self.count)
       } else {
           None
       }
    }
}

fn main() {
    let mut counter = Counter::new();

    let v = counter.next();
    println!("{:?}", v);
    let v = counter.next();
    println!("{:?}", v);
    let v = counter.next();
    println!("{:?}", v);
    let v = counter.next();
    println!("{:?}", v);
    let v = counter.next();
    println!("{:?}", v);

    let v = counter.next();
    println!("{:?}", v);

    let v: Vec<_> = Counter::new().zip(Counter::new().skip(1)).collect();
    println!("zip: {:?}", v);

    let v: u32 = Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b).filter(|x| x % 3 == 0).sum();
    println!("sum: {:?}", v);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter().enumerate().filter(|(&(idx, _))| idx % 2 == 0).map(|(idx, val)| val).fold(0u64, |sum , acm| sum + acm);
    // eq
    // let val: u64 = v.iter().enumerate().filter(|(&(idx, _))| idx % 2 == 0).map(|(idx, val)| val).sum::<u64>();
    println!("val: {}", val); 
}