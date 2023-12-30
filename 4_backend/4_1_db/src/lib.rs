use anyhow::{bail, Result};
use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Sqlite, SqlitePool};

#[derive(Debug, Default, FromRow, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, FromRow)]
pub struct Role {
    pub slug: String,
    pub name: String,
    pub permissions: String,
}

pub struct DataBase {
    pool: Pool<Sqlite>,
}

impl DataBase {
    pub async fn connect(url: &str) -> Result<Self> {
        let db_exists = Sqlite::database_exists(url).await;

        if !db_exists.unwrap_or(false) {
            if let Err(err) = Sqlite::create_database(url).await {
                bail!("There is error during db creation: {err:?}");
            }
        }

        SqlitePool::connect(url)
            .await
            .map(|pool| Self { pool })
            .map_err(Into::into)
    }

    pub async fn create_tables(&self) -> Result<()> {
        sqlx::query(
            r"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT
            );
            CREATE TABLE IF NOT EXISTS roles (
                slug TEXT PRIMARY KEY,
                name TEXT,
                permissions TEXT
            );
            CREATE TABLE IF NOT EXISTS users_roles (
                user_id INTEGER REFERENCES users(id),
                role_slug TEXT REFERENCES roles(slug),
                PRIMARY KEY (user_id, role_slug)
            );
            ",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn drop_tables(&self) -> Result<()> {
        sqlx::query(
            r"
            DROP TABLE IF EXISTS users;

            DROP TABLE IF EXISTS roles;

            DROP TABLE IF EXISTS users_roles;
            ",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn create_user(&self, name: &str) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO users (name)
            VALUES (?)
            ",
        )
        .bind(name)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_user(&self, id: i32) -> Result<()> {
        sqlx::query(
            r"
            DELETE FROM users
            WHERE id = ?
            ",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user(&self, id: i32, key: &str, value: &str) -> Result<()> {
        let query = format!("UPDATE users SET {key} = ? WHERE id = ?");

        sqlx::query(&query)
            .bind(value)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_role(&self, slug: &str, name: &str, permissions: &str) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO roles (slug, name, permissions)
            VALUES (?, ?, ?)
            ",
        )
        .bind(slug)
        .bind(name)
        .bind(permissions)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_role(&self, slug: &str) -> Result<()> {
        sqlx::query(
            r"
            DELETE FROM roles
            WHERE slug = ?
            ",
        )
        .bind(slug)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_role(&self, slug: &str, key: &str, value: &str) -> Result<()> {
        let query = format!("UPDATE roles SET {key} = ? WHERE slug = ?");

        sqlx::query(&query)
            .bind(value)
            .bind(slug)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_role_to_user(&self, user_id: i32, role_slug: &str) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO users_roles (user_id, role_slug)
            VALUES (?, ?)
            ",
        )
        .bind(user_id)
        .bind(role_slug)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_role_from_user(&self, user_id: i32, role_slug: &str) -> Result<()> {
        sqlx::query(
            r"
            DELETE FROM users_roles
            WHERE user_id = ? AND role_slug = ?
            ",
        )
        .bind(user_id)
        .bind(role_slug)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_roles(&self) -> Result<Vec<Role>> {
        sqlx::query_as(
            r"
            SELECT slug, name, permissions
            FROM roles
            ",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_role_by_slug(&self, slug: &str) -> Result<Role> {
        sqlx::query_as(
            r"
            SELECT slug, name, permissions
            FROM roles
            WHERE slug = ?
            ",
        )
        .bind(slug)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_users(&self) -> Result<Vec<User>> {
        sqlx::query_as(
            r"
            SELECT id, name
            FROM users
            ",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_user_with_roles(&self, id: i32) -> Result<(User, Vec<Role>)> {
        let user = sqlx::query_as(
            r"
            SELECT id, name
            FROM users
            WHERE id = ?
            ",
        )
        .bind(id)
        .fetch_one(&self.pool);

        let roles = sqlx::query_as(
            r"
            SELECT roles.slug, roles.name, roles.permissions
            FROM roles
            JOIN users_roles ON roles.slug = users_roles.role_slug
            WHERE users_roles.user_id = ?
            ",
        )
        .bind(id)
        .fetch_all(&self.pool);

        futures::try_join!(user, roles).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    async fn connect() -> Result<DataBase> {
        let db = DataBase::connect("sqlite://:memory:").await?;
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

    async fn create_user(db: &DataBase, name: &str) -> Result<User> {
        let users = db.get_users().await?;
        assert!(users.is_empty());

        db.create_user(name).await?;

        let users = db.get_users().await?;
        assert_eq!(users.len(), 1);

        let user = users.get(0).map(Clone::clone).unwrap_or_default();
        assert_eq!(&user.name, name);

        Ok(user)
    }

    async fn create_role(db: &DataBase, slug: &str, name: &str, permissions: &str) -> Result<Role> {
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
