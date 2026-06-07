#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Filter {
    active: Option<bool>,
    q: String,
    source: String,
    tag: String,
    title: String,
}

impl Filter {
    #[must_use]
    pub fn new(location: &crate::Location) -> Self {
        Self::from(location)
    }

    #[must_use]
    fn from(location: &crate::Location) -> Self {
        Self {
            active: location.active(),
            q: location.q(),
            source: location.param("source"),
            tag: location.param("tag"),
            title: location.param("title"),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.active.is_none()
            && self.q.is_empty()
            && self.source.is_empty()
            && self.tag.is_empty()
            && self.title.is_empty()
    }

    #[must_use]
    pub fn to_url_param(&self) -> String {
        let mut params = Vec::new();

        if let Some(active) = self.active {
            params.push(format!("active={active}"));
        }

        if !self.q.is_empty() {
            params.push(format!("q={}", urlencoding::encode(&self.q)));
        }

        if !self.source.is_empty() {
            params.push(format!("source={}", urlencoding::encode(&self.source)));
        }

        if !self.tag.is_empty() {
            params.push(format!("tag={}", urlencoding::encode(&self.tag)));
        }

        if !self.title.is_empty() {
            params.push(format!("title={}", urlencoding::encode(&self.title)));
        }

        params.join("&")
    }
}

impl From<String> for Filter {
    fn from(query: String) -> Self {
        static REGEX: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();

        let mut active = None;
        let mut q = query.clone();
        let mut source = String::new();
        let mut tag = String::new();
        let mut title = String::new();

        let regex = REGEX.get_or_init(|| {
            regex::Regex::new(r#"(:?active=(?P<active>[^ ]+) )?(:?source=(?P<source>[^ ]+) )?(tag=(?P<tag>[^ ]+) )?(title=(?P<title>[^ ]+) )?(?P<q>.*)"#)
                .unwrap()
        });

        if let Some(captures) = regex.captures(&query) {
            active = captures
                .name("active")
                .and_then(|x| x.as_str().parse().ok());
            q = captures
                .name("q")
                .map_or_else(|| query.clone(), |x| x.as_str().to_string());
            source = captures
                .name("source")
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
            tag = captures
                .name("tag")
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
            title = captures
                .name("title")
                .map(|x| x.as_str().to_string())
                .unwrap_or_default();
        }

        Self {
            active,
            q,
            source,
            tag,
            title,
        }
    }
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(active) = self.active {
            write!(f, "active={active} ")?;
        }

        if !self.source.is_empty() {
            write!(f, "source={} ", self.source)?;
        }

        if !self.tag.is_empty() {
            write!(f, "tag={} ", self.tag)?;
        }

        if !self.title.is_empty() {
            write!(f, "title={} ", self.title)?;
        }

        if !self.q.is_empty() {
            f.write_str(&self.q)?;
        }

        Ok(())
    }
}
