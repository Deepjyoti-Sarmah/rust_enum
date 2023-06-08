enum IpAddrKind{
    v4,
    v6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four : IpAddrKind = IpAddrKind::v4;
    let six : IpAddrKind = IpAddrKind::v6;

    let localhost: IpAddr = IpAddr{
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1")
    }

}

fn route(ip_kind: IpAddrKind) {}
