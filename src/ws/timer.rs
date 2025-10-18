use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use uuid::Uuid;

use crate::controllers::timer::AppState;
use crate::services::timer::{TimerService, to_response};

/// WebSocket endpoint for timer updates
pub async fn timer_ws_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_timer_socket(socket, id, state))
}

async fn handle_timer_socket(mut socket: WebSocket, timer_id: Uuid, state: Arc<AppState>) {
    let mut interval = interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        // Get the current timer state
        let timer = match TimerService::get_by_id(&state.db, timer_id).await {
            Ok(Some(timer)) => timer,
            Ok(None) => {
                let _ = socket.send(Message::Text("Timer not found".to_string().into())).await;
                break;
            }
            Err(e) => {
                tracing::error!("Error fetching timer: {:?}", e);
                let _ = socket.send(Message::Text("Error fetching timer".to_string().into())).await;
                break;
            }
        };

        // Convert to response and send
        let response = to_response(timer.clone());
        let json = match serde_json::to_string(&response) {
            Ok(json) => json,
            Err(e) => {
                tracing::error!("Error serializing timer: {:?}", e);
                break;
            }
        };

        if socket.send(Message::Text(json.into())).await.is_err() {
            // Client disconnected
            break;
        }

        // Stop sending updates if timer is completed or cancelled
        use crate::models::timer::TimerStatus;
        match timer.status {
            TimerStatus::Completed | TimerStatus::Cancelled => {
                break;
            }
            _ => {}
        }
    }

    tracing::debug!("WebSocket connection closed for timer {}", timer_id);
}

