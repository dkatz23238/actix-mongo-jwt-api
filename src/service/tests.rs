#[cfg(test)]
mod tests {
    use crate::database::get_user_collection;
    use crate::models::users::UpdateUser;
    use crate::service::users::UserService;
    use dotenv;

    #[actix_rt::test]
    async fn test_create_and_update_user() {
        dotenv::dotenv().ok();
        let user_collection = get_user_collection();
        let user_service = UserService::new(user_collection.clone());
        let res = user_service.create("david", "password", "david@gmail.com", "ACME.inc", "Admin");
        match res {
            Err(_) => panic!("Create user test failed"),
            _ => (),
        }

        let inserted_one = res.unwrap();
        let updates = UpdateUser {
            username: None,
            email: Some("test123@email.com".to_owned()),
        };
        let update_result = user_service.update(&inserted_one._id, updates);

        match update_result {
            Ok(r) => match r {
                Some(d) => assert_eq!(true, d.get("success").unwrap().as_bool().unwrap()),
                None => panic!("Failed updating user"),
            },
            Err(_) => panic!("Failed updating user"),
        };

        let delete_result = user_service.delete(&inserted_one._id);
        match delete_result {
            Ok(r) => match r {
                Some(d) => assert_eq!(true, d.get("success").unwrap().as_bool().unwrap()),
                None => panic!("Failed deleting user"),
            },
            Err(_) => panic!("Failed deleting user"),
        }
    }
}
