use rocket::serde::Serialize;
use uuid::Uuid;

/// A valid interest type.
///
/// # Usage
///
/// An `InterestType` specifies whether I am interested on a personal or
/// professional/technical level.
///
/// # Example
///
/// ```rust
/// use crate::models::interests::InterestType;
///
/// fn main() {
///     let interest_type: InterestType = InterestType::Personal;
///
///     fn print_enum(interest_type: &InterestType) {
///         match interest_type {
///             InterestType::Personal => println!("personal"),
///             InterestType::Technical => println!("technical"),
///         }
///     }
///
///     print_enum(&interest_type); // Output: "personal"
/// }
/// ```
// pub enum InterestType {
// Personal,
// Technical,
// }

// impl InterestType {
// fn as_str(&self) -> &'static str {
// match self {
// InterestType::Personal => "personal",
// InterestType::Technical => "technical",
// }
// }
// }

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Interest {
    pub id: Uuid,
    pub interest_type_id: Uuid,
    pub interest: String,
}

/// Model for a response to the interests endpoint.
///
/// # Usage
///
/// # Example
///
/// ```rust
/// use crate::models::interests::InterestResponse;
/// use rocket::serde::json::Json;
///
/// fn main() {
///     let interests: InterestResponse = InterestResponse {
///         personal: vec!["foo"],
///         technical: vec!["bar"]
///     },
///     println!(Json(interests)) // Output: '{"personal": ["foo"], "technical": ["bar"]}'
/// }
/// ```
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct InterestResponse {
    pub technical: Option<Vec<String>>,
    pub personal: Option<Vec<String>>,
}
