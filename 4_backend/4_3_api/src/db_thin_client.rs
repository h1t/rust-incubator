use crate::{db::DataBase, Command};
use anyhow::Result;

pub struct DBThinClient {
    db: DataBase,
}

impl DBThinClient {
    pub fn new(db: DataBase) -> Self {
        Self { db }
    }

    pub async fn execute_command(&self, command: Command) -> Result<Option<String>> {
        match command {
            Command::CreateTables => self.db.create_tables().await.map(|()| None),
            Command::DropTables => self.db.drop_tables().await.map(|()| None),
            Command::CreateUser { name } => self.db.create_user(&name).await.map(|()| None),
            Command::UpdateUser { id, key, value } => {
                self.db.update_user(id, &key, &value).await.map(|()| None)
            }
            Command::AddRoleToUser { id, slug } => {
                self.db.add_role_to_user(id, &slug).await.map(|()| None)
            }
            Command::DeleteRoleFromUser { id, slug } => self
                .db
                .delete_role_from_user(id, &slug)
                .await
                .map(|()| None),
            Command::GetUsers => {
                let users = self.db.get_users().await?;
                let data = serde_json::to_string(&users)?;
                Ok(Some(data))
            }
            Command::GetUserWithRoles { id } => {
                let users = self.db.get_user_with_roles(id).await?;
                let data = serde_json::to_string(&users)?;
                Ok(Some(data))
            }
            Command::DeleteUser { id } => self.db.delete_user(id).await.map(|()| None),
            Command::CreateRole {
                slag,
                name,
                permissions,
            } => self
                .db
                .create_role(&slag, &name, &permissions)
                .await
                .map(|()| None),
            Command::UpdateRole { slug, key, value } => self
                .db
                .update_role(&slug, &key, &value)
                .await
                .map(|()| None),
            Command::GetRoles => {
                let roles = self.db.get_roles().await?;
                let data = serde_json::to_string(&roles)?;
                Ok(Some(data))
            }
            Command::DeleteRole { slug } => self.db.delete_role(&slug).await.map(|()| None),
        }
    }
}
