pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ApproximateProgress {
        #[doc = "Obsolete."]
        #[serde(rename = "percentComplete", default)]
        pub percent_complete: Option<f32>,
        #[doc = "Obsolete."]
        #[serde(rename = "position", default)]
        pub position: Option<crate::schemas::Position>,
        #[doc = "Obsolete."]
        #[serde(rename = "remainingTime", default)]
        pub remaining_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for ApproximateProgress {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ApproximateReportedProgress {
        #[doc = "Total amount of parallelism in the portion of input of this task that has\nalready been consumed and is no longer active. In the first two examples\nabove (see remaining_parallelism), the value should be 29 or 2\nrespectively.  The sum of remaining_parallelism and consumed_parallelism\nshould equal the total amount of parallelism in this work item.  If\nspecified, must be finite."]
        #[serde(rename = "consumedParallelism", default)]
        pub consumed_parallelism: Option<crate::schemas::ReportedParallelism>,
        #[doc = "Completion as fraction of the input consumed, from 0.0 (beginning, nothing\nconsumed), to 1.0 (end of the input, entire input consumed)."]
        #[serde(rename = "fractionConsumed", default)]
        pub fraction_consumed: Option<f64>,
        #[doc = "A Position within the work to represent a progress."]
        #[serde(rename = "position", default)]
        pub position: Option<crate::schemas::Position>,
        #[doc = "Total amount of parallelism in the input of this task that remains,\n(i.e. can be delegated to this task and any new tasks via dynamic\nsplitting). Always at least 1 for non-finished work items and 0 for\nfinished.\n\n\"Amount of parallelism\" refers to how many non-empty parts of the input\ncan be read in parallel. This does not necessarily equal number\nof records. An input that can be read in parallel down to the\nindividual records is called \"perfectly splittable\".\nAn example of non-perfectly parallelizable input is a block-compressed\nfile format where a block of records has to be read as a whole,\nbut different blocks can be read in parallel.\n\nExamples:\n* If we are processing record #30 (starting at 1) out of 50 in a perfectly\n  splittable 50-record input, this value should be 21 (20 remaining + 1\n  current).\n* If we are reading through block 3 in a block-compressed file consisting\n  of 5 blocks, this value should be 3 (since blocks 4 and 5 can be\n  processed in parallel by new tasks via dynamic splitting and the current\n  task remains processing block 3).\n* If we are reading through the last block in a block-compressed file,\n  or reading or processing the last record in a perfectly splittable\n  input, this value should be 1, because apart from the current task, no\n  additional remainder can be split off."]
        #[serde(rename = "remainingParallelism", default)]
        pub remaining_parallelism: Option<crate::schemas::ReportedParallelism>,
    }
    impl ::field_selector::FieldSelector for ApproximateReportedProgress {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ApproximateSplitRequest {
        #[doc = "A fraction at which to split the work item, from 0.0 (beginning of the\ninput) to 1.0 (end of the input)."]
        #[serde(rename = "fractionConsumed", default)]
        pub fraction_consumed: Option<f64>,
        #[doc = "The fraction of the remainder of work to split the work item at, from 0.0\n(split at the current position) to 1.0 (end of the input)."]
        #[serde(rename = "fractionOfRemainder", default)]
        pub fraction_of_remainder: Option<f64>,
        #[doc = "A Position at which to split the work item."]
        #[serde(rename = "position", default)]
        pub position: Option<crate::schemas::Position>,
    }
    impl ::field_selector::FieldSelector for ApproximateSplitRequest {
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
    pub enum AutoscalingEventEventType {
        #[doc = "Default type for the enum.  Value should never be returned."]
        TypeUnknown,
        #[doc = "The TARGET_NUM_WORKERS_CHANGED type should be used when the target\nworker pool size has changed at the start of an actuation. An event\nshould always be specified as TARGET_NUM_WORKERS_CHANGED if it reflects\na change in the target_num_workers."]
        TargetNumWorkersChanged,
        #[doc = "The CURRENT_NUM_WORKERS_CHANGED type should be used when actual worker\npool size has been changed, but the target_num_workers has not changed."]
        CurrentNumWorkersChanged,
        #[doc = "The ACTUATION_FAILURE type should be used when we want to report\nan error to the user indicating why the current number of workers\nin the pool could not be changed.\nDisplayed in the current status and history widgets."]
        ActuationFailure,
        #[doc = "Used when we want to report to the user a reason why we are\nnot currently adjusting the number of workers.\nShould specify both target_num_workers, current_num_workers and a\ndecision_message."]
        NoChange,
    }
    impl AutoscalingEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                AutoscalingEventEventType::TypeUnknown => "TYPE_UNKNOWN",
                AutoscalingEventEventType::TargetNumWorkersChanged => "TARGET_NUM_WORKERS_CHANGED",
                AutoscalingEventEventType::CurrentNumWorkersChanged => {
                    "CURRENT_NUM_WORKERS_CHANGED"
                }
                AutoscalingEventEventType::ActuationFailure => "ACTUATION_FAILURE",
                AutoscalingEventEventType::NoChange => "NO_CHANGE",
            }
        }
    }
    impl ::std::fmt::Display for AutoscalingEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AutoscalingEventEventType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AutoscalingEventEventType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNKNOWN" => AutoscalingEventEventType::TypeUnknown,
                "TARGET_NUM_WORKERS_CHANGED" => AutoscalingEventEventType::TargetNumWorkersChanged,
                "CURRENT_NUM_WORKERS_CHANGED" => {
                    AutoscalingEventEventType::CurrentNumWorkersChanged
                }
                "ACTUATION_FAILURE" => AutoscalingEventEventType::ActuationFailure,
                "NO_CHANGE" => AutoscalingEventEventType::NoChange,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AutoscalingEvent {
        #[doc = "The current number of workers the job has."]
        #[serde(rename = "currentNumWorkers", default)]
        #[serde(with = "crate::parsed_string")]
        pub current_num_workers: Option<i64>,
        #[doc = "A message describing why the system decided to adjust the current\nnumber of workers, why it failed, or why the system decided to\nnot make any changes to the number of workers."]
        #[serde(rename = "description", default)]
        pub description: Option<crate::schemas::StructuredMessage>,
        #[doc = "The type of autoscaling event to report."]
        #[serde(rename = "eventType", default)]
        pub event_type: Option<crate::schemas::AutoscalingEventEventType>,
        #[doc = "The target number of workers the worker pool wants to resize to use."]
        #[serde(rename = "targetNumWorkers", default)]
        #[serde(with = "crate::parsed_string")]
        pub target_num_workers: Option<i64>,
        #[doc = "The time this event was emitted to indicate a new target or current\nnum_workers value."]
        #[serde(rename = "time", default)]
        pub time: Option<String>,
        #[doc = "A short and friendly name for the worker pool this event refers to,\npopulated from the value of PoolStageRelation::user_pool_name."]
        #[serde(rename = "workerPool", default)]
        pub worker_pool: Option<String>,
    }
    impl ::field_selector::FieldSelector for AutoscalingEvent {
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
    pub enum AutoscalingSettingsAlgorithm {
        #[doc = "The algorithm is unknown, or unspecified."]
        AutoscalingAlgorithmUnknown,
        #[doc = "Disable autoscaling."]
        AutoscalingAlgorithmNone,
        #[doc = "Increase worker count over time to reduce job execution time."]
        AutoscalingAlgorithmBasic,
    }
    impl AutoscalingSettingsAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                AutoscalingSettingsAlgorithm::AutoscalingAlgorithmUnknown => {
                    "AUTOSCALING_ALGORITHM_UNKNOWN"
                }
                AutoscalingSettingsAlgorithm::AutoscalingAlgorithmNone => {
                    "AUTOSCALING_ALGORITHM_NONE"
                }
                AutoscalingSettingsAlgorithm::AutoscalingAlgorithmBasic => {
                    "AUTOSCALING_ALGORITHM_BASIC"
                }
            }
        }
    }
    impl ::std::fmt::Display for AutoscalingSettingsAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AutoscalingSettingsAlgorithm {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AutoscalingSettingsAlgorithm {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTOSCALING_ALGORITHM_UNKNOWN" => {
                    AutoscalingSettingsAlgorithm::AutoscalingAlgorithmUnknown
                }
                "AUTOSCALING_ALGORITHM_NONE" => {
                    AutoscalingSettingsAlgorithm::AutoscalingAlgorithmNone
                }
                "AUTOSCALING_ALGORITHM_BASIC" => {
                    AutoscalingSettingsAlgorithm::AutoscalingAlgorithmBasic
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
    pub struct AutoscalingSettings {
        #[doc = "The algorithm to use for autoscaling."]
        #[serde(rename = "algorithm", default)]
        pub algorithm: Option<crate::schemas::AutoscalingSettingsAlgorithm>,
        #[doc = "The maximum number of workers to cap scaling at."]
        #[serde(rename = "maxNumWorkers", default)]
        pub max_num_workers: Option<i32>,
    }
    impl ::field_selector::FieldSelector for AutoscalingSettings {
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
    pub struct BigQueryIODetails {
        #[doc = "Dataset accessed in the connection."]
        #[serde(rename = "dataset", default)]
        pub dataset: Option<String>,
        #[doc = "Project accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Query used to access data in the connection."]
        #[serde(rename = "query", default)]
        pub query: Option<String>,
        #[doc = "Table accessed in the connection."]
        #[serde(rename = "table", default)]
        pub table: Option<String>,
    }
    impl ::field_selector::FieldSelector for BigQueryIODetails {
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
    pub struct BigTableIODetails {
        #[doc = "InstanceId accessed in the connection."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "ProjectId accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "TableId accessed in the connection."]
        #[serde(rename = "tableId", default)]
        pub table_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for BigTableIODetails {
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
    pub struct ComponentSource {
        #[doc = "Dataflow service generated name for this source."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User name for the original user transform or collection with which this\nsource is most closely associated."]
        #[serde(rename = "originalTransformOrCollection", default)]
        pub original_transform_or_collection: Option<String>,
        #[doc = "Human-readable name for this transform; may be user or system generated."]
        #[serde(rename = "userName", default)]
        pub user_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ComponentSource {
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
    pub struct ComponentTransform {
        #[doc = "Dataflow service generated name for this source."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User name for the original user transform with which this transform is\nmost closely associated."]
        #[serde(rename = "originalTransform", default)]
        pub original_transform: Option<String>,
        #[doc = "Human-readable name for this transform; may be user or system generated."]
        #[serde(rename = "userName", default)]
        pub user_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ComponentTransform {
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
    pub struct ComputationTopology {
        #[doc = "The ID of the computation."]
        #[serde(rename = "computationId", default)]
        pub computation_id: Option<String>,
        #[doc = "The inputs to the computation."]
        #[serde(rename = "inputs", default)]
        pub inputs: Option<Vec<crate::schemas::StreamLocation>>,
        #[doc = "The key ranges processed by the computation."]
        #[serde(rename = "keyRanges", default)]
        pub key_ranges: Option<Vec<crate::schemas::KeyRangeLocation>>,
        #[doc = "The outputs from the computation."]
        #[serde(rename = "outputs", default)]
        pub outputs: Option<Vec<crate::schemas::StreamLocation>>,
        #[doc = "The state family values."]
        #[serde(rename = "stateFamilies", default)]
        pub state_families: Option<Vec<crate::schemas::StateFamilyConfig>>,
        #[doc = "The system stage name."]
        #[serde(rename = "systemStageName", default)]
        pub system_stage_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ComputationTopology {
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
    pub struct ConcatPosition {
        #[doc = "Index of the inner source."]
        #[serde(rename = "index", default)]
        pub index: Option<i32>,
        #[doc = "Position within the inner source."]
        #[serde(rename = "position", default)]
        pub position: Option<crate::schemas::Position>,
    }
    impl ::field_selector::FieldSelector for ConcatPosition {
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
    pub enum CounterMetadataKind {
        #[doc = "Counter aggregation kind was not set."]
        Invalid,
        #[doc = "Aggregated value is the sum of all contributed values."]
        Sum,
        #[doc = "Aggregated value is the max of all contributed values."]
        Max,
        #[doc = "Aggregated value is the min of all contributed values."]
        Min,
        #[doc = "Aggregated value is the mean of all contributed values."]
        Mean,
        #[doc = "Aggregated value represents the logical 'or' of all contributed values."]
        Or,
        #[doc = "Aggregated value represents the logical 'and' of all contributed values."]
        And,
        #[doc = "Aggregated value is a set of unique contributed values."]
        Set,
        #[doc = "Aggregated value captures statistics about a distribution."]
        Distribution,
        #[doc = "Aggregated value tracks the latest value of a variable."]
        LatestValue,
    }
    impl CounterMetadataKind {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterMetadataKind::Invalid => "INVALID",
                CounterMetadataKind::Sum => "SUM",
                CounterMetadataKind::Max => "MAX",
                CounterMetadataKind::Min => "MIN",
                CounterMetadataKind::Mean => "MEAN",
                CounterMetadataKind::Or => "OR",
                CounterMetadataKind::And => "AND",
                CounterMetadataKind::Set => "SET",
                CounterMetadataKind::Distribution => "DISTRIBUTION",
                CounterMetadataKind::LatestValue => "LATEST_VALUE",
            }
        }
    }
    impl ::std::fmt::Display for CounterMetadataKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterMetadataKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterMetadataKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVALID" => CounterMetadataKind::Invalid,
                "SUM" => CounterMetadataKind::Sum,
                "MAX" => CounterMetadataKind::Max,
                "MIN" => CounterMetadataKind::Min,
                "MEAN" => CounterMetadataKind::Mean,
                "OR" => CounterMetadataKind::Or,
                "AND" => CounterMetadataKind::And,
                "SET" => CounterMetadataKind::Set,
                "DISTRIBUTION" => CounterMetadataKind::Distribution,
                "LATEST_VALUE" => CounterMetadataKind::LatestValue,
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
    pub enum CounterMetadataStandardUnits {
        #[doc = "Counter returns a value in bytes."]
        Bytes,
        #[doc = "Counter returns a value in bytes per second."]
        BytesPerSec,
        #[doc = "Counter returns a value in milliseconds."]
        Milliseconds,
        #[doc = "Counter returns a value in microseconds."]
        Microseconds,
        #[doc = "Counter returns a value in nanoseconds."]
        Nanoseconds,
        #[doc = "Counter returns a timestamp in milliseconds."]
        TimestampMsec,
        #[doc = "Counter returns a timestamp in microseconds."]
        TimestampUsec,
        #[doc = "Counter returns a timestamp in nanoseconds."]
        TimestampNsec,
    }
    impl CounterMetadataStandardUnits {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterMetadataStandardUnits::Bytes => "BYTES",
                CounterMetadataStandardUnits::BytesPerSec => "BYTES_PER_SEC",
                CounterMetadataStandardUnits::Milliseconds => "MILLISECONDS",
                CounterMetadataStandardUnits::Microseconds => "MICROSECONDS",
                CounterMetadataStandardUnits::Nanoseconds => "NANOSECONDS",
                CounterMetadataStandardUnits::TimestampMsec => "TIMESTAMP_MSEC",
                CounterMetadataStandardUnits::TimestampUsec => "TIMESTAMP_USEC",
                CounterMetadataStandardUnits::TimestampNsec => "TIMESTAMP_NSEC",
            }
        }
    }
    impl ::std::fmt::Display for CounterMetadataStandardUnits {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterMetadataStandardUnits {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterMetadataStandardUnits {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BYTES" => CounterMetadataStandardUnits::Bytes,
                "BYTES_PER_SEC" => CounterMetadataStandardUnits::BytesPerSec,
                "MILLISECONDS" => CounterMetadataStandardUnits::Milliseconds,
                "MICROSECONDS" => CounterMetadataStandardUnits::Microseconds,
                "NANOSECONDS" => CounterMetadataStandardUnits::Nanoseconds,
                "TIMESTAMP_MSEC" => CounterMetadataStandardUnits::TimestampMsec,
                "TIMESTAMP_USEC" => CounterMetadataStandardUnits::TimestampUsec,
                "TIMESTAMP_NSEC" => CounterMetadataStandardUnits::TimestampNsec,
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
    pub struct CounterMetadata {
        #[doc = "Human-readable description of the counter semantics."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Counter aggregation kind."]
        #[serde(rename = "kind", default)]
        pub kind: Option<crate::schemas::CounterMetadataKind>,
        #[doc = "A string referring to the unit type."]
        #[serde(rename = "otherUnits", default)]
        pub other_units: Option<String>,
        #[doc = "System defined Units, see above enum."]
        #[serde(rename = "standardUnits", default)]
        pub standard_units: Option<crate::schemas::CounterMetadataStandardUnits>,
    }
    impl ::field_selector::FieldSelector for CounterMetadata {
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
    pub enum CounterStructuredNameOrigin {
        #[doc = "Counter was created by the Dataflow system."]
        System,
        #[doc = "Counter was created by the user."]
        User,
    }
    impl CounterStructuredNameOrigin {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterStructuredNameOrigin::System => "SYSTEM",
                CounterStructuredNameOrigin::User => "USER",
            }
        }
    }
    impl ::std::fmt::Display for CounterStructuredNameOrigin {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterStructuredNameOrigin {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterStructuredNameOrigin {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYSTEM" => CounterStructuredNameOrigin::System,
                "USER" => CounterStructuredNameOrigin::User,
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
    pub enum CounterStructuredNamePortion {
        #[doc = "Counter portion has not been set."]
        All,
        #[doc = "Counter reports a key."]
        Key,
        #[doc = "Counter reports a value."]
        Value,
    }
    impl CounterStructuredNamePortion {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterStructuredNamePortion::All => "ALL",
                CounterStructuredNamePortion::Key => "KEY",
                CounterStructuredNamePortion::Value => "VALUE",
            }
        }
    }
    impl ::std::fmt::Display for CounterStructuredNamePortion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterStructuredNamePortion {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterStructuredNamePortion {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL" => CounterStructuredNamePortion::All,
                "KEY" => CounterStructuredNamePortion::Key,
                "VALUE" => CounterStructuredNamePortion::Value,
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
    pub struct CounterStructuredName {
        #[doc = "Name of the optimized step being executed by the workers."]
        #[serde(rename = "componentStepName", default)]
        pub component_step_name: Option<String>,
        #[doc = "Name of the stage. An execution step contains multiple component steps."]
        #[serde(rename = "executionStepName", default)]
        pub execution_step_name: Option<String>,
        #[doc = "Index of an input collection that's being read from/written to as a side\ninput.\nThe index identifies a step's side inputs starting by 1 (e.g. the first\nside input has input_index 1, the third has input_index 3).\nSide inputs are identified by a pair of (original_step_name, input_index).\nThis field helps uniquely identify them."]
        #[serde(rename = "inputIndex", default)]
        pub input_index: Option<i32>,
        #[doc = "Counter name. Not necessarily globally-unique, but unique within the\ncontext of the other fields.\nRequired."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "One of the standard Origins defined above."]
        #[serde(rename = "origin", default)]
        pub origin: Option<crate::schemas::CounterStructuredNameOrigin>,
        #[doc = "A string containing a more specific namespace of the counter's origin."]
        #[serde(rename = "originNamespace", default)]
        pub origin_namespace: Option<String>,
        #[doc = "The step name requesting an operation, such as GBK.\nI.e. the ParDo causing a read/write from shuffle to occur, or a\nread from side inputs."]
        #[serde(rename = "originalRequestingStepName", default)]
        pub original_requesting_step_name: Option<String>,
        #[doc = "System generated name of the original step in the user's graph, before\noptimization."]
        #[serde(rename = "originalStepName", default)]
        pub original_step_name: Option<String>,
        #[doc = "Portion of this counter, either key or value."]
        #[serde(rename = "portion", default)]
        pub portion: Option<crate::schemas::CounterStructuredNamePortion>,
        #[doc = "ID of a particular worker."]
        #[serde(rename = "workerId", default)]
        pub worker_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CounterStructuredName {
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
    pub struct CounterStructuredNameAndMetadata {
        #[doc = "Metadata associated with a counter"]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::CounterMetadata>,
        #[doc = "Structured name of the counter."]
        #[serde(rename = "name", default)]
        pub name: Option<crate::schemas::CounterStructuredName>,
    }
    impl ::field_selector::FieldSelector for CounterStructuredNameAndMetadata {
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
    pub struct CounterUpdate {
        #[doc = "Boolean value for And, Or."]
        #[serde(rename = "boolean", default)]
        pub boolean: Option<bool>,
        #[doc = "True if this counter is reported as the total cumulative aggregate\nvalue accumulated since the worker started working on this WorkItem.\nBy default this is false, indicating that this counter is reported\nas a delta."]
        #[serde(rename = "cumulative", default)]
        pub cumulative: Option<bool>,
        #[doc = "Distribution data"]
        #[serde(rename = "distribution", default)]
        pub distribution: Option<crate::schemas::DistributionUpdate>,
        #[doc = "Floating point value for Sum, Max, Min."]
        #[serde(rename = "floatingPoint", default)]
        pub floating_point: Option<f64>,
        #[doc = "List of floating point numbers, for Set."]
        #[serde(rename = "floatingPointList", default)]
        pub floating_point_list: Option<crate::schemas::FloatingPointList>,
        #[doc = "Floating point mean aggregation value for Mean."]
        #[serde(rename = "floatingPointMean", default)]
        pub floating_point_mean: Option<crate::schemas::FloatingPointMean>,
        #[doc = "Integer value for Sum, Max, Min."]
        #[serde(rename = "integer", default)]
        pub integer: Option<crate::schemas::SplitInt64>,
        #[doc = "Gauge data"]
        #[serde(rename = "integerGauge", default)]
        pub integer_gauge: Option<crate::schemas::IntegerGauge>,
        #[doc = "List of integers, for Set."]
        #[serde(rename = "integerList", default)]
        pub integer_list: Option<crate::schemas::IntegerList>,
        #[doc = "Integer mean aggregation value for Mean."]
        #[serde(rename = "integerMean", default)]
        pub integer_mean: Option<crate::schemas::IntegerMean>,
        #[doc = "Value for internally-defined counters used by the Dataflow service."]
        #[serde(rename = "internal", default)]
        pub internal: Option<::serde_json::Value>,
        #[doc = "Counter name and aggregation type."]
        #[serde(rename = "nameAndKind", default)]
        pub name_and_kind: Option<crate::schemas::NameAndKind>,
        #[doc = "The service-generated short identifier for this counter.\nThe short_id -> (name, metadata) mapping is constant for the lifetime of\na job."]
        #[serde(rename = "shortId", default)]
        #[serde(with = "crate::parsed_string")]
        pub short_id: Option<i64>,
        #[doc = "List of strings, for Set."]
        #[serde(rename = "stringList", default)]
        pub string_list: Option<crate::schemas::StringList>,
        #[doc = "Counter structured name and metadata."]
        #[serde(rename = "structuredNameAndMetadata", default)]
        pub structured_name_and_metadata: Option<crate::schemas::CounterStructuredNameAndMetadata>,
    }
    impl ::field_selector::FieldSelector for CounterUpdate {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cputime {
        #[doc = "Average CPU utilization rate (% non-idle cpu / second) since previous\nsample."]
        #[serde(rename = "rate", default)]
        pub rate: Option<f64>,
        #[doc = "Timestamp of the measurement."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: Option<String>,
        #[doc = "Total active CPU time across all cores (ie., non-idle) in milliseconds\nsince start-up."]
        #[serde(rename = "totalMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_ms: Option<u64>,
    }
    impl ::field_selector::FieldSelector for Cputime {
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
    pub struct CreateJobFromTemplateRequest {
        #[doc = "The runtime environment for the job."]
        #[serde(rename = "environment", default)]
        pub environment: Option<crate::schemas::RuntimeEnvironment>,
        #[doc = "Required. A Cloud Storage path to the template from which to\ncreate the job.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
        #[serde(rename = "gcsPath", default)]
        pub gcs_path: Option<String>,
        #[doc = "Required. The job name to use for the created job."]
        #[serde(rename = "jobName", default)]
        pub job_name: Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to\nwhich to direct the request."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The runtime parameters to pass to the job."]
        #[serde(rename = "parameters", default)]
        pub parameters: Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for CreateJobFromTemplateRequest {
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
    pub struct CustomSourceLocation {
        #[doc = "Whether this source is stateful."]
        #[serde(rename = "stateful", default)]
        pub stateful: Option<bool>,
    }
    impl ::field_selector::FieldSelector for CustomSourceLocation {
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
    pub struct DataDiskAssignment {
        #[doc = "Mounted data disks. The order is important a data disk's 0-based index in\nthis list defines which persistent directory the disk is mounted to, for\nexample the list of { \"myproject-1014-104817-4c2-harness-0-disk-0\" },\n{ \"myproject-1014-104817-4c2-harness-0-disk-1\" }."]
        #[serde(rename = "dataDisks", default)]
        pub data_disks: Option<Vec<String>>,
        #[doc = "VM instance name the data disks mounted to, for example\n\"myproject-1014-104817-4c2-harness-0\"."]
        #[serde(rename = "vmInstance", default)]
        pub vm_instance: Option<String>,
    }
    impl ::field_selector::FieldSelector for DataDiskAssignment {
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
    pub struct DatastoreIODetails {
        #[doc = "Namespace used in the connection."]
        #[serde(rename = "namespace", default)]
        pub namespace: Option<String>,
        #[doc = "ProjectId accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for DatastoreIODetails {
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
    pub struct DeleteSnapshotResponse;
    impl ::field_selector::FieldSelector for DeleteSnapshotResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DerivedSourceDerivationMode {
        #[doc = "The source derivation is unknown, or unspecified."]
        SourceDerivationModeUnknown,
        #[doc = "Produce a completely independent Source with no base."]
        SourceDerivationModeIndependent,
        #[doc = "Produce a Source based on the Source being split."]
        SourceDerivationModeChildOfCurrent,
        #[doc = "Produce a Source based on the base of the Source being split."]
        SourceDerivationModeSiblingOfCurrent,
    }
    impl DerivedSourceDerivationMode {
        pub fn as_str(self) -> &'static str {
            match self {
                DerivedSourceDerivationMode::SourceDerivationModeUnknown => {
                    "SOURCE_DERIVATION_MODE_UNKNOWN"
                }
                DerivedSourceDerivationMode::SourceDerivationModeIndependent => {
                    "SOURCE_DERIVATION_MODE_INDEPENDENT"
                }
                DerivedSourceDerivationMode::SourceDerivationModeChildOfCurrent => {
                    "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT"
                }
                DerivedSourceDerivationMode::SourceDerivationModeSiblingOfCurrent => {
                    "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT"
                }
            }
        }
    }
    impl ::std::fmt::Display for DerivedSourceDerivationMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DerivedSourceDerivationMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DerivedSourceDerivationMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_DERIVATION_MODE_UNKNOWN" => {
                    DerivedSourceDerivationMode::SourceDerivationModeUnknown
                }
                "SOURCE_DERIVATION_MODE_INDEPENDENT" => {
                    DerivedSourceDerivationMode::SourceDerivationModeIndependent
                }
                "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT" => {
                    DerivedSourceDerivationMode::SourceDerivationModeChildOfCurrent
                }
                "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT" => {
                    DerivedSourceDerivationMode::SourceDerivationModeSiblingOfCurrent
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DerivedSource {
        #[doc = "What source to base the produced source on (if any)."]
        #[serde(rename = "derivationMode", default)]
        pub derivation_mode: Option<crate::schemas::DerivedSourceDerivationMode>,
        #[doc = "Specification of the source."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for DerivedSource {
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
    pub struct Disk {
        #[doc = "Disk storage type, as defined by Google Compute Engine.  This\nmust be a disk type appropriate to the project and zone in which\nthe workers will run.  If unknown or unspecified, the service\nwill attempt to choose a reasonable default.\n\nFor example, the standard persistent disk type is a resource name\ntypically ending in \"pd-standard\".  If SSD persistent disks are\navailable, the resource name typically ends with \"pd-ssd\".  The\nactual valid values are defined the Google Compute Engine API,\nnot by the Cloud Dataflow API; consult the Google Compute Engine\ndocumentation for more information about determining the set of\navailable disk types for a particular project and zone.\n\nGoogle Compute Engine Disk types are local to a particular\nproject in a particular zone, and so the resource name will\ntypically look something like this:\n\ncompute.googleapis.com/projects/project-id/zones/zone/diskTypes/pd-standard"]
        #[serde(rename = "diskType", default)]
        pub disk_type: Option<String>,
        #[doc = "Directory in a VM where disk is mounted."]
        #[serde(rename = "mountPoint", default)]
        pub mount_point: Option<String>,
        #[doc = "Size of disk in GB.  If zero or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "sizeGb", default)]
        pub size_gb: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Disk {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DisplayData {
        #[doc = "Contains value if the data is of a boolean type."]
        #[serde(rename = "boolValue", default)]
        pub bool_value: Option<bool>,
        #[doc = "Contains value if the data is of duration type."]
        #[serde(rename = "durationValue", default)]
        pub duration_value: Option<String>,
        #[doc = "Contains value if the data is of float type."]
        #[serde(rename = "floatValue", default)]
        pub float_value: Option<f32>,
        #[doc = "Contains value if the data is of int64 type."]
        #[serde(rename = "int64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub int_64_value: Option<i64>,
        #[doc = "Contains value if the data is of java class type."]
        #[serde(rename = "javaClassValue", default)]
        pub java_class_value: Option<String>,
        #[doc = "The key identifying the display data.\nThis is intended to be used as a label for the display data\nwhen viewed in a dax monitoring system."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "An optional label to display in a dax UI for the element."]
        #[serde(rename = "label", default)]
        pub label: Option<String>,
        #[doc = "The namespace for the key. This is usually a class name or programming\nlanguage namespace (i.e. python module) which defines the display data.\nThis allows a dax monitoring system to specially handle the data\nand perform custom rendering."]
        #[serde(rename = "namespace", default)]
        pub namespace: Option<String>,
        #[doc = "A possible additional shorter value to display.\nFor example a java_class_name_value of com.mypackage.MyDoFn\nwill be stored with MyDoFn as the short_str_value and\ncom.mypackage.MyDoFn as the java_class_name value.\nshort_str_value can be displayed and java_class_name_value\nwill be displayed as a tooltip."]
        #[serde(rename = "shortStrValue", default)]
        pub short_str_value: Option<String>,
        #[doc = "Contains value if the data is of string type."]
        #[serde(rename = "strValue", default)]
        pub str_value: Option<String>,
        #[doc = "Contains value if the data is of timestamp type."]
        #[serde(rename = "timestampValue", default)]
        pub timestamp_value: Option<String>,
        #[doc = "An optional full URL."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for DisplayData {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DistributionUpdate {
        #[doc = "The count of the number of elements present in the distribution."]
        #[serde(rename = "count", default)]
        pub count: Option<crate::schemas::SplitInt64>,
        #[doc = "(Optional) Histogram of value counts for the distribution."]
        #[serde(rename = "histogram", default)]
        pub histogram: Option<crate::schemas::Histogram>,
        #[doc = "The maximum value present in the distribution."]
        #[serde(rename = "max", default)]
        pub max: Option<crate::schemas::SplitInt64>,
        #[doc = "The minimum value present in the distribution."]
        #[serde(rename = "min", default)]
        pub min: Option<crate::schemas::SplitInt64>,
        #[doc = "Use an int64 since we'd prefer the added precision. If overflow is a common\nproblem we can detect it and use an additional int64 or a double."]
        #[serde(rename = "sum", default)]
        pub sum: Option<crate::schemas::SplitInt64>,
        #[doc = "Use a double since the sum of squares is likely to overflow int64."]
        #[serde(rename = "sumOfSquares", default)]
        pub sum_of_squares: Option<f64>,
    }
    impl ::field_selector::FieldSelector for DistributionUpdate {
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
    pub struct DynamicSourceSplit {
        #[doc = "Primary part (continued to be processed by worker).\nSpecified relative to the previously-current source.\nBecomes current."]
        #[serde(rename = "primary", default)]
        pub primary: Option<crate::schemas::DerivedSource>,
        #[doc = "Residual part (returned to the pool of work).\nSpecified relative to the previously-current source."]
        #[serde(rename = "residual", default)]
        pub residual: Option<crate::schemas::DerivedSource>,
    }
    impl ::field_selector::FieldSelector for DynamicSourceSplit {
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
    pub enum EnvironmentFlexResourceSchedulingGoal {
        #[doc = "Run in the default mode."]
        FlexrsUnspecified,
        #[doc = "Optimize for lower execution time."]
        FlexrsSpeedOptimized,
        #[doc = "Optimize for lower cost."]
        FlexrsCostOptimized,
    }
    impl EnvironmentFlexResourceSchedulingGoal {
        pub fn as_str(self) -> &'static str {
            match self {
                EnvironmentFlexResourceSchedulingGoal::FlexrsUnspecified => "FLEXRS_UNSPECIFIED",
                EnvironmentFlexResourceSchedulingGoal::FlexrsSpeedOptimized => {
                    "FLEXRS_SPEED_OPTIMIZED"
                }
                EnvironmentFlexResourceSchedulingGoal::FlexrsCostOptimized => {
                    "FLEXRS_COST_OPTIMIZED"
                }
            }
        }
    }
    impl ::std::fmt::Display for EnvironmentFlexResourceSchedulingGoal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnvironmentFlexResourceSchedulingGoal {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnvironmentFlexResourceSchedulingGoal {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FLEXRS_UNSPECIFIED" => EnvironmentFlexResourceSchedulingGoal::FlexrsUnspecified,
                "FLEXRS_SPEED_OPTIMIZED" => {
                    EnvironmentFlexResourceSchedulingGoal::FlexrsSpeedOptimized
                }
                "FLEXRS_COST_OPTIMIZED" => {
                    EnvironmentFlexResourceSchedulingGoal::FlexrsCostOptimized
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Environment {
        #[doc = "The type of cluster manager API to use.  If unknown or\nunspecified, the service will attempt to choose a reasonable\ndefault.  This should be in the form of the API service name,\ne.g. \"compute.googleapis.com\"."]
        #[serde(rename = "clusterManagerApiService", default)]
        pub cluster_manager_api_service: Option<String>,
        #[doc = "The dataset for the current project where various workflow\nrelated tables are stored.\n\nThe supported resource type is:\n\nGoogle BigQuery:\n  bigquery.googleapis.com/{dataset}"]
        #[serde(rename = "dataset", default)]
        pub dataset: Option<String>,
        #[doc = "The list of experiments to enable."]
        #[serde(rename = "experiments", default)]
        pub experiments: Option<Vec<String>>,
        #[doc = "Which Flexible Resource Scheduling mode to run in."]
        #[serde(rename = "flexResourceSchedulingGoal", default)]
        pub flex_resource_scheduling_goal:
            Option<crate::schemas::EnvironmentFlexResourceSchedulingGoal>,
        #[doc = "Experimental settings."]
        #[serde(rename = "internalExperiments", default)]
        pub internal_experiments: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The Cloud Dataflow SDK pipeline options specified by the user. These\noptions are passed through the service and are used to recreate the\nSDK pipeline options on the worker in a language agnostic and platform\nindependent way."]
        #[serde(rename = "sdkPipelineOptions", default)]
        pub sdk_pipeline_options: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Identity to run virtual machines as. Defaults to the default account."]
        #[serde(rename = "serviceAccountEmail", default)]
        pub service_account_email: Option<String>,
        #[doc = "If set, contains the Cloud KMS key identifier used to encrypt data\nat rest, AKA a Customer Managed Encryption Key (CMEK).\n\nFormat:\n  projects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY"]
        #[serde(rename = "serviceKmsKeyName", default)]
        pub service_kms_key_name: Option<String>,
        #[doc = "The prefix of the resources the system should use for temporary\nstorage.  The system will append the suffix \"/temp-{JOBNAME} to\nthis resource prefix, where {JOBNAME} is the value of the\njob_name field.  The resulting bucket and object prefix is used\nas the prefix of the resources used to store temporary data\nneeded during the job execution.  NOTE: This will override the\nvalue in taskrunner_settings.\nThe supported resource type is:\n\nGoogle Cloud Storage:\n\n  storage.googleapis.com/{bucket}/{object}\n  bucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempStoragePrefix", default)]
        pub temp_storage_prefix: Option<String>,
        #[doc = "A description of the process that generated the request."]
        #[serde(rename = "userAgent", default)]
        pub user_agent: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A structure describing which components and their versions of the service\nare required in order to run the job."]
        #[serde(rename = "version", default)]
        pub version: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The worker pools. At least one \"harness\" worker pool must be\nspecified in order for the job to have workers."]
        #[serde(rename = "workerPools", default)]
        pub worker_pools: Option<Vec<crate::schemas::WorkerPool>>,
    }
    impl ::field_selector::FieldSelector for Environment {
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
    pub enum ExecutionStageStateExecutionStageState {
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not\nyet started to run."]
        JobStateStopped,
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed.\nThis is a terminal job state.  This state may be set by the Cloud Dataflow\nservice, as a transition from `JOB_STATE_RUNNING`. It may also be set via a\nCloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal\nstate."]
        JobStateDone,
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed.  This is a\nterminal job state.  This state may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly\ncancelled. This is a terminal job state. This state may only be\nset via a Cloud Dataflow `UpdateJob` call, and only if the job has not\nyet reached another terminal state."]
        JobStateCancelled,
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated,\nmeaning that this job was stopped and another job was started, inheriting\nstate from this one. This is a terminal job state. This state may only be\nset by the Cloud Dataflow service, and only as a transition from\n`JOB_STATE_RUNNING`."]
        JobStateUpdated,
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining.\nA draining job has stopped pulling from its input sources and is processing\nany data that remains in-flight. This state may be set via a Cloud Dataflow\n`UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs\nthat are draining may only transition to `JOB_STATE_DRAINED`,\n`JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained.\nA drained job terminated by stopping pulling from its input sources and\nprocessing any data that remained in-flight when draining was requested.\nThis state is a terminal state, may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet\nrunning.  Jobs that are pending may only transition to `JOB_STATE_RUNNING`,\nor `JOB_STATE_FAILED`."]
        JobStatePending,
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled\nand is in the process of stopping.  Jobs that are cancelling may only\ntransition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being\ndelayed until launch. Jobs that are queued may only transition to\n`JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
    }
    impl ExecutionStageStateExecutionStageState {
        pub fn as_str(self) -> &'static str {
            match self {
                ExecutionStageStateExecutionStageState::JobStateUnknown => "JOB_STATE_UNKNOWN",
                ExecutionStageStateExecutionStageState::JobStateStopped => "JOB_STATE_STOPPED",
                ExecutionStageStateExecutionStageState::JobStateRunning => "JOB_STATE_RUNNING",
                ExecutionStageStateExecutionStageState::JobStateDone => "JOB_STATE_DONE",
                ExecutionStageStateExecutionStageState::JobStateFailed => "JOB_STATE_FAILED",
                ExecutionStageStateExecutionStageState::JobStateCancelled => "JOB_STATE_CANCELLED",
                ExecutionStageStateExecutionStageState::JobStateUpdated => "JOB_STATE_UPDATED",
                ExecutionStageStateExecutionStageState::JobStateDraining => "JOB_STATE_DRAINING",
                ExecutionStageStateExecutionStageState::JobStateDrained => "JOB_STATE_DRAINED",
                ExecutionStageStateExecutionStageState::JobStatePending => "JOB_STATE_PENDING",
                ExecutionStageStateExecutionStageState::JobStateCancelling => {
                    "JOB_STATE_CANCELLING"
                }
                ExecutionStageStateExecutionStageState::JobStateQueued => "JOB_STATE_QUEUED",
            }
        }
    }
    impl ::std::fmt::Display for ExecutionStageStateExecutionStageState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExecutionStageStateExecutionStageState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExecutionStageStateExecutionStageState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_STATE_UNKNOWN" => ExecutionStageStateExecutionStageState::JobStateUnknown,
                "JOB_STATE_STOPPED" => ExecutionStageStateExecutionStageState::JobStateStopped,
                "JOB_STATE_RUNNING" => ExecutionStageStateExecutionStageState::JobStateRunning,
                "JOB_STATE_DONE" => ExecutionStageStateExecutionStageState::JobStateDone,
                "JOB_STATE_FAILED" => ExecutionStageStateExecutionStageState::JobStateFailed,
                "JOB_STATE_CANCELLED" => ExecutionStageStateExecutionStageState::JobStateCancelled,
                "JOB_STATE_UPDATED" => ExecutionStageStateExecutionStageState::JobStateUpdated,
                "JOB_STATE_DRAINING" => ExecutionStageStateExecutionStageState::JobStateDraining,
                "JOB_STATE_DRAINED" => ExecutionStageStateExecutionStageState::JobStateDrained,
                "JOB_STATE_PENDING" => ExecutionStageStateExecutionStageState::JobStatePending,
                "JOB_STATE_CANCELLING" => {
                    ExecutionStageStateExecutionStageState::JobStateCancelling
                }
                "JOB_STATE_QUEUED" => ExecutionStageStateExecutionStageState::JobStateQueued,
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
    pub struct ExecutionStageState {
        #[doc = "The time at which the stage transitioned to this state."]
        #[serde(rename = "currentStateTime", default)]
        pub current_state_time: Option<String>,
        #[doc = "The name of the execution stage."]
        #[serde(rename = "executionStageName", default)]
        pub execution_stage_name: Option<String>,
        #[doc = "Executions stage states allow the same set of values as JobState."]
        #[serde(rename = "executionStageState", default)]
        pub execution_stage_state: Option<crate::schemas::ExecutionStageStateExecutionStageState>,
    }
    impl ::field_selector::FieldSelector for ExecutionStageState {
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
    pub enum ExecutionStageSummaryKind {
        #[doc = "Unrecognized transform type."]
        UnknownKind,
        #[doc = "ParDo transform."]
        ParDoKind,
        #[doc = "Group By Key transform."]
        GroupByKeyKind,
        #[doc = "Flatten transform."]
        FlattenKind,
        #[doc = "Read transform."]
        ReadKind,
        #[doc = "Write transform."]
        WriteKind,
        #[doc = "Constructs from a constant value, such as with Create.of."]
        ConstantKind,
        #[doc = "Creates a Singleton view of a collection."]
        SingletonKind,
        #[doc = "Opening or closing a shuffle session, often as part of a GroupByKey."]
        ShuffleKind,
    }
    impl ExecutionStageSummaryKind {
        pub fn as_str(self) -> &'static str {
            match self {
                ExecutionStageSummaryKind::UnknownKind => "UNKNOWN_KIND",
                ExecutionStageSummaryKind::ParDoKind => "PAR_DO_KIND",
                ExecutionStageSummaryKind::GroupByKeyKind => "GROUP_BY_KEY_KIND",
                ExecutionStageSummaryKind::FlattenKind => "FLATTEN_KIND",
                ExecutionStageSummaryKind::ReadKind => "READ_KIND",
                ExecutionStageSummaryKind::WriteKind => "WRITE_KIND",
                ExecutionStageSummaryKind::ConstantKind => "CONSTANT_KIND",
                ExecutionStageSummaryKind::SingletonKind => "SINGLETON_KIND",
                ExecutionStageSummaryKind::ShuffleKind => "SHUFFLE_KIND",
            }
        }
    }
    impl ::std::fmt::Display for ExecutionStageSummaryKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExecutionStageSummaryKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExecutionStageSummaryKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_KIND" => ExecutionStageSummaryKind::UnknownKind,
                "PAR_DO_KIND" => ExecutionStageSummaryKind::ParDoKind,
                "GROUP_BY_KEY_KIND" => ExecutionStageSummaryKind::GroupByKeyKind,
                "FLATTEN_KIND" => ExecutionStageSummaryKind::FlattenKind,
                "READ_KIND" => ExecutionStageSummaryKind::ReadKind,
                "WRITE_KIND" => ExecutionStageSummaryKind::WriteKind,
                "CONSTANT_KIND" => ExecutionStageSummaryKind::ConstantKind,
                "SINGLETON_KIND" => ExecutionStageSummaryKind::SingletonKind,
                "SHUFFLE_KIND" => ExecutionStageSummaryKind::ShuffleKind,
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
    pub struct ExecutionStageSummary {
        #[doc = "Collections produced and consumed by component transforms of this stage."]
        #[serde(rename = "componentSource", default)]
        pub component_source: Option<Vec<crate::schemas::ComponentSource>>,
        #[doc = "Transforms that comprise this execution stage."]
        #[serde(rename = "componentTransform", default)]
        pub component_transform: Option<Vec<crate::schemas::ComponentTransform>>,
        #[doc = "Dataflow service generated id for this stage."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Input sources for this stage."]
        #[serde(rename = "inputSource", default)]
        pub input_source: Option<Vec<crate::schemas::StageSource>>,
        #[doc = "Type of tranform this stage is executing."]
        #[serde(rename = "kind", default)]
        pub kind: Option<crate::schemas::ExecutionStageSummaryKind>,
        #[doc = "Dataflow service generated name for this stage."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Output sources for this stage."]
        #[serde(rename = "outputSource", default)]
        pub output_source: Option<Vec<crate::schemas::StageSource>>,
    }
    impl ::field_selector::FieldSelector for ExecutionStageSummary {
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
    pub struct FailedLocation {
        #[doc = "The name of the [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\nfailed to respond."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for FailedLocation {
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
    pub struct FileIODetails {
        #[doc = "File Pattern used to access files by the connector."]
        #[serde(rename = "filePattern", default)]
        pub file_pattern: Option<String>,
    }
    impl ::field_selector::FieldSelector for FileIODetails {
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
    pub struct FlattenInstruction {
        #[doc = "Describes the inputs to the flatten instruction."]
        #[serde(rename = "inputs", default)]
        pub inputs: Option<Vec<crate::schemas::InstructionInput>>,
    }
    impl ::field_selector::FieldSelector for FlattenInstruction {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FloatingPointList {
        #[doc = "Elements of the list."]
        #[serde(rename = "elements", default)]
        pub elements: Option<Vec<f64>>,
    }
    impl ::field_selector::FieldSelector for FloatingPointList {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FloatingPointMean {
        #[doc = "The number of values being aggregated."]
        #[serde(rename = "count", default)]
        pub count: Option<crate::schemas::SplitInt64>,
        #[doc = "The sum of all values being aggregated."]
        #[serde(rename = "sum", default)]
        pub sum: Option<f64>,
    }
    impl ::field_selector::FieldSelector for FloatingPointMean {
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
    pub struct GetDebugConfigRequest {
        #[doc = "The internal component id for which debug configuration is\nrequested."]
        #[serde(rename = "componentId", default)]
        pub component_id: Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The worker id, i.e., VM hostname."]
        #[serde(rename = "workerId", default)]
        pub worker_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetDebugConfigRequest {
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
    pub struct GetDebugConfigResponse {
        #[doc = "The encoded debug configuration for the requested component."]
        #[serde(rename = "config", default)]
        pub config: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetDebugConfigResponse {
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
    pub struct GetTemplateResponse {
        #[doc = "The template metadata describing the template name, available\nparameters, etc."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::TemplateMetadata>,
        #[doc = "The status of the get template request. Any problems with the\nrequest will be indicated in the error_details."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::Status>,
    }
    impl ::field_selector::FieldSelector for GetTemplateResponse {
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
    pub struct Histogram {
        #[doc = "Counts of values in each bucket. For efficiency, prefix and trailing\nbuckets with count = 0 are elided. Buckets can store the full range of\nvalues of an unsigned long, with ULLONG_MAX falling into the 59th bucket\nwith range [1e19, 2e19)."]
        #[serde(rename = "bucketCounts", default)]
        pub bucket_counts: Option<Vec<i64>>,
        #[doc = "Starting index of first stored bucket. The non-inclusive upper-bound of\nthe ith bucket is given by:\n  pow(10,(i-first_bucket_offset)/3) * (1,2,5)[(i-first_bucket_offset)%3]"]
        #[serde(rename = "firstBucketOffset", default)]
        pub first_bucket_offset: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Histogram {
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
    pub struct HotKeyDetection {
        #[doc = "The age of the hot key measured from when it was first detected."]
        #[serde(rename = "hotKeyAge", default)]
        pub hot_key_age: Option<String>,
        #[doc = "System-defined name of the step containing this hot key.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: Option<String>,
        #[doc = "User-provided name of the step that contains this hot key."]
        #[serde(rename = "userStepName", default)]
        pub user_step_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for HotKeyDetection {
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
    pub struct InstructionInput {
        #[doc = "The output index (origin zero) within the producer."]
        #[serde(rename = "outputNum", default)]
        pub output_num: Option<i32>,
        #[doc = "The index (origin zero) of the parallel instruction that produces\nthe output to be consumed by this input.  This index is relative\nto the list of instructions in this input's instruction's\ncontaining MapTask."]
        #[serde(rename = "producerInstructionIndex", default)]
        pub producer_instruction_index: Option<i32>,
    }
    impl ::field_selector::FieldSelector for InstructionInput {
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
    pub struct InstructionOutput {
        #[doc = "The codec to use to encode data being written via this output."]
        #[serde(rename = "codec", default)]
        pub codec: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The user-provided name of this output."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "For system-generated byte and mean byte metrics, certain instructions\nshould only report the key size."]
        #[serde(rename = "onlyCountKeyBytes", default)]
        pub only_count_key_bytes: Option<bool>,
        #[doc = "For system-generated byte and mean byte metrics, certain instructions\nshould only report the value size."]
        #[serde(rename = "onlyCountValueBytes", default)]
        pub only_count_value_bytes: Option<bool>,
        #[doc = "System-defined name for this output in the original workflow graph.\nOutputs that do not contribute to an original instruction do not set this."]
        #[serde(rename = "originalName", default)]
        pub original_name: Option<String>,
        #[doc = "System-defined name of this output.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for InstructionOutput {
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
    pub struct IntegerGauge {
        #[doc = "The time at which this value was measured. Measured as msecs from epoch."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: Option<String>,
        #[doc = "The value of the variable represented by this gauge."]
        #[serde(rename = "value", default)]
        pub value: Option<crate::schemas::SplitInt64>,
    }
    impl ::field_selector::FieldSelector for IntegerGauge {
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
    pub struct IntegerList {
        #[doc = "Elements of the list."]
        #[serde(rename = "elements", default)]
        pub elements: Option<Vec<crate::schemas::SplitInt64>>,
    }
    impl ::field_selector::FieldSelector for IntegerList {
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
    pub struct IntegerMean {
        #[doc = "The number of values being aggregated."]
        #[serde(rename = "count", default)]
        pub count: Option<crate::schemas::SplitInt64>,
        #[doc = "The sum of all values being aggregated."]
        #[serde(rename = "sum", default)]
        pub sum: Option<crate::schemas::SplitInt64>,
    }
    impl ::field_selector::FieldSelector for IntegerMean {
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
    pub enum JobCurrentState {
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not\nyet started to run."]
        JobStateStopped,
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed.\nThis is a terminal job state.  This state may be set by the Cloud Dataflow\nservice, as a transition from `JOB_STATE_RUNNING`. It may also be set via a\nCloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal\nstate."]
        JobStateDone,
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed.  This is a\nterminal job state.  This state may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly\ncancelled. This is a terminal job state. This state may only be\nset via a Cloud Dataflow `UpdateJob` call, and only if the job has not\nyet reached another terminal state."]
        JobStateCancelled,
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated,\nmeaning that this job was stopped and another job was started, inheriting\nstate from this one. This is a terminal job state. This state may only be\nset by the Cloud Dataflow service, and only as a transition from\n`JOB_STATE_RUNNING`."]
        JobStateUpdated,
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining.\nA draining job has stopped pulling from its input sources and is processing\nany data that remains in-flight. This state may be set via a Cloud Dataflow\n`UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs\nthat are draining may only transition to `JOB_STATE_DRAINED`,\n`JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained.\nA drained job terminated by stopping pulling from its input sources and\nprocessing any data that remained in-flight when draining was requested.\nThis state is a terminal state, may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet\nrunning.  Jobs that are pending may only transition to `JOB_STATE_RUNNING`,\nor `JOB_STATE_FAILED`."]
        JobStatePending,
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled\nand is in the process of stopping.  Jobs that are cancelling may only\ntransition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being\ndelayed until launch. Jobs that are queued may only transition to\n`JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
    }
    impl JobCurrentState {
        pub fn as_str(self) -> &'static str {
            match self {
                JobCurrentState::JobStateUnknown => "JOB_STATE_UNKNOWN",
                JobCurrentState::JobStateStopped => "JOB_STATE_STOPPED",
                JobCurrentState::JobStateRunning => "JOB_STATE_RUNNING",
                JobCurrentState::JobStateDone => "JOB_STATE_DONE",
                JobCurrentState::JobStateFailed => "JOB_STATE_FAILED",
                JobCurrentState::JobStateCancelled => "JOB_STATE_CANCELLED",
                JobCurrentState::JobStateUpdated => "JOB_STATE_UPDATED",
                JobCurrentState::JobStateDraining => "JOB_STATE_DRAINING",
                JobCurrentState::JobStateDrained => "JOB_STATE_DRAINED",
                JobCurrentState::JobStatePending => "JOB_STATE_PENDING",
                JobCurrentState::JobStateCancelling => "JOB_STATE_CANCELLING",
                JobCurrentState::JobStateQueued => "JOB_STATE_QUEUED",
            }
        }
    }
    impl ::std::fmt::Display for JobCurrentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobCurrentState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobCurrentState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_STATE_UNKNOWN" => JobCurrentState::JobStateUnknown,
                "JOB_STATE_STOPPED" => JobCurrentState::JobStateStopped,
                "JOB_STATE_RUNNING" => JobCurrentState::JobStateRunning,
                "JOB_STATE_DONE" => JobCurrentState::JobStateDone,
                "JOB_STATE_FAILED" => JobCurrentState::JobStateFailed,
                "JOB_STATE_CANCELLED" => JobCurrentState::JobStateCancelled,
                "JOB_STATE_UPDATED" => JobCurrentState::JobStateUpdated,
                "JOB_STATE_DRAINING" => JobCurrentState::JobStateDraining,
                "JOB_STATE_DRAINED" => JobCurrentState::JobStateDrained,
                "JOB_STATE_PENDING" => JobCurrentState::JobStatePending,
                "JOB_STATE_CANCELLING" => JobCurrentState::JobStateCancelling,
                "JOB_STATE_QUEUED" => JobCurrentState::JobStateQueued,
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
    pub enum JobType {
        #[doc = "The type of the job is unspecified, or unknown."]
        JobTypeUnknown,
        #[doc = "A batch job with a well-defined end point: data is read, data is\nprocessed, data is written, and the job is done."]
        JobTypeBatch,
        #[doc = "A continuously streaming job with no end: data is read,\nprocessed, and written continuously."]
        JobTypeStreaming,
    }
    impl JobType {
        pub fn as_str(self) -> &'static str {
            match self {
                JobType::JobTypeUnknown => "JOB_TYPE_UNKNOWN",
                JobType::JobTypeBatch => "JOB_TYPE_BATCH",
                JobType::JobTypeStreaming => "JOB_TYPE_STREAMING",
            }
        }
    }
    impl ::std::fmt::Display for JobType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_TYPE_UNKNOWN" => JobType::JobTypeUnknown,
                "JOB_TYPE_BATCH" => JobType::JobTypeBatch,
                "JOB_TYPE_STREAMING" => JobType::JobTypeStreaming,
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
    pub enum JobRequestedState {
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not\nyet started to run."]
        JobStateStopped,
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed.\nThis is a terminal job state.  This state may be set by the Cloud Dataflow\nservice, as a transition from `JOB_STATE_RUNNING`. It may also be set via a\nCloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal\nstate."]
        JobStateDone,
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed.  This is a\nterminal job state.  This state may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly\ncancelled. This is a terminal job state. This state may only be\nset via a Cloud Dataflow `UpdateJob` call, and only if the job has not\nyet reached another terminal state."]
        JobStateCancelled,
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated,\nmeaning that this job was stopped and another job was started, inheriting\nstate from this one. This is a terminal job state. This state may only be\nset by the Cloud Dataflow service, and only as a transition from\n`JOB_STATE_RUNNING`."]
        JobStateUpdated,
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining.\nA draining job has stopped pulling from its input sources and is processing\nany data that remains in-flight. This state may be set via a Cloud Dataflow\n`UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs\nthat are draining may only transition to `JOB_STATE_DRAINED`,\n`JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained.\nA drained job terminated by stopping pulling from its input sources and\nprocessing any data that remained in-flight when draining was requested.\nThis state is a terminal state, may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet\nrunning.  Jobs that are pending may only transition to `JOB_STATE_RUNNING`,\nor `JOB_STATE_FAILED`."]
        JobStatePending,
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled\nand is in the process of stopping.  Jobs that are cancelling may only\ntransition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being\ndelayed until launch. Jobs that are queued may only transition to\n`JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
    }
    impl JobRequestedState {
        pub fn as_str(self) -> &'static str {
            match self {
                JobRequestedState::JobStateUnknown => "JOB_STATE_UNKNOWN",
                JobRequestedState::JobStateStopped => "JOB_STATE_STOPPED",
                JobRequestedState::JobStateRunning => "JOB_STATE_RUNNING",
                JobRequestedState::JobStateDone => "JOB_STATE_DONE",
                JobRequestedState::JobStateFailed => "JOB_STATE_FAILED",
                JobRequestedState::JobStateCancelled => "JOB_STATE_CANCELLED",
                JobRequestedState::JobStateUpdated => "JOB_STATE_UPDATED",
                JobRequestedState::JobStateDraining => "JOB_STATE_DRAINING",
                JobRequestedState::JobStateDrained => "JOB_STATE_DRAINED",
                JobRequestedState::JobStatePending => "JOB_STATE_PENDING",
                JobRequestedState::JobStateCancelling => "JOB_STATE_CANCELLING",
                JobRequestedState::JobStateQueued => "JOB_STATE_QUEUED",
            }
        }
    }
    impl ::std::fmt::Display for JobRequestedState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobRequestedState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobRequestedState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_STATE_UNKNOWN" => JobRequestedState::JobStateUnknown,
                "JOB_STATE_STOPPED" => JobRequestedState::JobStateStopped,
                "JOB_STATE_RUNNING" => JobRequestedState::JobStateRunning,
                "JOB_STATE_DONE" => JobRequestedState::JobStateDone,
                "JOB_STATE_FAILED" => JobRequestedState::JobStateFailed,
                "JOB_STATE_CANCELLED" => JobRequestedState::JobStateCancelled,
                "JOB_STATE_UPDATED" => JobRequestedState::JobStateUpdated,
                "JOB_STATE_DRAINING" => JobRequestedState::JobStateDraining,
                "JOB_STATE_DRAINED" => JobRequestedState::JobStateDrained,
                "JOB_STATE_PENDING" => JobRequestedState::JobStatePending,
                "JOB_STATE_CANCELLING" => JobRequestedState::JobStateCancelling,
                "JOB_STATE_QUEUED" => JobRequestedState::JobStateQueued,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Job {
        #[doc = "The client's unique identifier of the job, re-used across retried attempts.\nIf this field is set, the service will ensure its uniqueness.\nThe request to create a job will fail if the service has knowledge of a\npreviously submitted job with the same client's ID and job name.\nThe caller may use this field to ensure idempotence of job\ncreation across retried attempts to create a job.\nBy default, the field is empty and, in that case, the service ignores it."]
        #[serde(rename = "clientRequestId", default)]
        pub client_request_id: Option<String>,
        #[doc = "The timestamp when the job was initially created. Immutable and set by the\nCloud Dataflow service."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "If this is specified, the job's initial state is populated from the given\nsnapshot."]
        #[serde(rename = "createdFromSnapshotId", default)]
        pub created_from_snapshot_id: Option<String>,
        #[doc = "The current state of the job.\n\nJobs are created in the `JOB_STATE_STOPPED` state unless otherwise\nspecified.\n\nA job in the `JOB_STATE_RUNNING` state may asynchronously enter a\nterminal state. After a job has reached a terminal state, no\nfurther state updates may be made.\n\nThis field may be mutated by the Cloud Dataflow service;\ncallers cannot mutate it."]
        #[serde(rename = "currentState", default)]
        pub current_state: Option<crate::schemas::JobCurrentState>,
        #[doc = "The timestamp associated with the current state."]
        #[serde(rename = "currentStateTime", default)]
        pub current_state_time: Option<String>,
        #[doc = "The environment for the job."]
        #[serde(rename = "environment", default)]
        pub environment: Option<crate::schemas::Environment>,
        #[doc = "Deprecated."]
        #[serde(rename = "executionInfo", default)]
        pub execution_info: Option<crate::schemas::JobExecutionInfo>,
        #[doc = "The unique ID of this job.\n\nThis field is set by the Cloud Dataflow service when the Job is\ncreated, and is immutable for the life of the job."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "This field is populated by the Dataflow service to support filtering jobs\nby the metadata values provided here. Populated for ListJobs and all GetJob\nviews SUMMARY and higher."]
        #[serde(rename = "jobMetadata", default)]
        pub job_metadata: Option<crate::schemas::JobMetadata>,
        #[doc = "User-defined labels for this job.\n\nThe labels map can contain no more than 64 entries.  Entries of the labels\nmap are UTF8 strings that comply with the following restrictions:\n\n* Keys must conform to regexp:  \\p{Ll}\\p{Lo}{0,62}\n* Values must conform to regexp:  [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n* Both keys and values are additionally constrained to be <= 128 bytes in\nsize."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The user-specified Cloud Dataflow job name.\n\nOnly one Job with a given name may exist in a project at any\ngiven time. If a caller attempts to create a Job with the same\nname as an already-existing Job, the attempt returns the\nexisting Job.\n\nThe name must match the regular expression\n`[a-z]([-a-z0-9]{0,38}[a-z0-9])?`"]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Preliminary field: The format of this data may change at any time.\nA description of the user pipeline and stages through which it is executed.\nCreated by Cloud Dataflow service.  Only retrieved with\nJOB_VIEW_DESCRIPTION or JOB_VIEW_ALL."]
        #[serde(rename = "pipelineDescription", default)]
        pub pipeline_description: Option<crate::schemas::PipelineDescription>,
        #[doc = "The ID of the Cloud Platform project that the job belongs to."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "The type of Cloud Dataflow job."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::JobType>,
        #[doc = "If this job is an update of an existing job, this field is the job ID\nof the job it replaced.\n\nWhen sending a `CreateJobRequest`, you can update a job by specifying it\nhere. The job named here is stopped, and its intermediate state is\ntransferred to this job."]
        #[serde(rename = "replaceJobId", default)]
        pub replace_job_id: Option<String>,
        #[doc = "If another job is an update of this job (and thus, this job is in\n`JOB_STATE_UPDATED`), this field contains the ID of that job."]
        #[serde(rename = "replacedByJobId", default)]
        pub replaced_by_job_id: Option<String>,
        #[doc = "The job's requested state.\n\n`UpdateJob` may be used to switch between the `JOB_STATE_STOPPED` and\n`JOB_STATE_RUNNING` states, by setting requested_state.  `UpdateJob` may\nalso be used to directly set a job's requested state to\n`JOB_STATE_CANCELLED` or `JOB_STATE_DONE`, irrevocably terminating the\njob if it has not already reached a terminal state."]
        #[serde(rename = "requestedState", default)]
        pub requested_state: Option<crate::schemas::JobRequestedState>,
        #[doc = "This field may be mutated by the Cloud Dataflow service;\ncallers cannot mutate it."]
        #[serde(rename = "stageStates", default)]
        pub stage_states: Option<Vec<crate::schemas::ExecutionStageState>>,
        #[doc = "The timestamp when the job was started (transitioned to JOB_STATE_PENDING).\nFlexible resource scheduling jobs are started with some delay after job\ncreation, so start_time is unset before start and is updated when the\njob is started by the Cloud Dataflow service. For other jobs, start_time\nalways equals to create_time and is immutable and set by the Cloud Dataflow\nservice."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Exactly one of step or steps_location should be specified.\n\nThe top-level steps that constitute the entire job."]
        #[serde(rename = "steps", default)]
        pub steps: Option<Vec<crate::schemas::Step>>,
        #[doc = "The GCS location where the steps are stored."]
        #[serde(rename = "stepsLocation", default)]
        pub steps_location: Option<String>,
        #[doc = "A set of files the system should be aware of that are used\nfor temporary storage. These temporary files will be\nremoved on job completion.\nNo duplicates are allowed.\nNo file patterns are supported.\n\nThe supported files are:\n\nGoogle Cloud Storage:\n\n   storage.googleapis.com/{bucket}/{object}\n   bucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempFiles", default)]
        pub temp_files: Option<Vec<String>>,
        #[doc = "The map of transform name prefixes of the job to be replaced to the\ncorresponding name prefixes of the new job."]
        #[serde(rename = "transformNameMapping", default)]
        pub transform_name_mapping: Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for Job {
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
    pub struct JobExecutionInfo {
        #[doc = "A mapping from each stage to the information about that stage."]
        #[serde(rename = "stages", default)]
        pub stages:
            Option<::std::collections::BTreeMap<String, crate::schemas::JobExecutionStageInfo>>,
    }
    impl ::field_selector::FieldSelector for JobExecutionInfo {
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
    pub struct JobExecutionStageInfo {
        #[doc = "The steps associated with the execution stage.\nNote that stages may have several steps, and that a given step\nmight be run by more than one stage."]
        #[serde(rename = "stepName", default)]
        pub step_name: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for JobExecutionStageInfo {
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
    pub enum JobMessageMessageImportance {
        #[doc = "The message importance isn't specified, or is unknown."]
        JobMessageImportanceUnknown,
        #[doc = "The message is at the 'debug' level: typically only useful for\nsoftware engineers working on the code the job is running.\nTypically, Dataflow pipeline runners do not display log messages\nat this level by default."]
        JobMessageDebug,
        #[doc = "The message is at the 'detailed' level: somewhat verbose, but\npotentially useful to users.  Typically, Dataflow pipeline\nrunners do not display log messages at this level by default.\nThese messages are displayed by default in the Dataflow\nmonitoring UI."]
        JobMessageDetailed,
        #[doc = "The message is at the 'basic' level: useful for keeping\ntrack of the execution of a Dataflow pipeline.  Typically,\nDataflow pipeline runners display log messages at this level by\ndefault, and these messages are displayed by default in the\nDataflow monitoring UI."]
        JobMessageBasic,
        #[doc = "The message is at the 'warning' level: indicating a condition\npertaining to a job which may require human intervention.\nTypically, Dataflow pipeline runners display log messages at this\nlevel by default, and these messages are displayed by default in\nthe Dataflow monitoring UI."]
        JobMessageWarning,
        #[doc = "The message is at the 'error' level: indicating a condition\npreventing a job from succeeding.  Typically, Dataflow pipeline\nrunners display log messages at this level by default, and these\nmessages are displayed by default in the Dataflow monitoring UI."]
        JobMessageError,
    }
    impl JobMessageMessageImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                JobMessageMessageImportance::JobMessageImportanceUnknown => {
                    "JOB_MESSAGE_IMPORTANCE_UNKNOWN"
                }
                JobMessageMessageImportance::JobMessageDebug => "JOB_MESSAGE_DEBUG",
                JobMessageMessageImportance::JobMessageDetailed => "JOB_MESSAGE_DETAILED",
                JobMessageMessageImportance::JobMessageBasic => "JOB_MESSAGE_BASIC",
                JobMessageMessageImportance::JobMessageWarning => "JOB_MESSAGE_WARNING",
                JobMessageMessageImportance::JobMessageError => "JOB_MESSAGE_ERROR",
            }
        }
    }
    impl ::std::fmt::Display for JobMessageMessageImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobMessageMessageImportance {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobMessageMessageImportance {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_MESSAGE_IMPORTANCE_UNKNOWN" => {
                    JobMessageMessageImportance::JobMessageImportanceUnknown
                }
                "JOB_MESSAGE_DEBUG" => JobMessageMessageImportance::JobMessageDebug,
                "JOB_MESSAGE_DETAILED" => JobMessageMessageImportance::JobMessageDetailed,
                "JOB_MESSAGE_BASIC" => JobMessageMessageImportance::JobMessageBasic,
                "JOB_MESSAGE_WARNING" => JobMessageMessageImportance::JobMessageWarning,
                "JOB_MESSAGE_ERROR" => JobMessageMessageImportance::JobMessageError,
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
    pub struct JobMessage {
        #[doc = "Deprecated."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Importance level of the message."]
        #[serde(rename = "messageImportance", default)]
        pub message_importance: Option<crate::schemas::JobMessageMessageImportance>,
        #[doc = "The text of the message."]
        #[serde(rename = "messageText", default)]
        pub message_text: Option<String>,
        #[doc = "The timestamp of the message."]
        #[serde(rename = "time", default)]
        pub time: Option<String>,
    }
    impl ::field_selector::FieldSelector for JobMessage {
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
    pub struct JobMetadata {
        #[doc = "Identification of a BigTable source used in the Dataflow job."]
        #[serde(rename = "bigTableDetails", default)]
        pub big_table_details: Option<Vec<crate::schemas::BigTableIODetails>>,
        #[doc = "Identification of a BigQuery source used in the Dataflow job."]
        #[serde(rename = "bigqueryDetails", default)]
        pub bigquery_details: Option<Vec<crate::schemas::BigQueryIODetails>>,
        #[doc = "Identification of a Datastore source used in the Dataflow job."]
        #[serde(rename = "datastoreDetails", default)]
        pub datastore_details: Option<Vec<crate::schemas::DatastoreIODetails>>,
        #[doc = "Identification of a File source used in the Dataflow job."]
        #[serde(rename = "fileDetails", default)]
        pub file_details: Option<Vec<crate::schemas::FileIODetails>>,
        #[doc = "Identification of a PubSub source used in the Dataflow job."]
        #[serde(rename = "pubsubDetails", default)]
        pub pubsub_details: Option<Vec<crate::schemas::PubSubIODetails>>,
        #[doc = "The SDK version used to run the job."]
        #[serde(rename = "sdkVersion", default)]
        pub sdk_version: Option<crate::schemas::SdkVersion>,
        #[doc = "Identification of a Spanner source used in the Dataflow job."]
        #[serde(rename = "spannerDetails", default)]
        pub spanner_details: Option<Vec<crate::schemas::SpannerIODetails>>,
    }
    impl ::field_selector::FieldSelector for JobMetadata {
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
    pub struct JobMetrics {
        #[doc = "Timestamp as of which metric values are current."]
        #[serde(rename = "metricTime", default)]
        pub metric_time: Option<String>,
        #[doc = "All metrics for this job."]
        #[serde(rename = "metrics", default)]
        pub metrics: Option<Vec<crate::schemas::MetricUpdate>>,
    }
    impl ::field_selector::FieldSelector for JobMetrics {
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
    pub struct KeyRangeDataDiskAssignment {
        #[doc = "The name of the data disk where data for this range is stored.\nThis name is local to the Google Cloud Platform project and uniquely\nidentifies the disk within that project, for example\n\"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        #[serde(rename = "dataDisk", default)]
        pub data_disk: Option<String>,
        #[doc = "The end (exclusive) of the key range."]
        #[serde(rename = "end", default)]
        pub end: Option<String>,
        #[doc = "The start (inclusive) of the key range."]
        #[serde(rename = "start", default)]
        pub start: Option<String>,
    }
    impl ::field_selector::FieldSelector for KeyRangeDataDiskAssignment {
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
    pub struct KeyRangeLocation {
        #[doc = "The name of the data disk where data for this range is stored.\nThis name is local to the Google Cloud Platform project and uniquely\nidentifies the disk within that project, for example\n\"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        #[serde(rename = "dataDisk", default)]
        pub data_disk: Option<String>,
        #[doc = "The physical location of this range assignment to be used for\nstreaming computation cross-worker message delivery."]
        #[serde(rename = "deliveryEndpoint", default)]
        pub delivery_endpoint: Option<String>,
        #[doc = "DEPRECATED. The location of the persistent state for this range, as a\npersistent directory in the worker local filesystem."]
        #[serde(rename = "deprecatedPersistentDirectory", default)]
        pub deprecated_persistent_directory: Option<String>,
        #[doc = "The end (exclusive) of the key range."]
        #[serde(rename = "end", default)]
        pub end: Option<String>,
        #[doc = "The start (inclusive) of the key range."]
        #[serde(rename = "start", default)]
        pub start: Option<String>,
    }
    impl ::field_selector::FieldSelector for KeyRangeLocation {
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
    pub struct LaunchTemplateParameters {
        #[doc = "The runtime environment for the job."]
        #[serde(rename = "environment", default)]
        pub environment: Option<crate::schemas::RuntimeEnvironment>,
        #[doc = "Required. The job name to use for the created job."]
        #[serde(rename = "jobName", default)]
        pub job_name: Option<String>,
        #[doc = "The runtime parameters to pass to the job."]
        #[serde(rename = "parameters", default)]
        pub parameters: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Only applicable when updating a pipeline. Map of transform name prefixes of\nthe job to be replaced to the corresponding name prefixes of the new job."]
        #[serde(rename = "transformNameMapping", default)]
        pub transform_name_mapping: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "If set, replace the existing pipeline with the name specified by jobName\nwith this pipeline, preserving state."]
        #[serde(rename = "update", default)]
        pub update: Option<bool>,
    }
    impl ::field_selector::FieldSelector for LaunchTemplateParameters {
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
    pub struct LaunchTemplateResponse {
        #[doc = "The job that was launched, if the request was not a dry run and\nthe job was successfully launched."]
        #[serde(rename = "job", default)]
        pub job: Option<crate::schemas::Job>,
    }
    impl ::field_selector::FieldSelector for LaunchTemplateResponse {
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
    pub struct LeaseWorkItemRequest {
        #[doc = "The current timestamp at the worker."]
        #[serde(rename = "currentWorkerTime", default)]
        pub current_worker_time: Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the WorkItem's job."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The initial lease period."]
        #[serde(rename = "requestedLeaseDuration", default)]
        pub requested_lease_duration: Option<String>,
        #[doc = "Untranslated bag-of-bytes WorkRequest from UnifiedWorker."]
        #[serde(rename = "unifiedWorkerRequest", default)]
        pub unified_worker_request:
            Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Filter for WorkItem type."]
        #[serde(rename = "workItemTypes", default)]
        pub work_item_types: Option<Vec<String>>,
        #[doc = "Worker capabilities. WorkItems might be limited to workers with specific\ncapabilities."]
        #[serde(rename = "workerCapabilities", default)]
        pub worker_capabilities: Option<Vec<String>>,
        #[doc = "Identifies the worker leasing work -- typically the ID of the\nvirtual machine running the worker."]
        #[serde(rename = "workerId", default)]
        pub worker_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for LeaseWorkItemRequest {
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
    pub struct LeaseWorkItemResponse {
        #[doc = "Untranslated bag-of-bytes WorkResponse for UnifiedWorker."]
        #[serde(rename = "unifiedWorkerResponse", default)]
        pub unified_worker_response:
            Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A list of the leased WorkItems."]
        #[serde(rename = "workItems", default)]
        pub work_items: Option<Vec<crate::schemas::WorkItem>>,
    }
    impl ::field_selector::FieldSelector for LeaseWorkItemResponse {
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
    pub struct ListJobMessagesResponse {
        #[doc = "Autoscaling events in ascending timestamp order."]
        #[serde(rename = "autoscalingEvents", default)]
        pub autoscaling_events: Option<Vec<crate::schemas::AutoscalingEvent>>,
        #[doc = "Messages in ascending timestamp order."]
        #[serde(rename = "jobMessages", default)]
        pub job_messages: Option<Vec<crate::schemas::JobMessage>>,
        #[doc = "The token to obtain the next page of results if there are more."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListJobMessagesResponse {
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
    pub struct ListJobsResponse {
        #[doc = "Zero or more messages describing the [regional endpoints]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\nfailed to respond."]
        #[serde(rename = "failedLocation", default)]
        pub failed_location: Option<Vec<crate::schemas::FailedLocation>>,
        #[doc = "A subset of the requested job information."]
        #[serde(rename = "jobs", default)]
        pub jobs: Option<Vec<crate::schemas::Job>>,
        #[doc = "Set if there may be more results than fit in this response."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListJobsResponse {
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
    pub struct ListSnapshotsResponse {
        #[doc = "Returned snapshots."]
        #[serde(rename = "snapshots", default)]
        pub snapshots: Option<Vec<crate::schemas::Snapshot>>,
    }
    impl ::field_selector::FieldSelector for ListSnapshotsResponse {
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
    pub struct MapTask {
        #[doc = "Counter prefix that can be used to prefix counters. Not currently used in\nDataflow."]
        #[serde(rename = "counterPrefix", default)]
        pub counter_prefix: Option<String>,
        #[doc = "The instructions in the MapTask."]
        #[serde(rename = "instructions", default)]
        pub instructions: Option<Vec<crate::schemas::ParallelInstruction>>,
        #[doc = "System-defined name of the stage containing this MapTask.\nUnique across the workflow."]
        #[serde(rename = "stageName", default)]
        pub stage_name: Option<String>,
        #[doc = "System-defined name of this MapTask.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for MapTask {
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
    pub struct MetricShortId {
        #[doc = "The index of the corresponding metric in\nthe ReportWorkItemStatusRequest. Required."]
        #[serde(rename = "metricIndex", default)]
        pub metric_index: Option<i32>,
        #[doc = "The service-generated short identifier for the metric."]
        #[serde(rename = "shortId", default)]
        #[serde(with = "crate::parsed_string")]
        pub short_id: Option<i64>,
    }
    impl ::field_selector::FieldSelector for MetricShortId {
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
    pub struct MetricStructuredName {
        #[doc = "Zero or more labeled fields which identify the part of the job this\nmetric is associated with, such as the name of a step or collection.\n\nFor example, built-in counters associated with steps will have\ncontext['step'] = <step-name>. Counters associated with PCollections\nin the SDK will have context['pcollection'] = <pcollection-name>."]
        #[serde(rename = "context", default)]
        pub context: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Worker-defined metric name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Origin (namespace) of metric name. May be blank for user-define metrics;\nwill be \"dataflow\" for metrics defined by the Dataflow service or SDK."]
        #[serde(rename = "origin", default)]
        pub origin: Option<String>,
    }
    impl ::field_selector::FieldSelector for MetricStructuredName {
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
    pub struct MetricUpdate {
        #[doc = "True if this metric is reported as the total cumulative aggregate\nvalue accumulated since the worker started working on this WorkItem.\nBy default this is false, indicating that this metric is reported\nas a delta that is not associated with any WorkItem."]
        #[serde(rename = "cumulative", default)]
        pub cumulative: Option<bool>,
        #[doc = "A struct value describing properties of a distribution of numeric values."]
        #[serde(rename = "distribution", default)]
        pub distribution: Option<::serde_json::Value>,
        #[doc = "A struct value describing properties of a Gauge.\nMetrics of gauge type show the value of a metric across time, and is\naggregated based on the newest value."]
        #[serde(rename = "gauge", default)]
        pub gauge: Option<::serde_json::Value>,
        #[doc = "Worker-computed aggregate value for internal use by the Dataflow\nservice."]
        #[serde(rename = "internal", default)]
        pub internal: Option<::serde_json::Value>,
        #[doc = "Metric aggregation kind.  The possible metric aggregation kinds are\n\"Sum\", \"Max\", \"Min\", \"Mean\", \"Set\", \"And\", \"Or\", and \"Distribution\".\nThe specified aggregation kind is case-insensitive.\n\nIf omitted, this is not an aggregated value but instead\na single metric sample value."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Worker-computed aggregate value for the \"Mean\" aggregation kind.\nThis holds the count of the aggregated values and is used in combination\nwith mean_sum above to obtain the actual mean aggregate value.\nThe only possible value type is Long."]
        #[serde(rename = "meanCount", default)]
        pub mean_count: Option<::serde_json::Value>,
        #[doc = "Worker-computed aggregate value for the \"Mean\" aggregation kind.\nThis holds the sum of the aggregated values and is used in combination\nwith mean_count below to obtain the actual mean aggregate value.\nThe only possible value types are Long and Double."]
        #[serde(rename = "meanSum", default)]
        pub mean_sum: Option<::serde_json::Value>,
        #[doc = "Name of the metric."]
        #[serde(rename = "name", default)]
        pub name: Option<crate::schemas::MetricStructuredName>,
        #[doc = "Worker-computed aggregate value for aggregation kinds \"Sum\", \"Max\", \"Min\",\n\"And\", and \"Or\".  The possible value types are Long, Double, and Boolean."]
        #[serde(rename = "scalar", default)]
        pub scalar: Option<::serde_json::Value>,
        #[doc = "Worker-computed aggregate value for the \"Set\" aggregation kind.  The only\npossible value type is a list of Values whose type can be Long, Double,\nor String, according to the metric's type.  All Values in the list must\nbe of the same type."]
        #[serde(rename = "set", default)]
        pub set: Option<::serde_json::Value>,
        #[doc = "Timestamp associated with the metric value. Optional when workers are\nreporting work progress; it will be filled in responses from the\nmetrics API."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for MetricUpdate {
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
    pub struct MountedDataDisk {
        #[doc = "The name of the data disk.\nThis name is local to the Google Cloud Platform project and uniquely\nidentifies the disk within that project, for example\n\"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        #[serde(rename = "dataDisk", default)]
        pub data_disk: Option<String>,
    }
    impl ::field_selector::FieldSelector for MountedDataDisk {
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
    pub struct MultiOutputInfo {
        #[doc = "The id of the tag the user code will emit to this output by; this\nshould correspond to the tag of some SideInputInfo."]
        #[serde(rename = "tag", default)]
        pub tag: Option<String>,
    }
    impl ::field_selector::FieldSelector for MultiOutputInfo {
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
    pub enum NameAndKindKind {
        #[doc = "Counter aggregation kind was not set."]
        Invalid,
        #[doc = "Aggregated value is the sum of all contributed values."]
        Sum,
        #[doc = "Aggregated value is the max of all contributed values."]
        Max,
        #[doc = "Aggregated value is the min of all contributed values."]
        Min,
        #[doc = "Aggregated value is the mean of all contributed values."]
        Mean,
        #[doc = "Aggregated value represents the logical 'or' of all contributed values."]
        Or,
        #[doc = "Aggregated value represents the logical 'and' of all contributed values."]
        And,
        #[doc = "Aggregated value is a set of unique contributed values."]
        Set,
        #[doc = "Aggregated value captures statistics about a distribution."]
        Distribution,
        #[doc = "Aggregated value tracks the latest value of a variable."]
        LatestValue,
    }
    impl NameAndKindKind {
        pub fn as_str(self) -> &'static str {
            match self {
                NameAndKindKind::Invalid => "INVALID",
                NameAndKindKind::Sum => "SUM",
                NameAndKindKind::Max => "MAX",
                NameAndKindKind::Min => "MIN",
                NameAndKindKind::Mean => "MEAN",
                NameAndKindKind::Or => "OR",
                NameAndKindKind::And => "AND",
                NameAndKindKind::Set => "SET",
                NameAndKindKind::Distribution => "DISTRIBUTION",
                NameAndKindKind::LatestValue => "LATEST_VALUE",
            }
        }
    }
    impl ::std::fmt::Display for NameAndKindKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NameAndKindKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NameAndKindKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVALID" => NameAndKindKind::Invalid,
                "SUM" => NameAndKindKind::Sum,
                "MAX" => NameAndKindKind::Max,
                "MIN" => NameAndKindKind::Min,
                "MEAN" => NameAndKindKind::Mean,
                "OR" => NameAndKindKind::Or,
                "AND" => NameAndKindKind::And,
                "SET" => NameAndKindKind::Set,
                "DISTRIBUTION" => NameAndKindKind::Distribution,
                "LATEST_VALUE" => NameAndKindKind::LatestValue,
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
    pub struct NameAndKind {
        #[doc = "Counter aggregation kind."]
        #[serde(rename = "kind", default)]
        pub kind: Option<crate::schemas::NameAndKindKind>,
        #[doc = "Name of the counter."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for NameAndKind {
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
    pub struct Package {
        #[doc = "The resource to read the package from. The supported resource type is:\n\nGoogle Cloud Storage:\n\n  storage.googleapis.com/{bucket}\n  bucket.storage.googleapis.com/"]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The name of the package."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Package {
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
    pub struct ParDoInstruction {
        #[doc = "The input."]
        #[serde(rename = "input", default)]
        pub input: Option<crate::schemas::InstructionInput>,
        #[doc = "Information about each of the outputs, if user_fn is a  MultiDoFn."]
        #[serde(rename = "multiOutputInfos", default)]
        pub multi_output_infos: Option<Vec<crate::schemas::MultiOutputInfo>>,
        #[doc = "The number of outputs."]
        #[serde(rename = "numOutputs", default)]
        pub num_outputs: Option<i32>,
        #[doc = "Zero or more side inputs."]
        #[serde(rename = "sideInputs", default)]
        pub side_inputs: Option<Vec<crate::schemas::SideInputInfo>>,
        #[doc = "The user function to invoke."]
        #[serde(rename = "userFn", default)]
        pub user_fn: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for ParDoInstruction {
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
    pub struct ParallelInstruction {
        #[doc = "Additional information for Flatten instructions."]
        #[serde(rename = "flatten", default)]
        pub flatten: Option<crate::schemas::FlattenInstruction>,
        #[doc = "User-provided name of this operation."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "System-defined name for the operation in the original workflow graph."]
        #[serde(rename = "originalName", default)]
        pub original_name: Option<String>,
        #[doc = "Describes the outputs of the instruction."]
        #[serde(rename = "outputs", default)]
        pub outputs: Option<Vec<crate::schemas::InstructionOutput>>,
        #[doc = "Additional information for ParDo instructions."]
        #[serde(rename = "parDo", default)]
        pub par_do: Option<crate::schemas::ParDoInstruction>,
        #[doc = "Additional information for PartialGroupByKey instructions."]
        #[serde(rename = "partialGroupByKey", default)]
        pub partial_group_by_key: Option<crate::schemas::PartialGroupByKeyInstruction>,
        #[doc = "Additional information for Read instructions."]
        #[serde(rename = "read", default)]
        pub read: Option<crate::schemas::ReadInstruction>,
        #[doc = "System-defined name of this operation.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: Option<String>,
        #[doc = "Additional information for Write instructions."]
        #[serde(rename = "write", default)]
        pub write: Option<crate::schemas::WriteInstruction>,
    }
    impl ::field_selector::FieldSelector for ParallelInstruction {
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
    pub struct Parameter {
        #[doc = "Key or name for this parameter."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "Value for this parameter."]
        #[serde(rename = "value", default)]
        pub value: Option<::serde_json::Value>,
    }
    impl ::field_selector::FieldSelector for Parameter {
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
    pub struct ParameterMetadata {
        #[doc = "Required. The help text to display for the parameter."]
        #[serde(rename = "helpText", default)]
        pub help_text: Option<String>,
        #[doc = "Optional. Whether the parameter is optional. Defaults to false."]
        #[serde(rename = "isOptional", default)]
        pub is_optional: Option<bool>,
        #[doc = "Required. The label to display for the parameter."]
        #[serde(rename = "label", default)]
        pub label: Option<String>,
        #[doc = "Required. The name of the parameter."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Optional. Regexes that the parameter must match."]
        #[serde(rename = "regexes", default)]
        pub regexes: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ParameterMetadata {
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
    pub struct PartialGroupByKeyInstruction {
        #[doc = "Describes the input to the partial group-by-key instruction."]
        #[serde(rename = "input", default)]
        pub input: Option<crate::schemas::InstructionInput>,
        #[doc = "The codec to use for interpreting an element in the input PTable."]
        #[serde(rename = "inputElementCodec", default)]
        pub input_element_codec: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "If this instruction includes a combining function this is the name of the\nintermediate store between the GBK and the CombineValues."]
        #[serde(rename = "originalCombineValuesInputStoreName", default)]
        pub original_combine_values_input_store_name: Option<String>,
        #[doc = "If this instruction includes a combining function, this is the name of the\nCombineValues instruction lifted into this instruction."]
        #[serde(rename = "originalCombineValuesStepName", default)]
        pub original_combine_values_step_name: Option<String>,
        #[doc = "Zero or more side inputs."]
        #[serde(rename = "sideInputs", default)]
        pub side_inputs: Option<Vec<crate::schemas::SideInputInfo>>,
        #[doc = "The value combining function to invoke."]
        #[serde(rename = "valueCombiningFn", default)]
        pub value_combining_fn: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for PartialGroupByKeyInstruction {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PipelineDescription {
        #[doc = "Pipeline level display data."]
        #[serde(rename = "displayData", default)]
        pub display_data: Option<Vec<crate::schemas::DisplayData>>,
        #[doc = "Description of each stage of execution of the pipeline."]
        #[serde(rename = "executionPipelineStage", default)]
        pub execution_pipeline_stage: Option<Vec<crate::schemas::ExecutionStageSummary>>,
        #[doc = "Description of each transform in the pipeline and collections between them."]
        #[serde(rename = "originalPipelineTransform", default)]
        pub original_pipeline_transform: Option<Vec<crate::schemas::TransformSummary>>,
    }
    impl ::field_selector::FieldSelector for PipelineDescription {
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
    pub struct Position {
        #[doc = "Position is a byte offset."]
        #[serde(rename = "byteOffset", default)]
        #[serde(with = "crate::parsed_string")]
        pub byte_offset: Option<i64>,
        #[doc = "CloudPosition is a concat position."]
        #[serde(rename = "concatPosition", default)]
        pub concat_position: Option<crate::schemas::ConcatPosition>,
        #[doc = "Position is past all other positions. Also useful for the end\nposition of an unbounded range."]
        #[serde(rename = "end", default)]
        pub end: Option<bool>,
        #[doc = "Position is a string key, ordered lexicographically."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "Position is a record index."]
        #[serde(rename = "recordIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub record_index: Option<i64>,
        #[doc = "CloudPosition is a base64 encoded BatchShufflePosition (with FIXED\nsharding)."]
        #[serde(rename = "shufflePosition", default)]
        pub shuffle_position: Option<String>,
    }
    impl ::field_selector::FieldSelector for Position {
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
    pub struct PubSubIODetails {
        #[doc = "Subscription used in the connection."]
        #[serde(rename = "subscription", default)]
        pub subscription: Option<String>,
        #[doc = "Topic accessed in the connection."]
        #[serde(rename = "topic", default)]
        pub topic: Option<String>,
    }
    impl ::field_selector::FieldSelector for PubSubIODetails {
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
    pub struct PubsubLocation {
        #[doc = "Indicates whether the pipeline allows late-arriving data."]
        #[serde(rename = "dropLateData", default)]
        pub drop_late_data: Option<bool>,
        #[doc = "If set, contains a pubsub label from which to extract record ids.\nIf left empty, record deduplication will be strictly best effort."]
        #[serde(rename = "idLabel", default)]
        pub id_label: Option<String>,
        #[doc = "A pubsub subscription, in the form of\n\"pubsub.googleapis.com/subscriptions/<project-id>/<subscription-name>\""]
        #[serde(rename = "subscription", default)]
        pub subscription: Option<String>,
        #[doc = "If set, contains a pubsub label from which to extract record timestamps.\nIf left empty, record timestamps will be generated upon arrival."]
        #[serde(rename = "timestampLabel", default)]
        pub timestamp_label: Option<String>,
        #[doc = "A pubsub topic, in the form of\n\"pubsub.googleapis.com/topics/<project-id>/<topic-name>\""]
        #[serde(rename = "topic", default)]
        pub topic: Option<String>,
        #[doc = "If set, specifies the pubsub subscription that will be used for tracking\ncustom time timestamps for watermark estimation."]
        #[serde(rename = "trackingSubscription", default)]
        pub tracking_subscription: Option<String>,
        #[doc = "If true, then the client has requested to get pubsub attributes."]
        #[serde(rename = "withAttributes", default)]
        pub with_attributes: Option<bool>,
    }
    impl ::field_selector::FieldSelector for PubsubLocation {
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
    pub struct ReadInstruction {
        #[doc = "The source to read from."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for ReadInstruction {
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
    pub struct ReportWorkItemStatusRequest {
        #[doc = "The current timestamp at the worker."]
        #[serde(rename = "currentWorkerTime", default)]
        pub current_worker_time: Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the WorkItem's job."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "Untranslated bag-of-bytes WorkProgressUpdateRequest from UnifiedWorker."]
        #[serde(rename = "unifiedWorkerRequest", default)]
        pub unified_worker_request:
            Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The order is unimportant, except that the order of the\nWorkItemServiceState messages in the ReportWorkItemStatusResponse\ncorresponds to the order of WorkItemStatus messages here."]
        #[serde(rename = "workItemStatuses", default)]
        pub work_item_statuses: Option<Vec<crate::schemas::WorkItemStatus>>,
        #[doc = "The ID of the worker reporting the WorkItem status.  If this\ndoes not match the ID of the worker which the Dataflow service\nbelieves currently has the lease on the WorkItem, the report\nwill be dropped (with an error response)."]
        #[serde(rename = "workerId", default)]
        pub worker_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ReportWorkItemStatusRequest {
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
    pub struct ReportWorkItemStatusResponse {
        #[doc = "Untranslated bag-of-bytes WorkProgressUpdateResponse for UnifiedWorker."]
        #[serde(rename = "unifiedWorkerResponse", default)]
        pub unified_worker_response:
            Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A set of messages indicating the service-side state for each\nWorkItem whose status was reported, in the same order as the\nWorkItemStatus messages in the ReportWorkItemStatusRequest which\nresulting in this response."]
        #[serde(rename = "workItemServiceStates", default)]
        pub work_item_service_states: Option<Vec<crate::schemas::WorkItemServiceState>>,
    }
    impl ::field_selector::FieldSelector for ReportWorkItemStatusResponse {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ReportedParallelism {
        #[doc = "Specifies whether the parallelism is infinite. If true, \"value\" is\nignored.\nInfinite parallelism means the service will assume that the work item\ncan always be split into more non-empty work items by dynamic splitting.\nThis is a work-around for lack of support for infinity by the current\nJSON-based Java RPC stack."]
        #[serde(rename = "isInfinite", default)]
        pub is_infinite: Option<bool>,
        #[doc = "Specifies the level of parallelism in case it is finite."]
        #[serde(rename = "value", default)]
        pub value: Option<f64>,
    }
    impl ::field_selector::FieldSelector for ReportedParallelism {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResourceUtilizationReport {
        #[doc = "CPU utilization samples."]
        #[serde(rename = "cpuTime", default)]
        pub cpu_time: Option<Vec<crate::schemas::Cputime>>,
    }
    impl ::field_selector::FieldSelector for ResourceUtilizationReport {
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
    pub struct ResourceUtilizationReportResponse;
    impl ::field_selector::FieldSelector for ResourceUtilizationReportResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
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
    pub struct RuntimeEnvironment {
        #[doc = "Additional experiment flags for the job."]
        #[serde(rename = "additionalExperiments", default)]
        pub additional_experiments: Option<Vec<String>>,
        #[doc = "Additional user labels to be specified for the job.\nKeys and values should follow the restrictions specified in the [labeling\nrestrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions)\npage."]
        #[serde(rename = "additionalUserLabels", default)]
        pub additional_user_labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Whether to bypass the safety checks for the job's temporary directory.\nUse with caution."]
        #[serde(rename = "bypassTempDirValidation", default)]
        pub bypass_temp_dir_validation: Option<bool>,
        #[doc = "Optional. Name for the Cloud KMS key for the job.\nKey format is:\nprojects/<project>/locations/<location>/keyRings/<keyring>/cryptoKeys/<key>"]
        #[serde(rename = "kmsKeyName", default)]
        pub kms_key_name: Option<String>,
        #[doc = "The machine type to use for the job. Defaults to the value from the\ntemplate if not specified."]
        #[serde(rename = "machineType", default)]
        pub machine_type: Option<String>,
        #[doc = "The maximum number of Google Compute Engine instances to be made\navailable to your pipeline during execution, from 1 to 1000."]
        #[serde(rename = "maxWorkers", default)]
        pub max_workers: Option<i32>,
        #[doc = "Network to which VMs will be assigned.  If empty or unspecified,\nthe service will use the network \"default\"."]
        #[serde(rename = "network", default)]
        pub network: Option<String>,
        #[doc = "The initial number of Google Compute Engine instnaces for the job."]
        #[serde(rename = "numWorkers", default)]
        pub num_workers: Option<i32>,
        #[doc = "The email address of the service account to run the job as."]
        #[serde(rename = "serviceAccountEmail", default)]
        pub service_account_email: Option<String>,
        #[doc = "Subnetwork to which VMs will be assigned, if desired.  Expected to be of\nthe form \"regions/REGION/subnetworks/SUBNETWORK\"."]
        #[serde(rename = "subnetwork", default)]
        pub subnetwork: Option<String>,
        #[doc = "The Cloud Storage path to use for temporary files.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
        #[serde(rename = "tempLocation", default)]
        pub temp_location: Option<String>,
        #[doc = "The Compute Engine [availability\nzone](https://cloud.google.com/compute/docs/regions-zones/regions-zones)\nfor launching worker instances to run your pipeline."]
        #[serde(rename = "zone", default)]
        pub zone: Option<String>,
    }
    impl ::field_selector::FieldSelector for RuntimeEnvironment {
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
    pub enum SdkVersionSdkSupportStatus {
        #[doc = "Cloud Dataflow is unaware of this version."]
        Unknown,
        #[doc = "This is a known version of an SDK, and is supported."]
        Supported,
        #[doc = "A newer version of the SDK family exists, and an update is recommended."]
        Stale,
        #[doc = "This version of the SDK is deprecated and will eventually be no\nlonger supported."]
        Deprecated,
        #[doc = "Support for this SDK version has ended and it should no longer be used."]
        Unsupported,
    }
    impl SdkVersionSdkSupportStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                SdkVersionSdkSupportStatus::Unknown => "UNKNOWN",
                SdkVersionSdkSupportStatus::Supported => "SUPPORTED",
                SdkVersionSdkSupportStatus::Stale => "STALE",
                SdkVersionSdkSupportStatus::Deprecated => "DEPRECATED",
                SdkVersionSdkSupportStatus::Unsupported => "UNSUPPORTED",
            }
        }
    }
    impl ::std::fmt::Display for SdkVersionSdkSupportStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SdkVersionSdkSupportStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SdkVersionSdkSupportStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => SdkVersionSdkSupportStatus::Unknown,
                "SUPPORTED" => SdkVersionSdkSupportStatus::Supported,
                "STALE" => SdkVersionSdkSupportStatus::Stale,
                "DEPRECATED" => SdkVersionSdkSupportStatus::Deprecated,
                "UNSUPPORTED" => SdkVersionSdkSupportStatus::Unsupported,
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
    pub struct SdkVersion {
        #[doc = "The support status for this SDK version."]
        #[serde(rename = "sdkSupportStatus", default)]
        pub sdk_support_status: Option<crate::schemas::SdkVersionSdkSupportStatus>,
        #[doc = "The version of the SDK used to run the job."]
        #[serde(rename = "version", default)]
        pub version: Option<String>,
        #[doc = "A readable string describing the version of the SDK."]
        #[serde(rename = "versionDisplayName", default)]
        pub version_display_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for SdkVersion {
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
    pub struct SendDebugCaptureRequest {
        #[doc = "The internal component id for which debug information is sent."]
        #[serde(rename = "componentId", default)]
        pub component_id: Option<String>,
        #[doc = "The encoded debug information."]
        #[serde(rename = "data", default)]
        pub data: Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The worker id, i.e., VM hostname."]
        #[serde(rename = "workerId", default)]
        pub worker_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for SendDebugCaptureRequest {
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
    pub struct SendDebugCaptureResponse;
    impl ::field_selector::FieldSelector for SendDebugCaptureResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SendWorkerMessagesRequest {
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The WorkerMessages to send."]
        #[serde(rename = "workerMessages", default)]
        pub worker_messages: Option<Vec<crate::schemas::WorkerMessage>>,
    }
    impl ::field_selector::FieldSelector for SendWorkerMessagesRequest {
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
    pub struct SendWorkerMessagesResponse {
        #[doc = "The servers response to the worker messages."]
        #[serde(rename = "workerMessageResponses", default)]
        pub worker_message_responses: Option<Vec<crate::schemas::WorkerMessageResponse>>,
    }
    impl ::field_selector::FieldSelector for SendWorkerMessagesResponse {
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
    pub struct SeqMapTask {
        #[doc = "Information about each of the inputs."]
        #[serde(rename = "inputs", default)]
        pub inputs: Option<Vec<crate::schemas::SideInputInfo>>,
        #[doc = "The user-provided name of the SeqDo operation."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Information about each of the outputs."]
        #[serde(rename = "outputInfos", default)]
        pub output_infos: Option<Vec<crate::schemas::SeqMapTaskOutputInfo>>,
        #[doc = "System-defined name of the stage containing the SeqDo operation.\nUnique across the workflow."]
        #[serde(rename = "stageName", default)]
        pub stage_name: Option<String>,
        #[doc = "System-defined name of the SeqDo operation.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: Option<String>,
        #[doc = "The user function to invoke."]
        #[serde(rename = "userFn", default)]
        pub user_fn: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for SeqMapTask {
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
    pub struct SeqMapTaskOutputInfo {
        #[doc = "The sink to write the output value to."]
        #[serde(rename = "sink", default)]
        pub sink: Option<crate::schemas::Sink>,
        #[doc = "The id of the TupleTag the user code will tag the output value by."]
        #[serde(rename = "tag", default)]
        pub tag: Option<String>,
    }
    impl ::field_selector::FieldSelector for SeqMapTaskOutputInfo {
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
    pub struct ShellTask {
        #[doc = "The shell command to run."]
        #[serde(rename = "command", default)]
        pub command: Option<String>,
        #[doc = "Exit code for the task."]
        #[serde(rename = "exitCode", default)]
        pub exit_code: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ShellTask {
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
    pub struct SideInputInfo {
        #[doc = "How to interpret the source element(s) as a side input value."]
        #[serde(rename = "kind", default)]
        pub kind: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The source(s) to read element(s) from to get the value of this side input.\nIf more than one source, then the elements are taken from the\nsources, in the specified order if order matters.\nAt least one source is required."]
        #[serde(rename = "sources", default)]
        pub sources: Option<Vec<crate::schemas::Source>>,
        #[doc = "The id of the tag the user code will access this side input by;\nthis should correspond to the tag of some MultiOutputInfo."]
        #[serde(rename = "tag", default)]
        pub tag: Option<String>,
    }
    impl ::field_selector::FieldSelector for SideInputInfo {
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
    pub struct Sink {
        #[doc = "The codec to use to encode data written to the sink."]
        #[serde(rename = "codec", default)]
        pub codec: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The sink to write to, plus its parameters."]
        #[serde(rename = "spec", default)]
        pub spec: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Sink {
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
    pub enum SnapshotState {
        #[doc = "Unknown state."]
        UnknownSnapshotState,
        #[doc = "Snapshot intent to create has been persisted, snapshotting of state has not\nyet started."]
        Pending,
        #[doc = "Snapshotting is being performed."]
        Running,
        #[doc = "Snapshot has been created and is ready to be used."]
        Ready,
        #[doc = "Snapshot failed to be created."]
        Failed,
        #[doc = "Snapshot has been deleted."]
        Deleted,
    }
    impl SnapshotState {
        pub fn as_str(self) -> &'static str {
            match self {
                SnapshotState::UnknownSnapshotState => "UNKNOWN_SNAPSHOT_STATE",
                SnapshotState::Pending => "PENDING",
                SnapshotState::Running => "RUNNING",
                SnapshotState::Ready => "READY",
                SnapshotState::Failed => "FAILED",
                SnapshotState::Deleted => "DELETED",
            }
        }
    }
    impl ::std::fmt::Display for SnapshotState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SnapshotState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SnapshotState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_SNAPSHOT_STATE" => SnapshotState::UnknownSnapshotState,
                "PENDING" => SnapshotState::Pending,
                "RUNNING" => SnapshotState::Running,
                "READY" => SnapshotState::Ready,
                "FAILED" => SnapshotState::Failed,
                "DELETED" => SnapshotState::Deleted,
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
    pub struct Snapshot {
        #[doc = "The time this snapshot was created."]
        #[serde(rename = "creationTime", default)]
        pub creation_time: Option<String>,
        #[doc = "The unique ID of this snapshot."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The project this snapshot belongs to."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "The job this snapshot was created from."]
        #[serde(rename = "sourceJobId", default)]
        pub source_job_id: Option<String>,
        #[doc = "State of the snapshot."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::SnapshotState>,
        #[doc = "The time after which this snapshot will be automatically deleted."]
        #[serde(rename = "ttl", default)]
        pub ttl: Option<String>,
    }
    impl ::field_selector::FieldSelector for Snapshot {
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
    pub struct SnapshotJobRequest {
        #[doc = "The location that contains this job."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "TTL for the snapshot."]
        #[serde(rename = "ttl", default)]
        pub ttl: Option<String>,
    }
    impl ::field_selector::FieldSelector for SnapshotJobRequest {
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
    pub struct Source {
        #[doc = "While splitting, sources may specify the produced bundles\nas differences against another source, in order to save backend-side\nmemory and allow bigger jobs. For details, see SourceSplitRequest.\nTo support this use case, the full set of parameters of the source\nis logically obtained by taking the latest explicitly specified value\nof each parameter in the order:\nbase_specs (later items win), spec (overrides anything in base_specs)."]
        #[serde(rename = "baseSpecs", default)]
        pub base_specs: Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "The codec to use to decode data read from the source."]
        #[serde(rename = "codec", default)]
        pub codec: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Setting this value to true hints to the framework that the source\ndoesn't need splitting, and using SourceSplitRequest on it would\nyield SOURCE_SPLIT_OUTCOME_USE_CURRENT.\n\nE.g. a file splitter may set this to true when splitting a single file\ninto a set of byte ranges of appropriate size, and set this\nto false when splitting a filepattern into individual files.\nHowever, for efficiency, a file splitter may decide to produce\nfile subranges directly from the filepattern to avoid a splitting\nround-trip.\n\nSee SourceSplitRequest for an overview of the splitting process.\n\nThis field is meaningful only in the Source objects populated\nby the user (e.g. when filling in a DerivedSource).\nSource objects supplied by the framework to the user don't have\nthis field populated."]
        #[serde(rename = "doesNotNeedSplitting", default)]
        pub does_not_need_splitting: Option<bool>,
        #[doc = "Optionally, metadata for this source can be supplied right away,\navoiding a SourceGetMetadataOperation roundtrip\n(see SourceOperationRequest).\n\nThis field is meaningful only in the Source objects populated\nby the user (e.g. when filling in a DerivedSource).\nSource objects supplied by the framework to the user don't have\nthis field populated."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::SourceMetadata>,
        #[doc = "The source to read from, plus its parameters."]
        #[serde(rename = "spec", default)]
        pub spec: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Source {
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
    pub struct SourceFork {
        #[doc = "DEPRECATED"]
        #[serde(rename = "primary", default)]
        pub primary: Option<crate::schemas::SourceSplitShard>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "primarySource", default)]
        pub primary_source: Option<crate::schemas::DerivedSource>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "residual", default)]
        pub residual: Option<crate::schemas::SourceSplitShard>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "residualSource", default)]
        pub residual_source: Option<crate::schemas::DerivedSource>,
    }
    impl ::field_selector::FieldSelector for SourceFork {
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
    pub struct SourceGetMetadataRequest {
        #[doc = "Specification of the source whose metadata should be computed."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceGetMetadataRequest {
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
    pub struct SourceGetMetadataResponse {
        #[doc = "The computed metadata."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::SourceMetadata>,
    }
    impl ::field_selector::FieldSelector for SourceGetMetadataResponse {
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
    pub struct SourceMetadata {
        #[doc = "An estimate of the total size (in bytes) of the data that would be\nread from this source.  This estimate is in terms of external storage\nsize, before any decompression or other processing done by the reader."]
        #[serde(rename = "estimatedSizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_size_bytes: Option<i64>,
        #[doc = "Specifies that the size of this source is known to be infinite\n(this is a streaming source)."]
        #[serde(rename = "infinite", default)]
        pub infinite: Option<bool>,
        #[doc = "Whether this source is known to produce key/value pairs with\nthe (encoded) keys in lexicographically sorted order."]
        #[serde(rename = "producesSortedKeys", default)]
        pub produces_sorted_keys: Option<bool>,
    }
    impl ::field_selector::FieldSelector for SourceMetadata {
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
    pub struct SourceOperationRequest {
        #[doc = "Information about a request to get metadata about a source."]
        #[serde(rename = "getMetadata", default)]
        pub get_metadata: Option<crate::schemas::SourceGetMetadataRequest>,
        #[doc = "User-provided name of the Read instruction for this source."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "System-defined name for the Read instruction for this source\nin the original workflow graph."]
        #[serde(rename = "originalName", default)]
        pub original_name: Option<String>,
        #[doc = "Information about a request to split a source."]
        #[serde(rename = "split", default)]
        pub split: Option<crate::schemas::SourceSplitRequest>,
        #[doc = "System-defined name of the stage containing the source operation.\nUnique across the workflow."]
        #[serde(rename = "stageName", default)]
        pub stage_name: Option<String>,
        #[doc = "System-defined name of the Read instruction for this source.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for SourceOperationRequest {
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
    pub struct SourceOperationResponse {
        #[doc = "A response to a request to get metadata about a source."]
        #[serde(rename = "getMetadata", default)]
        pub get_metadata: Option<crate::schemas::SourceGetMetadataResponse>,
        #[doc = "A response to a request to split a source."]
        #[serde(rename = "split", default)]
        pub split: Option<crate::schemas::SourceSplitResponse>,
    }
    impl ::field_selector::FieldSelector for SourceOperationResponse {
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
    pub struct SourceSplitOptions {
        #[doc = "The source should be split into a set of bundles where the estimated size\nof each is approximately this many bytes."]
        #[serde(rename = "desiredBundleSizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub desired_bundle_size_bytes: Option<i64>,
        #[doc = "DEPRECATED in favor of desired_bundle_size_bytes."]
        #[serde(rename = "desiredShardSizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub desired_shard_size_bytes: Option<i64>,
    }
    impl ::field_selector::FieldSelector for SourceSplitOptions {
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
    pub struct SourceSplitRequest {
        #[doc = "Hints for tuning the splitting process."]
        #[serde(rename = "options", default)]
        pub options: Option<crate::schemas::SourceSplitOptions>,
        #[doc = "Specification of the source to be split."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceSplitRequest {
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
    pub enum SourceSplitResponseOutcome {
        #[doc = "The source split outcome is unknown, or unspecified."]
        SourceSplitOutcomeUnknown,
        #[doc = "The current source should be processed \"as is\" without splitting."]
        SourceSplitOutcomeUseCurrent,
        #[doc = "Splitting produced a list of bundles."]
        SourceSplitOutcomeSplittingHappened,
    }
    impl SourceSplitResponseOutcome {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceSplitResponseOutcome::SourceSplitOutcomeUnknown => {
                    "SOURCE_SPLIT_OUTCOME_UNKNOWN"
                }
                SourceSplitResponseOutcome::SourceSplitOutcomeUseCurrent => {
                    "SOURCE_SPLIT_OUTCOME_USE_CURRENT"
                }
                SourceSplitResponseOutcome::SourceSplitOutcomeSplittingHappened => {
                    "SOURCE_SPLIT_OUTCOME_SPLITTING_HAPPENED"
                }
            }
        }
    }
    impl ::std::fmt::Display for SourceSplitResponseOutcome {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceSplitResponseOutcome {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceSplitResponseOutcome {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_SPLIT_OUTCOME_UNKNOWN" => {
                    SourceSplitResponseOutcome::SourceSplitOutcomeUnknown
                }
                "SOURCE_SPLIT_OUTCOME_USE_CURRENT" => {
                    SourceSplitResponseOutcome::SourceSplitOutcomeUseCurrent
                }
                "SOURCE_SPLIT_OUTCOME_SPLITTING_HAPPENED" => {
                    SourceSplitResponseOutcome::SourceSplitOutcomeSplittingHappened
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceSplitResponse {
        #[doc = "If outcome is SPLITTING_HAPPENED, then this is a list of bundles\ninto which the source was split. Otherwise this field is ignored.\nThis list can be empty, which means the source represents an empty input."]
        #[serde(rename = "bundles", default)]
        pub bundles: Option<Vec<crate::schemas::DerivedSource>>,
        #[doc = "Indicates whether splitting happened and produced a list of bundles.\nIf this is USE_CURRENT_SOURCE_AS_IS, the current source should\nbe processed \"as is\" without splitting. \"bundles\" is ignored in this case.\nIf this is SPLITTING_HAPPENED, then \"bundles\" contains a list of\nbundles into which the source was split."]
        #[serde(rename = "outcome", default)]
        pub outcome: Option<crate::schemas::SourceSplitResponseOutcome>,
        #[doc = "DEPRECATED in favor of bundles."]
        #[serde(rename = "shards", default)]
        pub shards: Option<Vec<crate::schemas::SourceSplitShard>>,
    }
    impl ::field_selector::FieldSelector for SourceSplitResponse {
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
    pub enum SourceSplitShardDerivationMode {
        #[doc = "The source derivation is unknown, or unspecified."]
        SourceDerivationModeUnknown,
        #[doc = "Produce a completely independent Source with no base."]
        SourceDerivationModeIndependent,
        #[doc = "Produce a Source based on the Source being split."]
        SourceDerivationModeChildOfCurrent,
        #[doc = "Produce a Source based on the base of the Source being split."]
        SourceDerivationModeSiblingOfCurrent,
    }
    impl SourceSplitShardDerivationMode {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceSplitShardDerivationMode::SourceDerivationModeUnknown => {
                    "SOURCE_DERIVATION_MODE_UNKNOWN"
                }
                SourceSplitShardDerivationMode::SourceDerivationModeIndependent => {
                    "SOURCE_DERIVATION_MODE_INDEPENDENT"
                }
                SourceSplitShardDerivationMode::SourceDerivationModeChildOfCurrent => {
                    "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT"
                }
                SourceSplitShardDerivationMode::SourceDerivationModeSiblingOfCurrent => {
                    "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT"
                }
            }
        }
    }
    impl ::std::fmt::Display for SourceSplitShardDerivationMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceSplitShardDerivationMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceSplitShardDerivationMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_DERIVATION_MODE_UNKNOWN" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeUnknown
                }
                "SOURCE_DERIVATION_MODE_INDEPENDENT" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeIndependent
                }
                "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeChildOfCurrent
                }
                "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeSiblingOfCurrent
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceSplitShard {
        #[doc = "DEPRECATED"]
        #[serde(rename = "derivationMode", default)]
        pub derivation_mode: Option<crate::schemas::SourceSplitShardDerivationMode>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceSplitShard {
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
    pub struct SpannerIODetails {
        #[doc = "DatabaseId accessed in the connection."]
        #[serde(rename = "databaseId", default)]
        pub database_id: Option<String>,
        #[doc = "InstanceId accessed in the connection."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: Option<String>,
        #[doc = "ProjectId accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for SpannerIODetails {
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
    pub struct SplitInt64 {
        #[doc = "The high order bits, including the sign: n >> 32."]
        #[serde(rename = "highBits", default)]
        pub high_bits: Option<i32>,
        #[doc = "The low order bits: n & 0xffffffff."]
        #[serde(rename = "lowBits", default)]
        pub low_bits: Option<u32>,
    }
    impl ::field_selector::FieldSelector for SplitInt64 {
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
    pub struct StageSource {
        #[doc = "Dataflow service generated name for this source."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User name for the original user transform or collection with which this\nsource is most closely associated."]
        #[serde(rename = "originalTransformOrCollection", default)]
        pub original_transform_or_collection: Option<String>,
        #[doc = "Size of the source, if measurable."]
        #[serde(rename = "sizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: Option<i64>,
        #[doc = "Human-readable name for this source; may be user or system generated."]
        #[serde(rename = "userName", default)]
        pub user_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for StageSource {
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
    pub struct StateFamilyConfig {
        #[doc = "If true, this family corresponds to a read operation."]
        #[serde(rename = "isRead", default)]
        pub is_read: Option<bool>,
        #[doc = "The state family value."]
        #[serde(rename = "stateFamily", default)]
        pub state_family: Option<String>,
    }
    impl ::field_selector::FieldSelector for StateFamilyConfig {
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
    pub struct Step {
        #[doc = "The kind of step in the Cloud Dataflow job."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The name that identifies the step. This must be unique for each\nstep with respect to all other steps in the Cloud Dataflow job."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Named properties associated with the step. Each kind of\npredefined step has its own required set of properties.\nMust be provided on Create.  Only retrieved with JOB_VIEW_ALL."]
        #[serde(rename = "properties", default)]
        pub properties: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Step {
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
    pub struct StreamLocation {
        #[doc = "The stream is a custom source."]
        #[serde(rename = "customSourceLocation", default)]
        pub custom_source_location: Option<crate::schemas::CustomSourceLocation>,
        #[doc = "The stream is a pubsub stream."]
        #[serde(rename = "pubsubLocation", default)]
        pub pubsub_location: Option<crate::schemas::PubsubLocation>,
        #[doc = "The stream is a streaming side input."]
        #[serde(rename = "sideInputLocation", default)]
        pub side_input_location: Option<crate::schemas::StreamingSideInputLocation>,
        #[doc = "The stream is part of another computation within the current\nstreaming Dataflow job."]
        #[serde(rename = "streamingStageLocation", default)]
        pub streaming_stage_location: Option<crate::schemas::StreamingStageLocation>,
    }
    impl ::field_selector::FieldSelector for StreamLocation {
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
    pub struct StreamingApplianceSnapshotConfig {
        #[doc = "Indicates which endpoint is used to import appliance state."]
        #[serde(rename = "importStateEndpoint", default)]
        pub import_state_endpoint: Option<String>,
        #[doc = "If set, indicates the snapshot id for the snapshot being performed."]
        #[serde(rename = "snapshotId", default)]
        pub snapshot_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for StreamingApplianceSnapshotConfig {
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
    pub struct StreamingComputationConfig {
        #[doc = "Unique identifier for this computation."]
        #[serde(rename = "computationId", default)]
        pub computation_id: Option<String>,
        #[doc = "Instructions that comprise the computation."]
        #[serde(rename = "instructions", default)]
        pub instructions: Option<Vec<crate::schemas::ParallelInstruction>>,
        #[doc = "Stage name of this computation."]
        #[serde(rename = "stageName", default)]
        pub stage_name: Option<String>,
        #[doc = "System defined name for this computation."]
        #[serde(rename = "systemName", default)]
        pub system_name: Option<String>,
        #[doc = "Map from user name of stateful transforms in this stage to their state\nfamily."]
        #[serde(rename = "transformUserNameToStateFamily", default)]
        pub transform_user_name_to_state_family:
            Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for StreamingComputationConfig {
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
    pub struct StreamingComputationRanges {
        #[doc = "The ID of the computation."]
        #[serde(rename = "computationId", default)]
        pub computation_id: Option<String>,
        #[doc = "Data disk assignments for ranges from this computation."]
        #[serde(rename = "rangeAssignments", default)]
        pub range_assignments: Option<Vec<crate::schemas::KeyRangeDataDiskAssignment>>,
    }
    impl ::field_selector::FieldSelector for StreamingComputationRanges {
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
    pub enum StreamingComputationTaskTaskType {
        #[doc = "The streaming computation task is unknown, or unspecified."]
        StreamingComputationTaskUnknown,
        #[doc = "Stop processing specified streaming computation range(s)."]
        StreamingComputationTaskStop,
        #[doc = "Start processing specified streaming computation range(s)."]
        StreamingComputationTaskStart,
    }
    impl StreamingComputationTaskTaskType {
        pub fn as_str(self) -> &'static str {
            match self {
                StreamingComputationTaskTaskType::StreamingComputationTaskUnknown => {
                    "STREAMING_COMPUTATION_TASK_UNKNOWN"
                }
                StreamingComputationTaskTaskType::StreamingComputationTaskStop => {
                    "STREAMING_COMPUTATION_TASK_STOP"
                }
                StreamingComputationTaskTaskType::StreamingComputationTaskStart => {
                    "STREAMING_COMPUTATION_TASK_START"
                }
            }
        }
    }
    impl ::std::fmt::Display for StreamingComputationTaskTaskType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StreamingComputationTaskTaskType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StreamingComputationTaskTaskType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STREAMING_COMPUTATION_TASK_UNKNOWN" => {
                    StreamingComputationTaskTaskType::StreamingComputationTaskUnknown
                }
                "STREAMING_COMPUTATION_TASK_STOP" => {
                    StreamingComputationTaskTaskType::StreamingComputationTaskStop
                }
                "STREAMING_COMPUTATION_TASK_START" => {
                    StreamingComputationTaskTaskType::StreamingComputationTaskStart
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
    pub struct StreamingComputationTask {
        #[doc = "Contains ranges of a streaming computation this task should apply to."]
        #[serde(rename = "computationRanges", default)]
        pub computation_ranges: Option<Vec<crate::schemas::StreamingComputationRanges>>,
        #[doc = "Describes the set of data disks this task should apply to."]
        #[serde(rename = "dataDisks", default)]
        pub data_disks: Option<Vec<crate::schemas::MountedDataDisk>>,
        #[doc = "A type of streaming computation task."]
        #[serde(rename = "taskType", default)]
        pub task_type: Option<crate::schemas::StreamingComputationTaskTaskType>,
    }
    impl ::field_selector::FieldSelector for StreamingComputationTask {
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
    pub struct StreamingConfigTask {
        #[doc = "Maximum size for work item commit supported windmill storage layer."]
        #[serde(rename = "maxWorkItemCommitBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub max_work_item_commit_bytes: Option<i64>,
        #[doc = "Set of computation configuration information."]
        #[serde(rename = "streamingComputationConfigs", default)]
        pub streaming_computation_configs: Option<Vec<crate::schemas::StreamingComputationConfig>>,
        #[doc = "Map from user step names to state families."]
        #[serde(rename = "userStepToStateFamilyNameMap", default)]
        pub user_step_to_state_family_name_map:
            Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "If present, the worker must use this endpoint to communicate with Windmill\nService dispatchers, otherwise the worker must continue to use whatever\nendpoint it had been using."]
        #[serde(rename = "windmillServiceEndpoint", default)]
        pub windmill_service_endpoint: Option<String>,
        #[doc = "If present, the worker must use this port to communicate with Windmill\nService dispatchers. Only applicable when windmill_service_endpoint is\nspecified."]
        #[serde(rename = "windmillServicePort", default)]
        #[serde(with = "crate::parsed_string")]
        pub windmill_service_port: Option<i64>,
    }
    impl ::field_selector::FieldSelector for StreamingConfigTask {
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
    pub struct StreamingSetupTask {
        #[doc = "The user has requested drain."]
        #[serde(rename = "drain", default)]
        pub drain: Option<bool>,
        #[doc = "The TCP port on which the worker should listen for messages from\nother streaming computation workers."]
        #[serde(rename = "receiveWorkPort", default)]
        pub receive_work_port: Option<i32>,
        #[doc = "Configures streaming appliance snapshot."]
        #[serde(rename = "snapshotConfig", default)]
        pub snapshot_config: Option<crate::schemas::StreamingApplianceSnapshotConfig>,
        #[doc = "The global topology of the streaming Dataflow job."]
        #[serde(rename = "streamingComputationTopology", default)]
        pub streaming_computation_topology: Option<crate::schemas::TopologyConfig>,
        #[doc = "The TCP port used by the worker to communicate with the Dataflow\nworker harness."]
        #[serde(rename = "workerHarnessPort", default)]
        pub worker_harness_port: Option<i32>,
    }
    impl ::field_selector::FieldSelector for StreamingSetupTask {
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
    pub struct StreamingSideInputLocation {
        #[doc = "Identifies the state family where this side input is stored."]
        #[serde(rename = "stateFamily", default)]
        pub state_family: Option<String>,
        #[doc = "Identifies the particular side input within the streaming Dataflow job."]
        #[serde(rename = "tag", default)]
        pub tag: Option<String>,
    }
    impl ::field_selector::FieldSelector for StreamingSideInputLocation {
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
    pub struct StreamingStageLocation {
        #[doc = "Identifies the particular stream within the streaming Dataflow\njob."]
        #[serde(rename = "streamId", default)]
        pub stream_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for StreamingStageLocation {
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
    pub struct StringList {
        #[doc = "Elements of the list."]
        #[serde(rename = "elements", default)]
        pub elements: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for StringList {
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
    pub struct StructuredMessage {
        #[doc = "Identifier for this message type.  Used by external systems to\ninternationalize or personalize message."]
        #[serde(rename = "messageKey", default)]
        pub message_key: Option<String>,
        #[doc = "Human-readable version of message."]
        #[serde(rename = "messageText", default)]
        pub message_text: Option<String>,
        #[doc = "The structured data associated with this message."]
        #[serde(rename = "parameters", default)]
        pub parameters: Option<Vec<crate::schemas::Parameter>>,
    }
    impl ::field_selector::FieldSelector for StructuredMessage {
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
    pub struct TaskRunnerSettings {
        #[doc = "Whether to also send taskrunner log info to stderr."]
        #[serde(rename = "alsologtostderr", default)]
        pub alsologtostderr: Option<bool>,
        #[doc = "The location on the worker for task-specific subdirectories."]
        #[serde(rename = "baseTaskDir", default)]
        pub base_task_dir: Option<String>,
        #[doc = "The base URL for the taskrunner to use when accessing Google Cloud APIs.\n\nWhen workers access Google Cloud APIs, they logically do so via\nrelative URLs.  If this field is specified, it supplies the base\nURL to use for resolving these relative URLs.  The normative\nalgorithm used is defined by RFC 1808, \"Relative Uniform Resource\nLocators\".\n\nIf not specified, the default value is \"http://www.googleapis.com/\""]
        #[serde(rename = "baseUrl", default)]
        pub base_url: Option<String>,
        #[doc = "The file to store preprocessing commands in."]
        #[serde(rename = "commandlinesFileName", default)]
        pub commandlines_file_name: Option<String>,
        #[doc = "Whether to continue taskrunner if an exception is hit."]
        #[serde(rename = "continueOnException", default)]
        pub continue_on_exception: Option<bool>,
        #[doc = "The API version of endpoint, e.g. \"v1b3\""]
        #[serde(rename = "dataflowApiVersion", default)]
        pub dataflow_api_version: Option<String>,
        #[doc = "The command to launch the worker harness."]
        #[serde(rename = "harnessCommand", default)]
        pub harness_command: Option<String>,
        #[doc = "The suggested backend language."]
        #[serde(rename = "languageHint", default)]
        pub language_hint: Option<String>,
        #[doc = "The directory on the VM to store logs."]
        #[serde(rename = "logDir", default)]
        pub log_dir: Option<String>,
        #[doc = "Whether to send taskrunner log info to Google Compute Engine VM serial\nconsole."]
        #[serde(rename = "logToSerialconsole", default)]
        pub log_to_serialconsole: Option<bool>,
        #[doc = "Indicates where to put logs.  If this is not specified, the logs\nwill not be uploaded.\n\nThe supported resource type is:\n\nGoogle Cloud Storage:\n  storage.googleapis.com/{bucket}/{object}\n  bucket.storage.googleapis.com/{object}"]
        #[serde(rename = "logUploadLocation", default)]
        pub log_upload_location: Option<String>,
        #[doc = "The OAuth2 scopes to be requested by the taskrunner in order to\naccess the Cloud Dataflow API."]
        #[serde(rename = "oauthScopes", default)]
        pub oauth_scopes: Option<Vec<String>>,
        #[doc = "The settings to pass to the parallel worker harness."]
        #[serde(rename = "parallelWorkerSettings", default)]
        pub parallel_worker_settings: Option<crate::schemas::WorkerSettings>,
        #[doc = "The streaming worker main class name."]
        #[serde(rename = "streamingWorkerMainClass", default)]
        pub streaming_worker_main_class: Option<String>,
        #[doc = "The UNIX group ID on the worker VM to use for tasks launched by\ntaskrunner; e.g. \"wheel\"."]
        #[serde(rename = "taskGroup", default)]
        pub task_group: Option<String>,
        #[doc = "The UNIX user ID on the worker VM to use for tasks launched by\ntaskrunner; e.g. \"root\"."]
        #[serde(rename = "taskUser", default)]
        pub task_user: Option<String>,
        #[doc = "The prefix of the resources the taskrunner should use for\ntemporary storage.\n\nThe supported resource type is:\n\nGoogle Cloud Storage:\n  storage.googleapis.com/{bucket}/{object}\n  bucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempStoragePrefix", default)]
        pub temp_storage_prefix: Option<String>,
        #[doc = "The ID string of the VM."]
        #[serde(rename = "vmId", default)]
        pub vm_id: Option<String>,
        #[doc = "The file to store the workflow in."]
        #[serde(rename = "workflowFileName", default)]
        pub workflow_file_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for TaskRunnerSettings {
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
    pub struct TemplateMetadata {
        #[doc = "Optional. A description of the template."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Required. The name of the template."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The parameters for the template."]
        #[serde(rename = "parameters", default)]
        pub parameters: Option<Vec<crate::schemas::ParameterMetadata>>,
    }
    impl ::field_selector::FieldSelector for TemplateMetadata {
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
    pub struct TopologyConfig {
        #[doc = "The computations associated with a streaming Dataflow job."]
        #[serde(rename = "computations", default)]
        pub computations: Option<Vec<crate::schemas::ComputationTopology>>,
        #[doc = "The disks assigned to a streaming Dataflow job."]
        #[serde(rename = "dataDiskAssignments", default)]
        pub data_disk_assignments: Option<Vec<crate::schemas::DataDiskAssignment>>,
        #[doc = "The size (in bits) of keys that will be assigned to source messages."]
        #[serde(rename = "forwardingKeyBits", default)]
        pub forwarding_key_bits: Option<i32>,
        #[doc = "Version number for persistent state."]
        #[serde(rename = "persistentStateVersion", default)]
        pub persistent_state_version: Option<i32>,
        #[doc = "Maps user stage names to stable computation names."]
        #[serde(rename = "userStageToComputationNameMap", default)]
        pub user_stage_to_computation_name_map:
            Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for TopologyConfig {
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
    pub enum TransformSummaryKind {
        #[doc = "Unrecognized transform type."]
        UnknownKind,
        #[doc = "ParDo transform."]
        ParDoKind,
        #[doc = "Group By Key transform."]
        GroupByKeyKind,
        #[doc = "Flatten transform."]
        FlattenKind,
        #[doc = "Read transform."]
        ReadKind,
        #[doc = "Write transform."]
        WriteKind,
        #[doc = "Constructs from a constant value, such as with Create.of."]
        ConstantKind,
        #[doc = "Creates a Singleton view of a collection."]
        SingletonKind,
        #[doc = "Opening or closing a shuffle session, often as part of a GroupByKey."]
        ShuffleKind,
    }
    impl TransformSummaryKind {
        pub fn as_str(self) -> &'static str {
            match self {
                TransformSummaryKind::UnknownKind => "UNKNOWN_KIND",
                TransformSummaryKind::ParDoKind => "PAR_DO_KIND",
                TransformSummaryKind::GroupByKeyKind => "GROUP_BY_KEY_KIND",
                TransformSummaryKind::FlattenKind => "FLATTEN_KIND",
                TransformSummaryKind::ReadKind => "READ_KIND",
                TransformSummaryKind::WriteKind => "WRITE_KIND",
                TransformSummaryKind::ConstantKind => "CONSTANT_KIND",
                TransformSummaryKind::SingletonKind => "SINGLETON_KIND",
                TransformSummaryKind::ShuffleKind => "SHUFFLE_KIND",
            }
        }
    }
    impl ::std::fmt::Display for TransformSummaryKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TransformSummaryKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransformSummaryKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_KIND" => TransformSummaryKind::UnknownKind,
                "PAR_DO_KIND" => TransformSummaryKind::ParDoKind,
                "GROUP_BY_KEY_KIND" => TransformSummaryKind::GroupByKeyKind,
                "FLATTEN_KIND" => TransformSummaryKind::FlattenKind,
                "READ_KIND" => TransformSummaryKind::ReadKind,
                "WRITE_KIND" => TransformSummaryKind::WriteKind,
                "CONSTANT_KIND" => TransformSummaryKind::ConstantKind,
                "SINGLETON_KIND" => TransformSummaryKind::SingletonKind,
                "SHUFFLE_KIND" => TransformSummaryKind::ShuffleKind,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TransformSummary {
        #[doc = "Transform-specific display data."]
        #[serde(rename = "displayData", default)]
        pub display_data: Option<Vec<crate::schemas::DisplayData>>,
        #[doc = "SDK generated id of this transform instance."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "User names for all collection inputs to this transform."]
        #[serde(rename = "inputCollectionName", default)]
        pub input_collection_name: Option<Vec<String>>,
        #[doc = "Type of transform."]
        #[serde(rename = "kind", default)]
        pub kind: Option<crate::schemas::TransformSummaryKind>,
        #[doc = "User provided name for this transform instance."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User  names for all collection outputs to this transform."]
        #[serde(rename = "outputCollectionName", default)]
        pub output_collection_name: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TransformSummary {
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
    pub struct ValidateResponse {
        #[doc = "Will be empty if validation succeeds."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
    }
    impl ::field_selector::FieldSelector for ValidateResponse {
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
    pub struct WorkItem {
        #[doc = "Work item-specific configuration as an opaque blob."]
        #[serde(rename = "configuration", default)]
        pub configuration: Option<String>,
        #[doc = "Identifies this WorkItem."]
        #[serde(rename = "id", default)]
        #[serde(with = "crate::parsed_string")]
        pub id: Option<i64>,
        #[doc = "The initial index to use when reporting the status of the WorkItem."]
        #[serde(rename = "initialReportIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub initial_report_index: Option<i64>,
        #[doc = "Identifies the workflow job this WorkItem belongs to."]
        #[serde(rename = "jobId", default)]
        pub job_id: Option<String>,
        #[doc = "Time when the lease on this Work will expire."]
        #[serde(rename = "leaseExpireTime", default)]
        pub lease_expire_time: Option<String>,
        #[doc = "Additional information for MapTask WorkItems."]
        #[serde(rename = "mapTask", default)]
        pub map_task: Option<crate::schemas::MapTask>,
        #[doc = "Any required packages that need to be fetched in order to execute\nthis WorkItem."]
        #[serde(rename = "packages", default)]
        pub packages: Option<Vec<crate::schemas::Package>>,
        #[doc = "Identifies the cloud project this WorkItem belongs to."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Recommended reporting interval."]
        #[serde(rename = "reportStatusInterval", default)]
        pub report_status_interval: Option<String>,
        #[doc = "Additional information for SeqMapTask WorkItems."]
        #[serde(rename = "seqMapTask", default)]
        pub seq_map_task: Option<crate::schemas::SeqMapTask>,
        #[doc = "Additional information for ShellTask WorkItems."]
        #[serde(rename = "shellTask", default)]
        pub shell_task: Option<crate::schemas::ShellTask>,
        #[doc = "Additional information for source operation WorkItems."]
        #[serde(rename = "sourceOperationTask", default)]
        pub source_operation_task: Option<crate::schemas::SourceOperationRequest>,
        #[doc = "Additional information for StreamingComputationTask WorkItems."]
        #[serde(rename = "streamingComputationTask", default)]
        pub streaming_computation_task: Option<crate::schemas::StreamingComputationTask>,
        #[doc = "Additional information for StreamingConfigTask WorkItems."]
        #[serde(rename = "streamingConfigTask", default)]
        pub streaming_config_task: Option<crate::schemas::StreamingConfigTask>,
        #[doc = "Additional information for StreamingSetupTask WorkItems."]
        #[serde(rename = "streamingSetupTask", default)]
        pub streaming_setup_task: Option<crate::schemas::StreamingSetupTask>,
    }
    impl ::field_selector::FieldSelector for WorkItem {
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
    pub struct WorkItemServiceState {
        #[doc = "Other data returned by the service, specific to the particular\nworker harness."]
        #[serde(rename = "harnessData", default)]
        pub harness_data: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A hot key is a symptom of poor data distribution in which there are enough\nelements mapped to a single key to impact pipeline performance. When\npresent, this field includes metadata associated with any hot key."]
        #[serde(rename = "hotKeyDetection", default)]
        pub hot_key_detection: Option<crate::schemas::HotKeyDetection>,
        #[doc = "Time at which the current lease will expire."]
        #[serde(rename = "leaseExpireTime", default)]
        pub lease_expire_time: Option<String>,
        #[doc = "The short ids that workers should use in subsequent metric updates.\nWorkers should strive to use short ids whenever possible, but it is ok\nto request the short_id again if a worker lost track of it\n(e.g. if the worker is recovering from a crash).\nNOTE: it is possible that the response may have short ids for a subset\nof the metrics."]
        #[serde(rename = "metricShortId", default)]
        pub metric_short_id: Option<Vec<crate::schemas::MetricShortId>>,
        #[doc = "The index value to use for the next report sent by the worker.\nNote: If the report call fails for whatever reason, the worker should\nreuse this index for subsequent report attempts."]
        #[serde(rename = "nextReportIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub next_report_index: Option<i64>,
        #[doc = "New recommended reporting interval."]
        #[serde(rename = "reportStatusInterval", default)]
        pub report_status_interval: Option<String>,
        #[doc = "The progress point in the WorkItem where the Dataflow service\nsuggests that the worker truncate the task."]
        #[serde(rename = "splitRequest", default)]
        pub split_request: Option<crate::schemas::ApproximateSplitRequest>,
        #[doc = "DEPRECATED in favor of split_request."]
        #[serde(rename = "suggestedStopPoint", default)]
        pub suggested_stop_point: Option<crate::schemas::ApproximateProgress>,
        #[doc = "Obsolete, always empty."]
        #[serde(rename = "suggestedStopPosition", default)]
        pub suggested_stop_position: Option<crate::schemas::Position>,
    }
    impl ::field_selector::FieldSelector for WorkItemServiceState {
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
    pub struct WorkItemStatus {
        #[doc = "True if the WorkItem was completed (successfully or unsuccessfully)."]
        #[serde(rename = "completed", default)]
        pub completed: Option<bool>,
        #[doc = "Worker output counters for this WorkItem."]
        #[serde(rename = "counterUpdates", default)]
        pub counter_updates: Option<Vec<crate::schemas::CounterUpdate>>,
        #[doc = "See documentation of stop_position."]
        #[serde(rename = "dynamicSourceSplit", default)]
        pub dynamic_source_split: Option<crate::schemas::DynamicSourceSplit>,
        #[doc = "Specifies errors which occurred during processing.  If errors are\nprovided, and completed = true, then the WorkItem is considered\nto have failed."]
        #[serde(rename = "errors", default)]
        pub errors: Option<Vec<crate::schemas::Status>>,
        #[doc = "DEPRECATED in favor of counter_updates."]
        #[serde(rename = "metricUpdates", default)]
        pub metric_updates: Option<Vec<crate::schemas::MetricUpdate>>,
        #[doc = "DEPRECATED in favor of reported_progress."]
        #[serde(rename = "progress", default)]
        pub progress: Option<crate::schemas::ApproximateProgress>,
        #[doc = "The report index.  When a WorkItem is leased, the lease will\ncontain an initial report index.  When a WorkItem's status is\nreported to the system, the report should be sent with\nthat report index, and the response will contain the index the\nworker should use for the next report.  Reports received with\nunexpected index values will be rejected by the service.\n\nIn order to preserve idempotency, the worker should not alter the\ncontents of a report, even if the worker must submit the same\nreport multiple times before getting back a response.  The worker\nshould not submit a subsequent report until the response for the\nprevious report had been received from the service."]
        #[serde(rename = "reportIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_index: Option<i64>,
        #[doc = "The worker's progress through this WorkItem."]
        #[serde(rename = "reportedProgress", default)]
        pub reported_progress: Option<crate::schemas::ApproximateReportedProgress>,
        #[doc = "Amount of time the worker requests for its lease."]
        #[serde(rename = "requestedLeaseDuration", default)]
        pub requested_lease_duration: Option<String>,
        #[doc = "DEPRECATED in favor of dynamic_source_split."]
        #[serde(rename = "sourceFork", default)]
        pub source_fork: Option<crate::schemas::SourceFork>,
        #[doc = "If the work item represented a SourceOperationRequest, and the work\nis completed, contains the result of the operation."]
        #[serde(rename = "sourceOperationResponse", default)]
        pub source_operation_response: Option<crate::schemas::SourceOperationResponse>,
        #[doc = "A worker may split an active map task in two parts, \"primary\" and\n\"residual\", continuing to process the primary part and returning the\nresidual part into the pool of available work.\nThis event is called a \"dynamic split\" and is critical to the dynamic\nwork rebalancing feature. The two obtained sub-tasks are called\n\"parts\" of the split.\nThe parts, if concatenated, must represent the same input as would\nbe read by the current task if the split did not happen.\nThe exact way in which the original task is decomposed into the two\nparts is specified either as a position demarcating them\n(stop_position), or explicitly as two DerivedSources, if this\ntask consumes a user-defined source type (dynamic_source_split).\n\nThe \"current\" task is adjusted as a result of the split: after a task\nwith range [A, B) sends a stop_position update at C, its range is\nconsidered to be [A, C), e.g.:\n* Progress should be interpreted relative to the new range, e.g.\n  \"75% completed\" means \"75% of [A, C) completed\"\n* The worker should interpret proposed_stop_position relative to the\n  new range, e.g. \"split at 68%\" should be interpreted as\n  \"split at 68% of [A, C)\".\n* If the worker chooses to split again using stop_position, only\n  stop_positions in [A, C) will be accepted.\n* Etc.\ndynamic_source_split has similar semantics: e.g., if a task with\nsource S splits using dynamic_source_split into {P, R}\n(where P and R must be together equivalent to S), then subsequent\nprogress and proposed_stop_position should be interpreted relative\nto P, and in a potential subsequent dynamic_source_split into {P', R'},\nP' and R' must be together equivalent to P, etc."]
        #[serde(rename = "stopPosition", default)]
        pub stop_position: Option<crate::schemas::Position>,
        #[doc = "Total time the worker spent being throttled by external systems."]
        #[serde(rename = "totalThrottlerWaitTimeSeconds", default)]
        pub total_throttler_wait_time_seconds: Option<f64>,
        #[doc = "Identifies the WorkItem."]
        #[serde(rename = "workItemId", default)]
        pub work_item_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkItemStatus {
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
    pub struct WorkerHealthReport {
        #[doc = "A message describing any unusual health reports."]
        #[serde(rename = "msg", default)]
        pub msg: Option<String>,
        #[doc = "The pods running on the worker. See:\nhttp://kubernetes.io/v1.1/docs/api-reference/v1/definitions.html#_v1_pod\n\nThis field is used by the worker to send the status of the indvidual\ncontainers running on each worker."]
        #[serde(rename = "pods", default)]
        pub pods: Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "The interval at which the worker is sending health reports.\nThe default value of 0 should be interpreted as the field is not being\nexplicitly set by the worker."]
        #[serde(rename = "reportInterval", default)]
        pub report_interval: Option<String>,
        #[doc = "Whether the VM is in a permanently broken state.\nBroken VMs should be abandoned or deleted ASAP to avoid assigning or\ncompleting any work."]
        #[serde(rename = "vmIsBroken", default)]
        pub vm_is_broken: Option<bool>,
        #[doc = "Whether the VM is currently healthy."]
        #[serde(rename = "vmIsHealthy", default)]
        pub vm_is_healthy: Option<bool>,
        #[doc = "The time the VM was booted."]
        #[serde(rename = "vmStartupTime", default)]
        pub vm_startup_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerHealthReport {
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
    pub struct WorkerHealthReportResponse {
        #[doc = "A positive value indicates the worker should change its reporting interval\nto the specified value.\n\nThe default value of zero means no change in report rate is requested by\nthe server."]
        #[serde(rename = "reportInterval", default)]
        pub report_interval: Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerHealthReportResponse {
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
    pub enum WorkerLifecycleEventEvent {
        #[doc = "Invalid event."]
        UnknownEvent,
        #[doc = "The time the VM started."]
        OsStart,
        #[doc = "Our container code starts running. Multiple containers could be\ndistinguished with WorkerMessage.labels if desired."]
        ContainerStart,
        #[doc = "The worker has a functional external network connection."]
        NetworkUp,
        #[doc = "Started downloading staging files."]
        StagingFilesDownloadStart,
        #[doc = "Finished downloading all staging files."]
        StagingFilesDownloadFinish,
        #[doc = "For applicable SDKs, started installation of SDK and worker packages."]
        SdkInstallStart,
        #[doc = "Finished installing SDK."]
        SdkInstallFinish,
    }
    impl WorkerLifecycleEventEvent {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerLifecycleEventEvent::UnknownEvent => "UNKNOWN_EVENT",
                WorkerLifecycleEventEvent::OsStart => "OS_START",
                WorkerLifecycleEventEvent::ContainerStart => "CONTAINER_START",
                WorkerLifecycleEventEvent::NetworkUp => "NETWORK_UP",
                WorkerLifecycleEventEvent::StagingFilesDownloadStart => {
                    "STAGING_FILES_DOWNLOAD_START"
                }
                WorkerLifecycleEventEvent::StagingFilesDownloadFinish => {
                    "STAGING_FILES_DOWNLOAD_FINISH"
                }
                WorkerLifecycleEventEvent::SdkInstallStart => "SDK_INSTALL_START",
                WorkerLifecycleEventEvent::SdkInstallFinish => "SDK_INSTALL_FINISH",
            }
        }
    }
    impl ::std::fmt::Display for WorkerLifecycleEventEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerLifecycleEventEvent {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerLifecycleEventEvent {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_EVENT" => WorkerLifecycleEventEvent::UnknownEvent,
                "OS_START" => WorkerLifecycleEventEvent::OsStart,
                "CONTAINER_START" => WorkerLifecycleEventEvent::ContainerStart,
                "NETWORK_UP" => WorkerLifecycleEventEvent::NetworkUp,
                "STAGING_FILES_DOWNLOAD_START" => {
                    WorkerLifecycleEventEvent::StagingFilesDownloadStart
                }
                "STAGING_FILES_DOWNLOAD_FINISH" => {
                    WorkerLifecycleEventEvent::StagingFilesDownloadFinish
                }
                "SDK_INSTALL_START" => WorkerLifecycleEventEvent::SdkInstallStart,
                "SDK_INSTALL_FINISH" => WorkerLifecycleEventEvent::SdkInstallFinish,
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
    pub struct WorkerLifecycleEvent {
        #[doc = "The start time of this container. All events will report this so that\nevents can be grouped together across container/VM restarts."]
        #[serde(rename = "containerStartTime", default)]
        pub container_start_time: Option<String>,
        #[doc = "The event being reported."]
        #[serde(rename = "event", default)]
        pub event: Option<crate::schemas::WorkerLifecycleEventEvent>,
        #[doc = "Other stats that can accompany an event. E.g.\n{ \"downloaded_bytes\" : \"123456\" }"]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for WorkerLifecycleEvent {
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
    pub struct WorkerMessage {
        #[doc = "Labels are used to group WorkerMessages.\nFor example, a worker_message about a particular container\nmight have the labels:\n{ \"JOB_ID\": \"2015-04-22\",\n  \"WORKER_ID\": \"wordcount-vm-2015\u{2026}\"\n  \"CONTAINER_TYPE\": \"worker\",\n  \"CONTAINER_ID\": \"ac1234def\"}\nLabel tags typically correspond to Label enum values. However, for ease\nof development other strings can be used as tags. LABEL_UNSPECIFIED should\nnot be used here."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The timestamp of the worker_message."]
        #[serde(rename = "time", default)]
        pub time: Option<String>,
        #[doc = "The health of a worker."]
        #[serde(rename = "workerHealthReport", default)]
        pub worker_health_report: Option<crate::schemas::WorkerHealthReport>,
        #[doc = "Record of worker lifecycle events."]
        #[serde(rename = "workerLifecycleEvent", default)]
        pub worker_lifecycle_event: Option<crate::schemas::WorkerLifecycleEvent>,
        #[doc = "A worker message code."]
        #[serde(rename = "workerMessageCode", default)]
        pub worker_message_code: Option<crate::schemas::WorkerMessageCode>,
        #[doc = "Resource metrics reported by workers."]
        #[serde(rename = "workerMetrics", default)]
        pub worker_metrics: Option<crate::schemas::ResourceUtilizationReport>,
        #[doc = "Shutdown notice by workers."]
        #[serde(rename = "workerShutdownNotice", default)]
        pub worker_shutdown_notice: Option<crate::schemas::WorkerShutdownNotice>,
    }
    impl ::field_selector::FieldSelector for WorkerMessage {
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
    pub struct WorkerMessageCode {
        #[doc = "The code is a string intended for consumption by a machine that identifies\nthe type of message being sent.\nExamples:\n 1. \"HARNESS_STARTED\" might be used to indicate the worker harness has\n     started.\n 2. \"GCS_DOWNLOAD_ERROR\" might be used to indicate an error downloading\n    a GCS file as part of the boot process of one of the worker containers.\n\nThis is a string and not an enum to make it easy to add new codes without\nwaiting for an API change."]
        #[serde(rename = "code", default)]
        pub code: Option<String>,
        #[doc = "Parameters contains specific information about the code.\n\nThis is a struct to allow parameters of different types.\n\nExamples:\n 1. For a \"HARNESS_STARTED\" message parameters might provide the name\n    of the worker and additional data like timing information.\n 2. For a \"GCS_DOWNLOAD_ERROR\" parameters might contain fields listing\n    the GCS objects being downloaded and fields containing errors.\n\nIn general complex data structures should be avoided. If a worker\nneeds to send a specific and complicated data structure then please\nconsider defining a new proto and adding it to the data oneof in\nWorkerMessageResponse.\n\nConventions:\n Parameters should only be used for information that isn't typically passed\n as a label.\n hostname and other worker identifiers should almost always be passed\n as labels since they will be included on most messages."]
        #[serde(rename = "parameters", default)]
        pub parameters: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for WorkerMessageCode {
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
    pub struct WorkerMessageResponse {
        #[doc = "The service's response to a worker's health report."]
        #[serde(rename = "workerHealthReportResponse", default)]
        pub worker_health_report_response: Option<crate::schemas::WorkerHealthReportResponse>,
        #[doc = "Service's response to reporting worker metrics (currently empty)."]
        #[serde(rename = "workerMetricsResponse", default)]
        pub worker_metrics_response: Option<crate::schemas::ResourceUtilizationReportResponse>,
        #[doc = "Service's response to shutdown notice (currently empty)."]
        #[serde(rename = "workerShutdownNoticeResponse", default)]
        pub worker_shutdown_notice_response: Option<crate::schemas::WorkerShutdownNoticeResponse>,
    }
    impl ::field_selector::FieldSelector for WorkerMessageResponse {
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
    pub enum WorkerPoolDefaultPackageSet {
        #[doc = "The default set of packages to stage is unknown, or unspecified."]
        DefaultPackageSetUnknown,
        #[doc = "Indicates that no packages should be staged at the worker unless\nexplicitly specified by the job."]
        DefaultPackageSetNone,
        #[doc = "Stage packages typically useful to workers written in Java."]
        DefaultPackageSetJava,
        #[doc = "Stage pacakges typically useful to workers written in Python."]
        DefaultPackageSetPython,
    }
    impl WorkerPoolDefaultPackageSet {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerPoolDefaultPackageSet::DefaultPackageSetUnknown => {
                    "DEFAULT_PACKAGE_SET_UNKNOWN"
                }
                WorkerPoolDefaultPackageSet::DefaultPackageSetNone => "DEFAULT_PACKAGE_SET_NONE",
                WorkerPoolDefaultPackageSet::DefaultPackageSetJava => "DEFAULT_PACKAGE_SET_JAVA",
                WorkerPoolDefaultPackageSet::DefaultPackageSetPython => {
                    "DEFAULT_PACKAGE_SET_PYTHON"
                }
            }
        }
    }
    impl ::std::fmt::Display for WorkerPoolDefaultPackageSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolDefaultPackageSet {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolDefaultPackageSet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT_PACKAGE_SET_UNKNOWN" => {
                    WorkerPoolDefaultPackageSet::DefaultPackageSetUnknown
                }
                "DEFAULT_PACKAGE_SET_NONE" => WorkerPoolDefaultPackageSet::DefaultPackageSetNone,
                "DEFAULT_PACKAGE_SET_JAVA" => WorkerPoolDefaultPackageSet::DefaultPackageSetJava,
                "DEFAULT_PACKAGE_SET_PYTHON" => {
                    WorkerPoolDefaultPackageSet::DefaultPackageSetPython
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WorkerPoolIpConfiguration {
        #[doc = "The configuration is unknown, or unspecified."]
        WorkerIpUnspecified,
        #[doc = "Workers should have public IP addresses."]
        WorkerIpPublic,
        #[doc = "Workers should have private IP addresses."]
        WorkerIpPrivate,
    }
    impl WorkerPoolIpConfiguration {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerPoolIpConfiguration::WorkerIpUnspecified => "WORKER_IP_UNSPECIFIED",
                WorkerPoolIpConfiguration::WorkerIpPublic => "WORKER_IP_PUBLIC",
                WorkerPoolIpConfiguration::WorkerIpPrivate => "WORKER_IP_PRIVATE",
            }
        }
    }
    impl ::std::fmt::Display for WorkerPoolIpConfiguration {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolIpConfiguration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolIpConfiguration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "WORKER_IP_UNSPECIFIED" => WorkerPoolIpConfiguration::WorkerIpUnspecified,
                "WORKER_IP_PUBLIC" => WorkerPoolIpConfiguration::WorkerIpPublic,
                "WORKER_IP_PRIVATE" => WorkerPoolIpConfiguration::WorkerIpPrivate,
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
    pub enum WorkerPoolTeardownPolicy {
        #[doc = "The teardown policy isn't specified, or is unknown."]
        TeardownPolicyUnknown,
        #[doc = "Always teardown the resource."]
        TeardownAlways,
        #[doc = "Teardown the resource on success. This is useful for debugging\nfailures."]
        TeardownOnSuccess,
        #[doc = "Never teardown the resource. This is useful for debugging and\ndevelopment."]
        TeardownNever,
    }
    impl WorkerPoolTeardownPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerPoolTeardownPolicy::TeardownPolicyUnknown => "TEARDOWN_POLICY_UNKNOWN",
                WorkerPoolTeardownPolicy::TeardownAlways => "TEARDOWN_ALWAYS",
                WorkerPoolTeardownPolicy::TeardownOnSuccess => "TEARDOWN_ON_SUCCESS",
                WorkerPoolTeardownPolicy::TeardownNever => "TEARDOWN_NEVER",
            }
        }
    }
    impl ::std::fmt::Display for WorkerPoolTeardownPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolTeardownPolicy {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolTeardownPolicy {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TEARDOWN_POLICY_UNKNOWN" => WorkerPoolTeardownPolicy::TeardownPolicyUnknown,
                "TEARDOWN_ALWAYS" => WorkerPoolTeardownPolicy::TeardownAlways,
                "TEARDOWN_ON_SUCCESS" => WorkerPoolTeardownPolicy::TeardownOnSuccess,
                "TEARDOWN_NEVER" => WorkerPoolTeardownPolicy::TeardownNever,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkerPool {
        #[doc = "Settings for autoscaling of this WorkerPool."]
        #[serde(rename = "autoscalingSettings", default)]
        pub autoscaling_settings: Option<crate::schemas::AutoscalingSettings>,
        #[doc = "Data disks that are used by a VM in this workflow."]
        #[serde(rename = "dataDisks", default)]
        pub data_disks: Option<Vec<crate::schemas::Disk>>,
        #[doc = "The default package set to install.  This allows the service to\nselect a default set of packages which are useful to worker\nharnesses written in a particular language."]
        #[serde(rename = "defaultPackageSet", default)]
        pub default_package_set: Option<crate::schemas::WorkerPoolDefaultPackageSet>,
        #[doc = "Size of root disk for VMs, in GB.  If zero or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "diskSizeGb", default)]
        pub disk_size_gb: Option<i32>,
        #[doc = "Fully qualified source image for disks."]
        #[serde(rename = "diskSourceImage", default)]
        pub disk_source_image: Option<String>,
        #[doc = "Type of root disk for VMs.  If empty or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "diskType", default)]
        pub disk_type: Option<String>,
        #[doc = "Configuration for VM IPs."]
        #[serde(rename = "ipConfiguration", default)]
        pub ip_configuration: Option<crate::schemas::WorkerPoolIpConfiguration>,
        #[doc = "The kind of the worker pool; currently only `harness` and `shuffle`\nare supported."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Machine type (e.g. \"n1-standard-1\").  If empty or unspecified, the\nservice will attempt to choose a reasonable default."]
        #[serde(rename = "machineType", default)]
        pub machine_type: Option<String>,
        #[doc = "Metadata to set on the Google Compute Engine VMs."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Network to which VMs will be assigned.  If empty or unspecified,\nthe service will use the network \"default\"."]
        #[serde(rename = "network", default)]
        pub network: Option<String>,
        #[doc = "The number of threads per worker harness. If empty or unspecified, the\nservice will choose a number of threads (according to the number of cores\non the selected machine type for batch, or 1 by convention for streaming)."]
        #[serde(rename = "numThreadsPerWorker", default)]
        pub num_threads_per_worker: Option<i32>,
        #[doc = "Number of Google Compute Engine workers in this pool needed to\nexecute the job.  If zero or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "numWorkers", default)]
        pub num_workers: Option<i32>,
        #[doc = "The action to take on host maintenance, as defined by the Google\nCompute Engine API."]
        #[serde(rename = "onHostMaintenance", default)]
        pub on_host_maintenance: Option<String>,
        #[doc = "Packages to be installed on workers."]
        #[serde(rename = "packages", default)]
        pub packages: Option<Vec<crate::schemas::Package>>,
        #[doc = "Extra arguments for this worker pool."]
        #[serde(rename = "poolArgs", default)]
        pub pool_args: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Subnetwork to which VMs will be assigned, if desired.  Expected to be of\nthe form \"regions/REGION/subnetworks/SUBNETWORK\"."]
        #[serde(rename = "subnetwork", default)]
        pub subnetwork: Option<String>,
        #[doc = "Settings passed through to Google Compute Engine workers when\nusing the standard Dataflow task runner.  Users should ignore\nthis field."]
        #[serde(rename = "taskrunnerSettings", default)]
        pub taskrunner_settings: Option<crate::schemas::TaskRunnerSettings>,
        #[doc = "Sets the policy for determining when to turndown worker pool.\nAllowed values are: `TEARDOWN_ALWAYS`, `TEARDOWN_ON_SUCCESS`, and\n`TEARDOWN_NEVER`.\n`TEARDOWN_ALWAYS` means workers are always torn down regardless of whether\nthe job succeeds. `TEARDOWN_ON_SUCCESS` means workers are torn down\nif the job succeeds. `TEARDOWN_NEVER` means the workers are never torn\ndown.\n\nIf the workers are not torn down by the service, they will\ncontinue to run and use Google Compute Engine VM resources in the\nuser's project until they are explicitly terminated by the user.\nBecause of this, Google recommends using the `TEARDOWN_ALWAYS`\npolicy except for small, manually supervised test jobs.\n\nIf unknown or unspecified, the service will attempt to choose a reasonable\ndefault."]
        #[serde(rename = "teardownPolicy", default)]
        pub teardown_policy: Option<crate::schemas::WorkerPoolTeardownPolicy>,
        #[doc = "Required. Docker container image that executes the Cloud Dataflow worker\nharness, residing in Google Container Registry."]
        #[serde(rename = "workerHarnessContainerImage", default)]
        pub worker_harness_container_image: Option<String>,
        #[doc = "Zone to run the worker pools in.  If empty or unspecified, the service\nwill attempt to choose a reasonable default."]
        #[serde(rename = "zone", default)]
        pub zone: Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerPool {
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
    pub struct WorkerSettings {
        #[doc = "The base URL for accessing Google Cloud APIs.\n\nWhen workers access Google Cloud APIs, they logically do so via\nrelative URLs.  If this field is specified, it supplies the base\nURL to use for resolving these relative URLs.  The normative\nalgorithm used is defined by RFC 1808, \"Relative Uniform Resource\nLocators\".\n\nIf not specified, the default value is \"http://www.googleapis.com/\""]
        #[serde(rename = "baseUrl", default)]
        pub base_url: Option<String>,
        #[doc = "Whether to send work progress updates to the service."]
        #[serde(rename = "reportingEnabled", default)]
        pub reporting_enabled: Option<bool>,
        #[doc = "The Cloud Dataflow service path relative to the root URL, for example,\n\"dataflow/v1b3/projects\"."]
        #[serde(rename = "servicePath", default)]
        pub service_path: Option<String>,
        #[doc = "The Shuffle service path relative to the root URL, for example,\n\"shuffle/v1beta1\"."]
        #[serde(rename = "shuffleServicePath", default)]
        pub shuffle_service_path: Option<String>,
        #[doc = "The prefix of the resources the system should use for temporary\nstorage.\n\nThe supported resource type is:\n\nGoogle Cloud Storage:\n\n  storage.googleapis.com/{bucket}/{object}\n  bucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempStoragePrefix", default)]
        pub temp_storage_prefix: Option<String>,
        #[doc = "The ID of the worker running this pipeline."]
        #[serde(rename = "workerId", default)]
        pub worker_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerSettings {
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
    pub struct WorkerShutdownNotice {
        #[doc = "The reason for the worker shutdown.\nCurrent possible values are:\n  \"UNKNOWN\": shutdown reason is unknown.\n  \"PREEMPTION\": shutdown reason is preemption.\nOther possible reasons may be added in the future."]
        #[serde(rename = "reason", default)]
        pub reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerShutdownNotice {
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
    pub struct WorkerShutdownNoticeResponse;
    impl ::field_selector::FieldSelector for WorkerShutdownNoticeResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WriteInstruction {
        #[doc = "The input."]
        #[serde(rename = "input", default)]
        pub input: Option<crate::schemas::InstructionInput>,
        #[doc = "The sink to write to."]
        #[serde(rename = "sink", default)]
        pub sink: Option<crate::schemas::Sink>,
    }
    impl ::field_selector::FieldSelector for WriteInstruction {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::projects::ProjectsActions<A> {
        crate::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod projects {
    pub mod params {}
    pub struct ProjectsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
        #[doc = "Deletes a snapshot."]
        pub fn delete_snapshots(
            &self,
            project_id: impl Into<String>,
        ) -> DeleteSnapshotsRequestBuilder<A> {
            DeleteSnapshotsRequestBuilder {
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
                project_id: project_id.into(),
                location: None,
                snapshot_id: None,
            }
        }
        #[doc = "Send a worker_message to the service."]
        pub fn worker_messages(
            &self,
            request: crate::schemas::SendWorkerMessagesRequest,
            project_id: impl Into<String>,
        ) -> WorkerMessagesRequestBuilder<A> {
            WorkerMessagesRequestBuilder {
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
                project_id: project_id.into(),
            }
        }
        #[doc = "Actions that can be performed on the jobs resource"]
        pub fn jobs(&self) -> jobs::JobsActions<A> {
            jobs::JobsActions
        }
        #[doc = "Actions that can be performed on the locations resource"]
        pub fn locations(&self) -> locations::LocationsActions<A> {
            locations::LocationsActions
        }
        #[doc = "Actions that can be performed on the snapshots resource"]
        pub fn snapshots(&self) -> snapshots::SnapshotsActions<A> {
            snapshots::SnapshotsActions
        }
        #[doc = "Actions that can be performed on the templates resource"]
        pub fn templates(&self) -> templates::TemplatesActions<A> {
            templates::TemplatesActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteSnapshotsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        project_id: String,
        location: Option<String>,
        snapshot_id: Option<String>,
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
    impl<'a, A: yup_oauth2::GetToken> DeleteSnapshotsRequestBuilder<'a, A> {
        #[doc = "The location that contains this snapshot."]
        pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
            self.location = Some(value.into());
            self
        }
        #[doc = "The ID of the snapshot."]
        pub fn snapshot_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.snapshot_id = Some(value.into());
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
        ) -> Result<crate::schemas::DeleteSnapshotResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://dataflow.googleapis.com/".to_owned();
            output.push_str("v1b3/projects/");
            output.push_str(&self.project_id);
            output.push_str("/snapshots");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("location", &self.location)]);
            let req = req.query(&[("snapshotId", &self.snapshot_id)]);
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
    pub struct WorkerMessagesRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::SendWorkerMessagesRequest,
        project_id: String,
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
    impl<'a, A: yup_oauth2::GetToken> WorkerMessagesRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::SendWorkerMessagesResponse, Box<dyn ::std::error::Error>>
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
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://dataflow.googleapis.com/".to_owned();
            output.push_str("v1b3/projects/");
            output.push_str(&self.project_id);
            output.push_str("/WorkerMessages");
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
    pub mod jobs {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum AggregatedFilter {}
            impl AggregatedFilter {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for AggregatedFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for AggregatedFilter {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for AggregatedFilter {
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
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum AggregatedView {}
            impl AggregatedView {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for AggregatedView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for AggregatedView {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for AggregatedView {
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
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum CreateView {}
            impl CreateView {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for CreateView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for CreateView {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for CreateView {
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
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetView {}
            impl GetView {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for GetView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetView {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetView {
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
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListFilter {}
            impl ListFilter {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for ListFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListFilter {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListFilter {
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
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListView {}
            impl ListView {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for ListView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListView {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListView {
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
        pub struct JobsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> JobsActions<'a, A> {
            #[doc = "List the jobs of a project across all regions."]
            pub fn aggregated(&self, project_id: impl Into<String>) -> AggregatedRequestBuilder<A> {
                AggregatedRequestBuilder {
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
                    project_id: project_id.into(),
                    filter: None,
                    location: None,
                    page_size: None,
                    page_token: None,
                    view: None,
                }
            }
            #[doc = "Creates a Cloud Dataflow job.\n\nTo create a job, we recommend using `projects.locations.jobs.create` with a\n[regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.create` is not recommended, as your job will always start\nin `us-central1`."]
            pub fn create(
                &self,
                request: crate::schemas::Job,
                project_id: impl Into<String>,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
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
                    project_id: project_id.into(),
                    location: None,
                    replace_job_id: None,
                    view: None,
                }
            }
            #[doc = "Gets the state of the specified Cloud Dataflow job.\n\nTo get the state of a job, we recommend using `projects.locations.jobs.get`\nwith a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.get` is not recommended, as you can only get the state of\njobs that are running in `us-central1`."]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                job_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
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
                    project_id: project_id.into(),
                    job_id: job_id.into(),
                    location: None,
                    view: None,
                }
            }
            #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.getMetrics` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.getMetrics` is not recommended, as you can only request the\nstatus of jobs that are running in `us-central1`."]
            pub fn get_metrics(
                &self,
                project_id: impl Into<String>,
                job_id: impl Into<String>,
            ) -> GetMetricsRequestBuilder<A> {
                GetMetricsRequestBuilder {
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
                    project_id: project_id.into(),
                    job_id: job_id.into(),
                    location: None,
                    start_time: None,
                }
            }
            #[doc = "List the jobs of a project.\n\nTo list the jobs of a project in a region, we recommend using\n`projects.locations.jobs.get` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). To\nlist the all jobs across all regions, use `projects.jobs.aggregated`. Using\n`projects.jobs.list` is not recommended, as you can only get the list of\njobs that are running in `us-central1`."]
            pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
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
                    project_id: project_id.into(),
                    filter: None,
                    location: None,
                    page_size: None,
                    page_token: None,
                    view: None,
                }
            }
            #[doc = "Snapshot the state of a streaming job."]
            pub fn snapshot(
                &self,
                request: crate::schemas::SnapshotJobRequest,
                project_id: impl Into<String>,
                job_id: impl Into<String>,
            ) -> SnapshotRequestBuilder<A> {
                SnapshotRequestBuilder {
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
                    project_id: project_id.into(),
                    job_id: job_id.into(),
                }
            }
            #[doc = "Updates the state of an existing Cloud Dataflow job.\n\nTo update the state of an existing job, we recommend using\n`projects.locations.jobs.update` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.update` is not recommended, as you can only update the state\nof jobs that are running in `us-central1`."]
            pub fn update(
                &self,
                request: crate::schemas::Job,
                project_id: impl Into<String>,
                job_id: impl Into<String>,
            ) -> UpdateRequestBuilder<A> {
                UpdateRequestBuilder {
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
                    project_id: project_id.into(),
                    job_id: job_id.into(),
                    location: None,
                }
            }
            #[doc = "Actions that can be performed on the debug resource"]
            pub fn debug(&self) -> debug::DebugActions<A> {
                debug::DebugActions
            }
            #[doc = "Actions that can be performed on the messages resource"]
            pub fn messages(&self) -> messages::MessagesActions<A> {
                messages::MessagesActions
            }
            #[doc = "Actions that can be performed on the work_items resource"]
            pub fn work_items(&self) -> work_items::WorkItemsActions<A> {
                work_items::WorkItemsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct AggregatedRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            filter: Option<crate::jobs::params::AggregatedFilter>,
            location: Option<String>,
            page_size: Option<i32>,
            page_token: Option<String>,
            view: Option<crate::jobs::params::AggregatedView>,
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
        impl<'a, A: yup_oauth2::GetToken> AggregatedRequestBuilder<'a, A> {
            #[doc = "The kind of filter to use."]
            pub fn filter(&mut self, value: crate::jobs::params::AggregatedFilter) -> &mut Self {
                self.filter = Some(value);
                self
            }
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "If there are many jobs, limit response to at most this many.\nThe actual number of jobs returned will be the lesser of max_responses\nand an unspecified server-defined limit."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Set this to the 'next_page_token' field of a previous response\nto request additional results in a long list."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Level of information requested in response. Default is `JOB_VIEW_SUMMARY`."]
            pub fn view(&mut self, value: crate::jobs::params::AggregatedView) -> &mut Self {
                self.view = Some(value);
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
            pub fn iter_failed_location<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "failedLocation")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter_jobs<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "jobs")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
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
            ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/jobs:aggregated");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("view", &self.view)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/compute.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for AggregatedRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Job,
            project_id: String,
            location: Option<String>,
            replace_job_id: Option<String>,
            view: Option<crate::jobs::params::CreateView>,
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
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "Deprecated. This field is now in the Job message."]
            pub fn replace_job_id(&mut self, value: impl Into<String>) -> &mut Self {
                self.replace_job_id = Some(value.into());
                self
            }
            #[doc = "The level of information requested in response."]
            pub fn view(&mut self, value: crate::jobs::params::CreateView) -> &mut Self {
                self.view = Some(value);
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/jobs");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("replaceJobId", &self.replace_job_id)]);
                let req = req.query(&[("view", &self.view)]);
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
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            job_id: String,
            location: Option<String>,
            view: Option<crate::jobs::params::GetView>,
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
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "The level of information requested in response."]
            pub fn view(&mut self, value: crate::jobs::params::GetView) -> &mut Self {
                self.view = Some(value);
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
            ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/jobs/");
                output.push_str(&self.job_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("view", &self.view)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/compute.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetMetricsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            job_id: String,
            location: Option<String>,
            start_time: Option<String>,
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
        impl<'a, A: yup_oauth2::GetToken> GetMetricsRequestBuilder<'a, A> {
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "Return only metric data that has changed since this time.\nDefault is to return all information about all metrics for the job."]
            pub fn start_time(&mut self, value: impl Into<String>) -> &mut Self {
                self.start_time = Some(value.into());
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
            ) -> Result<crate::schemas::JobMetrics, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/jobs/");
                output.push_str(&self.job_id);
                output.push_str("/metrics");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("startTime", &self.start_time)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/compute.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            filter: Option<crate::jobs::params::ListFilter>,
            location: Option<String>,
            page_size: Option<i32>,
            page_token: Option<String>,
            view: Option<crate::jobs::params::ListView>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "The kind of filter to use."]
            pub fn filter(&mut self, value: crate::jobs::params::ListFilter) -> &mut Self {
                self.filter = Some(value);
                self
            }
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "If there are many jobs, limit response to at most this many.\nThe actual number of jobs returned will be the lesser of max_responses\nand an unspecified server-defined limit."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Set this to the 'next_page_token' field of a previous response\nto request additional results in a long list."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Level of information requested in response. Default is `JOB_VIEW_SUMMARY`."]
            pub fn view(&mut self, value: crate::jobs::params::ListView) -> &mut Self {
                self.view = Some(value);
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
            pub fn iter_failed_location<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "failedLocation")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter_jobs<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                struct ItemIter<'a, M, T> {
                    method: &'a mut M,
                    finished: bool,
                    items_iter: Option<::std::vec::IntoIter<T>>,
                }
                impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                where
                    M: crate::IterableMethod,
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    type Item = Result<T, Box<dyn ::std::error::Error>>;
                    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                        use ::field_selector::FieldSelector;
                        #[derive(:: serde :: Deserialize, FieldSelector)]
                        struct Resp<T>
                        where
                            T: FieldSelector,
                        {
                            #[serde(rename = "jobs")]
                            items: Option<Vec<T>>,
                            #[serde(rename = "nextPageToken")]
                            next_page_token: Option<String>,
                        }
                        loop {
                            if let Some(iter) = self.items_iter.as_mut() {
                                match iter.next() {
                                    Some(v) => return Some(Ok(v)),
                                    None => {}
                                }
                            }
                            if self.finished {
                                return None;
                            }
                            let resp: Resp<T> = match self.method.execute() {
                                Ok(r) => r,
                                Err(err) => return Some(Err(err)),
                            };
                            if let Some(next_page_token) = resp.next_page_token {
                                self.method.set_page_token(next_page_token);
                            } else {
                                self.finished = true;
                            }
                            self.items_iter = resp.items.map(|i| i.into_iter());
                        }
                    }
                }
                ItemIter {
                    method: self,
                    finished: false,
                    items_iter: None,
                }
            }
            pub fn iter<T>(
                &'a mut self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
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
            ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/jobs");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("view", &self.view)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/compute.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
        }
        #[derive(Debug, Clone)]
        pub struct SnapshotRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::SnapshotJobRequest,
            project_id: String,
            job_id: String,
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
        impl<'a, A: yup_oauth2::GetToken> SnapshotRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/jobs/");
                output.push_str(&self.job_id);
                output.push_str(":snapshot");
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
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Job,
            project_id: String,
            job_id: String,
            location: Option<String>,
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
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/jobs/");
                output.push_str(&self.job_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("location", &self.location)]);
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
        pub mod debug {
            pub mod params {}
            pub struct DebugActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> DebugActions<'a, A> {
                #[doc = "Get encoded debug configuration for component. Not cacheable."]
                pub fn get_config(
                    &self,
                    request: crate::schemas::GetDebugConfigRequest,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> GetConfigRequestBuilder<A> {
                    GetConfigRequestBuilder {
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
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                    }
                }
                #[doc = "Send encoded debug capture data for component."]
                pub fn send_capture(
                    &self,
                    request: crate::schemas::SendDebugCaptureRequest,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> SendCaptureRequestBuilder<A> {
                    SendCaptureRequestBuilder {
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
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetConfigRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::GetDebugConfigRequest,
                project_id: String,
                job_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> GetConfigRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::GetDebugConfigResponse, Box<dyn ::std::error::Error>>
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output.push_str("/debug/getConfig");
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
            #[derive(Debug, Clone)]
            pub struct SendCaptureRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::SendDebugCaptureRequest,
                project_id: String,
                job_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> SendCaptureRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::SendDebugCaptureResponse, Box<dyn ::std::error::Error>>
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output.push_str("/debug/sendCapture");
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
        pub mod messages {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListMinimumImportance {}
                impl ListMinimumImportance {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for ListMinimumImportance {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListMinimumImportance {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListMinimumImportance {
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
            pub struct MessagesActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> MessagesActions<'a, A> {
                #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.messages.list` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.messages.list` is not recommended, as you can only request\nthe status of jobs that are running in `us-central1`."]
                pub fn list(
                    &self,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
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
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                        end_time: None,
                        location: None,
                        minimum_importance: None,
                        page_size: None,
                        page_token: None,
                        start_time: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                job_id: String,
                end_time: Option<String>,
                location: Option<String>,
                minimum_importance: Option<crate::messages::params::ListMinimumImportance>,
                page_size: Option<i32>,
                page_token: Option<String>,
                start_time: Option<String>,
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Return only messages with timestamps < end_time. The default is now\n(i.e. return up to the latest messages available)."]
                pub fn end_time(&mut self, value: impl Into<String>) -> &mut Self {
                    self.end_time = Some(value.into());
                    self
                }
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
                pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "Filter to only get messages with importance >= level"]
                pub fn minimum_importance(
                    &mut self,
                    value: crate::messages::params::ListMinimumImportance,
                ) -> &mut Self {
                    self.minimum_importance = Some(value);
                    self
                }
                #[doc = "If specified, determines the maximum number of messages to\nreturn.  If unspecified, the service may choose an appropriate\ndefault, or may return an arbitrarily large number of results."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "If supplied, this should be the value of next_page_token returned\nby an earlier call. This will cause the next page of results to\nbe returned."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "If specified, return only messages with timestamps >= start_time.\nThe default is the job creation time (i.e. beginning of messages)."]
                pub fn start_time(&mut self, value: impl Into<String>) -> &mut Self {
                    self.start_time = Some(value.into());
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
                pub fn iter_autoscaling_events<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "autoscalingEvents")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter_job_messages<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "jobMessages")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
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
                ) -> Result<crate::schemas::ListJobMessagesResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output.push_str("/messages");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("endTime", &self.end_time)]);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("minimumImportance", &self.minimum_importance)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("startTime", &self.start_time)]);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/compute.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
        }
        pub mod work_items {
            pub mod params {}
            pub struct WorkItemsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> WorkItemsActions<'a, A> {
                #[doc = "Leases a dataflow WorkItem to run."]
                pub fn lease(
                    &self,
                    request: crate::schemas::LeaseWorkItemRequest,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> LeaseRequestBuilder<A> {
                    LeaseRequestBuilder {
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
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                    }
                }
                #[doc = "Reports the status of dataflow WorkItems leased by a worker."]
                pub fn report_status(
                    &self,
                    request: crate::schemas::ReportWorkItemStatusRequest,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> ReportStatusRequestBuilder<A> {
                    ReportStatusRequestBuilder {
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
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct LeaseRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::LeaseWorkItemRequest,
                project_id: String,
                job_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> LeaseRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::LeaseWorkItemResponse, Box<dyn ::std::error::Error>>
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output.push_str("/workItems:lease");
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
            #[derive(Debug, Clone)]
            pub struct ReportStatusRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::ReportWorkItemStatusRequest,
                project_id: String,
                job_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> ReportStatusRequestBuilder<'a, A> {
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
                ) -> Result<
                    crate::schemas::ReportWorkItemStatusResponse,
                    Box<dyn ::std::error::Error>,
                > {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output.push_str("/workItems:reportStatus");
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
    }
    pub mod locations {
        pub mod params {}
        pub struct LocationsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> LocationsActions<'a, A> {
            #[doc = "Send a worker_message to the service."]
            pub fn worker_messages(
                &self,
                request: crate::schemas::SendWorkerMessagesRequest,
                project_id: impl Into<String>,
                location: impl Into<String>,
            ) -> WorkerMessagesRequestBuilder<A> {
                WorkerMessagesRequestBuilder {
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
                    project_id: project_id.into(),
                    location: location.into(),
                }
            }
            #[doc = "Actions that can be performed on the jobs resource"]
            pub fn jobs(&self) -> jobs::JobsActions<A> {
                jobs::JobsActions
            }
            #[doc = "Actions that can be performed on the snapshots resource"]
            pub fn snapshots(&self) -> snapshots::SnapshotsActions<A> {
                snapshots::SnapshotsActions
            }
            #[doc = "Actions that can be performed on the sql resource"]
            pub fn sql(&self) -> sql::SqlActions<A> {
                sql::SqlActions
            }
            #[doc = "Actions that can be performed on the templates resource"]
            pub fn templates(&self) -> templates::TemplatesActions<A> {
                templates::TemplatesActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct WorkerMessagesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::SendWorkerMessagesRequest,
            project_id: String,
            location: String,
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
        impl<'a, A: yup_oauth2::GetToken> WorkerMessagesRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::SendWorkerMessagesResponse, Box<dyn ::std::error::Error>>
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/locations/");
                output.push_str(&self.location);
                output.push_str("/WorkerMessages");
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
        pub mod jobs {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum CreateView {}
                impl CreateView {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for CreateView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for CreateView {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for CreateView {
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
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetView {}
                impl GetView {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for GetView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetView {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetView {
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
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListFilter {}
                impl ListFilter {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for ListFilter {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListFilter {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListFilter {
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
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListView {}
                impl ListView {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for ListView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListView {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListView {
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
            pub struct JobsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> JobsActions<'a, A> {
                #[doc = "Creates a Cloud Dataflow job.\n\nTo create a job, we recommend using `projects.locations.jobs.create` with a\n[regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.create` is not recommended, as your job will always start\nin `us-central1`."]
                pub fn create(
                    &self,
                    request: crate::schemas::Job,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        replace_job_id: None,
                        view: None,
                    }
                }
                #[doc = "Gets the state of the specified Cloud Dataflow job.\n\nTo get the state of a job, we recommend using `projects.locations.jobs.get`\nwith a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.get` is not recommended, as you can only get the state of\njobs that are running in `us-central1`."]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> GetRequestBuilder<A> {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        job_id: job_id.into(),
                        view: None,
                    }
                }
                #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.getMetrics` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.getMetrics` is not recommended, as you can only request the\nstatus of jobs that are running in `us-central1`."]
                pub fn get_metrics(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> GetMetricsRequestBuilder<A> {
                    GetMetricsRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        job_id: job_id.into(),
                        start_time: None,
                    }
                }
                #[doc = "List the jobs of a project.\n\nTo list the jobs of a project in a region, we recommend using\n`projects.locations.jobs.get` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). To\nlist the all jobs across all regions, use `projects.jobs.aggregated`. Using\n`projects.jobs.list` is not recommended, as you can only get the list of\njobs that are running in `us-central1`."]
                pub fn list(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                        view: None,
                    }
                }
                #[doc = "Snapshot the state of a streaming job."]
                pub fn snapshot(
                    &self,
                    request: crate::schemas::SnapshotJobRequest,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> SnapshotRequestBuilder<A> {
                    SnapshotRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        job_id: job_id.into(),
                    }
                }
                #[doc = "Updates the state of an existing Cloud Dataflow job.\n\nTo update the state of an existing job, we recommend using\n`projects.locations.jobs.update` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.update` is not recommended, as you can only update the state\nof jobs that are running in `us-central1`."]
                pub fn update(
                    &self,
                    request: crate::schemas::Job,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> UpdateRequestBuilder<A> {
                    UpdateRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        job_id: job_id.into(),
                    }
                }
                #[doc = "Actions that can be performed on the debug resource"]
                pub fn debug(&self) -> debug::DebugActions<A> {
                    debug::DebugActions
                }
                #[doc = "Actions that can be performed on the messages resource"]
                pub fn messages(&self) -> messages::MessagesActions<A> {
                    messages::MessagesActions
                }
                #[doc = "Actions that can be performed on the snapshots resource"]
                pub fn snapshots(&self) -> snapshots::SnapshotsActions<A> {
                    snapshots::SnapshotsActions
                }
                #[doc = "Actions that can be performed on the work_items resource"]
                pub fn work_items(&self) -> work_items::WorkItemsActions<A> {
                    work_items::WorkItemsActions
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Job,
                project_id: String,
                location: String,
                replace_job_id: Option<String>,
                view: Option<crate::jobs::params::CreateView>,
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
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "Deprecated. This field is now in the Job message."]
                pub fn replace_job_id(&mut self, value: impl Into<String>) -> &mut Self {
                    self.replace_job_id = Some(value.into());
                    self
                }
                #[doc = "The level of information requested in response."]
                pub fn view(&mut self, value: crate::jobs::params::CreateView) -> &mut Self {
                    self.view = Some(value);
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/jobs");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("replaceJobId", &self.replace_job_id)]);
                    let req = req.query(&[("view", &self.view)]);
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
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                job_id: String,
                view: Option<crate::jobs::params::GetView>,
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
                #[doc = "The level of information requested in response."]
                pub fn view(&mut self, value: crate::jobs::params::GetView) -> &mut Self {
                    self.view = Some(value);
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("view", &self.view)]);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/compute.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetMetricsRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                job_id: String,
                start_time: Option<String>,
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
            impl<'a, A: yup_oauth2::GetToken> GetMetricsRequestBuilder<'a, A> {
                #[doc = "Return only metric data that has changed since this time.\nDefault is to return all information about all metrics for the job."]
                pub fn start_time(&mut self, value: impl Into<String>) -> &mut Self {
                    self.start_time = Some(value.into());
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
                ) -> Result<crate::schemas::JobMetrics, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output.push_str("/metrics");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("startTime", &self.start_time)]);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/compute.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                filter: Option<crate::jobs::params::ListFilter>,
                page_size: Option<i32>,
                page_token: Option<String>,
                view: Option<crate::jobs::params::ListView>,
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "The kind of filter to use."]
                pub fn filter(&mut self, value: crate::jobs::params::ListFilter) -> &mut Self {
                    self.filter = Some(value);
                    self
                }
                #[doc = "If there are many jobs, limit response to at most this many.\nThe actual number of jobs returned will be the lesser of max_responses\nand an unspecified server-defined limit."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Set this to the 'next_page_token' field of a previous response\nto request additional results in a long list."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Level of information requested in response. Default is `JOB_VIEW_SUMMARY`."]
                pub fn view(&mut self, value: crate::jobs::params::ListView) -> &mut Self {
                    self.view = Some(value);
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
                pub fn iter_failed_location<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "failedLocation")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter_jobs<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    struct ItemIter<'a, M, T> {
                        method: &'a mut M,
                        finished: bool,
                        items_iter: Option<::std::vec::IntoIter<T>>,
                    }
                    impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                    where
                        M: crate::IterableMethod,
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        type Item = Result<T, Box<dyn ::std::error::Error>>;
                        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                            use ::field_selector::FieldSelector;
                            #[derive(:: serde :: Deserialize, FieldSelector)]
                            struct Resp<T>
                            where
                                T: FieldSelector,
                            {
                                #[serde(rename = "jobs")]
                                items: Option<Vec<T>>,
                                #[serde(rename = "nextPageToken")]
                                next_page_token: Option<String>,
                            }
                            loop {
                                if let Some(iter) = self.items_iter.as_mut() {
                                    match iter.next() {
                                        Some(v) => return Some(Ok(v)),
                                        None => {}
                                    }
                                }
                                if self.finished {
                                    return None;
                                }
                                let resp: Resp<T> = match self.method.execute() {
                                    Ok(r) => r,
                                    Err(err) => return Some(Err(err)),
                                };
                                if let Some(next_page_token) = resp.next_page_token {
                                    self.method.set_page_token(next_page_token);
                                } else {
                                    self.finished = true;
                                }
                                self.items_iter = resp.items.map(|i| i.into_iter());
                            }
                        }
                    }
                    ItemIter {
                        method: self,
                        finished: false,
                        items_iter: None,
                    }
                }
                pub fn iter<T>(
                    &'a mut self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
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
                ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/jobs");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("view", &self.view)]);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/compute.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    self._execute()
                }
            }
            #[derive(Debug, Clone)]
            pub struct SnapshotRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::SnapshotJobRequest,
                project_id: String,
                location: String,
                job_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> SnapshotRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output.push_str(":snapshot");
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
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Job,
                project_id: String,
                location: String,
                job_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/jobs/");
                    output.push_str(&self.job_id);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
            pub mod debug {
                pub mod params {}
                pub struct DebugActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> DebugActions<'a, A> {
                    #[doc = "Get encoded debug configuration for component. Not cacheable."]
                    pub fn get_config(
                        &self,
                        request: crate::schemas::GetDebugConfigRequest,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> GetConfigRequestBuilder<A> {
                        GetConfigRequestBuilder {
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
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                        }
                    }
                    #[doc = "Send encoded debug capture data for component."]
                    pub fn send_capture(
                        &self,
                        request: crate::schemas::SendDebugCaptureRequest,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> SendCaptureRequestBuilder<A> {
                        SendCaptureRequestBuilder {
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
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetConfigRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::GetDebugConfigRequest,
                    project_id: String,
                    location: String,
                    job_id: String,
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
                impl<'a, A: yup_oauth2::GetToken> GetConfigRequestBuilder<'a, A> {
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
                    ) -> Result<crate::schemas::GetDebugConfigResponse, Box<dyn ::std::error::Error>>
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
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        output.push_str(&self.project_id);
                        output.push_str("/locations/");
                        output.push_str(&self.location);
                        output.push_str("/jobs/");
                        output.push_str(&self.job_id);
                        output.push_str("/debug/getConfig");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SendCaptureRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::SendDebugCaptureRequest,
                    project_id: String,
                    location: String,
                    job_id: String,
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
                impl<'a, A: yup_oauth2::GetToken> SendCaptureRequestBuilder<'a, A> {
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
                    ) -> Result<
                        crate::schemas::SendDebugCaptureResponse,
                        Box<dyn ::std::error::Error>,
                    > {
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        output.push_str(&self.project_id);
                        output.push_str("/locations/");
                        output.push_str(&self.location);
                        output.push_str("/jobs/");
                        output.push_str(&self.job_id);
                        output.push_str("/debug/sendCapture");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod messages {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListMinimumImportance {}
                    impl ListMinimumImportance {
                        pub fn as_str(self) -> &'static str {
                            match self {}
                        }
                    }
                    impl ::std::fmt::Display for ListMinimumImportance {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListMinimumImportance {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for ListMinimumImportance {
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
                pub struct MessagesActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> MessagesActions<'a, A> {
                    #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.messages.list` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.messages.list` is not recommended, as you can only request\nthe status of jobs that are running in `us-central1`."]
                    pub fn list(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
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
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                            end_time: None,
                            minimum_importance: None,
                            page_size: None,
                            page_token: None,
                            start_time: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    job_id: String,
                    end_time: Option<String>,
                    minimum_importance: Option<crate::messages::params::ListMinimumImportance>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    start_time: Option<String>,
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
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Return only messages with timestamps < end_time. The default is now\n(i.e. return up to the latest messages available)."]
                    pub fn end_time(&mut self, value: impl Into<String>) -> &mut Self {
                        self.end_time = Some(value.into());
                        self
                    }
                    #[doc = "Filter to only get messages with importance >= level"]
                    pub fn minimum_importance(
                        &mut self,
                        value: crate::messages::params::ListMinimumImportance,
                    ) -> &mut Self {
                        self.minimum_importance = Some(value);
                        self
                    }
                    #[doc = "If specified, determines the maximum number of messages to\nreturn.  If unspecified, the service may choose an appropriate\ndefault, or may return an arbitrarily large number of results."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "If supplied, this should be the value of next_page_token returned\nby an earlier call. This will cause the next page of results to\nbe returned."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "If specified, return only messages with timestamps >= start_time.\nThe default is the job creation time (i.e. beginning of messages)."]
                    pub fn start_time(&mut self, value: impl Into<String>) -> &mut Self {
                        self.start_time = Some(value.into());
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
                    pub fn iter_autoscaling_events<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "autoscalingEvents")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter_job_messages<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        struct ItemIter<'a, M, T> {
                            method: &'a mut M,
                            finished: bool,
                            items_iter: Option<::std::vec::IntoIter<T>>,
                        }
                        impl<'a, M, T> Iterator for ItemIter<'a, M, T>
                        where
                            M: crate::IterableMethod,
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            type Item = Result<T, Box<dyn ::std::error::Error>>;
                            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                                use ::field_selector::FieldSelector;
                                #[derive(:: serde :: Deserialize, FieldSelector)]
                                struct Resp<T>
                                where
                                    T: FieldSelector,
                                {
                                    #[serde(rename = "jobMessages")]
                                    items: Option<Vec<T>>,
                                    #[serde(rename = "nextPageToken")]
                                    next_page_token: Option<String>,
                                }
                                loop {
                                    if let Some(iter) = self.items_iter.as_mut() {
                                        match iter.next() {
                                            Some(v) => return Some(Ok(v)),
                                            None => {}
                                        }
                                    }
                                    if self.finished {
                                        return None;
                                    }
                                    let resp: Resp<T> = match self.method.execute() {
                                        Ok(r) => r,
                                        Err(err) => return Some(Err(err)),
                                    };
                                    if let Some(next_page_token) = resp.next_page_token {
                                        self.method.set_page_token(next_page_token);
                                    } else {
                                        self.finished = true;
                                    }
                                    self.items_iter = resp.items.map(|i| i.into_iter());
                                }
                            }
                        }
                        ItemIter {
                            method: self,
                            finished: false,
                            items_iter: None,
                        }
                    }
                    pub fn iter<T>(
                        &'a mut self,
                    ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                    {
                        crate::PageIter {
                            method: self,
                            finished: false,
                            _phantom: ::std::default::Default::default(),
                        }
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
                    ) -> Result<crate::schemas::ListJobMessagesResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        output.push_str(&self.project_id);
                        output.push_str("/locations/");
                        output.push_str(&self.location);
                        output.push_str("/jobs/");
                        output.push_str(&self.job_id);
                        output.push_str("/messages");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("endTime", &self.end_time)]);
                        let req = req.query(&[("minimumImportance", &self.minimum_importance)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("startTime", &self.start_time)]);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/compute.readonly",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        self._execute()
                    }
                }
            }
            pub mod snapshots {
                pub mod params {}
                pub struct SnapshotsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> SnapshotsActions<'a, A> {
                    #[doc = "Lists snapshots."]
                    pub fn list(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
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
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    job_id: String,
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
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
                    ) -> Result<crate::schemas::ListSnapshotsResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        output.push_str(&self.project_id);
                        output.push_str("/locations/");
                        output.push_str(&self.location);
                        output.push_str("/jobs/");
                        output.push_str(&self.job_id);
                        output.push_str("/snapshots");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/compute.readonly",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod work_items {
                pub mod params {}
                pub struct WorkItemsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> WorkItemsActions<'a, A> {
                    #[doc = "Leases a dataflow WorkItem to run."]
                    pub fn lease(
                        &self,
                        request: crate::schemas::LeaseWorkItemRequest,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> LeaseRequestBuilder<A> {
                        LeaseRequestBuilder {
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
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                        }
                    }
                    #[doc = "Reports the status of dataflow WorkItems leased by a worker."]
                    pub fn report_status(
                        &self,
                        request: crate::schemas::ReportWorkItemStatusRequest,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> ReportStatusRequestBuilder<A> {
                        ReportStatusRequestBuilder {
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
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct LeaseRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::LeaseWorkItemRequest,
                    project_id: String,
                    location: String,
                    job_id: String,
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
                impl<'a, A: yup_oauth2::GetToken> LeaseRequestBuilder<'a, A> {
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
                    ) -> Result<crate::schemas::LeaseWorkItemResponse, Box<dyn ::std::error::Error>>
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
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        output.push_str(&self.project_id);
                        output.push_str("/locations/");
                        output.push_str(&self.location);
                        output.push_str("/jobs/");
                        output.push_str(&self.job_id);
                        output.push_str("/workItems:lease");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ReportStatusRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::ReportWorkItemStatusRequest,
                    project_id: String,
                    location: String,
                    job_id: String,
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
                impl<'a, A: yup_oauth2::GetToken> ReportStatusRequestBuilder<'a, A> {
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
                    ) -> Result<
                        crate::schemas::ReportWorkItemStatusResponse,
                        Box<dyn ::std::error::Error>,
                    > {
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        output.push_str(&self.project_id);
                        output.push_str("/locations/");
                        output.push_str(&self.location);
                        output.push_str("/jobs/");
                        output.push_str(&self.job_id);
                        output.push_str("/workItems:reportStatus");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud-platform",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
        }
        pub mod snapshots {
            pub mod params {}
            pub struct SnapshotsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> SnapshotsActions<'a, A> {
                #[doc = "Deletes a snapshot."]
                pub fn delete(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                    snapshot_id: impl Into<String>,
                ) -> DeleteRequestBuilder<A> {
                    DeleteRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        snapshot_id: snapshot_id.into(),
                    }
                }
                #[doc = "Gets information about a snapshot."]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                    snapshot_id: impl Into<String>,
                ) -> GetRequestBuilder<A> {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        snapshot_id: snapshot_id.into(),
                    }
                }
                #[doc = "Lists snapshots."]
                pub fn list(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        job_id: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                snapshot_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::DeleteSnapshotResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/snapshots/");
                    output.push_str(&self.snapshot_id);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                snapshot_id: String,
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
                ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/snapshots/");
                    output.push_str(&self.snapshot_id);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/compute.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                job_id: Option<String>,
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "If specified, list snapshots created from this job."]
                pub fn job_id(&mut self, value: impl Into<String>) -> &mut Self {
                    self.job_id = Some(value.into());
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
                ) -> Result<crate::schemas::ListSnapshotsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/snapshots");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("jobId", &self.job_id)]);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/compute.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
        }
        pub mod sql {
            pub mod params {}
            pub struct SqlActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> SqlActions<'a, A> {
                #[doc = "Validates a GoogleSQL query for Cloud Dataflow syntax. Will always\nconfirm the given query parses correctly, and if able to look up\nschema information from DataCatalog, will validate that the query\nanalyzes properly as well."]
                pub fn validate(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> ValidateRequestBuilder<A> {
                    ValidateRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        query: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ValidateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                query: Option<String>,
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
            impl<'a, A: yup_oauth2::GetToken> ValidateRequestBuilder<'a, A> {
                #[doc = "The sql query to validate."]
                pub fn query(&mut self, value: impl Into<String>) -> &mut Self {
                    self.query = Some(value.into());
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
                ) -> Result<crate::schemas::ValidateResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/sql:validate");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("query", &self.query)]);
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
        pub mod templates {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetView {}
                impl GetView {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for GetView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetView {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetView {
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
            pub struct TemplatesActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> TemplatesActions<'a, A> {
                #[doc = "Creates a Cloud Dataflow job from a template."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateJobFromTemplateRequest,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                    }
                }
                #[doc = "Get the template associated with a template."]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> GetRequestBuilder<A> {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        gcs_path: None,
                        view: None,
                    }
                }
                #[doc = "Launch a template."]
                pub fn launch(
                    &self,
                    request: crate::schemas::LaunchTemplateParameters,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> LaunchRequestBuilder<A> {
                    LaunchRequestBuilder {
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
                        project_id: project_id.into(),
                        location: location.into(),
                        dynamic_template_gcs_path: None,
                        dynamic_template_staging_location: None,
                        gcs_path: None,
                        validate_only: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::CreateJobFromTemplateRequest,
                project_id: String,
                location: String,
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
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/templates");
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
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                location: String,
                gcs_path: Option<String>,
                view: Option<crate::templates::params::GetView>,
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
                #[doc = "Required. A Cloud Storage path to the template from which to\ncreate the job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
                pub fn gcs_path(&mut self, value: impl Into<String>) -> &mut Self {
                    self.gcs_path = Some(value.into());
                    self
                }
                #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
                pub fn view(&mut self, value: crate::templates::params::GetView) -> &mut Self {
                    self.view = Some(value);
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
                ) -> Result<crate::schemas::GetTemplateResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/templates:get");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("gcsPath", &self.gcs_path)]);
                    let req = req.query(&[("view", &self.view)]);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/compute.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct LaunchRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::LaunchTemplateParameters,
                project_id: String,
                location: String,
                dynamic_template_gcs_path: Option<String>,
                dynamic_template_staging_location: Option<String>,
                gcs_path: Option<String>,
                validate_only: Option<bool>,
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
            impl<'a, A: yup_oauth2::GetToken> LaunchRequestBuilder<'a, A> {
                #[doc = "Path to dynamic template spec file on GCS.\nThe file must be a Json serialized DynamicTemplateFieSpec object."]
                pub fn dynamic_template_gcs_path(&mut self, value: impl Into<String>) -> &mut Self {
                    self.dynamic_template_gcs_path = Some(value.into());
                    self
                }
                #[doc = "Cloud Storage path for staging dependencies.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
                pub fn dynamic_template_staging_location(
                    &mut self,
                    value: impl Into<String>,
                ) -> &mut Self {
                    self.dynamic_template_staging_location = Some(value.into());
                    self
                }
                #[doc = "A Cloud Storage path to the template from which to create\nthe job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
                pub fn gcs_path(&mut self, value: impl Into<String>) -> &mut Self {
                    self.gcs_path = Some(value.into());
                    self
                }
                #[doc = "If true, the request is validated but not actually executed.\nDefaults to false."]
                pub fn validate_only(&mut self, value: bool) -> &mut Self {
                    self.validate_only = Some(value);
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.text()?)
                }
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::LaunchTemplateResponse, Box<dyn ::std::error::Error>>
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    output.push_str(&self.project_id);
                    output.push_str("/locations/");
                    output.push_str(&self.location);
                    output.push_str("/templates:launch");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req =
                        req.query(&[("dynamicTemplate.gcsPath", &self.dynamic_template_gcs_path)]);
                    let req = req.query(&[(
                        "dynamicTemplate.stagingLocation",
                        &self.dynamic_template_staging_location,
                    )]);
                    let req = req.query(&[("gcsPath", &self.gcs_path)]);
                    let req = req.query(&[("validateOnly", &self.validate_only)]);
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
    }
    pub mod snapshots {
        pub mod params {}
        pub struct SnapshotsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> SnapshotsActions<'a, A> {
            #[doc = "Gets information about a snapshot."]
            pub fn get(
                &self,
                project_id: impl Into<String>,
                snapshot_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
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
                    project_id: project_id.into(),
                    snapshot_id: snapshot_id.into(),
                    location: None,
                }
            }
            #[doc = "Lists snapshots."]
            pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
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
                    project_id: project_id.into(),
                    job_id: None,
                    location: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            snapshot_id: String,
            location: Option<String>,
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
            #[doc = "The location that contains this snapshot."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
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
            ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/snapshots/");
                output.push_str(&self.snapshot_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("location", &self.location)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/compute.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            job_id: Option<String>,
            location: Option<String>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "If specified, list snapshots created from this job."]
            pub fn job_id(&mut self, value: impl Into<String>) -> &mut Self {
                self.job_id = Some(value.into());
                self
            }
            #[doc = "The location to list snapshots in."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
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
            ) -> Result<crate::schemas::ListSnapshotsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/snapshots");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("jobId", &self.job_id)]);
                let req = req.query(&[("location", &self.location)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/compute.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod templates {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetView {}
            impl GetView {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for GetView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetView {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetView {
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
        pub struct TemplatesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> TemplatesActions<'a, A> {
            #[doc = "Creates a Cloud Dataflow job from a template."]
            pub fn create(
                &self,
                request: crate::schemas::CreateJobFromTemplateRequest,
                project_id: impl Into<String>,
            ) -> CreateRequestBuilder<A> {
                CreateRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
            #[doc = "Get the template associated with a template."]
            pub fn get(&self, project_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                    project_id: project_id.into(),
                    gcs_path: None,
                    location: None,
                    view: None,
                }
            }
            #[doc = "Launch a template."]
            pub fn launch(
                &self,
                request: crate::schemas::LaunchTemplateParameters,
                project_id: impl Into<String>,
            ) -> LaunchRequestBuilder<A> {
                LaunchRequestBuilder {
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
                    project_id: project_id.into(),
                    dynamic_template_gcs_path: None,
                    dynamic_template_staging_location: None,
                    gcs_path: None,
                    location: None,
                    validate_only: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::CreateJobFromTemplateRequest,
            project_id: String,
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
        impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/templates");
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
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            gcs_path: Option<String>,
            location: Option<String>,
            view: Option<crate::templates::params::GetView>,
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
            #[doc = "Required. A Cloud Storage path to the template from which to\ncreate the job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
            pub fn gcs_path(&mut self, value: impl Into<String>) -> &mut Self {
                self.gcs_path = Some(value.into());
                self
            }
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to\nwhich to direct the request."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
            pub fn view(&mut self, value: crate::templates::params::GetView) -> &mut Self {
                self.view = Some(value);
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
            ) -> Result<crate::schemas::GetTemplateResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/templates:get");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("gcsPath", &self.gcs_path)]);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("view", &self.view)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/compute.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct LaunchRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::LaunchTemplateParameters,
            project_id: String,
            dynamic_template_gcs_path: Option<String>,
            dynamic_template_staging_location: Option<String>,
            gcs_path: Option<String>,
            location: Option<String>,
            validate_only: Option<bool>,
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
        impl<'a, A: yup_oauth2::GetToken> LaunchRequestBuilder<'a, A> {
            #[doc = "Path to dynamic template spec file on GCS.\nThe file must be a Json serialized DynamicTemplateFieSpec object."]
            pub fn dynamic_template_gcs_path(&mut self, value: impl Into<String>) -> &mut Self {
                self.dynamic_template_gcs_path = Some(value.into());
                self
            }
            #[doc = "Cloud Storage path for staging dependencies.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
            pub fn dynamic_template_staging_location(
                &mut self,
                value: impl Into<String>,
            ) -> &mut Self {
                self.dynamic_template_staging_location = Some(value.into());
                self
            }
            #[doc = "A Cloud Storage path to the template from which to create\nthe job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
            pub fn gcs_path(&mut self, value: impl Into<String>) -> &mut Self {
                self.gcs_path = Some(value.into());
                self
            }
            #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to\nwhich to direct the request."]
            pub fn location(&mut self, value: impl Into<String>) -> &mut Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "If true, the request is validated but not actually executed.\nDefaults to false."]
            pub fn validate_only(&mut self, value: bool) -> &mut Self {
                self.validate_only = Some(value);
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::LaunchTemplateResponse, Box<dyn ::std::error::Error>>
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                output.push_str(&self.project_id);
                output.push_str("/templates:launch");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req =
                    req.query(&[("dynamicTemplate.gcsPath", &self.dynamic_template_gcs_path)]);
                let req = req.query(&[(
                    "dynamicTemplate.stagingLocation",
                    &self.dynamic_template_staging_location,
                )]);
                let req = req.query(&[("gcsPath", &self.gcs_path)]);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("validateOnly", &self.validate_only)]);
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
