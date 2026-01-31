use argon2::{
    Argon2,
    PasswordHasher,
    password_hash::{
        SaltString,
        rand_core::{
            OsRng,
        },
        PasswordHash,
        PasswordVerifier,
    },
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash);

    match parsed_hash {
        Ok(hash) => Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok(),
        Err(_) => false,
    }
}