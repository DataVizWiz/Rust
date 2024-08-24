struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
       self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    // Rectangle is passed as a reference so dimensions can be checked without
    // taking ownership, allowing the caller to retain ownership of Rectangle.
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn scale(&mut self, scale_factor: f64) {
        if scale_factor <= 0.0 {
            println!("Warning: Scale factor must be positive.")
        } else {
            self.width *= scale_factor;
            self.height *= scale_factor;
        }

    }

    fn diagonal_length(&self) -> f64 {
        (self.width * self.width + self.height * self.height).sqrt()
    }
}

fn main() {
    let mut rec = Rectangle {
        width: 50.0,
        height: 20.0
    };

    println!("{}", rec.width);
    println!("{}", rec.height);

    let area: f64 = rec.area();
    println!("{}", area);

    let perimeter: f64 = rec.perimeter();
    println!("{}", perimeter);

    let rec2 = Rectangle {
        width: 25.0,
        height: 15.0
    };

    if rec.can_hold(&rec2) {
        println!("Rectangle 2 can fit in Rectangle 1");
    } else {
        println!("Rectangle 2 is too large for Rectangle 1");
    };

    rec.scale(3.0);
    println!("{}", rec.width);
    println!("{}", rec.height);

    let d_len = rec.diagonal_length();
    println!("{}", d_len);
}