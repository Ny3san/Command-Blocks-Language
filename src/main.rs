fn main() {
    std::process::exit(command_blocks::cli::main_entry(
        std::env::args().skip(1).collect(),
    ));
}
