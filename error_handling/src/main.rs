use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    /*
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
    */

    // The ? placed after a Result value is defined to work in almost the same way 
    // as the match expressions we defined to handle the Result values in Listing 9-6.
    // If the value of the Result is an Ok, the value inside the Ok will get returned
    // from this expression, and the program will continue. 
    // If the value is an Err, the Err will be returned from the whole function 
    // as if we had used the return keyword so the error value gets propagated to the calling code.
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/* 
fn main() {
    //panic!("crash and burn");

//    let v = vec![1, 2, 3];
//    v[99];

    /*
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {error:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {error:?}");
            },
        },
    };
    */

    //let greeting_file_result = File::open("hello.txt").unwrap();
    //let greeting_file_result = File::open("hello.txt").expect("hello.txt should be included in this project");
    let ret = read_username_from_file();
    println!("{:?}", ret);
}
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ret = File::open("hello.txt")?;
    println!("{:?}", ret);
    Ok(())
}
