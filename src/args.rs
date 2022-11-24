use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[allow(non_camel_case_types)]
pub struct UCF_Args {
    /// Name of file to be formatted
    pub file_name: String,

    #[arg(short, long)]
    /// Explicitly specify formatter to be used. 
    pub formatter: Option<String>,

//  #[arg(short, long)]
    #[arg(raw(true))]
    /// Custom args for the formatter, supply as trailing args (eg. `ucf file.java -f black --
    /// --diff --color`, where --diff and --color will be directly passed to black. 
    pub args_formatter: Option<String>,
}
