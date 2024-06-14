use crate::authentication::{UserId, UserInfo, Username};
use actix_web::web;

#[tracing::instrument(name = "Getting user info by User ID")]
pub async fn get_user_info_by_id(
    user_id: web::ReqData<UserId>,
    username: web::ReqData<Username>,
) -> Result<web::Json<UserInfo>, actix_web::Error> {
    Ok(web::Json(UserInfo {
        user_id: *user_id.into_inner(),
        username: username.into_inner().to_string(),
    }))
}
