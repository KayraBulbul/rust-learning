fn main() {
    let rec = Rectangle {
        width: 20,
        height: 30,
    };

    println!("The area of rec is {}", rec.get_area());
    dbg!(rec);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}
