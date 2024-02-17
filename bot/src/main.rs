mod bot;
mod command;
mod metrics;
mod webhook;

use crate::bot::with_bot;
use crate::command::SubCommands;
use bot::SeleneBot;
use clap::Parser;
use selene_helius_sdk::api::types::enhanced::EnhancedTransaction;
use serde::Serialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::task;
use tracing::error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

fn init_tracing() -> color_eyre::Result<()> {
  color_eyre::install()?;
  let filter = std::env::var("RUST_LOG")
    .unwrap_or_else(|_| "tracing=info,selene_helius_bot=info,selene_helius_sdk=debug,warp=info".to_owned());
  let subscriber = tracing_subscriber::FmtSubscriber::builder()
    .with_env_filter(filter)
    .with_target(true)
    .with_span_events(FmtSpan::CLOSE)
    .finish();

  tracing::subscriber::set_global_default(subscriber)?;
  Ok(())
}

pub struct Empty;

async fn handle_hook(transactions: Vec<EnhancedTransaction>, bot: Arc<SeleneBot>) -> Result<impl Reply, Rejection> {
  task::spawn(async move {
    bot.handle_hook(transactions).await;
  });
  Ok(warp::reply::with_status("", StatusCode::ACCEPTED))
}

#[derive(Serialize, Default)]
struct HealthCheck {
  pub height: u64,
}

async fn handle_health(bot: Arc<SeleneBot>) -> Result<impl Reply, Rejection> {
  let result = bot.health().await;
  match result {
    Ok(height) => Ok(warp::reply::json(&HealthCheck { height })),
    Err(e) => {
      error!("failed {e}");
      Err(warp::reject::reject())
    },
  }
}

async fn serve(args: command::ServeArgs) -> color_eyre::Result<()> {
  let selene_bot: Arc<SeleneBot> =
    Arc::new(SeleneBot::new(args.selene_chat_id, &args.helius_api_key, args.teloxide_token)?);

  let webhook = warp::post()
    .and(warp::path::end())
    .and(warp::body::json())
    .and(with_bot(selene_bot.clone()))
    .and_then(handle_hook)
    .with(warp::trace::named("helius-hook"));

  let health =
    warp::path("health").and(with_bot(selene_bot.clone())).and_then(handle_health).with(warp::trace::named("health"));

  let metrics_route = warp::path("metrics").and_then(metrics::handler).with(warp::trace::named("health"));

  let goaway =
    warp::any().map(|| warp::reply::with_status("go away", StatusCode::FORBIDDEN)).with(warp::trace::named("goaway"));

  let routes = webhook.or(health).or(metrics_route).or(goaway).with(warp::trace::request());
  warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
  Ok(())
}

pub async fn handle_rejection(_: Rejection) -> std::result::Result<impl Reply, Infallible> {
  Ok(warp::reply::with_status("go away", StatusCode::FORBIDDEN))
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  init_tracing()?;
  let cmds = command::Cli::parse();
  match cmds.subcommands {
    SubCommands::Webhook(args) => {
      webhook::process_webhook(args).await?;
      Ok(())
    },
    SubCommands::Serve(args) => serve(args).await,
    SubCommands::Version { .. } => Ok(()),
  }
}
