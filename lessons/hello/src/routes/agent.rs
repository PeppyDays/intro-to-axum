use axum::{headers::UserAgent, TypedHeader};

pub async fn agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}
