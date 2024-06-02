use crate::helpers::spawn_app;

#[tokio::test]
async fn login_returns_a_200_for_correct_credentials() {
    let test_app = spawn_app().await;

    let response = test_app.login_as_test_creditor().await;

    assert_eq!(200, response.status().as_u16());
}
