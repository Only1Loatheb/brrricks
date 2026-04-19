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
    ///      "examples": [
    ///        "Abandon"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Abandon"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Abandon {
        #[serde(rename = "type")]
        pub type_: AbandonType,
    }
    ///`AbandonType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Abandon"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Abandon"
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
    pub enum AbandonType {
        Abandon,
    }
    impl ::std::fmt::Display for AbandonType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Abandon => f.write_str("Abandon"),
            }
        }
    }
    impl ::std::str::FromStr for AbandonType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Abandon" => Ok(Self::Abandon),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for AbandonType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for AbandonType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for AbandonType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///`AbortSessionAbortReason`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/DuplicatedOperation"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/InsufficientBalanceInVirtualPurse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/InternalError"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MissingPrivilege"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/UnexpectedUssdAppResponse"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AbortSessionAbortReason {
        DuplicatedOperation(DuplicatedOperation),
        InsufficientBalanceInVirtualPurse(InsufficientBalanceInVirtualPurse),
        InternalError(InternalError),
        MissingPrivilege(MissingPrivilege),
        UnexpectedUssdAppResponse(UnexpectedUssdAppResponse),
    }
    impl ::std::convert::From<DuplicatedOperation> for AbortSessionAbortReason {
        fn from(value: DuplicatedOperation) -> Self {
            Self::DuplicatedOperation(value)
        }
    }
    impl ::std::convert::From<InsufficientBalanceInVirtualPurse>
    for AbortSessionAbortReason {
        fn from(value: InsufficientBalanceInVirtualPurse) -> Self {
            Self::InsufficientBalanceInVirtualPurse(value)
        }
    }
    impl ::std::convert::From<InternalError> for AbortSessionAbortReason {
        fn from(value: InternalError) -> Self {
            Self::InternalError(value)
        }
    }
    impl ::std::convert::From<MissingPrivilege> for AbortSessionAbortReason {
        fn from(value: MissingPrivilege) -> Self {
            Self::MissingPrivilege(value)
        }
    }
    impl ::std::convert::From<UnexpectedUssdAppResponse> for AbortSessionAbortReason {
        fn from(value: UnexpectedUssdAppResponse) -> Self {
            Self::UnexpectedUssdAppResponse(value)
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
    pub struct ChooserView {
        ///Chooser items, each will be presented to user in a new line. All items must have unique access keys.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub items: ::std::vec::Vec<UssdViewChooserViewItem>,
        ///Text, that separates each item's accessKey and label when the ChooserView is rendered
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub separator: ::std::option::Option<ChooserViewSeparator>,
        ///Text that will be displayed before the chooser items
        pub title: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ChooserViewType,
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
    pub struct ChooserViewSeparator(::std::string::String);
    impl ::std::ops::Deref for ChooserViewSeparator {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ChooserViewSeparator> for ::std::string::String {
        fn from(value: ChooserViewSeparator) -> Self {
            value.0
        }
    }
    impl ::std::str::FromStr for ChooserViewSeparator {
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
    impl ::std::convert::TryFrom<&str> for ChooserViewSeparator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ChooserViewSeparator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ChooserViewSeparator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ChooserViewSeparator {
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
    ///`ChooserViewType`
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
    pub enum ChooserViewType {
        ChooserView,
    }
    impl ::std::fmt::Display for ChooserViewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ChooserView => f.write_str("ChooserView"),
            }
        }
    }
    impl ::std::str::FromStr for ChooserViewType {
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
    impl ::std::convert::TryFrom<&str> for ChooserViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for ChooserViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for ChooserViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///`CloseSessionCloseReason`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Abandon"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/End"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Timeout"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum CloseSessionCloseReason {
        Abandon(Abandon),
        End(End),
        Timeout(Timeout),
    }
    impl ::std::convert::From<Abandon> for CloseSessionCloseReason {
        fn from(value: Abandon) -> Self {
            Self::Abandon(value)
        }
    }
    impl ::std::convert::From<End> for CloseSessionCloseReason {
        fn from(value: End) -> Self {
            Self::End(value)
        }
    }
    impl ::std::convert::From<Timeout> for CloseSessionCloseReason {
        fn from(value: Timeout) -> Self {
            Self::Timeout(value)
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
    pub struct Dial {
        ///Shortcode string dialed by the user
        #[serde(rename = "shortcodeString")]
        pub shortcode_string: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: DialType,
    }
    ///`DialType`
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
    pub enum DialType {
        Dial,
    }
    impl ::std::fmt::Display for DialType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Dial => f.write_str("Dial"),
            }
        }
    }
    impl ::std::str::FromStr for DialType {
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
    impl ::std::convert::TryFrom<&str> for DialType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DialType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DialType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "DuplicatedOperation"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "DuplicatedOperation"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct DuplicatedOperation {
        #[serde(rename = "operationId")]
        pub operation_id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: DuplicatedOperationType,
    }
    ///`DuplicatedOperationType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "DuplicatedOperation"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "DuplicatedOperation"
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
    pub enum DuplicatedOperationType {
        DuplicatedOperation,
    }
    impl ::std::fmt::Display for DuplicatedOperationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::DuplicatedOperation => f.write_str("DuplicatedOperation"),
            }
        }
    }
    impl ::std::str::FromStr for DuplicatedOperationType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "DuplicatedOperation" => Ok(Self::DuplicatedOperation),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for DuplicatedOperationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DuplicatedOperationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DuplicatedOperationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    pub struct EmbeddedProcess {
        pub params: Map,
        ///embedded process id
        #[serde(rename = "processId")]
        pub process_id: ::std::string::String,
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
    pub struct EmbeddedProcessResult {
        #[serde(rename = "resultParams")]
        pub result_params: UssdActionResultEmbeddedProcessResultResultParams,
        #[serde(rename = "type")]
        pub type_: EmbeddedProcessResultType,
    }
    ///`EmbeddedProcessResultType`
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
    pub enum EmbeddedProcessResultType {
        EmbeddedProcessResult,
    }
    impl ::std::fmt::Display for EmbeddedProcessResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::EmbeddedProcessResult => f.write_str("EmbeddedProcessResult"),
            }
        }
    }
    impl ::std::str::FromStr for EmbeddedProcessResultType {
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
    impl ::std::convert::TryFrom<&str> for EmbeddedProcessResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for EmbeddedProcessResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for EmbeddedProcessResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "End"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "End"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct End {
        #[serde(rename = "type")]
        pub type_: EndType,
    }
    ///`EndType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "End"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "End"
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
    pub enum EndType {
        End,
    }
    impl ::std::fmt::Display for EndType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::End => f.write_str("End"),
            }
        }
    }
    impl ::std::str::FromStr for EndType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "End" => Ok(Self::End),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for EndType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for EndType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for EndType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "Failure"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Failure"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Failure {
        pub cause: MerchantPaymentResultOperationStatusFailureCause,
        #[serde(rename = "type")]
        pub type_: FailureType,
    }
    ///`FailureType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Failure"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Failure"
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
    pub enum FailureType {
        Failure,
    }
    impl ::std::fmt::Display for FailureType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Failure => f.write_str("Failure"),
            }
        }
    }
    impl ::std::str::FromStr for FailureType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Failure" => Ok(Self::Failure),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FailureType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for FailureType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FailureType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "FixedAccount"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "FixedAccount"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FixedAccount {
        ///Number of the account which will be used in transaction. Account numbers can be obtained with Qrios API using `/merchants/accounts` endpoint.
        #[serde(rename = "accountNumber")]
        pub account_number: ::std::string::String,
        ///Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
        pub bank: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: FixedAccountType,
    }
    ///`FixedAccountType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "FixedAccount"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "FixedAccount"
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
    pub enum FixedAccountType {
        FixedAccount,
    }
    impl ::std::fmt::Display for FixedAccountType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FixedAccount => f.write_str("FixedAccount"),
            }
        }
    }
    impl ::std::str::FromStr for FixedAccountType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FixedAccount" => Ok(Self::FixedAccount),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FixedAccountType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for FixedAccountType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FixedAccountType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "FixedBank"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "FixedBank"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct FixedBank {
        ///Code of the bank which will be used in transaction. Bank codes can be obtained with Qrios API using `/merchants/accounts` endpoint.
        pub bank: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: FixedBankType,
    }
    ///`FixedBankType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "FixedBank"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "FixedBank"
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
    pub enum FixedBankType {
        FixedBank,
    }
    impl ::std::fmt::Display for FixedBankType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::FixedBank => f.write_str("FixedBank"),
            }
        }
    }
    impl ::std::str::FromStr for FixedBankType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FixedBank" => Ok(Self::FixedBank),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FixedBankType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for FixedBankType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FixedBankType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "Flexible"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Flexible"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Flexible {
        #[serde(rename = "type")]
        pub type_: FlexibleType,
    }
    ///`FlexibleType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Flexible"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Flexible"
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
    pub enum FlexibleType {
        Flexible,
    }
    impl ::std::fmt::Display for FlexibleType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Flexible => f.write_str("Flexible"),
            }
        }
    }
    impl ::std::str::FromStr for FlexibleType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Flexible" => Ok(Self::Flexible),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for FlexibleType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for FlexibleType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for FlexibleType {
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
    pub struct InfoView {
        ///Final message that will be displayed to the user
        pub message: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: InfoViewType,
    }
    ///`InfoViewType`
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
    pub enum InfoViewType {
        InfoView,
    }
    impl ::std::fmt::Display for InfoViewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InfoView => f.write_str("InfoView"),
            }
        }
    }
    impl ::std::str::FromStr for InfoViewType {
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
    impl ::std::convert::TryFrom<&str> for InfoViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for InfoViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for InfoViewType {
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
    pub struct InputResult {
        #[serde(rename = "type")]
        pub type_: InputResultType,
        ///Raw value typed in by the user.
        pub value: ::std::string::String,
    }
    ///`InputResultType`
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
    pub enum InputResultType {
        InputResult,
    }
    impl ::std::fmt::Display for InputResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InputResult => f.write_str("InputResult"),
            }
        }
    }
    impl ::std::str::FromStr for InputResultType {
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
    impl ::std::convert::TryFrom<&str> for InputResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for InputResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for InputResultType {
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
    pub struct InputView {
        ///Message that will be displayed to the user, for example asking the user to input some value
        pub message: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: InputViewType,
    }
    ///`InputViewType`
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
    pub enum InputViewType {
        InputView,
    }
    impl ::std::fmt::Display for InputViewType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InputView => f.write_str("InputView"),
            }
        }
    }
    impl ::std::str::FromStr for InputViewType {
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
    impl ::std::convert::TryFrom<&str> for InputViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for InputViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for InputViewType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "InsufficientBalanceInVirtualPurse"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "InsufficientBalanceInVirtualPurse"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InsufficientBalanceInVirtualPurse {
        #[serde(rename = "type")]
        pub type_: InsufficientBalanceInVirtualPurseType,
    }
    ///`InsufficientBalanceInVirtualPurseType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "InsufficientBalanceInVirtualPurse"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "InsufficientBalanceInVirtualPurse"
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
    pub enum InsufficientBalanceInVirtualPurseType {
        InsufficientBalanceInVirtualPurse,
    }
    impl ::std::fmt::Display for InsufficientBalanceInVirtualPurseType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InsufficientBalanceInVirtualPurse => {
                    f.write_str("InsufficientBalanceInVirtualPurse")
                }
            }
        }
    }
    impl ::std::str::FromStr for InsufficientBalanceInVirtualPurseType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "InsufficientBalanceInVirtualPurse" => {
                    Ok(Self::InsufficientBalanceInVirtualPurse)
                }
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for InsufficientBalanceInVirtualPurseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for InsufficientBalanceInVirtualPurseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for InsufficientBalanceInVirtualPurseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "InternalError"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "InternalError"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct InternalError {
        #[serde(rename = "type")]
        pub type_: InternalErrorType,
    }
    ///`InternalErrorType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "InternalError"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "InternalError"
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
    pub enum InternalErrorType {
        InternalError,
    }
    impl ::std::fmt::Display for InternalErrorType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::InternalError => f.write_str("InternalError"),
            }
        }
    }
    impl ::std::str::FromStr for InternalErrorType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "InternalError" => Ok(Self::InternalError),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for InternalErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for InternalErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for InternalErrorType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "examples": [
    ///        "LegacyAppRedirect"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "LegacyAppRedirect"
    ///      ]
    ///    },
    ///    "uri": {
    ///      "description": "Url of the Legacy USSD App",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct LegacyAppRedirect {
        #[serde(rename = "type")]
        pub type_: LegacyAppRedirectType,
        ///Url of the Legacy USSD App
        pub uri: ::std::string::String,
    }
    ///`LegacyAppRedirectType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "LegacyAppRedirect"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "LegacyAppRedirect"
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
    pub enum LegacyAppRedirectType {
        LegacyAppRedirect,
    }
    impl ::std::fmt::Display for LegacyAppRedirectType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::LegacyAppRedirect => f.write_str("LegacyAppRedirect"),
            }
        }
    }
    impl ::std::str::FromStr for LegacyAppRedirectType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "LegacyAppRedirect" => Ok(Self::LegacyAppRedirect),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for LegacyAppRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for LegacyAppRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for LegacyAppRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    pub struct MerchantPaymentProcess {
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
    ///`MerchantPaymentProcessExecutionMode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/WithBankResponseTimeout"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/WithoutWaitingForBank"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MerchantPaymentProcessExecutionMode {
        WithBankResponseTimeout(WithBankResponseTimeout),
        WithoutWaitingForBank(WithoutWaitingForBank),
    }
    impl ::std::convert::From<WithBankResponseTimeout>
    for MerchantPaymentProcessExecutionMode {
        fn from(value: WithBankResponseTimeout) -> Self {
            Self::WithBankResponseTimeout(value)
        }
    }
    impl ::std::convert::From<WithoutWaitingForBank>
    for MerchantPaymentProcessExecutionMode {
        fn from(value: WithoutWaitingForBank) -> Self {
            Self::WithoutWaitingForBank(value)
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
    ///      "$ref": "#/components/schemas/FixedAccount"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/FixedBank"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Flexible"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MerchantPaymentProcessPaymentMode {
        FixedAccount(FixedAccount),
        FixedBank(FixedBank),
        Flexible(Flexible),
    }
    impl ::std::convert::From<FixedAccount> for MerchantPaymentProcessPaymentMode {
        fn from(value: FixedAccount) -> Self {
            Self::FixedAccount(value)
        }
    }
    impl ::std::convert::From<FixedBank> for MerchantPaymentProcessPaymentMode {
        fn from(value: FixedBank) -> Self {
            Self::FixedBank(value)
        }
    }
    impl ::std::convert::From<Flexible> for MerchantPaymentProcessPaymentMode {
        fn from(value: Flexible) -> Self {
            Self::Flexible(value)
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
    pub struct MerchantPaymentResult {
        ///Merchant payment operation id
        #[serde(rename = "operationId")]
        pub operation_id: ::std::string::String,
        pub status: MerchantPaymentResultOperationStatus,
        #[serde(rename = "type")]
        pub type_: MerchantPaymentResultType,
    }
    ///`MerchantPaymentResultOperationStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Failure"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Success"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Unknown"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MerchantPaymentResultOperationStatus {
        Failure(Failure),
        Success(Success),
        Unknown(Unknown),
    }
    impl ::std::convert::From<Failure> for MerchantPaymentResultOperationStatus {
        fn from(value: Failure) -> Self {
            Self::Failure(value)
        }
    }
    impl ::std::convert::From<Success> for MerchantPaymentResultOperationStatus {
        fn from(value: Success) -> Self {
            Self::Success(value)
        }
    }
    impl ::std::convert::From<Unknown> for MerchantPaymentResultOperationStatus {
        fn from(value: Unknown) -> Self {
            Self::Unknown(value)
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
    ///`MerchantPaymentResultType`
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
    pub enum MerchantPaymentResultType {
        MerchantPaymentResult,
    }
    impl ::std::fmt::Display for MerchantPaymentResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::MerchantPaymentResult => f.write_str("MerchantPaymentResult"),
            }
        }
    }
    impl ::std::str::FromStr for MerchantPaymentResultType {
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
    impl ::std::convert::TryFrom<&str> for MerchantPaymentResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MerchantPaymentResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MerchantPaymentResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "MissingPrivilege"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "MissingPrivilege"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct MissingPrivilege {
        pub privilege: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: MissingPrivilegeType,
    }
    ///`MissingPrivilegeType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "MissingPrivilege"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "MissingPrivilege"
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
    pub enum MissingPrivilegeType {
        MissingPrivilege,
    }
    impl ::std::fmt::Display for MissingPrivilegeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::MissingPrivilege => f.write_str("MissingPrivilege"),
            }
        }
    }
    impl ::std::str::FromStr for MissingPrivilegeType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MissingPrivilege" => Ok(Self::MissingPrivilege),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for MissingPrivilegeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MissingPrivilegeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MissingPrivilegeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    pub struct Push {
        ///Context regarding the push message.
        #[serde(rename = "contextData")]
        pub context_data: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: PushType,
    }
    ///`PushType`
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
    pub enum PushType {
        Push,
    }
    impl ::std::fmt::Display for PushType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Push => f.write_str("Push"),
            }
        }
    }
    impl ::std::str::FromStr for PushType {
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
    impl ::std::convert::TryFrom<&str> for PushType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PushType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PushType {
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
    pub struct Redirect {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub process: ::std::option::Option<UssdAppProcess>,
        #[serde(
            rename = "processId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub process_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "type")]
        pub type_: RedirectType,
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
    pub struct ReturnFromRedirectResult {
        #[serde(rename = "resultParams")]
        pub result_params: UssdActionResultReturnFromRedirectResultResultParams,
        #[serde(rename = "type")]
        pub type_: ReturnFromRedirectResultType,
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
    ///`ReturnFromRedirectResultType`
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
    pub enum ReturnFromRedirectResultType {
        ReturnFromRedirectResult,
    }
    impl ::std::fmt::Display for ReturnFromRedirectResultType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ReturnFromRedirectResult => f.write_str("ReturnFromRedirectResult"),
            }
        }
    }
    impl ::std::str::FromStr for ReturnFromRedirectResultType {
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
    impl ::std::convert::TryFrom<&str> for ReturnFromRedirectResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReturnFromRedirectResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReturnFromRedirectResultType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "Success"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Success"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Success {
        #[serde(rename = "type")]
        pub type_: SuccessType,
    }
    ///`SuccessType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Success"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Success"
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
    pub enum SuccessType {
        Success,
    }
    impl ::std::fmt::Display for SuccessType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Success => f.write_str("Success"),
            }
        }
    }
    impl ::std::str::FromStr for SuccessType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Success" => Ok(Self::Success),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SuccessType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SuccessType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SuccessType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "Timeout"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Timeout"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Timeout {
        #[serde(rename = "type")]
        pub type_: TimeoutType,
    }
    ///`TimeoutType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Timeout"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Timeout"
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
    pub enum TimeoutType {
        Timeout,
    }
    impl ::std::fmt::Display for TimeoutType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Timeout => f.write_str("Timeout"),
            }
        }
    }
    impl ::std::str::FromStr for TimeoutType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Timeout" => Ok(Self::Timeout),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for TimeoutType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for TimeoutType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for TimeoutType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "UnexpectedUssdAppResponse"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "UnexpectedUssdAppResponse"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct UnexpectedUssdAppResponse {
        #[serde(rename = "type")]
        pub type_: UnexpectedUssdAppResponseType,
    }
    ///`UnexpectedUssdAppResponseType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "UnexpectedUssdAppResponse"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "UnexpectedUssdAppResponse"
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
    pub enum UnexpectedUssdAppResponseType {
        UnexpectedUssdAppResponse,
    }
    impl ::std::fmt::Display for UnexpectedUssdAppResponseType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::UnexpectedUssdAppResponse => {
                    f.write_str("UnexpectedUssdAppResponse")
                }
            }
        }
    }
    impl ::std::str::FromStr for UnexpectedUssdAppResponseType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "UnexpectedUssdAppResponse" => Ok(Self::UnexpectedUssdAppResponse),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UnexpectedUssdAppResponseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for UnexpectedUssdAppResponseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for UnexpectedUssdAppResponseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "Unknown"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "Unknown"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Unknown {
        #[serde(rename = "type")]
        pub type_: UnknownType,
    }
    ///`UnknownType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "Unknown"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "Unknown"
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
    pub enum UnknownType {
        Unknown,
    }
    impl ::std::fmt::Display for UnknownType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Unknown => f.write_str("Unknown"),
            }
        }
    }
    impl ::std::str::FromStr for UnknownType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "Unknown" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for UnknownType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UnknownType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UnknownType {
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
    ///      "$ref": "#/components/schemas/LegacyAppRedirect"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/UssdAction.Redirect"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/ReturnFromRedirect"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/RunProcess"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/ShowView"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdAction {
        LegacyAppRedirect(LegacyAppRedirect),
        UssdActionRedirect(UssdActionRedirect),
        ReturnFromRedirect(ReturnFromRedirect),
        RunProcess(RunProcess),
        ShowView(ShowView),
    }
    impl ::std::convert::From<LegacyAppRedirect> for UssdAction {
        fn from(value: LegacyAppRedirect) -> Self {
            Self::LegacyAppRedirect(value)
        }
    }
    impl ::std::convert::From<UssdActionRedirect> for UssdAction {
        fn from(value: UssdActionRedirect) -> Self {
            Self::UssdActionRedirect(value)
        }
    }
    impl ::std::convert::From<ReturnFromRedirect> for UssdAction {
        fn from(value: ReturnFromRedirect) -> Self {
            Self::ReturnFromRedirect(value)
        }
    }
    impl ::std::convert::From<RunProcess> for UssdAction {
        fn from(value: RunProcess) -> Self {
            Self::RunProcess(value)
        }
    }
    impl ::std::convert::From<ShowView> for UssdAction {
        fn from(value: ShowView) -> Self {
            Self::ShowView(value)
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
    pub struct UssdActionRedirect {
        ///Identifier of the USSD app created on Qrios platform.
        #[serde(rename = "destinationAppId")]
        pub destination_app_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub process: ::std::option::Option<UssdAppProcess>,
        #[serde(rename = "type")]
        pub type_: UssdActionRedirectType,
    }
    ///`UssdActionRedirectType`
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
    pub enum UssdActionRedirectType {
        Redirect,
    }
    impl ::std::fmt::Display for UssdActionRedirectType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Redirect => f.write_str("Redirect"),
            }
        }
    }
    impl ::std::str::FromStr for UssdActionRedirectType {
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
    impl ::std::convert::TryFrom<&str> for UssdActionRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UssdActionRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UssdActionRedirectType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "$ref": "#/components/schemas/EmbeddedProcessResult"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/InputResult"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentResult"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/ReturnFromRedirectResult"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdActionResult {
        EmbeddedProcessResult(EmbeddedProcessResult),
        InputResult(InputResult),
        MerchantPaymentResult(MerchantPaymentResult),
        ReturnFromRedirectResult(ReturnFromRedirectResult),
    }
    impl ::std::convert::From<EmbeddedProcessResult> for UssdActionResult {
        fn from(value: EmbeddedProcessResult) -> Self {
            Self::EmbeddedProcessResult(value)
        }
    }
    impl ::std::convert::From<InputResult> for UssdActionResult {
        fn from(value: InputResult) -> Self {
            Self::InputResult(value)
        }
    }
    impl ::std::convert::From<MerchantPaymentResult> for UssdActionResult {
        fn from(value: MerchantPaymentResult) -> Self {
            Self::MerchantPaymentResult(value)
        }
    }
    impl ::std::convert::From<ReturnFromRedirectResult> for UssdActionResult {
        fn from(value: ReturnFromRedirectResult) -> Self {
            Self::ReturnFromRedirectResult(value)
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
    ///The process control is handed over to (flow continues within the process until the process continues or aborts).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The process control is handed over to (flow continues within the process until the process continues or aborts).",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EmbeddedProcess"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MerchantPaymentProcess"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdProcess {
        EmbeddedProcess(EmbeddedProcess),
        MerchantPaymentProcess(MerchantPaymentProcess),
    }
    impl ::std::convert::From<EmbeddedProcess> for UssdProcess {
        fn from(value: EmbeddedProcess) -> Self {
            Self::EmbeddedProcess(value)
        }
    }
    impl ::std::convert::From<MerchantPaymentProcess> for UssdProcess {
        fn from(value: MerchantPaymentProcess) -> Self {
            Self::MerchantPaymentProcess(value)
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
    ///      "$ref": "#/components/schemas/Dial"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Push"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Redirect"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdSessionEventNewSessionSessionInput {
        Dial(Dial),
        Push(Push),
        Redirect(Redirect),
    }
    impl ::std::convert::From<Dial> for UssdSessionEventNewSessionSessionInput {
        fn from(value: Dial) -> Self {
            Self::Dial(value)
        }
    }
    impl ::std::convert::From<Push> for UssdSessionEventNewSessionSessionInput {
        fn from(value: Push) -> Self {
            Self::Push(value)
        }
    }
    impl ::std::convert::From<Redirect> for UssdSessionEventNewSessionSessionInput {
        fn from(value: Redirect) -> Self {
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
    ///      "$ref": "#/components/schemas/ChooserView"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/InfoView"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/InputView"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum UssdView {
        ChooserView(ChooserView),
        InfoView(InfoView),
        InputView(InputView),
    }
    impl ::std::convert::From<ChooserView> for UssdView {
        fn from(value: ChooserView) -> Self {
            Self::ChooserView(value)
        }
    }
    impl ::std::convert::From<InfoView> for UssdView {
        fn from(value: InfoView) -> Self {
            Self::InfoView(value)
        }
    }
    impl ::std::convert::From<InputView> for UssdView {
        fn from(value: InputView) -> Self {
            Self::InputView(value)
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
    ///      "examples": [
    ///        "WithBankResponseTimeout"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "WithBankResponseTimeout"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WithBankResponseTimeout {
        pub millis: i64,
        #[serde(rename = "type")]
        pub type_: WithBankResponseTimeoutType,
    }
    ///`WithBankResponseTimeoutType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "WithBankResponseTimeout"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "WithBankResponseTimeout"
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
    pub enum WithBankResponseTimeoutType {
        WithBankResponseTimeout,
    }
    impl ::std::fmt::Display for WithBankResponseTimeoutType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::WithBankResponseTimeout => f.write_str("WithBankResponseTimeout"),
            }
        }
    }
    impl ::std::str::FromStr for WithBankResponseTimeoutType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "WithBankResponseTimeout" => Ok(Self::WithBankResponseTimeout),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for WithBankResponseTimeoutType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for WithBankResponseTimeoutType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for WithBankResponseTimeoutType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
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
    ///      "examples": [
    ///        "WithoutWaitingForBank"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "WithoutWaitingForBank"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct WithoutWaitingForBank {
        #[serde(rename = "type")]
        pub type_: WithoutWaitingForBankType,
    }
    ///`WithoutWaitingForBankType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    "WithoutWaitingForBank"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "WithoutWaitingForBank"
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
    pub enum WithoutWaitingForBankType {
        WithoutWaitingForBank,
    }
    impl ::std::fmt::Display for WithoutWaitingForBankType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::WithoutWaitingForBank => f.write_str("WithoutWaitingForBank"),
            }
        }
    }
    impl ::std::str::FromStr for WithoutWaitingForBankType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "WithoutWaitingForBank" => Ok(Self::WithoutWaitingForBank),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for WithoutWaitingForBankType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for WithoutWaitingForBankType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for WithoutWaitingForBankType {
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

Swagger is modified so that the generated client and server matches the examples. Schemas like 'NewSession.SessionInput.Dial' have their 'NewSession.SessionInput.' prefix removed so that server generates correctly. When server generated from swagger with 'NewSession.SessionInput.' prefix parses a request the field named r_type inside Rust struct NewSessionSessionInputDial will not be filled automatically, because "type" json field is already consumed at the wrapper level.

Version: 1.6.4*/
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
        "1.6.4"
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
