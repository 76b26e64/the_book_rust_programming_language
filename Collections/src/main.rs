fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let mut vv = vec![1,2,3,4,5];
    let third: &i32 = &vv[2];
    println!("The third element is {third}");

    let third : Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Panic!
    //let does_not_exist = &v[100];

    // Not Panic! 
    let does_not_exist = v.get(100);
    match does_not_exist {
        Some(does_not_exist) => println!("The does_not_exist is {does_not_exist}"),
        None => println!("There is no does_not_exist element."),
    }

    // It's error
    //let first = &vv[0];
    //vv.push(6);
    //println!("The first element is : {first}");

    /*
    //`vv` moved due to this implicit call to `.into_iter()`
    for mut i in vv {
        i += 50;
    }
    println!("{:?}", vv);
    */

    for i in &mut vv {
        *i += 50;
    }
    println!("{:?}", vv);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");


    // Can not use index for String
    //let s1 = String::from("hello");
    //let u : usize = 0;
    //let h = s1[u];

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
    

}
