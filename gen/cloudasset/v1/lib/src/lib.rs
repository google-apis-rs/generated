pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Asset {
        #[doc = "Type of the asset. Example: \"compute.googleapis.com/Disk\"."]
        #[serde(rename = "assetType", default)]
        pub asset_type: Option<String>,
        #[doc = "Representation of the actual Cloud IAM policy set on a cloud resource. For\neach resource, there must be at most one Cloud IAM policy set on it."]
        #[serde(rename = "iamPolicy", default)]
        pub iam_policy: Option<crate::schemas::Policy>,
        #[doc = "The full name of the asset. For example:\n`//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.\nSee [Resource\nNames](https://cloud.google.com/apis/design/resource_names#full_resource_name)\nfor more information."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Representation of the Cloud Organization Policy set on an asset. For each\nasset, there could be multiple Organization policies with different\nconstraints."]
        #[serde(rename = "orgPolicy", default)]
        pub org_policy: Option<Vec<crate::schemas::GoogleCloudOrgpolicyV1Policy>>,
        #[doc = "Representation of the resource."]
        #[serde(rename = "resource", default)]
        pub resource: Option<crate::schemas::Resource>,
    }
    impl ::field_selector::FieldSelector for Asset {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        #[serde(rename = "auditLogConfigs", default)]
        pub audit_log_configs: Option<Vec<crate::schemas::AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging.\nFor example, `storage.googleapis.com`, `cloudsql.googleapis.com`.\n`allServices` is a special value that covers all services."]
        #[serde(rename = "service", default)]
        pub service: Option<String>,
    }
    impl ::field_selector::FieldSelector for AuditConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AuditLogConfigLogType {
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
    }
    impl AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
                AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                AuditLogConfigLogType::DataRead => "DATA_READ",
            }
        }
    }
    impl ::std::fmt::Display for AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOG_TYPE_UNSPECIFIED" => AuditLogConfigLogType::LogTypeUnspecified,
                "ADMIN_READ" => AuditLogConfigLogType::AdminRead,
                "DATA_WRITE" => AuditLogConfigLogType::DataWrite,
                "DATA_READ" => AuditLogConfigLogType::DataRead,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
        #[doc = "Specifies the identities that do not cause logging for this type of\npermission.\nFollows the same format of Binding.members."]
        #[serde(rename = "exemptedMembers", default)]
        pub exempted_members: Option<Vec<String>>,
        #[doc = "Specifies whether principals can be exempted for the same LogType in\nlower-level resource policies. If true, any lower-level exemptions will\nbe ignored."]
        #[serde(rename = "ignoreChildExemptions", default)]
        pub ignore_child_exemptions: Option<bool>,
        #[doc = "The log type that this config enables."]
        #[serde(rename = "logType", default)]
        pub log_type: Option<crate::schemas::AuditLogConfigLogType>,
    }
    impl ::field_selector::FieldSelector for AuditLogConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchGetAssetsHistoryResponse {
        #[doc = "A list of assets with valid time windows."]
        #[serde(rename = "assets", default)]
        pub assets: Option<Vec<crate::schemas::TemporalAsset>>,
    }
    impl ::field_selector::FieldSelector for BatchGetAssetsHistoryResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        #[serde(rename = "condition", default)]
        pub condition: Option<crate::schemas::Expr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource.\n`members` can have the following values:\n\n* `allUsers`: A special identifier that represents anyone who is\n  on the internet; with or without a Google account.\n\n* `allAuthenticatedUsers`: A special identifier that represents anyone\n  who is authenticated with a Google account or a service account.\n\n* `user:{emailid}`: An email address that represents a specific Google\n  account. For example, `alice@example.com` .\n\n* `serviceAccount:{emailid}`: An email address that represents a service\n  account. For example, `my-other-app@appspot.gserviceaccount.com`.\n\n* `group:{emailid}`: An email address that represents a Google group.\n  For example, `admins@example.com`.\n\n* `domain:{domain}`: The G Suite domain (primary) that represents all the\n  users of that domain. For example, `google.com` or `example.com`."]
        #[serde(rename = "members", default)]
        pub members: Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`.\nFor example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
    }
    impl ::field_selector::FieldSelector for Binding {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExportAssetsRequestContentType {
        #[doc = "Unspecified content type."]
        ContentTypeUnspecified,
        #[doc = "Resource metadata."]
        Resource,
        #[doc = "The actual IAM policy set on a resource."]
        IamPolicy,
        #[doc = "The Cloud Organization Policy set on an asset."]
        OrgPolicy,
    }
    impl ExportAssetsRequestContentType {
        pub fn as_str(self) -> &'static str {
            match self {
                ExportAssetsRequestContentType::ContentTypeUnspecified => {
                    "CONTENT_TYPE_UNSPECIFIED"
                }
                ExportAssetsRequestContentType::Resource => "RESOURCE",
                ExportAssetsRequestContentType::IamPolicy => "IAM_POLICY",
                ExportAssetsRequestContentType::OrgPolicy => "ORG_POLICY",
            }
        }
    }
    impl ::std::fmt::Display for ExportAssetsRequestContentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExportAssetsRequestContentType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExportAssetsRequestContentType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_TYPE_UNSPECIFIED" => {
                    ExportAssetsRequestContentType::ContentTypeUnspecified
                }
                "RESOURCE" => ExportAssetsRequestContentType::Resource,
                "IAM_POLICY" => ExportAssetsRequestContentType::IamPolicy,
                "ORG_POLICY" => ExportAssetsRequestContentType::OrgPolicy,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
    pub struct ExportAssetsRequest {
        #[doc = "A list of asset types of which to take a snapshot for. For example:\n\"compute.googleapis.com/Disk\". If specified, only matching assets will be\nreturned. See [Introduction to Cloud Asset\nInventory](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview)\nfor all supported asset types."]
        #[serde(rename = "assetTypes", default)]
        pub asset_types: Option<Vec<String>>,
        #[doc = "Asset content type. If not specified, no content but the asset name will be\nreturned."]
        #[serde(rename = "contentType", default)]
        pub content_type: Option<crate::schemas::ExportAssetsRequestContentType>,
        #[doc = "Required. Output configuration indicating where the results will be output\nto. All results will be in newline delimited JSON format."]
        #[serde(rename = "outputConfig", default)]
        pub output_config: Option<crate::schemas::OutputConfig>,
        #[doc = "Timestamp to take an asset snapshot. This can only be set to a timestamp\nbetween 2018-10-02 UTC (inclusive) and the current time. If not specified,\nthe current time will be used. Due to delays in resource data collection\nand indexing, there is a volatile window during which running the same\nquery may get different results."]
        #[serde(rename = "readTime", default)]
        pub read_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for ExportAssetsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        #[doc = "An optional description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Textual representation of an expression in\nCommon Expression Language syntax.\n\nThe application context of the containing message determines which\nwell-known feature set of CEL is supported."]
        #[serde(rename = "expression", default)]
        pub expression: Option<String>,
        #[doc = "An optional string indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "An optional title for the expression, i.e. a short string describing\nits purpose. This can be used e.g. in UIs which allow to enter the\nexpression."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Expr {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        #[doc = "The uri of the Cloud Storage object. It's the same uri that is used by\ngsutil. For example: \"gs://bucket_name/object_name\". See [Viewing and\nEditing Object\nMetadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)\nfor more information."]
        #[serde(rename = "uri", default)]
        pub uri: Option<String>,
        #[doc = "The uri prefix of all generated Cloud Storage objects. For example:\n\"gs://bucket_name/object_name_prefix\". Each object uri is in format:\n\"gs://bucket_name/object_name_prefix/<asset type>/<shard number> and only\ncontains assets for that type. <shard number> starts from 0. For example:\n\"gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0\" is\nthe first shard of output objects containing all\ncompute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be\nreturned if file with the same name \"gs://bucket_name/object_name_prefix\"\nalready exists."]
        #[serde(rename = "uriPrefix", default)]
        pub uri_prefix: Option<String>,
    }
    impl ::field_selector::FieldSelector for GcsDestination {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        #[doc = "If `true`, then the `Policy` is enforced. If `false`, then any\nconfiguration is acceptable.\n\nSuppose you have a `Constraint`\n`constraints/compute.disableSerialPortAccess` with `constraint_default`\nset to `ALLOW`. A `Policy` for that `Constraint` exhibits the following\nbehavior:\n\n* If the `Policy` at this resource has enforced set to `false`, serial\n  port connection attempts will be allowed.\n* If the `Policy` at this resource has enforced set to `true`, serial\n  port connection attempts will be refused.\n* If the `Policy` at this resource is `RestoreDefault`, serial port\n  connection attempts will be allowed.\n* If no `Policy` is set at this resource or anywhere higher in the\n  resource hierarchy, serial port connection attempts will be allowed.\n* If no `Policy` is set at this resource, but one exists higher in the\n  resource hierarchy, the behavior is as if the`Policy` were set at\n  this resource.\n\nThe following examples demonstrate the different possible layerings:\n\nExample 1 (nearest `Constraint` wins):\n`organizations/foo` has a `Policy` with:\n{enforced: false}\n`projects/bar` has no `Policy` set.\nThe constraint at `projects/bar` and `organizations/foo` will not be\nenforced.\n\nExample 2 (enforcement gets replaced):\n`organizations/foo` has a `Policy` with:\n{enforced: false}\n`projects/bar` has a `Policy` with:\n{enforced: true}\nThe constraint at `organizations/foo` is not enforced.\nThe constraint at `projects/bar` is enforced.\n\nExample 3 (RestoreDefault):\n`organizations/foo` has a `Policy` with:\n{enforced: true}\n`projects/bar` has a `Policy` with:\n{RestoreDefault: {}}\nThe constraint at `organizations/foo` is enforced.\nThe constraint at `projects/bar` is not enforced, because\n`constraint_default` for the `Constraint` is `ALLOW`."]
        #[serde(rename = "enforced", default)]
        pub enforced: Option<bool>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudOrgpolicyV1BooleanPolicy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    impl ::std::fmt::Display for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudOrgpolicyV1ListPolicyAllValues {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
        #[serde(rename = "allValues", default)]
        pub all_values: Option<crate::schemas::GoogleCloudOrgpolicyV1ListPolicyAllValues>,
        #[doc = "List of values allowed  at this resource. Can only be set if `all_values`\nis set to `ALL_VALUES_UNSPECIFIED`."]
        #[serde(rename = "allowedValues", default)]
        pub allowed_values: Option<Vec<String>>,
        #[doc = "List of values denied at this resource. Can only be set if `all_values`\nis set to `ALL_VALUES_UNSPECIFIED`."]
        #[serde(rename = "deniedValues", default)]
        pub denied_values: Option<Vec<String>>,
        #[doc = "Determines the inheritance behavior for this `Policy`.\n\nBy default, a `ListPolicy` set at a resource supercedes any `Policy` set\nanywhere up the resource hierarchy. However, if `inherit_from_parent` is\nset to `true`, then the values from the effective `Policy` of the parent\nresource are inherited, meaning the values set in this `Policy` are\nadded to the values inherited up the hierarchy.\n\nSetting `Policy` hierarchies that inherit both allowed values and denied\nvalues isn't recommended in most circumstances to keep the configuration\nsimple and understandable. However, it is possible to set a `Policy` with\n`allowed_values` set that inherits a `Policy` with `denied_values` set.\nIn this case, the values that are allowed must be in `allowed_values` and\nnot present in `denied_values`.\n\nFor example, suppose you have a `Constraint`\n`constraints/serviceuser.services`, which has a `constraint_type` of\n`list_constraint`, and with `constraint_default` set to `ALLOW`.\nSuppose that at the Organization level, a `Policy` is applied that\nrestricts the allowed API activations to {`E1`, `E2`}. Then, if a\n`Policy` is applied to a project below the Organization that has\n`inherit_from_parent` set to `false` and field all_values set to DENY,\nthen an attempt to activate any API will be denied.\n\nThe following examples demonstrate different possible layerings for\n`projects/bar` parented by `organizations/foo`:\n\nExample 1 (no inherited values):\n`organizations/foo` has a `Policy` with values:\n{allowed_values: \u{201c}E1\u{201d} allowed_values:\u{201d}E2\u{201d}}\n`projects/bar` has `inherit_from_parent` `false` and values:\n{allowed_values: \"E3\" allowed_values: \"E4\"}\nThe accepted values at `organizations/foo` are `E1`, `E2`.\nThe accepted values at `projects/bar` are `E3`, and `E4`.\n\nExample 2 (inherited values):\n`organizations/foo` has a `Policy` with values:\n{allowed_values: \u{201c}E1\u{201d} allowed_values:\u{201d}E2\u{201d}}\n`projects/bar` has a `Policy` with values:\n{value: \u{201c}E3\u{201d} value: \u{201d}E4\u{201d} inherit_from_parent: true}\nThe accepted values at `organizations/foo` are `E1`, `E2`.\nThe accepted values at `projects/bar` are `E1`, `E2`, `E3`, and `E4`.\n\nExample 3 (inheriting both allowed and denied values):\n`organizations/foo` has a `Policy` with values:\n{allowed_values: \"E1\" allowed_values: \"E2\"}\n`projects/bar` has a `Policy` with:\n{denied_values: \"E1\"}\nThe accepted values at `organizations/foo` are `E1`, `E2`.\nThe value accepted at `projects/bar` is `E2`.\n\nExample 4 (RestoreDefault):\n`organizations/foo` has a `Policy` with values:\n{allowed_values: \u{201c}E1\u{201d} allowed_values:\u{201d}E2\u{201d}}\n`projects/bar` has a `Policy` with values:\n{RestoreDefault: {}}\nThe accepted values at `organizations/foo` are `E1`, `E2`.\nThe accepted values at `projects/bar` are either all or none depending on\nthe value of `constraint_default` (if `ALLOW`, all; if\n`DENY`, none).\n\nExample 5 (no policy inherits parent policy):\n`organizations/foo` has no `Policy` set.\n`projects/bar` has no `Policy` set.\nThe accepted values at both levels are either all or none depending on\nthe value of `constraint_default` (if `ALLOW`, all; if\n`DENY`, none).\n\nExample 6 (ListConstraint allowing all):\n`organizations/foo` has a `Policy` with values:\n{allowed_values: \u{201c}E1\u{201d} allowed_values: \u{201d}E2\u{201d}}\n`projects/bar` has a `Policy` with:\n{all: ALLOW}\nThe accepted values at `organizations/foo` are `E1`, E2`. Any value is accepted at `projects/bar`.\n\nExample 7 (ListConstraint allowing none):\n`organizations/foo` has a `Policy` with values:\n{allowed_values: \u{201c}E1\u{201d} allowed_values: \u{201d}E2\u{201d}}\n`projects/bar` has a `Policy` with:\n{all: DENY}\nThe accepted values at `organizations/foo` are `E1`, E2`. No value is accepted at `projects/bar`.\n\nExample 10 (allowed and denied subtrees of Resource Manager hierarchy):\nGiven the following resource hierarchy\nO1->{F1, F2}; F1->{P1}; F2->{P2, P3},\n`organizations/foo` has a `Policy` with values:\n{allowed_values: \"under:organizations/O1\"}\n`projects/bar` has a `Policy` with:\n{allowed_values: \"under:projects/P3\"}\n{denied_values: \"under:folders/F2\"}\nThe accepted values at `organizations/foo` are `organizations/O1`,\n`folders/F1`, `folders/F2`, `projects/P1`, `projects/P2`,\n`projects/P3`.\nThe accepted values at `projects/bar` are `organizations/O1`,\n`folders/F1`, `projects/P1`."]
        #[serde(rename = "inheritFromParent", default)]
        pub inherit_from_parent: Option<bool>,
        #[doc = "Optional. The Google Cloud Console will try to default to a configuration\nthat matches the value specified in this `Policy`. If `suggested_value`\nis not set, it will inherit the value specified higher in the hierarchy,\nunless `inherit_from_parent` is `false`."]
        #[serde(rename = "suggestedValue", default)]
        pub suggested_value: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudOrgpolicyV1ListPolicy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        #[serde(rename = "booleanPolicy", default)]
        pub boolean_policy: Option<crate::schemas::GoogleCloudOrgpolicyV1BooleanPolicy>,
        #[doc = "The name of the `Constraint` the `Policy` is configuring, for example,\n`constraints/serviceuser.services`.\n\nImmutable after creation."]
        #[serde(rename = "constraint", default)]
        pub constraint: Option<String>,
        #[doc = "An opaque tag indicating the current version of the `Policy`, used for\nconcurrency control.\n\nWhen the `Policy` is returned from either a `GetPolicy` or a\n`ListOrgPolicy` request, this `etag` indicates the version of the current\n`Policy` to use when executing a read-modify-write loop.\n\nWhen the `Policy` is returned from a `GetEffectivePolicy` request, the\n`etag` will be unset.\n\nWhen the `Policy` is used in a `SetOrgPolicy` method, use the `etag` value\nthat was returned from a `GetOrgPolicy` request as part of a\nread-modify-write loop for concurrency control. Not setting the `etag`in a\n`SetOrgPolicy` request will result in an unconditional write of the\n`Policy`."]
        #[serde(rename = "etag", default)]
        pub etag: Option<Vec<u8>>,
        #[doc = "List of values either allowed or disallowed."]
        #[serde(rename = "listPolicy", default)]
        pub list_policy: Option<crate::schemas::GoogleCloudOrgpolicyV1ListPolicy>,
        #[doc = "Restores the default behavior of the constraint; independent of\n`Constraint` type."]
        #[serde(rename = "restoreDefault", default)]
        pub restore_default: Option<crate::schemas::GoogleCloudOrgpolicyV1RestoreDefault>,
        #[doc = "The time stamp the `Policy` was previously updated. This is set by the\nserver, not specified by the caller, and represents the last time a call to\n`SetOrgPolicy` was made for that `Policy`. Any value set by the client will\nbe ignored."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
        #[doc = "Version of the `Policy`. Default version is 0;"]
        #[serde(rename = "version", default)]
        pub version: Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudOrgpolicyV1Policy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct GoogleCloudOrgpolicyV1RestoreDefault;
    impl ::field_selector::FieldSelector for GoogleCloudOrgpolicyV1RestoreDefault {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Operation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct OutputConfig {
        #[doc = "Destination on Cloud Storage."]
        #[serde(rename = "gcsDestination", default)]
        pub gcs_destination: Option<crate::schemas::GcsDestination>,
    }
    impl ::field_selector::FieldSelector for OutputConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
        #[serde(rename = "auditConfigs", default)]
        pub audit_configs: Option<Vec<crate::schemas::AuditConfig>>,
        #[doc = "Associates a list of `members` to a `role`.\n`bindings` with no members will result in an error."]
        #[serde(rename = "bindings", default)]
        pub bindings: Option<Vec<crate::schemas::Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a policy from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform policy updates in order to avoid race\nconditions: An `etag` is returned in the response to `getIamPolicy`, and\nsystems are expected to put that etag in the request to `setIamPolicy` to\nensure that their change will be applied to the same version of the policy.\n\nIf no `etag` is provided in the call to `setIamPolicy`, then the existing\npolicy is overwritten."]
        #[serde(rename = "etag", default)]
        pub etag: Option<Vec<u8>>,
        #[doc = "Deprecated."]
        #[serde(rename = "version", default)]
        pub version: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Policy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Resource {
        #[doc = "The content of the resource, in which some sensitive fields are scrubbed\naway and may not be present."]
        #[serde(rename = "data", default)]
        pub data: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The URL of the discovery document containing the resource's JSON schema.\nFor example:\n`\"https://www.googleapis.com/discovery/v1/apis/compute/v1/rest\"`.\nIt will be left unspecified for resources without a discovery-based API,\nsuch as Cloud Bigtable."]
        #[serde(rename = "discoveryDocumentUri", default)]
        pub discovery_document_uri: Option<String>,
        #[doc = "The JSON schema name listed in the discovery document.\nExample: \"Project\". It will be left unspecified for resources (such as\nCloud Bigtable) without a discovery-based API."]
        #[serde(rename = "discoveryName", default)]
        pub discovery_name: Option<String>,
        #[doc = "The full name of the immediate parent of this resource. See\n[Resource\nNames](https://cloud.google.com/apis/design/resource_names#full_resource_name)\nfor more information.\n\nFor GCP assets, it is the parent resource defined in the [Cloud IAM policy\nhierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy).\nFor example:\n`\"//cloudresourcemanager.googleapis.com/projects/my_project_123\"`.\n\nFor third-party assets, it is up to the users to define."]
        #[serde(rename = "parent", default)]
        pub parent: Option<String>,
        #[doc = "The REST URL for accessing the resource. An HTTP GET operation using this\nURL returns the resource itself.\nExample:\n`https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`.\nIt will be left unspecified for resources without a REST API."]
        #[serde(rename = "resourceUrl", default)]
        pub resource_url: Option<String>,
        #[doc = "The API version. Example: \"v1\"."]
        #[serde(rename = "version", default)]
        pub version: Option<String>,
    }
    impl ::field_selector::FieldSelector for Resource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
    }
    impl ::field_selector::FieldSelector for Status {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct TemporalAsset {
        #[doc = "Asset."]
        #[serde(rename = "asset", default)]
        pub asset: Option<crate::schemas::Asset>,
        #[doc = "If the asset is deleted or not."]
        #[serde(rename = "deleted", default)]
        pub deleted: Option<bool>,
        #[doc = "The time window when the asset data and state was observed."]
        #[serde(rename = "window", default)]
        pub window: Option<crate::schemas::TimeWindow>,
    }
    impl ::field_selector::FieldSelector for TemporalAsset {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub struct TimeWindow {
        #[doc = "End time of the time window (inclusive).\nCurrent timestamp if not specified."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Start time of the time window (exclusive)."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for TimeWindow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    pub fn operations(&self) -> crate::operations::OperationsActions<A> {
        crate::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the v_1 resource"]
    pub fn v_1(&self) -> crate::v_1::V1Actions<A> {
        crate::v_1::V1Actions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod operations {
    pub mod params {}
    pub struct OperationsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> OperationsActions<'a, A> {
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
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://cloudasset.googleapis.com/".to_owned();
            output.push_str("v1/");
            output.push_str(&self.name);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod v_1 {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum BatchGetAssetsHistoryContentType {}
        impl BatchGetAssetsHistoryContentType {
            pub fn as_str(self) -> &'static str {
                match self {}
            }
        }
        impl ::std::fmt::Display for BatchGetAssetsHistoryContentType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for BatchGetAssetsHistoryContentType {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for BatchGetAssetsHistoryContentType {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    _ => {
                        return Err(::serde::de::Error::custom(format!(
                            "invalid enum for #name: {}",
                            value
                        )))
                    }
                })
            }
        }
    }
    pub struct V1Actions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> V1Actions<'a, A> {
        #[doc = "Batch gets the update history of assets that overlap a time window.\nFor RESOURCE content, this API outputs history with asset in both\nnon-delete or deleted status.\nFor IAM_POLICY content, this API outputs history when the asset and its\nattached IAM POLICY both exist. This can create gaps in the output history.\nIf a specified asset does not exist, this API returns an INVALID_ARGUMENT\nerror."]
        pub fn batch_get_assets_history(
            &self,
            parent: impl Into<String>,
        ) -> BatchGetAssetsHistoryRequestBuilder<A> {
            BatchGetAssetsHistoryRequestBuilder {
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
                asset_names: None,
                content_type: None,
                read_time_window_end_time: None,
                read_time_window_start_time: None,
            }
        }
        #[doc = "Exports assets with time and resource types to a given Cloud Storage\nlocation. The output format is newline-delimited JSON.\nThis API implements the google.longrunning.Operation API allowing you\nto keep track of the export."]
        pub fn export_assets(
            &self,
            request: crate::schemas::ExportAssetsRequest,
            parent: impl Into<String>,
        ) -> ExportAssetsRequestBuilder<A> {
            ExportAssetsRequestBuilder {
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
    }
    #[derive(Debug, Clone)]
    pub struct BatchGetAssetsHistoryRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        parent: String,
        asset_names: Option<String>,
        content_type: Option<crate::v_1::params::BatchGetAssetsHistoryContentType>,
        read_time_window_end_time: Option<String>,
        read_time_window_start_time: Option<String>,
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
    impl<'a, A: yup_oauth2::GetToken> BatchGetAssetsHistoryRequestBuilder<'a, A> {
        #[doc = "A list of the full names of the assets. For example:\n`//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.\nSee [Resource\nNames](https://cloud.google.com/apis/design/resource_names#full_resource_name)\nand [Resource Name\nFormat](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/resource-name-format)\nfor more info.\n\nThe request becomes a no-op if the asset name list is empty, and the max\nsize of the asset name list is 100 in one request."]
        pub fn asset_names(&mut self, value: impl Into<String>) -> &mut Self {
            self.asset_names = Some(value.into());
            self
        }
        #[doc = "Required. The content type."]
        pub fn content_type(
            &mut self,
            value: crate::v_1::params::BatchGetAssetsHistoryContentType,
        ) -> &mut Self {
            self.content_type = Some(value);
            self
        }
        #[doc = "End time of the time window (inclusive).\nCurrent timestamp if not specified."]
        pub fn read_time_window_end_time(&mut self, value: impl Into<String>) -> &mut Self {
            self.read_time_window_end_time = Some(value.into());
            self
        }
        #[doc = "Start time of the time window (exclusive)."]
        pub fn read_time_window_start_time(&mut self, value: impl Into<String>) -> &mut Self {
            self.read_time_window_start_time = Some(value.into());
            self
        }
        #[doc = "OAuth access token."]
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::BatchGetAssetsHistoryResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://cloudasset.googleapis.com/".to_owned();
            output.push_str("v1/");
            output.push_str(&self.parent);
            output.push_str(":batchGetAssetsHistory");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("assetNames", &self.asset_names)]);
            let req = req.query(&[("contentType", &self.content_type)]);
            let req = req.query(&[("readTimeWindow.endTime", &self.read_time_window_end_time)]);
            let req = req.query(&[(
                "readTimeWindow.startTime",
                &self.read_time_window_start_time,
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
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ExportAssetsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::ExportAssetsRequest,
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
    impl<'a, A: yup_oauth2::GetToken> ExportAssetsRequestBuilder<'a, A> {
        #[doc = "OAuth access token."]
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://cloudasset.googleapis.com/".to_owned();
            output.push_str("v1/");
            output.push_str(&self.parent);
            output.push_str(":exportAssets");
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
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

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
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
}
