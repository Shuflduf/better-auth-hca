# better-auth-rs
[Hack Club Auth](https://auth.hackclub.com/) integration with the [better-auth-rs](https://crates.io/crates/better-auth) crate

# Example
```rs
let auth = BetterAuth::new(
        AuthConfig::new("your-very-secure-secret-key-at-least-32-chars-long")
            .base_url("http://localhost:3000"),
    )
    .database(MemoryDatabaseAdapter::new())
    .plugin(
      OAuthPlugin::new().add_provider(
        "hca",
        better_auth_hca::oauth(env::var("HCA_ID")?, env::var("HCA_SECRET")?),
      ),
    )
    .build()
    .await?;
```
