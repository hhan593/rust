// fn main() {
//     let width = 10;
//     let hgight = 20;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, hgight)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn add(&self) -> u32 {
        let count = self.width + self.height;
        println!("add: {}", count);
        self.width + self.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("The add of the rectangle is {}.", rect1.add());
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The rectangle is {:?}.", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
