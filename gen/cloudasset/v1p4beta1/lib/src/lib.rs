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
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AnalyzeIamPolicyResponse {
        #[doc = "Represents whether all entries in the main_analysis and\nservice_account_impersonation_analysis have been fully explored to\nanswer the query in the request."]
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
        #[doc = "A list of non-critical errors happened during the request handling to\nexplain why `fully_explored` is false, or empty if no error happened."]
        #[serde(
            rename = "nonCriticalErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_critical_errors:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisState>>,
        #[doc = "The service account impersonation analysis if\nAnalyzeIamPolicyRequest.analyze_service_account_impersonation is\nenabled."]
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
    pub struct Binding {
        #[doc = "The condition that is associated with this binding.\nNOTE: An unsatisfied condition will not allow user access via current\nbinding. Different bindings, including their conditions, are examined\nindependently."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource.\n`members` can have the following values:\n\n* `allUsers`: A special identifier that represents anyone who is\n  on the internet; with or without a Google account.\n\n* `allAuthenticatedUsers`: A special identifier that represents anyone\n  who is authenticated with a Google account or a service account.\n\n* `user:{emailid}`: An email address that represents a specific Google\n  account. For example, `alice@example.com` .\n\n* `serviceAccount:{emailid}`: An email address that represents a service\n  account. For example, `my-other-app@appspot.gserviceaccount.com`.\n\n* `group:{emailid}`: An email address that represents a Google group.\n  For example, `admins@example.com`.\n\n* `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique\n  identifier) representing a user that has been recently deleted. For\n  example, `alice@example.com?uid=123456789012345678901`. If the user is\n  recovered, this value reverts to `user:{emailid}` and the recovered user\n  retains the role in the binding.\n\n* `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus\n  unique identifier) representing a service account that has been recently\n  deleted. For example,\n  `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`.\n  If the service account is undeleted, this value reverts to\n  `serviceAccount:{emailid}` and the undeleted service account retains the\n  role in the binding.\n\n* `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique\n  identifier) representing a Google group that has been recently\n  deleted. For example, `admins@example.com?uid=123456789012345678901`. If\n  the group is recovered, this value reverts to `group:{emailid}` and the\n  recovered group retains the role in the binding.\n\n* `domain:{domain}`: The G Suite domain (primary) that represents all the\n  users of that domain. For example, `google.com` or `example.com`."]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`.\nFor example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
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
        #[doc = "Optional. Description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language\nsyntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing\nits purpose. This can be used e.g. in UIs which allow to enter the\nexpression."]
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
        #[doc = "Required. The uri of the Cloud Storage object. It's the same uri that is used by\ngsutil. For example: \"gs://bucket_name/object_name\". See [Viewing and\nEditing Object\nMetadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)\nfor more information."]
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
        #[doc = "The analysis state of this access node."]
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
        #[doc = "The accesses that match one of the following conditions:\n\n* The access_selector, if it is specified in request;\n* Otherwise, access specifiers reachable from the policy binding's role."]
        #[serde(
            rename = "accesses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accesses: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Access>>,
        #[doc = "Resource edges of the graph starting from the policy attached\nresource to any descendant resources. The Edge.source_node contains\nthe full resource name of a parent resource and Edge.target_node\ncontains the full resource name of a child resource. This field is\npresent only if the output_resource_edges option is enabled in request."]
        #[serde(
            rename = "resourceEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_edges:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Edge>>,
        #[doc = "The resources that match one of the following conditions:\n\n* The resource_selector, if it is specified in request;\n* Otherwise, resources reachable from the policy attached resource."]
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
        #[doc = "The Google standard error code that best describes the state.\nFor example:\n\n* OK means the node has been successfully explored;\n* PERMISSION_DENIED means an access denied error is encountered;\n* DEADLINE_EXCEEDED means the node hasn't been explored in time;"]
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
        #[doc = "The operation was aborted, typically due to a concurrency issue such as\na sequencer check failure or transaction abort.\n\nSee the guidelines above for deciding between `FAILED_PRECONDITION`,\n`ABORTED`, and `UNAVAILABLE`.\n\nHTTP Mapping: 409 Conflict"]
        Aborted,
        #[doc = "The entity that a client attempted to create (e.g., file or directory)\nalready exists.\n\nHTTP Mapping: 409 Conflict"]
        AlreadyExists,
        #[doc = "The operation was cancelled, typically by the caller.\n\nHTTP Mapping: 499 Client Closed Request"]
        Cancelled,
        #[doc = "Unrecoverable data loss or corruption.\n\nHTTP Mapping: 500 Internal Server Error"]
        DataLoss,
        #[doc = "The deadline expired before the operation could complete. For operations\nthat change the state of the system, this error may be returned\neven if the operation has completed successfully.  For example, a\nsuccessful response from a server could have been delayed long\nenough for the deadline to expire.\n\nHTTP Mapping: 504 Gateway Timeout"]
        DeadlineExceeded,
        #[doc = "The operation was rejected because the system is not in a state\nrequired for the operation's execution.  For example, the directory\nto be deleted is non-empty, an rmdir operation is applied to\na non-directory, etc.\n\nService implementors can use the following guidelines to decide\nbetween `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:\n(a) Use `UNAVAILABLE` if the client can retry just the failing call.\n(b) Use `ABORTED` if the client should retry at a higher level\n(e.g., when a client-specified test-and-set fails, indicating the\nclient should restart a read-modify-write sequence).\n(c) Use `FAILED_PRECONDITION` if the client should not retry until\nthe system state has been explicitly fixed.  E.g., if an \"rmdir\"\nfails because the directory is non-empty, `FAILED_PRECONDITION`\nshould be returned since the client should not retry unless\nthe files are deleted from the directory.\n\nHTTP Mapping: 400 Bad Request"]
        FailedPrecondition,
        #[doc = "Internal errors.  This means that some invariants expected by the\nunderlying system have been broken.  This error code is reserved\nfor serious errors.\n\nHTTP Mapping: 500 Internal Server Error"]
        Internal,
        #[doc = "The client specified an invalid argument.  Note that this differs\nfrom `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments\nthat are problematic regardless of the state of the system\n(e.g., a malformed file name).\n\nHTTP Mapping: 400 Bad Request"]
        InvalidArgument,
        #[doc = "Some requested entity (e.g., file or directory) was not found.\n\nNote to server developers: if a request is denied for an entire class\nof users, such as gradual feature rollout or undocumented whitelist,\n`NOT_FOUND` may be used. If a request is denied for some users within\na class of users, such as user-based access control, `PERMISSION_DENIED`\nmust be used.\n\nHTTP Mapping: 404 Not Found"]
        NotFound,
        #[doc = "Not an error; returned on success\n\nHTTP Mapping: 200 OK"]
        Ok,
        #[doc = "The operation was attempted past the valid range.  E.g., seeking or\nreading past end-of-file.\n\nUnlike `INVALID_ARGUMENT`, this error indicates a problem that may\nbe fixed if the system state changes. For example, a 32-bit file\nsystem will generate `INVALID_ARGUMENT` if asked to read at an\noffset that is not in the range [0,2^32-1], but it will generate\n`OUT_OF_RANGE` if asked to read from an offset past the current\nfile size.\n\nThere is a fair bit of overlap between `FAILED_PRECONDITION` and\n`OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific\nerror) when it applies so that callers who are iterating through\na space can easily look for an `OUT_OF_RANGE` error to detect when\nthey are done.\n\nHTTP Mapping: 400 Bad Request"]
        OutOfRange,
        #[doc = "The caller does not have permission to execute the specified\noperation. `PERMISSION_DENIED` must not be used for rejections\ncaused by exhausting some resource (use `RESOURCE_EXHAUSTED`\ninstead for those errors). `PERMISSION_DENIED` must not be\nused if the caller can not be identified (use `UNAUTHENTICATED`\ninstead for those errors). This error code does not imply the\nrequest is valid or the requested entity exists or satisfies\nother pre-conditions.\n\nHTTP Mapping: 403 Forbidden"]
        PermissionDenied,
        #[doc = "Some resource has been exhausted, perhaps a per-user quota, or\nperhaps the entire file system is out of space.\n\nHTTP Mapping: 429 Too Many Requests"]
        ResourceExhausted,
        #[doc = "The request does not have valid authentication credentials for the\noperation.\n\nHTTP Mapping: 401 Unauthorized"]
        Unauthenticated,
        #[doc = "The service is currently unavailable.  This is most likely a\ntransient condition, which can be corrected by retrying with\na backoff. Note that it is not always safe to retry\nnon-idempotent operations.\n\nSee the guidelines above for deciding between `FAILED_PRECONDITION`,\n`ABORTED`, and `UNAVAILABLE`.\n\nHTTP Mapping: 503 Service Unavailable"]
        Unavailable,
        #[doc = "The operation is not implemented or is not supported/enabled in this\nservice.\n\nHTTP Mapping: 501 Not Implemented"]
        Unimplemented,
        #[doc = "Unknown error.  For example, this error may be returned when\na `Status` value received from another address space belongs to\nan error space that is not known in this address space.  Also\nerrors raised by APIs that do not return enough error information\nmay be converted to this error.\n\nHTTP Mapping: 500 Internal Server Error"]
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
        #[doc = "The analysis state of this identity node."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisState>,
        #[doc = "The identity name in any form of members appear in\n[IAM policy\nbinding](https://cloud.google.com/iam/reference/rest/v1/Binding), such\nas:\n\n* user:foo@google.com\n* group:group1@google.com\n* serviceAccount:s1@prj1.iam.gserviceaccount.com\n* projectOwner:some_project_id\n* domain:google.com\n* allUsers\n* etc."]
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
        #[doc = "Group identity edges of the graph starting from the binding's\ngroup members to any node of the identities. The Edge.source_node\ncontains a group, such as \"group:parent@google.com\". The\nEdge.target_node contains a member of the group,\nsuch as \"group:child@google.com\" or \"user:foo@google.com\".\nThis field is present only if the output_group_edges option is enabled in\nrequest."]
        #[serde(
            rename = "groupEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_edges: ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1Edge>>,
        #[doc = "Only the identities that match one of the following conditions will be\npresented:\n\n* The identity_selector, if it is specified in request;\n* Otherwise, identities reachable from the policy binding's members."]
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
        #[doc = "The analysis state of this resource node."]
        #[serde(
            rename = "analysisState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_state:
            ::std::option::Option<crate::schemas::GoogleCloudAssetV1P4Beta1AnalysisState>,
        #[doc = "The [full resource name](https://aip.dev/122#full-resource-names)."]
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
        #[doc = "A list of IamPolicyAnalysisResult that matches the analysis query, or\nempty if no result is found."]
        #[serde(
            rename = "analysisResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_results: ::std::option::Option<Vec<crate::schemas::IamPolicyAnalysisResult>>,
        #[doc = "Represents whether all entries in the analysis_results have been\nfully explored to answer the query."]
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
        #[doc = "Optional. Specifies roles or permissions for analysis. Leaving it empty\nmeans ANY."]
        #[serde(
            rename = "accessSelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_selector: ::std::option::Option<crate::schemas::AccessSelector>,
        #[doc = "Optional. Specifies an identity for analysis. Leaving it empty means ANY."]
        #[serde(
            rename = "identitySelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity_selector: ::std::option::Option<crate::schemas::IdentitySelector>,
        #[doc = "Required. The relative name of the root asset. Only resources and IAM policies within\nthe parent will be analyzed. This can only be an organization number (such\nas \"organizations/123\") or a folder number (such as \"folders/123\")."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "Optional. Specifies a resource for analysis. Leaving it empty means ANY."]
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
        #[doc = "The access control lists derived from the iam_binding that match or\npotentially match resource and access selectors specified in the request."]
        #[serde(
            rename = "accessControlLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_control_lists:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudAssetV1P4Beta1AccessControlList>>,
        #[doc = "The full name of the resource to which the iam_binding policy attaches."]
        #[serde(
            rename = "attachedResourceFullName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attached_resource_full_name: ::std::option::Option<String>,
        #[doc = "Represents whether all nodes in the transitive closure of the\niam_binding node have been explored."]
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
        #[doc = "The identity list derived from members of the iam_binding that match or\npotentially match identity selector specified in the request."]
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
        #[doc = "Required. The identity appear in the form of members in\n[IAM policy\nbinding](https://cloud.google.com/iam/reference/rest/v1/Binding)."]
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
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
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
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
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
        #[doc = "Optional. If true, the response will include access analysis from identities to\nresources via service account impersonation. This is a very expensive\noperation, because many derived queries will be executed.\n\nFor example, if the request analyzes for which resources user A has\npermission P, and there's an IAM policy states user A has\niam.serviceAccounts.getAccessToken permission to a service account SA,\nand there's another IAM policy states service account SA has permission P\nto a GCP folder F, then user A potentially has access to the GCP folder\nF. And those advanced analysis results will be included in\nAnalyzeIamPolicyResponse.service_account_impersonation_analysis.\n\nAnother example, if the request analyzes for who has\npermission P to a GCP folder F, and there's an IAM policy states user A\nhas iam.serviceAccounts.actAs permission to a service account SA, and\nthere's another IAM policy states service account SA has permission P to\nthe GCP folder F, then user A potentially has access to the GCP folder\nF. And those advanced analysis results will be included in\nAnalyzeIamPolicyResponse.service_account_impersonation_analysis.\n\nDefault is false."]
        #[serde(
            rename = "analyzeServiceAccountImpersonation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analyze_service_account_impersonation: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the identities section of the result will expand any\nGoogle groups appearing in an IAM policy binding.\n\nIf identity_selector is specified, the identity in the result will\nbe determined by the selector, and this flag will have no effect.\n\nDefault is false."]
        #[serde(
            rename = "expandGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_groups: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the resource section of the result will expand any\nresource attached to an IAM policy to include resources lower in the\nresource hierarchy.\n\nFor example, if the request analyzes for which resources user A has\npermission P, and the results include an IAM policy with P on a GCP\nfolder, the results will also include resources in that folder with\npermission P.\n\nIf resource_selector is specified, the resource section of the result\nwill be determined by the selector, and this flag will have no effect.\nDefault is false."]
        #[serde(
            rename = "expandResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_resources: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the access section of result will expand any roles\nappearing in IAM policy bindings to include their permissions.\n\nIf access_selector is specified, the access section of the result\nwill be determined by the selector, and this flag will have no effect.\n\nDefault is false."]
        #[serde(
            rename = "expandRoles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expand_roles: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the result will output group identity edges, starting\nfrom the binding's group members, to any expanded identities.\nDefault is false."]
        #[serde(
            rename = "outputGroupEdges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_group_edges: ::std::option::Option<bool>,
        #[doc = "Optional. If true, the result will output resource edges, starting\nfrom the policy attached resource, to any expanded resources.\nDefault is false."]
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
    pub struct ResourceSelector {
        #[doc = "Required. The [full resource\nname](https://cloud.google.com/apis/design/resource_names#full_resource_name)\n."]
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
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1P4Beta1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Analyzes IAM policies based on the specified request. Returns\na list of IamPolicyAnalysisResult matching the request."]
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
            #[doc = "Exports IAM policy analysis based on the specified request. This API\nimplements the google.longrunning.Operation API allowing you to keep\ntrack of the export. The metadata contains the request to help callers to\nmap responses to requests."]
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
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
            #[doc = "Required. The identity appear in the form of members in\n[IAM policy\nbinding](https://cloud.google.com/iam/reference/rest/v1/Binding)."]
            pub fn analysis_query_identity_selector_identity(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.analysis_query_identity_selector_identity = Some(value.into());
                self
            }
            #[doc = "Required. The [full resource\nname](https://cloud.google.com/apis/design/resource_names#full_resource_name)\n."]
            pub fn analysis_query_resource_selector_full_resource_name(
                mut self,
                value: impl Into<String>,
            ) -> Self {
                self.analysis_query_resource_selector_full_resource_name = Some(value.into());
                self
            }
            #[doc = "Optional. If true, the response will include access analysis from identities to\nresources via service account impersonation. This is a very expensive\noperation, because many derived queries will be executed. We highly\nrecommend you use ExportIamPolicyAnalysis rpc instead.\n\nFor example, if the request analyzes for which resources user A has\npermission P, and there's an IAM policy states user A has\niam.serviceAccounts.getAccessToken permission to a service account SA,\nand there's another IAM policy states service account SA has permission P\nto a GCP folder F, then user A potentially has access to the GCP folder\nF. And those advanced analysis results will be included in\nAnalyzeIamPolicyResponse.service_account_impersonation_analysis.\n\nAnother example, if the request analyzes for who has\npermission P to a GCP folder F, and there's an IAM policy states user A\nhas iam.serviceAccounts.actAs permission to a service account SA, and\nthere's another IAM policy states service account SA has permission P to\nthe GCP folder F, then user A potentially has access to the GCP folder\nF. And those advanced analysis results will be included in\nAnalyzeIamPolicyResponse.service_account_impersonation_analysis.\n\nDefault is false."]
            pub fn options_analyze_service_account_impersonation(mut self, value: bool) -> Self {
                self.options_analyze_service_account_impersonation = Some(value);
                self
            }
            #[doc = "Optional. Amount of time executable has to complete.  See JSON representation of\n[Duration](https://developers.google.com/protocol-buffers/docs/proto3#json).\n\nIf this field is set with a value less than the RPC deadline, and the\nexecution of your query hasn't finished in the specified\nexecution timeout,  you will get a response with partial result.\nOtherwise, your query's execution will continue until the RPC deadline.\nIf it's not finished until then, you will get a  DEADLINE_EXCEEDED error.\n\nDefault is empty."]
            pub fn options_execution_timeout(mut self, value: impl Into<String>) -> Self {
                self.options_execution_timeout = Some(value.into());
                self
            }
            #[doc = "Optional. If true, the identities section of the result will expand any\nGoogle groups appearing in an IAM policy binding.\n\nIf identity_selector is specified, the identity in the result will\nbe determined by the selector, and this flag will have no effect.\n\nDefault is false."]
            pub fn options_expand_groups(mut self, value: bool) -> Self {
                self.options_expand_groups = Some(value);
                self
            }
            #[doc = "Optional. If true, the resource section of the result will expand any\nresource attached to an IAM policy to include resources lower in the\nresource hierarchy.\n\nFor example, if the request analyzes for which resources user A has\npermission P, and the results include an IAM policy with P on a GCP\nfolder, the results will also include resources in that folder with\npermission P.\n\nIf resource_selector is specified, the resource section of the result\nwill be determined by the selector, and this flag will have no effect.\nDefault is false."]
            pub fn options_expand_resources(mut self, value: bool) -> Self {
                self.options_expand_resources = Some(value);
                self
            }
            #[doc = "Optional. If true, the access section of result will expand any roles\nappearing in IAM policy bindings to include their permissions.\n\nIf access_selector is specified, the access section of the result\nwill be determined by the selector, and this flag will have no effect.\n\nDefault is false."]
            pub fn options_expand_roles(mut self, value: bool) -> Self {
                self.options_expand_roles = Some(value);
                self
            }
            #[doc = "Optional. If true, the result will output group identity edges, starting\nfrom the binding's group members, to any expanded identities.\nDefault is false."]
            pub fn options_output_group_edges(mut self, value: bool) -> Self {
                self.options_output_group_edges = Some(value);
                self
            }
            #[doc = "Optional. If true, the result will output resource edges, starting\nfrom the policy attached resource, to any expanded resources.\nDefault is false."]
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
            ) -> Result<crate::schemas::AnalyzeIamPolicyResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AnalyzeIamPolicyResponse, crate::Error> {
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[(
                    "analysisQuery.accessSelector.permissions",
                    &self.analysis_query_access_selector_permissions,
                )]);
                let req = req.query(&[(
                    "analysisQuery.accessSelector.roles",
                    &self.analysis_query_access_selector_roles,
                )]);
                let req = req.query(&[(
                    "analysisQuery.identitySelector.identity",
                    &self.analysis_query_identity_selector_identity,
                )]);
                let req = req.query(&[(
                    "analysisQuery.resourceSelector.fullResourceName",
                    &self.analysis_query_resource_selector_full_resource_name,
                )]);
                let req = req.query(&[(
                    "options.analyzeServiceAccountImpersonation",
                    &self.options_analyze_service_account_impersonation,
                )]);
                let req =
                    req.query(&[("options.executionTimeout", &self.options_execution_timeout)]);
                let req = req.query(&[("options.expandGroups", &self.options_expand_groups)]);
                let req = req.query(&[("options.expandResources", &self.options_expand_resources)]);
                let req = req.query(&[("options.expandRoles", &self.options_expand_roles)]);
                let req =
                    req.query(&[("options.outputGroupEdges", &self.options_output_group_edges)]);
                let req = req.query(&[(
                    "options.outputResourceEdges",
                    &self.options_output_resource_edges,
                )]);
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
        #[doc = "Created via [V1P4Beta1Actions::export_iam_policy_analysis()](struct.V1P4Beta1Actions.html#method.export_iam_policy_analysis)"]
        #[derive(Debug, Clone)]
        pub struct ExportIamPolicyAnalysisRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
