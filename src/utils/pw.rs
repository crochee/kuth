use argon2::{self, Config, ThreadMode};
use rand::Rng;

use crate::{Error, Result};

pub fn sign_password(password: &str) -> Result<String> {
    let password_salt = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(255)
        .map(char::from)
        .collect::<String>();

    let cfg = Config {
        ad: b"kuth associated data",
        hash_length: 64,
        secret: b"kuth secret data",
        thread_mode: ThreadMode::Parallel,
        time_cost: 12,
        ..Default::default()
    };

    argon2::hash_encoded(password.as_bytes(), password_salt.as_bytes(), &cfg).map_err(Error::any)
}

pub fn verity_password(password: &str, src: &str) -> Result<bool> {
    argon2::verify_encoded_ext(
        src,
        password.as_bytes(),
        b"kuth secret data",
        b"kuth associated data",
    )
    .map_err(Error::any)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_argon2() {
        let p = String::from("io");
        let h = super::sign_password(&p).unwrap();
        assert_eq!(h.len(), 457);
        assert!(super::verity_password(&p, &h).unwrap())
    }

    #[test]
    fn test_sign() {
        let p = String::from("a01234567890123456");
        let h=String::from("$argon2i$v=19$m=4096,t=12,p=1$ZE1idTZHTmFLV25hNnZLaGR6dDF1eTVSc1BZcmlxN2VSaDlFN2NYRXlBbmdQQ2oxbU1CTjYzY09yWld6dGxKQlZLZHJ4UUtMc2lmRmZpVGZvbzBXRkh3NnpYcFpjU1dvWjJ2TGEwQTJ0NUw2bEsyREZKcFl3bWRhSGFIOU5rdG84QWl4UlV1QWFnSEUxY01SRmE3MTg0MjRlWFdyNjdRd0NsZGRQVEFBcUlta3BicXJteUtObnhGQWxaa2Z6QlBJYUlRUzBVa20wcVlydHdNR0tSc1BpcjBNckVNeVBSaHFhVUZwc0t4NEJxQWFqOFY3M3Nid0NHdlBHSXJqODd0$VQiBk+o3jOBJ0Ytja2v5QIxge2Kf5B5DG8T/WQpb6EezobKWjlmocOdm+t81LZTjuhJry5AhfKJ2XR1eVJ02XA");
        assert!(super::verity_password(&p, &h).unwrap())
    }
}
