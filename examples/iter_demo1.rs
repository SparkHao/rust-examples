fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        dbg!(val);
    }

    for (id, va) in v1.iter().enumerate(){
        dbg!(id, va);
    }
}

