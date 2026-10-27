//! Shared data model for fixtures. Pure data, no I/O.

mod fixture;
mod id;

pub use fixture::{
    CommandSpec, EnvFingerprint, FailureObservation, Fixture, RepoSnapshot, SCHEMA_VERSION,
};
pub use id::{ID_PREFIX, new_fixture_id};
