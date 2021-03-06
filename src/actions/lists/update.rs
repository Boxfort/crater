use config::Config;
use crates::lists::{GitHubList, List, LocalList, RegistryList};
use db::Database;
use errors::*;

pub struct UpdateLists {
    pub github: bool,
    pub registry: bool,
    pub local: bool,
}

impl Default for UpdateLists {
    fn default() -> Self {
        UpdateLists {
            github: true,
            registry: true,
            local: true,
        }
    }
}

impl UpdateLists {
    pub fn apply(self, db: &Database, _config: &Config) -> Result<()> {
        if self.github {
            info!("updating GitHub repositories list");
            GitHubList::default().update(db)?;
        }

        if self.registry {
            info!("updating crates.io crates list");
            RegistryList.update(db)?;
        }

        if self.local {
            info!("updating local crates list");
            LocalList::default().update(db)?;
        }

        Ok(())
    }
}
