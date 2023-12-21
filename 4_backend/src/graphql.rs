use crate::db::DataBase;
use anyhow::Result;
use async_graphql::{Context, Object, SimpleObject, ID};

#[derive(Clone, SimpleObject)]
pub struct User {
    id: ID,
    name: String,
}

#[derive(Clone, SimpleObject)]
pub struct UserFriendsInfo {
    user: User,
    friends: Vec<ID>,
}

impl From<crate::db::User> for User {
    fn from(value: crate::db::User) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
        }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = ctx.data_unchecked::<DataBase>();
        let users = db.get_users().await?;
        let res = users.into_iter().map(Into::into).collect();

        Ok(res)
    }

    async fn user_with_friends(&self, ctx: &Context<'_>, user_id: ID) -> Result<UserFriendsInfo> {
        let db = ctx.data_unchecked::<DataBase>();
        let user_id = user_id.parse::<i32>()?;
        let (user, friends) = db.get_user_with_friends(user_id).await?;
        let friend_ids = friends.into_iter().map(|friend| friend.id.into()).collect();
        let res = UserFriendsInfo {
            user: user.into(),
            friends: friend_ids,
        };

        Ok(res)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, name: String, password: String) -> Result<ID> {
        let db = ctx.data_unchecked::<DataBase>();
        db.create_user(&name, &password).await?;
        db.get_user_by_name(&name).await.map(|user| user.id.into())
    }

    async fn add_friend(
        &self,
        ctx: &Context<'_>,
        token: String,
        user_id: ID,
        friend_id: ID,
    ) -> Result<bool> {
        let db = ctx.data_unchecked::<DataBase>();
        let user_id = user_id.parse::<i32>()?;
        let friend_id = friend_id.parse::<i32>()?;
        let user_token = db.get_user_token(user_id).await?;

        if user_token.is_some_and(|user_token| user_token == token) {
            db.add_friend(user_id, friend_id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn remove_friend(
        &self,
        ctx: &Context<'_>,
        token: String,
        user_id: ID,
        friend_id: ID,
    ) -> Result<bool> {
        let db = ctx.data_unchecked::<DataBase>();
        let user_id = user_id.parse::<i32>()?;
        let friend_id = friend_id.parse::<i32>()?;
        let user_token = db.get_user_token(user_id).await?;

        if user_token.is_some_and(|user_token| user_token == token) {
            db.remove_friend(user_id, friend_id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn register_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<Option<String>> {
        let db = ctx.data_unchecked::<DataBase>();
        let user_id = user_id.parse::<i32>()?;

        db.register_user(user_id).await?;
        db.get_user_token(user_id).await
    }
}
