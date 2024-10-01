fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    takes_ownership(&s);
    println!("{s}");

    let x  = 5;
    let y  = x;
    make_copy(x);
    println!("x={x} y={y}");

    let s2 = s.clone();
    println!("{s}");
    
    change(&mut s);
    println!("{s}");
}

fn takes_ownership(some_string: &String) {
    println!("{some_string}");
}

fn make_copy(some_interger: i32){
    println!("{some_interger}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}