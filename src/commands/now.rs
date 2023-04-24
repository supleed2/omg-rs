use clap::Subcommand;

#[derive(Subcommand)]
pub enum Now {
    /// Get the /now page for an address
    Get,
    /// Get all listed /now pages from now.garden
    List,
    /// Set the contents of the /now page for an address, remember to set the -l flag if you want your /now page listed
    Set {
        /// New content for the /now page
        content: String,
        /// List this /now page in now.garden
        #[arg(short, long, default_value_t = false)]
        listed: bool,
    },
}
