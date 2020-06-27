#![doc = "# Resources and Methods\n    * [organizations](resources/organizations/struct.OrganizationsActions.html)\n      * [operations](resources/organizations/operations/struct.OperationsActions.html)\n        * [*cancel*](resources/organizations/operations/struct.CancelRequestBuilder.html), [*delete*](resources/organizations/operations/struct.DeleteRequestBuilder.html), [*get*](resources/organizations/operations/struct.GetRequestBuilder.html), [*list*](resources/organizations/operations/struct.ListRequestBuilder.html)\n"]
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Finding {
        #[doc = "The additional taxonomy group within findings from a given source.\nThis field is immutable after creation time.\nExample: \"XSS_FLASH_INJECTION\""]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "The time at which the finding was created in Security Command Center."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The time at which the event took place. For example, if the finding\nrepresents an open firewall it would capture the time the detector believes\nthe firewall became open. The accuracy is determined by the detector."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "The URI that, if available, points to a web page outside of Security\nCommand Center where additional information about the finding can be found.\nThis field is guaranteed to be either empty or a well formed URL."]
        #[serde(
            rename = "externalUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_uri: ::std::option::Option<String>,
        #[doc = "The relative resource name of this finding. See:\nhttps://cloud.google.com/apis/design/resource_names#relative_resource_name\nExample:\n\"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}\""]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The relative resource name of the source the finding belongs to. See:\nhttps://cloud.google.com/apis/design/resource_names#relative_resource_name\nThis field is immutable after creation time.\nFor example:\n\"organizations/{organization_id}/sources/{source_id}\""]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "For findings on Google Cloud resources, the full resource\nname of the Google Cloud resource this finding is for. See:\nhttps://cloud.google.com/apis/design/resource_names#full_resource_name\nWhen the finding is for a non-Google Cloud resource, the resourceName can\nbe a customer or partner defined string. This field is immutable after\ncreation time."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Output only. User specified security marks. These marks are entirely\nmanaged by the user and come from the SecurityMarks resource that belongs\nto the finding."]
        #[serde(
            rename = "securityMarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_marks: ::std::option::Option<crate::schemas::SecurityMarks>,
        #[doc = "Source specific properties. These properties are managed by the source\nthat writes the finding. The key names in the source_properties map must be\nbetween 1 and 255 characters, and must start with a letter and contain\nalphanumeric characters or underscores only."]
        #[serde(
            rename = "sourceProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The state of the finding."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::FindingState>,
    }
    impl ::google_field_selector::FieldSelector for Finding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Finding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FindingState {
        #[doc = "The finding requires attention and has not been addressed yet."]
        Active,
        #[doc = "The finding has been fixed, triaged as a non-issue or otherwise addressed\nand is no longer active."]
        Inactive,
        #[doc = "Unspecified state."]
        StateUnspecified,
    }
    impl FindingState {
        pub fn as_str(self) -> &'static str {
            match self {
                FindingState::Active => "ACTIVE",
                FindingState::Inactive => "INACTIVE",
                FindingState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FindingState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FindingState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FindingState, ()> {
            Ok(match s {
                "ACTIVE" => FindingState::Active,
                "INACTIVE" => FindingState::Inactive,
                "STATE_UNSPECIFIED" => FindingState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FindingState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FindingState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FindingState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => FindingState::Active,
                "INACTIVE" => FindingState::Inactive,
                "STATE_UNSPECIFIED" => FindingState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FindingState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FindingState {
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
    pub struct GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponse {
        #[doc = "The duration between asset discovery run start and end"]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "The state of an asset discovery run."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState {
        #[doc = "Asset discovery run completed successfully."]
        Completed,
        #[doc = "Asset discovery run state was unspecified."]
        StateUnspecified,
        #[doc = "Asset discovery run was cancelled with tasks still pending, as another\nrun for the same organization was started with a higher priority."]
        Superseded,
        #[doc = "Asset discovery run was killed and terminated."]
        Terminated,
    }
    impl GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState :: Completed => "COMPLETED" , GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState :: StateUnspecified => "STATE_UNSPECIFIED" , GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState :: Superseded => "SUPERSEDED" , GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState :: Terminated => "TERMINATED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState, ()>
        {
            Ok(match s {
                "COMPLETED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::Completed
                }
                "STATE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::StateUnspecified
                }
                "SUPERSEDED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::Superseded
                }
                "TERMINATED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::Terminated
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPLETED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::Completed
                }
                "STATE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::StateUnspecified
                }
                "SUPERSEDED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::Superseded
                }
                "TERMINATED" => {
                    GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState::Terminated
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
        for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1Beta1RunAssetDiscoveryResponseState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudSecuritycenterV1NotificationMessage {
        #[doc = "If it's a Finding based notification config, this field will be\npopulated."]
        #[serde(
            rename = "finding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finding: ::std::option::Option<crate::schemas::Finding>,
        #[doc = "Name of the notification config that generated current notification."]
        #[serde(
            rename = "notificationConfigName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notification_config_name: ::std::option::Option<String>,
        #[doc = "The Cloud resource tied to this notification's Finding."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1Resource>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1NotificationMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1NotificationMessage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudSecuritycenterV1P1Beta1Asset {
        #[doc = "The time at which the asset was created in Security Command Center."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Cloud IAM Policy information associated with the Google Cloud resource\ndescribed by the Security Command Center asset. This information is managed\nand defined by the Google Cloud resource and cannot be modified by the\nuser."]
        #[serde(
            rename = "iamPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iam_policy:
            ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1IamPolicy>,
        #[doc = "The relative resource name of this asset. See:\nhttps://cloud.google.com/apis/design/resource_names#relative_resource_name\nExample:\n\"organizations/{organization_id}/assets/{asset_id}\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Resource managed properties. These properties are managed and defined by\nthe Google Cloud resource and cannot be modified by the user."]
        #[serde(
            rename = "resourceProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Security Command Center managed properties. These properties are managed by\nSecurity Command Center and cannot be modified by the user."]
        #[serde(
            rename = "securityCenterProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_center_properties: ::std::option::Option<
            crate::schemas::GoogleCloudSecuritycenterV1P1Beta1SecurityCenterProperties,
        >,
        #[doc = "User specified security marks. These marks are entirely managed by the user\nand come from the SecurityMarks resource that belongs to the asset."]
        #[serde(
            rename = "securityMarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_marks:
            ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1SecurityMarks>,
        #[doc = "The time at which the asset was last updated, added, or deleted in Cloud\nSCC."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1P1Beta1Asset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1P1Beta1Asset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudSecuritycenterV1P1Beta1Finding {
        #[doc = "The additional taxonomy group within findings from a given source.\nThis field is immutable after creation time.\nExample: \"XSS_FLASH_INJECTION\""]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "The time at which the finding was created in Security Command Center."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The time at which the event took place. For example, if the finding\nrepresents an open firewall it would capture the time the detector believes\nthe firewall became open. The accuracy is determined by the detector."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "The URI that, if available, points to a web page outside of Security\nCommand Center where additional information about the finding can be found.\nThis field is guaranteed to be either empty or a well formed URL."]
        #[serde(
            rename = "externalUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_uri: ::std::option::Option<String>,
        #[doc = "The relative resource name of this finding. See:\nhttps://cloud.google.com/apis/design/resource_names#relative_resource_name\nExample:\n\"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}\""]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The relative resource name of the source the finding belongs to. See:\nhttps://cloud.google.com/apis/design/resource_names#relative_resource_name\nThis field is immutable after creation time.\nFor example:\n\"organizations/{organization_id}/sources/{source_id}\""]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "For findings on Google Cloud resources, the full resource\nname of the Google Cloud resource this finding is for. See:\nhttps://cloud.google.com/apis/design/resource_names#full_resource_name\nWhen the finding is for a non-Google Cloud resource, the resourceName can\nbe a customer or partner defined string. This field is immutable after\ncreation time."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Output only. User specified security marks. These marks are entirely\nmanaged by the user and come from the SecurityMarks resource that belongs\nto the finding."]
        #[serde(
            rename = "securityMarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_marks:
            ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1SecurityMarks>,
        #[doc = "Source specific properties. These properties are managed by the source\nthat writes the finding. The key names in the source_properties map must be\nbetween 1 and 255 characters, and must start with a letter and contain\nalphanumeric characters or underscores only."]
        #[serde(
            rename = "sourceProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The state of the finding."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1FindingState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1P1Beta1Finding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1P1Beta1Finding {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudSecuritycenterV1P1Beta1FindingState {
        #[doc = "The finding requires attention and has not been addressed yet."]
        Active,
        #[doc = "The finding has been fixed, triaged as a non-issue or otherwise addressed\nand is no longer active."]
        Inactive,
        #[doc = "Unspecified state."]
        StateUnspecified,
    }
    impl GoogleCloudSecuritycenterV1P1Beta1FindingState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudSecuritycenterV1P1Beta1FindingState::Active => "ACTIVE",
                GoogleCloudSecuritycenterV1P1Beta1FindingState::Inactive => "INACTIVE",
                GoogleCloudSecuritycenterV1P1Beta1FindingState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudSecuritycenterV1P1Beta1FindingState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudSecuritycenterV1P1Beta1FindingState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudSecuritycenterV1P1Beta1FindingState, ()> {
            Ok(match s {
                "ACTIVE" => GoogleCloudSecuritycenterV1P1Beta1FindingState::Active,
                "INACTIVE" => GoogleCloudSecuritycenterV1P1Beta1FindingState::Inactive,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1P1Beta1FindingState::StateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudSecuritycenterV1P1Beta1FindingState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudSecuritycenterV1P1Beta1FindingState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudSecuritycenterV1P1Beta1FindingState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => GoogleCloudSecuritycenterV1P1Beta1FindingState::Active,
                "INACTIVE" => GoogleCloudSecuritycenterV1P1Beta1FindingState::Inactive,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1P1Beta1FindingState::StateUnspecified
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
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1P1Beta1FindingState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1P1Beta1FindingState {
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
    pub struct GoogleCloudSecuritycenterV1P1Beta1IamPolicy {
        #[doc = "The JSON representation of the Policy associated with the asset.\nSee https://cloud.google.com/iam/docs/reference/rest/v1/Policy for\nformat details."]
        #[serde(
            rename = "policyBlob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_blob: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1P1Beta1IamPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1P1Beta1IamPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudSecuritycenterV1P1Beta1NotificationMessage {
        #[doc = "If it's a Finding based notification config, this field will be\npopulated."]
        #[serde(
            rename = "finding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finding:
            ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1Finding>,
        #[doc = "Name of the notification config that generated current notification."]
        #[serde(
            rename = "notificationConfigName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notification_config_name: ::std::option::Option<String>,
        #[doc = "The Cloud resource tied to the notification."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource:
            ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1Resource>,
        #[doc = "If it's an asset based notification config, this field will be\npopulated."]
        #[serde(
            rename = "temporalAsset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temporal_asset:
            ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1TemporalAsset>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudSecuritycenterV1P1Beta1NotificationMessage
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1P1Beta1NotificationMessage
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
    pub struct GoogleCloudSecuritycenterV1P1Beta1Resource {
        #[doc = "The full resource name of the resource. See:\nhttps://cloud.google.com/apis/design/resource_names#full_resource_name"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The full resource name of resource's parent."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The human readable name of resource's parent."]
        #[serde(
            rename = "parentDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_display_name: ::std::option::Option<String>,
        #[doc = "The full resource name of project that the resource belongs to."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "The human readable name of project that the resource belongs to."]
        #[serde(
            rename = "projectDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1P1Beta1Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1P1Beta1Resource {
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
    pub struct GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponse {
        #[doc = "The duration between asset discovery run start and end"]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "The state of an asset discovery run."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState {
        #[doc = "Asset discovery run completed successfully."]
        Completed,
        #[doc = "Asset discovery run state was unspecified."]
        StateUnspecified,
        #[doc = "Asset discovery run was cancelled with tasks still pending, as another\nrun for the same organization was started with a higher priority."]
        Superseded,
        #[doc = "Asset discovery run was killed and terminated."]
        Terminated,
    }
    impl GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Completed => "COMPLETED" , GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: StateUnspecified => "STATE_UNSPECIFIED" , GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Superseded => "SUPERSEDED" , GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Terminated => "TERMINATED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState,
            (),
        > {
            Ok ( match s { "COMPLETED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Completed , "STATE_UNSPECIFIED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: StateUnspecified , "SUPERSEDED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Superseded , "TERMINATED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Terminated , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "COMPLETED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Completed , "STATE_UNSPECIFIED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: StateUnspecified , "SUPERSEDED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Superseded , "TERMINATED" => GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState :: Terminated , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1P1Beta1RunAssetDiscoveryResponseState
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
    pub struct GoogleCloudSecuritycenterV1P1Beta1SecurityCenterProperties {
        #[doc = "The user defined display name for this resource."]
        #[serde(
            rename = "resourceDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_display_name: ::std::option::Option<String>,
        #[doc = "The full resource name of the Google Cloud resource this asset\nrepresents. This field is immutable after create time. See:\nhttps://cloud.google.com/apis/design/resource_names#full_resource_name"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Owners of the Google Cloud resource."]
        #[serde(
            rename = "resourceOwners",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_owners: ::std::option::Option<Vec<String>>,
        #[doc = "The full resource name of the immediate parent of the resource. See:\nhttps://cloud.google.com/apis/design/resource_names#full_resource_name"]
        #[serde(
            rename = "resourceParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_parent: ::std::option::Option<String>,
        #[doc = "The user defined display name for the parent of this resource."]
        #[serde(
            rename = "resourceParentDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_parent_display_name: ::std::option::Option<String>,
        #[doc = "The full resource name of the project the resource belongs to. See:\nhttps://cloud.google.com/apis/design/resource_names#full_resource_name"]
        #[serde(
            rename = "resourceProject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_project: ::std::option::Option<String>,
        #[doc = "The user defined display name for the project of this resource."]
        #[serde(
            rename = "resourceProjectDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_project_display_name: ::std::option::Option<String>,
        #[doc = "The type of the Google Cloud resource. Examples include: APPLICATION,\nPROJECT, and ORGANIZATION. This is a case insensitive field defined by\nSecurity Command Center and/or the producer of the resource and is\nimmutable after create time."]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudSecuritycenterV1P1Beta1SecurityCenterProperties
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1P1Beta1SecurityCenterProperties
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
    pub struct GoogleCloudSecuritycenterV1P1Beta1SecurityMarks {
        #[doc = "Mutable user specified security marks belonging to the parent resource.\nConstraints are as follows:\n\n* Keys and values are treated as case insensitive\n* Keys must be between 1 - 256 characters (inclusive)\n* Keys must be letters, numbers, underscores, or dashes\n* Values have leading and trailing whitespace trimmed, remaining\n  characters must be between 1 - 4096 characters (inclusive)"]
        #[serde(
            rename = "marks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub marks: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The relative resource name of the SecurityMarks. See:\nhttps://cloud.google.com/apis/design/resource_names#relative_resource_name\nExamples:\n\"organizations/{organization_id}/assets/{asset_id}/securityMarks\"\n\"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1P1Beta1SecurityMarks {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1P1Beta1SecurityMarks {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudSecuritycenterV1P1Beta1TemporalAsset {
        #[doc = "Asset data that includes attributes, properties and marks about the asset."]
        #[serde(
            rename = "asset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset: ::std::option::Option<crate::schemas::GoogleCloudSecuritycenterV1P1Beta1Asset>,
        #[doc = "Represents if the asset was created/updated/deleted."]
        #[serde(
            rename = "changeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub change_type: ::std::option::Option<
            crate::schemas::GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1P1Beta1TemporalAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1P1Beta1TemporalAsset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType {
        #[doc = "Unspecified or default."]
        ChangeTypeUnspecified,
        #[doc = "Newly created Asset"]
        Created,
        #[doc = "Asset was deleted."]
        Deleted,
        #[doc = "Asset was updated."]
        Updated,
    }
    impl GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType :: ChangeTypeUnspecified => "CHANGE_TYPE_UNSPECIFIED" , GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType :: Created => "CREATED" , GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType :: Deleted => "DELETED" , GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType :: Updated => "UPDATED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType, ()>
        {
            Ok(match s {
                "CHANGE_TYPE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::ChangeTypeUnspecified
                }
                "CREATED" => GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::Created,
                "DELETED" => GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::Deleted,
                "UPDATED" => GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::Updated,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHANGE_TYPE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::ChangeTypeUnspecified
                }
                "CREATED" => GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::Created,
                "DELETED" => GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::Deleted,
                "UPDATED" => GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType::Updated,
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
        for GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1P1Beta1TemporalAssetChangeType
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
    pub struct GoogleCloudSecuritycenterV1Resource {
        #[doc = "The full resource name of the resource. See:\nhttps://cloud.google.com/apis/design/resource_names#full_resource_name"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The full resource name of resource's parent."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The human readable name of resource's parent."]
        #[serde(
            rename = "parentDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_display_name: ::std::option::Option<String>,
        #[doc = "The full resource name of project that the resource belongs to."]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
        #[doc = "The human readable name of project that the resource belongs to."]
        #[serde(
            rename = "projectDisplayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudSecuritycenterV1Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1Resource {
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
    pub struct GoogleCloudSecuritycenterV1RunAssetDiscoveryResponse {
        #[doc = "The duration between asset discovery run start and end"]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "The state of an asset discovery run."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState {
        #[doc = "Asset discovery run completed successfully."]
        Completed,
        #[doc = "Asset discovery run state was unspecified."]
        StateUnspecified,
        #[doc = "Asset discovery run was cancelled with tasks still pending, as another\nrun for the same organization was started with a higher priority."]
        Superseded,
        #[doc = "Asset discovery run was killed and terminated."]
        Terminated,
    }
    impl GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Completed => "COMPLETED",
                GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Superseded => {
                    "SUPERSEDED"
                }
                GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Terminated => {
                    "TERMINATED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState, ()>
        {
            Ok(match s {
                "COMPLETED" => GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Completed,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::StateUnspecified
                }
                "SUPERSEDED" => {
                    GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Superseded
                }
                "TERMINATED" => {
                    GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Terminated
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPLETED" => GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Completed,
                "STATE_UNSPECIFIED" => {
                    GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::StateUnspecified
                }
                "SUPERSEDED" => {
                    GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Superseded
                }
                "TERMINATED" => {
                    GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState::Terminated
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
        for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudSecuritycenterV1RunAssetDiscoveryResponseState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
    }
    impl ::google_field_selector::FieldSelector for ListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOperationsResponse {
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
    pub struct SecurityMarks {
        #[doc = "Mutable user specified security marks belonging to the parent resource.\nConstraints are as follows:\n\n* Keys and values are treated as case insensitive\n* Keys must be between 1 - 256 characters (inclusive)\n* Keys must be letters, numbers, underscores, or dashes\n* Values have leading and trailing whitespace trimmed, remaining\n  characters must be between 1 - 4096 characters (inclusive)"]
        #[serde(
            rename = "marks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub marks: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The relative resource name of the SecurityMarks. See:\nhttps://cloud.google.com/apis/design/resource_names#relative_resource_name\nExamples:\n\"organizations/{organization_id}/assets/{asset_id}/securityMarks\"\n\"organizations/{organization_id}/sources/{source_id}/findings/{finding_id}/securityMarks\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SecurityMarks {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SecurityMarks {
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
    #[doc = "Actions that can be performed on the organizations resource"]
    pub fn organizations(&self) -> crate::resources::organizations::OrganizationsActions {
        crate::resources::organizations::OrganizationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod organizations {
        pub mod params {}
        pub struct OrganizationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> OrganizationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(
                &self,
            ) -> crate::resources::organizations::operations::OperationsActions {
                crate::resources::organizations::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OperationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn't support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`."]
                pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder {
                    CancelRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Deletes a long-running operation. This method indicates that the client is\nno longer interested in the operation result. It does not cancel the\noperation. If the server doesn't support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                    DeleteRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Lists operations that match the specified filter in the request. If the\nserver doesn't support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
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
                        name: name.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [OperationsActions::cancel()](struct.OperationsActions.html#method.cancel)"]
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            impl<'a> CancelRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    let mut output = "https://securitycenter.googleapis.com/".to_owned();
                    output.push_str("v1p1alpha1/");
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
            #[doc = "Created via [OperationsActions::delete()](struct.OperationsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            impl<'a> DeleteRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    let mut output = "https://securitycenter.googleapis.com/".to_owned();
                    output.push_str("v1p1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            impl<'a> GetRequestBuilder<'a> {
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
                    let mut output = "https://securitycenter.googleapis.com/".to_owned();
                    output.push_str("v1p1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The standard list filter."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
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
                pub fn iter_operations<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_operations_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_operations_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
                    self.iter_operations_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_operations_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Operation> {
                    self.iter_operations_with_fields(Some("*"))
                }
                pub fn iter_operations_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "operations").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "operations")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListOperationsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
                ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
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
                    let mut output = "https://securitycenter.googleapis.com/".to_owned();
                    output.push_str("v1p1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
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
