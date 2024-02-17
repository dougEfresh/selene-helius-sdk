use dashmap::DashMap;
use selene_helius_sdk::api::types::enhanced::EnhancedTransaction;
use selene_helius_sdk::{Helius, HeliusBuilder};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::Recipient;
use teloxide::Bot;
use tracing::{debug, error, info};
use warp::Filter;

pub struct SeleneBot {
  id: ChatId,
  bot: Bot,
  helius: Helius,
  name_cache: DashMap<String, AccountName>,
}

struct BotMessage(EnhancedTransaction);

struct BotMessages(Vec<BotMessage>);

#[derive(Clone)]
struct AccountName {
  pub address: String,
  pub name: String,
}

impl AccountName {
  pub fn new(address: String) -> Self {
    Self { address: address.clone(), name: address }
  }
}

impl Display for AccountName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.address != self.name {
      write!(f, "{}={}", self.address, self.name)?;
    }
    Ok(())
  }
}

struct AccountNames(Vec<AccountName>);

impl Display for AccountNames {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let msg: Vec<String> = self.0.iter().map(|m| format!("{m}")).collect();
    writeln!(f, "{}", msg.join("\n"))
  }
}

impl Display for BotMessage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}\nhttps://xray.helius.xyz/tx/{}", self.0.description, self.0.signature)
  }
}

impl Display for BotMessages {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let msg: Vec<String> = self.0.iter().map(|m| format!("{m}")).collect();
    writeln!(f, "{}", msg.join("\n"))
  }
}

impl SeleneBot {
  pub fn new(id: i64, api_key: &str, token: String) -> selene_helius_sdk::Result<Self> {
    let id = ChatId(id);
    let bot = Bot::new(token);
    let helius = HeliusBuilder::new(api_key).build()?;
    let name_cache: DashMap<String, AccountName> = DashMap::new();
    Ok(Self { id, bot, helius, name_cache })
  }

  async fn find_names(&self, transactions: &[EnhancedTransaction]) -> color_eyre::Result<Vec<AccountName>> {
    let mut names: Vec<AccountName> = transactions
      .iter()
      .flat_map(|t| t.account_data.iter())
      .map(|a| AccountName::new(String::from(&a.account)))
      .filter(|a| a.address != "11111111111111111111111111111111")
      .collect();

    for account_name in names.iter_mut() {
      if self.name_cache.contains_key(&account_name.address) {
        account_name.name = self.name_cache.get(&account_name.address).unwrap().name.clone();
        continue;
      }
      info!("looking name for account {}", account_name.address);
      let result = self.helius.get_names(&account_name.address).await;
      match result {
        Ok(domain_names) => {
          if !domain_names.domain_names.is_empty() {
            account_name.name = String::from(&domain_names.domain_names[0]);
            self.name_cache.insert(account_name.name.clone(), account_name.clone());
          } else {
            self.name_cache.insert(account_name.name.clone(), account_name.clone());
          }
        },
        Err(err) => {
          error!("failed getting name for account {} {}", &account_name.name, err);
          self.name_cache.insert(account_name.name.clone(), account_name.clone());
        },
      }
    }
    Ok(names)
  }

  #[tracing::instrument(skip_all)]
  pub async fn handle_hook(&self, transactions: Vec<EnhancedTransaction>) {
    if transactions.is_empty() {
      return;
    }
    let names: AccountNames = AccountNames(self.find_names(&transactions).await.unwrap_or_else(|_| Vec::new()));

    let messages: BotMessages = BotMessages(transactions.into_iter().map(BotMessage).collect());
    let to_send: String = format!("{}\n{}", messages, names);
    let r = Recipient::Id(self.id);
    let result = self.bot.send_message(r, &to_send).await;
    match result {
      Ok(message) => {
        info!("sent message");
        debug!("send_message result: {:#?}", message);
      },
      Err(err) => {
        error!("err:{:?}", err);
      },
    }
  }

  #[tracing::instrument(skip(self))]
  pub async fn health(&self) -> color_eyre::Result<u64> {
    let height = self.helius.connection().get_block_height().await?;
    Ok(height)
  }
}

pub fn with_bot(bot: Arc<SeleneBot>) -> impl Filter<Extract = (Arc<SeleneBot>,), Error = Infallible> + Clone {
  warp::any().map(move || bot.clone())
}
