use cedarling::bindings::cedar_policy::Decision;
use cedarling::*;
use std::fs;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let policy_store_content =
        fs::read_to_string("policy_store.json").expect("Should have been able to read the file");

    let cedarling = Cedarling::new(BootstrapConfig {
        application_name: "test_app".to_string(),
        log_config: LogConfig {
            log_type: LogTypeConfig::StdOut,
        },
        policy_store_config: PolicyStoreConfig {
            source: PolicyStoreSource::Json(policy_store_content),
        },
        jwt_config: JwtConfig::Disabled,
    })?;

    let access_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJib0c4ZGZjNU1LVG4zN283Z3NkQ2V5cUw4THBXUXRnb080MW0xS1p3ZHEwIiwiY29kZSI6ImJmMTkzNGY2LTM5MDUtNDIwYS04Mjk5LTZiMmUzZmZkZGQ2ZSIsImlzcyI6Imh0dHBzOi8vYWRtaW4tdWktdGVzdC5nbHV1Lm9yZyIsInRva2VuX3R5cGUiOiJCZWFyZXIiLCJjbGllbnRfaWQiOiI1YjQ0ODdjNC04ZGIxLTQwOWQtYTY1My1mOTA3YjgwOTQwMzkiLCJhdWQiOiI1YjQ0ODdjNC04ZGIxLTQwOWQtYTY1My1mOTA3YjgwOTQwMzkiLCJhY3IiOiJiYXNpYyIsIng1dCNTMjU2IjoiIiwic2NvcGUiOlsib3BlbmlkIiwicHJvZmlsZSJdLCJvcmdfaWQiOiJzb21lX2xvbmdfaWQiLCJhdXRoX3RpbWUiOjE3MjQ4MzA3NDYsImV4cCI6MTcyNDk0NTk3OCwiaWF0IjoxNzI0ODMyMjU5LCJqdGkiOiJseFRtQ1ZSRlR4T2pKZ3ZFRXBvek1RIiwibmFtZSI6IkRlZmF1bHQgQWRtaW4gVXNlciIsInN0YXR1cyI6eyJzdGF0dXNfbGlzdCI6eyJpZHgiOjIwMSwidXJpIjoiaHR0cHM6Ly9hZG1pbi11aS10ZXN0LmdsdXUub3JnL2phbnMtYXV0aC9yZXN0djEvc3RhdHVzX2xpc3QifX19._eQT-DsfE_kgdhA0YOyFxxPEMNw44iwoelWa5iU1n9s".to_string();

    // Id_token payload
    // is used to create `id_token` and `User` entity
    // {
    //     "name": "some_name",
    //     "acr": "basic",
    //     "amr": "10",
    //     "aud": "5b4487c4-8db1-409d-a653-f907b8094039",
    //     "exp": 1724835859,
    //     "iat": 1724832259,
    //     "sub": "boG8dfc5MKTn37o7gsdCeyqL8LpWQtgoO41m1KZwdq0",
    //     "iss": "https://admin-ui-test.gluu.org",
    //     "jti": "sk3T40NYSYuk5saHZNpkZw",
    //     "nonce": "c3872af9-a0f5-4c3f-a1af-f9d0e8846e81",
    //     "sid": "6a7fe50a-d810-454d-be5d-549d29595a09",
    //     "jansOpenIDConnectVersion": "openidconnect-1.0",
    //     "c_hash": "pGoK6Y_RKcWHkUecM9uw6Q",
    //     "auth_time": 1724830746,
    //     "grant": "authorization_code",
    //     "status": {
    //       "status_list": {
    //         "idx": 202,
    //         "uri": "https://admin-ui-test.gluu.org/jans-auth/restv1/status_list"
    //       }
    //     }
    //   }
    let id_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1lIjoic29tZV9uYW1lIiwiYWNyIjoiYmFzaWMiLCJhbXIiOiIxMCIsImF1ZCI6IjViNDQ4N2M0LThkYjEtNDA5ZC1hNjUzLWY5MDdiODA5NDAzOSIsImV4cCI6MTcyNDgzNTg1OSwiaWF0IjoxNzI0ODMyMjU5LCJzdWIiOiJib0c4ZGZjNU1LVG4zN283Z3NkQ2V5cUw4THBXUXRnb080MW0xS1p3ZHEwIiwiaXNzIjoiaHR0cHM6Ly9hZG1pbi11aS10ZXN0LmdsdXUub3JnIiwianRpIjoic2szVDQwTllTWXVrNXNhSFpOcGtadyIsIm5vbmNlIjoiYzM4NzJhZjktYTBmNS00YzNmLWExYWYtZjlkMGU4ODQ2ZTgxIiwic2lkIjoiNmE3ZmU1MGEtZDgxMC00NTRkLWJlNWQtNTQ5ZDI5NTk1YTA5IiwiamFuc09wZW5JRENvbm5lY3RWZXJzaW9uIjoib3BlbmlkY29ubmVjdC0xLjAiLCJjX2hhc2giOiJwR29LNllfUktjV0hrVWVjTTl1dzZRIiwiYXV0aF90aW1lIjoxNzI0ODMwNzQ2LCJncmFudCI6ImF1dGhvcml6YXRpb25fY29kZSIsInN0YXR1cyI6eyJzdGF0dXNfbGlzdCI6eyJpZHgiOjIwMiwidXJpIjoiaHR0cHM6Ly9hZG1pbi11aS10ZXN0LmdsdXUub3JnL2phbnMtYXV0aC9yZXN0djEvc3RhdHVzX2xpc3QifX19.Lm03K8fCLaMxfl4v1NvlgJnodyNQkjG_-UD6tZTIBB4".to_string();

    // userinfo_token payload
    // is used to create `User` entity
    // {
    //     "country": "US",
    //     "email": "user@example.com",
    //     "username": "UserNameExample",
    //     "sub": "boG8dfc5MKTn37o7gsdCeyqL8LpWQtgoO41m1KZwdq0",
    //     "iss": "https://admin-ui-test.gluu.org",
    //     "given_name": "Admin",
    //     "middle_name": "Admin",
    //     "inum": "8d1cde6a-1447-4766-b3c8-16663e13b458",
    //     "client_id": "5b4487c4-8db1-409d-a653-f907b8094039",
    //     "aud": "5b4487c4-8db1-409d-a653-f907b8094039",
    //     "updated_at": 1724778591,
    //     "name": "Default Admin User",
    //     "nickname": "Admin",
    //     "family_name": "User",
    //     "jti": "faiYvaYIT0cDAT7Fow0pQw",
    //     "jansAdminUIRole": [
    //       "api-admin"
    //     ],
    //     "exp": 1724945978,
    //     "organization_id": "some_organization_id"
    //   }
    let userinfo_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJjb3VudHJ5IjoiVVMiLCJlbWFpbCI6InVzZXJAZXhhbXBsZS5jb20iLCJ1c2VybmFtZSI6IlVzZXJOYW1lRXhhbXBsZSIsInN1YiI6ImJvRzhkZmM1TUtUbjM3bzdnc2RDZXlxTDhMcFdRdGdvTzQxbTFLWndkcTAiLCJpc3MiOiJodHRwczovL2FkbWluLXVpLXRlc3QuZ2x1dS5vcmciLCJnaXZlbl9uYW1lIjoiQWRtaW4iLCJtaWRkbGVfbmFtZSI6IkFkbWluIiwiaW51bSI6IjhkMWNkZTZhLTE0NDctNDc2Ni1iM2M4LTE2NjYzZTEzYjQ1OCIsImNsaWVudF9pZCI6IjViNDQ4N2M0LThkYjEtNDA5ZC1hNjUzLWY5MDdiODA5NDAzOSIsImF1ZCI6IjViNDQ4N2M0LThkYjEtNDA5ZC1hNjUzLWY5MDdiODA5NDAzOSIsInVwZGF0ZWRfYXQiOjE3MjQ3Nzg1OTEsIm5hbWUiOiJEZWZhdWx0IEFkbWluIFVzZXIiLCJuaWNrbmFtZSI6IkFkbWluIiwiZmFtaWx5X25hbWUiOiJVc2VyIiwianRpIjoiZmFpWXZhWUlUMGNEQVQ3Rm93MHBRdyIsImphbnNBZG1pblVJUm9sZSI6WyJhcGktYWRtaW4iXSwiZXhwIjoxNzI0OTQ1OTc4LCJvcmdhbml6YXRpb25faWQiOiJzb21lX29yZ2FuaXphdGlvbl9pZCJ9.bF-uQemN42Dj0pYggjRNbydDz_cT-6SzfZJqILhFQuA".to_string();
    // to create `User` entity we mix payload of id_token and userinfo_token

    // usually ResourceData will be parse from json
    let resource_json = serde_json::json!({
            "id": "random_id",
            "type": "Jans::Ticket",
            "author_user_id": "author_user_id",
            "organization_id": "some_organization_id",
            "ticket_name":"ticket_name"
    });

    // but also we can parse `Request` from json
    // example: https://github.com/JanssenProject/jans/blob/main/jans-cedarling/cedarling/src/tests/mod.rs#L116
    let result = cedarling.authorize(Request {
        access_token,
        id_token,
        userinfo_token,
        action: "Jans::Action::\"Write\"".to_string(),
        context: serde_json::json!({}),
        resource: serde_json::from_value(resource_json)
            .map_err(|err| format!("cold not parse ResourceData:{err}"))?,
    });

    match result {
        Ok(result) => {
            println!("\n");
            println!(
                "result for person: {}",
                // method  result.is_allowed() contain bug.
                result.person.decision() == Decision::Allow
            );
            println!(
                "result for workload: {}",
                // method  result.is_allowed() contain bug.
                result.workload.decision() == Decision::Allow
            );

            println!(
                "result for request is allowed: {}",
                // method  result.is_allowed() contain bug.
                result.person.decision() == Decision::Allow
                    && result.workload.decision() == Decision::Allow
            );
        }
        Err(e) => eprintln!("Error while authorizing: {}\n {:?}\n\n", e.to_string(), e),
    }

    Ok(())
}
