use super::live::LiveHub;
use axum::extract::ws::Message;
use tokio::sync::mpsc;

#[tokio::test]
async fn broadcaster_notifies_waiting_viewers() {
    let hub = LiveHub::default();
    let (viewer_tx, mut viewer_rx) = mpsc::unbounded_channel();
    let (broadcaster_tx, mut broadcaster_rx) = mpsc::unbounded_channel();

    let viewer = hub.register_viewer(viewer_tx).await;
    hub.register_broadcaster(broadcaster_tx).await.unwrap();

    assert!(text(&mut viewer_rx).await.contains("stream_started"));
    assert!(text(&mut broadcaster_rx).await.contains("viewer_ready"));
    hub.unregister(&viewer).await;
}

#[tokio::test]
async fn only_one_broadcaster_can_be_active() {
    let hub = LiveHub::default();
    let (first_tx, _first_rx) = mpsc::unbounded_channel();
    let (second_tx, _second_rx) = mpsc::unbounded_channel();

    hub.register_broadcaster(first_tx).await.unwrap();
    assert!(hub.register_broadcaster(second_tx).await.is_err());
}

#[tokio::test]
async fn broadcaster_disconnect_ends_stream_for_viewers() {
    let hub = LiveHub::default();
    let (viewer_tx, mut viewer_rx) = mpsc::unbounded_channel();
    let (broadcaster_tx, _broadcaster_rx) = mpsc::unbounded_channel();

    hub.register_viewer(viewer_tx).await;
    let broadcaster = hub.register_broadcaster(broadcaster_tx).await.unwrap();
    let _ = text(&mut viewer_rx).await;
    hub.unregister(&broadcaster).await;

    assert!(text(&mut viewer_rx).await.contains("stream_ended"));
}

async fn text(rx: &mut mpsc::UnboundedReceiver<Message>) -> String {
    match rx.recv().await.unwrap() {
        Message::Text(text) => text.to_string(),
        _ => String::new(),
    }
}
