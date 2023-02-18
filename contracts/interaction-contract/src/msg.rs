use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExecuteMsg{}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryMsg{}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryResponse{
   pub response: String
}