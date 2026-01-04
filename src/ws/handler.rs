use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use tokio::sync::broadcast;

use crate::models::event::AiEvent;
use crate::state::SharedState;

pub async fn ws_events(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse
{
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}

async fn handle_socket(
    state: SharedState,
    mut socket: WebSocket,
)

{
    let mut event_rx = state.event_tx.subscribe();
    let mut subscribed = false;
    let mut client_seq: u64 = 0;

    loop
    {
        tokio::select!
        {
            maybe_msg = socket.recv() =>
            {
                let msg = match maybe_msg
                {
                    Some(Ok(m)) => m,
                    _ => break,
                };

                if handle_client_message(&state, &msg, &mut subscribed, &mut client_seq).await == false
                {
                    break;
                }
            }

            event = recv_event(&mut event_rx), if subscribed =>
            {
                match event
                {
                    Some(ev) =>
                    {
                        let text = match serde_json::to_string(&ev)
                        {
                            Ok(s) => s,
                            Err(_) => continue,
                        };

                        if socket.send(Message::Text(text.into())).await.is_err()
                        {
                            break;
                        }
                    }
                    None =>
                    {
                        break;
                    }
                }
            }
        }
    }
}

async fn handle_client_message(
    state: &SharedState,
    msg: &Message,
    subscribed: &mut bool,
    client_seq: &mut u64,
) -> bool
{
    match msg
    {
        Message::Text(text) =>
        {
            let s: &str = text.as_str();
            if s == "ping"
            {
                return true;
            }

            if text.starts_with("{") == false
            {
                return true;
            }

            let v: serde_json::Value = match serde_json::from_str(text)
            {
                Ok(v) => v,
                Err(_) => return true,
            };

            let msg_type = v.get("type").and_then(|x| x.as_str()).unwrap_or("");

            if msg_type == "subscribe"
            {
                *subscribed = true;
                *client_seq += 1;

                let demo = AiEvent::demo(*client_seq);
                let _ = state.event_tx.send(demo);

                return true;
            }

            if msg_type == "ack"
            {
                return true;
            }

            true
        }
        Message::Close(_) =>
        {
            false
        }
        _ =>
        {
            true
        }
    }
}

async fn recv_event(
    rx: &mut broadcast::Receiver<AiEvent>
) -> Option<AiEvent>
{
    loop
    {
        match rx.recv().await
        {
            Ok(ev) => return Some(ev),
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(_) => return None,
        }
    }
}
