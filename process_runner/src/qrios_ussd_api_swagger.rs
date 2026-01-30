#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///The event sent when the USSD session is aborted (internal error).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The event sent when the USSD session is aborted (internal error).",
    ///  "type": "object",
    ///  "required": [
    ///    "appId",
    ///    "clientId",
    ///    "reason",
    ///    "sessionId"
    ///  ],
    ///  "properties": {
    ///    "appId": {
    ///      "description": "Identifier of the USSD app created on Qrios platform.",
    ///      "examples": [
    ///        "b56r455"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "description": "Client identifier of the developer using Qrios platform.",
    ///      "type": "string"
    ///    },
    ///    "reason": {
    ///      "$ref": "#/components/schemas/AbortSession.AbortReason"
    ///    },
    ///    "sessionId": {
    ///      "description": "Unique identifier of USSD session.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AbortSession {
        ///Identifier of the USSD app created on Qrios platform.
        #[serde(rename = "appId")]
        pub app_id: ::std::string::String,
        ///Client identifier of the developer using Qrios platform.
        #[serde(rename = "clientId")]
        pub client_id: ::std::string::String,
        pub reason: AbortSessionAbortReason,
        ///Unique identifier of USSD session.
        #[serde(rename = "sessionId")]
        pub session_id: ::std::string::String,
    }
    impl ::std::convert::From<&AbortSession> for AbortSession {
        fn from(value: &AbortSession) -> Self {
            value.clone()
        }
    }
    ///`AbortSessionAbortReason`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AbortSession.AbortReason.DuplicatedOperation"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AbortSession.AbortReason.InsufficientBalanceInVirtualPurse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AbortSession.AbortReason.InternalError"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AbortSession.AbortReason.MissingPrivilege"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AbortSession.AbortReason.UnexpectedUssdAppResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AbortSessionAbortReason {
        DuplicatedOperation(AbortSessionAbortReasonDuplicatedOperation),
        InsufficientBalanceInVirtualPurse(
            AbortSessionAbortReasonInsufficientBalanceInVirtualPurse,
        ),
        InternalError(AbortSessionAbortReasonInternalError),
        MissingPrivilege(AbortSessionAbortReasonMissingPrivilege),
        UnexpectedUssdAppResponse(AbortSessionAbortReasonUnexpectedUssdAppResponse),
    }
    impl ::std::convert::From<&Self> for AbortSessionAbortReason {
        fn from(value: &AbortSessionAbortReason) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<AbortSessionAbortReasonDuplicatedOperation>
    for AbortSessionAbortReason {
        fn from(value: AbortSessionAbortReasonDuplicatedOperation) -> Self {
            Self::DuplicatedOperation(value)
        }
    }
    impl ::std::convert::From<AbortSessionAbortReasonInsufficientBalanceInVirtualPurse>
    for AbortSessionAbortReason {
        fn from(
            value: AbortSessionAbortReasonInsufficientBalanceInVirtualPurse,
        ) -> Self {
            Self::InsufficientBalanceInVirtualPurse(value)
        }
    }
    impl ::std::convert::From<AbortSessionAbortReasonInternalError>
    for AbortSessionAbortReason {
        fn from(value: AbortSessionAbortReasonInternalError) -> Self {
            Self::InternalError(value)
        }
    }
    impl ::std::convert::From<AbortSessionAbortReasonMissingPrivilege>
    for AbortSessionAbortReason {
        fn from(value: AbortSessionAbortReasonMissingPrivilege) -> Self {
            Self::MissingPrivilege(value)
        }
    }
    impl ::std::convert::From<AbortSessionAbortReasonUnexpectedUssdAppResponse>
    for AbortSessionAbortReason {
        fn from(value: AbortSessionAbortReasonUnexpectedUssdAppResponse) -> Self {
            Self::UnexpectedUssdAppResponse(value)
        }
    }
    ///USSD process abort reason - operation, eg. Merchant Payment, was previously invoked with given identifier.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "USSD process abort reason - operation, eg. Merchant Payment, was previously invoked with given identifier.",
    ///  "type": "object",
    ///  "required": [
    ///    "operationId",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "operationId": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AbortSessionAbortReasonDuplicatedOperation {
        #[serde(rename = "operationId")]
        pub operation_id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&AbortSessionAbortReasonDuplicatedOperation>
    for AbortSessionAbortReasonDuplicatedOperation {
        fn from(value: &AbortSessionAbortReasonDuplicatedOperation) -> Self {
            value.clone()
        }
    }
    ///USSD process abort reason - not enough funds to fulfil the request.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "USSD process abort reason - not enough funds to fulfil the request.",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&AbortSessionAbortReasonInsufficientBalanceInVirtualPurse>
    for AbortSessionAbortReasonInsufficientBalanceInVirtualPurse {
        fn from(
            value: &AbortSessionAbortReasonInsufficientBalanceInVirtualPurse,
        ) -> Self {
            value.clone()
        }
    }
    ///USSD process abort reason - there was some internal error in Qrios API.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "USSD process abort reason - there was some internal error in Qrios API.",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AbortSessionAbortReasonInternalError {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&AbortSessionAbortReasonInternalError>
    for AbortSessionAbortReasonInternalError {
        fn from(value: &AbortSessionAbortReasonInternalError) -> Self {
            value.clone()
        }
    }
    ///USSD process abort reason - there are no required privileges to run a USSD operation.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "USSD process abort reason - there are no required privileges to run a USSD operation.",
    ///  "type": "object",
    ///  "required": [
    ///    "privilege",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "privilege": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AbortSessionAbortReasonMissingPrivilege {
        pub privilege: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&AbortSessionAbortReasonMissingPrivilege>
    for AbortSessionAbortReasonMissingPrivilege {
        fn from(value: &AbortSessionAbortReasonMissingPrivilege) -> Self {
            value.clone()
        }
    }
    ///USSD process abort reason - previous USSD app response was malformed or not expected.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "USSD process abort reason - previous USSD app response was malformed or not expected.",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AbortSessionAbortReasonUnexpectedUssdAppResponse {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&AbortSessionAbortReasonUnexpectedUssdAppResponse>
    for AbortSessionAbortReasonUnexpectedUssdAppResponse {
        fn from(value: &AbortSessionAbortReasonUnexpectedUssdAppResponse) -> Self {
            value.clone()
        }
    }
    ///The event sent when the USSD session is closed (user can no longer input data, user leaves the session, session times out).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The event sent when the USSD session is closed (user can no longer input data, user leaves the session, session times out).",
    ///  "type": "object",
    ///  "required": [
    ///    "appId",
    ///    "clientId",
    ///    "contextData",
    ///    "reason",
    ///    "sessionId"
    ///  ],
    ///  "properties": {
    ///    "appId": {
    ///      "description": "Identifier of the USSD app created on Qrios platform.",
    ///      "examples": [
    ///        "b56r455"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "description": "Client identifier of the developer using Qrios platform.",
    ///      "type": "string"
    ///    },
    ///    "contextData": {
    ///      "description": "String set by the developer and carried over throughout the session.",
    ///      "type": "string"
    ///    },
    ///    "reason": {
    ///      "$ref": "#/components/schemas/CloseSession.CloseReason"
    ///    },
    ///    "sessionId": {
    ///      "description": "Unique identifier of USSD session.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CloseSession {
        ///Identifier of the USSD app created on Qrios platform.
        #[serde(rename = "appId")]
        pub app_id: ::std::string::String,
        ///Client identifier of the developer using Qrios platform.
        #[serde(rename = "clientId")]
        pub client_id: ::std::string::String,
        ///String set by the developer and carried over throughout the session.
        #[serde(rename = "contextData")]
        pub context_data: ::std::string::String,
        pub reason: CloseSessionCloseReason,
        ///Unique identifier of USSD session.
        #[serde(rename = "sessionId")]
        pub session_id: ::std::string::String,
    }
    impl ::std::convert::From<&CloseSession> for CloseSession {
        fn from(value: &CloseSession) -> Self {
            value.clone()
        }
    }
    ///`CloseSessionCloseReason`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/CloseSession.CloseReason.Abandon"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CloseSession.CloseReason.End"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CloseSession.CloseReason.Timeout"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum CloseSessionCloseReason {
        Abandon(CloseSessionCloseReasonAbandon),
        End(CloseSessionCloseReasonEnd),
        Timeout(CloseSessionCloseReasonTimeout),
    }
    impl ::std::convert::From<&Self> for CloseSessionCloseReason {
        fn from(value: &CloseSessionCloseReason) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<CloseSessionCloseReasonAbandon>
    for CloseSessionCloseReason {
        fn from(value: CloseSessionCloseReasonAbandon) -> Self {
            Self::Abandon(value)
        }
    }
    impl ::std::convert::From<CloseSessionCloseReasonEnd> for CloseSessionCloseReason {
        fn from(value: CloseSessionCloseReasonEnd) -> Self {
            Self::End(value)
        }
    }
    impl ::std::convert::From<CloseSessionCloseReasonTimeout>
    for CloseSessionCloseReason {
        fn from(value: CloseSessionCloseReasonTimeout) -> Self {
            Self::Timeout(value)
        }
    }
    ///Abandon - the user leaves the session (e.g. the user presses "Cancel" on the phone).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Abandon - the user leaves the session (e.g. the user presses \"Cancel\" on the phone).",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CloseSessionCloseReasonAbandon {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&CloseSessionCloseReasonAbandon>
    for CloseSessionCloseReasonAbandon {
        fn from(value: &CloseSessionCloseReasonAbandon) -> Self {
            value.clone()
        }
    }
    ///End - the session ends naturally (e.g. the user can no longer input data).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "End - the session ends naturally (e.g. the user can no longer input data).",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CloseSessionCloseReasonEnd {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&CloseSessionCloseReasonEnd>
    for CloseSessionCloseReasonEnd {
        fn from(value: &CloseSessionCloseReasonEnd) -> Self {
            value.clone()
        }
    }
    ///Timeout - the session is ended by the mobile operator (e.g. after two minutes).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Timeout - the session is ended by the mobile operator (e.g. after two minutes).",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct CloseSessionCloseReasonTimeout {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&CloseSessionCloseReasonTimeout>
    for CloseSessionCloseReasonTimeout {
        fn from(value: &CloseSessionCloseReasonTimeout) -> Self {
            value.clone()
        }
    }
    ///The event sent when the USSD session continues (input is received or process ends).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The event sent when the USSD session continues (input is received or process ends).",
    ///  "type": "object",
    ///  "required": [
    ///    "appId",
    ///    "clientId",
    ///    "contextData",
    ///    "result",
    ///    "sessionId"
    ///  ],
    ///  "properties": {
    ///    "appId": {
    ///      "description": "Identifier of the USSD app created on Qrios platform.",
    ///      "examples": [
    ///        "b56r455"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "description": "Client identifier of the developer using Qrios platform.",
    ///      "type": "string"
    ///    },
    ///    "contextData": {
    ///      "description": "String set by the developer and carried over throughout the session.",
    ///      "type": "string"
    ///    },
    ///    "result": {
    ///      "$ref": "#/components/schemas/UssdActionResult"
    ///    },
    ///    "sessionId": {
    ///      "description": "Unique identifier of USSD session.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ContinueSession {
        ///Identifier of the USSD app created on Qrios platform.
        #[serde(rename = "appId")]
        pub app_id: ::std::string::String,
        ///Client identifier of the developer using Qrios platform.
        #[serde(rename = "clientId")]
        pub client_id: ::std::string::String,
        ///String set by the developer and carried over throughout the session.
        #[serde(rename = "contextData")]
        pub context_data: ::std::string::String,
        pub result: UssdActionResult,
        ///Unique identifier of USSD session.
        #[serde(rename = "sessionId")]
        pub session_id: ::std::string::String,
    }
    impl ::std::convert::From<&ContinueSession> for ContinueSession {
        fn from(value: &ContinueSession) -> Self {
            value.clone()
        }
    }
    ///Developer's USSD App requests a redirect to some Legacy USSD App
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Developer's USSD App requests a redirect to some Legacy USSD App",
    ///  "type": "object",
    ///  "required": [
    ///    "type",
    ///    "uri"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LegacyAppRedirect {
        #[serde(rename = "type")]
        pub type_: ::serde_json::Value,
        pub uri: ::serde_json::Value,
    }
    impl ::std::convert::From<&LegacyAppRedirect> for LegacyAppRedirect {
        fn from(value: &LegacyAppRedirect) -> Self {
            value.clone()
        }
    }
    ///Url of the Legacy USSD App
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Url of the Legacy USSD App",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    #[serde(transparent)]
    pub struct LegacyAppRedirectUri(pub ::std::string::String);
    impl ::std::ops::Deref for LegacyAppRedirectUri {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LegacyAppRedirectUri> for ::std::string::String {
        fn from(value: LegacyAppRedirectUri) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&LegacyAppRedirectUri> for LegacyAppRedirectUri {
        fn from(value: &LegacyAppRedirectUri) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::std::string::String> for LegacyAppRedirectUri {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for LegacyAppRedirectUri {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for LegacyAppRedirectUri {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    ///Url of the USSD API
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Url of the USSD API",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    #[serde(transparent)]
    pub struct LegacyAppRedirectUssdApiUrl(pub ::std::string::String);
    impl ::std::ops::Deref for LegacyAppRedirectUssdApiUrl {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<LegacyAppRedirectUssdApiUrl> for ::std::string::String {
        fn from(value: LegacyAppRedirectUssdApiUrl) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&LegacyAppRedirectUssdApiUrl>
    for LegacyAppRedirectUssdApiUrl {
        fn from(value: &LegacyAppRedirectUssdApiUrl) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::std::string::String> for LegacyAppRedirectUssdApiUrl {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }
    impl ::std::str::FromStr for LegacyAppRedirectUssdApiUrl {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::fmt::Display for LegacyAppRedirectUssdApiUrl {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }
    ///`Map`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "string"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Map(
        pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    );
    impl ::std::ops::Deref for Map {
        type Target = ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
            &self.0
        }
    }
    impl ::std::convert::From<Map>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        fn from(value: Map) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&Map> for Map {
        fn from(value: &Map) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > for Map {
        fn from(
            value: ::std::collections::HashMap<
                ::std::string::String,
                ::std::string::String,
            >,
        ) -> Self {
            Self(value)
        }
    }
    ///`MerchantPaymentProcessExecutionMode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess.ExecutionMode.WithBankResponseTimeout"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess.ExecutionMode.WithoutWaitingForBank"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MerchantPaymentProcessExecutionMode {
        WithBankResponseTimeout(
            MerchantPaymentProcessExecutionModeWithBankResponseTimeout,
        ),
        WithoutWaitingForBank(MerchantPaymentProcessExecutionModeWithoutWaitingForBank),
    }
    impl ::std::convert::From<&Self> for MerchantPaymentProcessExecutionMode {
        fn from(value: &MerchantPaymentProcessExecutionMode) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<MerchantPaymentProcessExecutionModeWithBankResponseTimeout>
    for MerchantPaymentProcessExecutionMode {
        fn from(
            value: MerchantPaymentProcessExecutionModeWithBankResponseTimeout,
        ) -> Self {
            Self::WithBankResponseTimeout(value)
        }
    }
    impl ::std::convert::From<MerchantPaymentProcessExecutionModeWithoutWaitingForBank>
    for MerchantPaymentProcessExecutionMode {
        fn from(
            value: MerchantPaymentProcessExecutionModeWithoutWaitingForBank,
        ) -> Self {
            Self::WithoutWaitingForBank(value)
        }
    }
    ///Merchant payment process will use a timeout when finalizing.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Merchant payment process will use a timeout when finalizing.",
    ///  "type": "object",
    ///  "required": [
    ///    "millis",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "millis": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
        pub millis: i64,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<
        &MerchantPaymentProcessExecutionModeWithBankResponseTimeout,
    > for MerchantPaymentProcessExecutionModeWithBankResponseTimeout {
        fn from(
            value: &MerchantPaymentProcessExecutionModeWithBankResponseTimeout,
        ) -> Self {
            value.clone()
        }
    }
    ///Merchant payment process will not use a timeout when finalizing.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Merchant payment process will not use a timeout when finalizing.",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&MerchantPaymentProcessExecutionModeWithoutWaitingForBank>
    for MerchantPaymentProcessExecutionModeWithoutWaitingForBank {
        fn from(
            value: &MerchantPaymentProcessExecutionModeWithoutWaitingForBank,
        ) -> Self {
            value.clone()
        }
    }
    ///`MerchantPaymentProcessPaymentMode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess.PaymentMode.FixedAccount"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess.PaymentMode.FixedBank"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess.PaymentMode.Flexible"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MerchantPaymentProcessPaymentMode {
        FixedAccount(MerchantPaymentProcessPaymentModeFixedAccount),
        FixedBank(MerchantPaymentProcessPaymentModeFixedBank),
        Flexible(MerchantPaymentProcessPaymentModeFlexible),
    }
    impl ::std::convert::From<&Self> for MerchantPaymentProcessPaymentMode {
        fn from(value: &MerchantPaymentProcessPaymentMode) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<MerchantPaymentProcessPaymentModeFixedAccount>
    for MerchantPaymentProcessPaymentMode {
        fn from(value: MerchantPaymentProcessPaymentModeFixedAccount) -> Self {
            Self::FixedAccount(value)
        }
    }
    impl ::std::convert::From<MerchantPaymentProcessPaymentModeFixedBank>
    for MerchantPaymentProcessPaymentMode {
        fn from(value: MerchantPaymentProcessPaymentModeFixedBank) -> Self {
            Self::FixedBank(value)
        }
    }
    impl ::std::convert::From<MerchantPaymentProcessPaymentModeFlexible>
    for MerchantPaymentProcessPaymentMode {
        fn from(value: MerchantPaymentProcessPaymentModeFlexible) -> Self {
            Self::Flexible(value)
        }
    }
    ///Fixed account mode in merchant payment process, with account ID specified.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Fixed account mode in merchant payment process, with account ID specified.",
    ///  "type": "object",
    ///  "required": [
    ///    "accountNumber",
    ///    "bank",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "accountNumber": {
    ///      "description": "Number of the account which will be used in transaction. Account numbers can be obtained with Qrios API using `/merchants/accounts` endpoint.",
    ///      "type": "string"
    ///    },
    ///    "bank": {
    ///      "description": "Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentProcessPaymentModeFixedAccount {
        ///Number of the account which will be used in transaction. Account numbers can be obtained with Qrios API using `/merchants/accounts` endpoint.
        #[serde(rename = "accountNumber")]
        pub account_number: ::std::string::String,
        ///Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
        pub bank: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&MerchantPaymentProcessPaymentModeFixedAccount>
    for MerchantPaymentProcessPaymentModeFixedAccount {
        fn from(value: &MerchantPaymentProcessPaymentModeFixedAccount) -> Self {
            value.clone()
        }
    }
    ///Fixed bank mode in merchant payment process, with bank ID specified. The account from provided bank will then be selected within the process.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Fixed bank mode in merchant payment process, with bank ID specified. The account from provided bank will then be selected within the process.",
    ///  "type": "object",
    ///  "required": [
    ///    "bank",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "bank": {
    ///      "description": "Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentProcessPaymentModeFixedBank {
        ///Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
        pub bank: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&MerchantPaymentProcessPaymentModeFixedBank>
    for MerchantPaymentProcessPaymentModeFixedBank {
        fn from(value: &MerchantPaymentProcessPaymentModeFixedBank) -> Self {
            value.clone()
        }
    }
    ///Flexible payment mode in merchant payment process. The account will then be selected within the process.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Flexible payment mode in merchant payment process. The account will then be selected within the process.",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentProcessPaymentModeFlexible {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&MerchantPaymentProcessPaymentModeFlexible>
    for MerchantPaymentProcessPaymentModeFlexible {
        fn from(value: &MerchantPaymentProcessPaymentModeFlexible) -> Self {
            value.clone()
        }
    }
    ///`MerchantPaymentResultOperationStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentResult.OperationStatus.Failure"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentResult.OperationStatus.Success"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentResult.OperationStatus.Unknown"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MerchantPaymentResultOperationStatus {
        Failure(MerchantPaymentResultOperationStatusFailure),
        Success(MerchantPaymentResultOperationStatusSuccess),
        Unknown(MerchantPaymentResultOperationStatusUnknown),
    }
    impl ::std::convert::From<&Self> for MerchantPaymentResultOperationStatus {
        fn from(value: &MerchantPaymentResultOperationStatus) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<MerchantPaymentResultOperationStatusFailure>
    for MerchantPaymentResultOperationStatus {
        fn from(value: MerchantPaymentResultOperationStatusFailure) -> Self {
            Self::Failure(value)
        }
    }
    impl ::std::convert::From<MerchantPaymentResultOperationStatusSuccess>
    for MerchantPaymentResultOperationStatus {
        fn from(value: MerchantPaymentResultOperationStatusSuccess) -> Self {
            Self::Success(value)
        }
    }
    impl ::std::convert::From<MerchantPaymentResultOperationStatusUnknown>
    for MerchantPaymentResultOperationStatus {
        fn from(value: MerchantPaymentResultOperationStatusUnknown) -> Self {
            Self::Unknown(value)
        }
    }
    ///Merchant payment operation failed (there was no charge for sure)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Merchant payment operation failed (there was no charge for sure)",
    ///  "type": "object",
    ///  "required": [
    ///    "cause",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "cause": {
    ///      "$ref": "#/components/schemas/MerchantPaymentResult.OperationStatus.Failure.Cause"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentResultOperationStatusFailure {
        pub cause: MerchantPaymentResultOperationStatusFailureCause,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&MerchantPaymentResultOperationStatusFailure>
    for MerchantPaymentResultOperationStatusFailure {
        fn from(value: &MerchantPaymentResultOperationStatusFailure) -> Self {
            value.clone()
        }
    }
    ///The reason why the operation failed.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The reason why the operation failed.",
    ///  "type": "string",
    ///  "enum": [
    ///    "AuthenticationFailed",
    ///    "InsufficientBalance",
    ///    "InvalidMerchant",
    ///    "NoAccounts",
    ///    "SwitchUnavailable",
    ///    "Other"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum MerchantPaymentResultOperationStatusFailureCause {
        AuthenticationFailed,
        InsufficientBalance,
        InvalidMerchant,
        NoAccounts,
        SwitchUnavailable,
        Other,
    }
    impl ::std::convert::From<&Self>
    for MerchantPaymentResultOperationStatusFailureCause {
        fn from(value: &MerchantPaymentResultOperationStatusFailureCause) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for MerchantPaymentResultOperationStatusFailureCause {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AuthenticationFailed => f.write_str("AuthenticationFailed"),
                Self::InsufficientBalance => f.write_str("InsufficientBalance"),
                Self::InvalidMerchant => f.write_str("InvalidMerchant"),
                Self::NoAccounts => f.write_str("NoAccounts"),
                Self::SwitchUnavailable => f.write_str("SwitchUnavailable"),
                Self::Other => f.write_str("Other"),
            }
        }
    }
    impl ::std::str::FromStr for MerchantPaymentResultOperationStatusFailureCause {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "AuthenticationFailed" => Ok(Self::AuthenticationFailed),
                "InsufficientBalance" => Ok(Self::InsufficientBalance),
                "InvalidMerchant" => Ok(Self::InvalidMerchant),
                "NoAccounts" => Ok(Self::NoAccounts),
                "SwitchUnavailable" => Ok(Self::SwitchUnavailable),
                "Other" => Ok(Self::Other),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str>
    for MerchantPaymentResultOperationStatusFailureCause {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MerchantPaymentResultOperationStatusFailureCause {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MerchantPaymentResultOperationStatusFailureCause {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Merchant payment operation finished with success
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Merchant payment operation finished with success",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentResultOperationStatusSuccess {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&MerchantPaymentResultOperationStatusSuccess>
    for MerchantPaymentResultOperationStatusSuccess {
        fn from(value: &MerchantPaymentResultOperationStatusSuccess) -> Self {
            value.clone()
        }
    }
    ///Cannot determine if merchant payment operation succeed or not
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Cannot determine if merchant payment operation succeed or not",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MerchantPaymentResultOperationStatusUnknown {
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }
    impl ::std::convert::From<&MerchantPaymentResultOperationStatusUnknown>
    for MerchantPaymentResultOperationStatusUnknown {
        fn from(value: &MerchantPaymentResultOperationStatusUnknown) -> Self {
            value.clone()
        }
    }
    ///Input provided when the user dials into the session (dials the shortcode string)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Input provided when the user dials into the session (dials the shortcode string)",
    ///  "type": "object",
    ///  "required": [
    ///    "shortcodeString",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "shortcodeString": {
    ///      "description": "Shortcode string dialed by the user",
    ///      "examples": [
    ///        "*425*001*123#"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "Dial"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Dial"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NewSessionSessionInputDial {
        ///Shortcode string dialed by the user
        #[serde(rename = "shortcodeString")]
        pub shortcode_string: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: NewSessionSessionInputDialType,
    }
    impl ::std::convert::From<&NewSessionSessionInputDial>
    for NewSessionSessionInputDial {
        fn from(value: &NewSessionSessionInputDial) -> Self {
            value.clone()
        }
    }
    ///`NewSessionSessionInputDialType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Dial"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Dial"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum NewSessionSessionInputDialType {
        Dial,
    }
    impl ::std::convert::From<&Self> for NewSessionSessionInputDialType {
        fn from(value: &NewSessionSessionInputDialType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for NewSessionSessionInputDialType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Dial => f.write_str("Dial"),
            }
        }
    }
    impl ::std::str::FromStr for NewSessionSessionInputDialType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Dial" => Ok(Self::Dial),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for NewSessionSessionInputDialType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for NewSessionSessionInputDialType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for NewSessionSessionInputDialType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Input provided when session is begun by a push message sent to a user.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Input provided when session is begun by a push message sent to a user.",
    ///  "type": "object",
    ///  "required": [
    ///    "contextData",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "contextData": {
    ///      "description": "Context regarding the push message.",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "Push"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Push"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NewSessionSessionInputPush {
        ///Context regarding the push message.
        #[serde(rename = "contextData")]
        pub context_data: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: NewSessionSessionInputPushType,
    }
    impl ::std::convert::From<&NewSessionSessionInputPush>
    for NewSessionSessionInputPush {
        fn from(value: &NewSessionSessionInputPush) -> Self {
            value.clone()
        }
    }
    ///`NewSessionSessionInputPushType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Push"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Push"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum NewSessionSessionInputPushType {
        Push,
    }
    impl ::std::convert::From<&Self> for NewSessionSessionInputPushType {
        fn from(value: &NewSessionSessionInputPushType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for NewSessionSessionInputPushType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("Push"),
            }
        }
    }
    impl ::std::str::FromStr for NewSessionSessionInputPushType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Push" => Ok(Self::Push),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for NewSessionSessionInputPushType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for NewSessionSessionInputPushType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for NewSessionSessionInputPushType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Input provided when session is redirected from different USSD application.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Input provided when session is redirected from different USSD application.",
    ///  "type": "object",
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "process": {
    ///      "$ref": "#/components/schemas/UssdApp.Process"
    ///    },
    ///    "processId": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "Redirect"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Redirect"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct NewSessionSessionInputRedirect {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub process: ::std::option::Option<UssdAppProcess>,
        #[serde(
            rename = "processId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub process_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "type")]
        pub type_: NewSessionSessionInputRedirectType,
    }
    impl ::std::convert::From<&NewSessionSessionInputRedirect>
    for NewSessionSessionInputRedirect {
        fn from(value: &NewSessionSessionInputRedirect) -> Self {
            value.clone()
        }
    }
    ///Parameters of the process. Keys should be considered as process param names and their values as process param values.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Parameters of the process. Keys should be considered as process param names and their values as process param values.",
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "string"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct NewSessionSessionInputRedirectParams(
        pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    );
    impl ::std::ops::Deref for NewSessionSessionInputRedirectParams {
        type Target = ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
            &self.0
        }
    }
    impl ::std::convert::From<NewSessionSessionInputRedirectParams>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        fn from(value: NewSessionSessionInputRedirectParams) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&NewSessionSessionInputRedirectParams>
    for NewSessionSessionInputRedirectParams {
        fn from(value: &NewSessionSessionInputRedirectParams) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > for NewSessionSessionInputRedirectParams {
        fn from(
            value: ::std::collections::HashMap<
                ::std::string::String,
                ::std::string::String,
            >,
        ) -> Self {
            Self(value)
        }
    }
    ///`NewSessionSessionInputRedirectType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Redirect"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Redirect"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum NewSessionSessionInputRedirectType {
        Redirect,
    }
    impl ::std::convert::From<&Self> for NewSessionSessionInputRedirectType {
        fn from(value: &NewSessionSessionInputRedirectType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for NewSessionSessionInputRedirectType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Redirect => f.write_str("Redirect"),
            }
        }
    }
    impl ::std::str::FromStr for NewSessionSessionInputRedirectType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Redirect" => Ok(Self::Redirect),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for NewSessionSessionInputRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for NewSessionSessionInputRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for NewSessionSessionInputRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Developer's USSD App returns USSD process result as a map of params
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Developer's USSD App returns USSD process result as a map of params",
    ///  "type": "object",
    ///  "required": [
    ///    "destinationAppId",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "destinationAppId": {
    ///      "description": "Identifier of the USSD app created on Qrios platform.",
    ///      "examples": [
    ///        "b56r455"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "process": {
    ///      "$ref": "#/components/schemas/UssdApp.Process"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "Redirect"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Redirect"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Redirect {
        ///Identifier of the USSD app created on Qrios platform.
        #[serde(rename = "destinationAppId")]
        pub destination_app_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub process: ::std::option::Option<UssdAppProcess>,
        #[serde(rename = "type")]
        pub type_: RedirectType,
    }
    impl ::std::convert::From<&Redirect> for Redirect {
        fn from(value: &Redirect) -> Self {
            value.clone()
        }
    }
    ///`RedirectType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Redirect"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Redirect"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum RedirectType {
        Redirect,
    }
    impl ::std::convert::From<&Self> for RedirectType {
        fn from(value: &RedirectType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for RedirectType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Redirect => f.write_str("Redirect"),
            }
        }
    }
    impl ::std::str::FromStr for RedirectType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Redirect" => Ok(Self::Redirect),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for RedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///If the current session was redirected from some other app, then this UssdAction redirects back to the calling app within the same USSD session
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "If the current session was redirected from some other app, then this UssdAction redirects back to the calling app within the same USSD session",
    ///  "type": "object",
    ///  "required": [
    ///    "resultParams",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "resultParams": {
    ///      "$ref": "#/components/schemas/ReturnFromRedirect.ResultParams"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ReturnFromRedirect {
        #[serde(rename = "resultParams")]
        pub result_params: ReturnFromRedirectResultParams,
        #[serde(rename = "type")]
        pub type_: ::serde_json::Value,
    }
    impl ::std::convert::From<&ReturnFromRedirect> for ReturnFromRedirect {
        fn from(value: &ReturnFromRedirect) -> Self {
            value.clone()
        }
    }
    ///Result parameters returned in the "return from redirect". Keys should be considered as result parameter names and their values as result parameter values.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result parameters returned in the \"return from redirect\". Keys should be considered as result parameter names and their values as result parameter values.",
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "string"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ReturnFromRedirectResultParams(
        pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    );
    impl ::std::ops::Deref for ReturnFromRedirectResultParams {
        type Target = ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
            &self.0
        }
    }
    impl ::std::convert::From<ReturnFromRedirectResultParams>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        fn from(value: ReturnFromRedirectResultParams) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReturnFromRedirectResultParams>
    for ReturnFromRedirectResultParams {
        fn from(value: &ReturnFromRedirectResultParams) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > for ReturnFromRedirectResultParams {
        fn from(
            value: ::std::collections::HashMap<
                ::std::string::String,
                ::std::string::String,
            >,
        ) -> Self {
            Self(value)
        }
    }
    ///Run a predefined process. The process will then take control of the session (will send prompts to the user and handle responses from the user) until it returns control along with a response. From the view of a user, they will interact with a single app.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Run a predefined process. The process will then take control of the session (will send prompts to the user and handle responses from the user) until it returns control along with a response. From the view of a user, they will interact with a single app.",
    ///  "type": "object",
    ///  "required": [
    ///    "process",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "process": {
    ///      "$ref": "#/components/schemas/UssdProcess"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "RunProcess"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "RunProcess"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct RunProcess {
        pub process: UssdProcess,
        #[serde(rename = "type")]
        pub type_: RunProcessType,
    }
    impl ::std::convert::From<&RunProcess> for RunProcess {
        fn from(value: &RunProcess) -> Self {
            value.clone()
        }
    }
    ///`RunProcessType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "RunProcess"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "RunProcess"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum RunProcessType {
        RunProcess,
    }
    impl ::std::convert::From<&Self> for RunProcessType {
        fn from(value: &RunProcessType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for RunProcessType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::RunProcess => f.write_str("RunProcess"),
            }
        }
    }
    impl ::std::str::FromStr for RunProcessType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "RunProcess" => Ok(Self::RunProcess),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for RunProcessType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for RunProcessType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for RunProcessType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Display a given view to the user. A view is a text message that will appear on the screen with optional ability to input data.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Display a given view to the user. A view is a text message that will appear on the screen with optional ability to input data.",
    ///  "type": "object",
    ///  "required": [
    ///    "type",
    ///    "view"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "examples": [
    ///        "ShowView"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "ShowView"
    ///      ]
    ///    },
    ///    "view": {
    ///      "$ref": "#/components/schemas/UssdView"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ShowView {
        #[serde(rename = "type")]
        pub type_: ShowViewType,
        pub view: UssdView,
    }
    impl ::std::convert::From<&ShowView> for ShowView {
        fn from(value: &ShowView) -> Self {
            value.clone()
        }
    }
    ///`ShowViewType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "ShowView"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "ShowView"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum ShowViewType {
        ShowView,
    }
    impl ::std::convert::From<&Self> for ShowViewType {
        fn from(value: &ShowViewType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for ShowViewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ShowView => f.write_str("ShowView"),
            }
        }
    }
    impl ::std::str::FromStr for ShowViewType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ShowView" => Ok(Self::ShowView),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for ShowViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ShowViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ShowViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Action to be sent back to the user. It can either start a predefined process or show a view.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Action to be sent back to the user. It can either start a predefined process or show a view.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "LegacyAppRedirect"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Redirect"
    ///      ],
    ///      "properties": {
    ///        "Redirect": {
    ///          "$ref": "#/components/schemas/Redirect"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ReturnFromRedirect"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "RunProcess"
    ///      ],
    ///      "properties": {
    ///        "RunProcess": {
    ///          "$ref": "#/components/schemas/RunProcess"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ShowView"
    ///      ],
    ///      "properties": {
    ///        "ShowView": {
    ///          "$ref": "#/components/schemas/ShowView"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdAction {
        Variant0 {
            #[serde(rename = "LegacyAppRedirect")]
            legacy_app_redirect: ::serde_json::Value,
        },
        Variant1 { #[serde(rename = "Redirect")] redirect: Redirect },
        Variant2 {
            #[serde(rename = "ReturnFromRedirect")]
            return_from_redirect: ::serde_json::Value,
        },
        Variant3 { #[serde(rename = "RunProcess")] run_process: RunProcess },
        Variant4 { #[serde(rename = "ShowView")] show_view: ShowView },
    }
    impl ::std::convert::From<&Self> for UssdAction {
        fn from(value: &UssdAction) -> Self {
            value.clone()
        }
    }
    ///Result of USSD action. Either input provided by the user or the result of a previously-initiated process.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result of USSD action. Either input provided by the user or the result of a previously-initiated process.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "EmbeddedProcessResult"
    ///      ],
    ///      "properties": {
    ///        "EmbeddedProcessResult": {
    ///          "$ref": "#/components/schemas/UssdActionResult.EmbeddedProcessResult"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "InputResult"
    ///      ],
    ///      "properties": {
    ///        "InputResult": {
    ///          "$ref": "#/components/schemas/UssdActionResult.InputResult"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "MerchantPaymentResult"
    ///      ],
    ///      "properties": {
    ///        "MerchantPaymentResult": {
    ///          "$ref": "#/components/schemas/UssdActionResult.MerchantPaymentResult"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ReturnFromRedirectResult"
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdActionResult {
        Variant0 {
            #[serde(rename = "EmbeddedProcessResult")]
            embedded_process_result: UssdActionResultEmbeddedProcessResult,
        },
        Variant1 {
            #[serde(rename = "InputResult")]
            input_result: UssdActionResultInputResult,
        },
        Variant2 {
            #[serde(rename = "MerchantPaymentResult")]
            merchant_payment_result: UssdActionResultMerchantPaymentResult,
        },
        Variant3 {
            #[serde(rename = "ReturnFromRedirectResult")]
            return_from_redirect_result: ::serde_json::Value,
        },
    }
    impl ::std::convert::From<&Self> for UssdActionResult {
        fn from(value: &UssdActionResult) -> Self {
            value.clone()
        }
    }
    ///Returns control of the session back to the developer and passes the result of embedded process execution.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Returns control of the session back to the developer and passes the result of embedded process execution.",
    ///  "type": "object",
    ///  "required": [
    ///    "resultParams",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "resultParams": {
    ///      "$ref": "#/components/schemas/UssdActionResult.EmbeddedProcessResult.ResultParams"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "EmbeddedProcessResult"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "EmbeddedProcessResult"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdActionResultEmbeddedProcessResult {
        #[serde(rename = "resultParams")]
        pub result_params: UssdActionResultEmbeddedProcessResultResultParams,
        #[serde(rename = "type")]
        pub type_: UssdActionResultEmbeddedProcessResultType,
    }
    impl ::std::convert::From<&UssdActionResultEmbeddedProcessResult>
    for UssdActionResultEmbeddedProcessResult {
        fn from(value: &UssdActionResultEmbeddedProcessResult) -> Self {
            value.clone()
        }
    }
    ///Result of the `embedded process`. Keys should be considered as result parameter names and their values as result parameter values.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result of the `embedded process`. Keys should be considered as result parameter names and their values as result parameter values.",
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "string"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct UssdActionResultEmbeddedProcessResultResultParams(
        pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    );
    impl ::std::ops::Deref for UssdActionResultEmbeddedProcessResultResultParams {
        type Target = ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
            &self.0
        }
    }
    impl ::std::convert::From<UssdActionResultEmbeddedProcessResultResultParams>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        fn from(value: UssdActionResultEmbeddedProcessResultResultParams) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UssdActionResultEmbeddedProcessResultResultParams>
    for UssdActionResultEmbeddedProcessResultResultParams {
        fn from(value: &UssdActionResultEmbeddedProcessResultResultParams) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > for UssdActionResultEmbeddedProcessResultResultParams {
        fn from(
            value: ::std::collections::HashMap<
                ::std::string::String,
                ::std::string::String,
            >,
        ) -> Self {
            Self(value)
        }
    }
    ///`UssdActionResultEmbeddedProcessResultType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "EmbeddedProcessResult"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "EmbeddedProcessResult"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdActionResultEmbeddedProcessResultType {
        EmbeddedProcessResult,
    }
    impl ::std::convert::From<&Self> for UssdActionResultEmbeddedProcessResultType {
        fn from(value: &UssdActionResultEmbeddedProcessResultType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdActionResultEmbeddedProcessResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::EmbeddedProcessResult => f.write_str("EmbeddedProcessResult"),
            }
        }
    }
    impl ::std::str::FromStr for UssdActionResultEmbeddedProcessResultType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EmbeddedProcessResult" => Ok(Self::EmbeddedProcessResult),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdActionResultEmbeddedProcessResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdActionResultEmbeddedProcessResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdActionResultEmbeddedProcessResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Input from the user; provided by UssdAction.UssdView.InputView.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Input from the user; provided by UssdAction.UssdView.InputView.",
    ///  "type": "object",
    ///  "required": [
    ///    "type",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "examples": [
    ///        "InputResult"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "InputResult"
    ///      ]
    ///    },
    ///    "value": {
    ///      "description": "Raw value typed in by the user.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdActionResultInputResult {
        #[serde(rename = "type")]
        pub type_: UssdActionResultInputResultType,
        ///Raw value typed in by the user.
        pub value: ::std::string::String,
    }
    impl ::std::convert::From<&UssdActionResultInputResult>
    for UssdActionResultInputResult {
        fn from(value: &UssdActionResultInputResult) -> Self {
            value.clone()
        }
    }
    ///`UssdActionResultInputResultType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "InputResult"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "InputResult"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdActionResultInputResultType {
        InputResult,
    }
    impl ::std::convert::From<&Self> for UssdActionResultInputResultType {
        fn from(value: &UssdActionResultInputResultType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdActionResultInputResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InputResult => f.write_str("InputResult"),
            }
        }
    }
    impl ::std::str::FromStr for UssdActionResultInputResultType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "InputResult" => Ok(Self::InputResult),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdActionResultInputResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdActionResultInputResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdActionResultInputResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Result from merchant payment process; provided by UssdProcess.MerchantPaymentProcess.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result from merchant payment process; provided by UssdProcess.MerchantPaymentProcess.",
    ///  "type": "object",
    ///  "required": [
    ///    "operationId",
    ///    "status",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "operationId": {
    ///      "description": "Merchant payment operation id",
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/MerchantPaymentResult.OperationStatus"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "MerchantPaymentResult"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "MerchantPaymentResult"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdActionResultMerchantPaymentResult {
        ///Merchant payment operation id
        #[serde(rename = "operationId")]
        pub operation_id: ::std::string::String,
        pub status: MerchantPaymentResultOperationStatus,
        #[serde(rename = "type")]
        pub type_: UssdActionResultMerchantPaymentResultType,
    }
    impl ::std::convert::From<&UssdActionResultMerchantPaymentResult>
    for UssdActionResultMerchantPaymentResult {
        fn from(value: &UssdActionResultMerchantPaymentResult) -> Self {
            value.clone()
        }
    }
    ///`UssdActionResultMerchantPaymentResultType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "MerchantPaymentResult"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "MerchantPaymentResult"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdActionResultMerchantPaymentResultType {
        MerchantPaymentResult,
    }
    impl ::std::convert::From<&Self> for UssdActionResultMerchantPaymentResultType {
        fn from(value: &UssdActionResultMerchantPaymentResultType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdActionResultMerchantPaymentResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::MerchantPaymentResult => f.write_str("MerchantPaymentResult"),
            }
        }
    }
    impl ::std::str::FromStr for UssdActionResultMerchantPaymentResultType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MerchantPaymentResult" => Ok(Self::MerchantPaymentResult),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdActionResultMerchantPaymentResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdActionResultMerchantPaymentResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdActionResultMerchantPaymentResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Result returned from the app, to which the session was previously redirected.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result returned from the app, to which the session was previously redirected.",
    ///  "type": "object",
    ///  "required": [
    ///    "resultParams",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "resultParams": {
    ///      "$ref": "#/components/schemas/UssdActionResult.ReturnFromRedirectResult.ResultParams"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "ReturnFromRedirectResult"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "ReturnFromRedirectResult"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdActionResultReturnFromRedirectResult {
        #[serde(rename = "resultParams")]
        pub result_params: UssdActionResultReturnFromRedirectResultResultParams,
        #[serde(rename = "type")]
        pub type_: UssdActionResultReturnFromRedirectResultType,
    }
    impl ::std::convert::From<&UssdActionResultReturnFromRedirectResult>
    for UssdActionResultReturnFromRedirectResult {
        fn from(value: &UssdActionResultReturnFromRedirectResult) -> Self {
            value.clone()
        }
    }
    ///Result of the `return from redirection`. Keys should be considered as result parameter names and their values as result parameter values.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Result of the `return from redirection`. Keys should be considered as result parameter names and their values as result parameter values.",
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "string"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct UssdActionResultReturnFromRedirectResultResultParams(
        pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    );
    impl ::std::ops::Deref for UssdActionResultReturnFromRedirectResultResultParams {
        type Target = ::std::collections::HashMap<
            ::std::string::String,
            ::std::string::String,
        >;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
            &self.0
        }
    }
    impl ::std::convert::From<UssdActionResultReturnFromRedirectResultResultParams>
    for ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        fn from(value: UssdActionResultReturnFromRedirectResultResultParams) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UssdActionResultReturnFromRedirectResultResultParams>
    for UssdActionResultReturnFromRedirectResultResultParams {
        fn from(value: &UssdActionResultReturnFromRedirectResultResultParams) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > for UssdActionResultReturnFromRedirectResultResultParams {
        fn from(
            value: ::std::collections::HashMap<
                ::std::string::String,
                ::std::string::String,
            >,
        ) -> Self {
            Self(value)
        }
    }
    ///`UssdActionResultReturnFromRedirectResultType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "ReturnFromRedirectResult"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "ReturnFromRedirectResult"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdActionResultReturnFromRedirectResultType {
        ReturnFromRedirectResult,
    }
    impl ::std::convert::From<&Self> for UssdActionResultReturnFromRedirectResultType {
        fn from(value: &UssdActionResultReturnFromRedirectResultType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdActionResultReturnFromRedirectResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReturnFromRedirectResult => f.write_str("ReturnFromRedirectResult"),
            }
        }
    }
    impl ::std::str::FromStr for UssdActionResultReturnFromRedirectResultType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ReturnFromRedirectResult" => Ok(Self::ReturnFromRedirectResult),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdActionResultReturnFromRedirectResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdActionResultReturnFromRedirectResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdActionResultReturnFromRedirectResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///It indicates which process inside USSD App should be invoked
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "It indicates which process inside USSD App should be invoked",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "params"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "description": "Identifier of the process. Based on this parameter, developer can decide eg. what USSD screen will be shown next.",
    ///      "type": "string"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/NewSession.SessionInput.Redirect.Params"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdAppProcess {
        ///Identifier of the process. Based on this parameter, developer can decide eg. what USSD screen will be shown next.
        pub id: ::std::string::String,
        pub params: NewSessionSessionInputRedirectParams,
    }
    impl ::std::convert::From<&UssdAppProcess> for UssdAppProcess {
        fn from(value: &UssdAppProcess) -> Self {
            value.clone()
        }
    }
    ///The process control is handed over to (flow continues within the process until the process continues or aborts).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The process control is handed over to (flow continues within the process until the process continues or aborts).",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "EmbeddedProcess"
    ///      ],
    ///      "properties": {
    ///        "EmbeddedProcess": {
    ///          "$ref": "#/components/schemas/UssdProcess.EmbeddedProcess"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "MerchantPaymentProcess"
    ///      ],
    ///      "properties": {
    ///        "MerchantPaymentProcess": {
    ///          "$ref": "#/components/schemas/UssdProcess.MerchantPaymentProcess"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub enum UssdProcess {
        EmbeddedProcess(UssdProcessEmbeddedProcess),
        MerchantPaymentProcess(UssdProcessMerchantPaymentProcess),
    }
    impl ::std::convert::From<&Self> for UssdProcess {
        fn from(value: &UssdProcess) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<UssdProcessEmbeddedProcess> for UssdProcess {
        fn from(value: UssdProcessEmbeddedProcess) -> Self {
            Self::EmbeddedProcess(value)
        }
    }
    impl ::std::convert::From<UssdProcessMerchantPaymentProcess> for UssdProcess {
        fn from(value: UssdProcessMerchantPaymentProcess) -> Self {
            Self::MerchantPaymentProcess(value)
        }
    }
    ///Initiate an embedded process. This action causes the Qrios system to hand over control to an external application. The user may be prompted to provide credentials during the embedded process. After the embedded process is complete, the ussdSessionEvent/continue API endpoint is called to deliver the result and return control of the session to the developer application.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Initiate an embedded process. This action causes the Qrios system to hand over control to an external application. The user may be prompted to provide credentials during the embedded process. After the embedded process is complete, the ussdSessionEvent/continue API endpoint is called to deliver the result and return control of the session to the developer application.",
    ///  "type": "object",
    ///  "required": [
    ///    "params",
    ///    "processId"
    ///  ],
    ///  "properties": {
    ///    "params": {
    ///      "$ref": "#/components/schemas/Map"
    ///    },
    ///    "processId": {
    ///      "description": "embedded process id",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdProcessEmbeddedProcess {
        pub params: Map,
        ///embedded process id
        #[serde(rename = "processId")]
        pub process_id: ::std::string::String,
    }
    impl ::std::convert::From<&UssdProcessEmbeddedProcess>
    for UssdProcessEmbeddedProcess {
        fn from(value: &UssdProcessEmbeddedProcess) -> Self {
            value.clone()
        }
    }
    ///Initiation of merchant payment process. In this process, the control is handed over to an external USSD merchant payment process. The user will be asked for credentials and the payment will commence. As the process ends, it will call this API to deliver the result and hand control over the session back to the server.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Initiation of merchant payment process. In this process, the control is handed over to an external USSD merchant payment process. The user will be asked for credentials and the payment will commence. As the process ends, it will call this API to deliver the result and hand control over the session back to the server.",
    ///  "type": "object",
    ///  "required": [
    ///    "amount",
    ///    "merchantCode",
    ///    "operationId",
    ///    "paymentMode"
    ///  ],
    ///  "properties": {
    ///    "amount": {
    ///      "examples": [
    ///        12.34
    ///      ],
    ///      "type": "number",
    ///      "format": "Amount in Naira with at most two decimal places."
    ///    },
    ///    "executionMode": {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess.ExecutionMode"
    ///    },
    ///    "merchantCode": {
    ///      "description": "Merchant code",
    ///      "type": "string"
    ///    },
    ///    "operationId": {
    ///      "description": "Merchant payment operation id",
    ///      "type": "string"
    ///    },
    ///    "paymentMode": {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess.PaymentMode"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdProcessMerchantPaymentProcess {
        pub amount: f64,
        #[serde(
            rename = "executionMode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub execution_mode: ::std::option::Option<MerchantPaymentProcessExecutionMode>,
        ///Merchant code
        #[serde(rename = "merchantCode")]
        pub merchant_code: ::std::string::String,
        ///Merchant payment operation id
        #[serde(rename = "operationId")]
        pub operation_id: ::std::string::String,
        #[serde(rename = "paymentMode")]
        pub payment_mode: MerchantPaymentProcessPaymentMode,
    }
    impl ::std::convert::From<&UssdProcessMerchantPaymentProcess>
    for UssdProcessMerchantPaymentProcess {
        fn from(value: &UssdProcessMerchantPaymentProcess) -> Self {
            value.clone()
        }
    }
    ///Command issued to the user, along with context data.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Command issued to the user, along with context data.",
    ///  "type": "object",
    ///  "required": [
    ///    "action",
    ///    "contextData"
    ///  ],
    ///  "properties": {
    ///    "action": {
    ///      "$ref": "#/components/schemas/UssdAction"
    ///    },
    ///    "contextData": {
    ///      "description": "String set by the developer and carried over throughout the session.",
    ///      "type": "string"
    ///    },
    ///    "sessionTag": {
    ///      "description": "String value set by the developer, it will be visible in the billing information. Most recent session tag replaces previous value.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdSessionCommand {
        pub action: UssdAction,
        ///String set by the developer and carried over throughout the session.
        #[serde(rename = "contextData")]
        pub context_data: ::std::string::String,
        ///String value set by the developer, it will be visible in the billing information. Most recent session tag replaces previous value.
        #[serde(
            rename = "sessionTag",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub session_tag: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&UssdSessionCommand> for UssdSessionCommand {
        fn from(value: &UssdSessionCommand) -> Self {
            value.clone()
        }
    }
    ///The event sent when the USSD session starts (user initiates the session).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The event sent when the USSD session starts (user initiates the session).",
    ///  "type": "object",
    ///  "required": [
    ///    "appId",
    ///    "clientId",
    ///    "input",
    ///    "msisdn",
    ///    "operator",
    ///    "sessionId"
    ///  ],
    ///  "properties": {
    ///    "appId": {
    ///      "description": "Identifier of the USSD app created on Qrios platform.",
    ///      "examples": [
    ///        "b56r455"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "description": "Client identifier of the developer using Qrios platform.",
    ///      "type": "string"
    ///    },
    ///    "input": {
    ///      "$ref": "#/components/schemas/UssdSessionEvent.NewSession.SessionInput"
    ///    },
    ///    "msisdn": {
    ///      "description": "Phone number of customer who initiated the USSD session (e.g. +2341234567891).",
    ///      "examples": [
    ///        "1234512345"
    ///      ],
    ///      "type": "string",
    ///      "format": "+234'10 digits' or 234'10 digits' or 0'10 digits' or +0'10 digits' or '10 digits'"
    ///    },
    ///    "operator": {
    ///      "description": "MSISDN's mobile operator (e.g. mtn).",
    ///      "type": "string",
    ///      "enum": [
    ///        "mtn",
    ///        "airtel",
    ///        "glo",
    ///        "etisalat"
    ///      ]
    ///    },
    ///    "sessionId": {
    ///      "description": "Unique identifier of USSD session.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdSessionEventNewSession {
        ///Identifier of the USSD app created on Qrios platform.
        #[serde(rename = "appId")]
        pub app_id: ::std::string::String,
        ///Client identifier of the developer using Qrios platform.
        #[serde(rename = "clientId")]
        pub client_id: ::std::string::String,
        pub input: UssdSessionEventNewSessionSessionInput,
        ///Phone number of customer who initiated the USSD session (e.g. +2341234567891).
        pub msisdn: ::std::string::String,
        ///MSISDN's mobile operator (e.g. mtn).
        pub operator: UssdSessionEventNewSessionOperator,
        ///Unique identifier of USSD session.
        #[serde(rename = "sessionId")]
        pub session_id: ::std::string::String,
    }
    impl ::std::convert::From<&UssdSessionEventNewSession>
    for UssdSessionEventNewSession {
        fn from(value: &UssdSessionEventNewSession) -> Self {
            value.clone()
        }
    }
    ///MSISDN's mobile operator (e.g. mtn).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "MSISDN's mobile operator (e.g. mtn).",
    ///  "type": "string",
    ///  "enum": [
    ///    "mtn",
    ///    "airtel",
    ///    "glo",
    ///    "etisalat"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdSessionEventNewSessionOperator {
        #[serde(rename = "mtn")]
        Mtn,
        #[serde(rename = "airtel")]
        Airtel,
        #[serde(rename = "glo")]
        Glo,
        #[serde(rename = "etisalat")]
        Etisalat,
    }
    impl ::std::convert::From<&Self> for UssdSessionEventNewSessionOperator {
        fn from(value: &UssdSessionEventNewSessionOperator) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdSessionEventNewSessionOperator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Mtn => f.write_str("mtn"),
                Self::Airtel => f.write_str("airtel"),
                Self::Glo => f.write_str("glo"),
                Self::Etisalat => f.write_str("etisalat"),
            }
        }
    }
    impl ::std::str::FromStr for UssdSessionEventNewSessionOperator {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "mtn" => Ok(Self::Mtn),
                "airtel" => Ok(Self::Airtel),
                "glo" => Ok(Self::Glo),
                "etisalat" => Ok(Self::Etisalat),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdSessionEventNewSessionOperator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdSessionEventNewSessionOperator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdSessionEventNewSessionOperator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///Input provided when starting the session.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Input provided when starting the session.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Dial"
    ///      ],
    ///      "properties": {
    ///        "Dial": {
    ///          "$ref": "#/components/schemas/NewSession.SessionInput.Dial"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Push"
    ///      ],
    ///      "properties": {
    ///        "Push": {
    ///          "$ref": "#/components/schemas/NewSession.SessionInput.Push"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Redirect"
    ///      ],
    ///      "properties": {
    ///        "Redirect": {
    ///          "$ref": "#/components/schemas/NewSession.SessionInput.Redirect"
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub enum UssdSessionEventNewSessionSessionInput {
        Dial(NewSessionSessionInputDial),
        Push(NewSessionSessionInputPush),
        Redirect(NewSessionSessionInputRedirect),
    }
    impl ::std::convert::From<&Self> for UssdSessionEventNewSessionSessionInput {
        fn from(value: &UssdSessionEventNewSessionSessionInput) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<NewSessionSessionInputDial>
    for UssdSessionEventNewSessionSessionInput {
        fn from(value: NewSessionSessionInputDial) -> Self {
            Self::Dial(value)
        }
    }
    impl ::std::convert::From<NewSessionSessionInputPush>
    for UssdSessionEventNewSessionSessionInput {
        fn from(value: NewSessionSessionInputPush) -> Self {
            Self::Push(value)
        }
    }
    impl ::std::convert::From<NewSessionSessionInputRedirect>
    for UssdSessionEventNewSessionSessionInput {
        fn from(value: NewSessionSessionInputRedirect) -> Self {
            Self::Redirect(value)
        }
    }
    ///`UssdView`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/UssdView.ChooserView"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/UssdView.InfoView"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/UssdView.InputView"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdView {
        ChooserView(UssdViewChooserView),
        InfoView(UssdViewInfoView),
        InputView(UssdViewInputView),
    }
    impl ::std::convert::From<&Self> for UssdView {
        fn from(value: &UssdView) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<UssdViewChooserView> for UssdView {
        fn from(value: UssdViewChooserView) -> Self {
            Self::ChooserView(value)
        }
    }
    impl ::std::convert::From<UssdViewInfoView> for UssdView {
        fn from(value: UssdViewInfoView) -> Self {
            Self::InfoView(value)
        }
    }
    impl ::std::convert::From<UssdViewInputView> for UssdView {
        fn from(value: UssdViewInputView) -> Self {
            Self::InputView(value)
        }
    }
    /**A chooser (menu) view. User can choose one of the chooser items.
If user picks invalid option (when user input does not match any of the available items), then the user will be asked to try again.
It will be done internally by the USSD API without involving developer's app.
It's guaranteed, that the user input sent in next request to the developer's app will match one of the provided items.
The session will continue when this message is sent.*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A chooser (menu) view. User can choose one of the chooser items.\nIf user picks invalid option (when user input does not match any of the available items), then the user will be asked to try again.\nIt will be done internally by the USSD API without involving developer's app.\nIt's guaranteed, that the user input sent in next request to the developer's app will match one of the provided items.\nThe session will continue when this message is sent.",
    ///  "type": "object",
    ///  "required": [
    ///    "title",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "Chooser items, each will be presented to user in a new line. All items must have unique access keys.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UssdView.ChooserView.Item"
    ///      },
    ///      "minItems": 1
    ///    },
    ///    "separator": {
    ///      "description": "Text, that separates each item's accessKey and label when the ChooserView is rendered",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "title": {
    ///      "description": "Text that will be displayed before the chooser items",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "ChooserView"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "ChooserView"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdViewChooserView {
        ///Chooser items, each will be presented to user in a new line. All items must have unique access keys.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub items: ::std::vec::Vec<UssdViewChooserViewItem>,
        ///Text, that separates each item's accessKey and label when the ChooserView is rendered
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub separator: ::std::option::Option<UssdViewChooserViewSeparator>,
        ///Text that will be displayed before the chooser items
        pub title: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: UssdViewChooserViewType,
    }
    impl ::std::convert::From<&UssdViewChooserView> for UssdViewChooserView {
        fn from(value: &UssdViewChooserView) -> Self {
            value.clone()
        }
    }
    ///Represents single chooser view item
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Represents single chooser view item",
    ///  "type": "object",
    ///  "required": [
    ///    "accessKey",
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "accessKey": {
    ///      "description": "Text, that identifies the chooser item. User will have to send this value as input in order to choose the item",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "label": {
    ///      "description": "Description of the choose item",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdViewChooserViewItem {
        ///Text, that identifies the chooser item. User will have to send this value as input in order to choose the item
        #[serde(rename = "accessKey")]
        pub access_key: UssdViewChooserViewItemAccessKey,
        ///Description of the choose item
        pub label: UssdViewChooserViewItemLabel,
    }
    impl ::std::convert::From<&UssdViewChooserViewItem> for UssdViewChooserViewItem {
        fn from(value: &UssdViewChooserViewItem) -> Self {
            value.clone()
        }
    }
    ///Text, that identifies the chooser item. User will have to send this value as input in order to choose the item
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Text, that identifies the chooser item. User will have to send this value as input in order to choose the item",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UssdViewChooserViewItemAccessKey(::std::string::String);
    impl ::std::ops::Deref for UssdViewChooserViewItemAccessKey {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UssdViewChooserViewItemAccessKey>
    for ::std::string::String {
        fn from(value: UssdViewChooserViewItemAccessKey) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UssdViewChooserViewItemAccessKey>
    for UssdViewChooserViewItemAccessKey {
        fn from(value: &UssdViewChooserViewItemAccessKey) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for UssdViewChooserViewItemAccessKey {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdViewChooserViewItemAccessKey {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdViewChooserViewItemAccessKey {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdViewChooserViewItemAccessKey {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UssdViewChooserViewItemAccessKey {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Description of the choose item
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Description of the choose item",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UssdViewChooserViewItemLabel(::std::string::String);
    impl ::std::ops::Deref for UssdViewChooserViewItemLabel {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UssdViewChooserViewItemLabel> for ::std::string::String {
        fn from(value: UssdViewChooserViewItemLabel) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UssdViewChooserViewItemLabel>
    for UssdViewChooserViewItemLabel {
        fn from(value: &UssdViewChooserViewItemLabel) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for UssdViewChooserViewItemLabel {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdViewChooserViewItemLabel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdViewChooserViewItemLabel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdViewChooserViewItemLabel {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UssdViewChooserViewItemLabel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Text, that separates each item's accessKey and label when the ChooserView is rendered
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Text, that separates each item's accessKey and label when the ChooserView is rendered",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UssdViewChooserViewSeparator(::std::string::String);
    impl ::std::ops::Deref for UssdViewChooserViewSeparator {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UssdViewChooserViewSeparator> for ::std::string::String {
        fn from(value: UssdViewChooserViewSeparator) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UssdViewChooserViewSeparator>
    for UssdViewChooserViewSeparator {
        fn from(value: &UssdViewChooserViewSeparator) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for UssdViewChooserViewSeparator {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdViewChooserViewSeparator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UssdViewChooserViewSeparator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UssdViewChooserViewSeparator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UssdViewChooserViewSeparator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`UssdViewChooserViewType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "ChooserView"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "ChooserView"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdViewChooserViewType {
        ChooserView,
    }
    impl ::std::convert::From<&Self> for UssdViewChooserViewType {
        fn from(value: &UssdViewChooserViewType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdViewChooserViewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ChooserView => f.write_str("ChooserView"),
            }
        }
    }
    impl ::std::str::FromStr for UssdViewChooserViewType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ChooserView" => Ok(Self::ChooserView),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdViewChooserViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UssdViewChooserViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UssdViewChooserViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A text-only view; does not take any user input. The session will be closed when this message is sent.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A text-only view; does not take any user input. The session will be closed when this message is sent.",
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "Final message that will be displayed to the user",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "InfoView"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "InfoView"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdViewInfoView {
        ///Final message that will be displayed to the user
        pub message: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: UssdViewInfoViewType,
    }
    impl ::std::convert::From<&UssdViewInfoView> for UssdViewInfoView {
        fn from(value: &UssdViewInfoView) -> Self {
            value.clone()
        }
    }
    ///`UssdViewInfoViewType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "InfoView"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "InfoView"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdViewInfoViewType {
        InfoView,
    }
    impl ::std::convert::From<&Self> for UssdViewInfoViewType {
        fn from(value: &UssdViewInfoViewType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdViewInfoViewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InfoView => f.write_str("InfoView"),
            }
        }
    }
    impl ::std::str::FromStr for UssdViewInfoViewType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "InfoView" => Ok(Self::InfoView),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdViewInfoViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UssdViewInfoViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UssdViewInfoViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///A view with text; takes user input. The session will continue when this message is sent.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A view with text; takes user input. The session will continue when this message is sent.",
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "Message that will be displayed to the user, for example asking the user to input some value",
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "InputView"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "InputView"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UssdViewInputView {
        ///Message that will be displayed to the user, for example asking the user to input some value
        pub message: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: UssdViewInputViewType,
    }
    impl ::std::convert::From<&UssdViewInputView> for UssdViewInputView {
        fn from(value: &UssdViewInputView) -> Self {
            value.clone()
        }
    }
    ///`UssdViewInputViewType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "InputView"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "InputView"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        ::serde::Deserialize,
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd
    )]
    pub enum UssdViewInputViewType {
        InputView,
    }
    impl ::std::convert::From<&Self> for UssdViewInputViewType {
        fn from(value: &UssdViewInputViewType) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for UssdViewInputViewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InputView => f.write_str("InputView"),
            }
        }
    }
    impl ::std::str::FromStr for UssdViewInputViewType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "InputView" => Ok(Self::InputView),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UssdViewInputViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UssdViewInputViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UssdViewInputViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for QRIOS USSD API documentation

This API is *not* exposed by Qrios, but it should be implemented by the developer. Qrios will call this API on developer's side to provide information about USSD sessions and the developer should answer with information regarding what should happen next in the aforementioned sessions.

Version: 1.6.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.6.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
#[allow(clippy::all)]
impl Client {
    /**Called when a new USSD session is starting

This endpoint is called when the user enters the USSD session (either by user entering the session via shortcode or developer initiating it via push message using Qrios API or redirecting from other USSD app). Here, we're provided with basic information about the user - their phone number and mobile operator. We also get input, which can either be a shortcode dialed by the user (e.g. \*425\*\<app_sub_shortcode>\*\<input>#) or context data associated with push message request from Qrios API (using `/ussd/app/push` endpoint) or redirection to optional process. In order to track the session, a unique session ID is also provided - it will be repeated through all messages sent to the server that relate to the session. There are also data about client ID (ID of the developer who runs the app) and app ID. <br/> The response should include what the user should see in USSD response message - you can show them a view (a message with optional ability to input text) or redirect to other USSD API application or ask a process to guide them towards an activity (e.g. making a payment). You can also pass context data, which will be included in the next message sent to the developer's app.

Sends a `POST` request to `/ussdSessionEvent/new`

Arguments:
- `authorization`: The authorization secret value according to the configured authorization method of the USSD app
- `body`
*/
    pub async fn post_ussdsessionevent_new<'a>(
        &'a self,
        authorization: Option<&'a str>,
        body: &'a types::UssdSessionEventNewSession,
    ) -> Result<ResponseValue<types::UssdSessionCommand>, Error<()>> {
        let url = format!("{}/ussdSessionEvent/new", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = authorization {
            header_map.append("Authorization", value.to_string().try_into()?);
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_ussdsessionevent_new",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Called when a USSD session is continuing (user responded to previous message)

This endpoint is called when the user decides to continue a continuing session by providing input. Here, we're provided with data that identify the application (client ID and app ID) and current session (session ID). Then, the request passes a result - it can either be a value inputted by the user (with InputView) or the result of a process (e.g. merchant payment process). The result is an outcome of previous response sent to the server. The request also passes contextData that have been previously submitted. <br/> The response should include what the user should see in USSD response message - you can show them a view (a message with optional ability to input text) or redirect user to the other USSD API application or ask a process to guide them towards an activity (e.g. making a payment). You can also pass context data, which will be included in the next message sent to the developer's app.

Sends a `POST` request to `/ussdSessionEvent/continue`

Arguments:
- `authorization`: The authorization secret value according to the configured authorization method of the USSD app
- `body`
*/
    pub async fn post_ussdsessionevent_continue<'a>(
        &'a self,
        authorization: Option<&'a str>,
        body: &'a types::ContinueSession,
    ) -> Result<ResponseValue<types::UssdSessionCommand>, Error<()>> {
        let url = format!("{}/ussdSessionEvent/continue", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = authorization {
            header_map.append("Authorization", value.to_string().try_into()?);
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                ::reqwest::header::ACCEPT,
                ::reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .headers(header_map)
            .build()?;
        let info = OperationInfo {
            operation_id: "post_ussdsessionevent_continue",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Called when a USSD session is closing gracefully (user cannot provide more input, user exits, session times out)

This endpoint is called when the session closes gracefully, i.e. the end of the session is not caused by an error. There are three ways in which that can happen - the session may end naturally, as no input can be provided by the user (that happens when InfoView is used); the user may end the session by pressing "Cancel" on their phone; or the session is ended by the mobile operator due to timeout (e.g. two minutes).<br/>No response body is expected at this point as the session has been closed.

Sends a `POST` request to `/ussdSessionEvent/close`

Arguments:
- `authorization`: The authorization secret value according to the configured authorization method of the USSD app
- `body`
*/
    pub async fn post_ussdsessionevent_close<'a>(
        &'a self,
        authorization: Option<&'a str>,
        body: &'a types::CloseSession,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/ussdSessionEvent/close", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = authorization {
            header_map.append("Authorization", value.to_string().try_into()?);
        }
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "post_ussdsessionevent_close",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**Called when a USSD session is being aborted (internal error)

This endpoint is called when the session is aborted due to an internal error. This can happen in a number of ways, which are not usually detailed - the exception being an error of insufficient funds available in the virtual purse. When such an error happens, the USSD session is ended and a request is sent to this endpoint to inform of an error. <br/>No response body is expected at this point as the session has been aborted.

Sends a `POST` request to `/ussdSessionEvent/abort`

Arguments:
- `authorization`: The authorization secret value according to the configured authorization method of the USSD app
- `body`
*/
    pub async fn post_ussdsessionevent_abort<'a>(
        &'a self,
        authorization: Option<&'a str>,
        body: &'a types::AbortSession,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/ussdSessionEvent/abort", self.baseurl,);
        let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
        header_map
            .append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(Self::api_version()),
            );
        if let Some(value) = authorization {
            header_map.append("Authorization", value.to_string().try_into()?);
        }
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).headers(header_map).build()?;
        let info = OperationInfo {
            operation_id: "post_ussdsessionevent_abort",
        };
        self.pre(&mut request, &info).await?;
        let result = self.exec(request, &info).await;
        self.post(&result, &info).await?;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
