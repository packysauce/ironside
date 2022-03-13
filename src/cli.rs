use std::ffi::OsString;

#[derive(clap::Parser)]
pub struct CliArgs {
    /// Input TTY name
    #[clap(short = 'I', long = "input-tty", default_value = "/tmp/printer")]
    input_tty: OsString,
    /// API server Unix Domain Socket filename
    #[clap(short = 'a', long = "api-server")]
    api_server: Option<OsString>,
    /// Write to log file instead of stderr
    #[clap(short = 'l', long)]
    logfile: Option<OsString>,
    /// Enable debug messages
    #[clap(short = 'v', long, parse(from_occurrences))]
    verbose: usize,
    /// Read commands from file instead of serial port
    #[clap(short = 'i', long)]
    debuginput: Option<OsString>,
    /// Write output to file instead of serial port
    #[clap(short = 'o', long)]
    debugoutput: Option<OsString>,
    /// File to read for MCU protocol dictionary
    #[clap(short = 'd', long)]
    dictionary: OsString,
    /// Perform an import module test
    #[clap(long)]
    import_test: bool,
    /// Name of the config file to use
    config_file: OsString,
}
