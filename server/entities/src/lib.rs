use actix_surreal_starter::pre_built::validators::*;
use actix_surreal_starter_macros::{api_entities, impl_display_for_error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use thiserror::Error;
use actix_surreal_starter::LoginData;

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
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
        user_id: RecordId,
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
        user_id: RecordId,
        metadata_id: RecordId,
    }

    MetadataTag|MetadataTagError("metadata_tags" ["metadata_id.user_id"]) {
        metadata_id: RecordId,
        tag_id: RecordId,
        exception: bool,
    }

    // TODO: make paths work properly with Options
    TagGroup|TagGroupError("tag_groups" ["user_id", "metadata_id.user_id"]) {
        user_id: RecordId,
        metadata_id: Option<RecordId>,
    }

    TagGroupTag|TagGroupTagError("tag_group_tags" ["tag_group_id.user_id", "tag_id.user_id"]) {
        tag_group_id: RecordId,
        tag_id: RecordId,
    }

    MetadataTagGroup|MetadataTagGroupError("metadata_tag_groups" ["metadata_id.user_id", "tag_group_id.user_id"]) {
        metadata_id: RecordId,
        tag_group_id: RecordId,
    }

    FinancialGoal|FinancialGoalError("financial_goals" ["user_id", "metadata_id.user_id"]) {
        user_id: RecordId,
        currency_id: RecordId,
        start_date: DateTime<Utc> [v1_gt_v2(end_date)],
        end_date: DateTime<Utc>,
        target_income: i64 [gt_zero],
        metadata_id: RecordId,
    }

    Transaction|TransactionError("transactions" ["account_id.user_id", "metadata_id.user_id"]) {
        account_id: RecordId,
        amount: i64,
        date: DateTime<Utc>,
        metadata_id: RecordId,
    }

    Transfer|TransferError("transfers" ["account_from.user_id", "metadata_id.user_id"]) {
        account_from: String,
        account_to: String,
        amount_from: i64 [gt_zero],
        amount_to: i64 [gt_zero],
        conversion_rate: f64 [gt_zero],
        fee: f64 [gt_zero],
        metadata_id: RecordId,
    }

    StableIncome|StableIncomeError("stable_incomes" ["user_id", "metadata_id.user_id"]) {
        user_id: RecordId,
        currency_id: RecordId,
        amount_per_month: i64 [ne_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        last_update_date: DateTime<Utc>,
        metadata_id: RecordId,
    }

    StableIncomeIncome|StableIncomeIncomeError("stable_income_incomes" ["stable_income_id.user_id", "transaction_id.user_id"]) {
        stable_income_id: RecordId,
        transaction_id: RecordId,
    }

    Loan|LoanError("loans" ["user_id", "metadata_id.user_id"]) {
        user_id: RecordId,
        currency_id: RecordId,
        principal_amount: i64 [gt_zero],
        interest_rate: f64 [gt_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(end_date)],
        end_date: Option<DateTime<Utc>>,
        interest_rate_type: String,
        compounding_frequency: String,
        metadata_id: RecordId,
    }

    LoanPayment|LoanPaymentError("loan_payments" ["loan_id.user_id", "transaction_id.user_id"]) {
        loan_id: RecordId,
        transaction_id: RecordId,
    }

    Investment|InvestmentError("investments" ["user_id", "metadata_id.user_id"]) {
        user_id: RecordId,
        currency_id: RecordId,
        r#type: String,
        compounding_frequency: String,
        principal_amount: i64 [gt_zero],
        start_date: DateTime<Utc> [optional_v2_gt_v1(expected_end_date), optional_v2_gt_v1(end_date)],
        expected_end_date: Option<DateTime<Utc>> ,
        end_date: Option<DateTime<Utc>>,
        risk_level: String,
        expected_return: f64 [gt_zero],
        metadata_id: RecordId,
    }

    InvestmentReturn|InvestmentReturnError("investment_returns" ["investment_id.user_id", "transaction_id.user_id"]) {
        investment_id: RecordId,
        transaction_id: RecordId,
    }

    Metadata|MetadataError("metadata" ["user_id"]) {
        title: Option<String>,
        description: Option<String>,
        user_id: RecordId,
    }

    Preference|PreferenceError("preferences" ["user_id"]) {
        user_id: RecordId,
        default_currency_id: RecordId,
        language: String,
    }

    AutoDistribution|AutoDistributionError("auto_distributions" ["account_id.user_id", "metadata_id.user_id"]) {
        ratio: f64 [gt_zero],
        account_id: RecordId,
        metadata_id: RecordId,
        record_metadata_id: RecordId,
    }

    FinancialGoalAutoDistribution|FinancialGoalAutoDistributionError("financial_goal_auto_distributions" ["financial_goal_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        financial_goal_id: RecordId,
        auto_distribution_id: RecordId,
    }

    StableIncomeAutoDistribution|StableIncomeAutoDistributionError("stable_income_auto_distributions" ["stable_income_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        stable_income_id: RecordId,
        auto_distribution_id: RecordId,
    }

    LoanAutoDistribution|LoanAutoDistributionError("loan_auto_distributions" ["loan_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        loan_id: RecordId,
        auto_distribution_id: RecordId,
    }

    InvestmentAutoDistribution|InvestmentAutoDistributionError("investment_auto_distributions" ["investment_id.user_id", "auto_distribution_id.account_id.user_id"]) {
        investment_id: RecordId,
        auto_distribution_id: RecordId,
    }

    TransferAutoDistribution|TransferAutoDistributionError("transfer_auto_distributions" ["auto_distribution_id.account_id.user_id", "metadata_id.user_id"]) {
        metadata_id: RecordId,
        account_to: String,
        auto_distribution_id: RecordId,
    }

    TransactionAutoDistribution|TransactionAutoDistributionError("transaction_auto_distributions" ["auto_distribution_id.account_id.user_id"]) {
        auto_distribution_id: RecordId,
    }

    FinancialGoalAllocations|FinancialGoalAllocationsError("financial_goal_allocations") {
        financial_goal_id: RecordId,
        account_id: Option<String>,
        date: DateTime<Utc>,
        amount: i64 [gt_zero],
        metadata_id: RecordId,
    }
);

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