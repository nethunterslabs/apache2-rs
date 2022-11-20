use crate::httpd::Request;

#[derive(PartialEq, Clone, Debug)]
pub struct Cookie<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub expires: Option<i64>,
    pub max_age: Option<u64>,
    pub domain: Option<&'a str>,
    pub path: Option<&'a str>,
    pub secure: bool,
    pub httponly: bool,
}

impl<'a> Cookie<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Cookie<'a> {
        Cookie::<'a> {
            name,
            value,
            expires: None,
            max_age: None,
            domain: None,
            path: Some("/"),
            secure: false,
            httponly: false,
        }
    }

    pub fn attrs(&self, r: &Request) -> String {
        let mut res = String::new();

        if self.httponly {
            res.push_str(";HttpOnly");
        }

        if self.secure {
            res.push_str(";Secure");
        }

        if let Some(ref s) = self.path {
            res.push_str(format!(";Path={}", s).as_ref())
        }

        if let Some(ref s) = self.domain {
            res.push_str(format!(";Domain={}", s).as_ref())
        }

        if let Some(n) = self.max_age {
            res.push_str(format!(";Max-Age={}", n).as_ref())
        }

        if let Some(ref t) = self.expires {
            if let Ok(s) = r.rfc822_date(*t) {
                res.push_str(format!(";Expires={}", s).as_ref())
            }
        }

        if !res.is_empty() {
            res.remove(0);
        }

        res
    }
}
