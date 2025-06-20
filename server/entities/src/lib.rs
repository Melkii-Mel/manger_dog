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

#[derive(Debug, Error, Serialize, Deserialize, Clone, ErrorEnum, PartialEq)]
pub enum ApiValidationError {
    DefaultError(#[from] ValidationError),
}

#[allow(dead_code)]
struct Validator;
impl DefaultValidations for Validator {}

impl_display_for_error!(ApiValidationError);

// TODO: remove unnecessary fields from the user api, leaving them only for authorization and registration
api_entities!(
    validator: Validator,
    error: ApiValidationError,
    User[] {
        email: String [email_format],
        username: String [not_empty],
        password: String [password_basic],
        registration_date: DateTime<Utc>,
        selected_preference: Option<String>,
    }
    
    Account["user_id"] {
        title: String [not_empty],
        currency_id: RecordId,
        balance: i64,
    }
    
    Register[] {
        username: String [not_empty],
        email: String [email_format],
        password: String [password_basic],
    }
    
    Creds[] {
        email: String,
        password: String,
    }
    
    Tag["user_id", "metadata_id.user_id"] {
        metadata_id: RecordOf<Metadata>,
    }
    
    MetadataTag["metadata_id.user_id"] {
        metadata_id: RecordOf<Metadata>,
        tag_id: RecordOf<Tag>,
        exception: bool,
    }
    
    TagGroup["user_id", "metadata_id.user_id"] {
        metadata_id: Option<RecordOf<Metadata>>,
    }
    
    TagGroupTag["tag_group_id.user_id", "tag_id.user_id"] {
        tag_group_id: RecordOf<TagGroup>,
        tag_id: RecordOf<Tag>,
    }
    
    MetadataTagGroup["metadata_id.user_id", "tag_group_id.user_id"] {
        metadata_id: RecordOf<Metadata>,
        tag_group_id: RecordOf<TagGroup>,
    }
    
    FinancialGoal["user_id", "metadata_id.user_id"] {
        currency_id: RecordId,
        start_date: DateTime<Utc> [v1_gt_v2(end_date)],
        end_date: DateTime<Utc>,
        target_income: u64,
        metadata_id: RecordOf<Metadata>,
    }
    
    Transaction["account_id.user_id", "metadata_id.user_id"] {
        account_id: RecordOf<Account>,
        amount: i64,
        date: DateTime<Utc>,
        metadata_id: RecordOf<Metadata>,
    }
    
    Transfer["account_from.user_id", "metadata_id.user_id"] {
        account_from: RecordOf<Account>,
        account_to: RecordOf<Account>,
        amount_from: u64,
        amount_to: u64,
        conversion_rate: f64 [gt_zero],
        fee: f64 [gt_zero],
        metadata_id: RecordOf<Metadata>,
    }
    
    StableIncome["user_id", "metadata_id.user_id"] {
        currency_id: RecordId,
        amount_per_month: i64 [ne_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        last_update_date: DateTime<Utc>,
        metadata_id: RecordOf<Metadata>,
    }
    
    StableIncomeIncome["stable_income_id.user_id", "transaction_id.user_id"] {
        stable_income_id: RecordOf<StableIncome>,
        transaction_id: RecordOf<Transaction>,
    }
    
    Loan["user_id", "metadata_id.user_id"] {
        currency_id: RecordId,
        principal_amount: u64,
        interest_rate: f64 [gt_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        interest_rate_type: String,
        compounding_frequency: String,
        metadata_id: RecordOf<Metadata>,
    }
    
    LoanPayment["loan_id.user_id", "transaction_id.user_id"] {
        loan_id: RecordOf<Loan>,
        transaction_id: RecordOf<Transaction>,
    }
    
    Investment["user_id", "metadata_id.user_id"] {
        currency_id: RecordId,
        r#type: String,
        compounding_frequency: String,
        principal_amount: u64,
        start_date: DateTime<Utc> [optional_v2_gt_v1(expected_end_date), optional_v2_gt_v1(end_date)],
        expected_end_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        risk_level: String,
        expected_return: f64 [gt_zero],
        metadata_id: RecordOf<Metadata>,
    }
    
    InvestmentReturn["investment_id.user_id", "transaction_id.user_id"] {
        investment_id: RecordOf<Investment>,
        transaction_id: RecordOf<Transaction>,
    }
    
    Metadata["user_id"] {
        title: Option<String>,
        description: Option<String>,
    }
    
    Preference["user_id"] {
        default_currency_id: RecordId,
        language: String,
    }
    
    AutoDistribution["account_id.user_id", "metadata_id.user_id", "record_metadata_id.user_id"] {
        ratio: f64 [gt_zero],
        account_id: RecordOf<Account>,
        metadata_id: RecordOf<Metadata>,
        record_metadata_id: RecordOf<Metadata>,
    }
    
    FinancialGoalAutoDistribution["financial_goal_id.user_id", "auto_distribution_id.account_id.user_id"] {
        financial_goal_id: RecordOf<FinancialGoal>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }
    
    StableIncomeAutoDistribution["stable_income_id.user_id", "auto_distribution_id.account_id.user_id"] {
        stable_income_id: RecordOf<StableIncome>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }
    
    LoanAutoDistribution["loan_id.user_id", "auto_distribution_id.account_id.user_id"] {
        loan_id: RecordOf<Loan>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }
    
    InvestmentAutoDistribution["investment_id.user_id", "auto_distribution_id.account_id.user_id"] {
        investment_id: RecordOf<Investment>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }
    
    TransferAutoDistribution["auto_distribution_id.account_id.user_id", "metadata_id.user_id"] {
        metadata_id: RecordOf<Metadata>,
        account_to: RecordOf<Account>,
        auto_distribution_id: RecordOf<AutoDistribution>,
    }
    
    TransactionAutoDistribution["auto_distribution_id.account_id.user_id"] {
        auto_distribution_id: RecordOf<AutoDistribution>,
    }
    
    FinancialGoalAllocations["financial_goal_id.user_id", "account_id.user_id", "metadata_id.user_id"] {
        financial_goal_id: RecordOf<FinancialGoal>,
        account_id: Option<RecordOf<Account>>,
        date: DateTime<Utc>,
        amount: u64,
        metadata_id: RecordOf<Metadata>,
    }
);

enums! {
    GroupRole("group_roles"),                           // 'owner', 'member', 'readonly'
    InterestRateType("interest_rate_types"),            // 'simple', 'compound'
    CompoundingFrequency("compounding_frequencies"),    // 'annually', 'semi_annually', 'quarterly', 'monthly', 'daily'
    InvestmentType("investment_types"),                 // 'stocks', 'bonds', 'real_estate', 'mutual_funds', 'etfs', 'cryptocurrency', 'precious_metals', 'commodities', 'p2p_lending'
    RiskLevel("risk_levels"),                           // 'low', 'moderate', 'high', 'very_high'
    Currency("currencies"),                             // 'USD', 'EUR', 'GBP', 'JPY', 'AUD', 'CAD', 'CHF', 'CNY', 'INR', 'BRL', 'BTC', 'ETH', 'USDT', 'USDC', 'RUB'
    Language("languages"),                              // 'EN', 'RU'
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
