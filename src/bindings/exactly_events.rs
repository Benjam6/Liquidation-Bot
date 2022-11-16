use ethers::{
    abi::{Error, RawLog},
    prelude::EthLogDecode,
};

use crate::credit_service::{
    auditor_mod::{RoleAdminChangedFilter, RoleGrantedFilter, RoleRevokedFilter},
    EarningsAccumulatorSmoothFactorSetFilter, AdjustFactorSetFilter, AnswerUpdatedFilter,
    ApprovalFilter, SeizeFilter, PriceFeedSetFilter, BorrowAtMaturityFilter,
    FixedParametersSetFilter, DampSpeedSetFilter, DepositAtMaturityFilter, DepositFilter,
    InterestRateModelSetFilter, LiquidateBorrowFilter, LiquidationIncentiveSetFilter,
    MarketEnteredFilter, MarketExitedFilter, MarketListedFilter, MarketUpdatedFilter, MarketUpdatedAtMaturityFilter,
    MaxFuturePoolsSetFilter, OracleSetFilter, PausedFilter, PenaltyRateSetFilter,
    RepayAtMaturityFilter, ReserveFactorSetFilter,
    BackupFeeRateSetFilter, TransferFilter, UnpausedFilter, WithdrawAtMaturityFilter, WithdrawFilter,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExactlyEvents {
    TransferFilter(TransferFilter),
    DepositFilter(DepositFilter),
    WithdrawFilter(WithdrawFilter),
    ApprovalFilter(ApprovalFilter),
    DepositAtMaturityFilter(DepositAtMaturityFilter),
    WithdrawAtMaturityFilter(WithdrawAtMaturityFilter),
    BorrowAtMaturityFilter(BorrowAtMaturityFilter),
    RepayAtMaturityFilter(RepayAtMaturityFilter),
    LiquidateBorrowFilter(LiquidateBorrowFilter),
    SeizeFilter(SeizeFilter),
    EarningsAccumulatorSmoothFactorSetFilter(EarningsAccumulatorSmoothFactorSetFilter),
    MaxFuturePoolsSetFilter(MaxFuturePoolsSetFilter),
    RoleGrantedFilter(RoleGrantedFilter),
    RoleAdminChangedFilter(RoleAdminChangedFilter),
    RoleRevokedFilter(RoleRevokedFilter),
    PausedFilter(PausedFilter),
    UnpausedFilter(UnpausedFilter),
    MarketUpdatedFilter(MarketUpdatedFilter),
    MarketUpdatedAtMaturityFilter(MarketUpdatedAtMaturityFilter),

    // Auditor events
    MarketListedFilter(MarketListedFilter),
    MarketEnteredFilter(MarketEnteredFilter),
    MarketExitedFilter(MarketExitedFilter),
    OracleSetFilter(OracleSetFilter),
    LiquidationIncentiveSetFilter(LiquidationIncentiveSetFilter),
    AdjustFactorSetFilter(AdjustFactorSetFilter),

    // PoolAccounting events
    InterestRateModelSetFilter(InterestRateModelSetFilter),
    PenaltyRateSetFilter(PenaltyRateSetFilter),
    ReserveFactorSetFilter(ReserveFactorSetFilter),
    DampSpeedSetFilter(DampSpeedSetFilter),

    // InterestRateModel events
    FixedParametersSetFilter(FixedParametersSetFilter),
    BackupFeeRateSetFilter(BackupFeeRateSetFilter),

    // ExactlyOracle events
    PriceFeedSetFilter(PriceFeedSetFilter),

    // PriceFeed
    AnswerUpdatedFilter(AnswerUpdatedFilter),
}

impl EthLogDecode for ExactlyEvents {
    fn decode_log(log: &RawLog) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
            return Ok(ExactlyEvents::RoleGrantedFilter(decoded));
        }
        if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
            return Ok(ExactlyEvents::RoleAdminChangedFilter(decoded));
        }
        if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
            return Ok(ExactlyEvents::RoleRevokedFilter(decoded));
        }
        if let Ok(decoded) = TransferFilter::decode_log(log) {
            return Ok(ExactlyEvents::TransferFilter(decoded));
        }
        if let Ok(decoded) = DepositFilter::decode_log(log) {
            return Ok(ExactlyEvents::DepositFilter(decoded));
        }
        if let Ok(decoded) = WithdrawFilter::decode_log(log) {
            return Ok(ExactlyEvents::WithdrawFilter(decoded));
        }
        if let Ok(decoded) = ApprovalFilter::decode_log(log) {
            return Ok(ExactlyEvents::ApprovalFilter(decoded));
        }
        if let Ok(decoded) = DepositAtMaturityFilter::decode_log(log) {
            return Ok(ExactlyEvents::DepositAtMaturityFilter(decoded));
        }
        if let Ok(decoded) = WithdrawAtMaturityFilter::decode_log(log) {
            return Ok(ExactlyEvents::WithdrawAtMaturityFilter(decoded));
        }
        if let Ok(decoded) = BorrowAtMaturityFilter::decode_log(log) {
            return Ok(ExactlyEvents::BorrowAtMaturityFilter(decoded));
        }
        if let Ok(decoded) = RepayAtMaturityFilter::decode_log(log) {
            return Ok(ExactlyEvents::RepayAtMaturityFilter(decoded));
        }
        if let Ok(decoded) = LiquidateBorrowFilter::decode_log(log) {
            return Ok(ExactlyEvents::LiquidateBorrowFilter(decoded));
        }
        if let Ok(decoded) = SeizeFilter::decode_log(log) {
            return Ok(ExactlyEvents::SeizeFilter(decoded));
        }
        if let Ok(decoded) = EarningsAccumulatorSmoothFactorSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::EarningsAccumulatorSmoothFactorSetFilter(
                decoded,
            ));
        }
        if let Ok(decoded) = MaxFuturePoolsSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::MaxFuturePoolsSetFilter(decoded));
        }
        if let Ok(decoded) = PausedFilter::decode_log(log) {
            return Ok(ExactlyEvents::PausedFilter(decoded));
        }
        if let Ok(decoded) = UnpausedFilter::decode_log(log) {
            return Ok(ExactlyEvents::UnpausedFilter(decoded));
        }
        if let Ok(decoded) = MarketUpdatedFilter::decode_log(log) {
            return Ok(ExactlyEvents::MarketUpdatedFilter(decoded));
        }
        if let Ok(decoded) = MarketUpdatedAtMaturityFilter::decode_log(log) {
            return Ok(ExactlyEvents::MarketUpdatedAtMaturityFilter(decoded));
        }

        // Auditor events
        if let Ok(decoded) = MarketListedFilter::decode_log(log) {
            return Ok(ExactlyEvents::MarketListedFilter(decoded));
        }
        if let Ok(decoded) = MarketEnteredFilter::decode_log(log) {
            return Ok(ExactlyEvents::MarketEnteredFilter(decoded));
        }
        if let Ok(decoded) = MarketExitedFilter::decode_log(log) {
            return Ok(ExactlyEvents::MarketExitedFilter(decoded));
        }
        if let Ok(decoded) = OracleSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::OracleSetFilter(decoded));
        }
        if let Ok(decoded) = LiquidationIncentiveSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::LiquidationIncentiveSetFilter(decoded));
        }
        if let Ok(decoded) = AdjustFactorSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::AdjustFactorSetFilter(decoded));
        }

        // PoolAccounting events
        if let Ok(decoded) = InterestRateModelSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::InterestRateModelSetFilter(decoded));
        }
        if let Ok(decoded) = PenaltyRateSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::PenaltyRateSetFilter(decoded));
        }
        if let Ok(decoded) = ReserveFactorSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::ReserveFactorSetFilter(decoded));
        }
        if let Ok(decoded) = DampSpeedSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::DampSpeedSetFilter(decoded));
        }

        // InterestRateModel events
        if let Ok(decoded) = FixedParametersSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::FixedParametersSetFilter(decoded));
        }
        if let Ok(decoded) = BackupFeeRateSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::BackupFeeRateSetFilter(decoded));
        }

        // ExactlyOracle events
        if let Ok(decoded) = PriceFeedSetFilter::decode_log(log) {
            return Ok(ExactlyEvents::PriceFeedSetFilter(decoded));
        }

        // PriceFeed
        if let Ok(decoded) = AnswerUpdatedFilter::decode_log(log) {
            return Ok(ExactlyEvents::AnswerUpdatedFilter(decoded));
        }

        println!("Missing event: {:?}", log);
        Err(Error::InvalidData)
    }
}
