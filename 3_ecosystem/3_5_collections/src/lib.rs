use im::{OrdMap, OrdSet};

type UserId = u64;
type Map<K, V> = OrdMap<K, V>;
type Set<K> = OrdSet<K>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    nickname: Box<str>,
}

impl User {
    fn new(nickname: impl AsRef<str>) -> Self {
        Self {
            nickname: nickname.as_ref().into(),
        }
    }
}

trait UserRepository {
    fn get_user<K: Into<UserId>>(&self, id: K) -> Option<User>;

    fn get_users<I>(&self, ids: I) -> Map<UserId, User>
    where
        I: IntoIterator,
        I::Item: Into<UserId>;

    fn get_ids_by_nickname(&self, nickname: impl AsRef<str>) -> Set<UserId>;
}

impl UserRepository for Map<UserId, User> {
    fn get_user<K: Into<UserId>>(&self, id: K) -> Option<User> {
        self.get(&id.into()).cloned()
    }

    fn get_users<I>(&self, ids: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<UserId>,
    {
        ids.into_iter()
            .map(Into::into)
            .filter_map(|id| Self::get_user(self, id).map(|user| (id, user)))
            .collect()
    }

    fn get_ids_by_nickname(&self, nickname: impl AsRef<str>) -> Set<UserId> {
        let nickname = nickname.as_ref();
        self.iter()
            .filter_map(|(&id, user)| user.nickname.contains(nickname).then_some(id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use im::{ordmap, ordset};

    const USER_1_ID: u64 = 1_u64;
    const USER_2_ID: u64 = 2_u64;
    const USER_3_ID: u64 = 3_u64;

    fn get_etalon() -> Map<UserId, User> {
        ordmap! {
          USER_1_ID => get_user1(),
          USER_2_ID => get_user2(),
          USER_3_ID => get_user3()
        }
    }

    fn get_user1() -> User {
        User::new("one")
    }

    fn get_user2() -> User {
        User::new("two")
    }

    fn get_user3() -> User {
        User::new("three")
    }

    #[test]
    fn test_get_user() {
        let map = get_etalon();

        assert!(map.get_user(0_u64).is_none());

        assert_eq!(map.get_user(USER_1_ID), Some(get_user1()));
        assert_eq!(map.get_user(USER_2_ID), Some(get_user2()));
        assert_eq!(map.get_user(USER_3_ID), Some(get_user3()));
    }

    #[test]
    fn test_get_users() {
        let map = get_etalon();
        let etalon_search_map = ordmap! {
            USER_1_ID => get_user1(),
            USER_2_ID => get_user2()
        };

        assert_eq!(map.get_users([USER_1_ID, USER_2_ID]), etalon_search_map);
    }

    #[test]
    fn test_get_ids_by_nickname() {
        let map = get_etalon();
        let etalon_search_set = ordset! {
            USER_2_ID,
            USER_3_ID
        };

        assert_eq!(map.get_ids_by_nickname("t"), etalon_search_set);
    }
}
