use migration_helpers::common_migrations::RemoveMetadataAndWeakSettingsMigration;
use migration_helpers::{migrate, Result};
use std::process;

// Remove the weak settings and metadata on downgrade
fn run() -> Result<()> {
    migrate(RemoveMetadataAndWeakSettingsMigration)
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
