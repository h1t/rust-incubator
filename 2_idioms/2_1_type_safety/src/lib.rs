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
    data: PostData,
    _state: PhantomData<S>,
}

#[derive(Clone)]
struct PostData {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

impl Post<New> {
    pub fn new(id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> Self {
        Self {
            data: PostData {
                id,
                user_id,
                title,
                body,
            },
            _state: PhantomData,
        }
    }

    pub fn publish(self) -> Post<Unmoderated> {
        Post {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl Post<Unmoderated> {
    pub fn allow(self) -> Post<Published> {
        Post {
            data: self.data,
            _state: PhantomData,
        }
    }

    pub fn deny(self) -> Post<Deleted> {
        Post {
            data: self.data,
            _state: PhantomData,
        }
    }
}

impl Post<Published> {
    pub fn delete(self) -> Post<Deleted> {
        Post {
            data: self.data,
            _state: PhantomData,
        }
    }
}
