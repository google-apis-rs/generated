pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Api {
        #[doc = "The methods of this interface, in unspecified order."]
        #[serde(rename = "methods", default)]
        pub methods: ::std::option::Option<Vec<crate::schemas::Method>>,
        #[doc = "Included interfaces. See Mixin."]
        #[serde(rename = "mixins", default)]
        pub mixins: ::std::option::Option<Vec<crate::schemas::Mixin>>,
        #[doc = "The fully qualified name of this interface, including package name\nfollowed by the interface's simple name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Any metadata attached to the interface."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "Source context for the protocol buffer service represented by this\nmessage."]
        #[serde(rename = "sourceContext", default)]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax of the service."]
        #[serde(rename = "syntax", default)]
        pub syntax: ::std::option::Option<crate::schemas::ApiSyntax>,
        #[doc = "A version string for this interface. If specified, must have the form\n`major-version.minor-version`, as in `1.10`. If the minor version is\nomitted, it defaults to zero. If the entire version field is empty, the\nmajor version is derived from the package name, as outlined below. If the\nfield is not empty, the version in the package name will be verified to be\nconsistent with what is provided here.\n\nThe versioning schema uses [semantic\nversioning](http://semver.org) where the major version number\nindicates a breaking change and the minor version an additive,\nnon-breaking change. Both version numbers are signals to users\nwhat to expect from different versions, and should be carefully\nchosen based on the product plan.\n\nThe major version is also reflected in the package name of the\ninterface, which must end in `v<major-version>`, as in\n`google.feature.v1`. For major versions 0 and 1, the suffix can\nbe omitted. Zero major versions must only be used for\nexperimental, non-GA interfaces."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Api {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApiSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl ApiSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                ApiSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                ApiSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::fmt::Display for ApiSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApiSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApiSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => ApiSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => ApiSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ApiSyntax {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct AuthProvider {
        #[doc = "The list of JWT\n[audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3).\nthat are allowed to access. A JWT containing any of these audiences will\nbe accepted. When this setting is absent, only JWTs with audience\n\"https://Service_name/API_name\"\nwill be accepted. For example, if no audiences are in the setting,\nLibraryService API will only accept JWTs with the following audience\n\"https://library-example.googleapis.com/google.example.library.v1.LibraryService\".\n\nExample:\n\n````text\naudiences: bookstore_android.apps.googleusercontent.com,\n           bookstore_web.apps.googleusercontent.com````"]
        #[serde(rename = "audiences", default)]
        pub audiences: ::std::option::Option<String>,
        #[doc = "Redirect URL if JWT token is required but not present or is expired.\nImplement authorizationUrl of securityDefinitions in OpenAPI spec."]
        #[serde(rename = "authorizationUrl", default)]
        pub authorization_url: ::std::option::Option<String>,
        #[doc = "The unique identifier of the auth provider. It will be referred to by\n`AuthRequirement.provider_id`.\n\nExample: \"bookstore_auth\"."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies the principal that issued the JWT. See\nhttps://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.1\nUsually a URL or an email address.\n\nExample: https://securetoken.google.com\nExample: 1234567-compute@developer.gserviceaccount.com"]
        #[serde(rename = "issuer", default)]
        pub issuer: ::std::option::Option<String>,
        #[doc = "URL of the provider's public key set to validate signature of the JWT. See\n[OpenID\nDiscovery](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata).\nOptional if the key set document:\n\n* can be retrieved from\n  [OpenID\n  Discovery](https://openid.net/specs/openid-connect-discovery-1_0.html of\n  the issuer.\n* can be inferred from the email domain of the issuer (e.g. a Google\n  service account).\n\nExample: https://www.googleapis.com/oauth2/v1/certs"]
        #[serde(rename = "jwksUri", default)]
        pub jwks_uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AuthProvider {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct AuthRequirement {
        #[doc = "NOTE: This will be deprecated soon, once AuthProvider.audiences is\nimplemented and accepted in all the runtime components.\n\nThe list of JWT\n[audiences](https://tools.ietf.org/html/draft-ietf-oauth-json-web-token-32#section-4.1.3).\nthat are allowed to access. A JWT containing any of these audiences will\nbe accepted. When this setting is absent, only JWTs with audience\n\"https://Service_name/API_name\"\nwill be accepted. For example, if no audiences are in the setting,\nLibraryService API will only accept JWTs with the following audience\n\"https://library-example.googleapis.com/google.example.library.v1.LibraryService\".\n\nExample:\n\n````text\naudiences: bookstore_android.apps.googleusercontent.com,\n           bookstore_web.apps.googleusercontent.com````"]
        #[serde(rename = "audiences", default)]
        pub audiences: ::std::option::Option<String>,
        #[doc = "id from authentication provider.\n\nExample:\n\n````text\nprovider_id: bookstore_auth````"]
        #[serde(rename = "providerId", default)]
        pub provider_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AuthRequirement {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Authentication {
        #[doc = "Defines a set of authentication providers that a service supports."]
        #[serde(rename = "providers", default)]
        pub providers: ::std::option::Option<Vec<crate::schemas::AuthProvider>>,
        #[doc = "A list of authentication rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::AuthenticationRule>>,
    }
    impl ::field_selector::FieldSelector for Authentication {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct AuthenticationRule {
        #[doc = "If true, the service accepts API keys without any other credential."]
        #[serde(rename = "allowWithoutCredential", default)]
        pub allow_without_credential: ::std::option::Option<bool>,
        #[doc = "The requirements for OAuth credentials."]
        #[serde(rename = "oauth", default)]
        pub oauth: ::std::option::Option<crate::schemas::OauthRequirements>,
        #[doc = "Requirements for additional authentication providers."]
        #[serde(rename = "requirements", default)]
        pub requirements: ::std::option::Option<Vec<crate::schemas::AuthRequirement>>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AuthenticationRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Backend {
        #[doc = "A list of API backend rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::BackendRule>>,
    }
    impl ::field_selector::FieldSelector for Backend {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BackendRule {
        #[doc = "The address of the API backend."]
        #[serde(rename = "address", default)]
        pub address: ::std::option::Option<String>,
        #[doc = "The number of seconds to wait for a response from a request.  The default\ndeadline for gRPC is infinite (no deadline) and HTTP requests is 5 seconds."]
        #[serde(rename = "deadline", default)]
        pub deadline: ::std::option::Option<f64>,
        #[doc = "The JWT audience is used when generating a JWT id token for the backend."]
        #[serde(rename = "jwtAudience", default)]
        pub jwt_audience: ::std::option::Option<String>,
        #[doc = "Minimum deadline in seconds needed for this method. Calls having deadline\nvalue lower than this will be rejected."]
        #[serde(rename = "minDeadline", default)]
        pub min_deadline: ::std::option::Option<f64>,
        #[doc = "The number of seconds to wait for the completion of a long running\noperation. The default is no deadline."]
        #[serde(rename = "operationDeadline", default)]
        pub operation_deadline: ::std::option::Option<f64>,
        #[serde(rename = "pathTranslation", default)]
        pub path_translation: ::std::option::Option<crate::schemas::BackendRulePathTranslation>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BackendRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BackendRulePathTranslation {
        #[doc = "The request path will be appended to the backend address.\n\n# Examples\n\nGiven the following operation config:\n\n````text\nMethod path:        /api/company/{cid}/user/{uid}\nBackend address:    https://example.appspot.com\n````\n\nRequests to the following request paths will call the backend at the\ntranslated path:\n\n````text\nRequest path: /api/company/widgetworks/user/johndoe\nTranslated:\nhttps://example.appspot.com/api/company/widgetworks/user/johndoe\n\nRequest path: /api/company/widgetworks/user/johndoe?timezone=EST\nTranslated:\nhttps://example.appspot.com/api/company/widgetworks/user/johndoe?timezone=EST````"]
        AppendPathToAddress,
        #[doc = "Use the backend address as-is, with no modification to the path. If the\nURL pattern contains variables, the variable names and values will be\nappended to the query string. If a query string parameter and a URL\npattern variable have the same name, this may result in duplicate keys in\nthe query string.\n\n# Examples\n\nGiven the following operation config:\n\n````text\nMethod path:        /api/company/{cid}/user/{uid}\nBackend address:    https://example.cloudfunctions.net/getUser\n````\n\nRequests to the following request paths will call the backend at the\ntranslated path:\n\n````text\nRequest path: /api/company/widgetworks/user/johndoe\nTranslated:\nhttps://example.cloudfunctions.net/getUser?cid=widgetworks&uid=johndoe\n\nRequest path: /api/company/widgetworks/user/johndoe?timezone=EST\nTranslated:\nhttps://example.cloudfunctions.net/getUser?timezone=EST&cid=widgetworks&uid=johndoe````"]
        ConstantAddress,
        PathTranslationUnspecified,
    }
    impl BackendRulePathTranslation {
        pub fn as_str(self) -> &'static str {
            match self {
                BackendRulePathTranslation::AppendPathToAddress => "APPEND_PATH_TO_ADDRESS",
                BackendRulePathTranslation::ConstantAddress => "CONSTANT_ADDRESS",
                BackendRulePathTranslation::PathTranslationUnspecified => {
                    "PATH_TRANSLATION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for BackendRulePathTranslation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BackendRulePathTranslation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BackendRulePathTranslation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPEND_PATH_TO_ADDRESS" => BackendRulePathTranslation::AppendPathToAddress,
                "CONSTANT_ADDRESS" => BackendRulePathTranslation::ConstantAddress,
                "PATH_TRANSLATION_UNSPECIFIED" => {
                    BackendRulePathTranslation::PathTranslationUnspecified
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
    impl ::field_selector::FieldSelector for BackendRulePathTranslation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct BatchCreateAdminOverridesResponse {
        #[doc = "The overrides that were created."]
        #[serde(rename = "overrides", default)]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::field_selector::FieldSelector for BatchCreateAdminOverridesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct BatchCreateConsumerOverridesResponse {
        #[doc = "The overrides that were created."]
        #[serde(rename = "overrides", default)]
        pub overrides: ::std::option::Option<Vec<crate::schemas::QuotaOverride>>,
    }
    impl ::field_selector::FieldSelector for BatchCreateConsumerOverridesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct BatchEnableServicesRequest {
        #[doc = "The identifiers of the services to enable on the project.\n\nA valid identifier would be:\nserviceusage.googleapis.com\n\nEnabling services requires that each service is public or is shared with\nthe user enabling the service.\n\nA single request can enable a maximum of 20 services at a time. If more\nthan 20 services are specified, the request will fail, and no state changes\nwill occur."]
        #[serde(rename = "serviceIds", default)]
        pub service_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for BatchEnableServicesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchEnableServicesResponse {
        #[doc = "If allow_partial_success is true, and one or more services could not be\nenabled, this field contains the details about each failure."]
        #[serde(rename = "failures", default)]
        pub failures: ::std::option::Option<Vec<crate::schemas::EnableFailure>>,
        #[doc = "The new state of the services after enabling."]
        #[serde(rename = "services", default)]
        pub services: ::std::option::Option<Vec<crate::schemas::GoogleApiServiceusageV1Service>>,
    }
    impl ::field_selector::FieldSelector for BatchEnableServicesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Billing {
        #[doc = "Billing configurations for sending metrics to the consumer project.\nThere can be multiple consumer destinations per service, each one must have\na different monitored resource type. A metric can be used in at most\none consumer destination."]
        #[serde(rename = "consumerDestinations", default)]
        pub consumer_destinations: ::std::option::Option<Vec<crate::schemas::BillingDestination>>,
    }
    impl ::field_selector::FieldSelector for Billing {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct BillingDestination {
        #[doc = "Names of the metrics to report to this billing destination.\nEach name must be defined in Service.metrics section."]
        #[serde(rename = "metrics", default)]
        pub metrics: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in\nService.monitored_resources section."]
        #[serde(rename = "monitoredResource", default)]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BillingDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CancelOperationRequest;
    impl ::field_selector::FieldSelector for CancelOperationRequest {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct Context {
        #[doc = "A list of RPC context rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::ContextRule>>,
    }
    impl ::field_selector::FieldSelector for Context {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct ContextRule {
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc\nside channel from client to backend."]
        #[serde(rename = "allowedRequestExtensions", default)]
        pub allowed_request_extensions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names or extension IDs of extensions allowed in grpc\nside channel from backend to client."]
        #[serde(rename = "allowedResponseExtensions", default)]
        pub allowed_response_extensions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names of provided contexts."]
        #[serde(rename = "provided", default)]
        pub provided: ::std::option::Option<Vec<String>>,
        #[doc = "A list of full type names of requested contexts."]
        #[serde(rename = "requested", default)]
        pub requested: ::std::option::Option<Vec<String>>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ContextRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Control {
        #[doc = "The service control environment to use. If empty, no control plane\nfeature (like quota and billing) will be enabled."]
        #[serde(rename = "environment", default)]
        pub environment: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Control {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct CustomError {
        #[doc = "The list of custom error rules that apply to individual API messages.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::CustomErrorRule>>,
        #[doc = "The list of custom error detail types, e.g. 'google.foo.v1.CustomError'."]
        #[serde(rename = "types", default)]
        pub types: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for CustomError {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct CustomErrorRule {
        #[doc = "Mark this message as possible payload in error response.  Otherwise,\nobjects of this type will be filtered when they appear in error payload."]
        #[serde(rename = "isErrorType", default)]
        pub is_error_type: ::std::option::Option<bool>,
        #[doc = "Selects messages to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomErrorRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct CustomHttpPattern {
        #[doc = "The name of this custom HTTP verb."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The path matched by this custom verb."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomHttpPattern {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct DisableServiceRequest {
        #[doc = "Indicates if services that are enabled and which depend on this service\nshould also be disabled. If not set, an error will be generated if any\nenabled services depend on the service to be disabled. When set, the\nservice, and any enabled services that depend on it, will be disabled\ntogether."]
        #[serde(rename = "disableDependentServices", default)]
        pub disable_dependent_services: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for DisableServiceRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DisableServiceResponse {
        #[doc = "The new state of the service after disabling."]
        #[serde(rename = "service", default)]
        pub service: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1Service>,
    }
    impl ::field_selector::FieldSelector for DisableServiceResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Documentation {
        #[doc = "The URL to the root of documentation."]
        #[serde(rename = "documentationRootUrl", default)]
        pub documentation_root_url: ::std::option::Option<String>,
        #[doc = "Declares a single overview page. For example:\n\n<pre><code>documentation:\n  summary: ...\n  overview: &#40;== include overview.md ==&#41;\n</code></pre>\n\nThis is a shortcut for the following declaration (using pages style):\n\n<pre><code>documentation:\n  summary: ...\n  pages:\n  - name: Overview\n    content: &#40;== include overview.md ==&#41;\n</code></pre>\n\nNote: you cannot specify both `overview` field and `pages` field."]
        #[serde(rename = "overview", default)]
        pub overview: ::std::option::Option<String>,
        #[doc = "The top level pages for the documentation set."]
        #[serde(rename = "pages", default)]
        pub pages: ::std::option::Option<Vec<crate::schemas::Page>>,
        #[doc = "A list of documentation rules that apply to individual API elements.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::DocumentationRule>>,
        #[doc = "Specifies the service root url if the default one (the service name\nfrom the yaml file) is not suitable. This can be seen in any fully\nspecified service urls as well as sections that show a base that other\nurls are relative to."]
        #[serde(rename = "serviceRootUrl", default)]
        pub service_root_url: ::std::option::Option<String>,
        #[doc = "A short summary of what the service does. Can only be provided by\nplain text."]
        #[serde(rename = "summary", default)]
        pub summary: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Documentation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct DocumentationRule {
        #[doc = "Deprecation description of the selected element(s). It can be provided if\nan element is marked as `deprecated`."]
        #[serde(rename = "deprecationDescription", default)]
        pub deprecation_description: ::std::option::Option<String>,
        #[doc = "Description of the selected API(s)."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The selector is a comma-separated list of patterns. Each pattern is a\nqualified name of the element which may end in \"*\", indicating a wildcard.\nWildcards are only allowed at the end and for a whole component of the\nqualified name, i.e. \"foo.*\" is ok, but not \"foo.b*\" or \"foo.*.bar\". A\nwildcard will match one or more components. To specify a default for all\napplicable elements, the whole pattern \"*\" is used."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DocumentationRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Empty;
    impl ::field_selector::FieldSelector for Empty {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct EnableFailure {
        #[doc = "An error message describing why the service could not be enabled."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[doc = "The service id of a service that could not be enabled."]
        #[serde(rename = "serviceId", default)]
        pub service_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for EnableFailure {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EnableServiceRequest;
    impl ::field_selector::FieldSelector for EnableServiceRequest {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct EnableServiceResponse {
        #[doc = "The new state of the service after enabling."]
        #[serde(rename = "service", default)]
        pub service: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1Service>,
    }
    impl ::field_selector::FieldSelector for EnableServiceResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Endpoint {
        #[doc = "DEPRECATED: This field is no longer supported. Instead of using aliases,\nplease specify multiple google.api.Endpoint for each of the intended\naliases.\n\nAdditional names that this endpoint will be hosted on."]
        #[serde(rename = "aliases", default)]
        pub aliases: ::std::option::Option<Vec<String>>,
        #[doc = "Allowing\n[CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_sharing), aka\ncross-domain traffic, would allow the backends served from this endpoint to\nreceive and respond to HTTP OPTIONS requests. The response will be used by\nthe browser to determine whether the subsequent cross-origin request is\nallowed to proceed."]
        #[serde(rename = "allowCors", default)]
        pub allow_cors: ::std::option::Option<bool>,
        #[doc = "The list of features enabled on this endpoint."]
        #[serde(rename = "features", default)]
        pub features: ::std::option::Option<Vec<String>>,
        #[doc = "The canonical name of this endpoint."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The specification of an Internet routable address of API frontend that will\nhandle requests to this [API\nEndpoint](https://cloud.google.com/apis/design/glossary). It should be\neither a valid IPv4 address or a fully-qualified domain name. For example,\n\"8.8.8.8\" or \"myservice.appspot.com\"."]
        #[serde(rename = "target", default)]
        pub target: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Endpoint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Enum {
        #[doc = "Enum value definitions."]
        #[serde(rename = "enumvalue", default)]
        pub enumvalue: ::std::option::Option<Vec<crate::schemas::EnumValue>>,
        #[doc = "Enum type name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Protocol buffer options."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "The source context."]
        #[serde(rename = "sourceContext", default)]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax."]
        #[serde(rename = "syntax", default)]
        pub syntax: ::std::option::Option<crate::schemas::EnumSyntax>,
    }
    impl ::field_selector::FieldSelector for Enum {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnumSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl EnumSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                EnumSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                EnumSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::fmt::Display for EnumSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnumSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnumSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => EnumSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => EnumSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for EnumSyntax {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct EnumValue {
        #[doc = "Enum value name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Enum value number."]
        #[serde(rename = "number", default)]
        pub number: ::std::option::Option<i32>,
        #[doc = "Protocol buffer options."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
    }
    impl ::field_selector::FieldSelector for EnumValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Field {
        #[doc = "The field cardinality."]
        #[serde(rename = "cardinality", default)]
        pub cardinality: ::std::option::Option<crate::schemas::FieldCardinality>,
        #[doc = "The string value of the default value of this field. Proto2 syntax only."]
        #[serde(rename = "defaultValue", default)]
        pub default_value: ::std::option::Option<String>,
        #[doc = "The field JSON name."]
        #[serde(rename = "jsonName", default)]
        pub json_name: ::std::option::Option<String>,
        #[doc = "The field type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<crate::schemas::FieldKind>,
        #[doc = "The field name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The field number."]
        #[serde(rename = "number", default)]
        pub number: ::std::option::Option<i32>,
        #[doc = "The index of the field type in `Type.oneofs`, for message or enumeration\ntypes. The first type has index 1; zero means the type is not in the list."]
        #[serde(rename = "oneofIndex", default)]
        pub oneof_index: ::std::option::Option<i32>,
        #[doc = "The protocol buffer options."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "Whether to use alternative packed wire representation."]
        #[serde(rename = "packed", default)]
        pub packed: ::std::option::Option<bool>,
        #[doc = "The field type URL, without the scheme, for message or enumeration\ntypes. Example: `\"type.googleapis.com/google.protobuf.Timestamp\"`."]
        #[serde(rename = "typeUrl", default)]
        pub type_url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Field {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldCardinality {
        #[doc = "For optional fields."]
        CardinalityOptional,
        #[doc = "For repeated fields."]
        CardinalityRepeated,
        #[doc = "For required fields. Proto2 syntax only."]
        CardinalityRequired,
        #[doc = "For fields with unknown cardinality."]
        CardinalityUnknown,
    }
    impl FieldCardinality {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldCardinality::CardinalityOptional => "CARDINALITY_OPTIONAL",
                FieldCardinality::CardinalityRepeated => "CARDINALITY_REPEATED",
                FieldCardinality::CardinalityRequired => "CARDINALITY_REQUIRED",
                FieldCardinality::CardinalityUnknown => "CARDINALITY_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for FieldCardinality {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldCardinality {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldCardinality {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CARDINALITY_OPTIONAL" => FieldCardinality::CardinalityOptional,
                "CARDINALITY_REPEATED" => FieldCardinality::CardinalityRepeated,
                "CARDINALITY_REQUIRED" => FieldCardinality::CardinalityRequired,
                "CARDINALITY_UNKNOWN" => FieldCardinality::CardinalityUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FieldCardinality {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FieldKind {
        #[doc = "Field type bool."]
        TypeBool,
        #[doc = "Field type bytes."]
        TypeBytes,
        #[doc = "Field type double."]
        TypeDouble,
        #[doc = "Field type enum."]
        TypeEnum,
        #[doc = "Field type fixed32."]
        TypeFixed32,
        #[doc = "Field type fixed64."]
        TypeFixed64,
        #[doc = "Field type float."]
        TypeFloat,
        #[doc = "Field type group. Proto2 syntax only, and deprecated."]
        TypeGroup,
        #[doc = "Field type int32."]
        TypeInt32,
        #[doc = "Field type int64."]
        TypeInt64,
        #[doc = "Field type message."]
        TypeMessage,
        #[doc = "Field type sfixed32."]
        TypeSfixed32,
        #[doc = "Field type sfixed64."]
        TypeSfixed64,
        #[doc = "Field type sint32."]
        TypeSint32,
        #[doc = "Field type sint64."]
        TypeSint64,
        #[doc = "Field type string."]
        TypeString,
        #[doc = "Field type uint32."]
        TypeUint32,
        #[doc = "Field type uint64."]
        TypeUint64,
        #[doc = "Field type unknown."]
        TypeUnknown,
    }
    impl FieldKind {
        pub fn as_str(self) -> &'static str {
            match self {
                FieldKind::TypeBool => "TYPE_BOOL",
                FieldKind::TypeBytes => "TYPE_BYTES",
                FieldKind::TypeDouble => "TYPE_DOUBLE",
                FieldKind::TypeEnum => "TYPE_ENUM",
                FieldKind::TypeFixed32 => "TYPE_FIXED32",
                FieldKind::TypeFixed64 => "TYPE_FIXED64",
                FieldKind::TypeFloat => "TYPE_FLOAT",
                FieldKind::TypeGroup => "TYPE_GROUP",
                FieldKind::TypeInt32 => "TYPE_INT32",
                FieldKind::TypeInt64 => "TYPE_INT64",
                FieldKind::TypeMessage => "TYPE_MESSAGE",
                FieldKind::TypeSfixed32 => "TYPE_SFIXED32",
                FieldKind::TypeSfixed64 => "TYPE_SFIXED64",
                FieldKind::TypeSint32 => "TYPE_SINT32",
                FieldKind::TypeSint64 => "TYPE_SINT64",
                FieldKind::TypeString => "TYPE_STRING",
                FieldKind::TypeUint32 => "TYPE_UINT32",
                FieldKind::TypeUint64 => "TYPE_UINT64",
                FieldKind::TypeUnknown => "TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for FieldKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FieldKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FieldKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_BOOL" => FieldKind::TypeBool,
                "TYPE_BYTES" => FieldKind::TypeBytes,
                "TYPE_DOUBLE" => FieldKind::TypeDouble,
                "TYPE_ENUM" => FieldKind::TypeEnum,
                "TYPE_FIXED32" => FieldKind::TypeFixed32,
                "TYPE_FIXED64" => FieldKind::TypeFixed64,
                "TYPE_FLOAT" => FieldKind::TypeFloat,
                "TYPE_GROUP" => FieldKind::TypeGroup,
                "TYPE_INT32" => FieldKind::TypeInt32,
                "TYPE_INT64" => FieldKind::TypeInt64,
                "TYPE_MESSAGE" => FieldKind::TypeMessage,
                "TYPE_SFIXED32" => FieldKind::TypeSfixed32,
                "TYPE_SFIXED64" => FieldKind::TypeSfixed64,
                "TYPE_SINT32" => FieldKind::TypeSint32,
                "TYPE_SINT64" => FieldKind::TypeSint64,
                "TYPE_STRING" => FieldKind::TypeString,
                "TYPE_UINT32" => FieldKind::TypeUint32,
                "TYPE_UINT64" => FieldKind::TypeUint64,
                "TYPE_UNKNOWN" => FieldKind::TypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FieldKind {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiService {
        #[doc = "A list of API interfaces exported by this service. Only the `name` field\nof the google.protobuf.Api needs to be provided by the configuration\nauthor, as the remaining fields will be derived from the IDL during the\nnormalization process. It is an error to specify an API interface here\nwhich cannot be resolved against the associated IDL files."]
        #[serde(rename = "apis", default)]
        pub apis: ::std::option::Option<Vec<crate::schemas::Api>>,
        #[doc = "Auth configuration."]
        #[serde(rename = "authentication", default)]
        pub authentication: ::std::option::Option<crate::schemas::Authentication>,
        #[doc = "API backend configuration."]
        #[serde(rename = "backend", default)]
        pub backend: ::std::option::Option<crate::schemas::Backend>,
        #[doc = "Billing configuration."]
        #[serde(rename = "billing", default)]
        pub billing: ::std::option::Option<crate::schemas::Billing>,
        #[doc = "The semantic version of the service configuration. The config version\naffects the interpretation of the service configuration. For example,\ncertain features are enabled by default for certain config versions.\nThe latest config version is `3`."]
        #[serde(rename = "configVersion", default)]
        pub config_version: ::std::option::Option<u32>,
        #[doc = "Context configuration."]
        #[serde(rename = "context", default)]
        pub context: ::std::option::Option<crate::schemas::Context>,
        #[doc = "Configuration for the service control plane."]
        #[serde(rename = "control", default)]
        pub control: ::std::option::Option<crate::schemas::Control>,
        #[doc = "Custom error configuration."]
        #[serde(rename = "customError", default)]
        pub custom_error: ::std::option::Option<crate::schemas::CustomError>,
        #[doc = "Additional API documentation."]
        #[serde(rename = "documentation", default)]
        pub documentation: ::std::option::Option<crate::schemas::Documentation>,
        #[doc = "Configuration for network endpoints.  If this is empty, then an endpoint\nwith the same name as the service is automatically generated to service all\ndefined APIs."]
        #[serde(rename = "endpoints", default)]
        pub endpoints: ::std::option::Option<Vec<crate::schemas::Endpoint>>,
        #[doc = "A list of all enum types included in this API service.  Enums\nreferenced directly or indirectly by the `apis` are automatically\nincluded.  Enums which are not referenced but shall be included\nshould be listed here by name. Example:\n\n````text\nenums:\n- name: google.someapi.v1.SomeEnum````"]
        #[serde(rename = "enums", default)]
        pub enums: ::std::option::Option<Vec<crate::schemas::Enum>>,
        #[doc = "HTTP configuration."]
        #[serde(rename = "http", default)]
        pub http: ::std::option::Option<crate::schemas::Http>,
        #[doc = "A unique ID for a specific instance of this message, typically assigned\nby the client for tracking purpose. If empty, the server may choose to\ngenerate one instead. Must be no longer than 60 characters."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Logging configuration."]
        #[serde(rename = "logging", default)]
        pub logging: ::std::option::Option<crate::schemas::Logging>,
        #[doc = "Defines the logs used by this service."]
        #[serde(rename = "logs", default)]
        pub logs: ::std::option::Option<Vec<crate::schemas::LogDescriptor>>,
        #[doc = "Defines the metrics used by this service."]
        #[serde(rename = "metrics", default)]
        pub metrics: ::std::option::Option<Vec<crate::schemas::MetricDescriptor>>,
        #[doc = "Defines the monitored resources used by this service. This is required\nby the Service.monitoring and Service.logging configurations."]
        #[serde(rename = "monitoredResources", default)]
        pub monitored_resources:
            ::std::option::Option<Vec<crate::schemas::MonitoredResourceDescriptor>>,
        #[doc = "Monitoring configuration."]
        #[serde(rename = "monitoring", default)]
        pub monitoring: ::std::option::Option<crate::schemas::Monitoring>,
        #[doc = "The service name, which is a DNS-like logical identifier for the\nservice, such as `calendar.googleapis.com`. The service name\ntypically goes through DNS verification to make sure the owner\nof the service also owns the DNS name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The Google project that owns this service."]
        #[serde(rename = "producerProjectId", default)]
        pub producer_project_id: ::std::option::Option<String>,
        #[doc = "Quota configuration."]
        #[serde(rename = "quota", default)]
        pub quota: ::std::option::Option<crate::schemas::Quota>,
        #[doc = "Output only. The source information for this configuration if available."]
        #[serde(rename = "sourceInfo", default)]
        pub source_info: ::std::option::Option<crate::schemas::SourceInfo>,
        #[doc = "System parameter configuration."]
        #[serde(rename = "systemParameters", default)]
        pub system_parameters: ::std::option::Option<crate::schemas::SystemParameters>,
        #[doc = "A list of all proto message types included in this API service.\nIt serves similar purpose as [google.api.Service.types], except that\nthese types are not needed by user-defined APIs. Therefore, they will not\nshow up in the generated discovery doc. This field should only be used\nto define system APIs in ESF."]
        #[serde(rename = "systemTypes", default)]
        pub system_types: ::std::option::Option<Vec<crate::schemas::Type>>,
        #[doc = "The product title for this service."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "A list of all proto message types included in this API service.\nTypes referenced directly or indirectly by the `apis` are\nautomatically included.  Messages which are not referenced but\nshall be included, such as types used by the `google.protobuf.Any` type,\nshould be listed here by name. Example:\n\n````text\ntypes:\n- name: google.protobuf.Int32````"]
        #[serde(rename = "types", default)]
        pub types: ::std::option::Option<Vec<crate::schemas::Type>>,
        #[doc = "Configuration controlling usage of this service."]
        #[serde(rename = "usage", default)]
        pub usage: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::field_selector::FieldSelector for GoogleApiService {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct GoogleApiServiceusageV1OperationMetadata {
        #[doc = "The full name of the resources that this operation is directly\nassociated with."]
        #[serde(rename = "resourceNames", default)]
        pub resource_names: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for GoogleApiServiceusageV1OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiServiceusageV1Service {
        #[doc = "The service configuration of the available service.\nSome fields may be filtered out of the configuration in responses to\nthe `ListServices` method. These fields are present only in responses to\nthe `GetService` method."]
        #[serde(rename = "config", default)]
        pub config: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1ServiceConfig>,
        #[doc = "The resource name of the consumer and service.\n\nA valid name would be:\n\n* projects/123/services/serviceusage.googleapis.com"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The resource name of the consumer.\n\nA valid name would be:\n\n* projects/123"]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
        #[doc = "Whether or not the service has been enabled for use by the consumer."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::GoogleApiServiceusageV1ServiceState>,
    }
    impl ::field_selector::FieldSelector for GoogleApiServiceusageV1Service {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleApiServiceusageV1ServiceState {
        #[doc = "The service cannot be used by this consumer. It has either been explicitly\ndisabled, or has never been enabled."]
        Disabled,
        #[doc = "The service has been explicitly enabled for use by this consumer."]
        Enabled,
        #[doc = "The default value, which indicates that the enabled state of the service\nis unspecified or not meaningful. Currently, all consumers other than\nprojects (such as folders and organizations) are always in this state."]
        StateUnspecified,
    }
    impl GoogleApiServiceusageV1ServiceState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleApiServiceusageV1ServiceState::Disabled => "DISABLED",
                GoogleApiServiceusageV1ServiceState::Enabled => "ENABLED",
                GoogleApiServiceusageV1ServiceState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleApiServiceusageV1ServiceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleApiServiceusageV1ServiceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleApiServiceusageV1ServiceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISABLED" => GoogleApiServiceusageV1ServiceState::Disabled,
                "ENABLED" => GoogleApiServiceusageV1ServiceState::Enabled,
                "STATE_UNSPECIFIED" => GoogleApiServiceusageV1ServiceState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleApiServiceusageV1ServiceState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiServiceusageV1ServiceConfig {
        #[doc = "A list of API interfaces exported by this service. Contains only the names,\nversions, and method names of the interfaces."]
        #[serde(rename = "apis", default)]
        pub apis: ::std::option::Option<Vec<crate::schemas::Api>>,
        #[doc = "Auth configuration. Contains only the OAuth rules."]
        #[serde(rename = "authentication", default)]
        pub authentication: ::std::option::Option<crate::schemas::Authentication>,
        #[doc = "Additional API documentation. Contains only the summary and the\ndocumentation URL."]
        #[serde(rename = "documentation", default)]
        pub documentation: ::std::option::Option<crate::schemas::Documentation>,
        #[doc = "Configuration for network endpoints. Contains only the names and aliases\nof the endpoints."]
        #[serde(rename = "endpoints", default)]
        pub endpoints: ::std::option::Option<Vec<crate::schemas::Endpoint>>,
        #[doc = "The DNS address at which this service is available.\n\nAn example DNS address would be:\n`calendar.googleapis.com`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Quota configuration."]
        #[serde(rename = "quota", default)]
        pub quota: ::std::option::Option<crate::schemas::Quota>,
        #[doc = "The product title for this service."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "Configuration controlling usage of this service."]
        #[serde(rename = "usage", default)]
        pub usage: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::field_selector::FieldSelector for GoogleApiServiceusageV1ServiceConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Http {
        #[doc = "When set to true, URL path parameters will be fully URI-decoded except in\ncases of single segment matches in reserved expansion, where \"%2F\" will be\nleft encoded.\n\nThe default behavior is to not decode RFC 6570 reserved characters in multi\nsegment matches."]
        #[serde(rename = "fullyDecodeReservedExpansion", default)]
        pub fully_decode_reserved_expansion: ::std::option::Option<bool>,
        #[doc = "A list of HTTP configuration rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::HttpRule>>,
    }
    impl ::field_selector::FieldSelector for Http {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct HttpRule {
        #[doc = "Additional HTTP bindings for the selector. Nested bindings must\nnot contain an `additional_bindings` field themselves (that is,\nthe nesting may only be one level deep)."]
        #[serde(rename = "additionalBindings", default)]
        pub additional_bindings: ::std::option::Option<Vec<crate::schemas::HttpRule>>,
        #[doc = "The name of the request field whose value is mapped to the HTTP request\nbody, or `*` for mapping all request fields not captured by the path\npattern to the HTTP body, or omitted for not having any HTTP request body.\n\nNOTE: the referred field must be present at the top-level of the request\nmessage type."]
        #[serde(rename = "body", default)]
        pub body: ::std::option::Option<String>,
        #[doc = "The custom pattern is used for specifying an HTTP method that is not\nincluded in the `pattern` field, such as HEAD, or \"*\" to leave the\nHTTP method unspecified for this rule. The wild-card rule is useful\nfor services that provide content to Web (HTML) clients."]
        #[serde(rename = "custom", default)]
        pub custom: ::std::option::Option<crate::schemas::CustomHttpPattern>,
        #[doc = "Maps to HTTP DELETE. Used for deleting a resource."]
        #[serde(rename = "delete", default)]
        pub delete: ::std::option::Option<String>,
        #[doc = "Maps to HTTP GET. Used for listing and getting information about\nresources."]
        #[serde(rename = "get", default)]
        pub get: ::std::option::Option<String>,
        #[doc = "Maps to HTTP PATCH. Used for updating a resource."]
        #[serde(rename = "patch", default)]
        pub patch: ::std::option::Option<String>,
        #[doc = "Maps to HTTP POST. Used for creating a resource or performing an action."]
        #[serde(rename = "post", default)]
        pub post: ::std::option::Option<String>,
        #[doc = "Maps to HTTP PUT. Used for replacing a resource."]
        #[serde(rename = "put", default)]
        pub put: ::std::option::Option<String>,
        #[doc = "Optional. The name of the response field whose value is mapped to the HTTP\nresponse body. When omitted, the entire response message will be used\nas the HTTP response body.\n\nNOTE: The referred field must be present at the top-level of the response\nmessage type."]
        #[serde(rename = "responseBody", default)]
        pub response_body: ::std::option::Option<String>,
        #[doc = "Selects a method to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for HttpRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct LabelDescriptor {
        #[doc = "A human-readable description for the label."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The label key."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "The type of data that can be assigned to the label."]
        #[serde(rename = "valueType", default)]
        pub value_type: ::std::option::Option<crate::schemas::LabelDescriptorValueType>,
    }
    impl ::field_selector::FieldSelector for LabelDescriptor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LabelDescriptorValueType {
        #[doc = "Boolean; true or false."]
        Bool,
        #[doc = "A 64-bit signed integer."]
        Int64,
        #[doc = "A variable-length string. This is the default."]
        String,
    }
    impl LabelDescriptorValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                LabelDescriptorValueType::Bool => "BOOL",
                LabelDescriptorValueType::Int64 => "INT64",
                LabelDescriptorValueType::String => "STRING",
            }
        }
    }
    impl ::std::fmt::Display for LabelDescriptorValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LabelDescriptorValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LabelDescriptorValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOL" => LabelDescriptorValueType::Bool,
                "INT64" => LabelDescriptorValueType::Int64,
                "STRING" => LabelDescriptorValueType::String,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for LabelDescriptorValueType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(rename = "operations", default)]
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
    }
    impl ::field_selector::FieldSelector for ListOperationsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListServicesResponse {
        #[doc = "Token that can be passed to `ListServices` to resume a paginated\nquery."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The available services for the requested project."]
        #[serde(rename = "services", default)]
        pub services: ::std::option::Option<Vec<crate::schemas::GoogleApiServiceusageV1Service>>,
    }
    impl ::field_selector::FieldSelector for ListServicesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct LogDescriptor {
        #[doc = "A human-readable description of this log. This information appears in\nthe documentation and can contain details."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The human-readable name for this log. This information appears on\nthe user interface and should be concise."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The set of labels that are available to describe a specific log entry.\nRuntime requests that contain labels not specified here are\nconsidered invalid."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "The name of the log. It must be less than 512 characters long and can\ninclude the following characters: upper- and lower-case alphanumeric\ncharacters [A-Za-z0-9], and punctuation characters including\nslash, underscore, hyphen, period [/_-.]."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for LogDescriptor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Logging {
        #[doc = "Logging configurations for sending logs to the consumer project.\nThere can be multiple consumer destinations, each one must have a\ndifferent monitored resource type. A log can be used in at most\none consumer destination."]
        #[serde(rename = "consumerDestinations", default)]
        pub consumer_destinations: ::std::option::Option<Vec<crate::schemas::LoggingDestination>>,
        #[doc = "Logging configurations for sending logs to the producer project.\nThere can be multiple producer destinations, each one must have a\ndifferent monitored resource type. A log can be used in at most\none producer destination."]
        #[serde(rename = "producerDestinations", default)]
        pub producer_destinations: ::std::option::Option<Vec<crate::schemas::LoggingDestination>>,
    }
    impl ::field_selector::FieldSelector for Logging {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct LoggingDestination {
        #[doc = "Names of the logs to be sent to this destination. Each name must\nbe defined in the Service.logs section. If the log name is\nnot a domain scoped name, it will be automatically prefixed with\nthe service name followed by \"/\"."]
        #[serde(rename = "logs", default)]
        pub logs: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in the\nService.monitored_resources section."]
        #[serde(rename = "monitoredResource", default)]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for LoggingDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Method {
        #[doc = "The simple name of this method."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Any metadata attached to the method."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "If true, the request is streamed."]
        #[serde(rename = "requestStreaming", default)]
        pub request_streaming: ::std::option::Option<bool>,
        #[doc = "A URL of the input message type."]
        #[serde(rename = "requestTypeUrl", default)]
        pub request_type_url: ::std::option::Option<String>,
        #[doc = "If true, the response is streamed."]
        #[serde(rename = "responseStreaming", default)]
        pub response_streaming: ::std::option::Option<bool>,
        #[doc = "The URL of the output message type."]
        #[serde(rename = "responseTypeUrl", default)]
        pub response_type_url: ::std::option::Option<String>,
        #[doc = "The source syntax of this method."]
        #[serde(rename = "syntax", default)]
        pub syntax: ::std::option::Option<crate::schemas::MethodSyntax>,
    }
    impl ::field_selector::FieldSelector for Method {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MethodSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl MethodSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                MethodSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                MethodSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::fmt::Display for MethodSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MethodSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MethodSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => MethodSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => MethodSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for MethodSyntax {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct MetricDescriptor {
        #[doc = "A detailed description of the metric, which can be used in documentation."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "A concise name for the metric, which can be displayed in user interfaces.\nUse sentence case without an ending period, for example \"Request count\".\nThis field is optional but it is recommended to be set for any metrics\nassociated with user-visible concepts, such as Quota."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The set of labels that can be used to describe a specific\ninstance of this metric type. For example, the\n`appengine.googleapis.com/http/server/response_latencies` metric\ntype has a label for the HTTP response code, `response_code`, so\nyou can look at latencies for successful responses or just\nfor responses that failed."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "Optional. The launch stage of the metric definition."]
        #[serde(rename = "launchStage", default)]
        pub launch_stage: ::std::option::Option<crate::schemas::MetricDescriptorLaunchStage>,
        #[doc = "Optional. Metadata which can be used to guide usage of the metric."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::MetricDescriptorMetadata>,
        #[doc = "Whether the metric records instantaneous values, changes to a value, etc.\nSome combinations of `metric_kind` and `value_type` might not be supported."]
        #[serde(rename = "metricKind", default)]
        pub metric_kind: ::std::option::Option<crate::schemas::MetricDescriptorMetricKind>,
        #[doc = "The resource name of the metric descriptor."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The metric type, including its DNS name prefix. The type is not\nURL-encoded.  All user-defined metric types have the DNS name\n`custom.googleapis.com` or `external.googleapis.com`.  Metric types should\nuse a natural hierarchical grouping. For example:\n\n````text\n\"custom.googleapis.com/invoice/paid/amount\"\n\"external.googleapis.com/prometheus/up\"\n\"appengine.googleapis.com/http/server/response_latencies\"````"]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The unit in which the metric value is reported. It is only applicable\nif the `value_type` is `INT64`, `DOUBLE`, or `DISTRIBUTION`. The\nsupported units are a subset of [The Unified Code for Units of\nMeasure](http://unitsofmeasure.org/ucum.html) standard:\n\n**Basic units (UNIT)**\n\n* `bit`   bit\n* `By`    byte\n* `s`     second\n* `min`   minute\n* `h`     hour\n* `d`     day\n\n**Prefixes (PREFIX)**\n\n* `k`     kilo    (10**3)\n* `M`     mega    (10**6)\n* `G`     giga    (10**9)\n* `T`     tera    (10**12)\n* `P`     peta    (10**15)\n* `E`     exa     (10**18)\n* `Z`     zetta   (10**21)\n* `Y`     yotta   (10**24)\n* `m`     milli   (10**-3)\n* `u`     micro   (10**-6)\n* `n`     nano    (10**-9)\n* `p`     pico    (10**-12)\n* `f`     femto   (10**-15)\n* `a`     atto    (10**-18)\n* `z`     zepto   (10**-21)\n* `y`     yocto   (10**-24)\n* `Ki`    kibi    (2**10)\n* `Mi`    mebi    (2**20)\n* `Gi`    gibi    (2**30)\n* `Ti`    tebi    (2**40)\n\n**Grammar**\n\nThe grammar also includes these connectors:\n\n* `/`    division (as an infix operator, e.g. `1/s`).\n* `.`    multiplication (as an infix operator, e.g. `GBy.d`)\n\nThe grammar for a unit is as follows:\n\n````text\nExpression = Component { \".\" Component } { \"/\" Component } ;\n\nComponent = ( [ PREFIX ] UNIT | \"%\" ) [ Annotation ]\n          | Annotation\n          | \"1\"\n          ;\n\nAnnotation = \"{\" NAME \"}\" ;\n````\n\nNotes:\n\n* `Annotation` is just a comment if it follows a `UNIT` and is\n  equivalent to `1` if it is used alone. For examples,\n  `{requests}/s == 1/s`, `By{transmitted}/s == By/s`.\n* `NAME` is a sequence of non-blank printable ASCII characters not\n  containing '{' or '}'.\n* `1` represents dimensionless value 1, such as in `1/s`.\n* `%` represents dimensionless value 1/100, and annotates values giving\n  a percentage."]
        #[serde(rename = "unit", default)]
        pub unit: ::std::option::Option<String>,
        #[doc = "Whether the measurement is an integer, a floating-point number, etc.\nSome combinations of `metric_kind` and `value_type` might not be supported."]
        #[serde(rename = "valueType", default)]
        pub value_type: ::std::option::Option<crate::schemas::MetricDescriptorValueType>,
    }
    impl ::field_selector::FieldSelector for MetricDescriptor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared\nfor widespread use. By Alpha, all significant design issues are resolved\nand we are in the process of verifying functionality. Alpha customers\nneed to apply for access, agree to applicable terms, and have their\nprojects whitelisted. Alpha releases don\u{2019}t have to be feature complete,\nno SLAs are provided, and there are no technical support obligations, but\nthey will be far enough along that customers can actually use them in\ntest environments or for limited-use tests -- just like they would in\nnormal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any\ncustomer to use. There are no SLA or technical support obligations in a\nBeta release. Products will be complete from a feature perspective, but\nmay have some open outstanding issues. Beta releases are suitable for\nlimited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more\ninformation, see the \u{201c}Deprecation Policy\u{201d} section of our [Terms of\nService](https://cloud.google.com/terms/)\nand the [Google Cloud Platform Subject to the Deprecation\nPolicy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use\nthese features, you must sign up in advance and sign a Trusted Tester\nagreement (which includes confidentiality provisions). These features may\nbe unstable, changed in backward-incompatible ways, and are not\nguaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and\nfully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
    }
    impl MetricDescriptorLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorLaunchStage::Alpha => "ALPHA",
                MetricDescriptorLaunchStage::Beta => "BETA",
                MetricDescriptorLaunchStage::Deprecated => "DEPRECATED",
                MetricDescriptorLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MetricDescriptorLaunchStage::Ga => "GA",
                MetricDescriptorLaunchStage::LaunchStageUnspecified => "LAUNCH_STAGE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for MetricDescriptorLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MetricDescriptorLaunchStage::Alpha,
                "BETA" => MetricDescriptorLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => MetricDescriptorLaunchStage::LaunchStageUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for MetricDescriptorLaunchStage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorMetricKind {
        #[doc = "A value accumulated over a time interval.  Cumulative\nmeasurements in a time series should have the same start time\nand increasing end times, until an event resets the cumulative\nvalue to zero and sets a new start time for the following\npoints."]
        Cumulative,
        #[doc = "The change in a value during a time interval."]
        Delta,
        #[doc = "An instantaneous measurement of a value."]
        Gauge,
        #[doc = "Do not use this default value."]
        MetricKindUnspecified,
    }
    impl MetricDescriptorMetricKind {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorMetricKind::Cumulative => "CUMULATIVE",
                MetricDescriptorMetricKind::Delta => "DELTA",
                MetricDescriptorMetricKind::Gauge => "GAUGE",
                MetricDescriptorMetricKind::MetricKindUnspecified => "METRIC_KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for MetricDescriptorMetricKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorMetricKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorMetricKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUMULATIVE" => MetricDescriptorMetricKind::Cumulative,
                "DELTA" => MetricDescriptorMetricKind::Delta,
                "GAUGE" => MetricDescriptorMetricKind::Gauge,
                "METRIC_KIND_UNSPECIFIED" => MetricDescriptorMetricKind::MetricKindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for MetricDescriptorMetricKind {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorValueType {
        #[doc = "The value is a boolean.\nThis value type can be used only if the metric kind is `GAUGE`."]
        Bool,
        #[doc = "The value is a `Distribution`."]
        Distribution,
        #[doc = "The value is a double precision floating point number."]
        Double,
        #[doc = "The value is a signed 64-bit integer."]
        Int64,
        #[doc = "The value is money."]
        Money,
        #[doc = "The value is a text string.\nThis value type can be used only if the metric kind is `GAUGE`."]
        String,
        #[doc = "Do not use this default value."]
        ValueTypeUnspecified,
    }
    impl MetricDescriptorValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorValueType::Bool => "BOOL",
                MetricDescriptorValueType::Distribution => "DISTRIBUTION",
                MetricDescriptorValueType::Double => "DOUBLE",
                MetricDescriptorValueType::Int64 => "INT64",
                MetricDescriptorValueType::Money => "MONEY",
                MetricDescriptorValueType::String => "STRING",
                MetricDescriptorValueType::ValueTypeUnspecified => "VALUE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for MetricDescriptorValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOL" => MetricDescriptorValueType::Bool,
                "DISTRIBUTION" => MetricDescriptorValueType::Distribution,
                "DOUBLE" => MetricDescriptorValueType::Double,
                "INT64" => MetricDescriptorValueType::Int64,
                "MONEY" => MetricDescriptorValueType::Money,
                "STRING" => MetricDescriptorValueType::String,
                "VALUE_TYPE_UNSPECIFIED" => MetricDescriptorValueType::ValueTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for MetricDescriptorValueType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct MetricDescriptorMetadata {
        #[doc = "The delay of data points caused by ingestion. Data points older than this\nage are guaranteed to be ingested and available to be read, excluding\ndata loss due to errors."]
        #[serde(rename = "ingestDelay", default)]
        pub ingest_delay: ::std::option::Option<String>,
        #[doc = "Deprecated. Please use the MetricDescriptor.launch_stage instead.\nThe launch stage of the metric definition."]
        #[serde(rename = "launchStage", default)]
        pub launch_stage:
            ::std::option::Option<crate::schemas::MetricDescriptorMetadataLaunchStage>,
        #[doc = "The sampling period of metric data points. For metrics which are written\nperiodically, consecutive data points are stored at this time interval,\nexcluding data loss due to errors. Metrics with a higher granularity have\na smaller sampling period."]
        #[serde(rename = "samplePeriod", default)]
        pub sample_period: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MetricDescriptorMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricDescriptorMetadataLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared\nfor widespread use. By Alpha, all significant design issues are resolved\nand we are in the process of verifying functionality. Alpha customers\nneed to apply for access, agree to applicable terms, and have their\nprojects whitelisted. Alpha releases don\u{2019}t have to be feature complete,\nno SLAs are provided, and there are no technical support obligations, but\nthey will be far enough along that customers can actually use them in\ntest environments or for limited-use tests -- just like they would in\nnormal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any\ncustomer to use. There are no SLA or technical support obligations in a\nBeta release. Products will be complete from a feature perspective, but\nmay have some open outstanding issues. Beta releases are suitable for\nlimited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more\ninformation, see the \u{201c}Deprecation Policy\u{201d} section of our [Terms of\nService](https://cloud.google.com/terms/)\nand the [Google Cloud Platform Subject to the Deprecation\nPolicy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use\nthese features, you must sign up in advance and sign a Trusted Tester\nagreement (which includes confidentiality provisions). These features may\nbe unstable, changed in backward-incompatible ways, and are not\nguaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and\nfully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
    }
    impl MetricDescriptorMetadataLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricDescriptorMetadataLaunchStage::Alpha => "ALPHA",
                MetricDescriptorMetadataLaunchStage::Beta => "BETA",
                MetricDescriptorMetadataLaunchStage::Deprecated => "DEPRECATED",
                MetricDescriptorMetadataLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MetricDescriptorMetadataLaunchStage::Ga => "GA",
                MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified => {
                    "LAUNCH_STAGE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for MetricDescriptorMetadataLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricDescriptorMetadataLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricDescriptorMetadataLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MetricDescriptorMetadataLaunchStage::Alpha,
                "BETA" => MetricDescriptorMetadataLaunchStage::Beta,
                "DEPRECATED" => MetricDescriptorMetadataLaunchStage::Deprecated,
                "EARLY_ACCESS" => MetricDescriptorMetadataLaunchStage::EarlyAccess,
                "GA" => MetricDescriptorMetadataLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MetricDescriptorMetadataLaunchStage::LaunchStageUnspecified
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
    impl ::field_selector::FieldSelector for MetricDescriptorMetadataLaunchStage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct MetricRule {
        #[doc = "Metrics to update when the selected methods are called, and the associated\ncost applied to each metric.\n\nThe key of the map is the metric name, and the values are the amount\nincreased for the metric against which the quota limits are defined.\nThe value must not be negative."]
        #[serde(rename = "metricCosts", default)]
        pub metric_costs: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
        #[doc = "Selects the methods to which this rule applies.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MetricRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Mixin {
        #[doc = "The fully qualified name of the interface which is included."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "If non-empty specifies a path under which inherited HTTP paths\nare rooted."]
        #[serde(rename = "root", default)]
        pub root: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Mixin {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct MonitoredResourceDescriptor {
        #[doc = "Optional. A detailed description of the monitored resource type that might\nbe used in documentation."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional. A concise name for the monitored resource type that might be\ndisplayed in user interfaces. It should be a Title Cased Noun Phrase,\nwithout any article or other determiners. For example,\n`\"Google Cloud SQL Database\"`."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Required. A set of labels used to describe instances of this monitored\nresource type. For example, an individual Google Cloud SQL database is\nidentified by values for the labels `\"database_id\"` and `\"zone\"`."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<Vec<crate::schemas::LabelDescriptor>>,
        #[doc = "Optional. The launch stage of the monitored resource definition."]
        #[serde(rename = "launchStage", default)]
        pub launch_stage:
            ::std::option::Option<crate::schemas::MonitoredResourceDescriptorLaunchStage>,
        #[doc = "Optional. The resource name of the monitored resource descriptor:\n`\"projects/{project_id}/monitoredResourceDescriptors/{type}\"` where\n{type} is the value of the `type` field in this object and\n{project_id} is a project ID that provides API-specific context for\naccessing the type.  APIs that do not use project information can use the\nresource name format `\"monitoredResourceDescriptors/{type}\"`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The monitored resource type. For example, the type\n`\"cloudsql_database\"` represents databases in Google Cloud SQL.\nThe maximum length of this value is 256 characters."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MonitoredResourceDescriptor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MonitoredResourceDescriptorLaunchStage {
        #[doc = "Alpha is a limited availability test for releases before they are cleared\nfor widespread use. By Alpha, all significant design issues are resolved\nand we are in the process of verifying functionality. Alpha customers\nneed to apply for access, agree to applicable terms, and have their\nprojects whitelisted. Alpha releases don\u{2019}t have to be feature complete,\nno SLAs are provided, and there are no technical support obligations, but\nthey will be far enough along that customers can actually use them in\ntest environments or for limited-use tests -- just like they would in\nnormal production cases."]
        Alpha,
        #[doc = "Beta is the point at which we are ready to open a release for any\ncustomer to use. There are no SLA or technical support obligations in a\nBeta release. Products will be complete from a feature perspective, but\nmay have some open outstanding issues. Beta releases are suitable for\nlimited production use cases."]
        Beta,
        #[doc = "Deprecated features are scheduled to be shut down and removed. For more\ninformation, see the \u{201c}Deprecation Policy\u{201d} section of our [Terms of\nService](https://cloud.google.com/terms/)\nand the [Google Cloud Platform Subject to the Deprecation\nPolicy](https://cloud.google.com/terms/deprecation) documentation."]
        Deprecated,
        #[doc = "Early Access features are limited to a closed group of testers. To use\nthese features, you must sign up in advance and sign a Trusted Tester\nagreement (which includes confidentiality provisions). These features may\nbe unstable, changed in backward-incompatible ways, and are not\nguaranteed to be released."]
        EarlyAccess,
        #[doc = "GA features are open to all developers and are considered stable and\nfully qualified for production use."]
        Ga,
        #[doc = "Do not use this default value."]
        LaunchStageUnspecified,
    }
    impl MonitoredResourceDescriptorLaunchStage {
        pub fn as_str(self) -> &'static str {
            match self {
                MonitoredResourceDescriptorLaunchStage::Alpha => "ALPHA",
                MonitoredResourceDescriptorLaunchStage::Beta => "BETA",
                MonitoredResourceDescriptorLaunchStage::Deprecated => "DEPRECATED",
                MonitoredResourceDescriptorLaunchStage::EarlyAccess => "EARLY_ACCESS",
                MonitoredResourceDescriptorLaunchStage::Ga => "GA",
                MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified => {
                    "LAUNCH_STAGE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for MonitoredResourceDescriptorLaunchStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MonitoredResourceDescriptorLaunchStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MonitoredResourceDescriptorLaunchStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => MonitoredResourceDescriptorLaunchStage::Alpha,
                "BETA" => MonitoredResourceDescriptorLaunchStage::Beta,
                "DEPRECATED" => MonitoredResourceDescriptorLaunchStage::Deprecated,
                "EARLY_ACCESS" => MonitoredResourceDescriptorLaunchStage::EarlyAccess,
                "GA" => MonitoredResourceDescriptorLaunchStage::Ga,
                "LAUNCH_STAGE_UNSPECIFIED" => {
                    MonitoredResourceDescriptorLaunchStage::LaunchStageUnspecified
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
    impl ::field_selector::FieldSelector for MonitoredResourceDescriptorLaunchStage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Monitoring {
        #[doc = "Monitoring configurations for sending metrics to the consumer project.\nThere can be multiple consumer destinations. A monitored resouce type may\nappear in multiple monitoring destinations if different aggregations are\nneeded for different sets of metrics associated with that monitored\nresource type. A monitored resource and metric pair may only be used once\nin the Monitoring configuration."]
        #[serde(rename = "consumerDestinations", default)]
        pub consumer_destinations:
            ::std::option::Option<Vec<crate::schemas::MonitoringDestination>>,
        #[doc = "Monitoring configurations for sending metrics to the producer project.\nThere can be multiple producer destinations. A monitored resouce type may\nappear in multiple monitoring destinations if different aggregations are\nneeded for different sets of metrics associated with that monitored\nresource type. A monitored resource and metric pair may only be used once\nin the Monitoring configuration."]
        #[serde(rename = "producerDestinations", default)]
        pub producer_destinations:
            ::std::option::Option<Vec<crate::schemas::MonitoringDestination>>,
    }
    impl ::field_selector::FieldSelector for Monitoring {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct MonitoringDestination {
        #[doc = "Types of the metrics to report to this monitoring destination.\nEach type must be defined in Service.metrics section."]
        #[serde(rename = "metrics", default)]
        pub metrics: ::std::option::Option<Vec<String>>,
        #[doc = "The monitored resource type. The type must be defined in\nService.monitored_resources section."]
        #[serde(rename = "monitoredResource", default)]
        pub monitored_resource: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MonitoringDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct OauthRequirements {
        #[doc = "The list of publicly documented OAuth scopes that are allowed access. An\nOAuth token containing any of these scopes will be accepted.\n\nExample:\n\n````text\n canonical_scopes: https://www.googleapis.com/auth/calendar,\n                   https://www.googleapis.com/auth/calendar.read````"]
        #[serde(rename = "canonicalScopes", default)]
        pub canonical_scopes: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for OauthRequirements {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Operation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct OperationMetadata {
        #[doc = "The full name of the resources that this operation is directly\nassociated with."]
        #[serde(rename = "resourceNames", default)]
        pub resource_names: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for OperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Option {
        #[doc = "The option's name. For protobuf built-in options (options defined in\ndescriptor.proto), this is the short name. For example, `\"map_entry\"`.\nFor custom options, it should be the fully-qualified name. For example,\n`\"google.api.http\"`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The option's value packed in an Any message. If the value is a primitive,\nthe corresponding wrapper type defined in google/protobuf/wrappers.proto\nshould be used. If the value is an enum, it should be stored as an int32\nvalue using the google.protobuf.Int32Value type."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Option {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Page {
        #[doc = "The Markdown content of the page. You can use <code>(== include {path}\n==)</code> to include content from a Markdown file."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<String>,
        #[doc = "The name of the page. It will be used as an identity of the page to\ngenerate URI of the page, text of the link to this page in navigation,\netc. The full page name (start from the root page name to this page\nconcatenated with `.`) can be used as reference to the page in your\ndocumentation. For example:\n\n<pre><code>pages:\n- name: Tutorial\n  content: &#40;== include tutorial.md ==&#41;\n  subpages:\n  - name: Java\n    content: &#40;== include tutorial_java.md ==&#41;\n</code></pre>\n\nYou can reference `Java` page using Markdown reference link syntax:\n`Java`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Subpages of this page. The order of subpages specified here will be\nhonored in the generated docset."]
        #[serde(rename = "subpages", default)]
        pub subpages: ::std::option::Option<Vec<crate::schemas::Page>>,
    }
    impl ::field_selector::FieldSelector for Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Quota {
        #[doc = "List of `QuotaLimit` definitions for the service."]
        #[serde(rename = "limits", default)]
        pub limits: ::std::option::Option<Vec<crate::schemas::QuotaLimit>>,
        #[doc = "List of `MetricRule` definitions, each one mapping a selected method to one\nor more metrics."]
        #[serde(rename = "metricRules", default)]
        pub metric_rules: ::std::option::Option<Vec<crate::schemas::MetricRule>>,
    }
    impl ::field_selector::FieldSelector for Quota {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct QuotaLimit {
        #[doc = "Default number of tokens that can be consumed during the specified\nduration. This is the number of tokens assigned when a client\napplication developer activates the service for his/her project.\n\nSpecifying a value of 0 will block all requests. This can be used if you\nare provisioning quota to selected consumers and blocking others.\nSimilarly, a value of -1 will indicate an unlimited quota. No other\nnegative values are allowed.\n\nUsed by group-based quotas only."]
        #[serde(rename = "defaultLimit", default)]
        #[serde(with = "crate::parsed_string")]
        pub default_limit: ::std::option::Option<i64>,
        #[doc = "Optional. User-visible, extended description for this quota limit.\nShould be used only when more context is needed to understand this limit\nthan provided by the limit's display name (see: `display_name`)."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "User-visible display name for this limit.\nOptional. If not set, the UI will provide a default display name based on\nthe quota configuration. This field can be used to override the default\ndisplay name generated from the configuration."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Duration of this limit in textual notation. Example: \"100s\", \"24h\", \"1d\".\nFor duration longer than a day, only multiple of days is supported. We\nsupport only \"100s\" and \"1d\" for now. Additional support will be added in\nthe future. \"0\" indicates indefinite duration.\n\nUsed by group-based quotas only."]
        #[serde(rename = "duration", default)]
        pub duration: ::std::option::Option<String>,
        #[doc = "Free tier value displayed in the Developers Console for this limit.\nThe free tier is the number of tokens that will be subtracted from the\nbilled amount when billing is enabled.\nThis field can only be set on a limit with duration \"1d\", in a billable\ngroup; it is invalid on any other limit. If this field is not set, it\ndefaults to 0, indicating that there is no free tier for this service.\n\nUsed by group-based quotas only."]
        #[serde(rename = "freeTier", default)]
        #[serde(with = "crate::parsed_string")]
        pub free_tier: ::std::option::Option<i64>,
        #[doc = "Maximum number of tokens that can be consumed during the specified\nduration. Client application developers can override the default limit up\nto this maximum. If specified, this value cannot be set to a value less\nthan the default limit. If not specified, it is set to the default limit.\n\nTo allow clients to apply overrides with no upper bound, set this to -1,\nindicating unlimited maximum quota.\n\nUsed by group-based quotas only."]
        #[serde(rename = "maxLimit", default)]
        #[serde(with = "crate::parsed_string")]
        pub max_limit: ::std::option::Option<i64>,
        #[doc = "The name of the metric this quota limit applies to. The quota limits with\nthe same metric will be checked together during runtime. The metric must be\ndefined within the service config."]
        #[serde(rename = "metric", default)]
        pub metric: ::std::option::Option<String>,
        #[doc = "Name of the quota limit.\n\nThe name must be provided, and it must be unique within the service. The\nname can only include alphanumeric characters as well as '-'.\n\nThe maximum length of the limit name is 64 characters."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Specify the unit of the quota limit. It uses the same syntax as\nMetric.unit. The supported unit kinds are determined by the quota\nbackend system.\n\nHere are some examples:\n\n* \"1/min/{project}\" for quota per minute per project.\n\nNote: the order of unit components is insignificant.\nThe \"1\" at the beginning is required to follow the metric unit syntax."]
        #[serde(rename = "unit", default)]
        pub unit: ::std::option::Option<String>,
        #[doc = "Tiered limit values. You must specify this as a key:value pair, with an\ninteger value that is the maximum number of requests allowed for the\nspecified unit. Currently only STANDARD is supported."]
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
    }
    impl ::field_selector::FieldSelector for QuotaLimit {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct QuotaOverride {
        #[doc = "If this map is nonempty, then this override applies only to specific values\nfor dimensions defined in the limit unit.\n\nFor example, an override on a limit with the unit 1/{project}/{region}\ncould contain an entry with the key \"region\" and the value \"us-east-1\";\nthe override is only applied to quota consumed in that region.\n\nThis map has the following restrictions:\n\n* Keys that are not defined in the limit's unit are not valid keys.\n  Any string appearing in {brackets} in the unit (besides {project} or\n  {user}) is a defined key.\n* \"project\" is not a valid key; the project is already specified in\n  the parent resource name.\n* \"user\" is not a valid key; the API does not support quota overrides\n  that apply only to a specific user.\n* If \"region\" appears as a key, its value must be a valid Cloud region.\n* If \"zone\" appears as a key, its value must be a valid Cloud zone.\n* If any valid key other than \"region\" or \"zone\" appears in the map, then\n  all valid keys other than \"region\" or \"zone\" must also appear in the map."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The resource name of the override.\nThis name is generated by the server when the override is created.\n\nExample names would be:\n`projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/adminOverrides/4a3f2c1d`\n`projects/123/services/compute.googleapis.com/consumerQuotaMetrics/compute.googleapis.com%2Fcpus/limits/%2Fproject%2Fregion/consumerOverrides/4a3f2c1d`\n\nThe resource name is intended to be opaque and should not be parsed for\nits component strings, since its representation could change in the future."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The overriding quota limit value.\nCan be any nonnegative integer, or -1 (unlimited quota)."]
        #[serde(rename = "overrideValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub override_value: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for QuotaOverride {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct SourceContext {
        #[doc = "The path-qualified name of the .proto file that contained the associated\nprotobuf element.  For example: `\"google/protobuf/source_context.proto\"`."]
        #[serde(rename = "fileName", default)]
        pub file_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SourceContext {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceInfo {
        #[doc = "All files used during config generation."]
        #[serde(rename = "sourceFiles", default)]
        pub source_files:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::field_selector::FieldSelector for SourceInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Status {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct SystemParameter {
        #[doc = "Define the HTTP header name to use for the parameter. It is case\ninsensitive."]
        #[serde(rename = "httpHeader", default)]
        pub http_header: ::std::option::Option<String>,
        #[doc = "Define the name of the parameter, such as \"api_key\" . It is case sensitive."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Define the URL query parameter name to use for the parameter. It is case\nsensitive."]
        #[serde(rename = "urlQueryParameter", default)]
        pub url_query_parameter: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SystemParameter {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct SystemParameterRule {
        #[doc = "Define parameters. Multiple names may be defined for a parameter.\nFor a given method call, only one of them should be used. If multiple\nnames are used the behavior is implementation-dependent.\nIf none of the specified names are present the behavior is\nparameter-dependent."]
        #[serde(rename = "parameters", default)]
        pub parameters: ::std::option::Option<Vec<crate::schemas::SystemParameter>>,
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all\nmethods in all APIs.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SystemParameterRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct SystemParameters {
        #[doc = "Define system parameters.\n\nThe parameters defined here will override the default parameters\nimplemented by the system. If this field is missing from the service\nconfig, default system parameters will be used. Default system parameters\nand names is implementation-dependent.\n\nExample: define api key for all methods\n\n````text\nsystem_parameters\n  rules:\n    - selector: \"*\"\n      parameters:\n        - name: api_key\n          url_query_parameter: api_key\n````\n\nExample: define 2 api key names for a specific method.\n\n````text\nsystem_parameters\n  rules:\n    - selector: \"/ListShelves\"\n      parameters:\n        - name: api_key\n          http_header: Api-Key1\n        - name: api_key\n          http_header: Api-Key2\n````\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::SystemParameterRule>>,
    }
    impl ::field_selector::FieldSelector for SystemParameters {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Type {
        #[doc = "The list of fields."]
        #[serde(rename = "fields", default)]
        pub fields: ::std::option::Option<Vec<crate::schemas::Field>>,
        #[doc = "The fully qualified message name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The list of types appearing in `oneof` definitions in this type."]
        #[serde(rename = "oneofs", default)]
        pub oneofs: ::std::option::Option<Vec<String>>,
        #[doc = "The protocol buffer options."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<Vec<crate::schemas::Option>>,
        #[doc = "The source context."]
        #[serde(rename = "sourceContext", default)]
        pub source_context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "The source syntax."]
        #[serde(rename = "syntax", default)]
        pub syntax: ::std::option::Option<crate::schemas::TypeSyntax>,
    }
    impl ::field_selector::FieldSelector for Type {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TypeSyntax {
        #[doc = "Syntax `proto2`."]
        SyntaxProto2,
        #[doc = "Syntax `proto3`."]
        SyntaxProto3,
    }
    impl TypeSyntax {
        pub fn as_str(self) -> &'static str {
            match self {
                TypeSyntax::SyntaxProto2 => "SYNTAX_PROTO2",
                TypeSyntax::SyntaxProto3 => "SYNTAX_PROTO3",
            }
        }
    }
    impl ::std::fmt::Display for TypeSyntax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TypeSyntax {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TypeSyntax {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYNTAX_PROTO2" => TypeSyntax::SyntaxProto2,
                "SYNTAX_PROTO3" => TypeSyntax::SyntaxProto3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TypeSyntax {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct Usage {
        #[doc = "The full resource name of a channel used for sending notifications to the\nservice producer.\n\nGoogle Service Management currently only supports\n[Google Cloud Pub/Sub](https://cloud.google.com/pubsub) as a notification\nchannel. To use Google Cloud Pub/Sub as the channel, this must be the name\nof a Cloud Pub/Sub topic that uses the Cloud Pub/Sub topic name format\ndocumented in https://cloud.google.com/pubsub/docs/overview."]
        #[serde(rename = "producerNotificationChannel", default)]
        pub producer_notification_channel: ::std::option::Option<String>,
        #[doc = "Requirements that must be satisfied before a consumer project can use the\nservice. Each requirement is of the form <service.name>/<requirement-id>;\nfor example 'serviceusage.googleapis.com/billing-enabled'."]
        #[serde(rename = "requirements", default)]
        pub requirements: ::std::option::Option<Vec<String>>,
        #[doc = "A list of usage rules that apply to individual API methods.\n\n**NOTE:** All service configuration rules follow \"last one wins\" order."]
        #[serde(rename = "rules", default)]
        pub rules: ::std::option::Option<Vec<crate::schemas::UsageRule>>,
    }
    impl ::field_selector::FieldSelector for Usage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    pub struct UsageRule {
        #[doc = "If true, the selected method allows unregistered calls, e.g. calls\nthat don't identify any user or application."]
        #[serde(rename = "allowUnregisteredCalls", default)]
        pub allow_unregistered_calls: ::std::option::Option<bool>,
        #[doc = "Selects the methods to which this rule applies. Use '*' to indicate all\nmethods in all APIs.\n\nRefer to selector for syntax details."]
        #[serde(rename = "selector", default)]
        pub selector: ::std::option::Option<String>,
        #[doc = "If true, the selected method should skip service control and the control\nplane features, such as quota and billing, will not be available.\nThis flag is used by Google Cloud Endpoints to bypass checks for internal\nmethods, such as service health check methods."]
        #[serde(rename = "skipServiceControl", default)]
        pub skip_service_control: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for UsageRule {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Alt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Xgafv {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions<A> {
        crate::resources::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the services resource"]
    pub fn services(&self) -> crate::resources::services::ServicesActions<A> {
        crate::resources::services::ServicesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod operations {
        pub mod params {}
        pub struct OperationsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> OperationsActions<'a, A> {
            #[doc = "Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn't support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`."]
            pub fn cancel(
                &self,
                request: crate::schemas::CancelOperationRequest,
                name: impl Into<String>,
            ) -> CancelRequestBuilder<A> {
                CancelRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
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
                    name: name.into(),
                }
            }
            #[doc = "Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn't support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`."]
            pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                    name: name.into(),
                }
            }
            #[doc = "Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                    name: name.into(),
                }
            }
            #[doc = "Lists operations that match the specified filter in the request. If the\nserver doesn't support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id."]
            pub fn list(&self) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                    filter: None,
                    name: None,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CancelRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::CancelOperationRequest,
            name: String,
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
        impl<'a, A: yup_oauth2::GetToken> CancelRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":cancel");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
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
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            filter: Option<String>,
            name: Option<String>,
            page_size: Option<i32>,
            page_token: Option<String>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "The standard list filter."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "The name of the operation's parent resource."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
                self
            }
            #[doc = "The standard list page size."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The standard list page token."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_operations<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "operations").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "operations")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_operations_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
                self.fields = Some(concat!("nextPageToken,", "operations").to_owned());
                crate::iter::PageItemIter::new(self, "operations")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_operations_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
                self.fields = Some(concat!("nextPageToken,", "operations", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "operations")
            }
            pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = T::field_selector();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
            }
            pub fn iter_standard(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse> {
                self.fields = Some("*".to_owned());
                crate::iter::PageIter::new(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::ListOperationsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListOperationsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/operations");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("name", &self.name)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
    }
    pub mod services {
        pub mod params {}
        pub struct ServicesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ServicesActions<'a, A> {
            #[doc = "Enable multiple services on a project. The operation is atomic: if enabling\nany service fails, then the entire batch fails, and no state changes occur.\nTo enable a single service, use the `EnableService` method instead."]
            pub fn batch_enable(
                &self,
                request: crate::schemas::BatchEnableServicesRequest,
                parent: impl Into<String>,
            ) -> BatchEnableRequestBuilder<A> {
                BatchEnableRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
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
                    parent: parent.into(),
                }
            }
            #[doc = "Disable a service so that it can no longer be used with a project.\nThis prevents unintended usage that may cause unexpected billing\ncharges or security leaks.\n\nIt is not valid to call the disable method on a service that is not\ncurrently enabled. Callers will receive a `FAILED_PRECONDITION` status if\nthe target service is not currently enabled."]
            pub fn disable(
                &self,
                request: crate::schemas::DisableServiceRequest,
                name: impl Into<String>,
            ) -> DisableRequestBuilder<A> {
                DisableRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
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
                    name: name.into(),
                }
            }
            #[doc = "Enable a service so that it can be used with a project."]
            pub fn enable(
                &self,
                request: crate::schemas::EnableServiceRequest,
                name: impl Into<String>,
            ) -> EnableRequestBuilder<A> {
                EnableRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
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
                    name: name.into(),
                }
            }
            #[doc = "Returns the service configuration and enabled state for a given service."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                    name: name.into(),
                }
            }
            #[doc = "List all services available to the specified project, and the current\nstate of those services with respect to the project. The list includes\nall public services, all services for which the calling user has the\n`servicemanagement.services.bind` permission, and all services that have\nalready been enabled on the project. The list can be filtered to\nonly include services in a specific state, for example to only include\nservices enabled on the project."]
            pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                    parent: parent.into(),
                    filter: None,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct BatchEnableRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::BatchEnableServicesRequest,
            parent: String,
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
        impl<'a, A: yup_oauth2::GetToken> BatchEnableRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/services:batchEnable");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct DisableRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::DisableServiceRequest,
            name: String,
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
        impl<'a, A: yup_oauth2::GetToken> DisableRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":disable");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct EnableRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::EnableServiceRequest,
            name: String,
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
        impl<'a, A: yup_oauth2::GetToken> EnableRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":enable");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            name: String,
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::GoogleApiServiceusageV1Service, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::GoogleApiServiceusageV1Service, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            parent: String,
            filter: Option<String>,
            page_size: Option<i32>,
            page_token: Option<String>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Only list services that conform to the given filter.\nThe allowed filter strings are `state:ENABLED` and `state:DISABLED`."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Requested size of the next page of data.\nRequested page size cannot exceed 200.\nIf not set, the default page size is 50."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Token identifying which result to start with, which is returned by a\nprevious list call."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_services<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "services").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "services")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_services_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleApiServiceusageV1Service>
            {
                self.fields = Some(concat!("nextPageToken,", "services").to_owned());
                crate::iter::PageItemIter::new(self, "services")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_services_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleApiServiceusageV1Service>
            {
                self.fields = Some(concat!("nextPageToken,", "services", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "services")
            }
            pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = T::field_selector();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
            }
            pub fn iter_standard(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListServicesResponse> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListServicesResponse> {
                self.fields = Some("*".to_owned());
                crate::iter::PageIter::new(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::ListServicesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListServicesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://serviceusage.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/services");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(format!(
                                    "no {} field found in iter response",
                                    self.items_field
                                )
                                .into()))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
}
