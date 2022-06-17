#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct User {
    pub id: i32,
}

struct Authorization {
    pub id: i32,
    pub user: User,
    pub integration: Integration,
}

struct Integration {
    pub id: i32,
}

struct Group {
    pub id: i32,
    pub owner: User,
    pub integration: Integration,
}
