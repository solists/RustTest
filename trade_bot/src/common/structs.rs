use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token_snbx: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Urls {
    pub base_url : String,
    pub base_url_sandbox : String,
    pub snbx_register : String,
    pub snbx_set_cur_bal : String,
    pub snbx_set_pos_bal : String,
    pub snbx_remove : String,
    pub snbx_clear : String,
    pub get_active_orders : String,
    pub create_limit_order : String,
    pub create_market_order : String,
    pub cancel_order : String,
    pub get_client_portfolio : String,
    pub get_client_currencies : String,
    pub get_stocks_list : String,
    pub get_bonds_list : String,
    pub get_etf_list : String,
    pub get_currencies_list : String,
    pub get_candles_list : String,
    pub get_orderbook : String,
    pub get_by_figi : String,
    pub get_by_ticker : String,
    pub get_operations_list : String,
    pub get_user_accounts : String,
}

// This used to sign what structure is actually fits the response json
pub enum ResponseKind {
    Portfolio,
}



// **************************************************
// Structs further are used to serialize responses, 
// Option values needed to prevent errors if some fields are missing
// **************************************************
#[derive(Serialize, Deserialize, Debug)]
pub struct Portfolio {
    pub trackingId: String,
    pub payload:    Positions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Positions {
    pub positions: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub figi:                   Option<String>,
    pub ticker:                 Option<String>,
    pub isin:                   Option<String>,
    pub instrumentType:         Option<String>,
    pub balance:                Option<f32>,
    pub lots:                   Option<f32>,
    pub expectedYield:          ExpectedYield,
    pub averagePositionPrice:   AveragePositionPrice, 
    pub name:                   Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpectedYield {
    pub currency:   Option<String>,
    pub value:      Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AveragePositionPrice {
    pub currency:   Option<String>,
    pub value:      Option<f32>,
}

