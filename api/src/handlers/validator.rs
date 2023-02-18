use inflector::cases::screamingsnakecase::is_screaming_snake_case;
use validator::ValidationError;

pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let estimated = zxcvbn::zxcvbn(password, &[])
        .ok()
        .map(|estimate| estimate.score())
        .unwrap_or(0);

    match estimated {
        0..=3 => return Err(ValidationError::new("weak password")),
        _ => Ok(()),
    }
}

pub fn validate_is_screamingsnake_case(name: &str) -> Result<(), ValidationError> {
    if !is_screaming_snake_case(name) {
        return Err(ValidationError::new("is not screaming snake case"))
    }
    Ok(())
}
