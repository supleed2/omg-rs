use clap::Subcommand;

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
