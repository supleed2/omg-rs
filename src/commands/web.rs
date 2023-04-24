use clap::Subcommand;

#[derive(Subcommand)]
pub enum Web {
    /// Get web content and information for an omg.lol address
    Get,
    /// Update web content for an omg.lol address
    Set {
        /// New content for the web page
        content: String,
        /// Publish this page
        #[arg(short, long, default_value_t = false)]
        publish: bool,
    },
    /// Set profile picture for an omg.lol address
    SetPFP {
        /// Path to image to upload as new profile picture
        image: String,
        // TODO: #[arg(value_parser = fn_that_takes_str_ref_and_returns_result_pathbuf)]
        // Eg: #[arg(value_parser = |arg: &str| -> Result<Duration, ParseIntError> {Ok(Duration::from_secs(arg.parse()?))})]
    },
}
