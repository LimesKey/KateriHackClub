fn main() {
    println!("Hello, world!");
    let returnednumber = plotter(40);
    println!("{}", returnednumber);
}

fn plotter(number: i32) -> i32{
    number * 5 / 100
}
