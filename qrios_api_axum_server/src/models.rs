#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[allow(dead_code)]
fn from_validation_error(e: validator::ValidationError) -> validator::ValidationErrors {
  let mut errs = validator::ValidationErrors::new();
  errs.add("na", e);
  errs
}

#[allow(dead_code)]
pub fn check_xss_string(v: &str) -> std::result::Result<(), validator::ValidationError> {
    if ammonia::is_html(v) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_vec_string(v: &[String]) -> std::result::Result<(), validator::ValidationError> {
    if v.iter().any(|i| ammonia::is_html(i)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_string(
    v: &std::collections::HashMap<String, String>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| ammonia::is_html(v)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_nested<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError>
where
    T: validator::Validate,
{
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| v.validate().is_err()) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map<T>(v: &std::collections::HashMap<String, T>) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostUssdsessioneventAbortHeaderParams {
        pub authorization: Option<String>,
    }



    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostUssdsessioneventCloseHeaderParams {
        pub authorization: Option<String>,
    }



    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostUssdsessioneventContinueHeaderParams {
        pub authorization: Option<String>,
    }



    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct PostUssdsessioneventNewHeaderParams {
        pub authorization: Option<String>,
    }




/// The event sent when the USSD session is aborted (internal error).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AbortSession {
    /// Client identifier of the developer using Qrios platform.
    #[serde(rename = "clientId")]
          #[validate(custom(function = "check_xss_string"))]
    pub client_id: String,

    /// Identifier of the USSD app created on Qrios platform.
    #[serde(rename = "appId")]
          #[validate(custom(function = "check_xss_string"))]
    pub app_id: String,

    /// Unique identifier of USSD session.
    #[serde(rename = "sessionId")]
          #[validate(custom(function = "check_xss_string"))]
    pub session_id: String,

    #[serde(rename = "reason")]
          #[validate(nested)]
    pub reason: models::AbortSessionAbortReason,

}



impl AbortSession {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(client_id: String, app_id: String, session_id: String, reason: models::AbortSessionAbortReason, ) -> AbortSession {
        AbortSession {
 client_id,
 app_id,
 session_id,
 reason,
        }
    }
}

/// Converts the AbortSession value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AbortSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("clientId".to_string()),
            Some(self.client_id.to_string()),


            Some("appId".to_string()),
            Some(self.app_id.to_string()),


            Some("sessionId".to_string()),
            Some(self.session_id.to_string()),

            // Skipping reason in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AbortSession value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AbortSession {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub client_id: Vec<String>,
            pub app_id: Vec<String>,
            pub session_id: Vec<String>,
            pub reason: Vec<models::AbortSessionAbortReason>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AbortSession".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "clientId" => intermediate_rep.client_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "appId" => intermediate_rep.app_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sessionId" => intermediate_rep.session_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reason" => intermediate_rep.reason.push(<models::AbortSessionAbortReason as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AbortSession".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AbortSession {
            client_id: intermediate_rep.client_id.into_iter().next().ok_or_else(|| "clientId missing in AbortSession".to_string())?,
            app_id: intermediate_rep.app_id.into_iter().next().ok_or_else(|| "appId missing in AbortSession".to_string())?,
            session_id: intermediate_rep.session_id.into_iter().next().ok_or_else(|| "sessionId missing in AbortSession".to_string())?,
            reason: intermediate_rep.reason.into_iter().next().ok_or_else(|| "reason missing in AbortSession".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AbortSession> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AbortSession>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AbortSession>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for AbortSession - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AbortSession> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AbortSession as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into AbortSession - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum AbortSessionAbortReason {
    #[serde(alias = "DuplicatedOperation")]
    AbortSessionAbortReasonDuplicatedOperation(models::AbortSessionAbortReasonDuplicatedOperation),
    #[serde(alias = "InsufficientBalanceInVirtualPurse")]
    AbortSessionAbortReasonInsufficientBalanceInVirtualPurse(models::AbortSessionAbortReasonInsufficientBalanceInVirtualPurse),
    #[serde(alias = "InternalError")]
    AbortSessionAbortReasonInternalError(models::AbortSessionAbortReasonInternalError),
    #[serde(alias = "MissingPrivilege")]
    AbortSessionAbortReasonMissingPrivilege(models::AbortSessionAbortReasonMissingPrivilege),
    #[serde(alias = "UnexpectedUssdAppResponse")]
    AbortSessionAbortReasonUnexpectedUssdAppResponse(models::AbortSessionAbortReasonUnexpectedUssdAppResponse),
}

impl validator::Validate for AbortSessionAbortReason
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::AbortSessionAbortReasonDuplicatedOperation(v) => v.validate(),
            Self::AbortSessionAbortReasonInsufficientBalanceInVirtualPurse(v) => v.validate(),
            Self::AbortSessionAbortReasonInternalError(v) => v.validate(),
            Self::AbortSessionAbortReasonMissingPrivilege(v) => v.validate(),
            Self::AbortSessionAbortReasonUnexpectedUssdAppResponse(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AbortSessionAbortReason value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AbortSessionAbortReason {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl serde::Serialize for AbortSessionAbortReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::AbortSessionAbortReasonDuplicatedOperation(x) => x.serialize(serializer),
                Self::AbortSessionAbortReasonInsufficientBalanceInVirtualPurse(x) => x.serialize(serializer),
                Self::AbortSessionAbortReasonInternalError(x) => x.serialize(serializer),
                Self::AbortSessionAbortReasonMissingPrivilege(x) => x.serialize(serializer),
                Self::AbortSessionAbortReasonUnexpectedUssdAppResponse(x) => x.serialize(serializer),
            }
    }
}

impl From<models::AbortSessionAbortReasonDuplicatedOperation> for AbortSessionAbortReason {
    fn from(value: models::AbortSessionAbortReasonDuplicatedOperation) -> Self {
        Self::AbortSessionAbortReasonDuplicatedOperation(value)
    }
}
impl From<models::AbortSessionAbortReasonInsufficientBalanceInVirtualPurse> for AbortSessionAbortReason {
    fn from(value: models::AbortSessionAbortReasonInsufficientBalanceInVirtualPurse) -> Self {
        Self::AbortSessionAbortReasonInsufficientBalanceInVirtualPurse(value)
    }
}
impl From<models::AbortSessionAbortReasonInternalError> for AbortSessionAbortReason {
    fn from(value: models::AbortSessionAbortReasonInternalError) -> Self {
        Self::AbortSessionAbortReasonInternalError(value)
    }
}
impl From<models::AbortSessionAbortReasonMissingPrivilege> for AbortSessionAbortReason {
    fn from(value: models::AbortSessionAbortReasonMissingPrivilege) -> Self {
        Self::AbortSessionAbortReasonMissingPrivilege(value)
    }
}
impl From<models::AbortSessionAbortReasonUnexpectedUssdAppResponse> for AbortSessionAbortReason {
    fn from(value: models::AbortSessionAbortReasonUnexpectedUssdAppResponse) -> Self {
        Self::AbortSessionAbortReasonUnexpectedUssdAppResponse(value)
    }
}





/// USSD process abort reason - operation, eg. Merchant Payment, was previously invoked with given identifier.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AbortSessionAbortReasonDuplicatedOperation {
    #[serde(rename = "operationId")]
          #[validate(custom(function = "check_xss_string"))]
    pub operation_id: String,

    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl AbortSessionAbortReasonDuplicatedOperation {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(operation_id: String, r_type: String, ) -> AbortSessionAbortReasonDuplicatedOperation {
        AbortSessionAbortReasonDuplicatedOperation {
 operation_id,
 r_type,
        }
    }
}

/// Converts the AbortSessionAbortReasonDuplicatedOperation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AbortSessionAbortReasonDuplicatedOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("operationId".to_string()),
            Some(self.operation_id.to_string()),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AbortSessionAbortReasonDuplicatedOperation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AbortSessionAbortReasonDuplicatedOperation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub operation_id: Vec<String>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AbortSessionAbortReasonDuplicatedOperation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "operationId" => intermediate_rep.operation_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AbortSessionAbortReasonDuplicatedOperation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AbortSessionAbortReasonDuplicatedOperation {
            operation_id: intermediate_rep.operation_id.into_iter().next().ok_or_else(|| "operationId missing in AbortSessionAbortReasonDuplicatedOperation".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in AbortSessionAbortReasonDuplicatedOperation".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AbortSessionAbortReasonDuplicatedOperation> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AbortSessionAbortReasonDuplicatedOperation>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AbortSessionAbortReasonDuplicatedOperation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for AbortSessionAbortReasonDuplicatedOperation - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AbortSessionAbortReasonDuplicatedOperation> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AbortSessionAbortReasonDuplicatedOperation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into AbortSessionAbortReasonDuplicatedOperation - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - not enough funds to fulfil the request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
        AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
 r_type,
        }
    }
}

/// Converts the AbortSessionAbortReasonInsufficientBalanceInVirtualPurse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AbortSessionAbortReasonInsufficientBalanceInVirtualPurse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AbortSessionAbortReasonInsufficientBalanceInVirtualPurse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AbortSessionAbortReasonInsufficientBalanceInVirtualPurse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in AbortSessionAbortReasonInsufficientBalanceInVirtualPurse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AbortSessionAbortReasonInsufficientBalanceInVirtualPurse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AbortSessionAbortReasonInsufficientBalanceInVirtualPurse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AbortSessionAbortReasonInsufficientBalanceInVirtualPurse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for AbortSessionAbortReasonInsufficientBalanceInVirtualPurse - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AbortSessionAbortReasonInsufficientBalanceInVirtualPurse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AbortSessionAbortReasonInsufficientBalanceInVirtualPurse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into AbortSessionAbortReasonInsufficientBalanceInVirtualPurse - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - there was some internal error in Qrios API.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AbortSessionAbortReasonInternalError {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl AbortSessionAbortReasonInternalError {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> AbortSessionAbortReasonInternalError {
        AbortSessionAbortReasonInternalError {
 r_type,
        }
    }
}

/// Converts the AbortSessionAbortReasonInternalError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AbortSessionAbortReasonInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AbortSessionAbortReasonInternalError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AbortSessionAbortReasonInternalError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AbortSessionAbortReasonInternalError".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AbortSessionAbortReasonInternalError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AbortSessionAbortReasonInternalError {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in AbortSessionAbortReasonInternalError".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AbortSessionAbortReasonInternalError> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AbortSessionAbortReasonInternalError>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AbortSessionAbortReasonInternalError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for AbortSessionAbortReasonInternalError - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AbortSessionAbortReasonInternalError> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AbortSessionAbortReasonInternalError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into AbortSessionAbortReasonInternalError - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - there are no required privileges to run a USSD operation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AbortSessionAbortReasonMissingPrivilege {
    #[serde(rename = "privilege")]
          #[validate(custom(function = "check_xss_string"))]
    pub privilege: String,

    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl AbortSessionAbortReasonMissingPrivilege {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(privilege: String, r_type: String, ) -> AbortSessionAbortReasonMissingPrivilege {
        AbortSessionAbortReasonMissingPrivilege {
 privilege,
 r_type,
        }
    }
}

/// Converts the AbortSessionAbortReasonMissingPrivilege value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AbortSessionAbortReasonMissingPrivilege {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("privilege".to_string()),
            Some(self.privilege.to_string()),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AbortSessionAbortReasonMissingPrivilege value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AbortSessionAbortReasonMissingPrivilege {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub privilege: Vec<String>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AbortSessionAbortReasonMissingPrivilege".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "privilege" => intermediate_rep.privilege.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AbortSessionAbortReasonMissingPrivilege".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AbortSessionAbortReasonMissingPrivilege {
            privilege: intermediate_rep.privilege.into_iter().next().ok_or_else(|| "privilege missing in AbortSessionAbortReasonMissingPrivilege".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in AbortSessionAbortReasonMissingPrivilege".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AbortSessionAbortReasonMissingPrivilege> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AbortSessionAbortReasonMissingPrivilege>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AbortSessionAbortReasonMissingPrivilege>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for AbortSessionAbortReasonMissingPrivilege - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AbortSessionAbortReasonMissingPrivilege> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AbortSessionAbortReasonMissingPrivilege as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into AbortSessionAbortReasonMissingPrivilege - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - previous USSD app response was malformed or not expected.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AbortSessionAbortReasonUnexpectedUssdAppResponse {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl AbortSessionAbortReasonUnexpectedUssdAppResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> AbortSessionAbortReasonUnexpectedUssdAppResponse {
        AbortSessionAbortReasonUnexpectedUssdAppResponse {
 r_type,
        }
    }
}

/// Converts the AbortSessionAbortReasonUnexpectedUssdAppResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for AbortSessionAbortReasonUnexpectedUssdAppResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AbortSessionAbortReasonUnexpectedUssdAppResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AbortSessionAbortReasonUnexpectedUssdAppResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AbortSessionAbortReasonUnexpectedUssdAppResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AbortSessionAbortReasonUnexpectedUssdAppResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AbortSessionAbortReasonUnexpectedUssdAppResponse {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in AbortSessionAbortReasonUnexpectedUssdAppResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AbortSessionAbortReasonUnexpectedUssdAppResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<AbortSessionAbortReasonUnexpectedUssdAppResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AbortSessionAbortReasonUnexpectedUssdAppResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for AbortSessionAbortReasonUnexpectedUssdAppResponse - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<AbortSessionAbortReasonUnexpectedUssdAppResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AbortSessionAbortReasonUnexpectedUssdAppResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into AbortSessionAbortReasonUnexpectedUssdAppResponse - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// The event sent when the USSD session is closed (user can no longer input data, user leaves the session, session times out).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloseSession {
    /// Client identifier of the developer using Qrios platform.
    #[serde(rename = "clientId")]
          #[validate(custom(function = "check_xss_string"))]
    pub client_id: String,

    /// Identifier of the USSD app created on Qrios platform.
    #[serde(rename = "appId")]
          #[validate(custom(function = "check_xss_string"))]
    pub app_id: String,

    /// Unique identifier of USSD session.
    #[serde(rename = "sessionId")]
          #[validate(custom(function = "check_xss_string"))]
    pub session_id: String,

    #[serde(rename = "reason")]
          #[validate(nested)]
    pub reason: models::CloseSessionCloseReason,

    /// String set by the developer and carried over throughout the session.
    #[serde(rename = "contextData")]
          #[validate(custom(function = "check_xss_string"))]
    pub context_data: String,

}



impl CloseSession {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(client_id: String, app_id: String, session_id: String, reason: models::CloseSessionCloseReason, context_data: String, ) -> CloseSession {
        CloseSession {
 client_id,
 app_id,
 session_id,
 reason,
 context_data,
        }
    }
}

/// Converts the CloseSession value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CloseSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("clientId".to_string()),
            Some(self.client_id.to_string()),


            Some("appId".to_string()),
            Some(self.app_id.to_string()),


            Some("sessionId".to_string()),
            Some(self.session_id.to_string()),

            // Skipping reason in query parameter serialization


            Some("contextData".to_string()),
            Some(self.context_data.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloseSession value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloseSession {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub client_id: Vec<String>,
            pub app_id: Vec<String>,
            pub session_id: Vec<String>,
            pub reason: Vec<models::CloseSessionCloseReason>,
            pub context_data: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloseSession".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "clientId" => intermediate_rep.client_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "appId" => intermediate_rep.app_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sessionId" => intermediate_rep.session_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reason" => intermediate_rep.reason.push(<models::CloseSessionCloseReason as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contextData" => intermediate_rep.context_data.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloseSession".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloseSession {
            client_id: intermediate_rep.client_id.into_iter().next().ok_or_else(|| "clientId missing in CloseSession".to_string())?,
            app_id: intermediate_rep.app_id.into_iter().next().ok_or_else(|| "appId missing in CloseSession".to_string())?,
            session_id: intermediate_rep.session_id.into_iter().next().ok_or_else(|| "sessionId missing in CloseSession".to_string())?,
            reason: intermediate_rep.reason.into_iter().next().ok_or_else(|| "reason missing in CloseSession".to_string())?,
            context_data: intermediate_rep.context_data.into_iter().next().ok_or_else(|| "contextData missing in CloseSession".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloseSession> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CloseSession>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloseSession>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for CloseSession - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CloseSession> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloseSession as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into CloseSession - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum CloseSessionCloseReason {
    #[serde(alias = "Abandon")]
    CloseSessionCloseReasonAbandon(models::CloseSessionCloseReasonAbandon),
    #[serde(alias = "End")]
    CloseSessionCloseReasonEnd(models::CloseSessionCloseReasonEnd),
    #[serde(alias = "Timeout")]
    CloseSessionCloseReasonTimeout(models::CloseSessionCloseReasonTimeout),
}

impl validator::Validate for CloseSessionCloseReason
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::CloseSessionCloseReasonAbandon(v) => v.validate(),
            Self::CloseSessionCloseReasonEnd(v) => v.validate(),
            Self::CloseSessionCloseReasonTimeout(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloseSessionCloseReason value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloseSessionCloseReason {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl serde::Serialize for CloseSessionCloseReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::CloseSessionCloseReasonAbandon(x) => x.serialize(serializer),
                Self::CloseSessionCloseReasonEnd(x) => x.serialize(serializer),
                Self::CloseSessionCloseReasonTimeout(x) => x.serialize(serializer),
            }
    }
}

impl From<models::CloseSessionCloseReasonAbandon> for CloseSessionCloseReason {
    fn from(value: models::CloseSessionCloseReasonAbandon) -> Self {
        Self::CloseSessionCloseReasonAbandon(value)
    }
}
impl From<models::CloseSessionCloseReasonEnd> for CloseSessionCloseReason {
    fn from(value: models::CloseSessionCloseReasonEnd) -> Self {
        Self::CloseSessionCloseReasonEnd(value)
    }
}
impl From<models::CloseSessionCloseReasonTimeout> for CloseSessionCloseReason {
    fn from(value: models::CloseSessionCloseReasonTimeout) -> Self {
        Self::CloseSessionCloseReasonTimeout(value)
    }
}





/// Abandon - the user leaves the session (e.g. the user presses \"Cancel\" on the phone).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloseSessionCloseReasonAbandon {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl CloseSessionCloseReasonAbandon {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> CloseSessionCloseReasonAbandon {
        CloseSessionCloseReasonAbandon {
 r_type,
        }
    }
}

/// Converts the CloseSessionCloseReasonAbandon value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CloseSessionCloseReasonAbandon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloseSessionCloseReasonAbandon value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloseSessionCloseReasonAbandon {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloseSessionCloseReasonAbandon".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloseSessionCloseReasonAbandon".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloseSessionCloseReasonAbandon {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in CloseSessionCloseReasonAbandon".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloseSessionCloseReasonAbandon> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CloseSessionCloseReasonAbandon>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloseSessionCloseReasonAbandon>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for CloseSessionCloseReasonAbandon - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CloseSessionCloseReasonAbandon> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloseSessionCloseReasonAbandon as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into CloseSessionCloseReasonAbandon - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// End - the session ends naturally (e.g. the user can no longer input data).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloseSessionCloseReasonEnd {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl CloseSessionCloseReasonEnd {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> CloseSessionCloseReasonEnd {
        CloseSessionCloseReasonEnd {
 r_type,
        }
    }
}

/// Converts the CloseSessionCloseReasonEnd value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CloseSessionCloseReasonEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloseSessionCloseReasonEnd value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloseSessionCloseReasonEnd {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloseSessionCloseReasonEnd".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloseSessionCloseReasonEnd".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloseSessionCloseReasonEnd {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in CloseSessionCloseReasonEnd".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloseSessionCloseReasonEnd> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CloseSessionCloseReasonEnd>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloseSessionCloseReasonEnd>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for CloseSessionCloseReasonEnd - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CloseSessionCloseReasonEnd> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloseSessionCloseReasonEnd as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into CloseSessionCloseReasonEnd - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Timeout - the session is ended by the mobile operator (e.g. after two minutes).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CloseSessionCloseReasonTimeout {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl CloseSessionCloseReasonTimeout {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> CloseSessionCloseReasonTimeout {
        CloseSessionCloseReasonTimeout {
 r_type,
        }
    }
}

/// Converts the CloseSessionCloseReasonTimeout value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CloseSessionCloseReasonTimeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CloseSessionCloseReasonTimeout value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CloseSessionCloseReasonTimeout {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CloseSessionCloseReasonTimeout".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CloseSessionCloseReasonTimeout".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CloseSessionCloseReasonTimeout {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in CloseSessionCloseReasonTimeout".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CloseSessionCloseReasonTimeout> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CloseSessionCloseReasonTimeout>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CloseSessionCloseReasonTimeout>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for CloseSessionCloseReasonTimeout - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CloseSessionCloseReasonTimeout> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CloseSessionCloseReasonTimeout as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into CloseSessionCloseReasonTimeout - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// The event sent when the USSD session continues (input is received or process ends).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContinueSession {
    /// Client identifier of the developer using Qrios platform.
    #[serde(rename = "clientId")]
          #[validate(custom(function = "check_xss_string"))]
    pub client_id: String,

    /// Identifier of the USSD app created on Qrios platform.
    #[serde(rename = "appId")]
          #[validate(custom(function = "check_xss_string"))]
    pub app_id: String,

    /// Unique identifier of USSD session.
    #[serde(rename = "sessionId")]
          #[validate(custom(function = "check_xss_string"))]
    pub session_id: String,

    #[serde(rename = "result")]
          #[validate(nested)]
    pub result: models::UssdActionResult,

    /// String set by the developer and carried over throughout the session.
    #[serde(rename = "contextData")]
          #[validate(custom(function = "check_xss_string"))]
    pub context_data: String,

}



impl ContinueSession {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(client_id: String, app_id: String, session_id: String, result: models::UssdActionResult, context_data: String, ) -> ContinueSession {
        ContinueSession {
 client_id,
 app_id,
 session_id,
 result,
 context_data,
        }
    }
}

/// Converts the ContinueSession value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ContinueSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("clientId".to_string()),
            Some(self.client_id.to_string()),


            Some("appId".to_string()),
            Some(self.app_id.to_string()),


            Some("sessionId".to_string()),
            Some(self.session_id.to_string()),

            // Skipping result in query parameter serialization


            Some("contextData".to_string()),
            Some(self.context_data.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContinueSession value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContinueSession {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub client_id: Vec<String>,
            pub app_id: Vec<String>,
            pub session_id: Vec<String>,
            pub result: Vec<models::UssdActionResult>,
            pub context_data: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContinueSession".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "clientId" => intermediate_rep.client_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "appId" => intermediate_rep.app_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sessionId" => intermediate_rep.session_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "result" => intermediate_rep.result.push(<models::UssdActionResult as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contextData" => intermediate_rep.context_data.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContinueSession".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContinueSession {
            client_id: intermediate_rep.client_id.into_iter().next().ok_or_else(|| "clientId missing in ContinueSession".to_string())?,
            app_id: intermediate_rep.app_id.into_iter().next().ok_or_else(|| "appId missing in ContinueSession".to_string())?,
            session_id: intermediate_rep.session_id.into_iter().next().ok_or_else(|| "sessionId missing in ContinueSession".to_string())?,
            result: intermediate_rep.result.into_iter().next().ok_or_else(|| "result missing in ContinueSession".to_string())?,
            context_data: intermediate_rep.context_data.into_iter().next().ok_or_else(|| "contextData missing in ContinueSession".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContinueSession> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ContinueSession>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContinueSession>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ContinueSession - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ContinueSession> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContinueSession as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ContinueSession - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Url of the Legacy USSD App
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LegacyAppRedirectUri(pub String);

impl validator::Validate for LegacyAppRedirectUri {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for LegacyAppRedirectUri {
    fn from(x: String) -> Self {
        LegacyAppRedirectUri(x)
    }
}

impl std::fmt::Display for LegacyAppRedirectUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for LegacyAppRedirectUri {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(LegacyAppRedirectUri(x.to_string()))
    }
}

impl std::convert::From<LegacyAppRedirectUri> for String {
    fn from(x: LegacyAppRedirectUri) -> Self {
        x.0
    }
}

impl std::ops::Deref for LegacyAppRedirectUri {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for LegacyAppRedirectUri {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// Url of the USSD API
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LegacyAppRedirectUssdApiUrl(pub String);

impl validator::Validate for LegacyAppRedirectUssdApiUrl {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for LegacyAppRedirectUssdApiUrl {
    fn from(x: String) -> Self {
        LegacyAppRedirectUssdApiUrl(x)
    }
}

impl std::fmt::Display for LegacyAppRedirectUssdApiUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for LegacyAppRedirectUssdApiUrl {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(LegacyAppRedirectUssdApiUrl(x.to_string()))
    }
}

impl std::convert::From<LegacyAppRedirectUssdApiUrl> for String {
    fn from(x: LegacyAppRedirectUssdApiUrl) -> Self {
        x.0
    }
}

impl std::ops::Deref for LegacyAppRedirectUssdApiUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for LegacyAppRedirectUssdApiUrl {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum MerchantPaymentProcessExecutionMode {
    #[serde(alias = "WithBankResponseTimeout")]
    MerchantPaymentProcessExecutionModeWithBankResponseTimeout(models::MerchantPaymentProcessExecutionModeWithBankResponseTimeout),
    #[serde(alias = "WithoutWaitingForBank")]
    MerchantPaymentProcessExecutionModeWithoutWaitingForBank(models::MerchantPaymentProcessExecutionModeWithoutWaitingForBank),
}

impl validator::Validate for MerchantPaymentProcessExecutionMode
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::MerchantPaymentProcessExecutionModeWithBankResponseTimeout(v) => v.validate(),
            Self::MerchantPaymentProcessExecutionModeWithoutWaitingForBank(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcessExecutionMode value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcessExecutionMode {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl serde::Serialize for MerchantPaymentProcessExecutionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::MerchantPaymentProcessExecutionModeWithBankResponseTimeout(x) => x.serialize(serializer),
                Self::MerchantPaymentProcessExecutionModeWithoutWaitingForBank(x) => x.serialize(serializer),
            }
    }
}

impl From<models::MerchantPaymentProcessExecutionModeWithBankResponseTimeout> for MerchantPaymentProcessExecutionMode {
    fn from(value: models::MerchantPaymentProcessExecutionModeWithBankResponseTimeout) -> Self {
        Self::MerchantPaymentProcessExecutionModeWithBankResponseTimeout(value)
    }
}
impl From<models::MerchantPaymentProcessExecutionModeWithoutWaitingForBank> for MerchantPaymentProcessExecutionMode {
    fn from(value: models::MerchantPaymentProcessExecutionModeWithoutWaitingForBank) -> Self {
        Self::MerchantPaymentProcessExecutionModeWithoutWaitingForBank(value)
    }
}





/// Merchant payment process will use a timeout when finalizing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
    #[serde(rename = "millis")]
    pub millis: i64,

    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(millis: i64, r_type: String, ) -> MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
        MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
 millis,
 r_type,
        }
    }
}

/// Converts the MerchantPaymentProcessExecutionModeWithBankResponseTimeout value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("millis".to_string()),
            Some(self.millis.to_string()),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcessExecutionModeWithBankResponseTimeout value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub millis: Vec<i64>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentProcessExecutionModeWithBankResponseTimeout".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "millis" => intermediate_rep.millis.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentProcessExecutionModeWithBankResponseTimeout".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
            millis: intermediate_rep.millis.into_iter().next().ok_or_else(|| "millis missing in MerchantPaymentProcessExecutionModeWithBankResponseTimeout".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentProcessExecutionModeWithBankResponseTimeout".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithBankResponseTimeout> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithBankResponseTimeout>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithBankResponseTimeout>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentProcessExecutionModeWithBankResponseTimeout - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithBankResponseTimeout> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentProcessExecutionModeWithBankResponseTimeout as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentProcessExecutionModeWithBankResponseTimeout - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Merchant payment process will not use a timeout when finalizing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
        MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
 r_type,
        }
    }
}

/// Converts the MerchantPaymentProcessExecutionModeWithoutWaitingForBank value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcessExecutionModeWithoutWaitingForBank value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentProcessExecutionModeWithoutWaitingForBank".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentProcessExecutionModeWithoutWaitingForBank".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentProcessExecutionModeWithoutWaitingForBank".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithoutWaitingForBank> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithoutWaitingForBank>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithoutWaitingForBank>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentProcessExecutionModeWithoutWaitingForBank - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentProcessExecutionModeWithoutWaitingForBank> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentProcessExecutionModeWithoutWaitingForBank as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentProcessExecutionModeWithoutWaitingForBank - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum MerchantPaymentProcessPaymentMode {
    #[serde(alias = "FixedAccount")]
    MerchantPaymentProcessPaymentModeFixedAccount(models::MerchantPaymentProcessPaymentModeFixedAccount),
    #[serde(alias = "FixedBank")]
    MerchantPaymentProcessPaymentModeFixedBank(models::MerchantPaymentProcessPaymentModeFixedBank),
    #[serde(alias = "Flexible")]
    MerchantPaymentProcessPaymentModeFlexible(models::MerchantPaymentProcessPaymentModeFlexible),
}

impl validator::Validate for MerchantPaymentProcessPaymentMode
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::MerchantPaymentProcessPaymentModeFixedAccount(v) => v.validate(),
            Self::MerchantPaymentProcessPaymentModeFixedBank(v) => v.validate(),
            Self::MerchantPaymentProcessPaymentModeFlexible(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcessPaymentMode value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcessPaymentMode {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl serde::Serialize for MerchantPaymentProcessPaymentMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::MerchantPaymentProcessPaymentModeFixedAccount(x) => x.serialize(serializer),
                Self::MerchantPaymentProcessPaymentModeFixedBank(x) => x.serialize(serializer),
                Self::MerchantPaymentProcessPaymentModeFlexible(x) => x.serialize(serializer),
            }
    }
}

impl From<models::MerchantPaymentProcessPaymentModeFixedAccount> for MerchantPaymentProcessPaymentMode {
    fn from(value: models::MerchantPaymentProcessPaymentModeFixedAccount) -> Self {
        Self::MerchantPaymentProcessPaymentModeFixedAccount(value)
    }
}
impl From<models::MerchantPaymentProcessPaymentModeFixedBank> for MerchantPaymentProcessPaymentMode {
    fn from(value: models::MerchantPaymentProcessPaymentModeFixedBank) -> Self {
        Self::MerchantPaymentProcessPaymentModeFixedBank(value)
    }
}
impl From<models::MerchantPaymentProcessPaymentModeFlexible> for MerchantPaymentProcessPaymentMode {
    fn from(value: models::MerchantPaymentProcessPaymentModeFlexible) -> Self {
        Self::MerchantPaymentProcessPaymentModeFlexible(value)
    }
}





/// Fixed account mode in merchant payment process, with account ID specified.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentProcessPaymentModeFixedAccount {
    /// Number of the account which will be used in transaction. Account numbers can be obtained with Qrios API using `/merchants/accounts` endpoint.
    #[serde(rename = "accountNumber")]
          #[validate(custom(function = "check_xss_string"))]
    pub account_number: String,

    /// Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
    #[serde(rename = "bank")]
          #[validate(custom(function = "check_xss_string"))]
    pub bank: String,

    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentProcessPaymentModeFixedAccount {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(account_number: String, bank: String, r_type: String, ) -> MerchantPaymentProcessPaymentModeFixedAccount {
        MerchantPaymentProcessPaymentModeFixedAccount {
 account_number,
 bank,
 r_type,
        }
    }
}

/// Converts the MerchantPaymentProcessPaymentModeFixedAccount value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentProcessPaymentModeFixedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("accountNumber".to_string()),
            Some(self.account_number.to_string()),


            Some("bank".to_string()),
            Some(self.bank.to_string()),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcessPaymentModeFixedAccount value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcessPaymentModeFixedAccount {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub account_number: Vec<String>,
            pub bank: Vec<String>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentProcessPaymentModeFixedAccount".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "accountNumber" => intermediate_rep.account_number.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "bank" => intermediate_rep.bank.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentProcessPaymentModeFixedAccount".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentProcessPaymentModeFixedAccount {
            account_number: intermediate_rep.account_number.into_iter().next().ok_or_else(|| "accountNumber missing in MerchantPaymentProcessPaymentModeFixedAccount".to_string())?,
            bank: intermediate_rep.bank.into_iter().next().ok_or_else(|| "bank missing in MerchantPaymentProcessPaymentModeFixedAccount".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentProcessPaymentModeFixedAccount".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedAccount> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedAccount>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedAccount>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentProcessPaymentModeFixedAccount - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedAccount> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentProcessPaymentModeFixedAccount as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentProcessPaymentModeFixedAccount - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Fixed bank mode in merchant payment process, with bank ID specified. The account from provided bank will then be selected within the process.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentProcessPaymentModeFixedBank {
    /// Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
    #[serde(rename = "bank")]
          #[validate(custom(function = "check_xss_string"))]
    pub bank: String,

    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentProcessPaymentModeFixedBank {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(bank: String, r_type: String, ) -> MerchantPaymentProcessPaymentModeFixedBank {
        MerchantPaymentProcessPaymentModeFixedBank {
 bank,
 r_type,
        }
    }
}

/// Converts the MerchantPaymentProcessPaymentModeFixedBank value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentProcessPaymentModeFixedBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("bank".to_string()),
            Some(self.bank.to_string()),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcessPaymentModeFixedBank value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcessPaymentModeFixedBank {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub bank: Vec<String>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentProcessPaymentModeFixedBank".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "bank" => intermediate_rep.bank.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentProcessPaymentModeFixedBank".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentProcessPaymentModeFixedBank {
            bank: intermediate_rep.bank.into_iter().next().ok_or_else(|| "bank missing in MerchantPaymentProcessPaymentModeFixedBank".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentProcessPaymentModeFixedBank".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedBank> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedBank>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedBank>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentProcessPaymentModeFixedBank - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFixedBank> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentProcessPaymentModeFixedBank as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentProcessPaymentModeFixedBank - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Flexible payment mode in merchant payment process. The account will then be selected within the process.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentProcessPaymentModeFlexible {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentProcessPaymentModeFlexible {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> MerchantPaymentProcessPaymentModeFlexible {
        MerchantPaymentProcessPaymentModeFlexible {
 r_type,
        }
    }
}

/// Converts the MerchantPaymentProcessPaymentModeFlexible value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentProcessPaymentModeFlexible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcessPaymentModeFlexible value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcessPaymentModeFlexible {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentProcessPaymentModeFlexible".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentProcessPaymentModeFlexible".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentProcessPaymentModeFlexible {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentProcessPaymentModeFlexible".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFlexible> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFlexible>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFlexible>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentProcessPaymentModeFlexible - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentProcessPaymentModeFlexible> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentProcessPaymentModeFlexible as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentProcessPaymentModeFlexible - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum MerchantPaymentResultOperationStatus {
    #[serde(alias = "Failure")]
    MerchantPaymentResultOperationStatusFailure(models::MerchantPaymentResultOperationStatusFailure),
    #[serde(alias = "Success")]
    MerchantPaymentResultOperationStatusSuccess(models::MerchantPaymentResultOperationStatusSuccess),
    #[serde(alias = "Unknown")]
    MerchantPaymentResultOperationStatusUnknown(models::MerchantPaymentResultOperationStatusUnknown),
}

impl validator::Validate for MerchantPaymentResultOperationStatus
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::MerchantPaymentResultOperationStatusFailure(v) => v.validate(),
            Self::MerchantPaymentResultOperationStatusSuccess(v) => v.validate(),
            Self::MerchantPaymentResultOperationStatusUnknown(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentResultOperationStatus value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentResultOperationStatus {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl serde::Serialize for MerchantPaymentResultOperationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::MerchantPaymentResultOperationStatusFailure(x) => x.serialize(serializer),
                Self::MerchantPaymentResultOperationStatusSuccess(x) => x.serialize(serializer),
                Self::MerchantPaymentResultOperationStatusUnknown(x) => x.serialize(serializer),
            }
    }
}

impl From<models::MerchantPaymentResultOperationStatusFailure> for MerchantPaymentResultOperationStatus {
    fn from(value: models::MerchantPaymentResultOperationStatusFailure) -> Self {
        Self::MerchantPaymentResultOperationStatusFailure(value)
    }
}
impl From<models::MerchantPaymentResultOperationStatusSuccess> for MerchantPaymentResultOperationStatus {
    fn from(value: models::MerchantPaymentResultOperationStatusSuccess) -> Self {
        Self::MerchantPaymentResultOperationStatusSuccess(value)
    }
}
impl From<models::MerchantPaymentResultOperationStatusUnknown> for MerchantPaymentResultOperationStatus {
    fn from(value: models::MerchantPaymentResultOperationStatusUnknown) -> Self {
        Self::MerchantPaymentResultOperationStatusUnknown(value)
    }
}





/// Merchant payment operation failed (there was no charge for sure)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentResultOperationStatusFailure {
    #[serde(rename = "cause")]
          #[validate(nested)]
    pub cause: models::MerchantPaymentResultOperationStatusFailureCause,

    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentResultOperationStatusFailure {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(cause: models::MerchantPaymentResultOperationStatusFailureCause, r_type: String, ) -> MerchantPaymentResultOperationStatusFailure {
        MerchantPaymentResultOperationStatusFailure {
 cause,
 r_type,
        }
    }
}

/// Converts the MerchantPaymentResultOperationStatusFailure value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentResultOperationStatusFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping cause in query parameter serialization


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentResultOperationStatusFailure value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentResultOperationStatusFailure {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cause: Vec<models::MerchantPaymentResultOperationStatusFailureCause>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentResultOperationStatusFailure".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cause" => intermediate_rep.cause.push(<models::MerchantPaymentResultOperationStatusFailureCause as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentResultOperationStatusFailure".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentResultOperationStatusFailure {
            cause: intermediate_rep.cause.into_iter().next().ok_or_else(|| "cause missing in MerchantPaymentResultOperationStatusFailure".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentResultOperationStatusFailure".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentResultOperationStatusFailure> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentResultOperationStatusFailure>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentResultOperationStatusFailure>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentResultOperationStatusFailure - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentResultOperationStatusFailure> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentResultOperationStatusFailure as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentResultOperationStatusFailure - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// The reason why the operation failed.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum MerchantPaymentResultOperationStatusFailureCause {
    #[serde(rename = "AuthenticationFailed")]
    AuthenticationFailed,
    #[serde(rename = "InsufficientBalance")]
    InsufficientBalance,
    #[serde(rename = "InvalidMerchant")]
    InvalidMerchant,
    #[serde(rename = "NoAccounts")]
    NoAccounts,
    #[serde(rename = "SwitchUnavailable")]
    SwitchUnavailable,
    #[serde(rename = "Other")]
    Other,
}

impl validator::Validate for MerchantPaymentResultOperationStatusFailureCause
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for MerchantPaymentResultOperationStatusFailureCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MerchantPaymentResultOperationStatusFailureCause::AuthenticationFailed => write!(f, "AuthenticationFailed"),
            MerchantPaymentResultOperationStatusFailureCause::InsufficientBalance => write!(f, "InsufficientBalance"),
            MerchantPaymentResultOperationStatusFailureCause::InvalidMerchant => write!(f, "InvalidMerchant"),
            MerchantPaymentResultOperationStatusFailureCause::NoAccounts => write!(f, "NoAccounts"),
            MerchantPaymentResultOperationStatusFailureCause::SwitchUnavailable => write!(f, "SwitchUnavailable"),
            MerchantPaymentResultOperationStatusFailureCause::Other => write!(f, "Other"),
        }
    }
}

impl std::str::FromStr for MerchantPaymentResultOperationStatusFailureCause {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "AuthenticationFailed" => std::result::Result::Ok(MerchantPaymentResultOperationStatusFailureCause::AuthenticationFailed),
            "InsufficientBalance" => std::result::Result::Ok(MerchantPaymentResultOperationStatusFailureCause::InsufficientBalance),
            "InvalidMerchant" => std::result::Result::Ok(MerchantPaymentResultOperationStatusFailureCause::InvalidMerchant),
            "NoAccounts" => std::result::Result::Ok(MerchantPaymentResultOperationStatusFailureCause::NoAccounts),
            "SwitchUnavailable" => std::result::Result::Ok(MerchantPaymentResultOperationStatusFailureCause::SwitchUnavailable),
            "Other" => std::result::Result::Ok(MerchantPaymentResultOperationStatusFailureCause::Other),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// Merchant payment operation finished with success
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentResultOperationStatusSuccess {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentResultOperationStatusSuccess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> MerchantPaymentResultOperationStatusSuccess {
        MerchantPaymentResultOperationStatusSuccess {
 r_type,
        }
    }
}

/// Converts the MerchantPaymentResultOperationStatusSuccess value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentResultOperationStatusSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentResultOperationStatusSuccess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentResultOperationStatusSuccess {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentResultOperationStatusSuccess".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentResultOperationStatusSuccess".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentResultOperationStatusSuccess {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentResultOperationStatusSuccess".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentResultOperationStatusSuccess> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentResultOperationStatusSuccess>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentResultOperationStatusSuccess>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentResultOperationStatusSuccess - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentResultOperationStatusSuccess> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentResultOperationStatusSuccess as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentResultOperationStatusSuccess - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Cannot determine if merchant payment operation succeed or not
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentResultOperationStatusUnknown {
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl MerchantPaymentResultOperationStatusUnknown {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> MerchantPaymentResultOperationStatusUnknown {
        MerchantPaymentResultOperationStatusUnknown {
 r_type,
        }
    }
}

/// Converts the MerchantPaymentResultOperationStatusUnknown value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentResultOperationStatusUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentResultOperationStatusUnknown value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentResultOperationStatusUnknown {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentResultOperationStatusUnknown".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentResultOperationStatusUnknown".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentResultOperationStatusUnknown {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentResultOperationStatusUnknown".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentResultOperationStatusUnknown> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentResultOperationStatusUnknown>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentResultOperationStatusUnknown>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentResultOperationStatusUnknown - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentResultOperationStatusUnknown> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentResultOperationStatusUnknown as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentResultOperationStatusUnknown - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input provided when the user dials into the session (dials the shortcode string)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewSessionSessionInputDial {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Shortcode string dialed by the user
    #[serde(rename = "shortcodeString")]
          #[validate(custom(function = "check_xss_string"))]
    pub shortcode_string: String,

}



impl NewSessionSessionInputDial {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, shortcode_string: String, ) -> NewSessionSessionInputDial {
        NewSessionSessionInputDial {
 r_type,
 shortcode_string,
        }
    }
}

/// Converts the NewSessionSessionInputDial value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewSessionSessionInputDial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("shortcodeString".to_string()),
            Some(self.shortcode_string.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewSessionSessionInputDial value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewSessionSessionInputDial {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub shortcode_string: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewSessionSessionInputDial".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "shortcodeString" => intermediate_rep.shortcode_string.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewSessionSessionInputDial".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewSessionSessionInputDial {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in NewSessionSessionInputDial".to_string())?,
            shortcode_string: intermediate_rep.shortcode_string.into_iter().next().ok_or_else(|| "shortcodeString missing in NewSessionSessionInputDial".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewSessionSessionInputDial> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewSessionSessionInputDial>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewSessionSessionInputDial>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NewSessionSessionInputDial - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewSessionSessionInputDial> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewSessionSessionInputDial as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NewSessionSessionInputDial - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input provided when session is begun by a push message sent to a user.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewSessionSessionInputPush {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Context regarding the push message.
    #[serde(rename = "contextData")]
          #[validate(custom(function = "check_xss_string"))]
    pub context_data: String,

}



impl NewSessionSessionInputPush {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, context_data: String, ) -> NewSessionSessionInputPush {
        NewSessionSessionInputPush {
 r_type,
 context_data,
        }
    }
}

/// Converts the NewSessionSessionInputPush value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewSessionSessionInputPush {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("contextData".to_string()),
            Some(self.context_data.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewSessionSessionInputPush value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewSessionSessionInputPush {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub context_data: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewSessionSessionInputPush".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contextData" => intermediate_rep.context_data.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewSessionSessionInputPush".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewSessionSessionInputPush {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in NewSessionSessionInputPush".to_string())?,
            context_data: intermediate_rep.context_data.into_iter().next().ok_or_else(|| "contextData missing in NewSessionSessionInputPush".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewSessionSessionInputPush> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewSessionSessionInputPush>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewSessionSessionInputPush>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NewSessionSessionInputPush - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewSessionSessionInputPush> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewSessionSessionInputPush as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NewSessionSessionInputPush - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input provided when session is redirected from different USSD application.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewSessionSessionInputRedirect {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "processId")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub process_id: Option<String>,

    #[serde(rename = "process")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub process: Option<models::UssdAppProcess>,

}



impl NewSessionSessionInputRedirect {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, ) -> NewSessionSessionInputRedirect {
        NewSessionSessionInputRedirect {
 r_type,
 process_id: None,
 process: None,
        }
    }
}

/// Converts the NewSessionSessionInputRedirect value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewSessionSessionInputRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            self.process_id.as_ref().map(|process_id| {
                [
                    "processId".to_string(),
                    process_id.to_string(),
                ].join(",")
            }),

            // Skipping process in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewSessionSessionInputRedirect value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewSessionSessionInputRedirect {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub process_id: Vec<String>,
            pub process: Vec<models::UssdAppProcess>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewSessionSessionInputRedirect".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "processId" => intermediate_rep.process_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "process" => intermediate_rep.process.push(<models::UssdAppProcess as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewSessionSessionInputRedirect".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewSessionSessionInputRedirect {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in NewSessionSessionInputRedirect".to_string())?,
            process_id: intermediate_rep.process_id.into_iter().next(),
            process: intermediate_rep.process.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewSessionSessionInputRedirect> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewSessionSessionInputRedirect>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewSessionSessionInputRedirect>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NewSessionSessionInputRedirect - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewSessionSessionInputRedirect> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewSessionSessionInputRedirect as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NewSessionSessionInputRedirect - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Developer's USSD App returns USSD process result as a map of params
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Redirect {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Identifier of the USSD app created on Qrios platform.
    #[serde(rename = "destinationAppId")]
          #[validate(custom(function = "check_xss_string"))]
    pub destination_app_id: String,

    #[serde(rename = "process")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub process: Option<models::UssdAppProcess>,

}



impl Redirect {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, destination_app_id: String, ) -> Redirect {
        Redirect {
 r_type,
 destination_app_id,
 process: None,
        }
    }
}

/// Converts the Redirect value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Redirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("destinationAppId".to_string()),
            Some(self.destination_app_id.to_string()),

            // Skipping process in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Redirect value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Redirect {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub destination_app_id: Vec<String>,
            pub process: Vec<models::UssdAppProcess>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Redirect".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "destinationAppId" => intermediate_rep.destination_app_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "process" => intermediate_rep.process.push(<models::UssdAppProcess as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Redirect".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Redirect {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Redirect".to_string())?,
            destination_app_id: intermediate_rep.destination_app_id.into_iter().next().ok_or_else(|| "destinationAppId missing in Redirect".to_string())?,
            process: intermediate_rep.process.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Redirect> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Redirect>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Redirect>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Redirect - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Redirect> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Redirect as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Redirect - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// If the current session was redirected from some other app, then this UssdAction redirects back to the calling app within the same USSD session
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReturnFromRedirect {
    /// Result parameters returned in the \"return from redirect\". Keys should be considered as result parameter names and their values as result parameter values.
    #[serde(rename = "resultParams")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub result_params: std::collections::HashMap<String, String>,

}



impl ReturnFromRedirect {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(result_params: std::collections::HashMap<String, String>, ) -> ReturnFromRedirect {
        ReturnFromRedirect {
 result_params,
        }
    }
}

/// Converts the ReturnFromRedirect value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ReturnFromRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping resultParams in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ReturnFromRedirect value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ReturnFromRedirect {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub result_params: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ReturnFromRedirect".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "resultParams" => return std::result::Result::Err("Parsing a container in this style is not supported in ReturnFromRedirect".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ReturnFromRedirect".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ReturnFromRedirect {
            result_params: intermediate_rep.result_params.into_iter().next().ok_or_else(|| "resultParams missing in ReturnFromRedirect".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ReturnFromRedirect> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ReturnFromRedirect>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ReturnFromRedirect>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ReturnFromRedirect - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ReturnFromRedirect> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ReturnFromRedirect as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ReturnFromRedirect - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Run a predefined process. The process will then take control of the session (will send prompts to the user and handle responses from the user) until it returns control along with a response. From the view of a user, they will interact with a single app.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RunProcess {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "process")]
          #[validate(nested)]
    pub process: models::UssdProcess,

}



impl RunProcess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, process: models::UssdProcess, ) -> RunProcess {
        RunProcess {
 r_type,
 process,
        }
    }
}

/// Converts the RunProcess value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RunProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

            // Skipping process in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RunProcess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RunProcess {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub process: Vec<models::UssdProcess>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RunProcess".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "process" => intermediate_rep.process.push(<models::UssdProcess as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RunProcess".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RunProcess {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in RunProcess".to_string())?,
            process: intermediate_rep.process.into_iter().next().ok_or_else(|| "process missing in RunProcess".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RunProcess> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RunProcess>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RunProcess>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for RunProcess - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RunProcess> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RunProcess as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into RunProcess - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Display a given view to the user. A view is a text message that will appear on the screen with optional ability to input data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShowView {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "view")]
          #[validate(nested)]
    pub view: models::UssdView,

}



impl ShowView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, view: models::UssdView, ) -> ShowView {
        ShowView {
 r_type,
 view,
        }
    }
}

/// Converts the ShowView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ShowView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

            // Skipping view in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShowView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShowView {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub view: Vec<models::UssdView>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShowView".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "view" => intermediate_rep.view.push(<models::UssdView as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShowView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShowView {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in ShowView".to_string())?,
            view: intermediate_rep.view.into_iter().next().ok_or_else(|| "view missing in ShowView".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShowView> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ShowView>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShowView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ShowView - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ShowView> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShowView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ShowView - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Action to be sent back to the user. It can either start a predefined process or show a view.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdAction {
    Object(crate::types::Object),
    UssdActionOneOf(models::UssdActionOneOf),
    Object1(crate::types::Object),
    UssdActionOneOf1(models::UssdActionOneOf1),
    UssdActionOneOf2(models::UssdActionOneOf2),
}

impl validator::Validate for UssdAction
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::Object(_) => std::result::Result::Ok(()),
            Self::UssdActionOneOf(v) => v.validate(),
            Self::Object1(_) => std::result::Result::Ok(()),
            Self::UssdActionOneOf1(v) => v.validate(),
            Self::UssdActionOneOf2(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdAction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdAction {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::UssdActionOneOf> for UssdAction {
    fn from(value: models::UssdActionOneOf) -> Self {
        Self::UssdActionOneOf(value)
    }
}
impl From<models::UssdActionOneOf1> for UssdAction {
    fn from(value: models::UssdActionOneOf1) -> Self {
        Self::UssdActionOneOf1(value)
    }
}
impl From<models::UssdActionOneOf2> for UssdAction {
    fn from(value: models::UssdActionOneOf2) -> Self {
        Self::UssdActionOneOf2(value)
    }
}





#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionOneOf {
    #[serde(rename = "Redirect")]
          #[validate(nested)]
    pub redirect: models::Redirect,

}



impl UssdActionOneOf {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(redirect: models::Redirect, ) -> UssdActionOneOf {
        UssdActionOneOf {
 redirect,
        }
    }
}

/// Converts the UssdActionOneOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping Redirect in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionOneOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionOneOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub redirect: Vec<models::Redirect>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionOneOf".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "Redirect" => intermediate_rep.redirect.push(<models::Redirect as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionOneOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionOneOf {
            redirect: intermediate_rep.redirect.into_iter().next().ok_or_else(|| "Redirect missing in UssdActionOneOf".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionOneOf> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionOneOf>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionOneOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionOneOf - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionOneOf> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionOneOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionOneOf - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionOneOf1 {
    #[serde(rename = "RunProcess")]
          #[validate(nested)]
    pub run_process: models::RunProcess,

}



impl UssdActionOneOf1 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(run_process: models::RunProcess, ) -> UssdActionOneOf1 {
        UssdActionOneOf1 {
 run_process,
        }
    }
}

/// Converts the UssdActionOneOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionOneOf1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping RunProcess in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionOneOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionOneOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub run_process: Vec<models::RunProcess>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionOneOf1".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "RunProcess" => intermediate_rep.run_process.push(<models::RunProcess as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionOneOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionOneOf1 {
            run_process: intermediate_rep.run_process.into_iter().next().ok_or_else(|| "RunProcess missing in UssdActionOneOf1".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionOneOf1> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionOneOf1>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionOneOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionOneOf1 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionOneOf1> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionOneOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionOneOf1 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionOneOf2 {
    #[serde(rename = "ShowView")]
          #[validate(nested)]
    pub show_view: models::ShowView,

}



impl UssdActionOneOf2 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(show_view: models::ShowView, ) -> UssdActionOneOf2 {
        UssdActionOneOf2 {
 show_view,
        }
    }
}

/// Converts the UssdActionOneOf2 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionOneOf2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping ShowView in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionOneOf2 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionOneOf2 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub show_view: Vec<models::ShowView>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionOneOf2".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "ShowView" => intermediate_rep.show_view.push(<models::ShowView as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionOneOf2".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionOneOf2 {
            show_view: intermediate_rep.show_view.into_iter().next().ok_or_else(|| "ShowView missing in UssdActionOneOf2".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionOneOf2> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionOneOf2>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionOneOf2>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionOneOf2 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionOneOf2> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionOneOf2 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionOneOf2 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Result of USSD action. Either input provided by the user or the result of a previously-initiated process.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdActionResult {
    UssdActionResultOneOf(models::UssdActionResultOneOf),
    UssdActionResultOneOf1(models::UssdActionResultOneOf1),
    UssdActionResultOneOf2(models::UssdActionResultOneOf2),
    Object(crate::types::Object),
}

impl validator::Validate for UssdActionResult
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::UssdActionResultOneOf(v) => v.validate(),
            Self::UssdActionResultOneOf1(v) => v.validate(),
            Self::UssdActionResultOneOf2(v) => v.validate(),
            Self::Object(_) => std::result::Result::Ok(()),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResult {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::UssdActionResultOneOf> for UssdActionResult {
    fn from(value: models::UssdActionResultOneOf) -> Self {
        Self::UssdActionResultOneOf(value)
    }
}
impl From<models::UssdActionResultOneOf1> for UssdActionResult {
    fn from(value: models::UssdActionResultOneOf1) -> Self {
        Self::UssdActionResultOneOf1(value)
    }
}
impl From<models::UssdActionResultOneOf2> for UssdActionResult {
    fn from(value: models::UssdActionResultOneOf2) -> Self {
        Self::UssdActionResultOneOf2(value)
    }
}
impl From<crate::types::Object> for UssdActionResult {
    fn from(value: crate::types::Object) -> Self {
        Self::Object(value)
    }
}





/// Returns control of the session back to the developer and passes the result of embedded process execution.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionResultEmbeddedProcessResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Result of the `embedded process`. Keys should be considered as result parameter names and their values as result parameter values.
    #[serde(rename = "resultParams")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub result_params: std::collections::HashMap<String, String>,

}



impl UssdActionResultEmbeddedProcessResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, result_params: std::collections::HashMap<String, String>, ) -> UssdActionResultEmbeddedProcessResult {
        UssdActionResultEmbeddedProcessResult {
 r_type,
 result_params,
        }
    }
}

/// Converts the UssdActionResultEmbeddedProcessResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionResultEmbeddedProcessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

            // Skipping resultParams in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResultEmbeddedProcessResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResultEmbeddedProcessResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub result_params: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionResultEmbeddedProcessResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "resultParams" => return std::result::Result::Err("Parsing a container in this style is not supported in UssdActionResultEmbeddedProcessResult".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionResultEmbeddedProcessResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionResultEmbeddedProcessResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdActionResultEmbeddedProcessResult".to_string())?,
            result_params: intermediate_rep.result_params.into_iter().next().ok_or_else(|| "resultParams missing in UssdActionResultEmbeddedProcessResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionResultEmbeddedProcessResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionResultEmbeddedProcessResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionResultEmbeddedProcessResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionResultEmbeddedProcessResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionResultEmbeddedProcessResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionResultEmbeddedProcessResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionResultEmbeddedProcessResult - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input from the user; provided by UssdAction.UssdView.InputView.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionResultInputResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Raw value typed in by the user.
    #[serde(rename = "value")]
          #[validate(custom(function = "check_xss_string"))]
    pub value: String,

}



impl UssdActionResultInputResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, value: String, ) -> UssdActionResultInputResult {
        UssdActionResultInputResult {
 r_type,
 value,
        }
    }
}

/// Converts the UssdActionResultInputResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionResultInputResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("value".to_string()),
            Some(self.value.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResultInputResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResultInputResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionResultInputResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionResultInputResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionResultInputResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdActionResultInputResult".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in UssdActionResultInputResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionResultInputResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionResultInputResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionResultInputResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionResultInputResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionResultInputResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionResultInputResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionResultInputResult - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Result from merchant payment process; provided by UssdProcess.MerchantPaymentProcess.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionResultMerchantPaymentResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Merchant payment operation id
    #[serde(rename = "operationId")]
          #[validate(custom(function = "check_xss_string"))]
    pub operation_id: String,

    #[serde(rename = "status")]
          #[validate(nested)]
    pub status: models::MerchantPaymentResultOperationStatus,

}



impl UssdActionResultMerchantPaymentResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, operation_id: String, status: models::MerchantPaymentResultOperationStatus, ) -> UssdActionResultMerchantPaymentResult {
        UssdActionResultMerchantPaymentResult {
 r_type,
 operation_id,
 status,
        }
    }
}

/// Converts the UssdActionResultMerchantPaymentResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionResultMerchantPaymentResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("operationId".to_string()),
            Some(self.operation_id.to_string()),

            // Skipping status in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResultMerchantPaymentResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResultMerchantPaymentResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub operation_id: Vec<String>,
            pub status: Vec<models::MerchantPaymentResultOperationStatus>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionResultMerchantPaymentResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "operationId" => intermediate_rep.operation_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<models::MerchantPaymentResultOperationStatus as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionResultMerchantPaymentResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionResultMerchantPaymentResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdActionResultMerchantPaymentResult".to_string())?,
            operation_id: intermediate_rep.operation_id.into_iter().next().ok_or_else(|| "operationId missing in UssdActionResultMerchantPaymentResult".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in UssdActionResultMerchantPaymentResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionResultMerchantPaymentResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionResultMerchantPaymentResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionResultMerchantPaymentResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionResultMerchantPaymentResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionResultMerchantPaymentResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionResultMerchantPaymentResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionResultMerchantPaymentResult - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionResultOneOf {
    #[serde(rename = "EmbeddedProcessResult")]
          #[validate(nested)]
    pub embedded_process_result: models::UssdActionResultEmbeddedProcessResult,

}



impl UssdActionResultOneOf {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(embedded_process_result: models::UssdActionResultEmbeddedProcessResult, ) -> UssdActionResultOneOf {
        UssdActionResultOneOf {
 embedded_process_result,
        }
    }
}

/// Converts the UssdActionResultOneOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionResultOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping EmbeddedProcessResult in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResultOneOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResultOneOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub embedded_process_result: Vec<models::UssdActionResultEmbeddedProcessResult>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionResultOneOf".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "EmbeddedProcessResult" => intermediate_rep.embedded_process_result.push(<models::UssdActionResultEmbeddedProcessResult as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionResultOneOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionResultOneOf {
            embedded_process_result: intermediate_rep.embedded_process_result.into_iter().next().ok_or_else(|| "EmbeddedProcessResult missing in UssdActionResultOneOf".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionResultOneOf> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionResultOneOf>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionResultOneOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionResultOneOf - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionResultOneOf> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionResultOneOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionResultOneOf - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionResultOneOf1 {
    #[serde(rename = "InputResult")]
          #[validate(nested)]
    pub input_result: models::UssdActionResultInputResult,

}



impl UssdActionResultOneOf1 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(input_result: models::UssdActionResultInputResult, ) -> UssdActionResultOneOf1 {
        UssdActionResultOneOf1 {
 input_result,
        }
    }
}

/// Converts the UssdActionResultOneOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionResultOneOf1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping InputResult in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResultOneOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResultOneOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub input_result: Vec<models::UssdActionResultInputResult>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionResultOneOf1".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "InputResult" => intermediate_rep.input_result.push(<models::UssdActionResultInputResult as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionResultOneOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionResultOneOf1 {
            input_result: intermediate_rep.input_result.into_iter().next().ok_or_else(|| "InputResult missing in UssdActionResultOneOf1".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionResultOneOf1> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionResultOneOf1>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionResultOneOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionResultOneOf1 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionResultOneOf1> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionResultOneOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionResultOneOf1 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionResultOneOf2 {
    #[serde(rename = "MerchantPaymentResult")]
          #[validate(nested)]
    pub merchant_payment_result: models::UssdActionResultMerchantPaymentResult,

}



impl UssdActionResultOneOf2 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(merchant_payment_result: models::UssdActionResultMerchantPaymentResult, ) -> UssdActionResultOneOf2 {
        UssdActionResultOneOf2 {
 merchant_payment_result,
        }
    }
}

/// Converts the UssdActionResultOneOf2 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionResultOneOf2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping MerchantPaymentResult in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResultOneOf2 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResultOneOf2 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub merchant_payment_result: Vec<models::UssdActionResultMerchantPaymentResult>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionResultOneOf2".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "MerchantPaymentResult" => intermediate_rep.merchant_payment_result.push(<models::UssdActionResultMerchantPaymentResult as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionResultOneOf2".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionResultOneOf2 {
            merchant_payment_result: intermediate_rep.merchant_payment_result.into_iter().next().ok_or_else(|| "MerchantPaymentResult missing in UssdActionResultOneOf2".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionResultOneOf2> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionResultOneOf2>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionResultOneOf2>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionResultOneOf2 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionResultOneOf2> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionResultOneOf2 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionResultOneOf2 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Result returned from the app, to which the session was previously redirected.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionResultReturnFromRedirectResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Result of the `return from redirection`. Keys should be considered as result parameter names and their values as result parameter values.
    #[serde(rename = "resultParams")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub result_params: std::collections::HashMap<String, String>,

}



impl UssdActionResultReturnFromRedirectResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, result_params: std::collections::HashMap<String, String>, ) -> UssdActionResultReturnFromRedirectResult {
        UssdActionResultReturnFromRedirectResult {
 r_type,
 result_params,
        }
    }
}

/// Converts the UssdActionResultReturnFromRedirectResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionResultReturnFromRedirectResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

            // Skipping resultParams in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionResultReturnFromRedirectResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionResultReturnFromRedirectResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub result_params: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdActionResultReturnFromRedirectResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "resultParams" => return std::result::Result::Err("Parsing a container in this style is not supported in UssdActionResultReturnFromRedirectResult".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionResultReturnFromRedirectResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionResultReturnFromRedirectResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdActionResultReturnFromRedirectResult".to_string())?,
            result_params: intermediate_rep.result_params.into_iter().next().ok_or_else(|| "resultParams missing in UssdActionResultReturnFromRedirectResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionResultReturnFromRedirectResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionResultReturnFromRedirectResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionResultReturnFromRedirectResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionResultReturnFromRedirectResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionResultReturnFromRedirectResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionResultReturnFromRedirectResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionResultReturnFromRedirectResult - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// It indicates which process inside USSD App should be invoked
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdAppProcess {
    /// Identifier of the process. Based on this parameter, developer can decide eg. what USSD screen will be shown next.
    #[serde(rename = "id")]
          #[validate(custom(function = "check_xss_string"))]
    pub id: String,

    /// Parameters of the process. Keys should be considered as process param names and their values as process param values.
    #[serde(rename = "params")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub params: std::collections::HashMap<String, String>,

}



impl UssdAppProcess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, params: std::collections::HashMap<String, String>, ) -> UssdAppProcess {
        UssdAppProcess {
 id,
 params,
        }
    }
}

/// Converts the UssdAppProcess value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdAppProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),

            // Skipping params in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdAppProcess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdAppProcess {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub params: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdAppProcess".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "params" => return std::result::Result::Err("Parsing a container in this style is not supported in UssdAppProcess".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdAppProcess".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdAppProcess {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in UssdAppProcess".to_string())?,
            params: intermediate_rep.params.into_iter().next().ok_or_else(|| "params missing in UssdAppProcess".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdAppProcess> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdAppProcess>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdAppProcess>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdAppProcess - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdAppProcess> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdAppProcess as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdAppProcess - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// The process control is handed over to (flow continues within the process until the process continues or aborts).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdProcess {
    UssdProcessOneOf(models::UssdProcessOneOf),
    UssdProcessOneOf1(models::UssdProcessOneOf1),
}

impl validator::Validate for UssdProcess
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::UssdProcessOneOf(v) => v.validate(),
            Self::UssdProcessOneOf1(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdProcess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdProcess {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::UssdProcessOneOf> for UssdProcess {
    fn from(value: models::UssdProcessOneOf) -> Self {
        Self::UssdProcessOneOf(value)
    }
}
impl From<models::UssdProcessOneOf1> for UssdProcess {
    fn from(value: models::UssdProcessOneOf1) -> Self {
        Self::UssdProcessOneOf1(value)
    }
}





/// Initiate an embedded process. This action causes the Qrios system to hand over control to an external application. The user may be prompted to provide credentials during the embedded process. After the embedded process is complete, the ussdSessionEvent/continue API endpoint is called to deliver the result and return control of the session to the developer application.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdProcessEmbeddedProcess {
    /// embedded process id
    #[serde(rename = "processId")]
          #[validate(custom(function = "check_xss_string"))]
    pub process_id: String,

    #[serde(rename = "params")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub params: std::collections::HashMap<String, String>,

}



impl UssdProcessEmbeddedProcess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(process_id: String, params: std::collections::HashMap<String, String>, ) -> UssdProcessEmbeddedProcess {
        UssdProcessEmbeddedProcess {
 process_id,
 params,
        }
    }
}

/// Converts the UssdProcessEmbeddedProcess value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdProcessEmbeddedProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("processId".to_string()),
            Some(self.process_id.to_string()),

            // Skipping params in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdProcessEmbeddedProcess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdProcessEmbeddedProcess {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub process_id: Vec<String>,
            pub params: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdProcessEmbeddedProcess".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "processId" => intermediate_rep.process_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "params" => return std::result::Result::Err("Parsing a container in this style is not supported in UssdProcessEmbeddedProcess".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdProcessEmbeddedProcess".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdProcessEmbeddedProcess {
            process_id: intermediate_rep.process_id.into_iter().next().ok_or_else(|| "processId missing in UssdProcessEmbeddedProcess".to_string())?,
            params: intermediate_rep.params.into_iter().next().ok_or_else(|| "params missing in UssdProcessEmbeddedProcess".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdProcessEmbeddedProcess> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdProcessEmbeddedProcess>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdProcessEmbeddedProcess>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdProcessEmbeddedProcess - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdProcessEmbeddedProcess> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdProcessEmbeddedProcess as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdProcessEmbeddedProcess - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Initiation of merchant payment process. In this process, the control is handed over to an external USSD merchant payment process. The user will be asked for credentials and the payment will commence. As the process ends, it will call this API to deliver the result and hand control over the session back to the server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdProcessMerchantPaymentProcess {
    /// Merchant payment operation id
    #[serde(rename = "operationId")]
          #[validate(custom(function = "check_xss_string"))]
    pub operation_id: String,

    /// Merchant code
    #[serde(rename = "merchantCode")]
          #[validate(custom(function = "check_xss_string"))]
    pub merchant_code: String,

    #[serde(rename = "amount")]
    pub amount: f64,

    #[serde(rename = "paymentMode")]
          #[validate(nested)]
    pub payment_mode: models::MerchantPaymentProcessPaymentMode,

    #[serde(rename = "executionMode")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub execution_mode: Option<models::MerchantPaymentProcessExecutionMode>,

}



impl UssdProcessMerchantPaymentProcess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(operation_id: String, merchant_code: String, amount: f64, payment_mode: models::MerchantPaymentProcessPaymentMode, ) -> UssdProcessMerchantPaymentProcess {
        UssdProcessMerchantPaymentProcess {
 operation_id,
 merchant_code,
 amount,
 payment_mode,
 execution_mode: None,
        }
    }
}

/// Converts the UssdProcessMerchantPaymentProcess value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdProcessMerchantPaymentProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("operationId".to_string()),
            Some(self.operation_id.to_string()),


            Some("merchantCode".to_string()),
            Some(self.merchant_code.to_string()),


            Some("amount".to_string()),
            Some(self.amount.to_string()),

            // Skipping paymentMode in query parameter serialization

            // Skipping executionMode in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdProcessMerchantPaymentProcess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdProcessMerchantPaymentProcess {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub operation_id: Vec<String>,
            pub merchant_code: Vec<String>,
            pub amount: Vec<f64>,
            pub payment_mode: Vec<models::MerchantPaymentProcessPaymentMode>,
            pub execution_mode: Vec<models::MerchantPaymentProcessExecutionMode>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdProcessMerchantPaymentProcess".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "operationId" => intermediate_rep.operation_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "merchantCode" => intermediate_rep.merchant_code.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "amount" => intermediate_rep.amount.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "paymentMode" => intermediate_rep.payment_mode.push(<models::MerchantPaymentProcessPaymentMode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "executionMode" => intermediate_rep.execution_mode.push(<models::MerchantPaymentProcessExecutionMode as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdProcessMerchantPaymentProcess".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdProcessMerchantPaymentProcess {
            operation_id: intermediate_rep.operation_id.into_iter().next().ok_or_else(|| "operationId missing in UssdProcessMerchantPaymentProcess".to_string())?,
            merchant_code: intermediate_rep.merchant_code.into_iter().next().ok_or_else(|| "merchantCode missing in UssdProcessMerchantPaymentProcess".to_string())?,
            amount: intermediate_rep.amount.into_iter().next().ok_or_else(|| "amount missing in UssdProcessMerchantPaymentProcess".to_string())?,
            payment_mode: intermediate_rep.payment_mode.into_iter().next().ok_or_else(|| "paymentMode missing in UssdProcessMerchantPaymentProcess".to_string())?,
            execution_mode: intermediate_rep.execution_mode.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdProcessMerchantPaymentProcess> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdProcessMerchantPaymentProcess>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdProcessMerchantPaymentProcess>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdProcessMerchantPaymentProcess - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdProcessMerchantPaymentProcess> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdProcessMerchantPaymentProcess as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdProcessMerchantPaymentProcess - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdProcessOneOf {
    #[serde(rename = "EmbeddedProcess")]
          #[validate(nested)]
    pub embedded_process: models::UssdProcessEmbeddedProcess,

}



impl UssdProcessOneOf {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(embedded_process: models::UssdProcessEmbeddedProcess, ) -> UssdProcessOneOf {
        UssdProcessOneOf {
 embedded_process,
        }
    }
}

/// Converts the UssdProcessOneOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdProcessOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping EmbeddedProcess in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdProcessOneOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdProcessOneOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub embedded_process: Vec<models::UssdProcessEmbeddedProcess>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdProcessOneOf".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "EmbeddedProcess" => intermediate_rep.embedded_process.push(<models::UssdProcessEmbeddedProcess as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdProcessOneOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdProcessOneOf {
            embedded_process: intermediate_rep.embedded_process.into_iter().next().ok_or_else(|| "EmbeddedProcess missing in UssdProcessOneOf".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdProcessOneOf> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdProcessOneOf>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdProcessOneOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdProcessOneOf - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdProcessOneOf> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdProcessOneOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdProcessOneOf - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdProcessOneOf1 {
    #[serde(rename = "MerchantPaymentProcess")]
          #[validate(nested)]
    pub merchant_payment_process: models::UssdProcessMerchantPaymentProcess,

}



impl UssdProcessOneOf1 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(merchant_payment_process: models::UssdProcessMerchantPaymentProcess, ) -> UssdProcessOneOf1 {
        UssdProcessOneOf1 {
 merchant_payment_process,
        }
    }
}

/// Converts the UssdProcessOneOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdProcessOneOf1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping MerchantPaymentProcess in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdProcessOneOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdProcessOneOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub merchant_payment_process: Vec<models::UssdProcessMerchantPaymentProcess>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdProcessOneOf1".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "MerchantPaymentProcess" => intermediate_rep.merchant_payment_process.push(<models::UssdProcessMerchantPaymentProcess as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdProcessOneOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdProcessOneOf1 {
            merchant_payment_process: intermediate_rep.merchant_payment_process.into_iter().next().ok_or_else(|| "MerchantPaymentProcess missing in UssdProcessOneOf1".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdProcessOneOf1> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdProcessOneOf1>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdProcessOneOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdProcessOneOf1 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdProcessOneOf1> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdProcessOneOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdProcessOneOf1 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Command issued to the user, along with context data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdSessionCommand {
    #[serde(rename = "action")]
          #[validate(nested)]
    pub action: models::UssdAction,

    /// String set by the developer and carried over throughout the session.
    #[serde(rename = "contextData")]
          #[validate(custom(function = "check_xss_string"))]
    pub context_data: String,

    /// String value set by the developer, it will be visible in the billing information. Most recent session tag replaces previous value.
    #[serde(rename = "sessionTag")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub session_tag: Option<String>,

}



impl UssdSessionCommand {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(action: models::UssdAction, context_data: String, ) -> UssdSessionCommand {
        UssdSessionCommand {
 action,
 context_data,
 session_tag: None,
        }
    }
}

/// Converts the UssdSessionCommand value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdSessionCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping action in query parameter serialization


            Some("contextData".to_string()),
            Some(self.context_data.to_string()),


            self.session_tag.as_ref().map(|session_tag| {
                [
                    "sessionTag".to_string(),
                    session_tag.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdSessionCommand value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdSessionCommand {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub action: Vec<models::UssdAction>,
            pub context_data: Vec<String>,
            pub session_tag: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdSessionCommand".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "action" => intermediate_rep.action.push(<models::UssdAction as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contextData" => intermediate_rep.context_data.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sessionTag" => intermediate_rep.session_tag.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdSessionCommand".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdSessionCommand {
            action: intermediate_rep.action.into_iter().next().ok_or_else(|| "action missing in UssdSessionCommand".to_string())?,
            context_data: intermediate_rep.context_data.into_iter().next().ok_or_else(|| "contextData missing in UssdSessionCommand".to_string())?,
            session_tag: intermediate_rep.session_tag.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdSessionCommand> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdSessionCommand>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdSessionCommand>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdSessionCommand - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdSessionCommand> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdSessionCommand as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdSessionCommand - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// The event sent when the USSD session starts (user initiates the session).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdSessionEventNewSession {
    /// Client identifier of the developer using Qrios platform.
    #[serde(rename = "clientId")]
          #[validate(custom(function = "check_xss_string"))]
    pub client_id: String,

    /// Identifier of the USSD app created on Qrios platform.
    #[serde(rename = "appId")]
          #[validate(custom(function = "check_xss_string"))]
    pub app_id: String,

    /// Unique identifier of USSD session.
    #[serde(rename = "sessionId")]
          #[validate(custom(function = "check_xss_string"))]
    pub session_id: String,

    /// Phone number of customer who initiated the USSD session (e.g. +2341234567891).
    #[serde(rename = "msisdn")]
          #[validate(custom(function = "check_xss_string"))]
    pub msisdn: String,

    /// MSISDN's mobile operator (e.g. mtn).
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "operator")]
          #[validate(custom(function = "check_xss_string"))]
    pub operator: String,

    #[serde(rename = "input")]
          #[validate(nested)]
    pub input: models::UssdSessionEventNewSessionSessionInput,

}



impl UssdSessionEventNewSession {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(client_id: String, app_id: String, session_id: String, msisdn: String, operator: String, input: models::UssdSessionEventNewSessionSessionInput, ) -> UssdSessionEventNewSession {
        UssdSessionEventNewSession {
 client_id,
 app_id,
 session_id,
 msisdn,
 operator,
 input,
        }
    }
}

/// Converts the UssdSessionEventNewSession value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdSessionEventNewSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("clientId".to_string()),
            Some(self.client_id.to_string()),


            Some("appId".to_string()),
            Some(self.app_id.to_string()),


            Some("sessionId".to_string()),
            Some(self.session_id.to_string()),


            Some("msisdn".to_string()),
            Some(self.msisdn.to_string()),


            Some("operator".to_string()),
            Some(self.operator.to_string()),

            // Skipping input in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdSessionEventNewSession value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdSessionEventNewSession {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub client_id: Vec<String>,
            pub app_id: Vec<String>,
            pub session_id: Vec<String>,
            pub msisdn: Vec<String>,
            pub operator: Vec<String>,
            pub input: Vec<models::UssdSessionEventNewSessionSessionInput>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdSessionEventNewSession".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "clientId" => intermediate_rep.client_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "appId" => intermediate_rep.app_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "sessionId" => intermediate_rep.session_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "msisdn" => intermediate_rep.msisdn.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "operator" => intermediate_rep.operator.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "input" => intermediate_rep.input.push(<models::UssdSessionEventNewSessionSessionInput as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdSessionEventNewSession".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdSessionEventNewSession {
            client_id: intermediate_rep.client_id.into_iter().next().ok_or_else(|| "clientId missing in UssdSessionEventNewSession".to_string())?,
            app_id: intermediate_rep.app_id.into_iter().next().ok_or_else(|| "appId missing in UssdSessionEventNewSession".to_string())?,
            session_id: intermediate_rep.session_id.into_iter().next().ok_or_else(|| "sessionId missing in UssdSessionEventNewSession".to_string())?,
            msisdn: intermediate_rep.msisdn.into_iter().next().ok_or_else(|| "msisdn missing in UssdSessionEventNewSession".to_string())?,
            operator: intermediate_rep.operator.into_iter().next().ok_or_else(|| "operator missing in UssdSessionEventNewSession".to_string())?,
            input: intermediate_rep.input.into_iter().next().ok_or_else(|| "input missing in UssdSessionEventNewSession".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdSessionEventNewSession> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdSessionEventNewSession>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdSessionEventNewSession>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdSessionEventNewSession - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdSessionEventNewSession> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdSessionEventNewSession as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdSessionEventNewSession - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input provided when starting the session.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdSessionEventNewSessionSessionInput {
    UssdSessionEventNewSessionSessionInputOneOf(models::UssdSessionEventNewSessionSessionInputOneOf),
    UssdSessionEventNewSessionSessionInputOneOf1(models::UssdSessionEventNewSessionSessionInputOneOf1),
    UssdSessionEventNewSessionSessionInputOneOf2(models::UssdSessionEventNewSessionSessionInputOneOf2),
}

impl validator::Validate for UssdSessionEventNewSessionSessionInput
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::UssdSessionEventNewSessionSessionInputOneOf(v) => v.validate(),
            Self::UssdSessionEventNewSessionSessionInputOneOf1(v) => v.validate(),
            Self::UssdSessionEventNewSessionSessionInputOneOf2(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdSessionEventNewSessionSessionInput value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdSessionEventNewSessionSessionInput {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::UssdSessionEventNewSessionSessionInputOneOf> for UssdSessionEventNewSessionSessionInput {
    fn from(value: models::UssdSessionEventNewSessionSessionInputOneOf) -> Self {
        Self::UssdSessionEventNewSessionSessionInputOneOf(value)
    }
}
impl From<models::UssdSessionEventNewSessionSessionInputOneOf1> for UssdSessionEventNewSessionSessionInput {
    fn from(value: models::UssdSessionEventNewSessionSessionInputOneOf1) -> Self {
        Self::UssdSessionEventNewSessionSessionInputOneOf1(value)
    }
}
impl From<models::UssdSessionEventNewSessionSessionInputOneOf2> for UssdSessionEventNewSessionSessionInput {
    fn from(value: models::UssdSessionEventNewSessionSessionInputOneOf2) -> Self {
        Self::UssdSessionEventNewSessionSessionInputOneOf2(value)
    }
}





#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdSessionEventNewSessionSessionInputOneOf {
    #[serde(rename = "Dial")]
          #[validate(nested)]
    pub dial: models::NewSessionSessionInputDial,

}



impl UssdSessionEventNewSessionSessionInputOneOf {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(dial: models::NewSessionSessionInputDial, ) -> UssdSessionEventNewSessionSessionInputOneOf {
        UssdSessionEventNewSessionSessionInputOneOf {
 dial,
        }
    }
}

/// Converts the UssdSessionEventNewSessionSessionInputOneOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdSessionEventNewSessionSessionInputOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping Dial in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdSessionEventNewSessionSessionInputOneOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdSessionEventNewSessionSessionInputOneOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub dial: Vec<models::NewSessionSessionInputDial>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdSessionEventNewSessionSessionInputOneOf".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "Dial" => intermediate_rep.dial.push(<models::NewSessionSessionInputDial as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdSessionEventNewSessionSessionInputOneOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdSessionEventNewSessionSessionInputOneOf {
            dial: intermediate_rep.dial.into_iter().next().ok_or_else(|| "Dial missing in UssdSessionEventNewSessionSessionInputOneOf".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdSessionEventNewSessionSessionInputOneOf - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdSessionEventNewSessionSessionInputOneOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdSessionEventNewSessionSessionInputOneOf - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdSessionEventNewSessionSessionInputOneOf1 {
    #[serde(rename = "Push")]
          #[validate(nested)]
    pub push: models::NewSessionSessionInputPush,

}



impl UssdSessionEventNewSessionSessionInputOneOf1 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(push: models::NewSessionSessionInputPush, ) -> UssdSessionEventNewSessionSessionInputOneOf1 {
        UssdSessionEventNewSessionSessionInputOneOf1 {
 push,
        }
    }
}

/// Converts the UssdSessionEventNewSessionSessionInputOneOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdSessionEventNewSessionSessionInputOneOf1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping Push in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdSessionEventNewSessionSessionInputOneOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdSessionEventNewSessionSessionInputOneOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub push: Vec<models::NewSessionSessionInputPush>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdSessionEventNewSessionSessionInputOneOf1".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "Push" => intermediate_rep.push.push(<models::NewSessionSessionInputPush as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdSessionEventNewSessionSessionInputOneOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdSessionEventNewSessionSessionInputOneOf1 {
            push: intermediate_rep.push.into_iter().next().ok_or_else(|| "Push missing in UssdSessionEventNewSessionSessionInputOneOf1".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf1> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf1>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdSessionEventNewSessionSessionInputOneOf1 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf1> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdSessionEventNewSessionSessionInputOneOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdSessionEventNewSessionSessionInputOneOf1 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdSessionEventNewSessionSessionInputOneOf2 {
    #[serde(rename = "Redirect")]
          #[validate(nested)]
    pub redirect: models::NewSessionSessionInputRedirect,

}



impl UssdSessionEventNewSessionSessionInputOneOf2 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(redirect: models::NewSessionSessionInputRedirect, ) -> UssdSessionEventNewSessionSessionInputOneOf2 {
        UssdSessionEventNewSessionSessionInputOneOf2 {
 redirect,
        }
    }
}

/// Converts the UssdSessionEventNewSessionSessionInputOneOf2 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdSessionEventNewSessionSessionInputOneOf2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping Redirect in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdSessionEventNewSessionSessionInputOneOf2 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdSessionEventNewSessionSessionInputOneOf2 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub redirect: Vec<models::NewSessionSessionInputRedirect>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdSessionEventNewSessionSessionInputOneOf2".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "Redirect" => intermediate_rep.redirect.push(<models::NewSessionSessionInputRedirect as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdSessionEventNewSessionSessionInputOneOf2".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdSessionEventNewSessionSessionInputOneOf2 {
            redirect: intermediate_rep.redirect.into_iter().next().ok_or_else(|| "Redirect missing in UssdSessionEventNewSessionSessionInputOneOf2".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf2> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf2>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf2>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdSessionEventNewSessionSessionInputOneOf2 - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdSessionEventNewSessionSessionInputOneOf2> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdSessionEventNewSessionSessionInputOneOf2 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdSessionEventNewSessionSessionInputOneOf2 - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdView {
    UssdViewChooserView(models::UssdViewChooserView),
    UssdViewInfoView(models::UssdViewInfoView),
    UssdViewInputView(models::UssdViewInputView),
}

impl validator::Validate for UssdView
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::UssdViewChooserView(v) => v.validate(),
            Self::UssdViewInfoView(v) => v.validate(),
            Self::UssdViewInputView(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdView {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::UssdViewChooserView> for UssdView {
    fn from(value: models::UssdViewChooserView) -> Self {
        Self::UssdViewChooserView(value)
    }
}
impl From<models::UssdViewInfoView> for UssdView {
    fn from(value: models::UssdViewInfoView) -> Self {
        Self::UssdViewInfoView(value)
    }
}
impl From<models::UssdViewInputView> for UssdView {
    fn from(value: models::UssdViewInputView) -> Self {
        Self::UssdViewInputView(value)
    }
}





/// A chooser (menu) view. User can choose one of the chooser items. If user picks invalid option (when user input does not match any of the available items), then the user will be asked to try again. It will be done internally by the USSD API without involving developer's app. It's guaranteed, that the user input sent in next request to the developer's app will match one of the provided items. The session will continue when this message is sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdViewChooserView {
    /// Text that will be displayed before the chooser items
    #[serde(rename = "title")]
          #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    /// Chooser items, each will be presented to user in a new line. All items must have unique access keys.
    #[serde(rename = "items")]
    #[validate(
            length(min = 1),
          nested,
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Vec<models::UssdViewChooserViewItem>>,

    /// Text, that separates each item's accessKey and label when the ChooserView is rendered
    #[serde(rename = "separator")]
    #[validate(
            length(min = 1),
          custom(function = "check_xss_string"),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub separator: Option<String>,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl UssdViewChooserView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, r_type: String, ) -> UssdViewChooserView {
        UssdViewChooserView {
 title,
 items: None,
 separator: None,
 r_type,
        }
    }
}

/// Converts the UssdViewChooserView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdViewChooserView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("title".to_string()),
            Some(self.title.to_string()),

            // Skipping items in query parameter serialization


            self.separator.as_ref().map(|separator| {
                [
                    "separator".to_string(),
                    separator.to_string(),
                ].join(",")
            }),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdViewChooserView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdViewChooserView {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub title: Vec<String>,
            pub items: Vec<Vec<models::UssdViewChooserViewItem>>,
            pub separator: Vec<String>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdViewChooserView".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "items" => return std::result::Result::Err("Parsing a container in this style is not supported in UssdViewChooserView".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "separator" => intermediate_rep.separator.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdViewChooserView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdViewChooserView {
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in UssdViewChooserView".to_string())?,
            items: intermediate_rep.items.into_iter().next(),
            separator: intermediate_rep.separator.into_iter().next(),
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdViewChooserView".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdViewChooserView> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdViewChooserView>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdViewChooserView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdViewChooserView - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdViewChooserView> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdViewChooserView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdViewChooserView - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Represents single chooser view item
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdViewChooserViewItem {
    /// Text, that identifies the chooser item. User will have to send this value as input in order to choose the item
    #[serde(rename = "accessKey")]
    #[validate(
            length(min = 1),
          custom(function = "check_xss_string"),
    )]
    pub access_key: String,

    /// Description of the choose item
    #[serde(rename = "label")]
    #[validate(
            length(min = 1),
          custom(function = "check_xss_string"),
    )]
    pub label: String,

}



impl UssdViewChooserViewItem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(access_key: String, label: String, ) -> UssdViewChooserViewItem {
        UssdViewChooserViewItem {
 access_key,
 label,
        }
    }
}

/// Converts the UssdViewChooserViewItem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdViewChooserViewItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("accessKey".to_string()),
            Some(self.access_key.to_string()),


            Some("label".to_string()),
            Some(self.label.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdViewChooserViewItem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdViewChooserViewItem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub access_key: Vec<String>,
            pub label: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdViewChooserViewItem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "accessKey" => intermediate_rep.access_key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "label" => intermediate_rep.label.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdViewChooserViewItem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdViewChooserViewItem {
            access_key: intermediate_rep.access_key.into_iter().next().ok_or_else(|| "accessKey missing in UssdViewChooserViewItem".to_string())?,
            label: intermediate_rep.label.into_iter().next().ok_or_else(|| "label missing in UssdViewChooserViewItem".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdViewChooserViewItem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdViewChooserViewItem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdViewChooserViewItem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdViewChooserViewItem - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdViewChooserViewItem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdViewChooserViewItem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdViewChooserViewItem - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// A text-only view; does not take any user input. The session will be closed when this message is sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdViewInfoView {
    /// Final message that will be displayed to the user
    #[serde(rename = "message")]
          #[validate(custom(function = "check_xss_string"))]
    pub message: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl UssdViewInfoView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(message: String, r_type: String, ) -> UssdViewInfoView {
        UssdViewInfoView {
 message,
 r_type,
        }
    }
}

/// Converts the UssdViewInfoView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdViewInfoView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("message".to_string()),
            Some(self.message.to_string()),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdViewInfoView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdViewInfoView {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdViewInfoView".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdViewInfoView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdViewInfoView {
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in UssdViewInfoView".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdViewInfoView".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdViewInfoView> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdViewInfoView>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdViewInfoView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdViewInfoView - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdViewInfoView> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdViewInfoView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdViewInfoView - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// A view with text; takes user input. The session will continue when this message is sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdViewInputView {
    /// Message that will be displayed to the user, for example asking the user to input some value
    #[serde(rename = "message")]
          #[validate(custom(function = "check_xss_string"))]
    pub message: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl UssdViewInputView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(message: String, r_type: String, ) -> UssdViewInputView {
        UssdViewInputView {
 message,
 r_type,
        }
    }
}

/// Converts the UssdViewInputView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdViewInputView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("message".to_string()),
            Some(self.message.to_string()),


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UssdViewInputView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdViewInputView {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UssdViewInputView".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdViewInputView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdViewInputView {
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in UssdViewInputView".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdViewInputView".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdViewInputView> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdViewInputView>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdViewInputView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdViewInputView - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdViewInputView> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdViewInputView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdViewInputView - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}


