use pnet::datalink;

fn main() {
    for interface in pnet::datalink::interfaces() {
        println!("{} {:?}", interface.name, interface.mac);
    }
}
