// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color{
    Brown,
    _Black,
    _Gray,
}

impl Color {
    fn print(&self){
        match self{
            Color::Brown => println!("brown"),
            Color::_Black => println!("black"),
            Color::_Gray => println!("gray")
        }
    }
}
struct Dimensions{
    height: f32,
    width: f32,
    depth: f32
}

impl Dimensions{
    fn print(&self){
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth)
    }
}
struct ShipingBox{
    weight: f32,
    color: Color,
    dimensions: Dimensions,
}

impl ShipingBox{
    fn new(weight: f32, color: Color, dimensions: Dimensions) -> Self{
        Self {weight,
            color,
            dimensions,
        }
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight)
    }
}
fn main() {
    let small_box_dimensions = Dimensions{
        width: 1.0,
        height: 1.0,
        depth: 2.0,
    };

    let small_box = ShipingBox::new(5.0, Color::Brown, small_box_dimensions);
    small_box.print();
}
