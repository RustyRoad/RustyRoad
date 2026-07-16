use super::guard;
use super::identity::Identity;

#[test]
fn generated_credentials_pass_guards() {
    let identity = Identity::generate();
    assert!(guard::database(&identity.database).is_ok());
    assert!(guard::user(&identity.user).is_ok());
    assert!(guard::password(&identity.password).is_ok());
}

#[test]
fn unsafe_credentials_are_rejected() {
    for name in [
        "test",
        "production",
        "rustyroad_validate_",
        "rustyroad_validate_bad-name",
    ] {
        assert!(guard::database(name).is_err());
    }
    assert!(guard::user("postgres").is_err());
    assert!(guard::user("rr_validate_bad-name").is_err());
    assert!(guard::password("unsafe'password").is_err());
}
