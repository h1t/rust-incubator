pub trait EventSourced<Ev: ?Sized> {
    fn apply(&mut self, event: &Ev);
}

pub mod user {
    use std::time::SystemTime;

    use crate::event::UserCreated;

    use super::{event, EventSourced};

    #[derive(Debug)]
    pub struct User {
        pub id: Id,
        pub name: Option<Name>,
        pub online_since: Option<SystemTime>,
        pub created_at: CreationDateTime,
        pub last_activity_at: LastActivityDateTime,
        pub deleted_at: Option<DeletionDateTime>,
    }

    impl EventSourced<event::UserCreated> for User {
        fn apply(&mut self, ev: &event::UserCreated) {
            let event::UserCreated { user_id, at } = *ev;
            self.id = user_id;
            self.created_at = at;
            self.last_activity_at = LastActivityDateTime(at.0);
        }
    }

    impl EventSourced<event::UserNameUpdated> for User {
        fn apply(&mut self, ev: &event::UserNameUpdated) {
            let event::UserNameUpdated {
                user_id,
                ref name,
                at,
            } = *ev;
            self.id = user_id;
            self.name = name.clone();
            self.last_activity_at = LastActivityDateTime(at);
        }
    }

    impl EventSourced<event::UserBecameOnline> for User {
        fn apply(&mut self, ev: &event::UserBecameOnline) {
            let event::UserBecameOnline { user_id, at } = *ev;
            self.id = user_id;
            self.online_since = Some(at);
        }
    }

    impl EventSourced<event::UserBecameOffline> for User {
        fn apply(&mut self, ev: &event::UserBecameOffline) {
            let event::UserBecameOffline { user_id, at } = *ev;
            self.id = user_id;
            self.online_since = None;
            self.last_activity_at = LastActivityDateTime(at);
        }
    }

    impl EventSourced<event::UserDeleted> for User {
        fn apply(&mut self, ev: &event::UserDeleted) {
            let event::UserDeleted { user_id, at } = *ev;
            self.id = user_id;
            self.deleted_at = Some(at);
            self.last_activity_at = LastActivityDateTime(at.0);
        }
    }

    #[derive(Debug)]
    pub enum Event {
        Created(event::UserCreated),
        NameUpdated(event::UserNameUpdated),
        Online(event::UserBecameOnline),
        Offline(event::UserBecameOffline),
        Deleted(event::UserDeleted),
    }

    impl EventSourced<Event> for User {
        fn apply(&mut self, ev: &Event) {
            match ev {
                Event::Created(ev) => self.apply(ev),
                Event::Online(ev) => self.apply(ev),
                Event::Offline(ev) => self.apply(ev),
                Event::Deleted(ev) => self.apply(ev),
                Event::NameUpdated(_) => (),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug)]
    pub struct Name(pub Box<str>);

    #[derive(Clone, Copy, Debug)]
    pub struct CreationDateTime(pub SystemTime);

    #[derive(Clone, Copy, Debug)]
    pub struct LastActivityDateTime(pub SystemTime);

    #[derive(Clone, Copy, Debug)]
    pub struct DeletionDateTime(pub SystemTime);
}

pub mod event {
    use std::time::SystemTime;

    use super::user;

    #[derive(Debug)]
    pub struct UserCreated {
        pub user_id: user::Id,
        pub at: user::CreationDateTime,
    }

    #[derive(Debug)]
    pub struct UserNameUpdated {
        pub user_id: user::Id,
        pub name: Option<user::Name>,
        pub at: SystemTime,
    }

    #[derive(Debug)]
    pub struct UserBecameOnline {
        pub user_id: user::Id,
        pub at: SystemTime,
    }

    #[derive(Debug)]
    pub struct UserBecameOffline {
        pub user_id: user::Id,
        pub at: SystemTime,
    }

    #[derive(Debug)]
    pub struct UserDeleted {
        pub user_id: user::Id,
        pub at: user::DeletionDateTime,
    }
}
