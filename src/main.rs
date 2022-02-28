use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Ed Sweeney <ed@onextent.com>")
        .about("Rust echor")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit newline")
                .short("n")
                .help("Input text")
                .takes_value(false),
        )
        .get_matches();
    println!("{:#?}", matches);
}
