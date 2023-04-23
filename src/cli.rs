use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    /// Set which omg.lol username to use, overrides config and environment variable (OMGLOL_USERNAME)
    #[clap(short, long)]
    pub name: Option<String>,
    /// Categories of commands to interact with the omg.lol API
    #[clap(subcommand)]
    pub command: Option<Commands>,
    /// Print debug information, repeat for higher levels of debug info (max 1)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

// TODO: allow content fields for some commands to provide filepaths, using the content of the file instead

#[derive(Subcommand)]
pub enum Commands {
    /// Get information and make changes to your account
    Account{ // TODO: include or not?
        /// Email of your omg.lol account, needed for Account commands only
        #[clap(short, long, global = true)]
        email: Option<String>,
        #[clap(subcommand)]
        command: Account,
    },
    /// Get information and make changes to your addresses
    Address{
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Address,
    },
    /// Save your omg.lol API key to the config.json (Rather than using the OMGLOL_API_KEY environment variable)
    Auth {
        /// API key to save to config.json
        api_key: String,
    },
    /// Get the address directory, consisting of addresses that have opted-in to be listed
    Directory,
    /// Adjust the switchboard / DNS records for your omg.lol address
    DNS {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: DNS,
    },
    /// Manage the email configuration for an omg.lol address
    Email {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Email,
    },
    /// Manage your /now page
    Now {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Now,
    },
    /// Manage the pastebin for an omg.lol address
    Pastebin {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Pastebin,
    },
    /// Manage preferences for omg.lol accounts, addresses and objects
    Preferences{ // TODO: include or not?
        /// Account to change settings for
        owner: String,
        /// ID of setting to update
        item: String,
        /// Value to set "item" to
        value: String,
    },
    /// Manage PURLs (Persistent URLs) for your omg.lol address
    PURL {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: PURL,
    },
    /// Get service information about omg.lol
    Service, // TODO: include or not?
    /// Manage the statuslog for an omg.lol address
    Status {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Status,
    },
    /// Manage omg.lol profile themes
    Theme { // TODO: include or not?
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Theme,
    },
    /// Manage profile page and web stuff for an omg.lol address
    Web {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Web,
    },
    /// Manage the weblog for an omg.lol address
    Weblog {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Weblog,
    },
}

#[derive(Subcommand)]
pub enum Account {
    /// Get information about your account
    GetInfo,
    /// Get all addresses associated with your account
    GetAddrs,
    /// Get the name associated with your account
    GetName,
    /// Update the name associated with your account
    SetName {
        name: String,
    },
    /// Get all sessions associated with your account
    GetSessions,
    /// Delete a session from your account
    RemoveSession {
        session_id: String,
    },
    /// Get settings associated with your account
    GetSettings,
    /// Update settings associated with your account
    SetSettings {
        json_data: String,
    },
}

#[derive(Subcommand)]
pub enum Address {
    /// Get information about the availability of an address
    IsAvailable,
    /// Get the expiration date for an address
    GetExpiry,
    /// Get limited (public) information about an address (no auth required)
    GetPublicInfo,
    ///Get comprehensive information about an address
    GetInfo,
}

#[derive(Subcommand)]
pub enum DNS {
    /// Get a list of all your DNS records
    GetRecords,
    /// Add a new DNS record
    AddRecord {
        json_data: String,
    },
    /// Update an existing DNS record
    UpdateRecord {
        json_data: String,
    },
    /// Delete a DNS record
    DeleteRecord {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum Email {
    /// Get forwarding address(es)
    GetForwards,
    /// Set forwarding address(es)
    SetForwards {
        /// Addresses to forward emails to
        json_data: String,
    },
}

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

#[derive(Subcommand)]
pub enum Pastebin {
    /// Get a specific paste for an omg.lol address
    Get {
        /// Name of the paste to get
        name: String,
    },
    /// Get all pastes for an omg.lol address
    GetAll,
    /// Get all public pastes for an omg.lol address
    GetAllPublic,
    /// Create/update a paste for an omg.lol address
    Set {
        /// Name of the paste to create (and the address used to retrieve it)
        name: String,
        /// Content of the paste
        content: String,
    },
    /// Detele a paste for an omg.lol address
    Delete {
        /// Name of the paste to delete
        name: String,
    },
}

#[derive(Subcommand)]
pub enum PURL {
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

#[derive(Subcommand)]
pub enum Status  {
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
        /// New contrent for the statuslog entry
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