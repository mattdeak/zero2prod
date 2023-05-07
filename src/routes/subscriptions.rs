use super::super::entity::subscriptions::ActiveModel as SubscriptionActiveModel;
use axum::extract::{Form, State};
use sea_orm::entity::ActiveModelTrait;

use axum::http::StatusCode;
use chrono::Utc;
use sea_orm::error::DbErr;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(db): State<Arc<DatabaseConnection>>,
    Form(subscription): Form<Subscription>,
) -> StatusCode {
    let subscription = SubscriptionActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(subscription.name),
        email: Set(subscription.email),
        subscribed_at: Set(Utc::now().into()),
    };

    match subscription.insert(&*db).await {
        Ok(_) => {
            println!("Subscription created");
            StatusCode::CREATED
        }
        Err(DbErr::Conn(err)) => {
            println!("Error connecting to database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Err(DbErr::Query(err)) => {
            println!("Error querying database: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
