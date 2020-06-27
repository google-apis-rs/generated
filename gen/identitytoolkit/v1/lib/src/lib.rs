#![doc = "# Resources and Methods\n    * [accounts](resources/accounts/struct.AccountsActions.html)\n      * [*createAuthUri*](resources/accounts/struct.CreateAuthUriRequestBuilder.html), [*delete*](resources/accounts/struct.DeleteRequestBuilder.html), [*issueSamlResponse*](resources/accounts/struct.IssueSamlResponseRequestBuilder.html), [*lookup*](resources/accounts/struct.LookupRequestBuilder.html), [*resetPassword*](resources/accounts/struct.ResetPasswordRequestBuilder.html), [*sendOobCode*](resources/accounts/struct.SendOobCodeRequestBuilder.html), [*sendVerificationCode*](resources/accounts/struct.SendVerificationCodeRequestBuilder.html), [*signInWithCustomToken*](resources/accounts/struct.SignInWithCustomTokenRequestBuilder.html), [*signInWithEmailLink*](resources/accounts/struct.SignInWithEmailLinkRequestBuilder.html), [*signInWithGameCenter*](resources/accounts/struct.SignInWithGameCenterRequestBuilder.html), [*signInWithIdp*](resources/accounts/struct.SignInWithIdpRequestBuilder.html), [*signInWithPassword*](resources/accounts/struct.SignInWithPasswordRequestBuilder.html), [*signInWithPhoneNumber*](resources/accounts/struct.SignInWithPhoneNumberRequestBuilder.html), [*signUp*](resources/accounts/struct.SignUpRequestBuilder.html), [*update*](resources/accounts/struct.UpdateRequestBuilder.html), [*verifyIosClient*](resources/accounts/struct.VerifyIosClientRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*accounts_method*](resources/projects/struct.AccountsMethodRequestBuilder.html), [*createSessionCookie*](resources/projects/struct.CreateSessionCookieRequestBuilder.html), [*queryAccounts*](resources/projects/struct.QueryAccountsRequestBuilder.html)\n      * [accounts](resources/projects/accounts/struct.AccountsActions.html)\n        * [*batchCreate*](resources/projects/accounts/struct.BatchCreateRequestBuilder.html), [*batchDelete*](resources/projects/accounts/struct.BatchDeleteRequestBuilder.html), [*batchGet*](resources/projects/accounts/struct.BatchGetRequestBuilder.html), [*delete*](resources/projects/accounts/struct.DeleteRequestBuilder.html), [*lookup*](resources/projects/accounts/struct.LookupRequestBuilder.html), [*query*](resources/projects/accounts/struct.QueryRequestBuilder.html), [*sendOobCode*](resources/projects/accounts/struct.SendOobCodeRequestBuilder.html), [*update*](resources/projects/accounts/struct.UpdateRequestBuilder.html)\n      * [tenants](resources/projects/tenants/struct.TenantsActions.html)\n        * [*accounts_method*](resources/projects/tenants/struct.AccountsMethodRequestBuilder.html), [*createSessionCookie*](resources/projects/tenants/struct.CreateSessionCookieRequestBuilder.html)\n        * [accounts](resources/projects/tenants/accounts/struct.AccountsActions.html)\n          * [*batchCreate*](resources/projects/tenants/accounts/struct.BatchCreateRequestBuilder.html), [*batchDelete*](resources/projects/tenants/accounts/struct.BatchDeleteRequestBuilder.html), [*batchGet*](resources/projects/tenants/accounts/struct.BatchGetRequestBuilder.html), [*delete*](resources/projects/tenants/accounts/struct.DeleteRequestBuilder.html), [*lookup*](resources/projects/tenants/accounts/struct.LookupRequestBuilder.html), [*query*](resources/projects/tenants/accounts/struct.QueryRequestBuilder.html), [*sendOobCode*](resources/projects/tenants/accounts/struct.SendOobCodeRequestBuilder.html), [*update*](resources/projects/tenants/accounts/struct.UpdateRequestBuilder.html)\n    * [v_1](resources/v_1/struct.V1Actions.html)\n      * [*getProjects*](resources/v_1/struct.GetProjectsRequestBuilder.html), [*getPublicKeys*](resources/v_1/struct.GetPublicKeysRequestBuilder.html), [*getRecaptchaParams*](resources/v_1/struct.GetRecaptchaParamsRequestBuilder.html), [*getSessionCookiePublicKeys*](resources/v_1/struct.GetSessionCookiePublicKeysRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View and administer all your Firebase data and settings\n\n`https://www.googleapis.com/auth/firebase`"]
    pub const FIREBASE: &str = "https://www.googleapis.com/auth/firebase";
}
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
    pub struct GoogleCloudIdentitytoolkitV1BatchDeleteAccountsRequest {
        #[doc = "Whether to force deleting accounts that are not in disabled state.\nIf false, only disabled accounts will be deleted, and accounts that are not\ndisabled will be added to the `errors`."]
        #[serde(
            rename = "force",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub force: ::std::option::Option<bool>,
        #[doc = "Required. List of user IDs to be deleted."]
        #[serde(
            rename = "localIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_ids: ::std::option::Option<Vec<String>>,
        #[doc = "If the accounts belong to an Identity Platform tenant, the ID of the\ntenant. If the accounts belong to an default Identity Platform project, the\nfield is not needed."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1BatchDeleteAccountsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1BatchDeleteAccountsRequest
    {
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
    pub struct GoogleCloudIdentitytoolkitV1BatchDeleteAccountsResponse {
        #[doc = "Detailed error info for accounts that cannot be deleted."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteErrorInfo>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1BatchDeleteAccountsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1BatchDeleteAccountsResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1BatchDeleteErrorInfo {
        #[doc = "The index of the errored item in the original local_ids field."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The corresponding user ID."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "Detailed error message."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1BatchDeleteErrorInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1BatchDeleteErrorInfo {
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
    pub struct GoogleCloudIdentitytoolkitV1CreateAuthUriRequest {
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Used for the Google provider. The type of the authentication flow to be\nused. If present, this should be `CODE_FLOW` to specify the authorization\ncode flow. Otherwise, the default ID Token flow will be used."]
        #[serde(
            rename = "authFlowType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auth_flow_type: ::std::option::Option<String>,
        #[doc = "An opaque string used to maintain contextual information between the\nauthentication request and the callback from the IdP."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<String>,
        #[doc = "A valid URL for the IdP to redirect the user back to. The URL cannot\ncontain fragments or the reserved `state` query parameter."]
        #[serde(
            rename = "continueUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub continue_uri: ::std::option::Option<String>,
        #[doc = "Additional customized query parameters to be added to the authorization\nURI. The following parameters are reserved and cannot be added:\n`client_id`, `response_type`, `scope`, `redirect_uri`, `state`.\n\nFor the Microsoft provider, the Azure AD tenant to sign-in to can be\nspecified in the `tenant` custom parameter."]
        #[serde(
            rename = "customParameter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_parameter: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Used for the Google provider. The G Suite hosted domain of the user in\norder to restrict sign-in to users at that domain."]
        #[serde(
            rename = "hostedDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hosted_domain: ::std::option::Option<String>,
        #[doc = "The email identifier of the user account to fetch associated providers for.\nAt least one of the fields `identifier` and `provider_id` must be set.\n\nThe length of the email address should be less than 256 characters and in\nthe format of `name@domain.tld`. The email address should also match the\n[RFC 822](https://tools.ietf.org/html/rfc822) addr-spec production."]
        #[serde(
            rename = "identifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identifier: ::std::option::Option<String>,
        #[serde(
            rename = "oauthConsumerKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_consumer_key: ::std::option::Option<String>,
        #[doc = "Additional space-delimited OAuth 2.0 scopes specifying the scope of the\nauthentication request with the IdP. Used for OAuth 2.0 IdPs.\n\nFor the Google provider, the authorization code flow will be used if this\nfield is set."]
        #[serde(
            rename = "oauthScope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_scope: ::std::option::Option<String>,
        #[serde(
            rename = "openidRealm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub openid_realm: ::std::option::Option<String>,
        #[serde(
            rename = "otaApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ota_app: ::std::option::Option<String>,
        #[doc = "The provider ID of the IdP for the user to sign in with. This should be a\nprovider ID enabled for sign-in, which is either from the list of\n[default supported\nIdPs](https://cloud.google.com/identity-platform/docs/reference/rest/v2/defaultSupportedIdps/list),\nor of the format `oidc.*` or `saml.*`. Some examples are `google.com`,\n`facebook.com`, `oidc.testapp`, and `saml.testapp`. At least one of the\nfields `identifier` and `provider_id` must be set."]
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "A session ID that can be verified against in SignInWithIdp to prevent\nsession fixation attacks. If absent, a random string will be generated and\nreturned as the session ID."]
        #[serde(
            rename = "sessionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_id: ::std::option::Option<String>,
        #[doc = "The ID of the Identity Platform tenant to create an authorization URI or\nlookup an email identifier for. If not set, the operation will be performed\nin the default Identity Platform instance in the project."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1CreateAuthUriRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1CreateAuthUriRequest {
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
    pub struct GoogleCloudIdentitytoolkitV1CreateAuthUriResponse {
        #[serde(
            rename = "allProviders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_providers: ::std::option::Option<Vec<String>>,
        #[doc = "The authorization URI for the requested provider. Present only when a\nprovider ID is set in the request."]
        #[serde(
            rename = "authUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auth_uri: ::std::option::Option<String>,
        #[doc = "Whether a CAPTCHA is needed because there have been too many failed login\nattempts by the user. Present only when a registered email identifier is\nset in the request."]
        #[serde(
            rename = "captchaRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_required: ::std::option::Option<bool>,
        #[doc = "Whether the user has previously signed in with the provider ID in the\nrequest. Present only when a registered email identifier is set in the\nrequest."]
        #[serde(
            rename = "forExistingProvider",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub for_existing_provider: ::std::option::Option<bool>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The provider ID from the request, if provided."]
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "Whether the email identifier represents an existing account. Present only\nwhen an email identifier is set in the request."]
        #[serde(
            rename = "registered",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub registered: ::std::option::Option<bool>,
        #[doc = "The session ID from the request, or a random string generated by\nCreateAuthUri if absent. It is used to prevent session fixation attacks."]
        #[serde(
            rename = "sessionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_id: ::std::option::Option<String>,
        #[doc = "The list of sign-in methods that the user has previously used. Each element\nis one of `password`, `emailLink`, or the provider ID of an IdP. Present\nonly when a registered email identifier is set in the request."]
        #[serde(
            rename = "signinMethods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signin_methods: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1CreateAuthUriResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1CreateAuthUriResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1CreateSessionCookieRequest {
        #[doc = "Required. A valid Identity Platform ID token."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The tenant ID of the Identity Platform tenant the account belongs to."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "The number of seconds until the session cookie expires. Specify a duration\nin seconds, between five minutes and fourteen days, inclusively."]
        #[serde(
            rename = "validDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub valid_duration: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1CreateSessionCookieRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1CreateSessionCookieRequest
    {
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
    pub struct GoogleCloudIdentitytoolkitV1CreateSessionCookieResponse {
        #[doc = "The session cookie that has been created from the Identity Platform\nID token specified in the request. It is in the form of a JSON Web Token\n(JWT). Always present."]
        #[serde(
            rename = "sessionCookie",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_cookie: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1CreateSessionCookieResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1CreateSessionCookieResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1DeleteAccountRequest {
        #[serde(
            rename = "delegatedProjectNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The Identity Platform ID token of the account to delete. Require to be\nspecified for requests from end users that lack Google OAuth 2.0\ncredential. Authenticated requests bearing a Google OAuth2 credential with\nproper permissions may pass local_id to specify the account to delete\nalternatively."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The ID of user account to delete. Specifying this field requires a Google\nOAuth 2.0 credential with proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control).\nRequests from users lacking the credential should pass an ID token instead."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The ID of the project which the account belongs to. Should only be\nspecified in authenticated requests that specify local_id of an account."]
        #[serde(
            rename = "targetProjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_project_id: ::std::option::Option<String>,
        #[doc = "The ID of the tenant that the account belongs to, if applicable. Only\nrequire to be specified for authenticated requests bearing a Google\nOAuth 2.0 credential that specify local_id of an account that belongs to an\nIdentity Platform tenant."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1DeleteAccountRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1DeleteAccountRequest {
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
    pub struct GoogleCloudIdentitytoolkitV1DeleteAccountResponse {
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1DeleteAccountResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1DeleteAccountResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudIdentitytoolkitV1DownloadAccountResponse {
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "If there are more accounts to be downloaded, a token that can be passed\nback to DownloadAccount to get more accounts. Otherwise, this is blank."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "All accounts belonging to the project/tenant limited by max_results in the\nrequest."]
        #[serde(
            rename = "users",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub users: ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1UserInfo>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1DownloadAccountResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1DownloadAccountResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1EmailTemplate {
        #[doc = "Email body"]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<String>,
        #[doc = "Whether the body or subject of the email is customized."]
        #[serde(
            rename = "customized",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customized: ::std::option::Option<bool>,
        #[doc = "Whether the template is disabled. If true, a default template will be used."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "Email body format"]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1EmailTemplateFormat>,
        #[doc = "From address of the email"]
        #[serde(
            rename = "from",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub from: ::std::option::Option<String>,
        #[doc = "From display name"]
        #[serde(
            rename = "fromDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub from_display_name: ::std::option::Option<String>,
        #[doc = "Local part of From address"]
        #[serde(
            rename = "fromLocalPart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub from_local_part: ::std::option::Option<String>,
        #[doc = "Value is in III language code format (e.g. \"zh-CN\", \"es\"). Both ‘-’  and\n‘_’ separators are accepted."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "Reply-to address"]
        #[serde(
            rename = "replyTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reply_to: ::std::option::Option<String>,
        #[doc = "Subject of the email"]
        #[serde(
            rename = "subject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1EmailTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1EmailTemplate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        #[doc = "Default value. Do not use."]
        EmailBodyFormatUnspecified,
        #[doc = "Email body is in HTML format."]
        Html,
        #[doc = "Email body is in plain text format."]
        Plaintext,
    }
    impl GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudIdentitytoolkitV1EmailTemplateFormat::EmailBodyFormatUnspecified => {
                    "EMAIL_BODY_FORMAT_UNSPECIFIED"
                }
                GoogleCloudIdentitytoolkitV1EmailTemplateFormat::Html => "HTML",
                GoogleCloudIdentitytoolkitV1EmailTemplateFormat::Plaintext => "PLAINTEXT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudIdentitytoolkitV1EmailTemplateFormat, ()> {
            Ok(match s {
                "EMAIL_BODY_FORMAT_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1EmailTemplateFormat::EmailBodyFormatUnspecified
                }
                "HTML" => GoogleCloudIdentitytoolkitV1EmailTemplateFormat::Html,
                "PLAINTEXT" => GoogleCloudIdentitytoolkitV1EmailTemplateFormat::Plaintext,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMAIL_BODY_FORMAT_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1EmailTemplateFormat::EmailBodyFormatUnspecified
                }
                "HTML" => GoogleCloudIdentitytoolkitV1EmailTemplateFormat::Html,
                "PLAINTEXT" => GoogleCloudIdentitytoolkitV1EmailTemplateFormat::Plaintext,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1EmailTemplateFormat {
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
    pub struct GoogleCloudIdentitytoolkitV1ErrorInfo {
        #[doc = "The index of the item, range is [0, request.size - 1]"]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "Detailed error message"]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1ErrorInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1ErrorInfo {
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
    pub struct GoogleCloudIdentitytoolkitV1FederatedUserIdentifier {
        #[doc = "The ID of supported identity providers. This should be a provider ID\nenabled for sign-in, which is either from the list of [default supported\nIdPs](https://cloud.google.com/identity-platform/docs/reference/rest/v2/defaultSupportedIdps/list),\nor of the format `oidc.*` or `saml.*`. Some examples are `google.com`,\n`facebook.com`, `oidc.testapp`, and `saml.testapp`."]
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "The user ID of the account at the third-party Identity Provider specified\nby `provider_id`."]
        #[serde(
            rename = "rawId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1FederatedUserIdentifier
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1FederatedUserIdentifier {
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
    pub struct GoogleCloudIdentitytoolkitV1GetAccountInfoRequest {
        #[serde(
            rename = "delegatedProjectNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The email address of one or more accounts to fetch. The length of email\nshould be less than 256 characters and in the format of `name@domain.tld`.\nThe email should also match the [RFC\n822](https://tools.ietf.org/html/rfc822) addr-spec production. Should only\nbe specified by authenticated requests from a developer."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<Vec<String>>,
        #[doc = "The federated user identifier of one or more accounts to fetch. Should only\nbe specified by authenticated requests bearing a Google OAuth 2.0\ncredential with proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "federatedUserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub federated_user_id: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudIdentitytoolkitV1FederatedUserIdentifier>,
        >,
        #[doc = "The Identity Platform ID token of the account to fetch. Require to be\nspecified for requests from end users."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The initial email of one or more accounts to fetch. The length of email\nshould be less than 256 characters and in the format of `name@domain.tld`.\nThe email should also match the [RFC\n822](https://tools.ietf.org/html/rfc822) addr-spec production. Should only\nbe specified by authenticated requests from a developer."]
        #[serde(
            rename = "initialEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub initial_email: ::std::option::Option<Vec<String>>,
        #[doc = "The ID of one or more accounts to fetch. Should only be specified by\nauthenticated requests bearing a Google OAuth 2.0 credential with proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<Vec<String>>,
        #[doc = "The phone number of one or more accounts to fetch. Should only be specified\nby authenticated requests from a developer and should be in E.164 format,\nfor example, +15555555555."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<Vec<String>>,
        #[doc = "The ID of the Google Cloud project that the account or the Identity\nPlatform tenant specified by `tenant_id` belongs to. Should only be\nspecified by authenticated requests bearing a Google OAuth 2.0 credential\nwith proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "targetProjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_project_id: ::std::option::Option<String>,
        #[doc = "The ID of the tenant that the account belongs to. Should only\nbe specified by authenticated requests from a developer."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1GetAccountInfoRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1GetAccountInfoRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudIdentitytoolkitV1GetAccountInfoResponse {
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The information of specific user account(s) matching the parameters in the\nrequest."]
        #[serde(
            rename = "users",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub users: ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1UserInfo>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1GetAccountInfoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1GetAccountInfoResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1GetOobCodeRequest {
        #[doc = "If an associated android app can handle the OOB code, whether or not to\ninstall the android app on the device where the link is opened if the app\nis not already installed."]
        #[serde(
            rename = "androidInstallApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_install_app: ::std::option::Option<bool>,
        #[doc = "If an associated android app can handle the OOB code, the minimum version\nof the app. If the version on the device is lower than this version then\nthe user is taken to Google Play Store to upgrade the app."]
        #[serde(
            rename = "androidMinimumVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_minimum_version: ::std::option::Option<String>,
        #[doc = "If an associated android app can handle the OOB code, the Android package\nname of the android app that will handle the callback when this OOB code is\nused. This will allow the correct app to open if it is already installed,\nor allow Google Play Store to open to the correct app if it is not yet\ninstalled."]
        #[serde(
            rename = "androidPackageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_package_name: ::std::option::Option<String>,
        #[doc = "When set to true, the OOB code link will be be sent as a Universal Link or\nan Android App Link and will be opened by the corresponding app if\ninstalled. If not set, or set to false, the OOB code will be sent to the\nweb widget first and then on continue will redirect to the app if\ninstalled."]
        #[serde(
            rename = "canHandleCodeInApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_handle_code_in_app: ::std::option::Option<bool>,
        #[doc = "For a PASSWORD_RESET request, a reCaptcha response is required when the\nsystem detects possible abuse activity. In those cases, this is the\nresponse from the reCaptcha challenge used to verify the caller."]
        #[serde(
            rename = "captchaResp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_resp: ::std::option::Option<String>,
        #[serde(
            rename = "challenge",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub challenge: ::std::option::Option<String>,
        #[doc = "The Url to continue after user clicks the link sent in email. This is the\nurl that will allow the web widget to handle the OOB code."]
        #[serde(
            rename = "continueUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub continue_url: ::std::option::Option<String>,
        #[doc = "In order to ensure that the url used can be easily opened up in iOS or\nandroid, we create a [Firebase Dynamic\nLink](https://firebase.google.com/docs/dynamic-links). Most Identity\nPlatform projects will only have one Dynamic Link domain enabled, and can\nleave this field blank.  This field contains a specified Dynamic Link\ndomain for projects that have multiple enabled."]
        #[serde(
            rename = "dynamicLinkDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_link_domain: ::std::option::Option<String>,
        #[doc = "The account's email address to send the OOB code to, and generally the\nemail address of the account that needs to be updated. Required for\nPASSWORD_RESET, EMAIL_SIGNIN, and VERIFY_EMAIL."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "If an associated iOS app can handle the OOB code, the App Store id of this\napp. This will allow App Store to open to the correct app if the app is not\nyet installed."]
        #[serde(
            rename = "iOSAppStoreId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub i_os_app_store_id: ::std::option::Option<String>,
        #[doc = "If an associated iOS app can handle the OOB code, the iOS bundle id of\nthis app. This will allow the correct app to open if it is already\ninstalled."]
        #[serde(
            rename = "iOSBundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub i_os_bundle_id: ::std::option::Option<String>,
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "newEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_email: ::std::option::Option<String>,
        #[doc = "Required. The type of out-of-band (OOB) code to send. Depending on this value, other\nfields in this request will be required and/or have different meanings.\nThere are 3 different OOB codes that can be sent:\n\n* PASSWORD_RESET\n* EMAIL_SIGNIN\n* VERIFY_EMAIL"]
        #[serde(
            rename = "requestType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_type: ::std::option::Option<
            crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType,
        >,
        #[doc = "Whether the confirmation link containing the OOB code should be returned in\nthe response (no email is sent). Used when a developer wants to construct\nthe email template and send it on their own. By default this is false; to\nspecify this field, and to set it to true, it requires a Google OAuth 2.0\ncredential with proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)"]
        #[serde(
            rename = "returnOobLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_oob_link: ::std::option::Option<bool>,
        #[doc = "The Project ID of the Identity Platform project which the account belongs\nto. To specify this field, it requires a Google OAuth 2.0 credential with\nproper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "targetProjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_project_id: ::std::option::Option<String>,
        #[doc = "The tenant ID of the Identity Platform tenant the account belongs to."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "The IP address of the caller. Required only for PASSWORD_RESET requests."]
        #[serde(
            rename = "userIp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_ip: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1GetOobCodeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1GetOobCodeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType {
        #[doc = "sign in with email only"]
        EmailSignin,
        NewEmailAccept,
        OldEmailAgree,
        #[doc = "Oob code type is not specified."]
        OobReqTypeUnspecified,
        #[doc = "reset password"]
        PasswordReset,
        RecoverEmail,
        RevertSecondFactorAddition,
        VerifyAndChangeEmail,
        #[doc = "verify the account's email address by sending an email"]
        VerifyEmail,
    }
    impl GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: EmailSignin => "EMAIL_SIGNIN" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: NewEmailAccept => "NEW_EMAIL_ACCEPT" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: OldEmailAgree => "OLD_EMAIL_AGREE" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: OobReqTypeUnspecified => "OOB_REQ_TYPE_UNSPECIFIED" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: PasswordReset => "PASSWORD_RESET" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: RecoverEmail => "RECOVER_EMAIL" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: RevertSecondFactorAddition => "REVERT_SECOND_FACTOR_ADDITION" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: VerifyAndChangeEmail => "VERIFY_AND_CHANGE_EMAIL" , GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: VerifyEmail => "VERIFY_EMAIL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType, ()>
        {
            Ok ( match s { "EMAIL_SIGNIN" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: EmailSignin , "NEW_EMAIL_ACCEPT" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: NewEmailAccept , "OLD_EMAIL_AGREE" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: OldEmailAgree , "OOB_REQ_TYPE_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: OobReqTypeUnspecified , "PASSWORD_RESET" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: PasswordReset , "RECOVER_EMAIL" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: RecoverEmail , "REVERT_SECOND_FACTOR_ADDITION" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: RevertSecondFactorAddition , "VERIFY_AND_CHANGE_EMAIL" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: VerifyAndChangeEmail , "VERIFY_EMAIL" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: VerifyEmail , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "EMAIL_SIGNIN" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: EmailSignin , "NEW_EMAIL_ACCEPT" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: NewEmailAccept , "OLD_EMAIL_AGREE" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: OldEmailAgree , "OOB_REQ_TYPE_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: OobReqTypeUnspecified , "PASSWORD_RESET" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: PasswordReset , "RECOVER_EMAIL" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: RecoverEmail , "REVERT_SECOND_FACTOR_ADDITION" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: RevertSecondFactorAddition , "VERIFY_AND_CHANGE_EMAIL" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: VerifyAndChangeEmail , "VERIFY_EMAIL" => GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType :: VerifyEmail , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1GetOobCodeRequestRequestType
    {
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
    pub struct GoogleCloudIdentitytoolkitV1GetOobCodeResponse {
        #[doc = "If return_oob_link is false in the request, the email address the\nverification was sent to."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "If return_oob_link is true in the request, the OOB code to send."]
        #[serde(
            rename = "oobCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oob_code: ::std::option::Option<String>,
        #[doc = "If return_oob_link is true in the request, the OOB link to be sent to the\nuser. This returns the constructed link including\n[Firebase Dynamic Link](https://firebase.google.com/docs/dynamic-links)\nrelated parameters."]
        #[serde(
            rename = "oobLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oob_link: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1GetOobCodeResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1GetOobCodeResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1GetProjectConfigResponse {
        #[doc = "Whether to allow password account sign up. This field is only returned for\nauthenticated calls from a developer."]
        #[serde(
            rename = "allowPasswordUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_password_user: ::std::option::Option<bool>,
        #[doc = "Google Cloud API key. This field is only returned for authenticated calls\nfrom a developer."]
        #[serde(
            rename = "apiKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_key: ::std::option::Option<String>,
        #[doc = "Authorized domains for widget redirect."]
        #[serde(
            rename = "authorizedDomains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authorized_domains: ::std::option::Option<Vec<String>>,
        #[doc = "Email template for change email. This field is only returned for\nauthenticated calls from a developer."]
        #[serde(
            rename = "changeEmailTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub change_email_template:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1EmailTemplate>,
        #[doc = "The Firebase Dynamic Links domain used to construct links for redirects to\nnative apps."]
        #[serde(
            rename = "dynamicLinksDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_links_domain: ::std::option::Option<String>,
        #[doc = "Whether anonymous user is enabled. This field is only returned for\nauthenticated calls from a developer."]
        #[serde(
            rename = "enableAnonymousUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_anonymous_user: ::std::option::Option<bool>,
        #[doc = "OAuth2 provider config. This field is only returned for authenticated calls\nfrom a developer."]
        #[serde(
            rename = "idpConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub idp_config:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1IdpConfig>>,
        #[doc = "Reset password email template for legacy Firebase V1 app. This field is\nonly returned for authenticated calls from a developer."]
        #[serde(
            rename = "legacyResetPasswordTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy_reset_password_template:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1EmailTemplate>,
        #[doc = "The project id of the retrieved configuration."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Email template for reset password. This field is only returned for\nauthenticated calls from a developer."]
        #[serde(
            rename = "resetPasswordTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reset_password_template:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1EmailTemplate>,
        #[doc = "Email template for reverting second factor additions. This field is only\nreturned for authenticated calls from a developer."]
        #[serde(
            rename = "revertSecondFactorAdditionTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revert_second_factor_addition_template:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1EmailTemplate>,
        #[doc = "Whether to use email sending. This field is only returned for authenticated\ncalls from a developer."]
        #[serde(
            rename = "useEmailSending",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_email_sending: ::std::option::Option<bool>,
        #[doc = "Email template for verify email. This field is only returned for\nauthenticated calls from a developer."]
        #[serde(
            rename = "verifyEmailTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verify_email_template:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1EmailTemplate>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1GetProjectConfigResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1GetProjectConfigResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1GetRecaptchaParamResponse {
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The reCAPTCHA v2 site key used to invoke the reCAPTCHA service. Always\npresent."]
        #[serde(
            rename = "recaptchaSiteKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recaptcha_site_key: ::std::option::Option<String>,
        #[serde(
            rename = "recaptchaStoken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recaptcha_stoken: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1GetRecaptchaParamResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1GetRecaptchaParamResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1GetSessionCookiePublicKeysResponse {
        #[doc = "Public keys of the session cookie signer, formatted as\n[JSON Web Keys (JWK)](https://tools.ietf.org/html/rfc7517)."]
        #[serde(
            rename = "keys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keys: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudIdentitytoolkitV1OpenIdConnectKey>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1GetSessionCookiePublicKeysResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1GetSessionCookiePublicKeysResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1IdpConfig {
        #[doc = "OAuth2 client ID."]
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_id: ::std::option::Option<String>,
        #[doc = "True if allows the user to sign in with the provider."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
        #[doc = "Percent of users who will be prompted/redirected federated login for this\nIdP"]
        #[serde(
            rename = "experimentPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub experiment_percent: ::std::option::Option<i32>,
        #[doc = "Name of the identity provider."]
        #[serde(
            rename = "provider",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1IdpConfigProvider>,
        #[doc = "OAuth2 client secret."]
        #[serde(
            rename = "secret",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secret: ::std::option::Option<String>,
        #[doc = "Whitelisted client IDs for audience check."]
        #[serde(
            rename = "whitelistedAudiences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub whitelisted_audiences: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1IdpConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1IdpConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        #[doc = "AOL as identity provider."]
        Aol,
        #[doc = "Facebook as identity provider."]
        Facebook,
        #[doc = "GitHub as identity provider."]
        Github,
        #[doc = "Google as identity provider."]
        Google,
        #[doc = "Google Play Games as identity provider."]
        GooglePlayGames,
        #[doc = "iOS Game Center as identity provider."]
        IosGameCenter,
        #[doc = "LinkedIn as identity provider."]
        Linkedin,
        #[doc = "Microsoft Live as identity provider."]
        Mslive,
        #[doc = "PayPal as identity provider."]
        Paypal,
        ProviderUnspecified,
        #[doc = "Twitter as identity provider."]
        Twitter,
        #[doc = "Yahoo as identity provider."]
        Yahoo,
    }
    impl GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Aol => "AOL",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Facebook => "FACEBOOK",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Github => "GITHUB",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Google => "GOOGLE",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::GooglePlayGames => {
                    "GOOGLE_PLAY_GAMES"
                }
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::IosGameCenter => "IOS_GAME_CENTER",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Linkedin => "LINKEDIN",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Mslive => "MSLIVE",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Paypal => "PAYPAL",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::ProviderUnspecified => {
                    "PROVIDER_UNSPECIFIED"
                }
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Twitter => "TWITTER",
                GoogleCloudIdentitytoolkitV1IdpConfigProvider::Yahoo => "YAHOO",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudIdentitytoolkitV1IdpConfigProvider, ()> {
            Ok(match s {
                "AOL" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Aol,
                "FACEBOOK" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Facebook,
                "GITHUB" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Github,
                "GOOGLE" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Google,
                "GOOGLE_PLAY_GAMES" => {
                    GoogleCloudIdentitytoolkitV1IdpConfigProvider::GooglePlayGames
                }
                "IOS_GAME_CENTER" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::IosGameCenter,
                "LINKEDIN" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Linkedin,
                "MSLIVE" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Mslive,
                "PAYPAL" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Paypal,
                "PROVIDER_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1IdpConfigProvider::ProviderUnspecified
                }
                "TWITTER" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Twitter,
                "YAHOO" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Yahoo,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AOL" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Aol,
                "FACEBOOK" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Facebook,
                "GITHUB" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Github,
                "GOOGLE" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Google,
                "GOOGLE_PLAY_GAMES" => {
                    GoogleCloudIdentitytoolkitV1IdpConfigProvider::GooglePlayGames
                }
                "IOS_GAME_CENTER" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::IosGameCenter,
                "LINKEDIN" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Linkedin,
                "MSLIVE" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Mslive,
                "PAYPAL" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Paypal,
                "PROVIDER_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1IdpConfigProvider::ProviderUnspecified
                }
                "TWITTER" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Twitter,
                "YAHOO" => GoogleCloudIdentitytoolkitV1IdpConfigProvider::Yahoo,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1IdpConfigProvider {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1IdpConfigProvider {
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
    pub struct GoogleCloudIdentitytoolkitV1IssueSamlResponseRequest {
        #[doc = "The Identity Platform ID token. It will be verified and then converted to a\nnew SAMLResponse."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Relying Party identifier, which is the audience of issued SAMLResponse."]
        #[serde(
            rename = "rpId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rp_id: ::std::option::Option<String>,
        #[doc = "SAML app entity id specified in Google Admin Console for each app.\nIf developers wants to redirect to third party app rather than GSuite apps,\nprobably they need this. When it's used, we'll return a RelayState\nincluding a SAMLRequest which can be used to trigger a SP-initiated SAML\nflow to redirect to the real app."]
        #[serde(
            rename = "samlAppEntityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saml_app_entity_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1IssueSamlResponseRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1IssueSamlResponseRequest {
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
    pub struct GoogleCloudIdentitytoolkitV1IssueSamlResponseResponse {
        #[doc = "The ACS endpoint which consumes the returned SAMLResponse."]
        #[serde(
            rename = "acsEndpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acs_endpoint: ::std::option::Option<String>,
        #[doc = "Email of the user."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "First name of the user."]
        #[serde(
            rename = "firstName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_name: ::std::option::Option<String>,
        #[doc = "Whether the logged in user was created by this request."]
        #[serde(
            rename = "isNewUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_new_user: ::std::option::Option<bool>,
        #[doc = "Last name of the user."]
        #[serde(
            rename = "lastName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_name: ::std::option::Option<String>,
        #[doc = "Generated RelayState."]
        #[serde(
            rename = "relayState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relay_state: ::std::option::Option<String>,
        #[doc = "Signed SAMLResponse created for the Relying Party."]
        #[serde(
            rename = "samlResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saml_response: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1IssueSamlResponseResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1IssueSamlResponseResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1MfaEnrollment {
        #[doc = "Display name for this mfa option e.g. \"corp cell phone\"."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Timestamp when the account enrolled this second factor."]
        #[serde(
            rename = "enrolledAt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enrolled_at: ::std::option::Option<String>,
        #[doc = "ID of this MFA option."]
        #[serde(
            rename = "mfaEnrollmentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_enrollment_id: ::std::option::Option<String>,
        #[doc = "Normally this will show the phone number associated with this enrollment.\nIn some situations, such as after a first factor sign in, it will only\nshow the obfuscated version of the associated phone number."]
        #[serde(
            rename = "phoneInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_info: ::std::option::Option<String>,
        #[doc = "Output only. Unobfuscated phone_info."]
        #[serde(
            rename = "unobfuscatedPhoneInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unobfuscated_phone_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1MfaEnrollment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1MfaEnrollment {
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
    pub struct GoogleCloudIdentitytoolkitV1MfaFactor {
        #[doc = "Display name for this mfa option e.g. \"corp cell phone\"."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Phone number to receive OTP for MFA."]
        #[serde(
            rename = "phoneInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1MfaFactor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1MfaFactor {
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
    pub struct GoogleCloudIdentitytoolkitV1MfaInfo {
        #[doc = "The second factors the user has enrolled."]
        #[serde(
            rename = "enrollments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enrollments:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1MfaEnrollment>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1MfaInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1MfaInfo {
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
    pub struct GoogleCloudIdentitytoolkitV1OpenIdConnectKey {
        #[doc = "Signature algorithm."]
        #[serde(
            rename = "alg",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alg: ::std::option::Option<String>,
        #[doc = "Exponent for the RSA public key, it is represented as the base64url\nencoding of the value's big endian representation."]
        #[serde(
            rename = "e",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub e: ::std::option::Option<String>,
        #[doc = "Unique string to identify this key."]
        #[serde(
            rename = "kid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kid: ::std::option::Option<String>,
        #[doc = "Key type."]
        #[serde(
            rename = "kty",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kty: ::std::option::Option<String>,
        #[doc = "Modulus for the RSA public key, it is represented as the base64url\nencoding of the value's big endian representation."]
        #[serde(
            rename = "n",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub n: ::std::option::Option<String>,
        #[doc = "Key use."]
        #[serde(
            rename = "use",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#use: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1OpenIdConnectKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1OpenIdConnectKey {
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
    pub struct GoogleCloudIdentitytoolkitV1ProviderUserInfo {
        #[doc = "The user's display name at the Identity Provider."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The user's email address at the Identity Provider."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The user's identifier at the Identity Provider."]
        #[serde(
            rename = "federatedId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub federated_id: ::std::option::Option<String>,
        #[doc = "The user's phone number at the Identity Provider."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The user's profile photo URL at the Identity Provider."]
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The ID of the Identity Provider."]
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "The user's raw identifier directly returned from Identity Provider."]
        #[serde(
            rename = "rawId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_id: ::std::option::Option<String>,
        #[doc = "The user's screen_name at Twitter or login name at Github."]
        #[serde(
            rename = "screenName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1ProviderUserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1ProviderUserInfo {
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
    pub struct GoogleCloudIdentitytoolkitV1QueryUserInfoRequest {
        #[doc = "Query conditions used to filter results. If more than one is passed, only\nthe first SqlExpression is evaluated."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1SqlExpression>>,
        #[doc = "The maximum number of accounts to return with an upper limit of **500**.\nDefaults to *500*.\nOnly valid when `return_user_info` is set to `true`."]
        #[serde(
            rename = "limit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "The number of accounts to skip from the beginning of matching records.\nOnly valid when `return_user_info` is set to `true`."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub offset: ::std::option::Option<i64>,
        #[doc = "The order for sorting query result.\nDefaults to **ascending** order.\nOnly valid when `return_user_info` is set to `true`."]
        #[serde(
            rename = "order",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order: ::std::option::Option<
            crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder,
        >,
        #[doc = "If `true`, this request will return the accounts matching the query. If\n`false`, only the **count** of accounts matching the query will be\nreturned. Defaults to `true`."]
        #[serde(
            rename = "returnUserInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_user_info: ::std::option::Option<bool>,
        #[doc = "The field to use for sorting user accounts.\nDefaults to `USER_ID`.\nNote: when `phone_number` is specified in `expression`, the result ignores\nthe sorting.\nOnly valid when `return_user_info` is set to `true`."]
        #[serde(
            rename = "sortBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_by: ::std::option::Option<
            crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy,
        >,
        #[doc = "The ID of the tenant to which the result is scoped."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1QueryUserInfoRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1QueryUserInfoRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder {
        #[doc = "Sort on ascending order."]
        Asc,
        #[doc = "Sort on descending order."]
        Desc,
        #[doc = "Order is not specified."]
        OrderUnspecified,
    }
    impl GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::Asc => "ASC",
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::Desc => "DESC",
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::OrderUnspecified => {
                    "ORDER_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder, ()>
        {
            Ok(match s {
                "ASC" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::Asc,
                "DESC" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::Desc,
                "ORDER_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::OrderUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASC" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::Asc,
                "DESC" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::Desc,
                "ORDER_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder::OrderUnspecified
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
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestOrder
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy {
        #[doc = "Sort result by createdAt."]
        CreatedAt,
        #[doc = "Sort result by lastLoginAt."]
        LastLoginAt,
        #[doc = "Sort result by name."]
        Name,
        #[doc = "Sort field is not specified."]
        SortByFieldUnspecified,
        #[doc = "Sort result by userEmail."]
        UserEmail,
        #[doc = "Sort result by userId."]
        UserId,
    }
    impl GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::CreatedAt => "CREATED_AT",
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::LastLoginAt => {
                    "LAST_LOGIN_AT"
                }
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::Name => "NAME",
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::SortByFieldUnspecified => {
                    "SORT_BY_FIELD_UNSPECIFIED"
                }
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::UserEmail => "USER_EMAIL",
                GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::UserId => "USER_ID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy, ()>
        {
            Ok(match s {
                "CREATED_AT" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::CreatedAt,
                "LAST_LOGIN_AT" => {
                    GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::LastLoginAt
                }
                "NAME" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::Name,
                "SORT_BY_FIELD_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::SortByFieldUnspecified
                }
                "USER_EMAIL" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::UserEmail,
                "USER_ID" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::UserId,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATED_AT" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::CreatedAt,
                "LAST_LOGIN_AT" => {
                    GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::LastLoginAt
                }
                "NAME" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::Name,
                "SORT_BY_FIELD_UNSPECIFIED" => {
                    GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::SortByFieldUnspecified
                }
                "USER_EMAIL" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::UserEmail,
                "USER_ID" => GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy::UserId,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1QueryUserInfoRequestSortBy
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudIdentitytoolkitV1QueryUserInfoResponse {
        #[doc = "If `return_user_info` in the request is true, this is the number of\nreturned accounts in this message. Otherwise, this is the total number of\naccounts matching the query."]
        #[serde(
            rename = "recordsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub records_count: ::std::option::Option<i64>,
        #[doc = "If `return_user_info` in the request is true, this is the accounts\nmatching the query."]
        #[serde(
            rename = "userInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_info:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1UserInfo>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1QueryUserInfoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1QueryUserInfoResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1ResetPasswordRequest {
        #[doc = "The email of the account to be modified. Specify this and the old password\nin order to change an account's password without using an out-of-band code."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The new password to be set for this account. Specifying this field will\nresult in a change to the account and consume the out-of-band code if one\nwas specified and it was of type PASSWORD_RESET."]
        #[serde(
            rename = "newPassword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_password: ::std::option::Option<String>,
        #[doc = "The current password of the account to be modified. Specify this and email\nto change an account's password without using an out-of-band code."]
        #[serde(
            rename = "oldPassword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_password: ::std::option::Option<String>,
        #[doc = "An out-of-band (OOB) code generated by GetOobCode request. Specify only\nthis parameter (or only this parameter and a tenant ID) to get the\nout-of-band code's type in the response without mutating the account's\nstate. Only a PASSWORD_RESET out-of-band code can be consumed via this\nmethod."]
        #[serde(
            rename = "oobCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oob_code: ::std::option::Option<String>,
        #[doc = "The tenant ID of the Identity Platform tenant the account belongs to."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1ResetPasswordRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1ResetPasswordRequest {
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
    pub struct GoogleCloudIdentitytoolkitV1ResetPasswordResponse {
        #[doc = "The email associated with the out-of-band code that was used."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "mfaInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_info:
            ::std::option::Option<crate::schemas::GoogleCloudIdentitytoolkitV1MfaEnrollment>,
        #[serde(
            rename = "newEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_email: ::std::option::Option<String>,
        #[serde(
            rename = "requestType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_type: ::std::option::Option<
            crate::schemas::GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1ResetPasswordResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1ResetPasswordResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType {
        #[doc = "sign in with email only"]
        EmailSignin,
        NewEmailAccept,
        OldEmailAgree,
        #[doc = "Oob code type is not specified."]
        OobReqTypeUnspecified,
        #[doc = "reset password"]
        PasswordReset,
        RecoverEmail,
        RevertSecondFactorAddition,
        VerifyAndChangeEmail,
        #[doc = "verify the account's email address by sending an email"]
        VerifyEmail,
    }
    impl GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: EmailSignin => "EMAIL_SIGNIN" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: NewEmailAccept => "NEW_EMAIL_ACCEPT" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: OldEmailAgree => "OLD_EMAIL_AGREE" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: OobReqTypeUnspecified => "OOB_REQ_TYPE_UNSPECIFIED" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: PasswordReset => "PASSWORD_RESET" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: RecoverEmail => "RECOVER_EMAIL" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: RevertSecondFactorAddition => "REVERT_SECOND_FACTOR_ADDITION" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: VerifyAndChangeEmail => "VERIFY_AND_CHANGE_EMAIL" , GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: VerifyEmail => "VERIFY_EMAIL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType, ()>
        {
            Ok ( match s { "EMAIL_SIGNIN" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: EmailSignin , "NEW_EMAIL_ACCEPT" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: NewEmailAccept , "OLD_EMAIL_AGREE" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: OldEmailAgree , "OOB_REQ_TYPE_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: OobReqTypeUnspecified , "PASSWORD_RESET" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: PasswordReset , "RECOVER_EMAIL" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: RecoverEmail , "REVERT_SECOND_FACTOR_ADDITION" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: RevertSecondFactorAddition , "VERIFY_AND_CHANGE_EMAIL" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: VerifyAndChangeEmail , "VERIFY_EMAIL" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: VerifyEmail , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "EMAIL_SIGNIN" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: EmailSignin , "NEW_EMAIL_ACCEPT" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: NewEmailAccept , "OLD_EMAIL_AGREE" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: OldEmailAgree , "OOB_REQ_TYPE_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: OobReqTypeUnspecified , "PASSWORD_RESET" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: PasswordReset , "RECOVER_EMAIL" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: RecoverEmail , "REVERT_SECOND_FACTOR_ADDITION" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: RevertSecondFactorAddition , "VERIFY_AND_CHANGE_EMAIL" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: VerifyAndChangeEmail , "VERIFY_EMAIL" => GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType :: VerifyEmail , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1ResetPasswordResponseRequestType
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SendVerificationCodeRequest {
        #[doc = "Receipt of successful iOS app token validation.\nAt least one of (`ios_receipt` and `ios_secret`), `recaptcha_token`, or a\nspatula header for an Android device must be specified to verify the\nverification code is being sent on behalf of a real app and not an\nemulator. This should come from the response of\nverifyIosClient. If present, the caller\nshould also provide the `ios_secret`, as well as a bundle ID in the\n`x-ios-bundle-identifier` header, which must match the bundle ID from the\nverifyIosClient request."]
        #[serde(
            rename = "iosReceipt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_receipt: ::std::option::Option<String>,
        #[doc = "Secret delivered to iOS app as a push notification. Should be passed with\nan `ios_receipt` as well as the `x-ios-bundle-identifier` header."]
        #[serde(
            rename = "iosSecret",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_secret: ::std::option::Option<String>,
        #[doc = "The phone number to send the verification code to in E.164 format."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "Recaptcha token for app verification.  At least one of (`ios_receipt` and\n`ios_secret`), `recaptcha_token`, or a spatula header for an Android device\nmust be specified to verify the verification code is being sent on behalf\nof a real app and not an emulator. The recaptcha should be generated by\ncalling getRecaptchaParams and the\nrecaptcha token will be generated on user completion of the recaptcha\nchallenge."]
        #[serde(
            rename = "recaptchaToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recaptcha_token: ::std::option::Option<String>,
        #[doc = "Tenant ID of the Identity Platform tenant the user is signing in to."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SendVerificationCodeRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SendVerificationCodeRequest
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SendVerificationCodeResponse {
        #[doc = "Encrypted session information. This can be used in\nsignInWithPhoneNumber to authenticate\nthe phone number."]
        #[serde(
            rename = "sessionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SendVerificationCodeResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SendVerificationCodeResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SetAccountInfoRequest { # [ serde ( rename = "captchaChallenge" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub captcha_challenge : :: std :: option :: Option < String > , # [ doc = "The response from reCaptcha challenge. This is required when the system\ndetects possible abuse activities." ] # [ serde ( rename = "captchaResponse" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub captcha_response : :: std :: option :: Option < String > , # [ doc = "The timestamp in milliseconds when the account was created." ] # [ serde ( rename = "createdAt" , default , skip_serializing_if = "std::option::Option::is_none" ) ] # [ serde ( with = "crate::parsed_string" ) ] pub created_at : :: std :: option :: Option < i64 > , # [ doc = "JSON formatted custom attributes to be stored in the Identity Platform ID\ntoken. Specifying this field requires a Google OAuth 2.0 credential with\nproper permissions\n(https://cloud.google.com/identity-platform/docs/access-control)." ] # [ serde ( rename = "customAttributes" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub custom_attributes : :: std :: option :: Option < String > , # [ serde ( rename = "delegatedProjectNumber" , default , skip_serializing_if = "std::option::Option::is_none" ) ] # [ serde ( with = "crate::parsed_string" ) ] pub delegated_project_number : :: std :: option :: Option < i64 > , # [ doc = "The account's attributes to be deleted." ] # [ serde ( rename = "deleteAttribute" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub delete_attribute : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems > > , # [ doc = "The Identity Providers to unlink from the user's account." ] # [ serde ( rename = "deleteProvider" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub delete_provider : :: std :: option :: Option < Vec < String > > , # [ doc = "If true, marks the account as disabled, meaning the user will no longer\nbe able to sign-in." ] # [ serde ( rename = "disableUser" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub disable_user : :: std :: option :: Option < bool > , # [ doc = "The user's new display name to be updated in the account's attributes.\nThe length of the display name must be less than or equal to 256\ncharacters." ] # [ serde ( rename = "displayName" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub display_name : :: std :: option :: Option < String > , # [ doc = "The user's new email to be updated in the account's attributes. The length\nof email should be less than 256 characters and in the format of\n`name@domain.tld`. The email should also match the\n[RFC 822](https://tools.ietf.org/html/rfc822) addr-spec production." ] # [ serde ( rename = "email" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub email : :: std :: option :: Option < String > , # [ doc = "Whether the user's email has been verified. Specifying this field requires\na Google OAuth 2.0 credential with proper permissions\n(https://cloud.google.com/identity-platform/docs/access-control)." ] # [ serde ( rename = "emailVerified" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub email_verified : :: std :: option :: Option < bool > , # [ doc = "A valid Identity Platform ID token. Required when attempting to change\nuser-related information." ] # [ serde ( rename = "idToken" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub id_token : :: std :: option :: Option < String > , # [ serde ( rename = "instanceId" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub instance_id : :: std :: option :: Option < String > , # [ doc = "The timestamp in milliseconds when the account last logged in." ] # [ serde ( rename = "lastLoginAt" , default , skip_serializing_if = "std::option::Option::is_none" ) ] # [ serde ( with = "crate::parsed_string" ) ] pub last_login_at : :: std :: option :: Option < i64 > , # [ doc = "The provider to be linked to the user's account. Specifying this field\nrequires a Google OAuth 2.0 credential with proper permissions\n(https://cloud.google.com/identity-platform/docs/access-control)." ] # [ serde ( rename = "linkProviderUserInfo" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub link_provider_user_info : :: std :: option :: Option < crate :: schemas :: GoogleCloudIdentitytoolkitV1ProviderUserInfo > , # [ doc = "The ID of the user. Specifying this field requires a Google OAuth\n2.0 credential with proper permissions\n(https://cloud.google.com/identity-platform/docs/access-control).\nFor requests from end-users, an ID token should be passed instead." ] # [ serde ( rename = "localId" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub local_id : :: std :: option :: Option < String > , # [ doc = "The multi-factor authentication related information to be set on the user's\naccount. This will overwrite any previous multi-factor related information\non the account. Specifying this field requires a Google OAuth 2.0\ncredential with proper permissions\n(https://cloud.google.com/identity-platform/docs/access-control)." ] # [ serde ( rename = "mfa" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub mfa : :: std :: option :: Option < crate :: schemas :: GoogleCloudIdentitytoolkitV1MfaInfo > , # [ doc = "The out-of-band code to be applied on the user's account.\nThe following out-of-band code types are supported:\n\n* VERIFY_EMAIL\n* RECOVER_EMAIL\n* REVERT_SECOND_FACTOR_ADDITION\n* VERIFY_AND_CHANGE_EMAIL" ] # [ serde ( rename = "oobCode" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub oob_code : :: std :: option :: Option < String > , # [ doc = "The user's new password to be updated in the account's attributes. The\nThe password must be at least 6 characters long." ] # [ serde ( rename = "password" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub password : :: std :: option :: Option < String > , # [ doc = "The phone number to be updated in the account's attributes." ] # [ serde ( rename = "phoneNumber" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub phone_number : :: std :: option :: Option < String > , # [ doc = "The user's new photo URL for the account's profile photo to be updated in\nthe account's attributes. The length of the URL must be less than or equal\nto 2048 characters." ] # [ serde ( rename = "photoUrl" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub photo_url : :: std :: option :: Option < String > , # [ doc = "The Identity Providers that the account should be associated with." ] # [ serde ( rename = "provider" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub provider : :: std :: option :: Option < Vec < String > > , # [ doc = "Whether or not to return an ID and refresh token. Should always be true." ] # [ serde ( rename = "returnSecureToken" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub return_secure_token : :: std :: option :: Option < bool > , # [ doc = "The project ID for the project that the account belongs to. Specifying this\nfield requires Google OAuth 2.0 credential with proper permissions\n(https://cloud.google.com/identity-platform/docs/access-control).\nRequests from end users should pass an Identity Platform ID token instead." ] # [ serde ( rename = "targetProjectId" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub target_project_id : :: std :: option :: Option < String > , # [ doc = "The tenant ID of the Identity Platform tenant that the account belongs to.\nRequests from end users should pass an Identity Platform ID token rather\nthan setting this field." ] # [ serde ( rename = "tenantId" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub tenant_id : :: std :: option :: Option < String > , # [ doc = "Whether the account should be restricted to only using federated\nlogin." ] # [ serde ( rename = "upgradeToFederatedLogin" , default , skip_serializing_if = "std::option::Option::is_none" ) ] pub upgrade_to_federated_login : :: std :: option :: Option < bool > , # [ doc = "Specifies the minimum timestamp in seconds for an Identity Platform ID\ntoken to be considered valid." ] # [ serde ( rename = "validSince" , default , skip_serializing_if = "std::option::Option::is_none" ) ] # [ serde ( with = "crate::parsed_string" ) ] pub valid_since : :: std :: option :: Option < i64 > , }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1SetAccountInfoRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1SetAccountInfoRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems {
        DisplayName,
        Email,
        Password,
        PhotoUrl,
        Provider,
        RawUserInfo,
        UserAttributeNameUnspecified,
    }
    impl GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: DisplayName => "DISPLAY_NAME" , GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Email => "EMAIL" , GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Password => "PASSWORD" , GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: PhotoUrl => "PHOTO_URL" , GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Provider => "PROVIDER" , GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: RawUserInfo => "RAW_USER_INFO" , GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: UserAttributeNameUnspecified => "USER_ATTRIBUTE_NAME_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems,
            (),
        > {
            Ok ( match s { "DISPLAY_NAME" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: DisplayName , "EMAIL" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Email , "PASSWORD" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Password , "PHOTO_URL" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: PhotoUrl , "PROVIDER" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Provider , "RAW_USER_INFO" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: RawUserInfo , "USER_ATTRIBUTE_NAME_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: UserAttributeNameUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "DISPLAY_NAME" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: DisplayName , "EMAIL" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Email , "PASSWORD" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Password , "PHOTO_URL" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: PhotoUrl , "PROVIDER" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: Provider , "RAW_USER_INFO" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: RawUserInfo , "USER_ATTRIBUTE_NAME_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems :: UserAttributeNameUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SetAccountInfoRequestDeleteAttributeItems
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SetAccountInfoResponse {
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Whether the account's email has been verified."]
        #[serde(
            rename = "emailVerified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "The number of seconds until the Identity Platform ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "An Identity Platform ID token for the account. This is used for legacy user\nsign up."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The ID of the authenticated user."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The new email that has been set on the user's account attributes."]
        #[serde(
            rename = "newEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_email: ::std::option::Option<String>,
        #[serde(
            rename = "passwordHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_hash: ::std::option::Option<String>,
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The linked Identity Providers on the account."]
        #[serde(
            rename = "providerUserInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_user_info: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudIdentitytoolkitV1ProviderUserInfo>,
        >,
        #[doc = "A refresh token for the account. This is used for legacy user sign up."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1SetAccountInfoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1SetAccountInfoResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithCustomTokenRequest {
        #[serde(
            rename = "delegatedProjectNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[serde(
            rename = "instanceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "Should always be true."]
        #[serde(
            rename = "returnSecureToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_secure_token: ::std::option::Option<bool>,
        #[doc = "The ID of the Identity Platform tenant the user is signing in to. If\npresent, the ID should match the tenant_id in the token."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Required. The custom Auth token asserted by the developer. The token should be a\n[JSON Web Token (JWT)](https://tools.ietf.org/html/rfc7519) that includes\nthe claims listed in the\n[API\nreference](https://cloud.google.com/identity-platform/docs/reference/rest/client/)\nunder the \"Custom Token Claims\" section."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithCustomTokenRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithCustomTokenRequest
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithCustomTokenResponse {
        #[doc = "The number of seconds until the ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "An Identity Platform ID token for the authenticated user."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Whether the authenticated user was created by this request."]
        #[serde(
            rename = "isNewUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_new_user: ::std::option::Option<bool>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "An Identity Platform refresh token for the authenticated user."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithCustomTokenResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithCustomTokenResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithEmailLinkRequest {
        #[doc = "Required. The email address the sign-in link was sent to. The length of\nemail should be less than 256 characters and in the format of\n`name@domain.tld`. The email should also match the [RFC\n822](https://tools.ietf.org/html/rfc822) addr-spec production."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "A valid ID token for an Identity Platform account. If passed, this request\nwill link the email address to the user represented by this ID\ntoken and enable sign-in with email link on the account for the future."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Required. The out-of-band code from the email link."]
        #[serde(
            rename = "oobCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oob_code: ::std::option::Option<String>,
        #[doc = "The ID of the Identity Platform tenant the user is signing in to. If not\nset, the user will sign in to the default Identity Platform project."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithEmailLinkRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithEmailLinkRequest
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithEmailLinkResponse {
        #[doc = "The email the user signed in with. Always present in the response."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The number of seconds until the ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "An Identity Platform ID token for the authenticated user."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Whether the authenticated user was created by this request."]
        #[serde(
            rename = "isNewUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_new_user: ::std::option::Option<bool>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The ID of the authenticated user. Always present in the response."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "Info on which multi-factor authentication providers are enabled. Present if\nthe user needs to complete the sign-in using multi-factor authentication."]
        #[serde(
            rename = "mfaInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_info:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1MfaEnrollment>>,
        #[doc = "An opaque string that functions as proof that the user has\nsuccessfully passed the first factor check."]
        #[serde(
            rename = "mfaPendingCredential",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_pending_credential: ::std::option::Option<String>,
        #[doc = "Refresh token for the authenticated user."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithEmailLinkResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithEmailLinkResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithGameCenterRequest {
        #[doc = "The user's Game Center display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A valid ID token for an Identity Platform account. If present, this request\nwill link the Game Center player ID to the account represented by this ID\ntoken."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Required. The user's Game Center player ID."]
        #[serde(
            rename = "playerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_id: ::std::option::Option<String>,
        #[doc = "Required. The URL to fetch the Apple public key in order to verify the given\nsignature is signed by Apple."]
        #[serde(
            rename = "publicKeyUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_key_url: ::std::option::Option<String>,
        #[doc = "Required. A random string used to generate the given signature."]
        #[serde(
            rename = "salt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub salt: ::std::option::Option<String>,
        #[doc = "Required. The verification signature data generated by Apple."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<String>,
        #[doc = "The ID of the Identity Platform tenant the user is signing in to."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Required. The time when the signature was created by Apple, in milliseconds since the\nepoch."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub timestamp: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithGameCenterRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithGameCenterRequest
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithGameCenterResponse {
        #[doc = "Display name of the authenticated user."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The number of seconds until the ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "An Identity Platform ID token for the authenticated user."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Whether the logged in user was created by this request."]
        #[serde(
            rename = "isNewUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_new_user: ::std::option::Option<bool>,
        #[doc = "The ID of the authenticated user. Always present in the response."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The user's Game Center player ID."]
        #[serde(
            rename = "playerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub player_id: ::std::option::Option<String>,
        #[doc = "An Identity Platform refresh token for the authenticated user."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithGameCenterResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithGameCenterResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithIdpRequest {
        #[serde(
            rename = "autoCreate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_create: ::std::option::Option<bool>,
        #[serde(
            rename = "delegatedProjectNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "A valid Identity Platform ID token. If passed, the user's account at the\nIdP will be linked to the account represented by this ID token."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "pendingIdToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pending_id_token: ::std::option::Option<String>,
        #[doc = "An opaque string from a previous SignInWithIdp response. If set, it can be\nused to repeat the sign-in operation from the previous SignInWithIdp\noperation."]
        #[serde(
            rename = "pendingToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pending_token: ::std::option::Option<String>,
        #[doc = "If the user is signing in with an authorization response obtained via a\nprevious CreateAuthUri authorization request, this is the body of the HTTP\nPOST callback from the IdP, if present.\n\nOtherwise, if the user is signing in with a manually provided IdP\ncredential, this should be a URL-encoded form that contains the credential\n(e.g. an ID token or access token for OAuth 2.0 IdPs) and the provider ID\nof the IdP that issued the credential.\n\nFor example, if the user is signing in to the Google provider using a\nGoogle ID token, this should be set to\n`id_token=[GOOGLE_ID_TOKEN]&providerId=google.com`, where\n`[GOOGLE_ID_TOKEN]` should be replaced with the Google ID token.\n\nIf the user is signing in to the Facebook provider using a Facebook access\ntoken, this should be set to\n`access_token=[FACEBOOK_ACCESS_TOKEN]&providerId=facebook.com`, where\n`[FACEBOOK_ACCESS_TOKEN]` should be replaced with the Facebook access\ntoken.\n\nIf the user is signing in to the Twitter provider using a Twitter OAuth 1.0\ncredential, this should be set to\n`access_token=[TWITTER_ACCESS_TOKEN]&oauth_token_secret=[TWITTER_TOKEN_SECRET]&providerId=twitter.com`,\nwhere `[TWITTER_ACCESS_TOKEN]` and `[TWITTER_TOKEN_SECRET]` should be\nreplaced with the Twitter OAuth access token and Twitter OAuth token secret\nrespectively."]
        #[serde(
            rename = "postBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub post_body: ::std::option::Option<String>,
        #[doc = "Required. The URL to which the IdP redirects the user back. This can be set to\n`http://localhost` if the user is signing in with a manually provided IdP\ncredential."]
        #[serde(
            rename = "requestUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_uri: ::std::option::Option<String>,
        #[doc = "Whether or not to return OAuth credentials from the IdP on the following\nerrors: `FEDERATED_USER_ID_ALREADY_LINKED` and `EMAIL_EXISTS`."]
        #[serde(
            rename = "returnIdpCredential",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_idp_credential: ::std::option::Option<bool>,
        #[doc = "Whether or not to return the OAuth refresh token from the IdP, if\navailable."]
        #[serde(
            rename = "returnRefreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_refresh_token: ::std::option::Option<bool>,
        #[doc = "Should always be true."]
        #[serde(
            rename = "returnSecureToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_secure_token: ::std::option::Option<bool>,
        #[doc = "The session ID returned from a previous CreateAuthUri call. This field is\nverified against that session ID to prevent session fixation attacks.\nRequired if the user is signing in with an authorization response from a\nprevious CreateAuthUri authorization request."]
        #[serde(
            rename = "sessionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_id: ::std::option::Option<String>,
        #[doc = "The ID of the Identity Platform tenant the user is signing in to. If not\nset, the user will sign in to the default Identity Platform project."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1SignInWithIdpRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1SignInWithIdpRequest {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithIdpResponse {
        #[doc = "The opaque string set in CreateAuthUri that is used to maintain contextual\ninformation between the authentication request and the callback from the\nIdP."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<String>,
        #[doc = "The date of birth for the user's account at the IdP."]
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_of_birth: ::std::option::Option<String>,
        #[doc = "The display name for the user's account at the IdP."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email address of the user's account at the IdP."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Whether or not there is an existing Identity Platform user account with the\nsame email address but linked to a different account at the same IdP. Only\npresent if the \"One account per email address\" setting is enabled and the\nemail address at the IdP is verified."]
        #[serde(
            rename = "emailRecycled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_recycled: ::std::option::Option<bool>,
        #[doc = "Whether the user account's email address is verified."]
        #[serde(
            rename = "emailVerified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "The error message returned if `return_idp_credential` is set to `true` and\neither the `FEDERATED_USER_ID_ALREADY_LINKED` or `EMAIL_EXISTS` error is\nencountered. This field's value is either\n`FEDERATED_USER_ID_ALREADY_LINKED` or `EMAIL_EXISTS`."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "The number of seconds until the Identity Platform ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "The user's account ID at the IdP. Always present in the response."]
        #[serde(
            rename = "federatedId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub federated_id: ::std::option::Option<String>,
        #[doc = "The first name for the user's account at the IdP."]
        #[serde(
            rename = "firstName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_name: ::std::option::Option<String>,
        #[doc = "The full name for the user's account at the IdP."]
        #[serde(
            rename = "fullName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_name: ::std::option::Option<String>,
        #[doc = "An Identity Platform ID token for the authenticated user."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "inputEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_email: ::std::option::Option<String>,
        #[doc = "Whether or not a new Identity Platform account was created for the\nauthenticated user."]
        #[serde(
            rename = "isNewUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_new_user: ::std::option::Option<bool>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The language preference for the user's account at the IdP."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "The last name for the user's account at the IdP."]
        #[serde(
            rename = "lastName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_name: ::std::option::Option<String>,
        #[doc = "The ID of the authenticated Identity Platform user. Always present in the\nresponse."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "Info on which multi-factor authentication providers are enabled for the\naccount. Present if the user needs to complete the sign-in using\nmulti-factor authentication."]
        #[serde(
            rename = "mfaInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_info:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1MfaEnrollment>>,
        #[doc = "An opaque string that functions as proof that the user has successfully\npassed the first factor authentication."]
        #[serde(
            rename = "mfaPendingCredential",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_pending_credential: ::std::option::Option<String>,
        #[doc = "Whether or not there is an existing Identity Platform user account with the\nsame email address as the current account signed in at the IdP, and the\naccount's email addresss is not verified at the IdP. The user will need to\nsign in to the existing Identity Platform account and then link the current\ncredential from the IdP to it. Only present if the \"One account per email\naddress\" setting is enabled."]
        #[serde(
            rename = "needConfirmation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub need_confirmation: ::std::option::Option<bool>,
        #[serde(
            rename = "needEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub need_email: ::std::option::Option<bool>,
        #[doc = "The nickname for the user's account at the IdP."]
        #[serde(
            rename = "nickName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nick_name: ::std::option::Option<String>,
        #[doc = "The OAuth access token from the IdP, if available."]
        #[serde(
            rename = "oauthAccessToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_access_token: ::std::option::Option<String>,
        #[doc = "The OAuth 2.0 authorization code, if available. Only present for the Google\nprovider."]
        #[serde(
            rename = "oauthAuthorizationCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_authorization_code: ::std::option::Option<String>,
        #[doc = "The number of seconds until the OAuth access token from the IdP expires."]
        #[serde(
            rename = "oauthExpireIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_expire_in: ::std::option::Option<i32>,
        #[doc = "The OpenID Connect ID token from the IdP, if available."]
        #[serde(
            rename = "oauthIdToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_id_token: ::std::option::Option<String>,
        #[doc = "The OAuth 2.0 refresh token from the IdP, if available and\n`return_refresh_token` is set to `true`."]
        #[serde(
            rename = "oauthRefreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_refresh_token: ::std::option::Option<String>,
        #[doc = "The OAuth 1.0 token secret from the IdP, if available. Only present for the\nTwitter provider."]
        #[serde(
            rename = "oauthTokenSecret",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_token_secret: ::std::option::Option<String>,
        #[doc = "The main (top-level) email address of the user's Identity Platform account,\nif different from the email address at the IdP. Only present if the \"One\naccount per email address\" setting is enabled."]
        #[serde(
            rename = "originalEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub original_email: ::std::option::Option<String>,
        #[doc = "An opaque string that can be used as a credential from the IdP the user is\nsigning into. The pending token obtained here can be set in a future\nSignInWithIdp request to sign the same user in with the IdP again."]
        #[serde(
            rename = "pendingToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pending_token: ::std::option::Option<String>,
        #[doc = "The URL of the user's profile picture at the IdP."]
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The provider ID of the IdP that the user is signing in to. Always present\nin the response."]
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "The stringified JSON response containing all the data corresponding to the\nuser's account at the IdP."]
        #[serde(
            rename = "rawUserInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_user_info: ::std::option::Option<String>,
        #[doc = "An Identity Platform refresh token for the authenticated user."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
        #[doc = "The screen name for the user's account at the Twitter IdP or the login name\nfor the user's account at the Github IdP."]
        #[serde(
            rename = "screenName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_name: ::std::option::Option<String>,
        #[doc = "The value of the `tenant_id` field in the request."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "The time zone for the user's account at the IdP."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
        #[doc = "A list of provider IDs that the user can sign in to in order to resolve a\n`need_confirmation` error. Only present if `need_confirmation` is set to\n`true`."]
        #[serde(
            rename = "verifiedProvider",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified_provider: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1SignInWithIdpResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1SignInWithIdpResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithPasswordRequest {
        #[serde(
            rename = "captchaChallenge",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_challenge: ::std::option::Option<String>,
        #[doc = "The response from a reCaptcha challenge. A recaptcha response is required\nwhen the service detects possible abuse activity."]
        #[serde(
            rename = "captchaResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_response: ::std::option::Option<String>,
        #[serde(
            rename = "delegatedProjectNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "Required. The email the user is signing in with. The length of email\nshould be less than 256 characters and in the format of `name@domain.tld`.\nThe email should also match the\n[RFC 822](https://tools.ietf.org/html/rfc822) addr-spec production."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "instanceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "Required. The password the user provides to sign in to the account."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[serde(
            rename = "pendingIdToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pending_id_token: ::std::option::Option<String>,
        #[doc = "Should always be true."]
        #[serde(
            rename = "returnSecureToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_secure_token: ::std::option::Option<bool>,
        #[doc = "The ID of the Identity Platform tenant the user is signing in to. If not\nset, the user will sign in to the default Identity Platform instance in the\nproject."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithPasswordRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithPasswordRequest
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithPasswordResponse {
        #[doc = "The user's display name stored in the account's attributes."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email of the authenticated user. Always present in the response."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The number of seconds until the Identity Platform ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "An Identity Platform ID token for the authenticated user."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The ID of the authenticated user. Always present in the response."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "Info on which multi-factor authentication providers are enabled for the\naccount. Present if the user needs to complete the sign-in using\nmulti-factor authentication."]
        #[serde(
            rename = "mfaInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_info:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1MfaEnrollment>>,
        #[doc = "An opaque string that functions as proof that the user has successfully\npassed the first factor authentication."]
        #[serde(
            rename = "mfaPendingCredential",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_pending_credential: ::std::option::Option<String>,
        #[doc = "The OAuth2 access token."]
        #[serde(
            rename = "oauthAccessToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_access_token: ::std::option::Option<String>,
        #[serde(
            rename = "oauthAuthorizationCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_authorization_code: ::std::option::Option<String>,
        #[doc = "The access token expiration time in seconds."]
        #[serde(
            rename = "oauthExpireIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_expire_in: ::std::option::Option<i32>,
        #[doc = "The user's profile picture stored in the account's attributes."]
        #[serde(
            rename = "profilePicture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_picture: ::std::option::Option<String>,
        #[doc = "An Identity Platform refresh token for the authenticated user."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
        #[doc = "Whether the email is for an existing account. Always true."]
        #[serde(
            rename = "registered",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub registered: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithPasswordResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithPasswordResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequest {
        #[doc = "User-entered verification code from an SMS sent to the user's phone."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<String>,
        #[doc = "A valid ID token for an Identity Platform account. If passed, this request\nwill link the phone number to the user represented by this ID token if the\nphone number is not in use, or will reauthenticate the user if the phone\nnumber is already linked to the user."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<
            crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation,
        >,
        #[doc = "The user's phone number to sign in with. This is necessary in the case of\nuing a temporary proof, in which case it must match the phone number that\nwas authenticated in the request that generated the temporary proof.\nThis field is ignored if a session info is passed."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "Encrypted session information from the response of\nsendVerificationCode. In the case of\nauthenticating with an SMS code this must be specified, but in the case of\nusing a temporary proof it can be unspecified."]
        #[serde(
            rename = "sessionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub session_info: ::std::option::Option<String>,
        #[doc = "A proof of the phone number verification, provided from a previous\nsignInWithPhoneNumber request. If this\nis passed, the caller must also pass in the phone_number field the phone\nnumber that was verified in the previous request."]
        #[serde(
            rename = "temporaryProof",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temporary_proof: ::std::option::Option<String>,
        #[doc = "The ID of the Identity Platform tenant the user is signing in to. If not\nset, the user will sign in to the default Identity Platform project."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Do not use."]
        #[serde(
            rename = "verificationProof",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verification_proof: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation {
        #[doc = "Verify operation is to link."]
        Link,
        #[doc = "Verify operation is to reauth."]
        Reauth,
        #[doc = "Verify operation is to sign up/sign in."]
        SignUpOrIn,
        #[doc = "Verify operation is to update."]
        Update,
        #[doc = "Operation is not specified."]
        VerifyOpUnspecified,
    }
    impl GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Link => "LINK" , GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Reauth => "REAUTH" , GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: SignUpOrIn => "SIGN_UP_OR_IN" , GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Update => "UPDATE" , GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: VerifyOpUnspecified => "VERIFY_OP_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation,
            (),
        > {
            Ok ( match s { "LINK" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Link , "REAUTH" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Reauth , "SIGN_UP_OR_IN" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: SignUpOrIn , "UPDATE" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Update , "VERIFY_OP_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: VerifyOpUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "LINK" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Link , "REAUTH" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Reauth , "SIGN_UP_OR_IN" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: SignUpOrIn , "UPDATE" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: Update , "VERIFY_OP_UNSPECIFIED" => GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation :: VerifyOpUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequestOperation
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberResponse {
        #[doc = "The number of seconds until the ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "Identity Platform ID token for the authenticated user."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Whether the authenticated user was created by this request."]
        #[serde(
            rename = "isNewUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_new_user: ::std::option::Option<bool>,
        #[doc = "The id of the authenticated user. Present in the case of a\nsuccessful authentication. In the case when the phone could be verified but\nthe account operation could not be performed, a temporary proof will be\nreturned instead."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "Phone number of the authenticated user. Always present in the response."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "Refresh token for the authenticated user."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
        #[doc = "A proof of the phone number verification, provided if a phone\nauthentication is successful but the user operation fails. This happens\nwhen the request tries to link a phone number to a user with an ID\ntoken or reauthenticate with an ID token but the phone number is linked to\na different user."]
        #[serde(
            rename = "temporaryProof",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temporary_proof: ::std::option::Option<String>,
        #[doc = "The number of seconds until the temporary proof expires."]
        #[serde(
            rename = "temporaryProofExpiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub temporary_proof_expires_in: ::std::option::Option<i64>,
        #[doc = "Do not use."]
        #[serde(
            rename = "verificationProof",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verification_proof: ::std::option::Option<String>,
        #[doc = "Do not use."]
        #[serde(
            rename = "verificationProofExpiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub verification_proof_expires_in: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberResponse
    {
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
    pub struct GoogleCloudIdentitytoolkitV1SignUpRequest {
        #[serde(
            rename = "captchaChallenge",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_challenge: ::std::option::Option<String>,
        #[doc = "The response from a reCaptcha challenge. A reCaptcha response is required\nwhen the service detects potential abuse activity."]
        #[serde(
            rename = "captchaResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub captcha_response: ::std::option::Option<String>,
        #[doc = "Whether the user will be disabled upon creation.  Disabled accounts are\ninaccessible except for requests bearing a Google OAuth2 credential with\nproper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "The display name of the user to be created."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email to assign to the created user. The length of the email should be\nless than 256 characters and in the format of `name@domain.tld`. The email\nshould also match the [RFC 822](https://tools.ietf.org/html/rfc822)\naddr-spec production. An anonymous user will be created if not provided."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Whether the user's email is verified. Specifying this field requires a\nGoogle OAuth 2.0 credential with the proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "emailVerified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "A valid ID token for an Identity Platform user. If set, this request\nwill link the authentication credential to the user represented by this ID\ntoken. For a non-admin request, both the `email` and `password` fields must\nbe set. For an admin request, `local_id` must not be set."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "instanceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "The ID of the user to create. The ID must be unique within the project that\nthe user is being created under. Specifying this field requires a Google\nOAuth 2.0 credential with the proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The multi-factor authentication providers for the user to create."]
        #[serde(
            rename = "mfaInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_info:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1MfaFactor>>,
        #[doc = "The password to assign to the created user. The password must be\nbe at least 6 characters long. If set, the `email` field must also be set."]
        #[serde(
            rename = "password",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password: ::std::option::Option<String>,
        #[doc = "The phone number of the user to create. Specifying this field requires a\nGoogle OAuth 2.0 credential with the proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The profile photo url of the user to create."]
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The project ID of the project which the user should belong to. Specifying\nthis field requires a Google OAuth 2.0 credential with the proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control).\nIf this is not set, the target project is inferred from the scope\nassociated to the Bearer access token."]
        #[serde(
            rename = "targetProjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_project_id: ::std::option::Option<String>,
        #[doc = "The ID of the Identity Platform tenant to create a user under. If not set,\nthe user will be created under the default Identity Platform project."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1SignUpRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1SignUpRequest {
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
    pub struct GoogleCloudIdentitytoolkitV1SignUpResponse {
        #[doc = "The created user's display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The created user's email."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The number of seconds until the ID token expires."]
        #[serde(
            rename = "expiresIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "An Identity Platform ID token for the created user. This field is only\nset for non-admin requests."]
        #[serde(
            rename = "idToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id_token: ::std::option::Option<String>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The ID of the created user. Always present in the response."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "An Identity Platform refresh token for the created user. This field is only\nset for non-admin requests."]
        #[serde(
            rename = "refreshToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1SignUpResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1SignUpResponse {
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
    pub struct GoogleCloudIdentitytoolkitV1SqlExpression {
        #[doc = "A case insensitive string that the account's email should match. Only\none of `email`, `phone_number`, or `user_id` should be specified in a\nSqlExpression.\nIf more than one is specified, only the first (in that order) will be\napplied."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "A string that the account's phone number should match. Only one of\n`email`, `phone_number`, or `user_id` should be specified in a\nSqlExpression.\nIf more than one is specified, only the first (in that order) will be\napplied."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "A string that the account's local ID should match. Only one of\n`email`, `phone_number`, or `user_id` should be specified in a\nSqlExpression\nIf more than one is specified, only the first (in that order) will be\napplied."]
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1SqlExpression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1SqlExpression {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudIdentitytoolkitV1UploadAccountRequest {
        #[doc = "Whether to overwrite an existing account in Identity Platform with a\nmatching `local_id` in the request. If true, the existing account will be\noverwritten. If false, an error will be returned."]
        #[serde(
            rename = "allowOverwrite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_overwrite: ::std::option::Option<bool>,
        #[doc = "The block size parameter used by the STANDARD_SCRYPT hashing function. This\nparameter, along with parallelization and cpu_mem_cost help tune the\nresources needed to hash a password, and should be tuned as processor\nspeeds and memory technologies advance."]
        #[serde(
            rename = "blockSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub block_size: ::std::option::Option<i32>,
        #[doc = "The CPU memory cost parameter to be used by the STANDARD_SCRYPT hashing\nfunction. This parameter, along with block_size and cpu_mem_cost help tune\nthe resources needed to hash a password, and should be tuned as processor\nspeeds and memory technologies advance."]
        #[serde(
            rename = "cpuMemCost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_mem_cost: ::std::option::Option<i32>,
        #[doc = "If true, the service will do the following list of checks before an account\nis uploaded:\n\n* Duplicate emails\n* Duplicate federated IDs\n* Federated ID provider validation\n  If the duplication exists within the list of accounts to be uploaded, it\n  will prevent the entire list from being uploaded. If the email or federated\n  ID is a duplicate of a user already within the project/tenant, the account\n  will not be uploaded, but the rest of the accounts will be unaffected.\n  If false, these checks will be skipped."]
        #[serde(
            rename = "delegatedProjectNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The desired key length for the STANDARD_SCRYPT hashing function.  Must be\nat least 1."]
        #[serde(
            rename = "dkLen",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dk_len: ::std::option::Option<i32>,
        #[doc = "Required. The hashing function used to hash the account passwords. Must be one of the\nfollowing:\n\n* HMAC_SHA256\n* HMAC_SHA1\n* HMAC_MD5\n* SCRYPT\n* PBKDF_SHA1\n* MD5\n* HMAC_SHA512\n* SHA1\n* BCRYPT\n* PBKDF2_SHA256\n* SHA256\n* SHA512\n* STANDARD_SCRYPT"]
        #[serde(
            rename = "hashAlgorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hash_algorithm: ::std::option::Option<String>,
        #[doc = "Memory cost for hash calculation. Only required when the hashing function\nis SCRYPT."]
        #[serde(
            rename = "memoryCost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_cost: ::std::option::Option<i32>,
        #[doc = "The parallelization cost parameter to be used by the STANDARD_SCRYPT\nhashing function. This parameter, along with block_size and cpu_mem_cost\nhelp tune the resources needed to hash a password, and should be tuned as\nprocessor speeds and memory technologies advance."]
        #[serde(
            rename = "parallelization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parallelization: ::std::option::Option<i32>,
        #[doc = "Password and salt order when verify password."]
        #[serde(
            rename = "passwordHashOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_hash_order: ::std::option::Option<
            crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder,
        >,
        #[doc = "The number of rounds used for hash calculation. Only required for the\nfollowing hashing functions:\n\n* MD5\n* SHA1\n* SHA256\n* SHA512\n* PBKDF_SHA1\n* PBKDF2_SHA256\n* SCRYPT"]
        #[serde(
            rename = "rounds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rounds: ::std::option::Option<i32>,
        #[doc = "One or more bytes to be inserted between the salt and plain text password.\nFor stronger security, this should be a single non-printable character."]
        #[serde(
            rename = "saltSeparator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub salt_separator: ::std::option::Option<::google_api_bytes::Bytes>,
        #[serde(
            rename = "sanityCheck",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sanity_check: ::std::option::Option<bool>,
        #[doc = "The signer key used to hash the password. Required for the following\nhashing functions:\n\n* SCRYPT,\n* HMAC_MD5,\n* HMAC_SHA1,\n* HMAC_SHA256,\n* HMAC_SHA512"]
        #[serde(
            rename = "signerKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signer_key: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The ID of the Identity Platform tenant the account belongs to."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "A list of accounts to upload."]
        #[serde(
            rename = "users",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub users: ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1UserInfo>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1UploadAccountRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1UploadAccountRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder {
        #[doc = "The order is password first, and then salt."]
        PasswordAndSalt,
        #[doc = "The order is salt first, and then password."]
        SaltAndPassword,
        #[doc = "The order is not specified."]
        UnspecifiedOrder,
    }
    impl GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: PasswordAndSalt => "PASSWORD_AND_SALT" , GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: SaltAndPassword => "SALT_AND_PASSWORD" , GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: UnspecifiedOrder => "UNSPECIFIED_ORDER" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder,
            (),
        > {
            Ok ( match s { "PASSWORD_AND_SALT" => GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: PasswordAndSalt , "SALT_AND_PASSWORD" => GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: SaltAndPassword , "UNSPECIFIED_ORDER" => GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: UnspecifiedOrder , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "PASSWORD_AND_SALT" => GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: PasswordAndSalt , "SALT_AND_PASSWORD" => GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: SaltAndPassword , "UNSPECIFIED_ORDER" => GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder :: UnspecifiedOrder , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudIdentitytoolkitV1UploadAccountRequestPasswordHashOrder
    {
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
    pub struct GoogleCloudIdentitytoolkitV1UploadAccountResponse {
        #[doc = "Detailed error info for accounts that cannot be uploaded."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1ErrorInfo>>,
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1UploadAccountResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1UploadAccountResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudIdentitytoolkitV1UserInfo {
        #[doc = "The time, in milliseconds from epoch, when the account was created."]
        #[serde(
            rename = "createdAt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub created_at: ::std::option::Option<i64>,
        #[doc = "Custom claims to be added to any ID tokens minted for the account. Should\nbe at most 1,000 characters in length and in valid JSON format."]
        #[serde(
            rename = "customAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_attributes: ::std::option::Option<String>,
        #[doc = "Output only. Whether this account has been authenticated using SignInWithCustomToken."]
        #[serde(
            rename = "customAuth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_auth: ::std::option::Option<bool>,
        #[doc = "Output only. The date of birth set for the account. This account attribute is not used\nby Identity Platform. It is available for informational purposes only."]
        #[serde(
            rename = "dateOfBirth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_of_birth: ::std::option::Option<String>,
        #[doc = "Whether the account is disabled. Disabled accounts are inaccessible except\nfor requests bearing a Google OAuth2 credential with proper permissions."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "The display name of the account. This account attribute is not used by\nIdentity Platform. It is available for informational purposes only."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The account's email address. The length of the email should be less than\n256 characters and in the format of `name@domain.tld`. The email should\nalso match the [RFC 822](https://tools.ietf.org/html/rfc822) addr-spec."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Output only. Whether the account can authenticate with email link."]
        #[serde(
            rename = "emailLinkSignin",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_link_signin: ::std::option::Option<bool>,
        #[doc = "Whether the account's email address has been verified."]
        #[serde(
            rename = "emailVerified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "The first email address associated with this account. The account's initial\nemail cannot be changed once set and is used to recover access to this\naccount if lost via the RECOVER_EMAIL flow in GetOobCode. Should match the\n[RFC 822](https://tools.ietf.org/html/rfc822) addr-spec."]
        #[serde(
            rename = "initialEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub initial_email: ::std::option::Option<String>,
        #[doc = "Output only. The language preference of the account. This account attribute is not used\nby Identity Platform. It is available for informational purposes only."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "The last time, in milliseconds from epoch, this account was logged into."]
        #[serde(
            rename = "lastLoginAt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_login_at: ::std::option::Option<i64>,
        #[doc = "Timestamp when an ID token was last minted for this account."]
        #[serde(
            rename = "lastRefreshAt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_refresh_at: ::std::option::Option<String>,
        #[doc = "Immutable. The unique ID of the account."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "Information on which multi-factor authentication providers are enabled for\nthis account."]
        #[serde(
            rename = "mfaInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mfa_info:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudIdentitytoolkitV1MfaEnrollment>>,
        #[doc = "The account's hashed password. Only accessible by requests bearing a Google\nOAuth2 credential with proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
        #[serde(
            rename = "passwordHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_hash: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The timestamp, in milliseconds from the epoch of 1970-01-01T00:00:00Z, when\nthe account's password was last updated."]
        #[serde(
            rename = "passwordUpdatedAt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub password_updated_at: ::std::option::Option<f64>,
        #[doc = "The account's phone number."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The URL of the account's profile photo. This account attribute is not used\nby Identity Platform. It is available for informational purposes only."]
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "Information about the user as provided by various Identity Providers."]
        #[serde(
            rename = "providerUserInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provider_user_info: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudIdentitytoolkitV1ProviderUserInfo>,
        >,
        #[doc = "Input only. Plain text password used to update a account's password. This field is\nonly ever used as input in a request. Identity Platform uses\ncryptographically secure hashing when managing passwords and will never\nstore or transmit a user's password in plain text."]
        #[serde(
            rename = "rawPassword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw_password: ::std::option::Option<String>,
        #[doc = "The account's password salt. Only accessible by requests bearing a Google\nOAuth2 credential with proper permissions."]
        #[serde(
            rename = "salt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub salt: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Output only. This account's screen name at Twitter or login name at Github."]
        #[serde(
            rename = "screenName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub screen_name: ::std::option::Option<String>,
        #[doc = "ID of the tenant this account belongs to. Only set if this account belongs\nto a tenant."]
        #[serde(
            rename = "tenantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Output only. The time zone preference of the account. This account attribute is not used\nby Identity Platform. It is available for informational purposes only."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
        #[doc = "Oldest timestamp, in seconds since epoch, that an ID token should be\nconsidered valid. All ID tokens issued before this time are considered\ninvalid."]
        #[serde(
            rename = "validSince",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub valid_since: ::std::option::Option<i64>,
        #[doc = "The version of the account's password. Only accessible by requests bearing\na Google OAuth2 credential with proper permissions."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1UserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1UserInfo {
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
    pub struct GoogleCloudIdentitytoolkitV1VerifyIosClientRequest {
        #[doc = "A device token that the iOS client gets after registering to APNs (Apple\nPush Notification service)."]
        #[serde(
            rename = "appToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_token: ::std::option::Option<String>,
        #[doc = "Whether the app token is in the iOS sandbox. If false, the app token is in\nthe production environment."]
        #[serde(
            rename = "isSandbox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_sandbox: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudIdentitytoolkitV1VerifyIosClientRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1VerifyIosClientRequest {
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
    pub struct GoogleCloudIdentitytoolkitV1VerifyIosClientResponse {
        #[doc = "Receipt of successful app token validation."]
        #[serde(
            rename = "receipt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub receipt: ::std::option::Option<String>,
        #[doc = "Suggested time that the client should wait in seconds for delivery of the\npush notification."]
        #[serde(
            rename = "suggestedTimeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub suggested_timeout: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudIdentitytoolkitV1VerifyIosClientResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudIdentitytoolkitV1VerifyIosClientResponse {
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
        A: ::google_api_auth::GetAccessToken + 'static,
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
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client {
            reqwest,
            auth: Box::new(auth),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the accounts resource"]
    pub fn accounts(&self) -> crate::resources::accounts::AccountsActions {
        crate::resources::accounts::AccountsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the v_1 resource"]
    pub fn v_1(&self) -> crate::resources::v_1::V1Actions {
        crate::resources::v_1::V1Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod accounts {
        pub mod params {}
        pub struct AccountsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AccountsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "If an email identifier is specified, checks and returns if any user account\nis registered with the email. If there is a registered account, fetches all\nproviders associated with the account's email.\n\nIf the provider ID of an Identity Provider (IdP) is specified, creates an\nauthorization URI for the IdP. The user can be directed to this URI to sign\nin with the IdP.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn create_auth_uri(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1CreateAuthUriRequest,
            ) -> CreateAuthUriRequestBuilder {
                CreateAuthUriRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Deletes a user's account."]
            pub fn delete(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountRequest,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Experimental"]
            pub fn issue_saml_response(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1IssueSamlResponseRequest,
            ) -> IssueSamlResponseRequestBuilder {
                IssueSamlResponseRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Gets account information for all matched accounts. For an end user request,\nretrieves the account of the end user. For an admin request with Google\nOAuth 2.0 credential, retrieves one or multiple account(s) with matching\ncriteria."]
            pub fn lookup(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoRequest,
            ) -> LookupRequestBuilder {
                LookupRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Resets the password of an account either using an out-of-band code\ngenerated by sendOobCode or by specifying the\nemail and password of the account to be modified. Can also check the\npurpose of an out-of-band code without consuming it."]
            pub fn reset_password(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1ResetPasswordRequest,
            ) -> ResetPasswordRequestBuilder {
                ResetPasswordRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Sends an out-of-band confirmation code for an account. Requests from a\nauthenticated request can optionally return a link including the OOB code\ninstead of sending it."]
            pub fn send_oob_code(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeRequest,
            ) -> SendOobCodeRequestBuilder {
                SendOobCodeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Sends a SMS verification code for phone number sign-in.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn send_verification_code(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SendVerificationCodeRequest,
            ) -> SendVerificationCodeRequestBuilder {
                SendVerificationCodeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Signs in or signs up a user by exchanging a custom Auth token. Upon a\nsuccessful sign-in or sign-up, a new Identity Platform ID token and refresh\ntoken are issued for the user.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn sign_in_with_custom_token(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithCustomTokenRequest,
            ) -> SignInWithCustomTokenRequestBuilder {
                SignInWithCustomTokenRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Signs in or signs up a user with a out-of-band code from an email link. If\na user does not exist with the given email address, a user record will be\ncreated. If the sign-in succeeds, an Identity Platform ID and refresh token\nare issued for the authenticated user.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn sign_in_with_email_link(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithEmailLinkRequest,
            ) -> SignInWithEmailLinkRequestBuilder {
                SignInWithEmailLinkRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Signs in or signs up a user with iOS Game Center credentials. If the\nsign-in succeeds, a new Identity Platform ID token and refresh token are\nissued for the authenticated user. The bundle ID is required in the request\nheader as `x-ios-bundle-identifier`.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn sign_in_with_game_center(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithGameCenterRequest,
            ) -> SignInWithGameCenterRequestBuilder {
                SignInWithGameCenterRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Signs in or signs up a user using credentials from an Identity Provider\n(IdP). This is done by manually providing an IdP credential, or by\nproviding the authorization response obtained via the authorization\nrequest from CreateAuthUri. If the sign-in succeeds, a new Identity\nPlatform ID token and refresh token are issued for the authenticated user.\n\nA new Identity Platform user account will be created if the user has not\npreviously signed in to the IdP with the same account. In addition, when\nthe \"One account per email address\" setting is enabled, there should not\nbe an existing Identity Platform user account with the same email address\nfor a new user account to be created.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn sign_in_with_idp(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithIdpRequest,
            ) -> SignInWithIdpRequestBuilder {
                SignInWithIdpRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Signs in a user with email and password. If the sign-in succeeds, a new\nIdentity Platform ID token and refresh token are issued for the\nauthenticated user.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn sign_in_with_password(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPasswordRequest,
            ) -> SignInWithPasswordRequestBuilder {
                SignInWithPasswordRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Completes a phone number authentication attempt. If a user already\nexists with the given phone number, an ID token is minted for that user.\nOtherwise, a new user is created and associated with the phone number.\nThis method may also be used to link a phone number to an existing user.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn sign_in_with_phone_number(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequest,
            ) -> SignInWithPhoneNumberRequestBuilder {
                SignInWithPhoneNumberRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Signs up a new email and password user or anonymous user, or upgrades an\nanonymous user to email and password. For an admin request with a Google\nOAuth 2.0 credential with the proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control),\ncreates a new anonymous, email and password, or phone number user.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn sign_up(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignUpRequest,
            ) -> SignUpRequestBuilder {
                SignUpRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Updates account-related information for the specified user by setting\nspecific fields or applying action codes. Requests from administrators\nand end users are supported."]
            pub fn update(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoRequest,
            ) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Verifies an iOS client is a real iOS device. If the request is valid, a\nreciept will be sent in the response and a secret will be sent via\nApple Push Notification Service. The client should send both of them back\nto certain Identity Platform APIs in a later call (for example,\n/accounts:sendVerificationCode), in order to verify the client. The bundle\nID is required in the request header as `x-ios-bundle-identifier`.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn verify_ios_client(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1VerifyIosClientRequest,
            ) -> VerifyIosClientRequestBuilder {
                VerifyIosClientRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[doc = "Created via [AccountsActions::create_auth_uri()](struct.AccountsActions.html#method.create_auth_uri)"]
        #[derive(Debug, Clone)]
        pub struct CreateAuthUriRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1CreateAuthUriRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> CreateAuthUriRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1CreateAuthUriResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1CreateAuthUriResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:createAuthUri");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::delete()](struct.AccountsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:delete");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::issue_saml_response()](struct.AccountsActions.html#method.issue_saml_response)"]
        #[derive(Debug, Clone)]
        pub struct IssueSamlResponseRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1IssueSamlResponseRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> IssueSamlResponseRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1IssueSamlResponseResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1IssueSamlResponseResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:issueSamlResponse");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::lookup()](struct.AccountsActions.html#method.lookup)"]
        #[derive(Debug, Clone)]
        pub struct LookupRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> LookupRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:lookup");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::reset_password()](struct.AccountsActions.html#method.reset_password)"]
        #[derive(Debug, Clone)]
        pub struct ResetPasswordRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1ResetPasswordRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> ResetPasswordRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1ResetPasswordResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1ResetPasswordResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:resetPassword");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::send_oob_code()](struct.AccountsActions.html#method.send_oob_code)"]
        #[derive(Debug, Clone)]
        pub struct SendOobCodeRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SendOobCodeRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeResponse, crate::Error>
            {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:sendOobCode");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::send_verification_code()](struct.AccountsActions.html#method.send_verification_code)"]
        #[derive(Debug, Clone)]
        pub struct SendVerificationCodeRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SendVerificationCodeRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SendVerificationCodeRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SendVerificationCodeResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SendVerificationCodeResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:sendVerificationCode");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::sign_in_with_custom_token()](struct.AccountsActions.html#method.sign_in_with_custom_token)"]
        #[derive(Debug, Clone)]
        pub struct SignInWithCustomTokenRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithCustomTokenRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SignInWithCustomTokenRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithCustomTokenResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithCustomTokenResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:signInWithCustomToken");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::sign_in_with_email_link()](struct.AccountsActions.html#method.sign_in_with_email_link)"]
        #[derive(Debug, Clone)]
        pub struct SignInWithEmailLinkRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithEmailLinkRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SignInWithEmailLinkRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithEmailLinkResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithEmailLinkResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:signInWithEmailLink");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::sign_in_with_game_center()](struct.AccountsActions.html#method.sign_in_with_game_center)"]
        #[derive(Debug, Clone)]
        pub struct SignInWithGameCenterRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithGameCenterRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SignInWithGameCenterRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithGameCenterResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithGameCenterResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:signInWithGameCenter");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::sign_in_with_idp()](struct.AccountsActions.html#method.sign_in_with_idp)"]
        #[derive(Debug, Clone)]
        pub struct SignInWithIdpRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithIdpRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SignInWithIdpRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithIdpResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithIdpResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:signInWithIdp");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::sign_in_with_password()](struct.AccountsActions.html#method.sign_in_with_password)"]
        #[derive(Debug, Clone)]
        pub struct SignInWithPasswordRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPasswordRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SignInWithPasswordRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPasswordResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPasswordResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:signInWithPassword");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::sign_in_with_phone_number()](struct.AccountsActions.html#method.sign_in_with_phone_number)"]
        #[derive(Debug, Clone)]
        pub struct SignInWithPhoneNumberRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SignInWithPhoneNumberRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SignInWithPhoneNumberResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:signInWithPhoneNumber");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::sign_up()](struct.AccountsActions.html#method.sign_up)"]
        #[derive(Debug, Clone)]
        pub struct SignUpRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignUpRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> SignUpRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1SignUpResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1SignUpResponse, crate::Error>
            {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:signUp");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::update()](struct.AccountsActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> UpdateRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:update");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [AccountsActions::verify_ios_client()](struct.AccountsActions.html#method.verify_ios_client)"]
        #[derive(Debug, Clone)]
        pub struct VerifyIosClientRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1VerifyIosClientRequest,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> VerifyIosClientRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1VerifyIosClientResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1VerifyIosClientResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/accounts:verifyIosClient");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Signs up a new email and password user or anonymous user, or upgrades an\nanonymous user to email and password. For an admin request with a Google\nOAuth 2.0 credential with the proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control),\ncreates a new anonymous, email and password, or phone number user.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
            pub fn accounts_method(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignUpRequest,
                target_project_id: impl Into<String>,
            ) -> AccountsMethodRequestBuilder {
                AccountsMethodRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    target_project_id: target_project_id.into(),
                }
            }
            #[doc = "Creates a session cookie for the given Identity Platform ID token. The\nsession cookie is used by the client to preserve the user's login state."]
            pub fn create_session_cookie(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieRequest,
                target_project_id: impl Into<String>,
            ) -> CreateSessionCookieRequestBuilder {
                CreateSessionCookieRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    target_project_id: target_project_id.into(),
                }
            }
            #[doc = "Looks up user accounts within a project or a tenant based on conditions\nin the request."]
            pub fn query_accounts(
                &self,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequest,
                target_project_id: impl Into<String>,
            ) -> QueryAccountsRequestBuilder {
                QueryAccountsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    target_project_id: target_project_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the accounts resource"]
            pub fn accounts(&self) -> crate::resources::projects::accounts::AccountsActions {
                crate::resources::projects::accounts::AccountsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the tenants resource"]
            pub fn tenants(&self) -> crate::resources::projects::tenants::TenantsActions {
                crate::resources::projects::tenants::TenantsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::accounts_method()](struct.ProjectsActions.html#method.accounts_method)"]
        #[derive(Debug, Clone)]
        pub struct AccountsMethodRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1SignUpRequest,
            target_project_id: String,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> AccountsMethodRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1SignUpResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1SignUpResponse, crate::Error>
            {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/projects/");
                {
                    let var_as_str = &self.target_project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/accounts");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ProjectsActions::create_session_cookie()](struct.ProjectsActions.html#method.create_session_cookie)"]
        #[derive(Debug, Clone)]
        pub struct CreateSessionCookieRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieRequest,
            target_project_id: String,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> CreateSessionCookieRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/projects/");
                {
                    let var_as_str = &self.target_project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":createSessionCookie");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [ProjectsActions::query_accounts()](struct.ProjectsActions.html#method.query_accounts)"]
        #[derive(Debug, Clone)]
        pub struct QueryAccountsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequest,
            target_project_id: String,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> QueryAccountsRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoResponse,
                crate::Error,
            > {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/projects/");
                {
                    let var_as_str = &self.target_project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":queryAccounts");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        pub mod accounts {
            pub mod params {}
            pub struct AccountsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AccountsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Uploads multiple accounts into the Google Cloud project. If there is a\nproblem uploading one or more of the accounts, the rest will be uploaded,\nand a list of the errors will be returned. To use this method requires a\nGoogle OAuth 2.0 credential with proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
                pub fn batch_create(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountRequest,
                    target_project_id: impl Into<String>,
                ) -> BatchCreateRequestBuilder {
                    BatchCreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                    }
                }
                #[doc = "Batch deletes multiple accounts. For accounts that fail to be deleted,\nerror info is contained in the response. The method ignores accounts that\ndo not exist or are duplicated in the request.\n\nThis method requires a Google OAuth 2.0 credential with proper permissions.\n(https://cloud.google.com/identity-platform/docs/access-control)"]
                pub fn batch_delete(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteAccountsRequest,
                    target_project_id: impl Into<String>,
                ) -> BatchDeleteRequestBuilder {
                    BatchDeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                    }
                }
                #[doc = "Download account information for all accounts on the project in a paginated\nmanner. To use this method requires a Google OAuth 2.0 credential with\nproper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)..\nFurthermore, additional permissions are needed to get password hash,\npassword salt, and password version from accounts; otherwise these fields\nare redacted."]
                pub fn batch_get(
                    &self,
                    target_project_id: impl Into<String>,
                ) -> BatchGetRequestBuilder {
                    BatchGetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                        delegated_project_number: None,
                        max_results: None,
                        next_page_token: None,
                        tenant_id: None,
                    }
                }
                #[doc = "Deletes a user's account."]
                pub fn delete(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountRequest,
                    target_project_id: impl Into<String>,
                ) -> DeleteRequestBuilder {
                    DeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                    }
                }
                #[doc = "Gets account information for all matched accounts. For an end user request,\nretrieves the account of the end user. For an admin request with Google\nOAuth 2.0 credential, retrieves one or multiple account(s) with matching\ncriteria."]
                pub fn lookup(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoRequest,
                    target_project_id: impl Into<String>,
                ) -> LookupRequestBuilder {
                    LookupRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                    }
                }
                #[doc = "Looks up user accounts within a project or a tenant based on conditions\nin the request."]
                pub fn query(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequest,
                    target_project_id: impl Into<String>,
                ) -> QueryRequestBuilder {
                    QueryRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                    }
                }
                #[doc = "Sends an out-of-band confirmation code for an account. Requests from a\nauthenticated request can optionally return a link including the OOB code\ninstead of sending it."]
                pub fn send_oob_code(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeRequest,
                    target_project_id: impl Into<String>,
                ) -> SendOobCodeRequestBuilder {
                    SendOobCodeRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                    }
                }
                #[doc = "Updates account-related information for the specified user by setting\nspecific fields or applying action codes. Requests from administrators\nand end users are supported."]
                pub fn update(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoRequest,
                    target_project_id: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                    }
                }
            }
            #[doc = "Created via [AccountsActions::batch_create()](struct.AccountsActions.html#method.batch_create)"]
            #[derive(Debug, Clone)]
            pub struct BatchCreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountRequest,
                target_project_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> BatchCreateRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:batchCreate");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [AccountsActions::batch_delete()](struct.AccountsActions.html#method.batch_delete)"]
            #[derive(Debug, Clone)]
            pub struct BatchDeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteAccountsRequest,
                target_project_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> BatchDeleteRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteAccountsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteAccountsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:batchDelete");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [AccountsActions::batch_get()](struct.AccountsActions.html#method.batch_get)"]
            #[derive(Debug, Clone)]
            pub struct BatchGetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                target_project_id: String,
                delegated_project_number: Option<i64>,
                max_results: Option<i32>,
                next_page_token: Option<String>,
                tenant_id: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> BatchGetRequestBuilder<'a> {
                #[doc = ""]
                pub fn delegated_project_number(mut self, value: i64) -> Self {
                    self.delegated_project_number = Some(value);
                    self
                }
                #[doc = "The maximum number of results to return. Must be at least 1 and no greater\nthan 1000.  By default, it is 20."]
                pub fn max_results(mut self, value: i32) -> Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "The pagination token from the response of a previous request."]
                pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
                    self.next_page_token = Some(value.into());
                    self
                }
                #[doc = "The ID of the Identity Platform tenant the accounts belongs to. If not\nspecified, accounts on the Identity Platform project are returned."]
                pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
                    self.tenant_id = Some(value.into());
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1DownloadAccountResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1DownloadAccountResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:batchGet");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req =
                        req.query(&[("delegatedProjectNumber", &self.delegated_project_number)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("nextPageToken", &self.next_page_token)]);
                    let req = req.query(&[("tenantId", &self.tenant_id)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [AccountsActions::delete()](struct.AccountsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountRequest,
                target_project_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> DeleteRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:delete");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [AccountsActions::lookup()](struct.AccountsActions.html#method.lookup)"]
            #[derive(Debug, Clone)]
            pub struct LookupRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoRequest,
                target_project_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> LookupRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:lookup");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [AccountsActions::query()](struct.AccountsActions.html#method.query)"]
            #[derive(Debug, Clone)]
            pub struct QueryRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequest,
                target_project_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> QueryRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:query");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [AccountsActions::send_oob_code()](struct.AccountsActions.html#method.send_oob_code)"]
            #[derive(Debug, Clone)]
            pub struct SendOobCodeRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeRequest,
                target_project_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> SendOobCodeRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:sendOobCode");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [AccountsActions::update()](struct.AccountsActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoRequest,
                target_project_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> UpdateRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts:update");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
        pub mod tenants {
            pub mod params {}
            pub struct TenantsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> TenantsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Signs up a new email and password user or anonymous user, or upgrades an\nanonymous user to email and password. For an admin request with a Google\nOAuth 2.0 credential with the proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control),\ncreates a new anonymous, email and password, or phone number user.\n\nAn [API key](https://cloud.google.com/docs/authentication/api-keys) is\nrequired in the request in order to identify the Google Cloud project."]
                pub fn accounts_method(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1SignUpRequest,
                    target_project_id: impl Into<String>,
                    tenant_id: impl Into<String>,
                ) -> AccountsMethodRequestBuilder {
                    AccountsMethodRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                        tenant_id: tenant_id.into(),
                    }
                }
                #[doc = "Creates a session cookie for the given Identity Platform ID token. The\nsession cookie is used by the client to preserve the user's login state."]
                pub fn create_session_cookie(
                    &self,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieRequest,
                    target_project_id: impl Into<String>,
                    tenant_id: impl Into<String>,
                ) -> CreateSessionCookieRequestBuilder {
                    CreateSessionCookieRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                        request,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        target_project_id: target_project_id.into(),
                        tenant_id: tenant_id.into(),
                    }
                }
                #[doc = "Actions that can be performed on the accounts resource"]
                pub fn accounts(
                    &self,
                ) -> crate::resources::projects::tenants::accounts::AccountsActions
                {
                    crate::resources::projects::tenants::accounts::AccountsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [TenantsActions::accounts_method()](struct.TenantsActions.html#method.accounts_method)"]
            #[derive(Debug, Clone)]
            pub struct AccountsMethodRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1SignUpRequest,
                target_project_id: String,
                tenant_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> AccountsMethodRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1SignUpResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudIdentitytoolkitV1SignUpResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/tenants/");
                    {
                        let var_as_str = &self.tenant_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/accounts");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            #[doc = "Created via [TenantsActions::create_session_cookie()](struct.TenantsActions.html#method.create_session_cookie)"]
            #[derive(Debug, Clone)]
            pub struct CreateSessionCookieRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieRequest,
                target_project_id: String,
                tenant_id: String,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> CreateSessionCookieRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudIdentitytoolkitV1CreateSessionCookieResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.target_project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/tenants/");
                    {
                        let var_as_str = &self.tenant_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":createSessionCookie");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
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
            pub mod accounts {
                pub mod params {}
                pub struct AccountsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> AccountsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Uploads multiple accounts into the Google Cloud project. If there is a\nproblem uploading one or more of the accounts, the rest will be uploaded,\nand a list of the errors will be returned. To use this method requires a\nGoogle OAuth 2.0 credential with proper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)."]
                    pub fn batch_create(
                        &self,
                        request: crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountRequest,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> BatchCreateRequestBuilder {
                        BatchCreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                        }
                    }
                    #[doc = "Batch deletes multiple accounts. For accounts that fail to be deleted,\nerror info is contained in the response. The method ignores accounts that\ndo not exist or are duplicated in the request.\n\nThis method requires a Google OAuth 2.0 credential with proper permissions.\n(https://cloud.google.com/identity-platform/docs/access-control)"]
                    pub fn batch_delete(
                        &self,
                        request : crate :: schemas :: GoogleCloudIdentitytoolkitV1BatchDeleteAccountsRequest,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> BatchDeleteRequestBuilder {
                        BatchDeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                        }
                    }
                    #[doc = "Download account information for all accounts on the project in a paginated\nmanner. To use this method requires a Google OAuth 2.0 credential with\nproper\n[permissions](https://cloud.google.com/identity-platform/docs/access-control)..\nFurthermore, additional permissions are needed to get password hash,\npassword salt, and password version from accounts; otherwise these fields\nare redacted."]
                    pub fn batch_get(
                        &self,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> BatchGetRequestBuilder {
                        BatchGetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                            delegated_project_number: None,
                            max_results: None,
                            next_page_token: None,
                        }
                    }
                    #[doc = "Deletes a user's account."]
                    pub fn delete(
                        &self,
                        request: crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountRequest,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> DeleteRequestBuilder {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                        }
                    }
                    #[doc = "Gets account information for all matched accounts. For an end user request,\nretrieves the account of the end user. For an admin request with Google\nOAuth 2.0 credential, retrieves one or multiple account(s) with matching\ncriteria."]
                    pub fn lookup(
                        &self,
                        request: crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoRequest,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> LookupRequestBuilder {
                        LookupRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                        }
                    }
                    #[doc = "Looks up user accounts within a project or a tenant based on conditions\nin the request."]
                    pub fn query(
                        &self,
                        request: crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequest,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> QueryRequestBuilder {
                        QueryRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                        }
                    }
                    #[doc = "Sends an out-of-band confirmation code for an account. Requests from a\nauthenticated request can optionally return a link including the OOB code\ninstead of sending it."]
                    pub fn send_oob_code(
                        &self,
                        request: crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeRequest,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> SendOobCodeRequestBuilder {
                        SendOobCodeRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                        }
                    }
                    #[doc = "Updates account-related information for the specified user by setting\nspecific fields or applying action codes. Requests from administrators\nand end users are supported."]
                    pub fn update(
                        &self,
                        request: crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoRequest,
                        target_project_id: impl Into<String>,
                        tenant_id: impl Into<String>,
                    ) -> UpdateRequestBuilder {
                        UpdateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            target_project_id: target_project_id.into(),
                            tenant_id: tenant_id.into(),
                        }
                    }
                }
                #[doc = "Created via [AccountsActions::batch_create()](struct.AccountsActions.html#method.batch_create)"]
                #[derive(Debug, Clone)]
                pub struct BatchCreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountRequest,
                    target_project_id: String,
                    tenant_id: String,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchCreateRequestBuilder<'a> {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1UploadAccountResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:batchCreate");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
                #[doc = "Created via [AccountsActions::batch_delete()](struct.AccountsActions.html#method.batch_delete)"]
                #[derive(Debug, Clone)]
                pub struct BatchDeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteAccountsRequest,
                    target_project_id: String,
                    tenant_id: String,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchDeleteRequestBuilder<'a> {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteAccountsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1BatchDeleteAccountsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:batchDelete");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
                #[doc = "Created via [AccountsActions::batch_get()](struct.AccountsActions.html#method.batch_get)"]
                #[derive(Debug, Clone)]
                pub struct BatchGetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    target_project_id: String,
                    tenant_id: String,
                    delegated_project_number: Option<i64>,
                    max_results: Option<i32>,
                    next_page_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> BatchGetRequestBuilder<'a> {
                    #[doc = ""]
                    pub fn delegated_project_number(mut self, value: i64) -> Self {
                        self.delegated_project_number = Some(value);
                        self
                    }
                    #[doc = "The maximum number of results to return. Must be at least 1 and no greater\nthan 1000.  By default, it is 20."]
                    pub fn max_results(mut self, value: i32) -> Self {
                        self.max_results = Some(value);
                        self
                    }
                    #[doc = "The pagination token from the response of a previous request."]
                    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
                        self.next_page_token = Some(value.into());
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1DownloadAccountResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1DownloadAccountResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:batchGet");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req
                            .query(&[("delegatedProjectNumber", &self.delegated_project_number)]);
                        let req = req.query(&[("maxResults", &self.max_results)]);
                        let req = req.query(&[("nextPageToken", &self.next_page_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
                #[doc = "Created via [AccountsActions::delete()](struct.AccountsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountRequest,
                    target_project_id: String,
                    tenant_id: String,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> DeleteRequestBuilder<'a> {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1DeleteAccountResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:delete");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
                #[doc = "Created via [AccountsActions::lookup()](struct.AccountsActions.html#method.lookup)"]
                #[derive(Debug, Clone)]
                pub struct LookupRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoRequest,
                    target_project_id: String,
                    tenant_id: String,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> LookupRequestBuilder<'a> {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1GetAccountInfoResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:lookup");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
                #[doc = "Created via [AccountsActions::query()](struct.AccountsActions.html#method.query)"]
                #[derive(Debug, Clone)]
                pub struct QueryRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoRequest,
                    target_project_id: String,
                    tenant_id: String,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> QueryRequestBuilder<'a> {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1QueryUserInfoResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:query");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
                #[doc = "Created via [AccountsActions::send_oob_code()](struct.AccountsActions.html#method.send_oob_code)"]
                #[derive(Debug, Clone)]
                pub struct SendOobCodeRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeRequest,
                    target_project_id: String,
                    tenant_id: String,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> SendOobCodeRequestBuilder<'a> {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1GetOobCodeResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:sendOobCode");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
                #[doc = "Created via [AccountsActions::update()](struct.AccountsActions.html#method.update)"]
                #[derive(Debug, Clone)]
                pub struct UpdateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoRequest,
                    target_project_id: String,
                    tenant_id: String,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> UpdateRequestBuilder<'a> {
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
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudIdentitytoolkitV1SetAccountInfoResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
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
                        let req = req.json(&self.request);
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                        output.push_str("v1/projects/");
                        {
                            let var_as_str = &self.target_project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/tenants/");
                        {
                            let var_as_str = &self.tenant_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/accounts:update");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
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
    }
    pub mod v_1 {
        pub mod params {}
        pub struct V1Actions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets a project's public Identity Toolkit configuration.\n(Legacy) This method also supports authenticated calls from a developer to\nretrieve non-public configuration."]
            pub fn get_projects(&self) -> GetProjectsRequestBuilder {
                GetProjectsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    android_package_name: None,
                    client_id: None,
                    delegated_project_number: None,
                    ios_bundle_id: None,
                    project_number: None,
                    return_dynamic_link: None,
                    sha_1_cert: None,
                }
            }
            #[doc = "Retrieves public keys of the legacy Identity Toolkit token signer to\nenable third parties to verify the legacy ID token. For now the X509 pem\ncert is the only format supported."]
            pub fn get_public_keys(&self) -> GetPublicKeysRequestBuilder {
                GetPublicKeysRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Gets parameters needed for generating a reCAPTCHA challenge."]
            pub fn get_recaptcha_params(&self) -> GetRecaptchaParamsRequestBuilder {
                GetRecaptchaParamsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
            #[doc = "Retrieves the set of public keys of the session cookie JSON Web Token (JWT)\nsigner that can be used to validate the session cookie created through\ncreateSessionCookie."]
            pub fn get_session_cookie_public_keys(
                &self,
            ) -> GetSessionCookiePublicKeysRequestBuilder {
                GetSessionCookiePublicKeysRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                }
            }
        }
        #[doc = "Created via [V1Actions::get_projects()](struct.V1Actions.html#method.get_projects)"]
        #[derive(Debug, Clone)]
        pub struct GetProjectsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            android_package_name: Option<String>,
            client_id: Option<String>,
            delegated_project_number: Option<i64>,
            ios_bundle_id: Option<String>,
            project_number: Option<i64>,
            return_dynamic_link: Option<bool>,
            sha_1_cert: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetProjectsRequestBuilder<'a> {
            #[doc = "Android package name to check against the real android package name.\nIf this field is provided, and sha1_cert_hash is not provided, the action\nwill throw an error if this does not match the real android package name."]
            pub fn android_package_name(mut self, value: impl Into<String>) -> Self {
                self.android_package_name = Some(value.into());
                self
            }
            #[doc = "The RP OAuth client ID. If set, a check will be performed to ensure that\nthe OAuth client is valid for the retrieved project and the request\nrejected with a client error if not valid."]
            pub fn client_id(mut self, value: impl Into<String>) -> Self {
                self.client_id = Some(value.into());
                self
            }
            #[doc = "Project Number of the delegated project request. This field should only be\nused as part of the Firebase V1 migration."]
            pub fn delegated_project_number(mut self, value: i64) -> Self {
                self.delegated_project_number = Some(value);
                self
            }
            #[doc = "iOS bundle id to check against the real ios bundle id.\nIf this field is provided, the action will throw an error if this does\nnot match the real iOS bundle id."]
            pub fn ios_bundle_id(mut self, value: impl Into<String>) -> Self {
                self.ios_bundle_id = Some(value.into());
                self
            }
            #[doc = "Project number of the configuration to retrieve. This field is deprecated\nand should  not be used by new integrations."]
            pub fn project_number(mut self, value: i64) -> Self {
                self.project_number = Some(value);
                self
            }
            #[doc = "Whether dynamic link should be returned."]
            pub fn return_dynamic_link(mut self, value: bool) -> Self {
                self.return_dynamic_link = Some(value);
                self
            }
            #[doc = "SHA-1 Android application cert hash. If set, a check will be performed to\nensure that the cert hash is valid for the retrieved project and\nandroid_package_name."]
            pub fn sha_1_cert(mut self, value: impl Into<String>) -> Self {
                self.sha_1_cert = Some(value.into());
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetProjectConfigResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetProjectConfigResponse,
                crate::Error,
            > {
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
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/projects");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("androidPackageName", &self.android_package_name)]);
                let req = req.query(&[("clientId", &self.client_id)]);
                let req = req.query(&[("delegatedProjectNumber", &self.delegated_project_number)]);
                let req = req.query(&[("iosBundleId", &self.ios_bundle_id)]);
                let req = req.query(&[("projectNumber", &self.project_number)]);
                let req = req.query(&[("returnDynamicLink", &self.return_dynamic_link)]);
                let req = req.query(&[("sha1Cert", &self.sha_1_cert)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [V1Actions::get_public_keys()](struct.V1Actions.html#method.get_public_keys)"]
        #[derive(Debug, Clone)]
        pub struct GetPublicKeysRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetPublicKeysRequestBuilder<'a> {
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
            ) -> Result<::std::collections::BTreeMap<String, ::serde_json::Value>, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<::std::collections::BTreeMap<String, ::serde_json::Value>, crate::Error>
            {
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
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/publicKeys");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [V1Actions::get_recaptcha_params()](struct.V1Actions.html#method.get_recaptcha_params)"]
        #[derive(Debug, Clone)]
        pub struct GetRecaptchaParamsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetRecaptchaParamsRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetRecaptchaParamResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetRecaptchaParamResponse,
                crate::Error,
            > {
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
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/recaptchaParams");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
        #[doc = "Created via [V1Actions::get_session_cookie_public_keys()](struct.V1Actions.html#method.get_session_cookie_public_keys)"]
        #[derive(Debug, Clone)]
        pub struct GetSessionCookiePublicKeysRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a> GetSessionCookiePublicKeysRequestBuilder<'a> {
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
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetSessionCookiePublicKeysResponse,
                crate::Error,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudIdentitytoolkitV1GetSessionCookiePublicKeysResponse,
                crate::Error,
            > {
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
                let mut output = "https://identitytoolkit.googleapis.com/".to_owned();
                output.push_str("v1/sessionCookiePublicKeys");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
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
