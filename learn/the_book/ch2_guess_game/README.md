### Ch.2 Guessing Game Notes

Crate Versions:  
- Cargo.lock file maintains crate (dependency) versions unless explicity upgraded
- `cargo update`: ignores Cargo.lock and attempts to meet Cargo.toml specifications, if succesfull updates the Cargo.lock to new crate versions

Documentation Generation:
- `cargo doc --open`: builds documentation provided by dependencies locally and open in browser

