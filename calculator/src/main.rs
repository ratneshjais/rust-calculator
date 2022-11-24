fn main() {
    ratnesh();
    let x = 10;
    let y = 10;
    println!("{}",add_val(x,y));
}

fn ratnesh() {
    let z = String::from("Hi! welcome to the rust calculator");
    println!("{}", z);
}

fn add_val(x: u8, y: u8) -> u8{
    return x+y;
}