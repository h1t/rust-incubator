use anyhow::Result;
use clap::Parser;
use step_4_3::{
    db::{Role, User},
    Command, CommandResponse,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// Path to configuration file
    #[clap(
        short,
        long,
        env = "DB_URL",
        default_value = "http://127.0.0.1:3000/command"
    )]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args { command, url } = Args::parse();
    let response: CommandResponse = reqwest::Client::new()
        .post(url)
        .json(&command)
        .send()
        .await?
        .json()
        .await?;

    match command {
        Command::GetRoles => {
            let response = response.data.unwrap();
            let roles: Vec<Role> = serde_json::from_str(&response)?;
            println!("{roles:?}");
        }
        Command::GetUsers => {
            let response = response.data.unwrap();
            let users: Vec<User> = serde_json::from_str(&response)?;
            println!("{users:?}");
        }
        Command::GetUserWithRoles { id: _ } => {
            let response = response.data.unwrap();
            let (user, roles): (User, Vec<Role>) = serde_json::from_str(&response)?;
            println!("{user:?}");
            println!("{roles:?}");
        }
        _ => {
            println!("empty response");
        }
    };

    Ok(())
}
