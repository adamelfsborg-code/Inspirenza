const WIDTH: u32 = 7;
const HEIGHT: u32 = 4;

fn main() {
    
    // Issue: "Calculute the area of an rectangle"

    // Solution 1. Standard.
    let a = area(WIDTH, HEIGHT);
    println!("The Area of 'width={}' and 'height={}' is equal to {}", WIDTH, HEIGHT, a);

    // Solution 2. Tuple.
    let a = area_tuple((WIDTH, HEIGHT));
    println!("The Area of 'width={}' and 'height={}' is equal to {}", WIDTH, HEIGHT, a);

    // Solution 3. Struct.
    let rectangle = Rectangle::new(WIDTH, HEIGHT);
    let a = rectangle.area();
    println!("The Area of 'width={}' and 'height={}' is equal to {}", WIDTH, HEIGHT, a);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}