enum IpAddrKind{
    v4{u8, u8, u8, u8},
    v6{String},
}

enum Message{
    Ouit,
    Move {x: i32, y:i32},
    Write{String},
    ChangeColor{i32, i32, i32},
}

impl  Message{
    fn some_function() {
        println!("Lets Get Rusty");
    }
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // let four : IpAddrKind = IpAddrKind::v4;
    // let six : IpAddrKind = IpAddrKind::v6;

    // let localhost: IpAddr = IpAddr{
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1")
    // }

    let localhost: IpAddrKind = IpAddrKind::v4(String::from(127, 0,0, 1));

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // let some_number: Option<i32> = Some(5);
    // let some_string: Option<&str> = Some("It's a string");
    
    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum: <i8 as Add<Option<i8>>>::Output = x + y;

    // let sum: i8 = x + y.unwrap_or(default: 0);


}

// fn route(ip_kind: IpAddrKind) {}
