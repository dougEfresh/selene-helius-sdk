use serde::{Deserialize, Serialize};
pub use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub mod enhanced;
pub mod webhook;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum AssetSortBy {
  #[serde(rename = "created")]
  #[default]
  Created,
  #[serde(rename = "updated")]
  Updated,
  #[serde(rename = "recent_action")]
  RecentAction,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum AssetSortDirection {
  #[serde(rename = "asc")]
  #[default]
  Asc,
  #[serde(rename = "desc")]
  Desc,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum OwnershipModel {
  #[serde(rename = "single")]
  #[default]
  Single,
  #[serde(rename = "token")]
  Token,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Scope {
  #[serde(rename = "full")]
  Full,
  #[serde(rename = "royalty")]
  Royalty,
  #[serde(rename = "metadata")]
  Metadata,
  #[serde(rename = "extension")]
  Extension,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum UseMethods {
  Burn,
  Single,
  Multiple,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Context {
  #[serde(rename = "wallet-default")]
  WalletDefault,
  #[serde(rename = "web-desktop")]
  WebDesktop,
  #[serde(rename = "web-mobile")]
  WebMobile,
  #[serde(rename = "app-mobile")]
  AppMobile,
  #[serde(rename = "app-desktop")]
  AppDesktop,
  #[serde(rename = "app")]
  App,
  #[serde(rename = "vr")]
  Vr,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
  #[serde(rename = "fungible")]
  Fungible,
  #[serde(rename = "nonFungible")]
  NonFungible,
  #[serde(rename = "regularNft")]
  RegularNft,
  #[serde(rename = "compressedNft")]
  CompressedNft,
  #[serde(rename = "all")]
  All,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize_enum_str, Serialize_enum_str, Default)]
pub enum Interface {
  #[allow(non_camel_case_types)]
  V1_NFT,
  #[default]
  Custom,
  #[allow(non_camel_case_types)]
  V1_PRINT,
  #[allow(non_camel_case_types)]
  Legacy_NFT,
  #[allow(non_camel_case_types)]
  V2_NFT,
  FungibleAsset,
  FungibleToken,
  Identity,
  Executable,
  ProgrammableNFT,
  #[serde(other)]
  Unknown(String),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum RoyaltyModel {
  #[serde(rename = "creators")]
  Creators,
  #[serde(rename = "fanout")]
  Fanout,
  #[serde(rename = "single")]
  Single,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RpcError {
  pub id: String,
  pub error: RpcErrorBody,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RpcErrorBody {
  pub code: i32,
  pub message: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HeliusOptions {
  pub limit: Option<u32>,
  #[serde(rename = "paginationToken")]
  pub pagination_token: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum CollectionIdentifier {
  #[serde(rename = "firstVerifiedCreators")]
  FirstVerifiedCreators(Vec<String>),
  #[serde(rename = "verifiedCollectionAddress")]
  VerifiedCollectionAddress(Vec<String>),
}

#[derive(Clone, Debug, Deserialize_enum_str, Serialize_enum_str, Default)]
#[serde(rename_all = "camelCase")]
pub enum AccountWebhookEncoding {
  #[default]
  JsonParsed,
  #[serde(other)]
  Other(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TxnStatus {
  #[default]
  All,
  Success,
  Failed,
}

#[derive(Clone, Debug, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionContext {
  Auction,
  InstantSale,
  Offer,
  GlobalOffer,
  Mint,
  Unknown,
  #[serde(other)]
  Other(String),
}

#[derive(Clone, Debug, Deserialize_enum_str, Serialize_enum_str)]
pub enum TokenStandard {
  ProgrammableNonFungible,
  NonFungible,
  Fungible,
  FungibleAsset,
  NonFungibleEdition,
  UnknownStandard,
  #[serde(other)]
  Other(String),
}

#[derive(Clone, Debug, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProgramName {
  Unkown,
  JupiterV1,
  JupiterV2,
  JupiterV3,
  JupiterV4,
  MercurialStableSwap,
  SaberStableSwap,
  SaberExchange,
  SerumDexV1,
  SerumDexV2,
  SerumDexV3,
  SerumSwap,
  StepFinance,
  Cropper,
  RaydiumLiquidityPoolV2,
  RaydiumLiquidityPoolV3,
  RaydiumLiquidityPoolV4,
  AldrinAmmV1,
  AldrinAmmV2,
  Crema,
  Lifinity,
  LifinityV2,
  Cykura,
  OrcaTokenSwapV1,
  OrcaTokenSwapV2,
  OrcaWhirlpools,
  Marinade,
  Stepn,
  SenchaExchange,
  SarosAmm,
  FoxyStake,
  FoxySwap,
  FoxyRaffle,
  FoxyTokenMarket,
  FoxyMissions,
  FoxyMarmalade,
  FoxyCoinflip,
  FoxyAuction,
  Citrus,
  HadeSwap,
  Zeta,
  CardinalRent,
  CardinalStaking,
  SharkyFi,
  OpenCreatorProtocol,
  Bubblegum,
  CoralCube,
  #[serde(other)]
  Other(String),
}

#[derive(Clone, Debug, Deserialize_enum_str, Serialize_enum_str)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
  FormFunction,
  ExchangeArt,
  CandyMachineV3,
  CandyMachineV2,
  CandyMachineV1,
  Unknown,
  Solanart,
  Solsea,
  MagicEden,
  Holaplex,
  Metaplex,
  Opensea,
  SolanaProgramLibrary,
  Anchor,
  Phantom,
  SystemProgram,
  StakeProgram,
  Coinbase,
  CoralCube,
  Hedge,
  LaunchMyNft,
  GemBank,
  GemFarm,
  Degods,
  Bsl,
  Yawww,
  Atadia,
  DigitalEyes,
  Hyperspace,
  Tensor,
  Bifrost,
  Jupiter,
  Mecurial,
  Saber,
  Serum,
  StepFinance,
  Cropper,
  Raydium,
  Aldrin,
  Crema,
  Lifinity,
  Cykura,
  Orca,
  Marinade,
  Stepn,
  Sencha,
  Saros,
  EnglishAuction,
  Foxy,
  Hadeswap,
  FoxyStaking,
  FoxyRaffle,
  FoxyTokenMarket,
  FoxyMissions,
  FoxyMarmalade,
  FoxyCoinflip,
  FoxyAuction,
  Citrus,
  Zeta,
  Elixir,
  ElixirLaunchpad,
  CardinalRent,
  CardinalStaking,
  BpfLoader,
  BpfUpgradeableLoader,
  Squads,
  SharkyFi,
  OpenCreatorProtocol,
  Bubblegum,
  // Mints
  W_SOL,
  DUST,
  SOLI,
  USDC,
  FLWR,
  HDG,
  MEAN,
  UXD,
  SHDW,
  POLIS,
  ATLAS,
  USH,
  TRTLS,
  RUNNER,
  INVICTUS,
  #[serde(other)]
  Other(String),
}

#[derive(Clone, Debug, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
  Unknown,
  NftBid,
  NftBidCancelled,
  NftListing,
  NftCancelListing,
  NftSale,
  NftMint,
  NftAuctionCreated,
  NftAuctionUpdated,
  NftAuctionCancelled,
  NftParticipationReward,
  NftMintRejected,
  CreateStore,
  WhitelistCreator,
  AddToWhitelist,
  RemoveFromWhitelist,
  AuctionManagerClaimBid,
  EmptyPaymentAccount,
  UpdatePrimarySaleMetadata,
  AddTokenToVault,
  ActivateVault,
  InitVault,
  InitBank,
  InitStake,
  MergeStake,
  SplitStake,
  SetBankFlags,
  SetVaultLock,
  UpdateVaultOwner,
  UpdateBankManager,
  RecordRarityPoints,
  AddRaritiesToBank,
  InitFarm,
  InitFarmer,
  RefreshFarmer,
  UpdateFarm,
  AuthorizeFunder,
  DeauthorizeFunder,
  FundReward,
  CancelReward,
  LockReward,
  Payout,
  ValidateSafetyDepositBoxV2,
  SetAuthority,
  InitAuctionManagerV2,
  UpdateExternalPriceAccount,
  AuctionHouseCreate,
  CloseEscrowAccount,
  Withdraw,
  Deposit,
  Transfer,
  Burn,
  BurnNft,
  PlatformFee,
  Loan,
  RepayLoan,
  AddToPool,
  RemoveFromPool,
  ClosePosition,
  Unlabeled,
  CloseAccount,
  WithdrawGem,
  DepositGem,
  StakeToken,
  UnstakeToken,
  StakeSol,
  UnstakeSol,
  ClaimRewards,
  BuySubscription,
  Swap,
  InitSwap,
  CancelSwap,
  RejectSwap,
  InitializeAccount,
  TokenMint,
  CreateApparaisal,
  Fuse,
  DepositFractionalPool,
  Fractionalize,
  CreateRaffle,
  BuyTickets,
  UpdateItem,
  ListItem,
  DelistItem,
  AddItem,
  CloseItem,
  BuyItem,
  FillOrder,
  UpdateOrder,
  CreateOrder,
  CloseOrder,
  CancelOrder,
  KickItem,
  UpgradeFox,
  UpgradeFoxRequest,
  LoanFox,
  BorrowBox,
  SwitchFoxRequest,
  SwitchFox,
  CreateEscrow,
  AcceptRequeestArtist,
  CancelEscrow,
  AcceptEscrowArtist,
  AcceptEscrowUser,
  PlaceBet,
  PlaceSolBet,
  CreateBet,
  NftRentUpdateListing,
  NftRentActivate,
  NftRentCancelListing,
  NftRentListing,
  FinalizeProgramInstruction,
  UpgradeProgramInstruction,
  NftGlobalBix,
  NftGlobalBidCancel,
  ExecuteTransaction,
  ApproveTransaction,
  CreateTransaction,
  RejectTransaction,
  CancelTransaction,
  AddInstruction,
  AttachMetadata,
  RequestPnftMigration,
  StartPnftMigration,
  MigrateToPnft,
  UpdateRaffle,
  #[serde(other)]
  Other(String),
}

#[allow(clippy::too_many_lines)]
impl TransactionType {
  pub fn all() -> Vec<Self> {
    vec![
      Self::NftBid,
      Self::NftBidCancelled,
      Self::NftListing,
      Self::NftCancelListing,
      Self::NftSale,
      Self::NftMint,
      Self::NftAuctionCreated,
      Self::NftAuctionUpdated,
      Self::NftAuctionCancelled,
      Self::NftParticipationReward,
      Self::NftMintRejected,
      Self::CreateStore,
      Self::WhitelistCreator,
      Self::AddToWhitelist,
      Self::RemoveFromWhitelist,
      Self::AuctionManagerClaimBid,
      Self::EmptyPaymentAccount,
      Self::UpdatePrimarySaleMetadata,
      Self::AddTokenToVault,
      Self::ActivateVault,
      Self::InitVault,
      Self::InitBank,
      Self::InitStake,
      Self::MergeStake,
      Self::SplitStake,
      Self::SetBankFlags,
      Self::SetVaultLock,
      Self::UpdateVaultOwner,
      Self::UpdateBankManager,
      Self::RecordRarityPoints,
      Self::AddRaritiesToBank,
      Self::InitFarm,
      Self::InitFarmer,
      Self::RefreshFarmer,
      Self::UpdateFarm,
      Self::AuthorizeFunder,
      Self::DeauthorizeFunder,
      Self::FundReward,
      Self::CancelReward,
      Self::LockReward,
      Self::Payout,
      Self::ValidateSafetyDepositBoxV2,
      Self::SetAuthority,
      Self::InitAuctionManagerV2,
      Self::UpdateExternalPriceAccount,
      Self::AuctionHouseCreate,
      Self::CloseEscrowAccount,
      Self::Withdraw,
      Self::Deposit,
      Self::Transfer,
      Self::Burn,
      Self::BurnNft,
      Self::PlatformFee,
      Self::Loan,
      Self::RepayLoan,
      Self::AddToPool,
      Self::RemoveFromPool,
      Self::ClosePosition,
      Self::Unlabeled,
      Self::CloseAccount,
      Self::WithdrawGem,
      Self::DepositGem,
      Self::StakeToken,
      Self::UnstakeToken,
      Self::StakeSol,
      Self::UnstakeSol,
      Self::ClaimRewards,
      Self::BuySubscription,
      Self::Swap,
      Self::InitSwap,
      Self::CancelSwap,
      Self::RejectSwap,
      Self::InitializeAccount,
      Self::TokenMint,
      Self::CreateApparaisal,
      Self::Fuse,
      Self::DepositFractionalPool,
      Self::Fractionalize,
      Self::CreateRaffle,
      Self::BuyTickets,
      Self::UpdateItem,
      Self::ListItem,
      Self::DelistItem,
      Self::AddItem,
      Self::CloseItem,
      Self::BuyItem,
      Self::FillOrder,
      Self::UpdateOrder,
      Self::CreateOrder,
      Self::CloseOrder,
      Self::CancelOrder,
      Self::KickItem,
      Self::UpgradeFox,
      Self::UpgradeFoxRequest,
      Self::LoanFox,
      Self::BorrowBox,
      Self::SwitchFoxRequest,
      Self::SwitchFox,
      Self::CreateEscrow,
      Self::AcceptRequeestArtist,
      Self::CancelEscrow,
      Self::AcceptEscrowArtist,
      Self::AcceptEscrowUser,
      Self::PlaceBet,
      Self::PlaceSolBet,
      Self::CreateBet,
      Self::NftRentUpdateListing,
      Self::NftRentActivate,
      Self::NftRentCancelListing,
      Self::NftRentListing,
      Self::FinalizeProgramInstruction,
      Self::UpgradeProgramInstruction,
      Self::NftGlobalBix,
      Self::NftGlobalBidCancel,
      Self::ExecuteTransaction,
      Self::ApproveTransaction,
      Self::CreateTransaction,
      Self::RejectTransaction,
      Self::CancelTransaction,
      Self::AddInstruction,
      Self::AttachMetadata,
      Self::RequestPnftMigration,
      Self::StartPnftMigration,
      Self::MigrateToPnft,
      Self::UpdateRaffle,
    ]
  }
}
