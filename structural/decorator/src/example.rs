fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {} value is {}", std::mem::size_of_val(&x), x);
    println!("size of `y` in bytes: {} value is {}", std::mem::size_of_val(&y), y);
    println!("size of `z` in bytes: {} value is {}", std::mem::size_of_val(&z), z);
    println!("size of `i` in bytes: {} value is {}", std::mem::size_of_val(&i), i);
    println!("size of `f` in bytes: {} value is {}", std::mem::size_of_val(&f), f);

    let buf = [0u8; 10];
    for byte in buf {
        print!("byte = {}, ", byte);
    }
}