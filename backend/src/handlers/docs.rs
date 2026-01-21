use axum::{extract::State, response::Html};

use crate::handlers::SharedState;

pub async fn docs(State(state): State<SharedState>) -> Html<String> {
    Html(state.docs_html.clone())
}
