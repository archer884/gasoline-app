use rwt::Rwt;

use chrono::duration::Duration;
use chrono::{
    DateTime,
    NaiveDateTime,
    TimeZone,
    UTC,
};

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer
};

pub type Token = Rwt<Claims>;

pub struct Claims {
    pub exp: DateTime<UTC>,
    pub usr: String,
}

impl Claims {
    pub fn new<T: Into<String>>(usr: T) -> Claims {
        Claims {
            usr: usr.into(),
            exp: UTC::now() + Duration::days(7)
        }
    }

    pub fn is_valid(&self) -> bool {
        UTC::now() < self.exp
    }
}

impl Serialize for Claims {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        #[derive(Serialize)]
        struct Template<'a> {
            exp: i64,
            usr: &'a str,
        }

        Template {
            exp: self.exp.num_seconds_from_unix_epoch(),
            usr: &self.usr,
        }.serialize(serializer)
    }
}

impl Deserialize for Claims {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        use serde::de::Error;

        #[derive(Deserialize)]
        struct Template {
            exp: i64,
            usr: String,
        }

        let template = Template::deserialize(deserializer)?;
        Ok(Claims {
            usr: template.usr,
            exp: match from_timestamp(template.exp) {
                None => return Err(Error::custom("Invalid datetime")),
                Some(datetime) => datetime,
            },
        })
    }
}

fn from_timestamp(expiration: i64) -> Option<DateTime<UTC>> {
    NaiveDateTime::from_num_seconds_from_unix_epoch_opt(expiration, 0).map(|datetime| UTC.from_utc_datetime(&datetime))
}
