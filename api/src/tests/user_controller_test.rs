use std::{collections::HashMap, sync::Arc};

use actix_http::{Request, StatusCode};
use actix_web::{dev::Service, test, web, App};

use ::actors::user_actor::player_factory::PlayerFactory;
use rstest::rstest;
use tokio::sync::Mutex;

use crate::{
    app_data::AppData,
    controllers::user_controller::{
        self, CreateUserParams, CreateUserResult, GetUserDataResult, UpdateUserParams,
        UserUpdateResult,
    }, user_provider::in_memory_user_provider::InMemoryUserProvider,
};

#[rstest]
#[actix_web::test]
async fn can_get_user() -> Result<(), String> {
    let user_name = "adrian";
    let email = "adrian.bercovici@gmail.com";

    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(HashMap::new())),
        user_resolver:Arc::new(InMemoryUserProvider::new())
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;
    let insert_result = insert_user(
        &mut app,
        CreateUserParams {
            user_name: user_name.to_string(),
            email: email.to_string(),
        },
    )
    .await?;
    let uri = format!("/user/get-user-state/{}", insert_result.user_id);
    let get_user_request: Request = test::TestRequest::get().uri(&uri).to_request();
    let result: GetUserDataResult = test::call_and_read_body_json(&app, get_user_request).await;
    Ok(())
}

#[rstest]
#[actix_web::test]
async fn can_insert_user() -> Result<(), String> {
    let user_name = "adrian";
    let email = "adrian.bercovici@gmail.com";

    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(HashMap::new())),
        user_resolver:Arc::new(InMemoryUserProvider::new())
    };
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/user/create")
        .set_json(CreateUserParams {
            user_name: user_name.to_string(),
            email: email.to_string(),
        })
        .to_request();

    let resp: CreateUserResult = test::call_and_read_body_json(&app, req).await;
    assert!(resp.user_name == user_name);
    assert!(resp.email == resp.email);
    Ok(())
}

#[actix_web::test]
async fn can_remove_user() -> Result<(), String> {
    let user_name = "adrian";
    let email = "adrian.bercovici@gmail.com";

    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(HashMap::new())),
        user_resolver:Arc::new(InMemoryUserProvider::new())
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;
    let create_request = CreateUserParams {
        user_name: user_name.to_string(),
        email: email.to_string(),
    };
    let rez = insert_user(&mut app, create_request).await?;

    let req = test::TestRequest::delete()
        .uri(&format!("/user/remove/{}", rez.user_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();

    assert!(matches!(status, StatusCode::NO_CONTENT));
    Ok(())
}

#[actix_web::test]
async fn can_update_user() -> Result<(), String> {
    let user_name = "adrian";
    let email = "adrian.bercovici@gmail.com";
    let new_user_name = "adrian2";
    let new_email = "adrian.bercovici2@yahoo.com";
    let app_data = AppData {
        player_factory: Arc::new(PlayerFactory {}),
        user_map: Arc::new(Mutex::new(HashMap::new())),
        user_resolver:Arc::new(InMemoryUserProvider::new())
    };
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_data))
            .service(web::scope("/user").configure(user_controller::init)),
    )
    .await;
    let create_request = CreateUserParams {
        user_name: user_name.to_string(),
        email: email.to_string(),
    };
    let rez = insert_user(&mut app, create_request).await?;
    let update_request = UpdateUserParams {
        email: new_email.to_string(),
        user_id: rez.user_id.clone(),
        user_name: new_user_name.to_string(),
    };
    let update_request = test::TestRequest::put()
        .uri("/user/update")
        .set_json(update_request)
        .to_request();

    let update_result: UserUpdateResult = test::call_and_read_body_json(&app, update_request).await;

    assert_eq!(update_result.user_id, rez.user_id);
    assert_eq!(update_result.new_email, new_email);
    assert_eq!(update_result.new_user_name, new_user_name);
    Ok(())
}

async fn insert_user(
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    user_params: CreateUserParams,
) -> Result<CreateUserResult, String> {
    let req = test::TestRequest::post()
        .uri("/user/create")
        .set_json(user_params)
        .to_request();
    let resp: CreateUserResult = test::call_and_read_body_json(&app, req).await;
    Ok(resp)
}

async fn remove_user(
    app: &mut impl Service<
        Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    user_id: &str,
) -> Result<(), String> {
    let req = test::TestRequest::delete()
        .uri(&format!("/user/remove/{}", user_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    match status {
        StatusCode::NO_CONTENT => Ok(()),
        _ => Err("Could not delete".to_string()),
    }
}
