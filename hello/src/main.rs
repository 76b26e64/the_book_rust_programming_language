const FORTY_TWO: i32 = 42;
static STATIC_VAL: i32 = 1;
static mut STATIC_MUT_VAL: i32 = 1;

fn print(s: Box<[u8]>) {
    println!("{:?}", s);

}

fn main() {

    let byte_array = [b'h', b'e',b'l',b'l',b'o'];
    print(Box::new(byte_array));

    println!("Hello, world!");
    let a: [i32; 5] = [1,2,3,4,5];
    println!("a = {:?}", a);
    println!("a = {}", FORTY_TWO);
    println!("a = {}", STATIC_VAL);
    unsafe {
        STATIC_MUT_VAL = 2;
        println!("a = {}", STATIC_MUT_VAL);
    }
    let x = 4.24242;
    let y: f32 = 4.24242;
    println!("x = {:.2}, y = {}", x, y);

    let slice: &[i32] = &a[1..5];
    println!("sclie = {:?}", slice);

    let abs = if x < 0.0 {-x} else {x};
    println!("abds = {}", abs);

    let mut i = 0;
    println!("loop:");
    loop {
        println!("i = {}", i);
        if i > 3 {
            break;
        }
        i += 1;
    }
    i = 0;
    println!("while:");
    while i <= 3 {
        println!("i = {}", i);
        i += 1;
    }

    println!("for:");
    for i in 0..4 {
        println!("i = {}", i);
    }

    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("c = {}", c);

    match a {
        1 => println!("1"),
        _ => println!("other"),
    }

    let one: Option<i32> = Some(1);
    println!("one = {}", one);

}

/// Add given two nubmer
fn add(a:i32, b:i32) -> i32 {
    a + b
}
