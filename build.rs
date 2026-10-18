use std::path::Path;

fn main() {
    // src/wifi_config.rs holds real credentials and is git-ignored.
    // On a fresh clone, create it from the template so the build works.
    let config = Path::new("src/wifi_config.rs");
    let template = Path::new("src/wifi_config.rs.example");
    if !config.exists() {
        std::fs::copy(template, config).expect("copy src/wifi_config.rs.example");
        println!("cargo:warning=Created src/wifi_config.rs from template - fill in your Wi-Fi credentials");
    }
    println!("cargo:rerun-if-changed=src/wifi_config.rs");
    println!("cargo:rerun-if-changed=src/wifi_config.rs.example");

    embuild::espidf::sysenv::output();
}
