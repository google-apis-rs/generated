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
    pub struct ArtifactObjects {
        #[doc = "Cloud Storage bucket and optional object path, in the form\n\"gs://bucket/path/to/somewhere/\". (see [Bucket Name\nRequirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).\n\nFiles in the workspace matching any path pattern will be uploaded to\nCloud Storage with this location as a prefix."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "Path globs used to match files in the build's workspace."]
        #[serde(rename = "paths", default)]
        pub paths: Option<Vec<String>>,
        #[doc = "Output only. Stores timing information for pushing all artifact objects."]
        #[serde(rename = "timing", default)]
        pub timing: Option<crate::schemas::TimeSpan>,
    }
    impl ::field_selector::FieldSelector for ArtifactObjects {
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
    pub struct ArtifactResult {
        #[doc = "The file hash of the artifact."]
        #[serde(rename = "fileHash", default)]
        pub file_hash: Option<Vec<crate::schemas::FileHashes>>,
        #[doc = "The path of an artifact in a Google Cloud Storage bucket, with the\ngeneration number. For example,\n`gs://mybucket/path/to/output.jar#generation`."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
    }
    impl ::field_selector::FieldSelector for ArtifactResult {
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
    pub struct Artifacts {
        #[doc = "A list of images to be pushed upon the successful completion of all build\nsteps.\n\nThe images will be pushed using the builder service account's credentials.\n\nThe digests of the pushed images will be stored in the Build resource's\nresults field.\n\nIf any of the images fail to be pushed, the build is marked FAILURE."]
        #[serde(rename = "images", default)]
        pub images: Option<Vec<String>>,
        #[doc = "A list of objects to be uploaded to Cloud Storage upon successful\ncompletion of all build steps.\n\nFiles in the workspace matching specified paths globs will be uploaded to\nthe specified Cloud Storage location using the builder service account's\ncredentials.\n\nThe location and generation of the uploaded objects will be stored in the\nBuild resource's results field.\n\nIf any objects fail to be pushed, the build is marked FAILURE."]
        #[serde(rename = "objects", default)]
        pub objects: Option<crate::schemas::ArtifactObjects>,
    }
    impl ::field_selector::FieldSelector for Artifacts {
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
    pub enum BuildStatus {
        #[doc = "Status of the build is unknown."]
        StatusUnknown,
        #[doc = "Build or step is queued; work has not yet begun."]
        Queued,
        #[doc = "Build or step is being executed."]
        Working,
        #[doc = "Build or step finished successfully."]
        Success,
        #[doc = "Build or step failed to complete successfully."]
        Failure,
        #[doc = "Build or step failed due to an internal cause."]
        InternalError,
        #[doc = "Build or step took longer than was allowed."]
        Timeout,
        #[doc = "Build or step was canceled by a user."]
        Cancelled,
    }
    impl BuildStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildStatus::StatusUnknown => "STATUS_UNKNOWN",
                BuildStatus::Queued => "QUEUED",
                BuildStatus::Working => "WORKING",
                BuildStatus::Success => "SUCCESS",
                BuildStatus::Failure => "FAILURE",
                BuildStatus::InternalError => "INTERNAL_ERROR",
                BuildStatus::Timeout => "TIMEOUT",
                BuildStatus::Cancelled => "CANCELLED",
            }
        }
    }
    impl ::std::fmt::Display for BuildStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNKNOWN" => BuildStatus::StatusUnknown,
                "QUEUED" => BuildStatus::Queued,
                "WORKING" => BuildStatus::Working,
                "SUCCESS" => BuildStatus::Success,
                "FAILURE" => BuildStatus::Failure,
                "INTERNAL_ERROR" => BuildStatus::InternalError,
                "TIMEOUT" => BuildStatus::Timeout,
                "CANCELLED" => BuildStatus::Cancelled,
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
    pub struct Build {
        #[doc = "Artifacts produced by the build that should be uploaded upon\nsuccessful completion of all build steps."]
        #[serde(rename = "artifacts", default)]
        pub artifacts: Option<crate::schemas::Artifacts>,
        #[doc = "Output only. The ID of the `BuildTrigger` that triggered this build, if it\nwas triggered automatically."]
        #[serde(rename = "buildTriggerId", default)]
        pub build_trigger_id: Option<String>,
        #[doc = "Output only. Time at which the request to create the build was received."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Output only. Time at which execution of the build was finished.\n\nThe difference between finish_time and start_time is the duration of the\nbuild's execution."]
        #[serde(rename = "finishTime", default)]
        pub finish_time: Option<String>,
        #[doc = "Output only. Unique identifier of the build."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "A list of images to be pushed upon the successful completion of all build\nsteps.\n\nThe images are pushed using the builder service account's credentials.\n\nThe digests of the pushed images will be stored in the `Build` resource's\nresults field.\n\nIf any of the images fail to be pushed, the build status is marked\n`FAILURE`."]
        #[serde(rename = "images", default)]
        pub images: Option<Vec<String>>,
        #[doc = "Output only. URL to logs for this build in Google Cloud Console."]
        #[serde(rename = "logUrl", default)]
        pub log_url: Option<String>,
        #[doc = "Google Cloud Storage bucket where logs should be written (see\n[Bucket Name\nRequirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).\nLogs file names will be of the format `${logs_bucket}/log-${build_id}.txt`."]
        #[serde(rename = "logsBucket", default)]
        pub logs_bucket: Option<String>,
        #[doc = "Special options for this build."]
        #[serde(rename = "options", default)]
        pub options: Option<crate::schemas::BuildOptions>,
        #[doc = "Output only. ID of the project."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Output only. Results of the build."]
        #[serde(rename = "results", default)]
        pub results: Option<crate::schemas::Results>,
        #[doc = "Secrets to decrypt using Cloud Key Management Service."]
        #[serde(rename = "secrets", default)]
        pub secrets: Option<Vec<crate::schemas::Secret>>,
        #[doc = "The location of the source files to build."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
        #[doc = "Output only. A permanent fixed identifier for source."]
        #[serde(rename = "sourceProvenance", default)]
        pub source_provenance: Option<crate::schemas::SourceProvenance>,
        #[doc = "Output only. Time at which execution of the build was started."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "Output only. Status of the build."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::BuildStatus>,
        #[doc = "Output only. Customer-readable message about the current status."]
        #[serde(rename = "statusDetail", default)]
        pub status_detail: Option<String>,
        #[doc = "Required. The operations to be performed on the workspace."]
        #[serde(rename = "steps", default)]
        pub steps: Option<Vec<crate::schemas::BuildStep>>,
        #[doc = "Substitutions data for `Build` resource."]
        #[serde(rename = "substitutions", default)]
        pub substitutions: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Tags for annotation of a `Build`. These are not docker tags."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
        #[doc = "Amount of time that this build should be allowed to run, to second\ngranularity. If this amount of time elapses, work on the build will cease\nand the build status will be `TIMEOUT`.\n\nDefault time is ten minutes."]
        #[serde(rename = "timeout", default)]
        pub timeout: Option<String>,
        #[doc = "Output only. Stores timing information for phases of the build. Valid keys\nare:\n\n* BUILD: time to execute all build steps\n* PUSH: time to push all specified images.\n* FETCHSOURCE: time to fetch source.\n\nIf the build does not specify source or images,\nthese keys will not be included."]
        #[serde(rename = "timing", default)]
        pub timing: Option<::std::collections::BTreeMap<String, crate::schemas::TimeSpan>>,
    }
    impl ::field_selector::FieldSelector for Build {
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
    pub struct BuildOperationMetadata {
        #[doc = "The build that the operation is tracking."]
        #[serde(rename = "build", default)]
        pub build: Option<crate::schemas::Build>,
    }
    impl ::field_selector::FieldSelector for BuildOperationMetadata {
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
    pub enum BuildOptionsLogStreamingOption {
        #[doc = "Service may automatically determine build log streaming behavior."]
        StreamDefault,
        #[doc = "Build logs should be streamed to Google Cloud Storage."]
        StreamOn,
        #[doc = "Build logs should not be streamed to Google Cloud Storage; they will be\nwritten when the build is completed."]
        StreamOff,
    }
    impl BuildOptionsLogStreamingOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildOptionsLogStreamingOption::StreamDefault => "STREAM_DEFAULT",
                BuildOptionsLogStreamingOption::StreamOn => "STREAM_ON",
                BuildOptionsLogStreamingOption::StreamOff => "STREAM_OFF",
            }
        }
    }
    impl ::std::fmt::Display for BuildOptionsLogStreamingOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildOptionsLogStreamingOption {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildOptionsLogStreamingOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STREAM_DEFAULT" => BuildOptionsLogStreamingOption::StreamDefault,
                "STREAM_ON" => BuildOptionsLogStreamingOption::StreamOn,
                "STREAM_OFF" => BuildOptionsLogStreamingOption::StreamOff,
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
    pub enum BuildOptionsLogging {
        #[doc = "The service determines the logging mode. The default is `LEGACY`. Do not\nrely on the default logging behavior as it may change in the future."]
        LoggingUnspecified,
        #[doc = "Stackdriver logging and Cloud Storage logging are enabled."]
        Legacy,
        #[doc = "Only Cloud Storage logging is enabled."]
        GcsOnly,
    }
    impl BuildOptionsLogging {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildOptionsLogging::LoggingUnspecified => "LOGGING_UNSPECIFIED",
                BuildOptionsLogging::Legacy => "LEGACY",
                BuildOptionsLogging::GcsOnly => "GCS_ONLY",
            }
        }
    }
    impl ::std::fmt::Display for BuildOptionsLogging {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildOptionsLogging {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildOptionsLogging {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LOGGING_UNSPECIFIED" => BuildOptionsLogging::LoggingUnspecified,
                "LEGACY" => BuildOptionsLogging::Legacy,
                "GCS_ONLY" => BuildOptionsLogging::GcsOnly,
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
    pub enum BuildOptionsMachineType {
        #[doc = "Standard machine type."]
        Unspecified,
        #[doc = "Highcpu machine with 8 CPUs."]
        N1Highcpu8,
        #[doc = "Highcpu machine with 32 CPUs."]
        N1Highcpu32,
    }
    impl BuildOptionsMachineType {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildOptionsMachineType::Unspecified => "UNSPECIFIED",
                BuildOptionsMachineType::N1Highcpu8 => "N1_HIGHCPU_8",
                BuildOptionsMachineType::N1Highcpu32 => "N1_HIGHCPU_32",
            }
        }
    }
    impl ::std::fmt::Display for BuildOptionsMachineType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildOptionsMachineType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildOptionsMachineType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => BuildOptionsMachineType::Unspecified,
                "N1_HIGHCPU_8" => BuildOptionsMachineType::N1Highcpu8,
                "N1_HIGHCPU_32" => BuildOptionsMachineType::N1Highcpu32,
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
    pub enum BuildOptionsRequestedVerifyOption {
        #[doc = "Not a verifiable build. (default)"]
        NotVerified,
        #[doc = "Verified build."]
        Verified,
    }
    impl BuildOptionsRequestedVerifyOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildOptionsRequestedVerifyOption::NotVerified => "NOT_VERIFIED",
                BuildOptionsRequestedVerifyOption::Verified => "VERIFIED",
            }
        }
    }
    impl ::std::fmt::Display for BuildOptionsRequestedVerifyOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildOptionsRequestedVerifyOption {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildOptionsRequestedVerifyOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOT_VERIFIED" => BuildOptionsRequestedVerifyOption::NotVerified,
                "VERIFIED" => BuildOptionsRequestedVerifyOption::Verified,
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
    pub enum BuildOptionsSourceProvenanceHashItems {}
    impl BuildOptionsSourceProvenanceHashItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for BuildOptionsSourceProvenanceHashItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildOptionsSourceProvenanceHashItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildOptionsSourceProvenanceHashItems {
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
    pub enum BuildOptionsSubstitutionOption {
        #[doc = "Fails the build if error in substitutions checks, like missing\na substitution in the template or in the map."]
        MustMatch,
        #[doc = "Do not fail the build if error in substitutions checks."]
        AllowLoose,
    }
    impl BuildOptionsSubstitutionOption {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildOptionsSubstitutionOption::MustMatch => "MUST_MATCH",
                BuildOptionsSubstitutionOption::AllowLoose => "ALLOW_LOOSE",
            }
        }
    }
    impl ::std::fmt::Display for BuildOptionsSubstitutionOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildOptionsSubstitutionOption {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildOptionsSubstitutionOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MUST_MATCH" => BuildOptionsSubstitutionOption::MustMatch,
                "ALLOW_LOOSE" => BuildOptionsSubstitutionOption::AllowLoose,
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
    pub struct BuildOptions {
        #[doc = "Requested disk size for the VM that runs the build. Note that this is *NOT*\n\"disk free\"; some of the space will be used by the operating system and\nbuild utilities. Also note that this is the minimum disk size that will be\nallocated for the build -- the build may run with a larger disk than\nrequested. At present, the maximum disk size is 1000GB; builds that request\nmore than the maximum are rejected with an error."]
        #[serde(rename = "diskSizeGb", default)]
        #[serde(with = "crate::parsed_string")]
        pub disk_size_gb: Option<i64>,
        #[doc = "A list of global environment variable definitions that will exist for all\nbuild steps in this build. If a variable is defined in both globally and in\na build step, the variable will use the build step value.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\"\nbeing given the value \"VALUE\"."]
        #[serde(rename = "env", default)]
        pub env: Option<Vec<String>>,
        #[doc = "Option to define build log streaming behavior to Google Cloud\nStorage."]
        #[serde(rename = "logStreamingOption", default)]
        pub log_streaming_option: Option<crate::schemas::BuildOptionsLogStreamingOption>,
        #[doc = "Option to specify the logging mode, which determines where the logs are\nstored."]
        #[serde(rename = "logging", default)]
        pub logging: Option<crate::schemas::BuildOptionsLogging>,
        #[doc = "Compute Engine machine type on which to run the build."]
        #[serde(rename = "machineType", default)]
        pub machine_type: Option<crate::schemas::BuildOptionsMachineType>,
        #[doc = "Requested verifiability options."]
        #[serde(rename = "requestedVerifyOption", default)]
        pub requested_verify_option: Option<crate::schemas::BuildOptionsRequestedVerifyOption>,
        #[doc = "A list of global environment variables, which are encrypted using a Cloud\nKey Management Service crypto key. These values must be specified in the\nbuild's `Secret`. These variables will be available to all build steps\nin this build."]
        #[serde(rename = "secretEnv", default)]
        pub secret_env: Option<Vec<String>>,
        #[doc = "Requested hash for SourceProvenance."]
        #[serde(rename = "sourceProvenanceHash", default)]
        pub source_provenance_hash:
            Option<Vec<crate::schemas::BuildOptionsSourceProvenanceHashItems>>,
        #[doc = "Option to specify behavior when there is an error in the substitution\nchecks."]
        #[serde(rename = "substitutionOption", default)]
        pub substitution_option: Option<crate::schemas::BuildOptionsSubstitutionOption>,
        #[doc = "Global list of volumes to mount for ALL build steps\n\nEach volume is created as an empty volume prior to starting the build\nprocess. Upon completion of the build, volumes and their contents are\ndiscarded. Global volume names and paths cannot conflict with the volumes\ndefined a build step.\n\nUsing a global volume in a build with only one step is not valid as\nit is indicative of a build request with an incorrect configuration."]
        #[serde(rename = "volumes", default)]
        pub volumes: Option<Vec<crate::schemas::Volume>>,
        #[doc = "Option to specify a `WorkerPool` for the build. User specifies the pool\nwith the format \"[WORKERPOOL_PROJECT_ID]/[WORKERPOOL_NAME]\".\nThis is an experimental field."]
        #[serde(rename = "workerPool", default)]
        pub worker_pool: Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildOptions {
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
    pub enum BuildStepStatus {
        #[doc = "Status of the build is unknown."]
        StatusUnknown,
        #[doc = "Build or step is queued; work has not yet begun."]
        Queued,
        #[doc = "Build or step is being executed."]
        Working,
        #[doc = "Build or step finished successfully."]
        Success,
        #[doc = "Build or step failed to complete successfully."]
        Failure,
        #[doc = "Build or step failed due to an internal cause."]
        InternalError,
        #[doc = "Build or step took longer than was allowed."]
        Timeout,
        #[doc = "Build or step was canceled by a user."]
        Cancelled,
    }
    impl BuildStepStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildStepStatus::StatusUnknown => "STATUS_UNKNOWN",
                BuildStepStatus::Queued => "QUEUED",
                BuildStepStatus::Working => "WORKING",
                BuildStepStatus::Success => "SUCCESS",
                BuildStepStatus::Failure => "FAILURE",
                BuildStepStatus::InternalError => "INTERNAL_ERROR",
                BuildStepStatus::Timeout => "TIMEOUT",
                BuildStepStatus::Cancelled => "CANCELLED",
            }
        }
    }
    impl ::std::fmt::Display for BuildStepStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildStepStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildStepStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNKNOWN" => BuildStepStatus::StatusUnknown,
                "QUEUED" => BuildStepStatus::Queued,
                "WORKING" => BuildStepStatus::Working,
                "SUCCESS" => BuildStepStatus::Success,
                "FAILURE" => BuildStepStatus::Failure,
                "INTERNAL_ERROR" => BuildStepStatus::InternalError,
                "TIMEOUT" => BuildStepStatus::Timeout,
                "CANCELLED" => BuildStepStatus::Cancelled,
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
    pub struct BuildStep {
        #[doc = "A list of arguments that will be presented to the step when it is started.\n\nIf the image used to run the step's container has an entrypoint, the `args`\nare used as arguments to that entrypoint. If the image does not define\nan entrypoint, the first element in args is used as the entrypoint,\nand the remainder will be used as arguments."]
        #[serde(rename = "args", default)]
        pub args: Option<Vec<String>>,
        #[doc = "Working directory to use when running this step's container.\n\nIf this value is a relative path, it is relative to the build's working\ndirectory. If this value is absolute, it may be outside the build's working\ndirectory, in which case the contents of the path may not be persisted\nacross build step executions, unless a `volume` for that path is specified.\n\nIf the build specifies a `RepoSource` with `dir` and a step with a `dir`,\nwhich specifies an absolute path, the `RepoSource` `dir` is ignored for\nthe step's execution."]
        #[serde(rename = "dir", default)]
        pub dir: Option<String>,
        #[doc = "Entrypoint to be used instead of the build step image's default entrypoint.\nIf unset, the image's default entrypoint is used."]
        #[serde(rename = "entrypoint", default)]
        pub entrypoint: Option<String>,
        #[doc = "A list of environment variable definitions to be used when running a step.\n\nThe elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\"\nbeing given the value \"VALUE\"."]
        #[serde(rename = "env", default)]
        pub env: Option<Vec<String>>,
        #[doc = "Unique identifier for this build step, used in `wait_for` to\nreference this build step as a dependency."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Required. The name of the container image that will run this particular\nbuild step.\n\nIf the image is available in the host's Docker daemon's cache, it\nwill be run directly. If not, the host will attempt to pull the image\nfirst, using the builder service account's credentials if necessary.\n\nThe Docker daemon's cache will already have the latest versions of all of\nthe officially supported build steps\n([https://github.com/GoogleCloudPlatform/cloud-builders](https://github.com/GoogleCloudPlatform/cloud-builders)).\nThe Docker daemon will also have cached many of the layers for some popular\nimages, like \"ubuntu\", \"debian\", but they will be refreshed at the time you\nattempt to use them.\n\nIf you built an image in a previous build step, it will be stored in the\nhost's Docker daemon's cache and is available to use as the name for a\nlater build step."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Output only. Stores timing information for pulling this build step's\nbuilder image only."]
        #[serde(rename = "pullTiming", default)]
        pub pull_timing: Option<crate::schemas::TimeSpan>,
        #[doc = "A list of environment variables which are encrypted using a Cloud Key\nManagement Service crypto key. These values must be specified in the\nbuild's `Secret`."]
        #[serde(rename = "secretEnv", default)]
        pub secret_env: Option<Vec<String>>,
        #[doc = "Output only. Status of the build step. At this time, build step status is\nonly updated on build completion; step status is not updated in real-time\nas the build progresses."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::BuildStepStatus>,
        #[doc = "Time limit for executing this build step. If not defined, the step has no\ntime limit and will be allowed to continue to run until either it completes\nor the build itself times out."]
        #[serde(rename = "timeout", default)]
        pub timeout: Option<String>,
        #[doc = "Output only. Stores timing information for executing this build step."]
        #[serde(rename = "timing", default)]
        pub timing: Option<crate::schemas::TimeSpan>,
        #[doc = "List of volumes to mount into the build step.\n\nEach volume is created as an empty volume prior to execution of the\nbuild step. Upon completion of the build, volumes and their contents are\ndiscarded.\n\nUsing a named volume in only one step is not valid as it is indicative\nof a build request with an incorrect configuration."]
        #[serde(rename = "volumes", default)]
        pub volumes: Option<Vec<crate::schemas::Volume>>,
        #[doc = "The ID(s) of the step(s) that this build step depends on.\nThis build step will not start until all the build steps in `wait_for`\nhave completed successfully. If `wait_for` is empty, this build step will\nstart when all previous build steps in the `Build.Steps` list have\ncompleted successfully."]
        #[serde(rename = "waitFor", default)]
        pub wait_for: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for BuildStep {
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
    pub struct BuiltImage {
        #[doc = "Docker Registry 2.0 digest."]
        #[serde(rename = "digest", default)]
        pub digest: Option<String>,
        #[doc = "Name used to push the container image to Google Container Registry, as\npresented to `docker push`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Output only. Stores timing information for pushing the specified image."]
        #[serde(rename = "pushTiming", default)]
        pub push_timing: Option<crate::schemas::TimeSpan>,
    }
    impl ::field_selector::FieldSelector for BuiltImage {
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
    pub struct Empty;
    impl ::field_selector::FieldSelector for Empty {
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
    pub struct FileHashes {
        #[doc = "Collection of file hashes."]
        #[serde(rename = "fileHash", default)]
        pub file_hash: Option<Vec<crate::schemas::Hash>>,
    }
    impl ::field_selector::FieldSelector for FileHashes {
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
    pub enum HashType {
        #[doc = "No hash requested."]
        None,
        #[doc = "Use a sha256 hash."]
        Sha256,
        #[doc = "Use a md5 hash."]
        Md5,
    }
    impl HashType {
        pub fn as_str(self) -> &'static str {
            match self {
                HashType::None => "NONE",
                HashType::Sha256 => "SHA256",
                HashType::Md5 => "MD5",
            }
        }
    }
    impl ::std::fmt::Display for HashType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for HashType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for HashType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => HashType::None,
                "SHA256" => HashType::Sha256,
                "MD5" => HashType::Md5,
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
    pub struct Hash {
        #[doc = "The type of hash that was performed."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::HashType>,
        #[doc = "The hash value."]
        #[serde(rename = "value", default)]
        pub value: Option<Vec<u8>>,
    }
    impl ::field_selector::FieldSelector for Hash {
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
    pub struct ListWorkerPoolsResponse {
        #[doc = "`WorkerPools` for the project."]
        #[serde(rename = "workerPools", default)]
        pub worker_pools: Option<Vec<crate::schemas::WorkerPool>>,
    }
    impl ::field_selector::FieldSelector for ListWorkerPoolsResponse {
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
    pub struct Network {
        #[doc = "Network on which the workers are created.\n\"default\" network is used if empty."]
        #[serde(rename = "network", default)]
        pub network: Option<String>,
        #[doc = "Project id containing the defined network and subnetwork. For a peered VPC,\nthis will be the same as the project_id in which the workers are created.\nFor a shared VPC, this will be the project sharing the network with the\nproject_id project in which workers will be created. For custom workers\nwith no VPC, this will be the same as project_id."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Subnetwork on which the workers are created.\n\"default\" subnetwork is used if empty."]
        #[serde(rename = "subnetwork", default)]
        pub subnetwork: Option<String>,
    }
    impl ::field_selector::FieldSelector for Network {
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
    pub struct RepoSource {
        #[doc = "Name of the branch to build."]
        #[serde(rename = "branchName", default)]
        pub branch_name: Option<String>,
        #[doc = "Explicit commit SHA to build."]
        #[serde(rename = "commitSha", default)]
        pub commit_sha: Option<String>,
        #[doc = "Directory, relative to the source root, in which to run the build.\n\nThis must be a relative path. If a step's `dir` is specified and is an\nabsolute path, this value is ignored for that step's execution."]
        #[serde(rename = "dir", default)]
        pub dir: Option<String>,
        #[doc = "ID of the project that owns the Cloud Source Repository. If omitted, the\nproject ID requesting the build is assumed."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Name of the Cloud Source Repository. If omitted, the name \"default\" is\nassumed."]
        #[serde(rename = "repoName", default)]
        pub repo_name: Option<String>,
        #[doc = "Name of the tag to build."]
        #[serde(rename = "tagName", default)]
        pub tag_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for RepoSource {
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
    pub struct Results {
        #[doc = "Path to the artifact manifest. Only populated when artifacts are uploaded."]
        #[serde(rename = "artifactManifest", default)]
        pub artifact_manifest: Option<String>,
        #[doc = "Time to push all non-container artifacts."]
        #[serde(rename = "artifactTiming", default)]
        pub artifact_timing: Option<crate::schemas::TimeSpan>,
        #[doc = "List of build step digests, in the order corresponding to build step\nindices."]
        #[serde(rename = "buildStepImages", default)]
        pub build_step_images: Option<Vec<String>>,
        #[doc = "List of build step outputs, produced by builder images, in the order\ncorresponding to build step indices.\n\n[Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders)\ncan produce this output by writing to `$BUILDER_OUTPUT/output`.\nOnly the first 4KB of data is stored."]
        #[serde(rename = "buildStepOutputs", default)]
        pub build_step_outputs: Option<Vec<Vec<u8>>>,
        #[doc = "Container images that were built as a part of the build."]
        #[serde(rename = "images", default)]
        pub images: Option<Vec<crate::schemas::BuiltImage>>,
        #[doc = "Number of artifacts uploaded. Only populated when artifacts are uploaded."]
        #[serde(rename = "numArtifacts", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_artifacts: Option<i64>,
    }
    impl ::field_selector::FieldSelector for Results {
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
    pub struct Secret {
        #[doc = "Cloud KMS key name to use to decrypt these envs."]
        #[serde(rename = "kmsKeyName", default)]
        pub kms_key_name: Option<String>,
        #[doc = "Map of environment variable name to its encrypted value.\n\nSecret environment variables must be unique across all of a build's\nsecrets, and must be used by at least one build step. Values can be at most\n64 KB in size. There can be at most 100 secret values across all of a\nbuild's secrets."]
        #[serde(rename = "secretEnv", default)]
        pub secret_env: Option<::std::collections::BTreeMap<String, Vec<u8>>>,
    }
    impl ::field_selector::FieldSelector for Secret {
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
    pub struct Source {
        #[doc = "If provided, get the source from this location in a Cloud Source\nRepository."]
        #[serde(rename = "repoSource", default)]
        pub repo_source: Option<crate::schemas::RepoSource>,
        #[doc = "If provided, get the source from this location in Google Cloud Storage."]
        #[serde(rename = "storageSource", default)]
        pub storage_source: Option<crate::schemas::StorageSource>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SourceProvenance {
        #[doc = "Output only. Hash(es) of the build source, which can be used to verify that\nthe original source integrity was maintained in the build. Note that\n`FileHashes` will only be populated if `BuildOptions` has requested a\n`SourceProvenanceHash`.\n\nThe keys to this map are file paths used as build source and the values\ncontain the hash values for those files.\n\nIf the build source came in a single package such as a gzipped tarfile\n(`.tar.gz`), the `FileHash` will be for the single path to that file."]
        #[serde(rename = "fileHashes", default)]
        pub file_hashes: Option<::std::collections::BTreeMap<String, crate::schemas::FileHashes>>,
        #[doc = "A copy of the build's `source.repo_source`, if exists, with any\nrevisions resolved."]
        #[serde(rename = "resolvedRepoSource", default)]
        pub resolved_repo_source: Option<crate::schemas::RepoSource>,
        #[doc = "A copy of the build's `source.storage_source`, if exists, with any\ngenerations resolved."]
        #[serde(rename = "resolvedStorageSource", default)]
        pub resolved_storage_source: Option<crate::schemas::StorageSource>,
    }
    impl ::field_selector::FieldSelector for SourceProvenance {
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
    pub struct StorageSource {
        #[doc = "Google Cloud Storage bucket containing the source (see\n[Bucket Name\nRequirements](https://cloud.google.com/storage/docs/bucket-naming#requirements))."]
        #[serde(rename = "bucket", default)]
        pub bucket: Option<String>,
        #[doc = "Google Cloud Storage generation for the object. If the generation is\nomitted, the latest generation will be used."]
        #[serde(rename = "generation", default)]
        #[serde(with = "crate::parsed_string")]
        pub generation: Option<i64>,
        #[doc = "Google Cloud Storage object containing the source.\n\nThis object must be a gzipped archive file (`.tar.gz`) containing source to\nbuild."]
        #[serde(rename = "object", default)]
        pub object: Option<String>,
    }
    impl ::field_selector::FieldSelector for StorageSource {
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
    pub struct TimeSpan {
        #[doc = "End of time span."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "Start of time span."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for TimeSpan {
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
    pub struct Volume {
        #[doc = "Name of the volume to mount.\n\nVolume names must be unique per build step and must be valid names for\nDocker volumes. Each named volume must be used by at least two build steps."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Path at which to mount the volume.\n\nPaths must be absolute and cannot conflict with other volume paths on the\nsame build step or with certain reserved volume paths."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
    }
    impl ::field_selector::FieldSelector for Volume {
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
    pub struct WorkerConfig {
        #[doc = "Size of the disk attached to the worker, in GB.\nSee https://cloud.google.com/compute/docs/disks/\nIf `0` is specified, Cloud Build will use a standard disk size.\n`disk_size` is overridden if you specify a different disk size in\n`build_options`. In this case, a VM with a disk size specified in the\n`build_options` will be created on demand at build time. For more\ninformation see\nhttps://cloud.google.com/cloud-build/docs/api/reference/rest/v1/projects.builds#buildoptions"]
        #[serde(rename = "diskSizeGb", default)]
        #[serde(with = "crate::parsed_string")]
        pub disk_size_gb: Option<i64>,
        #[doc = "Machine Type of the worker, such as n1-standard-1.\nSee https://cloud.google.com/compute/docs/machine-types.\nIf left blank, Cloud Build will use a standard unspecified machine to\ncreate the worker pool.\n`machine_type` is overridden if you specify a different machine type in\n`build_options`. In this case, the VM specified in the `build_options`\nwill be created on demand at build time. For more information see\nhttps://cloud.google.com/cloud-build/docs/speeding-up-builds#using_custom_virtual_machine_sizes"]
        #[serde(rename = "machineType", default)]
        pub machine_type: Option<String>,
        #[doc = "The network definition used to create the worker.\nIf this section is left empty, the workers will be created in\nWorkerPool.project_id on the default network."]
        #[serde(rename = "network", default)]
        pub network: Option<crate::schemas::Network>,
        #[doc = "The tag applied to the worker, and the same tag used by the firewall rule.\nIt is used to identify the Cloud Build workers among other VMs.\nThe default value for tag is `worker`."]
        #[serde(rename = "tag", default)]
        pub tag: Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerConfig {
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
    pub enum WorkerPoolRegionsItems {}
    impl WorkerPoolRegionsItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for WorkerPoolRegionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolRegionsItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolRegionsItems {
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
    pub enum WorkerPoolStatus {
        #[doc = "Status of the `WorkerPool` is unknown."]
        StatusUnspecified,
        #[doc = "`WorkerPool` is being created."]
        Creating,
        #[doc = "`WorkerPool` is running."]
        Running,
        #[doc = "`WorkerPool` is being deleting: cancelling builds and draining workers."]
        Deleting,
        #[doc = "`WorkerPool` is deleted."]
        Deleted,
    }
    impl WorkerPoolStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerPoolStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
                WorkerPoolStatus::Creating => "CREATING",
                WorkerPoolStatus::Running => "RUNNING",
                WorkerPoolStatus::Deleting => "DELETING",
                WorkerPoolStatus::Deleted => "DELETED",
            }
        }
    }
    impl ::std::fmt::Display for WorkerPoolStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATUS_UNSPECIFIED" => WorkerPoolStatus::StatusUnspecified,
                "CREATING" => WorkerPoolStatus::Creating,
                "RUNNING" => WorkerPoolStatus::Running,
                "DELETING" => WorkerPoolStatus::Deleting,
                "DELETED" => WorkerPoolStatus::Deleted,
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
    pub struct WorkerPool {
        #[doc = "Output only. Time at which the request to create the `WorkerPool` was\nreceived."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Output only. Time at which the request to delete the `WorkerPool` was\nreceived."]
        #[serde(rename = "deleteTime", default)]
        pub delete_time: Option<String>,
        #[doc = "User-defined name of the `WorkerPool`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The project ID of the GCP project for which the `WorkerPool` is created."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "List of regions to create the `WorkerPool`. Regions can't be empty.\nIf Cloud Build adds a new GCP region in the future, the existing\n`WorkerPool` will not be enabled in the new region automatically;\nyou must add the new region to the `regions` field to enable the\n`WorkerPool` in that region."]
        #[serde(rename = "regions", default)]
        pub regions: Option<Vec<crate::schemas::WorkerPoolRegionsItems>>,
        #[doc = "Output only. The service account used to manage the `WorkerPool`. The\nservice account must have the Compute Instance Admin (Beta) permission at\nthe project level."]
        #[serde(rename = "serviceAccountEmail", default)]
        pub service_account_email: Option<String>,
        #[doc = "Output only. WorkerPool Status."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::WorkerPoolStatus>,
        #[doc = "Output only. Time at which the request to update the `WorkerPool` was\nreceived."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
        #[doc = "Configuration to be used for a creating workers in the `WorkerPool`."]
        #[serde(rename = "workerConfig", default)]
        pub worker_config: Option<crate::schemas::WorkerConfig>,
        #[doc = "Total number of workers to be created across all requested regions."]
        #[serde(rename = "workerCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub worker_count: Option<i64>,
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
        #[doc = "Actions that can be performed on the worker_pools resource"]
        pub fn worker_pools(&self) -> worker_pools::WorkerPoolsActions<A> {
            worker_pools::WorkerPoolsActions
        }
    }
    pub mod worker_pools {
        pub mod params {}
        pub struct WorkerPoolsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> WorkerPoolsActions<'a, A> {
            #[doc = "Creates a `WorkerPool` to run the builds, and returns the new worker pool.\n\nThis API is experimental."]
            pub fn create(
                &self,
                request: crate::schemas::WorkerPool,
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
            #[doc = "Deletes a `WorkerPool` by its project ID and WorkerPool name.\n\nThis API is experimental."]
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
            #[doc = "Returns information about a `WorkerPool`.\n\nThis API is experimental."]
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
            #[doc = "List project's `WorkerPool`s.\n\nThis API is experimental."]
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
                }
            }
            #[doc = "Update a `WorkerPool`.\n\nThis API is experimental."]
            pub fn patch(
                &self,
                request: crate::schemas::WorkerPool,
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::WorkerPool,
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
            ) -> Result<crate::schemas::WorkerPool, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudbuild.googleapis.com/".to_owned();
                output.push_str("v1alpha1/");
                output.push_str(&self.parent);
                output.push_str("/workerPools");
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
        pub struct DeleteRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Empty, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudbuild.googleapis.com/".to_owned();
                output.push_str("v1alpha1/");
                output.push_str(&self.name);
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
            ) -> Result<crate::schemas::WorkerPool, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudbuild.googleapis.com/".to_owned();
                output.push_str("v1alpha1/");
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
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            ) -> Result<crate::schemas::ListWorkerPoolsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://cloudbuild.googleapis.com/".to_owned();
                output.push_str("v1alpha1/");
                output.push_str(&self.parent);
                output.push_str("/workerPools");
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
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::WorkerPool,
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
        impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::WorkerPool, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudbuild.googleapis.com/".to_owned();
                output.push_str("v1alpha1/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
