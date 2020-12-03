pub(crate) fn get(url: &str) -> oxfeed_common::Result<Vec<u8>> {
    let path = path(url);

    let body = if path.exists() {
        use std::io::Read;

        let mut content = Vec::new();
        let mut file = std::fs::File::open(&path)?;
        file.read_to_end(&mut content)?;

        content
    } else {
        use std::io::Write;

        let content = attohttpc::RequestBuilder::try_new(attohttpc::Method::GET, url)?
            .send()?
            .bytes()?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = std::fs::File::create(&path)?;
        file.write_all(&content)?;

        content
    };

    Ok(body)
}

fn path(url: &str) -> std::path::PathBuf {
    let digest = ring::digest::digest(&ring::digest::SHA256, url.as_bytes());

    let mut path = digest
        .as_ref()
        .chunks(4)
        .map(|x| {
            x.iter()
                .fold(String::new(), |acc, b| format!("{}{:02x}", acc, b))
        })
        .collect::<Vec<_>>();

    path.insert(
        0,
        std::env::var("CACHE_DIR").expect("Missing CACHE_DIR env variable"),
    );

    path.iter().collect()
}
