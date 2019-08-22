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
    pub struct AllocateInfo {
        #[doc = "A list of label keys that were unused by the server in processing the\nrequest. Thus, for similar requests repeated in a certain future time\nwindow, the caller can choose to ignore these labels in the requests\nto achieve better client-side cache hits and quota aggregation for rate\nquota. This field is not populated for allocation quota checks."]
        #[serde(rename = "unusedArguments", default)]
        pub unused_arguments: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for AllocateInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AllocateQuotaRequest {
        #[doc = "Operation that describes the quota allocation."]
        #[serde(rename = "allocateOperation", default)]
        pub allocate_operation: ::std::option::Option<crate::schemas::QuotaOperation>,
        #[doc = "Specifies which version of service configuration should be used to process\nthe request. If unspecified or no matching version can be found, the latest\none will be used."]
        #[serde(rename = "serviceConfigId", default)]
        pub service_config_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AllocateQuotaRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AllocateQuotaResponse {
        #[doc = "Indicates the decision of the allocate."]
        #[serde(rename = "allocateErrors", default)]
        pub allocate_errors: ::std::option::Option<Vec<crate::schemas::QuotaError>>,
        #[doc = "WARNING: DO NOT use this field until this warning message is removed."]
        #[serde(rename = "allocateInfo", default)]
        pub allocate_info: ::std::option::Option<crate::schemas::AllocateInfo>,
        #[doc = "The same operation_id value used in the AllocateQuotaRequest. Used for\nlogging and diagnostics purposes."]
        #[serde(rename = "operationId", default)]
        pub operation_id: ::std::option::Option<String>,
        #[doc = "Quota metrics to indicate the result of allocation. Depending on the\nrequest, one or more of the following metrics will be included:\n\n1. Per quota group or per quota metric incremental usage will be specified\n   using the following delta metric :\n   \"serviceruntime.googleapis.com/api/consumer/quota_used_count\"\n\n1. The quota limit reached condition will be specified using the following\n   boolean metric :\n   \"serviceruntime.googleapis.com/quota/exceeded\""]
        #[serde(rename = "quotaMetrics", default)]
        pub quota_metrics: ::std::option::Option<Vec<crate::schemas::MetricValueSet>>,
        #[doc = "ID of the actual config used to process the request."]
        #[serde(rename = "serviceConfigId", default)]
        pub service_config_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AllocateQuotaResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AuditLog {
        #[doc = "Authentication information."]
        #[serde(rename = "authenticationInfo", default)]
        pub authentication_info: ::std::option::Option<crate::schemas::AuthenticationInfo>,
        #[doc = "Authorization information. If there are multiple\nresources or permissions involved, then there is\none AuthorizationInfo element for each {resource, permission} tuple."]
        #[serde(rename = "authorizationInfo", default)]
        pub authorization_info: ::std::option::Option<Vec<crate::schemas::AuthorizationInfo>>,
        #[doc = "Other service-specific data about the request, response, and other\ninformation associated with the current audited event."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The name of the service method or operation.\nFor API calls, this should be the name of the API method.\nFor example,\n\n````text\n\"google.datastore.v1.Datastore.RunQuery\"\n\"google.logging.v1.LoggingService.DeleteLog\"````"]
        #[serde(rename = "methodName", default)]
        pub method_name: ::std::option::Option<String>,
        #[doc = "The number of items returned from a List or Query API method,\nif applicable."]
        #[serde(rename = "numResponseItems", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_response_items: ::std::option::Option<i64>,
        #[doc = "The operation request. This may not include all request parameters,\nsuch as those that are too large, privacy-sensitive, or duplicated\nelsewhere in the log record.\nIt should never include user-generated data, such as file contents.\nWhen the JSON object represented here has a proto equivalent, the proto\nname will be indicated in the `@type` property."]
        #[serde(rename = "request", default)]
        pub request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Metadata about the operation."]
        #[serde(rename = "requestMetadata", default)]
        pub request_metadata: ::std::option::Option<crate::schemas::RequestMetadata>,
        #[doc = "The resource location information."]
        #[serde(rename = "resourceLocation", default)]
        pub resource_location: ::std::option::Option<crate::schemas::ResourceLocation>,
        #[doc = "The resource or collection that is the target of the operation.\nThe name is a scheme-less URI, not including the API service name.\nFor example:\n\n````text\n\"shelves/SHELF_ID/books\"\n\"shelves/SHELF_ID/books/BOOK_ID\"````"]
        #[serde(rename = "resourceName", default)]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "The resource's original state before mutation. Present only for\noperations which have successfully modified the targeted resource(s).\nIn general, this field should contain all changed fields, except those\nthat are already been included in `request`, `response`, `metadata` or\n`service_data` fields.\nWhen the JSON object represented here has a proto equivalent,\nthe proto name will be indicated in the `@type` property."]
        #[serde(rename = "resourceOriginalState", default)]
        pub resource_original_state:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The operation response. This may not include all response elements,\nsuch as those that are too large, privacy-sensitive, or duplicated\nelsewhere in the log record.\nIt should never include user-generated data, such as file contents.\nWhen the JSON object represented here has a proto equivalent, the proto\nname will be indicated in the `@type` property."]
        #[serde(rename = "response", default)]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Deprecated, use `metadata` field instead.\nOther service-specific data about the request, response, and other\nactivities."]
        #[serde(rename = "serviceData", default)]
        pub service_data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The name of the API service performing the operation. For example,\n`\"datastore.googleapis.com\"`."]
        #[serde(rename = "serviceName", default)]
        pub service_name: ::std::option::Option<String>,
        #[doc = "The status of the overall operation."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::field_selector::FieldSelector for AuditLog {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Auth {
        #[doc = "A list of access level resource names that allow resources to be\naccessed by authenticated requester. It is part of Secure GCP processing\nfor the incoming request. An access level string has the format:\n\"//{api_service_name}/accessPolicies/{policy_id}/accessLevels/{short_name}\"\n\nExample:\n\"//accesscontextmanager.googleapis.com/accessPolicies/MY_POLICY_ID/accessLevels/MY_LEVEL\""]
        #[serde(rename = "accessLevels", default)]
        pub access_levels: ::std::option::Option<Vec<String>>,
        #[doc = "The intended audience(s) for this authentication information. Reflects\nthe audience (`aud`) claim within a JWT. The audience\nvalue(s) depends on the `issuer`, but typically include one or more of\nthe following pieces of information:\n\n* The services intended to receive the credential such as\n  [\"pubsub.googleapis.com\", \"storage.googleapis.com\"]\n* A set of service-based scopes. For example,\n  [\"https://www.googleapis.com/auth/cloud-platform\"]\n* The client id of an app, such as the Firebase project id for JWTs\n  from Firebase Auth.\n\nConsult the documentation for the credential issuer to determine the\ninformation provided."]
        #[serde(rename = "audiences", default)]
        pub audiences: ::std::option::Option<Vec<String>>,
        #[doc = "Structured claims presented with the credential. JWTs include\n`{key: value}` pairs for standard and private claims. The following\nis a subset of the standard required and optional claims that would\ntypically be presented for a Google-based JWT:\n\n{'iss': 'accounts.google.com',\n'sub': '113289723416554971153',\n'aud': ['123456789012', 'pubsub.googleapis.com'],\n'azp': '123456789012.apps.googleusercontent.com',\n'email': 'jsmith@example.com',\n'iat': 1353601026,\n'exp': 1353604926}\n\nSAML assertions are similarly specified, but with an identity provider\ndependent structure."]
        #[serde(rename = "claims", default)]
        pub claims:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The authorized presenter of the credential. Reflects the optional\nAuthorized Presenter (`azp`) claim within a JWT or the\nOAuth client id. For example, a Google Cloud Platform client id looks\nas follows: \"123456789012.apps.googleusercontent.com\"."]
        #[serde(rename = "presenter", default)]
        pub presenter: ::std::option::Option<String>,
        #[doc = "The authenticated principal. Reflects the issuer (`iss`) and subject\n(`sub`) claims within a JWT. The issuer and subject should be `/`\ndelimited, with `/` percent-encoded within the subject fragment. For\nGoogle accounts, the principal format is:\n\"https://accounts.google.com/{id}\""]
        #[serde(rename = "principal", default)]
        pub principal: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Auth {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AuthenticationInfo {
        #[doc = "The authority selector specified by the requestor, if any.\nIt is not guaranteed that the principal was allowed to use this authority."]
        #[serde(rename = "authoritySelector", default)]
        pub authority_selector: ::std::option::Option<String>,
        #[doc = "The email address of the authenticated user (or service account on behalf\nof third party principal) making the request. For privacy reasons, the\nprincipal email address is redacted for all read-only operations that fail\nwith a \"permission denied\" error."]
        #[serde(rename = "principalEmail", default)]
        pub principal_email: ::std::option::Option<String>,
        #[doc = "Identity delegation history of an authenticated service account that makes\nthe request. It contains information on the real authorities that try to\naccess GCP resources by delegating on a service account. When multiple\nauthorities present, they are guaranteed to be sorted based on the original\nordering of the identity delegation events."]
        #[serde(rename = "serviceAccountDelegationInfo", default)]
        pub service_account_delegation_info:
            ::std::option::Option<Vec<crate::schemas::ServiceAccountDelegationInfo>>,
        #[doc = "The name of the service account key used to create or exchange\ncredentials for authenticating the service account making the request.\nThis is a scheme-less URI full resource name. For example:\n\n\"//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}\""]
        #[serde(rename = "serviceAccountKeyName", default)]
        pub service_account_key_name: ::std::option::Option<String>,
        #[doc = "The third party identification (if any) of the authenticated user making\nthe request.\nWhen the JSON object represented here has a proto equivalent, the proto\nname will be indicated in the `@type` property."]
        #[serde(rename = "thirdPartyPrincipal", default)]
        pub third_party_principal:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for AuthenticationInfo {
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
    pub struct AuthorizationInfo {
        #[doc = "Whether or not authorization for `resource` and `permission`\nwas granted."]
        #[serde(rename = "granted", default)]
        pub granted: ::std::option::Option<bool>,
        #[doc = "The required IAM permission."]
        #[serde(rename = "permission", default)]
        pub permission: ::std::option::Option<String>,
        #[doc = "The resource being accessed, as a REST-style string. For example:\n\n````text\nbigquery.googleapis.com/projects/PROJECTID/datasets/DATASETID````"]
        #[serde(rename = "resource", default)]
        pub resource: ::std::option::Option<String>,
        #[doc = "Resource attributes used in IAM condition evaluation. This field contains\nresource attributes like resource type and resource name.\n\nTo get the whole view of the attributes used in IAM\ncondition evaluation, the user must also look into\n`AuditLog.request_metadata.request_attributes`."]
        #[serde(rename = "resourceAttributes", default)]
        pub resource_attributes: ::std::option::Option<crate::schemas::Resource>,
    }
    impl ::field_selector::FieldSelector for AuthorizationInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CheckErrorCode {
        #[doc = "This is never used in `CheckResponse`."]
        ErrorCodeUnspecified,
        #[doc = "The consumer's project id, network container, or resource container was\nnot found. Same as google.rpc.Code.NOT_FOUND."]
        NotFound,
        #[doc = "The consumer doesn't have access to the specified resource.\nSame as google.rpc.Code.PERMISSION_DENIED."]
        PermissionDenied,
        #[doc = "Quota check failed. Same as google.rpc.Code.RESOURCE_EXHAUSTED."]
        ResourceExhausted,
        #[doc = "Budget check failed."]
        BudgetExceeded,
        #[doc = "The consumer's request has been flagged as a DoS attack."]
        DenialOfServiceDetected,
        #[doc = "The consumer's request should be rejected in order to protect the service\nfrom being overloaded."]
        LoadShedding,
        #[doc = "The consumer has been flagged as an abuser."]
        AbuserDetected,
        #[doc = "The consumer hasn't activated the service."]
        ServiceNotActivated,
        #[doc = "The consumer cannot access the service due to visibility configuration."]
        VisibilityDenied,
        #[doc = "The consumer cannot access the service because billing is disabled."]
        BillingDisabled,
        #[doc = "The consumer's project has been marked as deleted (soft deletion)."]
        ProjectDeleted,
        #[doc = "The consumer's project number or id does not represent a valid project."]
        ProjectInvalid,
        #[doc = "The input consumer info does not represent a valid consumer folder or\norganization."]
        ConsumerInvalid,
        #[doc = "The IP address of the consumer is invalid for the specific consumer\nproject."]
        IpAddressBlocked,
        #[doc = "The referer address of the consumer request is invalid for the specific\nconsumer project."]
        RefererBlocked,
        #[doc = "The client application of the consumer request is invalid for the\nspecific consumer project."]
        ClientAppBlocked,
        #[doc = "The API targeted by this request is invalid for the specified consumer\nproject."]
        ApiTargetBlocked,
        #[doc = "The consumer's API key is invalid."]
        ApiKeyInvalid,
        #[doc = "The consumer's API Key has expired."]
        ApiKeyExpired,
        #[doc = "The consumer's API Key was not found in config record."]
        ApiKeyNotFound,
        #[doc = "The consumer's spatula header is invalid."]
        SpatulaHeaderInvalid,
        #[doc = "The consumer's LOAS role is invalid."]
        LoasRoleInvalid,
        #[doc = "The consumer's LOAS role has no associated project."]
        NoLoasProject,
        #[doc = "The consumer's LOAS project is not `ACTIVE` in LoquatV2."]
        LoasProjectDisabled,
        #[doc = "Request is not allowed as per security policies defined in Org Policy."]
        SecurityPolicyViolated,
        #[doc = "The credential in the request can not be verified."]
        InvalidCredential,
        #[doc = "Request is not allowed as per location policies defined in Org Policy."]
        LocationPolicyViolated,
        #[doc = "The backend server for looking up project id/number is unavailable."]
        NamespaceLookupUnavailable,
        #[doc = "The backend server for checking service status is unavailable."]
        ServiceStatusUnavailable,
        #[doc = "The backend server for checking billing status is unavailable."]
        BillingStatusUnavailable,
        #[doc = "The backend server for checking quota limits is unavailable."]
        QuotaCheckUnavailable,
        #[doc = "The Spanner for looking up LOAS project is unavailable."]
        LoasProjectLookupUnavailable,
        #[doc = "Cloud Resource Manager backend server is unavailable."]
        CloudResourceManagerBackendUnavailable,
        #[doc = "NOTE: for customers in the scope of Beta/GA of\nhttps://cloud.google.com/vpc-service-controls, this error\nis no longer returned. If the security backend is unavailable, rpc\nUNAVAILABLE status will be returned instead. It should be ignored and\nshould not be used to reject client requests."]
        SecurityPolicyBackendUnavailable,
        #[doc = "Backend server for evaluating location policy is unavailable."]
        LocationPolicyBackendUnavailable,
    }
    impl CheckErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                CheckErrorCode::ErrorCodeUnspecified => "ERROR_CODE_UNSPECIFIED",
                CheckErrorCode::NotFound => "NOT_FOUND",
                CheckErrorCode::PermissionDenied => "PERMISSION_DENIED",
                CheckErrorCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
                CheckErrorCode::BudgetExceeded => "BUDGET_EXCEEDED",
                CheckErrorCode::DenialOfServiceDetected => "DENIAL_OF_SERVICE_DETECTED",
                CheckErrorCode::LoadShedding => "LOAD_SHEDDING",
                CheckErrorCode::AbuserDetected => "ABUSER_DETECTED",
                CheckErrorCode::ServiceNotActivated => "SERVICE_NOT_ACTIVATED",
                CheckErrorCode::VisibilityDenied => "VISIBILITY_DENIED",
                CheckErrorCode::BillingDisabled => "BILLING_DISABLED",
                CheckErrorCode::ProjectDeleted => "PROJECT_DELETED",
                CheckErrorCode::ProjectInvalid => "PROJECT_INVALID",
                CheckErrorCode::ConsumerInvalid => "CONSUMER_INVALID",
                CheckErrorCode::IpAddressBlocked => "IP_ADDRESS_BLOCKED",
                CheckErrorCode::RefererBlocked => "REFERER_BLOCKED",
                CheckErrorCode::ClientAppBlocked => "CLIENT_APP_BLOCKED",
                CheckErrorCode::ApiTargetBlocked => "API_TARGET_BLOCKED",
                CheckErrorCode::ApiKeyInvalid => "API_KEY_INVALID",
                CheckErrorCode::ApiKeyExpired => "API_KEY_EXPIRED",
                CheckErrorCode::ApiKeyNotFound => "API_KEY_NOT_FOUND",
                CheckErrorCode::SpatulaHeaderInvalid => "SPATULA_HEADER_INVALID",
                CheckErrorCode::LoasRoleInvalid => "LOAS_ROLE_INVALID",
                CheckErrorCode::NoLoasProject => "NO_LOAS_PROJECT",
                CheckErrorCode::LoasProjectDisabled => "LOAS_PROJECT_DISABLED",
                CheckErrorCode::SecurityPolicyViolated => "SECURITY_POLICY_VIOLATED",
                CheckErrorCode::InvalidCredential => "INVALID_CREDENTIAL",
                CheckErrorCode::LocationPolicyViolated => "LOCATION_POLICY_VIOLATED",
                CheckErrorCode::NamespaceLookupUnavailable => "NAMESPACE_LOOKUP_UNAVAILABLE",
                CheckErrorCode::ServiceStatusUnavailable => "SERVICE_STATUS_UNAVAILABLE",
                CheckErrorCode::BillingStatusUnavailable => "BILLING_STATUS_UNAVAILABLE",
                CheckErrorCode::QuotaCheckUnavailable => "QUOTA_CHECK_UNAVAILABLE",
                CheckErrorCode::LoasProjectLookupUnavailable => "LOAS_PROJECT_LOOKUP_UNAVAILABLE",
                CheckErrorCode::CloudResourceManagerBackendUnavailable => {
                    "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE"
                }
                CheckErrorCode::SecurityPolicyBackendUnavailable => {
                    "SECURITY_POLICY_BACKEND_UNAVAILABLE"
                }
                CheckErrorCode::LocationPolicyBackendUnavailable => {
                    "LOCATION_POLICY_BACKEND_UNAVAILABLE"
                }
            }
        }
    }
    impl ::std::fmt::Display for CheckErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CheckErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CheckErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR_CODE_UNSPECIFIED" => CheckErrorCode::ErrorCodeUnspecified,
                "NOT_FOUND" => CheckErrorCode::NotFound,
                "PERMISSION_DENIED" => CheckErrorCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => CheckErrorCode::ResourceExhausted,
                "BUDGET_EXCEEDED" => CheckErrorCode::BudgetExceeded,
                "DENIAL_OF_SERVICE_DETECTED" => CheckErrorCode::DenialOfServiceDetected,
                "LOAD_SHEDDING" => CheckErrorCode::LoadShedding,
                "ABUSER_DETECTED" => CheckErrorCode::AbuserDetected,
                "SERVICE_NOT_ACTIVATED" => CheckErrorCode::ServiceNotActivated,
                "VISIBILITY_DENIED" => CheckErrorCode::VisibilityDenied,
                "BILLING_DISABLED" => CheckErrorCode::BillingDisabled,
                "PROJECT_DELETED" => CheckErrorCode::ProjectDeleted,
                "PROJECT_INVALID" => CheckErrorCode::ProjectInvalid,
                "CONSUMER_INVALID" => CheckErrorCode::ConsumerInvalid,
                "IP_ADDRESS_BLOCKED" => CheckErrorCode::IpAddressBlocked,
                "REFERER_BLOCKED" => CheckErrorCode::RefererBlocked,
                "CLIENT_APP_BLOCKED" => CheckErrorCode::ClientAppBlocked,
                "API_TARGET_BLOCKED" => CheckErrorCode::ApiTargetBlocked,
                "API_KEY_INVALID" => CheckErrorCode::ApiKeyInvalid,
                "API_KEY_EXPIRED" => CheckErrorCode::ApiKeyExpired,
                "API_KEY_NOT_FOUND" => CheckErrorCode::ApiKeyNotFound,
                "SPATULA_HEADER_INVALID" => CheckErrorCode::SpatulaHeaderInvalid,
                "LOAS_ROLE_INVALID" => CheckErrorCode::LoasRoleInvalid,
                "NO_LOAS_PROJECT" => CheckErrorCode::NoLoasProject,
                "LOAS_PROJECT_DISABLED" => CheckErrorCode::LoasProjectDisabled,
                "SECURITY_POLICY_VIOLATED" => CheckErrorCode::SecurityPolicyViolated,
                "INVALID_CREDENTIAL" => CheckErrorCode::InvalidCredential,
                "LOCATION_POLICY_VIOLATED" => CheckErrorCode::LocationPolicyViolated,
                "NAMESPACE_LOOKUP_UNAVAILABLE" => CheckErrorCode::NamespaceLookupUnavailable,
                "SERVICE_STATUS_UNAVAILABLE" => CheckErrorCode::ServiceStatusUnavailable,
                "BILLING_STATUS_UNAVAILABLE" => CheckErrorCode::BillingStatusUnavailable,
                "QUOTA_CHECK_UNAVAILABLE" => CheckErrorCode::QuotaCheckUnavailable,
                "LOAS_PROJECT_LOOKUP_UNAVAILABLE" => CheckErrorCode::LoasProjectLookupUnavailable,
                "CLOUD_RESOURCE_MANAGER_BACKEND_UNAVAILABLE" => {
                    CheckErrorCode::CloudResourceManagerBackendUnavailable
                }
                "SECURITY_POLICY_BACKEND_UNAVAILABLE" => {
                    CheckErrorCode::SecurityPolicyBackendUnavailable
                }
                "LOCATION_POLICY_BACKEND_UNAVAILABLE" => {
                    CheckErrorCode::LocationPolicyBackendUnavailable
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
    impl ::field_selector::FieldSelector for CheckErrorCode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CheckError {
        #[doc = "The error code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::CheckErrorCode>,
        #[doc = "Free-form text providing details on the error cause of the error."]
        #[serde(rename = "detail", default)]
        pub detail: ::std::option::Option<String>,
        #[doc = "Contains public information about the check error. If available,\n`status.code` will be non zero and client can propagate it out as public\nerror."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Subject to whom this error applies. See the specific code enum for more\ndetails on this field. For example:\n- \u{201c}project:<project-id or project-number>\u{201d}\n- \u{201c}folder:<folder-id>\u{201d}\n- \u{201c}organization:<organization-id>\u{201d}"]
        #[serde(rename = "subject", default)]
        pub subject: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CheckError {
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
    pub struct CheckInfo {
        #[doc = "Consumer info of this check."]
        #[serde(rename = "consumerInfo", default)]
        pub consumer_info: ::std::option::Option<crate::schemas::ConsumerInfo>,
        #[doc = "A list of fields and label keys that are ignored by the server.\nThe client doesn't need to send them for following requests to improve\nperformance and allow better aggregation."]
        #[serde(rename = "unusedArguments", default)]
        pub unused_arguments: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for CheckInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CheckRequest {
        #[doc = "The operation to be checked."]
        #[serde(rename = "operation", default)]
        pub operation: ::std::option::Option<crate::schemas::Operation>,
        #[doc = "Requests the project settings to be returned as part of the check response."]
        #[serde(rename = "requestProjectSettings", default)]
        pub request_project_settings: ::std::option::Option<bool>,
        #[doc = "Specifies which version of service configuration should be used to process\nthe request.\n\nIf unspecified or no matching version can be found, the\nlatest one will be used."]
        #[serde(rename = "serviceConfigId", default)]
        pub service_config_id: ::std::option::Option<String>,
        #[doc = "Indicates if service activation check should be skipped for this request.\nDefault behavior is to perform the check and apply relevant quota.\nWARNING: Setting this flag to \"true\" will disable quota enforcement."]
        #[serde(rename = "skipActivationCheck", default)]
        pub skip_activation_check: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for CheckRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CheckResponse {
        #[doc = "Indicate the decision of the check.\n\nIf no check errors are present, the service should process the operation.\nOtherwise the service should use the list of errors to determine the\nappropriate action."]
        #[serde(rename = "checkErrors", default)]
        pub check_errors: ::std::option::Option<Vec<crate::schemas::CheckError>>,
        #[doc = "Feedback data returned from the server during processing a Check request."]
        #[serde(rename = "checkInfo", default)]
        pub check_info: ::std::option::Option<crate::schemas::CheckInfo>,
        #[doc = "The same operation_id value used in the CheckRequest.\nUsed for logging and diagnostics purposes."]
        #[serde(rename = "operationId", default)]
        pub operation_id: ::std::option::Option<String>,
        #[doc = "Quota information for the check request associated with this response."]
        #[serde(rename = "quotaInfo", default)]
        pub quota_info: ::std::option::Option<crate::schemas::QuotaInfo>,
        #[doc = "The actual config id used to process the request."]
        #[serde(rename = "serviceConfigId", default)]
        pub service_config_id: ::std::option::Option<String>,
        #[doc = "Unimplemented. The current service rollout id used to process the request."]
        #[serde(rename = "serviceRolloutId", default)]
        pub service_rollout_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CheckResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConsumerInfoType {
        #[doc = "This is never used."]
        ConsumerTypeUnspecified,
        #[doc = "The consumer is a Google Cloud Project."]
        Project,
        #[doc = "The consumer is a Google Cloud Folder."]
        Folder,
        #[doc = "The consumer is a Google Cloud Organization."]
        Organization,
        #[doc = "Service-specific resource container which is defined by the service\nproducer to offer their users the ability to manage service control\nfunctionalities at a finer level of granularity than the PROJECT."]
        ServiceSpecific,
    }
    impl ConsumerInfoType {
        pub fn as_str(self) -> &'static str {
            match self {
                ConsumerInfoType::ConsumerTypeUnspecified => "CONSUMER_TYPE_UNSPECIFIED",
                ConsumerInfoType::Project => "PROJECT",
                ConsumerInfoType::Folder => "FOLDER",
                ConsumerInfoType::Organization => "ORGANIZATION",
                ConsumerInfoType::ServiceSpecific => "SERVICE_SPECIFIC",
            }
        }
    }
    impl ::std::fmt::Display for ConsumerInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConsumerInfoType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConsumerInfoType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONSUMER_TYPE_UNSPECIFIED" => ConsumerInfoType::ConsumerTypeUnspecified,
                "PROJECT" => ConsumerInfoType::Project,
                "FOLDER" => ConsumerInfoType::Folder,
                "ORGANIZATION" => ConsumerInfoType::Organization,
                "SERVICE_SPECIFIC" => ConsumerInfoType::ServiceSpecific,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ConsumerInfoType {
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
    pub struct ConsumerInfo {
        #[doc = "The consumer identity number, can be Google cloud project number, folder\nnumber or organization number e.g. 1234567890. A value of 0 indicates no\nconsumer number is found."]
        #[serde(rename = "consumerNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub consumer_number: ::std::option::Option<i64>,
        #[doc = "The Google cloud project number, e.g. 1234567890. A value of 0 indicates\nno project number is found.\n\nNOTE: This field is deprecated after Chemist support flexible consumer\nid. New code should not depend on this field anymore."]
        #[serde(rename = "projectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub project_number: ::std::option::Option<i64>,
        #[doc = "The type of the consumer which should have been defined in\n[Google Resource Manager](https://cloud.google.com/resource-manager/)."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::ConsumerInfoType>,
    }
    impl ::field_selector::FieldSelector for ConsumerInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Distribution {
        #[doc = "The number of samples in each histogram bucket. `bucket_counts` are\noptional. If present, they must sum to the `count` value.\n\nThe buckets are defined below in `bucket_option`. There are N buckets.\n`bucket_counts[0]` is the number of samples in the underflow bucket.\n`bucket_counts[1]` to `bucket_counts[N-1]` are the numbers of samples\nin each of the finite buckets. And `bucket_counts[N] is the number of samples in the overflow bucket. See the comments of `bucket_option`\nbelow for more details.\n\nAny suffix of trailing zeros may be omitted."]
        #[serde(rename = "bucketCounts", default)]
        pub bucket_counts: ::std::option::Option<Vec<i64>>,
        #[doc = "The total number of samples in the distribution. Must be >= 0."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Example points. Must be in increasing order of `value` field."]
        #[serde(rename = "exemplars", default)]
        pub exemplars: ::std::option::Option<Vec<crate::schemas::Exemplar>>,
        #[doc = "Buckets with arbitrary user-provided width."]
        #[serde(rename = "explicitBuckets", default)]
        pub explicit_buckets: ::std::option::Option<crate::schemas::ExplicitBuckets>,
        #[doc = "Buckets with exponentially growing width."]
        #[serde(rename = "exponentialBuckets", default)]
        pub exponential_buckets: ::std::option::Option<crate::schemas::ExponentialBuckets>,
        #[doc = "Buckets with constant width."]
        #[serde(rename = "linearBuckets", default)]
        pub linear_buckets: ::std::option::Option<crate::schemas::LinearBuckets>,
        #[doc = "The maximum of the population of values. Ignored if `count` is zero."]
        #[serde(rename = "maximum", default)]
        pub maximum: ::std::option::Option<f64>,
        #[doc = "The arithmetic mean of the samples in the distribution. If `count` is\nzero then this field must be zero."]
        #[serde(rename = "mean", default)]
        pub mean: ::std::option::Option<f64>,
        #[doc = "The minimum of the population of values. Ignored if `count` is zero."]
        #[serde(rename = "minimum", default)]
        pub minimum: ::std::option::Option<f64>,
        #[doc = "The sum of squared deviations from the mean:\nSum[i=1..count]((x_i - mean)^2)\nwhere each x_i is a sample values. If `count` is zero then this field\nmust be zero, otherwise validation of the request fails."]
        #[serde(rename = "sumOfSquaredDeviation", default)]
        pub sum_of_squared_deviation: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for Distribution {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Exemplar {
        #[doc = "Contextual information about the example value. Examples are:\n\nTrace: type.googleapis.com/google.monitoring.v3.SpanContext\n\nLiteral string: type.googleapis.com/google.protobuf.StringValue\n\nLabels dropped during aggregation:\ntype.googleapis.com/google.monitoring.v3.DroppedLabels\n\nThere may be only a single attachment of any given message type in a\nsingle exemplar, and this is enforced by the system."]
        #[serde(rename = "attachments", default)]
        pub attachments:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "The observation (sampling) time of the above value."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: ::std::option::Option<String>,
        #[doc = "Value of the exemplar point. This value determines to which bucket the\nexemplar belongs."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for Exemplar {
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
    pub struct ExplicitBuckets {
        #[doc = "'bound' is a list of strictly increasing boundaries between\nbuckets. Note that a list of length N-1 defines N buckets because\nof fenceposting. See comments on `bucket_options` for details.\n\nThe i'th finite bucket covers the interval\n[bound[i-1], bound[i])\nwhere i ranges from 1 to bound_size() - 1. Note that there are no\nfinite buckets at all if 'bound' only contains a single element; in\nthat special case the single bound defines the boundary between the\nunderflow and overflow buckets.\n\nbucket number                   lower bound    upper bound\ni == 0 (underflow)              -inf           bound[i]\n0 < i < bound_size()            bound[i-1]     bound[i]\ni == bound_size() (overflow)    bound[i-1]     +inf"]
        #[serde(rename = "bounds", default)]
        pub bounds: ::std::option::Option<Vec<f64>>,
    }
    impl ::field_selector::FieldSelector for ExplicitBuckets {
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
    pub struct ExponentialBuckets {
        #[doc = "The i'th exponential bucket covers the interval\n[scale * growth_factor^(i-1), scale * growth_factor^i)\nwhere i ranges from 1 to num_finite_buckets inclusive.\nMust be larger than 1.0."]
        #[serde(rename = "growthFactor", default)]
        pub growth_factor: ::std::option::Option<f64>,
        #[doc = "The number of finite buckets. With the underflow and overflow buckets,\nthe total number of buckets is `num_finite_buckets` + 2.\nSee comments on `bucket_options` for details."]
        #[serde(rename = "numFiniteBuckets", default)]
        pub num_finite_buckets: ::std::option::Option<i32>,
        #[doc = "The i'th exponential bucket covers the interval\n[scale * growth_factor^(i-1), scale * growth_factor^i)\nwhere i ranges from 1 to num_finite_buckets inclusive.\nMust be > 0."]
        #[serde(rename = "scale", default)]
        pub scale: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for ExponentialBuckets {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct FirstPartyPrincipal {
        #[doc = "The email address of a Google account.\n."]
        #[serde(rename = "principalEmail", default)]
        pub principal_email: ::std::option::Option<String>,
        #[doc = "Metadata about the service that uses the service account.\n."]
        #[serde(rename = "serviceMetadata", default)]
        pub service_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for FirstPartyPrincipal {
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
    pub struct HttpRequest {
        #[doc = "The number of HTTP response bytes inserted into cache. Set only when a\ncache fill was attempted."]
        #[serde(rename = "cacheFillBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub cache_fill_bytes: ::std::option::Option<i64>,
        #[doc = "Whether or not an entity was served from cache\n(with or without validation)."]
        #[serde(rename = "cacheHit", default)]
        pub cache_hit: ::std::option::Option<bool>,
        #[doc = "Whether or not a cache lookup was attempted."]
        #[serde(rename = "cacheLookup", default)]
        pub cache_lookup: ::std::option::Option<bool>,
        #[doc = "Whether or not the response was validated with the origin server before\nbeing served from cache. This field is only meaningful if `cache_hit` is\nTrue."]
        #[serde(rename = "cacheValidatedWithOriginServer", default)]
        pub cache_validated_with_origin_server: ::std::option::Option<bool>,
        #[doc = "The request processing latency on the server, from the time the request was\nreceived until the response was sent."]
        #[serde(rename = "latency", default)]
        pub latency: ::std::option::Option<String>,
        #[doc = "Protocol used for the request. Examples: \"HTTP/1.1\", \"HTTP/2\", \"websocket\""]
        #[serde(rename = "protocol", default)]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The referer URL of the request, as defined in\n[HTTP/1.1 Header Field\nDefinitions](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html)."]
        #[serde(rename = "referer", default)]
        pub referer: ::std::option::Option<String>,
        #[doc = "The IP address (IPv4 or IPv6) of the client that issued the HTTP\nrequest. Examples: `\"192.168.1.1\"`, `\"FE80::0202:B3FF:FE1E:8329\"`."]
        #[serde(rename = "remoteIp", default)]
        pub remote_ip: ::std::option::Option<String>,
        #[doc = "The request method. Examples: `\"GET\"`, `\"HEAD\"`, `\"PUT\"`, `\"POST\"`."]
        #[serde(rename = "requestMethod", default)]
        pub request_method: ::std::option::Option<String>,
        #[doc = "The size of the HTTP request message in bytes, including the request\nheaders and the request body."]
        #[serde(rename = "requestSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub request_size: ::std::option::Option<i64>,
        #[doc = "The scheme (http, https), the host name, the path, and the query\nportion of the URL that was requested.\nExample: `\"http://example.com/some/info?color=red\"`."]
        #[serde(rename = "requestUrl", default)]
        pub request_url: ::std::option::Option<String>,
        #[doc = "The size of the HTTP response message sent back to the client, in bytes,\nincluding the response headers and the response body."]
        #[serde(rename = "responseSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub response_size: ::std::option::Option<i64>,
        #[doc = "The IP address (IPv4 or IPv6) of the origin server that the request was\nsent to."]
        #[serde(rename = "serverIp", default)]
        pub server_ip: ::std::option::Option<String>,
        #[doc = "The response code indicating the status of the response.\nExamples: 200, 404."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<i32>,
        #[doc = "The user agent sent by the client. Example:\n`\"Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET CLR 1.0.3705)\"`."]
        #[serde(rename = "userAgent", default)]
        pub user_agent: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for HttpRequest {
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
    pub struct LinearBuckets {
        #[doc = "The number of finite buckets. With the underflow and overflow buckets,\nthe total number of buckets is `num_finite_buckets` + 2.\nSee comments on `bucket_options` for details."]
        #[serde(rename = "numFiniteBuckets", default)]
        pub num_finite_buckets: ::std::option::Option<i32>,
        #[doc = "The i'th linear bucket covers the interval\n[offset + (i-1) * width, offset + i * width)\nwhere i ranges from 1 to num_finite_buckets, inclusive."]
        #[serde(rename = "offset", default)]
        pub offset: ::std::option::Option<f64>,
        #[doc = "The i'th linear bucket covers the interval\n[offset + (i-1) * width, offset + i * width)\nwhere i ranges from 1 to num_finite_buckets, inclusive.\nMust be strictly positive."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for LinearBuckets {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LogEntrySeverity {
        #[doc = "(0) The log entry has no assigned severity level."]
        Default,
        #[doc = "(100) Debug or trace information."]
        Debug,
        #[doc = "(200) Routine information, such as ongoing status or performance."]
        Info,
        #[doc = "(300) Normal but significant events, such as start up, shut down, or\na configuration change."]
        Notice,
        #[doc = "(400) Warning events might cause problems."]
        Warning,
        #[doc = "(500) Error events are likely to cause problems."]
        Error,
        #[doc = "(600) Critical events cause more severe problems or outages."]
        Critical,
        #[doc = "(700) A person must take an action immediately."]
        Alert,
        #[doc = "(800) One or more systems are unusable."]
        Emergency,
    }
    impl LogEntrySeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                LogEntrySeverity::Default => "DEFAULT",
                LogEntrySeverity::Debug => "DEBUG",
                LogEntrySeverity::Info => "INFO",
                LogEntrySeverity::Notice => "NOTICE",
                LogEntrySeverity::Warning => "WARNING",
                LogEntrySeverity::Error => "ERROR",
                LogEntrySeverity::Critical => "CRITICAL",
                LogEntrySeverity::Alert => "ALERT",
                LogEntrySeverity::Emergency => "EMERGENCY",
            }
        }
    }
    impl ::std::fmt::Display for LogEntrySeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LogEntrySeverity {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LogEntrySeverity {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => LogEntrySeverity::Default,
                "DEBUG" => LogEntrySeverity::Debug,
                "INFO" => LogEntrySeverity::Info,
                "NOTICE" => LogEntrySeverity::Notice,
                "WARNING" => LogEntrySeverity::Warning,
                "ERROR" => LogEntrySeverity::Error,
                "CRITICAL" => LogEntrySeverity::Critical,
                "ALERT" => LogEntrySeverity::Alert,
                "EMERGENCY" => LogEntrySeverity::Emergency,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for LogEntrySeverity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LogEntry {
        #[doc = "Optional. Information about the HTTP request associated with this\nlog entry, if applicable."]
        #[serde(rename = "httpRequest", default)]
        pub http_request: ::std::option::Option<crate::schemas::HttpRequest>,
        #[doc = "A unique ID for the log entry used for deduplication. If omitted,\nthe implementation will generate one based on operation_id."]
        #[serde(rename = "insertId", default)]
        pub insert_id: ::std::option::Option<String>,
        #[doc = "A set of user-defined (key, value) data that provides additional\ninformation about the log entry."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Required. The log to which this log entry belongs. Examples: `\"syslog\"`,\n`\"book_log\"`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. Information about an operation associated with the log entry, if\napplicable."]
        #[serde(rename = "operation", default)]
        pub operation: ::std::option::Option<crate::schemas::LogEntryOperation>,
        #[doc = "The log entry payload, represented as a protocol buffer that is\nexpressed as a JSON object. The only accepted type currently is\nAuditLog."]
        #[serde(rename = "protoPayload", default)]
        pub proto_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The severity of the log entry. The default value is\n`LogSeverity.DEFAULT`."]
        #[serde(rename = "severity", default)]
        pub severity: ::std::option::Option<crate::schemas::LogEntrySeverity>,
        #[doc = "Optional. Source code location information associated with the log entry,\nif any."]
        #[serde(rename = "sourceLocation", default)]
        pub source_location: ::std::option::Option<crate::schemas::LogEntrySourceLocation>,
        #[doc = "The log entry payload, represented as a structure that\nis expressed as a JSON object."]
        #[serde(rename = "structPayload", default)]
        pub struct_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The log entry payload, represented as a Unicode string (UTF-8)."]
        #[serde(rename = "textPayload", default)]
        pub text_payload: ::std::option::Option<String>,
        #[doc = "The time the event described by the log entry occurred. If\nomitted, defaults to operation start time."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: ::std::option::Option<String>,
        #[doc = "Optional. Resource name of the trace associated with the log entry, if any.\nIf this field contains a relative resource name, you can assume the name is\nrelative to `//tracing.googleapis.com`. Example:\n`projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824`"]
        #[serde(rename = "trace", default)]
        pub trace: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for LogEntry {
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
    pub struct LogEntryOperation {
        #[doc = "Optional. Set this to True if this is the first log entry in the operation."]
        #[serde(rename = "first", default)]
        pub first: ::std::option::Option<bool>,
        #[doc = "Optional. An arbitrary operation identifier. Log entries with the\nsame identifier are assumed to be part of the same operation."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Set this to True if this is the last log entry in the operation."]
        #[serde(rename = "last", default)]
        pub last: ::std::option::Option<bool>,
        #[doc = "Optional. An arbitrary producer identifier. The combination of\n`id` and `producer` must be globally unique.  Examples for `producer`:\n`\"MyDivision.MyBigCompany.com\"`, `\"github.com/MyProject/MyApplication\"`."]
        #[serde(rename = "producer", default)]
        pub producer: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for LogEntryOperation {
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
    pub struct LogEntrySourceLocation {
        #[doc = "Optional. Source file name. Depending on the runtime environment, this\nmight be a simple name or a fully-qualified name."]
        #[serde(rename = "file", default)]
        pub file: ::std::option::Option<String>,
        #[doc = "Optional. Human-readable name of the function or method being invoked, with\noptional context such as the class or package name. This information may be\nused in contexts such as the logs viewer, where a file and line number are\nless meaningful. The format can vary by language. For example:\n`qual.if.ied.Class.method` (Java), `dir/package.func` (Go), `function`\n(Python)."]
        #[serde(rename = "function", default)]
        pub function: ::std::option::Option<String>,
        #[doc = "Optional. Line within the source file. 1-based; 0 indicates no line number\navailable."]
        #[serde(rename = "line", default)]
        #[serde(with = "crate::parsed_string")]
        pub line: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for LogEntrySourceLocation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct MetricValue {
        #[doc = "A boolean value."]
        #[serde(rename = "boolValue", default)]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "A distribution value."]
        #[serde(rename = "distributionValue", default)]
        pub distribution_value: ::std::option::Option<crate::schemas::Distribution>,
        #[doc = "A double precision floating point value."]
        #[serde(rename = "doubleValue", default)]
        pub double_value: ::std::option::Option<f64>,
        #[doc = "The end of the time period over which this metric value's measurement\napplies."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "A signed 64-bit integer value."]
        #[serde(rename = "int64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub int_64_value: ::std::option::Option<i64>,
        #[doc = "The labels describing the metric value.\nSee comments on google.api.servicecontrol.v1.Operation.labels for\nthe overriding relationship."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A money value."]
        #[serde(rename = "moneyValue", default)]
        pub money_value: ::std::option::Option<crate::schemas::Money>,
        #[doc = "The start of the time period over which this metric value's measurement\napplies. The time period has different semantics for different metric\ntypes (cumulative, delta, and gauge). See the metric definition\ndocumentation in the service configuration for details."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
        #[doc = "A text string value."]
        #[serde(rename = "stringValue", default)]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MetricValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct MetricValueSet {
        #[doc = "The metric name defined in the service configuration."]
        #[serde(rename = "metricName", default)]
        pub metric_name: ::std::option::Option<String>,
        #[doc = "The values in this metric."]
        #[serde(rename = "metricValues", default)]
        pub metric_values: ::std::option::Option<Vec<crate::schemas::MetricValue>>,
    }
    impl ::field_selector::FieldSelector for MetricValueSet {
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
    pub struct Money {
        #[doc = "The 3-letter currency code defined in ISO 4217."]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Number of nano (10^-9) units of the amount.\nThe value must be between -999,999,999 and +999,999,999 inclusive.\nIf `units` is positive, `nanos` must be positive or zero.\nIf `units` is zero, `nanos` can be positive, zero, or negative.\nIf `units` is negative, `nanos` must be negative or zero.\nFor example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        #[serde(rename = "nanos", default)]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "The whole units of the amount.\nFor example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        #[serde(rename = "units", default)]
        #[serde(with = "crate::parsed_string")]
        pub units: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for Money {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OperationImportance {
        #[doc = "The API implementation may cache and aggregate the data.\nThe data may be lost when rare and unexpected system failures occur."]
        Low,
        #[doc = "The API implementation doesn't cache and aggregate the data.\nIf the method returns successfully, it's guaranteed that the data has\nbeen persisted in durable storage."]
        High,
        #[doc = "In addition to the behavior described in HIGH, DEBUG enables\nadditional validation logic that is only useful during the onboarding\nprocess. This is only available to Google internal services and\nthe service must be whitelisted by chemist-dev@google.com in order\nto use this level."]
        Debug,
    }
    impl OperationImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                OperationImportance::Low => "LOW",
                OperationImportance::High => "HIGH",
                OperationImportance::Debug => "DEBUG",
            }
        }
    }
    impl ::std::fmt::Display for OperationImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OperationImportance {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OperationImportance {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOW" => OperationImportance::Low,
                "HIGH" => OperationImportance::High,
                "DEBUG" => OperationImportance::Debug,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for OperationImportance {
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
        #[doc = "Identity of the consumer who is using the service.\nThis field should be filled in for the operations initiated by a\nconsumer, but not for service-initiated operations that are\nnot related to a specific consumer.\n\n* This can be in one of the following formats:\n  * project:PROJECT_ID,\n  * project`_`number:PROJECT_NUMBER,\n  * projects/PROJECT_ID or PROJECT_NUMBER,\n  * folders/FOLDER_NUMBER,\n  * organizations/ORGANIZATION_NUMBER,\n  * api`_`key:API_KEY."]
        #[serde(rename = "consumerId", default)]
        pub consumer_id: ::std::option::Option<String>,
        #[doc = "End time of the operation.\nRequired when the operation is used in ServiceController.Report,\nbut optional when the operation is used in ServiceController.Check."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "DO NOT USE. This is an experimental field."]
        #[serde(rename = "importance", default)]
        pub importance: ::std::option::Option<crate::schemas::OperationImportance>,
        #[doc = "Labels describing the operation. Only the following labels are allowed:\n\n* Labels describing monitored resources as defined in\n  the service configuration.\n* Default labels of metric values. When specified, labels defined in the\n  metric value override these default.\n* The following labels defined by Google Cloud Platform:\n  * `cloud.googleapis.com/location` describing the location where the\n    operation happened,\n  * `servicecontrol.googleapis.com/user_agent` describing the user agent\n    of the API request,\n  * `servicecontrol.googleapis.com/service_agent` describing the service\n    used to handle the API request (e.g. ESP),\n  * `servicecontrol.googleapis.com/platform` describing the platform\n    where the API is served, such as App Engine, Compute Engine, or\n    Kubernetes Engine."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Represents information to be logged."]
        #[serde(rename = "logEntries", default)]
        pub log_entries: ::std::option::Option<Vec<crate::schemas::LogEntry>>,
        #[doc = "Represents information about this operation. Each MetricValueSet\ncorresponds to a metric defined in the service configuration.\nThe data type used in the MetricValueSet must agree with\nthe data type specified in the metric definition.\n\nWithin a single operation, it is not allowed to have more than one\nMetricValue instances that have the same metric names and identical\nlabel value combinations. If a request has such duplicated MetricValue\ninstances, the entire request is rejected with\nan invalid argument error."]
        #[serde(rename = "metricValueSets", default)]
        pub metric_value_sets: ::std::option::Option<Vec<crate::schemas::MetricValueSet>>,
        #[doc = "Identity of the operation. This must be unique within the scope of the\nservice that generated the operation. If the service calls\nCheck() and Report() on the same operation, the two calls should carry\nthe same id.\n\nUUID version 4 is recommended, though not required.\nIn scenarios where an operation is computed from existing information\nand an idempotent id is desirable for deduplication purpose, UUID version 5\nis recommended. See RFC 4122 for details."]
        #[serde(rename = "operationId", default)]
        pub operation_id: ::std::option::Option<String>,
        #[doc = "Fully qualified name of the operation. Reserved for future use."]
        #[serde(rename = "operationName", default)]
        pub operation_name: ::std::option::Option<String>,
        #[doc = "Represents the properties needed for quota check. Applicable only if this\noperation is for a quota check request. If this is not specified, no quota\ncheck will be performed."]
        #[serde(rename = "quotaProperties", default)]
        pub quota_properties: ::std::option::Option<crate::schemas::QuotaProperties>,
        #[doc = "DO NOT USE. This field is deprecated, use \"resources\" field instead.\nThe resource name of the parent of a resource in the resource hierarchy.\n\nThis can be in one of the following formats:\n- \u{201c}projects/<project-id or project-number>\u{201d}\n- \u{201c}folders/<folder-id>\u{201d}\n- \u{201c}organizations/<organization-id>\u{201d}"]
        #[serde(rename = "resourceContainer", default)]
        pub resource_container: ::std::option::Option<String>,
        #[doc = "The resources that are involved in the operation.\nThe maximum supported number of entries in this field is 100."]
        #[serde(rename = "resources", default)]
        pub resources: ::std::option::Option<Vec<crate::schemas::ResourceInfo>>,
        #[doc = "Required. Start time of the operation."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
        #[doc = "User defined labels for the resource that this operation is associated\nwith. Only a combination of 1000 user labels per consumer project are\nallowed."]
        #[serde(rename = "userLabels", default)]
        pub user_labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
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
    pub struct Peer {
        #[doc = "The IP address of the peer."]
        #[serde(rename = "ip", default)]
        pub ip: ::std::option::Option<String>,
        #[doc = "The labels associated with the peer."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The network port of the peer."]
        #[serde(rename = "port", default)]
        #[serde(with = "crate::parsed_string")]
        pub port: ::std::option::Option<i64>,
        #[doc = "The identity of this peer. Similar to `Request.auth.principal`, but\nrelative to the peer instead of the request. For example, the\nidenity associated with a load balancer that forwared the request."]
        #[serde(rename = "principal", default)]
        pub principal: ::std::option::Option<String>,
        #[doc = "The CLDR country/region code associated with the above IP address.\nIf the IP address is private, the `region_code` should reflect the\nphysical location where this peer is running."]
        #[serde(rename = "regionCode", default)]
        pub region_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Peer {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QuotaErrorCode {
        #[doc = "This is never used."]
        Unspecified,
        #[doc = "Quota allocation failed.\nSame as google.rpc.Code.RESOURCE_EXHAUSTED."]
        ResourceExhausted,
        #[doc = "Quota release failed.  This error is ONLY returned on a NORMAL release.\nMore formally:  if a user requests a release of 10 tokens, but only\n5 tokens were previously allocated, in a BEST_EFFORT release, this will\nbe considered a success, 5 tokens will be released, and the result will\nbe \"Ok\".  If this is done in NORMAL mode, no tokens will be released,\nand an OUT_OF_RANGE error will be returned.\nSame as google.rpc.Code.OUT_OF_RANGE."]
        OutOfRange,
        #[doc = "Consumer cannot access the service because the service requires active\nbilling."]
        BillingNotActive,
        #[doc = "Consumer's project has been marked as deleted (soft deletion)."]
        ProjectDeleted,
        #[doc = "Specified API key is invalid."]
        ApiKeyInvalid,
        #[doc = "Specified API Key has expired."]
        ApiKeyExpired,
        #[doc = "Consumer's spatula header is invalid."]
        SpatulaHeaderInvalid,
        #[doc = "The consumer's LOAS role is invalid."]
        LoasRoleInvalid,
        #[doc = "The consumer's LOAS role has no associated project."]
        NoLoasProject,
        #[doc = "The backend server for looking up project id/number is unavailable."]
        ProjectStatusUnavailable,
        #[doc = "The backend server for checking service status is unavailable."]
        ServiceStatusUnavailable,
        #[doc = "The backend server for checking billing status is unavailable."]
        BillingStatusUnavailable,
        #[doc = "The backend server for checking quota limits is unavailable."]
        QuotaSystemUnavailable,
    }
    impl QuotaErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                QuotaErrorCode::Unspecified => "UNSPECIFIED",
                QuotaErrorCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
                QuotaErrorCode::OutOfRange => "OUT_OF_RANGE",
                QuotaErrorCode::BillingNotActive => "BILLING_NOT_ACTIVE",
                QuotaErrorCode::ProjectDeleted => "PROJECT_DELETED",
                QuotaErrorCode::ApiKeyInvalid => "API_KEY_INVALID",
                QuotaErrorCode::ApiKeyExpired => "API_KEY_EXPIRED",
                QuotaErrorCode::SpatulaHeaderInvalid => "SPATULA_HEADER_INVALID",
                QuotaErrorCode::LoasRoleInvalid => "LOAS_ROLE_INVALID",
                QuotaErrorCode::NoLoasProject => "NO_LOAS_PROJECT",
                QuotaErrorCode::ProjectStatusUnavailable => "PROJECT_STATUS_UNAVAILABLE",
                QuotaErrorCode::ServiceStatusUnavailable => "SERVICE_STATUS_UNAVAILABLE",
                QuotaErrorCode::BillingStatusUnavailable => "BILLING_STATUS_UNAVAILABLE",
                QuotaErrorCode::QuotaSystemUnavailable => "QUOTA_SYSTEM_UNAVAILABLE",
            }
        }
    }
    impl ::std::fmt::Display for QuotaErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QuotaErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QuotaErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => QuotaErrorCode::Unspecified,
                "RESOURCE_EXHAUSTED" => QuotaErrorCode::ResourceExhausted,
                "OUT_OF_RANGE" => QuotaErrorCode::OutOfRange,
                "BILLING_NOT_ACTIVE" => QuotaErrorCode::BillingNotActive,
                "PROJECT_DELETED" => QuotaErrorCode::ProjectDeleted,
                "API_KEY_INVALID" => QuotaErrorCode::ApiKeyInvalid,
                "API_KEY_EXPIRED" => QuotaErrorCode::ApiKeyExpired,
                "SPATULA_HEADER_INVALID" => QuotaErrorCode::SpatulaHeaderInvalid,
                "LOAS_ROLE_INVALID" => QuotaErrorCode::LoasRoleInvalid,
                "NO_LOAS_PROJECT" => QuotaErrorCode::NoLoasProject,
                "PROJECT_STATUS_UNAVAILABLE" => QuotaErrorCode::ProjectStatusUnavailable,
                "SERVICE_STATUS_UNAVAILABLE" => QuotaErrorCode::ServiceStatusUnavailable,
                "BILLING_STATUS_UNAVAILABLE" => QuotaErrorCode::BillingStatusUnavailable,
                "QUOTA_SYSTEM_UNAVAILABLE" => QuotaErrorCode::QuotaSystemUnavailable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for QuotaErrorCode {
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
    pub struct QuotaError {
        #[doc = "Error code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::QuotaErrorCode>,
        #[doc = "Free-form text that provides details on the cause of the error."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Subject to whom this error applies. See the specific enum for more details\non this field. For example, \"clientip:<ip address of client>\" or\n\"project:<Google developer project id>\"."]
        #[serde(rename = "subject", default)]
        pub subject: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for QuotaError {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct QuotaInfo {
        #[doc = "Quota Metrics that have exceeded quota limits.\nFor QuotaGroup-based quota, this is QuotaGroup.name\nFor QuotaLimit-based quota, this is QuotaLimit.name\nSee: google.api.Quota\nDeprecated: Use quota_metrics to get per quota group limit exceeded status."]
        #[serde(rename = "limitExceeded", default)]
        pub limit_exceeded: ::std::option::Option<Vec<String>>,
        #[doc = "Map of quota group name to the actual number of tokens consumed. If the\nquota check was not successful, then this will not be populated due to no\nquota consumption.\n\nWe are not merging this field with 'quota_metrics' field because of the\ncomplexity of scaling in Chemist client code base. For simplicity, we will\nkeep this field for Castor (that scales quota usage) and 'quota_metrics'\nfor SuperQuota (that doesn't scale quota usage)."]
        #[serde(rename = "quotaConsumed", default)]
        pub quota_consumed: ::std::option::Option<::std::collections::BTreeMap<String, i32>>,
        #[doc = "Quota metrics to indicate the usage. Depending on the check request, one or\nmore of the following metrics will be included:\n\n1. For rate quota, per quota group or per quota metric incremental usage\n   will be specified using the following delta metric:\n   \"serviceruntime.googleapis.com/api/consumer/quota_used_count\"\n\n1. For allocation quota, per quota metric total usage will be specified\n   using the following gauge metric:\n   \"serviceruntime.googleapis.com/allocation/consumer/quota_used_count\"\n\n1. For both rate quota and allocation quota, the quota limit reached\n   condition will be specified using the following boolean metric:\n   \"serviceruntime.googleapis.com/quota/exceeded\""]
        #[serde(rename = "quotaMetrics", default)]
        pub quota_metrics: ::std::option::Option<Vec<crate::schemas::MetricValueSet>>,
    }
    impl ::field_selector::FieldSelector for QuotaInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QuotaOperationQuotaMode {
        #[doc = "Guard against implicit default. Must not be used."]
        Unspecified,
        #[doc = "For AllocateQuota request, allocates quota for the amount specified in\nthe service configuration or specified using the quota metrics. If the\namount is higher than the available quota, allocation error will be\nreturned and no quota will be allocated.\nIf multiple quotas are part of the request, and one fails, none of the\nquotas are allocated or released."]
        Normal,
        #[doc = "The operation allocates quota for the amount specified in the service\nconfiguration or specified using the quota metrics. If the amount is\nhigher than the available quota, request does not fail but all available\nquota will be allocated.\nFor rate quota, BEST_EFFORT will continue to deduct from other groups\neven if one does not have enough quota. For allocation, it will find the\nminimum available amount across all groups and deduct that amount from\nall the affected groups."]
        BestEffort,
        #[doc = "For AllocateQuota request, only checks if there is enough quota\navailable and does not change the available quota. No lock is placed on\nthe available quota either."]
        CheckOnly,
    }
    impl QuotaOperationQuotaMode {
        pub fn as_str(self) -> &'static str {
            match self {
                QuotaOperationQuotaMode::Unspecified => "UNSPECIFIED",
                QuotaOperationQuotaMode::Normal => "NORMAL",
                QuotaOperationQuotaMode::BestEffort => "BEST_EFFORT",
                QuotaOperationQuotaMode::CheckOnly => "CHECK_ONLY",
            }
        }
    }
    impl ::std::fmt::Display for QuotaOperationQuotaMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QuotaOperationQuotaMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QuotaOperationQuotaMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => QuotaOperationQuotaMode::Unspecified,
                "NORMAL" => QuotaOperationQuotaMode::Normal,
                "BEST_EFFORT" => QuotaOperationQuotaMode::BestEffort,
                "CHECK_ONLY" => QuotaOperationQuotaMode::CheckOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for QuotaOperationQuotaMode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct QuotaOperation {
        #[doc = "Identity of the consumer for whom this quota operation is being performed.\n\nThis can be in one of the following formats:\nproject:<project_id>,\nproject_number:<project_number>,\napi_key:<api_key>."]
        #[serde(rename = "consumerId", default)]
        pub consumer_id: ::std::option::Option<String>,
        #[doc = "Labels describing the operation."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Fully qualified name of the API method for which this quota operation is\nrequested. This name is used for matching quota rules or metric rules and\nbilling status rules defined in service configuration.\n\nThis field should not be set if any of the following is true:\n(1) the quota operation is performed on non-API resources.\n(2) quota_metrics is set because the caller is doing quota override.\n\nExample of an RPC method name:\ngoogle.example.library.v1.LibraryService.CreateShelf"]
        #[serde(rename = "methodName", default)]
        pub method_name: ::std::option::Option<String>,
        #[doc = "Identity of the operation. This is expected to be unique within the scope\nof the service that generated the operation, and guarantees idempotency in\ncase of retries.\n\nUUID version 4 is recommended, though not required. In scenarios where an\noperation is computed from existing information and an idempotent id is\ndesirable for deduplication purpose, UUID version 5 is recommended. See\nRFC 4122 for details."]
        #[serde(rename = "operationId", default)]
        pub operation_id: ::std::option::Option<String>,
        #[doc = "Represents information about this operation. Each MetricValueSet\ncorresponds to a metric defined in the service configuration.\nThe data type used in the MetricValueSet must agree with\nthe data type specified in the metric definition.\n\nWithin a single operation, it is not allowed to have more than one\nMetricValue instances that have the same metric names and identical\nlabel value combinations. If a request has such duplicated MetricValue\ninstances, the entire request is rejected with\nan invalid argument error.\n\nThis field is mutually exclusive with method_name."]
        #[serde(rename = "quotaMetrics", default)]
        pub quota_metrics: ::std::option::Option<Vec<crate::schemas::MetricValueSet>>,
        #[doc = "Quota mode for this operation."]
        #[serde(rename = "quotaMode", default)]
        pub quota_mode: ::std::option::Option<crate::schemas::QuotaOperationQuotaMode>,
    }
    impl ::field_selector::FieldSelector for QuotaOperation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QuotaPropertiesQuotaMode {
        #[doc = "Decreases available quota by the cost specified for the operation.\nIf cost is higher than available quota, operation fails and returns\nerror."]
        Acquire,
        #[doc = "Decreases available quota by the cost specified for the operation.\nIf cost is higher than available quota, operation does not fail and\navailable quota goes down to zero but it returns error."]
        AcquireBestEffort,
        #[doc = "Does not change any available quota. Only checks if there is enough\nquota.\nNo lock is placed on the checked tokens neither."]
        Check,
        #[doc = "Increases available quota by the operation cost specified for the\noperation."]
        Release,
    }
    impl QuotaPropertiesQuotaMode {
        pub fn as_str(self) -> &'static str {
            match self {
                QuotaPropertiesQuotaMode::Acquire => "ACQUIRE",
                QuotaPropertiesQuotaMode::AcquireBestEffort => "ACQUIRE_BEST_EFFORT",
                QuotaPropertiesQuotaMode::Check => "CHECK",
                QuotaPropertiesQuotaMode::Release => "RELEASE",
            }
        }
    }
    impl ::std::fmt::Display for QuotaPropertiesQuotaMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QuotaPropertiesQuotaMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QuotaPropertiesQuotaMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACQUIRE" => QuotaPropertiesQuotaMode::Acquire,
                "ACQUIRE_BEST_EFFORT" => QuotaPropertiesQuotaMode::AcquireBestEffort,
                "CHECK" => QuotaPropertiesQuotaMode::Check,
                "RELEASE" => QuotaPropertiesQuotaMode::Release,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for QuotaPropertiesQuotaMode {
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
    pub struct QuotaProperties {
        #[doc = "Quota mode for this operation."]
        #[serde(rename = "quotaMode", default)]
        pub quota_mode: ::std::option::Option<crate::schemas::QuotaPropertiesQuotaMode>,
    }
    impl ::field_selector::FieldSelector for QuotaProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportError {
        #[doc = "The Operation.operation_id value from the request."]
        #[serde(rename = "operationId", default)]
        pub operation_id: ::std::option::Option<String>,
        #[doc = "Details of the error when processing the Operation."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::field_selector::FieldSelector for ReportError {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportInfo {
        #[doc = "The Operation.operation_id value from the request."]
        #[serde(rename = "operationId", default)]
        pub operation_id: ::std::option::Option<String>,
        #[doc = "Quota usage info when processing the `Operation`."]
        #[serde(rename = "quotaInfo", default)]
        pub quota_info: ::std::option::Option<crate::schemas::QuotaInfo>,
    }
    impl ::field_selector::FieldSelector for ReportInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportRequest {
        #[doc = "Operations to be reported.\n\nTypically the service should report one operation per request.\nPutting multiple operations into a single request is allowed, but should\nbe used only when multiple operations are natually available at the time\nof the report.\n\nIf multiple operations are in a single request, the total request size\nshould be no larger than 1MB. See ReportResponse.report_errors for\npartial failure behavior."]
        #[serde(rename = "operations", default)]
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
        #[doc = "Specifies which version of service config should be used to process the\nrequest.\n\nIf unspecified or no matching version can be found, the\nlatest one will be used."]
        #[serde(rename = "serviceConfigId", default)]
        pub service_config_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ReportRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportResponse {
        #[doc = "Partial failures, one for each `Operation` in the request that failed\nprocessing. There are three possible combinations of the RPC status:\n\n1. The combination of a successful RPC status and an empty `report_errors`\n   list indicates a complete success where all `Operations` in the\n   request are processed successfully.\n1. The combination of a successful RPC status and a non-empty\n   `report_errors` list indicates a partial success where some\n   `Operations` in the request succeeded. Each\n   `Operation` that failed processing has a corresponding item\n   in this list.\n1. A failed RPC status indicates a general non-deterministic failure.\n   When this happens, it's impossible to know which of the\n   'Operations' in the request succeeded or failed."]
        #[serde(rename = "reportErrors", default)]
        pub report_errors: ::std::option::Option<Vec<crate::schemas::ReportError>>,
        #[doc = "Quota usage for each quota release `Operation` request.\n\nFully or partially failed quota release request may or may not be present\nin `report_quota_info`. For example, a failed quota release request will\nhave the current quota usage info when precise quota library returns the\ninfo. A deadline exceeded quota request will not have quota usage info.\n\nIf there is no quota release request, report_quota_info will be empty."]
        #[serde(rename = "reportInfos", default)]
        pub report_infos: ::std::option::Option<Vec<crate::schemas::ReportInfo>>,
        #[doc = "The actual config id used to process the request."]
        #[serde(rename = "serviceConfigId", default)]
        pub service_config_id: ::std::option::Option<String>,
        #[doc = "Unimplemented. The current service rollout id used to process the request."]
        #[serde(rename = "serviceRolloutId", default)]
        pub service_rollout_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ReportResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Request {
        #[doc = "The request authentication. May be absent for unauthenticated requests.\nDerived from the HTTP request `Authorization` header or equivalent."]
        #[serde(rename = "auth", default)]
        pub auth: ::std::option::Option<crate::schemas::Auth>,
        #[doc = "The HTTP request headers. If multiple headers share the same key, they\nmust be merged according to the HTTP spec. All header keys must be\nlowercased, because HTTP header keys are case-insensitive."]
        #[serde(rename = "headers", default)]
        pub headers: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The HTTP request `Host` header value."]
        #[serde(rename = "host", default)]
        pub host: ::std::option::Option<String>,
        #[doc = "The unique ID for a request, which can be propagated to downstream\nsystems. The ID should have low probability of collision\nwithin a single day for a specific service."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The HTTP request method, such as `GET`, `POST`."]
        #[serde(rename = "method", default)]
        pub method: ::std::option::Option<String>,
        #[doc = "The HTTP URL path."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
        #[doc = "The network protocol used with the request, such as \"http/1.1\",\n\"spdy/3\", \"h2\", \"h2c\", \"webrtc\", \"tcp\", \"udp\", \"quic\". See\nhttps://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids\nfor details."]
        #[serde(rename = "protocol", default)]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The HTTP URL query in the format of `name1=value`&name2=value2`, as it\nappears in the first line of the HTTP request. No decoding is performed."]
        #[serde(rename = "query", default)]
        pub query: ::std::option::Option<String>,
        #[doc = "A special parameter for request reason. It is used by security systems\nto associate auditing information with a request."]
        #[serde(rename = "reason", default)]
        pub reason: ::std::option::Option<String>,
        #[doc = "The HTTP URL scheme, such as `http` and `https`."]
        #[serde(rename = "scheme", default)]
        pub scheme: ::std::option::Option<String>,
        #[doc = "The HTTP request size in bytes. If unknown, it must be -1."]
        #[serde(rename = "size", default)]
        #[serde(with = "crate::parsed_string")]
        pub size: ::std::option::Option<i64>,
        #[doc = "The timestamp when the `destination` service receives the first byte of\nthe request."]
        #[serde(rename = "time", default)]
        pub time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Request {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct RequestMetadata {
        #[doc = "The IP address of the caller.\nFor caller from internet, this will be public IPv4 or IPv6 address.\nFor caller from a Compute Engine VM with external IP address, this\nwill be the VM's external IP address. For caller from a Compute\nEngine VM without external IP address, if the VM is in the same\norganization (or project) as the accessed resource, `caller_ip` will\nbe the VM's internal IPv4 address, otherwise the `caller_ip` will be\nredacted to \"gce-internal-ip\".\nSee https://cloud.google.com/compute/docs/vpc/ for more information."]
        #[serde(rename = "callerIp", default)]
        pub caller_ip: ::std::option::Option<String>,
        #[doc = "The network of the caller.\nSet only if the network host project is part of the same GCP organization\n(or project) as the accessed resource.\nSee https://cloud.google.com/compute/docs/vpc/ for more information.\nThis is a scheme-less URI full resource name. For example:\n\n````text\n\"//compute.googleapis.com/projects/PROJECT_ID/global/networks/NETWORK_ID\"````"]
        #[serde(rename = "callerNetwork", default)]
        pub caller_network: ::std::option::Option<String>,
        #[doc = "The user agent of the caller.\nThis information is not authenticated and should be treated accordingly.\nFor example:\n\n* `google-api-python-client/1.4.0`:\n  The request was made by the Google API client for Python.\n* `Cloud SDK Command Line Tool apitools-client/1.0 gcloud/0.9.62`:\n  The request was made by the Google Cloud SDK CLI (gcloud).\n* `AppEngine-Google; (+http://code.google.com/appengine; appid: s~my-project`:\n  The request was made from the `my-project` App Engine app.\n  NOLINT"]
        #[serde(rename = "callerSuppliedUserAgent", default)]
        pub caller_supplied_user_agent: ::std::option::Option<String>,
        #[doc = "The destination of a network activity, such as accepting a TCP connection.\nIn a multi hop network activity, the destination represents the receiver of\nthe last hop. Only two fields are used in this message, Peer.port and\nPeer.ip. These fields are optionally populated by those services utilizing\nthe IAM condition feature."]
        #[serde(rename = "destinationAttributes", default)]
        pub destination_attributes: ::std::option::Option<crate::schemas::Peer>,
        #[doc = "Request attributes used in IAM condition evaluation. This field contains\nrequest attributes like request time and access levels associated with\nthe request.\n\nTo get the whole view of the attributes used in IAM\ncondition evaluation, the user must also look into\n`AuditLog.authentication_info.resource_attributes`."]
        #[serde(rename = "requestAttributes", default)]
        pub request_attributes: ::std::option::Option<crate::schemas::Request>,
    }
    impl ::field_selector::FieldSelector for RequestMetadata {
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
    pub struct Resource {
        #[doc = "The labels or tags on the resource, such as AWS resource tags and\nKubernetes resource labels."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The stable identifier (name) of a resource on the `service`. A resource\ncan be logically identified as \"//{resource.service}/{resource.name}\".\nThe differences between a resource name and a URI are:\n\n* Resource name is a logical identifier, independent of network\n  protocol and API version. For example,\n  `//pubsub.googleapis.com/projects/123/topics/news-feed`.\n* URI often includes protocol and version information, so it can\n  be used directly by applications. For example,\n  `https://pubsub.googleapis.com/v1/projects/123/topics/news-feed`.\n\nSee https://cloud.google.com/apis/design/resource_names for details."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of the resource. The syntax is platform-specific because\ndifferent platforms define their resources differently.\n\nFor Google APIs, the type format must be \"{service}/{kind}\"."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The name of the service that this resource belongs to, such as\n`pubsub.googleapis.com`. The service may be different from the DNS\nhostname that actually serves the request."]
        #[serde(rename = "service", default)]
        pub service: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Resource {
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
    pub struct ResourceInfo {
        #[doc = "The identifier of the parent of this resource instance.\nMust be in one of the following formats:\n- \u{201c}projects/<project-id or project-number>\u{201d}\n- \u{201c}folders/<folder-id>\u{201d}\n- \u{201c}organizations/<organization-id>\u{201d}"]
        #[serde(rename = "resourceContainer", default)]
        pub resource_container: ::std::option::Option<String>,
        #[doc = "The location of the resource. If not empty, the resource will be checked\nagainst location policy. The value must be a valid zone, region or\nmultiregion. For example: \"europe-west4\" or \"northamerica-northeast1-a\""]
        #[serde(rename = "resourceLocation", default)]
        pub resource_location: ::std::option::Option<String>,
        #[doc = "Name of the resource. This is used for auditing purposes."]
        #[serde(rename = "resourceName", default)]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ResourceInfo {
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
    pub struct ResourceLocation {
        #[doc = "The locations of a resource after the execution of the operation.\nRequests to create or delete a location based resource must populate\nthe 'current_locations' field and not the 'original_locations' field.\nFor example:\n\n````text\n\"europe-west1-a\"\n\"us-east1\"\n\"nam3\"````"]
        #[serde(rename = "currentLocations", default)]
        pub current_locations: ::std::option::Option<Vec<String>>,
        #[doc = "The locations of a resource prior to the execution of the operation.\nRequests that mutate the resource's location must populate both the\n'original_locations' as well as the 'current_locations' fields.\nFor example:\n\n````text\n\"europe-west1-a\"\n\"us-east1\"\n\"nam3\"````"]
        #[serde(rename = "originalLocations", default)]
        pub original_locations: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ResourceLocation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ServiceAccountDelegationInfo {
        #[doc = "First party (Google) identity as the real authority."]
        #[serde(rename = "firstPartyPrincipal", default)]
        pub first_party_principal: ::std::option::Option<crate::schemas::FirstPartyPrincipal>,
        #[doc = "Third party identity as the real authority."]
        #[serde(rename = "thirdPartyPrincipal", default)]
        pub third_party_principal: ::std::option::Option<crate::schemas::ThirdPartyPrincipal>,
    }
    impl ::field_selector::FieldSelector for ServiceAccountDelegationInfo {
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ThirdPartyPrincipal {
        #[doc = "Metadata about third party identity."]
        #[serde(rename = "thirdPartyClaims", default)]
        pub third_party_claims:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for ThirdPartyPrincipal {
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    #[doc = "Actions that can be performed on the services resource"]
    pub fn services(&self) -> crate::resources::services::ServicesActions<A> {
        crate::resources::services::ServicesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
mod resources {
    pub mod services {
        pub mod params {}
        pub struct ServicesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ServicesActions<'a, A> {
            #[doc = "Attempts to allocate quota for the specified consumer. It should be called\nbefore the operation is executed.\n\nThis method requires the `servicemanagement.services.quota`\npermission on the specified service. For more information, see\n[Cloud IAM](https://cloud.google.com/iam).\n\n**NOTE:** The client **must** fail-open on server errors `INTERNAL`,\n`UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system\nreliability, the server may inject these errors to prohibit any hard\ndependency on the quota functionality."]
            pub fn allocate_quota(
                &self,
                request: crate::schemas::AllocateQuotaRequest,
                service_name: impl Into<String>,
            ) -> AllocateQuotaRequestBuilder<A> {
                AllocateQuotaRequestBuilder {
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
                    service_name: service_name.into(),
                }
            }
            #[doc = "Checks whether an operation on a service should be allowed to proceed\nbased on the configuration of the service and related policies. It must be\ncalled before the operation is executed.\n\nIf feasible, the client should cache the check results and reuse them for\n60 seconds. In case of any server errors, the client should rely on the\ncached results for much longer time to avoid outage.\nWARNING: There is general 60s delay for the configuration and policy\npropagation, therefore callers MUST NOT depend on the `Check` method having\nthe latest policy information.\n\nNOTE: the CheckRequest has the size limit of 64KB.\n\nThis method requires the `servicemanagement.services.check` permission\non the specified service. For more information, see\n[Cloud IAM](https://cloud.google.com/iam)."]
            pub fn check(
                &self,
                request: crate::schemas::CheckRequest,
                service_name: impl Into<String>,
            ) -> CheckRequestBuilder<A> {
                CheckRequestBuilder {
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
                    service_name: service_name.into(),
                }
            }
            #[doc = "Reports operation results to Google Service Control, such as logs and\nmetrics. It should be called after an operation is completed.\n\nIf feasible, the client should aggregate reporting data for up to 5\nseconds to reduce API traffic. Limiting aggregation to 5 seconds is to\nreduce data loss during client crashes. Clients should carefully choose\nthe aggregation time window to avoid data loss risk more than 0.01%\nfor business and compliance reasons.\n\nNOTE: the ReportRequest has the size limit of 1MB.\n\nThis method requires the `servicemanagement.services.report` permission\non the specified service. For more information, see\n[Google Cloud IAM](https://cloud.google.com/iam)."]
            pub fn report(
                &self,
                request: crate::schemas::ReportRequest,
                service_name: impl Into<String>,
            ) -> ReportRequestBuilder<A> {
                ReportRequestBuilder {
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
                    service_name: service_name.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct AllocateQuotaRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::AllocateQuotaRequest,
            service_name: String,
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
        impl<'a, A: yup_oauth2::GetToken> AllocateQuotaRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            ) -> Result<crate::schemas::AllocateQuotaResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AllocateQuotaResponse, Box<dyn ::std::error::Error>>
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicecontrol.googleapis.com/".to_owned();
                output.push_str("v1/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":allocateQuota");
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
        pub struct CheckRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::CheckRequest,
            service_name: String,
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
        impl<'a, A: yup_oauth2::GetToken> CheckRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            ) -> Result<crate::schemas::CheckResponse, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::CheckResponse, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://servicecontrol.googleapis.com/".to_owned();
                output.push_str("v1/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":check");
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
        pub struct ReportRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::ReportRequest,
            service_name: String,
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
        impl<'a, A: yup_oauth2::GetToken> ReportRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            ) -> Result<crate::schemas::ReportResponse, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ReportResponse, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://servicecontrol.googleapis.com/".to_owned();
                output.push_str("v1/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":report");
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
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
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

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

#[allow(dead_code)]
struct PageIter<M, T> {
    method: M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<M, T> Iterator for PageIter<M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
} // Bytes in google apis are represented as urlsafe base64 encoded strings.
  // This defines a Bytes type that is a simple wrapper around a Vec<u8> used
  // internally to handle byte fields in google apis.
#[allow(dead_code)]
mod bytes {
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
