use clap::Parser;

use crate::utils::version;

const DEFAULT_PROGRAM_PATH: &str = "./program.tasm";
const DEFAULT_FRAME_RATE: f64 = 32.0;

#[derive(Debug, Clone, PartialEq, Parser)]
#[command(author, version = version(), about)]
pub(crate) struct Args {
    #[arg(
        short,
        long,
        value_name = "PATH",
        default_value_t = String::from(DEFAULT_PROGRAM_PATH),
    )]
    /// Path to program to run
    pub program: String,

    #[arg(short, long, value_name = "PATH")]
    /// Path to file containing public input
    pub input: Option<String>,

    #[arg(short, long, value_name = "PATH")]
    /// Path to JSON file containing all non-determinism
    pub non_determinism: Option<String>,

    #[arg(
        short,
        long,
        value_name = "FLOAT",
        default_value_t = DEFAULT_FRAME_RATE
    )]
    /// Frame rate, i.e. number of frames per second
    pub frame_rate: f64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            program: DEFAULT_PROGRAM_PATH.into(),
            input: None,
            non_determinism: None,
            frame_rate: DEFAULT_FRAME_RATE,
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::assert;

    use super::*;

    #[test]
    fn default_cli_args_and_clap_parsing_with_no_args_are_identical() {
        let cli_args: Vec<String> = vec![];
        let args = Args::parse_from(cli_args);
        assert!(Args::default() == args);
    }
}
