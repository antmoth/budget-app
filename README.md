To build:
Install postgresql
Set up a postgresql database called `budget_app`
If you're on systemd, set postgresql to run always: `systemctl enable postgresl`
Install Rust Nightly using [rustup.sh](https://rustup.rs/)
If for some ungodly reason you haven't installed one, make sure you have `gcc` or another C compiler installed.
Install the Diesel CLI to manage database migrations: `cargo install diesel_cli --no-default-features --features postgres`
Run the database migrations: `diesel migration run`
Install dependencies: `cargo install`
Run: `cargo run`
