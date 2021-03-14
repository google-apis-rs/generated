#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*allocateIds*](resources/projects/struct.AllocateIdsRequestBuilder.html), [*beginTransaction*](resources/projects/struct.BeginTransactionRequestBuilder.html), [*commit*](resources/projects/struct.CommitRequestBuilder.html), [*lookup*](resources/projects/struct.LookupRequestBuilder.html), [*reserveIds*](resources/projects/struct.ReserveIdsRequestBuilder.html), [*rollback*](resources/projects/struct.RollbackRequestBuilder.html), [*runQuery*](resources/projects/struct.RunQueryRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "View and manage your Google Cloud Datastore data\n\n`https://www.googleapis.com/auth/datastore`"]
    pub const DATASTORE: &str = "https://www.googleapis.com/auth/datastore";
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
    pub struct AllocateIdsRequest {
        #[doc = "Required. A list of keys with incomplete key paths for which to allocate IDs. No key may be reserved/read-only."]
        #[serde(
            rename = "keys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keys: ::std::option::Option<Vec<crate::schemas::Key>>,
    }
    impl ::google_field_selector::FieldSelector for AllocateIdsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AllocateIdsRequest {
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
    pub struct AllocateIdsResponse {
        #[doc = "The keys specified in the request (in the same order), each with its key path completed with a newly allocated ID."]
        #[serde(
            rename = "keys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keys: ::std::option::Option<Vec<crate::schemas::Key>>,
    }
    impl ::google_field_selector::FieldSelector for AllocateIdsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AllocateIdsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ArrayValue {
        #[doc = "Values in the array. The order of values in an array is preserved as long as all values have identical settings for 'exclude_from_indexes'."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::Value>>,
    }
    impl ::google_field_selector::FieldSelector for ArrayValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ArrayValue {
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
    pub struct BeginTransactionRequest {
        #[doc = "Options for a new transaction."]
        #[serde(
            rename = "transactionOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transaction_options: ::std::option::Option<crate::schemas::TransactionOptions>,
    }
    impl ::google_field_selector::FieldSelector for BeginTransactionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BeginTransactionRequest {
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
    pub struct BeginTransactionResponse {
        #[doc = "The transaction identifier (always present)."]
        #[serde(
            rename = "transaction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transaction: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for BeginTransactionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BeginTransactionResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CommitRequest {
        #[doc = "The type of commit to perform. Defaults to `TRANSACTIONAL`."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::CommitRequestMode>,
        #[doc = "The mutations to perform. When mode is `TRANSACTIONAL`, mutations affecting a single entity are applied in order. The following sequences of mutations affecting a single entity are not permitted in a single `Commit` request: - `insert` followed by `insert` - `update` followed by `insert` - `upsert` followed by `insert` - `delete` followed by `update` When mode is `NON_TRANSACTIONAL`, no two mutations may affect a single entity."]
        #[serde(
            rename = "mutations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mutations: ::std::option::Option<Vec<crate::schemas::Mutation>>,
        #[doc = "The identifier of the transaction associated with the commit. A transaction identifier is returned by a call to Datastore.BeginTransaction."]
        #[serde(
            rename = "transaction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transaction: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for CommitRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommitRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommitRequestMode {
        #[doc = "Unspecified. This value must not be used."]
        ModeUnspecified,
        #[doc = "Non-transactional: The mutations may not apply as all or none."]
        NonTransactional,
        #[doc = "Transactional: The mutations are either all applied, or none are applied. Learn about transactions [here](https://cloud.google.com/datastore/docs/concepts/transactions)."]
        Transactional,
    }
    impl CommitRequestMode {
        pub fn as_str(self) -> &'static str {
            match self {
                CommitRequestMode::ModeUnspecified => "MODE_UNSPECIFIED",
                CommitRequestMode::NonTransactional => "NON_TRANSACTIONAL",
                CommitRequestMode::Transactional => "TRANSACTIONAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommitRequestMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommitRequestMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommitRequestMode, ()> {
            Ok(match s {
                "MODE_UNSPECIFIED" => CommitRequestMode::ModeUnspecified,
                "NON_TRANSACTIONAL" => CommitRequestMode::NonTransactional,
                "TRANSACTIONAL" => CommitRequestMode::Transactional,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommitRequestMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommitRequestMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommitRequestMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MODE_UNSPECIFIED" => CommitRequestMode::ModeUnspecified,
                "NON_TRANSACTIONAL" => CommitRequestMode::NonTransactional,
                "TRANSACTIONAL" => CommitRequestMode::Transactional,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommitRequestMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommitRequestMode {
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
    pub struct CommitResponse {
        #[doc = "The number of index entries updated during the commit, or zero if none were updated."]
        #[serde(
            rename = "indexUpdates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index_updates: ::std::option::Option<i32>,
        #[doc = "The result of performing the mutations. The i-th mutation result corresponds to the i-th mutation in the request."]
        #[serde(
            rename = "mutationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mutation_results: ::std::option::Option<Vec<crate::schemas::MutationResult>>,
    }
    impl ::google_field_selector::FieldSelector for CommitResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommitResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompositeFilter {
        #[doc = "The list of filters to combine. Must contain at least one filter."]
        #[serde(
            rename = "filters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filters: ::std::option::Option<Vec<crate::schemas::Filter>>,
        #[doc = "The operator for combining multiple filters."]
        #[serde(
            rename = "op",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub op: ::std::option::Option<crate::schemas::CompositeFilterOp>,
    }
    impl ::google_field_selector::FieldSelector for CompositeFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompositeFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompositeFilterOp {
        #[doc = "The results are required to satisfy each of the combined filters."]
        And,
        #[doc = "Unspecified. This value must not be used."]
        OperatorUnspecified,
    }
    impl CompositeFilterOp {
        pub fn as_str(self) -> &'static str {
            match self {
                CompositeFilterOp::And => "AND",
                CompositeFilterOp::OperatorUnspecified => "OPERATOR_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompositeFilterOp {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompositeFilterOp {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompositeFilterOp, ()> {
            Ok(match s {
                "AND" => CompositeFilterOp::And,
                "OPERATOR_UNSPECIFIED" => CompositeFilterOp::OperatorUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompositeFilterOp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompositeFilterOp {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompositeFilterOp {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => CompositeFilterOp::And,
                "OPERATOR_UNSPECIFIED" => CompositeFilterOp::OperatorUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompositeFilterOp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompositeFilterOp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Entity {
        #[doc = "The entity's key. An entity must have a key, unless otherwise documented (for example, an entity in `Value.entity_value` may have no key). An entity's kind is its key path's last element's kind, or null if it has no key."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<crate::schemas::Key>,
        #[doc = "The entity's properties. The map's keys are property names. A property name matching regex `__.*__` is reserved. A reserved property name is forbidden in certain documented contexts. The name must not contain more than 500 characters. The name cannot be `\"\"`."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Entity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Entity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EntityResult {
        #[doc = "A cursor that points to the position after the result entity. Set only when the `EntityResult` is part of a `QueryResultBatch` message."]
        #[serde(
            rename = "cursor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cursor: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The resulting entity."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity: ::std::option::Option<crate::schemas::Entity>,
        #[doc = "The version of the entity, a strictly positive number that monotonically increases with changes to the entity. This field is set for `FULL` entity results. For missing entities in `LookupResponse`, this is the version of the snapshot that was used to look up the entity, and it is always set except for eventually consistent reads."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for EntityResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EntityResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Filter {
        #[doc = "A composite filter."]
        #[serde(
            rename = "compositeFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub composite_filter: ::std::option::Option<crate::schemas::CompositeFilter>,
        #[doc = "A filter on a property."]
        #[serde(
            rename = "propertyFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_filter: ::std::option::Option<crate::schemas::PropertyFilter>,
    }
    impl ::google_field_selector::FieldSelector for Filter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Filter {
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
    pub struct GoogleDatastoreAdminV1Beta1CommonMetadata {
        #[doc = "The time the operation ended, either successfully or otherwise."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadataOperationType,
        >,
        #[doc = "The time that work began on the operation."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The current state of the Operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadataState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1CommonMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1CommonMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        #[doc = "ExportEntities."]
        ExportEntities,
        #[doc = "ImportEntities."]
        ImportEntities,
        #[doc = "Unspecified."]
        OperationTypeUnspecified,
    }
    impl GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: ExportEntities => "EXPORT_ENTITIES" , GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: ImportEntities => "IMPORT_ENTITIES" , GoogleDatastoreAdminV1Beta1CommonMetadataOperationType :: OperationTypeUnspecified => "OPERATION_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1Beta1CommonMetadataOperationType, ()>
        {
            Ok(match s {
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::OperationTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataOperationType::OperationTypeUnspecified
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
        for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDatastoreAdminV1Beta1CommonMetadataOperationType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1Beta1CommonMetadataState {
        #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
        Cancelled,
        #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
        Cancelling,
        #[doc = "Request has finished being processed, but encountered an error."]
        Failed,
        #[doc = "Request has been processed and is in its finalization stage."]
        Finalizing,
        #[doc = "Request is being prepared for processing."]
        Initializing,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Unspecified."]
        StateUnspecified,
        #[doc = "Request has completed successfully."]
        Successful,
    }
    impl GoogleDatastoreAdminV1Beta1CommonMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled => "CANCELLED",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling => "CANCELLING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed => "FAILED",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing => "FINALIZING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing => "INITIALIZING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing => "PROCESSING",
                GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1Beta1CommonMetadataState, ()> {
            Ok(match s {
                "CANCELLED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified
                }
                "SUCCESSFUL" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1Beta1CommonMetadataState::StateUnspecified
                }
                "SUCCESSFUL" => GoogleDatastoreAdminV1Beta1CommonMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1CommonMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1CommonMetadataState {
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
    pub struct GoogleDatastoreAdminV1Beta1EntityFilter {
        #[doc = "If empty, then this represents all kinds."]
        #[serde(
            rename = "kinds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kinds: ::std::option::Option<Vec<String>>,
        #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don't use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique."]
        #[serde(
            rename = "namespaceIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1EntityFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1EntityFilter {
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
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadata>,
        #[doc = "Description of which entities are being exported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url."]
        #[serde(
            rename = "outputUrlPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url_prefix: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ExportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
        #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
        #[serde(
            rename = "outputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ExportEntitiesResponse {
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
    pub struct GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1CommonMetadata>,
        #[doc = "Description of which entities are being imported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1EntityFilter>,
        #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url field."]
        #[serde(
            rename = "inputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_url: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Beta1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1ImportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1Beta1Progress {
        #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
        #[serde(
            rename = "workCompleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_completed: ::std::option::Option<i64>,
        #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
        #[serde(
            rename = "workEstimated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_estimated: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Beta1Progress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Beta1Progress {
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
    pub struct GoogleDatastoreAdminV1CommonMetadata {
        #[doc = "The time the operation ended, either successfully or otherwise."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The client-assigned labels which were provided when the operation was created. May also include additional labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The type of the operation. Can be used as a filter in ListOperationsRequest."]
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_type: ::std::option::Option<
            crate::schemas::GoogleDatastoreAdminV1CommonMetadataOperationType,
        >,
        #[doc = "The time that work began on the operation."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The current state of the Operation."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadataState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1CommonMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1CommonMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1CommonMetadataOperationType {
        #[doc = "CreateIndex."]
        CreateIndex,
        #[doc = "DeleteIndex."]
        DeleteIndex,
        #[doc = "ExportEntities."]
        ExportEntities,
        #[doc = "ImportEntities."]
        ImportEntities,
        #[doc = "Unspecified."]
        OperationTypeUnspecified,
    }
    impl GoogleDatastoreAdminV1CommonMetadataOperationType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex => "CREATE_INDEX",
                GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex => "DELETE_INDEX",
                GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities => {
                    "EXPORT_ENTITIES"
                }
                GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities => {
                    "IMPORT_ENTITIES"
                }
                GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified => {
                    "OPERATION_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1CommonMetadataOperationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1CommonMetadataOperationType, ()> {
            Ok(match s {
                "CREATE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex,
                "DELETE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex,
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::CreateIndex,
                "DELETE_INDEX" => GoogleDatastoreAdminV1CommonMetadataOperationType::DeleteIndex,
                "EXPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ExportEntities
                }
                "IMPORT_ENTITIES" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::ImportEntities
                }
                "OPERATION_TYPE_UNSPECIFIED" => {
                    GoogleDatastoreAdminV1CommonMetadataOperationType::OperationTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1CommonMetadataOperationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDatastoreAdminV1CommonMetadataState {
        #[doc = "Request has finished being cancelled after user called google.longrunning.Operations.CancelOperation."]
        Cancelled,
        #[doc = "Request is in the process of being cancelled after user called google.longrunning.Operations.CancelOperation on the operation."]
        Cancelling,
        #[doc = "Request has finished being processed, but encountered an error."]
        Failed,
        #[doc = "Request has been processed and is in its finalization stage."]
        Finalizing,
        #[doc = "Request is being prepared for processing."]
        Initializing,
        #[doc = "Request is actively being processed."]
        Processing,
        #[doc = "Unspecified."]
        StateUnspecified,
        #[doc = "Request has completed successfully."]
        Successful,
    }
    impl GoogleDatastoreAdminV1CommonMetadataState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDatastoreAdminV1CommonMetadataState::Cancelled => "CANCELLED",
                GoogleDatastoreAdminV1CommonMetadataState::Cancelling => "CANCELLING",
                GoogleDatastoreAdminV1CommonMetadataState::Failed => "FAILED",
                GoogleDatastoreAdminV1CommonMetadataState::Finalizing => "FINALIZING",
                GoogleDatastoreAdminV1CommonMetadataState::Initializing => "INITIALIZING",
                GoogleDatastoreAdminV1CommonMetadataState::Processing => "PROCESSING",
                GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleDatastoreAdminV1CommonMetadataState::Successful => "SUCCESSFUL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDatastoreAdminV1CommonMetadataState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDatastoreAdminV1CommonMetadataState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDatastoreAdminV1CommonMetadataState, ()> {
            Ok(match s {
                "CANCELLED" => GoogleDatastoreAdminV1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified,
                "SUCCESSFUL" => GoogleDatastoreAdminV1CommonMetadataState::Successful,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDatastoreAdminV1CommonMetadataState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDatastoreAdminV1CommonMetadataState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDatastoreAdminV1CommonMetadataState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => GoogleDatastoreAdminV1CommonMetadataState::Cancelled,
                "CANCELLING" => GoogleDatastoreAdminV1CommonMetadataState::Cancelling,
                "FAILED" => GoogleDatastoreAdminV1CommonMetadataState::Failed,
                "FINALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Finalizing,
                "INITIALIZING" => GoogleDatastoreAdminV1CommonMetadataState::Initializing,
                "PROCESSING" => GoogleDatastoreAdminV1CommonMetadataState::Processing,
                "STATE_UNSPECIFIED" => GoogleDatastoreAdminV1CommonMetadataState::StateUnspecified,
                "SUCCESSFUL" => GoogleDatastoreAdminV1CommonMetadataState::Successful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1CommonMetadataState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1CommonMetadataState {
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
    pub struct GoogleDatastoreAdminV1EntityFilter {
        #[doc = "If empty, then this represents all kinds."]
        #[serde(
            rename = "kinds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kinds: ::std::option::Option<Vec<String>>,
        #[doc = "An empty list represents all namespaces. This is the preferred usage for projects that don't use namespaces. An empty string element represents the default namespace. This should be used if the project has data in non-default namespaces, but doesn't want to include them. Each namespace in this list must be unique."]
        #[serde(
            rename = "namespaceIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1EntityFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1EntityFilter {
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
    pub struct GoogleDatastoreAdminV1ExportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "Description of which entities are being exported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1EntityFilter>,
        #[doc = "Location for the export metadata and data files. This will be the same value as the google.datastore.admin.v1.ExportEntitiesRequest.output_url_prefix field. The final output location is provided in google.datastore.admin.v1.ExportEntitiesResponse.output_url."]
        #[serde(
            rename = "outputUrlPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url_prefix: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1ExportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1ExportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1ExportEntitiesResponse {
        #[doc = "Location of the output metadata file. This can be used to begin an import into Cloud Datastore (this project or another project). See google.datastore.admin.v1.ImportEntitiesRequest.input_url. Only present if the operation completed successfully."]
        #[serde(
            rename = "outputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1ExportEntitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1ExportEntitiesResponse {
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
    pub struct GoogleDatastoreAdminV1ImportEntitiesMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "Description of which entities are being imported."]
        #[serde(
            rename = "entityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_filter:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1EntityFilter>,
        #[doc = "The location of the import metadata file. This will be the same value as the google.datastore.admin.v1.ExportEntitiesResponse.output_url field."]
        #[serde(
            rename = "inputUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_url: ::std::option::Option<String>,
        #[doc = "An estimate of the number of bytes processed."]
        #[serde(
            rename = "progressBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_bytes: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1ImportEntitiesMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1ImportEntitiesMetadata {
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
    pub struct GoogleDatastoreAdminV1IndexOperationMetadata {
        #[doc = "Metadata common to all Datastore Admin operations."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common: ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1CommonMetadata>,
        #[doc = "The index resource ID that this operation is acting on."]
        #[serde(
            rename = "indexId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index_id: ::std::option::Option<String>,
        #[doc = "An estimate of the number of entities processed."]
        #[serde(
            rename = "progressEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_entities:
            ::std::option::Option<crate::schemas::GoogleDatastoreAdminV1Progress>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1IndexOperationMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1IndexOperationMetadata {
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
    pub struct GoogleDatastoreAdminV1Progress {
        #[doc = "The amount of work that has been completed. Note that this may be greater than work_estimated."]
        #[serde(
            rename = "workCompleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_completed: ::std::option::Option<i64>,
        #[doc = "An estimate of how much work needs to be performed. May be zero if the work estimate is unavailable."]
        #[serde(
            rename = "workEstimated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub work_estimated: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDatastoreAdminV1Progress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDatastoreAdminV1Progress {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GqlQuery {
        #[doc = "When false, the query string must not contain any literals and instead must bind all values. For example, `SELECT * FROM Kind WHERE a = 'string literal'` is not allowed, while `SELECT * FROM Kind WHERE a = @value` is."]
        #[serde(
            rename = "allowLiterals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_literals: ::std::option::Option<bool>,
        #[doc = "For each non-reserved named binding site in the query string, there must be a named parameter with that name, but not necessarily the inverse. Key must match regex `A-Za-z_$*`, must not match regex `__.*__`, and must not be `\"\"`."]
        #[serde(
            rename = "namedBindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_bindings: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::GqlQueryParameter>,
        >,
        #[doc = "Numbered binding site @1 references the first numbered parameter, effectively using 1-based indexing, rather than the usual 0. For each binding site numbered i in `query_string`, there must be an i-th numbered parameter. The inverse must also be true."]
        #[serde(
            rename = "positionalBindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positional_bindings: ::std::option::Option<Vec<crate::schemas::GqlQueryParameter>>,
        #[doc = "A string of the format described [here](https://cloud.google.com/datastore/docs/apis/gql/gql_reference)."]
        #[serde(
            rename = "queryString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_string: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GqlQuery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GqlQuery {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GqlQueryParameter {
        #[doc = "A query cursor. Query cursors are returned in query result batches."]
        #[serde(
            rename = "cursor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cursor: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A value parameter."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::google_field_selector::FieldSelector for GqlQueryParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GqlQueryParameter {
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
    pub struct Key {
        #[doc = "Entities are partitioned into subsets, currently identified by a project ID and namespace ID. Queries are scoped to a single partition."]
        #[serde(
            rename = "partitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partition_id: ::std::option::Option<crate::schemas::PartitionId>,
        #[doc = "The entity path. An entity path consists of one or more elements composed of a kind and a string or numerical identifier, which identify entities. The first element identifies a *root entity*, the second element identifies a *child* of the root entity, the third element identifies a child of the second entity, and so forth. The entities identified by all prefixes of the path are called the element's *ancestors*. An entity path is always fully complete: *all* of the entity's ancestors are required to be in the path along with the entity identifier itself. The only exception is that in some documented cases, the identifier in the last path element (for the entity) itself may be omitted. For example, the last path element of the key of `Mutation.insert` may have no identifier. A path can never be empty, and a path can have at most 100 elements."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<Vec<crate::schemas::PathElement>>,
    }
    impl ::google_field_selector::FieldSelector for Key {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Key {
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
    pub struct KindExpression {
        #[doc = "The name of the kind."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for KindExpression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KindExpression {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LatLng {
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        #[serde(
            rename = "latitude",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        #[serde(
            rename = "longitude",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for LatLng {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LatLng {
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
    pub struct LookupRequest {
        #[doc = "Required. Keys of entities to look up."]
        #[serde(
            rename = "keys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keys: ::std::option::Option<Vec<crate::schemas::Key>>,
        #[doc = "The options for this lookup request."]
        #[serde(
            rename = "readOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_options: ::std::option::Option<crate::schemas::ReadOptions>,
    }
    impl ::google_field_selector::FieldSelector for LookupRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LookupRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LookupResponse {
        #[doc = "A list of keys that were not looked up due to resource constraints. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
        #[serde(
            rename = "deferred",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deferred: ::std::option::Option<Vec<crate::schemas::Key>>,
        #[doc = "Entities found as `ResultType.FULL` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
        #[serde(
            rename = "found",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub found: ::std::option::Option<Vec<crate::schemas::EntityResult>>,
        #[doc = "Entities not found as `ResultType.KEY_ONLY` entities. The order of results in this field is undefined and has no relation to the order of the keys in the input."]
        #[serde(
            rename = "missing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub missing: ::std::option::Option<Vec<crate::schemas::EntityResult>>,
    }
    impl ::google_field_selector::FieldSelector for LookupResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LookupResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Mutation {
        #[doc = "The version of the entity that this mutation is being applied to. If this does not match the current version on the server, the mutation conflicts."]
        #[serde(
            rename = "baseVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub base_version: ::std::option::Option<i64>,
        #[doc = "The key of the entity to delete. The entity may or may not already exist. Must have a complete key path and must not be reserved/read-only."]
        #[serde(
            rename = "delete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete: ::std::option::Option<crate::schemas::Key>,
        #[doc = "The entity to insert. The entity must not already exist. The entity key's final path element may be incomplete."]
        #[serde(
            rename = "insert",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert: ::std::option::Option<crate::schemas::Entity>,
        #[doc = "The entity to update. The entity must already exist. Must have a complete key path."]
        #[serde(
            rename = "update",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update: ::std::option::Option<crate::schemas::Entity>,
        #[doc = "The entity to upsert. The entity may or may not already exist. The entity key's final path element may be incomplete."]
        #[serde(
            rename = "upsert",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upsert: ::std::option::Option<crate::schemas::Entity>,
    }
    impl ::google_field_selector::FieldSelector for Mutation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Mutation {
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
    pub struct MutationResult {
        #[doc = "Whether a conflict was detected for this mutation. Always false when a conflict detection strategy field is not set in the mutation."]
        #[serde(
            rename = "conflictDetected",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conflict_detected: ::std::option::Option<bool>,
        #[doc = "The automatically allocated key. Set only when the mutation allocated a key."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<crate::schemas::Key>,
        #[doc = "The version of the entity on the server after processing the mutation. If the mutation doesn't change anything on the server, then the version will be the version of the current entity or, if no entity is present, a version that is strictly greater than the version of any previous entity and less than the version of any possible future entity."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MutationResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MutationResult {
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
    pub struct PartitionId {
        #[doc = "If not empty, the ID of the namespace to which the entities belong."]
        #[serde(
            rename = "namespaceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace_id: ::std::option::Option<String>,
        #[doc = "The ID of the project to which the entities belong."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PartitionId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PartitionId {
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
    pub struct PathElement {
        #[doc = "The auto-allocated ID of the entity. Never equal to zero. Values less than zero are discouraged and may not be supported in the future."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "The kind of the entity. A kind matching regex `__.*__` is reserved/read-only. A kind must not contain more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name of the entity. A name matching regex `__.*__` is reserved/read-only. A name must not be more than 1500 bytes when UTF-8 encoded. Cannot be `\"\"`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PathElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathElement {
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
    pub struct Projection {
        #[doc = "The property to project."]
        #[serde(
            rename = "property",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property: ::std::option::Option<crate::schemas::PropertyReference>,
    }
    impl ::google_field_selector::FieldSelector for Projection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Projection {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PropertyFilter {
        #[doc = "The operator to filter by."]
        #[serde(
            rename = "op",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub op: ::std::option::Option<crate::schemas::PropertyFilterOp>,
        #[doc = "The property to filter by."]
        #[serde(
            rename = "property",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property: ::std::option::Option<crate::schemas::PropertyReference>,
        #[doc = "The value to compare the property to."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::google_field_selector::FieldSelector for PropertyFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PropertyFilterOp {
        #[doc = "Equal."]
        Equal,
        #[doc = "Greater than."]
        GreaterThan,
        #[doc = "Greater than or equal."]
        GreaterThanOrEqual,
        #[doc = "Has ancestor."]
        HasAncestor,
        #[doc = "Less than."]
        LessThan,
        #[doc = "Less than or equal."]
        LessThanOrEqual,
        #[doc = "Unspecified. This value must not be used."]
        OperatorUnspecified,
    }
    impl PropertyFilterOp {
        pub fn as_str(self) -> &'static str {
            match self {
                PropertyFilterOp::Equal => "EQUAL",
                PropertyFilterOp::GreaterThan => "GREATER_THAN",
                PropertyFilterOp::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
                PropertyFilterOp::HasAncestor => "HAS_ANCESTOR",
                PropertyFilterOp::LessThan => "LESS_THAN",
                PropertyFilterOp::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
                PropertyFilterOp::OperatorUnspecified => "OPERATOR_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PropertyFilterOp {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PropertyFilterOp {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PropertyFilterOp, ()> {
            Ok(match s {
                "EQUAL" => PropertyFilterOp::Equal,
                "GREATER_THAN" => PropertyFilterOp::GreaterThan,
                "GREATER_THAN_OR_EQUAL" => PropertyFilterOp::GreaterThanOrEqual,
                "HAS_ANCESTOR" => PropertyFilterOp::HasAncestor,
                "LESS_THAN" => PropertyFilterOp::LessThan,
                "LESS_THAN_OR_EQUAL" => PropertyFilterOp::LessThanOrEqual,
                "OPERATOR_UNSPECIFIED" => PropertyFilterOp::OperatorUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PropertyFilterOp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PropertyFilterOp {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PropertyFilterOp {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EQUAL" => PropertyFilterOp::Equal,
                "GREATER_THAN" => PropertyFilterOp::GreaterThan,
                "GREATER_THAN_OR_EQUAL" => PropertyFilterOp::GreaterThanOrEqual,
                "HAS_ANCESTOR" => PropertyFilterOp::HasAncestor,
                "LESS_THAN" => PropertyFilterOp::LessThan,
                "LESS_THAN_OR_EQUAL" => PropertyFilterOp::LessThanOrEqual,
                "OPERATOR_UNSPECIFIED" => PropertyFilterOp::OperatorUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PropertyFilterOp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyFilterOp {
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
    pub struct PropertyOrder {
        #[doc = "The direction to order by. Defaults to `ASCENDING`."]
        #[serde(
            rename = "direction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direction: ::std::option::Option<crate::schemas::PropertyOrderDirection>,
        #[doc = "The property to order by."]
        #[serde(
            rename = "property",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property: ::std::option::Option<crate::schemas::PropertyReference>,
    }
    impl ::google_field_selector::FieldSelector for PropertyOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyOrder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PropertyOrderDirection {
        #[doc = "Ascending."]
        Ascending,
        #[doc = "Descending."]
        Descending,
        #[doc = "Unspecified. This value must not be used."]
        DirectionUnspecified,
    }
    impl PropertyOrderDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                PropertyOrderDirection::Ascending => "ASCENDING",
                PropertyOrderDirection::Descending => "DESCENDING",
                PropertyOrderDirection::DirectionUnspecified => "DIRECTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PropertyOrderDirection {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PropertyOrderDirection {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PropertyOrderDirection, ()> {
            Ok(match s {
                "ASCENDING" => PropertyOrderDirection::Ascending,
                "DESCENDING" => PropertyOrderDirection::Descending,
                "DIRECTION_UNSPECIFIED" => PropertyOrderDirection::DirectionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PropertyOrderDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PropertyOrderDirection {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PropertyOrderDirection {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => PropertyOrderDirection::Ascending,
                "DESCENDING" => PropertyOrderDirection::Descending,
                "DIRECTION_UNSPECIFIED" => PropertyOrderDirection::DirectionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PropertyOrderDirection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyOrderDirection {
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
    pub struct PropertyReference {
        #[doc = "The name of the property. If name includes \".\"s, it may be interpreted as a property name path."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PropertyReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Query {
        #[doc = "The properties to make distinct. The query results will contain the first result for each distinct combination of values for the given properties (if empty, all results are returned)."]
        #[serde(
            rename = "distinctOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distinct_on: ::std::option::Option<Vec<crate::schemas::PropertyReference>>,
        #[doc = "An ending point for the query results. Query cursors are returned in query result batches and [can only be used to limit the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets)."]
        #[serde(
            rename = "endCursor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_cursor: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The filter to apply."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::Filter>,
        #[doc = "The kinds to query (if empty, returns entities of all kinds). Currently at most 1 kind may be specified."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<Vec<crate::schemas::KindExpression>>,
        #[doc = "The maximum number of results to return. Applies after all other constraints. Optional. Unspecified is interpreted as no limit. Must be >= 0 if specified."]
        #[serde(
            rename = "limit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub limit: ::std::option::Option<i32>,
        #[doc = "The number of results to skip. Applies before limit, but after all other constraints. Optional. Must be >= 0 if specified."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset: ::std::option::Option<i32>,
        #[doc = "The order to apply to the query results (if empty, order is unspecified)."]
        #[serde(
            rename = "order",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order: ::std::option::Option<Vec<crate::schemas::PropertyOrder>>,
        #[doc = "The projection to return. Defaults to returning all properties."]
        #[serde(
            rename = "projection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub projection: ::std::option::Option<Vec<crate::schemas::Projection>>,
        #[doc = "A starting point for the query results. Query cursors are returned in query result batches and [can only be used to continue the same query](https://cloud.google.com/datastore/docs/concepts/queries#cursors_limits_and_offsets)."]
        #[serde(
            rename = "startCursor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_cursor: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Query {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Query {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct QueryResultBatch {
        #[doc = "A cursor that points to the position after the last result in the batch."]
        #[serde(
            rename = "endCursor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_cursor: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The result type for every entity in `entity_results`."]
        #[serde(
            rename = "entityResultType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_result_type:
            ::std::option::Option<crate::schemas::QueryResultBatchEntityResultType>,
        #[doc = "The results for this batch."]
        #[serde(
            rename = "entityResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_results: ::std::option::Option<Vec<crate::schemas::EntityResult>>,
        #[doc = "The state of the query after the current batch."]
        #[serde(
            rename = "moreResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub more_results: ::std::option::Option<crate::schemas::QueryResultBatchMoreResults>,
        #[doc = "A cursor that points to the position after the last skipped result. Will be set when `skipped_results` != 0."]
        #[serde(
            rename = "skippedCursor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skipped_cursor: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The number of results skipped, typically because of an offset."]
        #[serde(
            rename = "skippedResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skipped_results: ::std::option::Option<i32>,
        #[doc = "The version number of the snapshot this batch was returned from. This applies to the range of results from the query's `start_cursor` (or the beginning of the query if no cursor was given) to this batch's `end_cursor` (not the query's `end_cursor`). In a single transaction, subsequent query result batches for the same query can have a greater snapshot version number. Each batch's snapshot version is valid for all preceding batches. The value will be zero for eventually consistent queries."]
        #[serde(
            rename = "snapshotVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub snapshot_version: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for QueryResultBatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryResultBatch {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryResultBatchEntityResultType {
        #[doc = "The key and properties."]
        Full,
        #[doc = "Only the key."]
        KeyOnly,
        #[doc = "A projected subset of properties. The entity may have no key."]
        Projection,
        #[doc = "Unspecified. This value is never used."]
        ResultTypeUnspecified,
    }
    impl QueryResultBatchEntityResultType {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryResultBatchEntityResultType::Full => "FULL",
                QueryResultBatchEntityResultType::KeyOnly => "KEY_ONLY",
                QueryResultBatchEntityResultType::Projection => "PROJECTION",
                QueryResultBatchEntityResultType::ResultTypeUnspecified => {
                    "RESULT_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryResultBatchEntityResultType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryResultBatchEntityResultType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryResultBatchEntityResultType, ()> {
            Ok(match s {
                "FULL" => QueryResultBatchEntityResultType::Full,
                "KEY_ONLY" => QueryResultBatchEntityResultType::KeyOnly,
                "PROJECTION" => QueryResultBatchEntityResultType::Projection,
                "RESULT_TYPE_UNSPECIFIED" => {
                    QueryResultBatchEntityResultType::ResultTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryResultBatchEntityResultType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryResultBatchEntityResultType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryResultBatchEntityResultType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FULL" => QueryResultBatchEntityResultType::Full,
                "KEY_ONLY" => QueryResultBatchEntityResultType::KeyOnly,
                "PROJECTION" => QueryResultBatchEntityResultType::Projection,
                "RESULT_TYPE_UNSPECIFIED" => {
                    QueryResultBatchEntityResultType::ResultTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for QueryResultBatchEntityResultType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryResultBatchEntityResultType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryResultBatchMoreResults {
        #[doc = "The query is finished, but there may be more results after the end cursor."]
        MoreResultsAfterCursor,
        #[doc = "The query is finished, but there may be more results after the limit."]
        MoreResultsAfterLimit,
        #[doc = "Unspecified. This value is never used."]
        MoreResultsTypeUnspecified,
        #[doc = "The query is finished, and there are no more results."]
        NoMoreResults,
        #[doc = "There may be additional batches to fetch from this query."]
        NotFinished,
    }
    impl QueryResultBatchMoreResults {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryResultBatchMoreResults::MoreResultsAfterCursor => "MORE_RESULTS_AFTER_CURSOR",
                QueryResultBatchMoreResults::MoreResultsAfterLimit => "MORE_RESULTS_AFTER_LIMIT",
                QueryResultBatchMoreResults::MoreResultsTypeUnspecified => {
                    "MORE_RESULTS_TYPE_UNSPECIFIED"
                }
                QueryResultBatchMoreResults::NoMoreResults => "NO_MORE_RESULTS",
                QueryResultBatchMoreResults::NotFinished => "NOT_FINISHED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryResultBatchMoreResults {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryResultBatchMoreResults {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryResultBatchMoreResults, ()> {
            Ok(match s {
                "MORE_RESULTS_AFTER_CURSOR" => QueryResultBatchMoreResults::MoreResultsAfterCursor,
                "MORE_RESULTS_AFTER_LIMIT" => QueryResultBatchMoreResults::MoreResultsAfterLimit,
                "MORE_RESULTS_TYPE_UNSPECIFIED" => {
                    QueryResultBatchMoreResults::MoreResultsTypeUnspecified
                }
                "NO_MORE_RESULTS" => QueryResultBatchMoreResults::NoMoreResults,
                "NOT_FINISHED" => QueryResultBatchMoreResults::NotFinished,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryResultBatchMoreResults {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryResultBatchMoreResults {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryResultBatchMoreResults {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MORE_RESULTS_AFTER_CURSOR" => QueryResultBatchMoreResults::MoreResultsAfterCursor,
                "MORE_RESULTS_AFTER_LIMIT" => QueryResultBatchMoreResults::MoreResultsAfterLimit,
                "MORE_RESULTS_TYPE_UNSPECIFIED" => {
                    QueryResultBatchMoreResults::MoreResultsTypeUnspecified
                }
                "NO_MORE_RESULTS" => QueryResultBatchMoreResults::NoMoreResults,
                "NOT_FINISHED" => QueryResultBatchMoreResults::NotFinished,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryResultBatchMoreResults {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryResultBatchMoreResults {
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
    pub struct ReadOnly {}
    impl ::google_field_selector::FieldSelector for ReadOnly {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReadOnly {
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
    pub struct ReadOptions {
        #[doc = "The non-transactional read consistency to use. Cannot be set to `STRONG` for global queries."]
        #[serde(
            rename = "readConsistency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_consistency: ::std::option::Option<crate::schemas::ReadOptionsReadConsistency>,
        #[doc = "The identifier of the transaction in which to read. A transaction identifier is returned by a call to Datastore.BeginTransaction."]
        #[serde(
            rename = "transaction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transaction: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for ReadOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReadOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReadOptionsReadConsistency {
        #[doc = "Eventual consistency."]
        Eventual,
        #[doc = "Unspecified. This value must not be used."]
        ReadConsistencyUnspecified,
        #[doc = "Strong consistency."]
        Strong,
    }
    impl ReadOptionsReadConsistency {
        pub fn as_str(self) -> &'static str {
            match self {
                ReadOptionsReadConsistency::Eventual => "EVENTUAL",
                ReadOptionsReadConsistency::ReadConsistencyUnspecified => {
                    "READ_CONSISTENCY_UNSPECIFIED"
                }
                ReadOptionsReadConsistency::Strong => "STRONG",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReadOptionsReadConsistency {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReadOptionsReadConsistency {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ReadOptionsReadConsistency, ()> {
            Ok(match s {
                "EVENTUAL" => ReadOptionsReadConsistency::Eventual,
                "READ_CONSISTENCY_UNSPECIFIED" => {
                    ReadOptionsReadConsistency::ReadConsistencyUnspecified
                }
                "STRONG" => ReadOptionsReadConsistency::Strong,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReadOptionsReadConsistency {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReadOptionsReadConsistency {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReadOptionsReadConsistency {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EVENTUAL" => ReadOptionsReadConsistency::Eventual,
                "READ_CONSISTENCY_UNSPECIFIED" => {
                    ReadOptionsReadConsistency::ReadConsistencyUnspecified
                }
                "STRONG" => ReadOptionsReadConsistency::Strong,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ReadOptionsReadConsistency {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReadOptionsReadConsistency {
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
    pub struct ReadWrite {
        #[doc = "The transaction identifier of the transaction being retried."]
        #[serde(
            rename = "previousTransaction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub previous_transaction: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for ReadWrite {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReadWrite {
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
    pub struct ReserveIdsRequest {
        #[doc = "If not empty, the ID of the database against which to make the request."]
        #[serde(
            rename = "databaseId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub database_id: ::std::option::Option<String>,
        #[doc = "Required. A list of keys with complete key paths whose numeric IDs should not be auto-allocated."]
        #[serde(
            rename = "keys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keys: ::std::option::Option<Vec<crate::schemas::Key>>,
    }
    impl ::google_field_selector::FieldSelector for ReserveIdsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReserveIdsRequest {
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
    pub struct ReserveIdsResponse {}
    impl ::google_field_selector::FieldSelector for ReserveIdsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReserveIdsResponse {
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
    pub struct RollbackRequest {
        #[doc = "Required. The transaction identifier, returned by a call to Datastore.BeginTransaction."]
        #[serde(
            rename = "transaction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transaction: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for RollbackRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RollbackRequest {
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
    pub struct RollbackResponse {}
    impl ::google_field_selector::FieldSelector for RollbackResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RollbackResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RunQueryRequest {
        #[doc = "The GQL query to run."]
        #[serde(
            rename = "gqlQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gql_query: ::std::option::Option<crate::schemas::GqlQuery>,
        #[doc = "Entities are partitioned into subsets, identified by a partition ID. Queries are scoped to a single partition. This partition ID is normalized with the standard default context partition ID."]
        #[serde(
            rename = "partitionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partition_id: ::std::option::Option<crate::schemas::PartitionId>,
        #[doc = "The query to run."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<crate::schemas::Query>,
        #[doc = "The options for this query."]
        #[serde(
            rename = "readOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_options: ::std::option::Option<crate::schemas::ReadOptions>,
    }
    impl ::google_field_selector::FieldSelector for RunQueryRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunQueryRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RunQueryResponse {
        #[doc = "A batch of query results (always present)."]
        #[serde(
            rename = "batch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub batch: ::std::option::Option<crate::schemas::QueryResultBatch>,
        #[doc = "The parsed form of the `GqlQuery` from the request, if it was set."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<crate::schemas::Query>,
    }
    impl ::google_field_selector::FieldSelector for RunQueryResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunQueryResponse {
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
    pub struct TransactionOptions {
        #[doc = "The transaction should only allow reads."]
        #[serde(
            rename = "readOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_only: ::std::option::Option<crate::schemas::ReadOnly>,
        #[doc = "The transaction should allow both reads and writes."]
        #[serde(
            rename = "readWrite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_write: ::std::option::Option<crate::schemas::ReadWrite>,
    }
    impl ::google_field_selector::FieldSelector for TransactionOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TransactionOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Value {
        #[doc = "An array value. Cannot contain another array value. A `Value` instance that sets field `array_value` must not set fields `meaning` or `exclude_from_indexes`."]
        #[serde(
            rename = "arrayValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub array_value: ::std::option::Option<crate::schemas::ArrayValue>,
        #[doc = "A blob value. May have at most 1,000,000 bytes. When `exclude_from_indexes` is false, may have at most 1500 bytes. In JSON requests, must be base64-encoded."]
        #[serde(
            rename = "blobValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blob_value: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "A boolean value."]
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[doc = "A double value."]
        #[serde(
            rename = "doubleValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_value: ::std::option::Option<f64>,
        #[doc = "An entity value. - May have no key. - May have a key with an incomplete key path. - May have a reserved/read-only key."]
        #[serde(
            rename = "entityValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_value: ::std::option::Option<crate::schemas::Entity>,
        #[doc = "If the value should be excluded from all indexes including those defined explicitly."]
        #[serde(
            rename = "excludeFromIndexes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_from_indexes: ::std::option::Option<bool>,
        #[doc = "A geo point value representing a point on the surface of Earth."]
        #[serde(
            rename = "geoPointValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo_point_value: ::std::option::Option<crate::schemas::LatLng>,
        #[doc = "An integer value."]
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub integer_value: ::std::option::Option<i64>,
        #[doc = "A key value."]
        #[serde(
            rename = "keyValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_value: ::std::option::Option<crate::schemas::Key>,
        #[doc = "The `meaning` field should only be populated for backwards compatibility."]
        #[serde(
            rename = "meaning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub meaning: ::std::option::Option<i32>,
        #[doc = "A null value."]
        #[serde(
            rename = "nullValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub null_value: ::std::option::Option<crate::schemas::ValueNullValue>,
        #[doc = "A UTF-8 encoded string value. When `exclude_from_indexes` is false (it is indexed) , may have at most 1500 bytes. Otherwise, may be set to at most 1,000,000 bytes."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
        #[doc = "A timestamp value. When stored in the Datastore, precise only to microseconds; any additional precision is rounded down."]
        #[serde(
            rename = "timestampValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Value {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Value {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ValueNullValue {
        #[doc = "Null value."]
        NullValue,
    }
    impl ValueNullValue {
        pub fn as_str(self) -> &'static str {
            match self {
                ValueNullValue::NullValue => "NULL_VALUE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ValueNullValue {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ValueNullValue {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ValueNullValue, ()> {
            Ok(match s {
                "NULL_VALUE" => ValueNullValue::NullValue,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ValueNullValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ValueNullValue {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ValueNullValue {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NULL_VALUE" => ValueNullValue::NullValue,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ValueNullValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueNullValue {
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
            #[doc = "Allocates IDs for the given keys, which is useful for referencing an entity before it is inserted."]
            pub fn allocate_ids(
                &self,
                request: crate::schemas::AllocateIdsRequest,
                project_id: impl Into<String>,
            ) -> AllocateIdsRequestBuilder {
                AllocateIdsRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
            #[doc = "Begins a new transaction."]
            pub fn begin_transaction(
                &self,
                request: crate::schemas::BeginTransactionRequest,
                project_id: impl Into<String>,
            ) -> BeginTransactionRequestBuilder {
                BeginTransactionRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
            #[doc = "Commits a transaction, optionally creating, deleting or modifying some entities."]
            pub fn commit(
                &self,
                request: crate::schemas::CommitRequest,
                project_id: impl Into<String>,
            ) -> CommitRequestBuilder {
                CommitRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
            #[doc = "Looks up entities by key."]
            pub fn lookup(
                &self,
                request: crate::schemas::LookupRequest,
                project_id: impl Into<String>,
            ) -> LookupRequestBuilder {
                LookupRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
            #[doc = "Prevents the supplied keys' IDs from being auto-allocated by Cloud Datastore."]
            pub fn reserve_ids(
                &self,
                request: crate::schemas::ReserveIdsRequest,
                project_id: impl Into<String>,
            ) -> ReserveIdsRequestBuilder {
                ReserveIdsRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
            #[doc = "Rolls back a transaction."]
            pub fn rollback(
                &self,
                request: crate::schemas::RollbackRequest,
                project_id: impl Into<String>,
            ) -> RollbackRequestBuilder {
                RollbackRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
            #[doc = "Queries for entities."]
            pub fn run_query(
                &self,
                request: crate::schemas::RunQueryRequest,
                project_id: impl Into<String>,
            ) -> RunQueryRequestBuilder {
                RunQueryRequestBuilder {
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
                    project_id: project_id.into(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::allocate_ids()](struct.ProjectsActions.html#method.allocate_ids)"]
        #[derive(Debug, Clone)]
        pub struct AllocateIdsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::AllocateIdsRequest,
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
        impl<'a> AllocateIdsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::AllocateIdsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AllocateIdsResponse, crate::Error> {
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
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":allocateIds");
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
        #[doc = "Created via [ProjectsActions::begin_transaction()](struct.ProjectsActions.html#method.begin_transaction)"]
        #[derive(Debug, Clone)]
        pub struct BeginTransactionRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BeginTransactionRequest,
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
        impl<'a> BeginTransactionRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::BeginTransactionResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BeginTransactionResponse, crate::Error> {
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
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":beginTransaction");
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
        #[doc = "Created via [ProjectsActions::commit()](struct.ProjectsActions.html#method.commit)"]
        #[derive(Debug, Clone)]
        pub struct CommitRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CommitRequest,
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
        impl<'a> CommitRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::CommitResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CommitResponse, crate::Error> {
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
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":commit");
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
        #[doc = "Created via [ProjectsActions::lookup()](struct.ProjectsActions.html#method.lookup)"]
        #[derive(Debug, Clone)]
        pub struct LookupRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::LookupRequest,
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
        impl<'a> LookupRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::LookupResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::LookupResponse, crate::Error> {
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
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":lookup");
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
        #[doc = "Created via [ProjectsActions::reserve_ids()](struct.ProjectsActions.html#method.reserve_ids)"]
        #[derive(Debug, Clone)]
        pub struct ReserveIdsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ReserveIdsRequest,
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
        impl<'a> ReserveIdsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ReserveIdsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ReserveIdsResponse, crate::Error> {
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
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":reserveIds");
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
        #[doc = "Created via [ProjectsActions::rollback()](struct.ProjectsActions.html#method.rollback)"]
        #[derive(Debug, Clone)]
        pub struct RollbackRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::RollbackRequest,
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
        impl<'a> RollbackRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::RollbackResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RollbackResponse, crate::Error> {
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
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":rollback");
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
        #[doc = "Created via [ProjectsActions::run_query()](struct.ProjectsActions.html#method.run_query)"]
        #[derive(Debug, Clone)]
        pub struct RunQueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::RunQueryRequest,
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
        impl<'a> RunQueryRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::RunQueryResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RunQueryResponse, crate::Error> {
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
                let mut output = "https://datastore.googleapis.com/".to_owned();
                output.push_str("v1beta3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":runQuery");
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
