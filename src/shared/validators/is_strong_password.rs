pub struct PasswordContext {
    min_entropy: f64,
    entropy: cracken::password_entropy::EntropyEstimator,
}

pub fn is_strong_password(value: &str, context: &PasswordContext) -> garde::Result {
    let bits = context
        .entropy
        .estimate_password_entropy(value.as_bytes())
        .map(|e| e.mask_entropy)
        .unwrap_or(0.0);
    if bits < context.min_entropy {
        return Err(garde::Error::new("password is not strong enough"));
    }
    Ok(())
}
