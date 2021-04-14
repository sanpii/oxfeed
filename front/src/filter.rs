#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Filter {
    q: String,
    tag: String,
}

impl Filter {
    pub fn new() -> Self {
        Self::from(&crate::Location::new())
    }

    pub fn from(location: &crate::Location) -> Self {
        Self {
            q: location.q(),
            tag: location.param("tag"),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty() && self.tag.is_empty()
    }

    pub fn to_url_param(&self) -> String {
        let mut params = Vec::new();

        if !self.q.is_empty() {
            params.push(format!("q={}", urlencoding::encode(self.q.trim())));
        }

        if !self.tag.is_empty() {
            params.push(format!("tag={}", urlencoding::encode(&self.tag)));
        }

        params.join("&")
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for Filter {
    fn from(query: String) -> Self {
        let mut q = query.clone();
        let mut tag = String::new();

        let regex = regex::Regex::new(r#"(tag=(?P<tag>[^ ]+))?(?P<q>.*)"#).unwrap();

        if let Some(captures) = regex.captures(&query) {
            q = captures
                .name("q")
                .map(|x| x.as_str().to_string())
                .unwrap_or_else(|| query.clone());
            tag = captures
                .name("tag")
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
        }

        Self { q, tag }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.tag.is_empty() {
            write!(f, "tag={}", self.tag)?;
        }

        if !self.q.is_empty() {
            f.write_str(&self.q)?;
        }

        Ok(())
    }
}
