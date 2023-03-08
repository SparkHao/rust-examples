fn main() {
    let mut sum = 0;
    dbg!(fib(&mut sum, 11));
}

fn fib(sum: &mut i32, idx: i32) -> i32{
    if idx == 0{
        0
    }else if idx == 1 {
        1
    }else {
        fib(sum, idx - 1) + fib(sum, idx - 2)
    }

}