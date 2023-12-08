#![allow(dead_code)]

use std::{cell::Cell, marker::PhantomData, rc::Rc, sync::MutexGuard};

struct OnlySync<'a> {
    _phantom: PhantomData<MutexGuard<'a, u64>>,
}

struct OnlySend {
    _phantom: PhantomData<Cell<u64>>,
}

struct SyncAndSend {
    _phantom: PhantomData<u64>,
}

struct NotSyncNotSend {
    _phantom: PhantomData<Rc<u64>>,
}

fn require_sync<T: Sync>(_: &T) {}

fn require_send<T: Send>(_: &T) {}

fn check_only_sync(v: OnlySync<'_>) {
    require_sync(&v);

    //NOTE: compile error
    // require_send(&v);
}

fn check_only_send(v: OnlySend) {
    //NOTE: compile error
    // require_sync(&v);

    require_send(&v);
}

fn check_sync_and_send(v: SyncAndSend) {
    require_send(&v);
    require_sync(&v);
}

fn check_not_sync_and_send(v: NotSyncNotSend) {
    //NOTE: compile error
    // require_send(&v);

    //NOTE: compile error
    // require_sync(&v);
}

fn main() {}
