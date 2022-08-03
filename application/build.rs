use library::Configuration;

fn main() {
    Configuration::builder()
        .secure(true)
        .build()
        .to_file("config.json");
}