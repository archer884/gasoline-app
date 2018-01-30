use chrono::Duration;
use chrono::prelude::*;
use rwt::{Rwt, RwtError};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use service;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Token(pub Rwt<Claims>);

impl Token {
    pub fn is_valid(&self, secret: &[u8]) -> bool {
        self.0.is_valid(secret)
    }

    pub fn inner(&self) -> &Rwt<Claims> {
        &self.0
    }

    pub fn payload(&self) -> &Claims {
        &self.0.payload
    }

    pub fn user(&self) -> &str {
        &self.0.payload.usr
    }

    pub fn timestamp(&self) -> i64 {
        self.0.payload.exp.timestamp()
    }
}

impl FromStr for Token {
    type Err = RwtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Token(s.parse::<Rwt<Claims>>()?))
    }
}

#[derive(Debug)]
pub struct Claims {
    pub exp: DateTime<Utc>,
    pub uid: i64,
    pub usr: String,
}

impl Claims {
    pub fn new<T: Into<String>>(uid: i64, usr: T) -> Claims {
        Claims {
            exp: Utc::now() + Duration::days(7),
            uid,
            usr: usr.into(),
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.exp
    }
}

impl FromStr for Claims {
    type Err = RwtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use serde_json;
        Ok(serde_json::from_str(s).map_err(RwtError::Json)?)
    }
}

impl Serialize for Claims {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Template<'a> {
            exp: i64,
            uid: String,
            usr: &'a str,
        }

        let template = Template {
            exp: self.exp.timestamp(),
            uid: service::encode(self.uid as u64),
            usr: &self.usr,
        };

        template.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Claims {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        fn from_timestamp(expiration: i64) -> Option<DateTime<Utc>> {
            NaiveDateTime::from_timestamp_opt(expiration, 0)
                .map(|datetime| Utc.from_utc_datetime(&datetime))
        }

        #[derive(Deserialize)]
        struct Template {
            exp: i64,
            uid: String,
            usr: String,
        }

        let Template { exp, uid, usr } = Template::deserialize(deserializer)?;
        let exp = from_timestamp(exp).ok_or_else(|| Error::custom("Invalid timestamp"))?;
        let uid = service::decode(&uid).map_err(|_| Error::custom("Invalid user id"))? as i64;

        Ok(Claims { exp, uid, usr })
    }
}
