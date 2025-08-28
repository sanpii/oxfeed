pub(crate) async fn get(url: &str) -> oxfeed::Result<Vec<u8>> {
    let path = path(url);

    let body = if path.exists() {
        use std::io::Read as _;

        let mut content = Vec::new();
        let mut file = std::fs::File::open(&path)?;
        file.read_to_end(&mut content)?;

        content
    } else {
        use std::io::Write as _;

        let content = reqwest::get(url).await?.bytes().await?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut file = std::fs::File::create(&path)?;
        file.write_all(&content)?;

        content.to_vec()
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
                .fold(String::new(), |acc, b| format!("{acc}{b:02x}"))
        })
        .collect::<Vec<_>>();

    path.insert(0, envir::get("CACHE_DIR").unwrap());

    path.iter().collect()
}
