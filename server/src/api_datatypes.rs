use crate::Json;
use actix_surreal_starter::crud_ops::CrudError;
use actix_web::web;
use actix_web::web::ServiceConfig;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use actix_surreal_starter::query_builder::QueryBuilder;
use actix_surreal_starter::UserId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WithId<T> {
    pub id: String,
    pub inner: T,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id(String);

impl<T: DeserializeOwned> WithId<T> {
    pub fn wrap(value: serde_json::Value) -> Option<WithId<T>> {
        let mut obj = value.as_object()?.clone();

        let id = obj.remove("id")?.as_str()?.to_string();
        let inner = serde_json::from_value(serde_json::Value::Object(obj)).ok()?;

        Some(WithId { id, inner })
    }
}

pub trait ApiEntity: Sized + Debug + DeserializeOwned + Serialize + Default + Clone {
    fn paths() -> &'static [&'static str];
    fn table_name() -> &'static str;
    fn query_builder() -> QueryBuilder;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Add { entity: Entity },
    Get { id: String },
    Update { entity: WithId<Entity> },
    Delete { id: String },

    SignUp { user: User },
    SignIn { creds: Creds },
}

pub enum Response {
    Add { id: String },
    Get { entity: Entity },
    GetAll { entities: Vec<WithId<Entity>> },
    Empty,
}


macro_rules! api_entities {
    ($($name:ident($db_table_name:literal$([$($path_to_ownership:literal)*])?) { $($field:ident: $type:ty),*$(,)* })*) => {

        #[derive(Debug, Deserialize, Serialize, Clone)]
        pub enum Entity {
            $($name($name)),*
        }

        pub static PATHS:phf::Map<&'static str, &'static [&'static str]> = phf::phf_map! {
            $($db_table_name => &[$(concat!($($path_to_ownership,)?".user_id"),)*] as &[&str],)*
        };

        pub fn configure_endpoints(cfg: &mut ServiceConfig) {
            cfg
            $(
            .route(concat!("/api/", $db_table_name, "/all"), web::get().to(
                |user_id: UserId| async move {
                    actix_surreal_starter::crud_ops::select_all::<$name>(user_id.0, $name::query_builder()).await.map(web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), web::get().to(
                |id: Json<Id>, user_id: UserId| async move {
                    actix_surreal_starter::crud_ops::select::<$name>(id.0.0, user_id.0, $name::query_builder()).await.map(web::Json)
                }
            ))
            .route(concat!("/api/", $db_table_name), web::post().to(
                |entity: Json<$name>, user_id: UserId| async move {
                    actix_surreal_starter::crud_ops::insert(entity.0, user_id.0, $name::query_builder()).await
                }
            ))
            .route(concat!("/api/", $db_table_name), web::put().to(
                |entity: Json<WithId<serde_json::Value>>, user_id: UserId| async move {
                    actix_surreal_starter::crud_ops::update(entity.0.id, entity.0.inner, user_id.0, $name::query_builder()).await
                }
            ))
            .route(concat!("/api/", $db_table_name), web::delete().to(
                |id: Json<Id>, user_id: UserId| async move {
                    actix_surreal_starter::crud_ops::delete(id.0.0, user_id.0, $name::query_builder()).await
                }
            ))
            )*;
        }

        $(
        #[derive(Debug, Deserialize, Serialize, Default, Clone)]
        pub struct $name {
            $(pub $field: $type),*
        }

        impl ApiEntity for $name {
            fn paths() -> &'static [&'static str] {
                PATHS.get($db_table_name).unwrap()
            }
            fn table_name() -> &'static str {
                $db_table_name
            }
            fn query_builder() -> QueryBuilder {
                QueryBuilder {
                    paths: Self::paths(),
                    table_name: Self::table_name(),
                    fkey_path_map: None, //TODO: nah oh it can't be None it's just a placeholder
                }
            }
        }
        )*
    };
}

api_entities!(
    User("users") {
        email: String,
        username: String,
        password: String,
        registration_date: DateTime<Utc>,
        selected_preference: Option<String>,
    }

    Account("accounts") {
        title: String,
        user_id: String,
        currency_id: String,
        balance: i64,
    }

    Creds("creds") {
        email: String,
        password: String,
    }

    Tag("tags") {
        user_id: String,
        metadata_id: String,
    }

    FinancialGoal("financial_goals") {
        user_id: String,
        currency_id: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        target_income: i64,
        metadata_id: String,
    }

    Transaction("transactions" ["account_id"]) {
        account_id: String,
        amount: i64,
        date: DateTime<Utc>,
        metadata_id: String,
    }

    Transfer("transfers" ["account_id"]) {
        account_from: String,
        account_to: String,
        amount_from: i64,
        amount_to: i64,
        conversion_rate: Option<f64>,
        fee: f64,
        metadata_id: String,
    }

    StableIncome("stable_incomes") {
        user_id: String,
        currency_id: String,
        amount_per_month: i64,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        last_update_date: DateTime<Utc>,
        metadata_id: String,
    }

    StableIncomeIncome("stable_income_incomes") {
        stable_income_id: String,
        account_id: String,
        date: DateTime<Utc>,
        amount: i64,
        metadata_id: String,
    }

    Loan("loans") {
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

    LoanPayment("loan_payments") {
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

    Investment("investments") {
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

    InvestmentReturn("investment_returns") {
        investment_id: String,
        account_id: String,
        date: DateTime<Utc>,
        amount: i64,
        metadata_id: String,
    }

    Metadata("metadata") {
        title: Option<String>,
        description: Option<String>,
        category: Option<String>,
        tags: Vec<String>,
        tag_exceptions: Vec<String>,
        tag_groups: Vec<String>,
    }

    Preference("preferences") {
        user_id: String,
        default_currency_id: String,
        language: String,
    }

    AutoDistribution("auto_distributions") {
        ratio: f64,
        account_id: String,
        metadata_id: String,
        record_metadata_id: String,
    }

    FinancialGoalAutoDistribution("financial_goal_auto_distributions") {
        financial_goal_id: String,
        auto_distribution_id: String,
    }

    StableIncomeAutoDistribution("stable_income_auto_distributions") {
        stable_income_id: String,
        auto_distribution_id: String,
    }

    LoanAutoDistribution("loan_auto_distributions") {
        loan_id: String,
        auto_distribution_id: String,
    }

    InvestmentAutoDistribution("investment_auto_distributions") {
        investment_id: String,
        auto_distribution_id: String,
    }

    TransferAutoDistribution("transfer_auto_distributions") {
        metadata_id: String,
        account_to: String,
        auto_distribution_id: String,
    }

    TransactionAutoDistribution("transaction_auto_distributions") {
        auto_distribution_id: String,
    }

    FinancialGoalInvestment("financial_goal_investments") {
        financial_goal_id: String,
        date: DateTime<Utc>,
        amount: i64,
        metadata_id: String,
    }
);
