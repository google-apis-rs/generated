#![doc = "# Resources and Methods\n    * [v_1p_4beta_1](resources/v_1p_4beta_1/struct.V1P4Beta1Actions.html)\n      * [*analyzeIamPolicy*](resources/v_1p_4beta_1/struct.AnalyzeIamPolicyRequestBuilder.html), [*exportIamPolicyAnalysis*](resources/v_1p_4beta_1/struct.ExportIamPolicyAnalysisRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
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
    pub struct AccessSelector {
        #[doc = "Optional. The permissions to appear in result."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The roles to appear in result."]
        #[serde(
            rename = "roles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub roles: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for AccessSelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccessSelector {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AnalyzeIamPolicyLongrunningResponse {}
    impl ::google_field_selector::FieldSelector for AnalyzeIamPolicyLongrunningResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeIamPolicyLongrunningResponse {
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
    pub struct AnalyzeIamPolicyResponse {
        #[doc = "Represents whether all entries in the main_analysis and service_account_impersonation_analysis have been fully explored to answer the query in the request."]
        #[serde(
            rename = "fullyExplored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_explored: ::std::option::Option<bool>,
        #[doc = "The main analysis that matches the original request."]
        #[serde(
            rename = "mainAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub main_analysis: ::std::option::Option<crate::schemas::IamPolicyAnalysis>,
        #[doc = "A list of non-critical errors happened during the request handling to explain why `fully_explored` is false, or empty if no error happened."]
        #[serde(
            rename = "nonCriticalErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_critical_errors:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisState>>,
        #[doc = "The service account impersonation analysis if AnalyzeIamPolicyRequest.analyze_service_account_impersonation is enabled."]
        #[serde(
            rename = "serviceAccountImpersonationAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account_impersonation_analysis:
            ::std::option::Option<Vec<crate::schemas::IamPolicyAnalysis>>,
    }
    impl ::google_field_selector::FieldSelector for AnalyzeIamPolicyResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyzeIamPolicyResponse {
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
    pub struct AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(
            rename = "auditLogConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_log_configs: ::std::option::Option<Vec<crate::schemas::AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditConfig {
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
    pub struct AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members."]
        #[serde(
            rename = "exemptedMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exempted_members: ::std::option::Option<Vec<String>>,
        #[doc = "The log type that this config enables."]
        #[serde(
            rename = "logType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_type: ::std::option::Option<crate::schemas::AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                AuditLogConfigLogType::DataRead => "DATA_READ",
                AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AuditLogConfigLogType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AuditLogConfigLogType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AuditLogConfigLogType, ()> {
            Ok(match s {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLogConfigLogType {
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
    pub struct Binding {
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Binding {
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
    pub struct ExportIamPolicyAnalysisRequest {
        #[doc = "Required. The request query."]
        #[serde(
            rename = "analysisQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_query: ::std::option::Option<crate::schemas::IamPolicyAnalysisQuery>,
        #[doc = "Optional. The request options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<crate::schemas::Options>,
        #[doc = "Required. Output configuration indicating where the results will be output to."]
        #[serde(
            rename = "outputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_config: ::std::option::Option<crate::schemas::IamPolicyAnalysisOutputConfig>,
    }
    impl ::google_field_selector::FieldSelector for ExportIamPolicyAnalysisRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExportIamPolicyAnalysisRequest {
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
    pub struct Expr {
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Expr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Expr {
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
    pub struct GcsDestination {
        #[doc = "Required. The uri of the Cloud Storage object. It's the same uri that is used by gsutil. For example: \"gs://bucket_name/object_name\". See [Quickstart: Using the gsutil tool] (https://cloud.google.com/storage/docs/quickstart-gsutil) for examples."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GcsDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GcsDestination {
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
    pub struct GoogleCloudAssetV1P4Beta1Access {
        #[doc = "The analysis state of this access."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisState>,
        #[doc = "The permission."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
        #[doc = "The role."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1Access {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1Access {
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
    pub struct GoogleCloudAssetV1P4Beta1AccessControlList {
        #[doc = "The accesses that match one of the following conditions: - The access_selector, if it is specified in request; - Otherwise, access specifiers reachable from the policy binding's role."]
        #[serde(
            rename = "accesses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accesses: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Access>>,
        #[doc = "Resource edges of the graph starting from the policy attached resource to any descendant resources. The Edge.source_node contains the full resource name of a parent resource and Edge.target_node contains the full resource name of a child resource. This field is present only if the output_resource_edges option is enabled in request."]
        #[serde(
            rename = "resourceEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_edges:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Edge>>,
        #[doc = "The resources that match one of the following conditions: - The resource_selector, if it is specified in request; - Otherwise, resources reachable from the policy attached resource."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Resource>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1AccessControlList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1AccessControlList {
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
    pub struct GoogleCloudAssetV1P4Beta1AnalysisState {
        #[doc = "The human-readable description of the cause of failure."]
        #[serde(
            rename = "cause",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cause: ::std::option::Option<String>,
        #[doc = "The Google standard error code that best describes the state. For example: - OK means the analysis on this entity has been successfully finished; - PERMISSION_DENIED means an access denied error is encountered; - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started in time;"]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisStateCode>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1AnalysisState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1AnalysisState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        #[doc = "The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict"]
        Aborted,
        #[doc = "The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict"]
        AlreadyExists,
        #[doc = "The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request"]
        Cancelled,
        #[doc = "Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error"]
        DataLoss,
        #[doc = "The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout"]
        DeadlineExceeded,
        #[doc = "The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level (e.g., when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence). (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. E.g., if an \"rmdir\" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request"]
        FailedPrecondition,
        #[doc = "Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error"]
        Internal,
        #[doc = "The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request"]
        InvalidArgument,
        #[doc = "Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found"]
        NotFound,
        #[doc = "Not an error; returned on success HTTP Mapping: 200 OK"]
        Ok,
        #[doc = "The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request"]
        OutOfRange,
        #[doc = "The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden"]
        PermissionDenied,
        #[doc = "Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests"]
        ResourceExhausted,
        #[doc = "The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized"]
        Unauthenticated,
        #[doc = "The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable"]
        Unavailable,
        #[doc = "The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented"]
        Unimplemented,
        #[doc = "Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error"]
        Unknown,
    }
    impl GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Aborted => "ABORTED",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::AlreadyExists => "ALREADY_EXISTS",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Cancelled => "CANCELLED",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::DataLoss => "DATA_LOSS",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::DeadlineExceeded => "DEADLINE_EXCEEDED",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::FailedPrecondition => {
                    "FAILED_PRECONDITION"
                }
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Internal => "INTERNAL",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::InvalidArgument => "INVALID_ARGUMENT",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::NotFound => "NOT_FOUND",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Ok => "OK",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::OutOfRange => "OUT_OF_RANGE",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::PermissionDenied => "PERMISSION_DENIED",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::ResourceExhausted => {
                    "RESOURCE_EXHAUSTED"
                }
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unauthenticated => "UNAUTHENTICATED",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unavailable => "UNAVAILABLE",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unimplemented => "UNIMPLEMENTED",
                GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudAssetV1P4Beta1AnalysisStateCode, ()> {
            Ok(match s {
                "ABORTED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Aborted,
                "ALREADY_EXISTS" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::AlreadyExists,
                "CANCELLED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Cancelled,
                "DATA_LOSS" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::DataLoss,
                "DEADLINE_EXCEEDED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::DeadlineExceeded,
                "FAILED_PRECONDITION" => {
                    GoogleCloudAssetV1P4Beta1AnalysisStateCode::FailedPrecondition
                }
                "INTERNAL" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Internal,
                "INVALID_ARGUMENT" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::InvalidArgument,
                "NOT_FOUND" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::NotFound,
                "OK" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Ok,
                "OUT_OF_RANGE" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::OutOfRange,
                "PERMISSION_DENIED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => {
                    GoogleCloudAssetV1P4Beta1AnalysisStateCode::ResourceExhausted
                }
                "UNAUTHENTICATED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unauthenticated,
                "UNAVAILABLE" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unavailable,
                "UNIMPLEMENTED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unimplemented,
                "UNKNOWN" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABORTED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Aborted,
                "ALREADY_EXISTS" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::AlreadyExists,
                "CANCELLED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Cancelled,
                "DATA_LOSS" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::DataLoss,
                "DEADLINE_EXCEEDED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::DeadlineExceeded,
                "FAILED_PRECONDITION" => {
                    GoogleCloudAssetV1P4Beta1AnalysisStateCode::FailedPrecondition
                }
                "INTERNAL" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Internal,
                "INVALID_ARGUMENT" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::InvalidArgument,
                "NOT_FOUND" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::NotFound,
                "OK" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Ok,
                "OUT_OF_RANGE" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::OutOfRange,
                "PERMISSION_DENIED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => {
                    GoogleCloudAssetV1P4Beta1AnalysisStateCode::ResourceExhausted
                }
                "UNAUTHENTICATED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unauthenticated,
                "UNAVAILABLE" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unavailable,
                "UNIMPLEMENTED" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unimplemented,
                "UNKNOWN" => GoogleCloudAssetV1P4Beta1AnalysisStateCode::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1AnalysisStateCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1AnalysisStateCode {
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
    pub struct GoogleCloudAssetV1P4Beta1Edge {
        #[doc = "The source node of the edge."]
        #[serde(
            rename = "sourceNode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_node: ::std::option::Option<String>,
        #[doc = "The target node of the edge."]
        #[serde(
            rename = "targetNode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_node: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1Edge {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1Edge {
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
    pub struct GoogleCloudAssetV1P4Beta1Identity {
        #[doc = "The analysis state of this identity."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisState>,
        #[doc = "The identity name in any form of members appear in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding), such as: - user:foo@google.com - group:group1@google.com - serviceAccount:s1@prj1.iam.gserviceaccount.com - projectOwner:some_project_id - domain:google.com - allUsers - etc."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1Identity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1Identity {
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
    pub struct GoogleCloudAssetV1P4Beta1IdentityList {
        #[doc = "Group identity edges of the graph starting from the binding's group members to any node of the identities. The Edge.source_node contains a group, such as \"group:parent@google.com\". The Edge.target_node contains a member of the group, such as \"group:child@google.com\" or \"user:foo@google.com\". This field is present only if the output_group_edges option is enabled in request."]
        #[serde(
            rename = "groupEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_edges: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Edge>>,
        #[doc = "Only the identities that match one of the following conditions will be presented: - The identity_selector, if it is specified in request; - Otherwise, identities reachable from the policy binding's members."]
        #[serde(
            rename = "identities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Identity>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1IdentityList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1IdentityList {
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
    pub struct GoogleCloudAssetV1P4Beta1Resource {
        #[doc = "The analysis state of this resource."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisState>,
        #[doc = "The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format)"]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P4Beta1Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P4Beta1Resource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudAssetV1P7Beta1Asset {
        #[doc = "Please also refer to the [access level user guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels)."]
        #[serde(
            rename = "accessLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_level:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1AccessLevel>,
        #[doc = "Please also refer to the [access policy user guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies)."]
        #[serde(
            rename = "accessPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_policy:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1AccessPolicy>,
        #[doc = "The ancestry path of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. If the asset is a project, folder, or organization, the ancestry path starts from the asset itself. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
        #[serde(
            rename = "ancestors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ancestors: ::std::option::Option<Vec<String>>,
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
        #[doc = "A representation of the Cloud IAM policy set on a Google Cloud resource. There can be a maximum of one Cloud IAM policy set on any given resource. In addition, Cloud IAM policies inherit their granted access scope from any policies set on parent resources in the resource hierarchy. Therefore, the effectively policy is the union of both the policy set on this resource and each policy set on all of the resource's ancestry resource levels in the hierarchy. See [this topic](https://cloud.google.com/iam/docs/policies#inheritance) for more information."]
        #[serde(
            rename = "iamPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_policy: ::std::option::Option<crate::schemas::Policy>,
        #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A representation of an [organization policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy). There can be more than one organization policy with different constraints set on a given resource."]
        #[serde(
            rename = "orgPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_policy: ::std::option::Option<Vec<crate::schemas::GoogleCloudOrgpolicyV1Policy>>,
        #[doc = "The related assets of the asset of one relationship type. One asset only represents one type of relationship."]
        #[serde(
            rename = "relatedAssets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_assets:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P7Beta1RelatedAssets>,
        #[doc = "A representation of the resource."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<crate::schemas::GoogleCloudAssetV1P7Beta1Resource>,
        #[doc = "Please also refer to the [service perimeter user guide](https://cloud.google.com/vpc-service-controls/docs/overview)."]
        #[serde(
            rename = "servicePerimeter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_perimeter: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeter,
        >,
        #[doc = "The last update timestamp of an asset. update_time is updated when create/update/delete operation is performed."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1Asset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1Asset {
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
    pub struct GoogleCloudAssetV1P7Beta1RelatedAsset {
        #[doc = "The ancestors of an asset in Google Cloud [resource hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy), represented as a list of relative resource names. An ancestry path starts with the closest ancestor in the hierarchy and ends at root. Example: `[\"projects/123456789\", \"folders/5432\", \"organizations/1234\"]`"]
        #[serde(
            rename = "ancestors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ancestors: ::std::option::Option<Vec<String>>,
        #[doc = "The full name of the asset. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1` See [Resource names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information."]
        #[serde(
            rename = "asset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset: ::std::option::Option<String>,
        #[doc = "The type of the asset. Example: `compute.googleapis.com/Disk` See [Supported asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types) for more information."]
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1RelatedAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1RelatedAsset {
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
    pub struct GoogleCloudAssetV1P7Beta1RelatedAssets {
        #[doc = "The peer resources of the relationship."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P7Beta1RelatedAsset>>,
        #[doc = "The detailed relation attributes."]
        #[serde(
            rename = "relationshipAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_attributes:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P7Beta1RelationshipAttributes>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1RelatedAssets {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1RelatedAssets {
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
    pub struct GoogleCloudAssetV1P7Beta1RelationshipAttributes {
        #[doc = "The detail of the relationship, e.g. `contains`, `attaches`"]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<String>,
        #[doc = "The unique identifier of the relationship type. Example: `INSTANCE_TO_INSTANCEGROUP`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The source asset type. Example: `compute.googleapis.com/Instance`"]
        #[serde(
            rename = "sourceResourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_resource_type: ::std::option::Option<String>,
        #[doc = "The target asset type. Example: `compute.googleapis.com/Disk`"]
        #[serde(
            rename = "targetResourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_resource_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1RelationshipAttributes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1RelationshipAttributes {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudAssetV1P7Beta1Resource {
        #[doc = "The content of the resource, in which some sensitive fields are removed and may not be present."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The URL of the discovery document containing the resource's JSON schema. Example: `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        #[serde(
            rename = "discoveryDocumentUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_document_uri: ::std::option::Option<String>,
        #[doc = "The JSON schema name listed in the discovery document. Example: `Project` This value is unspecified for resources that do not have an API based on a discovery document, such as Cloud Bigtable."]
        #[serde(
            rename = "discoveryName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_name: ::std::option::Option<String>,
        #[doc = "The location of the resource in Google Cloud, such as its zone and region. For more information, see https://cloud.google.com/about/locations/."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The full name of the immediate parent of this resource. See [Resource Names](https://cloud.google.com/apis/design/resource_names#full_resource_name) for more information. For Google Cloud assets, this value is the parent resource defined in the [Cloud IAM policy hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy). Example: `//cloudresourcemanager.googleapis.com/projects/my_project_123` For third-party assets, this field may be set differently."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The REST URL for accessing the resource. An HTTP `GET` request using this URL returns the resource itself. Example: `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123` This value is unspecified for resources without a REST API."]
        #[serde(
            rename = "resourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_url: ::std::option::Option<String>,
        #[doc = "The API version. Example: `v1`"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudAssetV1P7Beta1Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudAssetV1P7Beta1Resource {
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
    pub struct GoogleCloudOrgpolicyV1BooleanPolicy {
        #[doc = "If `true`, then the `Policy` is enforced. If `false`, then any configuration is acceptable. Suppose you have a `Constraint` `constraints/compute.disableSerialPortAccess` with `constraint_default` set to `ALLOW`. A `Policy` for that `Constraint` exhibits the following behavior: - If the `Policy` at this resource has enforced set to `false`, serial port connection attempts will be allowed. - If the `Policy` at this resource has enforced set to `true`, serial port connection attempts will be refused. - If the `Policy` at this resource is `RestoreDefault`, serial port connection attempts will be allowed. - If no `Policy` is set at this resource or anywhere higher in the resource hierarchy, serial port connection attempts will be allowed. - If no `Policy` is set at this resource, but one exists higher in the resource hierarchy, the behavior is as if the`Policy` were set at this resource. The following examples demonstrate the different possible layerings: Example 1 (nearest `Constraint` wins): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has no `Policy` set. The constraint at `projects/bar` and `organizations/foo` will not be enforced. Example 2 (enforcement gets replaced): `organizations/foo` has a `Policy` with: {enforced: false} `projects/bar` has a `Policy` with: {enforced: true} The constraint at `organizations/foo` is not enforced. The constraint at `projects/bar` is enforced. Example 3 (RestoreDefault): `organizations/foo` has a `Policy` with: {enforced: true} `projects/bar` has a `Policy` with: {RestoreDefault: {}} The constraint at `organizations/foo` is enforced. The constraint at `projects/bar` is not enforced, because `constraint_default` for the `Constraint` is `ALLOW`."]
        #[serde(
            rename = "enforced",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enforced: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1BooleanPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1BooleanPolicy {
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
    pub struct GoogleCloudOrgpolicyV1ListPolicy {
        #[doc = "The policy all_values state."]
        #[serde(
            rename = "allValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_values:
            ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1ListPolicyAllValues>,
        #[doc = "List of values allowed at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
        #[serde(
            rename = "allowedValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_values: ::std::option::Option<Vec<String>>,
        #[doc = "List of values denied at this resource. Can only be set if `all_values` is set to `ALL_VALUES_UNSPECIFIED`."]
        #[serde(
            rename = "deniedValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub denied_values: ::std::option::Option<Vec<String>>,
        #[doc = "Determines the inheritance behavior for this `Policy`. By default, a `ListPolicy` set at a resource supersedes any `Policy` set anywhere up the resource hierarchy. However, if `inherit_from_parent` is set to `true`, then the values from the effective `Policy` of the parent resource are inherited, meaning the values set in this `Policy` are added to the values inherited up the hierarchy. Setting `Policy` hierarchies that inherit both allowed values and denied values isn't recommended in most circumstances to keep the configuration simple and understandable. However, it is possible to set a `Policy` with `allowed_values` set that inherits a `Policy` with `denied_values` set. In this case, the values that are allowed must be in `allowed_values` and not present in `denied_values`. For example, suppose you have a `Constraint` `constraints/serviceuser.services`, which has a `constraint_type` of `list_constraint`, and with `constraint_default` set to `ALLOW`. Suppose that at the Organization level, a `Policy` is applied that restricts the allowed API activations to {`E1`, `E2`}. Then, if a `Policy` is applied to a project below the Organization that has `inherit_from_parent` set to `false` and field all_values set to DENY, then an attempt to activate any API will be denied. The following examples demonstrate different possible layerings for `projects/bar` parented by `organizations/foo`: Example 1 (no inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has `inherit_from_parent` `false` and values: {allowed_values: \"E3\" allowed_values: \"E4\"} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E3`, and `E4`. Example 2 (inherited values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has a `Policy` with values: {value: \"E3\" value: \"E4\" inherit_from_parent: true} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`. Example 3 (inheriting both allowed and denied values): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {denied_values: \"E1\"} The accepted values at `organizations/foo` are `E1`, `E2`. The value accepted at `projects/bar` is `E2`. Example 4 (RestoreDefault): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values:\"E2\"} `projects/bar` has a `Policy` with values: {RestoreDefault: {}} The accepted values at `organizations/foo` are `E1`, `E2`. The accepted values at `projects/bar` are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 5 (no policy inherits parent policy): `organizations/foo` has no `Policy` set. `projects/bar` has no `Policy` set. The accepted values at both levels are either all or none depending on the value of `constraint_default` (if `ALLOW`, all; if `DENY`, none). Example 6 (ListConstraint allowing all): `organizations/foo` has a `Policy` with values: {allowed_values: \"E1\" allowed_values: \"E2\"} `projects/bar` has a `Policy` with: {all: ALLOW} The accepted values at `organizations/foo` are `E1`, E2`. Any value is accepted at `projects/bar`. Example 7 (ListConstraint allowing none): `organizations/foo`has a`Policy`with values: {allowed_values: \"E1\" allowed_values: \"E2\"}`projects/bar`has a`Policy`with: {all: DENY} The accepted values at`organizations/foo`are`E1`, E2`. No value is accepted at `projects/bar`. Example 10 (allowed and denied subtrees of Resource Manager hierarchy): Given the following resource hierarchy O1->{F1, F2}; F1->{P1}; F2->{P2, P3}, `organizations/foo` has a `Policy` with values: {allowed_values: \"under:organizations/O1\"} `projects/bar` has a `Policy` with: {allowed_values: \"under:projects/P3\"} {denied_values: \"under:folders/F2\"} The accepted values at `organizations/foo` are `organizations/O1`, `folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`, `projects/P3`. The accepted values at `projects/bar` are `organizations/O1`, `folders/F1`, `projects/P1`."]
        #[serde(
            rename = "inheritFromParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inherit_from_parent: ::std::option::Option<bool>,
        #[doc = "Optional. The Google Cloud Console will try to default to a configuration that matches the value specified in this `Policy`. If `suggested_value` is not set, it will inherit the value specified higher in the hierarchy, unless `inherit_from_parent` is `false`."]
        #[serde(
            rename = "suggestedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1ListPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1ListPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudOrgpolicyV1ListPolicyAllValues {
        #[doc = "Indicates that allowed_values or denied_values must be set."]
        AllValuesUnspecified,
        #[doc = "A policy with this set allows all values."]
        Allow,
        #[doc = "A policy with this set denies all values."]
        Deny,
    }
    impl GoogleCloudOrgpolicyV1ListPolicyAllValues {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudOrgpolicyV1ListPolicyAllValues::AllValuesUnspecified => {
                    "ALL_VALUES_UNSPECIFIED"
                }
                GoogleCloudOrgpolicyV1ListPolicyAllValues::Allow => "ALLOW",
                GoogleCloudOrgpolicyV1ListPolicyAllValues::Deny => "DENY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudOrgpolicyV1ListPolicyAllValues, ()> {
            Ok(match s {
                "ALL_VALUES_UNSPECIFIED" => {
                    GoogleCloudOrgpolicyV1ListPolicyAllValues::AllValuesUnspecified
                }
                "ALLOW" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Allow,
                "DENY" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Deny,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_VALUES_UNSPECIFIED" => {
                    GoogleCloudOrgpolicyV1ListPolicyAllValues::AllValuesUnspecified
                }
                "ALLOW" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Allow,
                "DENY" => GoogleCloudOrgpolicyV1ListPolicyAllValues::Deny,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1ListPolicyAllValues {
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
    pub struct GoogleCloudOrgpolicyV1Policy {
        #[doc = "For boolean `Constraints`, whether to enforce the `Constraint` or not."]
        #[serde(
            rename = "booleanPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_policy:
            ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1BooleanPolicy>,
        #[doc = "The name of the `Constraint` the `Policy` is configuring, for example, `constraints/serviceuser.services`. A [list of available constraints](/resource-manager/docs/organization-policy/org-policy-constraints) is available. Immutable after creation."]
        #[serde(
            rename = "constraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub constraint: ::std::option::Option<String>,
        #[doc = "An opaque tag indicating the current version of the `Policy`, used for concurrency control. When the `Policy` is returned from either a `GetPolicy` or a `ListOrgPolicy` request, this `etag` indicates the version of the current `Policy` to use when executing a read-modify-write loop. When the `Policy` is returned from a `GetEffectivePolicy` request, the `etag` will be unset. When the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value that was returned from a `GetOrgPolicy` request as part of a read-modify-write loop for concurrency control. Not setting the `etag`in a `SetOrgPolicy` request will result in an unconditional write of the `Policy`."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "List of values either allowed or disallowed."]
        #[serde(
            rename = "listPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_policy: ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1ListPolicy>,
        #[doc = "Restores the default behavior of the constraint; independent of `Constraint` type."]
        #[serde(
            rename = "restoreDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restore_default:
            ::std::option::Option<crate::schemas::GoogleCloudOrgpolicyV1RestoreDefault>,
        #[doc = "The time stamp the `Policy` was previously updated. This is set by the server, not specified by the caller, and represents the last time a call to `SetOrgPolicy` was made for that `Policy`. Any value set by the client will be ignored."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Version of the `Policy`. Default version is 0;"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1Policy {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudOrgpolicyV1RestoreDefault {}
    impl ::google_field_selector::FieldSelector for GoogleCloudOrgpolicyV1RestoreDefault {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudOrgpolicyV1RestoreDefault {
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
    pub struct GoogleIdentityAccesscontextmanagerV1AccessLevel {
        #[doc = "A `BasicLevel` composed of `Conditions`."]
        #[serde(
            rename = "basic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub basic:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1BasicLevel>,
        #[doc = "A `CustomLevel` written in the Common Expression Language."]
        #[serde(
            rename = "custom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1CustomLevel>,
        #[doc = "Description of the `AccessLevel` and its use. Does not affect behavior."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. Resource name for the Access Level. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/accessLevels/{short_name}`. The maximum length of the `short_name` component is 50 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Human readable title. Must be unique within the Policy."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1AccessLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1AccessLevel {
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
    pub struct GoogleIdentityAccesscontextmanagerV1AccessPolicy {
        #[doc = "Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Polices will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{policy_id}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}`"]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "Required. Human readable title. Does not affect behavior."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1AccessPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1AccessPolicy {
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
    pub struct GoogleIdentityAccesscontextmanagerV1ApiOperation {
        #[doc = "API methods or permissions to allow. Method or permission must belong to the service specified by `service_name` field. A single MethodSelector entry with `*` specified for the `method` field will allow all methods AND permissions for the service specified in `service_name`."]
        #[serde(
            rename = "methodSelectors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method_selectors: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1MethodSelector>,
        >,
        #[doc = "The name of the API whose methods or permissions the IngressPolicy or EgressPolicy want to allow. A single ApiOperation with `service_name` field set to `*` will allow all methods AND permissions for all services."]
        #[serde(
            rename = "serviceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1ApiOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1ApiOperation {
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
    pub struct GoogleIdentityAccesscontextmanagerV1BasicLevel {
        #[doc = "How the `conditions` list should be combined to determine if a request is granted this `AccessLevel`. If AND is used, each `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. If OR is used, at least one `Condition` in `conditions` must be satisfied for the `AccessLevel` to be applied. Default behavior is AND."]
        #[serde(
            rename = "combiningFunction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub combining_function: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction,
        >,
        #[doc = "Required. A list of requirements for the `AccessLevel` to be granted."]
        #[serde(
            rename = "conditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditions: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1Condition>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1BasicLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1BasicLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        #[doc = "All `Conditions` must be true for the `BasicLevel` to be true."]
        And,
        #[doc = "If at least one `Condition` is true, then the `BasicLevel` is true."]
        Or,
    }
    impl GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::And => "AND",
                GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::Or => "OR",
            }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction,
            (),
        > {
            Ok(match s {
                "AND" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::And,
                "OR" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::Or,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::And,
                "OR" => GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction::Or,
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
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1BasicLevelCombiningFunction
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
    pub struct GoogleIdentityAccesscontextmanagerV1Condition {
        #[doc = "Device specific restrictions, all restrictions must hold for the Condition to be true. If not specified, all devices are allowed."]
        #[serde(
            rename = "devicePolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_policy:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1DevicePolicy>,
        #[doc = "CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for a CIDR IP address block, the specified IP address portion must be properly truncated (i.e. all the host bits must be zero) or the input is considered malformed. For example, \"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly, for IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\" is not. The originating IP of a request must be in one of the listed subnets in order for this Condition to be true. If empty, all IP addresses are allowed."]
        #[serde(
            rename = "ipSubnetworks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_subnetworks: ::std::option::Option<Vec<String>>,
        #[doc = "The request must be made by one of the provided user or service accounts. Groups are not supported. Syntax: `user:{emailid}` `serviceAccount:{emailid}` If not specified, a request may come from any user."]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Whether to negate the Condition. If true, the Condition becomes a NAND over its non-empty fields, each field must be false for the Condition overall to be satisfied. Defaults to false."]
        #[serde(
            rename = "negate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negate: ::std::option::Option<bool>,
        #[doc = "The request must originate from one of the provided countries/regions. Must be valid ISO 3166-1 alpha-2 codes."]
        #[serde(
            rename = "regions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regions: ::std::option::Option<Vec<String>>,
        #[doc = "A list of other access levels defined in the same `Policy`, referenced by resource name. Referencing an `AccessLevel` which does not exist is an error. All access levels listed must be granted for the Condition to be true. Example: \"`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME\"`"]
        #[serde(
            rename = "requiredAccessLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required_access_levels: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1Condition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1Condition {
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
    pub struct GoogleIdentityAccesscontextmanagerV1CustomLevel {
        #[doc = "Required. A Cloud CEL expression evaluating to a boolean."]
        #[serde(
            rename = "expr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expr: ::std::option::Option<crate::schemas::Expr>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1CustomLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1CustomLevel {
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
    pub struct GoogleIdentityAccesscontextmanagerV1DevicePolicy { # [doc = "Allowed device management levels, an empty list allows all management levels."] # [serde (rename = "allowedDeviceManagementLevels" , default , skip_serializing_if = "std::option::Option::is_none")] pub allowed_device_management_levels : :: std :: option :: Option < Vec < crate :: schemas :: GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems > > , # [doc = "Allowed encryptions statuses, an empty list allows all statuses."] # [serde (rename = "allowedEncryptionStatuses" , default , skip_serializing_if = "std::option::Option::is_none")] pub allowed_encryption_statuses : :: std :: option :: Option < Vec < crate :: schemas :: GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems > > , # [doc = "Allowed OS versions, an empty list allows all types and all versions."] # [serde (rename = "osConstraints" , default , skip_serializing_if = "std::option::Option::is_none")] pub os_constraints : :: std :: option :: Option < Vec < crate :: schemas :: GoogleIdentityAccesscontextmanagerV1OsConstraint > > , # [doc = "Whether the device needs to be approved by the customer admin."] # [serde (rename = "requireAdminApproval" , default , skip_serializing_if = "std::option::Option::is_none")] pub require_admin_approval : :: std :: option :: Option < bool > , # [doc = "Whether the device needs to be corp owned."] # [serde (rename = "requireCorpOwned" , default , skip_serializing_if = "std::option::Option::is_none")] pub require_corp_owned : :: std :: option :: Option < bool > , # [doc = "Whether or not screenlock is required for the DevicePolicy to be true. Defaults to `false`."] # [serde (rename = "requireScreenlock" , default , skip_serializing_if = "std::option::Option::is_none")] pub require_screenlock : :: std :: option :: Option < bool > , }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1DevicePolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1DevicePolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems {
        #[doc = "Basic management is enabled, which is generally limited to monitoring and wiping the corporate account."]
        Basic,
        #[doc = "Complete device management. This includes more thorough monitoring and the ability to directly manage the device (such as remote wiping). This can be enabled through the Android Enterprise Platform."]
        Complete,
        #[doc = "The device's management level is not specified or not known."]
        ManagementUnspecified,
        #[doc = "The device is not managed."]
        None,
    }
    impl GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Basic => "BASIC" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Complete => "COMPLETE" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: ManagementUnspecified => "MANAGEMENT_UNSPECIFIED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: None => "NONE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems,
            (),
        > {
            Ok (match s { "BASIC" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Basic , "COMPLETE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Complete , "MANAGEMENT_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: ManagementUnspecified , "NONE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: None , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BASIC" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Basic , "COMPLETE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: Complete , "MANAGEMENT_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: ManagementUnspecified , "NONE" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems :: None , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedDeviceManagementLevelsItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems {
        #[doc = "The device is encrypted."]
        Encrypted,
        #[doc = "The encryption status of the device is not specified or not known."]
        EncryptionUnspecified,
        #[doc = "The device does not support encryption."]
        EncryptionUnsupported,
        #[doc = "The device supports encryption, but is currently unencrypted."]
        Unencrypted,
    }
    impl GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Encrypted => "ENCRYPTED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnspecified => "ENCRYPTION_UNSPECIFIED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnsupported => "ENCRYPTION_UNSUPPORTED" , GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Unencrypted => "UNENCRYPTED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems,
            (),
        > {
            Ok (match s { "ENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Encrypted , "ENCRYPTION_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnspecified , "ENCRYPTION_UNSUPPORTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnsupported , "UNENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Unencrypted , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Encrypted , "ENCRYPTION_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnspecified , "ENCRYPTION_UNSUPPORTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: EncryptionUnsupported , "UNENCRYPTED" => GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems :: Unencrypted , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1DevicePolicyAllowedEncryptionStatusesItems
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
    pub struct GoogleIdentityAccesscontextmanagerV1EgressFrom {
        #[doc = "A list of identities that are allowed access through this [EgressPolicy]. Should be in the format of email address. The email address should represent individual user or service account only."]
        #[serde(
            rename = "identities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identities: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies the type of identities that are allowed access to outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
        #[serde(
            rename = "identityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1EgressFrom {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1EgressFrom {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        #[doc = "Authorize access from all identities outside the perimeter."]
        AnyIdentity,
        #[doc = "Authorize access from all service accounts outside the perimeter."]
        AnyServiceAccount,
        #[doc = "Authorize access from all human users outside the perimeter."]
        AnyUserAccount,
        #[doc = "No blanket identity group specified."]
        IdentityTypeUnspecified,
    }
    impl GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyIdentity => "ANY_IDENTITY" , GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyServiceAccount => "ANY_SERVICE_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyUserAccount => "ANY_USER_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: IdentityTypeUnspecified => "IDENTITY_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType, ()>
        {
            Ok (match s { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1EgressFromIdentityType
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
    pub struct GoogleIdentityAccesscontextmanagerV1EgressPolicy {
        #[doc = "Defines conditions on the source of a request causing this EgressPolicy to apply."]
        #[serde(
            rename = "egressFrom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_from:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressFrom>,
        #[doc = "Defines the conditions on the ApiOperation and destination resources that cause this EgressPolicy to apply."]
        #[serde(
            rename = "egressTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_to:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressTo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1EgressPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1EgressPolicy {
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
    pub struct GoogleIdentityAccesscontextmanagerV1EgressTo {
        #[doc = "A list of ApiOperations that this egress rule applies to. A request matches if it contains an operation/service in this list."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1ApiOperation>,
        >,
        #[doc = "A list of resources, currently only projects in the form `projects/`, that match this to stanza. A request matches if it contains a resource in this list. If `*` is specified for resources, then this EgressTo rule will authorize access to all resources outside the perimeter."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1EgressTo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1EgressTo {
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressFrom {
        #[doc = "A list of identities that are allowed access through this ingress policy. Should be in the format of email address. The email address should represent individual user or service account only."]
        #[serde(
            rename = "identities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identities: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies the type of identities that are allowed access from outside the perimeter. If left unspecified, then members of `identities` field will be allowed access."]
        #[serde(
            rename = "identityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType,
        >,
        #[doc = "Sources that this IngressPolicy authorizes access from."]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressSource>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressFrom {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressFrom {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        #[doc = "Authorize access from all identities outside the perimeter."]
        AnyIdentity,
        #[doc = "Authorize access from all service accounts outside the perimeter."]
        AnyServiceAccount,
        #[doc = "Authorize access from all human users outside the perimeter."]
        AnyUserAccount,
        #[doc = "No blanket identity group specified."]
        IdentityTypeUnspecified,
    }
    impl GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyIdentity => "ANY_IDENTITY" , GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyServiceAccount => "ANY_SERVICE_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyUserAccount => "ANY_USER_ACCOUNT" , GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: IdentityTypeUnspecified => "IDENTITY_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType, ()>
        {
            Ok (match s { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ANY_IDENTITY" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyIdentity , "ANY_SERVICE_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyServiceAccount , "ANY_USER_ACCOUNT" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: AnyUserAccount , "IDENTITY_TYPE_UNSPECIFIED" => GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType :: IdentityTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1IngressFromIdentityType
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressPolicy {
        #[doc = "Defines the conditions on the source of a request causing this IngressPolicy to apply."]
        #[serde(
            rename = "ingressFrom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingress_from:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressFrom>,
        #[doc = "Defines the conditions on the ApiOperation and request destination that cause this IngressPolicy to apply."]
        #[serde(
            rename = "ingressTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingress_to:
            ::std::option::Option<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressTo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressPolicy {
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressSource {
        #[doc = "An AccessLevel resource name that allow resources within the ServicePerimeters to be accessed from the internet. AccessLevels listed must be in the same policy as this ServicePerimeter. Referencing a nonexistent AccessLevel will cause an error. If no AccessLevel names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL`. If `*` is specified, then all IngressSources will be allowed."]
        #[serde(
            rename = "accessLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_level: ::std::option::Option<String>,
        #[doc = "A Google Cloud resource that is allowed to ingress the perimeter. Requests from these resources will be allowed to access perimeter data. Currently only projects are allowed. Format: `projects/{project_number}` The project may be in any Google Cloud organization, not just the organization that the perimeter is defined in. `*` is not allowed, the case of allowing all Google Cloud resources only is not supported."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressSource {
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
    pub struct GoogleIdentityAccesscontextmanagerV1IngressTo {
        #[doc = "A list of ApiOperations the sources specified in corresponding IngressFrom are allowed to perform in this ServicePerimeter."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1ApiOperation>,
        >,
        #[doc = "A list of resources, currently only projects in the form `projects/`, protected by this ServicePerimeter that are allowed to be accessed by sources defined in the corresponding IngressFrom. A request matches if it contains a resource in this list. If `*` is specified for resources, then this IngressTo rule will authorize access to all resources inside the perimeter, provided that the request also matches the `operations` field."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1IngressTo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1IngressTo {
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
    pub struct GoogleIdentityAccesscontextmanagerV1MethodSelector {
        #[doc = "Value for `method` should be a valid method name for the corresponding `service_name` in ApiOperation. If `*` used as value for `method`, then ALL methods and permissions are allowed."]
        #[serde(
            rename = "method",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method: ::std::option::Option<String>,
        #[doc = "Value for `permission` should be a valid Cloud IAM permission for the corresponding `service_name` in ApiOperation."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1MethodSelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1MethodSelector {
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
    pub struct GoogleIdentityAccesscontextmanagerV1OsConstraint {
        #[doc = "The minimum allowed OS version. If not set, any version of this OS satisfies the constraint. Format: `\"major.minor.patch\"`. Examples: `\"10.5.301\"`, `\"9.2.1\"`."]
        #[serde(
            rename = "minimumVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum_version: ::std::option::Option<String>,
        #[doc = "Required. The allowed OS type."]
        #[serde(
            rename = "osType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1OsConstraintOsType,
        >,
        #[doc = "Only allows requests from devices with a verified Chrome OS. Verifications includes requirements that the device is enterprise-managed, conformant to domain policies, and the caller has permission to call the API targeted by the request."]
        #[serde(
            rename = "requireVerifiedChromeOs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub require_verified_chrome_os: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIdentityAccesscontextmanagerV1OsConstraint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1OsConstraint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        #[doc = "An Android operating system."]
        Android,
        #[doc = "A desktop ChromeOS operating system."]
        DesktopChromeOs,
        #[doc = "A desktop Linux operating system."]
        DesktopLinux,
        #[doc = "A desktop Mac operating system."]
        DesktopMac,
        #[doc = "A desktop Windows operating system."]
        DesktopWindows,
        #[doc = "An iOS operating system."]
        Ios,
        #[doc = "The operating system of the device is not specified or not known."]
        OsUnspecified,
    }
    impl GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Android => "ANDROID",
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopChromeOs => {
                    "DESKTOP_CHROME_OS"
                }
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopLinux => {
                    "DESKTOP_LINUX"
                }
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopMac => "DESKTOP_MAC",
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopWindows => {
                    "DESKTOP_WINDOWS"
                }
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Ios => "IOS",
                GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::OsUnspecified => {
                    "OS_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleIdentityAccesscontextmanagerV1OsConstraintOsType, ()>
        {
            Ok(match s {
                "ANDROID" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Android,
                "DESKTOP_CHROME_OS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopChromeOs
                }
                "DESKTOP_LINUX" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopLinux
                }
                "DESKTOP_MAC" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopMac,
                "DESKTOP_WINDOWS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopWindows
                }
                "IOS" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Ios,
                "OS_UNSPECIFIED" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::OsUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Android,
                "DESKTOP_CHROME_OS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopChromeOs
                }
                "DESKTOP_LINUX" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopLinux
                }
                "DESKTOP_MAC" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopMac,
                "DESKTOP_WINDOWS" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::DesktopWindows
                }
                "IOS" => GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::Ios,
                "OS_UNSPECIFIED" => {
                    GoogleIdentityAccesscontextmanagerV1OsConstraintOsType::OsUnspecified
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
        for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1OsConstraintOsType
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
    pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
        #[doc = "Description of the `ServicePerimeter` and its use. Does not affect behavior."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. Resource name for the ServicePerimeter. The `short_name` component must begin with a letter and only include alphanumeric and '_'. Format: `accessPolicies/{policy_id}/servicePerimeters/{short_name}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty."]
        #[serde(
            rename = "perimeterType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub perimeter_type: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType,
        >,
        #[doc = "Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the \"use_explicit_dry_run_spec\" flag is set."]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig,
        >,
        #[doc = "Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig,
        >,
        #[doc = "Human readable title. Must be unique within the Policy."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration (\"spec\") to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config (\"status\") without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values."]
        #[serde(
            rename = "useExplicitDryRunSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_explicit_dry_run_spec: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeter
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIdentityAccesscontextmanagerV1ServicePerimeter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        #[doc = "Perimeter Bridge."]
        PerimeterTypeBridge,
        #[doc = "Regular Perimeter."]
        PerimeterTypeRegular,
    }
    impl GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeBridge => "PERIMETER_TYPE_BRIDGE" , GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeRegular => "PERIMETER_TYPE_REGULAR" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType,
            (),
        > {
            Ok (match s { "PERIMETER_TYPE_BRIDGE" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeBridge , "PERIMETER_TYPE_REGULAR" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeRegular , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "PERIMETER_TYPE_BRIDGE" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeBridge , "PERIMETER_TYPE_REGULAR" => GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType :: PerimeterTypeRegular , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterPerimeterType
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
    pub struct GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig {
        #[doc = "A list of `AccessLevel` resource names that allow resources within the `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed must be in the same policy as this `ServicePerimeter`. Referencing a nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are listed, resources within the perimeter can only be accessed via Google Cloud calls with request origins within the perimeter. Example: `\"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL\"`. For Service Perimeter Bridge, must be empty."]
        #[serde(
            rename = "accessLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_levels: ::std::option::Option<Vec<String>>,
        #[doc = "List of EgressPolicies to apply to the perimeter. A perimeter may have multiple EgressPolicies, each of which is evaluated separately. Access is granted if any EgressPolicy grants it. Must be empty for a perimeter bridge."]
        #[serde(
            rename = "egressPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_policies: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1EgressPolicy>,
        >,
        #[doc = "List of IngressPolicies to apply to the perimeter. A perimeter may have multiple IngressPolicies, each of which is evaluated separately. Access is granted if any Ingress Policy grants it. Must be empty for a perimeter bridge."]
        #[serde(
            rename = "ingressPolicies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingress_policies: ::std::option::Option<
            Vec<crate::schemas::GoogleIdentityAccesscontextmanagerV1IngressPolicy>,
        >,
        #[doc = "A list of Google Cloud resources that are inside of the service perimeter. Currently only projects are allowed. Format: `projects/{project_number}`"]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<String>>,
        #[doc = "Google Cloud services that are subject to the Service Perimeter restrictions. For example, if `storage.googleapis.com` is specified, access to the storage buckets inside the perimeter must meet the perimeter's access restrictions."]
        #[serde(
            rename = "restrictedServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restricted_services: ::std::option::Option<Vec<String>>,
        #[doc = "Configuration for APIs allowed within Perimeter."]
        #[serde(
            rename = "vpcAccessibleServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vpc_accessible_services: ::std::option::Option<
            crate::schemas::GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1ServicePerimeterConfig
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
    pub struct GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices {
        #[doc = "The list of APIs usable within the Service Perimeter. Must be empty unless 'enable_restriction' is True. You can specify a list of individual services, as well as include the 'RESTRICTED-SERVICES' value, which automatically includes all of the services protected by the perimeter."]
        #[serde(
            rename = "allowedServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_services: ::std::option::Option<Vec<String>>,
        #[doc = "Whether to restrict API calls within the Service Perimeter to the list of APIs specified in 'allowed_services'."]
        #[serde(
            rename = "enableRestriction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_restriction: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleIdentityAccesscontextmanagerV1VpcAccessibleServices
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
    pub struct IamPolicyAnalysis {
        #[doc = "The analysis query."]
        #[serde(
            rename = "analysisQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_query: ::std::option::Option<crate::schemas::IamPolicyAnalysisQuery>,
        #[doc = "A list of IamPolicyAnalysisResult that matches the analysis query, or empty if no result is found."]
        #[serde(
            rename = "analysisResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_results: ::std::option::Option<Vec<crate::schemas::IamPolicyAnalysisResult>>,
        #[doc = "Represents whether all entries in the analysis_results have been fully explored to answer the query."]
        #[serde(
            rename = "fullyExplored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_explored: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysis {
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
    pub struct IamPolicyAnalysisOutputConfig {
        #[doc = "Destination on Cloud Storage."]
        #[serde(
            rename = "gcsDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_destination: ::std::option::Option<crate::schemas::GcsDestination>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisOutputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisOutputConfig {
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
    pub struct IamPolicyAnalysisQuery {
        #[doc = "Optional. Specifies roles or permissions for analysis. This is optional."]
        #[serde(
            rename = "accessSelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_selector: ::std::option::Option<crate::schemas::AccessSelector>,
        #[doc = "Optional. Specifies an identity for analysis."]
        #[serde(
            rename = "identitySelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_selector: ::std::option::Option<crate::schemas::IdentitySelector>,
        #[doc = "Required. The relative name of the root asset. Only resources and IAM policies within the parent will be analyzed. This can only be an organization number (such as \"organizations/123\"), a folder number (such as \"folders/123\"), a project ID (such as \"projects/my-project-id\"), or a project number (such as \"projects/12345\"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects)."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "Optional. Specifies a resource for analysis."]
        #[serde(
            rename = "resourceSelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_selector: ::std::option::Option<crate::schemas::ResourceSelector>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisQuery {
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
    pub struct IamPolicyAnalysisResult {
        #[doc = "The access control lists derived from the iam_binding that match or potentially match resource and access selectors specified in the request."]
        #[serde(
            rename = "accessControlLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_control_lists:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1AccessControlList>>,
        #[doc = "The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format) of the resource to which the iam_binding policy attaches."]
        #[serde(
            rename = "attachedResourceFullName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attached_resource_full_name: ::std::option::Option<String>,
        #[doc = "Represents whether all analyses on the iam_binding have successfully finished."]
        #[serde(
            rename = "fullyExplored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fully_explored: ::std::option::Option<bool>,
        #[doc = "The Cloud IAM policy binding under analysis."]
        #[serde(
            rename = "iamBinding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_binding: ::std::option::Option<crate::schemas::Binding>,
        #[doc = "The identity list derived from members of the iam_binding that match or potentially match identity selector specified in the request."]
        #[serde(
            rename = "identityList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_list:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P4Beta1IdentityList>,
    }
    impl ::google_field_selector::FieldSelector for IamPolicyAnalysisResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IamPolicyAnalysisResult {
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
    pub struct IdentitySelector {
        #[doc = "Required. The identity appear in the form of members in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding). The examples of supported forms are: \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\". Notice that wildcard characters (such as * and ?) are not supported. You must give a specific identity."]
        #[serde(
            rename = "identity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IdentitySelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IdentitySelector {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
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
    pub struct Options {
        #[doc = "Optional. If true, the response will include access analysis from identities to resources via service account impersonation. This is a very expensive operation, because many derived queries will be executed. For example, if the request analyzes for which resources user A has permission P, and there's an IAM policy states user A has iam.serviceAccounts.getAccessToken permission to a service account SA, and there's another IAM policy states service account SA has permission P to a GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Another example, if the request analyzes for who has permission P to a GCP folder F, and there's an IAM policy states user A has iam.serviceAccounts.actAs permission to a service account SA, and there's another IAM policy states service account SA has permission P to the GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Default is false."]
        #[serde(
            rename = "analyzeServiceAccountImpersonation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analyze_service_account_impersonation: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the identities section of the result will expand any Google groups appearing in an IAM policy binding. If identity_selector is specified, the identity in the result will be determined by the selector, and this flag will have no effect. Default is false."]
        #[serde(
            rename = "expandGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_groups: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the resource section of the result will expand any resource attached to an IAM policy to include resources lower in the resource hierarchy. For example, if the request analyzes for which resources user A has permission P, and the results include an IAM policy with P on a GCP folder, the results will also include resources in that folder with permission P. If resource_selector is specified, the resource section of the result will be determined by the selector, and this flag will have no effect. Default is false."]
        #[serde(
            rename = "expandResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_resources: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the access section of result will expand any roles appearing in IAM policy bindings to include their permissions. If access_selector is specified, the access section of the result will be determined by the selector, and this flag will have no effect. Default is false."]
        #[serde(
            rename = "expandRoles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_roles: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the result will output group identity edges, starting from the binding's group members, to any expanded identities. Default is false."]
        #[serde(
            rename = "outputGroupEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_group_edges: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the result will output resource edges, starting from the policy attached resource, to any expanded resources. Default is false."]
        #[serde(
            rename = "outputResourceEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_resource_edges: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Options {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Options {
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
    pub struct Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(
            rename = "auditConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::AuditConfig>>,
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Policy {
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
    pub struct ResourceSelector {
        #[doc = "Required. The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format) of a resource of [supported resource types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types)."]
        #[serde(
            rename = "fullResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResourceSelector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceSelector {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
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
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
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
    #[doc = "Actions that can be performed on the v_1p_4beta_1 resource"]
    pub fn v_1p_4beta_1(&self) -> crate::resources::v_1p_4beta_1::V1P4Beta1Actions {
        crate::resources::v_1p_4beta_1::V1P4Beta1Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod v_1p_4beta_1 {
        pub mod params {}
        pub struct V1P4Beta1Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1P4Beta1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Analyzes IAM policies to answer which identities have what accesses on which resources."]
            pub fn analyze_iam_policy(
                &self,
                parent: impl Into<String>,
            ) -> AnalyzeIamPolicyRequestBuilder {
                AnalyzeIamPolicyRequestBuilder {
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
                    parent: parent.into(),
                    analysis_query_access_selector_permissions: None,
                    analysis_query_access_selector_roles: None,
                    analysis_query_identity_selector_identity: None,
                    analysis_query_resource_selector_full_resource_name: None,
                    options_analyze_service_account_impersonation: None,
                    options_execution_timeout: None,
                    options_expand_groups: None,
                    options_expand_resources: None,
                    options_expand_roles: None,
                    options_output_group_edges: None,
                    options_output_resource_edges: None,
                }
            }
            #[doc = "Exports the answers of which identities have what accesses on which resources to a Google Cloud Storage destination. The output format is the JSON format that represents a AnalyzeIamPolicyResponse in the JSON format. This method implements the google.longrunning.Operation, which allows you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. The metadata contains the request to help callers to map responses to requests."]
            pub fn export_iam_policy_analysis(
                &self,
                request: crate::schemas::ExportIamPolicyAnalysisRequest,
                parent: impl Into<String>,
            ) -> ExportIamPolicyAnalysisRequestBuilder {
                ExportIamPolicyAnalysisRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        }
        #[doc = "Created via [V1P4Beta1Actions::analyze_iam_policy()](struct.V1P4Beta1Actions.html#method.analyze_iam_policy)"]
        #[derive(Debug, Clone)]
        pub struct AnalyzeIamPolicyRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            parent: String,
            analysis_query_access_selector_permissions: Option<Vec<String>>,
            analysis_query_access_selector_roles: Option<Vec<String>>,
            analysis_query_identity_selector_identity: Option<String>,
            analysis_query_resource_selector_full_resource_name: Option<String>,
            options_analyze_service_account_impersonation: Option<bool>,
            options_execution_timeout: Option<String>,
            options_expand_groups: Option<bool>,
            options_expand_resources: Option<bool>,
            options_expand_roles: Option<bool>,
            options_output_group_edges: Option<bool>,
            options_output_resource_edges: Option<bool>,
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
        impl<'a> AnalyzeIamPolicyRequestBuilder<'a> {
            #[doc = "Optional. The permissions to appear in result."]
            pub fn analysis_query_access_selector_permissions(
                mut self,
                value: impl Into<Vec<String>>,
            ) -> Self {
                self.analysis_query_access_selector_permissions = Some(value.into());
                self
            }
            #[doc = "Optional. The roles to appear in result."]
            pub fn analysis_query_access_selector_roles(
                mut self,
                value: impl Into<Vec<String>>,
            ) -> Self {
                self.analysis_query_access_selector_roles = Some(value.into());
                self
            }
            #[doc = "Required. The identity appear in the form of members in [IAM policy binding](https://cloud.google.com/iam/reference/rest/v1/Binding). The examples of supported forms are: \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\". Notice that wildcard characters (such as * and ?) are not supported. You must give a specific identity."]
            pub fn analysis_query_identity_selector_identity(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.analysis_query_identity_selector_identity = Some(value.into());
                self
            }
            #[doc = "Required. The [full resource name](https://cloud.google.com/asset-inventory/docs/resource-name-format) of a resource of [supported resource types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types)."]
            pub fn analysis_query_resource_selector_full_resource_name(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.analysis_query_resource_selector_full_resource_name = Some(value.into());
                self
            }
            #[doc = "Optional. If true, the response will include access analysis from identities to resources via service account impersonation. This is a very expensive operation, because many derived queries will be executed. We highly recommend you use AssetService.ExportIamPolicyAnalysis rpc instead. For example, if the request analyzes for which resources user A has permission P, and there's an IAM policy states user A has iam.serviceAccounts.getAccessToken permission to a service account SA, and there's another IAM policy states service account SA has permission P to a GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Another example, if the request analyzes for who has permission P to a GCP folder F, and there's an IAM policy states user A has iam.serviceAccounts.actAs permission to a service account SA, and there's another IAM policy states service account SA has permission P to the GCP folder F, then user A potentially has access to the GCP folder F. And those advanced analysis results will be included in AnalyzeIamPolicyResponse.service_account_impersonation_analysis. Default is false."]
            pub fn options_analyze_service_account_impersonation(mut self, value: bool) -> Self {
                self.options_analyze_service_account_impersonation = Some(value);
                self
            }
            #[doc = "Optional. Amount of time executable has to complete. See JSON representation of [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json). If this field is set with a value less than the RPC deadline, and the execution of your query hasn't finished in the specified execution timeout, you will get a response with partial result. Otherwise, your query's execution will continue until the RPC deadline. If it's not finished until then, you will get a DEADLINE_EXCEEDED error. Default is empty."]
            pub fn options_execution_timeout(mut self, value: impl Into<String>) -> Self {
                self.options_execution_timeout = Some(value.into());
                self
            }
            #[doc = "Optional. If true, the identities section of the result will expand any Google groups appearing in an IAM policy binding. If identity_selector is specified, the identity in the result will be determined by the selector, and this flag will have no effect. Default is false."]
            pub fn options_expand_groups(mut self, value: bool) -> Self {
                self.options_expand_groups = Some(value);
                self
            }
            #[doc = "Optional. If true, the resource section of the result will expand any resource attached to an IAM policy to include resources lower in the resource hierarchy. For example, if the request analyzes for which resources user A has permission P, and the results include an IAM policy with P on a GCP folder, the results will also include resources in that folder with permission P. If resource_selector is specified, the resource section of the result will be determined by the selector, and this flag will have no effect. Default is false."]
            pub fn options_expand_resources(mut self, value: bool) -> Self {
                self.options_expand_resources = Some(value);
                self
            }
            #[doc = "Optional. If true, the access section of result will expand any roles appearing in IAM policy bindings to include their permissions. If access_selector is specified, the access section of the result will be determined by the selector, and this flag will have no effect. Default is false."]
            pub fn options_expand_roles(mut self, value: bool) -> Self {
                self.options_expand_roles = Some(value);
                self
            }
            #[doc = "Optional. If true, the result will output group identity edges, starting from the binding's group members, to any expanded identities. Default is false."]
            pub fn options_output_group_edges(mut self, value: bool) -> Self {
                self.options_output_group_edges = Some(value);
                self
            }
            #[doc = "Optional. If true, the result will output resource edges, starting from the policy attached resource, to any expanded resources. Default is false."]
            pub fn options_output_resource_edges(mut self, value: bool) -> Self {
                self.options_output_resource_edges = Some(value);
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeIamPolicyResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeIamPolicyResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1p4beta1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":analyzeIamPolicy");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self
                    .analysis_query_access_selector_permissions
                    .iter()
                    .flatten()
                {
                    req = req.query(&[("analysisQuery.accessSelector.permissions", value)]);
                }
                for value in self.analysis_query_access_selector_roles.iter().flatten() {
                    req = req.query(&[("analysisQuery.accessSelector.roles", value)]);
                }
                req = req.query(&[(
                    "analysisQuery.identitySelector.identity",
                    &self.analysis_query_identity_selector_identity,
                )]);
                req = req.query(&[(
                    "analysisQuery.resourceSelector.fullResourceName",
                    &self.analysis_query_resource_selector_full_resource_name,
                )]);
                req = req.query(&[(
                    "options.analyzeServiceAccountImpersonation",
                    &self.options_analyze_service_account_impersonation,
                )]);
                req = req.query(&[("options.executionTimeout", &self.options_execution_timeout)]);
                req = req.query(&[("options.expandGroups", &self.options_expand_groups)]);
                req = req.query(&[("options.expandResources", &self.options_expand_resources)]);
                req = req.query(&[("options.expandRoles", &self.options_expand_roles)]);
                req = req.query(&[("options.outputGroupEdges", &self.options_output_group_edges)]);
                req = req.query(&[(
                    "options.outputResourceEdges",
                    &self.options_output_resource_edges,
                )]);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [V1P4Beta1Actions::export_iam_policy_analysis()](struct.V1P4Beta1Actions.html#method.export_iam_policy_analysis)"]
        #[derive(Debug, Clone)]
        pub struct ExportIamPolicyAnalysisRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ExportIamPolicyAnalysisRequest,
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
        impl<'a> ExportIamPolicyAnalysisRequestBuilder<'a> {
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
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://cloudasset.googleapis.com/".to_owned();
                output.push_str("v1p4beta1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":exportIamPolicyAnalysis");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
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
    IO(std::io::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::IO(_) => None,
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
            Error::IO(err) => write!(f, "IO Error: {}", err),
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
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
        body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>>,
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
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        },
    }

    impl futures::io::AsyncRead for RelatedMultiPartReader {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            ctx: &mut futures::task::Context,
            buf: &mut [u8],
        ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
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
                        let body = std::pin::Pin::new(body);
                        let written = match futures::io::AsyncRead::poll_read(body, ctx, rem_buf) {
                            futures::task::Poll::Ready(Ok(n)) => n,
                            futures::task::Poll::Ready(Err(err)) => {
                                return futures::task::Poll::Ready(Err(err));
                            }
                            futures::task::Poll::Pending => return futures::task::Poll::Pending,
                        };
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

            futures::task::Poll::Ready(Ok(bytes_written))
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
