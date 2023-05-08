use clap::Subcommand;

#[derive(Subcommand)]
pub enum Account {
    /// Get information about your account
    GetInfo,
    /// Get all addresses associated with your account
    GetAddresses,
    /// Get the name associated with your account
    GetName,
    /// Update the name associated with your account
    SetName {
        /// Name to set for your account
        name: String,
    },
    /// Get all sessions associated with your account
    GetSessions,
    /// Delete a session from your account
    RemoveSession {
        /// ID of the session to remove
        session_id: String,
    },
    /// Get settings associated with your account
    GetSettings,
    /// Update settings associated with your account
    SetSettings {
        /// Temporary JSON data input
        json_data: String,
    },
}

impl Account {
    pub fn process(&self, _email: &str) {
        match self {
            Account::GetInfo => todo!(),
            Account::GetAddresses => todo!(),
            Account::GetName => todo!(),
            Account::SetName { name: _ } => todo!(),
            Account::GetSessions => todo!(),
            Account::RemoveSession { session_id: _ } => todo!(),
            Account::GetSettings => todo!(),
            Account::SetSettings { json_data: _ } => todo!(),
        }
    }
}
