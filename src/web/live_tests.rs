use super::live::LiveHub;
use axum::extract::ws::Message;
use tokio::sync::mpsc;

#[tokio::test]
async fn viewer_count_starts_when_broadcaster_registers() {
    let hub = LiveHub::test().await;
    let (viewer_tx, mut viewer_rx) = mpsc::unbounded_channel();
    let (broadcaster_tx, mut broadcaster_rx) = mpsc::unbounded_channel();

    let viewer = hub.register_viewer(viewer_tx, None).await;
    hub.register_broadcaster(broadcaster_tx, None)
        .await
        .unwrap();

    assert!(viewer_rx.try_recv().is_err());
    assert!(text(&mut broadcaster_rx).await.contains(r#""count":1"#));
    hub.unregister(&viewer).await;
}

#[tokio::test]
async fn only_one_broadcaster_can_be_active() {
    let hub = LiveHub::test().await;
    let (first_tx, _first_rx) = mpsc::unbounded_channel();
    let (second_tx, _second_rx) = mpsc::unbounded_channel();

    hub.register_broadcaster(first_tx, None).await.unwrap();
    assert!(hub.register_broadcaster(second_tx, None).await.is_err());
}

#[tokio::test]
async fn broadcaster_disconnect_ends_stream_for_viewers() {
    let hub = LiveHub::test().await;
    let (viewer_tx, mut viewer_rx) = mpsc::unbounded_channel();
    let (broadcaster_tx, _broadcaster_rx) = mpsc::unbounded_channel();

    let viewer = hub.register_viewer(viewer_tx, None).await;
    let broadcaster = hub
        .register_broadcaster(broadcaster_tx, None)
        .await
        .unwrap();
    hub.unregister(&broadcaster).await;

    assert!(text(&mut viewer_rx).await.contains("stream_ended"));
    hub.unregister(&viewer).await;
}

#[tokio::test]
async fn broadcaster_receives_viewer_count_updates() {
    let hub = LiveHub::test().await;
    let (broadcaster_tx, mut broadcaster_rx) = mpsc::unbounded_channel();
    let (first_tx, _first_rx) = mpsc::unbounded_channel();
    let (second_tx, _second_rx) = mpsc::unbounded_channel();

    hub.register_broadcaster(broadcaster_tx, None)
        .await
        .unwrap();
    assert!(text(&mut broadcaster_rx).await.contains(r#""count":0"#));
    let first = hub.register_viewer(first_tx, None).await;
    assert!(text(&mut broadcaster_rx).await.contains(r#""count":1"#));
    let second = hub.register_viewer(second_tx, None).await;
    assert!(text(&mut broadcaster_rx).await.contains(r#""count":2"#));

    hub.unregister(&first).await;
    assert!(text(&mut broadcaster_rx).await.contains(r#""count":1"#));
    hub.unregister(&second).await;
    assert!(text(&mut broadcaster_rx).await.contains(r#""count":0"#));
}

async fn text(rx: &mut mpsc::UnboundedReceiver<Message>) -> String {
    match rx.recv().await.unwrap() {
        Message::Text(text) => text.to_string(),
        _ => String::new(),
    }
}
