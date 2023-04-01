pub(crate) trait ApplyTrait {
    fn apply<T>(&self, f: T) -> &Self
    where T: FnOnce(&Self) -> &Self, {
        f(self);
        self
    }
}

pub(crate) trait LetTrait {
    fn let_owned<R, F>(&self, block: F) -> R
    where F: FnOnce(&Self) -> R, {
        block(self)
    }

    fn let_ref<R, F>(self, block: F) -> R
    where Self: Sized, F: FnOnce(Self) -> R, {
        block(self)
    }
}