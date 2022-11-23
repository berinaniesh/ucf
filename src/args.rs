use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[allow(non_camel_case_types)]
pub struct UCF_Args {
    /// Name of file to be formatted
    pub file_name: String,
}
