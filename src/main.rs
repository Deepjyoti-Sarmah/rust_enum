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

}

fn route(ip_kind: IpAddrKind) {}
