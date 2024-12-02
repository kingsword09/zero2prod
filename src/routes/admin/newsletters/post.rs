use actix_web::{http::header::HeaderMap, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use base64::Engine;
use secrecy::SecretString;
use sqlx::PgPool;

use crate::{
    authentication::{Credentials, UserId},
    domain::SubscriberEmail,
    email_client::EmailClient,
    utils::{e500, see_other},
};

// use super::dashboard::get_username;
#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    text_content: String,
    html_content: String,
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(
    name = "Publish a newsletter issue",
    skip(form, pool, user_id, email_client),
    fields(user_id=%*user_id)
)]
pub async fn publish_newsletter(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, actix_web::Error> {
    let FormData {
        title,
        text_content,
        html_content,
    } = form.0;

    let subscribers = get_confirmed_subscribers(&pool).await.map_err(e500)?;

    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(&subscriber.email, &title, &html_content, &text_content)
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {:?}", subscriber.email)
                    })
                    .map_err(e500)?;
            }
            Err(error) => {
                tracing::warn!(error.cause_chain = ?error, error.message = %error, "Skipping a confirmed subscriber. Their stored contact details are invalid");
            }
        }
    }

    FlashMessage::info("The newsletter issue has been published!").send();

    Ok(see_other("/admin/newsletters"))
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| match SubscriberEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();

    Ok(confirmed_subscribers)
}
