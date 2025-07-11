trait Drawable {
    fn draw(&self);
}

trait HasArea {
    fn area(&self) -> f64;
}
trait HasPerimeter {
    fn perimeter(&self) -> f64;
}

trait Shape: HasArea + HasPerimeter {
    fn describe(&self) {
        println!(
            "This shape has an area of {} and a perimeter of {}.",
            self.area(),
            self.perimeter()
        );
    }
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!(
            "Drawing a rectangle with width: {} and height: {}",
            self.width, self.height
        );
    }
}
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
impl HasPerimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Rectangle {}

fn analyze<T: Shape>(shape: &T) {
    shape.describe();
}

fn render<T: Drawable>(shape: &T) {
    shape.draw();
}

fn render_all(shape: &Vec<Box<dyn Drawable>>) {
    for s in shape {
        s.draw();
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r: Rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    render(&c);
    render(&r);
    analyze(&r);

    //second example

    println!("Rendering multiple shapes together:");
    let multi_shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle {
            width: 8.0,
            height: 4.0,
        }),
    ];

    render_all(&multi_shapes);
}
