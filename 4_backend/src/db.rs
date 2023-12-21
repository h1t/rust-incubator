use anyhow::{bail, Result};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Sqlite, SqlitePool};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const MAX_TOKEN_LENGTH: usize = 10;

fn hash(str: &str) -> i64 {
    //TODO: add use of secure hasher
    let mut s: DefaultHasher = Default::default();
    str.hash(&mut s);

    //TODO: try to find better solution
    s.finish().try_into().unwrap_or_default()
}

fn generate_token() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(MAX_TOKEN_LENGTH)
        .map(char::from)
        .collect()
}

#[derive(Debug, Default, FromRow, Clone, Eq, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, FromRow, Clone, Eq, PartialEq)]
pub struct UserToken {
    pub user_id: i32,
    pub token: String,
}

#[derive(Debug)]
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
                name TEXT UNIQUE,
                password INTEGER
            );
            CREATE TABLE IF NOT EXISTS users_friends (
                user_id INTEGER REFERENCES users(id),
                friend_id INTEGER REFERENCES users(id),
                PRIMARY KEY (user_id, friend_id)
            );
            CREATE TABLE IF NOT EXISTS user_tokens (
                user_id INTEGER REFERENCES users(id),
                token TEXT,             
                PRIMARY KEY (user_id)
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

            DROP TABLE IF EXISTS users_friends;

            DROP TABLE IF EXISTS user_tokens;
            ",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_user(&self, name: &str, password: &str) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO users (name, password)
            VALUES (?, ?)
            ",
        )
        .bind(name)
        .bind(hash(password))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn add_friend(&self, user_id: i32, friend_id: i32) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO users_friends (user_id, friend_id)
            VALUES (?, ?)
            ",
        )
        .bind(user_id)
        .bind(friend_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn remove_friend(&self, user_id: i32, friend_id: i32) -> Result<()> {
        sqlx::query(
            r"
            DELETE FROM users_friends
            WHERE user_id = ? AND friend_id = ?
            ",
        )
        .bind(user_id)
        .bind(friend_id)
        .execute(&self.pool)
        .await?;

        Ok(())
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

    pub async fn get_user(&self, id: i32) -> Result<User> {
        sqlx::query_as(
            r"
            SELECT id, name
            FROM users
            WHERE id = ?
            ",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_user_by_name(&self, name: &str) -> Result<User> {
        sqlx::query_as(
            r"
            SELECT id, name
            FROM users
            WHERE name = ?
            ",
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_user_with_friends(&self, id: i32) -> Result<(User, Vec<User>)> {
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
            SELECT users.id, users.name
            FROM users
            JOIN users_friends ON users.id = users_friends.friend_id
            WHERE users_friends.user_id = ?
            ",
        )
        .bind(id)
        .fetch_all(&self.pool);

        futures::try_join!(user, roles).map_err(Into::into)
    }

    pub async fn register_user(&self, user_id: i32) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO user_tokens (user_id, token)
            VALUES (?, ?)
            ",
        )
        .bind(user_id)
        .bind(&generate_token())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_token(&self, user_id: i32) -> Result<Option<String>> {
        sqlx::query_as::<_, UserToken>(
            r"
            SELECT user_id, token
            FROM user_tokens
            WHERE user_id = ?
            ",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Into::into)
        .map(|info| info.map(|u| u.token))
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
        let user = create_user(&db, "user1", 0).await?;
        let (user_1, friends) = db.get_user_with_friends(user.id).await?;
        assert_eq!(user, user_1);
        assert!(friends.is_empty());

        let friend1 = create_user(&db, "friend1", 1).await?;
        db.add_friend(user.id, friend1.id).await?;
        let (user_1, friends) = db.get_user_with_friends(user.id).await?;
        assert_eq!(user, user_1);
        assert_eq!(friends.len(), 1);
        assert_eq!(friends[0], friend1);

        db.remove_friend(user.id, friend1.id).await?;
        let (user_1, friends) = db.get_user_with_friends(user.id).await?;
        assert_eq!(user, user_1);
        assert!(friends.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_user_registration() -> Result<()> {
        let db = connect().await?;
        let user = create_user(&db, "user1", 0).await?;
        let user_token = db.get_user_token(user.id).await?;
        assert!(user_token.is_none());

        db.register_user(user.id).await?;

        let user_token = db.get_user_token(user.id).await?;
        assert!(user_token.is_some());

        let user_token = user_token.unwrap();
        assert_eq!(user_token.len(), MAX_TOKEN_LENGTH);

        Ok(())
    }

    async fn create_user(db: &DataBase, name: &str, index: usize) -> Result<User> {
        let password = "password";
        let users = db.get_users().await?;
        assert_eq!(users.len(), index);

        db.create_user(name, password).await?;

        let users = db.get_users().await?;
        assert_eq!(users.len(), index + 1);

        let user = users
            .into_iter()
            .find(|u| u.name == name)
            .unwrap_or_default();
        assert_eq!(&user.name, name);

        let user1 = db.get_user(user.id).await?;
        assert_eq!(user, user1);

        let user2 = db.get_user_by_name(&user.name).await?;
        assert_eq!(user, user2);

        Ok(user)
    }
}
