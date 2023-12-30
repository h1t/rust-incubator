use clap::Subcommand;
use serde::{Deserialize, Serialize};

pub mod db;
pub mod db_thin_client;
pub mod web;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    pub data: Option<String>,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Command {
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
