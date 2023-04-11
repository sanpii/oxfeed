fn main() {
    #[cfg(debug_assertions)]
    envir::dotenv();

    println!(
        "cargo:rustc-env=API_URL={}",
        envir::get("API_URL").unwrap_or_default()
    );
    println!(
        "cargo:rustc-env=SECRET={}",
        envir::get("SECRET").unwrap_or_default()
    );
}
