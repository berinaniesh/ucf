use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[allow(non_camel_case_types)]
pub struct UCF_Args {
    /// Name of file to be formatted
    pub file_name: String,
    #[arg(short, long)]
    /// Explicitly specify formatter to be used. Specify all arguments as a single string separated by space (eg. ucf file.java -f black -a "--color --preview")
    pub formatter: Option<String>,

    #[arg(short, long)]
    /// Custom args for formatter (Doesn't work yet)
    pub args_formatter: Option<String>,
}
