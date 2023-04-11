fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    println!(
        "cargo:rustc-env=API_URL={}",
        std::env::var("API_URL").unwrap_or_default()
    );
    println!(
        "cargo:rustc-env=SECRET={}",
        std::env::var("SECRET").unwrap_or_default()
    );
}
