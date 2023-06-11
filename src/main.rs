// enum IpAddrKind{
//     v4{u8, u8, u8, u8},
//     v6{String},
// }

// enum Message{
//     Ouit,
//     Move {x: i32, y:i32},
//     Write{String},
//     ChangeColor{i32, i32, i32},
// }

// impl  Message{
//     fn some_function() {
//         println!("Lets Get Rusty");
//     }
// }

// struct IpAddr{
//     kind: IpAddrKind,
//     address: String,
// }

// #[derive(Debug)]
// enum UsState{
//     Alabama,
//     Alaska,
//     Arizona,
//     Arkansas,
//     California,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quater from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
    // let four : IpAddrKind = IpAddrKind::v4;
    // let six : IpAddrKind = IpAddrKind::v6;

    // let localhost: IpAddr = IpAddr{
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1")
    // }

    // let localhost: IpAddrKind = IpAddrKind::v4(String::from(127, 0,0, 1));

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let some_number: Option<i32> = Some(5);
    // let some_string: Option<&str> = Some("It's a string");
    
    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum: <i8 as Add<Option<i8>>>::Output = x + y;

    // let sum: i8 = x + y.unwrap_or(default: 0);

    // value_in_cents(Coin::Quarter(UsState::Alaska));
    
    // let five: Option<i32> = Some(5);
    // let six: Option<i32> = plus_one(five);
    // let none: Option<i32> = plus_one(None);

//     let some_value: Option<i32> = Some(3);
//     match some_value {
//         Some(3) => println!("three"),
//         _ => (),
//     }
    
//     if let Some(3) = some_value {
//         println!("three");
//     }
// }

// fn route(ip_kind: IpAddrKind) {}


// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         // None => None,
//         Some(i: i32) => Some(i + 1),
//         _ => None
//     }
// }



fn main() {
    let opt: Option<String> = 
        Some(String::from("Hello world"));
    
    // opt became &opt
    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    };
    
    println!("{:?}", opt);
}