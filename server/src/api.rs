use crate::api_datatypes::*;
use actix_web::web::Json;
use actix_web::{post, HttpResponse, Responder};
use RequestBody::*;

#[post("/api")]
pub async fn api_responder(data: Json<Request>) -> impl Responder {
    let mut json = 0;
    match data.0.body {
        User(u) => {}
        Account(a) => {}
        Tag(t) => {}
        FinancialGoal(g) => {}
        Transaction(t) => {}
        Transfer(t) => {}
        StableIncome(i) => {}
        StableIncomeIncome(i) => {}
        Loan(l) => {}
        LoanPayment(p) => {}
        Investment(i) => {}
        InvestmentReturn(r) => {}
        Metadata(m) => {}
        Preference(p) => {}
        AutoDistribution(d) => {}
        FinancialGoalAutoDistribution(d) => {}
        StableIncomeAutoDistribution(d) => {}
        LoanAutoDistribution(d) => {}
        InvestmentAutoDistribution(d) => {}
        TransferAutoDistribution(d) => {}
        TransactionAutoDistribution(d) => {}
        FinancialGoalInvestment(i) => {}
        Creds(_) => {}
    }

    HttpResponse::Ok().json(json)
}
