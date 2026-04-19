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




/// Abandon - the user leaves the session (e.g. the user presses \"Cancel\" on the phone).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Abandon {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Abandon::_name_for_r_type")]
    #[serde(serialize_with = "Abandon::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl Abandon {
    fn _name_for_r_type() -> String {
        String::from("Abandon")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Abandon {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Abandon {
        Abandon {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the Abandon value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Abandon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Abandon value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Abandon {
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
                None => return std::result::Result::Err("Missing value while parsing Abandon".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Abandon".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Abandon {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Abandon".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Abandon> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Abandon>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Abandon>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Abandon - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Abandon> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Abandon as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Abandon - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
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
    DuplicatedOperation(models::DuplicatedOperation),
    #[serde(alias = "InsufficientBalanceInVirtualPurse")]
    InsufficientBalanceInVirtualPurse(models::InsufficientBalanceInVirtualPurse),
    #[serde(alias = "InternalError")]
    InternalError(models::InternalError),
    #[serde(alias = "MissingPrivilege")]
    MissingPrivilege(models::MissingPrivilege),
    #[serde(alias = "UnexpectedUssdAppResponse")]
    UnexpectedUssdAppResponse(models::UnexpectedUssdAppResponse),
}

impl validator::Validate for AbortSessionAbortReason
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::DuplicatedOperation(v) => v.validate(),
            Self::InsufficientBalanceInVirtualPurse(v) => v.validate(),
            Self::InternalError(v) => v.validate(),
            Self::MissingPrivilege(v) => v.validate(),
            Self::UnexpectedUssdAppResponse(v) => v.validate(),
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
                Self::DuplicatedOperation(x) => x.serialize(serializer),
                Self::InsufficientBalanceInVirtualPurse(x) => x.serialize(serializer),
                Self::InternalError(x) => x.serialize(serializer),
                Self::MissingPrivilege(x) => x.serialize(serializer),
                Self::UnexpectedUssdAppResponse(x) => x.serialize(serializer),
            }
    }
}

impl From<models::DuplicatedOperation> for AbortSessionAbortReason {
    fn from(value: models::DuplicatedOperation) -> Self {
        Self::DuplicatedOperation(value)
    }
}
impl From<models::InsufficientBalanceInVirtualPurse> for AbortSessionAbortReason {
    fn from(value: models::InsufficientBalanceInVirtualPurse) -> Self {
        Self::InsufficientBalanceInVirtualPurse(value)
    }
}
impl From<models::InternalError> for AbortSessionAbortReason {
    fn from(value: models::InternalError) -> Self {
        Self::InternalError(value)
    }
}
impl From<models::MissingPrivilege> for AbortSessionAbortReason {
    fn from(value: models::MissingPrivilege) -> Self {
        Self::MissingPrivilege(value)
    }
}
impl From<models::UnexpectedUssdAppResponse> for AbortSessionAbortReason {
    fn from(value: models::UnexpectedUssdAppResponse) -> Self {
        Self::UnexpectedUssdAppResponse(value)
    }
}





/// A chooser (menu) view. User can choose one of the chooser items. If user picks invalid option (when user input does not match any of the available items), then the user will be asked to try again. It will be done internally by the USSD API without involving developer's app. It's guaranteed, that the user input sent in next request to the developer's app will match one of the provided items. The session will continue when this message is sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ChooserView {
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
    #[serde(default = "ChooserView::_name_for_r_type")]
    #[serde(serialize_with = "ChooserView::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl ChooserView {
    fn _name_for_r_type() -> String {
        String::from("ChooserView")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl ChooserView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, ) -> ChooserView {
        ChooserView {
 title,
 items: None,
 separator: None,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the ChooserView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ChooserView {
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

/// Converts Query Parameters representation (style=form, explode=false) to a ChooserView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ChooserView {
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
                None => return std::result::Result::Err("Missing value while parsing ChooserView".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "items" => return std::result::Result::Err("Parsing a container in this style is not supported in ChooserView".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "separator" => intermediate_rep.separator.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ChooserView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ChooserView {
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in ChooserView".to_string())?,
            items: intermediate_rep.items.into_iter().next(),
            separator: intermediate_rep.separator.into_iter().next(),
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in ChooserView".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ChooserView> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ChooserView>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ChooserView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ChooserView - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ChooserView> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ChooserView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ChooserView - {err}"#))
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
    Abandon(models::Abandon),
    #[serde(alias = "End")]
    End(models::End),
    #[serde(alias = "Timeout")]
    Timeout(models::Timeout),
}

impl validator::Validate for CloseSessionCloseReason
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::Abandon(v) => v.validate(),
            Self::End(v) => v.validate(),
            Self::Timeout(v) => v.validate(),
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
                Self::Abandon(x) => x.serialize(serializer),
                Self::End(x) => x.serialize(serializer),
                Self::Timeout(x) => x.serialize(serializer),
            }
    }
}

impl From<models::Abandon> for CloseSessionCloseReason {
    fn from(value: models::Abandon) -> Self {
        Self::Abandon(value)
    }
}
impl From<models::End> for CloseSessionCloseReason {
    fn from(value: models::End) -> Self {
        Self::End(value)
    }
}
impl From<models::Timeout> for CloseSessionCloseReason {
    fn from(value: models::Timeout) -> Self {
        Self::Timeout(value)
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



/// Input provided when the user dials into the session (dials the shortcode string)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Dial {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Dial::_name_for_r_type")]
    #[serde(serialize_with = "Dial::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Shortcode string dialed by the user
    #[serde(rename = "shortcodeString")]
          #[validate(custom(function = "check_xss_string"))]
    pub shortcode_string: String,

}

impl Dial {
    fn _name_for_r_type() -> String {
        String::from("Dial")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Dial {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(shortcode_string: String, ) -> Dial {
        Dial {
 r_type: Self::_name_for_r_type(),
 shortcode_string,
        }
    }
}

/// Converts the Dial value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Dial {
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

/// Converts Query Parameters representation (style=form, explode=false) to a Dial value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Dial {
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
                None => return std::result::Result::Err("Missing value while parsing Dial".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "shortcodeString" => intermediate_rep.shortcode_string.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Dial".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Dial {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Dial".to_string())?,
            shortcode_string: intermediate_rep.shortcode_string.into_iter().next().ok_or_else(|| "shortcodeString missing in Dial".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Dial> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Dial>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Dial>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Dial - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Dial> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Dial as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Dial - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - operation, eg. Merchant Payment, was previously invoked with given identifier.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DuplicatedOperation {
    #[serde(rename = "operationId")]
          #[validate(custom(function = "check_xss_string"))]
    pub operation_id: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "DuplicatedOperation::_name_for_r_type")]
    #[serde(serialize_with = "DuplicatedOperation::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl DuplicatedOperation {
    fn _name_for_r_type() -> String {
        String::from("DuplicatedOperation")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl DuplicatedOperation {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(operation_id: String, ) -> DuplicatedOperation {
        DuplicatedOperation {
 operation_id,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the DuplicatedOperation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DuplicatedOperation {
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

/// Converts Query Parameters representation (style=form, explode=false) to a DuplicatedOperation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DuplicatedOperation {
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
                None => return std::result::Result::Err("Missing value while parsing DuplicatedOperation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "operationId" => intermediate_rep.operation_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DuplicatedOperation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DuplicatedOperation {
            operation_id: intermediate_rep.operation_id.into_iter().next().ok_or_else(|| "operationId missing in DuplicatedOperation".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in DuplicatedOperation".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DuplicatedOperation> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DuplicatedOperation>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DuplicatedOperation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for DuplicatedOperation - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DuplicatedOperation> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DuplicatedOperation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into DuplicatedOperation - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Initiate an embedded process. This action causes the Qrios system to hand over control to an external application. The user may be prompted to provide credentials during the embedded process. After the embedded process is complete, the ussdSessionEvent/continue API endpoint is called to deliver the result and return control of the session to the developer application.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EmbeddedProcess {
    /// embedded process id
    #[serde(rename = "processId")]
          #[validate(custom(function = "check_xss_string"))]
    pub process_id: String,

    #[serde(rename = "params")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub params: std::collections::HashMap<String, String>,

    #[serde(default = "EmbeddedProcess::_name_for_r_type")]
    #[serde(serialize_with = "EmbeddedProcess::_serialize_r_type")]
    #[serde(rename = "type")]
    pub r_type: String,

}

impl EmbeddedProcess {
    fn _name_for_r_type() -> String {
        String::from("EmbeddedProcess")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl EmbeddedProcess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(process_id: String, params: std::collections::HashMap<String, String>, ) -> EmbeddedProcess {
        EmbeddedProcess {
 process_id,
 params,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the EmbeddedProcess value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for EmbeddedProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("processId".to_string()),
            Some(self.process_id.to_string()),

            // Skipping params in query parameter serialization


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EmbeddedProcess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EmbeddedProcess {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub process_id: Vec<String>,
            pub params: Vec<std::collections::HashMap<String, String>>,
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing EmbeddedProcess".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "processId" => intermediate_rep.process_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "params" => return std::result::Result::Err("Parsing a container in this style is not supported in EmbeddedProcess".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing EmbeddedProcess".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EmbeddedProcess {
            process_id: intermediate_rep.process_id.into_iter().next().ok_or_else(|| "processId missing in EmbeddedProcess".to_string())?,
            params: intermediate_rep.params.into_iter().next().ok_or_else(|| "params missing in EmbeddedProcess".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in EmbeddedProcess".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EmbeddedProcess> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<EmbeddedProcess>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EmbeddedProcess>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for EmbeddedProcess - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<EmbeddedProcess> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EmbeddedProcess as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into EmbeddedProcess - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Returns control of the session back to the developer and passes the result of embedded process execution.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EmbeddedProcessResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "EmbeddedProcessResult::_name_for_r_type")]
    #[serde(serialize_with = "EmbeddedProcessResult::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Result of the `embedded process`. Keys should be considered as result parameter names and their values as result parameter values.
    #[serde(rename = "resultParams")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub result_params: std::collections::HashMap<String, String>,

}

impl EmbeddedProcessResult {
    fn _name_for_r_type() -> String {
        String::from("EmbeddedProcessResult")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl EmbeddedProcessResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(result_params: std::collections::HashMap<String, String>, ) -> EmbeddedProcessResult {
        EmbeddedProcessResult {
 r_type: Self::_name_for_r_type(),
 result_params,
        }
    }
}

/// Converts the EmbeddedProcessResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for EmbeddedProcessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

            // Skipping resultParams in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EmbeddedProcessResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EmbeddedProcessResult {
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
                None => return std::result::Result::Err("Missing value while parsing EmbeddedProcessResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "resultParams" => return std::result::Result::Err("Parsing a container in this style is not supported in EmbeddedProcessResult".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing EmbeddedProcessResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EmbeddedProcessResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in EmbeddedProcessResult".to_string())?,
            result_params: intermediate_rep.result_params.into_iter().next().ok_or_else(|| "resultParams missing in EmbeddedProcessResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EmbeddedProcessResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<EmbeddedProcessResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EmbeddedProcessResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for EmbeddedProcessResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<EmbeddedProcessResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EmbeddedProcessResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into EmbeddedProcessResult - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// End - the session ends naturally (e.g. the user can no longer input data).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct End {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "End::_name_for_r_type")]
    #[serde(serialize_with = "End::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl End {
    fn _name_for_r_type() -> String {
        String::from("End")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl End {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> End {
        End {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the End value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for End {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a End value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for End {
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
                None => return std::result::Result::Err("Missing value while parsing End".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing End".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(End {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in End".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<End> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<End>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<End>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for End - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<End> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <End as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into End - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Merchant payment operation failed (there was no charge for sure)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Failure {
    #[serde(rename = "cause")]
          #[validate(nested)]
    pub cause: models::MerchantPaymentResultOperationStatusFailureCause,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Failure::_name_for_r_type")]
    #[serde(serialize_with = "Failure::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl Failure {
    fn _name_for_r_type() -> String {
        String::from("Failure")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Failure {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(cause: models::MerchantPaymentResultOperationStatusFailureCause, ) -> Failure {
        Failure {
 cause,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the Failure value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping cause in query parameter serialization


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Failure value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Failure {
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
                None => return std::result::Result::Err("Missing value while parsing Failure".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cause" => intermediate_rep.cause.push(<models::MerchantPaymentResultOperationStatusFailureCause as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Failure".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Failure {
            cause: intermediate_rep.cause.into_iter().next().ok_or_else(|| "cause missing in Failure".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Failure".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Failure> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Failure>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Failure>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Failure - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Failure> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Failure as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Failure - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Fixed account mode in merchant payment process, with account ID specified.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FixedAccount {
    /// Number of the account which will be used in transaction. Account numbers can be obtained with Qrios API using `/merchants/accounts` endpoint.
    #[serde(rename = "accountNumber")]
          #[validate(custom(function = "check_xss_string"))]
    pub account_number: String,

    /// Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
    #[serde(rename = "bank")]
          #[validate(custom(function = "check_xss_string"))]
    pub bank: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "FixedAccount::_name_for_r_type")]
    #[serde(serialize_with = "FixedAccount::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl FixedAccount {
    fn _name_for_r_type() -> String {
        String::from("FixedAccount")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl FixedAccount {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(account_number: String, bank: String, ) -> FixedAccount {
        FixedAccount {
 account_number,
 bank,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the FixedAccount value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for FixedAccount {
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

/// Converts Query Parameters representation (style=form, explode=false) to a FixedAccount value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FixedAccount {
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
                None => return std::result::Result::Err("Missing value while parsing FixedAccount".to_string())
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
                    _ => return std::result::Result::Err("Unexpected key while parsing FixedAccount".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FixedAccount {
            account_number: intermediate_rep.account_number.into_iter().next().ok_or_else(|| "accountNumber missing in FixedAccount".to_string())?,
            bank: intermediate_rep.bank.into_iter().next().ok_or_else(|| "bank missing in FixedAccount".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in FixedAccount".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FixedAccount> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<FixedAccount>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FixedAccount>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for FixedAccount - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<FixedAccount> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FixedAccount as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into FixedAccount - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Fixed bank mode in merchant payment process, with bank ID specified. The account from provided bank will then be selected within the process.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FixedBank {
    /// Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
    #[serde(rename = "bank")]
          #[validate(custom(function = "check_xss_string"))]
    pub bank: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "FixedBank::_name_for_r_type")]
    #[serde(serialize_with = "FixedBank::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl FixedBank {
    fn _name_for_r_type() -> String {
        String::from("FixedBank")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl FixedBank {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(bank: String, ) -> FixedBank {
        FixedBank {
 bank,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the FixedBank value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for FixedBank {
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

/// Converts Query Parameters representation (style=form, explode=false) to a FixedBank value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FixedBank {
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
                None => return std::result::Result::Err("Missing value while parsing FixedBank".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "bank" => intermediate_rep.bank.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FixedBank".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FixedBank {
            bank: intermediate_rep.bank.into_iter().next().ok_or_else(|| "bank missing in FixedBank".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in FixedBank".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FixedBank> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<FixedBank>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FixedBank>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for FixedBank - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<FixedBank> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FixedBank as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into FixedBank - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Flexible payment mode in merchant payment process. The account will then be selected within the process.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Flexible {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Flexible::_name_for_r_type")]
    #[serde(serialize_with = "Flexible::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl Flexible {
    fn _name_for_r_type() -> String {
        String::from("Flexible")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Flexible {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Flexible {
        Flexible {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the Flexible value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Flexible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Flexible value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Flexible {
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
                None => return std::result::Result::Err("Missing value while parsing Flexible".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Flexible".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Flexible {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Flexible".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Flexible> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Flexible>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Flexible>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Flexible - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Flexible> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Flexible as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Flexible - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// A text-only view; does not take any user input. The session will be closed when this message is sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InfoView {
    /// Final message that will be displayed to the user
    #[serde(rename = "message")]
          #[validate(custom(function = "check_xss_string"))]
    pub message: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "InfoView::_name_for_r_type")]
    #[serde(serialize_with = "InfoView::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl InfoView {
    fn _name_for_r_type() -> String {
        String::from("InfoView")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl InfoView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(message: String, ) -> InfoView {
        InfoView {
 message,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the InfoView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InfoView {
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

/// Converts Query Parameters representation (style=form, explode=false) to a InfoView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InfoView {
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
                None => return std::result::Result::Err("Missing value while parsing InfoView".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InfoView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InfoView {
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in InfoView".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in InfoView".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InfoView> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InfoView>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InfoView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InfoView - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InfoView> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InfoView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InfoView - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input from the user; provided by UssdAction.UssdView.InputView.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InputResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "InputResult::_name_for_r_type")]
    #[serde(serialize_with = "InputResult::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Raw value typed in by the user.
    #[serde(rename = "value")]
          #[validate(custom(function = "check_xss_string"))]
    pub value: String,

}

impl InputResult {
    fn _name_for_r_type() -> String {
        String::from("InputResult")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl InputResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(value: String, ) -> InputResult {
        InputResult {
 r_type: Self::_name_for_r_type(),
 value,
        }
    }
}

/// Converts the InputResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InputResult {
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

/// Converts Query Parameters representation (style=form, explode=false) to a InputResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InputResult {
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
                None => return std::result::Result::Err("Missing value while parsing InputResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InputResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InputResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in InputResult".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in InputResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InputResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InputResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InputResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InputResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InputResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InputResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InputResult - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// A view with text; takes user input. The session will continue when this message is sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InputView {
    /// Message that will be displayed to the user, for example asking the user to input some value
    #[serde(rename = "message")]
          #[validate(custom(function = "check_xss_string"))]
    pub message: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "InputView::_name_for_r_type")]
    #[serde(serialize_with = "InputView::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl InputView {
    fn _name_for_r_type() -> String {
        String::from("InputView")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl InputView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(message: String, ) -> InputView {
        InputView {
 message,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the InputView value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InputView {
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

/// Converts Query Parameters representation (style=form, explode=false) to a InputView value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InputView {
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
                None => return std::result::Result::Err("Missing value while parsing InputView".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InputView".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InputView {
            message: intermediate_rep.message.into_iter().next().ok_or_else(|| "message missing in InputView".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in InputView".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InputView> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InputView>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InputView>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InputView - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InputView> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InputView as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InputView - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - not enough funds to fulfil the request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InsufficientBalanceInVirtualPurse {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "InsufficientBalanceInVirtualPurse::_name_for_r_type")]
    #[serde(serialize_with = "InsufficientBalanceInVirtualPurse::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl InsufficientBalanceInVirtualPurse {
    fn _name_for_r_type() -> String {
        String::from("InsufficientBalanceInVirtualPurse")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl InsufficientBalanceInVirtualPurse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InsufficientBalanceInVirtualPurse {
        InsufficientBalanceInVirtualPurse {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the InsufficientBalanceInVirtualPurse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InsufficientBalanceInVirtualPurse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InsufficientBalanceInVirtualPurse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InsufficientBalanceInVirtualPurse {
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
                None => return std::result::Result::Err("Missing value while parsing InsufficientBalanceInVirtualPurse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InsufficientBalanceInVirtualPurse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InsufficientBalanceInVirtualPurse {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in InsufficientBalanceInVirtualPurse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InsufficientBalanceInVirtualPurse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InsufficientBalanceInVirtualPurse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InsufficientBalanceInVirtualPurse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InsufficientBalanceInVirtualPurse - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InsufficientBalanceInVirtualPurse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InsufficientBalanceInVirtualPurse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InsufficientBalanceInVirtualPurse - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - there was some internal error in Qrios API.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InternalError {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "InternalError::_name_for_r_type")]
    #[serde(serialize_with = "InternalError::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl InternalError {
    fn _name_for_r_type() -> String {
        String::from("InternalError")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl InternalError {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> InternalError {
        InternalError {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the InternalError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InternalError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InternalError {
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
                None => return std::result::Result::Err("Missing value while parsing InternalError".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InternalError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InternalError {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in InternalError".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InternalError> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InternalError>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InternalError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InternalError - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InternalError> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InternalError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InternalError - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Developer's USSD App requests a redirect to some Legacy USSD App
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LegacyAppRedirect {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "LegacyAppRedirect::_name_for_r_type")]
    #[serde(serialize_with = "LegacyAppRedirect::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Url of the Legacy USSD App
    #[serde(rename = "uri")]
          #[validate(custom(function = "check_xss_string"))]
    pub uri: String,

}

impl LegacyAppRedirect {
    fn _name_for_r_type() -> String {
        String::from("LegacyAppRedirect")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl LegacyAppRedirect {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(uri: String, ) -> LegacyAppRedirect {
        LegacyAppRedirect {
 r_type: Self::_name_for_r_type(),
 uri,
        }
    }
}

/// Converts the LegacyAppRedirect value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for LegacyAppRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("uri".to_string()),
            Some(self.uri.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LegacyAppRedirect value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LegacyAppRedirect {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub uri: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LegacyAppRedirect".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "uri" => intermediate_rep.uri.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LegacyAppRedirect".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LegacyAppRedirect {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in LegacyAppRedirect".to_string())?,
            uri: intermediate_rep.uri.into_iter().next().ok_or_else(|| "uri missing in LegacyAppRedirect".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LegacyAppRedirect> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<LegacyAppRedirect>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LegacyAppRedirect>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for LegacyAppRedirect - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<LegacyAppRedirect> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LegacyAppRedirect as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into LegacyAppRedirect - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
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



/// Initiation of merchant payment process. In this process, the control is handed over to an external USSD merchant payment process. The user will be asked for credentials and the payment will commence. As the process ends, it will call this API to deliver the result and hand control over the session back to the server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentProcess {
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

    #[serde(default = "MerchantPaymentProcess::_name_for_r_type")]
    #[serde(serialize_with = "MerchantPaymentProcess::_serialize_r_type")]
    #[serde(rename = "type")]
    pub r_type: String,

}

impl MerchantPaymentProcess {
    fn _name_for_r_type() -> String {
        String::from("MerchantPaymentProcess")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl MerchantPaymentProcess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(operation_id: String, merchant_code: String, amount: f64, payment_mode: models::MerchantPaymentProcessPaymentMode, ) -> MerchantPaymentProcess {
        MerchantPaymentProcess {
 operation_id,
 merchant_code,
 amount,
 payment_mode,
 execution_mode: None,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the MerchantPaymentProcess value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentProcess {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentProcess value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentProcess {
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
            pub r_type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentProcess".to_string())
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
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentProcess".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentProcess {
            operation_id: intermediate_rep.operation_id.into_iter().next().ok_or_else(|| "operationId missing in MerchantPaymentProcess".to_string())?,
            merchant_code: intermediate_rep.merchant_code.into_iter().next().ok_or_else(|| "merchantCode missing in MerchantPaymentProcess".to_string())?,
            amount: intermediate_rep.amount.into_iter().next().ok_or_else(|| "amount missing in MerchantPaymentProcess".to_string())?,
            payment_mode: intermediate_rep.payment_mode.into_iter().next().ok_or_else(|| "paymentMode missing in MerchantPaymentProcess".to_string())?,
            execution_mode: intermediate_rep.execution_mode.into_iter().next(),
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentProcess".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentProcess> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentProcess>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentProcess>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentProcess - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentProcess> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentProcess as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentProcess - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum MerchantPaymentProcessExecutionMode {
    #[serde(alias = "WithBankResponseTimeout")]
    WithBankResponseTimeout(models::WithBankResponseTimeout),
    #[serde(alias = "WithoutWaitingForBank")]
    WithoutWaitingForBank(models::WithoutWaitingForBank),
}

impl validator::Validate for MerchantPaymentProcessExecutionMode
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::WithBankResponseTimeout(v) => v.validate(),
            Self::WithoutWaitingForBank(v) => v.validate(),
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
                Self::WithBankResponseTimeout(x) => x.serialize(serializer),
                Self::WithoutWaitingForBank(x) => x.serialize(serializer),
            }
    }
}

impl From<models::WithBankResponseTimeout> for MerchantPaymentProcessExecutionMode {
    fn from(value: models::WithBankResponseTimeout) -> Self {
        Self::WithBankResponseTimeout(value)
    }
}
impl From<models::WithoutWaitingForBank> for MerchantPaymentProcessExecutionMode {
    fn from(value: models::WithoutWaitingForBank) -> Self {
        Self::WithoutWaitingForBank(value)
    }
}





#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum MerchantPaymentProcessPaymentMode {
    #[serde(alias = "FixedAccount")]
    FixedAccount(models::FixedAccount),
    #[serde(alias = "FixedBank")]
    FixedBank(models::FixedBank),
    #[serde(alias = "Flexible")]
    Flexible(models::Flexible),
}

impl validator::Validate for MerchantPaymentProcessPaymentMode
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::FixedAccount(v) => v.validate(),
            Self::FixedBank(v) => v.validate(),
            Self::Flexible(v) => v.validate(),
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
                Self::FixedAccount(x) => x.serialize(serializer),
                Self::FixedBank(x) => x.serialize(serializer),
                Self::Flexible(x) => x.serialize(serializer),
            }
    }
}

impl From<models::FixedAccount> for MerchantPaymentProcessPaymentMode {
    fn from(value: models::FixedAccount) -> Self {
        Self::FixedAccount(value)
    }
}
impl From<models::FixedBank> for MerchantPaymentProcessPaymentMode {
    fn from(value: models::FixedBank) -> Self {
        Self::FixedBank(value)
    }
}
impl From<models::Flexible> for MerchantPaymentProcessPaymentMode {
    fn from(value: models::Flexible) -> Self {
        Self::Flexible(value)
    }
}





/// Result from merchant payment process; provided by UssdProcess.MerchantPaymentProcess.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MerchantPaymentResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "MerchantPaymentResult::_name_for_r_type")]
    #[serde(serialize_with = "MerchantPaymentResult::_serialize_r_type")]
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

impl MerchantPaymentResult {
    fn _name_for_r_type() -> String {
        String::from("MerchantPaymentResult")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl MerchantPaymentResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(operation_id: String, status: models::MerchantPaymentResultOperationStatus, ) -> MerchantPaymentResult {
        MerchantPaymentResult {
 r_type: Self::_name_for_r_type(),
 operation_id,
 status,
        }
    }
}

/// Converts the MerchantPaymentResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MerchantPaymentResult {
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

/// Converts Query Parameters representation (style=form, explode=false) to a MerchantPaymentResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MerchantPaymentResult {
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
                None => return std::result::Result::Err("Missing value while parsing MerchantPaymentResult".to_string())
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
                    _ => return std::result::Result::Err("Unexpected key while parsing MerchantPaymentResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MerchantPaymentResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MerchantPaymentResult".to_string())?,
            operation_id: intermediate_rep.operation_id.into_iter().next().ok_or_else(|| "operationId missing in MerchantPaymentResult".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in MerchantPaymentResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MerchantPaymentResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MerchantPaymentResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MerchantPaymentResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MerchantPaymentResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MerchantPaymentResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MerchantPaymentResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MerchantPaymentResult - {err}"#))
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
    Failure(models::Failure),
    #[serde(alias = "Success")]
    Success(models::Success),
    #[serde(alias = "Unknown")]
    Unknown(models::Unknown),
}

impl validator::Validate for MerchantPaymentResultOperationStatus
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::Failure(v) => v.validate(),
            Self::Success(v) => v.validate(),
            Self::Unknown(v) => v.validate(),
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
                Self::Failure(x) => x.serialize(serializer),
                Self::Success(x) => x.serialize(serializer),
                Self::Unknown(x) => x.serialize(serializer),
            }
    }
}

impl From<models::Failure> for MerchantPaymentResultOperationStatus {
    fn from(value: models::Failure) -> Self {
        Self::Failure(value)
    }
}
impl From<models::Success> for MerchantPaymentResultOperationStatus {
    fn from(value: models::Success) -> Self {
        Self::Success(value)
    }
}
impl From<models::Unknown> for MerchantPaymentResultOperationStatus {
    fn from(value: models::Unknown) -> Self {
        Self::Unknown(value)
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


/// USSD process abort reason - there are no required privileges to run a USSD operation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MissingPrivilege {
    #[serde(rename = "privilege")]
          #[validate(custom(function = "check_xss_string"))]
    pub privilege: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "MissingPrivilege::_name_for_r_type")]
    #[serde(serialize_with = "MissingPrivilege::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl MissingPrivilege {
    fn _name_for_r_type() -> String {
        String::from("MissingPrivilege")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl MissingPrivilege {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(privilege: String, ) -> MissingPrivilege {
        MissingPrivilege {
 privilege,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the MissingPrivilege value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MissingPrivilege {
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

/// Converts Query Parameters representation (style=form, explode=false) to a MissingPrivilege value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MissingPrivilege {
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
                None => return std::result::Result::Err("Missing value while parsing MissingPrivilege".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "privilege" => intermediate_rep.privilege.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MissingPrivilege".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MissingPrivilege {
            privilege: intermediate_rep.privilege.into_iter().next().ok_or_else(|| "privilege missing in MissingPrivilege".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in MissingPrivilege".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MissingPrivilege> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MissingPrivilege>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MissingPrivilege>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MissingPrivilege - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MissingPrivilege> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MissingPrivilege as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MissingPrivilege - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input provided when session is begun by a push message sent to a user.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Push {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Push::_name_for_r_type")]
    #[serde(serialize_with = "Push::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Context regarding the push message.
    #[serde(rename = "contextData")]
          #[validate(custom(function = "check_xss_string"))]
    pub context_data: String,

}

impl Push {
    fn _name_for_r_type() -> String {
        String::from("Push")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Push {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(context_data: String, ) -> Push {
        Push {
 r_type: Self::_name_for_r_type(),
 context_data,
        }
    }
}

/// Converts the Push value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Push {
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

/// Converts Query Parameters representation (style=form, explode=false) to a Push value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Push {
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
                None => return std::result::Result::Err("Missing value while parsing Push".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contextData" => intermediate_rep.context_data.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Push".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Push {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Push".to_string())?,
            context_data: intermediate_rep.context_data.into_iter().next().ok_or_else(|| "contextData missing in Push".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Push> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Push>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Push>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Push - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Push> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Push as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Push - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Input provided when session is redirected from different USSD application.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Redirect {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Redirect::_name_for_r_type")]
    #[serde(serialize_with = "Redirect::_serialize_r_type")]
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

impl Redirect {
    fn _name_for_r_type() -> String {
        String::from("Redirect")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Redirect {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Redirect {
        Redirect {
 r_type: Self::_name_for_r_type(),
 process_id: None,
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
                None => return std::result::Result::Err("Missing value while parsing Redirect".to_string())
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
                    _ => return std::result::Result::Err("Unexpected key while parsing Redirect".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Redirect {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Redirect".to_string())?,
            process_id: intermediate_rep.process_id.into_iter().next(),
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

    #[serde(default = "ReturnFromRedirect::_name_for_r_type")]
    #[serde(serialize_with = "ReturnFromRedirect::_serialize_r_type")]
    #[serde(rename = "type")]
    pub r_type: String,

}

impl ReturnFromRedirect {
    fn _name_for_r_type() -> String {
        String::from("ReturnFromRedirect")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl ReturnFromRedirect {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(result_params: std::collections::HashMap<String, String>, ) -> ReturnFromRedirect {
        ReturnFromRedirect {
 result_params,
 r_type: Self::_name_for_r_type(),
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),

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
            pub r_type: Vec<String>,
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
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ReturnFromRedirect".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ReturnFromRedirect {
            result_params: intermediate_rep.result_params.into_iter().next().ok_or_else(|| "resultParams missing in ReturnFromRedirect".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in ReturnFromRedirect".to_string())?,
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



/// Result returned from the app, to which the session was previously redirected.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ReturnFromRedirectResult {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "ReturnFromRedirectResult::_name_for_r_type")]
    #[serde(serialize_with = "ReturnFromRedirectResult::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// Result of the `return from redirection`. Keys should be considered as result parameter names and their values as result parameter values.
    #[serde(rename = "resultParams")]
          #[validate(custom(function = "check_xss_map_string"))]
    pub result_params: std::collections::HashMap<String, String>,

}

impl ReturnFromRedirectResult {
    fn _name_for_r_type() -> String {
        String::from("ReturnFromRedirectResult")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl ReturnFromRedirectResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(result_params: std::collections::HashMap<String, String>, ) -> ReturnFromRedirectResult {
        ReturnFromRedirectResult {
 r_type: Self::_name_for_r_type(),
 result_params,
        }
    }
}

/// Converts the ReturnFromRedirectResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ReturnFromRedirectResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

            // Skipping resultParams in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ReturnFromRedirectResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ReturnFromRedirectResult {
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
                None => return std::result::Result::Err("Missing value while parsing ReturnFromRedirectResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "resultParams" => return std::result::Result::Err("Parsing a container in this style is not supported in ReturnFromRedirectResult".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ReturnFromRedirectResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ReturnFromRedirectResult {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in ReturnFromRedirectResult".to_string())?,
            result_params: intermediate_rep.result_params.into_iter().next().ok_or_else(|| "resultParams missing in ReturnFromRedirectResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ReturnFromRedirectResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ReturnFromRedirectResult>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ReturnFromRedirectResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ReturnFromRedirectResult - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ReturnFromRedirectResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ReturnFromRedirectResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ReturnFromRedirectResult - {err}"#))
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
    #[serde(default = "RunProcess::_name_for_r_type")]
    #[serde(serialize_with = "RunProcess::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "process")]
          #[validate(nested)]
    pub process: models::UssdProcess,

}

impl RunProcess {
    fn _name_for_r_type() -> String {
        String::from("RunProcess")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl RunProcess {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(process: models::UssdProcess, ) -> RunProcess {
        RunProcess {
 r_type: Self::_name_for_r_type(),
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
    #[serde(default = "ShowView::_name_for_r_type")]
    #[serde(serialize_with = "ShowView::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "view")]
          #[validate(nested)]
    pub view: models::UssdView,

}

impl ShowView {
    fn _name_for_r_type() -> String {
        String::from("ShowView")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl ShowView {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(view: models::UssdView, ) -> ShowView {
        ShowView {
 r_type: Self::_name_for_r_type(),
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



/// Merchant payment operation finished with success
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Success {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Success::_name_for_r_type")]
    #[serde(serialize_with = "Success::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl Success {
    fn _name_for_r_type() -> String {
        String::from("Success")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Success {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Success {
        Success {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the Success value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Success {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Success value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Success {
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
                None => return std::result::Result::Err("Missing value while parsing Success".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Success".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Success {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Success".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Success> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Success>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Success>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Success - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Success> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Success as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Success - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Timeout - the session is ended by the mobile operator (e.g. after two minutes).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Timeout {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Timeout::_name_for_r_type")]
    #[serde(serialize_with = "Timeout::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl Timeout {
    fn _name_for_r_type() -> String {
        String::from("Timeout")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Timeout {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Timeout {
        Timeout {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the Timeout value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Timeout value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Timeout {
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
                None => return std::result::Result::Err("Missing value while parsing Timeout".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Timeout".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Timeout {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Timeout".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Timeout> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Timeout>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Timeout>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Timeout - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Timeout> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Timeout as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Timeout - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// USSD process abort reason - previous USSD app response was malformed or not expected.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnexpectedUssdAppResponse {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "UnexpectedUssdAppResponse::_name_for_r_type")]
    #[serde(serialize_with = "UnexpectedUssdAppResponse::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl UnexpectedUssdAppResponse {
    fn _name_for_r_type() -> String {
        String::from("UnexpectedUssdAppResponse")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl UnexpectedUssdAppResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> UnexpectedUssdAppResponse {
        UnexpectedUssdAppResponse {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the UnexpectedUssdAppResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UnexpectedUssdAppResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnexpectedUssdAppResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnexpectedUssdAppResponse {
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
                None => return std::result::Result::Err("Missing value while parsing UnexpectedUssdAppResponse".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UnexpectedUssdAppResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnexpectedUssdAppResponse {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UnexpectedUssdAppResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnexpectedUssdAppResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UnexpectedUssdAppResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UnexpectedUssdAppResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UnexpectedUssdAppResponse - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UnexpectedUssdAppResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UnexpectedUssdAppResponse as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UnexpectedUssdAppResponse - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Cannot determine if merchant payment operation succeed or not
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Unknown {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "Unknown::_name_for_r_type")]
    #[serde(serialize_with = "Unknown::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl Unknown {
    fn _name_for_r_type() -> String {
        String::from("Unknown")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl Unknown {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Unknown {
        Unknown {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the Unknown value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Unknown value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Unknown {
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
                None => return std::result::Result::Err("Missing value while parsing Unknown".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Unknown".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Unknown {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Unknown".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Unknown> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Unknown>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Unknown>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Unknown - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Unknown> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Unknown as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Unknown - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Action to be sent back to the user. It can either start a predefined process or show a view.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdAction {
    #[serde(alias = "LegacyAppRedirect")]
    LegacyAppRedirect(models::LegacyAppRedirect),
    #[serde(alias = "Redirect")]
    UssdActionRedirect(models::UssdActionRedirect),
    #[serde(alias = "ReturnFromRedirect")]
    ReturnFromRedirect(models::ReturnFromRedirect),
    #[serde(alias = "RunProcess")]
    RunProcess(models::RunProcess),
    #[serde(alias = "ShowView")]
    ShowView(models::ShowView),
}

impl validator::Validate for UssdAction
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::LegacyAppRedirect(v) => v.validate(),
            Self::UssdActionRedirect(v) => v.validate(),
            Self::ReturnFromRedirect(v) => v.validate(),
            Self::RunProcess(v) => v.validate(),
            Self::ShowView(v) => v.validate(),
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

impl serde::Serialize for UssdAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::LegacyAppRedirect(x) => x.serialize(serializer),
                Self::UssdActionRedirect(x) => x.serialize(serializer),
                Self::ReturnFromRedirect(x) => x.serialize(serializer),
                Self::RunProcess(x) => x.serialize(serializer),
                Self::ShowView(x) => x.serialize(serializer),
            }
    }
}

impl From<models::LegacyAppRedirect> for UssdAction {
    fn from(value: models::LegacyAppRedirect) -> Self {
        Self::LegacyAppRedirect(value)
    }
}
impl From<models::UssdActionRedirect> for UssdAction {
    fn from(value: models::UssdActionRedirect) -> Self {
        Self::UssdActionRedirect(value)
    }
}
impl From<models::ReturnFromRedirect> for UssdAction {
    fn from(value: models::ReturnFromRedirect) -> Self {
        Self::ReturnFromRedirect(value)
    }
}
impl From<models::RunProcess> for UssdAction {
    fn from(value: models::RunProcess) -> Self {
        Self::RunProcess(value)
    }
}
impl From<models::ShowView> for UssdAction {
    fn from(value: models::ShowView) -> Self {
        Self::ShowView(value)
    }
}





/// Developer's USSD App returns USSD process result as a map of params
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UssdActionRedirect {
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



impl UssdActionRedirect {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(r_type: String, destination_app_id: String, ) -> UssdActionRedirect {
        UssdActionRedirect {
 r_type,
 destination_app_id,
 process: None,
        }
    }
}

/// Converts the UssdActionRedirect value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UssdActionRedirect {
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

/// Converts Query Parameters representation (style=form, explode=false) to a UssdActionRedirect value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UssdActionRedirect {
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
                None => return std::result::Result::Err("Missing value while parsing UssdActionRedirect".to_string())
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
                    _ => return std::result::Result::Err("Unexpected key while parsing UssdActionRedirect".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UssdActionRedirect {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UssdActionRedirect".to_string())?,
            destination_app_id: intermediate_rep.destination_app_id.into_iter().next().ok_or_else(|| "destinationAppId missing in UssdActionRedirect".to_string())?,
            process: intermediate_rep.process.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UssdActionRedirect> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UssdActionRedirect>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UssdActionRedirect>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UssdActionRedirect - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UssdActionRedirect> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UssdActionRedirect as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UssdActionRedirect - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Result of USSD action. Either input provided by the user or the result of a previously-initiated process.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdActionResult {
    #[serde(alias = "EmbeddedProcessResult")]
    EmbeddedProcessResult(models::EmbeddedProcessResult),
    #[serde(alias = "InputResult")]
    InputResult(models::InputResult),
    #[serde(alias = "MerchantPaymentResult")]
    MerchantPaymentResult(models::MerchantPaymentResult),
    #[serde(alias = "ReturnFromRedirectResult")]
    ReturnFromRedirectResult(models::ReturnFromRedirectResult),
}

impl validator::Validate for UssdActionResult
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::EmbeddedProcessResult(v) => v.validate(),
            Self::InputResult(v) => v.validate(),
            Self::MerchantPaymentResult(v) => v.validate(),
            Self::ReturnFromRedirectResult(v) => v.validate(),
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

impl serde::Serialize for UssdActionResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::EmbeddedProcessResult(x) => x.serialize(serializer),
                Self::InputResult(x) => x.serialize(serializer),
                Self::MerchantPaymentResult(x) => x.serialize(serializer),
                Self::ReturnFromRedirectResult(x) => x.serialize(serializer),
            }
    }
}

impl From<models::EmbeddedProcessResult> for UssdActionResult {
    fn from(value: models::EmbeddedProcessResult) -> Self {
        Self::EmbeddedProcessResult(value)
    }
}
impl From<models::InputResult> for UssdActionResult {
    fn from(value: models::InputResult) -> Self {
        Self::InputResult(value)
    }
}
impl From<models::MerchantPaymentResult> for UssdActionResult {
    fn from(value: models::MerchantPaymentResult) -> Self {
        Self::MerchantPaymentResult(value)
    }
}
impl From<models::ReturnFromRedirectResult> for UssdActionResult {
    fn from(value: models::ReturnFromRedirectResult) -> Self {
        Self::ReturnFromRedirectResult(value)
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
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdProcess {
    #[serde(alias = "EmbeddedProcess")]
    EmbeddedProcess(models::EmbeddedProcess),
    #[serde(alias = "MerchantPaymentProcess")]
    MerchantPaymentProcess(models::MerchantPaymentProcess),
}

impl validator::Validate for UssdProcess
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::EmbeddedProcess(v) => v.validate(),
            Self::MerchantPaymentProcess(v) => v.validate(),
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

impl serde::Serialize for UssdProcess {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::EmbeddedProcess(x) => x.serialize(serializer),
                Self::MerchantPaymentProcess(x) => x.serialize(serializer),
            }
    }
}

impl From<models::EmbeddedProcess> for UssdProcess {
    fn from(value: models::EmbeddedProcess) -> Self {
        Self::EmbeddedProcess(value)
    }
}
impl From<models::MerchantPaymentProcess> for UssdProcess {
    fn from(value: models::MerchantPaymentProcess) -> Self {
        Self::MerchantPaymentProcess(value)
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
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdSessionEventNewSessionSessionInput {
    #[serde(alias = "Dial")]
    Dial(models::Dial),
    #[serde(alias = "Push")]
    Push(models::Push),
    #[serde(alias = "Redirect")]
    Redirect(models::Redirect),
}

impl validator::Validate for UssdSessionEventNewSessionSessionInput
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::Dial(v) => v.validate(),
            Self::Push(v) => v.validate(),
            Self::Redirect(v) => v.validate(),
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

impl serde::Serialize for UssdSessionEventNewSessionSessionInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::Dial(x) => x.serialize(serializer),
                Self::Push(x) => x.serialize(serializer),
                Self::Redirect(x) => x.serialize(serializer),
            }
    }
}

impl From<models::Dial> for UssdSessionEventNewSessionSessionInput {
    fn from(value: models::Dial) -> Self {
        Self::Dial(value)
    }
}
impl From<models::Push> for UssdSessionEventNewSessionSessionInput {
    fn from(value: models::Push) -> Self {
        Self::Push(value)
    }
}
impl From<models::Redirect> for UssdSessionEventNewSessionSessionInput {
    fn from(value: models::Redirect) -> Self {
        Self::Redirect(value)
    }
}





#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum UssdView {
    #[serde(alias = "ChooserView")]
    ChooserView(models::ChooserView),
    #[serde(alias = "InfoView")]
    InfoView(models::InfoView),
    #[serde(alias = "InputView")]
    InputView(models::InputView),
}

impl validator::Validate for UssdView
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::ChooserView(v) => v.validate(),
            Self::InfoView(v) => v.validate(),
            Self::InputView(v) => v.validate(),
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

impl serde::Serialize for UssdView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
            match self {
                Self::ChooserView(x) => x.serialize(serializer),
                Self::InfoView(x) => x.serialize(serializer),
                Self::InputView(x) => x.serialize(serializer),
            }
    }
}

impl From<models::ChooserView> for UssdView {
    fn from(value: models::ChooserView) -> Self {
        Self::ChooserView(value)
    }
}
impl From<models::InfoView> for UssdView {
    fn from(value: models::InfoView) -> Self {
        Self::InfoView(value)
    }
}
impl From<models::InputView> for UssdView {
    fn from(value: models::InputView) -> Self {
        Self::InputView(value)
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



/// Merchant payment process will use a timeout when finalizing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WithBankResponseTimeout {
    #[serde(rename = "millis")]
    pub millis: i64,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "WithBankResponseTimeout::_name_for_r_type")]
    #[serde(serialize_with = "WithBankResponseTimeout::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl WithBankResponseTimeout {
    fn _name_for_r_type() -> String {
        String::from("WithBankResponseTimeout")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl WithBankResponseTimeout {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(millis: i64, ) -> WithBankResponseTimeout {
        WithBankResponseTimeout {
 millis,
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the WithBankResponseTimeout value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for WithBankResponseTimeout {
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

/// Converts Query Parameters representation (style=form, explode=false) to a WithBankResponseTimeout value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WithBankResponseTimeout {
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
                None => return std::result::Result::Err("Missing value while parsing WithBankResponseTimeout".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "millis" => intermediate_rep.millis.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing WithBankResponseTimeout".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WithBankResponseTimeout {
            millis: intermediate_rep.millis.into_iter().next().ok_or_else(|| "millis missing in WithBankResponseTimeout".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in WithBankResponseTimeout".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WithBankResponseTimeout> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<WithBankResponseTimeout>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WithBankResponseTimeout>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for WithBankResponseTimeout - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<WithBankResponseTimeout> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WithBankResponseTimeout as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into WithBankResponseTimeout - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Merchant payment process will not use a timeout when finalizing.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WithoutWaitingForBank {
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(default = "WithoutWaitingForBank::_name_for_r_type")]
    #[serde(serialize_with = "WithoutWaitingForBank::_serialize_r_type")]
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}

impl WithoutWaitingForBank {
    fn _name_for_r_type() -> String {
        String::from("WithoutWaitingForBank")
    }

    fn _serialize_r_type<S>(_: &String, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_str(&Self::_name_for_r_type())
    }
}


impl WithoutWaitingForBank {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> WithoutWaitingForBank {
        WithoutWaitingForBank {
 r_type: Self::_name_for_r_type(),
        }
    }
}

/// Converts the WithoutWaitingForBank value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for WithoutWaitingForBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WithoutWaitingForBank value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WithoutWaitingForBank {
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
                None => return std::result::Result::Err("Missing value while parsing WithoutWaitingForBank".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing WithoutWaitingForBank".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WithoutWaitingForBank {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in WithoutWaitingForBank".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WithoutWaitingForBank> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<WithoutWaitingForBank>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WithoutWaitingForBank>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for WithoutWaitingForBank - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<WithoutWaitingForBank> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WithoutWaitingForBank as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into WithoutWaitingForBank - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}


