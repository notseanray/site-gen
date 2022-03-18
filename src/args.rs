pub fn parse_args(args: Vec<String>) {
    println!("recieved args: {}", args.join(" "));
    for argument in args {
        match argument.as_str() {
            "setup" => println!("setup"),
            _ => {
                println!("invalid argument: {argument}, exiting");
                std::process::exit(1);
            }
        }
    }
}
