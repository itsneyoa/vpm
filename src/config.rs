use std::sync::OnceLock;

pub static VERBOSE: OnceLock<bool> = OnceLock::new();

pub fn init(args: &crate::arguments::Args) {
    VERBOSE
        .set(args.verbose)
        .expect("Could not set global verbose level")
}
