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
    pub struct Cluster {
        #[doc = "(`CreationOnly`)\nThe type of storage used by this cluster to serve its\nparent instance's tables, unless explicitly overridden."]
        #[serde(rename = "defaultStorageType", default)]
        pub default_storage_type: ::std::option::Option<crate::schemas::ClusterDefaultStorageType>,
        #[doc = "(`CreationOnly`)\nThe location where this cluster's nodes and storage reside. For best\nperformance, clients should be located as close as possible to this\ncluster. Currently only zones are supported, so values should be of the\nform `projects/<project>/locations/<zone>`."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "(`OutputOnly`)\nThe unique name of the cluster. Values are of the form\n`projects/<project>/instances/<instance>/clusters/a-z*`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The number of nodes allocated to this cluster. More nodes enable higher\nthroughput and more consistent performance."]
        #[serde(rename = "serveNodes", default)]
        pub serve_nodes: ::std::option::Option<i32>,
        #[doc = "(`OutputOnly`)\nThe current state of the cluster."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::ClusterState>,
    }
    impl ::field_selector::FieldSelector for Cluster {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ClusterDefaultStorageType {
        #[doc = "Magnetic drive (HDD) storage should be used."]
        Hdd,
        #[doc = "Flash (SSD) storage should be used."]
        Ssd,
        #[doc = "The user did not specify a storage type."]
        StorageTypeUnspecified,
    }
    impl ClusterDefaultStorageType {
        pub fn as_str(self) -> &'static str {
            match self {
                ClusterDefaultStorageType::Hdd => "HDD",
                ClusterDefaultStorageType::Ssd => "SSD",
                ClusterDefaultStorageType::StorageTypeUnspecified => "STORAGE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for ClusterDefaultStorageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ClusterDefaultStorageType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ClusterDefaultStorageType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HDD" => ClusterDefaultStorageType::Hdd,
                "SSD" => ClusterDefaultStorageType::Ssd,
                "STORAGE_TYPE_UNSPECIFIED" => ClusterDefaultStorageType::StorageTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ClusterDefaultStorageType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ClusterState {
        #[doc = "The cluster is currently being created, and may be destroyed\nif the creation process encounters an error.\nA cluster may not be able to serve requests while being created."]
        Creating,
        #[doc = "The cluster has no backing nodes. The data (tables) still\nexist, but no operations can be performed on the cluster."]
        Disabled,
        #[doc = "The cluster has been successfully created and is ready to serve requests."]
        Ready,
        #[doc = "The cluster is currently being resized, and may revert to its previous\nnode count if the process encounters an error.\nA cluster is still capable of serving requests while being resized,\nbut may exhibit performance as if its number of allocated nodes is\nbetween the starting and requested states."]
        Resizing,
        #[doc = "The state of the cluster could not be determined."]
        StateNotKnown,
    }
    impl ClusterState {
        pub fn as_str(self) -> &'static str {
            match self {
                ClusterState::Creating => "CREATING",
                ClusterState::Disabled => "DISABLED",
                ClusterState::Ready => "READY",
                ClusterState::Resizing => "RESIZING",
                ClusterState::StateNotKnown => "STATE_NOT_KNOWN",
            }
        }
    }
    impl ::std::fmt::Display for ClusterState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ClusterState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ClusterState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => ClusterState::Creating,
                "DISABLED" => ClusterState::Disabled,
                "READY" => ClusterState::Ready,
                "RESIZING" => ClusterState::Resizing,
                "STATE_NOT_KNOWN" => ClusterState::StateNotKnown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ClusterState {
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
    pub struct CreateClusterMetadata {
        #[doc = "The time at which the operation failed or was completed successfully."]
        #[serde(rename = "finishTime", default)]
        pub finish_time: ::std::option::Option<String>,
        #[doc = "The request that prompted the initiation of this CreateCluster operation."]
        #[serde(rename = "originalRequest", default)]
        pub original_request: ::std::option::Option<crate::schemas::CreateClusterRequest>,
        #[doc = "The time at which the original request was received."]
        #[serde(rename = "requestTime", default)]
        pub request_time: ::std::option::Option<String>,
        #[doc = "Keys: the full `name` of each table that existed in the instance when\nCreateCluster was first called, i.e.\n`projects/<project>/instances/<instance>/tables/<table>`. Any table added\nto the instance by a later API call will be created in the new cluster by\nthat API call, not this one.\n\nValues: information on how much of a table's data has been copied to the\nnewly-created cluster so far."]
        #[serde(rename = "tables", default)]
        pub tables: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::TableProgress>,
        >,
    }
    impl ::field_selector::FieldSelector for CreateClusterMetadata {
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
    pub struct CreateClusterRequest {
        #[doc = "The cluster to be created.\nFields marked `OutputOnly` must be left blank."]
        #[serde(rename = "cluster", default)]
        pub cluster: ::std::option::Option<crate::schemas::Cluster>,
        #[doc = "The ID to be used when referring to the new cluster within its instance,\ne.g., just `mycluster` rather than\n`projects/myproject/instances/myinstance/clusters/mycluster`."]
        #[serde(rename = "clusterId", default)]
        pub cluster_id: ::std::option::Option<String>,
        #[doc = "The unique name of the instance in which to create the new cluster.\nValues are of the form\n`projects/<project>/instances/<instance>`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateClusterRequest {
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
    pub struct CreateInstanceMetadata {
        #[doc = "The time at which the operation failed or was completed successfully."]
        #[serde(rename = "finishTime", default)]
        pub finish_time: ::std::option::Option<String>,
        #[doc = "The request that prompted the initiation of this CreateInstance operation."]
        #[serde(rename = "originalRequest", default)]
        pub original_request: ::std::option::Option<crate::schemas::CreateInstanceRequest>,
        #[doc = "The time at which the original request was received."]
        #[serde(rename = "requestTime", default)]
        pub request_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateInstanceMetadata {
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
    pub struct CreateInstanceRequest {
        #[doc = "The clusters to be created within the instance, mapped by desired\ncluster ID, e.g., just `mycluster` rather than\n`projects/myproject/instances/myinstance/clusters/mycluster`.\nFields marked `OutputOnly` must be left blank.\nCurrently, at most two clusters can be specified."]
        #[serde(rename = "clusters", default)]
        pub clusters:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Cluster>>,
        #[doc = "The instance to create.\nFields marked `OutputOnly` must be left blank."]
        #[serde(rename = "instance", default)]
        pub instance: ::std::option::Option<crate::schemas::Instance>,
        #[doc = "The ID to be used when referring to the new instance within its project,\ne.g., just `myinstance` rather than\n`projects/myproject/instances/myinstance`."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "The unique name of the project in which to create the new instance.\nValues are of the form `projects/<project>`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateInstanceRequest {
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
    pub struct Instance {
        #[doc = "The descriptive name for this instance as it appears in UIs.\nCan be changed at any time, but should be kept globally unique\nto avoid confusion."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Labels are a flexible and lightweight mechanism for organizing cloud\nresources into groups that reflect a customer's organizational needs and\ndeployment strategies. They can be used to filter resources and aggregate\nmetrics.\n\n* Label keys must be between 1 and 63 characters long and must conform to\n  the regular expression: `\\p{Ll}\\p{Lo}{0,62}`.\n* Label values must be between 0 and 63 characters long and must conform to\n  the regular expression: `[\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}`.\n* No more than 64 labels can be associated with a given resource.\n* Keys and values must both be under 128 bytes."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "(`OutputOnly`)\nThe unique name of the instance. Values are of the form\n`projects/<project>/instances/a-z+[a-z0-9]`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of the instance. Defaults to `PRODUCTION`."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::InstanceType>,
        #[doc = "(`OutputOnly`)\nThe current state of the instance."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::InstanceState>,
    }
    impl ::field_selector::FieldSelector for Instance {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InstanceType {
        #[doc = "The instance is meant for development and testing purposes only; it has\nno performance or uptime guarantees and is not covered by SLA.\nAfter a development instance is created, it can be upgraded by\nupdating the instance to type `PRODUCTION`. An instance created\nas a production instance cannot be changed to a development instance.\nWhen creating a development instance, `serve_nodes` on the cluster must\nnot be set."]
        Development,
        #[doc = "An instance meant for production use. `serve_nodes` must be set\non the cluster."]
        Production,
        #[doc = "The type of the instance is unspecified. If set when creating an\ninstance, a `PRODUCTION` instance will be created. If set when updating\nan instance, the type will be left unchanged."]
        TypeUnspecified,
    }
    impl InstanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                InstanceType::Development => "DEVELOPMENT",
                InstanceType::Production => "PRODUCTION",
                InstanceType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for InstanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InstanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InstanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVELOPMENT" => InstanceType::Development,
                "PRODUCTION" => InstanceType::Production,
                "TYPE_UNSPECIFIED" => InstanceType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for InstanceType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InstanceState {
        #[doc = "The instance is currently being created, and may be destroyed\nif the creation process encounters an error."]
        Creating,
        #[doc = "The instance has been successfully created and can serve requests\nto its tables."]
        Ready,
        #[doc = "The state of the instance could not be determined."]
        StateNotKnown,
    }
    impl InstanceState {
        pub fn as_str(self) -> &'static str {
            match self {
                InstanceState::Creating => "CREATING",
                InstanceState::Ready => "READY",
                InstanceState::StateNotKnown => "STATE_NOT_KNOWN",
            }
        }
    }
    impl ::std::fmt::Display for InstanceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InstanceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InstanceState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => InstanceState::Creating,
                "READY" => InstanceState::Ready,
                "STATE_NOT_KNOWN" => InstanceState::StateNotKnown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for InstanceState {
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
    pub struct PartialUpdateInstanceRequest {
        #[doc = "The Instance which will (partially) replace the current value."]
        #[serde(rename = "instance", default)]
        pub instance: ::std::option::Option<crate::schemas::Instance>,
        #[doc = "The subset of Instance fields which should be replaced.\nMust be explicitly set."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PartialUpdateInstanceRequest {
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
    pub struct TableProgress {
        #[doc = "Estimate of the number of bytes copied so far for this table.\nThis will eventually reach 'estimated_size_bytes' unless the table copy\nis CANCELLED."]
        #[serde(rename = "estimatedCopiedBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_copied_bytes: ::std::option::Option<i64>,
        #[doc = "Estimate of the size of the table to be copied."]
        #[serde(rename = "estimatedSizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_size_bytes: ::std::option::Option<i64>,
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::TableProgressState>,
    }
    impl ::field_selector::FieldSelector for TableProgress {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableProgressState {
        #[doc = "The table was deleted before it finished copying to the new cluster.\nNote that tables deleted after completion will stay marked as\nCOMPLETED, not CANCELLED."]
        Cancelled,
        #[doc = "The table has been fully copied to the new cluster."]
        Completed,
        #[doc = "The table is actively being copied to the new cluster."]
        Copying,
        #[doc = "The table has not yet begun copying to the new cluster."]
        Pending,
        StateUnspecified,
    }
    impl TableProgressState {
        pub fn as_str(self) -> &'static str {
            match self {
                TableProgressState::Cancelled => "CANCELLED",
                TableProgressState::Completed => "COMPLETED",
                TableProgressState::Copying => "COPYING",
                TableProgressState::Pending => "PENDING",
                TableProgressState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for TableProgressState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableProgressState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableProgressState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => TableProgressState::Cancelled,
                "COMPLETED" => TableProgressState::Completed,
                "COPYING" => TableProgressState::Copying,
                "PENDING" => TableProgressState::Pending,
                "STATE_UNSPECIFIED" => TableProgressState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TableProgressState {
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
    pub struct UpdateAppProfileMetadata;
    impl ::field_selector::FieldSelector for UpdateAppProfileMetadata {
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
    pub struct UpdateClusterMetadata {
        #[doc = "The time at which the operation failed or was completed successfully."]
        #[serde(rename = "finishTime", default)]
        pub finish_time: ::std::option::Option<String>,
        #[doc = "The request that prompted the initiation of this UpdateCluster operation."]
        #[serde(rename = "originalRequest", default)]
        pub original_request: ::std::option::Option<crate::schemas::Cluster>,
        #[doc = "The time at which the original request was received."]
        #[serde(rename = "requestTime", default)]
        pub request_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdateClusterMetadata {
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
    pub struct UpdateInstanceMetadata {
        #[doc = "The time at which the operation failed or was completed successfully."]
        #[serde(rename = "finishTime", default)]
        pub finish_time: ::std::option::Option<String>,
        #[doc = "The request that prompted the initiation of this UpdateInstance operation."]
        #[serde(rename = "originalRequest", default)]
        pub original_request: ::std::option::Option<crate::schemas::PartialUpdateInstanceRequest>,
        #[doc = "The time at which the original request was received."]
        #[serde(rename = "requestTime", default)]
        pub request_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdateInstanceMetadata {
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
}
pub mod resources {}
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
