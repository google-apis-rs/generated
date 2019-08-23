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
    pub struct AndroidAppInfo {
        #[doc = "The name of the app. Optional"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The package name of the app. Required."]
        #[serde(rename = "packageName", default)]
        pub package_name: ::std::option::Option<String>,
        #[doc = "The internal version code of the app. Optional."]
        #[serde(rename = "versionCode", default)]
        pub version_code: ::std::option::Option<String>,
        #[doc = "The version name of the app. Optional."]
        #[serde(rename = "versionName", default)]
        pub version_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AndroidAppInfo {
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
    pub struct AndroidInstrumentationTest {
        #[doc = "The java package for the test to be executed. Required"]
        #[serde(rename = "testPackageId", default)]
        pub test_package_id: ::std::option::Option<String>,
        #[doc = "The InstrumentationTestRunner class. Required"]
        #[serde(rename = "testRunnerClass", default)]
        pub test_runner_class: ::std::option::Option<String>,
        #[doc = "Each target must be fully qualified with the package name or class name, in one of these formats: - \"package package_name\" - \"class package_name.class_name\" - \"class package_name.class_name#method_name\"\n\nIf empty, all targets in the module will be run."]
        #[serde(rename = "testTargets", default)]
        pub test_targets: ::std::option::Option<Vec<String>>,
        #[doc = "The flag indicates whether Android Test Orchestrator will be used to run test or not."]
        #[serde(rename = "useOrchestrator", default)]
        pub use_orchestrator: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for AndroidInstrumentationTest {
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
    pub struct AndroidRoboTest {
        #[doc = "The initial activity that should be used to start the app. Optional"]
        #[serde(rename = "appInitialActivity", default)]
        pub app_initial_activity: ::std::option::Option<String>,
        #[doc = "The java package for the bootstrap. Optional"]
        #[serde(rename = "bootstrapPackageId", default)]
        pub bootstrap_package_id: ::std::option::Option<String>,
        #[doc = "The runner class for the bootstrap. Optional"]
        #[serde(rename = "bootstrapRunnerClass", default)]
        pub bootstrap_runner_class: ::std::option::Option<String>,
        #[doc = "The max depth of the traversal stack Robo can explore. Optional"]
        #[serde(rename = "maxDepth", default)]
        pub max_depth: ::std::option::Option<i32>,
        #[doc = "The max number of steps/actions Robo can execute. Default is no limit (0). Optional"]
        #[serde(rename = "maxSteps", default)]
        pub max_steps: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for AndroidRoboTest {
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
    pub struct AndroidTest {
        #[doc = "Information about the application under test."]
        #[serde(rename = "androidAppInfo", default)]
        pub android_app_info: ::std::option::Option<crate::schemas::AndroidAppInfo>,
        #[doc = "An Android instrumentation test."]
        #[serde(rename = "androidInstrumentationTest", default)]
        pub android_instrumentation_test:
            ::std::option::Option<crate::schemas::AndroidInstrumentationTest>,
        #[doc = "An Android robo test."]
        #[serde(rename = "androidRoboTest", default)]
        pub android_robo_test: ::std::option::Option<crate::schemas::AndroidRoboTest>,
        #[doc = "Max time a test is allowed to run before it is automatically cancelled."]
        #[serde(rename = "testTimeout", default)]
        pub test_timeout: ::std::option::Option<crate::schemas::Duration>,
    }
    impl ::field_selector::FieldSelector for AndroidTest {
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
    pub struct Any {
        #[doc = "A URL/resource name that uniquely identifies the type of the serialized protocol buffer message. This string must contain at least one \"/\" character. The last segment of the URL's path must represent the fully qualified name of the type (as in `path/google.protobuf.Duration`). The name should be in a canonical form (e.g., leading \".\" is not accepted).\n\nIn practice, teams usually precompile into the binary all types that they expect it to use in the context of Any. However, for URLs which use the scheme `http`, `https`, or no scheme, one can optionally set up a type server that maps type URLs to message definitions as follows:\n\n* If no scheme is provided, `https` is assumed. * An HTTP GET on the URL must yield a [google.protobuf.Type][] value in binary format, or produce an error. * Applications are allowed to cache lookup results based on the URL, or have them precompiled into a binary to avoid any lookup. Therefore, binary compatibility needs to be preserved on changes to types. (Use versioned type names to manage breaking changes.)\n\nNote: this functionality is not currently available in the official protobuf release, and it is not used for type URLs beginning with type.googleapis.com.\n\nSchemes other than `http`, `https` (or the empty scheme) might be used with implementation specific semantics."]
        #[serde(rename = "typeUrl", default)]
        pub type_url: ::std::option::Option<String>,
        #[doc = "Must be a valid serialized protocol buffer of the above specified type."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::field_selector::FieldSelector for Any {
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
    pub struct AppStartTime {
        #[doc = "Optional. The time from app start to reaching the developer-reported \"fully drawn\" time. This is only stored if the app includes a call to Activity.reportFullyDrawn(). See https://developer.android.com/topic/performance/launch-time.html#time-full"]
        #[serde(rename = "fullyDrawnTime", default)]
        pub fully_drawn_time: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "The time from app start to the first displayed activity being drawn, as reported in Logcat. See https://developer.android.com/topic/performance/launch-time.html#time-initial"]
        #[serde(rename = "initialDisplayTime", default)]
        pub initial_display_time: ::std::option::Option<crate::schemas::Duration>,
    }
    impl ::field_selector::FieldSelector for AppStartTime {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicPerfSampleSeriesPerfMetricType {
        Cpu,
        Graphics,
        Memory,
        Network,
        PerfMetricTypeUnspecified,
    }
    impl BasicPerfSampleSeriesPerfMetricType {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicPerfSampleSeriesPerfMetricType::Cpu => "cpu",
                BasicPerfSampleSeriesPerfMetricType::Graphics => "graphics",
                BasicPerfSampleSeriesPerfMetricType::Memory => "memory",
                BasicPerfSampleSeriesPerfMetricType::Network => "network",
                BasicPerfSampleSeriesPerfMetricType::PerfMetricTypeUnspecified => {
                    "perfMetricTypeUnspecified"
                }
            }
        }
    }
    impl ::std::fmt::Display for BasicPerfSampleSeriesPerfMetricType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicPerfSampleSeriesPerfMetricType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicPerfSampleSeriesPerfMetricType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cpu" => BasicPerfSampleSeriesPerfMetricType::Cpu,
                "graphics" => BasicPerfSampleSeriesPerfMetricType::Graphics,
                "memory" => BasicPerfSampleSeriesPerfMetricType::Memory,
                "network" => BasicPerfSampleSeriesPerfMetricType::Network,
                "perfMetricTypeUnspecified" => {
                    BasicPerfSampleSeriesPerfMetricType::PerfMetricTypeUnspecified
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
    impl ::field_selector::FieldSelector for BasicPerfSampleSeriesPerfMetricType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicPerfSampleSeriesPerfUnit {
        Byte,
        BytesPerSecond,
        FramesPerSecond,
        Kibibyte,
        Percent,
        PerfUnitUnspecified,
    }
    impl BasicPerfSampleSeriesPerfUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicPerfSampleSeriesPerfUnit::Byte => "byte",
                BasicPerfSampleSeriesPerfUnit::BytesPerSecond => "bytesPerSecond",
                BasicPerfSampleSeriesPerfUnit::FramesPerSecond => "framesPerSecond",
                BasicPerfSampleSeriesPerfUnit::Kibibyte => "kibibyte",
                BasicPerfSampleSeriesPerfUnit::Percent => "percent",
                BasicPerfSampleSeriesPerfUnit::PerfUnitUnspecified => "perfUnitUnspecified",
            }
        }
    }
    impl ::std::fmt::Display for BasicPerfSampleSeriesPerfUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicPerfSampleSeriesPerfUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicPerfSampleSeriesPerfUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "byte" => BasicPerfSampleSeriesPerfUnit::Byte,
                "bytesPerSecond" => BasicPerfSampleSeriesPerfUnit::BytesPerSecond,
                "framesPerSecond" => BasicPerfSampleSeriesPerfUnit::FramesPerSecond,
                "kibibyte" => BasicPerfSampleSeriesPerfUnit::Kibibyte,
                "percent" => BasicPerfSampleSeriesPerfUnit::Percent,
                "perfUnitUnspecified" => BasicPerfSampleSeriesPerfUnit::PerfUnitUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for BasicPerfSampleSeriesPerfUnit {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BasicPerfSampleSeriesSampleSeriesLabel {
        CpuKernel,
        CpuTotal,
        CpuUser,
        GraphicsFrameRate,
        MemoryRssPrivate,
        MemoryRssShared,
        MemoryRssTotal,
        MemoryTotal,
        NetworkReceived,
        NetworkSent,
        NtBytesReceived,
        NtBytesTransferred,
        SampleSeriesTypeUnspecified,
    }
    impl BasicPerfSampleSeriesSampleSeriesLabel {
        pub fn as_str(self) -> &'static str {
            match self {
                BasicPerfSampleSeriesSampleSeriesLabel::CpuKernel => "cpuKernel",
                BasicPerfSampleSeriesSampleSeriesLabel::CpuTotal => "cpuTotal",
                BasicPerfSampleSeriesSampleSeriesLabel::CpuUser => "cpuUser",
                BasicPerfSampleSeriesSampleSeriesLabel::GraphicsFrameRate => "graphicsFrameRate",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssPrivate => "memoryRssPrivate",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssShared => "memoryRssShared",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssTotal => "memoryRssTotal",
                BasicPerfSampleSeriesSampleSeriesLabel::MemoryTotal => "memoryTotal",
                BasicPerfSampleSeriesSampleSeriesLabel::NetworkReceived => "networkReceived",
                BasicPerfSampleSeriesSampleSeriesLabel::NetworkSent => "networkSent",
                BasicPerfSampleSeriesSampleSeriesLabel::NtBytesReceived => "ntBytesReceived",
                BasicPerfSampleSeriesSampleSeriesLabel::NtBytesTransferred => "ntBytesTransferred",
                BasicPerfSampleSeriesSampleSeriesLabel::SampleSeriesTypeUnspecified => {
                    "sampleSeriesTypeUnspecified"
                }
            }
        }
    }
    impl ::std::fmt::Display for BasicPerfSampleSeriesSampleSeriesLabel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BasicPerfSampleSeriesSampleSeriesLabel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BasicPerfSampleSeriesSampleSeriesLabel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cpuKernel" => BasicPerfSampleSeriesSampleSeriesLabel::CpuKernel,
                "cpuTotal" => BasicPerfSampleSeriesSampleSeriesLabel::CpuTotal,
                "cpuUser" => BasicPerfSampleSeriesSampleSeriesLabel::CpuUser,
                "graphicsFrameRate" => BasicPerfSampleSeriesSampleSeriesLabel::GraphicsFrameRate,
                "memoryRssPrivate" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssPrivate,
                "memoryRssShared" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssShared,
                "memoryRssTotal" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryRssTotal,
                "memoryTotal" => BasicPerfSampleSeriesSampleSeriesLabel::MemoryTotal,
                "networkReceived" => BasicPerfSampleSeriesSampleSeriesLabel::NetworkReceived,
                "networkSent" => BasicPerfSampleSeriesSampleSeriesLabel::NetworkSent,
                "ntBytesReceived" => BasicPerfSampleSeriesSampleSeriesLabel::NtBytesReceived,
                "ntBytesTransferred" => BasicPerfSampleSeriesSampleSeriesLabel::NtBytesTransferred,
                "sampleSeriesTypeUnspecified" => {
                    BasicPerfSampleSeriesSampleSeriesLabel::SampleSeriesTypeUnspecified
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
    impl ::field_selector::FieldSelector for BasicPerfSampleSeriesSampleSeriesLabel {
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
    pub struct BasicPerfSampleSeries {
        #[serde(rename = "perfMetricType", default)]
        pub perf_metric_type:
            ::std::option::Option<crate::schemas::BasicPerfSampleSeriesPerfMetricType>,
        #[serde(rename = "perfUnit", default)]
        pub perf_unit: ::std::option::Option<crate::schemas::BasicPerfSampleSeriesPerfUnit>,
        #[serde(rename = "sampleSeriesLabel", default)]
        pub sample_series_label:
            ::std::option::Option<crate::schemas::BasicPerfSampleSeriesSampleSeriesLabel>,
    }
    impl ::field_selector::FieldSelector for BasicPerfSampleSeries {
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
    pub struct BatchCreatePerfSamplesRequest {
        #[doc = "The set of PerfSamples to create should not include existing timestamps"]
        #[serde(rename = "perfSamples", default)]
        pub perf_samples: ::std::option::Option<Vec<crate::schemas::PerfSample>>,
    }
    impl ::field_selector::FieldSelector for BatchCreatePerfSamplesRequest {
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
    pub struct BatchCreatePerfSamplesResponse {
        #[serde(rename = "perfSamples", default)]
        pub perf_samples: ::std::option::Option<Vec<crate::schemas::PerfSample>>,
    }
    impl ::field_selector::FieldSelector for BatchCreatePerfSamplesResponse {
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
    pub struct Cpuinfo {
        #[doc = "description of the device processor ie '1.8 GHz hexa core 64-bit ARMv8-A'"]
        #[serde(rename = "cpuProcessor", default)]
        pub cpu_processor: ::std::option::Option<String>,
        #[doc = "the CPU clock speed in GHz"]
        #[serde(rename = "cpuSpeedInGhz", default)]
        pub cpu_speed_in_ghz: ::std::option::Option<f32>,
        #[doc = "the number of CPU cores"]
        #[serde(rename = "numberOfCores", default)]
        pub number_of_cores: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for Cpuinfo {
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
    pub struct Duration {
        #[doc = "Signed fractions of a second at nanosecond resolution of the span of time. Durations less than one second are represented with a 0 `seconds` field and a positive or negative `nanos` field. For durations of one second or more, a non-zero value for the `nanos` field must be of the same sign as the `seconds` field. Must be from -999,999,999 to +999,999,999 inclusive."]
        #[serde(rename = "nanos", default)]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Signed seconds of the span of time. Must be from -315,576,000,000 to +315,576,000,000 inclusive. Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
        #[serde(rename = "seconds", default)]
        #[serde(with = "crate::parsed_string")]
        pub seconds: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for Duration {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExecutionState {
        Complete,
        InProgress,
        Pending,
        UnknownState,
    }
    impl ExecutionState {
        pub fn as_str(self) -> &'static str {
            match self {
                ExecutionState::Complete => "complete",
                ExecutionState::InProgress => "inProgress",
                ExecutionState::Pending => "pending",
                ExecutionState::UnknownState => "unknownState",
            }
        }
    }
    impl ::std::fmt::Display for ExecutionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExecutionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExecutionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "complete" => ExecutionState::Complete,
                "inProgress" => ExecutionState::InProgress,
                "pending" => ExecutionState::Pending,
                "unknownState" => ExecutionState::UnknownState,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ExecutionState {
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
    pub struct Execution {
        #[doc = "The time when the Execution status transitioned to COMPLETE.\n\nThis value will be set automatically when state transitions to COMPLETE.\n\n* In response: set if the execution state is COMPLETE. - In create/update request: never set"]
        #[serde(rename = "completionTime", default)]
        pub completion_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "The time when the Execution was created.\n\nThis value will be set automatically when CreateExecution is called.\n\n* In response: always set - In create/update request: never set"]
        #[serde(rename = "creationTime", default)]
        pub creation_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "A unique identifier within a History for this Execution.\n\nReturns INVALID_ARGUMENT if this field is set or overwritten by the caller.\n\n* In response always set - In create/update request: never set"]
        #[serde(rename = "executionId", default)]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "Classify the result, for example into SUCCESS or FAILURE\n\n* In response: present if set by create/update request - In create/update request: optional"]
        #[serde(rename = "outcome", default)]
        pub outcome: ::std::option::Option<crate::schemas::Outcome>,
        #[doc = "Lightweight information about execution request.\n\n* In response: present if set by create - In create: optional - In update: optional"]
        #[serde(rename = "specification", default)]
        pub specification: ::std::option::Option<crate::schemas::Specification>,
        #[doc = "The initial state is IN_PROGRESS.\n\nThe only legal state transitions is from IN_PROGRESS to COMPLETE.\n\nA PRECONDITION_FAILED will be returned if an invalid transition is requested.\n\nThe state can only be set to COMPLETE once. A FAILED_PRECONDITION will be returned if the state is set to COMPLETE multiple times.\n\nIf the state is set to COMPLETE, all the in-progress steps within the execution will be set as COMPLETE. If the outcome of the step is not set, the outcome will be set to INCONCLUSIVE.\n\n* In response always set - In create/update request: optional"]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::ExecutionState>,
        #[doc = "TestExecution Matrix ID that the TestExecutionService uses.\n\n* In response: present if set by create - In create: optional - In update: never set"]
        #[serde(rename = "testExecutionMatrixId", default)]
        pub test_execution_matrix_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Execution {
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
    pub struct FailureDetail {
        #[doc = "If the failure was severe because the system (app) under test crashed."]
        #[serde(rename = "crashed", default)]
        pub crashed: ::std::option::Option<bool>,
        #[doc = "If an app is not installed and thus no test can be run with the app. This might be caused by trying to run a test on an unsupported platform."]
        #[serde(rename = "notInstalled", default)]
        pub not_installed: ::std::option::Option<bool>,
        #[doc = "If a native process (including any other than the app) crashed."]
        #[serde(rename = "otherNativeCrash", default)]
        pub other_native_crash: ::std::option::Option<bool>,
        #[doc = "If the test overran some time limit, and that is why it failed."]
        #[serde(rename = "timedOut", default)]
        pub timed_out: ::std::option::Option<bool>,
        #[doc = "If the robo was unable to crawl the app; perhaps because the app did not start."]
        #[serde(rename = "unableToCrawl", default)]
        pub unable_to_crawl: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for FailureDetail {
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
    pub struct FileReference {
        #[doc = "The URI of a file stored in Google Cloud Storage.\n\nFor example: http://storage.googleapis.com/mybucket/path/to/test.xml or in gsutil format: gs://mybucket/path/to/test.xml with version-specific info, gs://mybucket/path/to/test.xml#1360383693690000\n\nAn INVALID_ARGUMENT error will be returned if the URI format is not supported.\n\n* In response: always set - In create/update request: always set"]
        #[serde(rename = "fileUri", default)]
        pub file_uri: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for FileReference {
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
    pub struct GraphicsStats {
        #[doc = "Histogram of frame render times. There should be 154 buckets ranging from [5ms, 6ms) to [4950ms, infinity)"]
        #[serde(rename = "buckets", default)]
        pub buckets: ::std::option::Option<Vec<crate::schemas::GraphicsStatsBucket>>,
        #[doc = "Total \"high input latency\" events."]
        #[serde(rename = "highInputLatencyCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub high_input_latency_count: ::std::option::Option<i64>,
        #[doc = "Total frames with slow render time. Should be <= total_frames."]
        #[serde(rename = "jankyFrames", default)]
        #[serde(with = "crate::parsed_string")]
        pub janky_frames: ::std::option::Option<i64>,
        #[doc = "Total \"missed vsync\" events."]
        #[serde(rename = "missedVsyncCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub missed_vsync_count: ::std::option::Option<i64>,
        #[doc = "50th percentile frame render time in milliseconds."]
        #[serde(rename = "p50Millis", default)]
        #[serde(with = "crate::parsed_string")]
        pub p_50_millis: ::std::option::Option<i64>,
        #[doc = "90th percentile frame render time in milliseconds."]
        #[serde(rename = "p90Millis", default)]
        #[serde(with = "crate::parsed_string")]
        pub p_90_millis: ::std::option::Option<i64>,
        #[doc = "95th percentile frame render time in milliseconds."]
        #[serde(rename = "p95Millis", default)]
        #[serde(with = "crate::parsed_string")]
        pub p_95_millis: ::std::option::Option<i64>,
        #[doc = "99th percentile frame render time in milliseconds."]
        #[serde(rename = "p99Millis", default)]
        #[serde(with = "crate::parsed_string")]
        pub p_99_millis: ::std::option::Option<i64>,
        #[doc = "Total \"slow bitmap upload\" events."]
        #[serde(rename = "slowBitmapUploadCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub slow_bitmap_upload_count: ::std::option::Option<i64>,
        #[doc = "Total \"slow draw\" events."]
        #[serde(rename = "slowDrawCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub slow_draw_count: ::std::option::Option<i64>,
        #[doc = "Total \"slow UI thread\" events."]
        #[serde(rename = "slowUiThreadCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub slow_ui_thread_count: ::std::option::Option<i64>,
        #[doc = "Total frames rendered by package."]
        #[serde(rename = "totalFrames", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_frames: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for GraphicsStats {
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
    pub struct GraphicsStatsBucket {
        #[doc = "Number of frames in the bucket."]
        #[serde(rename = "frameCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub frame_count: ::std::option::Option<i64>,
        #[doc = "Lower bound of render time in milliseconds."]
        #[serde(rename = "renderMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub render_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for GraphicsStatsBucket {
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
    pub struct History {
        #[doc = "A short human-readable (plain text) name to display in the UI. Maximum of 100 characters.\n\n* In response: present if set during create. - In create request: optional"]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A unique identifier within a project for this History.\n\nReturns INVALID_ARGUMENT if this field is set or overwritten by the caller.\n\n* In response always set - In create request: never set"]
        #[serde(rename = "historyId", default)]
        pub history_id: ::std::option::Option<String>,
        #[doc = "A name to uniquely identify a history within a project. Maximum of 200 characters.\n\n* In response always set - In create request: always set"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for History {
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
    pub struct Image {
        #[doc = "An error explaining why the thumbnail could not be rendered."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "A reference to the full-size, original image.\n\nThis is the same as the tool_outputs entry for the image under its Step.\n\nAlways set."]
        #[serde(rename = "sourceImage", default)]
        pub source_image: ::std::option::Option<crate::schemas::ToolOutputReference>,
        #[doc = "The step to which the image is attached.\n\nAlways set."]
        #[serde(rename = "stepId", default)]
        pub step_id: ::std::option::Option<String>,
        #[doc = "The thumbnail."]
        #[serde(rename = "thumbnail", default)]
        pub thumbnail: ::std::option::Option<crate::schemas::Thumbnail>,
    }
    impl ::field_selector::FieldSelector for Image {
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
    pub struct InconclusiveDetail {
        #[doc = "If the end user aborted the test execution before a pass or fail could be determined. For example, the user pressed ctrl-c which sent a kill signal to the test runner while the test was running."]
        #[serde(rename = "abortedByUser", default)]
        pub aborted_by_user: ::std::option::Option<bool>,
        #[doc = "If results are being provided to the user in certain cases of infrastructure failures"]
        #[serde(rename = "hasErrorLogs", default)]
        pub has_error_logs: ::std::option::Option<bool>,
        #[doc = "If the test runner could not determine success or failure because the test depends on a component other than the system under test which failed.\n\nFor example, a mobile test requires provisioning a device where the test executes, and that provisioning can fail."]
        #[serde(rename = "infrastructureFailure", default)]
        pub infrastructure_failure: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for InconclusiveDetail {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IndividualOutcomeOutcomeSummary {
        Failure,
        Flaky,
        Inconclusive,
        Skipped,
        Success,
        Unset,
    }
    impl IndividualOutcomeOutcomeSummary {
        pub fn as_str(self) -> &'static str {
            match self {
                IndividualOutcomeOutcomeSummary::Failure => "failure",
                IndividualOutcomeOutcomeSummary::Flaky => "flaky",
                IndividualOutcomeOutcomeSummary::Inconclusive => "inconclusive",
                IndividualOutcomeOutcomeSummary::Skipped => "skipped",
                IndividualOutcomeOutcomeSummary::Success => "success",
                IndividualOutcomeOutcomeSummary::Unset => "unset",
            }
        }
    }
    impl ::std::fmt::Display for IndividualOutcomeOutcomeSummary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IndividualOutcomeOutcomeSummary {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IndividualOutcomeOutcomeSummary {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "failure" => IndividualOutcomeOutcomeSummary::Failure,
                "flaky" => IndividualOutcomeOutcomeSummary::Flaky,
                "inconclusive" => IndividualOutcomeOutcomeSummary::Inconclusive,
                "skipped" => IndividualOutcomeOutcomeSummary::Skipped,
                "success" => IndividualOutcomeOutcomeSummary::Success,
                "unset" => IndividualOutcomeOutcomeSummary::Unset,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for IndividualOutcomeOutcomeSummary {
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
    pub struct IndividualOutcome {
        #[doc = "Unique int given to each step. Ranges from 0(inclusive) to total number of steps(exclusive). The primary step is 0."]
        #[serde(rename = "multistepNumber", default)]
        pub multistep_number: ::std::option::Option<i32>,
        #[serde(rename = "outcomeSummary", default)]
        pub outcome_summary: ::std::option::Option<crate::schemas::IndividualOutcomeOutcomeSummary>,
        #[doc = "How long it took for this step to run."]
        #[serde(rename = "runDuration", default)]
        pub run_duration: ::std::option::Option<crate::schemas::Duration>,
        #[serde(rename = "stepId", default)]
        pub step_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for IndividualOutcome {
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
    pub struct ListExecutionsResponse {
        #[doc = "Executions.\n\nAlways set."]
        #[serde(rename = "executions", default)]
        pub executions: ::std::option::Option<Vec<crate::schemas::Execution>>,
        #[doc = "A continuation token to resume the query at the next item.\n\nWill only be set if there are more Executions to fetch."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ListExecutionsResponse {
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
    pub struct ListHistoriesResponse {
        #[doc = "Histories."]
        #[serde(rename = "histories", default)]
        pub histories: ::std::option::Option<Vec<crate::schemas::History>>,
        #[doc = "A continuation token to resume the query at the next item.\n\nWill only be set if there are more histories to fetch.\n\nTokens are valid for up to one hour from the time of the first list request. For instance, if you make a list request at 1PM and use the token from this first request 10 minutes later, the token from this second response will only be valid for 50 minutes."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ListHistoriesResponse {
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
    pub struct ListPerfSampleSeriesResponse {
        #[doc = "The resulting PerfSampleSeries sorted by id"]
        #[serde(rename = "perfSampleSeries", default)]
        pub perf_sample_series: ::std::option::Option<Vec<crate::schemas::PerfSampleSeries>>,
    }
    impl ::field_selector::FieldSelector for ListPerfSampleSeriesResponse {
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
    pub struct ListPerfSamplesResponse {
        #[doc = "Optional, returned if result size exceeds the page size specified in the request (or the default page size, 500, if unspecified). It indicates the last sample timestamp to be used as page_token in subsequent request"]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(rename = "perfSamples", default)]
        pub perf_samples: ::std::option::Option<Vec<crate::schemas::PerfSample>>,
    }
    impl ::field_selector::FieldSelector for ListPerfSamplesResponse {
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
    pub struct ListScreenshotClustersResponse {
        #[doc = "The set of clusters associated with an execution Always set"]
        #[serde(rename = "clusters", default)]
        pub clusters: ::std::option::Option<Vec<crate::schemas::ScreenshotCluster>>,
    }
    impl ::field_selector::FieldSelector for ListScreenshotClustersResponse {
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
    pub struct ListStepThumbnailsResponse {
        #[doc = "A continuation token to resume the query at the next item.\n\nIf set, indicates that there are more thumbnails to read, by calling list again with this value in the page_token field."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of image data.\n\nImages are returned in a deterministic order; they are ordered by these factors, in order of importance: * First, by their associated test case. Images without a test case are considered greater than images with one. * Second, by their creation time. Images without a creation time are greater than images with one. * Third, by the order in which they were added to the step (by calls to CreateStep or UpdateStep)."]
        #[serde(rename = "thumbnails", default)]
        pub thumbnails: ::std::option::Option<Vec<crate::schemas::Image>>,
    }
    impl ::field_selector::FieldSelector for ListStepThumbnailsResponse {
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
    pub struct ListStepsResponse {
        #[doc = "A continuation token to resume the query at the next item.\n\nIf set, indicates that there are more steps to read, by calling list again with this value in the page_token field."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Steps."]
        #[serde(rename = "steps", default)]
        pub steps: ::std::option::Option<Vec<crate::schemas::Step>>,
    }
    impl ::field_selector::FieldSelector for ListStepsResponse {
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
    pub struct ListTestCasesResponse {
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of test cases."]
        #[serde(rename = "testCases", default)]
        pub test_cases: ::std::option::Option<Vec<crate::schemas::TestCase>>,
    }
    impl ::field_selector::FieldSelector for ListTestCasesResponse {
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
    pub struct MemoryInfo {
        #[doc = "Maximum memory that can be allocated to the process in KiB"]
        #[serde(rename = "memoryCapInKibibyte", default)]
        #[serde(with = "crate::parsed_string")]
        pub memory_cap_in_kibibyte: ::std::option::Option<i64>,
        #[doc = "Total memory available on the device in KiB"]
        #[serde(rename = "memoryTotalInKibibyte", default)]
        #[serde(with = "crate::parsed_string")]
        pub memory_total_in_kibibyte: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for MemoryInfo {
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
    pub struct MultiStep {
        #[doc = "Unique int given to each step. Ranges from 0(inclusive) to total number of steps(exclusive). The primary step is 0."]
        #[serde(rename = "multistepNumber", default)]
        pub multistep_number: ::std::option::Option<i32>,
        #[doc = "Present if it is a primary (original) step."]
        #[serde(rename = "primaryStep", default)]
        pub primary_step: ::std::option::Option<crate::schemas::PrimaryStep>,
        #[doc = "Step Id of the primary (original) step, which might be this step."]
        #[serde(rename = "primaryStepId", default)]
        pub primary_step_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MultiStep {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OutcomeSummary {
        Failure,
        Flaky,
        Inconclusive,
        Skipped,
        Success,
        Unset,
    }
    impl OutcomeSummary {
        pub fn as_str(self) -> &'static str {
            match self {
                OutcomeSummary::Failure => "failure",
                OutcomeSummary::Flaky => "flaky",
                OutcomeSummary::Inconclusive => "inconclusive",
                OutcomeSummary::Skipped => "skipped",
                OutcomeSummary::Success => "success",
                OutcomeSummary::Unset => "unset",
            }
        }
    }
    impl ::std::fmt::Display for OutcomeSummary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OutcomeSummary {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OutcomeSummary {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "failure" => OutcomeSummary::Failure,
                "flaky" => OutcomeSummary::Flaky,
                "inconclusive" => OutcomeSummary::Inconclusive,
                "skipped" => OutcomeSummary::Skipped,
                "success" => OutcomeSummary::Success,
                "unset" => OutcomeSummary::Unset,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for OutcomeSummary {
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
    pub struct Outcome {
        #[doc = "More information about a FAILURE outcome.\n\nReturns INVALID_ARGUMENT if this field is set but the summary is not FAILURE.\n\nOptional"]
        #[serde(rename = "failureDetail", default)]
        pub failure_detail: ::std::option::Option<crate::schemas::FailureDetail>,
        #[doc = "More information about an INCONCLUSIVE outcome.\n\nReturns INVALID_ARGUMENT if this field is set but the summary is not INCONCLUSIVE.\n\nOptional"]
        #[serde(rename = "inconclusiveDetail", default)]
        pub inconclusive_detail: ::std::option::Option<crate::schemas::InconclusiveDetail>,
        #[doc = "More information about a SKIPPED outcome.\n\nReturns INVALID_ARGUMENT if this field is set but the summary is not SKIPPED.\n\nOptional"]
        #[serde(rename = "skippedDetail", default)]
        pub skipped_detail: ::std::option::Option<crate::schemas::SkippedDetail>,
        #[doc = "More information about a SUCCESS outcome.\n\nReturns INVALID_ARGUMENT if this field is set but the summary is not SUCCESS.\n\nOptional"]
        #[serde(rename = "successDetail", default)]
        pub success_detail: ::std::option::Option<crate::schemas::SuccessDetail>,
        #[doc = "The simplest way to interpret a result.\n\nRequired"]
        #[serde(rename = "summary", default)]
        pub summary: ::std::option::Option<crate::schemas::OutcomeSummary>,
    }
    impl ::field_selector::FieldSelector for Outcome {
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
    pub struct PerfEnvironment {
        #[doc = "CPU related environment info"]
        #[serde(rename = "cpuInfo", default)]
        pub cpu_info: ::std::option::Option<crate::schemas::Cpuinfo>,
        #[doc = "Memory related environment info"]
        #[serde(rename = "memoryInfo", default)]
        pub memory_info: ::std::option::Option<crate::schemas::MemoryInfo>,
    }
    impl ::field_selector::FieldSelector for PerfEnvironment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PerfMetricsSummaryPerfMetricsItems {
        Cpu,
        Graphics,
        Memory,
        Network,
        PerfMetricTypeUnspecified,
    }
    impl PerfMetricsSummaryPerfMetricsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PerfMetricsSummaryPerfMetricsItems::Cpu => "cpu",
                PerfMetricsSummaryPerfMetricsItems::Graphics => "graphics",
                PerfMetricsSummaryPerfMetricsItems::Memory => "memory",
                PerfMetricsSummaryPerfMetricsItems::Network => "network",
                PerfMetricsSummaryPerfMetricsItems::PerfMetricTypeUnspecified => {
                    "perfMetricTypeUnspecified"
                }
            }
        }
    }
    impl ::std::fmt::Display for PerfMetricsSummaryPerfMetricsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PerfMetricsSummaryPerfMetricsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PerfMetricsSummaryPerfMetricsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cpu" => PerfMetricsSummaryPerfMetricsItems::Cpu,
                "graphics" => PerfMetricsSummaryPerfMetricsItems::Graphics,
                "memory" => PerfMetricsSummaryPerfMetricsItems::Memory,
                "network" => PerfMetricsSummaryPerfMetricsItems::Network,
                "perfMetricTypeUnspecified" => {
                    PerfMetricsSummaryPerfMetricsItems::PerfMetricTypeUnspecified
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
    impl ::field_selector::FieldSelector for PerfMetricsSummaryPerfMetricsItems {
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
    pub struct PerfMetricsSummary {
        #[serde(rename = "appStartTime", default)]
        pub app_start_time: ::std::option::Option<crate::schemas::AppStartTime>,
        #[doc = "A tool results execution ID."]
        #[serde(rename = "executionId", default)]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "Graphics statistics for the entire run. Statistics are reset at the beginning of the run and collected at the end of the run."]
        #[serde(rename = "graphicsStats", default)]
        pub graphics_stats: ::std::option::Option<crate::schemas::GraphicsStats>,
        #[doc = "A tool results history ID."]
        #[serde(rename = "historyId", default)]
        pub history_id: ::std::option::Option<String>,
        #[doc = "Describes the environment in which the performance metrics were collected"]
        #[serde(rename = "perfEnvironment", default)]
        pub perf_environment: ::std::option::Option<crate::schemas::PerfEnvironment>,
        #[doc = "Set of resource collected"]
        #[serde(rename = "perfMetrics", default)]
        pub perf_metrics:
            ::std::option::Option<Vec<crate::schemas::PerfMetricsSummaryPerfMetricsItems>>,
        #[doc = "The cloud project"]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "A tool results step ID."]
        #[serde(rename = "stepId", default)]
        pub step_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PerfMetricsSummary {
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
    pub struct PerfSample {
        #[doc = "Timestamp of collection"]
        #[serde(rename = "sampleTime", default)]
        pub sample_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "Value observed"]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for PerfSample {
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
    pub struct PerfSampleSeries {
        #[doc = "Basic series represented by a line chart"]
        #[serde(rename = "basicPerfSampleSeries", default)]
        pub basic_perf_sample_series: ::std::option::Option<crate::schemas::BasicPerfSampleSeries>,
        #[doc = "A tool results execution ID."]
        #[serde(rename = "executionId", default)]
        pub execution_id: ::std::option::Option<String>,
        #[doc = "A tool results history ID."]
        #[serde(rename = "historyId", default)]
        pub history_id: ::std::option::Option<String>,
        #[doc = "The cloud project"]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "A sample series id"]
        #[serde(rename = "sampleSeriesId", default)]
        pub sample_series_id: ::std::option::Option<String>,
        #[doc = "A tool results step ID."]
        #[serde(rename = "stepId", default)]
        pub step_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PerfSampleSeries {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PrimaryStepRollUp {
        Failure,
        Flaky,
        Inconclusive,
        Skipped,
        Success,
        Unset,
    }
    impl PrimaryStepRollUp {
        pub fn as_str(self) -> &'static str {
            match self {
                PrimaryStepRollUp::Failure => "failure",
                PrimaryStepRollUp::Flaky => "flaky",
                PrimaryStepRollUp::Inconclusive => "inconclusive",
                PrimaryStepRollUp::Skipped => "skipped",
                PrimaryStepRollUp::Success => "success",
                PrimaryStepRollUp::Unset => "unset",
            }
        }
    }
    impl ::std::fmt::Display for PrimaryStepRollUp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PrimaryStepRollUp {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PrimaryStepRollUp {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "failure" => PrimaryStepRollUp::Failure,
                "flaky" => PrimaryStepRollUp::Flaky,
                "inconclusive" => PrimaryStepRollUp::Inconclusive,
                "skipped" => PrimaryStepRollUp::Skipped,
                "success" => PrimaryStepRollUp::Success,
                "unset" => PrimaryStepRollUp::Unset,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PrimaryStepRollUp {
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
    pub struct PrimaryStep {
        #[doc = "Step Id and outcome of each individual step."]
        #[serde(rename = "individualOutcome", default)]
        pub individual_outcome: ::std::option::Option<Vec<crate::schemas::IndividualOutcome>>,
        #[doc = "Rollup test status of multiple steps that were run with the same configuration as a group."]
        #[serde(rename = "rollUp", default)]
        pub roll_up: ::std::option::Option<crate::schemas::PrimaryStepRollUp>,
    }
    impl ::field_selector::FieldSelector for PrimaryStep {
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
    pub struct ProjectSettings {
        #[doc = "The name of the Google Cloud Storage bucket to which results are written.\n\nBy default, this is unset.\n\nIn update request: optional In response: optional"]
        #[serde(rename = "defaultBucket", default)]
        pub default_bucket: ::std::option::Option<String>,
        #[doc = "The name of the project's settings.\n\nAlways of the form: projects/{project-id}/settings\n\nIn update request: never set In response: always set"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ProjectSettings {
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
    pub struct PublishXunitXmlFilesRequest {
        #[doc = "URI of the Xunit XML files to publish.\n\nThe maximum size of the file this reference is pointing to is 50MB.\n\nRequired."]
        #[serde(rename = "xunitXmlFiles", default)]
        pub xunit_xml_files: ::std::option::Option<Vec<crate::schemas::FileReference>>,
    }
    impl ::field_selector::FieldSelector for PublishXunitXmlFilesRequest {
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
    pub struct Screen {
        #[doc = "File reference of the png file. Required."]
        #[serde(rename = "fileReference", default)]
        pub file_reference: ::std::option::Option<String>,
        #[doc = "Locale of the device that the screenshot was taken on. Required."]
        #[serde(rename = "locale", default)]
        pub locale: ::std::option::Option<String>,
        #[doc = "Model of the device that the screenshot was taken on. Required."]
        #[serde(rename = "model", default)]
        pub model: ::std::option::Option<String>,
        #[doc = "OS version of the device that the screenshot was taken on. Required."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Screen {
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
    pub struct ScreenshotCluster {
        #[doc = "A string that describes the activity of every screen in the cluster."]
        #[serde(rename = "activity", default)]
        pub activity: ::std::option::Option<String>,
        #[doc = "A unique identifier for the cluster."]
        #[serde(rename = "clusterId", default)]
        pub cluster_id: ::std::option::Option<String>,
        #[doc = "A singular screen that represents the cluster as a whole. This screen will act as the \"cover\" of the entire cluster. When users look at the clusters, only the key screen from each cluster will be shown. Which screen is the key screen is determined by the ClusteringAlgorithm"]
        #[serde(rename = "keyScreen", default)]
        pub key_screen: ::std::option::Option<crate::schemas::Screen>,
        #[doc = "Full list of screens."]
        #[serde(rename = "screens", default)]
        pub screens: ::std::option::Option<Vec<crate::schemas::Screen>>,
    }
    impl ::field_selector::FieldSelector for ScreenshotCluster {
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
    pub struct SkippedDetail {
        #[doc = "If the App doesn't support the specific API level."]
        #[serde(rename = "incompatibleAppVersion", default)]
        pub incompatible_app_version: ::std::option::Option<bool>,
        #[doc = "If the App doesn't run on the specific architecture, for example, x86."]
        #[serde(rename = "incompatibleArchitecture", default)]
        pub incompatible_architecture: ::std::option::Option<bool>,
        #[doc = "If the requested OS version doesn't run on the specific device model."]
        #[serde(rename = "incompatibleDevice", default)]
        pub incompatible_device: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for SkippedDetail {
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
    pub struct Specification {
        #[doc = "An Android mobile test execution specification."]
        #[serde(rename = "androidTest", default)]
        pub android_test: ::std::option::Option<crate::schemas::AndroidTest>,
    }
    impl ::field_selector::FieldSelector for Specification {
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
    pub struct StackTrace {
        #[doc = "The stack trace message.\n\nRequired"]
        #[serde(rename = "exception", default)]
        pub exception: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StackTrace {
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
    pub struct Status {
        #[doc = "The status code, which should be an enum value of [google.rpc.Code][]."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details: ::std::option::Option<Vec<crate::schemas::Any>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the [google.rpc.Status.details][] field, or localized by the client."]
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StepState {
        Complete,
        InProgress,
        Pending,
        UnknownState,
    }
    impl StepState {
        pub fn as_str(self) -> &'static str {
            match self {
                StepState::Complete => "complete",
                StepState::InProgress => "inProgress",
                StepState::Pending => "pending",
                StepState::UnknownState => "unknownState",
            }
        }
    }
    impl ::std::fmt::Display for StepState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StepState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StepState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "complete" => StepState::Complete,
                "inProgress" => StepState::InProgress,
                "pending" => StepState::Pending,
                "unknownState" => StepState::UnknownState,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for StepState {
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
    pub struct Step {
        #[doc = "The time when the step status was set to complete.\n\nThis value will be set automatically when state transitions to COMPLETE.\n\n* In response: set if the execution state is COMPLETE. - In create/update request: never set"]
        #[serde(rename = "completionTime", default)]
        pub completion_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "The time when the step was created.\n\n* In response: always set - In create/update request: never set"]
        #[serde(rename = "creationTime", default)]
        pub creation_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "A description of this tool For example: mvn clean package -D skipTests=true\n\n* In response: present if set by create/update request - In create/update request: optional"]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "How much the device resource is used to perform the test.\n\nThis is the device usage used for billing purpose, which is different from the run_duration, for example, infrastructure failure won't be charged for device usage.\n\nPRECONDITION_FAILED will be returned if one attempts to set a device_usage on a step which already has this field set.\n\n* In response: present if previously set. - In create request: optional - In update request: optional"]
        #[serde(rename = "deviceUsageDuration", default)]
        pub device_usage_duration: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "If the execution containing this step has any dimension_definition set, then this field allows the child to specify the values of the dimensions.\n\nThe keys must exactly match the dimension_definition of the execution.\n\nFor example, if the execution has `dimension_definition = ['attempt', 'device']` then a step must define values for those dimensions, eg. `dimension_value = ['attempt': '1', 'device': 'Nexus 6']`\n\nIf a step does not participate in one dimension of the matrix, the value for that dimension should be empty string. For example, if one of the tests is executed by a runner which does not support retries, the step could have `dimension_value = ['attempt': '', 'device': 'Nexus 6']`\n\nIf the step does not participate in any dimensions of the matrix, it may leave dimension_value unset.\n\nA PRECONDITION_FAILED will be returned if any of the keys do not exist in the dimension_definition of the execution.\n\nA PRECONDITION_FAILED will be returned if another step in this execution already has the same name and dimension_value, but differs on other data fields, for example, step field is different.\n\nA PRECONDITION_FAILED will be returned if dimension_value is set, and there is a dimension_definition in the execution which is not specified as one of the keys.\n\n* In response: present if set by create - In create request: optional - In update request: never set"]
        #[serde(rename = "dimensionValue", default)]
        pub dimension_value: ::std::option::Option<Vec<crate::schemas::StepDimensionValueEntry>>,
        #[doc = "Whether any of the outputs of this step are images whose thumbnails can be fetched with ListThumbnails.\n\n* In response: always set - In create/update request: never set"]
        #[serde(rename = "hasImages", default)]
        pub has_images: ::std::option::Option<bool>,
        #[doc = "Arbitrary user-supplied key/value pairs that are associated with the step.\n\nUsers are responsible for managing the key namespace such that keys don't accidentally collide.\n\nAn INVALID_ARGUMENT will be returned if the number of labels exceeds 100 or if the length of any of the keys or values exceeds 100 characters.\n\n* In response: always set - In create request: optional - In update request: optional; any new key/value pair will be added to the map, and any new value for an existing key will update that key's value"]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<Vec<crate::schemas::StepLabelsEntry>>,
        #[doc = "Details when multiple steps are run with the same configuration as a group. These details can be used identify which group this step is part of. It also identifies the groups 'primary step' which indexes all the group members.\n\n* In response: present if previously set. - In create request: optional, set iff this step was performed more than once. - In update request: optional"]
        #[serde(rename = "multiStep", default)]
        pub multi_step: ::std::option::Option<crate::schemas::MultiStep>,
        #[doc = "A short human-readable name to display in the UI. Maximum of 100 characters. For example: Clean build\n\nA PRECONDITION_FAILED will be returned upon creating a new step if it shares its name and dimension_value with an existing step. If two steps represent a similar action, but have different dimension values, they should share the same name. For instance, if the same set of tests is run on two different platforms, the two steps should have the same name.\n\n* In response: always set - In create request: always set - In update request: never set"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Classification of the result, for example into SUCCESS or FAILURE\n\n* In response: present if set by create/update request - In create/update request: optional"]
        #[serde(rename = "outcome", default)]
        pub outcome: ::std::option::Option<crate::schemas::Outcome>,
        #[doc = "How long it took for this step to run.\n\nIf unset, this is set to the difference between creation_time and completion_time when the step is set to the COMPLETE state. In some cases, it is appropriate to set this value separately: For instance, if a step is created, but the operation it represents is queued for a few minutes before it executes, it would be appropriate not to include the time spent queued in its run_duration.\n\nPRECONDITION_FAILED will be returned if one attempts to set a run_duration on a step which already has this field set.\n\n* In response: present if previously set; always present on COMPLETE step - In create request: optional - In update request: optional"]
        #[serde(rename = "runDuration", default)]
        pub run_duration: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "The initial state is IN_PROGRESS. The only legal state transitions are * IN_PROGRESS -> COMPLETE\n\nA PRECONDITION_FAILED will be returned if an invalid transition is requested.\n\nIt is valid to create Step with a state set to COMPLETE. The state can only be set to COMPLETE once. A PRECONDITION_FAILED will be returned if the state is set to COMPLETE multiple times.\n\n* In response: always set - In create/update request: optional"]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::StepState>,
        #[doc = "A unique identifier within a Execution for this Step.\n\nReturns INVALID_ARGUMENT if this field is set or overwritten by the caller.\n\n* In response: always set - In create/update request: never set"]
        #[serde(rename = "stepId", default)]
        pub step_id: ::std::option::Option<String>,
        #[doc = "An execution of a test runner."]
        #[serde(rename = "testExecutionStep", default)]
        pub test_execution_step: ::std::option::Option<crate::schemas::TestExecutionStep>,
        #[doc = "An execution of a tool (used for steps we don't explicitly support)."]
        #[serde(rename = "toolExecutionStep", default)]
        pub tool_execution_step: ::std::option::Option<crate::schemas::ToolExecutionStep>,
    }
    impl ::field_selector::FieldSelector for Step {
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
    pub struct StepDimensionValueEntry {
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StepDimensionValueEntry {
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
    pub struct StepLabelsEntry {
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StepLabelsEntry {
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
    pub struct SuccessDetail {
        #[doc = "If a native process other than the app crashed."]
        #[serde(rename = "otherNativeCrash", default)]
        pub other_native_crash: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for SuccessDetail {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestCaseStatus {
        Error,
        Failed,
        Flaky,
        Passed,
        Skipped,
    }
    impl TestCaseStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                TestCaseStatus::Error => "error",
                TestCaseStatus::Failed => "failed",
                TestCaseStatus::Flaky => "flaky",
                TestCaseStatus::Passed => "passed",
                TestCaseStatus::Skipped => "skipped",
            }
        }
    }
    impl ::std::fmt::Display for TestCaseStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestCaseStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestCaseStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "error" => TestCaseStatus::Error,
                "failed" => TestCaseStatus::Failed,
                "flaky" => TestCaseStatus::Flaky,
                "passed" => TestCaseStatus::Passed,
                "skipped" => TestCaseStatus::Skipped,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TestCaseStatus {
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
    pub struct TestCase {
        #[doc = "The elapsed run time of the test case.\n\nRequired."]
        #[serde(rename = "elapsedTime", default)]
        pub elapsed_time: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "The end time of the test case.\n\nOptional."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "Why the test case was skipped.\n\nPresent only for skipped test case"]
        #[serde(rename = "skippedMessage", default)]
        pub skipped_message: ::std::option::Option<String>,
        #[doc = "The stack trace details if the test case failed or encountered an error.\n\nThe maximum size of the stack traces is 100KiB, beyond which the stack track will be truncated.\n\nZero if the test case passed."]
        #[serde(rename = "stackTraces", default)]
        pub stack_traces: ::std::option::Option<Vec<crate::schemas::StackTrace>>,
        #[doc = "The start time of the test case.\n\nOptional."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "The status of the test case.\n\nRequired."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::TestCaseStatus>,
        #[doc = "A unique identifier within a Step for this Test Case."]
        #[serde(rename = "testCaseId", default)]
        pub test_case_id: ::std::option::Option<String>,
        #[doc = "Test case reference, e.g. name, class name and test suite name.\n\nRequired."]
        #[serde(rename = "testCaseReference", default)]
        pub test_case_reference: ::std::option::Option<crate::schemas::TestCaseReference>,
        #[doc = "References to opaque files of any format output by the tool execution."]
        #[serde(rename = "toolOutputs", default)]
        pub tool_outputs: ::std::option::Option<Vec<crate::schemas::ToolOutputReference>>,
    }
    impl ::field_selector::FieldSelector for TestCase {
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
    pub struct TestCaseReference {
        #[doc = "The name of the class."]
        #[serde(rename = "className", default)]
        pub class_name: ::std::option::Option<String>,
        #[doc = "The name of the test case.\n\nRequired."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The name of the test suite to which this test case belongs."]
        #[serde(rename = "testSuiteName", default)]
        pub test_suite_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TestCaseReference {
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
    pub struct TestExecutionStep {
        #[doc = "Issues observed during the test execution.\n\nFor example, if the mobile app under test crashed during the test, the error message and the stack trace content can be recorded here to assist debugging.\n\n* In response: present if set by create or update - In create/update request: optional"]
        #[serde(rename = "testIssues", default)]
        pub test_issues: ::std::option::Option<Vec<crate::schemas::TestIssue>>,
        #[doc = "List of test suite overview contents. This could be parsed from xUnit XML log by server, or uploaded directly by user. This references should only be called when test suites are fully parsed or uploaded.\n\nThe maximum allowed number of test suite overviews per step is 1000.\n\n* In response: always set - In create request: optional - In update request: never (use publishXunitXmlFiles custom method instead)"]
        #[serde(rename = "testSuiteOverviews", default)]
        pub test_suite_overviews: ::std::option::Option<Vec<crate::schemas::TestSuiteOverview>>,
        #[doc = "The timing break down of the test execution.\n\n* In response: present if set by create or update - In create/update request: optional"]
        #[serde(rename = "testTiming", default)]
        pub test_timing: ::std::option::Option<crate::schemas::TestTiming>,
        #[doc = "Represents the execution of the test runner.\n\nThe exit code of this tool will be used to determine if the test passed.\n\n* In response: always set - In create/update request: optional"]
        #[serde(rename = "toolExecution", default)]
        pub tool_execution: ::std::option::Option<crate::schemas::ToolExecution>,
    }
    impl ::field_selector::FieldSelector for TestExecutionStep {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestIssueCategory {
        Common,
        Robo,
        UnspecifiedCategory,
    }
    impl TestIssueCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                TestIssueCategory::Common => "common",
                TestIssueCategory::Robo => "robo",
                TestIssueCategory::UnspecifiedCategory => "unspecifiedCategory",
            }
        }
    }
    impl ::std::fmt::Display for TestIssueCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestIssueCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestIssueCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "common" => TestIssueCategory::Common,
                "robo" => TestIssueCategory::Robo,
                "unspecifiedCategory" => TestIssueCategory::UnspecifiedCategory,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TestIssueCategory {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestIssueType {
        Anr,
        AvailableDeepLinks,
        CompatibleWithOrchestrator,
        CompleteRoboScriptExecution,
        CrashDialogError,
        EncounteredLoginScreen,
        EncounteredNonAndroidUiWidgetScreen,
        FailedToInstall,
        FatalException,
        InAppPurchases,
        IncompleteRoboScriptExecution,
        InsufficientCoverage,
        IosCrash,
        IosException,
        LauncherActivityNotFound,
        NativeCrash,
        NonSdkApiUsageReport,
        NonSdkApiUsageViolation,
        PerformedGoogleLogin,
        PerformedMonkeyActions,
        StartActivityNotFound,
        UiElementsTooDeep,
        UnspecifiedType,
        UnusedRoboDirective,
        UsedRoboDirective,
        UsedRoboIgnoreDirective,
    }
    impl TestIssueType {
        pub fn as_str(self) -> &'static str {
            match self {
                TestIssueType::Anr => "anr",
                TestIssueType::AvailableDeepLinks => "availableDeepLinks",
                TestIssueType::CompatibleWithOrchestrator => "compatibleWithOrchestrator",
                TestIssueType::CompleteRoboScriptExecution => "completeRoboScriptExecution",
                TestIssueType::CrashDialogError => "crashDialogError",
                TestIssueType::EncounteredLoginScreen => "encounteredLoginScreen",
                TestIssueType::EncounteredNonAndroidUiWidgetScreen => {
                    "encounteredNonAndroidUiWidgetScreen"
                }
                TestIssueType::FailedToInstall => "failedToInstall",
                TestIssueType::FatalException => "fatalException",
                TestIssueType::InAppPurchases => "inAppPurchases",
                TestIssueType::IncompleteRoboScriptExecution => "incompleteRoboScriptExecution",
                TestIssueType::InsufficientCoverage => "insufficientCoverage",
                TestIssueType::IosCrash => "iosCrash",
                TestIssueType::IosException => "iosException",
                TestIssueType::LauncherActivityNotFound => "launcherActivityNotFound",
                TestIssueType::NativeCrash => "nativeCrash",
                TestIssueType::NonSdkApiUsageReport => "nonSdkApiUsageReport",
                TestIssueType::NonSdkApiUsageViolation => "nonSdkApiUsageViolation",
                TestIssueType::PerformedGoogleLogin => "performedGoogleLogin",
                TestIssueType::PerformedMonkeyActions => "performedMonkeyActions",
                TestIssueType::StartActivityNotFound => "startActivityNotFound",
                TestIssueType::UiElementsTooDeep => "uiElementsTooDeep",
                TestIssueType::UnspecifiedType => "unspecifiedType",
                TestIssueType::UnusedRoboDirective => "unusedRoboDirective",
                TestIssueType::UsedRoboDirective => "usedRoboDirective",
                TestIssueType::UsedRoboIgnoreDirective => "usedRoboIgnoreDirective",
            }
        }
    }
    impl ::std::fmt::Display for TestIssueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestIssueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestIssueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "anr" => TestIssueType::Anr,
                "availableDeepLinks" => TestIssueType::AvailableDeepLinks,
                "compatibleWithOrchestrator" => TestIssueType::CompatibleWithOrchestrator,
                "completeRoboScriptExecution" => TestIssueType::CompleteRoboScriptExecution,
                "crashDialogError" => TestIssueType::CrashDialogError,
                "encounteredLoginScreen" => TestIssueType::EncounteredLoginScreen,
                "encounteredNonAndroidUiWidgetScreen" => {
                    TestIssueType::EncounteredNonAndroidUiWidgetScreen
                }
                "failedToInstall" => TestIssueType::FailedToInstall,
                "fatalException" => TestIssueType::FatalException,
                "inAppPurchases" => TestIssueType::InAppPurchases,
                "incompleteRoboScriptExecution" => TestIssueType::IncompleteRoboScriptExecution,
                "insufficientCoverage" => TestIssueType::InsufficientCoverage,
                "iosCrash" => TestIssueType::IosCrash,
                "iosException" => TestIssueType::IosException,
                "launcherActivityNotFound" => TestIssueType::LauncherActivityNotFound,
                "nativeCrash" => TestIssueType::NativeCrash,
                "nonSdkApiUsageReport" => TestIssueType::NonSdkApiUsageReport,
                "nonSdkApiUsageViolation" => TestIssueType::NonSdkApiUsageViolation,
                "performedGoogleLogin" => TestIssueType::PerformedGoogleLogin,
                "performedMonkeyActions" => TestIssueType::PerformedMonkeyActions,
                "startActivityNotFound" => TestIssueType::StartActivityNotFound,
                "uiElementsTooDeep" => TestIssueType::UiElementsTooDeep,
                "unspecifiedType" => TestIssueType::UnspecifiedType,
                "unusedRoboDirective" => TestIssueType::UnusedRoboDirective,
                "usedRoboDirective" => TestIssueType::UsedRoboDirective,
                "usedRoboIgnoreDirective" => TestIssueType::UsedRoboIgnoreDirective,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TestIssueType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TestIssueSeverity {
        Info,
        Severe,
        Suggestion,
        UnspecifiedSeverity,
        Warning,
    }
    impl TestIssueSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                TestIssueSeverity::Info => "info",
                TestIssueSeverity::Severe => "severe",
                TestIssueSeverity::Suggestion => "suggestion",
                TestIssueSeverity::UnspecifiedSeverity => "unspecifiedSeverity",
                TestIssueSeverity::Warning => "warning",
            }
        }
    }
    impl ::std::fmt::Display for TestIssueSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestIssueSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestIssueSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "info" => TestIssueSeverity::Info,
                "severe" => TestIssueSeverity::Severe,
                "suggestion" => TestIssueSeverity::Suggestion,
                "unspecifiedSeverity" => TestIssueSeverity::UnspecifiedSeverity,
                "warning" => TestIssueSeverity::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TestIssueSeverity {
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
    pub struct TestIssue {
        #[doc = "Category of issue. Required."]
        #[serde(rename = "category", default)]
        pub category: ::std::option::Option<crate::schemas::TestIssueCategory>,
        #[doc = "A brief human-readable message describing the issue. Required."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Type of issue. Required."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::TestIssueType>,
        #[doc = "Severity of issue. Required."]
        #[serde(rename = "severity", default)]
        pub severity: ::std::option::Option<crate::schemas::TestIssueSeverity>,
        #[doc = "Deprecated in favor of stack trace fields inside specific warnings."]
        #[serde(rename = "stackTrace", default)]
        pub stack_trace: ::std::option::Option<crate::schemas::StackTrace>,
        #[doc = "Warning message with additional details of the issue. Should always be a message from com.google.devtools.toolresults.v1.warnings"]
        #[serde(rename = "warning", default)]
        pub warning: ::std::option::Option<crate::schemas::Any>,
    }
    impl ::field_selector::FieldSelector for TestIssue {
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
    pub struct TestSuiteOverview {
        #[doc = "Elapsed time of test suite."]
        #[serde(rename = "elapsedTime", default)]
        pub elapsed_time: ::std::option::Option<crate::schemas::Duration>,
        #[doc = "Number of test cases in error, typically set by the service by parsing the xml_source.\n\n* In create/response: always set - In update request: never"]
        #[serde(rename = "errorCount", default)]
        pub error_count: ::std::option::Option<i32>,
        #[doc = "Number of failed test cases, typically set by the service by parsing the xml_source. May also be set by the user.\n\n* In create/response: always set - In update request: never"]
        #[serde(rename = "failureCount", default)]
        pub failure_count: ::std::option::Option<i32>,
        #[doc = "The name of the test suite.\n\n* In create/response: always set - In update request: never"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Number of test cases not run, typically set by the service by parsing the xml_source.\n\n* In create/response: always set - In update request: never"]
        #[serde(rename = "skippedCount", default)]
        pub skipped_count: ::std::option::Option<i32>,
        #[doc = "Number of test cases, typically set by the service by parsing the xml_source.\n\n* In create/response: always set - In update request: never"]
        #[serde(rename = "totalCount", default)]
        pub total_count: ::std::option::Option<i32>,
        #[doc = "If this test suite was parsed from XML, this is the URI where the original XML file is stored.\n\nNote: Multiple test suites can share the same xml_source\n\nReturns INVALID_ARGUMENT if the uri format is not supported.\n\n* In create/response: optional - In update request: never"]
        #[serde(rename = "xmlSource", default)]
        pub xml_source: ::std::option::Option<crate::schemas::FileReference>,
    }
    impl ::field_selector::FieldSelector for TestSuiteOverview {
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
    pub struct TestTiming {
        #[doc = "How long it took to run the test process.\n\n* In response: present if previously set. - In create/update request: optional"]
        #[serde(rename = "testProcessDuration", default)]
        pub test_process_duration: ::std::option::Option<crate::schemas::Duration>,
    }
    impl ::field_selector::FieldSelector for TestTiming {
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
    pub struct Thumbnail {
        #[doc = "The thumbnail's content type, i.e. \"image/png\".\n\nAlways set."]
        #[serde(rename = "contentType", default)]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The thumbnail file itself.\n\nThat is, the bytes here are precisely the bytes that make up the thumbnail file; they can be served as an image as-is (with the appropriate content type.)\n\nAlways set."]
        #[serde(rename = "data", default)]
        pub data: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The height of the thumbnail, in pixels.\n\nAlways set."]
        #[serde(rename = "heightPx", default)]
        pub height_px: ::std::option::Option<i32>,
        #[doc = "The width of the thumbnail, in pixels.\n\nAlways set."]
        #[serde(rename = "widthPx", default)]
        pub width_px: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for Thumbnail {
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
    pub struct Timestamp {
        #[doc = "Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nanos values that count forward in time. Must be from 0 to 999,999,999 inclusive."]
        #[serde(rename = "nanos", default)]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive."]
        #[serde(rename = "seconds", default)]
        #[serde(with = "crate::parsed_string")]
        pub seconds: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for Timestamp {
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
    pub struct ToolExecution {
        #[doc = "The full tokenized command line including the program name (equivalent to argv in a C program).\n\n* In response: present if set by create request - In create request: optional - In update request: never set"]
        #[serde(rename = "commandLineArguments", default)]
        pub command_line_arguments: ::std::option::Option<Vec<String>>,
        #[doc = "Tool execution exit code. This field will be set once the tool has exited.\n\n* In response: present if set by create/update request - In create request: optional - In update request: optional, a FAILED_PRECONDITION error will be returned if an exit_code is already set."]
        #[serde(rename = "exitCode", default)]
        pub exit_code: ::std::option::Option<crate::schemas::ToolExitCode>,
        #[doc = "References to any plain text logs output the tool execution.\n\nThis field can be set before the tool has exited in order to be able to have access to a live view of the logs while the tool is running.\n\nThe maximum allowed number of tool logs per step is 1000.\n\n* In response: present if set by create/update request - In create request: optional - In update request: optional, any value provided will be appended to the existing list"]
        #[serde(rename = "toolLogs", default)]
        pub tool_logs: ::std::option::Option<Vec<crate::schemas::FileReference>>,
        #[doc = "References to opaque files of any format output by the tool execution.\n\nThe maximum allowed number of tool outputs per step is 1000.\n\n* In response: present if set by create/update request - In create request: optional - In update request: optional, any value provided will be appended to the existing list"]
        #[serde(rename = "toolOutputs", default)]
        pub tool_outputs: ::std::option::Option<Vec<crate::schemas::ToolOutputReference>>,
    }
    impl ::field_selector::FieldSelector for ToolExecution {
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
    pub struct ToolExecutionStep {
        #[doc = "A Tool execution.\n\n* In response: present if set by create/update request - In create/update request: optional"]
        #[serde(rename = "toolExecution", default)]
        pub tool_execution: ::std::option::Option<crate::schemas::ToolExecution>,
    }
    impl ::field_selector::FieldSelector for ToolExecutionStep {
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
    pub struct ToolExitCode {
        #[doc = "Tool execution exit code. A value of 0 means that the execution was successful.\n\n* In response: always set - In create/update request: always set"]
        #[serde(rename = "number", default)]
        pub number: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ToolExitCode {
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
    pub struct ToolOutputReference {
        #[doc = "The creation time of the file.\n\n* In response: present if set by create/update request - In create/update request: optional"]
        #[serde(rename = "creationTime", default)]
        pub creation_time: ::std::option::Option<crate::schemas::Timestamp>,
        #[doc = "A FileReference to an output file.\n\n* In response: always set - In create/update request: always set"]
        #[serde(rename = "output", default)]
        pub output: ::std::option::Option<crate::schemas::FileReference>,
        #[doc = "The test case to which this output file belongs.\n\n* In response: present if set by create/update request - In create/update request: optional"]
        #[serde(rename = "testCase", default)]
        pub test_case: ::std::option::Option<crate::schemas::TestCaseReference>,
    }
    impl ::field_selector::FieldSelector for ToolOutputReference {
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
    impl ::field_selector::FieldSelector for Alt {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions<A> {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
            #[doc = "Gets the Tool Results settings for a project.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read from project"]
            pub fn get_settings(
                &self,
                project_id: impl Into<String>,
            ) -> GetSettingsRequestBuilder<A> {
                GetSettingsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                }
            }
            #[doc = "Creates resources for settings which have not yet been set.\n\nCurrently, this creates a single resource: a Google Cloud Storage bucket, to be used as the default bucket for this project. The bucket is created in an FTL-own storage project. Except for in rare cases, calling this method in parallel from multiple clients will only create a single bucket. In order to avoid unnecessary storage charges, the bucket is configured to automatically delete objects older than 90 days.\n\nThe bucket is created with the following permissions: - Owner access for owners of central storage project (FTL-owned) - Writer access for owners/editors of customer project - Reader access for viewers of customer project The default ACL on objects created in the bucket is: - Owner access for owners of central storage project - Reader access for owners/editors/viewers of customer project See Google Cloud Storage documentation for more details.\n\nIf there is already a default bucket set and the project can access the bucket, this call does nothing. However, if the project doesn't have the permission to access the bucket or the bucket is deleted, a new bucket will be created.\n\nMay return any canonical error codes, including the following:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - Any error code raised by Google Cloud Storage"]
            pub fn initialize_settings(
                &self,
                project_id: impl Into<String>,
            ) -> InitializeSettingsRequestBuilder<A> {
                InitializeSettingsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    project_id: project_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the histories resource"]
            pub fn histories(&self) -> crate::resources::projects::histories::HistoriesActions<A> {
                crate::resources::projects::histories::HistoriesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetSettingsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetSettingsRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::ProjectSettings, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ProjectSettings, Box<dyn ::std::error::Error>> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/settings");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct InitializeSettingsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> InitializeSettingsRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::ProjectSettings, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ProjectSettings, Box<dyn ::std::error::Error>> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":initializeSettings");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        pub mod histories {
            pub mod params {}
            pub struct HistoriesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> HistoriesActions<'a, A> {
                #[doc = "Creates a History.\n\nThe returned History will have the id set.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing project does not exist"]
                pub fn create(
                    &self,
                    request: crate::schemas::History,
                    project_id: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
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
                        project_id: project_id.into(),
                        request_id: None,
                    }
                }
                #[doc = "Gets a History.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the History does not exist"]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    history_id: impl Into<String>,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        project_id: project_id.into(),
                        history_id: history_id.into(),
                    }
                }
                #[doc = "Lists Histories for a given Project.\n\nThe histories are sorted by modification time in descending order. The history_id key will be used to order the history with the same modification time.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the History does not exist"]
                pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        project_id: project_id.into(),
                        filter_by_name: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the executions resource"]
                pub fn executions(
                    &self,
                ) -> crate::resources::projects::histories::executions::ExecutionsActions<A>
                {
                    crate::resources::projects::histories::executions::ExecutionsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::History,
                project_id: String,
                request_id: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID.\n\nOptional, but strongly recommended."]
                pub fn request_id(mut self, value: impl Into<String>) -> Self {
                    self.request_id = Some(value.into());
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
                ) -> Result<crate::schemas::History, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::History, Box<dyn ::std::error::Error>> {
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
                    let mut output =
                        "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/histories");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("requestId", &self.request_id)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                history_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::History, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::History, Box<dyn ::std::error::Error>> {
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/histories/");
                    {
                        let var_as_str = &self.history_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                filter_by_name: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "If set, only return histories with the given name.\n\nOptional."]
                pub fn filter_by_name(mut self, value: impl Into<String>) -> Self {
                    self.filter_by_name = Some(value.into());
                    self
                }
                #[doc = "The maximum number of Histories to fetch.\n\nDefault value: 20. The server will use this default if the field is not set or has a value of 0. Any value greater than 100 will be treated as 100.\n\nOptional."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A continuation token to resume the query at the next item.\n\nOptional."]
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
                pub fn iter_histories<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "histories").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "histories")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_histories_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::History> {
                    self.fields = Some(concat!("nextPageToken,", "histories").to_owned());
                    crate::iter::PageItemIter::new(self, "histories")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_histories_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::History> {
                    self.fields = Some(concat!("nextPageToken,", "histories", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "histories")
                }
                pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = T::field_selector();
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
                pub fn iter_standard(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListHistoriesResponse>
                {
                    crate::iter::PageIter::new(self)
                }
                pub fn iter_debug(
                    mut self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListHistoriesResponse>
                {
                    self.fields = Some("*".to_owned());
                    crate::iter::PageIter::new(self)
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
                ) -> Result<crate::schemas::ListHistoriesResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListHistoriesResponse, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/histories");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filterByName", &self.filter_by_name)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            pub mod executions {
                pub mod params {}
                pub struct ExecutionsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> ExecutionsActions<'a, A> {
                    #[doc = "Creates an Execution.\n\nThe returned Execution will have the id set.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing History does not exist"]
                    pub fn create(
                        &self,
                        request: crate::schemas::Execution,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
                    ) -> CreateRequestBuilder<A> {
                        CreateRequestBuilder {
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
                            project_id: project_id.into(),
                            history_id: history_id.into(),
                            request_id: None,
                        }
                    }
                    #[doc = "Gets an Execution.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Execution does not exist"]
                    pub fn get(
                        &self,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
                        execution_id: impl Into<String>,
                    ) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            project_id: project_id.into(),
                            history_id: history_id.into(),
                            execution_id: execution_id.into(),
                        }
                    }
                    #[doc = "Lists Executions for a given History.\n\nThe executions are sorted by creation_time in descending order. The execution_id key will be used to order the executions with the same creation_time.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing History does not exist"]
                    pub fn list(
                        &self,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            project_id: project_id.into(),
                            history_id: history_id.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates an existing Execution with the supplied partial entity.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal - NOT_FOUND - if the containing History does not exist"]
                    pub fn patch(
                        &self,
                        request: crate::schemas::Execution,
                        project_id: impl Into<String>,
                        history_id: impl Into<String>,
                        execution_id: impl Into<String>,
                    ) -> PatchRequestBuilder<A> {
                        PatchRequestBuilder {
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
                            project_id: project_id.into(),
                            history_id: history_id.into(),
                            execution_id: execution_id.into(),
                            request_id: None,
                        }
                    }
                    #[doc = "Actions that can be performed on the clusters resource"]pub fn clusters ( & self ) -> crate :: resources :: projects :: histories :: executions :: clusters :: ClustersActions < A >{
                        crate :: resources :: projects :: histories :: executions :: clusters :: ClustersActions { reqwest : & self . reqwest , auth : & self . auth }
                    }
                    #[doc = "Actions that can be performed on the steps resource"]
                    pub fn steps(
                        &self,
                    ) -> crate::resources::projects::histories::executions::steps::StepsActions<A>
                    {
                        crate::resources::projects::histories::executions::steps::StepsActions {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Execution,
                    project_id: String,
                    history_id: String,
                    request_id: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                    #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID.\n\nOptional, but strongly recommended."]
                    pub fn request_id(mut self, value: impl Into<String>) -> Self {
                        self.request_id = Some(value.into());
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
                    ) -> Result<crate::schemas::Execution, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Execution, Box<dyn ::std::error::Error>>
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
                        let mut output =
                            "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("requestId", &self.request_id)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    history_id: String,
                    execution_id: String,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
                    ) -> Result<crate::schemas::Execution, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Execution, Box<dyn ::std::error::Error>>
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
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output =
                            "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions/");
                        {
                            let var_as_str = &self.execution_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    history_id: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "The maximum number of Executions to fetch.\n\nDefault value: 25. The server will use this default if the field is not set or has a value of 0.\n\nOptional."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A continuation token to resume the query at the next item.\n\nOptional."]
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
                    pub fn iter_executions<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "executions").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "executions")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_executions_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Execution>
                    {
                        self.fields = Some(concat!("nextPageToken,", "executions").to_owned());
                        crate::iter::PageItemIter::new(self, "executions")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_executions_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Execution>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "executions", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "executions")
                    }
                    pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = T::field_selector();
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
                    pub fn iter_standard(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListExecutionsResponse>
                    {
                        crate::iter::PageIter::new(self)
                    }
                    pub fn iter_debug(
                        mut self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListExecutionsResponse>
                    {
                        self.fields = Some("*".to_owned());
                        crate::iter::PageIter::new(self)
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
                    ) -> Result<crate::schemas::ListExecutionsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListExecutionsResponse, Box<dyn ::std::error::Error>>
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
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output =
                            "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Execution,
                    project_id: String,
                    history_id: String,
                    execution_id: String,
                    request_id: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                    #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID.\n\nOptional, but strongly recommended."]
                    pub fn request_id(mut self, value: impl Into<String>) -> Self {
                        self.request_id = Some(value.into());
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
                    ) -> Result<crate::schemas::Execution, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Execution, Box<dyn ::std::error::Error>>
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
                        let mut output =
                            "https://www.googleapis.com/toolresults/v1beta3/projects/".to_owned();
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/histories/");
                        {
                            let var_as_str = &self.history_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/executions/");
                        {
                            let var_as_str = &self.execution_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("requestId", &self.request_id)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                pub mod clusters {
                    pub mod params {}
                    pub struct ClustersActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ClustersActions<'a, A> {
                        #[doc = "Retrieves a single screenshot cluster by its ID"]
                        pub fn get(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            cluster_id: impl Into<String>,
                        ) -> GetRequestBuilder<A> {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                alt: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                user_ip: None,
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                cluster_id: cluster_id.into(),
                            }
                        }
                        #[doc = "Lists Screenshot Clusters\n\nReturns the list of screenshot clusters corresponding to an execution. Screenshot clusters are created after the execution is finished. Clusters are created from a set of screenshots. Between any two screenshots, a matching score is calculated based off their metadata that determines how similar they are. Screenshots are placed in the cluster that has screens which have the highest matching scores."]
                        pub fn list(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                        ) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                alt: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                user_ip: None,
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        cluster_id: String,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
                        ) -> Result<crate::schemas::ScreenshotCluster, Box<dyn ::std::error::Error>>
                        {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::ScreenshotCluster, Box<dyn ::std::error::Error>>
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
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/clusters/");
                            {
                                let var_as_str = &self.cluster_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
                        ) -> Result<
                            crate::schemas::ListScreenshotClustersResponse,
                            Box<dyn ::std::error::Error>,
                        > {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<
                            crate::schemas::ListScreenshotClustersResponse,
                            Box<dyn ::std::error::Error>,
                        > {
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
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/clusters");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            req
                        }
                    }
                }
                pub mod steps {
                    pub mod params {}
                    pub struct StepsActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> StepsActions<'a, A> {
                        #[doc = "Creates a Step.\n\nThe returned Step will have the id set.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the step is too large (more than 10Mib) - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn create(
                            &self,
                            request: crate::schemas::Step,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                        ) -> CreateRequestBuilder<A> {
                            CreateRequestBuilder {
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
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                request_id: None,
                            }
                        }
                        #[doc = "Gets a Step.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Step does not exist"]
                        pub fn get(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
                        ) -> GetRequestBuilder<A> {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                alt: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                user_ip: None,
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                            }
                        }
                        #[doc = "Retrieves a PerfMetricsSummary.\n\nMay return any of the following error code(s): - NOT_FOUND - The specified PerfMetricsSummary does not exist"]
                        pub fn get_perf_metrics_summary(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
                        ) -> GetPerfMetricsSummaryRequestBuilder<A> {
                            GetPerfMetricsSummaryRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                alt: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                user_ip: None,
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                            }
                        }
                        #[doc = "Lists Steps for a given Execution.\n\nThe steps are sorted by creation_time in descending order. The step_id key will be used to order the steps with the same creation_time.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if an argument in the request happens to be invalid; e.g. if an attempt is made to list the children of a nonexistent Step - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn list(
                            &self,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                        ) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                alt: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                user_ip: None,
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                page_size: None,
                                page_token: None,
                            }
                        }
                        #[doc = "Updates an existing Step with the supplied partial entity.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal (e.g try to upload a duplicate xml file), if the updated step is too large (more than 10Mib) - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn patch(
                            &self,
                            request: crate::schemas::Step,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
                        ) -> PatchRequestBuilder<A> {
                            PatchRequestBuilder {
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
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                                request_id: None,
                            }
                        }
                        #[doc = "Publish xml files to an existing Step.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the requested state transition is illegal, e.g try to upload a duplicate xml file or a file too large. - NOT_FOUND - if the containing Execution does not exist"]
                        pub fn publish_xunit_xml_files(
                            &self,
                            request: crate::schemas::PublishXunitXmlFilesRequest,
                            project_id: impl Into<String>,
                            history_id: impl Into<String>,
                            execution_id: impl Into<String>,
                            step_id: impl Into<String>,
                        ) -> PublishXunitXmlFilesRequestBuilder<A> {
                            PublishXunitXmlFilesRequestBuilder {
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
                                project_id: project_id.into(),
                                history_id: history_id.into(),
                                execution_id: execution_id.into(),
                                step_id: step_id.into(),
                            }
                        }
                        #[doc = "Actions that can be performed on the perf_metrics_summary resource"]pub fn perf_metrics_summary ( & self ) -> crate :: resources :: projects :: histories :: executions :: steps :: perf_metrics_summary :: PerfMetricsSummaryActions < A >{
                            crate :: resources :: projects :: histories :: executions :: steps :: perf_metrics_summary :: PerfMetricsSummaryActions { reqwest : & self . reqwest , auth : & self . auth }
                        }
                        #[doc = "Actions that can be performed on the perf_sample_series resource"]pub fn perf_sample_series ( & self ) -> crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: PerfSampleSeriesActions < A >{
                            crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: PerfSampleSeriesActions { reqwest : & self . reqwest , auth : & self . auth }
                        }
                        #[doc = "Actions that can be performed on the test_cases resource"]pub fn test_cases ( & self ) -> crate :: resources :: projects :: histories :: executions :: steps :: test_cases :: TestCasesActions < A >{
                            crate :: resources :: projects :: histories :: executions :: steps :: test_cases :: TestCasesActions { reqwest : & self . reqwest , auth : & self . auth }
                        }
                        #[doc = "Actions that can be performed on the thumbnails resource"]pub fn thumbnails ( & self ) -> crate :: resources :: projects :: histories :: executions :: steps :: thumbnails :: ThumbnailsActions < A >{
                            crate :: resources :: projects :: histories :: executions :: steps :: thumbnails :: ThumbnailsActions { reqwest : & self . reqwest , auth : & self . auth }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::Step,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        request_id: Option<String>,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                        #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID.\n\nOptional, but strongly recommended."]
                        pub fn request_id(mut self, value: impl Into<String>) -> Self {
                            self.request_id = Some(value.into());
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
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
                        {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
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
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("requestId", &self.request_id)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
                        {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
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
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct GetPerfMetricsSummaryRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> GetPerfMetricsSummaryRequestBuilder<'a, A> {
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
                        ) -> Result<crate::schemas::PerfMetricsSummary, Box<dyn ::std::error::Error>>
                        {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::PerfMetricsSummary, Box<dyn ::std::error::Error>>
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
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/perfMetricsSummary");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                        #[doc = "The maximum number of Steps to fetch.\n\nDefault value: 25. The server will use this default if the field is not set or has a value of 0.\n\nOptional."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "A continuation token to resume the query at the next item.\n\nOptional."]
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
                        pub fn iter_steps<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            let mut fields = concat!("nextPageToken,", "steps").to_owned();
                            let items_fields = T::field_selector();
                            if !items_fields.is_empty() {
                                fields.push_str("(");
                                fields.push_str(&items_fields);
                                fields.push_str(")");
                            }
                            self.fields = Some(fields);
                            crate::iter::PageItemIter::new(self, "steps")
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_steps_standard(
                            mut self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::Step>
                        {
                            self.fields = Some(concat!("nextPageToken,", "steps").to_owned());
                            crate::iter::PageItemIter::new(self, "steps")
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_steps_debug(
                            mut self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::Step>
                        {
                            self.fields =
                                Some(concat!("nextPageToken,", "steps", "(*)").to_owned());
                            crate::iter::PageItemIter::new(self, "steps")
                        }
                        pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            let mut fields = T::field_selector();
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
                        pub fn iter_standard(
                            self,
                        ) -> crate::iter::PageIter<Self, crate::schemas::ListStepsResponse>
                        {
                            crate::iter::PageIter::new(self)
                        }
                        pub fn iter_debug(
                            mut self,
                        ) -> crate::iter::PageIter<Self, crate::schemas::ListStepsResponse>
                        {
                            self.fields = Some("*".to_owned());
                            crate::iter::PageIter::new(self)
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
                        ) -> Result<crate::schemas::ListStepsResponse, Box<dyn ::std::error::Error>>
                        {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::ListStepsResponse, Box<dyn ::std::error::Error>>
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
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("pageSize", &self.page_size)]);
                            let req = req.query(&[("pageToken", &self.page_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                        fn set_page_token(&mut self, value: String) {
                            self.page_token = value.into();
                        }
                        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            self._execute()
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::Step,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
                        request_id: Option<String>,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
                        #[doc = "A unique request ID for server to detect duplicated requests. For example, a UUID.\n\nOptional, but strongly recommended."]
                        pub fn request_id(mut self, value: impl Into<String>) -> Self {
                            self.request_id = Some(value.into());
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
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
                        {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
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
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            let req = req.query(&[("requestId", &self.request_id)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct PublishXunitXmlFilesRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::PublishXunitXmlFilesRequest,
                        project_id: String,
                        history_id: String,
                        execution_id: String,
                        step_id: String,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> PublishXunitXmlFilesRequestBuilder<'a, A> {
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
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
                        {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<crate::schemas::Step, Box<dyn ::std::error::Error>>
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
                            let mut output =
                                "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                    .to_owned();
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/histories/");
                            {
                                let var_as_str = &self.history_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/executions/");
                            {
                                let var_as_str = &self.execution_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/steps/");
                            {
                                let var_as_str = &self.step_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str(":publishXunitXmlFiles");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    pub mod perf_metrics_summary {
                        pub mod params {}
                        pub struct PerfMetricsSummaryActions<'a, A> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a std::sync::Mutex<A>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> PerfMetricsSummaryActions<'a, A> {
                            #[doc = "Creates a PerfMetricsSummary resource. Returns the existing one if it has already been created.\n\nMay return any of the following error code(s): - NOT_FOUND - The containing Step does not exist"]
                            pub fn create(
                                &self,
                                request: crate::schemas::PerfMetricsSummary,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> CreateRequestBuilder<A> {
                                CreateRequestBuilder {
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
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                }
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct CreateRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            request: crate::schemas::PerfMetricsSummary,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            alt: Option<crate::params::Alt>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            user_ip: Option<String>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                            ) -> Result<
                                crate::schemas::PerfMetricsSummary,
                                Box<dyn ::std::error::Error>,
                            > {
                                self.execute_fields::<_, &str>(None)
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub fn execute_debug(
                                self,
                            ) -> Result<
                                crate::schemas::PerfMetricsSummary,
                                Box<dyn ::std::error::Error>,
                            > {
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
                                let mut output =
                                    "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                        .to_owned();
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfMetricsSummary");
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::POST, path);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("userIp", &self.user_ip)]);
                                let mut auth = self.auth.lock().unwrap();
                                let fut = auth
                                    .token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                let token = runtime.block_on(fut).unwrap().access_token;
                                let req = req.bearer_auth(&token);
                                req
                            }
                        }
                    }
                    pub mod perf_sample_series {
                        pub mod params {
                            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                            pub enum ListFilterItems {
                                Cpu,
                                Graphics,
                                Memory,
                                Network,
                                PerfMetricTypeUnspecified,
                            }
                            impl ListFilterItems {
                                pub fn as_str(self) -> &'static str {
                                    match self {
                                        ListFilterItems::Cpu => "cpu",
                                        ListFilterItems::Graphics => "graphics",
                                        ListFilterItems::Memory => "memory",
                                        ListFilterItems::Network => "network",
                                        ListFilterItems::PerfMetricTypeUnspecified => {
                                            "perfMetricTypeUnspecified"
                                        }
                                    }
                                }
                            }
                            impl ::std::fmt::Display for ListFilterItems {
                                fn fmt(
                                    &self,
                                    f: &mut std::fmt::Formatter<'_>,
                                ) -> ::std::fmt::Result {
                                    f.write_str(self.as_str())
                                }
                            }
                            impl ::serde::Serialize for ListFilterItems {
                                fn serialize<S>(
                                    &self,
                                    serializer: S,
                                ) -> ::std::result::Result<S::Ok, S::Error>
                                where
                                    S: ::serde::ser::Serializer,
                                {
                                    serializer.serialize_str(self.as_str())
                                }
                            }
                            impl<'de> ::serde::Deserialize<'de> for ListFilterItems {
                                fn deserialize<D>(
                                    deserializer: D,
                                ) -> ::std::result::Result<Self, D::Error>
                                where
                                    D: ::serde::de::Deserializer<'de>,
                                {
                                    let value: &'de str = <&str>::deserialize(deserializer)?;
                                    Ok(match value {
                                        "cpu" => ListFilterItems::Cpu,
                                        "graphics" => ListFilterItems::Graphics,
                                        "memory" => ListFilterItems::Memory,
                                        "network" => ListFilterItems::Network,
                                        "perfMetricTypeUnspecified" => {
                                            ListFilterItems::PerfMetricTypeUnspecified
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
                            impl ::field_selector::FieldSelector for ListFilterItems {
                                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                                    match selector.chars().rev().nth(0) {
                                        Some(',') | None => {}
                                        _ => selector.push_str(","),
                                    }
                                    selector.push_str(ident);
                                }
                            }
                        }
                        pub struct PerfSampleSeriesActions<'a, A> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a std::sync::Mutex<A>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> PerfSampleSeriesActions<'a, A> {
                            #[doc = "Creates a PerfSampleSeries.\n\nMay return any of the following error code(s): - ALREADY_EXISTS - PerfMetricSummary already exists for the given Step - NOT_FOUND - The containing Step does not exist"]
                            pub fn create(
                                &self,
                                request: crate::schemas::PerfSampleSeries,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> CreateRequestBuilder<A> {
                                CreateRequestBuilder {
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
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                }
                            }
                            #[doc = "Gets a PerfSampleSeries.\n\nMay return any of the following error code(s): - NOT_FOUND - The specified PerfSampleSeries does not exist"]
                            pub fn get(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                                sample_series_id: impl Into<String>,
                            ) -> GetRequestBuilder<A> {
                                GetRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    alt: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    user_ip: None,
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    sample_series_id: sample_series_id.into(),
                                }
                            }
                            #[doc = "Lists PerfSampleSeries for a given Step.\n\nThe request provides an optional filter which specifies one or more PerfMetricsType to include in the result; if none returns all. The resulting PerfSampleSeries are sorted by ids.\n\nMay return any of the following canonical error codes: - NOT_FOUND - The containing Step does not exist"]
                            pub fn list(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> ListRequestBuilder<A> {
                                ListRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    alt: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    user_ip: None,
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    filter: None,
                                }
                            }
                            #[doc = "Actions that can be performed on the samples resource"]pub fn samples ( & self ) -> crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: samples :: SamplesActions < A >{
                                crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: samples :: SamplesActions { reqwest : & self . reqwest , auth : & self . auth }
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct CreateRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            request: crate::schemas::PerfSampleSeries,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            alt: Option<crate::params::Alt>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            user_ip: Option<String>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                            ) -> Result<
                                crate::schemas::PerfSampleSeries,
                                Box<dyn ::std::error::Error>,
                            > {
                                self.execute_fields::<_, &str>(None)
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub fn execute_debug(
                                self,
                            ) -> Result<
                                crate::schemas::PerfSampleSeries,
                                Box<dyn ::std::error::Error>,
                            > {
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
                                let mut output =
                                    "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                        .to_owned();
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfSampleSeries");
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::POST, path);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("userIp", &self.user_ip)]);
                                let mut auth = self.auth.lock().unwrap();
                                let fut = auth
                                    .token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                let token = runtime.block_on(fut).unwrap().access_token;
                                let req = req.bearer_auth(&token);
                                req
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct GetRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            sample_series_id: String,
                            alt: Option<crate::params::Alt>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            user_ip: Option<String>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
                            ) -> Result<
                                crate::schemas::PerfSampleSeries,
                                Box<dyn ::std::error::Error>,
                            > {
                                self.execute_fields::<_, &str>(None)
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub fn execute_debug(
                                self,
                            ) -> Result<
                                crate::schemas::PerfSampleSeries,
                                Box<dyn ::std::error::Error>,
                            > {
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
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                        .to_owned();
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfSampleSeries/");
                                {
                                    let var_as_str = &self.sample_series_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("userIp", &self.user_ip)]);
                                let mut auth = self.auth.lock().unwrap();
                                let fut = auth
                                    .token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                let token = runtime.block_on(fut).unwrap().access_token;
                                let req = req.bearer_auth(&token);
                                req
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a :: std :: sync :: Mutex < A > , project_id : String , history_id : String , execution_id : String , step_id : String , filter : Option < Vec < crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: params :: ListFilterItems > > , alt : Option < crate :: params :: Alt > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , user_ip : Option < String > , }
                        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                            #[doc = "Specify one or more PerfMetricType values such as CPU to filter the result"]
                            pub fn filter(
                                mut self,
                                value : impl Into < Vec < crate :: resources :: projects :: histories :: executions :: steps :: perf_sample_series :: params :: ListFilterItems > >,
                            ) -> Self {
                                self.filter = Some(value.into());
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
                            ) -> Result<
                                crate::schemas::ListPerfSampleSeriesResponse,
                                Box<dyn ::std::error::Error>,
                            > {
                                self.execute_fields::<_, &str>(None)
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub fn execute_debug(
                                self,
                            ) -> Result<
                                crate::schemas::ListPerfSampleSeriesResponse,
                                Box<dyn ::std::error::Error>,
                            > {
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
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                        .to_owned();
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/perfSampleSeries");
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("filter", &self.filter)]);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("userIp", &self.user_ip)]);
                                let mut auth = self.auth.lock().unwrap();
                                let fut = auth
                                    .token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                let token = runtime.block_on(fut).unwrap().access_token;
                                let req = req.bearer_auth(&token);
                                req
                            }
                        }
                        pub mod samples {
                            pub mod params {}
                            pub struct SamplesActions<'a, A> {
                                pub(crate) reqwest: &'a reqwest::Client,
                                pub(crate) auth: &'a std::sync::Mutex<A>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> SamplesActions<'a, A> {
                                #[doc = "Creates a batch of PerfSamples - a client can submit multiple batches of Perf Samples through repeated calls to this method in order to split up a large request payload - duplicates and existing timestamp entries will be ignored. - the batch operation may partially succeed - the set of elements successfully inserted is returned in the response (omits items which already existed in the database).\n\nMay return any of the following canonical error codes: - NOT_FOUND - The containing PerfSampleSeries does not exist"]
                                pub fn batch_create(
                                    &self,
                                    request: crate::schemas::BatchCreatePerfSamplesRequest,
                                    project_id: impl Into<String>,
                                    history_id: impl Into<String>,
                                    execution_id: impl Into<String>,
                                    step_id: impl Into<String>,
                                    sample_series_id: impl Into<String>,
                                ) -> BatchCreateRequestBuilder<A> {
                                    BatchCreateRequestBuilder {
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
                                        project_id: project_id.into(),
                                        history_id: history_id.into(),
                                        execution_id: execution_id.into(),
                                        step_id: step_id.into(),
                                        sample_series_id: sample_series_id.into(),
                                    }
                                }
                                #[doc = "Lists the Performance Samples of a given Sample Series - The list results are sorted by timestamps ascending - The default page size is 500 samples; and maximum size allowed 5000 - The response token indicates the last returned PerfSample timestamp - When the results size exceeds the page size, submit a subsequent request including the page token to return the rest of the samples up to the page limit\n\nMay return any of the following canonical error codes: - OUT_OF_RANGE - The specified request page_token is out of valid range - NOT_FOUND - The containing PerfSampleSeries does not exist"]
                                pub fn list(
                                    &self,
                                    project_id: impl Into<String>,
                                    history_id: impl Into<String>,
                                    execution_id: impl Into<String>,
                                    step_id: impl Into<String>,
                                    sample_series_id: impl Into<String>,
                                ) -> ListRequestBuilder<A> {
                                    ListRequestBuilder {
                                        reqwest: &self.reqwest,
                                        auth: &self.auth,
                                        alt: None,
                                        fields: None,
                                        key: None,
                                        oauth_token: None,
                                        pretty_print: None,
                                        quota_user: None,
                                        user_ip: None,
                                        project_id: project_id.into(),
                                        history_id: history_id.into(),
                                        execution_id: execution_id.into(),
                                        step_id: step_id.into(),
                                        sample_series_id: sample_series_id.into(),
                                        page_size: None,
                                        page_token: None,
                                    }
                                }
                            }
                            #[derive(Debug, Clone)]
                            pub struct BatchCreateRequestBuilder<'a, A> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                request: crate::schemas::BatchCreatePerfSamplesRequest,
                                project_id: String,
                                history_id: String,
                                execution_id: String,
                                step_id: String,
                                sample_series_id: String,
                                alt: Option<crate::params::Alt>,
                                fields: Option<String>,
                                key: Option<String>,
                                oauth_token: Option<String>,
                                pretty_print: Option<bool>,
                                quota_user: Option<String>,
                                user_ip: Option<String>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> BatchCreateRequestBuilder<'a, A> {
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
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
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
                                ) -> Result<
                                    crate::schemas::BatchCreatePerfSamplesResponse,
                                    Box<dyn ::std::error::Error>,
                                > {
                                    self.execute_fields::<_, &str>(None)
                                }
                                #[doc = r" Execute the given operation. This will provide a `fields`"]
                                #[doc = r" selector of `*`. This will include every attribute of the"]
                                #[doc = r" response resource and should be limited to use during"]
                                #[doc = r" development or debugging."]
                                pub fn execute_debug(
                                    self,
                                ) -> Result<
                                    crate::schemas::BatchCreatePerfSamplesResponse,
                                    Box<dyn ::std::error::Error>,
                                > {
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
                                    let mut output =
                                        "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                            .to_owned();
                                    {
                                        let var_as_str = &self.project_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/histories/");
                                    {
                                        let var_as_str = &self.history_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/executions/");
                                    {
                                        let var_as_str = &self.execution_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/steps/");
                                    {
                                        let var_as_str = &self.step_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/perfSampleSeries/");
                                    {
                                        let var_as_str = &self.sample_series_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/samples:batchCreate");
                                    output
                                }
                                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                                    let req = req.query(&[("alt", &self.alt)]);
                                    let req = req.query(&[("fields", &self.fields)]);
                                    let req = req.query(&[("key", &self.key)]);
                                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                                    let req = req.query(&[("userIp", &self.user_ip)]);
                                    let mut auth = self.auth.lock().unwrap();
                                    let fut = auth.token(vec![
                                        "https://www.googleapis.com/auth/cloud-platform",
                                    ]);
                                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                    let token = runtime.block_on(fut).unwrap().access_token;
                                    let req = req.bearer_auth(&token);
                                    req
                                }
                            }
                            #[derive(Debug, Clone)]
                            pub struct ListRequestBuilder<'a, A> {
                                pub(crate) reqwest: &'a ::reqwest::Client,
                                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                                project_id: String,
                                history_id: String,
                                execution_id: String,
                                step_id: String,
                                sample_series_id: String,
                                page_size: Option<i32>,
                                page_token: Option<String>,
                                alt: Option<crate::params::Alt>,
                                fields: Option<String>,
                                key: Option<String>,
                                oauth_token: Option<String>,
                                pretty_print: Option<bool>,
                                quota_user: Option<String>,
                                user_ip: Option<String>,
                            }
                            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                                #[doc = "The default page size is 500 samples, and the maximum size is 5000. If the page_size is greater than 5000, the effective page size will be 5000"]
                                pub fn page_size(mut self, value: i32) -> Self {
                                    self.page_size = Some(value);
                                    self
                                }
                                #[doc = "Optional, the next_page_token returned in the previous response"]
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
                                pub fn iter_perf_samples<T>(
                                    mut self,
                                ) -> crate::iter::PageItemIter<Self, T>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    let mut fields =
                                        concat!("nextPageToken,", "perfSamples").to_owned();
                                    let items_fields = T::field_selector();
                                    if !items_fields.is_empty() {
                                        fields.push_str("(");
                                        fields.push_str(&items_fields);
                                        fields.push_str(")");
                                    }
                                    self.fields = Some(fields);
                                    crate::iter::PageItemIter::new(self, "perfSamples")
                                }
                                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                                #[doc = r" the server."]
                                pub fn iter_perf_samples_standard(
                                    mut self,
                                ) -> crate::iter::PageItemIter<Self, crate::schemas::PerfSample>
                                {
                                    self.fields =
                                        Some(concat!("nextPageToken,", "perfSamples").to_owned());
                                    crate::iter::PageItemIter::new(self, "perfSamples")
                                }
                                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                                #[doc = r" primarily be used during developement and debugging as fetching"]
                                #[doc = r" all fields can be expensive both in bandwidth and server"]
                                #[doc = r" resources."]
                                pub fn iter_perf_samples_debug(
                                    mut self,
                                ) -> crate::iter::PageItemIter<Self, crate::schemas::PerfSample>
                                {
                                    self.fields = Some(
                                        concat!("nextPageToken,", "perfSamples", "(*)").to_owned(),
                                    );
                                    crate::iter::PageItemIter::new(self, "perfSamples")
                                }
                                pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                                where
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
                                {
                                    let mut fields = T::field_selector();
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
                                pub fn iter_standard(
                                    self,
                                ) -> crate::iter::PageIter<
                                    Self,
                                    crate::schemas::ListPerfSamplesResponse,
                                > {
                                    crate::iter::PageIter::new(self)
                                }
                                pub fn iter_debug(
                                    mut self,
                                ) -> crate::iter::PageIter<
                                    Self,
                                    crate::schemas::ListPerfSamplesResponse,
                                > {
                                    self.fields = Some("*".to_owned());
                                    crate::iter::PageIter::new(self)
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
                                    T: ::serde::de::DeserializeOwned
                                        + ::field_selector::FieldSelector,
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
                                ) -> Result<
                                    crate::schemas::ListPerfSamplesResponse,
                                    Box<dyn ::std::error::Error>,
                                > {
                                    self.execute_fields::<_, &str>(None)
                                }
                                #[doc = r" Execute the given operation. This will provide a `fields`"]
                                #[doc = r" selector of `*`. This will include every attribute of the"]
                                #[doc = r" response resource and should be limited to use during"]
                                #[doc = r" development or debugging."]
                                pub fn execute_debug(
                                    self,
                                ) -> Result<
                                    crate::schemas::ListPerfSamplesResponse,
                                    Box<dyn ::std::error::Error>,
                                > {
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
                                    Ok(req.send()?.error_for_status()?.json()?)
                                }
                                fn _path(&self) -> String {
                                    let mut output =
                                        "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                            .to_owned();
                                    {
                                        let var_as_str = &self.project_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/histories/");
                                    {
                                        let var_as_str = &self.history_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/executions/");
                                    {
                                        let var_as_str = &self.execution_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/steps/");
                                    {
                                        let var_as_str = &self.step_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/perfSampleSeries/");
                                    {
                                        let var_as_str = &self.sample_series_id;
                                        output.extend(::percent_encoding::utf8_percent_encode(
                                            &var_as_str,
                                            crate::SIMPLE,
                                        ));
                                    }
                                    output.push_str("/samples");
                                    output
                                }
                                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                                    let req = req.query(&[("pageSize", &self.page_size)]);
                                    let req = req.query(&[("pageToken", &self.page_token)]);
                                    let req = req.query(&[("alt", &self.alt)]);
                                    let req = req.query(&[("fields", &self.fields)]);
                                    let req = req.query(&[("key", &self.key)]);
                                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                                    let req = req.query(&[("userIp", &self.user_ip)]);
                                    let mut auth = self.auth.lock().unwrap();
                                    let fut = auth.token(vec![
                                        "https://www.googleapis.com/auth/cloud-platform",
                                    ]);
                                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                    let token = runtime.block_on(fut).unwrap().access_token;
                                    let req = req.bearer_auth(&token);
                                    req
                                }
                            }
                            impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                                fn set_page_token(&mut self, value: String) {
                                    self.page_token = value.into();
                                }
                                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                                where
                                    T: ::serde::de::DeserializeOwned,
                                {
                                    self._execute()
                                }
                            }
                        }
                    }
                    pub mod test_cases {
                        pub mod params {}
                        pub struct TestCasesActions<'a, A> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a std::sync::Mutex<A>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> TestCasesActions<'a, A> {
                            #[doc = "Gets details of a Test Case for a Step. Experimental test cases API. Still in active development.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing Test Case does not exist"]
                            pub fn get(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                                test_case_id: impl Into<String>,
                            ) -> GetRequestBuilder<A> {
                                GetRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    alt: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    user_ip: None,
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    test_case_id: test_case_id.into(),
                                }
                            }
                            #[doc = "Lists Test Cases attached to a Step. Experimental test cases API. Still in active development.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing Step does not exist"]
                            pub fn list(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> ListRequestBuilder<A> {
                                ListRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    alt: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    user_ip: None,
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    page_size: None,
                                    page_token: None,
                                }
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct GetRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            test_case_id: String,
                            alt: Option<crate::params::Alt>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            user_ip: Option<String>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
                            ) -> Result<crate::schemas::TestCase, Box<dyn ::std::error::Error>>
                            {
                                self.execute_fields::<_, &str>(None)
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub fn execute_debug(
                                self,
                            ) -> Result<crate::schemas::TestCase, Box<dyn ::std::error::Error>>
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
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                        .to_owned();
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/testCases/");
                                {
                                    let var_as_str = &self.test_case_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("userIp", &self.user_ip)]);
                                let mut auth = self.auth.lock().unwrap();
                                let fut = auth
                                    .token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                let token = runtime.block_on(fut).unwrap().access_token;
                                let req = req.bearer_auth(&token);
                                req
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            page_size: Option<i32>,
                            page_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            user_ip: Option<String>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                            #[doc = "The maximum number of TestCases to fetch.\n\nDefault value: 100. The server will use this default if the field is not set or has a value of 0.\n\nOptional."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "A continuation token to resume the query at the next item.\n\nOptional."]
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
                            pub fn iter_test_cases<T>(
                                mut self,
                            ) -> crate::iter::PageItemIter<Self, T>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                let mut fields = concat!("nextPageToken,", "testCases").to_owned();
                                let items_fields = T::field_selector();
                                if !items_fields.is_empty() {
                                    fields.push_str("(");
                                    fields.push_str(&items_fields);
                                    fields.push_str(")");
                                }
                                self.fields = Some(fields);
                                crate::iter::PageItemIter::new(self, "testCases")
                            }
                            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                            #[doc = r" fields in `#items_type` will be the default fields populated by"]
                            #[doc = r" the server."]
                            pub fn iter_test_cases_standard(
                                mut self,
                            ) -> crate::iter::PageItemIter<Self, crate::schemas::TestCase>
                            {
                                self.fields =
                                    Some(concat!("nextPageToken,", "testCases").to_owned());
                                crate::iter::PageItemIter::new(self, "testCases")
                            }
                            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                            #[doc = r" fields in `#items_type` will be all fields available. This should"]
                            #[doc = r" primarily be used during developement and debugging as fetching"]
                            #[doc = r" all fields can be expensive both in bandwidth and server"]
                            #[doc = r" resources."]
                            pub fn iter_test_cases_debug(
                                mut self,
                            ) -> crate::iter::PageItemIter<Self, crate::schemas::TestCase>
                            {
                                self.fields =
                                    Some(concat!("nextPageToken,", "testCases", "(*)").to_owned());
                                crate::iter::PageItemIter::new(self, "testCases")
                            }
                            pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                let mut fields = T::field_selector();
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
                            pub fn iter_standard(
                                self,
                            ) -> crate::iter::PageIter<Self, crate::schemas::ListTestCasesResponse>
                            {
                                crate::iter::PageIter::new(self)
                            }
                            pub fn iter_debug(
                                mut self,
                            ) -> crate::iter::PageIter<Self, crate::schemas::ListTestCasesResponse>
                            {
                                self.fields = Some("*".to_owned());
                                crate::iter::PageIter::new(self)
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
                            ) -> Result<
                                crate::schemas::ListTestCasesResponse,
                                Box<dyn ::std::error::Error>,
                            > {
                                self.execute_fields::<_, &str>(None)
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub fn execute_debug(
                                self,
                            ) -> Result<
                                crate::schemas::ListTestCasesResponse,
                                Box<dyn ::std::error::Error>,
                            > {
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
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                        .to_owned();
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/testCases");
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("pageSize", &self.page_size)]);
                                let req = req.query(&[("pageToken", &self.page_token)]);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("userIp", &self.user_ip)]);
                                let mut auth = self.auth.lock().unwrap();
                                let fut = auth
                                    .token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                let token = runtime.block_on(fut).unwrap().access_token;
                                let req = req.bearer_auth(&token);
                                req
                            }
                        }
                        impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                            fn set_page_token(&mut self, value: String) {
                                self.page_token = value.into();
                            }
                            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                self._execute()
                            }
                        }
                    }
                    pub mod thumbnails {
                        pub mod params {}
                        pub struct ThumbnailsActions<'a, A> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a std::sync::Mutex<A>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> ThumbnailsActions<'a, A> {
                            #[doc = "Lists thumbnails of images attached to a step.\n\nMay return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read from the project, or from any of the images - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the step does not exist, or if any of the images do not exist"]
                            pub fn list(
                                &self,
                                project_id: impl Into<String>,
                                history_id: impl Into<String>,
                                execution_id: impl Into<String>,
                                step_id: impl Into<String>,
                            ) -> ListRequestBuilder<A> {
                                ListRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: &self.auth,
                                    alt: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    user_ip: None,
                                    project_id: project_id.into(),
                                    history_id: history_id.into(),
                                    execution_id: execution_id.into(),
                                    step_id: step_id.into(),
                                    page_size: None,
                                    page_token: None,
                                }
                            }
                        }
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder<'a, A> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a ::std::sync::Mutex<A>,
                            project_id: String,
                            history_id: String,
                            execution_id: String,
                            step_id: String,
                            page_size: Option<i32>,
                            page_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            user_ip: Option<String>,
                        }
                        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                            #[doc = "The maximum number of thumbnails to fetch.\n\nDefault value: 50. The server will use this default if the field is not set or has a value of 0.\n\nOptional."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "A continuation token to resume the query at the next item.\n\nOptional."]
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
                            pub fn iter_thumbnails<T>(
                                mut self,
                            ) -> crate::iter::PageItemIter<Self, T>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                let mut fields = concat!("nextPageToken,", "thumbnails").to_owned();
                                let items_fields = T::field_selector();
                                if !items_fields.is_empty() {
                                    fields.push_str("(");
                                    fields.push_str(&items_fields);
                                    fields.push_str(")");
                                }
                                self.fields = Some(fields);
                                crate::iter::PageItemIter::new(self, "thumbnails")
                            }
                            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                            #[doc = r" fields in `#items_type` will be the default fields populated by"]
                            #[doc = r" the server."]
                            pub fn iter_thumbnails_standard(
                                mut self,
                            ) -> crate::iter::PageItemIter<Self, crate::schemas::Image>
                            {
                                self.fields =
                                    Some(concat!("nextPageToken,", "thumbnails").to_owned());
                                crate::iter::PageItemIter::new(self, "thumbnails")
                            }
                            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                            #[doc = r" fields in `#items_type` will be all fields available. This should"]
                            #[doc = r" primarily be used during developement and debugging as fetching"]
                            #[doc = r" all fields can be expensive both in bandwidth and server"]
                            #[doc = r" resources."]
                            pub fn iter_thumbnails_debug(
                                mut self,
                            ) -> crate::iter::PageItemIter<Self, crate::schemas::Image>
                            {
                                self.fields =
                                    Some(concat!("nextPageToken,", "thumbnails", "(*)").to_owned());
                                crate::iter::PageItemIter::new(self, "thumbnails")
                            }
                            pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                            where
                                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                            {
                                let mut fields = T::field_selector();
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
                            pub fn iter_standard(
                                self,
                            ) -> crate::iter::PageIter<
                                Self,
                                crate::schemas::ListStepThumbnailsResponse,
                            > {
                                crate::iter::PageIter::new(self)
                            }
                            pub fn iter_debug(
                                mut self,
                            ) -> crate::iter::PageIter<
                                Self,
                                crate::schemas::ListStepThumbnailsResponse,
                            > {
                                self.fields = Some("*".to_owned());
                                crate::iter::PageIter::new(self)
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
                            ) -> Result<
                                crate::schemas::ListStepThumbnailsResponse,
                                Box<dyn ::std::error::Error>,
                            > {
                                self.execute_fields::<_, &str>(None)
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub fn execute_debug(
                                self,
                            ) -> Result<
                                crate::schemas::ListStepThumbnailsResponse,
                                Box<dyn ::std::error::Error>,
                            > {
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
                                Ok(req.send()?.error_for_status()?.json()?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://www.googleapis.com/toolresults/v1beta3/projects/"
                                        .to_owned();
                                {
                                    let var_as_str = &self.project_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/histories/");
                                {
                                    let var_as_str = &self.history_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/executions/");
                                {
                                    let var_as_str = &self.execution_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/steps/");
                                {
                                    let var_as_str = &self.step_id;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::SIMPLE,
                                    ));
                                }
                                output.push_str("/thumbnails");
                                output
                            }
                            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                                let req = self.reqwest.request(::reqwest::Method::GET, path);
                                let req = req.query(&[("pageSize", &self.page_size)]);
                                let req = req.query(&[("pageToken", &self.page_token)]);
                                let req = req.query(&[("alt", &self.alt)]);
                                let req = req.query(&[("fields", &self.fields)]);
                                let req = req.query(&[("key", &self.key)]);
                                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                let req = req.query(&[("quotaUser", &self.quota_user)]);
                                let req = req.query(&[("userIp", &self.user_ip)]);
                                let mut auth = self.auth.lock().unwrap();
                                let fut = auth
                                    .token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                                let token = runtime.block_on(fut).unwrap().access_token;
                                let req = req.bearer_auth(&token);
                                req
                            }
                        }
                        impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                            fn set_page_token(&mut self, value: String) {
                                self.page_token = value.into();
                            }
                            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                self._execute()
                            }
                        }
                    }
                }
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
#[allow(dead_code)]
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
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
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
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
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
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
                                return Some(Err(format!(
                                    "no {} field found in iter response",
                                    self.items_field
                                )
                                .into()))
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
