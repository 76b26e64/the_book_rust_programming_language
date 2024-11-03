fn main() {
    /* 
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    struct IpAddr_s<T> {
        address: T,
    }

    impl<T> IpAddr_s<T> {
        fn new(&self, addr: T) -> Self {
                IpAddr_s {address: addr},
        }
    }
    */

    //let config_max = Some(3u8);
    let config_max = Some("string");
    if let Some(max) = config_max {
        println!("the maximum is configured to be {max}");
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    //let five = Some(5);
    let five = 5;  // Compile error
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six = {:?}", six);
    println!("none = {:?}", none);


}
