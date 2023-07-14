use leptos::*;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async_fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {

}
