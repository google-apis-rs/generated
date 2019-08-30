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
    pub struct Accelerator {
        #[doc = "How many accelerators of this type to attach."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "The accelerator type string (for example, \"nvidia-tesla-k80\").\n\nOnly NVIDIA GPU accelerators are currently supported. If an NVIDIA GPU is\nattached, the required runtime libraries will be made available to all\ncontainers under `/usr/local/nvidia`. The driver version to install must\nbe specified using the NVIDIA driver version parameter on the virtual\nmachine specification. Note that attaching a GPU increases the worker VM\nstartup time by a few minutes."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Accelerator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Accelerator {
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
    pub struct Action {
        #[doc = "If specified, overrides the `CMD` specified in the container. If the\ncontainer also has an `ENTRYPOINT` the values are used as entrypoint\narguments. Otherwise, they are used as a command and arguments to run\ninside the container."]
        #[serde(rename = "commands", default)]
        pub commands: ::std::option::Option<Vec<String>>,
        #[doc = "If the specified image is hosted on a private registry other than Google\nContainer Registry, the credentials required to pull the image must be\nspecified here as an encrypted secret.\n\nThe secret must decrypt to a JSON-encoded dictionary containing both\n`username` and `password` keys."]
        #[serde(rename = "credentials", default)]
        pub credentials: ::std::option::Option<crate::schemas::Secret>,
        #[doc = "If specified, overrides the `ENTRYPOINT` specified in the container."]
        #[serde(rename = "entrypoint", default)]
        pub entrypoint: ::std::option::Option<String>,
        #[doc = "The environment to pass into the container. This environment is merged\nwith values specified in the google.genomics.v2alpha1.Pipeline\nmessage, overwriting any duplicate values.\n\nIn addition to the values passed here, a few other values are\nautomatically injected into the environment. These cannot be hidden or\noverwritten.\n\n`GOOGLE_PIPELINE_FAILED` will be set to \"1\" if the pipeline failed\nbecause an action has exited with a non-zero status (and did not have the\n`IGNORE_EXIT_STATUS` flag set). This can be used to determine if additional\ndebug or logging actions should execute.\n\n`GOOGLE_LAST_EXIT_STATUS` will be set to the exit status of the last\nnon-background action that executed. This can be used by workflow engine\nauthors to determine whether an individual action has succeeded or failed."]
        #[serde(rename = "environment", default)]
        pub environment: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The set of flags to apply to this action."]
        #[serde(rename = "flags", default)]
        pub flags: ::std::option::Option<Vec<crate::schemas::ActionFlagsItems>>,
        #[doc = "Required. The URI to pull the container image from. Note that all images referenced\nby actions in the pipeline are pulled before the first action runs. If\nmultiple actions reference the same image, it is only pulled once,\nensuring that the same image is used for all actions in a single pipeline.\n\nThe image URI can be either a complete host and image specification (e.g.,\nquay.io/biocontainers/samtools), a library and image name (e.g.,\ngoogle/cloud-sdk) or a bare image name ('bash') to pull from the default\nlibrary.  No schema is required in any of these cases.\n\nIf the specified image is not public, the service account specified for\nthe Virtual Machine must have access to pull the images from GCR, or\nappropriate credentials must be specified in the\ngoogle.genomics.v2alpha1.Action.credentials field."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
        #[doc = "Labels to associate with the action. This field is provided to assist\nworkflow engine authors in identifying actions (for example, to indicate\nwhat sort of action they perform, such as localization or debugging).\nThey are returned in the operation metadata, but are otherwise ignored."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A list of mounts to make available to the action.\n\nIn addition to the values specified here, every action has a special\nvirtual disk mounted under `/google` that contains log files and other\noperational components.\n\n<ul>\n  <li><code>/google/logs</code> All logs written during the pipeline\n  execution.</li>\n  <li><code>/google/logs/output</code> The combined standard output and\n  standard error of all actions run as part of the pipeline\n  execution.</li>\n  <li><code>/google/logs/action/*/stdout</code> The complete contents of\n  each individual action's standard output.</li>\n  <li><code>/google/logs/action/*/stderr</code> The complete contents of\n  each individual action's standard error output.</li>\n</ul>"]
        #[serde(rename = "mounts", default)]
        pub mounts: ::std::option::Option<Vec<crate::schemas::Mount>>,
        #[doc = "An optional name for the container. The container hostname will be set to\nthis name, making it useful for inter-container communication. The name\nmust contain only upper and lowercase alphanumeric characters and hypens\nand cannot start with a hyphen."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "An optional identifier for a PID namespace to run the action inside.\nMultiple actions should use the same string to share a namespace.  If\nunspecified, a separate isolated namespace is used."]
        #[serde(rename = "pidNamespace", default)]
        pub pid_namespace: ::std::option::Option<String>,
        #[doc = "A map of containers to host port mappings for this container. If the\ncontainer already specifies exposed ports, use the\n`PUBLISH_EXPOSED_PORTS` flag instead.\n\nThe host port number must be less than 65536. If it is zero, an unused\nrandom port is assigned. To determine the resulting port number, consult\nthe `ContainerStartedEvent` in the operation metadata."]
        #[serde(rename = "portMappings", default)]
        pub port_mappings: ::std::option::Option<::std::collections::BTreeMap<String, i32>>,
        #[doc = "The maximum amount of time to give the action to complete. If the action\nfails to complete before the timeout, it will be terminated and the exit\nstatus will be non-zero. The pipeline will continue or terminate based\non the rules defined by the `ALWAYS_RUN` and `IGNORE_EXIT_STATUS` flags."]
        #[serde(rename = "timeout", default)]
        pub timeout: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Action {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Action {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ActionFlagsItems {
        AlwaysRun,
        DisableImagePrefetch,
        DisableStandardErrorCapture,
        EnableFuse,
        FlagUnspecified,
        IgnoreExitStatus,
        PublishExposedPorts,
        RunInBackground,
    }
    impl ActionFlagsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ActionFlagsItems::AlwaysRun => "ALWAYS_RUN",
                ActionFlagsItems::DisableImagePrefetch => "DISABLE_IMAGE_PREFETCH",
                ActionFlagsItems::DisableStandardErrorCapture => "DISABLE_STANDARD_ERROR_CAPTURE",
                ActionFlagsItems::EnableFuse => "ENABLE_FUSE",
                ActionFlagsItems::FlagUnspecified => "FLAG_UNSPECIFIED",
                ActionFlagsItems::IgnoreExitStatus => "IGNORE_EXIT_STATUS",
                ActionFlagsItems::PublishExposedPorts => "PUBLISH_EXPOSED_PORTS",
                ActionFlagsItems::RunInBackground => "RUN_IN_BACKGROUND",
            }
        }
    }
    impl ::std::fmt::Display for ActionFlagsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ActionFlagsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ActionFlagsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALWAYS_RUN" => ActionFlagsItems::AlwaysRun,
                "DISABLE_IMAGE_PREFETCH" => ActionFlagsItems::DisableImagePrefetch,
                "DISABLE_STANDARD_ERROR_CAPTURE" => ActionFlagsItems::DisableStandardErrorCapture,
                "ENABLE_FUSE" => ActionFlagsItems::EnableFuse,
                "FLAG_UNSPECIFIED" => ActionFlagsItems::FlagUnspecified,
                "IGNORE_EXIT_STATUS" => ActionFlagsItems::IgnoreExitStatus,
                "PUBLISH_EXPOSED_PORTS" => ActionFlagsItems::PublishExposedPorts,
                "RUN_IN_BACKGROUND" => ActionFlagsItems::RunInBackground,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ActionFlagsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionFlagsItems {
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
    pub struct CancelOperationRequest;
    impl ::google_field_selector::FieldSelector for CancelOperationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CancelOperationRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CheckInRequest {
        #[doc = "The deadline has expired and the worker needs more time."]
        #[serde(rename = "deadlineExpired", default)]
        pub deadline_expired: ::std::option::Option<crate::schemas::Empty>,
        #[doc = "A workflow specific event occurred."]
        #[serde(rename = "event", default)]
        pub event: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The operation has finished with the given result."]
        #[serde(rename = "result", default)]
        pub result: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Data about the status of the worker VM."]
        #[serde(rename = "workerStatus", default)]
        pub worker_status: ::std::option::Option<crate::schemas::WorkerStatus>,
    }
    impl ::google_field_selector::FieldSelector for CheckInRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckInRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CheckInResponse {
        #[doc = "The deadline by which the worker must request an extension.  The backend\nwill allow for network transmission time and other delays, but the worker\nmust attempt to transmit the extension request no later than the deadline."]
        #[serde(rename = "deadline", default)]
        pub deadline: ::std::option::Option<String>,
        #[doc = "The metadata that describes the operation assigned to the worker."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for CheckInResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckInResponse {
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
    pub struct ComputeEngine {
        #[doc = "The names of the disks that were created for this pipeline."]
        #[serde(rename = "diskNames", default)]
        pub disk_names: ::std::option::Option<Vec<String>>,
        #[doc = "The instance on which the operation is running."]
        #[serde(rename = "instanceName", default)]
        pub instance_name: ::std::option::Option<String>,
        #[doc = "The machine type of the instance."]
        #[serde(rename = "machineType", default)]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "The availability zone in which the instance resides."]
        #[serde(rename = "zone", default)]
        pub zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ComputeEngine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ComputeEngine {
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
    pub struct ContainerKilledEvent {
        #[doc = "The numeric ID of the action that started the container."]
        #[serde(rename = "actionId", default)]
        pub action_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ContainerKilledEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContainerKilledEvent {
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
    pub struct ContainerStartedEvent {
        #[doc = "The numeric ID of the action that started this container."]
        #[serde(rename = "actionId", default)]
        pub action_id: ::std::option::Option<i32>,
        #[doc = "The public IP address that can be used to connect to the container. This\nfield is only populated when at least one port mapping is present. If the\ninstance was created with a private address, this field will be empty even\nif port mappings exist."]
        #[serde(rename = "ipAddress", default)]
        pub ip_address: ::std::option::Option<String>,
        #[doc = "The container-to-host port mappings installed for this container. This\nset will contain any ports exposed using the `PUBLISH_EXPOSED_PORTS` flag\nas well as any specified in the `Action` definition."]
        #[serde(rename = "portMappings", default)]
        pub port_mappings: ::std::option::Option<::std::collections::BTreeMap<String, i32>>,
    }
    impl ::google_field_selector::FieldSelector for ContainerStartedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContainerStartedEvent {
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
    pub struct ContainerStoppedEvent {
        #[doc = "The numeric ID of the action that started this container."]
        #[serde(rename = "actionId", default)]
        pub action_id: ::std::option::Option<i32>,
        #[doc = "The exit status of the container."]
        #[serde(rename = "exitStatus", default)]
        pub exit_status: ::std::option::Option<i32>,
        #[doc = "The tail end of any content written to standard error by the container.\nIf the content emits large amounts of debugging noise or contains\nsensitive information, you can prevent the content from being printed by\nsetting the `DISABLE_STANDARD_ERROR_CAPTURE` flag.\n\nNote that only a small amount of the end of the stream is captured here.\nThe entire stream is stored in the `/google/logs` directory mounted into\neach action, and can be copied off the machine as described elsewhere."]
        #[serde(rename = "stderr", default)]
        pub stderr: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContainerStoppedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContainerStoppedEvent {
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
    pub struct DelayedEvent {
        #[doc = "A textual description of the cause of the delay. The string can change\nwithout notice because it is often generated by another service (such as\nCompute Engine)."]
        #[serde(rename = "cause", default)]
        pub cause: ::std::option::Option<String>,
        #[doc = "If the delay was caused by a resource shortage, this field lists the\nCompute Engine metrics that are preventing this operation from running\n(for example, `CPUS` or `INSTANCES`). If the particular metric is not\nknown, a single `UNKNOWN` metric will be present."]
        #[serde(rename = "metrics", default)]
        pub metrics: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for DelayedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DelayedEvent {
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
    pub struct Disk {
        #[doc = "A user-supplied name for the disk. Used when mounting the disk into\nactions. The name must contain only upper and lowercase alphanumeric\ncharacters and hypens and cannot start with a hyphen."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The Compute Engine disk type. If unspecified, `pd-standard` is used."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The size, in GB, of the disk to attach. If the size is not\nspecified, a default is chosen to ensure reasonable I/O performance.\n\nIf the disk type is specified as `local-ssd`, multiple local drives are\nautomatically combined to provide the requested size. Note, however, that\neach physical SSD is 375GB in size, and no more than 8 drives can be\nattached to a single instance."]
        #[serde(rename = "sizeGb", default)]
        pub size_gb: ::std::option::Option<i32>,
        #[doc = "An optional image to put on the disk before attaching it to the VM."]
        #[serde(rename = "sourceImage", default)]
        pub source_image: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Disk {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Disk {
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
    pub struct DiskStatus {
        #[doc = "Free disk space."]
        #[serde(rename = "freeSpaceBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub free_space_bytes: ::std::option::Option<u64>,
        #[doc = "Total disk space."]
        #[serde(rename = "totalSpaceBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_space_bytes: ::std::option::Option<u64>,
    }
    impl ::google_field_selector::FieldSelector for DiskStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiskStatus {
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
    pub struct Empty;
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
    pub struct Event {
        #[doc = "A human-readable description of the event. Note that these strings can\nchange at any time without notice. Any application logic must use the\ninformation in the `details` field."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Machine-readable details about the event."]
        #[serde(rename = "details", default)]
        pub details:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The time at which the event occurred."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Event {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Event {
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
    pub struct FailedEvent {
        #[doc = "The human-readable description of the cause of the failure."]
        #[serde(rename = "cause", default)]
        pub cause: ::std::option::Option<String>,
        #[doc = "The Google standard error code that best describes this failure."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::FailedEventCode>,
    }
    impl ::google_field_selector::FieldSelector for FailedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FailedEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FailedEventCode {
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
    impl FailedEventCode {
        pub fn as_str(self) -> &'static str {
            match self {
                FailedEventCode::Aborted => "ABORTED",
                FailedEventCode::AlreadyExists => "ALREADY_EXISTS",
                FailedEventCode::Cancelled => "CANCELLED",
                FailedEventCode::DataLoss => "DATA_LOSS",
                FailedEventCode::DeadlineExceeded => "DEADLINE_EXCEEDED",
                FailedEventCode::FailedPrecondition => "FAILED_PRECONDITION",
                FailedEventCode::Internal => "INTERNAL",
                FailedEventCode::InvalidArgument => "INVALID_ARGUMENT",
                FailedEventCode::NotFound => "NOT_FOUND",
                FailedEventCode::Ok => "OK",
                FailedEventCode::OutOfRange => "OUT_OF_RANGE",
                FailedEventCode::PermissionDenied => "PERMISSION_DENIED",
                FailedEventCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
                FailedEventCode::Unauthenticated => "UNAUTHENTICATED",
                FailedEventCode::Unavailable => "UNAVAILABLE",
                FailedEventCode::Unimplemented => "UNIMPLEMENTED",
                FailedEventCode::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for FailedEventCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FailedEventCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FailedEventCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABORTED" => FailedEventCode::Aborted,
                "ALREADY_EXISTS" => FailedEventCode::AlreadyExists,
                "CANCELLED" => FailedEventCode::Cancelled,
                "DATA_LOSS" => FailedEventCode::DataLoss,
                "DEADLINE_EXCEEDED" => FailedEventCode::DeadlineExceeded,
                "FAILED_PRECONDITION" => FailedEventCode::FailedPrecondition,
                "INTERNAL" => FailedEventCode::Internal,
                "INVALID_ARGUMENT" => FailedEventCode::InvalidArgument,
                "NOT_FOUND" => FailedEventCode::NotFound,
                "OK" => FailedEventCode::Ok,
                "OUT_OF_RANGE" => FailedEventCode::OutOfRange,
                "PERMISSION_DENIED" => FailedEventCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => FailedEventCode::ResourceExhausted,
                "UNAUTHENTICATED" => FailedEventCode::Unauthenticated,
                "UNAVAILABLE" => FailedEventCode::Unavailable,
                "UNIMPLEMENTED" => FailedEventCode::Unimplemented,
                "UNKNOWN" => FailedEventCode::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FailedEventCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FailedEventCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(rename = "operations", default)]
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
    pub struct Metadata {
        #[doc = "The time at which the operation was created by the API."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The time at which execution was completed and resources were cleaned up."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The list of events that have happened so far during the execution of this\noperation."]
        #[serde(rename = "events", default)]
        pub events: ::std::option::Option<Vec<crate::schemas::Event>>,
        #[doc = "The user-defined labels associated with this operation."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The pipeline this operation represents."]
        #[serde(rename = "pipeline", default)]
        pub pipeline: ::std::option::Option<crate::schemas::Pipeline>,
        #[doc = "The first time at which resources were allocated to execute the pipeline."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Metadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metadata {
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
    pub struct Mount {
        #[doc = "The name of the disk to mount, as specified in the resources section."]
        #[serde(rename = "disk", default)]
        pub disk: ::std::option::Option<String>,
        #[doc = "The path to mount the disk inside the container."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
        #[doc = "If true, the disk is mounted read-only inside the container."]
        #[serde(rename = "readOnly", default)]
        pub read_only: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Mount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Mount {
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
    pub struct Network {
        #[doc = "The network name to attach the VM's network interface to. The value will\nbe prefixed with `global/networks/` unless it contains a `/`, in which\ncase it is assumed to be a fully specified network resource URL.\n\nIf unspecified, the global default network is used."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "If the specified network is configured for custom subnet creation, the\nname of the subnetwork to attach the instance to must be specified here.\n\nThe value is prefixed with `regions/*/subnetworks/` unless it contains a\n`/`, in which case it is assumed to be a fully specified subnetwork\nresource URL.\n\nIf the `*` character appears in the value, it is replaced with the region\nthat the virtual machine has been allocated in."]
        #[serde(rename = "subnetwork", default)]
        pub subnetwork: ::std::option::Option<String>,
        #[doc = "If set to true, do not attach a public IP address to the VM. Note that\nwithout a public IP address, additional configuration is required to\nallow the VM to access Google services.\n\nSee https://cloud.google.com/vpc/docs/configure-private-google-access\nfor more information."]
        #[serde(rename = "usePrivateAddress", default)]
        pub use_private_address: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Network {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Network {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "An OperationMetadata or Metadata object. This will always be returned with the Operation."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. For example: `operations/CJHU7Oi_ChDrveSpBRjfuL-qzoWAgEw`"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "An Empty object."]
        #[serde(rename = "response", default)]
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
    pub struct OperationEvent {
        #[doc = "Required description of event."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Optional time of when event finished. An event can have a start time and no\nfinish time. If an event has a finish time, there must be a start time."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Optional time of when event started."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OperationEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OperationEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct OperationMetadata {
        #[doc = "This field is deprecated. Use `labels` instead. Optionally provided by the\ncaller when submitting the request that creates the operation."]
        #[serde(rename = "clientId", default)]
        pub client_id: ::std::option::Option<String>,
        #[doc = "The time at which the job was submitted to the Genomics service."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The time at which the job stopped running."]
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Optional event messages that were generated during the job's execution.\nThis also contains any warnings that were generated during import\nor export."]
        #[serde(rename = "events", default)]
        pub events: ::std::option::Option<Vec<crate::schemas::OperationEvent>>,
        #[doc = "Optionally provided by the caller when submitting the request that creates\nthe operation."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The Google Cloud Project in which the job is scoped."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The original request that started the operation. Note that this will be in\ncurrent version of the API. If the operation was started with v1beta2 API\nand a GetOperation is performed on v1 API, a v1 request will be returned."]
        #[serde(rename = "request", default)]
        pub request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Runtime metadata on this Operation."]
        #[serde(rename = "runtimeMetadata", default)]
        pub runtime_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The time at which the job began to run."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Pipeline {
        #[doc = "The list of actions to execute, in the order they are specified."]
        #[serde(rename = "actions", default)]
        pub actions: ::std::option::Option<Vec<crate::schemas::Action>>,
        #[doc = "The environment to pass into every action. Each action can also specify\nadditional environment variables but cannot delete an entry from this map\n(though they can overwrite it with a different value)."]
        #[serde(rename = "environment", default)]
        pub environment: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The resources required for execution."]
        #[serde(rename = "resources", default)]
        pub resources: ::std::option::Option<crate::schemas::Resources>,
        #[doc = "The maximum amount of time to give the pipeline to complete.  This includes\nthe time spent waiting for a worker to be allocated.  If the pipeline fails\nto complete before the timeout, it will be cancelled and the error code\nwill be set to DEADLINE_EXCEEDED.\n\nIf unspecified, it will default to 7 days."]
        #[serde(rename = "timeout", default)]
        pub timeout: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Pipeline {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Pipeline {
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
    pub struct PullStartedEvent {
        #[doc = "The URI of the image that was pulled."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PullStartedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PullStartedEvent {
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
    pub struct PullStoppedEvent {
        #[doc = "The URI of the image that was pulled."]
        #[serde(rename = "imageUri", default)]
        pub image_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PullStoppedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PullStoppedEvent {
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
    pub struct Resources {
        #[doc = "The project ID to allocate resources in."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The list of regions allowed for VM allocation. If set, the `zones` field\nmust not be set."]
        #[serde(rename = "regions", default)]
        pub regions: ::std::option::Option<Vec<String>>,
        #[doc = "The virtual machine specification."]
        #[serde(rename = "virtualMachine", default)]
        pub virtual_machine: ::std::option::Option<crate::schemas::VirtualMachine>,
        #[doc = "The list of zones allowed for VM allocation. If set, the `regions` field\nmust not be set."]
        #[serde(rename = "zones", default)]
        pub zones: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Resources {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Resources {
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
    pub struct RunPipelineRequest {
        #[doc = "User-defined labels to associate with the returned operation. These\nlabels are not propagated to any Google Cloud Platform resources used by\nthe operation, and can be modified at any time.\n\nTo associate labels with resources created while executing the operation,\nsee the appropriate resource message (for example, `VirtualMachine`)."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Required. The description of the pipeline to run."]
        #[serde(rename = "pipeline", default)]
        pub pipeline: ::std::option::Option<crate::schemas::Pipeline>,
    }
    impl ::google_field_selector::FieldSelector for RunPipelineRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunPipelineRequest {
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
    pub struct RunPipelineResponse;
    impl ::google_field_selector::FieldSelector for RunPipelineResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunPipelineResponse {
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
    pub struct RuntimeMetadata {
        #[doc = "Execution information specific to Google Compute Engine."]
        #[serde(rename = "computeEngine", default)]
        pub compute_engine: ::std::option::Option<crate::schemas::ComputeEngine>,
    }
    impl ::google_field_selector::FieldSelector for RuntimeMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuntimeMetadata {
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
    pub struct Secret {
        #[doc = "The value of the cipherText response from the `encrypt` method. This field\nis intentionally unaudited."]
        #[serde(rename = "cipherText", default)]
        pub cipher_text: ::std::option::Option<String>,
        #[doc = "The name of the Cloud KMS key that will be used to decrypt the secret\nvalue. The VM service account must have the required permissions and\nauthentication scopes to invoke the `decrypt` method on the specified key."]
        #[serde(rename = "keyName", default)]
        pub key_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Secret {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Secret {
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
    pub struct ServiceAccount {
        #[doc = "Email address of the service account. If not specified, the default\nCompute Engine service account for the project will be used."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "List of scopes to be enabled for this service account on the VM, in\naddition to the cloud-platform API scope that will be added by default."]
        #[serde(rename = "scopes", default)]
        pub scopes: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ServiceAccount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceAccount {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
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
    pub struct UnexpectedExitStatusEvent {
        #[doc = "The numeric ID of the action that started the container."]
        #[serde(rename = "actionId", default)]
        pub action_id: ::std::option::Option<i32>,
        #[doc = "The exit status of the container."]
        #[serde(rename = "exitStatus", default)]
        pub exit_status: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for UnexpectedExitStatusEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnexpectedExitStatusEvent {
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
    pub struct VirtualMachine {
        #[doc = "The list of accelerators to attach to the VM."]
        #[serde(rename = "accelerators", default)]
        pub accelerators: ::std::option::Option<Vec<crate::schemas::Accelerator>>,
        #[doc = "The size of the boot disk, in GB. The boot disk must be large\nenough to accommodate all of the Docker images from each action in the\npipeline at the same time. If not specified, a small but reasonable\ndefault value is used."]
        #[serde(rename = "bootDiskSizeGb", default)]
        pub boot_disk_size_gb: ::std::option::Option<i32>,
        #[doc = "The host operating system image to use.\n\nCurrently, only Container-Optimized OS images can be used.\n\nThe default value is `projects/cos-cloud/global/images/family/cos-stable`,\nwhich selects the latest stable release of Container-Optimized OS.\n\nThis option is provided to allow testing against the beta release of the\noperating system to ensure that the new version does not interact\nnegatively with production pipelines.\n\nTo test a pipeline against the beta release of Container-Optimized OS,\nuse the value `projects/cos-cloud/global/images/family/cos-beta`."]
        #[serde(rename = "bootImage", default)]
        pub boot_image: ::std::option::Option<String>,
        #[doc = "The CPU platform to request. An instance based on a newer platform can be\nallocated, but never one with fewer capabilities. The value of this\nparameter must be a valid Compute Engine CPU platform name (such as \"Intel\nSkylake\"). This parameter is only useful for carefully optimized work\nloads where the CPU platform has a significant impact.\n\nFor more information about the effect of this parameter, see\nhttps://cloud.google.com/compute/docs/instances/specify-min-cpu-platform."]
        #[serde(rename = "cpuPlatform", default)]
        pub cpu_platform: ::std::option::Option<String>,
        #[doc = "The list of disks to create and attach to the VM."]
        #[serde(rename = "disks", default)]
        pub disks: ::std::option::Option<Vec<crate::schemas::Disk>>,
        #[doc = "Whether Stackdriver monitoring should be enabled on the VM."]
        #[serde(rename = "enableStackdriverMonitoring", default)]
        pub enable_stackdriver_monitoring: ::std::option::Option<bool>,
        #[doc = "Optional set of labels to apply to the VM and any attached disk resources.\nThese labels must adhere to the [name and value\nrestrictions](https://cloud.google.com/compute/docs/labeling-resources) on\nVM labels imposed by Compute Engine.\n\nLabels keys with the prefix 'google-' are reserved for use by Google.\n\nLabels applied at creation time to the VM. Applied on a best-effort basis\nto attached disk resources shortly after VM creation."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Required. The machine type of the virtual machine to create. Must be the short name\nof a standard machine type (such as \"n1-standard-1\") or a custom machine\ntype (such as \"custom-1-4096\", where \"1\" indicates the number of vCPUs and\n\"4096\" indicates the memory in MB). See\n[Creating an instance with a custom machine\ntype](https://cloud.google.com/compute/docs/instances/creating-instance-with-custom-machine-type#create)\nfor more specifications on creating a custom machine type."]
        #[serde(rename = "machineType", default)]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "The VM network configuration."]
        #[serde(rename = "network", default)]
        pub network: ::std::option::Option<crate::schemas::Network>,
        #[doc = "The NVIDIA driver version to use when attaching an NVIDIA GPU accelerator.\nThe version specified here must be compatible with the GPU libraries\ncontained in the container being executed, and must be one of the drivers\nhosted in the `nvidia-drivers-us-public` bucket on Google Cloud Storage."]
        #[serde(rename = "nvidiaDriverVersion", default)]
        pub nvidia_driver_version: ::std::option::Option<String>,
        #[doc = "If true, allocate a preemptible VM."]
        #[serde(rename = "preemptible", default)]
        pub preemptible: ::std::option::Option<bool>,
        #[doc = "The service account to install on the VM. This account does not need\nany permissions other than those required by the pipeline."]
        #[serde(rename = "serviceAccount", default)]
        pub service_account: ::std::option::Option<crate::schemas::ServiceAccount>,
    }
    impl ::google_field_selector::FieldSelector for VirtualMachine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VirtualMachine {
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
    pub struct WorkerAssignedEvent {
        #[doc = "The worker's instance name."]
        #[serde(rename = "instance", default)]
        pub instance: ::std::option::Option<String>,
        #[doc = "The machine type that was assigned for the worker."]
        #[serde(rename = "machineType", default)]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "The zone the worker is running in."]
        #[serde(rename = "zone", default)]
        pub zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WorkerAssignedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WorkerAssignedEvent {
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
    pub struct WorkerReleasedEvent {
        #[doc = "The worker's instance name."]
        #[serde(rename = "instance", default)]
        pub instance: ::std::option::Option<String>,
        #[doc = "The zone the worker was running in."]
        #[serde(rename = "zone", default)]
        pub zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WorkerReleasedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WorkerReleasedEvent {
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
    pub struct WorkerStatus {
        #[doc = "Status of attached disks."]
        #[serde(rename = "attachedDisks", default)]
        pub attached_disks:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::DiskStatus>>,
        #[doc = "Status of the boot disk."]
        #[serde(rename = "bootDisk", default)]
        pub boot_disk: ::std::option::Option<crate::schemas::DiskStatus>,
        #[doc = "Free RAM."]
        #[serde(rename = "freeRamBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub free_ram_bytes: ::std::option::Option<u64>,
        #[doc = "Total RAM."]
        #[serde(rename = "totalRamBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_ram_bytes: ::std::option::Option<u64>,
        #[doc = "System uptime."]
        #[serde(rename = "uptimeSeconds", default)]
        #[serde(with = "crate::parsed_string")]
        pub uptime_seconds: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for WorkerStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WorkerStatus {
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
    #[doc = "Actions that can be performed on the pipelines resource"]
    pub fn pipelines(&self) -> crate::resources::pipelines::PipelinesActions<A> {
        crate::resources::pipelines::PipelinesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions<A> {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the workers resource"]
    pub fn workers(&self) -> crate::resources::workers::WorkersActions<A> {
        crate::resources::workers::WorkersActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod pipelines {
        pub mod params {}
        pub struct PipelinesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> PipelinesActions<'a, A> {
            #[doc = "Runs a pipeline.  The returned Operation's metadata field will contain a\ngoogle.genomics.v2alpha1.Metadata object describing the status of the\npipeline execution.  The [response] field will contain a\ngoogle.genomics.v2alpha1.RunPipelineResponse object if the pipeline\ncompletes successfully.\n\n**Note:** Before you can use this method, the Genomics Service Agent\nmust have access to your project. This is done automatically when the\nCloud Genomics API is first enabled, but if you delete this permission,\nor if you enabled the Cloud Genomics API before the v2alpha1 API\nlaunch, you must disable and re-enable the API to grant the Genomics\nService Agent the required permissions.\nAuthorization requires the following [Google\nIAM](https://cloud.google.com/iam/) permission:\n\n* `genomics.operations.create`"]
            pub fn run(&self, request: crate::schemas::RunPipelineRequest) -> RunRequestBuilder<A> {
                RunRequestBuilder {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct RunRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::RunPipelineRequest,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> RunRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://genomics.googleapis.com/".to_owned();
                output.push_str("v2alpha1/pipelines:run");
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
    }
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ProjectsActions<'a, A> {
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
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> OperationsActions<'a, A> {
                #[doc = "Starts asynchronous cancellation on a long-running operation.\nThe server makes a best effort to cancel the operation, but success is not\nguaranteed. Clients may use Operations.GetOperation\nor Operations.ListOperations\nto check whether the cancellation succeeded or the operation completed\ndespite cancellation.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission:\n\n* `genomics.operations.cancel`"]
                pub fn cancel(
                    &self,
                    request: crate::schemas::CancelOperationRequest,
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
                #[doc = "Gets the latest state of a long-running operation.\nClients can use this method to poll the operation result at intervals as\nrecommended by the API service.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission:\n\n* `genomics.operations.get`"]
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
                #[doc = "Lists operations that match the specified filter in the request.\nAuthorization requires the following [Google IAM](https://cloud.google.com/iam) permission:\n\n* `genomics.operations.list`"]
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
                request: crate::schemas::CancelOperationRequest,
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
                ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://genomics.googleapis.com/".to_owned();
                    output.push_str("v2alpha1/");
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
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://genomics.googleapis.com/".to_owned();
                    output.push_str("v2alpha1/");
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
                #[doc = "A string for filtering Operations.\nIn v2alpha1, the following filter fields are supported:\n\n* createTime: The time this job was created\n* events: The set of event (names) that have occurred while running\n  the pipeline.  The : operator can be used to determine if a\n  particular event has occurred.\n* error: If the pipeline is running, this value is NULL.  Once the\n  pipeline finishes, the value is the standard Google error code.\n* labels.key or labels.\"key with space\" where key is a label key.\n* done: If the pipeline is running, this value is false. Once the\n  pipeline finishes, the value is true.\n\nIn v1 and v1alpha2, the following filter fields are supported:\n\n* projectId: Required. Corresponds to\n  OperationMetadata.projectId.\n* createTime: The time this job was created, in seconds from the\n  [epoch](http://en.wikipedia.org/wiki/Unix_time). Can use `>=` and/or `<=`\n  operators.\n* status: Can be `RUNNING`, `SUCCESS`, `FAILURE`, or `CANCELED`. Only\n  one status may be specified.\n* labels.key where key is a label key.\n\nExamples:\n\n* `projectId = my-project AND createTime >= 1432140000`\n* `projectId = my-project AND createTime >= 1432140000 AND createTime <= 1432150000 AND status = RUNNING`\n* `projectId = my-project AND labels.color = *`\n* `projectId = my-project AND labels.color = red`"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The maximum number of results to return. The maximum value is 256."]
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
                ) -> Result<crate::schemas::ListOperationsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListOperationsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://genomics.googleapis.com/".to_owned();
                    output.push_str("v2alpha1/");
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
    pub mod workers {
        pub mod params {}
        pub struct WorkersActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> WorkersActions<'a, A> {
            #[doc = "The worker uses this method to retrieve the assigned operation and\nprovide periodic status updates."]
            pub fn check_in(
                &self,
                request: crate::schemas::CheckInRequest,
                id: impl Into<String>,
            ) -> CheckInRequestBuilder<A> {
                CheckInRequestBuilder {
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
                    id: id.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CheckInRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::CheckInRequest,
            id: String,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> CheckInRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::CheckInResponse, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CheckInResponse, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://genomics.googleapis.com/".to_owned();
                output.push_str("v2alpha1/workers/");
                {
                    let var_as_str = &self.id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":checkIn");
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
}
