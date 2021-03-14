#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [locations](resources/projects/locations/struct.LocationsActions.html)\n        * [environments](resources/projects/locations/environments/struct.EnvironmentsActions.html)\n          * [*create*](resources/projects/locations/environments/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/environments/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/environments/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/environments/struct.ListRequestBuilder.html), [*patch*](resources/projects/locations/environments/struct.PatchRequestBuilder.html)\n        * [image_versions](resources/projects/locations/image_versions/struct.ImageVersionsActions.html)\n          * [*list*](resources/projects/locations/image_versions/struct.ListRequestBuilder.html)\n        * [operations](resources/projects/locations/operations/struct.OperationsActions.html)\n          * [*delete*](resources/projects/locations/operations/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/operations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/operations/struct.ListRequestBuilder.html)\n"]
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
    pub struct AllowedIpRange {
        #[doc = "Optional. User-provided description. It must contain at most 300 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "IP address or range, defined using CIDR notation, of requests that this rule applies to. Examples: `192.168.1.1` or `192.168.0.0/16` or `2001:db8::/32` or `2001:0db8:0000:0042:0000:8a2e:0370:7334`. IP range prefixes should be properly truncated. For example, `1.2.3.4/24` should be truncated to `1.2.3.0/24`. Similarly, for IPv6, `2001:db8::1/32` should be truncated to `2001:db8::/32`."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AllowedIpRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AllowedIpRange {
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
    pub struct DatabaseConfig {
        #[doc = "Optional. Cloud SQL machine type used by Airflow database. It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8 or db-n1-standard-16. If not specified, db-n1-standard-2 will be used."]
        #[serde(
            rename = "machineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DatabaseConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatabaseConfig {
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
    pub struct Date {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Date {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Date {
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EncryptionConfig {
        #[doc = "Optional. Customer-managed Encryption Key available through Google's Key Management Service. Cannot be updated. If not specified, Google-managed key will be used."]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EncryptionConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EncryptionConfig {
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
    pub struct Environment {
        #[doc = "Configuration parameters for this environment."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::EnvironmentConfig>,
        #[doc = "Output only. The time at which this environment was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Optional. User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \\p{Ll}\\p{Lo}{0,62} * Values must conform to regexp: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The resource name of the environment, in the form: \"projects/{projectId}/locations/{locationId}/environments/{environmentId}\" EnvironmentId must start with a lowercase letter followed by up to 63 lowercase letters, numbers, or hyphens, and cannot end with a hyphen."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The current state of the environment."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::EnvironmentState>,
        #[doc = "Output only. The time at which this environment was last modified."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Output only. The UUID (Universally Unique IDentifier) associated with this environment. This value is generated when the environment is created."]
        #[serde(
            rename = "uuid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uuid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Environment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Environment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnvironmentState {
        #[doc = "The environment is in the process of being created."]
        Creating,
        #[doc = "The environment is undergoing deletion. It cannot be used."]
        Deleting,
        #[doc = "The environment has encountered an error and cannot be used."]
        Error,
        #[doc = "The environment is currently running and healthy. It is ready for use."]
        Running,
        #[doc = "The state of the environment is unknown."]
        StateUnspecified,
        #[doc = "The environment is being updated. It remains usable but cannot receive additional update requests or be deleted at this time."]
        Updating,
    }
    impl EnvironmentState {
        pub fn as_str(self) -> &'static str {
            match self {
                EnvironmentState::Creating => "CREATING",
                EnvironmentState::Deleting => "DELETING",
                EnvironmentState::Error => "ERROR",
                EnvironmentState::Running => "RUNNING",
                EnvironmentState::StateUnspecified => "STATE_UNSPECIFIED",
                EnvironmentState::Updating => "UPDATING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EnvironmentState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EnvironmentState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EnvironmentState, ()> {
            Ok(match s {
                "CREATING" => EnvironmentState::Creating,
                "DELETING" => EnvironmentState::Deleting,
                "ERROR" => EnvironmentState::Error,
                "RUNNING" => EnvironmentState::Running,
                "STATE_UNSPECIFIED" => EnvironmentState::StateUnspecified,
                "UPDATING" => EnvironmentState::Updating,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EnvironmentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnvironmentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnvironmentState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => EnvironmentState::Creating,
                "DELETING" => EnvironmentState::Deleting,
                "ERROR" => EnvironmentState::Error,
                "RUNNING" => EnvironmentState::Running,
                "STATE_UNSPECIFIED" => EnvironmentState::StateUnspecified,
                "UPDATING" => EnvironmentState::Updating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EnvironmentState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnvironmentState {
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
    pub struct EnvironmentConfig {
        #[doc = "Output only. The URI of the Apache Airflow Web UI hosted within this environment (see [Airflow web interface](/composer/docs/how-to/accessing/airflow-web-interface))."]
        #[serde(
            rename = "airflowUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub airflow_uri: ::std::option::Option<String>,
        #[doc = "Output only. The Cloud Storage prefix of the DAGs for this environment. Although Cloud Storage objects reside in a flat namespace, a hierarchical file tree can be simulated using \"/\"-delimited object name prefixes. DAG objects for this environment reside in a simulated directory with the given prefix."]
        #[serde(
            rename = "dagGcsPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dag_gcs_prefix: ::std::option::Option<String>,
        #[doc = "Optional. The configuration settings for Cloud SQL instance used internally by Apache Airflow software."]
        #[serde(
            rename = "databaseConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub database_config: ::std::option::Option<crate::schemas::DatabaseConfig>,
        #[doc = "Optional. The encryption options for the Cloud Composer environment and its dependencies. Cannot be updated."]
        #[serde(
            rename = "encryptionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encryption_config: ::std::option::Option<crate::schemas::EncryptionConfig>,
        #[doc = "Output only. The Kubernetes Engine cluster used to run this environment."]
        #[serde(
            rename = "gkeCluster",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gke_cluster: ::std::option::Option<String>,
        #[doc = "The configuration used for the Kubernetes Engine cluster."]
        #[serde(
            rename = "nodeConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_config: ::std::option::Option<crate::schemas::NodeConfig>,
        #[doc = "The number of nodes in the Kubernetes Engine cluster that will be used to run this environment."]
        #[serde(
            rename = "nodeCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_count: ::std::option::Option<i32>,
        #[doc = "The configuration used for the Private IP Cloud Composer environment."]
        #[serde(
            rename = "privateEnvironmentConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_environment_config:
            ::std::option::Option<crate::schemas::PrivateEnvironmentConfig>,
        #[doc = "The configuration settings for software inside the environment."]
        #[serde(
            rename = "softwareConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub software_config: ::std::option::Option<crate::schemas::SoftwareConfig>,
        #[doc = "Optional. The configuration settings for the Airflow web server App Engine instance."]
        #[serde(
            rename = "webServerConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_server_config: ::std::option::Option<crate::schemas::WebServerConfig>,
        #[doc = "Optional. The network-level access control policy for the Airflow web server. If unspecified, no network-level access restrictions will be applied."]
        #[serde(
            rename = "webServerNetworkAccessControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_server_network_access_control:
            ::std::option::Option<crate::schemas::WebServerNetworkAccessControl>,
    }
    impl ::google_field_selector::FieldSelector for EnvironmentConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnvironmentConfig {
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
    pub struct ImageVersion {
        #[doc = "Whether it is impossible to create an environment with the image version."]
        #[serde(
            rename = "creationDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_disabled: ::std::option::Option<bool>,
        #[doc = "The string identifier of the ImageVersion, in the form: \"composer-x.y.z-airflow-a.b(.c)\""]
        #[serde(
            rename = "imageVersionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_version_id: ::std::option::Option<String>,
        #[doc = "Whether this is the default ImageVersion used by Composer during environment creation if no input ImageVersion is specified."]
        #[serde(
            rename = "isDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_default: ::std::option::Option<bool>,
        #[doc = "The date of the version release."]
        #[serde(
            rename = "releaseDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub release_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "supported python versions"]
        #[serde(
            rename = "supportedPythonVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supported_python_versions: ::std::option::Option<Vec<String>>,
        #[doc = "Whether it is impossible to upgrade an environment running with the image version."]
        #[serde(
            rename = "upgradeDisabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade_disabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ImageVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageVersion {
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
    pub struct IpallocationPolicy {
        #[doc = "Optional. The IP address range used to allocate IP addresses to pods in the GKE cluster. This field is applicable only when `use_ip_aliases` is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
        #[serde(
            rename = "clusterIpv4CidrBlock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_ipv_4_cidr_block: ::std::option::Option<String>,
        #[doc = "Optional. The name of the GKE cluster's secondary range used to allocate IP addresses to pods. This field is applicable only when `use_ip_aliases` is true."]
        #[serde(
            rename = "clusterSecondaryRangeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_secondary_range_name: ::std::option::Option<String>,
        #[doc = "Optional. The IP address range of the services IP addresses in this GKE cluster. This field is applicable only when `use_ip_aliases` is true. Set to blank to have GKE choose a range with the default size. Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific netmask. Set to a [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g. `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range to use."]
        #[serde(
            rename = "servicesIpv4CidrBlock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub services_ipv_4_cidr_block: ::std::option::Option<String>,
        #[doc = "Optional. The name of the services' secondary range used to allocate IP addresses to the GKE cluster. This field is applicable only when `use_ip_aliases` is true."]
        #[serde(
            rename = "servicesSecondaryRangeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub services_secondary_range_name: ::std::option::Option<String>,
        #[doc = "Optional. Whether or not to enable Alias IPs in the GKE cluster. If `true`, a VPC-native cluster is created."]
        #[serde(
            rename = "useIpAliases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_ip_aliases: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for IpallocationPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IpallocationPolicy {
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
    pub struct ListEnvironmentsResponse {
        #[doc = "The list of environments returned by a ListEnvironmentsRequest."]
        #[serde(
            rename = "environments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environments: ::std::option::Option<Vec<crate::schemas::Environment>>,
        #[doc = "The page token used to query for the next page if one exists."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListEnvironmentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListEnvironmentsResponse {
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
    pub struct ListImageVersionsResponse {
        #[doc = "The list of supported ImageVersions in a location."]
        #[serde(
            rename = "imageVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_versions: ::std::option::Option<Vec<crate::schemas::ImageVersion>>,
        #[doc = "The page token used to query for the next page if one exists."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListImageVersionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListImageVersionsResponse {
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NodeConfig {
        #[doc = "Optional. The disk size in GB used for node VMs. Minimum size is 20GB. If unspecified, defaults to 100GB. Cannot be updated."]
        #[serde(
            rename = "diskSizeGb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk_size_gb: ::std::option::Option<i32>,
        #[doc = "Optional. The configuration for controlling how IPs are allocated in the GKE cluster."]
        #[serde(
            rename = "ipAllocationPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_allocation_policy: ::std::option::Option<crate::schemas::IpallocationPolicy>,
        #[doc = "Optional. The Compute Engine [zone](/compute/docs/regions-zones) in which to deploy the VMs used to run the Apache Airflow software, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/zones/{zoneId}\". This `location` must belong to the enclosing environment's project and location. If both this field and `nodeConfig.machineType` are specified, `nodeConfig.machineType` must belong to this `location`; if both are unspecified, the service will pick a zone in the Compute Engine region corresponding to the Cloud Composer location, and propagate that choice to both fields. If only one field (`location` or `nodeConfig.machineType`) is specified, the location information from the specified field will be propagated to the unspecified field."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. The Compute Engine [machine type](/compute/docs/machine-types) used for cluster instances, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/zones/{zoneId}/machineTypes/{machineTypeId}\". The `machineType` must belong to the enclosing environment's project and location. If both this field and `nodeConfig.location` are specified, this `machineType` must belong to the `nodeConfig.location`; if both are unspecified, the service will pick a zone in the Compute Engine region corresponding to the Cloud Composer location, and propagate that choice to both fields. If exactly one of this field and `nodeConfig.location` is specified, the location information from the specified field will be propagated to the unspecified field. The `machineTypeId` must not be a [shared-core machine type](/compute/docs/machine-types#sharedcore). If this field is unspecified, the `machineTypeId` defaults to \"n1-standard-1\"."]
        #[serde(
            rename = "machineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "Optional. The Compute Engine network to be used for machine communications, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/global/networks/{networkId}\". If unspecified, the \"default\" network ID in the environment's project is used. If a [Custom Subnet Network](/vpc/docs/vpc#vpc_networks_and_subnets) is provided, `nodeConfig.subnetwork` must also be provided. For [Shared VPC](/vpc/docs/shared-vpc) subnetwork requirements, see `nodeConfig.subnetwork`."]
        #[serde(
            rename = "network",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network: ::std::option::Option<String>,
        #[doc = "Optional. The set of Google API scopes to be made available on all node VMs. If `oauth_scopes` is empty, defaults to [\"https://www.googleapis.com/auth/cloud-platform\"]. Cannot be updated."]
        #[serde(
            rename = "oauthScopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_scopes: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The Google Cloud Platform Service Account to be used by the node VMs. If a service account is not specified, the \"default\" Compute Engine service account is used. Cannot be updated."]
        #[serde(
            rename = "serviceAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account: ::std::option::Option<String>,
        #[doc = "Optional. The Compute Engine subnetwork to be used for machine communications, specified as a [relative resource name](/apis/design/resource_names#relative_resource_name). For example: \"projects/{projectId}/regions/{regionId}/subnetworks/{subnetworkId}\" If a subnetwork is provided, `nodeConfig.network` must also be provided, and the subnetwork must belong to the enclosing environment's project and location."]
        #[serde(
            rename = "subnetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subnetwork: ::std::option::Option<String>,
        #[doc = "Optional. The list of instance tags applied to all node VMs. Tags are used to identify valid sources or targets for network firewalls. Each tag within the list must comply with [RFC1035](https://www.ietf.org/rfc/rfc1035.txt). Cannot be updated."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for NodeConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NodeConfig {
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
    pub struct OperationMetadata {
        #[doc = "Output only. The time the operation was submitted to the server."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The time when the operation terminated, regardless of its success. This field is unset if the operation is still ongoing."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. The type of operation being performed."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type: ::std::option::Option<crate::schemas::OperationMetadataOperationType>,
        #[doc = "Output only. The resource being operated on, as a [relative resource name](/apis/design/resource_names#relative_resource_name)."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
        #[doc = "Output only. The UUID of the resource being operated on."]
        #[serde(
            rename = "resourceUuid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uuid: ::std::option::Option<String>,
        #[doc = "Output only. The current operation state."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::OperationMetadataState>,
    }
    impl ::google_field_selector::FieldSelector for OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OperationMetadataOperationType {
        #[doc = "A resource creation operation."]
        Create,
        #[doc = "A resource deletion operation."]
        Delete,
        #[doc = "Unused."]
        TypeUnspecified,
        #[doc = "A resource update operation."]
        Update,
    }
    impl OperationMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self {
                OperationMetadataOperationType::Create => "CREATE",
                OperationMetadataOperationType::Delete => "DELETE",
                OperationMetadataOperationType::TypeUnspecified => "TYPE_UNSPECIFIED",
                OperationMetadataOperationType::Update => "UPDATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OperationMetadataOperationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OperationMetadataOperationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OperationMetadataOperationType, ()> {
            Ok(match s {
                "CREATE" => OperationMetadataOperationType::Create,
                "DELETE" => OperationMetadataOperationType::Delete,
                "TYPE_UNSPECIFIED" => OperationMetadataOperationType::TypeUnspecified,
                "UPDATE" => OperationMetadataOperationType::Update,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OperationMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OperationMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OperationMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATE" => OperationMetadataOperationType::Create,
                "DELETE" => OperationMetadataOperationType::Delete,
                "TYPE_UNSPECIFIED" => OperationMetadataOperationType::TypeUnspecified,
                "UPDATE" => OperationMetadataOperationType::Update,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OperationMetadataOperationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationMetadataOperationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OperationMetadataState {
        #[doc = "The operation is no longer running but did not succeed."]
        Failed,
        #[doc = "The operation has been created but is not yet started."]
        Pending,
        #[doc = "The operation is underway."]
        Running,
        #[doc = "Unused."]
        StateUnspecified,
        #[doc = "The operation completed successfully."]
        Succeeded,
        Successful,
    }
    impl OperationMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                OperationMetadataState::Failed => "FAILED",
                OperationMetadataState::Pending => "PENDING",
                OperationMetadataState::Running => "RUNNING",
                OperationMetadataState::StateUnspecified => "STATE_UNSPECIFIED",
                OperationMetadataState::Succeeded => "SUCCEEDED",
                OperationMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OperationMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OperationMetadataState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OperationMetadataState, ()> {
            Ok(match s {
                "FAILED" => OperationMetadataState::Failed,
                "PENDING" => OperationMetadataState::Pending,
                "RUNNING" => OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => OperationMetadataState::StateUnspecified,
                "SUCCEEDED" => OperationMetadataState::Succeeded,
                "SUCCESSFUL" => OperationMetadataState::Successful,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OperationMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OperationMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OperationMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FAILED" => OperationMetadataState::Failed,
                "PENDING" => OperationMetadataState::Pending,
                "RUNNING" => OperationMetadataState::Running,
                "STATE_UNSPECIFIED" => OperationMetadataState::StateUnspecified,
                "SUCCEEDED" => OperationMetadataState::Succeeded,
                "SUCCESSFUL" => OperationMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OperationMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationMetadataState {
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
    pub struct PrivateClusterConfig {
        #[doc = "Optional. If `true`, access to the public endpoint of the GKE cluster is denied."]
        #[serde(
            rename = "enablePrivateEndpoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_private_endpoint: ::std::option::Option<bool>,
        #[doc = "Optional. The CIDR block from which IPv4 range for GKE master will be reserved. If left blank, the default value of '172.16.0.0/23' is used."]
        #[serde(
            rename = "masterIpv4CidrBlock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_ipv_4_cidr_block: ::std::option::Option<String>,
        #[doc = "Output only. The IP range in CIDR notation to use for the hosted master network. This range is used for assigning internal IP addresses to the GKE cluster master or set of masters and to the internal load balancer virtual IP. This range must not overlap with any other ranges in use within the cluster's network."]
        #[serde(
            rename = "masterIpv4ReservedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_ipv_4_reserved_range: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PrivateClusterConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PrivateClusterConfig {
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
    pub struct PrivateEnvironmentConfig {
        #[doc = "Optional. The CIDR block from which IP range in tenant project will be reserved for Cloud SQL. Needs to be disjoint from `web_server_ipv4_cidr_block`."]
        #[serde(
            rename = "cloudSqlIpv4CidrBlock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_sql_ipv_4_cidr_block: ::std::option::Option<String>,
        #[doc = "Optional. If `true`, a Private IP Cloud Composer environment is created. If this field is set to true, `IPAllocationPolicy.use_ip_aliases` must be set to true."]
        #[serde(
            rename = "enablePrivateEnvironment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_private_environment: ::std::option::Option<bool>,
        #[doc = "Optional. Configuration for the private GKE cluster for a Private IP Cloud Composer environment."]
        #[serde(
            rename = "privateClusterConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_cluster_config: ::std::option::Option<crate::schemas::PrivateClusterConfig>,
        #[doc = "Optional. The CIDR block from which IP range for web server will be reserved. Needs to be disjoint from `private_cluster_config.master_ipv4_cidr_block` and `cloud_sql_ipv4_cidr_block`."]
        #[serde(
            rename = "webServerIpv4CidrBlock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_server_ipv_4_cidr_block: ::std::option::Option<String>,
        #[doc = "Output only. The IP range reserved for the tenant project's App Engine VMs."]
        #[serde(
            rename = "webServerIpv4ReservedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_server_ipv_4_reserved_range: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PrivateEnvironmentConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PrivateEnvironmentConfig {
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
    pub struct SoftwareConfig {
        #[doc = "Optional. Apache Airflow configuration properties to override. Property keys contain the section and property names, separated by a hyphen, for example \"core-dags_are_paused_at_creation\". Section names must not contain hyphens (\"-\"), opening square brackets (\"[\"), or closing square brackets (\"]\"). The property name must not be empty and must not contain an equals sign (\"=\") or semicolon (\";\"). Section and property names must not contain a period (\".\"). Apache Airflow configuration property names must be written in [snake_case](https://en.wikipedia.org/wiki/Snake_case). Property values can contain any character, and can be written in any lower/upper case format. Certain Apache Airflow configuration property values are [blocked](/composer/docs/concepts/airflow-configurations), and cannot be overridden."]
        #[serde(
            rename = "airflowConfigOverrides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub airflow_config_overrides:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Optional. Additional environment variables to provide to the Apache Airflow scheduler, worker, and webserver processes. Environment variable names must match the regular expression `a-zA-Z_*`. They cannot specify Apache Airflow software configuration overrides (they cannot match the regular expression `AIRFLOW__[A-Z0-9_]+__[A-Z0-9_]+`), and they cannot match any of the following reserved names: * `AIRFLOW_HOME` * `C_FORCE_ROOT` * `CONTAINER_NAME` * `DAGS_FOLDER` * `GCP_PROJECT` * `GCS_BUCKET` * `GKE_CLUSTER_NAME` * `SQL_DATABASE` * `SQL_INSTANCE` * `SQL_PASSWORD` * `SQL_PROJECT` * `SQL_REGION` * `SQL_USER`"]
        #[serde(
            rename = "envVariables",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env_variables: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The version of the software running in the environment. This encapsulates both the version of Cloud Composer functionality and the version of Apache Airflow. It must match the regular expression `composer-([0-9]+\\.[0-9]+\\.[0-9]+|latest)-airflow-[0-9]+\\.[0-9]+(\\.[0-9]+.*)?`. When used as input, the server also checks if the provided version is supported and denies the request for an unsupported version. The Cloud Composer portion of the version is a [semantic version](https://semver.org) or `latest`. When the patch version is omitted, the current Cloud Composer patch version is selected. When `latest` is provided instead of an explicit version number, the server replaces `latest` with the current Cloud Composer version and stores that version number in the same field. The portion of the image version that follows *airflow-* is an official Apache Airflow repository [release name](https://github.com/apache/incubator-airflow/releases). See also [Version List](/composer/docs/concepts/versioning/composer-versions)."]
        #[serde(
            rename = "imageVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_version: ::std::option::Option<String>,
        #[doc = "Optional. Custom Python Package Index (PyPI) packages to be installed in the environment. Keys refer to the lowercase package name such as \"numpy\" and values are the lowercase extras and version specifier such as \"==1.12.0\", \"[devel,gcp_api]\", or \"[devel]>=1.8.2, <1.9.2\". To specify a package without pinning it to a version specifier, use the empty string as the value."]
        #[serde(
            rename = "pypiPackages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pypi_packages: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Optional. The major version of Python used to run the Apache Airflow scheduler, worker, and webserver processes. Can be set to '2' or '3'. If not specified, the default is '2'. Cannot be updated."]
        #[serde(
            rename = "pythonVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub python_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SoftwareConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SoftwareConfig {
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WebServerConfig {
        #[doc = "Optional. Machine type on which Airflow web server is running. It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or composer-n1-webserver-8. If not specified, composer-n1-webserver-2 will be used. Value custom is returned only in response, if Airflow web server parameters were manually changed to a non-standard values."]
        #[serde(
            rename = "machineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WebServerConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebServerConfig {
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
    pub struct WebServerNetworkAccessControl {
        #[doc = "A collection of allowed IP ranges with descriptions."]
        #[serde(
            rename = "allowedIpRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_ip_ranges: ::std::option::Option<Vec<crate::schemas::AllowedIpRange>>,
    }
    impl ::google_field_selector::FieldSelector for WebServerNetworkAccessControl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WebServerNetworkAccessControl {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the environments resource"]
                pub fn environments(
                    &self,
                ) -> crate::resources::projects::locations::environments::EnvironmentsActions
                {
                    crate::resources::projects::locations::environments::EnvironmentsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the image_versions resource"]
                pub fn image_versions(
                    &self,
                ) -> crate::resources::projects::locations::image_versions::ImageVersionsActions
                {
                    crate::resources::projects::locations::image_versions::ImageVersionsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the operations resource"]
                pub fn operations(
                    &self,
                ) -> crate::resources::projects::locations::operations::OperationsActions
                {
                    crate::resources::projects::locations::operations::OperationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod environments {
                pub mod params {}
                pub struct EnvironmentsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> EnvironmentsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Create a new environment."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Environment,
                        parent: impl Into<String>,
                    ) -> CreateRequestBuilder {
                        CreateRequestBuilder {
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
                    #[doc = "Delete an environment."]
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
                    #[doc = "Get an existing environment."]
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
                    #[doc = "List environments."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                            parent: parent.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Update an environment."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::Environment,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder {
                        PatchRequestBuilder {
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
                            name: name.into(),
                            update_mask: None,
                        }
                    }
                }
                #[doc = "Created via [EnvironmentsActions::create()](struct.EnvironmentsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Environment,
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
                impl<'a> CreateRequestBuilder<'a> {
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
                        let mut output = "https://composer.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/environments");
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
                #[doc = "Created via [EnvironmentsActions::delete()](struct.EnvironmentsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://composer.googleapis.com/".to_owned();
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
                #[doc = "Created via [EnvironmentsActions::get()](struct.EnvironmentsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
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
                    ) -> Result<crate::schemas::Environment, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Environment, crate::Error> {
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
                        let mut output = "https://composer.googleapis.com/".to_owned();
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                #[doc = "Created via [EnvironmentsActions::list()](struct.EnvironmentsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
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
                    #[doc = "The maximum number of environments to return."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
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
                    ) -> Result<crate::schemas::ListEnvironmentsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListEnvironmentsResponse, crate::Error>
                    {
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
                        let mut output = "https://composer.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/environments");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
                #[doc = "Created via [EnvironmentsActions::patch()](struct.EnvironmentsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Environment,
                    name: String,
                    update_mask: Option<String>,
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
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "Required. A comma-separated list of paths, relative to `Environment`, of fields to update. For example, to set the version of scikit-learn to install in the environment to 0.19.0 and to remove an existing installation of numpy, the `updateMask` parameter would include the following two `paths` values: \"config.softwareConfig.pypiPackages.scikit-learn\" and \"config.softwareConfig.pypiPackages.numpy\". The included patch environment would specify the scikit-learn version as follows: { \"config\":{ \"softwareConfig\":{ \"pypiPackages\":{ \"scikit-learn\":\"==0.19.0\" } } } } Note that in the above example, any existing PyPI packages other than scikit-learn and numpy will be unaffected. Only one update type may be included in a single request's `updateMask`. For example, one cannot update both the PyPI packages and labels in the same request. However, it is possible to update multiple members of a map field simultaneously in the same request. For example, to set the labels \"label1\" and \"label2\" while clearing \"label3\" (assuming it already exists), one can provide the paths \"labels.label1\", \"labels.label2\", and \"labels.label3\" and populate the patch environment as follows: { \"labels\":{ \"label1\":\"new-label1-value\" \"label2\":\"new-label2-value\" } } Note that in the above example, any existing labels that are not included in the `updateMask` will be unaffected. It is also possible to replace an entire map field by providing the map field's path in the `updateMask`. The new value of the field will be that which is provided in the patch environment. For example, to delete all pre-existing user-specified PyPI packages and install botocore at version 1.7.14, the `updateMask` would contain the path \"config.softwareConfig.pypiPackages\", and the patch environment would be the following: { \"config\":{ \"softwareConfig\":{ \"pypiPackages\":{ \"botocore\":\"==1.7.14\" } } } } **Note:** Only the following fields can be updated: *Mask* *Purpose* config.softwareConfig.pypiPackages Replace all custom custom PyPI packages. If a replacement package map is not included in `environment`, all custom PyPI packages are cleared. It is an error to provide both this mask and a mask specifying an individual package. config.softwareConfig.pypiPackages.packagename Update the custom PyPI package packagename, preserving other packages. To delete the package, include it in `updateMask`, and omit the mapping for it in `environment.config.softwareConfig.pypiPackages`. It is an error to provide both a mask of this form and the \"config.softwareConfig.pypiPackages\" mask. labels Replace all environment labels. If a replacement labels map is not included in `environment`, all labels are cleared. It is an error to provide both this mask and a mask specifying one or more individual labels. labels.labelName Set the label named labelName, while preserving other labels. To delete the label, include it in `updateMask` and omit its mapping in `environment.labels`. It is an error to provide both a mask of this form and the \"labels\" mask. config.nodeCount Horizontally scale the number of nodes in the environment. An integer greater than or equal to 3 must be provided in the `config.nodeCount` field. config.webServerNetworkAccessControl Replace the environment's current WebServerNetworkAccessControl. config.databaseConfig Replace the environment's current DatabaseConfig. config.webServerConfig Replace the environment's current WebServerConfig. config.softwareConfig.airflowConfigOverrides Replace all Apache Airflow config overrides. If a replacement config overrides map is not included in `environment`, all config overrides are cleared. It is an error to provide both this mask and a mask specifying one or more individual config overrides. config.softwareConfig.airflowConfigOverrides.section-name Override the Apache Airflow config property name in the section named section, preserving other properties. To delete the property override, include it in `updateMask` and omit its mapping in `environment.config.softwareConfig.airflowConfigOverrides`. It is an error to provide both a mask of this form and the \"config.softwareConfig.airflowConfigOverrides\" mask. config.softwareConfig.envVariables Replace all environment variables. If a replacement environment variable map is not included in `environment`, all custom environment variables are cleared. It is an error to provide both this mask and a mask specifying one or more individual environment variables. "]
                    pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                        self.update_mask = Some(value.into());
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
                        let mut output = "https://composer.googleapis.com/".to_owned();
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        req = req.query(&[("updateMask", &self.update_mask)]);
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
            pub mod image_versions {
                pub mod params {}
                pub struct ImageVersionsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ImageVersionsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "List ImageVersions for provided location."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                            parent: parent.into(),
                            include_past_releases: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[doc = "Created via [ImageVersionsActions::list()](struct.ImageVersionsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    include_past_releases: Option<bool>,
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
                    #[doc = "Whether or not image versions from old releases should be included."]
                    pub fn include_past_releases(mut self, value: bool) -> Self {
                        self.include_past_releases = Some(value);
                        self
                    }
                    #[doc = "The maximum number of image_versions to return."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
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
                    ) -> Result<crate::schemas::ListImageVersionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListImageVersionsResponse, crate::Error>
                    {
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
                        let mut output = "https://composer.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/imageVersions");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("includePastReleases", &self.include_past_releases)]);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
            pub mod operations {
                pub mod params {}
                pub struct OperationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OperationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`."]
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
                    #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
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
                    #[doc = "Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
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
                #[doc = "Created via [OperationsActions::delete()](struct.OperationsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
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
                    ) -> Result<crate::schemas::Empty, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Empty, crate::Error> {
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
                        let mut output = "https://composer.googleapis.com/".to_owned();
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
                #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://composer.googleapis.com/".to_owned();
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
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
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListOperationsResponse, crate::Error>
                    {
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
                        let mut output = "https://composer.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/operations");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
