// `sqlx::migrate!` embeds the migration files at compile time; without this Cargo does not notice
// a new file in `migrations/` and the server would start without applying it.
fn main() {
    println!("cargo:rerun-if-changed=../../migrations");
}
