use clap::Subcommand;

#[derive(Subcommand)]
pub enum Dns {
    /// Get a list of all your DNS records
    GetRecords,
    /// Add a new DNS record
    AddRecord {
        /// Temporary JSON data input
        json_data: String,
    },
    /// Update an existing DNS record
    UpdateRecord {
        /// Temporary JSON data input
        json_data: String,
    },
    /// Delete a DNS record
    DeleteRecord {
        /// ID of the DNS record to delete
        id: String,
    },
}

impl Dns {
    pub fn process(&self) {
        match self {
            Dns::GetRecords => todo!(),
            Dns::AddRecord { json_data } => todo!(),
            Dns::UpdateRecord { json_data } => todo!(),
            Dns::DeleteRecord { id } => todo!(),
        }
    }
}
