fn main() {
    let version = match version() {
        Ok(version) => version,
        Err(_) => env!("CARGO_PKG_VERSION").to_string(),
    };

    let tera = tera::Tera::new("static/*.html.tera").unwrap();
    let mut context = tera::Context::new();
    context.insert("version", &version);
    let content = tera.render("index.html.tera", &context).unwrap();

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("index.html");

    std::fs::write(path, content).unwrap();
}

fn version() -> Result<String, git2::Error> {
    let dir = std::env::var("GIT_DIR")
        .or_else(|_| std::env::var("CARGO_MANIFEST_DIR").map(|x| format!("{}/..", x)))
        .map_err(|_| git2::Error::from_str("Missing GIT_DIR env var"))?;

    git2::Repository::open(&dir)?
        .head()?
        .target()
        .ok_or_else(|| git2::Error::from_str("Missing target"))
        .map(|x| x.to_string())
}
