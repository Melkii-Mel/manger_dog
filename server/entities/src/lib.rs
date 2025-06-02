#[cfg(feature = "actix-surreal-starter")]
use actix_surreal_starter::LoginData;
use actix_surreal_starter_macros::{api_entities, enums, impl_display_for_error};
use actix_surreal_starter_types::pre_built::validators::*;
use actix_surreal_starter_types::{RecordOf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use actix_surreal_starter_types::RecordId;
use thiserror::Error;
use actix_surreal_starter_types::ErrorEnum;

#[derive(Debug, Error, Serialize, Deserialize, Clone, ErrorEnum)]
pub enum ApiValidationError {
    DefaultError(#[from] ValidationError),
}

#[allow(dead_code)]
struct Validator;
impl DefaultValidations for Validator {}

impl_display_for_error!(ApiValidationError);

api_entities!(
    validator: Validator,
    error: ApiValidationError,
    // TODO: remove unnecessary fields from the user api, leaving them only for authorization and registration
    User|UserError("users") {
        email: String [email_format],
        username: String [not_empty],
        password: String [password_basic],
        registration_date: DateTime<Utc>,
        selected_preference: Option<String>,
    }
    // TODO: Validate other fields like currency_id. It appears that by default surrealDB allows adding non-existing ids freely
    Account|AccountError("accounts" ["user_id"]) {
        title: String [not_empty],
        currency_id: RecordId,
        balance: i64,
    }

    Register|RegisterError("register") {
        username: String [not_empty],
        email: String [email_format],
        password: String [password_basic],
    }

    Creds|CredsError("creds") {
        email: String,
        password: String,
    }

    Tag|TagError("tags" ["user_id", "metadata_id.user_id"]) {
        metadata_id: RecordOf<Metadata>|MetadataError,
    }

    MetadataTag|MetadataTagError("metadata_tags" ["metadata_id.user_id"]) {
        metadata_id: RecordOf<Metadata>,
        tag_id: RecordOf<Tag>,
        exception: bool,
    }

    // TODO: make paths work properly with Options
    TagGroup|TagGroupError("tag_groups" ["user_id", "metadata_id.user_id"]) {
        metadata_id: Option<RecordOf<Metadata>>,
    }

    TagGroupTag|TagGroupTagError("tag_group_tags" ["tag_group_id.user_id", "tag_id.user_id"]) {
        tag_group_id: RecordOf<TagGroup>,
        tag_id: RecordOf<Tag>,
    }

    MetadataTagGroup|MetadataTagGroupError("metadata_tag_groups" ["metadata_id.user_id", "tag_group_id.user_id"]) {
        metadata_id: RecordOf<Metadata>,
        tag_group_id: RecordOf<TagGroup>,
    }

    FinancialGoal|FinancialGoalError("financial_goals" ["user_id", "metadata_id.user_id"]) {
        currency_id: RecordId,
        start_date: DateTime<Utc> [v1_gt_v2(end_date)],
        end_date: DateTime<Utc>,
        target_income: u64,
        metadata_id: RecordOf<Metadata>,
    }

    Transaction|TransactionError("transactions" ["account_id.user_id", "metadata_id.user_id"]) {
        account_id: RecordOf<Account>,
        amount: i64,
        date: DateTime<Utc>,
        metadata_id: RecordOf<Metadata>,
    }

    Transfer|TransferError("transfers" ["account_from.user_id", "metadata_id.user_id"]) {
        account_from: RecordOf<Account>,
        account_to: RecordOf<Account>,
        amount_from: u64,
        amount_to: u64,
        conversion_rate: f64 [gt_zero],
        fee: f64 [gt_zero],
        metadata_id: RecordOf<Metadata>,
    }

    StableIncome|StableIncomeError("stable_incomes" ["user_id", "metadata_id.user_id"]) {
        currency_id: RecordId,
        amount_per_month: i64 [ne_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        last_update_date: DateTime<Utc>,
        metadata_id: RecordOf<Metadata>,
    }

    StableIncomeIncome|StableIncomeIncomeError("stable_income_incomes" ["stable_income_id.user_id", "transaction_id.user_id"]) {
        stable_income_id: RecordOf<StableIncome>,
        transaction_id: RecordOf<Transaction>,
    }

    Loan|LoanError("loans" ["user_id", "metadata_id.user_id"]) {
        currency_id: RecordId,
        principal_amount: u64,
        interest_rate: f64 [gt_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        interest_rate_type: String,
        compounding_frequency: String,
        metadata_id: RecordOf<Metadata>,
    }

    LoanPayment|LoanPaymentError("loan_payments" ["loan_id.user_id", "transaction_id.user_id"]) {
        loan_id: RecordOf<Loan>,
        transaction_id: RecordOf<Transaction>,
    }

    Investment|InvestmentError("investments" ["user_id", "metadata_id.user_id"]) {
        currency_id: RecordId,
        r#type: String,
        compounding_frequency: String,
        principal_amount: u64,
        start_date: DateTime<Utc> [optional_v2_gt_v1(expected_end_date), optional_v2_gt_v1(end_date)],
        expected_end_date: Option<DateTime<Utc>> ,
        end_date: Option<DateTime<Utc>>,
        risk_level: String,
        expected_return: f64 [gt_zero],
        metadata_id: RecordOf<Metadata>,
    }

    InvestmentReturn|InvestmentReturnError("investment_returns" ["investment_id.user_id", "transaction_id.user_id"]) {
        investment_id: RecordOf<Investment>,
        transaction_id: RecordOf<Transaction>,
    }

    Metadata|MetadataError("metadata" ["user_id"]) {
        title: Option<String>,
        description: Option<String>,
    }

    Preference|PreferenceError("preferences" ["user_id"]) {
        default_currency_id: RecordId,
        language: String,
    }

    AutoDistribution|AutoDistributionError("auto_distributions" ["account_id.user_id", "metadata_id.user_id", "record_metadata_id.user_id"]) {
        ratio: f64 [gt_zero],
        account_id: RecordOf<Account>,
        metadata_id: RecordOf<Metadata>,
        record_metadata_id: RecordOf<Metadata>,
    }

    FinancialGoalAutoDistribution|FinancialGoalAutoDistributionError("financial_goal_auto_distributions" ["financial_goal_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        financial_goal_id: RecordOf<FinancialGoal>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }

    StableIncomeAutoDistribution|StableIncomeAutoDistributionError("stable_income_auto_distributions" ["stable_income_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        stable_income_id: RecordOf<StableIncome>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }

    LoanAutoDistribution|LoanAutoDistributionError("loan_auto_distributions" ["loan_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        loan_id: RecordOf<Loan>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }

    InvestmentAutoDistribution|InvestmentAutoDistributionError("investment_auto_distributions" ["investment_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        investment_id: RecordOf<Investment>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }

    TransferAutoDistribution|TransferAutoDistributionError("transfer_auto_distributions" ["auto_distribution_id.account_id.user_id", "metadata_id.user_id"]) {
        metadata_id: RecordOf<Metadata>,
        account_to: RecordOf<Account>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }

    TransactionAutoDistribution|TransactionAutoDistributionError("transaction_auto_distributions" ["auto_distribution_id.account_id.user_id"]) {
        auto_distribution_id: RecordOf<AutoDistribution>,
    }

    FinancialGoalAllocations|FinancialGoalAllocationsError("financial_goal_allocations" ["financial_goal_id.user_id", "account_id.user_id", "metadata_id.user_id"]) {
        financial_goal_id: RecordOf<FinancialGoal>,
        account_id: Option<RecordOf<Account>>,
        date: DateTime<Utc>,
        amount: u64,
        metadata_id: RecordOf<Metadata>,
    }
);

enums! {
    GroupRoles("group_roles"),                          // 'owner', 'member', 'readonly'
    InterestRateTypes("interest_rate_types"),           // 'simple', 'compound'
    CompoundingFrequencies("compounding_frequencies"),  // 'annually', 'semi_annually', 'quarterly', 'monthly', 'daily'
    InvestmentTypes("investment_types"),                // 'stocks', 'bonds', 'real_estate', 'mutual_funds', 'etfs', 'cryptocurrency', 'precious_metals', 'commodities', 'p2p_lending'
    RiskLevels("risk_levels"),                          // 'low', 'moderate', 'high', 'very_high'
    Currencies("currencies"),                           // 'USD', 'EUR', 'GBP', 'JPY', 'AUD', 'CAD', 'CHF', 'CNY', 'INR', 'BRL', 'BTC', 'ETH', 'USDT', 'USDC', 'RUB'
    Languages("languages"),                             // 'EN', 'RU'
}


#[cfg(feature = "actix-surreal-starter")]
impl LoginData for Register {
    fn get_password_mut(&mut self) -> &mut String {
        &mut self.password
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_login(&self) -> &String {
        &self.email
    }
}

#[cfg(feature = "actix-surreal-starter")]
impl LoginData for Creds {
    fn get_password_mut(&mut self) -> &mut String {
        &mut self.password
    }

    fn get_password(&self) -> &String {
        &self.password
    }

    fn get_login(&self) -> &String {
        &self.email
    }
}
