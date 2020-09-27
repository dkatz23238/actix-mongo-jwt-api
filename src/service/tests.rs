#[cfg(test)]
mod tests {
    use crate::database::get_user_collection;
    use crate::service::users::UserService;
    use dotenv;

    #[actix_rt::test]
    async fn test_create_user() {
        dotenv::dotenv().ok();
        let user_collection = get_user_collection();
        let user_service = UserService::new(user_collection.clone());
        let res = user_service.create("david", "password", "david@gmail.com", "ACME.inc", "Admin");
        match res {
            Err(_) => panic!("Create user test failed"),
            _ => (),
        }
    }
}
