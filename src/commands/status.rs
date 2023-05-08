use clap::Subcommand;

#[derive(Subcommand)]
pub enum Status {
    /// Get a single statuslog entry for an omg.lol address
    Get {
        /// ID of the statuslog entry to get
        id: String,
    },
    /// Get entire statuslog for an omg.lol address
    GetAll,
    /// Create a new statuslog entry for an omg.lol address
    Create {
        /// Emoji to use for the statuslog entry
        emoji: String,
        /// Content for the statuslog entry
        content: String,
        /// External URL to link to from the statuslog entry
        external_url: String, // TODO: should this be optional?
    },
    /// Create a new statuslog entry for an omg.lol address from a single string
    EasyCreate {
        /// Status to share
        status: String,
    },
    /// Update the content of an existing statuslog entry for an omg.lol address
    Update {
        /// ID of the statuslog entry to update
        id: String,
        /// New emoji to use for the statuslog entry
        emoji: String,
        /// New content for the statuslog entry
        content: String,
        // TODO: should there be an external url here?
    },
    /// Get a statuslog bio
    GetBio,
    /// Update a statuslog bio
    SetBio {
        /// New content for statuslog bio
        content: String,
    },
    /// Get all statuslog entries for all addresses
    GetAllHistorical,
    /// Get the most recent statuslog entries across omg.lol
    Timeline,
}

impl Status {
    pub fn process(&self) {
        match self {
            Status::Get { id } => todo!(),
            Status::GetAll => todo!(),
            Status::Create { emoji, content, external_url } => todo!(),
            Status::EasyCreate { status } => todo!(),
            Status::Update { id, emoji, content } => todo!(),
            Status::GetBio => todo!(),
            Status::SetBio { content } => todo!(),
            Status::GetAllHistorical => todo!(),
            Status::Timeline => todo!(),
        }
    }
}
