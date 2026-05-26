use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("balance") => {
            let acc = &args[2];
            println!("Saldo {} = <RPC placeholder>", acc);
        }
        Some("transfer") => {
            println!("Invio tx (RPC placeholder)");
        }
        _ => println!("Comandi: balance, transfer"),
    }
}
