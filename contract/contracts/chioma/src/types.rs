use soroban_sdk::{contracttype, Address, Bytes, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgreementStatus {
    Draft,
    Active,
    Completed,
    Terminated,
    Disputed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RentAgreement {
    pub agreement_id: String,
    pub landlord: Address,
    pub tenant: Address,
    pub agent: Option<Address>,
    pub monthly_rent: i128,
    pub security_deposit: i128,
    pub start_date: u64,
    pub end_date: u64,
    pub agent_commission_rate: u32,
    pub status: AgreementStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentRecord {
    pub payment_id: String,
    pub agreement_id: String,
    pub amount: i128,
    pub payment_date: u64,
    pub payer: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserProfile {
    pub account_id: Address,
    pub account_type: u8,
    pub data_hash: Bytes,
    pub last_updated: u64,
    pub is_verified: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Agreement(String),
    AgreementCount,
    Payment(String),
    PaymentCount,
    UserProfile(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AgreementAlreadyExists = 4,
    InvalidAmount = 5,
    InvalidDate = 6,
    InvalidCommissionRate = 7,
    AgreementNotActive = 10,
    PaymentNotFound = 11,
    PaymentFailed = 12,
    // Add these:
    RateLimited = 20,
    ProfileNotFound = 21,
    InvalidAccountType = 22,
    InvalidDataHash = 23,
}