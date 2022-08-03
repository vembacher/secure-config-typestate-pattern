use std::path::PathBuf;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    library::run(PathBuf::from(argv[1].as_str()));
}
