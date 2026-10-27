//! Saves and loads fixtures under `<repo>/.f2f/fixtures/`.

use std::path::PathBuf;

use f2f_core::Fixture;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),
}

pub struct FixtureStore {
    repo_root: PathBuf,
}

impl FixtureStore {
    pub fn open(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }

    pub fn fixtures_dir(&self) -> PathBuf {
        self.repo_root.join(".f2f").join("fixtures")
    }

    pub fn load(&self, _id: &str) -> Result<Fixture, StoreError> {
        Err(StoreError::NotImplemented("load"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixtures_live_under_dot_f2f() {
        let store = FixtureStore::open("repo");
        assert_eq!(store.fixtures_dir(), PathBuf::from("repo/.f2f/fixtures"));
    }
}
