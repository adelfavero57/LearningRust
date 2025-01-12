fn main() {
    let rect1 = Rectangle {
        width:30,
        height: 50,
    };

    println!("The area of the rectangle is {}", rect1.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {i
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, rec: &Rectangle) -> bool {
        if self.width > rec.width && self.height > rec.height {
            true
        }
        else {
            false
        }
    // this is an associated function. It doesn't get a "self" passed and is generally used to
    // instantiate an object
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
}


