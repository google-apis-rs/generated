#![doc = "# Resources and Methods\n    * [assetlinks](resources/assetlinks/struct.AssetlinksActions.html)\n      * [*check*](resources/assetlinks/struct.CheckRequestBuilder.html)\n    * [statements](resources/statements/struct.StatementsActions.html)\n      * [*list*](resources/statements/struct.ListRequestBuilder.html)\n"]
pub mod scopes {}
pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidAppAsset {
        #[doc = "Because there is no global enforcement of package name uniqueness, we also\nrequire a signing certificate, which in combination with the package name\nuniquely identifies an app.\n\nSome apps' signing keys are rotated, so they may be signed by different\nkeys over time.  We treat these as distinct assets, since we use (package\nname, cert) as the unique ID.  This should not normally pose any problems\nas both versions of the app will make the same or similar statements.\nOther assets making statements about the app will have to be updated when a\nkey is rotated, however.\n\n(Note that the syntaxes for publishing and querying for statements contain\nsyntactic sugar to easily let you specify apps that are known by multiple\ncertificates.)\nREQUIRED"]
        #[serde(
            rename = "certificate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub certificate: ::std::option::Option<crate::schemas::CertificateInfo>,
        #[doc = "Android App assets are naturally identified by their Java package name.\nFor example, the Google Maps app uses the package name\n`com.google.android.apps.maps`.\nREQUIRED"]
        #[serde(
            rename = "packageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AndroidAppAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidAppAsset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Asset {
        #[doc = "Set if this is an Android App asset."]
        #[serde(
            rename = "androidApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_app: ::std::option::Option<crate::schemas::AndroidAppAsset>,
        #[doc = "Set if this is a web asset."]
        #[serde(
            rename = "web",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web: ::std::option::Option<crate::schemas::WebAsset>,
    }
    impl ::google_field_selector::FieldSelector for Asset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Asset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CertificateInfo {
        #[doc = "The uppercase SHA-265 fingerprint of the certificate.  From the PEM\ncertificate, it can be acquired like this:\n\n````text\n$ keytool -printcert -file $CERTFILE | grep SHA256:\nSHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\\n    42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nor like this:\n\n````text\n$ openssl x509 -in $CERTFILE -noout -fingerprint -sha256\nSHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\\n    16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nIn this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.\n\nIf these tools are not available to you, you can convert the PEM\ncertificate into the DER format, compute the SHA-256 hash of that string\nand represent the result as a hexstring (that is, uppercase hexadecimal\nrepresentations of each octet, separated by colons)."]
        #[serde(
            rename = "sha256Fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sha_256_fingerprint: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CertificateInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CertificateInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CheckResponse {
        #[doc = "Human-readable message containing information intended to help end users\nunderstand, reproduce and debug the result.\n\nThe message will be in English and we are currently not planning to offer\nany translations.\n\nPlease note that no guarantees are made about the contents or format of\nthis string.  Any aspect of it may be subject to change without notice.\nYou should not attempt to programmatically parse this data.  For\nprogrammatic access, use the error_code field below."]
        #[serde(
            rename = "debugString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_string: ::std::option::Option<String>,
        #[doc = "Error codes that describe the result of the Check operation."]
        #[serde(
            rename = "errorCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_code: ::std::option::Option<Vec<crate::schemas::CheckResponseErrorCodeItems>>,
        #[doc = "Set to true if the assets specified in the request are linked by the\nrelation specified in the request."]
        #[serde(
            rename = "linked",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linked: ::std::option::Option<bool>,
        #[doc = "From serving time, how much longer the response should be considered valid\nbarring further updates.\nREQUIRED"]
        #[serde(
            rename = "maxAge",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_age: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CheckResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CheckResponseErrorCodeItems {
        ErrorCodeFailedSslValidation,
        ErrorCodeFetchBudgetExhausted,
        ErrorCodeFetchError,
        ErrorCodeInvalidQuery,
        ErrorCodeMalformedContent,
        ErrorCodeMalformedHttpResponse,
        ErrorCodeRedirect,
        ErrorCodeSecureAssetIncludesInsecure,
        ErrorCodeTooLarge,
        ErrorCodeUnspecified,
        ErrorCodeWrongContentType,
    }
    impl CheckResponseErrorCodeItems {
        pub fn as_str(self) -> &'static str {
            match self {
                CheckResponseErrorCodeItems::ErrorCodeFailedSslValidation => {
                    "ERROR_CODE_FAILED_SSL_VALIDATION"
                }
                CheckResponseErrorCodeItems::ErrorCodeFetchBudgetExhausted => {
                    "ERROR_CODE_FETCH_BUDGET_EXHAUSTED"
                }
                CheckResponseErrorCodeItems::ErrorCodeFetchError => "ERROR_CODE_FETCH_ERROR",
                CheckResponseErrorCodeItems::ErrorCodeInvalidQuery => "ERROR_CODE_INVALID_QUERY",
                CheckResponseErrorCodeItems::ErrorCodeMalformedContent => {
                    "ERROR_CODE_MALFORMED_CONTENT"
                }
                CheckResponseErrorCodeItems::ErrorCodeMalformedHttpResponse => {
                    "ERROR_CODE_MALFORMED_HTTP_RESPONSE"
                }
                CheckResponseErrorCodeItems::ErrorCodeRedirect => "ERROR_CODE_REDIRECT",
                CheckResponseErrorCodeItems::ErrorCodeSecureAssetIncludesInsecure => {
                    "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE"
                }
                CheckResponseErrorCodeItems::ErrorCodeTooLarge => "ERROR_CODE_TOO_LARGE",
                CheckResponseErrorCodeItems::ErrorCodeUnspecified => "ERROR_CODE_UNSPECIFIED",
                CheckResponseErrorCodeItems::ErrorCodeWrongContentType => {
                    "ERROR_CODE_WRONG_CONTENT_TYPE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CheckResponseErrorCodeItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CheckResponseErrorCodeItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CheckResponseErrorCodeItems, ()> {
            Ok(match s {
                "ERROR_CODE_FAILED_SSL_VALIDATION" => {
                    CheckResponseErrorCodeItems::ErrorCodeFailedSslValidation
                }
                "ERROR_CODE_FETCH_BUDGET_EXHAUSTED" => {
                    CheckResponseErrorCodeItems::ErrorCodeFetchBudgetExhausted
                }
                "ERROR_CODE_FETCH_ERROR" => CheckResponseErrorCodeItems::ErrorCodeFetchError,
                "ERROR_CODE_INVALID_QUERY" => CheckResponseErrorCodeItems::ErrorCodeInvalidQuery,
                "ERROR_CODE_MALFORMED_CONTENT" => {
                    CheckResponseErrorCodeItems::ErrorCodeMalformedContent
                }
                "ERROR_CODE_MALFORMED_HTTP_RESPONSE" => {
                    CheckResponseErrorCodeItems::ErrorCodeMalformedHttpResponse
                }
                "ERROR_CODE_REDIRECT" => CheckResponseErrorCodeItems::ErrorCodeRedirect,
                "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE" => {
                    CheckResponseErrorCodeItems::ErrorCodeSecureAssetIncludesInsecure
                }
                "ERROR_CODE_TOO_LARGE" => CheckResponseErrorCodeItems::ErrorCodeTooLarge,
                "ERROR_CODE_UNSPECIFIED" => CheckResponseErrorCodeItems::ErrorCodeUnspecified,
                "ERROR_CODE_WRONG_CONTENT_TYPE" => {
                    CheckResponseErrorCodeItems::ErrorCodeWrongContentType
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CheckResponseErrorCodeItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CheckResponseErrorCodeItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CheckResponseErrorCodeItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR_CODE_FAILED_SSL_VALIDATION" => {
                    CheckResponseErrorCodeItems::ErrorCodeFailedSslValidation
                }
                "ERROR_CODE_FETCH_BUDGET_EXHAUSTED" => {
                    CheckResponseErrorCodeItems::ErrorCodeFetchBudgetExhausted
                }
                "ERROR_CODE_FETCH_ERROR" => CheckResponseErrorCodeItems::ErrorCodeFetchError,
                "ERROR_CODE_INVALID_QUERY" => CheckResponseErrorCodeItems::ErrorCodeInvalidQuery,
                "ERROR_CODE_MALFORMED_CONTENT" => {
                    CheckResponseErrorCodeItems::ErrorCodeMalformedContent
                }
                "ERROR_CODE_MALFORMED_HTTP_RESPONSE" => {
                    CheckResponseErrorCodeItems::ErrorCodeMalformedHttpResponse
                }
                "ERROR_CODE_REDIRECT" => CheckResponseErrorCodeItems::ErrorCodeRedirect,
                "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE" => {
                    CheckResponseErrorCodeItems::ErrorCodeSecureAssetIncludesInsecure
                }
                "ERROR_CODE_TOO_LARGE" => CheckResponseErrorCodeItems::ErrorCodeTooLarge,
                "ERROR_CODE_UNSPECIFIED" => CheckResponseErrorCodeItems::ErrorCodeUnspecified,
                "ERROR_CODE_WRONG_CONTENT_TYPE" => {
                    CheckResponseErrorCodeItems::ErrorCodeWrongContentType
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CheckResponseErrorCodeItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckResponseErrorCodeItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListResponse {
        #[doc = "Human-readable message containing information intended to help end users\nunderstand, reproduce and debug the result.\n\nThe message will be in English and we are currently not planning to offer\nany translations.\n\nPlease note that no guarantees are made about the contents or format of\nthis string.  Any aspect of it may be subject to change without notice.\nYou should not attempt to programmatically parse this data.  For\nprogrammatic access, use the error_code field below."]
        #[serde(
            rename = "debugString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_string: ::std::option::Option<String>,
        #[doc = "Error codes that describe the result of the List operation."]
        #[serde(
            rename = "errorCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_code: ::std::option::Option<Vec<crate::schemas::ListResponseErrorCodeItems>>,
        #[doc = "From serving time, how much longer the response should be considered valid\nbarring further updates.\nREQUIRED"]
        #[serde(
            rename = "maxAge",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_age: ::std::option::Option<String>,
        #[doc = "A list of all the matching statements that have been found."]
        #[serde(
            rename = "statements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statements: ::std::option::Option<Vec<crate::schemas::Statement>>,
    }
    impl ::google_field_selector::FieldSelector for ListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ListResponseErrorCodeItems {
        ErrorCodeFailedSslValidation,
        ErrorCodeFetchBudgetExhausted,
        ErrorCodeFetchError,
        ErrorCodeInvalidQuery,
        ErrorCodeMalformedContent,
        ErrorCodeMalformedHttpResponse,
        ErrorCodeRedirect,
        ErrorCodeSecureAssetIncludesInsecure,
        ErrorCodeTooLarge,
        ErrorCodeUnspecified,
        ErrorCodeWrongContentType,
    }
    impl ListResponseErrorCodeItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ListResponseErrorCodeItems::ErrorCodeFailedSslValidation => {
                    "ERROR_CODE_FAILED_SSL_VALIDATION"
                }
                ListResponseErrorCodeItems::ErrorCodeFetchBudgetExhausted => {
                    "ERROR_CODE_FETCH_BUDGET_EXHAUSTED"
                }
                ListResponseErrorCodeItems::ErrorCodeFetchError => "ERROR_CODE_FETCH_ERROR",
                ListResponseErrorCodeItems::ErrorCodeInvalidQuery => "ERROR_CODE_INVALID_QUERY",
                ListResponseErrorCodeItems::ErrorCodeMalformedContent => {
                    "ERROR_CODE_MALFORMED_CONTENT"
                }
                ListResponseErrorCodeItems::ErrorCodeMalformedHttpResponse => {
                    "ERROR_CODE_MALFORMED_HTTP_RESPONSE"
                }
                ListResponseErrorCodeItems::ErrorCodeRedirect => "ERROR_CODE_REDIRECT",
                ListResponseErrorCodeItems::ErrorCodeSecureAssetIncludesInsecure => {
                    "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE"
                }
                ListResponseErrorCodeItems::ErrorCodeTooLarge => "ERROR_CODE_TOO_LARGE",
                ListResponseErrorCodeItems::ErrorCodeUnspecified => "ERROR_CODE_UNSPECIFIED",
                ListResponseErrorCodeItems::ErrorCodeWrongContentType => {
                    "ERROR_CODE_WRONG_CONTENT_TYPE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ListResponseErrorCodeItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ListResponseErrorCodeItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ListResponseErrorCodeItems, ()> {
            Ok(match s {
                "ERROR_CODE_FAILED_SSL_VALIDATION" => {
                    ListResponseErrorCodeItems::ErrorCodeFailedSslValidation
                }
                "ERROR_CODE_FETCH_BUDGET_EXHAUSTED" => {
                    ListResponseErrorCodeItems::ErrorCodeFetchBudgetExhausted
                }
                "ERROR_CODE_FETCH_ERROR" => ListResponseErrorCodeItems::ErrorCodeFetchError,
                "ERROR_CODE_INVALID_QUERY" => ListResponseErrorCodeItems::ErrorCodeInvalidQuery,
                "ERROR_CODE_MALFORMED_CONTENT" => {
                    ListResponseErrorCodeItems::ErrorCodeMalformedContent
                }
                "ERROR_CODE_MALFORMED_HTTP_RESPONSE" => {
                    ListResponseErrorCodeItems::ErrorCodeMalformedHttpResponse
                }
                "ERROR_CODE_REDIRECT" => ListResponseErrorCodeItems::ErrorCodeRedirect,
                "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE" => {
                    ListResponseErrorCodeItems::ErrorCodeSecureAssetIncludesInsecure
                }
                "ERROR_CODE_TOO_LARGE" => ListResponseErrorCodeItems::ErrorCodeTooLarge,
                "ERROR_CODE_UNSPECIFIED" => ListResponseErrorCodeItems::ErrorCodeUnspecified,
                "ERROR_CODE_WRONG_CONTENT_TYPE" => {
                    ListResponseErrorCodeItems::ErrorCodeWrongContentType
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ListResponseErrorCodeItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ListResponseErrorCodeItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ListResponseErrorCodeItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR_CODE_FAILED_SSL_VALIDATION" => {
                    ListResponseErrorCodeItems::ErrorCodeFailedSslValidation
                }
                "ERROR_CODE_FETCH_BUDGET_EXHAUSTED" => {
                    ListResponseErrorCodeItems::ErrorCodeFetchBudgetExhausted
                }
                "ERROR_CODE_FETCH_ERROR" => ListResponseErrorCodeItems::ErrorCodeFetchError,
                "ERROR_CODE_INVALID_QUERY" => ListResponseErrorCodeItems::ErrorCodeInvalidQuery,
                "ERROR_CODE_MALFORMED_CONTENT" => {
                    ListResponseErrorCodeItems::ErrorCodeMalformedContent
                }
                "ERROR_CODE_MALFORMED_HTTP_RESPONSE" => {
                    ListResponseErrorCodeItems::ErrorCodeMalformedHttpResponse
                }
                "ERROR_CODE_REDIRECT" => ListResponseErrorCodeItems::ErrorCodeRedirect,
                "ERROR_CODE_SECURE_ASSET_INCLUDES_INSECURE" => {
                    ListResponseErrorCodeItems::ErrorCodeSecureAssetIncludesInsecure
                }
                "ERROR_CODE_TOO_LARGE" => ListResponseErrorCodeItems::ErrorCodeTooLarge,
                "ERROR_CODE_UNSPECIFIED" => ListResponseErrorCodeItems::ErrorCodeUnspecified,
                "ERROR_CODE_WRONG_CONTENT_TYPE" => {
                    ListResponseErrorCodeItems::ErrorCodeWrongContentType
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ListResponseErrorCodeItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListResponseErrorCodeItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Statement {
        #[doc = "The relation identifies the use of the statement as intended by the source\nasset's owner (that is, the person or entity who issued the statement).\nEvery complete statement has a relation.\n\nWe identify relations with strings of the format `<kind>/<detail>`, where\n`<kind>` must be one of a set of pre-defined purpose categories, and\n`<detail>` is a free-form lowercase alphanumeric string that describes the\nspecific use case of the statement.\n\nRefer to [our API documentation](/digital-asset-links/v1/relation-strings)\nfor the current list of supported relations.\n\nExample: `delegate_permission/common.handle_all_urls`\nREQUIRED"]
        #[serde(
            rename = "relation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relation: ::std::option::Option<String>,
        #[doc = "Every statement has a source asset.\nREQUIRED"]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Asset>,
        #[doc = "Every statement has a target asset.\nREQUIRED"]
        #[serde(
            rename = "target",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target: ::std::option::Option<crate::schemas::Asset>,
    }
    impl ::google_field_selector::FieldSelector for Statement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Statement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WebAsset {
        #[doc = "Web assets are identified by a URL that contains only the scheme, hostname\nand port parts.  The format is\n\n````text\nhttp[s]://<hostname>[:<port>]\n````\n\nHostnames must be fully qualified: they must end in a single period\n(\"`.`\").\n\nOnly the schemes \"http\" and \"https\" are currently allowed.\n\nPort numbers are given as a decimal number, and they must be omitted if the\nstandard port numbers are used: 80 for http and 443 for https.\n\nWe call this limited URL the \"site\".  All URLs that share the same scheme,\nhostname and port are considered to be a part of the site and thus belong\nto the web asset.\n\nExample: the asset with the site `https://www.google.com` contains all\nthese URLs:\n\n* `https://www.google.com/`\n* `https://www.google.com:443/`\n* `https://www.google.com/foo`\n* `https://www.google.com/foo?bar`\n* `https://www.google.com/foo#bar`\n* `https://user@password:www.google.com/`\n\nBut it does not contain these URLs:\n\n* `http://www.google.com/`       (wrong scheme)\n* `https://google.com/`          (hostname does not match)\n* `https://www.google.com:444/`  (port does not match)\n  REQUIRED"]
        #[serde(
            rename = "site",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WebAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebAsset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest,
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the assetlinks resource"]
    pub fn assetlinks(&self) -> crate::resources::assetlinks::AssetlinksActions {
        crate::resources::assetlinks::AssetlinksActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the statements resource"]
    pub fn statements(&self) -> crate::resources::statements::StatementsActions {
        crate::resources::statements::StatementsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod assetlinks {
        pub mod params {}
        pub struct AssetlinksActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AssetlinksActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Determines whether the specified (directional) relationship exists between\nthe specified source and target assets.\n\nThe relation describes the intent of the link between the two assets as\nclaimed by the source asset.  An example for such relationships is the\ndelegation of privileges or permissions.\n\nThis command is most often used by infrastructure systems to check\npreconditions for an action.  For example, a client may want to know if it\nis OK to send a web URL to a particular mobile app instead. The client can\ncheck for the relevant asset link from the website to the mobile app to\ndecide if the operation should be allowed.\n\nA note about security: if you specify a secure asset as the source, such as\nan HTTPS website or an Android app, the API will ensure that any\nstatements used to generate the response have been made in a secure way by\nthe owner of that asset.  Conversely, if the source asset is an insecure\nHTTP website (that is, the URL starts with `http://` instead of\n`https://`), the API cannot verify its statements securely, and it is not\npossible to ensure that the website's statements have not been altered by a\nthird party.  For more information, see the [Digital Asset Links technical\ndesign\nspecification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md)."]
            pub fn check(&self) -> CheckRequestBuilder {
                CheckRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    relation: None,
                    source_android_app_certificate_sha_256_fingerprint: None,
                    source_android_app_package_name: None,
                    source_web_site: None,
                    target_android_app_certificate_sha_256_fingerprint: None,
                    target_android_app_package_name: None,
                    target_web_site: None,
                }
            }
        }
        #[doc = "Created via [AssetlinksActions::check()](struct.AssetlinksActions.html#method.check)"]
        #[derive(Debug, Clone)]
        pub struct CheckRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            relation: Option<String>,
            source_android_app_certificate_sha_256_fingerprint: Option<String>,
            source_android_app_package_name: Option<String>,
            source_web_site: Option<String>,
            target_android_app_certificate_sha_256_fingerprint: Option<String>,
            target_android_app_package_name: Option<String>,
            target_web_site: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> CheckRequestBuilder<'a> {
            #[doc = "Query string for the relation.\n\nWe identify relations with strings of the format `<kind>/<detail>`, where\n`<kind>` must be one of a set of pre-defined purpose categories, and\n`<detail>` is a free-form lowercase alphanumeric string that describes the\nspecific use case of the statement.\n\nRefer to [our API documentation](/digital-asset-links/v1/relation-strings)\nfor the current list of supported relations.\n\nFor a query to match an asset link, both the query's and the asset link's\nrelation strings must match exactly.\n\nExample: A query with relation `delegate_permission/common.handle_all_urls`\nmatches an asset link with relation\n`delegate_permission/common.handle_all_urls`."]
            pub fn relation(mut self, value: impl Into<String>) -> Self {
                self.relation = Some(value.into());
                self
            }
            #[doc = "The uppercase SHA-265 fingerprint of the certificate.  From the PEM\ncertificate, it can be acquired like this:\n\n````text\n$ keytool -printcert -file $CERTFILE | grep SHA256:\nSHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\\n    42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nor like this:\n\n````text\n$ openssl x509 -in $CERTFILE -noout -fingerprint -sha256\nSHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\\n    16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nIn this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.\n\nIf these tools are not available to you, you can convert the PEM\ncertificate into the DER format, compute the SHA-256 hash of that string\nand represent the result as a hexstring (that is, uppercase hexadecimal\nrepresentations of each octet, separated by colons)."]
            pub fn source_android_app_certificate_sha_256_fingerprint(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.source_android_app_certificate_sha_256_fingerprint = Some(value.into());
                self
            }
            #[doc = "Android App assets are naturally identified by their Java package name.\nFor example, the Google Maps app uses the package name\n`com.google.android.apps.maps`.\nREQUIRED"]
            pub fn source_android_app_package_name(mut self, value: impl Into<String>) -> Self {
                self.source_android_app_package_name = Some(value.into());
                self
            }
            #[doc = "Web assets are identified by a URL that contains only the scheme, hostname\nand port parts.  The format is\n\n````text\nhttp[s]://<hostname>[:<port>]\n````\n\nHostnames must be fully qualified: they must end in a single period\n(\"`.`\").\n\nOnly the schemes \"http\" and \"https\" are currently allowed.\n\nPort numbers are given as a decimal number, and they must be omitted if the\nstandard port numbers are used: 80 for http and 443 for https.\n\nWe call this limited URL the \"site\".  All URLs that share the same scheme,\nhostname and port are considered to be a part of the site and thus belong\nto the web asset.\n\nExample: the asset with the site `https://www.google.com` contains all\nthese URLs:\n\n* `https://www.google.com/`\n* `https://www.google.com:443/`\n* `https://www.google.com/foo`\n* `https://www.google.com/foo?bar`\n* `https://www.google.com/foo#bar`\n* `https://user@password:www.google.com/`\n\nBut it does not contain these URLs:\n\n* `http://www.google.com/`       (wrong scheme)\n* `https://google.com/`          (hostname does not match)\n* `https://www.google.com:444/`  (port does not match)\n  REQUIRED"]
            pub fn source_web_site(mut self, value: impl Into<String>) -> Self {
                self.source_web_site = Some(value.into());
                self
            }
            #[doc = "The uppercase SHA-265 fingerprint of the certificate.  From the PEM\ncertificate, it can be acquired like this:\n\n````text\n$ keytool -printcert -file $CERTFILE | grep SHA256:\nSHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\\n    42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nor like this:\n\n````text\n$ openssl x509 -in $CERTFILE -noout -fingerprint -sha256\nSHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\\n    16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nIn this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.\n\nIf these tools are not available to you, you can convert the PEM\ncertificate into the DER format, compute the SHA-256 hash of that string\nand represent the result as a hexstring (that is, uppercase hexadecimal\nrepresentations of each octet, separated by colons)."]
            pub fn target_android_app_certificate_sha_256_fingerprint(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.target_android_app_certificate_sha_256_fingerprint = Some(value.into());
                self
            }
            #[doc = "Android App assets are naturally identified by their Java package name.\nFor example, the Google Maps app uses the package name\n`com.google.android.apps.maps`.\nREQUIRED"]
            pub fn target_android_app_package_name(mut self, value: impl Into<String>) -> Self {
                self.target_android_app_package_name = Some(value.into());
                self
            }
            #[doc = "Web assets are identified by a URL that contains only the scheme, hostname\nand port parts.  The format is\n\n````text\nhttp[s]://<hostname>[:<port>]\n````\n\nHostnames must be fully qualified: they must end in a single period\n(\"`.`\").\n\nOnly the schemes \"http\" and \"https\" are currently allowed.\n\nPort numbers are given as a decimal number, and they must be omitted if the\nstandard port numbers are used: 80 for http and 443 for https.\n\nWe call this limited URL the \"site\".  All URLs that share the same scheme,\nhostname and port are considered to be a part of the site and thus belong\nto the web asset.\n\nExample: the asset with the site `https://www.google.com` contains all\nthese URLs:\n\n* `https://www.google.com/`\n* `https://www.google.com:443/`\n* `https://www.google.com/foo`\n* `https://www.google.com/foo?bar`\n* `https://www.google.com/foo#bar`\n* `https://user@password:www.google.com/`\n\nBut it does not contain these URLs:\n\n* `http://www.google.com/`       (wrong scheme)\n* `https://google.com/`          (hostname does not match)\n* `https://www.google.com:444/`  (port does not match)\n  REQUIRED"]
            pub fn target_web_site(mut self, value: impl Into<String>) -> Self {
                self.target_web_site = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::CheckResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CheckResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://digitalassetlinks.googleapis.com/".to_owned();
                output.push_str("v1/assetlinks:check");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("relation", &self.relation)]);
                let req = req.query(&[(
                    "source.androidApp.certificate.sha256Fingerprint",
                    &self.source_android_app_certificate_sha_256_fingerprint,
                )]);
                let req = req.query(&[(
                    "source.androidApp.packageName",
                    &self.source_android_app_package_name,
                )]);
                let req = req.query(&[("source.web.site", &self.source_web_site)]);
                let req = req.query(&[(
                    "target.androidApp.certificate.sha256Fingerprint",
                    &self.target_android_app_certificate_sha_256_fingerprint,
                )]);
                let req = req.query(&[(
                    "target.androidApp.packageName",
                    &self.target_android_app_package_name,
                )]);
                let req = req.query(&[("target.web.site", &self.target_web_site)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod statements {
        pub mod params {}
        pub struct StatementsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> StatementsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves a list of all statements from a given source that match the\nspecified target and statement string.\n\nThe API guarantees that all statements with secure source assets, such as\nHTTPS websites or Android apps, have been made in a secure way by the owner\nof those assets, as described in the [Digital Asset Links technical design\nspecification](https://github.com/google/digitalassetlinks/blob/master/well-known/details.md).\nSpecifically, you should consider that for insecure websites (that is,\nwhere the URL starts with `http://` instead of `https://`), this guarantee\ncannot be made.\n\nThe `List` command is most useful in cases where the API client wants to\nknow all the ways in which two assets are related, or enumerate all the\nrelationships from a particular source asset.  Example: a feature that\nhelps users navigate to related items.  When a mobile app is running on a\ndevice, the feature would make it easy to navigate to the corresponding web\nsite or Google+ profile."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    relation: None,
                    source_android_app_certificate_sha_256_fingerprint: None,
                    source_android_app_package_name: None,
                    source_web_site: None,
                }
            }
        }
        #[doc = "Created via [StatementsActions::list()](struct.StatementsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            relation: Option<String>,
            source_android_app_certificate_sha_256_fingerprint: Option<String>,
            source_android_app_package_name: Option<String>,
            source_web_site: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Use only associations that match the specified relation.\n\nSee the [`Statement`](#Statement) message for a detailed definition of\nrelation strings.\n\nFor a query to match a statement, one of the following must be true:\n\n* both the query's and the statement's relation strings match exactly,\n  or\n* the query's relation string is empty or missing.\n\nExample: A query with relation `delegate_permission/common.handle_all_urls`\nmatches an asset link with relation\n`delegate_permission/common.handle_all_urls`."]
            pub fn relation(mut self, value: impl Into<String>) -> Self {
                self.relation = Some(value.into());
                self
            }
            #[doc = "The uppercase SHA-265 fingerprint of the certificate.  From the PEM\ncertificate, it can be acquired like this:\n\n````text\n$ keytool -printcert -file $CERTFILE | grep SHA256:\nSHA256: 14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83: \\\n    42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nor like this:\n\n````text\n$ openssl x509 -in $CERTFILE -noout -fingerprint -sha256\nSHA256 Fingerprint=14:6D:E9:83:C5:73:06:50:D8:EE:B9:95:2F:34:FC:64: \\\n    16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF:44:E5\n````\n\nIn this example, the contents of this field would be `14:6D:E9:83:C5:73: 06:50:D8:EE:B9:95:2F:34:FC:64:16:A0:83:42:E6:1D:BE:A8:8A:04:96:B2:3F:CF: 44:E5`.\n\nIf these tools are not available to you, you can convert the PEM\ncertificate into the DER format, compute the SHA-256 hash of that string\nand represent the result as a hexstring (that is, uppercase hexadecimal\nrepresentations of each octet, separated by colons)."]
            pub fn source_android_app_certificate_sha_256_fingerprint(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.source_android_app_certificate_sha_256_fingerprint = Some(value.into());
                self
            }
            #[doc = "Android App assets are naturally identified by their Java package name.\nFor example, the Google Maps app uses the package name\n`com.google.android.apps.maps`.\nREQUIRED"]
            pub fn source_android_app_package_name(mut self, value: impl Into<String>) -> Self {
                self.source_android_app_package_name = Some(value.into());
                self
            }
            #[doc = "Web assets are identified by a URL that contains only the scheme, hostname\nand port parts.  The format is\n\n````text\nhttp[s]://<hostname>[:<port>]\n````\n\nHostnames must be fully qualified: they must end in a single period\n(\"`.`\").\n\nOnly the schemes \"http\" and \"https\" are currently allowed.\n\nPort numbers are given as a decimal number, and they must be omitted if the\nstandard port numbers are used: 80 for http and 443 for https.\n\nWe call this limited URL the \"site\".  All URLs that share the same scheme,\nhostname and port are considered to be a part of the site and thus belong\nto the web asset.\n\nExample: the asset with the site `https://www.google.com` contains all\nthese URLs:\n\n* `https://www.google.com/`\n* `https://www.google.com:443/`\n* `https://www.google.com/foo`\n* `https://www.google.com/foo?bar`\n* `https://www.google.com/foo#bar`\n* `https://user@password:www.google.com/`\n\nBut it does not contain these URLs:\n\n* `http://www.google.com/`       (wrong scheme)\n* `https://google.com/`          (hostname does not match)\n* `https://www.google.com:444/`  (port does not match)\n  REQUIRED"]
            pub fn source_web_site(mut self, value: impl Into<String>) -> Self {
                self.source_web_site = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://digitalassetlinks.googleapis.com/".to_owned();
                output.push_str("v1/statements:list");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("relation", &self.relation)]);
                let req = req.query(&[(
                    "source.androidApp.certificate.sha256Fingerprint",
                    &self.source_android_app_certificate_sha_256_fingerprint,
                )]);
                let req = req.query(&[(
                    "source.androidApp.packageName",
                    &self.source_android_app_package_name,
                )]);
                let req = req.query(&[("source.web.site", &self.source_web_site)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
