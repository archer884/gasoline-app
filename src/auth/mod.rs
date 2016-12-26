mod claims;
mod error;
mod user;

pub use auth::claims::{Claims, Token};
pub use auth::error::AuthError;
pub use auth::user::{authorize, AuthResult};

pub struct Authentication { secret: Vec<u8> }

impl Authentication {
    pub fn new<T: AsRef<[u8]>>(secret: T) -> Authentication {
        Authentication {
            secret: Vec::from(secret.as_ref())
        }
    }
}

// impl BeforeMiddleware for Authentication {
//     fn before(&self, request: &mut Request) -> IronResult<()> {
//         use iron::headers::{Authorization, Bearer};

//         match request.headers.get::<Authorization<Bearer>>().and_then(|header| header.token.parse::<Token>().ok()) {
//             None => Err(IronError::new(AuthError::Unauthorized, "Unauthorized")),
//             Some(token) => {
//                 if !token.is_valid(&self.secret) {
//                     return Err(IronError::new(AuthError::Unauthorized, "Unauthorized"));
//                 }

//                 if !token.payload().is_valid() {
//                     return Err(IronError::new(AuthError::Expired(token.payload().exp), "Token expired"));
//                 }

//                 request.extensions.insert::<Token>(token);

//                 Ok(())
//             }
//         }
//     }
// }
