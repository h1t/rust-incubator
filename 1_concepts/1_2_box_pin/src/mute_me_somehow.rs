use std::pin::Pin;

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {}
}

impl<T> MutMeSomehow for T {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = unsafe { Pin::get_unchecked_mut(self) };
        let _check = unsafe { Pin::new_unchecked(this) };
    }
}
