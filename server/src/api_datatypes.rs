use chrono::{DateTime, Utc};
use thiserror::Error;
use actix_surreal_starter_macros::{api_entities, impl_display_for_error};
use actix_surreal_starter::pre_built::validators::*;

#[derive(Debug, Error)]
pub enum ApiValidationError {
    DefaultError(#[from] ValidationError),
}

struct Validator;
impl DefaultValidations for Validator{}

impl_display_for_error!(ApiValidationError);

api_entities!(
    validator: Validator,
    error: ApiValidationError,
    // TODO: remove unnecessary fields from the user api, leaving them only for authorization and registration
    User("users") {
        email: String [email_format],
        username: String [not_empty],
        password: String [password_basic],
        registration_date: DateTime<Utc>,
        selected_preference: Option<String>,
    }
    // TODO: Validate other fields like currency_id. It appears that by default surrealDB allows adding non-existing ids freely
    Account("accounts" ["user_id"]) {
        title: String [not_empty],
        user_id: String,
        currency_id: String,
        balance: i64,
    }

    Creds("creds") {
        email: String [email_format],
        password: String [password_basic],
    }

    Tag("tags" ["user_id", "metadata_id.user_id"]) {
        user_id: String,
        metadata_id: String,
    }

    MetadataTag("metadata_tags" ["metadata_id.user_id"]) {
        metadata_id: String,
        tag_id: String,
        exception: bool,
    }

    // TODO: make paths work properly with Options
    TagGroup("tag_groups" ["user_id", "metadata_id.user_id"]) {
        user_id: String,
        metadata_id: Option<String>,
    }

    TagGroupTag("tag_group_tags" ["tag_group_id.user_id", "tag_id.user_id"]) {
        tag_group_id: String,
        tag_id: String,
    }

    MetadataTagGroup("metadata_tag_groups" ["metadata_id.user_id", "tag_group_id.user_id"]) {
        metadata_id: String,
        tag_group_id: String,
    }

    FinancialGoal("financial_goals" ["user_id", "metadata_id.user_id"]) {
        user_id: String,
        currency_id: String,
        start_date: DateTime<Utc> [v1_gt_v2(end_date)],
        end_date: DateTime<Utc>,
        target_income: i64 [gt_zero],
        metadata_id: String,
    }

    Transaction("transactions" ["account_id.user_id", "metadata_id.user_id"]) {
        account_id: String,
        amount: i64,
        date: DateTime<Utc>,
        metadata_id: String,
    }

    Transfer("transfers" ["account_from.user_id", "metadata_id.user_id"]) {
        account_from: String,
        account_to: String,
        amount_from: i64 [gt_zero],
        amount_to: i64 [gt_zero],
        conversion_rate: f64 [gt_zero],
        fee: f64 [gt_zero],
        metadata_id: String,
    }

    StableIncome("stable_incomes" ["user_id", "metadata_id.user_id"]) {
        user_id: String,
        currency_id: String,
        amount_per_month: i64 [ne_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        last_update_date: DateTime<Utc>,
        metadata_id: String,
    }

    StableIncomeIncome("stable_income_incomes" ["stable_income_id.user_id", "transaction_id.user_id"]) {
        stable_income_id: String,
        transaction_id: String,
    }

    Loan("loans" ["user_id", "metadata_id.user_id"]) {
        user_id: String,
        currency_id: String,
        principal_amount: i64 [gt_zero],
        interest_rate: f64 [gt_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        interest_rate_type: String,
        compounding_frequency: String,
        metadata_id: String,
    }

    LoanPayment("loan_payments" ["loan_id.user_id", "transaction_id.user_id"]) {
        loan_id: String,
        transaction_id: String,
    }

    Investment("investments" ["user_id", "metadata_id.user_id"]) {
        user_id: String,
        currency_id: String,
        r#type: String,
        compounding_frequency: String,
        principal_amount: i64 [gt_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(expected_end_date), optional_v2_gt_v1(end_date)],
        expected_end_date: Option<DateTime<Utc>> ,
        end_date: Option<DateTime<Utc>>,
        risk_level: String,
        expected_return: f64 [gt_zero],
        metadata_id: String,
    }

    InvestmentReturn("investment_returns" ["investment_id.user_id", "transaction_id.user_id"]) {
        investment_id: String,
        transaction_id: String,
    }

    Metadata("metadata" ["user_id"]) {
        title: Option<String>,
        description: Option<String>,
        user_id: String,
    }

    Preference("preferences" ["user_id"]) {
        user_id: String,
        default_currency_id: String,
        language: String,
    }

    AutoDistribution("auto_distributions" ["account_id.user_id", "metadata_id.user_id"]) {
        ratio: f64 [gt_zero],
        account_id: String,
        metadata_id: String,
        record_metadata_id: String,
    }

    FinancialGoalAutoDistribution("financial_goal_auto_distributions" ["financial_goal_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        financial_goal_id: String,
        auto_distribution_id: String,
    }

    StableIncomeAutoDistribution("stable_income_auto_distributions" ["stable_income_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        stable_income_id: String,
        auto_distribution_id: String,
    }

    LoanAutoDistribution("loan_auto_distributions" ["loan_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        loan_id: String,
        auto_distribution_id: String,
    }

    InvestmentAutoDistribution("investment_auto_distributions" ["investment_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        investment_id: String,
        auto_distribution_id: String,
    }

    TransferAutoDistribution("transfer_auto_distributions" ["auto_distribution_id.account_id.user_id", "metadata_id.user_id"]) {
        metadata_id: String,
        account_to: String,
        auto_distribution_id: String,
    }

    TransactionAutoDistribution("transaction_auto_distributions" ["auto_distribution_id.account_id.user_id"]) {
        auto_distribution_id: String,
    }

    FinancialGoalAllocations("financial_goal_allocations") {
        financial_goal_id: String,
        account_id: Option<String>,
        date: DateTime<Utc>,
        amount: i64 [gt_zero],
        metadata_id: String,
    }
);
