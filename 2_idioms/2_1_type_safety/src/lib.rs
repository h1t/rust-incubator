use std::marker::PhantomData;

pub mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}

pub mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

pub struct New;

#[derive(Clone)]
pub struct Unmoderated;

pub struct Published;

pub struct Deleted;

#[derive(Clone)]
pub struct Post<S> {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
    _state: PhantomData<S>,
}

impl Post<New> {
    pub fn new(id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> Self {
        Self {
            id,
            user_id,
            title,
            body,
            _state: PhantomData,
        }
    }

    pub fn publish(self) -> Post<Unmoderated> {
        todo!()
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        todo!()
    }

    pub fn deny(self) -> Post<Deleted> {
        todo!()
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        todo!()
    }
}
