use std::fmt;

fn main(){
    let w = Wrapper(vec![String::from("Hello"), String::from(" world")]);
    println!("w: {}", w);
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}