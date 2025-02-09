use super::ProfileOpt;
use crate::{options::LicenseAccepter, utils::client::StudioClientConfig, Result};

use saucer::{clap, Parser};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Parser)]
pub struct PluginOpts {
    #[clap(flatten)]
    pub profile: ProfileOpt,

    #[clap(flatten)]
    pub elv2_license_accepter: LicenseAccepter,

    /// Skip the update check for a plugin.
    ///
    /// Passing this flag will attempt to use the latest compatible version of a plugin already installed on this machine.
    #[clap(long = "skip-update")]
    pub skip_update: bool,
}

impl PluginOpts {
    pub fn prompt_for_license_accept(&self, client_config: &StudioClientConfig) -> Result<()> {
        self.elv2_license_accepter
            .require_elv2_license(client_config)
    }
}
