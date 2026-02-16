/// T1 integration tests: Automation CRUD.
///
/// Spec: /docs/spec/domain/automation.md
mod test_helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use kjxlkj_http::state::AppState;
use test_helpers::*;
use tower::ServiceExt;

#[tokio::test]
async fn automation_rule_create_and_list() {
    let state = AppState::new();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/automation/rules")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "workspace_id": uuid::Uuid::new_v4(),
                        "trigger": "on_create",
                        "condition_json": {"kind": "note"},
                        "action_json": {"type": "classify"},
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = json_body(resp).await;
    assert!(body["id"].as_str().is_some());
    // List rules
    let app2 = build_app_with_state(state.clone());
    let resp2 = app2
        .oneshot(Request::get("/api/automation/rules").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);
    let body2 = json_body(resp2).await;
    assert!(body2.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn automation_launch_and_get_run() {
    let state = AppState::new();
    let app = build_app_with_state(state.clone());
    let resp = app
        .oneshot(
            Request::post("/api/automation/rules")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "workspace_id": uuid::Uuid::new_v4(),
                        "trigger": "on_create",
                        "condition_json": {},
                        "action_json": {}
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = json_body(resp).await;
    let rule_id = body["id"].as_str().unwrap();
    // Launch → returns 202 ACCEPTED with run_id
    let app2 = build_app_with_state(state.clone());
    let resp2 = app2
        .oneshot(
            Request::post(&format!("/api/automation/rules/{}/launch", rule_id))
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::ACCEPTED);
    let body2 = json_body(resp2).await;
    let run_id = body2["run_id"].as_str().unwrap();
    // Get run
    let app3 = build_app_with_state(state.clone());
    let resp3 = app3
        .oneshot(
            Request::get(&format!("/api/automation/runs/{}", run_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp3.status(), StatusCode::OK);
    // List runs
    let app4 = build_app_with_state(state.clone());
    let resp4 = app4
        .oneshot(Request::get("/api/automation/runs").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp4.status(), StatusCode::OK);
    let body4 = json_body(resp4).await;
    assert!(body4.as_array().unwrap().len() >= 1);
}
