use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

pub fn hash_password(password: &str) -> String {
    let hashed = hash(password, DEFAULT_COST).expect("hash have problem");
    println!("{}", &hashed);
    hashed
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    let check_password_validation = verify(password, hash);
    check_password_validation
}
