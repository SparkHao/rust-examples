
fn generic() {

    println!("Hello, world!");
    let p = Point{
        x: 4,
        y: 10,
    };
    let p2 = Point{
        x: 4f32,
        y: 10f32,
    };
    println!("p.x = {}", p.x);
    println!("p.x = {}", p.x());
    println!("p.distance_from_origin: {}", p2.distance_from_origin())
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}