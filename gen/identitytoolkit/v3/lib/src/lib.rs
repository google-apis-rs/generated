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
    pub struct CreateAuthUriResponse {
        #[doc = "all providers the user has once used to do federated login"]
        #[serde(rename = "allProviders", default)]
        pub all_providers: ::std::option::Option<Vec<String>>,
        #[doc = "The URI used by the IDP to authenticate the user."]
        #[serde(rename = "authUri", default)]
        pub auth_uri: ::std::option::Option<String>,
        #[doc = "True if captcha is required."]
        #[serde(rename = "captchaRequired", default)]
        pub captcha_required: ::std::option::Option<bool>,
        #[doc = "True if the authUri is for user's existing provider."]
        #[serde(rename = "forExistingProvider", default)]
        pub for_existing_provider: ::std::option::Option<bool>,
        #[doc = "The fixed string identitytoolkit#CreateAuthUriResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The provider ID of the auth URI."]
        #[serde(rename = "providerId", default)]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "Whether the user is registered if the identifier is an email."]
        #[serde(rename = "registered", default)]
        pub registered: ::std::option::Option<bool>,
        #[doc = "Session ID which should be passed in the following verifyAssertion request."]
        #[serde(rename = "sessionId", default)]
        pub session_id: ::std::option::Option<String>,
        #[doc = "All sign-in methods this user has used."]
        #[serde(rename = "signinMethods", default)]
        pub signin_methods: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for CreateAuthUriResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateAuthUriResponse {
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
    pub struct DeleteAccountResponse {
        #[doc = "The fixed string \"identitytoolkit#DeleteAccountResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteAccountResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteAccountResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DownloadAccountResponse {
        #[doc = "The fixed string \"identitytoolkit#DownloadAccountResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The next page token. To be used in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The user accounts data."]
        #[serde(rename = "users", default)]
        pub users: ::std::option::Option<Vec<crate::schemas::UserInfo>>,
    }
    impl ::google_field_selector::FieldSelector for DownloadAccountResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DownloadAccountResponse {
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
    pub struct EmailLinkSigninResponse {
        #[doc = "The user's email."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "Expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "The STS id token to login the newly signed in user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Whether the user is new."]
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: ::std::option::Option<bool>,
        #[doc = "The fixed string \"identitytoolkit#EmailLinkSigninResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The RP local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The refresh token for the signed in user."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EmailLinkSigninResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmailLinkSigninResponse {
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
    pub struct EmailTemplate {
        #[doc = "Email body."]
        #[serde(rename = "body", default)]
        pub body: ::std::option::Option<String>,
        #[doc = "Email body format."]
        #[serde(rename = "format", default)]
        pub format: ::std::option::Option<String>,
        #[doc = "From address of the email."]
        #[serde(rename = "from", default)]
        pub from: ::std::option::Option<String>,
        #[doc = "From display name."]
        #[serde(rename = "fromDisplayName", default)]
        pub from_display_name: ::std::option::Option<String>,
        #[doc = "Reply-to address."]
        #[serde(rename = "replyTo", default)]
        pub reply_to: ::std::option::Option<String>,
        #[doc = "Subject of the email."]
        #[serde(rename = "subject", default)]
        pub subject: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EmailTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmailTemplate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GetAccountInfoResponse {
        #[doc = "The fixed string \"identitytoolkit#GetAccountInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The info of the users."]
        #[serde(rename = "users", default)]
        pub users: ::std::option::Option<Vec<crate::schemas::UserInfo>>,
    }
    impl ::google_field_selector::FieldSelector for GetAccountInfoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetAccountInfoResponse {
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
    pub struct GetOobConfirmationCodeResponse {
        #[doc = "The email address that the email is sent to."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The fixed string \"identitytoolkit#GetOobConfirmationCodeResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The code to be send to the user."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GetOobConfirmationCodeResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetOobConfirmationCodeResponse {
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
    pub struct GetRecaptchaParamResponse {
        #[doc = "The fixed string \"identitytoolkit#GetRecaptchaParamResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Site key registered at recaptcha."]
        #[serde(rename = "recaptchaSiteKey", default)]
        pub recaptcha_site_key: ::std::option::Option<String>,
        #[doc = "The stoken field for the recaptcha widget, used to request captcha challenge."]
        #[serde(rename = "recaptchaStoken", default)]
        pub recaptcha_stoken: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GetRecaptchaParamResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetRecaptchaParamResponse {
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
    pub struct IdentitytoolkitRelyingpartyCreateAuthUriRequest {
        #[doc = "The app ID of the mobile app, base64(CERT_SHA1):PACKAGE_NAME for Android, BUNDLE_ID for iOS."]
        #[serde(rename = "appId", default)]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Explicitly specify the auth flow type. Currently only support \"CODE_FLOW\" type. The field is only used for Google provider."]
        #[serde(rename = "authFlowType", default)]
        pub auth_flow_type: ::std::option::Option<String>,
        #[doc = "The relying party OAuth client ID."]
        #[serde(rename = "clientId", default)]
        pub client_id: ::std::option::Option<String>,
        #[doc = "The opaque value used by the client to maintain context info between the authentication request and the IDP callback."]
        #[serde(rename = "context", default)]
        pub context: ::std::option::Option<String>,
        #[doc = "The URI to which the IDP redirects the user after the federated login flow."]
        #[serde(rename = "continueUri", default)]
        pub continue_uri: ::std::option::Option<String>,
        #[doc = "The query parameter that client can customize by themselves in auth url. The following parameters are reserved for server so that they cannot be customized by clients: client_id, response_type, scope, redirect_uri, state, oauth_token."]
        #[serde(rename = "customParameter", default)]
        pub custom_parameter: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The hosted domain to restrict sign-in to accounts at that domain for Google Apps hosted accounts."]
        #[serde(rename = "hostedDomain", default)]
        pub hosted_domain: ::std::option::Option<String>,
        #[doc = "The email or federated ID of the user."]
        #[serde(rename = "identifier", default)]
        pub identifier: ::std::option::Option<String>,
        #[doc = "The developer's consumer key for OpenId OAuth Extension"]
        #[serde(rename = "oauthConsumerKey", default)]
        pub oauth_consumer_key: ::std::option::Option<String>,
        #[doc = "Additional oauth scopes, beyond the basid user profile, that the user would be prompted to grant"]
        #[serde(rename = "oauthScope", default)]
        pub oauth_scope: ::std::option::Option<String>,
        #[doc = "Optional realm for OpenID protocol. The sub string \"scheme://domain:port\" of the param \"continueUri\" is used if this is not set."]
        #[serde(rename = "openidRealm", default)]
        pub openid_realm: ::std::option::Option<String>,
        #[doc = "The native app package for OTA installation."]
        #[serde(rename = "otaApp", default)]
        pub ota_app: ::std::option::Option<String>,
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        #[serde(rename = "providerId", default)]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "The session_id passed by client."]
        #[serde(rename = "sessionId", default)]
        pub session_id: ::std::option::Option<String>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyCreateAuthUriRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyCreateAuthUriRequest {
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
    pub struct IdentitytoolkitRelyingpartyDeleteAccountRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The GITKit token or STS id token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyDeleteAccountRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyDeleteAccountRequest {
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
    pub struct IdentitytoolkitRelyingpartyDownloadAccountRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The max number of results to return in the response."]
        #[serde(rename = "maxResults", default)]
        pub max_results: ::std::option::Option<u32>,
        #[doc = "The token for the next page. This should be taken from the previous response."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Specify which project (field value is actually project id) to operate. Only used when provided credential."]
        #[serde(rename = "targetProjectId", default)]
        pub target_project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyDownloadAccountRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyDownloadAccountRequest {
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
    pub struct IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
        #[doc = "The email address of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "Token for linking flow."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The confirmation code."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyEmailLinkSigninRequest {
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
    pub struct IdentitytoolkitRelyingpartyGetAccountInfoRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The list of emails of the users to inquiry."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<Vec<String>>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The list of local ID's of the users to inquiry."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<Vec<String>>,
        #[doc = "Privileged caller can query users by specified phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyGetAccountInfoRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyGetAccountInfoRequest {
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
    pub struct IdentitytoolkitRelyingpartyGetProjectConfigResponse {
        #[doc = "Whether to allow password user sign in or sign up."]
        #[serde(rename = "allowPasswordUser", default)]
        pub allow_password_user: ::std::option::Option<bool>,
        #[doc = "Browser API key, needed when making http request to Apiary."]
        #[serde(rename = "apiKey", default)]
        pub api_key: ::std::option::Option<String>,
        #[doc = "Authorized domains."]
        #[serde(rename = "authorizedDomains", default)]
        pub authorized_domains: ::std::option::Option<Vec<String>>,
        #[doc = "Change email template."]
        #[serde(rename = "changeEmailTemplate", default)]
        pub change_email_template: ::std::option::Option<crate::schemas::EmailTemplate>,
        #[serde(rename = "dynamicLinksDomain", default)]
        pub dynamic_links_domain: ::std::option::Option<String>,
        #[doc = "Whether anonymous user is enabled."]
        #[serde(rename = "enableAnonymousUser", default)]
        pub enable_anonymous_user: ::std::option::Option<bool>,
        #[doc = "OAuth2 provider configuration."]
        #[serde(rename = "idpConfig", default)]
        pub idp_config: ::std::option::Option<Vec<crate::schemas::IdpConfig>>,
        #[doc = "Legacy reset password email template."]
        #[serde(rename = "legacyResetPasswordTemplate", default)]
        pub legacy_reset_password_template: ::std::option::Option<crate::schemas::EmailTemplate>,
        #[doc = "Project ID of the relying party."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Reset password email template."]
        #[serde(rename = "resetPasswordTemplate", default)]
        pub reset_password_template: ::std::option::Option<crate::schemas::EmailTemplate>,
        #[doc = "Whether to use email sending provided by Firebear."]
        #[serde(rename = "useEmailSending", default)]
        pub use_email_sending: ::std::option::Option<bool>,
        #[doc = "Verify email template."]
        #[serde(rename = "verifyEmailTemplate", default)]
        pub verify_email_template: ::std::option::Option<crate::schemas::EmailTemplate>,
    }
    impl ::google_field_selector::FieldSelector
        for IdentitytoolkitRelyingpartyGetProjectConfigResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyGetProjectConfigResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    pub type IdentitytoolkitRelyingpartyGetPublicKeysResponse =
        ::std::collections::BTreeMap<String, String>;
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
    pub struct IdentitytoolkitRelyingpartyResetPasswordRequest {
        #[doc = "The email address of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The new password inputted by the user."]
        #[serde(rename = "newPassword", default)]
        pub new_password: ::std::option::Option<String>,
        #[doc = "The old password inputted by the user."]
        #[serde(rename = "oldPassword", default)]
        pub old_password: ::std::option::Option<String>,
        #[doc = "The confirmation code."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyResetPasswordRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyResetPasswordRequest {
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
    pub struct IdentitytoolkitRelyingpartySendVerificationCodeRequest {
        #[doc = "Receipt of successful app token validation with APNS."]
        #[serde(rename = "iosReceipt", default)]
        pub ios_receipt: ::std::option::Option<String>,
        #[doc = "Secret delivered to iOS app via APNS."]
        #[serde(rename = "iosSecret", default)]
        pub ios_secret: ::std::option::Option<String>,
        #[doc = "The phone number to send the verification code to in E.164 format."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "Recaptcha solution."]
        #[serde(rename = "recaptchaToken", default)]
        pub recaptcha_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for IdentitytoolkitRelyingpartySendVerificationCodeRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for IdentitytoolkitRelyingpartySendVerificationCodeRequest
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
    pub struct IdentitytoolkitRelyingpartySendVerificationCodeResponse {
        #[doc = "Encrypted session information"]
        #[serde(rename = "sessionInfo", default)]
        pub session_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for IdentitytoolkitRelyingpartySendVerificationCodeResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for IdentitytoolkitRelyingpartySendVerificationCodeResponse
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
    pub struct IdentitytoolkitRelyingpartySetAccountInfoRequest {
        #[doc = "The captcha challenge."]
        #[serde(rename = "captchaChallenge", default)]
        pub captcha_challenge: ::std::option::Option<String>,
        #[doc = "Response to the captcha."]
        #[serde(rename = "captchaResponse", default)]
        pub captcha_response: ::std::option::Option<String>,
        #[doc = "The timestamp when the account is created."]
        #[serde(rename = "createdAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub created_at: ::std::option::Option<i64>,
        #[doc = "The custom attributes to be set in the user's id token."]
        #[serde(rename = "customAttributes", default)]
        pub custom_attributes: ::std::option::Option<String>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The attributes users request to delete."]
        #[serde(rename = "deleteAttribute", default)]
        pub delete_attribute: ::std::option::Option<Vec<String>>,
        #[doc = "The IDPs the user request to delete."]
        #[serde(rename = "deleteProvider", default)]
        pub delete_provider: ::std::option::Option<Vec<String>>,
        #[doc = "Whether to disable the user."]
        #[serde(rename = "disableUser", default)]
        pub disable_user: ::std::option::Option<bool>,
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "Mark the email as verified or not."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "Last login timestamp."]
        #[serde(rename = "lastLoginAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_login_at: ::std::option::Option<i64>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The out-of-band code of the change email request."]
        #[serde(rename = "oobCode", default)]
        pub oob_code: ::std::option::Option<String>,
        #[doc = "The new password of the user."]
        #[serde(rename = "password", default)]
        pub password: ::std::option::Option<String>,
        #[doc = "Privileged caller can update user with specified phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The photo url of the user."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The associated IDPs of the user."]
        #[serde(rename = "provider", default)]
        pub provider: ::std::option::Option<Vec<String>>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: ::std::option::Option<bool>,
        #[doc = "Mark the user to upgrade to federated login."]
        #[serde(rename = "upgradeToFederatedLogin", default)]
        pub upgrade_to_federated_login: ::std::option::Option<bool>,
        #[doc = "Timestamp in seconds for valid login token."]
        #[serde(rename = "validSince", default)]
        #[serde(with = "crate::parsed_string")]
        pub valid_since: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartySetAccountInfoRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartySetAccountInfoRequest {
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
    pub struct IdentitytoolkitRelyingpartySetProjectConfigRequest {
        #[doc = "Whether to allow password user sign in or sign up."]
        #[serde(rename = "allowPasswordUser", default)]
        pub allow_password_user: ::std::option::Option<bool>,
        #[doc = "Browser API key, needed when making http request to Apiary."]
        #[serde(rename = "apiKey", default)]
        pub api_key: ::std::option::Option<String>,
        #[doc = "Authorized domains for widget redirect."]
        #[serde(rename = "authorizedDomains", default)]
        pub authorized_domains: ::std::option::Option<Vec<String>>,
        #[doc = "Change email template."]
        #[serde(rename = "changeEmailTemplate", default)]
        pub change_email_template: ::std::option::Option<crate::schemas::EmailTemplate>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "Whether to enable anonymous user."]
        #[serde(rename = "enableAnonymousUser", default)]
        pub enable_anonymous_user: ::std::option::Option<bool>,
        #[doc = "Oauth2 provider configuration."]
        #[serde(rename = "idpConfig", default)]
        pub idp_config: ::std::option::Option<Vec<crate::schemas::IdpConfig>>,
        #[doc = "Legacy reset password email template."]
        #[serde(rename = "legacyResetPasswordTemplate", default)]
        pub legacy_reset_password_template: ::std::option::Option<crate::schemas::EmailTemplate>,
        #[doc = "Reset password email template."]
        #[serde(rename = "resetPasswordTemplate", default)]
        pub reset_password_template: ::std::option::Option<crate::schemas::EmailTemplate>,
        #[doc = "Whether to use email sending provided by Firebear."]
        #[serde(rename = "useEmailSending", default)]
        pub use_email_sending: ::std::option::Option<bool>,
        #[doc = "Verify email template."]
        #[serde(rename = "verifyEmailTemplate", default)]
        pub verify_email_template: ::std::option::Option<crate::schemas::EmailTemplate>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartySetProjectConfigRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartySetProjectConfigRequest {
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
    pub struct IdentitytoolkitRelyingpartySetProjectConfigResponse {
        #[doc = "Project ID of the relying party."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for IdentitytoolkitRelyingpartySetProjectConfigResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartySetProjectConfigResponse {
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
    pub struct IdentitytoolkitRelyingpartySignOutUserRequest {
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartySignOutUserRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartySignOutUserRequest {
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
    pub struct IdentitytoolkitRelyingpartySignOutUserResponse {
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartySignOutUserResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartySignOutUserResponse {
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
    pub struct IdentitytoolkitRelyingpartySignupNewUserRequest {
        #[doc = "The captcha challenge."]
        #[serde(rename = "captchaChallenge", default)]
        pub captcha_challenge: ::std::option::Option<String>,
        #[doc = "Response to the captcha."]
        #[serde(rename = "captchaResponse", default)]
        pub captcha_response: ::std::option::Option<String>,
        #[doc = "Whether to disable the user. Only can be used by service account."]
        #[serde(rename = "disabled", default)]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "Mark the email as verified or not. Only can be used by service account."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "Privileged caller can create user with specified user id."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The new password of the user."]
        #[serde(rename = "password", default)]
        pub password: ::std::option::Option<String>,
        #[doc = "Privileged caller can create user with specified phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The photo url of the user."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartySignupNewUserRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartySignupNewUserRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct IdentitytoolkitRelyingpartyUploadAccountRequest {
        #[doc = "Whether allow overwrite existing account when user local_id exists."]
        #[serde(rename = "allowOverwrite", default)]
        pub allow_overwrite: ::std::option::Option<bool>,
        #[serde(rename = "blockSize", default)]
        pub block_size: ::std::option::Option<i32>,
        #[doc = "The following 4 fields are for standard scrypt algorithm."]
        #[serde(rename = "cpuMemCost", default)]
        pub cpu_mem_cost: ::std::option::Option<i32>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[serde(rename = "dkLen", default)]
        pub dk_len: ::std::option::Option<i32>,
        #[doc = "The password hash algorithm."]
        #[serde(rename = "hashAlgorithm", default)]
        pub hash_algorithm: ::std::option::Option<String>,
        #[doc = "Memory cost for hash calculation. Used by scrypt similar algorithms."]
        #[serde(rename = "memoryCost", default)]
        pub memory_cost: ::std::option::Option<i32>,
        #[serde(rename = "parallelization", default)]
        pub parallelization: ::std::option::Option<i32>,
        #[doc = "Rounds for hash calculation. Used by scrypt and similar algorithms."]
        #[serde(rename = "rounds", default)]
        pub rounds: ::std::option::Option<i32>,
        #[doc = "The salt separator."]
        #[serde(rename = "saltSeparator", default)]
        pub salt_separator: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "If true, backend will do sanity check(including duplicate email and federated id) when uploading account."]
        #[serde(rename = "sanityCheck", default)]
        pub sanity_check: ::std::option::Option<bool>,
        #[doc = "The key for to hash the password."]
        #[serde(rename = "signerKey", default)]
        pub signer_key: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Specify which project (field value is actually project id) to operate. Only used when provided credential."]
        #[serde(rename = "targetProjectId", default)]
        pub target_project_id: ::std::option::Option<String>,
        #[doc = "The account info to be stored."]
        #[serde(rename = "users", default)]
        pub users: ::std::option::Option<Vec<crate::schemas::UserInfo>>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyUploadAccountRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyUploadAccountRequest {
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
    pub struct IdentitytoolkitRelyingpartyVerifyAssertionRequest {
        #[doc = "When it's true, automatically creates a new account if the user doesn't exist. When it's false, allows existing user to sign in normally and throws exception if the user doesn't exist."]
        #[serde(rename = "autoCreate", default)]
        pub auto_create: ::std::option::Option<bool>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "The GITKit token for the non-trusted IDP pending to be confirmed by the user."]
        #[serde(rename = "pendingIdToken", default)]
        pub pending_id_token: ::std::option::Option<String>,
        #[doc = "The post body if the request is a HTTP POST."]
        #[serde(rename = "postBody", default)]
        pub post_body: ::std::option::Option<String>,
        #[doc = "The URI to which the IDP redirects the user back. It may contain federated login result params added by the IDP."]
        #[serde(rename = "requestUri", default)]
        pub request_uri: ::std::option::Option<String>,
        #[doc = "Whether return 200 and IDP credential rather than throw exception when federated id is already linked."]
        #[serde(rename = "returnIdpCredential", default)]
        pub return_idp_credential: ::std::option::Option<bool>,
        #[doc = "Whether to return refresh tokens."]
        #[serde(rename = "returnRefreshToken", default)]
        pub return_refresh_token: ::std::option::Option<bool>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: ::std::option::Option<bool>,
        #[doc = "Session ID, which should match the one in previous createAuthUri request."]
        #[serde(rename = "sessionId", default)]
        pub session_id: ::std::option::Option<String>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyVerifyAssertionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyVerifyAssertionRequest {
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
    pub struct IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: ::std::option::Option<bool>,
        #[doc = "The custom token to verify"]
        #[serde(rename = "token", default)]
        pub token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for IdentitytoolkitRelyingpartyVerifyCustomTokenRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyVerifyCustomTokenRequest {
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
    pub struct IdentitytoolkitRelyingpartyVerifyPasswordRequest {
        #[doc = "The captcha challenge."]
        #[serde(rename = "captchaChallenge", default)]
        pub captcha_challenge: ::std::option::Option<String>,
        #[doc = "Response to the captcha."]
        #[serde(rename = "captchaResponse", default)]
        pub captcha_response: ::std::option::Option<String>,
        #[doc = "GCP project number of the requesting delegated app. Currently only intended for Firebase V1 migration."]
        #[serde(rename = "delegatedProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub delegated_project_number: ::std::option::Option<i64>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The GITKit token of the authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "Instance id token of the app."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "The password inputed by the user."]
        #[serde(rename = "password", default)]
        pub password: ::std::option::Option<String>,
        #[doc = "The GITKit token for the non-trusted IDP, which is to be confirmed by the user."]
        #[serde(rename = "pendingIdToken", default)]
        pub pending_id_token: ::std::option::Option<String>,
        #[doc = "Whether return sts id token and refresh token instead of gitkit token."]
        #[serde(rename = "returnSecureToken", default)]
        pub return_secure_token: ::std::option::Option<bool>,
        #[doc = "For multi-tenant use cases, in order to construct sign-in URL with the correct IDP parameters, Firebear needs to know which Tenant to retrieve IDP configs from."]
        #[serde(rename = "tenantId", default)]
        pub tenant_id: ::std::option::Option<String>,
        #[doc = "Tenant project number to be used for idp discovery."]
        #[serde(rename = "tenantProjectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub tenant_project_number: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for IdentitytoolkitRelyingpartyVerifyPasswordRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyVerifyPasswordRequest {
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
    pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<String>,
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[serde(rename = "operation", default)]
        pub operation: ::std::option::Option<String>,
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The session info previously returned by IdentityToolkit-SendVerificationCode."]
        #[serde(rename = "sessionInfo", default)]
        pub session_info: ::std::option::Option<String>,
        #[serde(rename = "temporaryProof", default)]
        pub temporary_proof: ::std::option::Option<String>,
        #[serde(rename = "verificationProof", default)]
        pub verification_proof: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest {
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
    pub struct IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: ::std::option::Option<bool>,
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: ::std::option::Option<String>,
        #[serde(rename = "temporaryProof", default)]
        pub temporary_proof: ::std::option::Option<String>,
        #[serde(rename = "temporaryProofExpiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub temporary_proof_expires_in: ::std::option::Option<i64>,
        #[serde(rename = "verificationProof", default)]
        pub verification_proof: ::std::option::Option<String>,
        #[serde(rename = "verificationProofExpiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub verification_proof_expires_in: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse {
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
    pub struct IdpConfig {
        #[doc = "OAuth2 client ID."]
        #[serde(rename = "clientId", default)]
        pub client_id: ::std::option::Option<String>,
        #[doc = "Whether this IDP is enabled."]
        #[serde(rename = "enabled", default)]
        pub enabled: ::std::option::Option<bool>,
        #[doc = "Percent of users who will be prompted/redirected federated login for this IDP."]
        #[serde(rename = "experimentPercent", default)]
        pub experiment_percent: ::std::option::Option<i32>,
        #[doc = "OAuth2 provider."]
        #[serde(rename = "provider", default)]
        pub provider: ::std::option::Option<String>,
        #[doc = "OAuth2 client secret."]
        #[serde(rename = "secret", default)]
        pub secret: ::std::option::Option<String>,
        #[doc = "Whitelisted client IDs for audience check."]
        #[serde(rename = "whitelistedAudiences", default)]
        pub whitelisted_audiences: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for IdpConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdpConfig {
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
    pub struct Relyingparty {
        #[doc = "whether or not to install the android app on the device where the link is opened"]
        #[serde(rename = "androidInstallApp", default)]
        pub android_install_app: ::std::option::Option<bool>,
        #[doc = "minimum version of the app. if the version on the device is lower than this version then the user is taken to the play store to upgrade the app"]
        #[serde(rename = "androidMinimumVersion", default)]
        pub android_minimum_version: ::std::option::Option<String>,
        #[doc = "android package name of the android app to handle the action code"]
        #[serde(rename = "androidPackageName", default)]
        pub android_package_name: ::std::option::Option<String>,
        #[doc = "whether or not the app can handle the oob code without first going to web"]
        #[serde(rename = "canHandleCodeInApp", default)]
        pub can_handle_code_in_app: ::std::option::Option<bool>,
        #[doc = "The recaptcha response from the user."]
        #[serde(rename = "captchaResp", default)]
        pub captcha_resp: ::std::option::Option<String>,
        #[doc = "The recaptcha challenge presented to the user."]
        #[serde(rename = "challenge", default)]
        pub challenge: ::std::option::Option<String>,
        #[doc = "The url to continue to the Gitkit app"]
        #[serde(rename = "continueUrl", default)]
        pub continue_url: ::std::option::Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "iOS app store id to download the app if it's not already installed"]
        #[serde(rename = "iOSAppStoreId", default)]
        pub i_os_app_store_id: ::std::option::Option<String>,
        #[doc = "the iOS bundle id of iOS app to handle the action code"]
        #[serde(rename = "iOSBundleId", default)]
        pub i_os_bundle_id: ::std::option::Option<String>,
        #[doc = "The user's Gitkit login token for email change."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The fixed string \"identitytoolkit#relyingparty\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The new email if the code is for email change."]
        #[serde(rename = "newEmail", default)]
        pub new_email: ::std::option::Option<String>,
        #[doc = "The request type."]
        #[serde(rename = "requestType", default)]
        pub request_type: ::std::option::Option<String>,
        #[doc = "The IP address of the user."]
        #[serde(rename = "userIp", default)]
        pub user_ip: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Relyingparty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Relyingparty {
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
    pub struct ResetPasswordResponse {
        #[doc = "The user's email. If the out-of-band code is for email recovery, the user's original email."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The fixed string \"identitytoolkit#ResetPasswordResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "If the out-of-band code is for email recovery, the user's new email."]
        #[serde(rename = "newEmail", default)]
        pub new_email: ::std::option::Option<String>,
        #[doc = "The request type."]
        #[serde(rename = "requestType", default)]
        pub request_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResetPasswordResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResetPasswordResponse {
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
    pub struct SetAccountInfoResponse {
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "If email has been verified."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "The Gitkit id token to login the newly sign up user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The fixed string \"identitytoolkit#SetAccountInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The new email the user attempts to change to."]
        #[serde(rename = "newEmail", default)]
        pub new_email: ::std::option::Option<String>,
        #[doc = "The user's hashed password."]
        #[serde(rename = "passwordHash", default)]
        pub password_hash: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The photo url of the user."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The user's profiles at the associated IdPs."]
        #[serde(rename = "providerUserInfo", default)]
        pub provider_user_info:
            ::std::option::Option<Vec<crate::schemas::SetAccountInfoResponseProviderUserInfoItems>>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SetAccountInfoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetAccountInfoResponse {
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
    pub struct SetAccountInfoResponseProviderUserInfoItems {
        #[doc = "The user's display name at the IDP."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "User's identifier at IDP."]
        #[serde(rename = "federatedId", default)]
        pub federated_id: ::std::option::Option<String>,
        #[doc = "The user's photo url at the IDP."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The IdP ID. For whitelisted IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        #[serde(rename = "providerId", default)]
        pub provider_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SetAccountInfoResponseProviderUserInfoItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetAccountInfoResponseProviderUserInfoItems {
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
    pub struct SignupNewUserResponse {
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "The Gitkit id token to login the newly sign up user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The fixed string \"identitytoolkit#SignupNewUserResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The RP local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SignupNewUserResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SignupNewUserResponse {
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
    pub struct UploadAccountResponse {
        #[doc = "The error encountered while processing the account info."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<Vec<crate::schemas::UploadAccountResponseErrorItems>>,
        #[doc = "The fixed string \"identitytoolkit#UploadAccountResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UploadAccountResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UploadAccountResponse {
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
    pub struct UploadAccountResponseErrorItems {
        #[doc = "The index of the malformed account, starting from 0."]
        #[serde(rename = "index", default)]
        pub index: ::std::option::Option<i32>,
        #[doc = "Detailed error message for the account info."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UploadAccountResponseErrorItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UploadAccountResponseErrorItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UserInfo {
        #[doc = "User creation timestamp."]
        #[serde(rename = "createdAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub created_at: ::std::option::Option<i64>,
        #[doc = "The custom attributes to be set in the user's id token."]
        #[serde(rename = "customAttributes", default)]
        pub custom_attributes: ::std::option::Option<String>,
        #[doc = "Whether the user is authenticated by the developer."]
        #[serde(rename = "customAuth", default)]
        pub custom_auth: ::std::option::Option<bool>,
        #[doc = "Whether the user is disabled."]
        #[serde(rename = "disabled", default)]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email of the user."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "Whether the email has been verified."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "last login timestamp."]
        #[serde(rename = "lastLoginAt", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_login_at: ::std::option::Option<i64>,
        #[doc = "The local ID of the user."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The user's hashed password."]
        #[serde(rename = "passwordHash", default)]
        pub password_hash: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The timestamp when the password was last updated."]
        #[serde(rename = "passwordUpdatedAt", default)]
        pub password_updated_at: ::std::option::Option<f64>,
        #[doc = "User's phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The URL of the user profile photo."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The IDP of the user."]
        #[serde(rename = "providerUserInfo", default)]
        pub provider_user_info:
            ::std::option::Option<Vec<crate::schemas::UserInfoProviderUserInfoItems>>,
        #[doc = "The user's plain text password."]
        #[serde(rename = "rawPassword", default)]
        pub raw_password: ::std::option::Option<String>,
        #[doc = "The user's password salt."]
        #[serde(rename = "salt", default)]
        pub salt: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "User's screen name at Twitter or login name at Github."]
        #[serde(rename = "screenName", default)]
        pub screen_name: ::std::option::Option<String>,
        #[doc = "Timestamp in seconds for valid login token."]
        #[serde(rename = "validSince", default)]
        #[serde(with = "crate::parsed_string")]
        pub valid_since: ::std::option::Option<i64>,
        #[doc = "Version of the user's password."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserInfo {
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
    pub struct UserInfoProviderUserInfoItems {
        #[doc = "The user's display name at the IDP."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "User's email at IDP."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "User's identifier at IDP."]
        #[serde(rename = "federatedId", default)]
        pub federated_id: ::std::option::Option<String>,
        #[doc = "User's phone number."]
        #[serde(rename = "phoneNumber", default)]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "The user's photo url at the IDP."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name, e.g., google.com, aol.com, live.net and yahoo.com. For other OpenID IdPs it's the OP identifier."]
        #[serde(rename = "providerId", default)]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "User's raw identifier directly returned from IDP."]
        #[serde(rename = "rawId", default)]
        pub raw_id: ::std::option::Option<String>,
        #[doc = "User's screen name at Twitter or login name at Github."]
        #[serde(rename = "screenName", default)]
        pub screen_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UserInfoProviderUserInfoItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserInfoProviderUserInfoItems {
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
    pub struct VerifyAssertionResponse {
        #[doc = "The action code."]
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<String>,
        #[doc = "URL for OTA app installation."]
        #[serde(rename = "appInstallationUrl", default)]
        pub app_installation_url: ::std::option::Option<String>,
        #[doc = "The custom scheme used by mobile app."]
        #[serde(rename = "appScheme", default)]
        pub app_scheme: ::std::option::Option<String>,
        #[doc = "The opaque value used by the client to maintain context info between the authentication request and the IDP callback."]
        #[serde(rename = "context", default)]
        pub context: ::std::option::Option<String>,
        #[doc = "The birth date of the IdP account."]
        #[serde(rename = "dateOfBirth", default)]
        pub date_of_birth: ::std::option::Option<String>,
        #[doc = "The display name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email returned by the IdP. NOTE: The federated login user may not own the email."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "It's true if the email is recycled."]
        #[serde(rename = "emailRecycled", default)]
        pub email_recycled: ::std::option::Option<bool>,
        #[doc = "The value is true if the IDP is also the email provider. It means the user owns the email."]
        #[serde(rename = "emailVerified", default)]
        pub email_verified: ::std::option::Option<bool>,
        #[doc = "Client error code."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "The unique ID identifies the IdP account."]
        #[serde(rename = "federatedId", default)]
        pub federated_id: ::std::option::Option<String>,
        #[doc = "The first name of the user."]
        #[serde(rename = "firstName", default)]
        pub first_name: ::std::option::Option<String>,
        #[doc = "The full name of the user."]
        #[serde(rename = "fullName", default)]
        pub full_name: ::std::option::Option<String>,
        #[doc = "The ID token."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "It's the identifier param in the createAuthUri request if the identifier is an email. It can be used to check whether the user input email is different from the asserted email."]
        #[serde(rename = "inputEmail", default)]
        pub input_email: ::std::option::Option<String>,
        #[doc = "True if it's a new user sign-in, false if it's a returning user."]
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: ::std::option::Option<bool>,
        #[doc = "The fixed string \"identitytoolkit#VerifyAssertionResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The language preference of the user."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "The last name of the user."]
        #[serde(rename = "lastName", default)]
        pub last_name: ::std::option::Option<String>,
        #[doc = "The RP local ID if it's already been mapped to the IdP account identified by the federated ID."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "Whether the assertion is from a non-trusted IDP and need account linking confirmation."]
        #[serde(rename = "needConfirmation", default)]
        pub need_confirmation: ::std::option::Option<bool>,
        #[doc = "Whether need client to supply email to complete the federated login flow."]
        #[serde(rename = "needEmail", default)]
        pub need_email: ::std::option::Option<bool>,
        #[doc = "The nick name of the user."]
        #[serde(rename = "nickName", default)]
        pub nick_name: ::std::option::Option<String>,
        #[doc = "The OAuth2 access token."]
        #[serde(rename = "oauthAccessToken", default)]
        pub oauth_access_token: ::std::option::Option<String>,
        #[doc = "The OAuth2 authorization code."]
        #[serde(rename = "oauthAuthorizationCode", default)]
        pub oauth_authorization_code: ::std::option::Option<String>,
        #[doc = "The lifetime in seconds of the OAuth2 access token."]
        #[serde(rename = "oauthExpireIn", default)]
        pub oauth_expire_in: ::std::option::Option<i32>,
        #[doc = "The OIDC id token."]
        #[serde(rename = "oauthIdToken", default)]
        pub oauth_id_token: ::std::option::Option<String>,
        #[doc = "The user approved request token for the OpenID OAuth extension."]
        #[serde(rename = "oauthRequestToken", default)]
        pub oauth_request_token: ::std::option::Option<String>,
        #[doc = "The scope for the OpenID OAuth extension."]
        #[serde(rename = "oauthScope", default)]
        pub oauth_scope: ::std::option::Option<String>,
        #[doc = "The OAuth1 access token secret."]
        #[serde(rename = "oauthTokenSecret", default)]
        pub oauth_token_secret: ::std::option::Option<String>,
        #[doc = "The original email stored in the mapping storage. It's returned when the federated ID is associated to a different email."]
        #[serde(rename = "originalEmail", default)]
        pub original_email: ::std::option::Option<String>,
        #[doc = "The URI of the public accessible profiel picture."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The IdP ID. For white listed IdPs it's a short domain name e.g. google.com, aol.com, live.net and yahoo.com. If the \"providerId\" param is set to OpenID OP identifer other than the whilte listed IdPs the OP identifier is returned. If the \"identifier\" param is federated ID in the createAuthUri request. The domain part of the federated ID is returned."]
        #[serde(rename = "providerId", default)]
        pub provider_id: ::std::option::Option<String>,
        #[doc = "Raw IDP-returned user info."]
        #[serde(rename = "rawUserInfo", default)]
        pub raw_user_info: ::std::option::Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: ::std::option::Option<String>,
        #[doc = "The screen_name of a Twitter user or the login name at Github."]
        #[serde(rename = "screenName", default)]
        pub screen_name: ::std::option::Option<String>,
        #[doc = "The timezone of the user."]
        #[serde(rename = "timeZone", default)]
        pub time_zone: ::std::option::Option<String>,
        #[doc = "When action is 'map', contains the idps which can be used for confirmation."]
        #[serde(rename = "verifiedProvider", default)]
        pub verified_provider: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for VerifyAssertionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VerifyAssertionResponse {
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
    pub struct VerifyCustomTokenResponse {
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "The GITKit token for authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "True if it's a new user sign-in, false if it's a returning user."]
        #[serde(rename = "isNewUser", default)]
        pub is_new_user: ::std::option::Option<bool>,
        #[doc = "The fixed string \"identitytoolkit#VerifyCustomTokenResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VerifyCustomTokenResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VerifyCustomTokenResponse {
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
    pub struct VerifyPasswordResponse {
        #[doc = "The name of the user."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The email returned by the IdP. NOTE: The federated login user may not own the email."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "If idToken is STS id token, then this field will be expiration time of STS id token in seconds."]
        #[serde(rename = "expiresIn", default)]
        #[serde(with = "crate::parsed_string")]
        pub expires_in: ::std::option::Option<i64>,
        #[doc = "The GITKit token for authenticated user."]
        #[serde(rename = "idToken", default)]
        pub id_token: ::std::option::Option<String>,
        #[doc = "The fixed string \"identitytoolkit#VerifyPasswordResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The RP local ID if it's already been mapped to the IdP account identified by the federated ID."]
        #[serde(rename = "localId", default)]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The OAuth2 access token."]
        #[serde(rename = "oauthAccessToken", default)]
        pub oauth_access_token: ::std::option::Option<String>,
        #[doc = "The OAuth2 authorization code."]
        #[serde(rename = "oauthAuthorizationCode", default)]
        pub oauth_authorization_code: ::std::option::Option<String>,
        #[doc = "The lifetime in seconds of the OAuth2 access token."]
        #[serde(rename = "oauthExpireIn", default)]
        pub oauth_expire_in: ::std::option::Option<i32>,
        #[doc = "The URI of the user's photo at IdP"]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "If idToken is STS id token, then this field will be refresh token."]
        #[serde(rename = "refreshToken", default)]
        pub refresh_token: ::std::option::Option<String>,
        #[doc = "Whether the email is registered."]
        #[serde(rename = "registered", default)]
        pub registered: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for VerifyPasswordResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VerifyPasswordResponse {
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
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
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
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: A,
}
impl<A> Client<A>
where
    A: ::google_api_auth::GetAccessToken,
{
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth,
        }
    }
    #[doc = "Actions that can be performed on the relyingparty resource"]
    pub fn relyingparty(&self) -> crate::resources::relyingparty::RelyingpartyActions<A> {
        crate::resources::relyingparty::RelyingpartyActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod relyingparty {
        pub mod params {}
        pub struct RelyingpartyActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> RelyingpartyActions<'a, A> {
            #[doc = "Creates the URI used by the IdP to authenticate the user."]
            pub fn create_auth_uri(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyCreateAuthUriRequest,
            ) -> CreateAuthUriRequestBuilder<A> {
                CreateAuthUriRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Delete user account."]
            pub fn delete_account(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyDeleteAccountRequest,
            ) -> DeleteAccountRequestBuilder<A> {
                DeleteAccountRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Batch download user accounts."]
            pub fn download_account(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyDownloadAccountRequest,
            ) -> DownloadAccountRequestBuilder<A> {
                DownloadAccountRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Reset password for a user."]
            pub fn email_link_signin(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyEmailLinkSigninRequest,
            ) -> EmailLinkSigninRequestBuilder<A> {
                EmailLinkSigninRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Returns the account info."]
            pub fn get_account_info(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyGetAccountInfoRequest,
            ) -> GetAccountInfoRequestBuilder<A> {
                GetAccountInfoRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Get a code for user action confirmation."]
            pub fn get_oob_confirmation_code(
                &self,
                request: crate::schemas::Relyingparty,
            ) -> GetOobConfirmationCodeRequestBuilder<A> {
                GetOobConfirmationCodeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Get project configuration."]
            pub fn get_project_config(&self) -> GetProjectConfigRequestBuilder<A> {
                GetProjectConfigRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    delegated_project_number: None,
                    project_number: None,
                }
            }
            #[doc = "Get token signing public key."]
            pub fn get_public_keys(&self) -> GetPublicKeysRequestBuilder<A> {
                GetPublicKeysRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Get recaptcha secure param."]
            pub fn get_recaptcha_param(&self) -> GetRecaptchaParamRequestBuilder<A> {
                GetRecaptchaParamRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Reset password for a user."]
            pub fn reset_password(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyResetPasswordRequest,
            ) -> ResetPasswordRequestBuilder<A> {
                ResetPasswordRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Send SMS verification code."]
            pub fn send_verification_code(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartySendVerificationCodeRequest,
            ) -> SendVerificationCodeRequestBuilder<A> {
                SendVerificationCodeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Set account info for a user."]
            pub fn set_account_info(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartySetAccountInfoRequest,
            ) -> SetAccountInfoRequestBuilder<A> {
                SetAccountInfoRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Set project configuration."]
            pub fn set_project_config(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartySetProjectConfigRequest,
            ) -> SetProjectConfigRequestBuilder<A> {
                SetProjectConfigRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Sign out user."]
            pub fn sign_out_user(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartySignOutUserRequest,
            ) -> SignOutUserRequestBuilder<A> {
                SignOutUserRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Signup new user."]
            pub fn signup_new_user(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartySignupNewUserRequest,
            ) -> SignupNewUserRequestBuilder<A> {
                SignupNewUserRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Batch upload existing user accounts."]
            pub fn upload_account(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyUploadAccountRequest,
            ) -> UploadAccountRequestBuilder<A> {
                UploadAccountRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Verifies the assertion returned by the IdP."]
            pub fn verify_assertion(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyVerifyAssertionRequest,
            ) -> VerifyAssertionRequestBuilder<A> {
                VerifyAssertionRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Verifies the developer asserted ID token."]
            pub fn verify_custom_token(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyVerifyCustomTokenRequest,
            ) -> VerifyCustomTokenRequestBuilder<A> {
                VerifyCustomTokenRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Verifies the user entered password."]
            pub fn verify_password(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPasswordRequest,
            ) -> VerifyPasswordRequestBuilder<A> {
                VerifyPasswordRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Verifies ownership of a phone number and creates/updates the user account accordingly."]
            pub fn verify_phone_number(
                &self,
                request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest,
            ) -> VerifyPhoneNumberRequestBuilder<A> {
                VerifyPhoneNumberRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateAuthUriRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyCreateAuthUriRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> CreateAuthUriRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::CreateAuthUriResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CreateAuthUriResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("createAuthUri");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteAccountRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyDeleteAccountRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> DeleteAccountRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::DeleteAccountResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DeleteAccountResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("deleteAccount");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct DownloadAccountRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyDownloadAccountRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> DownloadAccountRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::DownloadAccountResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DownloadAccountResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("downloadAccount");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct EmailLinkSigninRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyEmailLinkSigninRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> EmailLinkSigninRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::EmailLinkSigninResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::EmailLinkSigninResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("emailLinkSignin");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetAccountInfoRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyGetAccountInfoRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetAccountInfoRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::GetAccountInfoResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetAccountInfoResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("getAccountInfo");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetOobConfirmationCodeRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::Relyingparty,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetOobConfirmationCodeRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::GetOobConfirmationCodeResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetOobConfirmationCodeResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("getOobConfirmationCode");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetProjectConfigRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            delegated_project_number: Option<String>,
            project_number: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetProjectConfigRequestBuilder<'a, A> {
            #[doc = "Delegated GCP project number of the request."]
            pub fn delegated_project_number(mut self, value: impl Into<String>) -> Self {
                self.delegated_project_number = Some(value.into());
                self
            }
            #[doc = "GCP project number of the request."]
            pub fn project_number(mut self, value: impl Into<String>) -> Self {
                self.project_number = Some(value.into());
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
                crate::schemas::IdentitytoolkitRelyingpartyGetProjectConfigResponse,
                Box<dyn ::std::error::Error>,
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
                crate::schemas::IdentitytoolkitRelyingpartyGetProjectConfigResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("getProjectConfig");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("delegatedProjectNumber", &self.delegated_project_number)]);
                let req = req.query(&[("projectNumber", &self.project_number)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetPublicKeysRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetPublicKeysRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
                crate::schemas::IdentitytoolkitRelyingpartyGetPublicKeysResponse,
                Box<dyn ::std::error::Error>,
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
                crate::schemas::IdentitytoolkitRelyingpartyGetPublicKeysResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("publicKeys");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRecaptchaParamRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRecaptchaParamRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::GetRecaptchaParamResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetRecaptchaParamResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("getRecaptchaParam");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct ResetPasswordRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyResetPasswordRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ResetPasswordRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::ResetPasswordResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ResetPasswordResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("resetPassword");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct SendVerificationCodeRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartySendVerificationCodeRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SendVerificationCodeRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
                crate::schemas::IdentitytoolkitRelyingpartySendVerificationCodeResponse,
                Box<dyn ::std::error::Error>,
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
                crate::schemas::IdentitytoolkitRelyingpartySendVerificationCodeResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("sendVerificationCode");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct SetAccountInfoRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartySetAccountInfoRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SetAccountInfoRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::SetAccountInfoResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SetAccountInfoResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("setAccountInfo");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct SetProjectConfigRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartySetProjectConfigRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SetProjectConfigRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
                crate::schemas::IdentitytoolkitRelyingpartySetProjectConfigResponse,
                Box<dyn ::std::error::Error>,
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
                crate::schemas::IdentitytoolkitRelyingpartySetProjectConfigResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("setProjectConfig");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct SignOutUserRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartySignOutUserRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SignOutUserRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
                crate::schemas::IdentitytoolkitRelyingpartySignOutUserResponse,
                Box<dyn ::std::error::Error>,
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
                crate::schemas::IdentitytoolkitRelyingpartySignOutUserResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("signOutUser");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct SignupNewUserRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartySignupNewUserRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SignupNewUserRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::SignupNewUserResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SignupNewUserResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("signupNewUser");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct UploadAccountRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyUploadAccountRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> UploadAccountRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::UploadAccountResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::UploadAccountResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("uploadAccount");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct VerifyAssertionRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyAssertionRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> VerifyAssertionRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::VerifyAssertionResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::VerifyAssertionResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("verifyAssertion");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct VerifyCustomTokenRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyCustomTokenRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> VerifyCustomTokenRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::VerifyCustomTokenResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::VerifyCustomTokenResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("verifyCustomToken");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct VerifyPasswordRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPasswordRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> VerifyPasswordRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
            ) -> Result<crate::schemas::VerifyPasswordResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::VerifyPasswordResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("verifyPassword");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct VerifyPhoneNumberRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::IdentitytoolkitRelyingpartyVerifyPhoneNumberRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> VerifyPhoneNumberRequestBuilder<'a, A> {
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
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
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
                crate::schemas::IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse,
                Box<dyn ::std::error::Error>,
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
                crate::schemas::IdentitytoolkitRelyingpartyVerifyPhoneNumberResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/identitytoolkit/v3/relyingparty/".to_owned();
                output.push_str("verifyPhoneNumber");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
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
// Bytes in google apis are represented as urlsafe base64 encoded strings.
// This defines a Bytes type that is a simple wrapper around a Vec<u8> used
// internally to handle byte fields in google apis.
pub mod bytes {
    use radix64::URL_SAFE as BASE64_CFG;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Bytes(Vec<u8>);

    impl ::std::convert::From<Vec<u8>> for Bytes {
        fn from(x: Vec<u8>) -> Bytes {
            Bytes(x)
        }
    }

    impl ::std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            ::radix64::Display::new(BASE64_CFG, &self.0).fmt(f)
        }
    }

    impl ::serde::Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            let encoded = BASE64_CFG.encode(&self.0);
            encoded.serialize(serializer)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bytes, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let encoded = String::deserialize(deserializer)?;
            let decoded = BASE64_CFG
                .decode(&encoded)
                .map_err(|_| ::serde::de::Error::custom("invalid base64 input"))?;
            Ok(Bytes(decoded))
        }
    }
}
