#![doc = "# Resources and Methods\n    * [changes](resources/changes/struct.ChangesActions.html)\n      * [*create*](resources/changes/struct.CreateRequestBuilder.html), [*get*](resources/changes/struct.GetRequestBuilder.html), [*list*](resources/changes/struct.ListRequestBuilder.html)\n    * [dns_keys](resources/dns_keys/struct.DnsKeysActions.html)\n      * [*get*](resources/dns_keys/struct.GetRequestBuilder.html), [*list*](resources/dns_keys/struct.ListRequestBuilder.html)\n    * [managed_zone_operations](resources/managed_zone_operations/struct.ManagedZoneOperationsActions.html)\n      * [*get*](resources/managed_zone_operations/struct.GetRequestBuilder.html), [*list*](resources/managed_zone_operations/struct.ListRequestBuilder.html)\n    * [managed_zones](resources/managed_zones/struct.ManagedZonesActions.html)\n      * [*create*](resources/managed_zones/struct.CreateRequestBuilder.html), [*delete*](resources/managed_zones/struct.DeleteRequestBuilder.html), [*get*](resources/managed_zones/struct.GetRequestBuilder.html), [*list*](resources/managed_zones/struct.ListRequestBuilder.html), [*patch*](resources/managed_zones/struct.PatchRequestBuilder.html), [*update*](resources/managed_zones/struct.UpdateRequestBuilder.html)\n    * [policies](resources/policies/struct.PoliciesActions.html)\n      * [*create*](resources/policies/struct.CreateRequestBuilder.html), [*delete*](resources/policies/struct.DeleteRequestBuilder.html), [*get*](resources/policies/struct.GetRequestBuilder.html), [*list*](resources/policies/struct.ListRequestBuilder.html), [*patch*](resources/policies/struct.PatchRequestBuilder.html), [*update*](resources/policies/struct.UpdateRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*get*](resources/projects/struct.GetRequestBuilder.html)\n    * [resource_record_sets](resources/resource_record_sets/struct.ResourceRecordSetsActions.html)\n      * [*list*](resources/resource_record_sets/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform.read-only`"]
    pub const CLOUD_PLATFORM_READ_ONLY: &str =
        "https://www.googleapis.com/auth/cloud-platform.read-only";
    #[doc = "View your DNS records hosted by Google Cloud DNS\n\n`https://www.googleapis.com/auth/ndev.clouddns.readonly`"]
    pub const NDEV_CLOUDDNS_READONLY: &str =
        "https://www.googleapis.com/auth/ndev.clouddns.readonly";
    #[doc = "View and manage your DNS records hosted by Google Cloud DNS\n\n`https://www.googleapis.com/auth/ndev.clouddns.readwrite`"]
    pub const NDEV_CLOUDDNS_READWRITE: &str =
        "https://www.googleapis.com/auth/ndev.clouddns.readwrite";
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
    pub struct Change {
        #[serde(
            rename = "additions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additions: ::std::option::Option<Vec<crate::schemas::ResourceRecordSet>>,
        #[serde(
            rename = "deletions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deletions: ::std::option::Option<Vec<crate::schemas::ResourceRecordSet>>,
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[serde(
            rename = "isServing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_serving: ::std::option::Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#change\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::ChangeStatus>,
    }
    impl ::google_field_selector::FieldSelector for Change {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Change {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ChangeStatus {
        Done,
        Pending,
    }
    impl ChangeStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                ChangeStatus::Done => "done",
                ChangeStatus::Pending => "pending",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ChangeStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ChangeStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ChangeStatus, ()> {
            Ok(match s {
                "done" => ChangeStatus::Done,
                "pending" => ChangeStatus::Pending,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ChangeStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ChangeStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ChangeStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "done" => ChangeStatus::Done,
                "pending" => ChangeStatus::Pending,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ChangeStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChangeStatus {
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
    pub struct ChangesListResponse {
        #[serde(
            rename = "changes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub changes: ::std::option::Option<Vec<crate::schemas::Change>>,
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[doc = "Type of resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ChangesListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChangesListResponse {
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
    pub struct DnsKey {
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm: ::std::option::Option<crate::schemas::DnsKeyAlgorithm>,
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<String>,
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[serde(
            rename = "digests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digests: ::std::option::Option<Vec<crate::schemas::DnsKeyDigest>>,
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[serde(
            rename = "isActive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_active: ::std::option::Option<bool>,
        #[serde(
            rename = "keyLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_length: ::std::option::Option<u32>,
        #[serde(
            rename = "keyTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_tag: ::std::option::Option<i32>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#dnsKey\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "publicKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_key: ::std::option::Option<String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DnsKeyType>,
    }
    impl ::google_field_selector::FieldSelector for DnsKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKey {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DnsKeyAlgorithm {
        Ecdsap256Sha256,
        Ecdsap384Sha384,
        Rsasha1,
        Rsasha256,
        Rsasha512,
    }
    impl DnsKeyAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                DnsKeyAlgorithm::Ecdsap256Sha256 => "ecdsap256sha256",
                DnsKeyAlgorithm::Ecdsap384Sha384 => "ecdsap384sha384",
                DnsKeyAlgorithm::Rsasha1 => "rsasha1",
                DnsKeyAlgorithm::Rsasha256 => "rsasha256",
                DnsKeyAlgorithm::Rsasha512 => "rsasha512",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DnsKeyAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DnsKeyAlgorithm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DnsKeyAlgorithm, ()> {
            Ok(match s {
                "ecdsap256sha256" => DnsKeyAlgorithm::Ecdsap256Sha256,
                "ecdsap384sha384" => DnsKeyAlgorithm::Ecdsap384Sha384,
                "rsasha1" => DnsKeyAlgorithm::Rsasha1,
                "rsasha256" => DnsKeyAlgorithm::Rsasha256,
                "rsasha512" => DnsKeyAlgorithm::Rsasha512,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DnsKeyAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DnsKeyAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DnsKeyAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ecdsap256sha256" => DnsKeyAlgorithm::Ecdsap256Sha256,
                "ecdsap384sha384" => DnsKeyAlgorithm::Ecdsap384Sha384,
                "rsasha1" => DnsKeyAlgorithm::Rsasha1,
                "rsasha256" => DnsKeyAlgorithm::Rsasha256,
                "rsasha512" => DnsKeyAlgorithm::Rsasha512,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DnsKeyAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeyAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DnsKeyType {
        KeySigning,
        ZoneSigning,
    }
    impl DnsKeyType {
        pub fn as_str(self) -> &'static str {
            match self {
                DnsKeyType::KeySigning => "keySigning",
                DnsKeyType::ZoneSigning => "zoneSigning",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DnsKeyType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DnsKeyType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DnsKeyType, ()> {
            Ok(match s {
                "keySigning" => DnsKeyType::KeySigning,
                "zoneSigning" => DnsKeyType::ZoneSigning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DnsKeyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DnsKeyType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DnsKeyType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "keySigning" => DnsKeyType::KeySigning,
                "zoneSigning" => DnsKeyType::ZoneSigning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DnsKeyType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeyType {
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
    pub struct DnsKeyDigest {
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DnsKeyDigestType>,
    }
    impl ::google_field_selector::FieldSelector for DnsKeyDigest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeyDigest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DnsKeyDigestType {
        Sha1,
        Sha256,
        Sha384,
    }
    impl DnsKeyDigestType {
        pub fn as_str(self) -> &'static str {
            match self {
                DnsKeyDigestType::Sha1 => "sha1",
                DnsKeyDigestType::Sha256 => "sha256",
                DnsKeyDigestType::Sha384 => "sha384",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DnsKeyDigestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DnsKeyDigestType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DnsKeyDigestType, ()> {
            Ok(match s {
                "sha1" => DnsKeyDigestType::Sha1,
                "sha256" => DnsKeyDigestType::Sha256,
                "sha384" => DnsKeyDigestType::Sha384,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DnsKeyDigestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DnsKeyDigestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DnsKeyDigestType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "sha1" => DnsKeyDigestType::Sha1,
                "sha256" => DnsKeyDigestType::Sha256,
                "sha384" => DnsKeyDigestType::Sha384,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DnsKeyDigestType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeyDigestType {
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
    pub struct DnsKeySpec {
        #[serde(
            rename = "algorithm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub algorithm: ::std::option::Option<crate::schemas::DnsKeySpecAlgorithm>,
        #[serde(
            rename = "keyLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_length: ::std::option::Option<u32>,
        #[serde(
            rename = "keyType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_type: ::std::option::Option<crate::schemas::DnsKeySpecKeyType>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#dnsKeySpec\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DnsKeySpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeySpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DnsKeySpecAlgorithm {
        Ecdsap256Sha256,
        Ecdsap384Sha384,
        Rsasha1,
        Rsasha256,
        Rsasha512,
    }
    impl DnsKeySpecAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                DnsKeySpecAlgorithm::Ecdsap256Sha256 => "ecdsap256sha256",
                DnsKeySpecAlgorithm::Ecdsap384Sha384 => "ecdsap384sha384",
                DnsKeySpecAlgorithm::Rsasha1 => "rsasha1",
                DnsKeySpecAlgorithm::Rsasha256 => "rsasha256",
                DnsKeySpecAlgorithm::Rsasha512 => "rsasha512",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DnsKeySpecAlgorithm {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DnsKeySpecAlgorithm {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DnsKeySpecAlgorithm, ()> {
            Ok(match s {
                "ecdsap256sha256" => DnsKeySpecAlgorithm::Ecdsap256Sha256,
                "ecdsap384sha384" => DnsKeySpecAlgorithm::Ecdsap384Sha384,
                "rsasha1" => DnsKeySpecAlgorithm::Rsasha1,
                "rsasha256" => DnsKeySpecAlgorithm::Rsasha256,
                "rsasha512" => DnsKeySpecAlgorithm::Rsasha512,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DnsKeySpecAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DnsKeySpecAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DnsKeySpecAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ecdsap256sha256" => DnsKeySpecAlgorithm::Ecdsap256Sha256,
                "ecdsap384sha384" => DnsKeySpecAlgorithm::Ecdsap384Sha384,
                "rsasha1" => DnsKeySpecAlgorithm::Rsasha1,
                "rsasha256" => DnsKeySpecAlgorithm::Rsasha256,
                "rsasha512" => DnsKeySpecAlgorithm::Rsasha512,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DnsKeySpecAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeySpecAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DnsKeySpecKeyType {
        KeySigning,
        ZoneSigning,
    }
    impl DnsKeySpecKeyType {
        pub fn as_str(self) -> &'static str {
            match self {
                DnsKeySpecKeyType::KeySigning => "keySigning",
                DnsKeySpecKeyType::ZoneSigning => "zoneSigning",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DnsKeySpecKeyType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DnsKeySpecKeyType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DnsKeySpecKeyType, ()> {
            Ok(match s {
                "keySigning" => DnsKeySpecKeyType::KeySigning,
                "zoneSigning" => DnsKeySpecKeyType::ZoneSigning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DnsKeySpecKeyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DnsKeySpecKeyType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DnsKeySpecKeyType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "keySigning" => DnsKeySpecKeyType::KeySigning,
                "zoneSigning" => DnsKeySpecKeyType::ZoneSigning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DnsKeySpecKeyType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeySpecKeyType {
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
    pub struct DnsKeysListResponse {
        #[serde(
            rename = "dnsKeys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dns_keys: ::std::option::Option<Vec<crate::schemas::DnsKey>>,
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[doc = "Type of resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DnsKeysListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DnsKeysListResponse {
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
    pub struct ManagedZone {
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<String>,
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[serde(
            rename = "dnsName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dns_name: ::std::option::Option<String>,
        #[serde(
            rename = "dnssecConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dnssec_config: ::std::option::Option<crate::schemas::ManagedZoneDnsSecConfig>,
        #[serde(
            rename = "forwardingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub forwarding_config: ::std::option::Option<crate::schemas::ManagedZoneForwardingConfig>,
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<u64>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZone\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "nameServerSet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name_server_set: ::std::option::Option<String>,
        #[serde(
            rename = "nameServers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name_servers: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "peeringConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub peering_config: ::std::option::Option<crate::schemas::ManagedZonePeeringConfig>,
        #[serde(
            rename = "privateVisibilityConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_visibility_config:
            ::std::option::Option<crate::schemas::ManagedZonePrivateVisibilityConfig>,
        #[serde(
            rename = "reverseLookupConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reverse_lookup_config:
            ::std::option::Option<crate::schemas::ManagedZoneReverseLookupConfig>,
        #[serde(
            rename = "visibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visibility: ::std::option::Option<crate::schemas::ManagedZoneVisibility>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZone {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManagedZoneVisibility {
        Private,
        Public,
    }
    impl ManagedZoneVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedZoneVisibility::Private => "private",
                ManagedZoneVisibility::Public => "public",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManagedZoneVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManagedZoneVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ManagedZoneVisibility, ()> {
            Ok(match s {
                "private" => ManagedZoneVisibility::Private,
                "public" => ManagedZoneVisibility::Public,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManagedZoneVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedZoneVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedZoneVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "private" => ManagedZoneVisibility::Private,
                "public" => ManagedZoneVisibility::Public,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneVisibility {
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
    pub struct ManagedZoneDnsSecConfig {
        #[serde(
            rename = "defaultKeySpecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_key_specs: ::std::option::Option<Vec<crate::schemas::DnsKeySpec>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZoneDnsSecConfig\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "nonExistence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_existence:
            ::std::option::Option<crate::schemas::ManagedZoneDnsSecConfigNonExistence>,
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ManagedZoneDnsSecConfigState>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneDnsSecConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneDnsSecConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManagedZoneDnsSecConfigNonExistence {
        Nsec,
        Nsec3,
    }
    impl ManagedZoneDnsSecConfigNonExistence {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedZoneDnsSecConfigNonExistence::Nsec => "nsec",
                ManagedZoneDnsSecConfigNonExistence::Nsec3 => "nsec3",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManagedZoneDnsSecConfigNonExistence {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManagedZoneDnsSecConfigNonExistence {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ManagedZoneDnsSecConfigNonExistence, ()> {
            Ok(match s {
                "nsec" => ManagedZoneDnsSecConfigNonExistence::Nsec,
                "nsec3" => ManagedZoneDnsSecConfigNonExistence::Nsec3,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManagedZoneDnsSecConfigNonExistence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedZoneDnsSecConfigNonExistence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedZoneDnsSecConfigNonExistence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "nsec" => ManagedZoneDnsSecConfigNonExistence::Nsec,
                "nsec3" => ManagedZoneDnsSecConfigNonExistence::Nsec3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneDnsSecConfigNonExistence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneDnsSecConfigNonExistence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManagedZoneDnsSecConfigState {
        Off,
        On,
        Transfer,
    }
    impl ManagedZoneDnsSecConfigState {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedZoneDnsSecConfigState::Off => "off",
                ManagedZoneDnsSecConfigState::On => "on",
                ManagedZoneDnsSecConfigState::Transfer => "transfer",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManagedZoneDnsSecConfigState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManagedZoneDnsSecConfigState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ManagedZoneDnsSecConfigState, ()> {
            Ok(match s {
                "off" => ManagedZoneDnsSecConfigState::Off,
                "on" => ManagedZoneDnsSecConfigState::On,
                "transfer" => ManagedZoneDnsSecConfigState::Transfer,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManagedZoneDnsSecConfigState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedZoneDnsSecConfigState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedZoneDnsSecConfigState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "off" => ManagedZoneDnsSecConfigState::Off,
                "on" => ManagedZoneDnsSecConfigState::On,
                "transfer" => ManagedZoneDnsSecConfigState::Transfer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneDnsSecConfigState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneDnsSecConfigState {
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
    pub struct ManagedZoneForwardingConfig {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZoneForwardingConfig\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "targetNameServers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_name_servers:
            ::std::option::Option<Vec<crate::schemas::ManagedZoneForwardingConfigNameServerTarget>>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneForwardingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneForwardingConfig {
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
    pub struct ManagedZoneForwardingConfigNameServerTarget {
        #[serde(
            rename = "forwardingPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub forwarding_path: ::std::option::Option<
            crate::schemas::ManagedZoneForwardingConfigNameServerTargetForwardingPath,
        >,
        #[serde(
            rename = "ipv4Address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ipv_4_address: ::std::option::Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZoneForwardingConfigNameServerTarget\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneForwardingConfigNameServerTarget {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneForwardingConfigNameServerTarget {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManagedZoneForwardingConfigNameServerTargetForwardingPath {
        Default,
        Private,
    }
    impl ManagedZoneForwardingConfigNameServerTargetForwardingPath {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedZoneForwardingConfigNameServerTargetForwardingPath::Default => "default",
                ManagedZoneForwardingConfigNameServerTargetForwardingPath::Private => "private",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManagedZoneForwardingConfigNameServerTargetForwardingPath {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManagedZoneForwardingConfigNameServerTargetForwardingPath {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ManagedZoneForwardingConfigNameServerTargetForwardingPath, ()>
        {
            Ok(match s {
                "default" => ManagedZoneForwardingConfigNameServerTargetForwardingPath::Default,
                "private" => ManagedZoneForwardingConfigNameServerTargetForwardingPath::Private,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManagedZoneForwardingConfigNameServerTargetForwardingPath {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedZoneForwardingConfigNameServerTargetForwardingPath {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedZoneForwardingConfigNameServerTargetForwardingPath {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "default" => ManagedZoneForwardingConfigNameServerTargetForwardingPath::Default,
                "private" => ManagedZoneForwardingConfigNameServerTargetForwardingPath::Private,
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
        for ManagedZoneForwardingConfigNameServerTargetForwardingPath
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ManagedZoneForwardingConfigNameServerTargetForwardingPath
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
    pub struct ManagedZoneOperationsListResponse {
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[doc = "Type of resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneOperationsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneOperationsListResponse {
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
    pub struct ManagedZonePeeringConfig {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZonePeeringConfig\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "targetNetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_network:
            ::std::option::Option<crate::schemas::ManagedZonePeeringConfigTargetNetwork>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZonePeeringConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZonePeeringConfig {
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
    pub struct ManagedZonePeeringConfigTargetNetwork {
        #[serde(
            rename = "deactivateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deactivate_time: ::std::option::Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZonePeeringConfigTargetNetwork\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "networkUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZonePeeringConfigTargetNetwork {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZonePeeringConfigTargetNetwork {
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
    pub struct ManagedZonePrivateVisibilityConfig {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZonePrivateVisibilityConfig\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "networks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub networks:
            ::std::option::Option<Vec<crate::schemas::ManagedZonePrivateVisibilityConfigNetwork>>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZonePrivateVisibilityConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZonePrivateVisibilityConfig {
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
    pub struct ManagedZonePrivateVisibilityConfigNetwork {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZonePrivateVisibilityConfigNetwork\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "networkUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZonePrivateVisibilityConfigNetwork {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZonePrivateVisibilityConfigNetwork {
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
    pub struct ManagedZoneReverseLookupConfig {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#managedZoneReverseLookupConfig\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZoneReverseLookupConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZoneReverseLookupConfig {
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
    pub struct ManagedZonesListResponse {
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[doc = "Type of resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "managedZones",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_zones: ::std::option::Option<Vec<crate::schemas::ManagedZone>>,
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ManagedZonesListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedZonesListResponse {
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
    pub struct Operation {
        #[serde(
            rename = "dnsKeyContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dns_key_context: ::std::option::Option<crate::schemas::OperationDnsKeyContext>,
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#operation\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::OperationStatus>,
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<String>,
        #[serde(
            rename = "zoneContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zone_context: ::std::option::Option<crate::schemas::OperationManagedZoneContext>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OperationStatus {
        Done,
        Pending,
    }
    impl OperationStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                OperationStatus::Done => "done",
                OperationStatus::Pending => "pending",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OperationStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OperationStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OperationStatus, ()> {
            Ok(match s {
                "done" => OperationStatus::Done,
                "pending" => OperationStatus::Pending,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OperationStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OperationStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OperationStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "done" => OperationStatus::Done,
                "pending" => OperationStatus::Pending,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OperationStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationStatus {
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
    pub struct OperationDnsKeyContext {
        #[serde(
            rename = "newValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_value: ::std::option::Option<crate::schemas::DnsKey>,
        #[serde(
            rename = "oldValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_value: ::std::option::Option<crate::schemas::DnsKey>,
    }
    impl ::google_field_selector::FieldSelector for OperationDnsKeyContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationDnsKeyContext {
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
    pub struct OperationManagedZoneContext {
        #[serde(
            rename = "newValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_value: ::std::option::Option<crate::schemas::ManagedZone>,
        #[serde(
            rename = "oldValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_value: ::std::option::Option<crate::schemas::ManagedZone>,
    }
    impl ::google_field_selector::FieldSelector for OperationManagedZoneContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationManagedZoneContext {
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
    pub struct PoliciesListResponse {
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[doc = "Type of resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "policies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policies: ::std::option::Option<Vec<crate::schemas::Policy>>,
    }
    impl ::google_field_selector::FieldSelector for PoliciesListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PoliciesListResponse {
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
    pub struct PoliciesPatchResponse {
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
    }
    impl ::google_field_selector::FieldSelector for PoliciesPatchResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PoliciesPatchResponse {
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
    pub struct PoliciesUpdateResponse {
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
    }
    impl ::google_field_selector::FieldSelector for PoliciesUpdateResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PoliciesUpdateResponse {
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
        #[serde(
            rename = "alternativeNameServerConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alternative_name_server_config:
            ::std::option::Option<crate::schemas::PolicyAlternativeNameServerConfig>,
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[serde(
            rename = "enableInboundForwarding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_inbound_forwarding: ::std::option::Option<bool>,
        #[serde(
            rename = "enableLogging",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_logging: ::std::option::Option<bool>,
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<u64>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#policy\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "networks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub networks: ::std::option::Option<Vec<crate::schemas::PolicyNetwork>>,
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
    pub struct PolicyAlternativeNameServerConfig {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#policyAlternativeNameServerConfig\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "targetNameServers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_name_servers: ::std::option::Option<
            Vec<crate::schemas::PolicyAlternativeNameServerConfigTargetNameServer>,
        >,
    }
    impl ::google_field_selector::FieldSelector for PolicyAlternativeNameServerConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyAlternativeNameServerConfig {
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
    pub struct PolicyAlternativeNameServerConfigTargetNameServer {
        #[serde(
            rename = "forwardingPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub forwarding_path: ::std::option::Option<
            crate::schemas::PolicyAlternativeNameServerConfigTargetNameServerForwardingPath,
        >,
        #[serde(
            rename = "ipv4Address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ipv_4_address: ::std::option::Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#policyAlternativeNameServerConfigTargetNameServer\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PolicyAlternativeNameServerConfigTargetNameServer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyAlternativeNameServerConfigTargetNameServer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyAlternativeNameServerConfigTargetNameServerForwardingPath {
        Default,
        Private,
    }
    impl PolicyAlternativeNameServerConfigTargetNameServerForwardingPath {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyAlternativeNameServerConfigTargetNameServerForwardingPath::Default => {
                    "default"
                }
                PolicyAlternativeNameServerConfigTargetNameServerForwardingPath::Private => {
                    "private"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str>
        for PolicyAlternativeNameServerConfigTargetNameServerForwardingPath
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyAlternativeNameServerConfigTargetNameServerForwardingPath {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            PolicyAlternativeNameServerConfigTargetNameServerForwardingPath,
            (),
        > {
            Ok(match s {
                "default" => {
                    PolicyAlternativeNameServerConfigTargetNameServerForwardingPath::Default
                }
                "private" => {
                    PolicyAlternativeNameServerConfigTargetNameServerForwardingPath::Private
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyAlternativeNameServerConfigTargetNameServerForwardingPath {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyAlternativeNameServerConfigTargetNameServerForwardingPath {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for PolicyAlternativeNameServerConfigTargetNameServerForwardingPath
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "default" => {
                    PolicyAlternativeNameServerConfigTargetNameServerForwardingPath::Default
                }
                "private" => {
                    PolicyAlternativeNameServerConfigTargetNameServerForwardingPath::Private
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
        for PolicyAlternativeNameServerConfigTargetNameServerForwardingPath
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for PolicyAlternativeNameServerConfigTargetNameServerForwardingPath
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
    pub struct PolicyNetwork {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#policyNetwork\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "networkUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PolicyNetwork {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyNetwork {
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
    pub struct Project {
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#project\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub number: ::std::option::Option<u64>,
        #[serde(
            rename = "quota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota: ::std::option::Option<crate::schemas::Quota>,
    }
    impl ::google_field_selector::FieldSelector for Project {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Project {
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
    pub struct Quota {
        #[serde(
            rename = "dnsKeysPerManagedZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dns_keys_per_managed_zone: ::std::option::Option<i32>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#quota\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "managedZones",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_zones: ::std::option::Option<i32>,
        #[serde(
            rename = "managedZonesPerNetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_zones_per_network: ::std::option::Option<i32>,
        #[serde(
            rename = "networksPerManagedZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub networks_per_managed_zone: ::std::option::Option<i32>,
        #[serde(
            rename = "networksPerPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub networks_per_policy: ::std::option::Option<i32>,
        #[serde(
            rename = "policies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policies: ::std::option::Option<i32>,
        #[serde(
            rename = "resourceRecordsPerRrset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_records_per_rrset: ::std::option::Option<i32>,
        #[serde(
            rename = "rrsetAdditionsPerChange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rrset_additions_per_change: ::std::option::Option<i32>,
        #[serde(
            rename = "rrsetDeletionsPerChange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rrset_deletions_per_change: ::std::option::Option<i32>,
        #[serde(
            rename = "rrsetsPerManagedZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rrsets_per_managed_zone: ::std::option::Option<i32>,
        #[serde(
            rename = "targetNameServersPerManagedZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_name_servers_per_managed_zone: ::std::option::Option<i32>,
        #[serde(
            rename = "targetNameServersPerPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_name_servers_per_policy: ::std::option::Option<i32>,
        #[serde(
            rename = "totalRrdataSizePerChange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_rrdata_size_per_change: ::std::option::Option<i32>,
        #[serde(
            rename = "whitelistedKeySpecs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub whitelisted_key_specs: ::std::option::Option<Vec<crate::schemas::DnsKeySpec>>,
    }
    impl ::google_field_selector::FieldSelector for Quota {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Quota {
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
    pub struct ResourceRecordSet {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"dns#resourceRecordSet\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[serde(
            rename = "rrdatas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rrdatas: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "signatureRrdatas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature_rrdatas: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "ttl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ttl: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ResourceRecordSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceRecordSet {
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
    pub struct ResourceRecordSetsListResponse {
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ResponseHeader>,
        #[doc = "Type of resource."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "rrsets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rrsets: ::std::option::Option<Vec<crate::schemas::ResourceRecordSet>>,
    }
    impl ::google_field_selector::FieldSelector for ResourceRecordSetsListResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceRecordSetsListResponse {
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
    pub struct ResponseHeader {
        #[serde(
            rename = "operationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResponseHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResponseHeader {
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
    #[doc = "Actions that can be performed on the changes resource"]
    pub fn changes(&self) -> crate::resources::changes::ChangesActions {
        crate::resources::changes::ChangesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the dns_keys resource"]
    pub fn dns_keys(&self) -> crate::resources::dns_keys::DnsKeysActions {
        crate::resources::dns_keys::DnsKeysActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the managed_zone_operations resource"]
    pub fn managed_zone_operations(
        &self,
    ) -> crate::resources::managed_zone_operations::ManagedZoneOperationsActions {
        crate::resources::managed_zone_operations::ManagedZoneOperationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the managed_zones resource"]
    pub fn managed_zones(&self) -> crate::resources::managed_zones::ManagedZonesActions {
        crate::resources::managed_zones::ManagedZonesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the policies resource"]
    pub fn policies(&self) -> crate::resources::policies::PoliciesActions {
        crate::resources::policies::PoliciesActions {
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
    #[doc = "Actions that can be performed on the resource_record_sets resource"]
    pub fn resource_record_sets(
        &self,
    ) -> crate::resources::resource_record_sets::ResourceRecordSetsActions {
        crate::resources::resource_record_sets::ResourceRecordSetsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod changes {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSortBy {
                ChangeSequence,
            }
            impl ListSortBy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSortBy::ChangeSequence => "changeSequence",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSortBy {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSortBy {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSortBy, ()> {
                    Ok(match s {
                        "changeSequence" => ListSortBy::ChangeSequence,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSortBy {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSortBy {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSortBy {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "changeSequence" => ListSortBy::ChangeSequence,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListSortBy {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSortBy {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ChangesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ChangesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn create(
                &self,
                request: crate::schemas::Change,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> CreateRequestBuilder {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn get(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
                change_id: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    change_id: change_id.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn list(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    max_results: None,
                    page_token: None,
                    sort_by: None,
                    sort_order: None,
                }
            }
        }
        #[doc = "Created via [ChangesActions::create()](struct.ChangesActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Change,
            project: String,
            managed_zone: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> CreateRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::Change, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Change, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/changes");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ChangesActions::get()](struct.ChangesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            change_id: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::Change, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Change, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/changes/");
                {
                    let var_as_str = &self.change_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ChangesActions::list()](struct.ChangesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            max_results: Option<i32>,
            page_token: Option<String>,
            sort_by: Option<crate::resources::changes::params::ListSortBy>,
            sort_order: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = ""]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sort_by(mut self, value: crate::resources::changes::params::ListSortBy) -> Self {
                self.sort_by = Some(value);
                self
            }
            #[doc = ""]
            pub fn sort_order(mut self, value: impl Into<String>) -> Self {
                self.sort_order = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_changes<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_changes_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_changes_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Change> {
                self.iter_changes_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_changes_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Change> {
                self.iter_changes_with_fields(Some("*"))
            }
            pub fn iter_changes_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "changes").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "changes")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ChangesListResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ChangesListResponse> {
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
            ) -> Result<crate::schemas::ChangesListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ChangesListResponse, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/changes");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("sortBy", &self.sort_by)]);
                let req = req.query(&[("sortOrder", &self.sort_order)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
    pub mod dns_keys {
        pub mod params {}
        pub struct DnsKeysActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DnsKeysActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn get(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
                dns_key_id: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    dns_key_id: dns_key_id.into(),
                    client_operation_id: None,
                    digest_type: None,
                }
            }
            #[doc = ""]
            pub fn list(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    digest_type: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [DnsKeysActions::get()](struct.DnsKeysActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            dns_key_id: String,
            client_operation_id: Option<String>,
            digest_type: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn digest_type(mut self, value: impl Into<String>) -> Self {
                self.digest_type = Some(value.into());
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
            ) -> Result<crate::schemas::DnsKey, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::DnsKey, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/dnsKeys/");
                {
                    let var_as_str = &self.dns_key_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("digestType", &self.digest_type)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [DnsKeysActions::list()](struct.DnsKeysActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            digest_type: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = ""]
            pub fn digest_type(mut self, value: impl Into<String>) -> Self {
                self.digest_type = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_dns_keys<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_dns_keys_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_dns_keys_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DnsKey> {
                self.iter_dns_keys_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_dns_keys_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DnsKey> {
                self.iter_dns_keys_with_fields(Some("*"))
            }
            pub fn iter_dns_keys_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "dnsKeys").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "dnsKeys")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::DnsKeysListResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::DnsKeysListResponse> {
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
            ) -> Result<crate::schemas::DnsKeysListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DnsKeysListResponse, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/dnsKeys");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("digestType", &self.digest_type)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
    pub mod managed_zone_operations {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSortBy {
                Id,
                StartTime,
            }
            impl ListSortBy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSortBy::Id => "id",
                        ListSortBy::StartTime => "startTime",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSortBy {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSortBy {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSortBy, ()> {
                    Ok(match s {
                        "id" => ListSortBy::Id,
                        "startTime" => ListSortBy::StartTime,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSortBy {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSortBy {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSortBy {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "id" => ListSortBy::Id,
                        "startTime" => ListSortBy::StartTime,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListSortBy {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSortBy {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ManagedZoneOperationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ManagedZoneOperationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn get(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
                operation: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    operation: operation.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn list(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    max_results: None,
                    page_token: None,
                    sort_by: None,
                }
            }
        }
        #[doc = "Created via [ManagedZoneOperationsActions::get()](struct.ManagedZoneOperationsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            operation: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/operations/");
                {
                    let var_as_str = &self.operation;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ManagedZoneOperationsActions::list()](struct.ManagedZoneOperationsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            max_results: Option<i32>,
            page_token: Option<String>,
            sort_by: Option<crate::resources::managed_zone_operations::params::ListSortBy>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = ""]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sort_by(
                mut self,
                value: crate::resources::managed_zone_operations::params::ListSortBy,
            ) -> Self {
                self.sort_by = Some(value);
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ManagedZoneOperationsListResponse>
            {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ManagedZoneOperationsListResponse>
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
            ) -> Result<crate::schemas::ManagedZoneOperationsListResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ManagedZoneOperationsListResponse, crate::Error>
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/operations");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("sortBy", &self.sort_by)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
    pub mod managed_zones {
        pub mod params {}
        pub struct ManagedZonesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ManagedZonesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn create(
                &self,
                request: crate::schemas::ManagedZone,
                project: impl Into<String>,
            ) -> CreateRequestBuilder {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn delete(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn get(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn list(&self, project: impl Into<String>) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    dns_name: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = ""]
            pub fn patch(
                &self,
                request: crate::schemas::ManagedZone,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn update(
                &self,
                request: crate::schemas::ManagedZone,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    client_operation_id: None,
                }
            }
        }
        #[doc = "Created via [ManagedZonesActions::create()](struct.ManagedZonesActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ManagedZone,
            project: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> CreateRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::ManagedZone, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ManagedZone, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ManagedZonesActions::delete()](struct.ManagedZonesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ManagedZonesActions::get()](struct.ManagedZonesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::ManagedZone, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ManagedZone, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ManagedZonesActions::list()](struct.ManagedZonesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            dns_name: Option<String>,
            max_results: Option<i32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = ""]
            pub fn dns_name(mut self, value: impl Into<String>) -> Self {
                self.dns_name = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_managed_zones<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_managed_zones_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_managed_zones_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ManagedZone> {
                self.iter_managed_zones_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_managed_zones_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ManagedZone> {
                self.iter_managed_zones_with_fields(Some("*"))
            }
            pub fn iter_managed_zones_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "managedZones").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "managedZones")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ManagedZonesListResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ManagedZonesListResponse> {
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
            ) -> Result<crate::schemas::ManagedZonesListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ManagedZonesListResponse, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("dnsName", &self.dns_name)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
        #[doc = "Created via [ManagedZonesActions::patch()](struct.ManagedZonesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ManagedZone,
            project: String,
            managed_zone: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> PatchRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [ManagedZonesActions::update()](struct.ManagedZonesActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ManagedZone,
            project: String,
            managed_zone: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> UpdateRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod policies {
        pub mod params {}
        pub struct PoliciesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PoliciesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn create(
                &self,
                request: crate::schemas::Policy,
                project: impl Into<String>,
            ) -> CreateRequestBuilder {
                CreateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn delete(
                &self,
                project: impl Into<String>,
                policy: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    policy: policy.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn get(
                &self,
                project: impl Into<String>,
                policy: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    policy: policy.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn list(&self, project: impl Into<String>) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = ""]
            pub fn patch(
                &self,
                request: crate::schemas::Policy,
                project: impl Into<String>,
                policy: impl Into<String>,
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    policy: policy.into(),
                    client_operation_id: None,
                }
            }
            #[doc = ""]
            pub fn update(
                &self,
                request: crate::schemas::Policy,
                project: impl Into<String>,
                policy: impl Into<String>,
            ) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    policy: policy.into(),
                    client_operation_id: None,
                }
            }
        }
        #[doc = "Created via [PoliciesActions::create()](struct.PoliciesActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Policy,
            project: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> CreateRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Policy, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/policies");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [PoliciesActions::delete()](struct.PoliciesActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            policy: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> DeleteRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/policies/");
                {
                    let var_as_str = &self.policy;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [PoliciesActions::get()](struct.PoliciesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            policy: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::Policy, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Policy, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/policies/");
                {
                    let var_as_str = &self.policy;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [PoliciesActions::list()](struct.PoliciesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            max_results: Option<i32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = ""]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_policies<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_policies_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_policies_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Policy> {
                self.iter_policies_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_policies_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Policy> {
                self.iter_policies_with_fields(Some("*"))
            }
            pub fn iter_policies_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "policies").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "policies")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::PoliciesListResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::PoliciesListResponse> {
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
            ) -> Result<crate::schemas::PoliciesListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PoliciesListResponse, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/policies");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
        #[doc = "Created via [PoliciesActions::patch()](struct.PoliciesActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Policy,
            project: String,
            policy: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> PatchRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::PoliciesPatchResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PoliciesPatchResponse, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/policies/");
                {
                    let var_as_str = &self.policy;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [PoliciesActions::update()](struct.PoliciesActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Policy,
            project: String,
            policy: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> UpdateRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::PoliciesUpdateResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PoliciesUpdateResponse, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/policies/");
                {
                    let var_as_str = &self.policy;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
            #[doc = ""]
            pub fn get(&self, project: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    client_operation_id: None,
                }
            }
        }
        #[doc = "Created via [ProjectsActions::get()](struct.ProjectsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            client_operation_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = ""]
            pub fn client_operation_id(mut self, value: impl Into<String>) -> Self {
                self.client_operation_id = Some(value.into());
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
            ) -> Result<crate::schemas::Project, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Project, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("clientOperationId", &self.client_operation_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod resource_record_sets {
        pub mod params {}
        pub struct ResourceRecordSetsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ResourceRecordSetsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn list(
                &self,
                project: impl Into<String>,
                managed_zone: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project: project.into(),
                    managed_zone: managed_zone.into(),
                    max_results: None,
                    name: None,
                    page_token: None,
                    r#type: None,
                }
            }
        }
        #[doc = "Created via [ResourceRecordSetsActions::list()](struct.ResourceRecordSetsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project: String,
            managed_zone: String,
            max_results: Option<i32>,
            name: Option<String>,
            page_token: Option<String>,
            r#type: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = ""]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r#type(mut self, value: impl Into<String>) -> Self {
                self.r#type = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_rrsets<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_rrsets_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_rrsets_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ResourceRecordSet> {
                self.iter_rrsets_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_rrsets_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ResourceRecordSet> {
                self.iter_rrsets_with_fields(Some("*"))
            }
            pub fn iter_rrsets_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "rrsets").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "rrsets")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ResourceRecordSetsListResponse>
            {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ResourceRecordSetsListResponse>
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
            ) -> Result<crate::schemas::ResourceRecordSetsListResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ResourceRecordSetsListResponse, crate::Error> {
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
                let mut output = "https://dns.googleapis.com/dns/v1/projects/".to_owned();
                {
                    let var_as_str = &self.project;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/managedZones/");
                {
                    let var_as_str = &self.managed_zone;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/rrsets");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("name", &self.name)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("type", &self.r#type)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
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
