use clap::Subcommand;

#[derive(Subcommand)]
pub enum Theme {
    /// List available omg.lol profile themes
    List,
    /// Get information about a specific theme
    Info {
        /// ID of the theme to get information for
        id: String,
    },
    /// Get a preview (HTML) of a theme
    Preview {
        /// ID of the theme to get a preview (HTML) of
        id: String,
    },
}

impl Theme {
    pub fn process(&self) {
        match self {
            Theme::List => todo!(),
            Theme::Info { id } => todo!(),
            Theme::Preview { id } => todo!(),
        }
    }
}
