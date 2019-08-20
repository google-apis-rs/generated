pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        #[doc = "Unspecified."]
        OperationTypeUnspecified,
        #[doc = "ExportEntities."]
        ExportEntities,
        #[doc = "ImportEntities."]
        ImportEntities,
    }
    impl GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: OperationTypeUnspecified => "OPERATION_TYPE_UNSPECIFIED" , GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: ExportEntities => "EXPORT_ENTITIES" , GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: ImportEntities => "IMPORT_ENTITIES" , }
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::OperationTypeUnspecified
                }
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ImportEntities
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
    pub enum GoogleDatastoreAdminV1Beta1CommonMetadataState {
        #[doc = "Unspecified."]
        StateUnspecified,
        #[doc = "Request is being prepared for processing."]
        Initializing,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Request is in the process of being cancelled after user called\ngoogle.longrunning.Operations.CancelOperation on the operation."]
        Cancelling,
        #[doc = "Request has been processed and is in its finalization stage."]
        Finalizing,
        #[doc = "Request has completed successfully."]
        Successful,
        #[doc = "Request has finished being processed, but encountered an error."]
        Failed,
        #[doc = "Request has finished being cancelled after user called\ngoogle.longrunning.Operations.CancelOperation."]
        Cancelled,
    }
    impl GoogleDatastoreAdminV1Beta1CommonMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing => "INITIALIZING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing => "PROCESSING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling => "CANCELLING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing => "FINALIZING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful => "SUCCESSFUL",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed => "FAILED",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled => "CANCELLED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified
                }
                "INITIALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing,
                "CANCELLING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling,
                "FINALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing,
                "SUCCESSFUL" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful,
                "FAILED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed,
                "CANCELLED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled,
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
    pub struct GoogleDatastoreAdminV1Beta1CommonMetadata {
        #[doc = "The time the operation ended, either successfully or otherwise."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "The client-assigned labels which were provided when the operation was\ncreated. May also include additional labels."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of the operation. Can be used as a filter in\nListOperationsRequest."]
        #[serde(rename = "operationType", default)]
        pub operation_type:
            Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadataOperationType>,
        #[doc = "The time that work began on the operation."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "The current state of the Operation."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadataState>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1CommonMetadata {
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
    pub struct GoogleDatastoreAdminV1Beta1EntityFilter {
        #[doc = "If empty, then this represents all kinds."]
        #[serde(rename = "kinds", default)]
        pub kinds: Option<Vec<String>>,
        #[doc = "An empty list represents all namespaces. This is the preferred\nusage for projects that don't use namespaces.\n\nAn empty string element represents the default namespace. This should be\nused if the project has data in non-default namespaces, but doesn't want to\ninclude them.\nEach namespace in this list must be unique."]
        #[serde(rename = "namespaceIds", default)]
        pub namespace_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1EntityFilter {
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
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(rename = "common", default)]
        pub common: Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadata>,
        #[doc = "Description of which entities are being exported."]
        #[serde(rename = "entityFilter", default)]
        pub entity_filter: Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "Location for the export metadata and data files. This will be the same\nvalue as the\ngoogle.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix\nfield. The final output location is provided in\ngoogle.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
        #[serde(rename = "outputUrlPrefix", default)]
        pub output_url_prefix: Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(rename = "progressBytes", default)]
        pub progress_bytes: Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(rename = "progressEntities", default)]
        pub progress_entities: Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesRequest {
        #[doc = "Description of what data from the project is included in the export."]
        #[serde(rename = "entityFilter", default)]
        pub entity_filter: Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "Client-assigned labels."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Location for the export metadata and data files.\n\nThe full resource URL of the external storage location. Currently, only\nGoogle Cloud Storage is supported. So output_url_prefix should be of the\nform: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the\nname of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud\nStorage namespace path (this is not a Cloud Datastore namespace). For more\ninformation about Cloud Storage namespace paths, see\n[Object name\nconsiderations](https://cloud.google.com/storage/docs/naming#object-considerations).\n\nThe resulting files will be nested deeper than the specified URL prefix.\nThe final output URL will be provided in the\ngoogle.datastore.admin.v1beta1.ExportEntitiesResponse.output_url\nfield. That value should be used for subsequent ImportEntities operations.\n\nBy nesting the data files deeper, the same Cloud Storage bucket can be used\nin multiple ExportEntities operations without conflict."]
        #[serde(rename = "outputUrlPrefix", default)]
        pub output_url_prefix: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesRequest {
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
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
        #[doc = "Location of the output metadata file. This can be used to begin an import\ninto Cloud Datastore (this project or another project). See\ngoogle.datastore.admin.v1beta1.ImportEntitiesRequest.input_url.\nOnly present if the operation completed successfully."]
        #[serde(rename = "outputUrl", default)]
        pub output_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
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
    pub struct GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(rename = "common", default)]
        pub common: Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadata>,
        #[doc = "Description of which entities are being imported."]
        #[serde(rename = "entityFilter", default)]
        pub entity_filter: Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "The location of the import metadata file. This will be the same value as\nthe google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url\nfield."]
        #[serde(rename = "inputUrl", default)]
        pub input_url: Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(rename = "progressBytes", default)]
        pub progress_bytes: Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(rename = "progressEntities", default)]
        pub progress_entities: Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1Beta1ImportEntitiesRequest {
        #[doc = "Optionally specify which kinds/namespaces are to be imported. If provided,\nthe list must be a subset of the EntityFilter used in creating the export,\notherwise a FAILED_PRECONDITION error will be returned. If no filter is\nspecified then all entities from the export are imported."]
        #[serde(rename = "entityFilter", default)]
        pub entity_filter: Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "The full resource URL of the external storage location. Currently, only\nGoogle Cloud Storage is supported. So input_url should be of the form:\n`gs://BUCKET_NAME[/NAMESPACE_PATH]/OVERALL_EXPORT_METADATA_FILE`, where\n`BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is\nan optional Cloud Storage namespace path (this is not a Cloud Datastore\nnamespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written\nby the ExportEntities operation. For more information about Cloud Storage\nnamespace paths, see\n[Object name\nconsiderations](https://cloud.google.com/storage/docs/naming#object-considerations).\n\nFor more information, see\ngoogle.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
        #[serde(rename = "inputUrl", default)]
        pub input_url: Option<String>,
        #[doc = "Client-assigned labels."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ImportEntitiesRequest {
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
    pub struct GoogleDatastoreAdminV1Beta1Progress {
        #[doc = "The amount of work that has been completed. Note that this may be greater\nthan work_estimated."]
        #[serde(rename = "workCompleted", default)]
        #[serde(with = "crate::parsed_string")]
        pub work_completed: Option<i64>,
        #[doc = "An estimate of how much work needs to be performed. May be zero if the\nwork estimate is unavailable."]
        #[serde(rename = "workEstimated", default)]
        #[serde(with = "crate::parsed_string")]
        pub work_estimated: Option<i64>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1Progress {
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
    pub enum GoogleDatastoreAdminV1CommonMetadataOperationType {
        #[doc = "Unspecified."]
        OperationTypeUnspecified,
        #[doc = "ExportEntities."]
        ExportEntities,
        #[doc = "ImportEntities."]
        ImportEntities,
        #[doc = "CreateIndex."]
        CreateIndex,
        #[doc = "DeleteIndex."]
        DeleteIndex,
    }
    impl GoogleDatastoreAdminV1CommonMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
                GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities => {
                    "EXPORT_ENTITIES"
                }
                GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities => {
                    "IMPORT_ENTITIES"
                }
                GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex => "CREATE_INDEX",
                GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex => "DELETE_INDEX",
            }
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified
                }
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities
                }
                "CREATE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex,
                "DELETE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex,
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
    pub enum GoogleDatastoreAdminV1CommonMetadataState {
        #[doc = "Unspecified."]
        StateUnspecified,
        #[doc = "Request is being prepared for processing."]
        Initializing,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Request is in the process of being cancelled after user called\ngoogle.longrunning.Operations.CancelOperation on the operation."]
        Cancelling,
        #[doc = "Request has been processed and is in its finalization stage."]
        Finalizing,
        #[doc = "Request has completed successfully."]
        Successful,
        #[doc = "Request has finished being processed, but encountered an error."]
        Failed,
        #[doc = "Request has finished being cancelled after user called\ngoogle.longrunning.Operations.CancelOperation."]
        Cancelled,
    }
    impl GoogleDatastoreAdminV1CommonMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleDatastoreAdminV1CommonMetadataState::Initializing => "INITIALIZING",
                GoogleDatastoreAdminV1CommonMetadataState::Processing => "PROCESSING",
                GoogleDatastoreAdminV1CommonMetadataState::Cancelling => "CANCELLING",
                GoogleDatastoreAdminV1CommonMetadataState::Finalizing => "FINALIZING",
                GoogleDatastoreAdminV1CommonMetadataState::Successful => "SUCCESSFUL",
                GoogleDatastoreAdminV1CommonMetadataState::Failed => "FAILED",
                GoogleDatastoreAdminV1CommonMetadataState::Cancelled => "CANCELLED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1CommonMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1CommonMetadataState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1CommonMetadataState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STATE_UNSPECIFIED" => GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified,
                "INITIALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1CommonMetadataState::Processing,
                "CANCELLING" => GoogleDatastoreAdminV1CommonMetadataState::Cancelling,
                "FINALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Finalizing,
                "SUCCESSFUL" => GoogleDatastoreAdminV1CommonMetadataState::Successful,
                "FAILED" => GoogleDatastoreAdminV1CommonMetadataState::Failed,
                "CANCELLED" => GoogleDatastoreAdminV1CommonMetadataState::Cancelled,
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
    pub struct GoogleDatastoreAdminV1CommonMetadata {
        #[doc = "The time the operation ended, either successfully or otherwise."]
        #[serde(rename = "endTime", default)]
        pub end_time: Option<String>,
        #[doc = "The client-assigned labels which were provided when the operation was\ncreated. May also include additional labels."]
        #[serde(rename = "labels", default)]
        pub labels: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of the operation. Can be used as a filter in\nListOperationsRequest."]
        #[serde(rename = "operationType", default)]
        pub operation_type:
            Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadataOperationType>,
        #[doc = "The time that work began on the operation."]
        #[serde(rename = "startTime", default)]
        pub start_time: Option<String>,
        #[doc = "The current state of the Operation."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadataState>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1CommonMetadata {
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
    pub struct GoogleDatastoreAdminV1EntityFilter {
        #[doc = "If empty, then this represents all kinds."]
        #[serde(rename = "kinds", default)]
        pub kinds: Option<Vec<String>>,
        #[doc = "An empty list represents all namespaces. This is the preferred\nusage for projects that don't use namespaces.\n\nAn empty string element represents the default namespace. This should be\nused if the project has data in non-default namespaces, but doesn't want to\ninclude them.\nEach namespace in this list must be unique."]
        #[serde(rename = "namespaceIds", default)]
        pub namespace_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1EntityFilter {
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
    pub struct GoogleDatastoreAdminV1ExportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(rename = "common", default)]
        pub common: Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "Description of which entities are being exported."]
        #[serde(rename = "entityFilter", default)]
        pub entity_filter: Option<crate::schemas::GoogleDatastoreAdminV1EntityFilter>,
        #[doc = "Location for the export metadata and data files. This will be the same\nvalue as the\ngoogle.datastore.admin.v1.ExportEntitiesRequest.output_url_prefix\nfield. The final output location is provided in\ngoogle.datastore.admin.v1.ExportEntitiesResponse.output_url."]
        #[serde(rename = "outputUrlPrefix", default)]
        pub output_url_prefix: Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(rename = "progressBytes", default)]
        pub progress_bytes: Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(rename = "progressEntities", default)]
        pub progress_entities: Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1ExportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1ExportEntitiesResponse {
        #[doc = "Location of the output metadata file. This can be used to begin an import\ninto Cloud Datastore (this project or another project). See\ngoogle.datastore.admin.v1.ImportEntitiesRequest.input_url.\nOnly present if the operation completed successfully."]
        #[serde(rename = "outputUrl", default)]
        pub output_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1ExportEntitiesResponse {
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
    pub struct GoogleDatastoreAdminV1ImportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(rename = "common", default)]
        pub common: Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "Description of which entities are being imported."]
        #[serde(rename = "entityFilter", default)]
        pub entity_filter: Option<crate::schemas::GoogleDatastoreAdminV1EntityFilter>,
        #[doc = "The location of the import metadata file. This will be the same value as\nthe google.datastore.admin.v1.ExportEntitiesResponse.output_url field."]
        #[serde(rename = "inputUrl", default)]
        pub input_url: Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(rename = "progressBytes", default)]
        pub progress_bytes: Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(rename = "progressEntities", default)]
        pub progress_entities: Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1ImportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1IndexOperationMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(rename = "common", default)]
        pub common: Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "The index resource ID that this operation is acting on."]
        #[serde(rename = "indexId", default)]
        pub index_id: Option<String>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(rename = "progressEntities", default)]
        pub progress_entities: Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1IndexOperationMetadata {
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
    pub struct GoogleDatastoreAdminV1Progress {
        #[doc = "The amount of work that has been completed. Note that this may be greater\nthan work_estimated."]
        #[serde(rename = "workCompleted", default)]
        #[serde(with = "crate::parsed_string")]
        pub work_completed: Option<i64>,
        #[doc = "An estimate of how much work needs to be performed. May be zero if the\nwork estimate is unavailable."]
        #[serde(rename = "workEstimated", default)]
        #[serde(with = "crate::parsed_string")]
        pub work_estimated: Option<i64>,
    }
    impl ::field_selector::FieldSelector for GoogleDatastoreAdminV1Progress {
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
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should have the format of `operations/some/unique/name`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for GoogleLongrunningOperation {
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
        #[doc = "Exports a copy of all or a subset of entities from Google Cloud Datastore\nto another storage system, such as Google Cloud Storage. Recent updates to\nentities may not be reflected in the export. The export occurs in the\nbackground and its progress can be monitored and managed via the\nOperation resource that is created. The output of an export may only be\nused once the associated operation is done. If an export operation is\ncancelled before completion it may leave partial data behind in Google\nCloud Storage."]
        pub fn export(
            &self,
            request: crate::schemas::GoogleDatastoreAdminV1Beta1ExportEntitiesRequest,
            project_id: impl Into<String>,
        ) -> ExportRequestBuilder<A> {
            ExportRequestBuilder {
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
        #[doc = "Imports entities into Google Cloud Datastore. Existing entities with the\nsame key are overwritten. The import occurs in the background and its\nprogress can be monitored and managed via the Operation resource that is\ncreated. If an ImportEntities operation is cancelled, it is possible\nthat a subset of the data has already been imported to Cloud Datastore."]
        pub fn import(
            &self,
            request: crate::schemas::GoogleDatastoreAdminV1Beta1ImportEntitiesRequest,
            project_id: impl Into<String>,
        ) -> ImportRequestBuilder<A> {
            ImportRequestBuilder {
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
    }
    #[derive(Debug, Clone)]
    pub struct ExportRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::GoogleDatastoreAdminV1Beta1ExportEntitiesRequest,
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
    impl<'a, A: yup_oauth2::GetToken> ExportRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
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
            let mut output = "https://datastore.googleapis.com/".to_owned();
            output.push_str("v1beta1/projects/");
            output.push_str(&self.project_id);
            output.push_str(":export");
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
    pub struct ImportRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::GoogleDatastoreAdminV1Beta1ImportEntitiesRequest,
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
    impl<'a, A: yup_oauth2::GetToken> ImportRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
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
            let mut output = "https://datastore.googleapis.com/".to_owned();
            output.push_str("v1beta1/projects/");
            output.push_str(&self.project_id);
            output.push_str(":import");
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
