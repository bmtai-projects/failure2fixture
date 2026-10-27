use ulid::Ulid;

pub const ID_PREFIX: &str = "fx_";

pub fn new_fixture_id() -> String {
    format!("{ID_PREFIX}{}", Ulid::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_has_prefix_and_fixed_length() {
        let id = new_fixture_id();
        assert!(id.starts_with(ID_PREFIX));
        assert_eq!(id.len(), ID_PREFIX.len() + 26);
    }

    #[test]
    fn ids_are_unique() {
        assert_ne!(new_fixture_id(), new_fixture_id());
    }
}
