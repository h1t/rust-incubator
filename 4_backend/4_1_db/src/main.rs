use anyhow::Result;
use clap::{Parser, Subcommand};
use step_4_1::DataBase;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Path to configuration file
    #[clap(short, long, env = "DB_URL", default_value = "db.sqlite3")]
    url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// create tables
    CreateTables,

    /// drop tables
    DropTables,

    /// Create user
    CreateUser {
        /// user name
        #[arg(short, long)]
        name: String,
    },

    /// Update user
    UpdateUser {
        /// id
        #[arg(short, long)]
        id: i32,

        /// property name
        #[arg(short, long)]
        key: String,

        /// property value
        #[arg(short, long)]
        value: String,
    },

    /// Add role to user
    AddRoleToUser {
        /// id
        #[arg(short, long)]
        id: i32,

        /// slug
        #[arg(short, long)]
        slug: String,
    },

    /// Delete role from user
    DeleteRoleFromUser {
        /// id
        #[arg(short, long)]
        id: i32,

        /// slug
        #[arg(short, long)]
        slug: String,
    },

    /// Get all users
    GetUsers,

    /// Get user with roles
    GetUserWithRoles {
        /// id
        #[arg(short, long)]
        id: i32,
    },

    /// Delete user
    DeleteUser {
        /// user name
        #[arg(short, long)]
        id: i32,
    },

    /// Create role
    CreateRole {
        /// slag
        #[arg(short, long)]
        slag: String,

        /// role name
        #[arg(short, long)]
        name: String,

        /// persmissions
        #[arg(short, long)]
        permissions: String,
    },

    /// Update role
    UpdateRole {
        /// slug
        #[arg(short, long)]
        slug: String,

        /// property name
        #[arg(short, long)]
        key: String,

        /// property value
        #[arg(short, long)]
        value: String,
    },

    /// Get all roles
    GetRoles,

    /// Delete role
    DeleteRole {
        /// slug
        #[arg(short, long)]
        slug: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args { command, url } = Args::parse();
    let db = DataBase::connect(&format!("sqlite://{url}")).await?;

    match command {
        Commands::CreateTables => db.create_tables().await,
        Commands::DropTables => db.drop_tables().await,
        Commands::CreateUser { name } => db.create_user(&name).await,
        Commands::UpdateUser { id, key, value } => db.update_user(id, &key, &value).await,
        Commands::AddRoleToUser { id, slug } => db.add_role_to_user(id, &slug).await,
        Commands::DeleteRoleFromUser { id, slug } => db.delete_role_from_user(id, &slug).await,
        Commands::GetUsers => {
            let users = db.get_users().await?;
            println!("{users:?}");
            Ok(())
        }
        Commands::GetUserWithRoles { id } => {
            let (user, roles) = db.get_user_with_roles(id).await?;
            println!("{user:?}");
            println!("{roles:?}");
            Ok(())
        }
        Commands::DeleteUser { id } => db.delete_user(id).await,
        Commands::CreateRole {
            slag,
            name,
            permissions,
        } => db.create_role(&slag, &name, &permissions).await,
        Commands::UpdateRole { slug, key, value } => db.update_role(&slug, &key, &value).await,
        Commands::GetRoles => {
            let roles = db.get_roles().await?;
            println!("{roles:?}");
            Ok(())
        }
        Commands::DeleteRole { slug } => db.delete_role(&slug).await,
    }
}
