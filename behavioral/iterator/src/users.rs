pub struct UserCollection {
    users: [&'static str; 3],
}

/// A custom collection contains an arbitrary user array under the hood.
impl UserCollection {
    /// Returns a custom user collection
    pub fn new() -> Self {
        Self {
            users: ["Alice", "Bob", "Cart"],
        }
    }

    /// Returns an iterator over a user collection
    ///
    /// The method name may be different, however, `iter` is used as a de facto
    /// standard in a Rust naming convention
    pub fn iter(&self) -> UserIterator {
        UserIterator {
            index: 0,
            user_collection: self,
        }
    }
}

/// UserIterator allows sequential traversal through a complex user collection
/// without exposing its internal details.
pub struct UserIterator<'a> {
    index: usize,
    user_collection: &'a UserCollection,
}

impl Iterator for UserIterator<'_> {
    type Item = &'static str;

    /// `Iterator` is a standard interface for dealing with iterators
    /// form the Rust standard library.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = Some(self.user_collection.users[self.index]);
            self.index += 1;
            return user;
        }
        None
    }
}