use clap::{Arg, Command};

fn main() {
    let matches = Command::new("simple-hash-sigs")
        .arg(
            Arg::new("scheme")
                .help("Select the signature scheme")
                .required(true)
                .value_parser(["simple", "ld-ots", "w-ots"]),
        )
        .get_matches();

    let scheme = matches.get_one::<String>("scheme").unwrap();

    println!("Selected scheme: {}", scheme);
}
