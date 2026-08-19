use rand::distr::Alphanumeric;
use rand::RngExt;

pub(super) const DATABASE_PREFIX: &str = "rustyroad_validate_";
pub(super) const USER_PREFIX: &str = "rr_validate_";

pub(super) struct Identity {
    pub(super) database: String,
    pub(super) user: String,
    pub(super) password: String,
}

impl Identity {
    pub(super) fn generate() -> Self {
        Self {
            database: format!("{DATABASE_PREFIX}{}", random(16).to_ascii_lowercase()),
            user: format!("{USER_PREFIX}{}", random(16).to_ascii_lowercase()),
            password: random(32),
        }
    }
}

fn random(length: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
