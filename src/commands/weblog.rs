use clap::Subcommand;

#[derive(Subcommand)]
pub enum Weblog {
    /// Get a specific weblog entry for an omg.lol address
    Get {
        /// ID of the weblog entry to get
        id: String,
    },
    /// Get the latest weblog entry for an omg.lol address
    Latest,
    /// Get all weblog entries for an omg.lol address
    GetAll,
    /// Create a new weblog entry for an omg.lol address
    Create {
        /// Content for the weblog entry
        content: String,
    },
    /// Delete a weblog entry for an omg.lol address
    Delete {
        /// ID of the weblog entry to delete
        id: String,
    },
    /// Get weblog configuration for an omg.lol address
    GetConfig,
    /// Update weblog configuration for an omg.lol address
    SetConfig {
        /// Content for the weblog configuration entry
        content: String,
    },
    /// Get the weblog template for an omg.lol address
    GetTemplate,
    /// Update the weblog template for an omg.lol address
    SetTemplate {
        /// Content for the weblog template entry
        content: String,
    },
}
