use dotenv;

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();
    let grant_type = "password".to_string();
    let scope = "openid".to_string();
    let code = "token".to_string();
    let auth_path = "/auth/";

    let client_secret = dotenv::var("CLIENT_SECRET").unwrap();
    let username = dotenv::var("USERNAME").unwrap();
    let password = dotenv::var("PASSWORD").unwrap();
    let realm = dotenv::var("REALM").unwrap();
    let client_id = dotenv::var("CLIENT_ID").unwrap();
    let redirect_url = dotenv::var("REDIRECT_URL").unwrap();
    let provider_url = dotenv::var("PROVIDER_URL").unwrap();

    let token_request = rust_keycloak::serde_json::json!({
    "grant_type": grant_type,
    "username": username.to_string(),
    "password": password.to_string(),
    "realm": realm.to_string(),
    "client_id": client_id.to_string(),
    "redirect_uri": redirect_url.to_string(),
    "scope": scope.to_string(),
    "code": code.to_string(),
    "client_secret": client_secret.to_string()
    });

    let tok = rust_keycloak::keycloak::OpenId::token(&*(provider_url.to_owned() + &*auth_path.to_owned()).to_string(), token_request, &*realm.to_string()).await;
    println!("AUTH TOKEN {:?}", tok);
}