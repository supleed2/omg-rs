use clap::Subcommand;

#[derive(Subcommand)]
pub enum Purl {
    /// Create a new PURL for an omg.lol address
    Create {
        /// Name of the PURL to create
        name: String,
        /// URL for the PURL to redirect to
        url: String,
    },
    /// Get a specific PURL for an omg.lol address
    Get {
        /// Name of the PURL to get
        name: String,
    },
    /// List all PURLs for an omg.lol address
    List,
    /// Delete a PURL for an omg.lol address
    Delete {
        /// Name of the PURL to delete
        name: String,
    },
}

impl Purl {
    pub fn process(&self) {
        match self {
            Purl::Create { name, url } => todo!(),
            Purl::Get { name } => todo!(),
            Purl::List => todo!(),
            Purl::Delete { name } => todo!(),
        }
    }
}
