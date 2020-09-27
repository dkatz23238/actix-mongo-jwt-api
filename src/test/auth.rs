mod tests {
    use crate::auth::get_identity_service;
    use crate::cache::add_cache;
    use crate::config::CONFIG;
    use crate::database::{add_pool, init_pool, Pool};
    use crate::handlers::auth::LoginRequest;
    use crate::routes::routes;
    use crate::state::{new_state, AppState};
    use actix_web::dev::ServiceResponse;
    use actix_web::{test, web::Data, App};
    use diesel::mysql::MysqlConnection;
    use serde::Serialize;

    /// Assert that a route is successful for HTTP GET requests
    pub async fn assert_get(route: &str) -> ServiceResponse {
        let response = test_get(route).await;
        assert!(response.status().is_success());
        response
    }

    /// Assert that a route is successful for HTTP POST requests
    pub async fn assert_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let response = test_post(route, params).await;
        assert!(response.status().is_success());
        response
    }

    const PATH: &str = "/";

    #[actix_rt::test]
    async fn it_logs_a_user_in() {
        let params = LoginRequest {
            email: "satoshi@nakamotoinstitute.org".into(),
            password: "123456".into(),
        };
        let url = format!("{}", PATH);
        assert_post(&url, params).await;
    }
}
