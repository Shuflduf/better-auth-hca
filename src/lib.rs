use better_auth::{
    self,
    plugins::oauth::{OAuthProvider, OAuthUserInfo},
};

pub fn oauth(client_id: impl Into<String>, client_secret: impl Into<String>) -> OAuthProvider {
    OAuthProvider {
        client_id: client_id.into(),
        client_secret: client_secret.into(),
        auth_url: "https://auth.hackclub.com/oauth/authorize".into(),
        token_url: "https://auth.hackclub.com/oauth/token".into(),
        user_info_url: "https://auth.hackclub.com/api/v1/me".into(),
        scopes: vec!["openid", "profile", "email", "name"]
            .into_iter()
            .map(String::from)
            .collect(),
        map_user_info: |v| {
            let identity = &v["identity"];
            println!("{identity:?}");
            Ok(OAuthUserInfo {
                id: identity["id"].as_str().ok_or("Missing ID")?.to_string(),
                email: identity["primary_email"]
                    .as_str()
                    .ok_or("Missing primary email")?
                    .to_string(),
                name: identity["first_name"].as_str().map(|n| n.to_string()),
                image: None,
                email_verified: true,
            })
        },
    }
}
