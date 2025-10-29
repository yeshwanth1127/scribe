use axum::{
    extract::{Json, State},
    response::{sse::Event, IntoResponse},
};
use futures::StreamExt;
use serde_json::json;

use crate::models::ChatRequest;
use crate::services::AppState;

pub async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> impl IntoResponse {
    let stream = match state
        .openrouter_service
        .chat(
            &request.user_message,
            request.system_prompt.as_deref(),
            request.image_base64.as_ref(),
            request.history.as_deref(),
        )
        .await
    {
        Ok(s) => s,
        Err(e) => {
            return Json(json!({"error": format!("Failed to start chat: {}", e)})).into_response();
        }
    };

    // Repackage OpenRouter stream into SSE `data: {json}` lines the client expects
    let event_stream = async_stream::stream! {
        futures::pin_mut!(stream);
        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    // Chunk may contain multiple lines; unwrap only `data: ` JSON lines
                    for line in chunk.split('\n') {
                        let trimmed = line.trim();
                        if trimmed.is_empty() { continue; }

                        // Pass through only the JSON part of SSE lines
                        if let Some(json_str) = trimmed.strip_prefix("data: ") {
                            if json_str == "[DONE]" { continue; }
                            yield Ok::<Event, std::convert::Infallible>(Event::default().data(json_str.to_string()));
                        }
                    }
                }
                Err(e) => {
                    yield Ok::<Event, std::convert::Infallible>(Event::default().data(format!("{{\"error\": \"{}\"}}", e)));
                    break;
                }
            }
        }
    };

    axum::response::Sse::new(event_stream).into_response()
}
