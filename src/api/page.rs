use api::*;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct Page {
    idx: i64,
    size: i64,
}

impl Page {
    pub fn new(idx: i64) -> Page {
        Page::with_size(idx, 10)
    }

    pub fn with_size(idx: i64, size: i64) -> Page {
        Page {
            idx: idx,
            size: size,
        }
    }
}

impl Paging for Page {
    fn offset(&self) -> i64 { self.idx * self.size }
    fn limit(&self) -> i64 { self.size }
}

impl<'a, 'r> FromRequest<'a, 'r> for Page {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        use rocket;

        rocket::outcome::Outcome::Success(
            match request.uri().query() {
                None => Page::new(0),
                Some(query) => query.split(|c| c == '?' || c == '&')
                    .filter(|q| q.starts_with("page="))
                    .nth(0)
                    .and_then(|q| q.split('=').nth(1))
                    .and_then(|n| n.parse::<i64>().ok())
                    .map(|n| Page::new(n))
                    .unwrap_or_else(|| Page::new(0)), 
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use api::Page;
    use service::Paging;
    
    #[test]
    fn page_0_10() {
        let page = Page::new(0);
        
        assert_eq!(0, page.offset());
        assert_eq!(10, page.limit());
    }

    #[test]
    fn page_90_100() {
        let page = Page::new(9);

        assert_eq!(90, page.offset());
        assert_eq!(100, page.offset());
    }
}
