use clap::{Args, Parser, Subcommand};

/// Telegram bot for helius webhooks
#[derive(Debug, Subcommand)]
pub enum SubCommands {
  /// Show git version
  #[command()]
  Version {},

  /// List, Create and Delete webhooks
  Webhook(WebhookArgs),

  /// A telegram bot for helius webhooks, listens on port 3030
  Serve(ServeArgs),
}

#[derive(Debug, Parser)]
pub struct Cli {
  #[clap(subcommand)]
  pub subcommands: SubCommands,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
#[command(visible_aliases = ["w"])]
pub struct WebhookArgs {
  #[arg(long, env)]
  pub helius_api_key: String,

  #[command(subcommand)]
  pub command: Option<WebhookCommands>,
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
#[command(visible_aliases = ["s"])]
pub struct ServeArgs {
  #[arg(long, env)]
  pub helius_api_key: String,

  /// This is your telegram channel id
  #[arg(long, env)]
  pub selene_chat_id: i64,

  #[arg(long, env)]
  /// Your bot's token
  pub teloxide_token: String,

  #[arg(long, default_value = "3030")]
  pub port: i32,
}

#[derive(Debug, Subcommand)]
pub enum WebhookCommands {
  List,
  Create(CreateArgs),
  Delete(DeleteArgs),
  Add(AddArgs),
}

#[derive(Debug, Args, Default)]
pub struct CreateArgs {
  #[arg(long)]
  pub url: String,

  #[arg(long, default_value_t = false)]
  pub transfer_only: bool,

  #[arg(long, default_value_t = false)]
  pub devnet: bool,

  pub addresses: Vec<String>,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
  #[arg(long)]
  pub id: String,
}

#[derive(Debug, Args)]
pub struct AddArgs {
  #[arg(long)]
  pub id: String,
  pub addresses: Vec<String>,
}
