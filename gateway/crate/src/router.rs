//! src/router.rs
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::Client;
use serde_json::Value;

use crate::policy::{dsl::RulePack, engine};

/// Embed delle regole YAML al compile-time
static POLICY_YAML: &str = include_str!("../policy/gdpr.yml");

/// Handler principale: applica le policy e, se tutto ok, inoltra la richiesta a OpenAI.
pub async fn proxy(Json(mut payload): Json<Value>) -> Response {
    // ─────────────────────────── 1️⃣  Valutazione delle policy ───────────────────────────
    let pack: RulePack = serde_yaml::from_str(POLICY_YAML).expect("valid yaml in gdpr.yml");

    if let Err(err) = engine::evaluate(&mut payload, &pack) {
        // Bloccato: restituisci 403 + messaggio
        return (StatusCode::FORBIDDEN, err).into_response();
    }

    // ─────────────────────────── 2️⃣  Proxy verso l'LLM upstream ─────────────────────────
    let api_key = std::env::var("OPENAI_API_KEY").expect("set OPENAI_API_KEY");
    let client  = Client::new();

    // Costruisci e invia la POST. Con `match` gestiamo l'eventuale errore di rete.
    let resp = match client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await
    {
        Ok(ok)  => ok,
        Err(e)  => return (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    };

    // Ritorna al chiamante lo stesso status code e corpo ricevuti da OpenAI
    let status = resp.status();
    let body   = resp.bytes().await.unwrap_or_default();
    (status, body).into_response()
}