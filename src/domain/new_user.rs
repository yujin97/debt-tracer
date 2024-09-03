use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use uuid::Uuid;

pub struct NewUser {
    pub user_id: String,
    pub username: String,
    pub password_hash: String,
    pub email: String,
}

impl NewUser {
    pub fn new(username: String, password: String, email: String) -> Self {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

        Self {
            user_id: Uuid::new_v4().to_string(),
            username,
            password_hash,
            email,
        }
    }
}
