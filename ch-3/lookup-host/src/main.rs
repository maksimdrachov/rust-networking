

use std::env;
use tokio::net;

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide only one host name!");
        std::process::exit(1);
    } else {
        let addresses = net::lookup_host(&args[1]).await.unwrap();
        for address in addresses {
            println!("{}", address.ip());
        }
    }
}