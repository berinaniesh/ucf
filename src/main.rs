use std::env;
use args::{Args, ArgsError};

const PROGRAM_NAME: &'static str = "ucf";
const PROGRAM_DESC: &'static str = "Universal Code Formatter";

fn print_help() {
    let help_string: String = String::from("Help");
    println!("{}", help_string);
}

fn parse(input: &Vec<String>) -> Result<(String), ArgsError> {
   let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
   args.flag("h", "help", "Print the usage menu");

   args.parse(input)?;

   let is_help = args.value_of("help")?;
   if is_help {
       args.full_usage();
   }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let (file_name) = parse(&args).unwrap();

    let _languages = [
        ("C", ".c", "clang-format"),
        ("C++", ".cpp", "clang-format"),
        ("C++", ".cc", "clang-format"),
        ("Java", ".java", "google-java-format"),
        ("Python", ".py", "black"),
        ("Javascript", ".js", "prettier"),
        ("Typescript", "ts", "prettier"),
    ];

    match args[1].as_str() {
        "--help" => print_help(),
        _ => {}
    }
}
