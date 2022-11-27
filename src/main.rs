fn main() {
    let mut x: i32 = 0;

    while x < 1000000000 {
        x += 1;
    }

    println!("x = {}", x);
}
