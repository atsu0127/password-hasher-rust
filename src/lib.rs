use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use argon2::password_hash::SaltString;
use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Deserialize, Serialize)]
struct Hash {
    text: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get("/hash/:password", |_req, ctx| {
            if let Some(password) = ctx.param("password") {
                let hash = compute_password_hash(password);
                return match hash {
                    Ok(text) => {
                        let hash = Hash {
                            text
                        };
                        Response::from_json(&hash)
                    }
                    Err(err) => Response::error(format!("Internal Error: {}", err.to_string()), 500),
                };
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}

fn compute_password_hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    println!("salt: {}", salt);
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(65536, 3, 4, Some(32)).unwrap(),
    )
        .hash_password(password.as_bytes(), &salt);
    println!("password_hash: {:?}", password_hash);
    match password_hash {
        Ok(hash) => {
            Ok(hash.to_string())
        }
        Err(err) => {
            Err(Error::from(err.to_string()))
        }
    }
}