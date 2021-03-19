#![doc = "# Resources and Methods\n    * [discovery](resources/discovery/struct.DiscoveryActions.html)\n      * [*client_status*](resources/discovery/struct.ClientStatusRequestBuilder.html)\n"]
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
    pub struct Address {
        #[serde(
            rename = "pipe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pipe: ::std::option::Option<crate::schemas::Pipe>,
        #[serde(
            rename = "socketAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub socket_address: ::std::option::Option<crate::schemas::SocketAddress>,
    }
    impl ::google_field_selector::FieldSelector for Address {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Address {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BuildVersion {
        #[doc = "Free-form build information. Envoy defines several well known keys in the source/common/version/version.h file"]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "SemVer version of extension."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::SemanticVersion>,
    }
    impl ::google_field_selector::FieldSelector for BuildVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ClientConfig {
        #[doc = "Node for a particular client."]
        #[serde(
            rename = "node",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node: ::std::option::Option<crate::schemas::Node>,
        #[serde(
            rename = "xdsConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xds_config: ::std::option::Option<Vec<crate::schemas::PerXdsConfig>>,
    }
    impl ::google_field_selector::FieldSelector for ClientConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClientConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ClientStatusRequest {
        #[doc = "Management server can use these match criteria to identify clients. The match follows OR semantics."]
        #[serde(
            rename = "nodeMatchers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_matchers: ::std::option::Option<Vec<crate::schemas::NodeMatcher>>,
    }
    impl ::google_field_selector::FieldSelector for ClientStatusRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClientStatusRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ClientStatusResponse {
        #[doc = "Client configs for the clients specified in the ClientStatusRequest."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<Vec<crate::schemas::ClientConfig>>,
    }
    impl ::google_field_selector::FieldSelector for ClientStatusResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClientStatusResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ClustersConfigDump {
        #[doc = "The dynamically loaded active clusters. These are clusters that are available to service data plane traffic."]
        #[serde(
            rename = "dynamicActiveClusters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_active_clusters: ::std::option::Option<Vec<crate::schemas::DynamicCluster>>,
        #[doc = "The dynamically loaded warming clusters. These are clusters that are currently undergoing warming in preparation to service data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the warming clusters should generally be discarded."]
        #[serde(
            rename = "dynamicWarmingClusters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_warming_clusters: ::std::option::Option<Vec<crate::schemas::DynamicCluster>>,
        #[doc = "The statically loaded cluster configs."]
        #[serde(
            rename = "staticClusters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub static_clusters: ::std::option::Option<Vec<crate::schemas::StaticCluster>>,
        #[doc = "This is the :ref:`version_info ` in the last processed CDS discovery response. If there are only static bootstrap clusters, this field will be \"\"."]
        #[serde(
            rename = "versionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ClustersConfigDump {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ClustersConfigDump {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DoubleMatcher {
        #[doc = "If specified, the input double value must be equal to the value specified here."]
        #[serde(
            rename = "exact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact: ::std::option::Option<f64>,
        #[doc = "If specified, the input double value must be in the range specified here. Note: The range is using half-open interval semantics [start, end)."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::DoubleRange>,
    }
    impl ::google_field_selector::FieldSelector for DoubleMatcher {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoubleMatcher {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DoubleRange {
        #[doc = "end of the range (exclusive)"]
        #[serde(
            rename = "end",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end: ::std::option::Option<f64>,
        #[doc = "start of the range (inclusive)"]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for DoubleRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoubleRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DynamicCluster {
        #[doc = "The cluster config."]
        #[serde(
            rename = "cluster",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The timestamp when the Cluster was last updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the cluster was loaded. In the future, discrete per-cluster versions may be supported by the API."]
        #[serde(
            rename = "versionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DynamicCluster {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicCluster {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DynamicListener {
        #[doc = "The listener state for any active listener by this name. These are listeners that are available to service data plane traffic."]
        #[serde(
            rename = "activeState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub active_state: ::std::option::Option<crate::schemas::DynamicListenerState>,
        #[doc = "The listener state for any draining listener by this name. These are listeners that are currently undergoing draining in preparation to stop servicing data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the draining listeners should generally be discarded."]
        #[serde(
            rename = "drainingState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub draining_state: ::std::option::Option<crate::schemas::DynamicListenerState>,
        #[doc = "Set if the last update failed, cleared after the next successful update."]
        #[serde(
            rename = "errorState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_state: ::std::option::Option<crate::schemas::UpdateFailureState>,
        #[doc = "The name or unique id of this listener, pulled from the DynamicListenerState config."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The listener state for any warming listener by this name. These are listeners that are currently undergoing warming in preparation to service data plane traffic. Note that if attempting to recreate an Envoy configuration from a configuration dump, the warming listeners should generally be discarded."]
        #[serde(
            rename = "warmingState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warming_state: ::std::option::Option<crate::schemas::DynamicListenerState>,
    }
    impl ::google_field_selector::FieldSelector for DynamicListener {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicListener {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DynamicListenerState {
        #[doc = "The timestamp when the Listener was last successfully updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
        #[doc = "The listener config."]
        #[serde(
            rename = "listener",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub listener:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the listener was loaded. In the future, discrete per-listener versions may be supported by the API."]
        #[serde(
            rename = "versionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DynamicListenerState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicListenerState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DynamicRouteConfig {
        #[doc = "The timestamp when the Route was last updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
        #[doc = "The route config."]
        #[serde(
            rename = "routeConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub route_config:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the route configuration was loaded."]
        #[serde(
            rename = "versionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DynamicRouteConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicRouteConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DynamicScopedRouteConfigs {
        #[doc = "The timestamp when the scoped route config set was last updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
        #[doc = "The name assigned to the scoped route configurations."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The scoped route configurations."]
        #[serde(
            rename = "scopedRouteConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scoped_route_configs:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "This is the per-resource version information. This version is currently taken from the :ref:`version_info ` field at the time that the scoped routes configuration was loaded."]
        #[serde(
            rename = "versionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DynamicScopedRouteConfigs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicScopedRouteConfigs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Extension {
        #[doc = "Category of the extension. Extension category names use reverse DNS notation. For instance \"envoy.filters.listener\" for Envoy's built-in listener filters or \"com.acme.filters.http\" for HTTP filters from acme.com vendor. [#comment:"]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "Indicates that the extension is present but was disabled via dynamic configuration."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "This is the name of the Envoy filter as specified in the Envoy configuration, e.g. envoy.filters.http.router, com.acme.widget."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "[#not-implemented-hide:] Type descriptor of extension configuration proto. [#comment:"]
        #[serde(
            rename = "typeDescriptor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_descriptor: ::std::option::Option<String>,
        #[doc = "The version is a property of the extension and maintained independently of other extensions and the Envoy API. This field is not set when extension did not provide version information."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::BuildVersion>,
    }
    impl ::google_field_selector::FieldSelector for Extension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Extension {
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
    pub struct GoogleRE2 {
        #[doc = "This field controls the RE2 \"program size\" which is a rough estimate of how complex a compiled regex is to evaluate. A regex that has a program size greater than the configured value will fail to compile. In this case, the configured max program size can be increased or the regex can be simplified. If not specified, the default is 100. This field is deprecated; regexp validation should be performed on the management server instead of being done by each individual client."]
        #[serde(
            rename = "maxProgramSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_program_size: ::std::option::Option<u32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleRE2 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRE2 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct InlineScopedRouteConfigs {
        #[doc = "The timestamp when the scoped route config set was last updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
        #[doc = "The name assigned to the scoped route configurations."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The scoped route configurations."]
        #[serde(
            rename = "scopedRouteConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scoped_route_configs:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for InlineScopedRouteConfigs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InlineScopedRouteConfigs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListMatcher {
        #[doc = "If specified, at least one of the values in the list must match the value specified."]
        #[serde(
            rename = "oneOf",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub one_of: ::std::option::Option<Box<crate::schemas::ValueMatcher>>,
    }
    impl ::google_field_selector::FieldSelector for ListMatcher {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListMatcher {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListenersConfigDump {
        #[doc = "State for any warming, active, or draining listeners."]
        #[serde(
            rename = "dynamicListeners",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_listeners: ::std::option::Option<Vec<crate::schemas::DynamicListener>>,
        #[doc = "The statically loaded listener configs."]
        #[serde(
            rename = "staticListeners",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub static_listeners: ::std::option::Option<Vec<crate::schemas::StaticListener>>,
        #[doc = "This is the :ref:`version_info ` in the last processed LDS discovery response. If there are only static bootstrap listeners, this field will be \"\"."]
        #[serde(
            rename = "versionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListenersConfigDump {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListenersConfigDump {
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
    pub struct Locality {
        #[doc = "Region this :ref:`zone ` belongs to."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "When used for locality of upstream hosts, this field further splits zone into smaller chunks of sub-zones so they can be load balanced independently."]
        #[serde(
            rename = "subZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sub_zone: ::std::option::Option<String>,
        #[doc = "Defines the local service zone where Envoy is running. Though optional, it should be set if discovery service routing is used and the discovery service exposes :ref:`zone data `, either in this message or via :option:`--service-zone`. The meaning of zone is context dependent, e.g. `Availability Zone (AZ) `_ on AWS, `Zone `_ on GCP, etc."]
        #[serde(
            rename = "zone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Locality {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Locality {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Node {
        #[doc = "This is motivated by informing a management server during canary which version of Envoy is being tested in a heterogeneous fleet. This will be set by Envoy in management server RPCs. This field is deprecated in favor of the user_agent_name and user_agent_version values."]
        #[serde(
            rename = "buildVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_version: ::std::option::Option<String>,
        #[doc = "Client feature support list. These are well known features described in the Envoy API repository for a given major version of an API. Client features use reverse DNS naming scheme, for example `com.acme.feature`. See :ref:`the list of features ` that xDS client may support."]
        #[serde(
            rename = "clientFeatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_features: ::std::option::Option<Vec<String>>,
        #[doc = "Defines the local service cluster name where Envoy is running. Though optional, it should be set if any of the following features are used: :ref:`statsd `, :ref:`health check cluster verification `, :ref:`runtime override directory `, :ref:`user agent addition `, :ref:`HTTP global rate limiting `, :ref:`CDS `, and :ref:`HTTP tracing `, either in this message or via :option:`--service-cluster`."]
        #[serde(
            rename = "cluster",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster: ::std::option::Option<String>,
        #[doc = "List of extensions and their versions supported by the node."]
        #[serde(
            rename = "extensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extensions: ::std::option::Option<Vec<crate::schemas::Extension>>,
        #[doc = "An opaque node identifier for the Envoy node. This also provides the local service node name. It should be set if any of the following features are used: :ref:`statsd `, :ref:`CDS `, and :ref:`HTTP tracing `, either in this message or via :option:`--service-node`."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Known listening ports on the node as a generic hint to the management server for filtering :ref:`listeners ` to be returned. For example, if there is a listener bound to port 80, the list can optionally contain the SocketAddress `(0.0.0.0,80)`. The field is optional and just a hint."]
        #[serde(
            rename = "listeningAddresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub listening_addresses: ::std::option::Option<Vec<crate::schemas::Address>>,
        #[doc = "Locality specifying where the Envoy instance is running."]
        #[serde(
            rename = "locality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locality: ::std::option::Option<crate::schemas::Locality>,
        #[doc = "Opaque metadata extending the node identifier. Envoy will pass this directly to the management server."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Structured version of the entity requesting config."]
        #[serde(
            rename = "userAgentBuildVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent_build_version: ::std::option::Option<crate::schemas::BuildVersion>,
        #[doc = "Free-form string that identifies the entity requesting config. E.g. \"envoy\" or \"grpc\""]
        #[serde(
            rename = "userAgentName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent_name: ::std::option::Option<String>,
        #[doc = "Free-form string that identifies the version of the entity requesting config. E.g. \"1.12.2\" or \"abcd1234\", or \"SpecialEnvoyBuild\""]
        #[serde(
            rename = "userAgentVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Node {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Node {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NodeMatcher {
        #[doc = "Specifies match criteria on the node id."]
        #[serde(
            rename = "nodeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_id: ::std::option::Option<crate::schemas::StringMatcher>,
        #[doc = "Specifies match criteria on the node metadata."]
        #[serde(
            rename = "nodeMetadatas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub node_metadatas: ::std::option::Option<Vec<crate::schemas::StructMatcher>>,
    }
    impl ::google_field_selector::FieldSelector for NodeMatcher {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NodeMatcher {
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
    pub struct NullMatch {}
    impl ::google_field_selector::FieldSelector for NullMatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NullMatch {
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
    pub struct PathSegment {
        #[doc = "If specified, use the key to retrieve the value in a Struct."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PathSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct PerXdsConfig {
        #[serde(
            rename = "clusterConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_config: ::std::option::Option<crate::schemas::ClustersConfigDump>,
        #[serde(
            rename = "listenerConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub listener_config: ::std::option::Option<crate::schemas::ListenersConfigDump>,
        #[serde(
            rename = "routeConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub route_config: ::std::option::Option<crate::schemas::RoutesConfigDump>,
        #[serde(
            rename = "scopedRouteConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scoped_route_config: ::std::option::Option<crate::schemas::ScopedRoutesConfigDump>,
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::PerXdsConfigStatus>,
    }
    impl ::google_field_selector::FieldSelector for PerXdsConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerXdsConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PerXdsConfigStatus {
        #[doc = "Management server has sent the config to client but received NACK."]
        Error,
        #[doc = "Config is not sent."]
        NotSent,
        #[doc = "Management server has sent the config to client but hasn’t received ACK/NACK."]
        Stale,
        #[doc = "Management server has sent the config to client and received ACK."]
        Synced,
        #[doc = "Status info is not available/unknown."]
        Unknown,
    }
    impl PerXdsConfigStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                PerXdsConfigStatus::Error => "ERROR",
                PerXdsConfigStatus::NotSent => "NOT_SENT",
                PerXdsConfigStatus::Stale => "STALE",
                PerXdsConfigStatus::Synced => "SYNCED",
                PerXdsConfigStatus::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PerXdsConfigStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PerXdsConfigStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PerXdsConfigStatus, ()> {
            Ok(match s {
                "ERROR" => PerXdsConfigStatus::Error,
                "NOT_SENT" => PerXdsConfigStatus::NotSent,
                "STALE" => PerXdsConfigStatus::Stale,
                "SYNCED" => PerXdsConfigStatus::Synced,
                "UNKNOWN" => PerXdsConfigStatus::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PerXdsConfigStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PerXdsConfigStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PerXdsConfigStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => PerXdsConfigStatus::Error,
                "NOT_SENT" => PerXdsConfigStatus::NotSent,
                "STALE" => PerXdsConfigStatus::Stale,
                "SYNCED" => PerXdsConfigStatus::Synced,
                "UNKNOWN" => PerXdsConfigStatus::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PerXdsConfigStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerXdsConfigStatus {
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
    pub struct Pipe {
        #[doc = "The mode for the Pipe. Not applicable for abstract sockets."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<u32>,
        #[doc = "Unix Domain Socket path. On Linux, paths starting with '@' will use the abstract namespace. The starting '@' is replaced by a null byte by Envoy. Paths starting with '@' will result in an error in environments other than Linux."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Pipe {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Pipe {
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
    pub struct RegexMatcher {
        #[doc = "Google's RE2 regex engine."]
        #[serde(
            rename = "googleRe2",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_re_2: ::std::option::Option<crate::schemas::GoogleRE2>,
        #[doc = "The regex match string. The string must be supported by the configured engine."]
        #[serde(
            rename = "regex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regex: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RegexMatcher {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RegexMatcher {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct RoutesConfigDump {
        #[doc = "The dynamically loaded route configs."]
        #[serde(
            rename = "dynamicRouteConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_route_configs: ::std::option::Option<Vec<crate::schemas::DynamicRouteConfig>>,
        #[doc = "The statically loaded route configs."]
        #[serde(
            rename = "staticRouteConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub static_route_configs: ::std::option::Option<Vec<crate::schemas::StaticRouteConfig>>,
    }
    impl ::google_field_selector::FieldSelector for RoutesConfigDump {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RoutesConfigDump {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ScopedRoutesConfigDump {
        #[doc = "The dynamically loaded scoped route configs."]
        #[serde(
            rename = "dynamicScopedRouteConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_scoped_route_configs:
            ::std::option::Option<Vec<crate::schemas::DynamicScopedRouteConfigs>>,
        #[doc = "The statically loaded scoped route configs."]
        #[serde(
            rename = "inlineScopedRouteConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_scoped_route_configs:
            ::std::option::Option<Vec<crate::schemas::InlineScopedRouteConfigs>>,
    }
    impl ::google_field_selector::FieldSelector for ScopedRoutesConfigDump {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScopedRoutesConfigDump {
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
    pub struct SemanticVersion {
        #[serde(
            rename = "majorNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub major_number: ::std::option::Option<u32>,
        #[serde(
            rename = "minorNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minor_number: ::std::option::Option<u32>,
        #[serde(
            rename = "patch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub patch: ::std::option::Option<u32>,
    }
    impl ::google_field_selector::FieldSelector for SemanticVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SemanticVersion {
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
    pub struct SocketAddress {
        #[doc = "The address for this socket. :ref:`Listeners ` will bind to the address. An empty address is not allowed. Specify `0.0.0.0` or `::` to bind to any address. [#comment:TODO(zuercher) reinstate when implemented: It is possible to distinguish a Listener address via the prefix/suffix matching in :ref:`FilterChainMatch `.] When used within an upstream :ref:`BindConfig `, the address controls the source address of outbound connections. For :ref:`clusters `, the cluster type determines whether the address must be an IP (*STATIC* or *EDS* clusters) or a hostname resolved by DNS (*STRICT_DNS* or *LOGICAL_DNS* clusters). Address resolution can be customized via :ref:`resolver_name `."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<String>,
        #[doc = "When binding to an IPv6 address above, this enables `IPv4 compatibility `_. Binding to `::` will allow both IPv4 and IPv6 connections, with peer IPv4 addresses mapped into IPv6 space as `::FFFF:`."]
        #[serde(
            rename = "ipv4Compat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ipv_4_compat: ::std::option::Option<bool>,
        #[doc = "This is only valid if :ref:`resolver_name ` is specified below and the named resolver is capable of named port resolution."]
        #[serde(
            rename = "namedPort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_port: ::std::option::Option<String>,
        #[serde(
            rename = "portValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub port_value: ::std::option::Option<u32>,
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<crate::schemas::SocketAddressProtocol>,
        #[doc = "The name of the custom resolver. This must have been registered with Envoy. If this is empty, a context dependent default applies. If the address is a concrete IP address, no resolution will occur. If address is a hostname this should be set for resolution other than DNS. Specifying a custom resolver with *STRICT_DNS* or *LOGICAL_DNS* will generate an error at runtime."]
        #[serde(
            rename = "resolverName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolver_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SocketAddress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SocketAddress {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SocketAddressProtocol {
        Tcp,
        Udp,
    }
    impl SocketAddressProtocol {
        pub fn as_str(self) -> &'static str {
            match self {
                SocketAddressProtocol::Tcp => "TCP",
                SocketAddressProtocol::Udp => "UDP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SocketAddressProtocol {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SocketAddressProtocol {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SocketAddressProtocol, ()> {
            Ok(match s {
                "TCP" => SocketAddressProtocol::Tcp,
                "UDP" => SocketAddressProtocol::Udp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SocketAddressProtocol {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SocketAddressProtocol {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SocketAddressProtocol {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TCP" => SocketAddressProtocol::Tcp,
                "UDP" => SocketAddressProtocol::Udp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SocketAddressProtocol {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SocketAddressProtocol {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct StaticCluster {
        #[doc = "The cluster config."]
        #[serde(
            rename = "cluster",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The timestamp when the Cluster was last updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StaticCluster {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StaticCluster {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct StaticListener {
        #[doc = "The timestamp when the Listener was last successfully updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
        #[doc = "The listener config."]
        #[serde(
            rename = "listener",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub listener:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for StaticListener {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StaticListener {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct StaticRouteConfig {
        #[doc = "The timestamp when the Route was last updated."]
        #[serde(
            rename = "lastUpdated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updated: ::std::option::Option<String>,
        #[doc = "The route config."]
        #[serde(
            rename = "routeConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub route_config:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for StaticRouteConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StaticRouteConfig {
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
    pub struct StringMatcher {
        #[doc = "The input string must match exactly the string specified here. Examples: * *abc* only matches the value *abc*."]
        #[serde(
            rename = "exact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact: ::std::option::Option<String>,
        #[doc = "If true, indicates the exact/prefix/suffix matching should be case insensitive. This has no effect for the safe_regex match. For example, the matcher *data* will match both input string *Data* and *data* if set to true."]
        #[serde(
            rename = "ignoreCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ignore_case: ::std::option::Option<bool>,
        #[doc = "The input string must have the prefix specified here. Note: empty prefix is not allowed, please use regex instead. Examples: * *abc* matches the value *abc.xyz*"]
        #[serde(
            rename = "prefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prefix: ::std::option::Option<String>,
        #[doc = "The input string must match the regular expression specified here. The regex grammar is defined `here `_. Examples: * The regex `\\d{3}` matches the value *123* * The regex `\\d{3}` does not match the value *1234* * The regex `\\d{3}` does not match the value *123.456* .. attention:: This field has been deprecated in favor of `safe_regex` as it is not safe for use with untrusted input in all cases."]
        #[serde(
            rename = "regex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub regex: ::std::option::Option<String>,
        #[doc = "The input string must match the regular expression specified here."]
        #[serde(
            rename = "safeRegex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe_regex: ::std::option::Option<crate::schemas::RegexMatcher>,
        #[doc = "The input string must have the suffix specified here. Note: empty prefix is not allowed, please use regex instead. Examples: * *abc* matches the value *xyz.abc*"]
        #[serde(
            rename = "suffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suffix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StringMatcher {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringMatcher {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StructMatcher {
        #[doc = "The path to retrieve the Value from the Struct."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<Vec<crate::schemas::PathSegment>>,
        #[doc = "The StructMatcher is matched if the value retrieved by path is matched to this value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::ValueMatcher>,
    }
    impl ::google_field_selector::FieldSelector for StructMatcher {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StructMatcher {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct UpdateFailureState {
        #[doc = "Details about the last failed update attempt."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<String>,
        #[doc = "What the component configuration would have been if the update had succeeded."]
        #[serde(
            rename = "failedConfiguration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failed_configuration:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Time of the latest failed update attempt."]
        #[serde(
            rename = "lastUpdateAttempt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_attempt: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateFailureState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateFailureState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ValueMatcher {
        #[doc = "If specified, a match occurs if and only if the target value is a bool value and is equal to this field."]
        #[serde(
            rename = "boolMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bool_match: ::std::option::Option<bool>,
        #[doc = "If specified, a match occurs if and only if the target value is a double value and is matched to this field."]
        #[serde(
            rename = "doubleMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_match: ::std::option::Option<crate::schemas::DoubleMatcher>,
        #[doc = "If specified, a match occurs if and only if the target value is a list value and is matched to this field."]
        #[serde(
            rename = "listMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_match: ::std::option::Option<Box<crate::schemas::ListMatcher>>,
        #[doc = "If specified, a match occurs if and only if the target value is a NullValue."]
        #[serde(
            rename = "nullMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub null_match: ::std::option::Option<crate::schemas::NullMatch>,
        #[doc = "If specified, value match will be performed based on whether the path is referring to a valid primitive value in the metadata. If the path is referring to a non-primitive value, the result is always not matched."]
        #[serde(
            rename = "presentMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub present_match: ::std::option::Option<bool>,
        #[doc = "If specified, a match occurs if and only if the target value is a string value and is matched to this field."]
        #[serde(
            rename = "stringMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_match: ::std::option::Option<crate::schemas::StringMatcher>,
    }
    impl ::google_field_selector::FieldSelector for ValueMatcher {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueMatcher {
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
    #[doc = "Actions that can be performed on the discovery resource"]
    pub fn discovery(&self) -> crate::resources::discovery::DiscoveryActions {
        crate::resources::discovery::DiscoveryActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod discovery {
        pub mod params {}
        pub struct DiscoveryActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DiscoveryActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = ""]
            pub fn client_status(
                &self,
                request: crate::schemas::ClientStatusRequest,
            ) -> ClientStatusRequestBuilder {
                ClientStatusRequestBuilder {
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
                }
            }
        }
        #[doc = "Created via [DiscoveryActions::client_status()](struct.DiscoveryActions.html#method.client_status)"]
        #[derive(Debug, Clone)]
        pub struct ClientStatusRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ClientStatusRequest,
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
        impl<'a> ClientStatusRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ClientStatusResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ClientStatusResponse, crate::Error> {
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
                let mut output = "https://trafficdirector.googleapis.com/".to_owned();
                output.push_str("v2/discovery:client_status");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
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
