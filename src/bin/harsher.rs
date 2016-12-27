extern crate harsh;

fn main() {
    let harsh = harsh::HarshBuilder::new()
        .length(8)
        .salt("this is a terrible salt")
        .init()
        .unwrap();

    let id = match std::env::args().nth(1).map(|n| n.parse::<u64>()) {
        Some(Ok(id)) => id,
        _ => {
            println!("enter an id");
            std::process::exit(1);
        }
    };
    
    println!("{}", harsh.encode(&[id]).unwrap());
}