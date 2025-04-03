use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

macro_rules! api_structs {
    ($($name:ident { $($field:ident: $type:ty),*$(,)* })*) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        pub enum RequestBody {
            $($name($name)),*
        }

        $(
        #[derive(Debug, Deserialize, Serialize, Default, Clone)]
        pub struct $name {
            $(pub $field: $type),*
        }
        )*
    };
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ApiActions {
    Add,
    #[default]
    Get,
    Update,
    Delete,

    SignUp,
    SignIn,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub action: ApiActions,
    pub body: RequestBody,
    pub metadata: Option<Metadata>,
}

api_structs!(
    User {
        email: String,
        username: String,
        password: String,
        registration_date: DateTime<Utc>,
        selected_preference: Option<String>,
    }

    Account {
        title: String,
        user_id: String,
        currency_id: String,
        balance: i64,
    }

    Creds {
        email: String,
        password: String,
    }

    Tag {
        user_id: String,
        metadata_id: String,
    }

    FinancialGoal {
        user_id: String,
        currency_id: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        target_income: i64,
        metadata_id: String,
    }

    Transaction {
        account_id: String,
        amount: i64,
        date: DateTime<Utc>,
        metadata_id: String,
    }

    Transfer {
        account_from: String,
        account_to: String,
        amount_from: i64,
        amount_to: i64,
        conversion_rate: Option<f64>,
        fee: f64,
        metadata_id: String,
    }

    StableIncome {
        user_id: String,
        currency_id: String,
        amount_per_month: i64,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        last_update_date: DateTime<Utc>,
        metadata_id: String,
    }

    StableIncomeIncome {
        stable_income_id: String,
        account_id: String,
        date: DateTime<Utc>,
        amount: i64,
        metadata_id: String,
    }

    Loan {
        user_id: String,
        currency_id: String,
        principal_amount: i64,
        interest_rate: f64,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
        interest_rate_type: String,
        compounding_frequency: String,
        metadata_id: String,
    }

    LoanPayment {
        loan_id: String,
        account_id: String,
        number: i32,
        date: DateTime<Utc>,
        payment_amount: i64,
        principal_amount: i64,
        remaining_amount: i64,
        payment_type: String,
        metadata_id: String,
    }

    Investment {
        user_id: String,
        currency_id: String,
        r#type: String,
        compounding_frequency: String,
        principal_amount: i64,
        start_date: DateTime<Utc>,
        expected_end_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        risk_level: String,
        expected_return: f64,
        metadata_id: String,
    }

    InvestmentReturn {
        investment_id: String,
        account_id: String,
        date: DateTime<Utc>,
        amount: i64,
        metadata_id: String,
    }

    Metadata {
        title: Option<String>,
        description: Option<String>,
        category: Option<String>,
        tags: Vec<String>,
        tag_exceptions: Vec<String>,
        tag_groups: Vec<String>,
    }

    Preference {
        user_id: String,
        default_currency_id: String,
        language: String,
    }

    AutoDistribution {
        ratio: f64,
        account_id: String,
        metadata_id: String,
        record_metadata_id: String,
    }

    FinancialGoalAutoDistribution {
        financial_goal_id: String,
        auto_distribution_id: String,
    }

    StableIncomeAutoDistribution {
        stable_income_id: String,
        auto_distribution_id: String,
    }

    LoanAutoDistribution {
        loan_id: String,
        auto_distribution_id: String,
    }

    InvestmentAutoDistribution {
        investment_id: String,
        auto_distribution_id: String,
    }

    TransferAutoDistribution {
        metadata_id: String,
        account_to: String,
        auto_distribution_id: String,
    }

    TransactionAutoDistribution {
        auto_distribution_id: String,
    }

    FinancialGoalInvestment {
        financial_goal_id: String,
        date: DateTime<Utc>,
        amount: i64,
        metadata_id: String,
    }
);
