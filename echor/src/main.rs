use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Heejun Shin")
        .about("Rust echo")
        // .arg(
        //     Arg::with_name("test")
        //         .value_name("TEST")
        //         .help("Input test")
        //         .required(true)
        //         .min_values(1),
        // )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("test input third value")
                .value_name("Third value")
                .help("Third value test")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    println!("{:#?}", matches);
}
