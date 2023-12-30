use crate::db::Role;
use crate::db::User;
use anyhow::Result;
use reqwest::Client;

#[derive(Debug)]
pub struct DbThinClient {
    client: Client,
    url: String,
}

impl DbThinClient {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn create_tables(&self) -> Result<()> {
        self.client
            .post(&format!("{}/create_tables", self.url))
            .send()
            .await?;
        Ok(())
    }

    pub async fn drop_tables(&self) -> Result<()> {
        let url = format!("{}/drop_tables", self.url);
        println!("url: {url}");

        self.client
            .delete(&format!("{}/drop_tables", self.url))
            .send()
            .await?;
        Ok(())
    }
    pub async fn create_user(&self, name: &str) -> Result<()> {
        self.client
            .post(&format!("{}/users?name={name}", self.url))
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<()> {
        self.client
            .delete(&format!("{}/users/{id}", self.url))
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_user(&self, id: i32, key: &str, value: &str) -> Result<()> {
        self.client
            .patch(&format!("{}/users/{id}?key={key}&value={value}", self.url))
            .send()
            .await?;
        Ok(())
    }

    pub async fn create_role(&self, slug: &str, name: &str, permissions: &str) -> Result<()> {
        self.client
            .post(&format!(
                "{}/roles?slug={slug}&name={name}&permissions={permissions}",
                self.url
            ))
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_role(&self, slug: &str) -> Result<()> {
        self.client
            .delete(&format!("{}/roles/{slug}", self.url))
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_role(&self, slug: &str, key: &str, value: &str) -> Result<()> {
        self.client
            .patch(&format!(
                "{}/roles/{slug}?key={key}&value={value}",
                self.url
            ))
            .send()
            .await?;
        Ok(())
    }

    pub async fn add_role_to_user(&self, user_id: i32, role_slug: &str) -> Result<()> {
        self.client
            .post(&format!(
                "{}/users/roles/{user_id}?slug={role_slug}",
                self.url
            ))
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_role_from_user(&self, user_id: i32, role_slug: &str) -> Result<()> {
        self.client
            .delete(&format!(
                "{}/users/roles/{user_id}?slug={role_slug}",
                self.url
            ))
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_roles(&self) -> Result<Vec<Role>> {
        let roles = self
            .client
            .get(&format!("{}/roles", self.url))
            .send()
            .await?
            .json()
            .await?;
        Ok(roles)
    }

    pub async fn get_role_by_slug(&self, slug: &str) -> Result<Role> {
        let role = self
            .client
            .get(&format!("{}/roles/{slug}", self.url))
            .send()
            .await?
            .json()
            .await?;
        Ok(role)
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        let users = self
            .client
            .get(&format!("{}/users", self.url))
            .send()
            .await?
            .json()
            .await?;
        Ok(users)
    }

    pub async fn get_user_with_roles(&self, id: i32) -> Result<(User, Vec<Role>)> {
        let res = self
            .client
            .get(&format!("{}/users/{id}", self.url))
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    async fn connect() -> Result<DbThinClient> {
        let db = DbThinClient {
            client: Client::new(),
            url: "http://127.0.0.1:3000".to_string(),
        };
        db.drop_tables().await?;
        db.create_tables().await?;

        Ok(db)
    }

    #[tokio::test]
    async fn test_user() -> Result<()> {
        let db = connect().await?;
        let user = create_user(&db, "user1").await?;

        let (user_with_roles, _) = db.get_user_with_roles(user.id).await?;
        assert_eq!(user, user_with_roles);

        db.update_user(user.id, "name", "new_name").await?;
        let (updated_user, _) = db.get_user_with_roles(user.id).await?;
        assert_eq!(&updated_user.name, "new_name");

        assert!(!db.get_users().await?.is_empty());
        db.delete_user(user.id).await?;
        assert!(db.get_users().await?.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_roles() -> Result<()> {
        let db = connect().await?;
        let role = create_role(&db, "slug", "role_name", "permission_name").await?;

        let role_by_slug = db.get_role_by_slug(&role.slug).await?;
        assert_eq!(role, role_by_slug);

        db.update_role(&role.slug, "name", "new_role_name").await?;
        db.update_role(&role.slug, "permissions", "new_permissions_name")
            .await?;

        let update_role = db.get_role_by_slug(&role.slug).await?;
        assert_eq!(&update_role.name, "new_role_name");
        assert_eq!(&update_role.permissions, "new_permissions_name");

        assert!(!db.get_roles().await?.is_empty());
        db.delete_role(&role.slug).await?;
        assert!(db.get_roles().await?.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_user_with_roles() -> Result<()> {
        let db = connect().await?;
        let user = create_user(&db, "user1").await?;
        let role = create_role(&db, "slug", "role_name", "permission_name").await?;

        let (_, roles) = db.get_user_with_roles(user.id).await?;
        assert!(roles.is_empty());

        db.add_role_to_user(user.id, &role.slug).await?;
        let (_, roles) = db.get_user_with_roles(user.id).await?;
        assert!(!roles.is_empty());

        let user_role = roles.get(0).map(Clone::clone).unwrap_or_default();
        assert_eq!(role, user_role);

        db.delete_role_from_user(user.id, &role.slug).await?;
        let (_, roles) = db.get_user_with_roles(user.id).await?;
        assert!(roles.is_empty());

        Ok(())
    }

    async fn create_user(db: &DbThinClient, name: &str) -> Result<User> {
        let users = db.get_users().await?;
        assert!(users.is_empty());

        db.create_user(name).await?;

        let users = db.get_users().await?;
        assert_eq!(users.len(), 1);

        let user = users.get(0).map(Clone::clone).unwrap_or_default();
        assert_eq!(&user.name, name);

        Ok(user)
    }

    async fn create_role(
        db: &DbThinClient,
        slug: &str,
        name: &str,
        permissions: &str,
    ) -> Result<Role> {
        let roles = db.get_roles().await?;
        assert!(roles.is_empty());

        db.create_role(slug, name, permissions).await?;

        let roles = db.get_roles().await?;
        assert_eq!(roles.len(), 1);

        let role = roles.get(0).map(Clone::clone).unwrap_or_default();
        assert_eq!(&role.slug, slug);
        assert_eq!(&role.name, name);
        assert_eq!(&role.permissions, permissions);

        Ok(role)
    }
}
