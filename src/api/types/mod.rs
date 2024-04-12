use serde::{Deserialize, Serialize};
pub use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub mod enhanced;
mod fee;
pub mod webhook;
pub use fee::{
  FeeLevelRequest, GetPriorityFeeEstimateOptions, GetPriorityFeeEstimateRequest, GetPriorityFeeEstimateResponse,
  MicroLamportPriorityFeeLevels, PriorityLevel,
};

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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TxnStatus {
  #[default]
  All,
  Success,
  Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
  AcceptEscrowArtist,
  AcceptEscrowUser,
  AcceptRequestArtist,
  ActivateTransaction,
  ActivateVault,
  AddInstruction,
  AddItem,
  AddRaritiesToBank,
  AddTokenToVault,
  AddToPool,
  AddToWhitelist,
  Any,
  ApproveTransaction,
  AttachMetadata,
  AuctionHouseCreate,
  AuctionManagerClaimBid,
  AuthorizeFunder,
  BorrowFox,
  BorrowSolForNft,
  Burn,
  BurnNft,
  BuyItem,
  BuySubscription,
  BuyTickets,
  CancelEscrow,
  CancelLoanRequest,
  CancelOffer,
  CancelOrder,
  CancelReward,
  CancelSwap,
  CancelTransaction,
  CandyMachineRoute,
  CandyMachineUnwrap,
  CandyMachineUpdate,
  CandyMachineWrap,
  ChangeComicState,
  ClaimNft,
  ClaimRewards,
  CloseAccount,
  CloseEscrowAccount,
  CloseItem,
  CloseOrder,
  ClosePosition,
  CompressedNftBurn,
  CompressedNftCancelRedeem,
  CompressedNftDelegate,
  CompressedNftMint,
  CompressedNftRedeem,
  CompressedNftSetVerifyCollection,
  CompressedNftTransfer,
  CompressedNftUnverifyCollection,
  CompressedNftUnverifyCreator,
  CompressedNftVerifyCollection,
  CompressedNftVerifyCreator,
  CompressNft,
  CreateAppraisal,
  CreateBet,
  CreateEscrow,
  CreateMasterEdition,
  CreateMerkleTree,
  CreateOrder,
  CreatePool,
  CreateRaffle,
  CreateStore,
  CreateTransaction,
  DeauthorizeFunder,
  DecompressNft,
  DelegateMerkleTree,
  DelistItem,
  Deposit,
  DepositFractionalPool,
  DepositGem,
  DistributeCompressionRewards,
  EmptyPaymentAccount,
  ExecuteTransaction,
  FillOrder,
  FinalizeProgramInstruction,
  ForecloseLoan,
  Fractionalize,
  FundReward,
  Fuse,
  InitAuctionManagerV2,
  InitBank,
  InitFarm,
  InitFarmer,
  InitializeAccount,
  InitRent,
  InitStake,
  InitSwap,
  InitVault,
  KickItem,
  LendForNft,
  ListItem,
  Loan,
  LoanFox,
  LockReward,
  MergeStake,
  MigrateToPnft,
  NftAuctionCancelled,
  NftAuctionCreated,
  NftAuctionUpdated,
  NftBid,
  NftBidCancelled,
  NftCancelListing,
  NftGlobalBid,
  NftGlobalBidCancelled,
  NftListing,
  NftMint,
  NftMintRejected,
  NftParticipationReward,
  NftRentActivate,
  NftRentCancelListing,
  NftRentEnd,
  NftRentListing,
  NftRentUpdateListing,
  NftSale,
  OfferLoan,
  Payout,
  PlaceBet,
  PlaceSolBet,
  PlatformFee,
  ReborrowSolForNft,
  RecordRarityPoints,
  RefreshFarmer,
  RejectSwap,
  RejectTransaction,
  RemoveFromPool,
  RemoveFromWhitelist,
  RepayLoan,
  RequestLoan,
  RequestPnftMigration,
  RescindLoan,
  SetAuthority,
  SetBankFlags,
  SetVaultLock,
  SplitStake,
  StakeSol,
  StakeToken,
  StartPnftMigration,
  Swap,
  SwitchFox,
  SwitchFoxRequest,
  TakeLoan,
  TokenMint,
  Transfer,
  Unknown,
  Unlabeled,
  UnstakeSol,
  UnstakeToken,
  UpdateBankManager,
  UpdateExternalPriceAccount,
  UpdateFarm,
  UpdateItem,
  UpdateOffer,
  UpdateOrder,
  UpdatePrimarySaleMetadata,
  UpdateRaffle,
  UpdateRecordAuthorityData,
  UpdateVaultOwner,
  UpgradeFox,
  UpgradeFoxRequest,
  UpgradeProgramInstruction,
  ValidateSafetyDepositBoxV2,
  WhitelistCreator,
  Withdraw,
  WithdrawGem,
  #[serde(other)]
  Other(String),
}

#[allow(clippy::too_many_lines)]
impl TransactionType {
  pub fn all() -> Vec<Self> {
    vec![
      Self::AcceptEscrowArtist,
      Self::AcceptEscrowUser,
      Self::AcceptRequestArtist,
      Self::ActivateTransaction,
      Self::ActivateVault,
      Self::AddInstruction,
      Self::AddItem,
      Self::AddRaritiesToBank,
      Self::AddTokenToVault,
      Self::AddToPool,
      Self::AddToWhitelist,
      Self::Any,
      Self::ApproveTransaction,
      Self::AttachMetadata,
      Self::AuctionHouseCreate,
      Self::AuctionManagerClaimBid,
      Self::AuthorizeFunder,
      Self::BorrowFox,
      Self::BorrowSolForNft,
      Self::Burn,
      Self::BurnNft,
      Self::BuyItem,
      Self::BuySubscription,
      Self::BuyTickets,
      Self::CancelEscrow,
      Self::CancelLoanRequest,
      Self::CancelOffer,
      Self::CancelOrder,
      Self::CancelReward,
      Self::CancelSwap,
      Self::CancelTransaction,
      Self::CandyMachineRoute,
      Self::CandyMachineUnwrap,
      Self::CandyMachineUpdate,
      Self::CandyMachineWrap,
      Self::ChangeComicState,
      Self::ClaimNft,
      Self::ClaimRewards,
      Self::CloseAccount,
      Self::CloseEscrowAccount,
      Self::CloseItem,
      Self::CloseOrder,
      Self::ClosePosition,
      Self::CompressedNftBurn,
      Self::CompressedNftCancelRedeem,
      Self::CompressedNftDelegate,
      Self::CompressedNftMint,
      Self::CompressedNftRedeem,
      Self::CompressedNftSetVerifyCollection,
      Self::CompressedNftTransfer,
      Self::CompressedNftUnverifyCollection,
      Self::CompressedNftUnverifyCreator,
      Self::CompressedNftVerifyCollection,
      Self::CompressedNftVerifyCreator,
      Self::CompressNft,
      Self::CreateAppraisal,
      Self::CreateBet,
      Self::CreateEscrow,
      Self::CreateMasterEdition,
      Self::CreateMerkleTree,
      Self::CreateOrder,
      Self::CreatePool,
      Self::CreateRaffle,
      Self::CreateStore,
      Self::CreateTransaction,
      Self::DeauthorizeFunder,
      Self::DecompressNft,
      Self::DelegateMerkleTree,
      Self::DelistItem,
      Self::Deposit,
      Self::DepositFractionalPool,
      Self::DepositGem,
      Self::DistributeCompressionRewards,
      Self::EmptyPaymentAccount,
      Self::ExecuteTransaction,
      Self::FillOrder,
      Self::FinalizeProgramInstruction,
      Self::ForecloseLoan,
      Self::Fractionalize,
      Self::FundReward,
      Self::Fuse,
      Self::InitAuctionManagerV2,
      Self::InitBank,
      Self::InitFarm,
      Self::InitFarmer,
      Self::InitializeAccount,
      Self::InitRent,
      Self::InitStake,
      Self::InitSwap,
      Self::InitVault,
      Self::KickItem,
      Self::LendForNft,
      Self::ListItem,
      Self::Loan,
      Self::LoanFox,
      Self::LockReward,
      Self::MergeStake,
      Self::MigrateToPnft,
      Self::NftAuctionCancelled,
      Self::NftAuctionCreated,
      Self::NftAuctionUpdated,
      Self::NftBid,
      Self::NftBidCancelled,
      Self::NftCancelListing,
      Self::NftGlobalBid,
      Self::NftGlobalBidCancelled,
      Self::NftListing,
      Self::NftMint,
      Self::NftMintRejected,
      Self::NftParticipationReward,
      Self::NftRentActivate,
      Self::NftRentCancelListing,
      Self::NftRentEnd,
      Self::NftRentListing,
      Self::NftRentUpdateListing,
      Self::NftSale,
      Self::OfferLoan,
      Self::Payout,
      Self::PlaceBet,
      Self::PlaceSolBet,
      Self::PlatformFee,
      Self::ReborrowSolForNft,
      Self::RecordRarityPoints,
      Self::RefreshFarmer,
      Self::RejectSwap,
      Self::RejectTransaction,
      Self::RemoveFromPool,
      Self::RemoveFromWhitelist,
      Self::RepayLoan,
      Self::RequestLoan,
      Self::RequestPnftMigration,
      Self::RescindLoan,
      Self::SetAuthority,
      Self::SetBankFlags,
      Self::SetVaultLock,
      Self::SplitStake,
      Self::StakeSol,
      Self::StakeToken,
      Self::StartPnftMigration,
      Self::Swap,
      Self::SwitchFox,
      Self::SwitchFoxRequest,
      Self::TakeLoan,
      Self::TokenMint,
      Self::Transfer,
      Self::Unknown,
      Self::Unlabeled,
      Self::UnstakeSol,
      Self::UnstakeToken,
      Self::UpdateBankManager,
      Self::UpdateExternalPriceAccount,
      Self::UpdateFarm,
      Self::UpdateItem,
      Self::UpdateOffer,
      Self::UpdateOrder,
      Self::UpdatePrimarySaleMetadata,
      Self::UpdateRaffle,
      Self::UpdateRecordAuthorityData,
      Self::UpdateVaultOwner,
      Self::UpgradeFox,
      Self::UpgradeFoxRequest,
      Self::UpgradeProgramInstruction,
      Self::ValidateSafetyDepositBoxV2,
      Self::WhitelistCreator,
      Self::Withdraw,
      Self::WithdrawGem,
    ]
  }
}
