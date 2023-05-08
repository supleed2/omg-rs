use clap::Subcommand;

pub mod account;
pub use account::Account;
pub mod address;
pub use address::Address;
pub mod dns;
pub use dns::Dns;
pub mod email;
pub use email::Email;
pub mod now;
pub use now::Now;
pub mod pastebin;
pub use pastebin::Pastebin;
pub mod purl;
pub use purl::Purl;
pub mod status;
pub use status::Status;
pub mod theme;
pub use theme::Theme;
pub mod web;
pub use web::Web;
pub mod weblog;
pub use weblog::Weblog;

// TODO: allow content fields for some commands to provide filepaths, using the content of the file instead
#[derive(Subcommand)]
pub enum Commands {
    /// Get information and make changes to your account
    Account {
        /// Email of your omg.lol account, needed for Account commands only
        #[clap(short, long, global = true)]
        email: String,
        #[clap(subcommand)]
        command: Account,
    },
    /// Get information and make changes to your addresses
    Address {
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
    Dns {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Dns,
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
    Preferences {
        /// Account to change settings for
        owner: String,
        /// ID of setting to update
        item: String,
        /// Value to set "item" to
        value: String,
    },
    /// Manage PURLs (Persistent URLs) for your omg.lol address
    Purl {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Purl,
    },
    /// Get service information about omg.lol
    Service,
    /// Manage the statuslog for an omg.lol address
    Status {
        /// omg.lol address to interact with
        #[clap(short, long, global = true)]
        address: Option<String>,
        #[clap(subcommand)]
        command: Status,
    },
    /// Manage omg.lol profile themes
    Theme {
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

impl Commands {
    fn process(&self) {
        match self {
            Commands::Account { email, command } => {
                command.process(email);
            }
            Commands::Address { address, command } => todo!(),
            Commands::Auth { api_key } => todo!(),
            Commands::Directory => todo!(),
            Commands::Dns { address, command } => todo!(),
            Commands::Email { address, command } => todo!(),
            Commands::Now { address, command } => todo!(),
            Commands::Pastebin { address, command } => todo!(),
            Commands::Preferences { owner, item, value } => todo!(),
            Commands::Purl { address, command } => todo!(),
            Commands::Service => todo!(),
            Commands::Status { address, command } => todo!(),
            Commands::Theme { address, command } => todo!(),
            Commands::Web { address, command } => todo!(),
            Commands::Weblog { address, command } => todo!(),
        }
    }
}
