#[derive(Clone, Default, Eq, PartialEq)]
pub(crate) struct Filter {
    q: String,
    source: String,
    tag: String,
}

impl Filter {
    pub fn new() -> Self {
        Self::from(&crate::Location::new())
    }

    pub fn from(location: &crate::Location) -> Self {
        Self {
            q: location.q(),
            source: location.param("source"),
            tag: location.param("tag"),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty() && self.tag.is_empty() && self.source.is_empty()
    }

    pub fn to_url_param(&self) -> String {
        let mut params = Vec::new();

        if !self.q.is_empty() {
            params.push(format!("q={}", urlencoding::encode(self.q.trim())));
        }

        if !self.source.is_empty() {
            params.push(format!("source={}", urlencoding::encode(&self.source)));
        }

        if !self.tag.is_empty() {
            params.push(format!("tag={}", urlencoding::encode(&self.tag)));
        }

        params.join("&")
    }
}

impl From<String> for Filter {
    fn from(query: String) -> Self {
        let mut q = query.clone();
        let mut source = String::new();
        let mut tag = String::new();

        let regex = regex::Regex::new(r#"(:?source=(?P<source>[^ ]+) )?(tag=(?P<tag>[^ ]+) )?(?P<q>.*)"#).unwrap();

        if let Some(captures) = regex.captures(&query) {
            q = captures
                .name("q")
                .map(|x| x.as_str().to_string())
                .unwrap_or_else(|| query.clone());
            source = captures
                .name("source")
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
            tag = captures
                .name("tag")
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
        }

        Self { q, source, tag }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.source.is_empty() {
            write!(f, "source={} ", self.source)?;
        }

        if !self.tag.is_empty() {
            write!(f, "tag={} ", self.tag)?;
        }

        if !self.q.is_empty() {
            f.write_str(&self.q)?;
        }

        Ok(())
    }
}
