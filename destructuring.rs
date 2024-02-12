struct Point {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    rgb (u8, u8, u8),
    hsv (u8, u8, u8),
}

fn main() {
    let p = Point { x: 3, y:5, z: 9};

    // value of x is assigned to v1, and value of y to v2
    // ignoring remaining value with ".." from destructuring
    let Point { x: v1, y: v2,  ..} = p;
    println!("1. v1 and v2 are {} {}",v1,v2);

    let msg = Color::rgb(120,0,255);

    // Enum Destructuring
    match msg {
        // returned value of msg will be matched with (r,g,b)
        Color::rgb(r, g, b) => {
            println!("2. We have 3 RGB Colors Combination : {}, {}, {}", r, g, b);
        }
        Color::hsv(h,s,v) => {
            print!("2. We have 3 HSV Colors Combination: {},  {}, {}",h,s,v);
        }
    }
}
