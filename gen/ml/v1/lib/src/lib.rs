pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiHttpBody {
        #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
        #[serde(rename = "contentType", default)]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The HTTP request/response body as raw binary."]
        #[serde(rename = "data", default)]
        pub data: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Application specific response metadata. Must be set in the first response\nfor streaming APIs."]
        #[serde(rename = "extensions", default)]
        pub extensions:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiHttpBody {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiHttpBody {
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
    pub struct GoogleCloudMlV1AcceleratorConfig {
        #[doc = "The number of accelerators to attach to each machine running the job."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "The type of accelerator to use."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GoogleCloudMlV1AcceleratorConfigType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AcceleratorConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AcceleratorConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1AcceleratorConfigType {
        #[doc = "Unspecified accelerator type. Default to no GPU."]
        AcceleratorTypeUnspecified,
        #[doc = "Nvidia Tesla K80 GPU."]
        NvidiaTeslaK80,
        #[doc = "Nvidia Tesla P100 GPU."]
        NvidiaTeslaP100,
        #[doc = "Nvidia Tesla P4 GPU."]
        NvidiaTeslaP4,
        #[doc = "Nvidia Tesla T4 GPU."]
        NvidiaTeslaT4,
        #[doc = "Nvidia Tesla V100 GPU."]
        NvidiaTeslaV100,
        #[doc = "TPU v2."]
        TpuV2,
        #[doc = "TPU v3."]
        TpuV3,
    }
    impl GoogleCloudMlV1AcceleratorConfigType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1AcceleratorConfigType::AcceleratorTypeUnspecified => {
                    "ACCELERATOR_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaK80 => "NVIDIA_TESLA_K80",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP100 => "NVIDIA_TESLA_P100",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP4 => "NVIDIA_TESLA_P4",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaT4 => "NVIDIA_TESLA_T4",
                GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaV100 => "NVIDIA_TESLA_V100",
                GoogleCloudMlV1AcceleratorConfigType::TpuV2 => "TPU_V2",
                GoogleCloudMlV1AcceleratorConfigType::TpuV3 => "TPU_V3",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1AcceleratorConfigType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1AcceleratorConfigType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1AcceleratorConfigType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCELERATOR_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1AcceleratorConfigType::AcceleratorTypeUnspecified
                }
                "NVIDIA_TESLA_K80" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaK80,
                "NVIDIA_TESLA_P100" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP100,
                "NVIDIA_TESLA_P4" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaP4,
                "NVIDIA_TESLA_T4" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaT4,
                "NVIDIA_TESLA_V100" => GoogleCloudMlV1AcceleratorConfigType::NvidiaTeslaV100,
                "TPU_V2" => GoogleCloudMlV1AcceleratorConfigType::TpuV2,
                "TPU_V3" => GoogleCloudMlV1AcceleratorConfigType::TpuV3,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AcceleratorConfigType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AcceleratorConfigType {
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
    pub struct GoogleCloudMlV1AutoScaling {
        #[doc = "Optional. The minimum number of nodes to allocate for this model. These\nnodes are always up, starting from the time the model is deployed.\nTherefore, the cost of operating this model will be at least\n`rate` * `min_nodes` * number of hours since last billing cycle,\nwhere `rate` is the cost per node-hour as documented in the\n[pricing guide](/ml-engine/docs/pricing),\neven if no predictions are performed. There is additional cost for each\nprediction performed.\n\nUnlike manual scaling, if the load gets too heavy for the nodes\nthat are up, the service will automatically add nodes to handle the\nincreased load as well as scale back as traffic drops, always maintaining\nat least `min_nodes`. You will be charged for the time in which additional\nnodes are used.\n\nIf not specified, `min_nodes` defaults to 0, in which case, when traffic\nto a model stops (and after a cool-down period), nodes will be shut down\nand no charges will be incurred until traffic to the model resumes.\n\nYou can set `min_nodes` when creating the model version, and you can also\nupdate `min_nodes` for an existing version:\n\n<pre>\nupdate_body.json:\n{\n  'autoScaling': {\n    'minNodes': 5\n  }\n}\n</pre>\n\nHTTP request:\n\n<pre>\nPATCH\nhttps://ml.googleapis.com/v1/{name=projects/*/models/*/versions/*}?update_mask=autoScaling.minNodes\n-d @./update_body.json\n</pre>"]
        #[serde(rename = "minNodes", default)]
        pub min_nodes: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1AutoScaling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1AutoScaling {
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
    pub struct GoogleCloudMlV1BuiltInAlgorithmOutput {
        #[doc = "Framework on which the built-in algorithm was trained."]
        #[serde(rename = "framework", default)]
        pub framework: ::std::option::Option<String>,
        #[doc = "The Cloud Storage path to the `model/` directory where the training job\nsaves the trained model. Only set for successful jobs that don't use\nhyperparameter tuning."]
        #[serde(rename = "modelPath", default)]
        pub model_path: ::std::option::Option<String>,
        #[doc = "Python version on which the built-in algorithm was trained."]
        #[serde(rename = "pythonVersion", default)]
        pub python_version: ::std::option::Option<String>,
        #[doc = "AI Platform runtime version on which the built-in algorithm was\ntrained."]
        #[serde(rename = "runtimeVersion", default)]
        pub runtime_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1BuiltInAlgorithmOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1BuiltInAlgorithmOutput {
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
    pub struct GoogleCloudMlV1CancelJobRequest;
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1CancelJobRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CancelJobRequest {
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
    pub struct GoogleCloudMlV1Capability {
        #[doc = "Available accelerators for the capability."]
        #[serde(rename = "availableAccelerators", default)]
        pub available_accelerators: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudMlV1CapabilityAvailableAcceleratorsItems>,
        >,
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GoogleCloudMlV1CapabilityType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Capability {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Capability {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        AcceleratorTypeUnspecified,
        NvidiaTeslaK80,
        NvidiaTeslaP100,
        NvidiaTeslaP4,
        NvidiaTeslaT4,
        NvidiaTeslaV100,
        TpuV2,
        TpuV3,
    }
    impl GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::AcceleratorTypeUnspecified => {
                    "ACCELERATOR_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaK80 => {
                    "NVIDIA_TESLA_K80"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP100 => {
                    "NVIDIA_TESLA_P100"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP4 => {
                    "NVIDIA_TESLA_P4"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaT4 => {
                    "NVIDIA_TESLA_T4"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaV100 => {
                    "NVIDIA_TESLA_V100"
                }
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV2 => "TPU_V2",
                GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV3 => "TPU_V3",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCELERATOR_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::AcceleratorTypeUnspecified
                }
                "NVIDIA_TESLA_K80" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaK80
                }
                "NVIDIA_TESLA_P100" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP100
                }
                "NVIDIA_TESLA_P4" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaP4
                }
                "NVIDIA_TESLA_T4" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaT4
                }
                "NVIDIA_TESLA_V100" => {
                    GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::NvidiaTeslaV100
                }
                "TPU_V2" => GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV2,
                "TPU_V3" => GoogleCloudMlV1CapabilityAvailableAcceleratorsItems::TpuV3,
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
        for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CapabilityAvailableAcceleratorsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1CapabilityType {
        BatchPrediction,
        OnlinePrediction,
        Training,
        TypeUnspecified,
    }
    impl GoogleCloudMlV1CapabilityType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1CapabilityType::BatchPrediction => "BATCH_PREDICTION",
                GoogleCloudMlV1CapabilityType::OnlinePrediction => "ONLINE_PREDICTION",
                GoogleCloudMlV1CapabilityType::Training => "TRAINING",
                GoogleCloudMlV1CapabilityType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1CapabilityType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1CapabilityType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1CapabilityType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BATCH_PREDICTION" => GoogleCloudMlV1CapabilityType::BatchPrediction,
                "ONLINE_PREDICTION" => GoogleCloudMlV1CapabilityType::OnlinePrediction,
                "TRAINING" => GoogleCloudMlV1CapabilityType::Training,
                "TYPE_UNSPECIFIED" => GoogleCloudMlV1CapabilityType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1CapabilityType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1CapabilityType {
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
    pub struct GoogleCloudMlV1Config {
        #[doc = "The service account Cloud ML uses to run on TPU node."]
        #[serde(rename = "tpuServiceAccount", default)]
        pub tpu_service_account: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Config {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Config {
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
    pub struct GoogleCloudMlV1GetConfigResponse {
        #[serde(rename = "config", default)]
        pub config: ::std::option::Option<crate::schemas::GoogleCloudMlV1Config>,
        #[doc = "The service account Cloud ML uses to access resources in the project."]
        #[serde(rename = "serviceAccount", default)]
        pub service_account: ::std::option::Option<String>,
        #[doc = "The project number for `service_account`."]
        #[serde(rename = "serviceAccountProject", default)]
        #[serde(with = "crate::parsed_string")]
        pub service_account_project: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1GetConfigResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1GetConfigResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1HyperparameterOutput {
        #[doc = "All recorded object metrics for this trial. This field is not currently\npopulated."]
        #[serde(rename = "allMetrics", default)]
        pub all_metrics: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudMlV1HyperparameterOutputHyperparameterMetric>,
        >,
        #[doc = "Details related to built-in algorithms jobs.\nOnly set for trials of built-in algorithms jobs that have succeeded."]
        #[serde(rename = "builtInAlgorithmOutput", default)]
        pub built_in_algorithm_output:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1BuiltInAlgorithmOutput>,
        #[doc = "Output only. End time for the trial."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The final objective metric seen for this trial."]
        #[serde(rename = "finalMetric", default)]
        pub final_metric: ::std::option::Option<
            crate::schemas::GoogleCloudMlV1HyperparameterOutputHyperparameterMetric,
        >,
        #[doc = "The hyperparameters given to this trial."]
        #[serde(rename = "hyperparameters", default)]
        pub hyperparameters: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "True if the trial is stopped early."]
        #[serde(rename = "isTrialStoppedEarly", default)]
        pub is_trial_stopped_early: ::std::option::Option<bool>,
        #[doc = "Output only. Start time for the trial."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Output only. The detailed state of the trial."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterOutputState>,
        #[doc = "The trial id for these results."]
        #[serde(rename = "trialId", default)]
        pub trial_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterOutput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1HyperparameterOutputState {
        #[doc = "The job has been cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelled,
        #[doc = "The job is being cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelling,
        #[doc = "The job failed.\n`error_message` should contain the details of the failure."]
        Failed,
        #[doc = "The service is preparing to run the job."]
        Preparing,
        #[doc = "The job has been just created and processing has not yet begun."]
        Queued,
        #[doc = "The job is in progress."]
        Running,
        #[doc = "The job state is unspecified."]
        StateUnspecified,
        #[doc = "The job completed successfully."]
        Succeeded,
    }
    impl GoogleCloudMlV1HyperparameterOutputState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1HyperparameterOutputState::Cancelled => "CANCELLED",
                GoogleCloudMlV1HyperparameterOutputState::Cancelling => "CANCELLING",
                GoogleCloudMlV1HyperparameterOutputState::Failed => "FAILED",
                GoogleCloudMlV1HyperparameterOutputState::Preparing => "PREPARING",
                GoogleCloudMlV1HyperparameterOutputState::Queued => "QUEUED",
                GoogleCloudMlV1HyperparameterOutputState::Running => "RUNNING",
                GoogleCloudMlV1HyperparameterOutputState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleCloudMlV1HyperparameterOutputState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1HyperparameterOutputState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1HyperparameterOutputState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1HyperparameterOutputState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudMlV1HyperparameterOutputState::Cancelled,
                "CANCELLING" => GoogleCloudMlV1HyperparameterOutputState::Cancelling,
                "FAILED" => GoogleCloudMlV1HyperparameterOutputState::Failed,
                "PREPARING" => GoogleCloudMlV1HyperparameterOutputState::Preparing,
                "QUEUED" => GoogleCloudMlV1HyperparameterOutputState::Queued,
                "RUNNING" => GoogleCloudMlV1HyperparameterOutputState::Running,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1HyperparameterOutputState::StateUnspecified,
                "SUCCEEDED" => GoogleCloudMlV1HyperparameterOutputState::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterOutputState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterOutputState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1HyperparameterOutputHyperparameterMetric {
        #[doc = "The objective value at this training step."]
        #[serde(rename = "objectiveValue", default)]
        pub objective_value: ::std::option::Option<f64>,
        #[doc = "The global training step for this metric."]
        #[serde(rename = "trainingStep", default)]
        #[serde(with = "crate::parsed_string")]
        pub training_step: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudMlV1HyperparameterOutputHyperparameterMetric
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudMlV1HyperparameterOutputHyperparameterMetric
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1HyperparameterSpec {
        #[doc = "Optional. The search algorithm specified for the hyperparameter\ntuning job.\nUses the default AI Platform hyperparameter tuning\nalgorithm if unspecified."]
        #[serde(rename = "algorithm", default)]
        pub algorithm:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterSpecAlgorithm>,
        #[doc = "Optional. Indicates if the hyperparameter tuning job enables auto trial\nearly stopping."]
        #[serde(rename = "enableTrialEarlyStopping", default)]
        pub enable_trial_early_stopping: ::std::option::Option<bool>,
        #[doc = "Required. The type of goal to use for tuning. Available types are\n`MAXIMIZE` and `MINIMIZE`.\n\nDefaults to `MAXIMIZE`."]
        #[serde(rename = "goal", default)]
        pub goal: ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterSpecGoal>,
        #[doc = "Optional. The TensorFlow summary tag name to use for optimizing trials. For\ncurrent versions of TensorFlow, this tag name should exactly match what is\nshown in TensorBoard, including all scopes.  For versions of TensorFlow\nprior to 0.12, this should be only the tag passed to tf.Summary.\nBy default, \"training/hptuning/metric\" will be used."]
        #[serde(rename = "hyperparameterMetricTag", default)]
        pub hyperparameter_metric_tag: ::std::option::Option<String>,
        #[doc = "Optional. The number of failed trials that need to be seen before failing\nthe hyperparameter tuning job. You can specify this field to override the\ndefault failing criteria for AI Platform hyperparameter tuning jobs.\n\nDefaults to zero, which means the service decides when a hyperparameter\njob should fail."]
        #[serde(rename = "maxFailedTrials", default)]
        pub max_failed_trials: ::std::option::Option<i32>,
        #[doc = "Optional. The number of training trials to run concurrently.\nYou can reduce the time it takes to perform hyperparameter tuning by adding\ntrials in parallel. However, each trail only benefits from the information\ngained in completed trials. That means that a trial does not get access to\nthe results of trials running at the same time, which could reduce the\nquality of the overall optimization.\n\nEach trial will use the same scale tier and machine types.\n\nDefaults to one."]
        #[serde(rename = "maxParallelTrials", default)]
        pub max_parallel_trials: ::std::option::Option<i32>,
        #[doc = "Optional. How many training trials should be attempted to optimize\nthe specified hyperparameters.\n\nDefaults to one."]
        #[serde(rename = "maxTrials", default)]
        pub max_trials: ::std::option::Option<i32>,
        #[doc = "Required. The set of parameters to tune."]
        #[serde(rename = "params", default)]
        pub params: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1ParameterSpec>>,
        #[doc = "Optional. The prior hyperparameter tuning job id that users hope to\ncontinue with. The job id will be used to find the corresponding vizier\nstudy guid and resume the study."]
        #[serde(rename = "resumePreviousJobId", default)]
        pub resume_previous_job_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1HyperparameterSpecAlgorithm {
        #[doc = "The default algorithm used by the hyperparameter tuning service. This is\na Bayesian optimization algorithm."]
        AlgorithmUnspecified,
        #[doc = "Simple grid search within the feasible space. To use grid search,\nall parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`."]
        GridSearch,
        #[doc = "Simple random search within the feasible space."]
        RandomSearch,
    }
    impl GoogleCloudMlV1HyperparameterSpecAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1HyperparameterSpecAlgorithm::AlgorithmUnspecified => {
                    "ALGORITHM_UNSPECIFIED"
                }
                GoogleCloudMlV1HyperparameterSpecAlgorithm::GridSearch => "GRID_SEARCH",
                GoogleCloudMlV1HyperparameterSpecAlgorithm::RandomSearch => "RANDOM_SEARCH",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALGORITHM_UNSPECIFIED" => {
                    GoogleCloudMlV1HyperparameterSpecAlgorithm::AlgorithmUnspecified
                }
                "GRID_SEARCH" => GoogleCloudMlV1HyperparameterSpecAlgorithm::GridSearch,
                "RANDOM_SEARCH" => GoogleCloudMlV1HyperparameterSpecAlgorithm::RandomSearch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterSpecAlgorithm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1HyperparameterSpecGoal {
        #[doc = "Goal Type will default to maximize."]
        GoalTypeUnspecified,
        #[doc = "Maximize the goal metric."]
        Maximize,
        #[doc = "Minimize the goal metric."]
        Minimize,
    }
    impl GoogleCloudMlV1HyperparameterSpecGoal {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1HyperparameterSpecGoal::GoalTypeUnspecified => {
                    "GOAL_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1HyperparameterSpecGoal::Maximize => "MAXIMIZE",
                GoogleCloudMlV1HyperparameterSpecGoal::Minimize => "MINIMIZE",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1HyperparameterSpecGoal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1HyperparameterSpecGoal {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1HyperparameterSpecGoal {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GOAL_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1HyperparameterSpecGoal::GoalTypeUnspecified
                }
                "MAXIMIZE" => GoogleCloudMlV1HyperparameterSpecGoal::Maximize,
                "MINIMIZE" => GoogleCloudMlV1HyperparameterSpecGoal::Minimize,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1HyperparameterSpecGoal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1HyperparameterSpecGoal {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Job {
        #[doc = "Output only. When the job was created."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. When the job processing was completed."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. The details of a failure or a cancellation."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a job from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform job updates in order to avoid race\nconditions: An `etag` is returned in the response to `GetJob`, and\nsystems are expected to put that etag in the request to `UpdateJob` to\nensure that their change will be applied to the same version of the job."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Required. The user-specified id of the job."]
        #[serde(rename = "jobId", default)]
        pub job_id: ::std::option::Option<String>,
        #[doc = "Optional. One or more labels that you can add, to organize your jobs.\nEach label is a key-value pair, where both the key and the value are\narbitrary strings that you supply.\nFor more information, see the documentation on\n<a href=\"/ml-engine/docs/tensorflow/resource-labels\">using labels</a>."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Input parameters to create a prediction job."]
        #[serde(rename = "predictionInput", default)]
        pub prediction_input: ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionInput>,
        #[doc = "The current prediction job result."]
        #[serde(rename = "predictionOutput", default)]
        pub prediction_output:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionOutput>,
        #[doc = "Output only. When the job processing was started."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Output only. The detailed state of a job."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1JobState>,
        #[doc = "Input parameters to create a training job."]
        #[serde(rename = "trainingInput", default)]
        pub training_input: ::std::option::Option<crate::schemas::GoogleCloudMlV1TrainingInput>,
        #[doc = "The current training job result."]
        #[serde(rename = "trainingOutput", default)]
        pub training_output: ::std::option::Option<crate::schemas::GoogleCloudMlV1TrainingOutput>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Job {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Job {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1JobState {
        #[doc = "The job has been cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelled,
        #[doc = "The job is being cancelled.\n`error_message` should describe the reason for the cancellation."]
        Cancelling,
        #[doc = "The job failed.\n`error_message` should contain the details of the failure."]
        Failed,
        #[doc = "The service is preparing to run the job."]
        Preparing,
        #[doc = "The job has been just created and processing has not yet begun."]
        Queued,
        #[doc = "The job is in progress."]
        Running,
        #[doc = "The job state is unspecified."]
        StateUnspecified,
        #[doc = "The job completed successfully."]
        Succeeded,
    }
    impl GoogleCloudMlV1JobState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1JobState::Cancelled => "CANCELLED",
                GoogleCloudMlV1JobState::Cancelling => "CANCELLING",
                GoogleCloudMlV1JobState::Failed => "FAILED",
                GoogleCloudMlV1JobState::Preparing => "PREPARING",
                GoogleCloudMlV1JobState::Queued => "QUEUED",
                GoogleCloudMlV1JobState::Running => "RUNNING",
                GoogleCloudMlV1JobState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleCloudMlV1JobState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1JobState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1JobState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1JobState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleCloudMlV1JobState::Cancelled,
                "CANCELLING" => GoogleCloudMlV1JobState::Cancelling,
                "FAILED" => GoogleCloudMlV1JobState::Failed,
                "PREPARING" => GoogleCloudMlV1JobState::Preparing,
                "QUEUED" => GoogleCloudMlV1JobState::Queued,
                "RUNNING" => GoogleCloudMlV1JobState::Running,
                "STATE_UNSPECIFIED" => GoogleCloudMlV1JobState::StateUnspecified,
                "SUCCEEDED" => GoogleCloudMlV1JobState::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1JobState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1JobState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListJobsResponse {
        #[doc = "The list of jobs."]
        #[serde(rename = "jobs", default)]
        pub jobs: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Job>>,
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListJobsResponse {
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
    pub struct GoogleCloudMlV1ListLocationsResponse {
        #[doc = "Locations where at least one type of CMLE capability is available."]
        #[serde(rename = "locations", default)]
        pub locations: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Location>>,
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListLocationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListLocationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListModelsResponse {
        #[doc = "The list of models."]
        #[serde(rename = "models", default)]
        pub models: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Model>>,
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListModelsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListModelsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ListVersionsResponse {
        #[doc = "Optional. Pass this token as the `page_token` field of the request for a\nsubsequent call."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of versions."]
        #[serde(rename = "versions", default)]
        pub versions: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Version>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ListVersionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ListVersionsResponse {
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
    pub struct GoogleCloudMlV1Location {
        #[doc = "Capabilities available in the location."]
        #[serde(rename = "capabilities", default)]
        pub capabilities: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1Capability>>,
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Location {
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
    pub struct GoogleCloudMlV1ManualScaling {
        #[doc = "The number of nodes to allocate for this model. These nodes are always up,\nstarting from the time the model is deployed, so the cost of operating\nthis model will be proportional to `nodes` * number of hours since\nlast billing cycle plus the cost for each prediction performed."]
        #[serde(rename = "nodes", default)]
        pub nodes: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ManualScaling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ManualScaling {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Model {
        #[doc = "Output only. The default version of the model. This version will be used to\nhandle prediction requests that do not specify a version.\n\nYou can change the default version by calling\n[projects.methods.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault)."]
        #[serde(rename = "defaultVersion", default)]
        pub default_version: ::std::option::Option<crate::schemas::GoogleCloudMlV1Version>,
        #[doc = "Optional. The description specified for the model when it was created."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a model from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform model updates in order to avoid race\nconditions: An `etag` is returned in the response to `GetModel`, and\nsystems are expected to put that etag in the request to `UpdateModel` to\nensure that their change will be applied to the model as intended."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Optional. One or more labels that you can add, to organize your models.\nEach label is a key-value pair, where both the key and the value are\narbitrary strings that you supply.\nFor more information, see the documentation on\n<a href=\"/ml-engine/docs/tensorflow/resource-labels\">using labels</a>."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Required. The name specified for the model when it was created.\n\nThe model name must be unique within the project it is created in."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. If true, online prediction nodes send `stderr` and `stdout`\nstreams to Stackdriver Logging. These can be more verbose than the standard\naccess logs (see `onlinePredictionLogging`) and can incur higher cost.\nHowever, they are helpful for debugging. Note that\n[Stackdriver logs may incur a cost](/stackdriver/pricing), especially if\nyour project receives prediction requests at a high QPS. Estimate your\ncosts before enabling this option.\n\nDefault is false."]
        #[serde(rename = "onlinePredictionConsoleLogging", default)]
        pub online_prediction_console_logging: ::std::option::Option<bool>,
        #[doc = "Optional. If true, online prediction access logs are sent to StackDriver\nLogging. These logs are like standard server access logs, containing\ninformation like timestamp and latency for each request. Note that\n[Stackdriver logs may incur a cost](/stackdriver/pricing), especially if\nyour project receives prediction requests at a high queries per second rate\n(QPS). Estimate your costs before enabling this option.\n\nDefault is false."]
        #[serde(rename = "onlinePredictionLogging", default)]
        pub online_prediction_logging: ::std::option::Option<bool>,
        #[doc = "Optional. The list of regions where the model is going to be deployed.\nCurrently only one region per model is supported.\nDefaults to 'us-central1' if nothing is set.\nSee the <a href=\"/ml-engine/docs/tensorflow/regions\">available regions</a>\nfor AI Platform services.\nNote:\n\n* No matter where a model is deployed, it can always be accessed by\n  users from anywhere, both for online and batch prediction.\n* The region for a batch prediction job is set by the region field when\n  submitting the batch prediction job and does not take its value from\n  this field."]
        #[serde(rename = "regions", default)]
        pub regions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Model {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Model {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1OperationMetadata {
        #[doc = "The time the operation was submitted."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The time operation processing completed."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Indicates whether a request to cancel this operation has been made."]
        #[serde(rename = "isCancellationRequested", default)]
        pub is_cancellation_requested: ::std::option::Option<bool>,
        #[doc = "The user labels, inherited from the model or the model version being\noperated on."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Contains the name of the model associated with the operation."]
        #[serde(rename = "modelName", default)]
        pub model_name: ::std::option::Option<String>,
        #[doc = "The operation type."]
        #[serde(rename = "operationType", default)]
        pub operation_type:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1OperationMetadataOperationType>,
        #[doc = "Contains the project number associated with the operation."]
        #[serde(rename = "projectNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub project_number: ::std::option::Option<i64>,
        #[doc = "The time operation processing started."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Contains the version associated with the operation."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<crate::schemas::GoogleCloudMlV1Version>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1OperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1OperationMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1OperationMetadataOperationType {
        #[doc = "An operation to create a new version."]
        CreateVersion,
        #[doc = "An operation to delete an existing model."]
        DeleteModel,
        #[doc = "An operation to delete an existing version."]
        DeleteVersion,
        #[doc = "Unspecified operation type."]
        OperationTypeUnspecified,
        #[doc = "An operation to update project configuration."]
        UpdateConfig,
        #[doc = "An operation to update an existing model."]
        UpdateModel,
        #[doc = "An operation to update an existing version."]
        UpdateVersion,
    }
    impl GoogleCloudMlV1OperationMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1OperationMetadataOperationType::CreateVersion => "CREATE_VERSION",
                GoogleCloudMlV1OperationMetadataOperationType::DeleteModel => "DELETE_MODEL",
                GoogleCloudMlV1OperationMetadataOperationType::DeleteVersion => "DELETE_VERSION",
                GoogleCloudMlV1OperationMetadataOperationType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
                GoogleCloudMlV1OperationMetadataOperationType::UpdateConfig => "UPDATE_CONFIG",
                GoogleCloudMlV1OperationMetadataOperationType::UpdateModel => "UPDATE_MODEL",
                GoogleCloudMlV1OperationMetadataOperationType::UpdateVersion => "UPDATE_VERSION",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1OperationMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1OperationMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1OperationMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::CreateVersion,
                "DELETE_MODEL" => GoogleCloudMlV1OperationMetadataOperationType::DeleteModel,
                "DELETE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::DeleteVersion,
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1OperationMetadataOperationType::OperationTypeUnspecified
                }
                "UPDATE_CONFIG" => GoogleCloudMlV1OperationMetadataOperationType::UpdateConfig,
                "UPDATE_MODEL" => GoogleCloudMlV1OperationMetadataOperationType::UpdateModel,
                "UPDATE_VERSION" => GoogleCloudMlV1OperationMetadataOperationType::UpdateVersion,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1OperationMetadataOperationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1OperationMetadataOperationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1ParameterSpec {
        #[doc = "Required if type is `CATEGORICAL`. The list of possible categories."]
        #[serde(rename = "categoricalValues", default)]
        pub categorical_values: ::std::option::Option<Vec<String>>,
        #[doc = "Required if type is `DISCRETE`.\nA list of feasible points.\nThe list should be in strictly increasing order. For instance, this\nparameter might have possible settings of 1.5, 2.5, and 4.0. This list\nshould not contain more than 1,000 values."]
        #[serde(rename = "discreteValues", default)]
        pub discrete_values: ::std::option::Option<Vec<f64>>,
        #[doc = "Required if type is `DOUBLE` or `INTEGER`. This field\nshould be unset if type is `CATEGORICAL`. This value should be integers if\ntype is `INTEGER`."]
        #[serde(rename = "maxValue", default)]
        pub max_value: ::std::option::Option<f64>,
        #[doc = "Required if type is `DOUBLE` or `INTEGER`. This field\nshould be unset if type is `CATEGORICAL`. This value should be integers if\ntype is INTEGER."]
        #[serde(rename = "minValue", default)]
        pub min_value: ::std::option::Option<f64>,
        #[doc = "Required. The parameter name must be unique amongst all ParameterConfigs in\na HyperparameterSpec message. E.g., \"learning_rate\"."]
        #[serde(rename = "parameterName", default)]
        pub parameter_name: ::std::option::Option<String>,
        #[doc = "Required. The type of the parameter."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GoogleCloudMlV1ParameterSpecType>,
        #[doc = "Optional. How the parameter should be scaled to the hypercube.\nLeave unset for categorical parameters.\nSome kind of scaling is strongly recommended for real or integral\nparameters (e.g., `UNIT_LINEAR_SCALE`)."]
        #[serde(rename = "scaleType", default)]
        pub scale_type:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1ParameterSpecScaleType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ParameterSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ParameterSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1ParameterSpecType {
        #[doc = "The parameter is categorical, with a value chosen from the categories\nfield."]
        Categorical,
        #[doc = "The parameter is real valued, with a fixed set of feasible points. If\n`type==DISCRETE`, feasible_points must be provided, and\n{`min_value`, `max_value`} will be ignored."]
        Discrete,
        #[doc = "Type for real-valued parameters."]
        Double,
        #[doc = "Type for integral parameters."]
        Integer,
        #[doc = "You must specify a valid type. Using this unspecified type will result in\nan error."]
        ParameterTypeUnspecified,
    }
    impl GoogleCloudMlV1ParameterSpecType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1ParameterSpecType::Categorical => "CATEGORICAL",
                GoogleCloudMlV1ParameterSpecType::Discrete => "DISCRETE",
                GoogleCloudMlV1ParameterSpecType::Double => "DOUBLE",
                GoogleCloudMlV1ParameterSpecType::Integer => "INTEGER",
                GoogleCloudMlV1ParameterSpecType::ParameterTypeUnspecified => {
                    "PARAMETER_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1ParameterSpecType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1ParameterSpecType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1ParameterSpecType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CATEGORICAL" => GoogleCloudMlV1ParameterSpecType::Categorical,
                "DISCRETE" => GoogleCloudMlV1ParameterSpecType::Discrete,
                "DOUBLE" => GoogleCloudMlV1ParameterSpecType::Double,
                "INTEGER" => GoogleCloudMlV1ParameterSpecType::Integer,
                "PARAMETER_TYPE_UNSPECIFIED" => {
                    GoogleCloudMlV1ParameterSpecType::ParameterTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ParameterSpecType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ParameterSpecType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1ParameterSpecScaleType {
        #[doc = "By default, no scaling is applied."]
        None,
        #[doc = "Scales the feasible space to (0, 1) linearly."]
        UnitLinearScale,
        #[doc = "Scales the feasible space logarithmically to (0, 1). The entire feasible\nspace must be strictly positive."]
        UnitLogScale,
        #[doc = "Scales the feasible space \"reverse\" logarithmically to (0, 1). The result\nis that values close to the top of the feasible space are spread out more\nthan points near the bottom. The entire feasible space must be strictly\npositive."]
        UnitReverseLogScale,
    }
    impl GoogleCloudMlV1ParameterSpecScaleType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1ParameterSpecScaleType::None => "NONE",
                GoogleCloudMlV1ParameterSpecScaleType::UnitLinearScale => "UNIT_LINEAR_SCALE",
                GoogleCloudMlV1ParameterSpecScaleType::UnitLogScale => "UNIT_LOG_SCALE",
                GoogleCloudMlV1ParameterSpecScaleType::UnitReverseLogScale => {
                    "UNIT_REVERSE_LOG_SCALE"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1ParameterSpecScaleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1ParameterSpecScaleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1ParameterSpecScaleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => GoogleCloudMlV1ParameterSpecScaleType::None,
                "UNIT_LINEAR_SCALE" => GoogleCloudMlV1ParameterSpecScaleType::UnitLinearScale,
                "UNIT_LOG_SCALE" => GoogleCloudMlV1ParameterSpecScaleType::UnitLogScale,
                "UNIT_REVERSE_LOG_SCALE" => {
                    GoogleCloudMlV1ParameterSpecScaleType::UnitReverseLogScale
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
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ParameterSpecScaleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ParameterSpecScaleType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudMlV1PredictRequest {
        #[doc = "Required. The prediction request body."]
        #[serde(rename = "httpBody", default)]
        pub http_body: ::std::option::Option<crate::schemas::GoogleApiHttpBody>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictRequest {
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
    pub struct GoogleCloudMlV1PredictionInput {
        #[doc = "Optional. Number of records per batch, defaults to 64.\nThe service will buffer batch_size number of records in memory before\ninvoking one Tensorflow prediction call internally. So take the record\nsize and memory available into consideration when setting this parameter."]
        #[serde(rename = "batchSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub batch_size: ::std::option::Option<i64>,
        #[doc = "Required. The format of the input data files."]
        #[serde(rename = "dataFormat", default)]
        pub data_format:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionInputDataFormat>,
        #[doc = "Required. The Cloud Storage location of the input data files. May contain\n<a href=\"/storage/docs/gsutil/addlhelp/WildcardNames\">wildcards</a>."]
        #[serde(rename = "inputPaths", default)]
        pub input_paths: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The maximum number of workers to be used for parallel processing.\nDefaults to 10 if not specified."]
        #[serde(rename = "maxWorkerCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub max_worker_count: ::std::option::Option<i64>,
        #[doc = "Use this field if you want to use the default version for the specified\nmodel. The string must use the following format:\n\n`\"projects/YOUR_PROJECT/models/YOUR_MODEL\"`"]
        #[serde(rename = "modelName", default)]
        pub model_name: ::std::option::Option<String>,
        #[doc = "Optional. Format of the output data files, defaults to JSON."]
        #[serde(rename = "outputDataFormat", default)]
        pub output_data_format:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1PredictionInputOutputDataFormat>,
        #[doc = "Required. The output Google Cloud Storage location."]
        #[serde(rename = "outputPath", default)]
        pub output_path: ::std::option::Option<String>,
        #[doc = "Required. The Google Compute Engine region to run the prediction job in.\nSee the <a href=\"/ml-engine/docs/tensorflow/regions\">available regions</a>\nfor AI Platform services."]
        #[serde(rename = "region", default)]
        pub region: ::std::option::Option<String>,
        #[doc = "Optional. The AI Platform runtime version to use for this batch\nprediction. If not set, AI Platform will pick the runtime version used\nduring the CreateVersion request for this model version, or choose the\nlatest stable version when model version information is not available\nsuch as when the model is specified by uri."]
        #[serde(rename = "runtimeVersion", default)]
        pub runtime_version: ::std::option::Option<String>,
        #[doc = "Optional. The name of the signature defined in the SavedModel to use for\nthis job. Please refer to\n[SavedModel](https://tensorflow.github.io/serving/serving_basic.html)\nfor information about how to use signatures.\n\nDefaults to\n[DEFAULT_SERVING_SIGNATURE_DEF_KEY](https://www.tensorflow.org/api_docs/python/tf/saved_model/signature_constants)\n, which is \"serving_default\"."]
        #[serde(rename = "signatureName", default)]
        pub signature_name: ::std::option::Option<String>,
        #[doc = "Use this field if you want to specify a Google Cloud Storage path for\nthe model to use."]
        #[serde(rename = "uri", default)]
        pub uri: ::std::option::Option<String>,
        #[doc = "Use this field if you want to specify a version of the model to use. The\nstring is formatted the same way as `model_version`, with the addition\nof the version information:\n\n`\"projects/YOUR_PROJECT/models/YOUR_MODEL/versions/YOUR_VERSION\"`"]
        #[serde(rename = "versionName", default)]
        pub version_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1PredictionInputDataFormat {
        #[doc = "Values are comma-separated rows, with keys in a separate file.\nCurrently available only for output data."]
        Csv,
        #[doc = "Unspecified format."]
        DataFormatUnspecified,
        #[doc = "Each line of the file is a JSON dictionary representing one record."]
        Json,
        #[doc = "Deprecated. Use JSON instead."]
        Text,
        #[doc = "The source file is a TFRecord file.\nCurrently available only for input data."]
        TfRecord,
        #[doc = "The source file is a GZIP-compressed TFRecord file.\nCurrently available only for input data."]
        TfRecordGzip,
    }
    impl GoogleCloudMlV1PredictionInputDataFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1PredictionInputDataFormat::Csv => "CSV",
                GoogleCloudMlV1PredictionInputDataFormat::DataFormatUnspecified => {
                    "DATA_FORMAT_UNSPECIFIED"
                }
                GoogleCloudMlV1PredictionInputDataFormat::Json => "JSON",
                GoogleCloudMlV1PredictionInputDataFormat::Text => "TEXT",
                GoogleCloudMlV1PredictionInputDataFormat::TfRecord => "TF_RECORD",
                GoogleCloudMlV1PredictionInputDataFormat::TfRecordGzip => "TF_RECORD_GZIP",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1PredictionInputDataFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1PredictionInputDataFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1PredictionInputDataFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => GoogleCloudMlV1PredictionInputDataFormat::Csv,
                "DATA_FORMAT_UNSPECIFIED" => {
                    GoogleCloudMlV1PredictionInputDataFormat::DataFormatUnspecified
                }
                "JSON" => GoogleCloudMlV1PredictionInputDataFormat::Json,
                "TEXT" => GoogleCloudMlV1PredictionInputDataFormat::Text,
                "TF_RECORD" => GoogleCloudMlV1PredictionInputDataFormat::TfRecord,
                "TF_RECORD_GZIP" => GoogleCloudMlV1PredictionInputDataFormat::TfRecordGzip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionInputDataFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionInputDataFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1PredictionInputOutputDataFormat {
        #[doc = "Values are comma-separated rows, with keys in a separate file.\nCurrently available only for output data."]
        Csv,
        #[doc = "Unspecified format."]
        DataFormatUnspecified,
        #[doc = "Each line of the file is a JSON dictionary representing one record."]
        Json,
        #[doc = "Deprecated. Use JSON instead."]
        Text,
        #[doc = "The source file is a TFRecord file.\nCurrently available only for input data."]
        TfRecord,
        #[doc = "The source file is a GZIP-compressed TFRecord file.\nCurrently available only for input data."]
        TfRecordGzip,
    }
    impl GoogleCloudMlV1PredictionInputOutputDataFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1PredictionInputOutputDataFormat::Csv => "CSV",
                GoogleCloudMlV1PredictionInputOutputDataFormat::DataFormatUnspecified => {
                    "DATA_FORMAT_UNSPECIFIED"
                }
                GoogleCloudMlV1PredictionInputOutputDataFormat::Json => "JSON",
                GoogleCloudMlV1PredictionInputOutputDataFormat::Text => "TEXT",
                GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecord => "TF_RECORD",
                GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecordGzip => "TF_RECORD_GZIP",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => GoogleCloudMlV1PredictionInputOutputDataFormat::Csv,
                "DATA_FORMAT_UNSPECIFIED" => {
                    GoogleCloudMlV1PredictionInputOutputDataFormat::DataFormatUnspecified
                }
                "JSON" => GoogleCloudMlV1PredictionInputOutputDataFormat::Json,
                "TEXT" => GoogleCloudMlV1PredictionInputOutputDataFormat::Text,
                "TF_RECORD" => GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecord,
                "TF_RECORD_GZIP" => GoogleCloudMlV1PredictionInputOutputDataFormat::TfRecordGzip,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionInputOutputDataFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1PredictionOutput {
        #[doc = "The number of data instances which resulted in errors."]
        #[serde(rename = "errorCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub error_count: ::std::option::Option<i64>,
        #[doc = "Node hours used by the batch prediction job."]
        #[serde(rename = "nodeHours", default)]
        pub node_hours: ::std::option::Option<f64>,
        #[doc = "The output Google Cloud Storage location provided at the job creation time."]
        #[serde(rename = "outputPath", default)]
        pub output_path: ::std::option::Option<String>,
        #[doc = "The number of generated predictions."]
        #[serde(rename = "predictionCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub prediction_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1PredictionOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1PredictionOutput {
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
    pub struct GoogleCloudMlV1ReplicaConfig {
        #[doc = "Represents the type and number of accelerators used by the replica.\n[Learn about restrictions on accelerator configurations for\ntraining.](/ml-engine/docs/tensorflow/using-gpus#compute-engine-machine-types-with-gpu)"]
        #[serde(rename = "acceleratorConfig", default)]
        pub accelerator_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1AcceleratorConfig>,
        #[doc = "The Docker image to run on the replica. This image must be in Container\nRegistry. Learn more about [configuring custom\ncontainers](/ml-engine/docs/distributed-training-containers)."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "The AI Platform runtime version that includes a TensorFlow version matching\nthe one used in the custom container. This field is required if the replica\nis a TPU worker that uses a custom container. Otherwise, do not specify\nthis field. This must be a [runtime version that currently supports\ntraining with\nTPUs](/ml-engine/docs/tensorflow/runtime-version-list#tpu-support).\n\nNote that the version of TensorFlow included in a runtime version may\ndiffer from the numbering of the runtime version itself, because it may\nhave a different [patch\nversion](https://www.tensorflow.org/guide/version_compat#semantic_versioning_20).\nIn this field, you must specify the runtime version (TensorFlow minor\nversion). For example, if your custom container runs TensorFlow `1.x.y`,\nspecify `1.x`."]
        #[serde(rename = "tpuTfVersion", default)]
        pub tpu_tf_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1ReplicaConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1ReplicaConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1RequestLoggingConfig {
        #[doc = "Fully qualified BigQuery table name in the format of\n\"[project_id].[dataset_name].[table_name]\"."]
        #[serde(rename = "bigqueryTableName", default)]
        pub bigquery_table_name: ::std::option::Option<String>,
        #[doc = "Percentage of the request being logged. The sampling window is the lifetime\nof the Version. Defaults to 0."]
        #[serde(rename = "samplingPercentage", default)]
        pub sampling_percentage: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1RequestLoggingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1RequestLoggingConfig {
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
    pub struct GoogleCloudMlV1SetDefaultVersionRequest;
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1SetDefaultVersionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1SetDefaultVersionRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1TrainingInput {
        #[doc = "Optional. Command line arguments to pass to the program."]
        #[serde(rename = "args", default)]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The set of Hyperparameters to tune."]
        #[serde(rename = "hyperparameters", default)]
        pub hyperparameters:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1HyperparameterSpec>,
        #[doc = "Optional. A Google Cloud Storage path in which to store training outputs\nand other data needed for training. This path is passed to your TensorFlow\nprogram as the '--job-dir' command-line argument. The benefit of specifying\nthis field is that Cloud ML validates the path for use in training."]
        #[serde(rename = "jobDir", default)]
        pub job_dir: ::std::option::Option<String>,
        #[doc = "Optional. The configuration for your master worker.\n\nYou should only set `masterConfig.acceleratorConfig` if `masterType` is set\nto a Compute Engine machine type. Learn about [restrictions on accelerator\nconfigurations for\ntraining.](/ml-engine/docs/tensorflow/using-gpus#compute-engine-machine-types-with-gpu)\n\nSet `masterConfig.imageUri` only if you build a custom image. Only one of\n`masterConfig.imageUri` and `runtimeVersion` should be set. Learn more about\n[configuring custom\ncontainers](/ml-engine/docs/distributed-training-containers)."]
        #[serde(rename = "masterConfig", default)]
        pub master_config: ::std::option::Option<crate::schemas::GoogleCloudMlV1ReplicaConfig>,
        #[doc = "Optional. Specifies the type of virtual machine to use for your training\njob's master worker.\n\nThe following types are supported:\n\n<dl>\n  <dt>standard</dt>\n  <dd>\n  A basic machine configuration suitable for training simple models with\n  small to moderate datasets.\n  </dd>\n  <dt>large_model</dt>\n  <dd>\n  A machine with a lot of memory, specially suited for parameter servers\n  when your model is large (having many hidden layers or layers with very\n  large numbers of nodes).\n  </dd>\n  <dt>complex_model_s</dt>\n  <dd>\n  A machine suitable for the master and workers of the cluster when your\n  model requires more computation than the standard machine can handle\n  satisfactorily.\n  </dd>\n  <dt>complex_model_m</dt>\n  <dd>\n  A machine with roughly twice the number of cores and roughly double the\n  memory of <i>complex_model_s</i>.\n  </dd>\n  <dt>complex_model_l</dt>\n  <dd>\n  A machine with roughly twice the number of cores and roughly double the\n  memory of <i>complex_model_m</i>.\n  </dd>\n  <dt>standard_gpu</dt>\n  <dd>\n  A machine equivalent to <i>standard</i> that\n  also includes a single NVIDIA Tesla K80 GPU. See more about\n  <a href=\"/ml-engine/docs/tensorflow/using-gpus\">using GPUs to\n  train your model</a>.\n  </dd>\n  <dt>complex_model_m_gpu</dt>\n  <dd>\n  A machine equivalent to <i>complex_model_m</i> that also includes\n  four NVIDIA Tesla K80 GPUs.\n  </dd>\n  <dt>complex_model_l_gpu</dt>\n  <dd>\n  A machine equivalent to <i>complex_model_l</i> that also includes\n  eight NVIDIA Tesla K80 GPUs.\n  </dd>\n  <dt>standard_p100</dt>\n  <dd>\n  A machine equivalent to <i>standard</i> that\n  also includes a single NVIDIA Tesla P100 GPU.\n  </dd>\n  <dt>complex_model_m_p100</dt>\n  <dd>\n  A machine equivalent to <i>complex_model_m</i> that also includes\n  four NVIDIA Tesla P100 GPUs.\n  </dd>\n  <dt>standard_v100</dt>\n  <dd>\n  A machine equivalent to <i>standard</i> that\n  also includes a single NVIDIA Tesla V100 GPU.\n  </dd>\n  <dt>large_model_v100</dt>\n  <dd>\n  A machine equivalent to <i>large_model</i> that\n  also includes a single NVIDIA Tesla V100 GPU.\n  </dd>\n  <dt>complex_model_m_v100</dt>\n  <dd>\n  A machine equivalent to <i>complex_model_m</i> that\n  also includes four NVIDIA Tesla V100 GPUs.\n  </dd>\n  <dt>complex_model_l_v100</dt>\n  <dd>\n  A machine equivalent to <i>complex_model_l</i> that\n  also includes eight NVIDIA Tesla V100 GPUs.\n  </dd>\n  <dt>cloud_tpu</dt>\n  <dd>\n  A TPU VM including one Cloud TPU. See more about\n  <a href=\"/ml-engine/docs/tensorflow/using-tpus\">using TPUs to train\n  your model</a>.\n  </dd>\n</dl>\n\nYou may also use certain Compute Engine machine types directly in this\nfield. The following types are supported:\n\n* `n1-standard-4`\n* `n1-standard-8`\n* `n1-standard-16`\n* `n1-standard-32`\n* `n1-standard-64`\n* `n1-standard-96`\n* `n1-highmem-2`\n* `n1-highmem-4`\n* `n1-highmem-8`\n* `n1-highmem-16`\n* `n1-highmem-32`\n* `n1-highmem-64`\n* `n1-highmem-96`\n* `n1-highcpu-16`\n* `n1-highcpu-32`\n* `n1-highcpu-64`\n* `n1-highcpu-96`\n\nSee more about [using Compute Engine machine\ntypes](/ml-engine/docs/tensorflow/machine-types#compute-engine-machine-types).\n\nYou must set this value when `scaleTier` is set to `CUSTOM`."]
        #[serde(rename = "masterType", default)]
        pub master_type: ::std::option::Option<String>,
        #[doc = "Optional. The maximum job running time. The default is 7 days."]
        #[serde(rename = "maxRunningTime", default)]
        pub max_running_time: ::std::option::Option<String>,
        #[doc = "Required. The Google Cloud Storage location of the packages with\nthe training program and any additional dependencies.\nThe maximum number of package URIs is 100."]
        #[serde(rename = "packageUris", default)]
        pub package_uris: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The configuration for parameter servers.\n\nYou should only set `parameterServerConfig.acceleratorConfig` if\n`parameterServerConfigType` is set to a Compute Engine machine type. [Learn\nabout restrictions on accelerator configurations for\ntraining.](/ml-engine/docs/tensorflow/using-gpus#compute-engine-machine-types-with-gpu)\n\nSet `parameterServerConfig.imageUri` only if you build a custom image for\nyour parameter server. If `parameterServerConfig.imageUri` has not been\nset, AI Platform uses the value of `masterConfig.imageUri`.\nLearn more about [configuring custom\ncontainers](/ml-engine/docs/distributed-training-containers)."]
        #[serde(rename = "parameterServerConfig", default)]
        pub parameter_server_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1ReplicaConfig>,
        #[doc = "Optional. The number of parameter server replicas to use for the training\njob. Each replica in the cluster will be of the type specified in\n`parameter_server_type`.\n\nThis value can only be used when `scale_tier` is set to `CUSTOM`.If you\nset this value, you must also set `parameter_server_type`.\n\nThe default value is zero."]
        #[serde(rename = "parameterServerCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub parameter_server_count: ::std::option::Option<i64>,
        #[doc = "Optional. Specifies the type of virtual machine to use for your training\njob's parameter server.\n\nThe supported values are the same as those described in the entry for\n`master_type`.\n\nThis value must be consistent with the category of machine type that\n`masterType` uses. In other words, both must be AI Platform machine\ntypes or both must be Compute Engine machine types.\n\nThis value must be present when `scaleTier` is set to `CUSTOM` and\n`parameter_server_count` is greater than zero."]
        #[serde(rename = "parameterServerType", default)]
        pub parameter_server_type: ::std::option::Option<String>,
        #[doc = "Required. The Python module name to run after installing the packages."]
        #[serde(rename = "pythonModule", default)]
        pub python_module: ::std::option::Option<String>,
        #[doc = "Optional. The version of Python used in training. If not set, the default\nversion is '2.7'. Python '3.5' is available when `runtime_version` is set\nto '1.4' and above. Python '2.7' works with all supported\n<a href=\"/ml-engine/docs/runtime-version-list\">runtime versions</a>."]
        #[serde(rename = "pythonVersion", default)]
        pub python_version: ::std::option::Option<String>,
        #[doc = "Required. The Google Compute Engine region to run the training job in.\nSee the <a href=\"/ml-engine/docs/tensorflow/regions\">available regions</a>\nfor AI Platform services."]
        #[serde(rename = "region", default)]
        pub region: ::std::option::Option<String>,
        #[doc = "Optional. The AI Platform runtime version to use for training. If not\nset, AI Platform uses the default stable version, 1.0. For more\ninformation, see the\n<a href=\"/ml-engine/docs/runtime-version-list\">runtime version list</a>\nand\n<a href=\"/ml-engine/docs/versioning\">how to manage runtime versions</a>."]
        #[serde(rename = "runtimeVersion", default)]
        pub runtime_version: ::std::option::Option<String>,
        #[doc = "Required. Specifies the machine types, the number of replicas for workers\nand parameter servers."]
        #[serde(rename = "scaleTier", default)]
        pub scale_tier:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1TrainingInputScaleTier>,
        #[doc = "Optional. The configuration for workers.\n\nYou should only set `workerConfig.acceleratorConfig` if `workerType` is set\nto a Compute Engine machine type. [Learn about restrictions on accelerator\nconfigurations for\ntraining.](/ml-engine/docs/tensorflow/using-gpus#compute-engine-machine-types-with-gpu)\n\nSet `workerConfig.imageUri` only if you build a custom image for your\nworker. If `workerConfig.imageUri` has not been set, AI Platform uses\nthe value of `masterConfig.imageUri`. Learn more about\n[configuring custom\ncontainers](/ml-engine/docs/distributed-training-containers)."]
        #[serde(rename = "workerConfig", default)]
        pub worker_config: ::std::option::Option<crate::schemas::GoogleCloudMlV1ReplicaConfig>,
        #[doc = "Optional. The number of worker replicas to use for the training job. Each\nreplica in the cluster will be of the type specified in `worker_type`.\n\nThis value can only be used when `scale_tier` is set to `CUSTOM`. If you\nset this value, you must also set `worker_type`.\n\nThe default value is zero."]
        #[serde(rename = "workerCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub worker_count: ::std::option::Option<i64>,
        #[doc = "Optional. Specifies the type of virtual machine to use for your training\njob's worker nodes.\n\nThe supported values are the same as those described in the entry for\n`masterType`.\n\nThis value must be consistent with the category of machine type that\n`masterType` uses. In other words, both must be AI Platform machine\ntypes or both must be Compute Engine machine types.\n\nIf you use `cloud_tpu` for this value, see special instructions for\n[configuring a custom TPU\nmachine](/ml-engine/docs/tensorflow/using-tpus#configuring_a_custom_tpu_machine).\n\nThis value must be present when `scaleTier` is set to `CUSTOM` and\n`workerCount` is greater than zero."]
        #[serde(rename = "workerType", default)]
        pub worker_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrainingInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrainingInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1TrainingInputScaleTier {
        #[doc = "A single worker instance. This tier is suitable for learning how to use\nCloud ML, and for experimenting with new models using small datasets."]
        Basic,
        #[doc = "A single worker instance [with a\nGPU](/ml-engine/docs/tensorflow/using-gpus)."]
        BasicGpu,
        #[doc = "A single worker instance with a\n[Cloud TPU](/ml-engine/docs/tensorflow/using-tpus)."]
        BasicTpu,
        #[doc = "The CUSTOM tier is not a set tier, but rather enables you to use your\nown cluster specification. When you use this tier, set values to\nconfigure your processing cluster according to these guidelines:\n\n* You *must* set `TrainingInput.masterType` to specify the type\n  of machine to use for your master node. This is the only required\n  setting.\n\n* You *may* set `TrainingInput.workerCount` to specify the number of\n  workers to use. If you specify one or more workers, you *must* also\n  set `TrainingInput.workerType` to specify the type of machine to use\n  for your worker nodes.\n\n* You *may* set `TrainingInput.parameterServerCount` to specify the\n  number of parameter servers to use. If you specify one or more\n  parameter servers, you *must* also set\n  `TrainingInput.parameterServerType` to specify the type of machine to\n  use for your parameter servers.\n\nNote that all of your workers must use the same machine type, which can\nbe different from your parameter server type and master type. Your\nparameter servers must likewise use the same machine type, which can be\ndifferent from your worker type and master type."]
        Custom,
        #[doc = "A large number of workers with many parameter servers."]
        Premium1,
        #[doc = "Many workers and a few parameter servers."]
        Standard1,
    }
    impl GoogleCloudMlV1TrainingInputScaleTier {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1TrainingInputScaleTier::Basic => "BASIC",
                GoogleCloudMlV1TrainingInputScaleTier::BasicGpu => "BASIC_GPU",
                GoogleCloudMlV1TrainingInputScaleTier::BasicTpu => "BASIC_TPU",
                GoogleCloudMlV1TrainingInputScaleTier::Custom => "CUSTOM",
                GoogleCloudMlV1TrainingInputScaleTier::Premium1 => "PREMIUM_1",
                GoogleCloudMlV1TrainingInputScaleTier::Standard1 => "STANDARD_1",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1TrainingInputScaleTier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1TrainingInputScaleTier {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1TrainingInputScaleTier {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASIC" => GoogleCloudMlV1TrainingInputScaleTier::Basic,
                "BASIC_GPU" => GoogleCloudMlV1TrainingInputScaleTier::BasicGpu,
                "BASIC_TPU" => GoogleCloudMlV1TrainingInputScaleTier::BasicTpu,
                "CUSTOM" => GoogleCloudMlV1TrainingInputScaleTier::Custom,
                "PREMIUM_1" => GoogleCloudMlV1TrainingInputScaleTier::Premium1,
                "STANDARD_1" => GoogleCloudMlV1TrainingInputScaleTier::Standard1,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrainingInputScaleTier {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrainingInputScaleTier {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1TrainingOutput {
        #[doc = "Details related to built-in algorithms jobs.\nOnly set for built-in algorithms jobs."]
        #[serde(rename = "builtInAlgorithmOutput", default)]
        pub built_in_algorithm_output:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1BuiltInAlgorithmOutput>,
        #[doc = "The number of hyperparameter tuning trials that completed successfully.\nOnly set for hyperparameter tuning jobs."]
        #[serde(rename = "completedTrialCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub completed_trial_count: ::std::option::Option<i64>,
        #[doc = "The amount of ML units consumed by the job."]
        #[serde(rename = "consumedMLUnits", default)]
        pub consumed_ml_units: ::std::option::Option<f64>,
        #[doc = "The TensorFlow summary tag name used for optimizing hyperparameter tuning\ntrials. See\n[`HyperparameterSpec.hyperparameterMetricTag`](#HyperparameterSpec.FIELDS.hyperparameter_metric_tag)\nfor more information. Only set for hyperparameter tuning jobs."]
        #[serde(rename = "hyperparameterMetricTag", default)]
        pub hyperparameter_metric_tag: ::std::option::Option<String>,
        #[doc = "Whether this job is a built-in Algorithm job."]
        #[serde(rename = "isBuiltInAlgorithmJob", default)]
        pub is_built_in_algorithm_job: ::std::option::Option<bool>,
        #[doc = "Whether this job is a hyperparameter tuning job."]
        #[serde(rename = "isHyperparameterTuningJob", default)]
        pub is_hyperparameter_tuning_job: ::std::option::Option<bool>,
        #[doc = "Results for individual Hyperparameter trials.\nOnly set for hyperparameter tuning jobs."]
        #[serde(rename = "trials", default)]
        pub trials: ::std::option::Option<Vec<crate::schemas::GoogleCloudMlV1HyperparameterOutput>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1TrainingOutput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1TrainingOutput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudMlV1Version {
        #[doc = "Automatically scale the number of nodes used to serve the model in\nresponse to increases and decreases in traffic. Care should be\ntaken to ramp up traffic according to the model's ability to scale\nor you will start seeing increases in latency and 429 response codes."]
        #[serde(rename = "autoScaling", default)]
        pub auto_scaling: ::std::option::Option<crate::schemas::GoogleCloudMlV1AutoScaling>,
        #[doc = "Output only. The time the version was created."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Required. The Cloud Storage location of the trained model used to\ncreate the version. See the\n[guide to model\ndeployment](/ml-engine/docs/tensorflow/deploying-models) for more\ninformation.\n\nWhen passing Version to\n[projects.models.versions.create](/ml-engine/reference/rest/v1/projects.models.versions/create)\nthe model service uses the specified location as the source of the model.\nOnce deployed, the model version is hosted by the prediction service, so\nthis location is useful only as a historical record.\nThe total number of model files can't exceed 1000."]
        #[serde(rename = "deploymentUri", default)]
        pub deployment_uri: ::std::option::Option<String>,
        #[doc = "Optional. The description specified for the version when it was created."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. The details of a failure or a cancellation."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a model from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform model updates in order to avoid race\nconditions: An `etag` is returned in the response to `GetVersion`, and\nsystems are expected to put that etag in the request to `UpdateVersion` to\nensure that their change will be applied to the model as intended."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Optional. The machine learning framework AI Platform uses to train\nthis version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`,\n`XGBOOST`. If you do not specify a framework, AI Platform\nwill analyze files in the deployment_uri to determine a framework. If you\nchoose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version\nof the model to 1.4 or greater.\n\nDo **not** specify a framework if you're deploying a [custom\nprediction routine](/ml-engine/docs/tensorflow/custom-prediction-routines)."]
        #[serde(rename = "framework", default)]
        pub framework: ::std::option::Option<crate::schemas::GoogleCloudMlV1VersionFramework>,
        #[doc = "Output only. If true, this version will be used to handle prediction\nrequests that do not specify a version.\n\nYou can change the default version by calling\n[projects.methods.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault)."]
        #[serde(rename = "isDefault", default)]
        pub is_default: ::std::option::Option<bool>,
        #[doc = "Optional. One or more labels that you can add, to organize your model\nversions. Each label is a key-value pair, where both the key and the value\nare arbitrary strings that you supply.\nFor more information, see the documentation on\n<a href=\"/ml-engine/docs/tensorflow/resource-labels\">using labels</a>."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. The time the version was last used for prediction."]
        #[serde(rename = "lastUseTime", default)]
        pub last_use_time: ::std::option::Option<String>,
        #[doc = "Optional. The type of machine on which to serve the model. Currently only\napplies to online prediction service.\n\n<dl>\n  <dt>mls1-c1-m2</dt>\n  <dd>\n  The <b>default</b> machine type, with 1 core and 2 GB RAM. The deprecated\n  name for this machine type is \"mls1-highmem-1\".\n  </dd>\n  <dt>mls1-c4-m2</dt>\n  <dd>\n  In <b>Beta</b>. This machine type has 4 cores and 2 GB RAM. The\n  deprecated name for this machine type is \"mls1-highcpu-4\".\n  </dd>\n</dl>"]
        #[serde(rename = "machineType", default)]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "Manually select the number of nodes to use for serving the\nmodel. You should generally use `auto_scaling` with an appropriate\n`min_nodes` instead, but this option is available if you want more\npredictable billing. Beware that latency and error rates will increase\nif the traffic exceeds that capability of the system to serve it based\non the selected number of nodes."]
        #[serde(rename = "manualScaling", default)]
        pub manual_scaling: ::std::option::Option<crate::schemas::GoogleCloudMlV1ManualScaling>,
        #[doc = "Required. The name specified for the version when it was created.\n\nThe version name must be unique within the model it is created in."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. Cloud Storage paths (`gs://\u{2026}`) of packages for [custom\nprediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines)\nor [scikit-learn pipelines with custom\ncode](/ml-engine/docs/scikit/exporting-for-prediction#custom-pipeline-code).\n\nFor a custom prediction routine, one of these packages must contain your\nPredictor class (see\n[`predictionClass`](#Version.FIELDS.prediction_class)). Additionally,\ninclude any dependencies used by your Predictor or scikit-learn pipeline\nuses that are not already included in your selected [runtime\nversion](/ml-engine/docs/tensorflow/runtime-version-list).\n\nIf you specify this field, you must also set\n[`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater."]
        #[serde(rename = "packageUris", default)]
        pub package_uris: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The fully qualified name\n(<var>module_name</var>.<var>class_name</var>) of a class that implements\nthe Predictor interface described in this reference field. The module\ncontaining this class should be included in a package provided to the\n[`packageUris` field](#Version.FIELDS.package_uris).\n\nSpecify this field if and only if you are deploying a [custom prediction\nroutine (beta)](/ml-engine/docs/tensorflow/custom-prediction-routines).\nIf you specify this field, you must set\n[`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater.\n\nThe following code sample provides the Predictor interface:\n\n````textpy\nclass Predictor(object):\n\"\"\"Interface for constructing custom predictors.\"\"\"\n\ndef predict(self, instances, **kwargs):\n    \"\"\"Performs custom prediction.\n\n    Instances are the decoded values from the request. They have already\n    been deserialized from JSON.\n\n    Args:\n        instances: A list of prediction input instances.\n        **kwargs: A dictionary of keyword args provided as additional\n            fields on the predict request body.\n\n    Returns:\n        A list of outputs containing the prediction results. This list must\n        be JSON serializable.\n    \"\"\"\n    raise NotImplementedError()\n\n@classmethod\ndef from_path(cls, model_dir):\n    \"\"\"Creates an instance of Predictor using the given path.\n\n    Loading of the predictor should be done in this method.\n\n    Args:\n        model_dir: The local directory that contains the exported model\n            file along with any additional files uploaded when creating the\n            version resource.\n\n    Returns:\n        An instance implementing this Predictor class.\n    \"\"\"\n    raise NotImplementedError()\n````\n\nLearn more about [the Predictor interface and custom prediction\nroutines](/ml-engine/docs/tensorflow/custom-prediction-routines)."]
        #[serde(rename = "predictionClass", default)]
        pub prediction_class: ::std::option::Option<String>,
        #[doc = "Optional. The version of Python used in prediction. If not set, the default\nversion is '2.7'. Python '3.5' is available when `runtime_version` is set\nto '1.4' and above. Python '2.7' works with all supported runtime versions."]
        #[serde(rename = "pythonVersion", default)]
        pub python_version: ::std::option::Option<String>,
        #[doc = "Optional. Configures the request-response pair logging on predictions from\nthis Version."]
        #[serde(rename = "requestLoggingConfig", default)]
        pub request_logging_config:
            ::std::option::Option<crate::schemas::GoogleCloudMlV1RequestLoggingConfig>,
        #[doc = "Optional. The AI Platform runtime version to use for this deployment.\nIf not set, AI Platform uses the default stable version, 1.0. For more\ninformation, see the\n[runtime version list](/ml-engine/docs/runtime-version-list) and\n[how to manage runtime versions](/ml-engine/docs/versioning)."]
        #[serde(rename = "runtimeVersion", default)]
        pub runtime_version: ::std::option::Option<String>,
        #[doc = "Optional. Specifies the service account for resource access control."]
        #[serde(rename = "serviceAccount", default)]
        pub service_account: ::std::option::Option<String>,
        #[doc = "Output only. The state of a version."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::GoogleCloudMlV1VersionState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1Version {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1Version {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1VersionFramework {
        #[doc = "Unspecified framework. Assigns a value based on the file suffix."]
        FrameworkUnspecified,
        #[doc = "Scikit-learn framework."]
        ScikitLearn,
        #[doc = "Tensorflow framework."]
        Tensorflow,
        #[doc = "XGBoost framework."]
        Xgboost,
    }
    impl GoogleCloudMlV1VersionFramework {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1VersionFramework::FrameworkUnspecified => "FRAMEWORK_UNSPECIFIED",
                GoogleCloudMlV1VersionFramework::ScikitLearn => "SCIKIT_LEARN",
                GoogleCloudMlV1VersionFramework::Tensorflow => "TENSORFLOW",
                GoogleCloudMlV1VersionFramework::Xgboost => "XGBOOST",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1VersionFramework {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1VersionFramework {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1VersionFramework {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FRAMEWORK_UNSPECIFIED" => GoogleCloudMlV1VersionFramework::FrameworkUnspecified,
                "SCIKIT_LEARN" => GoogleCloudMlV1VersionFramework::ScikitLearn,
                "TENSORFLOW" => GoogleCloudMlV1VersionFramework::Tensorflow,
                "XGBOOST" => GoogleCloudMlV1VersionFramework::Xgboost,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1VersionFramework {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1VersionFramework {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudMlV1VersionState {
        #[doc = "The version is being created. New UpdateVersion and DeleteVersion\nrequests will fail if a version is in the CREATING state."]
        Creating,
        #[doc = "The version is being deleted. New UpdateVersion and DeleteVersion\nrequests will fail if a version is in the DELETING state."]
        Deleting,
        #[doc = "The version failed to be created, possibly cancelled.\n`error_message` should contain the details of the failure."]
        Failed,
        #[doc = "The version is ready for prediction."]
        Ready,
        #[doc = "The version state is unspecified."]
        Unknown,
        #[doc = "The version is being updated. New UpdateVersion and DeleteVersion\nrequests will fail if a version is in the UPDATING state."]
        Updating,
    }
    impl GoogleCloudMlV1VersionState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudMlV1VersionState::Creating => "CREATING",
                GoogleCloudMlV1VersionState::Deleting => "DELETING",
                GoogleCloudMlV1VersionState::Failed => "FAILED",
                GoogleCloudMlV1VersionState::Ready => "READY",
                GoogleCloudMlV1VersionState::Unknown => "UNKNOWN",
                GoogleCloudMlV1VersionState::Updating => "UPDATING",
            }
        }
    }
    impl ::std::fmt::Display for GoogleCloudMlV1VersionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudMlV1VersionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudMlV1VersionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => GoogleCloudMlV1VersionState::Creating,
                "DELETING" => GoogleCloudMlV1VersionState::Deleting,
                "FAILED" => GoogleCloudMlV1VersionState::Failed,
                "READY" => GoogleCloudMlV1VersionState::Ready,
                "UNKNOWN" => GoogleCloudMlV1VersionState::Unknown,
                "UPDATING" => GoogleCloudMlV1VersionState::Updating,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudMlV1VersionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudMlV1VersionState {
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
    pub struct GoogleIamV1AuditConfig {
        #[doc = "The configuration for logging of each type of permission."]
        #[serde(rename = "auditLogConfigs", default)]
        pub audit_log_configs:
            ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditLogConfig>>,
        #[doc = "Specifies a service that will be enabled for audit logging.\nFor example, `storage.googleapis.com`, `cloudsql.googleapis.com`.\n`allServices` is a special value that covers all services."]
        #[serde(rename = "service", default)]
        pub service: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditConfig {
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
    pub struct GoogleIamV1AuditLogConfig {
        #[doc = "Specifies the identities that do not cause logging for this type of\npermission.\nFollows the same format of Binding.members."]
        #[serde(rename = "exemptedMembers", default)]
        pub exempted_members: ::std::option::Option<Vec<String>>,
        #[doc = "The log type that this config enables."]
        #[serde(rename = "logType", default)]
        pub log_type: ::std::option::Option<crate::schemas::GoogleIamV1AuditLogConfigLogType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleIamV1AuditLogConfigLogType {
        #[doc = "Admin reads. Example: CloudIAM getIamPolicy"]
        AdminRead,
        #[doc = "Data reads. Example: CloudSQL Users list"]
        DataRead,
        #[doc = "Data writes. Example: CloudSQL Users create"]
        DataWrite,
        #[doc = "Default case. Should never be this."]
        LogTypeUnspecified,
    }
    impl GoogleIamV1AuditLogConfigLogType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleIamV1AuditLogConfigLogType::AdminRead => "ADMIN_READ",
                GoogleIamV1AuditLogConfigLogType::DataRead => "DATA_READ",
                GoogleIamV1AuditLogConfigLogType::DataWrite => "DATA_WRITE",
                GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified => "LOG_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleIamV1AuditLogConfigLogType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleIamV1AuditLogConfigLogType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleIamV1AuditLogConfigLogType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN_READ" => GoogleIamV1AuditLogConfigLogType::AdminRead,
                "DATA_READ" => GoogleIamV1AuditLogConfigLogType::DataRead,
                "DATA_WRITE" => GoogleIamV1AuditLogConfigLogType::DataWrite,
                "LOG_TYPE_UNSPECIFIED" => GoogleIamV1AuditLogConfigLogType::LogTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1AuditLogConfigLogType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1AuditLogConfigLogType {
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
    pub struct GoogleIamV1Binding {
        #[doc = "The condition that is associated with this binding.\nNOTE: An unsatisfied condition will not allow user access via current\nbinding. Different bindings, including their conditions, are examined\nindependently."]
        #[serde(rename = "condition", default)]
        pub condition: ::std::option::Option<crate::schemas::GoogleTypeExpr>,
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource.\n`members` can have the following values:\n\n* `allUsers`: A special identifier that represents anyone who is\n  on the internet; with or without a Google account.\n\n* `allAuthenticatedUsers`: A special identifier that represents anyone\n  who is authenticated with a Google account or a service account.\n\n* `user:{emailid}`: An email address that represents a specific Google\n  account. For example, `alice@example.com` .\n\n* `serviceAccount:{emailid}`: An email address that represents a service\n  account. For example, `my-other-app@appspot.gserviceaccount.com`.\n\n* `group:{emailid}`: An email address that represents a Google group.\n  For example, `admins@example.com`.\n\n* `domain:{domain}`: The G Suite domain (primary) that represents all the\n  users of that domain. For example, `google.com` or `example.com`."]
        #[serde(rename = "members", default)]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to `members`.\nFor example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(rename = "role", default)]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Binding {
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
    pub struct GoogleIamV1Policy {
        #[doc = "Specifies cloud audit logging configuration for this policy."]
        #[serde(rename = "auditConfigs", default)]
        pub audit_configs: ::std::option::Option<Vec<crate::schemas::GoogleIamV1AuditConfig>>,
        #[doc = "Associates a list of `members` to a `role`.\n`bindings` with no members will result in an error."]
        #[serde(rename = "bindings", default)]
        pub bindings: ::std::option::Option<Vec<crate::schemas::GoogleIamV1Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help\nprevent simultaneous updates of a policy from overwriting each other.\nIt is strongly suggested that systems make use of the `etag` in the\nread-modify-write cycle to perform policy updates in order to avoid race\nconditions: An `etag` is returned in the response to `getIamPolicy`, and\nsystems are expected to put that etag in the request to `setIamPolicy` to\nensure that their change will be applied to the same version of the policy.\n\nIf no `etag` is provided in the call to `setIamPolicy`, then the existing\npolicy is overwritten."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Deprecated."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1Policy {
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
    pub struct GoogleIamV1SetIamPolicyRequest {
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of\nthe policy is limited to a few 10s of KB. An empty policy is a\nvalid policy but certain Cloud Platform services (such as Projects)\nmight reject them."]
        #[serde(rename = "policy", default)]
        pub policy: ::std::option::Option<crate::schemas::GoogleIamV1Policy>,
        #[doc = "OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only\nthe fields in the mask will be modified. If no mask is provided, the\nfollowing default mask is used:\npaths: \"bindings, etag\"\nThis field is only used by Cloud IAM."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1SetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1SetIamPolicyRequest {
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
    pub struct GoogleIamV1TestIamPermissionsRequest {
        #[doc = "The set of permissions to check for the `resource`. Permissions with\nwildcards (such as '*' or 'storage.*') are not allowed. For more\ninformation see\n[IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        #[serde(rename = "permissions", default)]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1TestIamPermissionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1TestIamPermissionsRequest {
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
    pub struct GoogleIamV1TestIamPermissionsResponse {
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is\nallowed."]
        #[serde(rename = "permissions", default)]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleIamV1TestIamPermissionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleIamV1TestIamPermissionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(rename = "operations", default)]
        pub operations: ::std::option::Option<Vec<crate::schemas::GoogleLongrunningOperation>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningListOperationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningOperation {
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
    pub struct GoogleProtobufEmpty;
    impl ::google_field_selector::FieldSelector for GoogleProtobufEmpty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleProtobufEmpty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
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
    pub struct GoogleTypeExpr {
        #[doc = "An optional description of the expression. This is a longer text which\ndescribes the expression, e.g. when hovered over it in a UI."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in\nCommon Expression Language syntax.\n\nThe application context of the containing message determines which\nwell-known feature set of CEL is supported."]
        #[serde(rename = "expression", default)]
        pub expression: ::std::option::Option<String>,
        #[doc = "An optional string indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "An optional title for the expression, i.e. a short string describing\nits purpose. This can be used e.g. in UIs which allow to enter the\nexpression."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeExpr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeExpr {
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
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: A,
}
impl<A> Client<A>
where
    A: ::google_api_auth::GetAccessToken,
{
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth,
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
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ProjectsActions<'a, A> {
            #[doc = "Get the service account information associated with your project. You need\nthis information in order to grant the service account permissions for\nthe Google Cloud Storage location where you put your model training code\nfor training the model with Google Cloud Machine Learning."]
            pub fn get_config(&self, name: impl Into<String>) -> GetConfigRequestBuilder<A> {
                GetConfigRequestBuilder {
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
            #[doc = "Performs prediction on the data in the request.\nAI Platform implements a custom `predict` verb on top of an HTTP POST\nmethod. <p>For details of the request and response format, see the **guide\nto the [predict request format](/ml-engine/docs/v1/predict-request)**."]
            pub fn predict(
                &self,
                request: crate::schemas::GoogleCloudMlV1PredictRequest,
                name: impl Into<String>,
            ) -> PredictRequestBuilder<A> {
                PredictRequestBuilder {
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
                    name: name.into(),
                }
            }
            #[doc = "Actions that can be performed on the jobs resource"]
            pub fn jobs(&self) -> crate::resources::projects::jobs::JobsActions<A> {
                crate::resources::projects::jobs::JobsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions<A> {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the models resource"]
            pub fn models(&self) -> crate::resources::projects::models::ModelsActions<A> {
                crate::resources::projects::models::ModelsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(
                &self,
            ) -> crate::resources::projects::operations::OperationsActions<A> {
                crate::resources::projects::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetConfigRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> GetConfigRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
            ) -> Result<
                crate::schemas::GoogleCloudMlV1GetConfigResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleCloudMlV1GetConfigResponse,
                Box<dyn ::std::error::Error>,
            > {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://ml.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":getConfig");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
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
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct PredictRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::GoogleCloudMlV1PredictRequest,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> PredictRequestBuilder<'a, A> {
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
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
            ) -> Result<crate::schemas::GoogleApiHttpBody, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleApiHttpBody, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(
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
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://ml.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":predict");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
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
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        pub mod jobs {
            pub mod params {}
            pub struct JobsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> JobsActions<'a, A> {
                #[doc = "Cancels a running job."]
                pub fn cancel(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1CancelJobRequest,
                    name: impl Into<String>,
                ) -> CancelRequestBuilder<A> {
                    CancelRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Creates a training or a batch prediction job."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Job,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Describes a job."]
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
                #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                pub fn get_iam_policy(
                    &self,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder<A> {
                    GetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                        options_requested_policy_version: None,
                    }
                }
                #[doc = "Lists the jobs in the project.\n\nIf there are no jobs that match the request parameters, the list\nrequest returns an empty response body: {}."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a specific job resource.\n\nCurrently the only supported fields to update are `labels`."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Job,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder<A> {
                    PatchRequestBuilder {
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
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder<A> {
                    SetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder<A> {
                    TestIamPermissionsRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleCloudMlV1CancelJobRequest,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> CancelRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleProtobufEmpty, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleCloudMlV1Job,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> CreateRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                resource: String,
                options_requested_policy_version: Option<i32>,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> GetIamPolicyRequestBuilder<'a, A> {
                #[doc = "Optional. The policy format version to be returned.\nAcceptable values are 0, 1, and 3.\nIf the value is 0, or the field is omitted, policy format version 1 will be\nreturned."]
                pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                    self.options_requested_policy_version = Some(value);
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "options.requestedPolicyVersion",
                        &self.options_requested_policy_version,
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                parent: String,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "Optional. Specifies the subset of jobs to retrieve.\nYou can filter on the value of one or more attributes of the job object.\nFor example, retrieve jobs with a job identifier that starts with 'census':\n\n<p><code>gcloud ai-platform jobs list --filter='jobId:census*'</code>\n<p>List all failed jobs with names that start with 'rnn':\n<p><code>gcloud ai-platform jobs list --filter='jobId:rnn*\nAND state:FAILED'</code>\n<p>For more examples, see the guide to\n<a href=\"/ml-engine/docs/tensorflow/monitor-training\">monitoring jobs</a>."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The number of jobs to retrieve per \"page\" of results. If there\nare more remaining results than this number, the response message will\ncontain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
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
                pub fn iter_jobs<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_jobs_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_jobs_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Job>
                {
                    self.iter_jobs_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_jobs_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Job>
                {
                    self.iter_jobs_with_fields(Some("*"))
                }
                pub fn iter_jobs_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "jobs").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "jobs")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListJobsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListJobsResponse>
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<
                    crate::schemas::GoogleCloudMlV1ListJobsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudMlV1ListJobsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> crate::iter::IterableMethod
                for ListRequestBuilder<'a, A>
            {
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
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleCloudMlV1Job,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> PatchRequestBuilder<'a, A> {
                #[doc = "Required. Specifies the path, relative to `Job`, of the field to update.\nTo adopt etag mechanism, include `etag` field in the mask, and include the\n`etag` value in your job resource.\n\nFor example, to change the labels of a job, the `update_mask` parameter\nwould be specified as `labels`, `etag`, and the\n`PATCH` request body would specify the new value, as follows:\n{\n\"labels\": {\n\"owner\": \"Google\",\n\"color\": \"Blue\"\n}\n\"etag\": \"33a64df551425fcc55e4d42a148795d9f25f89d4\"\n}\nIf `etag` matches the one on the server, the labels of the job will be\nreplaced with the given ones, and the server end `etag` will be\nrecalculated.\n\nCurrently the only supported update masks are `labels` and `etag`."]
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Job, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    let req = req.query(&[("updateMask", &self.update_mask)]);
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                resource: String,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> SetIamPolicyRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":setIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                resource: String,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> TestIamPermissionsRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<
                    crate::schemas::GoogleIamV1TestIamPermissionsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleIamV1TestIamPermissionsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":testIamPermissions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> LocationsActions<'a, A> {
                #[doc = "Get the complete list of CMLE capabilities in a location, along with their\nlocation-specific properties."]
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
                #[doc = "List all locations that provides at least one type of CMLE capability."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleCloudMlV1Location, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Location, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "Optional. The number of locations to retrieve per \"page\" of results. If\nthere are more remaining results than this number, the response message\nwill contain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
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
                pub fn iter_locations<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_locations_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_locations_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Location>
                {
                    self.iter_locations_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_locations_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Location>
                {
                    self.iter_locations_with_fields(Some("*"))
                }
                pub fn iter_locations_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "locations").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "locations")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListLocationsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListLocationsResponse>
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<
                    crate::schemas::GoogleCloudMlV1ListLocationsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudMlV1ListLocationsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/locations");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> crate::iter::IterableMethod
                for ListRequestBuilder<'a, A>
            {
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
        pub mod models {
            pub mod params {}
            pub struct ModelsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ModelsActions<'a, A> {
                #[doc = "Creates a model which will later contain one or more versions.\n\nYou must add at least one version before you can request predictions from\nthe model. Add versions by calling\n[projects.models.versions.create](/ml-engine/reference/rest/v1/projects.models.versions/create)."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Model,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Deletes a model.\n\nYou can only delete a model if there are no versions in it. You can delete\nversions by calling\n[projects.models.versions.delete](/ml-engine/reference/rest/v1/projects.models.versions/delete)."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets information about a model, including its name, the description (if\nset), and the default version (if at least one version of the model has\nbeen deployed)."]
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
                #[doc = "Gets the access control policy for a resource.\nReturns an empty policy if the resource exists and does not have a policy\nset."]
                pub fn get_iam_policy(
                    &self,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder<A> {
                    GetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                        options_requested_policy_version: None,
                    }
                }
                #[doc = "Lists the models in a project.\n\nEach project can contain multiple models, and each model can have multiple\nversions.\n\nIf there are no models that match the request parameters, the list request\nreturns an empty response body: {}."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a specific model resource.\n\nCurrently the only supported fields to update are `description` and\n`default_version.name`."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GoogleCloudMlV1Model,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder<A> {
                    PatchRequestBuilder {
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
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Sets the access control policy on the specified resource. Replaces any\nexisting policy."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder<A> {
                    SetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns permissions that a caller has on the specified resource.\nIf the resource does not exist, this will return an empty set of\npermissions, not a NOT_FOUND error.\n\nNote: This operation is designed to be used for building permission-aware\nUIs and command-line tools, not for authorization checking. This operation\nmay \"fail open\" without warning."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder<A> {
                    TestIamPermissionsRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Actions that can be performed on the versions resource"]
                pub fn versions(
                    &self,
                ) -> crate::resources::projects::models::versions::VersionsActions<A>
                {
                    crate::resources::projects::models::versions::VersionsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleCloudMlV1Model,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> CreateRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/models");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> DeleteRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudMlV1Model, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                resource: String,
                options_requested_policy_version: Option<i32>,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> GetIamPolicyRequestBuilder<'a, A> {
                #[doc = "Optional. The policy format version to be returned.\nAcceptable values are 0, 1, and 3.\nIf the value is 0, or the field is omitted, policy format version 1 will be\nreturned."]
                pub fn options_requested_policy_version(mut self, value: i32) -> Self {
                    self.options_requested_policy_version = Some(value);
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "options.requestedPolicyVersion",
                        &self.options_requested_policy_version,
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                parent: String,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "Optional. Specifies the subset of models to retrieve."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The number of models to retrieve per \"page\" of results. If there\nare more remaining results than this number, the response message will\ncontain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
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
                pub fn iter_models<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_models_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_models_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Model>
                {
                    self.iter_models_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_models_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Model>
                {
                    self.iter_models_with_fields(Some("*"))
                }
                pub fn iter_models_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "models").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "models")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListModelsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::GoogleCloudMlV1ListModelsResponse>
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<
                    crate::schemas::GoogleCloudMlV1ListModelsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudMlV1ListModelsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/models");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> crate::iter::IterableMethod
                for ListRequestBuilder<'a, A>
            {
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
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleCloudMlV1Model,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> PatchRequestBuilder<'a, A> {
                #[doc = "Required. Specifies the path, relative to `Model`, of the field to update.\n\nFor example, to change the description of a model to \"foo\" and set its\ndefault version to \"version_1\", the `update_mask` parameter would be\nspecified as `description`, `default_version.name`, and the `PATCH`\nrequest body would specify the new value, as follows:\n{\n\"description\": \"foo\",\n\"defaultVersion\": {\n\"name\":\"version_1\"\n}\n}\n\nCurrently the supported update masks are `description` and\n`default_version.name`."]
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    let req = req.query(&[("updateMask", &self.update_mask)]);
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleIamV1SetIamPolicyRequest,
                resource: String,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> SetIamPolicyRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleIamV1Policy, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":setIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::GoogleIamV1TestIamPermissionsRequest,
                resource: String,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> TestIamPermissionsRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<
                    crate::schemas::GoogleIamV1TestIamPermissionsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleIamV1TestIamPermissionsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":testIamPermissions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            pub mod versions {
                pub mod params {}
                pub struct VersionsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> VersionsActions<'a, A> {
                    #[doc = "Creates a new version of a model from a trained TensorFlow model.\n\nIf the version created in the cloud by this call is the first deployed\nversion of the specified model, it will be made the default version of the\nmodel. When you add a version to a model that already has one or more\nversions, the default version does not automatically change. If you want a\nnew version to be the default, you must call\n[projects.models.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault)."]
                    pub fn create(
                        &self,
                        request: crate::schemas::GoogleCloudMlV1Version,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a model version.\n\nEach model can have multiple versions deployed and in use at any given\ntime. Use this method to remove a single version.\n\nNote: You cannot delete the version that is set as the default version\nof the model unless it is the only remaining version."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Gets information about a model version.\n\nModels can have multiple versions. You can call\n[projects.models.versions.list](/ml-engine/reference/rest/v1/projects.models.versions/list)\nto get the same information that this method returns for all of the\nversions of a model."]
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
                    #[doc = "Gets basic information about all the versions of a model.\n\nIf you expect that a model has many versions, or if you need to handle\nonly a limited number of results at a time, you can request that the list\nbe retrieved in batches (called pages).\n\nIf there are no versions that match the request parameters, the list\nrequest returns an empty response body: {}."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the specified Version resource.\n\nCurrently the only update-able fields are `description` and\n`autoScaling.minNodes`."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::GoogleCloudMlV1Version,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder<A> {
                        PatchRequestBuilder {
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
                            name: name.into(),
                            update_mask: None,
                        }
                    }
                    #[doc = "Designates a version to be the default for the model.\n\nThe default version is used for prediction requests made against the model\nthat don't specify a version.\n\nThe first version to be created for a model is automatically set as the\ndefault. You must make any subsequent changes to the default version\nsetting manually using this method."]
                    pub fn set_default(
                        &self,
                        request: crate::schemas::GoogleCloudMlV1SetDefaultVersionRequest,
                        name: impl Into<String>,
                    ) -> SetDefaultRequestBuilder<A> {
                        SetDefaultRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::GoogleCloudMlV1Version,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> CreateRequestBuilder<'a, A> {
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
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
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
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/versions");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
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
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> DeleteRequestBuilder<'a, A> {
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
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
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
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
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
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
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
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
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
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
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
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    parent: String,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                    #[doc = "Optional. Specifies the subset of versions to retrieve."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Optional. The number of versions to retrieve per \"page\" of results. If\nthere are more remaining results than this number, the response message\nwill contain a valid value in the `next_page_token` field.\n\nThe default value is 20, and the maximum page size is 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. A page token to request the next page of results.\n\nYou get the token from the `next_page_token` field of the response from\nthe previous call."]
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
                    pub fn iter_versions<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_versions_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_versions_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Version>
                    {
                        self.iter_versions_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_versions_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleCloudMlV1Version>
                    {
                        self.iter_versions_with_fields(Some("*"))
                    }
                    pub fn iter_versions_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "versions").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "versions")
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
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudMlV1ListVersionsResponse,
                    > {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::GoogleCloudMlV1ListVersionsResponse,
                    > {
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
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
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
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                    ) -> Result<
                        crate::schemas::GoogleCloudMlV1ListVersionsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudMlV1ListVersionsResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
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
                        let req = self._request(&self._path())?;
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/versions");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
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
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> crate::iter::IterableMethod
                    for ListRequestBuilder<'a, A>
                {
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
                    pub(crate) auth: &'a A,
                    request: crate::schemas::GoogleCloudMlV1Version,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> PatchRequestBuilder<'a, A> {
                    #[doc = "Required. Specifies the path, relative to `Version`, of the field to\nupdate. Must be present and non-empty.\n\nFor example, to change the description of a version to \"foo\", the\n`update_mask` parameter would be specified as `description`, and the\n`PATCH` request body would specify the new value, as follows:\n{\n\"description\": \"foo\"\n}\n\nCurrently the only supported update mask fields are `description` and\n`autoScaling.minNodes`."]
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
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
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
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
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
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        let req = req.query(&[("updateMask", &self.update_mask)]);
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
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SetDefaultRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::GoogleCloudMlV1SetDefaultVersionRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> SetDefaultRequestBuilder<'a, A> {
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
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleCloudMlV1Version, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
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
                        let req = self._request(&self._path())?;
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://ml.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":setDefault");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
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
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
            }
        }
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> OperationsActions<'a, A> {
                #[doc = "Starts asynchronous cancellation on a long-running operation.  The server\nmakes a best effort to cancel the operation, but success is not\nguaranteed.  If the server doesn't support this method, it returns\n`google.rpc.Code.UNIMPLEMENTED`.  Clients can use\nOperations.GetOperation or\nother methods to check whether the cancellation succeeded or whether the\noperation completed despite cancellation. On successful cancellation,\nthe operation is not deleted; instead, it becomes an operation with\nan Operation.error value with a google.rpc.Status.code of 1,\ncorresponding to `Code.CANCELLED`."]
                pub fn cancel(&self, name: impl Into<String>) -> CancelRequestBuilder<A> {
                    CancelRequestBuilder {
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
                #[doc = "Lists operations that match the specified filter in the request. If the\nserver doesn't support this method, it returns `UNIMPLEMENTED`.\n\nNOTE: the `name` binding allows API services to override the binding\nto use different resource name schemes, such as `users/*/operations`. To\noverride the binding, API services can add a binding such as\n`\"/v1/{name=users/*}/operations\"` to their service configuration.\nFor backwards compatibility, the default name includes the operations\ncollection id, however overriding users must ensure the name binding\nis the parent resource, without the operations collection id."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder<A> {
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
                        name: name.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> CancelRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleProtobufEmpty, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
                    output.push_str("v1/");
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                {
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleLongrunningOperation>
                {
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                > {
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
                pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
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
                ) -> Result<
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleLongrunningListOperationsResponse,
                    Box<dyn ::std::error::Error>,
                > {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
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
                    let req = self._request(&self._path())?;
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://ml.googleapis.com/".to_owned();
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> crate::iter::IterableMethod
                for ListRequestBuilder<'a, A>
            {
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
pub mod bytes {
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
