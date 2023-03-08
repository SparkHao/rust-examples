
#[repr(i32)]
enum MyEnum {
    A = 1, B, C
}

fn main() {
    let x = MyEnum::B;
    let y = x as i32;
    let z: MyEnum = unsafe {
        std::mem::transmute(y)
    };

    match z {
        MyEnum::A => println!("Found A"),
        MyEnum::B => println!("Found B"),
        MyEnum::C => println!("Found C"),
    }
}