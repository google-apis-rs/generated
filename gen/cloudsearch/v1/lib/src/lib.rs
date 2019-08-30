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
    pub struct BooleanOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\nboolean property. For example, if operatorName is *closed* and the\nproperty's name is *isClosed*, then queries like\n*closed:<value>* will show results only where the value of the\nproperty named *isClosed* matches *<value>*. By contrast, a\nsearch that uses the same *<value>* without an operator will return\nall items where *<value>* matches the value of any\nString properties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BooleanOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooleanOperatorOptions {
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
    pub struct BooleanPropertyOptions {
        #[doc = "If set, describes how the boolean should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::BooleanOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for BooleanPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooleanPropertyOptions {
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
    pub struct CheckAccessResponse {
        #[doc = "Returns true if principal has access.  Returns false otherwise."]
        #[serde(rename = "hasAccess", default)]
        pub has_access: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CheckAccessResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckAccessResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompositeFilter {
        #[doc = "The logic operator of the sub filter."]
        #[serde(rename = "logicOperator", default)]
        pub logic_operator: ::std::option::Option<crate::schemas::CompositeFilterLogicOperator>,
        #[doc = "Sub filters."]
        #[serde(rename = "subFilters", default)]
        pub sub_filters: ::std::option::Option<Vec<crate::schemas::Filter>>,
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
    pub enum CompositeFilterLogicOperator {
        #[doc = "Logical operators, which can only be applied to sub filters."]
        And,
        #[doc = "NOT can only be applied on a single sub filter."]
        Not,
        Or,
    }
    impl CompositeFilterLogicOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                CompositeFilterLogicOperator::And => "AND",
                CompositeFilterLogicOperator::Not => "NOT",
                CompositeFilterLogicOperator::Or => "OR",
            }
        }
    }
    impl ::std::fmt::Display for CompositeFilterLogicOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompositeFilterLogicOperator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompositeFilterLogicOperator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => CompositeFilterLogicOperator::And,
                "NOT" => CompositeFilterLogicOperator::Not,
                "OR" => CompositeFilterLogicOperator::Or,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompositeFilterLogicOperator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompositeFilterLogicOperator {
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
    pub struct CustomerIndexStats {
        #[doc = "Date for which statistics were calculated."]
        #[serde(rename = "date", default)]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Number of items aggregrated by status code."]
        #[serde(rename = "itemCountByStatus", default)]
        pub item_count_by_status: ::std::option::Option<Vec<crate::schemas::ItemCountByStatus>>,
    }
    impl ::google_field_selector::FieldSelector for CustomerIndexStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerIndexStats {
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
    pub struct DataSource {
        #[doc = "If true, Indexing API rejects any modification calls to this datasource\nsuch as create, update, and delete.\nDisabling this does not imply halting process of previously\naccepted data."]
        #[serde(rename = "disableModifications", default)]
        pub disable_modifications: ::std::option::Option<bool>,
        #[doc = "Disable serving any search or assist results."]
        #[serde(rename = "disableServing", default)]
        pub disable_serving: ::std::option::Option<bool>,
        #[doc = "Required. Display name of the datasource\nThe maximum length is 300 characters."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "List of service accounts that have indexing access."]
        #[serde(rename = "indexingServiceAccounts", default)]
        pub indexing_service_accounts: ::std::option::Option<Vec<String>>,
        #[doc = "This field restricts visibility to items at the datasource level. Items\nwithin the datasource are restricted to the union of users and groups\nincluded in this field. Note that, this does not ensure access to a\nspecific item, as users need to have ACL permissions on the contained\nitems. This ensures a high level access on the entire datasource, and\nthat the individual items are not shared outside this visibility."]
        #[serde(rename = "itemsVisibility", default)]
        pub items_visibility: ::std::option::Option<Vec<crate::schemas::GsuitePrincipal>>,
        #[doc = "Name of the datasource resource.\nFormat: datasources/{source_id}.\n<br />The name is ignored when creating a datasource."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this\nschema."]
        #[serde(rename = "operationIds", default)]
        pub operation_ids: ::std::option::Option<Vec<String>>,
        #[doc = "A short name or alias for the source.  This value will be used to match the\n'source' operator. For example, if the short name is *<value>* then\nqueries like *source:<value>* will only return results for this\nsource. The value must be unique across all datasources. The value must\nonly contain alphanumeric characters (a-zA-Z0-9). The value cannot start\nwith 'google' and cannot be one of the following: mail, gmail, docs, drive,\ngroups, sites, calendar, hangouts, gplus, keep, people, teams.\nIts maximum length is 32 characters."]
        #[serde(rename = "shortName", default)]
        pub short_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DataSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSource {
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
    pub struct DataSourceIndexStats {
        #[doc = "Date for which index stats were calculated. If the date of request is not\nthe current date then stats calculated on the next day are returned. Stats\nare calculated close to mid night in this case. If date of request is\ncurrent date, then real time stats are returned."]
        #[serde(rename = "date", default)]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Number of items aggregrated by status code."]
        #[serde(rename = "itemCountByStatus", default)]
        pub item_count_by_status: ::std::option::Option<Vec<crate::schemas::ItemCountByStatus>>,
    }
    impl ::google_field_selector::FieldSelector for DataSourceIndexStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSourceIndexStats {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DataSourceRestriction {
        #[doc = "Filter options restricting the results. If multiple filters\nare present, they are grouped by object type before joining.\nFilters with the same object type are joined conjunctively, then\nthe resulting expressions are joined disjunctively.\n\nThe maximum number of elements is 20.\n\nNOTE: Suggest API supports only few filters at the moment:\n\"objecttype\", \"type\" and \"mimetype\".\nFor now, schema specific filters cannot be used to filter suggestions."]
        #[serde(rename = "filterOptions", default)]
        pub filter_options: ::std::option::Option<Vec<crate::schemas::FilterOptions>>,
        #[doc = "The source of restriction."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for DataSourceRestriction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSourceRestriction {
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
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
        #[serde(rename = "day", default)]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of date. Must be from 1 to 12."]
        #[serde(rename = "month", default)]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of date. Must be from 1 to 9999."]
        #[serde(rename = "year", default)]
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
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DateOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\ndate property using the greater-than operator. For example, if\ngreaterThanOperatorName is *closedafter* and the property's name is\n*closeDate*, then queries like *closedafter:<value>* will\nshow results only where the value of the property named *closeDate* is\nlater than *<value>*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ndate property using the less-than operator. For example, if\nlessThanOperatorName is *closedbefore* and the property's name is\n*closeDate*, then queries like *closedbefore:<value>* will\nshow results only where the value of the property named *closeDate* is\nearlier than *<value>*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the actual string required in the query in order to isolate the\ndate property. For example, suppose an issue tracking schema object\nhas a property named *closeDate* that specifies an operator with an\noperatorName of *closedon*. For searches on that data, queries like\n*closedon:<value>* will show results only where the value of the\n*closeDate* property matches *<value>*. By contrast, a\nsearch that uses the same *<value>* without an operator will return\nall items where *<value>* matches the value of any String\nproperties or text within the content field for the indexed datasource.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DateOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateOperatorOptions {
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
    pub struct DatePropertyOptions {
        #[doc = "If set, describes how the date should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::DateOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for DatePropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatePropertyOptions {
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
    pub struct DateValues {
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<crate::schemas::Date>>,
    }
    impl ::google_field_selector::FieldSelector for DateValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateValues {
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
    pub struct DebugOptions {
        #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
        #[serde(rename = "enableDebugging", default)]
        pub enable_debugging: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DebugOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DebugOptions {
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
    pub struct DeleteQueueItemsRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Name of a queue to delete items from."]
        #[serde(rename = "queue", default)]
        pub queue: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteQueueItemsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteQueueItemsRequest {
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
    pub struct DisplayedProperty {
        #[doc = "The name of the top-level property as defined in a property definition\nfor the object. If the name is not a defined property in the schema, an\nerror will be given when attempting to update the schema."]
        #[serde(rename = "propertyName", default)]
        pub property_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DisplayedProperty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DisplayedProperty {
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
    pub struct DoubleOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to use the\ndouble property in sorting or as a facet.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DoubleOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoubleOperatorOptions {
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
    pub struct DoublePropertyOptions {
        #[doc = "If set, describes how the double should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::DoubleOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for DoublePropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoublePropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DoubleValues {
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<f64>>,
    }
    impl ::google_field_selector::FieldSelector for DoubleValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoubleValues {
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
    pub struct DriveFollowUpRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DriveFollowUpRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveFollowUpRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveFollowUpRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveFollowUpRestrictType {
        FollowupActionItems,
        FollowupSuggestions,
        Unspecified,
    }
    impl DriveFollowUpRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveFollowUpRestrictType::FollowupActionItems => "FOLLOWUP_ACTION_ITEMS",
                DriveFollowUpRestrictType::FollowupSuggestions => "FOLLOWUP_SUGGESTIONS",
                DriveFollowUpRestrictType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for DriveFollowUpRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveFollowUpRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveFollowUpRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FOLLOWUP_ACTION_ITEMS" => DriveFollowUpRestrictType::FollowupActionItems,
                "FOLLOWUP_SUGGESTIONS" => DriveFollowUpRestrictType::FollowupSuggestions,
                "UNSPECIFIED" => DriveFollowUpRestrictType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveFollowUpRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveFollowUpRestrictType {
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
    pub struct DriveLocationRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DriveLocationRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveLocationRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveLocationRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveLocationRestrictType {
        Starred,
        Trashed,
        Unspecified,
    }
    impl DriveLocationRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveLocationRestrictType::Starred => "STARRED",
                DriveLocationRestrictType::Trashed => "TRASHED",
                DriveLocationRestrictType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for DriveLocationRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveLocationRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveLocationRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STARRED" => DriveLocationRestrictType::Starred,
                "TRASHED" => DriveLocationRestrictType::Trashed,
                "UNSPECIFIED" => DriveLocationRestrictType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveLocationRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveLocationRestrictType {
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
    pub struct DriveMimeTypeRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DriveMimeTypeRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveMimeTypeRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveMimeTypeRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveMimeTypeRestrictType {
        Archive,
        Audio,
        Document,
        Drawing,
        Folder,
        Form,
        Image,
        Map,
        Pdf,
        Presentation,
        Script,
        Site,
        Spreadsheet,
        Unspecified,
        Video,
    }
    impl DriveMimeTypeRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveMimeTypeRestrictType::Archive => "ARCHIVE",
                DriveMimeTypeRestrictType::Audio => "AUDIO",
                DriveMimeTypeRestrictType::Document => "DOCUMENT",
                DriveMimeTypeRestrictType::Drawing => "DRAWING",
                DriveMimeTypeRestrictType::Folder => "FOLDER",
                DriveMimeTypeRestrictType::Form => "FORM",
                DriveMimeTypeRestrictType::Image => "IMAGE",
                DriveMimeTypeRestrictType::Map => "MAP",
                DriveMimeTypeRestrictType::Pdf => "PDF",
                DriveMimeTypeRestrictType::Presentation => "PRESENTATION",
                DriveMimeTypeRestrictType::Script => "SCRIPT",
                DriveMimeTypeRestrictType::Site => "SITE",
                DriveMimeTypeRestrictType::Spreadsheet => "SPREADSHEET",
                DriveMimeTypeRestrictType::Unspecified => "UNSPECIFIED",
                DriveMimeTypeRestrictType::Video => "VIDEO",
            }
        }
    }
    impl ::std::fmt::Display for DriveMimeTypeRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveMimeTypeRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveMimeTypeRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHIVE" => DriveMimeTypeRestrictType::Archive,
                "AUDIO" => DriveMimeTypeRestrictType::Audio,
                "DOCUMENT" => DriveMimeTypeRestrictType::Document,
                "DRAWING" => DriveMimeTypeRestrictType::Drawing,
                "FOLDER" => DriveMimeTypeRestrictType::Folder,
                "FORM" => DriveMimeTypeRestrictType::Form,
                "IMAGE" => DriveMimeTypeRestrictType::Image,
                "MAP" => DriveMimeTypeRestrictType::Map,
                "PDF" => DriveMimeTypeRestrictType::Pdf,
                "PRESENTATION" => DriveMimeTypeRestrictType::Presentation,
                "SCRIPT" => DriveMimeTypeRestrictType::Script,
                "SITE" => DriveMimeTypeRestrictType::Site,
                "SPREADSHEET" => DriveMimeTypeRestrictType::Spreadsheet,
                "UNSPECIFIED" => DriveMimeTypeRestrictType::Unspecified,
                "VIDEO" => DriveMimeTypeRestrictType::Video,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveMimeTypeRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveMimeTypeRestrictType {
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
    pub struct DriveTimeSpanRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DriveTimeSpanRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveTimeSpanRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveTimeSpanRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveTimeSpanRestrictType {
        #[doc = "Not Enabled"]
        Last30Days,
        Last7Days,
        #[doc = "Not Enabled"]
        Last90Days,
        Today,
        Unspecified,
        Yesterday,
    }
    impl DriveTimeSpanRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveTimeSpanRestrictType::Last30Days => "LAST_30_DAYS",
                DriveTimeSpanRestrictType::Last7Days => "LAST_7_DAYS",
                DriveTimeSpanRestrictType::Last90Days => "LAST_90_DAYS",
                DriveTimeSpanRestrictType::Today => "TODAY",
                DriveTimeSpanRestrictType::Unspecified => "UNSPECIFIED",
                DriveTimeSpanRestrictType::Yesterday => "YESTERDAY",
            }
        }
    }
    impl ::std::fmt::Display for DriveTimeSpanRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveTimeSpanRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveTimeSpanRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LAST_30_DAYS" => DriveTimeSpanRestrictType::Last30Days,
                "LAST_7_DAYS" => DriveTimeSpanRestrictType::Last7Days,
                "LAST_90_DAYS" => DriveTimeSpanRestrictType::Last90Days,
                "TODAY" => DriveTimeSpanRestrictType::Today,
                "UNSPECIFIED" => DriveTimeSpanRestrictType::Unspecified,
                "YESTERDAY" => DriveTimeSpanRestrictType::Yesterday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveTimeSpanRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveTimeSpanRestrictType {
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
    pub struct EmailAddress {
        #[doc = "The email address."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EmailAddress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmailAddress {
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
    pub struct EnumOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\nenum property. For example, if operatorName is *priority* and the\nproperty's name is *priorityVal*, then queries like\n*priority:<value>* will show results only where the value of the\nproperty named *priorityVal* matches *<value>*. By contrast, a\nsearch that uses the same *<value>* without an operator will return\nall items where *<value>* matches the value of any String\nproperties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnumOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumOperatorOptions {
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
    pub struct EnumPropertyOptions {
        #[doc = "If set, describes how the enum should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::EnumOperatorOptions>,
        #[doc = "Used to specify the ordered ranking for the enumeration that determines how\nthe integer values provided in the possible EnumValuePairs are used to rank\nresults. If specified, integer values must be provided for all possible\nEnumValuePair values given for this property. Can only be used if\nisRepeatable\nis false."]
        #[serde(rename = "orderedRanking", default)]
        pub ordered_ranking:
            ::std::option::Option<crate::schemas::EnumPropertyOptionsOrderedRanking>,
        #[doc = "The list of possible values for the enumeration property. All\nEnumValuePairs must provide a string value. If you specify an integer value\nfor one EnumValuePair, then all possible EnumValuePairs must provide an\ninteger value. Both the string value and integer value must be unique over\nall possible values. Once set, possible values cannot be removed or\nmodified. If you supply an ordered ranking and think you might insert\nadditional enum values in the future, leave gaps in the initial integer\nvalues to allow adding a value in between previously registered values.\nThe maximum number of elements is 100."]
        #[serde(rename = "possibleValues", default)]
        pub possible_values: ::std::option::Option<Vec<crate::schemas::EnumValuePair>>,
    }
    impl ::google_field_selector::FieldSelector for EnumPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumPropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnumPropertyOptionsOrderedRanking {
        #[doc = "This property is ranked in ascending order. Lower values indicate lower\nranking."]
        Ascending,
        #[doc = "This property is ranked in descending order. Lower values indicate\nhigher ranking."]
        Descending,
        #[doc = "There is no ranking order for the property. Results will not be adjusted\nby this property's value."]
        NoOrder,
    }
    impl EnumPropertyOptionsOrderedRanking {
        pub fn as_str(self) -> &'static str {
            match self {
                EnumPropertyOptionsOrderedRanking::Ascending => "ASCENDING",
                EnumPropertyOptionsOrderedRanking::Descending => "DESCENDING",
                EnumPropertyOptionsOrderedRanking::NoOrder => "NO_ORDER",
            }
        }
    }
    impl ::std::fmt::Display for EnumPropertyOptionsOrderedRanking {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnumPropertyOptionsOrderedRanking {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnumPropertyOptionsOrderedRanking {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => EnumPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => EnumPropertyOptionsOrderedRanking::Descending,
                "NO_ORDER" => EnumPropertyOptionsOrderedRanking::NoOrder,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EnumPropertyOptionsOrderedRanking {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumPropertyOptionsOrderedRanking {
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
    pub struct EnumValuePair {
        #[doc = "The integer value of the EnumValuePair which must be non-negative.\nOptional."]
        #[serde(rename = "integerValue", default)]
        pub integer_value: ::std::option::Option<i32>,
        #[doc = "The string value of the EnumValuePair.\nThe maximum length is 32 characters."]
        #[serde(rename = "stringValue", default)]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnumValuePair {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumValuePair {
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
    pub struct EnumValues {
        #[doc = "The maximum allowable length for string values is 32 characters."]
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for EnumValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumValues {
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
    pub struct ErrorInfo {
        #[serde(rename = "errorMessages", default)]
        pub error_messages: ::std::option::Option<Vec<crate::schemas::ErrorMessage>>,
    }
    impl ::google_field_selector::FieldSelector for ErrorInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorInfo {
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
    pub struct ErrorMessage {
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for ErrorMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorMessage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FacetBucket {
        #[doc = "Number of results that match the bucket value. Counts are only returned\nfor searches when count accuracy is ensured. Can be empty."]
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<i32>,
        #[doc = "Percent of results that match the bucket value. This value is between\n(0-100]. Percentages are returned for all searches, but are an estimate.\nBecause percentages are always returned, you should render percentages\ninstead of counts."]
        #[serde(rename = "percentage", default)]
        pub percentage: ::std::option::Option<i32>,
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::google_field_selector::FieldSelector for FacetBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FacetBucket {
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
    pub struct FacetOptions {
        #[doc = "Maximum number of facet buckets that should be returned for this facet.\nDefaults to 10.\nMaximum value is 100."]
        #[serde(rename = "numFacetBuckets", default)]
        pub num_facet_buckets: ::std::option::Option<i32>,
        #[doc = "If object_type is set, only those objects of that type will be used to\ncompute facets. If empty, then all objects will be used to compute facets."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Name of the operator chosen for faceting. @see\ncloudsearch.SchemaPropertyOptions"]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Source name to facet on. Format: datasources/{source_id}\nIf empty, all data sources will be used."]
        #[serde(rename = "sourceName", default)]
        pub source_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FacetOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FacetOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FacetResult {
        #[doc = "FacetBuckets for values in response containing at least a single result."]
        #[serde(rename = "buckets", default)]
        pub buckets: ::std::option::Option<Vec<crate::schemas::FacetBucket>>,
        #[doc = "Object type for which facet results are returned. Can be empty."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Name of the operator chosen for faceting. @see\ncloudsearch.SchemaPropertyOptions"]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Source name for which facet results are returned. Will not be empty."]
        #[serde(rename = "sourceName", default)]
        pub source_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FacetResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FacetResult {
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
    pub struct FieldViolation {
        #[doc = "Description of the error."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Path of field with violation."]
        #[serde(rename = "field", default)]
        pub field: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FieldViolation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldViolation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Filter {
        #[serde(rename = "compositeFilter", default)]
        pub composite_filter: ::std::option::Option<crate::schemas::CompositeFilter>,
        #[serde(rename = "valueFilter", default)]
        pub value_filter: ::std::option::Option<crate::schemas::ValueFilter>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FilterOptions {
        #[doc = "Generic filter to restrict the search, such as `lang:en`, `site:xyz`."]
        #[serde(rename = "filter", default)]
        pub filter: ::std::option::Option<crate::schemas::Filter>,
        #[doc = "If object_type is set, only objects of that type are returned. This should\ncorrespond to the name of the object that was registered within the\ndefinition of schema. The maximum length is 256 characters."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FilterOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterOptions {
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
    pub struct FreshnessOptions {
        #[doc = "The duration after which an object should be considered\nstale. The default value is 180 days (in seconds)."]
        #[serde(rename = "freshnessDuration", default)]
        pub freshness_duration: ::std::option::Option<String>,
        #[doc = "This property indicates the freshness level of the object in the index.\nIf set, this property must be a top-level property within the\nproperty definitions\nand it must be a\ntimestamp type\nor\ndate type.\nOtherwise, the Indexing API uses\nupdateTime\nas the freshness indicator.\nThe maximum length is 256 characters.\n\nWhen a property is used to calculate fresheness, the value defaults\nto 2 years from the current time."]
        #[serde(rename = "freshnessProperty", default)]
        pub freshness_property: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FreshnessOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FreshnessOptions {
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
    pub struct GetCustomerIndexStatsResponse {
        #[doc = "Summary of indexed item counts, one for each day in the requested range."]
        #[serde(rename = "stats", default)]
        pub stats: ::std::option::Option<Vec<crate::schemas::CustomerIndexStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetCustomerIndexStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetCustomerIndexStatsResponse {
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
    pub struct GetDataSourceIndexStatsResponse {
        #[doc = "Summary of indexed item counts, one for each day in the requested range."]
        #[serde(rename = "stats", default)]
        pub stats: ::std::option::Option<Vec<crate::schemas::DataSourceIndexStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetDataSourceIndexStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetDataSourceIndexStatsResponse {
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
    pub struct GmailActionRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GmailActionRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for GmailActionRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailActionRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GmailActionRestrictType {
        #[doc = "label:mute"]
        Muted,
        #[doc = "is:read"]
        Read,
        #[doc = "label:^io_re"]
        RepliedTo,
        #[doc = "is:unread"]
        Unread,
        Unspecified,
    }
    impl GmailActionRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailActionRestrictType::Muted => "MUTED",
                GmailActionRestrictType::Read => "READ",
                GmailActionRestrictType::RepliedTo => "REPLIED_TO",
                GmailActionRestrictType::Unread => "UNREAD",
                GmailActionRestrictType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GmailActionRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailActionRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailActionRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MUTED" => GmailActionRestrictType::Muted,
                "READ" => GmailActionRestrictType::Read,
                "REPLIED_TO" => GmailActionRestrictType::RepliedTo,
                "UNREAD" => GmailActionRestrictType::Unread,
                "UNSPECIFIED" => GmailActionRestrictType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GmailActionRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailActionRestrictType {
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
    pub struct GmailAttachmentRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GmailAttachmentRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for GmailAttachmentRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailAttachmentRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GmailAttachmentRestrictType {
        #[doc = "has:attachment"]
        HasAttachment,
        #[doc = "has:document"]
        HasDocument,
        #[doc = "has:drive"]
        HasDrive,
        #[doc = "filename:pdf"]
        HasPdf,
        #[doc = "has photos (changes to filename:(jpg OR jpeg OR png)  when typed)"]
        HasPhoto,
        #[doc = "has:presentation"]
        HasPresentation,
        #[doc = "has:spreadsheet"]
        HasSpreadsheet,
        #[doc = "has:youtube"]
        HasYoutube,
        Unspecified,
    }
    impl GmailAttachmentRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailAttachmentRestrictType::HasAttachment => "HAS_ATTACHMENT",
                GmailAttachmentRestrictType::HasDocument => "HAS_DOCUMENT",
                GmailAttachmentRestrictType::HasDrive => "HAS_DRIVE",
                GmailAttachmentRestrictType::HasPdf => "HAS_PDF",
                GmailAttachmentRestrictType::HasPhoto => "HAS_PHOTO",
                GmailAttachmentRestrictType::HasPresentation => "HAS_PRESENTATION",
                GmailAttachmentRestrictType::HasSpreadsheet => "HAS_SPREADSHEET",
                GmailAttachmentRestrictType::HasYoutube => "HAS_YOUTUBE",
                GmailAttachmentRestrictType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GmailAttachmentRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailAttachmentRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailAttachmentRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HAS_ATTACHMENT" => GmailAttachmentRestrictType::HasAttachment,
                "HAS_DOCUMENT" => GmailAttachmentRestrictType::HasDocument,
                "HAS_DRIVE" => GmailAttachmentRestrictType::HasDrive,
                "HAS_PDF" => GmailAttachmentRestrictType::HasPdf,
                "HAS_PHOTO" => GmailAttachmentRestrictType::HasPhoto,
                "HAS_PRESENTATION" => GmailAttachmentRestrictType::HasPresentation,
                "HAS_SPREADSHEET" => GmailAttachmentRestrictType::HasSpreadsheet,
                "HAS_YOUTUBE" => GmailAttachmentRestrictType::HasYoutube,
                "UNSPECIFIED" => GmailAttachmentRestrictType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GmailAttachmentRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailAttachmentRestrictType {
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
    pub struct GmailFolderRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GmailFolderRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for GmailFolderRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailFolderRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GmailFolderRestrictType {
        #[doc = "label:chats"]
        Chats,
        #[doc = "in:draft"]
        InDraft,
        #[doc = "in:sent"]
        InSent,
        #[doc = "in:trash"]
        InTrash,
        Unspecified,
        #[doc = "label:<user generated>"]
        UserGeneratedLabel,
    }
    impl GmailFolderRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailFolderRestrictType::Chats => "CHATS",
                GmailFolderRestrictType::InDraft => "IN_DRAFT",
                GmailFolderRestrictType::InSent => "IN_SENT",
                GmailFolderRestrictType::InTrash => "IN_TRASH",
                GmailFolderRestrictType::Unspecified => "UNSPECIFIED",
                GmailFolderRestrictType::UserGeneratedLabel => "USER_GENERATED_LABEL",
            }
        }
    }
    impl ::std::fmt::Display for GmailFolderRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailFolderRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailFolderRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHATS" => GmailFolderRestrictType::Chats,
                "IN_DRAFT" => GmailFolderRestrictType::InDraft,
                "IN_SENT" => GmailFolderRestrictType::InSent,
                "IN_TRASH" => GmailFolderRestrictType::InTrash,
                "UNSPECIFIED" => GmailFolderRestrictType::Unspecified,
                "USER_GENERATED_LABEL" => GmailFolderRestrictType::UserGeneratedLabel,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GmailFolderRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailFolderRestrictType {
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
    pub struct GmailIntelligentRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GmailIntelligentRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for GmailIntelligentRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailIntelligentRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GmailIntelligentRestrictType {
        #[doc = "label:^cob_sm_busreservation"]
        BusReservation,
        #[doc = "label:^cob_sm_rentalcarreservation"]
        CarReservation,
        #[doc = "label:^cob_sm_flightreservation"]
        FlightReservation,
        #[doc = "category:forums"]
        Forums,
        #[doc = "is:important"]
        Important,
        #[doc = "label:^cob_sm_lodgingreservation"]
        LodgingReservation,
        #[doc = "category:promotions"]
        Promotions,
        #[doc = "category:social"]
        Social,
        Unspecified,
        #[doc = "category:updates"]
        Updates,
    }
    impl GmailIntelligentRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailIntelligentRestrictType::BusReservation => "BUS_RESERVATION",
                GmailIntelligentRestrictType::CarReservation => "CAR_RESERVATION",
                GmailIntelligentRestrictType::FlightReservation => "FLIGHT_RESERVATION",
                GmailIntelligentRestrictType::Forums => "FORUMS",
                GmailIntelligentRestrictType::Important => "IMPORTANT",
                GmailIntelligentRestrictType::LodgingReservation => "LODGING_RESERVATION",
                GmailIntelligentRestrictType::Promotions => "PROMOTIONS",
                GmailIntelligentRestrictType::Social => "SOCIAL",
                GmailIntelligentRestrictType::Unspecified => "UNSPECIFIED",
                GmailIntelligentRestrictType::Updates => "UPDATES",
            }
        }
    }
    impl ::std::fmt::Display for GmailIntelligentRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailIntelligentRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailIntelligentRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BUS_RESERVATION" => GmailIntelligentRestrictType::BusReservation,
                "CAR_RESERVATION" => GmailIntelligentRestrictType::CarReservation,
                "FLIGHT_RESERVATION" => GmailIntelligentRestrictType::FlightReservation,
                "FORUMS" => GmailIntelligentRestrictType::Forums,
                "IMPORTANT" => GmailIntelligentRestrictType::Important,
                "LODGING_RESERVATION" => GmailIntelligentRestrictType::LodgingReservation,
                "PROMOTIONS" => GmailIntelligentRestrictType::Promotions,
                "SOCIAL" => GmailIntelligentRestrictType::Social,
                "UNSPECIFIED" => GmailIntelligentRestrictType::Unspecified,
                "UPDATES" => GmailIntelligentRestrictType::Updates,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GmailIntelligentRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailIntelligentRestrictType {
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
    pub struct GmailTimeRestrict {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GmailTimeRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for GmailTimeRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailTimeRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GmailTimeRestrictType {
        #[doc = "This will read as something like \"From November\" and will have operator\nbefore:X after:Y"]
        FromCertainMonth,
        #[doc = "newer_than:30d"]
        FromThisMonth,
        #[doc = "newer_than:7d"]
        FromThisWeek,
        #[doc = "newer_than:1d"]
        FromToday,
        #[doc = "newer_than:2d older_than:1d"]
        FromYesterday,
        #[doc = "older_than:30d"]
        OlderThanAMonth,
        #[doc = "older_than:7d"]
        OlderThanAWeek,
        #[doc = "older_than:1y"]
        OlderThanOneYear,
        #[doc = "older_than:1d"]
        OlderThanToday,
        #[doc = "older_than:2d"]
        OlderThanYesterday,
        Unspecified,
    }
    impl GmailTimeRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailTimeRestrictType::FromCertainMonth => "FROM_CERTAIN_MONTH",
                GmailTimeRestrictType::FromThisMonth => "FROM_THIS_MONTH",
                GmailTimeRestrictType::FromThisWeek => "FROM_THIS_WEEK",
                GmailTimeRestrictType::FromToday => "FROM_TODAY",
                GmailTimeRestrictType::FromYesterday => "FROM_YESTERDAY",
                GmailTimeRestrictType::OlderThanAMonth => "OLDER_THAN_A_MONTH",
                GmailTimeRestrictType::OlderThanAWeek => "OLDER_THAN_A_WEEK",
                GmailTimeRestrictType::OlderThanOneYear => "OLDER_THAN_ONE_YEAR",
                GmailTimeRestrictType::OlderThanToday => "OLDER_THAN_TODAY",
                GmailTimeRestrictType::OlderThanYesterday => "OLDER_THAN_YESTERDAY",
                GmailTimeRestrictType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GmailTimeRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailTimeRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailTimeRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FROM_CERTAIN_MONTH" => GmailTimeRestrictType::FromCertainMonth,
                "FROM_THIS_MONTH" => GmailTimeRestrictType::FromThisMonth,
                "FROM_THIS_WEEK" => GmailTimeRestrictType::FromThisWeek,
                "FROM_TODAY" => GmailTimeRestrictType::FromToday,
                "FROM_YESTERDAY" => GmailTimeRestrictType::FromYesterday,
                "OLDER_THAN_A_MONTH" => GmailTimeRestrictType::OlderThanAMonth,
                "OLDER_THAN_A_WEEK" => GmailTimeRestrictType::OlderThanAWeek,
                "OLDER_THAN_ONE_YEAR" => GmailTimeRestrictType::OlderThanOneYear,
                "OLDER_THAN_TODAY" => GmailTimeRestrictType::OlderThanToday,
                "OLDER_THAN_YESTERDAY" => GmailTimeRestrictType::OlderThanYesterday,
                "UNSPECIFIED" => GmailTimeRestrictType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GmailTimeRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailTimeRestrictType {
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
    pub struct GsuitePrincipal {
        #[doc = "This principal represents all users of the G Suite domain of the\ncustomer."]
        #[serde(rename = "gsuiteDomain", default)]
        pub gsuite_domain: ::std::option::Option<bool>,
        #[doc = "This principal references a G Suite group account"]
        #[serde(rename = "gsuiteGroupEmail", default)]
        pub gsuite_group_email: ::std::option::Option<String>,
        #[doc = "This principal references a G Suite user account"]
        #[serde(rename = "gsuiteUserEmail", default)]
        pub gsuite_user_email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GsuitePrincipal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GsuitePrincipal {
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
    pub struct HtmlOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\nhtml property. For example, if operatorName is *subject* and the\nproperty's name is *subjectLine*, then queries like\n*subject:<value>* will show results only where the value of the\nproperty named *subjectLine* matches *<value>*. By contrast, a\nsearch that uses the same *<value>* without an operator will return\nall items where *<value>* matches the value of any\nhtml properties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HtmlOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HtmlOperatorOptions {
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
    pub struct HtmlPropertyOptions {
        #[doc = "If set, describes how the property should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::HtmlOperatorOptions>,
        #[doc = "Indicates the search quality importance of the tokens within the\nfield when used for retrieval. Can only be set to DEFAULT or NONE."]
        #[serde(rename = "retrievalImportance", default)]
        pub retrieval_importance: ::std::option::Option<crate::schemas::RetrievalImportance>,
    }
    impl ::google_field_selector::FieldSelector for HtmlPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HtmlPropertyOptions {
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
    pub struct HtmlValues {
        #[doc = "The maximum allowable length for html values is 2048 characters."]
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for HtmlValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HtmlValues {
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
    pub struct IndexItemOptions {
        #[doc = "Specifies if the index request should allow gsuite principals that do not\nexist or are deleted in the index request."]
        #[serde(rename = "allowUnknownGsuitePrincipals", default)]
        pub allow_unknown_gsuite_principals: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for IndexItemOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndexItemOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct IndexItemRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[serde(rename = "indexItemOptions", default)]
        pub index_item_options: ::std::option::Option<crate::schemas::IndexItemOptions>,
        #[doc = "Name of the item.  Format:\ndatasources/{source_id}/items/{item_id}"]
        #[serde(rename = "item", default)]
        pub item: ::std::option::Option<crate::schemas::Item>,
        #[doc = "Required. The RequestMode for this request."]
        #[serde(rename = "mode", default)]
        pub mode: ::std::option::Option<crate::schemas::IndexItemRequestMode>,
    }
    impl ::google_field_selector::FieldSelector for IndexItemRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndexItemRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IndexItemRequestMode {
        #[doc = "For changes that are executed after the response is sent back to the\ncaller."]
        Asynchronous,
        #[doc = "For real-time updates."]
        Synchronous,
        #[doc = "Priority is not specified in the update request.\nLeaving priority unspecified results in an update failure."]
        Unspecified,
    }
    impl IndexItemRequestMode {
        pub fn as_str(self) -> &'static str {
            match self {
                IndexItemRequestMode::Asynchronous => "ASYNCHRONOUS",
                IndexItemRequestMode::Synchronous => "SYNCHRONOUS",
                IndexItemRequestMode::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for IndexItemRequestMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IndexItemRequestMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IndexItemRequestMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASYNCHRONOUS" => IndexItemRequestMode::Asynchronous,
                "SYNCHRONOUS" => IndexItemRequestMode::Synchronous,
                "UNSPECIFIED" => IndexItemRequestMode::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IndexItemRequestMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndexItemRequestMode {
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
    pub struct IntegerOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\ninteger property using the greater-than operator. For example, if\ngreaterThanOperatorName is *priorityabove* and the property's name is\n*priorityVal*, then queries like *priorityabove:<value>* will\nshow results only where the value of the property named *priorityVal* is\ngreater than *<value>*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ninteger property using the less-than operator. For example, if\nlessThanOperatorName is *prioritybelow* and the property's name is\n*priorityVal*, then queries like *prioritybelow:<value>* will\nshow results only where the value of the property named *priorityVal* is\nless than *<value>*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ninteger property. For example, if operatorName is *priority* and the\nproperty's name is *priorityVal*, then queries like\n*priority:<value>* will show results only where the value of the\nproperty named *priorityVal* matches *<value>*. By contrast, a\nsearch that uses the same *<value>* without an operator will return\nall items where *<value>* matches the value of any String\nproperties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IntegerOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerOperatorOptions {
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
    pub struct IntegerPropertyOptions {
        #[doc = "The maximum value of the property. The minimum and maximum values for the\nproperty are used to rank results according to the\nordered ranking.\nIndexing requests with values greater than the maximum are accepted and\nranked with the same weight as items indexed with the maximum value."]
        #[serde(rename = "maximumValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub maximum_value: ::std::option::Option<i64>,
        #[doc = "The minimum value of the property. The minimum and maximum values for the\nproperty are used to rank results according to the\nordered ranking.\nIndexing requests with values less than the minimum are accepted and\nranked with the same weight as items indexed with the minimum value."]
        #[serde(rename = "minimumValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub minimum_value: ::std::option::Option<i64>,
        #[doc = "If set, describes how the integer should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::IntegerOperatorOptions>,
        #[doc = "Used to specify the ordered ranking for the integer. Can only be used if\nisRepeatable\nis false."]
        #[serde(rename = "orderedRanking", default)]
        pub ordered_ranking:
            ::std::option::Option<crate::schemas::IntegerPropertyOptionsOrderedRanking>,
    }
    impl ::google_field_selector::FieldSelector for IntegerPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerPropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IntegerPropertyOptionsOrderedRanking {
        #[doc = "This property is ranked in ascending order. Lower values indicate lower\nranking."]
        Ascending,
        #[doc = "This property is ranked in descending order. Lower values indicate\nhigher ranking."]
        Descending,
        #[doc = "There is no ranking order for the property. Results will not be adjusted\nby this property's value."]
        NoOrder,
    }
    impl IntegerPropertyOptionsOrderedRanking {
        pub fn as_str(self) -> &'static str {
            match self {
                IntegerPropertyOptionsOrderedRanking::Ascending => "ASCENDING",
                IntegerPropertyOptionsOrderedRanking::Descending => "DESCENDING",
                IntegerPropertyOptionsOrderedRanking::NoOrder => "NO_ORDER",
            }
        }
    }
    impl ::std::fmt::Display for IntegerPropertyOptionsOrderedRanking {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IntegerPropertyOptionsOrderedRanking {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IntegerPropertyOptionsOrderedRanking {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => IntegerPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => IntegerPropertyOptionsOrderedRanking::Descending,
                "NO_ORDER" => IntegerPropertyOptionsOrderedRanking::NoOrder,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IntegerPropertyOptionsOrderedRanking {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerPropertyOptionsOrderedRanking {
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
    pub struct IntegerValues {
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<i64>>,
    }
    impl ::google_field_selector::FieldSelector for IntegerValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerValues {
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
    pub struct Interaction {
        #[doc = "The time when the user acted on the item.  If multiple actions of the same\ntype exist for a single user, only the most recent action is recorded."]
        #[serde(rename = "interactionTime", default)]
        pub interaction_time: ::std::option::Option<String>,
        #[doc = "The user that acted on the item."]
        #[serde(rename = "principal", default)]
        pub principal: ::std::option::Option<crate::schemas::Principal>,
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::InteractionType>,
    }
    impl ::google_field_selector::FieldSelector for Interaction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Interaction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InteractionType {
        #[doc = "This interaction indicates the user edited the item."]
        Edit,
        #[doc = "Invalid value."]
        Unspecified,
        #[doc = "This interaction indicates the user viewed the item."]
        View,
    }
    impl InteractionType {
        pub fn as_str(self) -> &'static str {
            match self {
                InteractionType::Edit => "EDIT",
                InteractionType::Unspecified => "UNSPECIFIED",
                InteractionType::View => "VIEW",
            }
        }
    }
    impl ::std::fmt::Display for InteractionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InteractionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InteractionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EDIT" => InteractionType::Edit,
                "UNSPECIFIED" => InteractionType::Unspecified,
                "VIEW" => InteractionType::View,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InteractionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InteractionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Item {
        #[doc = "Access control list for this item."]
        #[serde(rename = "acl", default)]
        pub acl: ::std::option::Option<crate::schemas::ItemAcl>,
        #[doc = "Item content to be indexed and made text searchable."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::schemas::ItemContent>,
        #[doc = "Type for this item."]
        #[serde(rename = "itemType", default)]
        pub item_type: ::std::option::Option<crate::schemas::ItemItemType>,
        #[doc = "Metadata information."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::ItemMetadata>,
        #[doc = "Name of the Item. Format:\ndatasources/{source_id}/items/{item_id}\n<br />This is a required field.\nThe maximum length is 1536 characters."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Additional state connector can store for this item.\nThe maximum length is 10000 bytes."]
        #[serde(rename = "payload", default)]
        pub payload: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Queue this item belongs to.\nThe maximum length is 100 characters."]
        #[serde(rename = "queue", default)]
        pub queue: ::std::option::Option<String>,
        #[doc = "Status of the item.\nOutput only field."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::ItemStatus>,
        #[doc = "The structured data for the item that should conform to a registered\nobject definition in the schema for the data source."]
        #[serde(rename = "structuredData", default)]
        pub structured_data: ::std::option::Option<crate::schemas::ItemStructuredData>,
        #[doc = "Required. The indexing system stores the version from the datasource as a\nbyte string and compares the Item version in the index\nto the version of the queued Item using lexical ordering.\n<br /><br />\nCloud Search Indexing won't index or delete any queued item with\na version value that is less than or equal to the version of the\ncurrently indexed item.\nThe maximum length for this field is 1024 bytes."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Item {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Item {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemItemType {
        #[doc = "An item that gets indexed and whose purpose is to supply other items\nwith ACLs and/or contain other items."]
        ContainerItem,
        #[doc = "An item that is indexed for the only purpose of serving information.\nThese items cannot be referred in\ncontainerName\nor inheritAclFrom\nfields."]
        ContentItem,
        Unspecified,
        #[doc = "An item that does not get indexed, but otherwise has the same purpose\nas CONTAINER_ITEM."]
        VirtualContainerItem,
    }
    impl ItemItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemItemType::ContainerItem => "CONTAINER_ITEM",
                ItemItemType::ContentItem => "CONTENT_ITEM",
                ItemItemType::Unspecified => "UNSPECIFIED",
                ItemItemType::VirtualContainerItem => "VIRTUAL_CONTAINER_ITEM",
            }
        }
    }
    impl ::std::fmt::Display for ItemItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemItemType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemItemType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTAINER_ITEM" => ItemItemType::ContainerItem,
                "CONTENT_ITEM" => ItemItemType::ContentItem,
                "UNSPECIFIED" => ItemItemType::Unspecified,
                "VIRTUAL_CONTAINER_ITEM" => ItemItemType::VirtualContainerItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemItemType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemItemType {
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
    pub struct ItemAcl {
        #[doc = "Sets the type of access rules to apply when an item inherits its ACL from a\nparent. This should always be set in tandem with the\ninheritAclFrom\nfield. Also, when the\ninheritAclFrom field\nis set, this field should be set to a valid AclInheritanceType."]
        #[serde(rename = "aclInheritanceType", default)]
        pub acl_inheritance_type: ::std::option::Option<crate::schemas::ItemAclAclInheritanceType>,
        #[doc = "List of principals who are explicitly denied access to the item in search\nresults. While principals are denied access by default, use denied readers\nto handle exceptions and override the list allowed readers.\nThe maximum number of elements is 100."]
        #[serde(rename = "deniedReaders", default)]
        pub denied_readers: ::std::option::Option<Vec<crate::schemas::Principal>>,
        #[doc = "Name of the item to inherit the Access Permission List (ACL) from.\nNote: ACL inheritance *only* provides access permissions\nto child items and does not define structural relationships, nor does it\nprovide convenient ways to delete large groups of items.\nDeleting an ACL parent from the index only alters the access permissions of\nchild items that reference the parent in the\ninheritAclFrom\nfield. The item is still in the index, but may not\nvisible in search results. By contrast, deletion of a container item\nalso deletes all items that reference the container via the\ncontainerName\nfield.\nThe maximum length for this field is 1536 characters."]
        #[serde(rename = "inheritAclFrom", default)]
        pub inherit_acl_from: ::std::option::Option<String>,
        #[doc = "Optional. List of owners for the item. This field has no bearing on\ndocument access permissions. It does, however, offer\na slight ranking boosts items where the querying user is an owner.\nThe maximum number of elements is 5."]
        #[serde(rename = "owners", default)]
        pub owners: ::std::option::Option<Vec<crate::schemas::Principal>>,
        #[doc = "List of principals who are allowed to see the item in search results.\nOptional if inheriting permissions from another item or if the item\nis not intended to be visible, such as\nvirtual\ncontainers.\nThe maximum number of elements is 1000."]
        #[serde(rename = "readers", default)]
        pub readers: ::std::option::Option<Vec<crate::schemas::Principal>>,
    }
    impl ::google_field_selector::FieldSelector for ItemAcl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemAcl {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemAclAclInheritanceType {
        #[doc = "Access is granted only if this item and the parent item specified in\nthe inheritAclFrom\nfield both permit read access."]
        BothPermit,
        #[doc = "During an authorization conflict, the ACL of the child item determines\nits read access."]
        ChildOverride,
        #[doc = "The default value when this item does not inherit an ACL.\nUse NOT_APPLICABLE when\ninheritAclFrom\nis empty.  An item without ACL inheritance can still have ACLs supplied\nby its own readers and\ndeniedReaders fields."]
        NotApplicable,
        #[doc = "During an authorization conflict, the ACL of the parent item\nspecified in the\ninheritAclFrom\nfield determines read access."]
        ParentOverride,
    }
    impl ItemAclAclInheritanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemAclAclInheritanceType::BothPermit => "BOTH_PERMIT",
                ItemAclAclInheritanceType::ChildOverride => "CHILD_OVERRIDE",
                ItemAclAclInheritanceType::NotApplicable => "NOT_APPLICABLE",
                ItemAclAclInheritanceType::ParentOverride => "PARENT_OVERRIDE",
            }
        }
    }
    impl ::std::fmt::Display for ItemAclAclInheritanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemAclAclInheritanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemAclAclInheritanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTH_PERMIT" => ItemAclAclInheritanceType::BothPermit,
                "CHILD_OVERRIDE" => ItemAclAclInheritanceType::ChildOverride,
                "NOT_APPLICABLE" => ItemAclAclInheritanceType::NotApplicable,
                "PARENT_OVERRIDE" => ItemAclAclInheritanceType::ParentOverride,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemAclAclInheritanceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemAclAclInheritanceType {
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
    pub struct ItemContent {
        #[doc = "Upload reference ID of a previously uploaded content via write method."]
        #[serde(rename = "contentDataRef", default)]
        pub content_data_ref: ::std::option::Option<crate::schemas::UploadItemRef>,
        #[serde(rename = "contentFormat", default)]
        pub content_format: ::std::option::Option<crate::schemas::ItemContentContentFormat>,
        #[doc = "Hashing info calculated and provided by the API client for content.\nCan be used with the items.push method to calculate modified state.\nThe maximum length is 2048 characters."]
        #[serde(rename = "hash", default)]
        pub hash: ::std::option::Option<String>,
        #[doc = "Content that is supplied inlined within the update method.\nThe maximum length is 102400 bytes (100 KiB)."]
        #[serde(rename = "inlineContent", default)]
        pub inline_content: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for ItemContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemContent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemContentContentFormat {
        #[doc = "contentFormat is HTML."]
        Html,
        #[doc = "contentFormat is raw bytes."]
        Raw,
        #[doc = "contentFormat is free text."]
        Text,
        #[doc = "Invalid value."]
        Unspecified,
    }
    impl ItemContentContentFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemContentContentFormat::Html => "HTML",
                ItemContentContentFormat::Raw => "RAW",
                ItemContentContentFormat::Text => "TEXT",
                ItemContentContentFormat::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for ItemContentContentFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemContentContentFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemContentContentFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HTML" => ItemContentContentFormat::Html,
                "RAW" => ItemContentContentFormat::Raw,
                "TEXT" => ItemContentContentFormat::Text,
                "UNSPECIFIED" => ItemContentContentFormat::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemContentContentFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemContentContentFormat {
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
    pub struct ItemCountByStatus {
        #[doc = "Number of items matching the status code."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Status of the items."]
        #[serde(rename = "statusCode", default)]
        pub status_code: ::std::option::Option<crate::schemas::ItemCountByStatusStatusCode>,
    }
    impl ::google_field_selector::FieldSelector for ItemCountByStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemCountByStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemCountByStatusStatusCode {
        #[doc = "API has accepted the up-to-date data of this item."]
        Accepted,
        #[doc = "Input-only value.  Used with\nItems.list\nto list all items in the queue, regardless of status."]
        CodeUnspecified,
        #[doc = "Error encountered by Cloud Search while processing this item.\nDetails of the error are in\nrepositoryError."]
        Error,
        #[doc = "Item has been modified in the repository, and is out of date with\nthe version previously accepted into Cloud Search."]
        Modified,
        #[doc = "Item is known to exist in the repository, but is not yet accepted by\nCloud Search.\nAn item can be in this state when\nItems.push\nhas been called for\nan item of this name that did not exist previously."]
        NewItem,
    }
    impl ItemCountByStatusStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemCountByStatusStatusCode::Accepted => "ACCEPTED",
                ItemCountByStatusStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ItemCountByStatusStatusCode::Error => "ERROR",
                ItemCountByStatusStatusCode::Modified => "MODIFIED",
                ItemCountByStatusStatusCode::NewItem => "NEW_ITEM",
            }
        }
    }
    impl ::std::fmt::Display for ItemCountByStatusStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemCountByStatusStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemCountByStatusStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => ItemCountByStatusStatusCode::Accepted,
                "CODE_UNSPECIFIED" => ItemCountByStatusStatusCode::CodeUnspecified,
                "ERROR" => ItemCountByStatusStatusCode::Error,
                "MODIFIED" => ItemCountByStatusStatusCode::Modified,
                "NEW_ITEM" => ItemCountByStatusStatusCode::NewItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemCountByStatusStatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemCountByStatusStatusCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ItemMetadata {
        #[doc = "The name of the container for this item.\nDeletion of the container item leads to automatic deletion of this\nitem.  Note: ACLs are not inherited from a container item.\nTo provide ACL inheritance for an item, use the\ninheritAclFrom\nfield. The maximum length is 1536 characters."]
        #[serde(rename = "containerName", default)]
        pub container_name: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for the item, such as \"en-US\" or \"sr-Latn\". For\nmore information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier.\nThe maximum length is 32 characters."]
        #[serde(rename = "contentLanguage", default)]
        pub content_language: ::std::option::Option<String>,
        #[doc = "The time when the item was created in the source repository."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Hashing value provided by the API caller.\nThis can be used with the\nitems.push\nmethod to calculate modified state.\nThe maximum length is 2048 characters."]
        #[serde(rename = "hash", default)]
        pub hash: ::std::option::Option<String>,
        #[doc = "A list of interactions for the item.  Interactions are used to improve\nSearch quality, but are not exposed to end users.\nThe maximum number of elements is 1000."]
        #[serde(rename = "interactions", default)]
        pub interactions: ::std::option::Option<Vec<crate::schemas::Interaction>>,
        #[doc = "Additional keywords or phrases that should match the item.\nUsed internally for user generated content.\nThe maximum number of elements is 100.\nThe maximum length is 8192 characters."]
        #[serde(rename = "keywords", default)]
        pub keywords: ::std::option::Option<Vec<String>>,
        #[doc = "The original mime-type of\nItemContent.content\nin the source repository.\nThe maximum length is 256 characters."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The type of the item.  This should correspond to the name of an object\ndefinition in the schema registered for the data source.  For example, if\nthe schema for the data source contains an object definition with name\n'document', then item indexing requests for objects of that type should set\nobjectType to 'document'.\nThe maximum length is 256 characters."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Additional search quality metadata of the item"]
        #[serde(rename = "searchQualityMetadata", default)]
        pub search_quality_metadata: ::std::option::Option<crate::schemas::SearchQualityMetadata>,
        #[doc = "Link to the source repository serving the data.  Search results apply\nthis link to the title.\nThe maximum length is 2048 characters."]
        #[serde(rename = "sourceRepositoryUrl", default)]
        pub source_repository_url: ::std::option::Option<String>,
        #[doc = "The title of the item.  If given, this will be the displayed title of the\nSearch result.\nThe maximum length is 2048 characters."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The time when the item was last modified in the source repository."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ItemMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemMetadata {
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
    pub struct ItemStatus {
        #[doc = "Status code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::ItemStatusCode>,
        #[doc = "Error details in case the item is in ERROR state."]
        #[serde(rename = "processingErrors", default)]
        pub processing_errors: ::std::option::Option<Vec<crate::schemas::ProcessingError>>,
        #[doc = "Repository error reported by connector."]
        #[serde(rename = "repositoryErrors", default)]
        pub repository_errors: ::std::option::Option<Vec<crate::schemas::RepositoryError>>,
    }
    impl ::google_field_selector::FieldSelector for ItemStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemStatusCode {
        #[doc = "API has accepted the up-to-date data of this item."]
        Accepted,
        #[doc = "Input-only value.  Used with\nItems.list\nto list all items in the queue, regardless of status."]
        CodeUnspecified,
        #[doc = "Error encountered by Cloud Search while processing this item.\nDetails of the error are in\nrepositoryError."]
        Error,
        #[doc = "Item has been modified in the repository, and is out of date with\nthe version previously accepted into Cloud Search."]
        Modified,
        #[doc = "Item is known to exist in the repository, but is not yet accepted by\nCloud Search.\nAn item can be in this state when\nItems.push\nhas been called for\nan item of this name that did not exist previously."]
        NewItem,
    }
    impl ItemStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemStatusCode::Accepted => "ACCEPTED",
                ItemStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ItemStatusCode::Error => "ERROR",
                ItemStatusCode::Modified => "MODIFIED",
                ItemStatusCode::NewItem => "NEW_ITEM",
            }
        }
    }
    impl ::std::fmt::Display for ItemStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => ItemStatusCode::Accepted,
                "CODE_UNSPECIFIED" => ItemStatusCode::CodeUnspecified,
                "ERROR" => ItemStatusCode::Error,
                "MODIFIED" => ItemStatusCode::Modified,
                "NEW_ITEM" => ItemStatusCode::NewItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemStatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemStatusCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ItemStructuredData {
        #[doc = "Hashing value provided by the API caller.\nThis can be used with the\nitems.push\nmethod to calculate modified state.\nThe maximum length is 2048 characters."]
        #[serde(rename = "hash", default)]
        pub hash: ::std::option::Option<String>,
        #[doc = "The structured data object that should conform to a registered object\ndefinition in the schema for the data source."]
        #[serde(rename = "object", default)]
        pub object: ::std::option::Option<crate::schemas::StructuredDataObject>,
    }
    impl ::google_field_selector::FieldSelector for ItemStructuredData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemStructuredData {
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
    pub struct ListDataSourceResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(rename = "sources", default)]
        pub sources: ::std::option::Option<Vec<crate::schemas::DataSource>>,
    }
    impl ::google_field_selector::FieldSelector for ListDataSourceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDataSourceResponse {
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
    pub struct ListItemNamesForUnmappedIdentityResponse {
        #[serde(rename = "itemNames", default)]
        pub item_names: ::std::option::Option<Vec<String>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListItemNamesForUnmappedIdentityResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListItemNamesForUnmappedIdentityResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListItemsResponse {
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Item>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListItemsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListItemsResponse {
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
    pub struct ListQuerySourcesResponse {
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(rename = "sources", default)]
        pub sources: ::std::option::Option<Vec<crate::schemas::QuerySource>>,
    }
    impl ::google_field_selector::FieldSelector for ListQuerySourcesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListQuerySourcesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListSearchApplicationsResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(rename = "searchApplications", default)]
        pub search_applications: ::std::option::Option<Vec<crate::schemas::SearchApplication>>,
    }
    impl ::google_field_selector::FieldSelector for ListSearchApplicationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSearchApplicationsResponse {
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
    pub struct ListUnmappedIdentitiesResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(rename = "unmappedIdentities", default)]
        pub unmapped_identities: ::std::option::Option<Vec<crate::schemas::UnmappedIdentity>>,
    }
    impl ::google_field_selector::FieldSelector for ListUnmappedIdentitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUnmappedIdentitiesResponse {
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
    pub struct MatchRange {
        #[doc = "End of the match in the snippet."]
        #[serde(rename = "end", default)]
        pub end: ::std::option::Option<i32>,
        #[doc = "Starting position of the match in the snippet."]
        #[serde(rename = "start", default)]
        pub start: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for MatchRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatchRange {
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
    pub struct Media {
        #[doc = "Name of the media resource."]
        #[serde(rename = "resourceName", default)]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Media {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Media {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Metadata {
        #[doc = "The creation time for this document or object in the search result."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Options that specify how to display a structured data search result."]
        #[serde(rename = "displayOptions", default)]
        pub display_options: ::std::option::Option<crate::schemas::ResultDisplayMetadata>,
        #[doc = "Indexed fields in structured data, returned as a generic named property."]
        #[serde(rename = "fields", default)]
        pub fields: ::std::option::Option<Vec<crate::schemas::NamedProperty>>,
        #[doc = "Mime type of the search result."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Object type of the search result."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Owner (usually creator) of the document or object of the search result."]
        #[serde(rename = "owner", default)]
        pub owner: ::std::option::Option<crate::schemas::Person>,
        #[doc = "The named source for the result, such as Gmail."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
        #[doc = "The last modified date for the object in the search result. If not\nset in the item, the value returned here is empty. When\n`updateTime` is used for calculating freshness and is not set, this\nvalue defaults to 2 years from the current time."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
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
    pub struct Metaline {
        #[doc = "The list of displayed properties for the metaline. The maxiumum number of\nproperties is 5."]
        #[serde(rename = "properties", default)]
        pub properties: ::std::option::Option<Vec<crate::schemas::DisplayedProperty>>,
    }
    impl ::google_field_selector::FieldSelector for Metaline {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metaline {
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
    pub struct Name {
        #[doc = "The read-only display name formatted according to the locale specified by\nthe viewer's account or the <code>Accept-Language</code> HTTP header."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Name {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Name {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NamedProperty {
        #[serde(rename = "booleanValue", default)]
        pub boolean_value: ::std::option::Option<bool>,
        #[serde(rename = "dateValues", default)]
        pub date_values: ::std::option::Option<crate::schemas::DateValues>,
        #[serde(rename = "doubleValues", default)]
        pub double_values: ::std::option::Option<crate::schemas::DoubleValues>,
        #[serde(rename = "enumValues", default)]
        pub enum_values: ::std::option::Option<crate::schemas::EnumValues>,
        #[serde(rename = "htmlValues", default)]
        pub html_values: ::std::option::Option<crate::schemas::HtmlValues>,
        #[serde(rename = "integerValues", default)]
        pub integer_values: ::std::option::Option<crate::schemas::IntegerValues>,
        #[doc = "The name of the property.  This name should correspond to the name of the\nproperty that was registered for object definition in the schema.\nThe maximum allowable length for this property is 256 characters."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[serde(rename = "objectValues", default)]
        pub object_values: ::std::option::Option<crate::schemas::ObjectValues>,
        #[serde(rename = "textValues", default)]
        pub text_values: ::std::option::Option<crate::schemas::TextValues>,
        #[serde(rename = "timestampValues", default)]
        pub timestamp_values: ::std::option::Option<crate::schemas::TimestampValues>,
    }
    impl ::google_field_selector::FieldSelector for NamedProperty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedProperty {
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
    pub struct ObjectDefinition {
        #[doc = "Name for the object, which then defines its type. Item indexing requests\nshould set the\nobjectType field\nequal to this value. For example, if *name* is *Document*, then indexing\nrequests for items of type Document should set\nobjectType equal to\n*Document*. Each object definition must be uniquely named within a schema.\nThe name must start with a letter and can only contain letters (A-Z, a-z)\nor numbers (0-9).\nThe maximum length is 256 characters."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The optional object-specific options."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<crate::schemas::ObjectOptions>,
        #[doc = "The property definitions for the object.\nThe maximum number of elements is 1000."]
        #[serde(rename = "propertyDefinitions", default)]
        pub property_definitions: ::std::option::Option<Vec<crate::schemas::PropertyDefinition>>,
    }
    impl ::google_field_selector::FieldSelector for ObjectDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectDefinition {
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
    pub struct ObjectDisplayOptions {
        #[doc = "Defines the properties that will be displayed in the metalines of the\nsearch results. The property values will be displayed in the order given\nhere. If a property holds multiple values, all of the values will be\ndiplayed before the next properties. For this reason, it is a good practice\nto specify singular properties before repeated properties in this list. All\nof the properties must set\nis_returnable\nto true. The maximum number of metalines is 3."]
        #[serde(rename = "metalines", default)]
        pub metalines: ::std::option::Option<Vec<crate::schemas::Metaline>>,
        #[doc = "The user friendly label to display in the search result to inidicate the\ntype of the item. This is OPTIONAL; if not given, an object label will not\nbe displayed on the context line of the search results. The maximum length\nis 32 characters."]
        #[serde(rename = "objectDisplayLabel", default)]
        pub object_display_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ObjectDisplayOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectDisplayOptions {
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
    pub struct ObjectOptions {
        #[doc = "Options that determine how the object is displayed in the Cloud Search\nresults page."]
        #[serde(rename = "displayOptions", default)]
        pub display_options: ::std::option::Option<crate::schemas::ObjectDisplayOptions>,
        #[doc = "The freshness options for an object."]
        #[serde(rename = "freshnessOptions", default)]
        pub freshness_options: ::std::option::Option<crate::schemas::FreshnessOptions>,
    }
    impl ::google_field_selector::FieldSelector for ObjectOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectOptions {
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
    pub struct ObjectPropertyOptions {
        #[doc = "The properties of the sub-object. These properties represent a nested\nobject. For example, if this property represents a postal address, the\nsubobjectProperties might be named *street*, *city*, and *state*.\nThe maximum number of elements is 1000."]
        #[serde(rename = "subobjectProperties", default)]
        pub subobject_properties: ::std::option::Option<Vec<crate::schemas::PropertyDefinition>>,
    }
    impl ::google_field_selector::FieldSelector for ObjectPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectPropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ObjectValues {
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<crate::schemas::StructuredDataObject>>,
    }
    impl ::google_field_selector::FieldSelector for ObjectValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectValues {
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
    pub struct PeopleSuggestion {
        #[doc = "Suggested person. All fields of the person object might not be populated."]
        #[serde(rename = "person", default)]
        pub person: ::std::option::Option<crate::schemas::Person>,
    }
    impl ::google_field_selector::FieldSelector for PeopleSuggestion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PeopleSuggestion {
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
    pub struct Person {
        #[doc = "The person's email addresses"]
        #[serde(rename = "emailAddresses", default)]
        pub email_addresses: ::std::option::Option<Vec<crate::schemas::EmailAddress>>,
        #[doc = "The resource name of the person to provide information about.\nSee <a href=\"https://developers.google.com/people/api/rest/v1/people/get\">\nPeople.get</a> from Google People API."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Obfuscated ID of a person."]
        #[serde(rename = "obfuscatedId", default)]
        pub obfuscated_id: ::std::option::Option<String>,
        #[doc = "The person's name"]
        #[serde(rename = "personNames", default)]
        pub person_names: ::std::option::Option<Vec<crate::schemas::Name>>,
        #[doc = "A person's read-only photo. A picture shown next to the person's name to\nhelp others recognize the person in search results."]
        #[serde(rename = "photos", default)]
        pub photos: ::std::option::Option<Vec<crate::schemas::Photo>>,
    }
    impl ::google_field_selector::FieldSelector for Person {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Person {
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
    pub struct Photo {
        #[doc = "The URL of the photo."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Photo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Photo {
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
    pub struct PollItemsRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Maximum number of items to return.\n<br />The maximum and the default value is 1000"]
        #[serde(rename = "limit", default)]
        pub limit: ::std::option::Option<i32>,
        #[doc = "Queue name to fetch items from.  If unspecified, PollItems will\nfetch from 'default' queue.\nThe maximum length is 100 characters."]
        #[serde(rename = "queue", default)]
        pub queue: ::std::option::Option<String>,
        #[doc = "Limit the items polled to the ones with these statuses."]
        #[serde(rename = "statusCodes", default)]
        pub status_codes:
            ::std::option::Option<Vec<crate::schemas::PollItemsRequestStatusCodesItems>>,
    }
    impl ::google_field_selector::FieldSelector for PollItemsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PollItemsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PollItemsRequestStatusCodesItems {
        Accepted,
        CodeUnspecified,
        Error,
        Modified,
        NewItem,
    }
    impl PollItemsRequestStatusCodesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PollItemsRequestStatusCodesItems::Accepted => "ACCEPTED",
                PollItemsRequestStatusCodesItems::CodeUnspecified => "CODE_UNSPECIFIED",
                PollItemsRequestStatusCodesItems::Error => "ERROR",
                PollItemsRequestStatusCodesItems::Modified => "MODIFIED",
                PollItemsRequestStatusCodesItems::NewItem => "NEW_ITEM",
            }
        }
    }
    impl ::std::fmt::Display for PollItemsRequestStatusCodesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PollItemsRequestStatusCodesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PollItemsRequestStatusCodesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => PollItemsRequestStatusCodesItems::Accepted,
                "CODE_UNSPECIFIED" => PollItemsRequestStatusCodesItems::CodeUnspecified,
                "ERROR" => PollItemsRequestStatusCodesItems::Error,
                "MODIFIED" => PollItemsRequestStatusCodesItems::Modified,
                "NEW_ITEM" => PollItemsRequestStatusCodesItems::NewItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PollItemsRequestStatusCodesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PollItemsRequestStatusCodesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PollItemsResponse {
        #[doc = "Set of items from the queue available for connector to process.\n<br />These items have the following subset of fields populated: <br />\n<br />version\n<br />metadata.hash\n<br />structured_data.hash\n<br />content.hash\n<br />payload\n<br />status\n<br />queue"]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Item>>,
    }
    impl ::google_field_selector::FieldSelector for PollItemsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PollItemsResponse {
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
    pub struct Principal {
        #[doc = "This principal is a group identified using an external identity.\nThe name field must specify the group resource name with this format:\nidentitysources/{source_id}/groups/{ID}"]
        #[serde(rename = "groupResourceName", default)]
        pub group_resource_name: ::std::option::Option<String>,
        #[doc = "This principal is a GSuite user, group or domain."]
        #[serde(rename = "gsuitePrincipal", default)]
        pub gsuite_principal: ::std::option::Option<crate::schemas::GsuitePrincipal>,
        #[doc = "This principal is a user identified using an external identity.\nThe name field must specify the user resource name with this format:\nidentitysources/{source_id}/users/{ID}"]
        #[serde(rename = "userResourceName", default)]
        pub user_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Principal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Principal {
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
    pub struct ProcessingError {
        #[doc = "Error code indicating the nature of the error."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::ProcessingErrorCode>,
        #[doc = "Description of the error."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[doc = "In case the item fields are invalid, this field contains the details\nabout the validation errors."]
        #[serde(rename = "fieldViolations", default)]
        pub field_violations: ::std::option::Option<Vec<crate::schemas::FieldViolation>>,
    }
    impl ::google_field_selector::FieldSelector for ProcessingError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProcessingError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProcessingErrorCode {
        #[doc = "ACL inheritance graph formed a cycle."]
        AclCycle,
        #[doc = "Items with incomplete ACL information due to inheriting other\nitems with broken ACL or having groups with unmapped descendants."]
        IndirectBrokenAcl,
        #[doc = "Item's ACL, metadata, or content is malformed or in invalid state.\nFieldViolations contains more details on where the problem is."]
        MalformedRequest,
        #[doc = "Input only value.  Use this value in Items."]
        ProcessingErrorCodeUnspecified,
        #[doc = "Countent format is unsupported."]
        UnsupportedContentFormat,
    }
    impl ProcessingErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ProcessingErrorCode::AclCycle => "ACL_CYCLE",
                ProcessingErrorCode::IndirectBrokenAcl => "INDIRECT_BROKEN_ACL",
                ProcessingErrorCode::MalformedRequest => "MALFORMED_REQUEST",
                ProcessingErrorCode::ProcessingErrorCodeUnspecified => {
                    "PROCESSING_ERROR_CODE_UNSPECIFIED"
                }
                ProcessingErrorCode::UnsupportedContentFormat => "UNSUPPORTED_CONTENT_FORMAT",
            }
        }
    }
    impl ::std::fmt::Display for ProcessingErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProcessingErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProcessingErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACL_CYCLE" => ProcessingErrorCode::AclCycle,
                "INDIRECT_BROKEN_ACL" => ProcessingErrorCode::IndirectBrokenAcl,
                "MALFORMED_REQUEST" => ProcessingErrorCode::MalformedRequest,
                "PROCESSING_ERROR_CODE_UNSPECIFIED" => {
                    ProcessingErrorCode::ProcessingErrorCodeUnspecified
                }
                "UNSUPPORTED_CONTENT_FORMAT" => ProcessingErrorCode::UnsupportedContentFormat,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ProcessingErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProcessingErrorCode {
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
    pub struct PropertyDefinition {
        #[serde(rename = "booleanPropertyOptions", default)]
        pub boolean_property_options: ::std::option::Option<crate::schemas::BooleanPropertyOptions>,
        #[serde(rename = "datePropertyOptions", default)]
        pub date_property_options: ::std::option::Option<crate::schemas::DatePropertyOptions>,
        #[doc = "Options that determine how the property is displayed in the Cloud Search\nresults page if it is specified to be displayed in the object's\ndisplay options\n."]
        #[serde(rename = "displayOptions", default)]
        pub display_options: ::std::option::Option<crate::schemas::PropertyDisplayOptions>,
        #[serde(rename = "doublePropertyOptions", default)]
        pub double_property_options: ::std::option::Option<crate::schemas::DoublePropertyOptions>,
        #[serde(rename = "enumPropertyOptions", default)]
        pub enum_property_options: ::std::option::Option<crate::schemas::EnumPropertyOptions>,
        #[serde(rename = "htmlPropertyOptions", default)]
        pub html_property_options: ::std::option::Option<crate::schemas::HtmlPropertyOptions>,
        #[serde(rename = "integerPropertyOptions", default)]
        pub integer_property_options: ::std::option::Option<crate::schemas::IntegerPropertyOptions>,
        #[doc = "Indicates that the property can be used for generating facets. Cannot be\ntrue for properties whose type is object. IsReturnable must be true to set\nthis option.\nOnly supported for Boolean, Enum, and Text properties."]
        #[serde(rename = "isFacetable", default)]
        pub is_facetable: ::std::option::Option<bool>,
        #[doc = "Indicates that multiple values are allowed for the property. For example, a\ndocument only has one description but can have multiple comments. Cannot be\ntrue for properties whose type is a boolean.\nIf set to false, properties that contain more than one value will cause the\nindexing request for that item to be rejected."]
        #[serde(rename = "isRepeatable", default)]
        pub is_repeatable: ::std::option::Option<bool>,
        #[doc = "Indicates that the property identifies data that should be returned in\nsearch results via the Query API. If set to *true*, indicates that Query\nAPI users can use matching property fields in results. However, storing\nfields requires more space allocation and uses more bandwidth for search\nqueries, which impacts performance over large datasets. Set to *true* here\nonly if the field is needed for search results. Cannot be true for\nproperties whose type is an object."]
        #[serde(rename = "isReturnable", default)]
        pub is_returnable: ::std::option::Option<bool>,
        #[doc = "Indicates that the property can be used for sorting. Cannot be true for\nproperties that are repeatable. Cannot be true for properties whose type\nis object or user identifier. IsReturnable must be true to set this option.\nOnly supported for Boolean, Date, Double, Integer, and Timestamp\nproperties."]
        #[serde(rename = "isSortable", default)]
        pub is_sortable: ::std::option::Option<bool>,
        #[doc = "Indicates that the property can be used for generating query suggestions."]
        #[serde(rename = "isSuggestable", default)]
        pub is_suggestable: ::std::option::Option<bool>,
        #[doc = "Indicates that users can perform wildcard search for this\nproperty. Only supported for Text properties. IsReturnable must be true to\nset this option. In a given datasource maximum of 5 properties can be\nmarked as is_wildcard_searchable.\n\nNote: This is an alpha feature and is enabled for whitelisted users only."]
        #[serde(rename = "isWildcardSearchable", default)]
        pub is_wildcard_searchable: ::std::option::Option<bool>,
        #[doc = "The name of the property. Item indexing requests sent to the Indexing API\nshould set the property name\nequal to this value. For example, if name is *subject_line*, then indexing\nrequests for document items with subject fields should set the\nname for that field equal to\n*subject_line*. Use the name as the identifier for the object property.\nOnce registered as a property for an object, you cannot re-use this name\nfor another property within that object.\nThe name must start with a letter and can only contain letters (A-Z, a-z)\nor numbers (0-9).\nThe maximum length is 256 characters."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[serde(rename = "objectPropertyOptions", default)]
        pub object_property_options: ::std::option::Option<crate::schemas::ObjectPropertyOptions>,
        #[serde(rename = "textPropertyOptions", default)]
        pub text_property_options: ::std::option::Option<crate::schemas::TextPropertyOptions>,
        #[serde(rename = "timestampPropertyOptions", default)]
        pub timestamp_property_options:
            ::std::option::Option<crate::schemas::TimestampPropertyOptions>,
    }
    impl ::google_field_selector::FieldSelector for PropertyDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyDefinition {
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
    pub struct PropertyDisplayOptions {
        #[doc = "The user friendly label for the property that will be used if the property\nis specified to be displayed in ObjectDisplayOptions. If given, the display\nlabel will be shown in front of the property values when the property is\npart of the object display options. For example, if the property value is\n'1', the value by itself may not be useful context for the user. If the\ndisplay name given was 'priority', then the user will see 'priority : 1' in\nthe search results which provides clear conext to search users. This is\nOPTIONAL; if not given, only the property values will be displayed.\nThe maximum length is 32 characters."]
        #[serde(rename = "displayLabel", default)]
        pub display_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PropertyDisplayOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyDisplayOptions {
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
    pub struct PushItem {
        #[doc = "Content hash of the item according to the repository. If specified, this is\nused to determine how to modify this\nitem's status. Setting this field and the\ntype field results in argument\nerror.\nThe maximum length is 2048 characters."]
        #[serde(rename = "contentHash", default)]
        pub content_hash: ::std::option::Option<String>,
        #[doc = "Metadata hash of the item according to the repository. If specified, this\nis used to determine how to modify this\nitem's status. Setting this field and the\ntype field results in argument\nerror.\nThe maximum length is 2048 characters."]
        #[serde(rename = "metadataHash", default)]
        pub metadata_hash: ::std::option::Option<String>,
        #[doc = "Provides additional document state information for the connector,\nsuch as an alternate repository ID and other metadata.\nThe maximum length is 8192 bytes."]
        #[serde(rename = "payload", default)]
        pub payload: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Queue to which this item belongs to.  The <code>default</code> queue is\nchosen if this field is not specified. The maximum length is\n512 characters."]
        #[serde(rename = "queue", default)]
        pub queue: ::std::option::Option<String>,
        #[doc = "The type of the push operation that defines the push behavior."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::PushItemType>,
        #[doc = "Populate this field to store Connector or repository error details.\nThis information is displayed in the Admin Console.\nThis field may only be populated when the\nType is\nREPOSITORY_ERROR."]
        #[serde(rename = "repositoryError", default)]
        pub repository_error: ::std::option::Option<crate::schemas::RepositoryError>,
        #[doc = "Structured data hash of the item according to the repository. If specified,\nthis is used to determine how to modify this item's status. Setting this\nfield and the type field\nresults in argument error.\nThe maximum length is 2048 characters."]
        #[serde(rename = "structuredDataHash", default)]
        pub structured_data_hash: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PushItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PushItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PushItemType {
        #[doc = "Indicates that the repository document has been modified or updated since\nthe previous\nupdate\ncall. This changes status to\nMODIFIED state for\nan existing item. If this is called on a non existing item, the status is\nchanged to\nNEW_ITEM."]
        Modified,
        #[doc = "Item in the repository has not been modified since the last update\ncall.  This push operation will set status to\nACCEPTED state."]
        NotModified,
        #[doc = "Connector is facing a repository error regarding this item.  Change\nstatus to\nREPOSITORY_ERROR\nstate. Item is unreserved and rescheduled at a future time determined by\nexponential backoff."]
        RepositoryError,
        #[doc = "Call push with REQUEUE only for items that have been reserved.\nThis action unreserves the item and resets its available time to the\nwall clock time."]
        Requeue,
        #[doc = "Default UNSPECIFIED.  Specifies that the push operation should not modify\nItemStatus"]
        Unspecified,
    }
    impl PushItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                PushItemType::Modified => "MODIFIED",
                PushItemType::NotModified => "NOT_MODIFIED",
                PushItemType::RepositoryError => "REPOSITORY_ERROR",
                PushItemType::Requeue => "REQUEUE",
                PushItemType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for PushItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PushItemType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PushItemType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MODIFIED" => PushItemType::Modified,
                "NOT_MODIFIED" => PushItemType::NotModified,
                "REPOSITORY_ERROR" => PushItemType::RepositoryError,
                "REQUEUE" => PushItemType::Requeue,
                "UNSPECIFIED" => PushItemType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PushItemType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PushItemType {
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
    pub struct PushItemRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Item to push onto the queue."]
        #[serde(rename = "item", default)]
        pub item: ::std::option::Option<crate::schemas::PushItem>,
    }
    impl ::google_field_selector::FieldSelector for PushItemRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PushItemRequest {
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
    pub struct QueryInterpretation {
        #[serde(rename = "interpretationType", default)]
        pub interpretation_type:
            ::std::option::Option<crate::schemas::QueryInterpretationInterpretationType>,
        #[doc = "The interpretation of the query used in search. For example, queries with\nnatural language intent like \"email from john\" will be interpreted as\n\"from:john source:mail\". This field will not be filled when the reason is\nNO_RESULTS_FOUND_FOR_USER_QUERY."]
        #[serde(rename = "interpretedQuery", default)]
        pub interpreted_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryInterpretationInterpretationType {
        #[doc = "The natural language results is mixed with results from original query."]
        Blend,
        #[doc = "No natural language interpretation or the natural language interpretation\nis not used to fetch the search results."]
        None,
        #[doc = "The results from original query are replaced. The reason for replacing\nthe results from original query is populated in the 'Reason' field below."]
        Replace,
    }
    impl QueryInterpretationInterpretationType {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryInterpretationInterpretationType::Blend => "BLEND",
                QueryInterpretationInterpretationType::None => "NONE",
                QueryInterpretationInterpretationType::Replace => "REPLACE",
            }
        }
    }
    impl ::std::fmt::Display for QueryInterpretationInterpretationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryInterpretationInterpretationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryInterpretationInterpretationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLEND" => QueryInterpretationInterpretationType::Blend,
                "NONE" => QueryInterpretationInterpretationType::None,
                "REPLACE" => QueryInterpretationInterpretationType::Replace,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretationInterpretationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretationInterpretationType {
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
    pub struct QueryInterpretationOptions {
        #[doc = "Flag to disable natural language (NL) interpretation of queries. Default is\nfalse, Set to true to disable natural language interpretation. NL\ninterpretation only applies to predefined datasources."]
        #[serde(rename = "disableNlInterpretation", default)]
        pub disable_nl_interpretation: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretationOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretationOptions {
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
    pub struct QueryItem {
        #[doc = "True if the text was generated by means other than a previous user search."]
        #[serde(rename = "isSynthetic", default)]
        pub is_synthetic: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for QueryItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryItem {
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
    pub struct QueryOperator {
        #[doc = "Display name of the operator"]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Potential list of values for the opeatror field. This field is only filled\nwhen we can safely enumerate all the possible values of this operator."]
        #[serde(rename = "enumValues", default)]
        pub enum_values: ::std::option::Option<Vec<String>>,
        #[doc = "Indicates the operator name that can be used to  isolate the property using\nthe greater-than operator."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Can this operator be used to get facets."]
        #[serde(rename = "isFacetable", default)]
        pub is_facetable: ::std::option::Option<bool>,
        #[doc = "Indicates if multiple values can be set for this property."]
        #[serde(rename = "isRepeatable", default)]
        pub is_repeatable: ::std::option::Option<bool>,
        #[doc = "Will the property associated with this facet be returned as part of search\nresults."]
        #[serde(rename = "isReturnable", default)]
        pub is_returnable: ::std::option::Option<bool>,
        #[doc = "Can this operator be used to sort results."]
        #[serde(rename = "isSortable", default)]
        pub is_sortable: ::std::option::Option<bool>,
        #[doc = "Can get suggestions for this field."]
        #[serde(rename = "isSuggestable", default)]
        pub is_suggestable: ::std::option::Option<bool>,
        #[doc = "Indicates the operator name that can be used to  isolate the property using\nthe less-than operator."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "The name of the operator."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Type of the operator."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::QueryOperatorType>,
    }
    impl ::google_field_selector::FieldSelector for QueryOperator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryOperator {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryOperatorType {
        Boolean,
        Date,
        Double,
        Enum,
        Html,
        Integer,
        Text,
        Timestamp,
        #[doc = "Invalid value."]
        Unknown,
    }
    impl QueryOperatorType {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryOperatorType::Boolean => "BOOLEAN",
                QueryOperatorType::Date => "DATE",
                QueryOperatorType::Double => "DOUBLE",
                QueryOperatorType::Enum => "ENUM",
                QueryOperatorType::Html => "HTML",
                QueryOperatorType::Integer => "INTEGER",
                QueryOperatorType::Text => "TEXT",
                QueryOperatorType::Timestamp => "TIMESTAMP",
                QueryOperatorType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for QueryOperatorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryOperatorType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryOperatorType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOLEAN" => QueryOperatorType::Boolean,
                "DATE" => QueryOperatorType::Date,
                "DOUBLE" => QueryOperatorType::Double,
                "ENUM" => QueryOperatorType::Enum,
                "HTML" => QueryOperatorType::Html,
                "INTEGER" => QueryOperatorType::Integer,
                "TEXT" => QueryOperatorType::Text,
                "TIMESTAMP" => QueryOperatorType::Timestamp,
                "UNKNOWN" => QueryOperatorType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryOperatorType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryOperatorType {
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
    pub struct QuerySource {
        #[doc = "Display name of the data source."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "List of all operators applicable for this source."]
        #[serde(rename = "operators", default)]
        pub operators: ::std::option::Option<Vec<crate::schemas::QueryOperator>>,
        #[doc = "A short name or alias for the source.  This value can be used with the\n'source' operator."]
        #[serde(rename = "shortName", default)]
        pub short_name: ::std::option::Option<String>,
        #[doc = "Name of the source"]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for QuerySource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuerySource {
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
    pub struct QuerySuggestion;
    impl ::google_field_selector::FieldSelector for QuerySuggestion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuerySuggestion {
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
    pub struct RepositoryError {
        #[doc = "Message that describes the error. The maximum allowable length\nof the message is 8192 characters."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Error codes.  Matches the definition of HTTP status codes."]
        #[serde(rename = "httpStatusCode", default)]
        pub http_status_code: ::std::option::Option<i32>,
        #[doc = "Type of error."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::RepositoryErrorType>,
    }
    impl ::google_field_selector::FieldSelector for RepositoryError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepositoryError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RepositoryErrorType {
        #[doc = "Failed authentication due to incorrect credentials."]
        AuthenticationError,
        #[doc = "Service account is not authorized for the repository."]
        AuthorizationError,
        #[doc = "Client-related error, such as an invalid request from the connector to\nthe repository server."]
        ClientError,
        #[doc = "Cannot connect to the repository server."]
        ConnectionError,
        #[doc = "DNS problem, such as the DNS server is not responding."]
        DnsError,
        #[doc = "Unknown or unreachable host."]
        NetworkError,
        #[doc = "Quota exceeded."]
        QuotaExceeded,
        #[doc = "Repository server error."]
        ServerError,
        #[doc = "Server temporarily unavailable."]
        ServiceUnavailable,
        #[doc = "Unknown error."]
        Unknown,
    }
    impl RepositoryErrorType {
        pub fn as_str(self) -> &'static str {
            match self {
                RepositoryErrorType::AuthenticationError => "AUTHENTICATION_ERROR",
                RepositoryErrorType::AuthorizationError => "AUTHORIZATION_ERROR",
                RepositoryErrorType::ClientError => "CLIENT_ERROR",
                RepositoryErrorType::ConnectionError => "CONNECTION_ERROR",
                RepositoryErrorType::DnsError => "DNS_ERROR",
                RepositoryErrorType::NetworkError => "NETWORK_ERROR",
                RepositoryErrorType::QuotaExceeded => "QUOTA_EXCEEDED",
                RepositoryErrorType::ServerError => "SERVER_ERROR",
                RepositoryErrorType::ServiceUnavailable => "SERVICE_UNAVAILABLE",
                RepositoryErrorType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for RepositoryErrorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RepositoryErrorType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RepositoryErrorType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTHENTICATION_ERROR" => RepositoryErrorType::AuthenticationError,
                "AUTHORIZATION_ERROR" => RepositoryErrorType::AuthorizationError,
                "CLIENT_ERROR" => RepositoryErrorType::ClientError,
                "CONNECTION_ERROR" => RepositoryErrorType::ConnectionError,
                "DNS_ERROR" => RepositoryErrorType::DnsError,
                "NETWORK_ERROR" => RepositoryErrorType::NetworkError,
                "QUOTA_EXCEEDED" => RepositoryErrorType::QuotaExceeded,
                "SERVER_ERROR" => RepositoryErrorType::ServerError,
                "SERVICE_UNAVAILABLE" => RepositoryErrorType::ServiceUnavailable,
                "UNKNOWN" => RepositoryErrorType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RepositoryErrorType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepositoryErrorType {
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
    pub struct RequestOptions {
        #[doc = "Debug options of the request"]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier.\nFor translations.\n\nWhen specified, the documents in search results are biased towards the\nspecified language.\nSuggest API does not use this parameter. It autocompletes only based on\ncharacters in the query."]
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Id of the application created using SearchApplicationsService."]
        #[serde(rename = "searchApplicationId", default)]
        pub search_application_id: ::std::option::Option<String>,
        #[doc = "Current user's time zone id, such as \"America/Los_Angeles\" or\n\"Australia/Sydney\". These IDs are defined by\n[Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/)\nproject, and currently available in the file\n[timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml)"]
        #[serde(rename = "timeZone", default)]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RequestOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RequestOptions {
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
    pub struct ResetSearchApplicationRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
    }
    impl ::google_field_selector::FieldSelector for ResetSearchApplicationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResetSearchApplicationRequest {
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
    pub struct ResponseDebugInfo {
        #[doc = "General debug info formatted for display."]
        #[serde(rename = "formattedDebugInfo", default)]
        pub formatted_debug_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResponseDebugInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResponseDebugInfo {
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
    pub struct RestrictItem {
        #[doc = "LINT.ThenChange(//depot/google3/java/com/google/apps/search/quality/itemsuggest/utils/SubtypeRerankingUtils.java)"]
        #[serde(rename = "driveFollowUpRestrict", default)]
        pub drive_follow_up_restrict: ::std::option::Option<crate::schemas::DriveFollowUpRestrict>,
        #[serde(rename = "driveLocationRestrict", default)]
        pub drive_location_restrict: ::std::option::Option<crate::schemas::DriveLocationRestrict>,
        #[doc = "LINT.IfChange\nDrive Types."]
        #[serde(rename = "driveMimeTypeRestrict", default)]
        pub drive_mime_type_restrict: ::std::option::Option<crate::schemas::DriveMimeTypeRestrict>,
        #[serde(rename = "driveTimeSpanRestrict", default)]
        pub drive_time_span_restrict: ::std::option::Option<crate::schemas::DriveTimeSpanRestrict>,
        #[serde(rename = "gmailActionRestrict", default)]
        pub gmail_action_restrict: ::std::option::Option<crate::schemas::GmailActionRestrict>,
        #[serde(rename = "gmailAttachmentRestrict", default)]
        pub gmail_attachment_restrict:
            ::std::option::Option<crate::schemas::GmailAttachmentRestrict>,
        #[doc = "Gmail Types."]
        #[serde(rename = "gmailFolderRestrict", default)]
        pub gmail_folder_restrict: ::std::option::Option<crate::schemas::GmailFolderRestrict>,
        #[serde(rename = "gmailIntelligentRestrict", default)]
        pub gmail_intelligent_restrict:
            ::std::option::Option<crate::schemas::GmailIntelligentRestrict>,
        #[serde(rename = "gmailTimeRestrict", default)]
        pub gmail_time_restrict: ::std::option::Option<crate::schemas::GmailTimeRestrict>,
        #[doc = "The search restrict (e.g. \"after:2017-09-11 before:2017-09-12\")."]
        #[serde(rename = "searchOperator", default)]
        pub search_operator: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestrictItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestrictItem {
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
    pub struct ResultCounts {
        #[doc = "Result count information for each source with results."]
        #[serde(rename = "sourceResultCounts", default)]
        pub source_result_counts: ::std::option::Option<Vec<crate::schemas::SourceResultCount>>,
    }
    impl ::google_field_selector::FieldSelector for ResultCounts {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultCounts {
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
    pub struct ResultDebugInfo {
        #[doc = "General debug info formatted for display."]
        #[serde(rename = "formattedDebugInfo", default)]
        pub formatted_debug_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResultDebugInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDebugInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultDisplayField {
        #[doc = "The display label for the property."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The operator name of the property."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "The name value pair for the property."]
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<crate::schemas::NamedProperty>,
    }
    impl ::google_field_selector::FieldSelector for ResultDisplayField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDisplayField {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultDisplayLine {
        #[serde(rename = "fields", default)]
        pub fields: ::std::option::Option<Vec<crate::schemas::ResultDisplayField>>,
    }
    impl ::google_field_selector::FieldSelector for ResultDisplayLine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDisplayLine {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultDisplayMetadata {
        #[doc = "The metalines content to be displayed with the result."]
        #[serde(rename = "metalines", default)]
        pub metalines: ::std::option::Option<Vec<crate::schemas::ResultDisplayLine>>,
        #[doc = "The display label for the object."]
        #[serde(rename = "objectTypeLabel", default)]
        pub object_type_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResultDisplayMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDisplayMetadata {
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
    pub struct RetrievalImportance {
        #[doc = "Indicates the ranking importance given to property when it is matched\nduring retrieval. Once set, the token importance of a property cannot be\nchanged."]
        #[serde(rename = "importance", default)]
        pub importance: ::std::option::Option<crate::schemas::RetrievalImportanceImportance>,
    }
    impl ::google_field_selector::FieldSelector for RetrievalImportance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RetrievalImportance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RetrievalImportanceImportance {
        #[doc = "Treat the match like a body text match."]
        Default,
        #[doc = "Treat the match with higher importance than body text."]
        High,
        #[doc = "Treat the match like a match against title of the item."]
        Highest,
        #[doc = "Treat the match with lower importance than body text."]
        Low,
        #[doc = "Do not match against this field during retrieval. The property can still\nbe used for operator matching, faceting, and suggest if\ndesired."]
        None,
    }
    impl RetrievalImportanceImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                RetrievalImportanceImportance::Default => "DEFAULT",
                RetrievalImportanceImportance::High => "HIGH",
                RetrievalImportanceImportance::Highest => "HIGHEST",
                RetrievalImportanceImportance::Low => "LOW",
                RetrievalImportanceImportance::None => "NONE",
            }
        }
    }
    impl ::std::fmt::Display for RetrievalImportanceImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RetrievalImportanceImportance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RetrievalImportanceImportance {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => RetrievalImportanceImportance::Default,
                "HIGH" => RetrievalImportanceImportance::High,
                "HIGHEST" => RetrievalImportanceImportance::Highest,
                "LOW" => RetrievalImportanceImportance::Low,
                "NONE" => RetrievalImportanceImportance::None,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RetrievalImportanceImportance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RetrievalImportanceImportance {
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
    pub struct Schema {
        #[doc = "The list of top-level objects for the data source.\nThe maximum number of elements is 10."]
        #[serde(rename = "objectDefinitions", default)]
        pub object_definitions: ::std::option::Option<Vec<crate::schemas::ObjectDefinition>>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this\nschema. After modifying the schema, wait for operations to complete\nbefore indexing additional content."]
        #[serde(rename = "operationIds", default)]
        pub operation_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Schema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Schema {
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
    pub struct ScoringConfig {
        #[doc = "Whether to use freshness as a ranking signal. By default, freshness is used\nas a ranking signal. Note that this setting is not available in the Admin\nUI."]
        #[serde(rename = "disableFreshness", default)]
        pub disable_freshness: ::std::option::Option<bool>,
        #[doc = "Whether to personalize the results. By default, personal signals will\nbe used to boost results."]
        #[serde(rename = "disablePersonalization", default)]
        pub disable_personalization: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ScoringConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScoringConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchApplication {
        #[doc = "Retrictions applied to the configurations.\nThe maximum number of elements is 10."]
        #[serde(rename = "dataSourceRestrictions", default)]
        pub data_source_restrictions:
            ::std::option::Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[doc = "The default fields for returning facet results.\nThe sources specified here also have been included in\ndata_source_restrictions\nabove."]
        #[serde(rename = "defaultFacetOptions", default)]
        pub default_facet_options: ::std::option::Option<Vec<crate::schemas::FacetOptions>>,
        #[doc = "The default options for sorting the search results"]
        #[serde(rename = "defaultSortOptions", default)]
        pub default_sort_options: ::std::option::Option<crate::schemas::SortOptions>,
        #[doc = "Display name of the Search Application.\nThe maximum length is 300 characters."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Name of the Search Application.\n<br />Format: searchapplications/{application_id}."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this\nschema. Output only field."]
        #[serde(rename = "operationIds", default)]
        pub operation_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Configuration for ranking results."]
        #[serde(rename = "scoringConfig", default)]
        pub scoring_config: ::std::option::Option<crate::schemas::ScoringConfig>,
        #[doc = "Configuration for a sources specified in data_source_restrictions."]
        #[serde(rename = "sourceConfig", default)]
        pub source_config: ::std::option::Option<Vec<crate::schemas::SourceConfig>>,
    }
    impl ::google_field_selector::FieldSelector for SearchApplication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchApplication {
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
    pub struct SearchItemsByViewUrlRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "The next_page_token value returned from a previous request, if any."]
        #[serde(rename = "pageToken", default)]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Specify the full view URL to find the corresponding item.\nThe maximum length is 2048 characters."]
        #[serde(rename = "viewUrl", default)]
        pub view_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchItemsByViewUrlRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchItemsByViewUrlRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchItemsByViewUrlResponse {
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Item>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchItemsByViewUrlResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchItemsByViewUrlResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchQualityMetadata {
        #[doc = "An indication of the quality of the item, used to influence search quality.\nValue should be between 0.0 (lowest quality) and 1.0 (highest quality). The\ndefault value is 0.0."]
        #[serde(rename = "quality", default)]
        pub quality: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQualityMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQualityMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchRequest {
        #[doc = "The sources to use for querying. If not specified, all data sources\nfrom the current search application are used."]
        #[serde(rename = "dataSourceRestrictions", default)]
        pub data_source_restrictions:
            ::std::option::Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[serde(rename = "facetOptions", default)]
        pub facet_options: ::std::option::Option<Vec<crate::schemas::FacetOptions>>,
        #[doc = "Maximum number of search results to return in one page.\nValid values are between 1 and 100, inclusive.\nDefault value is 10."]
        #[serde(rename = "pageSize", default)]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "The raw query string.\nSee supported search operators in the [Cloud search\nCheat\nSheet](https://gsuite.google.com/learning-center/products/cloudsearch/cheat-sheet/)"]
        #[serde(rename = "query", default)]
        pub query: ::std::option::Option<String>,
        #[doc = "Options to interpret the user query."]
        #[serde(rename = "queryInterpretationOptions", default)]
        pub query_interpretation_options:
            ::std::option::Option<crate::schemas::QueryInterpretationOptions>,
        #[doc = "Request options, such as the search application and user timezone."]
        #[serde(rename = "requestOptions", default)]
        pub request_options: ::std::option::Option<crate::schemas::RequestOptions>,
        #[doc = "The options for sorting the search results"]
        #[serde(rename = "sortOptions", default)]
        pub sort_options: ::std::option::Option<crate::schemas::SortOptions>,
        #[doc = "Starting index of the results."]
        #[serde(rename = "start", default)]
        pub start: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SearchRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchResponse {
        #[doc = "Debugging information about the response."]
        #[serde(rename = "debugInfo", default)]
        pub debug_info: ::std::option::Option<crate::schemas::ResponseDebugInfo>,
        #[doc = "Error information about the response."]
        #[serde(rename = "errorInfo", default)]
        pub error_info: ::std::option::Option<crate::schemas::ErrorInfo>,
        #[doc = "Repeated facet results."]
        #[serde(rename = "facetResults", default)]
        pub facet_results: ::std::option::Option<Vec<crate::schemas::FacetResult>>,
        #[doc = "Whether there are more search results matching the query."]
        #[serde(rename = "hasMoreResults", default)]
        pub has_more_results: ::std::option::Option<bool>,
        #[doc = "Query interpretation result for user query. Empty if query interpretation\nis disabled."]
        #[serde(rename = "queryInterpretation", default)]
        pub query_interpretation: ::std::option::Option<crate::schemas::QueryInterpretation>,
        #[doc = "The estimated result count for this query."]
        #[serde(rename = "resultCountEstimate", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_estimate: ::std::option::Option<i64>,
        #[doc = "The exact result count for this query."]
        #[serde(rename = "resultCountExact", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_exact: ::std::option::Option<i64>,
        #[doc = "Expanded result count information."]
        #[serde(rename = "resultCounts", default)]
        pub result_counts: ::std::option::Option<crate::schemas::ResultCounts>,
        #[doc = "Results from a search query."]
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<Vec<crate::schemas::SearchResult>>,
        #[doc = "Suggested spelling for the query."]
        #[serde(rename = "spellResults", default)]
        pub spell_results: ::std::option::Option<Vec<crate::schemas::SpellResult>>,
        #[doc = "Structured results for the user query. These results are not counted\nagainst the page_size."]
        #[serde(rename = "structuredResults", default)]
        pub structured_results: ::std::option::Option<Vec<crate::schemas::StructuredResult>>,
    }
    impl ::google_field_selector::FieldSelector for SearchResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchResult {
        #[doc = "If source is clustered, provide list of clustered results. There will only\nbe one level of clustered results. If current source is not enabled for\nclustering, this field will be empty."]
        #[serde(rename = "clusteredResults", default)]
        pub clustered_results: ::std::option::Option<Vec<crate::schemas::SearchResult>>,
        #[doc = "Debugging information about this search result."]
        #[serde(rename = "debugInfo", default)]
        pub debug_info: ::std::option::Option<crate::schemas::ResultDebugInfo>,
        #[doc = "Metadata of the search result."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::Metadata>,
        #[doc = "The concatenation of all snippets (summaries) available for this result."]
        #[serde(rename = "snippet", default)]
        pub snippet: ::std::option::Option<crate::schemas::Snippet>,
        #[doc = "Title of the search result."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The URL of the search result. The URL contains a Google redirect to the\nactual item. This URL is signed and shouldn't be changed."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchResult {
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
    pub struct Snippet {
        #[doc = "The matched ranges in the snippet."]
        #[serde(rename = "matchRanges", default)]
        pub match_ranges: ::std::option::Option<Vec<crate::schemas::MatchRange>>,
        #[doc = "The snippet of the document.\nThe snippet of the document. May contain escaped HTML character that\nshould be unescaped prior to rendering."]
        #[serde(rename = "snippet", default)]
        pub snippet: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Snippet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Snippet {
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
    pub struct SortOptions {
        #[doc = "Name of the operator corresponding to the field to sort on.\nThe corresponding property must be marked as\nsortable."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Ascending is the default sort order"]
        #[serde(rename = "sortOrder", default)]
        pub sort_order: ::std::option::Option<crate::schemas::SortOptionsSortOrder>,
    }
    impl ::google_field_selector::FieldSelector for SortOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SortOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SortOptionsSortOrder {
        Ascending,
        Descending,
    }
    impl SortOptionsSortOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                SortOptionsSortOrder::Ascending => "ASCENDING",
                SortOptionsSortOrder::Descending => "DESCENDING",
            }
        }
    }
    impl ::std::fmt::Display for SortOptionsSortOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SortOptionsSortOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SortOptionsSortOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => SortOptionsSortOrder::Ascending,
                "DESCENDING" => SortOptionsSortOrder::Descending,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SortOptionsSortOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SortOptionsSortOrder {
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
    pub struct Source {
        #[doc = "Source name for content indexed by the\nIndexing API."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Predefined content source for Google Apps."]
        #[serde(rename = "predefinedSource", default)]
        pub predefined_source: ::std::option::Option<crate::schemas::SourcePredefinedSource>,
    }
    impl ::google_field_selector::FieldSelector for Source {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Source {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourcePredefinedSource {
        GoogleCalendar,
        GoogleDrive,
        GoogleGmail,
        GoogleGroups,
        GoogleKeep,
        GoogleSites,
        None,
        #[doc = "Suggests people in the organization. Only valid when used\nwith the suggest API. Results in an error when used in the query API."]
        Person,
        #[doc = "Suggests queries issued by the user in the past. Only valid when used\nwith the suggest API. Ignored when used in the query API."]
        QueryHistory,
    }
    impl SourcePredefinedSource {
        pub fn as_str(self) -> &'static str {
            match self {
                SourcePredefinedSource::GoogleCalendar => "GOOGLE_CALENDAR",
                SourcePredefinedSource::GoogleDrive => "GOOGLE_DRIVE",
                SourcePredefinedSource::GoogleGmail => "GOOGLE_GMAIL",
                SourcePredefinedSource::GoogleGroups => "GOOGLE_GROUPS",
                SourcePredefinedSource::GoogleKeep => "GOOGLE_KEEP",
                SourcePredefinedSource::GoogleSites => "GOOGLE_SITES",
                SourcePredefinedSource::None => "NONE",
                SourcePredefinedSource::Person => "PERSON",
                SourcePredefinedSource::QueryHistory => "QUERY_HISTORY",
            }
        }
    }
    impl ::std::fmt::Display for SourcePredefinedSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourcePredefinedSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourcePredefinedSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GOOGLE_CALENDAR" => SourcePredefinedSource::GoogleCalendar,
                "GOOGLE_DRIVE" => SourcePredefinedSource::GoogleDrive,
                "GOOGLE_GMAIL" => SourcePredefinedSource::GoogleGmail,
                "GOOGLE_GROUPS" => SourcePredefinedSource::GoogleGroups,
                "GOOGLE_KEEP" => SourcePredefinedSource::GoogleKeep,
                "GOOGLE_SITES" => SourcePredefinedSource::GoogleSites,
                "NONE" => SourcePredefinedSource::None,
                "PERSON" => SourcePredefinedSource::Person,
                "QUERY_HISTORY" => SourcePredefinedSource::QueryHistory,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SourcePredefinedSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourcePredefinedSource {
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
    pub struct SourceConfig {
        #[doc = "The crowding configuration for the source."]
        #[serde(rename = "crowdingConfig", default)]
        pub crowding_config: ::std::option::Option<crate::schemas::SourceCrowdingConfig>,
        #[doc = "The scoring configuration for the source."]
        #[serde(rename = "scoringConfig", default)]
        pub scoring_config: ::std::option::Option<crate::schemas::SourceScoringConfig>,
        #[doc = "The source for which this configuration is to be used."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for SourceConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceConfig {
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
    pub struct SourceCrowdingConfig {
        #[doc = "Maximum number of results allowed from a source.\nNo limits will be set on results if this value is less than or equal to 0."]
        #[serde(rename = "numResults", default)]
        pub num_results: ::std::option::Option<i32>,
        #[doc = "Maximum number of suggestions allowed from a source.\nNo limits will be set on results if this value is less than or equal to 0."]
        #[serde(rename = "numSuggestions", default)]
        pub num_suggestions: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SourceCrowdingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceCrowdingConfig {
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
    pub struct SourceResultCount {
        #[doc = "Whether there are more search results for this source."]
        #[serde(rename = "hasMoreResults", default)]
        pub has_more_results: ::std::option::Option<bool>,
        #[doc = "The estimated result count for this source."]
        #[serde(rename = "resultCountEstimate", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_estimate: ::std::option::Option<i64>,
        #[doc = "The exact result count for this source."]
        #[serde(rename = "resultCountExact", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_exact: ::std::option::Option<i64>,
        #[doc = "The source the result count information is associated with."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for SourceResultCount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceResultCount {
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
    pub struct SourceScoringConfig {
        #[doc = "Importance of the source."]
        #[serde(rename = "sourceImportance", default)]
        pub source_importance:
            ::std::option::Option<crate::schemas::SourceScoringConfigSourceImportance>,
    }
    impl ::google_field_selector::FieldSelector for SourceScoringConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceScoringConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourceScoringConfigSourceImportance {
        Default,
        High,
        Low,
    }
    impl SourceScoringConfigSourceImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceScoringConfigSourceImportance::Default => "DEFAULT",
                SourceScoringConfigSourceImportance::High => "HIGH",
                SourceScoringConfigSourceImportance::Low => "LOW",
            }
        }
    }
    impl ::std::fmt::Display for SourceScoringConfigSourceImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceScoringConfigSourceImportance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceScoringConfigSourceImportance {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => SourceScoringConfigSourceImportance::Default,
                "HIGH" => SourceScoringConfigSourceImportance::High,
                "LOW" => SourceScoringConfigSourceImportance::Low,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SourceScoringConfigSourceImportance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceScoringConfigSourceImportance {
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
    pub struct SpellResult {
        #[doc = "The suggested spelling of the query."]
        #[serde(rename = "suggestedQuery", default)]
        pub suggested_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SpellResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpellResult {
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
    pub struct StartUploadItemRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
    }
    impl ::google_field_selector::FieldSelector for StartUploadItemRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StartUploadItemRequest {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StructuredDataObject {
        #[doc = "The properties for the object.\nThe maximum number of elements is 1000."]
        #[serde(rename = "properties", default)]
        pub properties: ::std::option::Option<Vec<crate::schemas::NamedProperty>>,
    }
    impl ::google_field_selector::FieldSelector for StructuredDataObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StructuredDataObject {
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
    pub struct StructuredResult {
        #[doc = "Representation of a person"]
        #[serde(rename = "person", default)]
        pub person: ::std::option::Option<crate::schemas::Person>,
    }
    impl ::google_field_selector::FieldSelector for StructuredResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StructuredResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestRequest {
        #[doc = "The sources to use for suggestions. If not specified, all data sources\nfrom the current search application are used.\nSuggestions are based on Gmail titles. Suggestions from third party sources\nare not available."]
        #[serde(rename = "dataSourceRestrictions", default)]
        pub data_source_restrictions:
            ::std::option::Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[doc = "Partial query for which autocomplete suggestions will be shown.\nFor example, if the query is \"sea\", then the server might return\n\"season\", \"search\", \"seagull\" and so on."]
        #[serde(rename = "query", default)]
        pub query: ::std::option::Option<String>,
        #[doc = "Request options, such as the search application and user timezone."]
        #[serde(rename = "requestOptions", default)]
        pub request_options: ::std::option::Option<crate::schemas::RequestOptions>,
    }
    impl ::google_field_selector::FieldSelector for SuggestRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestRequest {
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
    pub struct SuggestResponse {
        #[doc = "List of suggestions."]
        #[serde(rename = "suggestResults", default)]
        pub suggest_results: ::std::option::Option<Vec<crate::schemas::SuggestResult>>,
    }
    impl ::google_field_selector::FieldSelector for SuggestResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestResponse {
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
    pub struct SuggestResult {
        #[doc = "This is present when the suggestion indicates a person. It\ncontains more information about the person - like their email ID,\nname etc."]
        #[serde(rename = "peopleSuggestion", default)]
        pub people_suggestion: ::std::option::Option<crate::schemas::PeopleSuggestion>,
        #[doc = "This field will be present if the suggested query is a word/phrase\ncompletion."]
        #[serde(rename = "querySuggestion", default)]
        pub query_suggestion: ::std::option::Option<crate::schemas::QuerySuggestion>,
        #[doc = "The source of the suggestion."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
        #[doc = "The suggested query that will be used for search, when the user\nclicks on the suggestion"]
        #[serde(rename = "suggestedQuery", default)]
        pub suggested_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SuggestResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestResult {
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
    pub struct TextOperatorOptions {
        #[doc = "If true, the text value will be tokenized as one atomic value in\noperator searches and facet matches. For example, if the operator name is\n\"genre\" and the value is \"science-fiction\" the query restrictions\n\"genre:science\" and \"genre:fiction\" will not match the item;\n\"genre:science-fiction\" will. Value matching is case-sensitive\nand does not remove special characters.\nIf false, the text will be tokenized. For example, if the value is\n\"science-fiction\" the queries \"genre:science\" and \"genre:fiction\" will\nmatch the item."]
        #[serde(rename = "exactMatchWithOperator", default)]
        pub exact_match_with_operator: ::std::option::Option<bool>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntext property. For example, if operatorName is *subject* and the\nproperty's name is *subjectLine*, then queries like\n*subject:<value>* will show results only where the value of the\nproperty named *subjectLine* matches *<value>*. By contrast, a\nsearch that uses the same *<value>* without an operator will return\nall items where *<value>* matches the value of any\ntext properties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TextOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextOperatorOptions {
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
    pub struct TextPropertyOptions {
        #[doc = "If set, describes how the property should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::TextOperatorOptions>,
        #[doc = "Indicates the search quality importance of the tokens within the\nfield when used for retrieval."]
        #[serde(rename = "retrievalImportance", default)]
        pub retrieval_importance: ::std::option::Option<crate::schemas::RetrievalImportance>,
    }
    impl ::google_field_selector::FieldSelector for TextPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextPropertyOptions {
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
    pub struct TextValues {
        #[doc = "The maximum allowable length for text values is 2048 characters."]
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TextValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextValues {
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
    pub struct TimestampOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntimestamp property using the greater-than operator. For example, if\ngreaterThanOperatorName is *closedafter* and the property's name is\n*closeDate*, then queries like *closedafter:<value>* will\nshow results only where the value of the property named *closeDate* is\nlater than *<value>*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntimestamp property using the less-than operator. For example, if\nlessThanOperatorName is *closedbefore* and the property's name is\n*closeDate*, then queries like *closedbefore:<value>* will\nshow results only where the value of the property named *closeDate* is\nearlier than *<value>*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntimestamp property. For example, if operatorName is *closedon* and the\nproperty's name is *closeDate*, then queries like\n*closedon:<value>* will show results only where the value of the\nproperty named *closeDate* matches *<value>*. By contrast, a\nsearch that uses the same *<value>* without an operator will return\nall items where *<value>* matches the value of any String\nproperties or text within the content field for the item. The operator\nname can only contain lowercase letters (a-z). The maximum length is 32\ncharacters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimestampOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimestampOperatorOptions {
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
    pub struct TimestampPropertyOptions {
        #[doc = "If set, describes how the timestamp should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: ::std::option::Option<crate::schemas::TimestampOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for TimestampPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimestampPropertyOptions {
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
    pub struct TimestampValues {
        #[serde(rename = "values", default)]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TimestampValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimestampValues {
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
    pub struct UnmappedIdentity {
        #[doc = "The resource name for an external user."]
        #[serde(rename = "externalIdentity", default)]
        pub external_identity: ::std::option::Option<crate::schemas::Principal>,
        #[doc = "The resolution status for the external identity."]
        #[serde(rename = "resolutionStatusCode", default)]
        pub resolution_status_code:
            ::std::option::Option<crate::schemas::UnmappedIdentityResolutionStatusCode>,
    }
    impl ::google_field_selector::FieldSelector for UnmappedIdentity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnmappedIdentity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UnmappedIdentityResolutionStatusCode {
        #[doc = "Input-only value.  Used to list all unmapped identities regardless of\nstatus."]
        CodeUnspecified,
        #[doc = "IDaaS does not understand the identity source, probably because the\nschema was modified in a non compatible way."]
        IdentitySourceMisconfigured,
        #[doc = "The identity source associated with the identity was either not found or\ndeleted."]
        IdentitySourceNotFound,
        #[doc = "Internal error."]
        InternalError,
        #[doc = "The unmapped identity was not found in IDaaS, and needs to be provided by\nthe user."]
        NotFound,
        #[doc = "The number of users associated with the external identity is too large."]
        TooManyMappingsFound,
    }
    impl UnmappedIdentityResolutionStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                UnmappedIdentityResolutionStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                UnmappedIdentityResolutionStatusCode::IdentitySourceMisconfigured => {
                    "IDENTITY_SOURCE_MISCONFIGURED"
                }
                UnmappedIdentityResolutionStatusCode::IdentitySourceNotFound => {
                    "IDENTITY_SOURCE_NOT_FOUND"
                }
                UnmappedIdentityResolutionStatusCode::InternalError => "INTERNAL_ERROR",
                UnmappedIdentityResolutionStatusCode::NotFound => "NOT_FOUND",
                UnmappedIdentityResolutionStatusCode::TooManyMappingsFound => {
                    "TOO_MANY_MAPPINGS_FOUND"
                }
            }
        }
    }
    impl ::std::fmt::Display for UnmappedIdentityResolutionStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UnmappedIdentityResolutionStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UnmappedIdentityResolutionStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => UnmappedIdentityResolutionStatusCode::CodeUnspecified,
                "IDENTITY_SOURCE_MISCONFIGURED" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceMisconfigured
                }
                "IDENTITY_SOURCE_NOT_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceNotFound
                }
                "INTERNAL_ERROR" => UnmappedIdentityResolutionStatusCode::InternalError,
                "NOT_FOUND" => UnmappedIdentityResolutionStatusCode::NotFound,
                "TOO_MANY_MAPPINGS_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::TooManyMappingsFound
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
    impl ::google_field_selector::FieldSelector for UnmappedIdentityResolutionStatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnmappedIdentityResolutionStatusCode {
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
    pub struct UnreserveItemsRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Name of a queue to unreserve items from."]
        #[serde(rename = "queue", default)]
        pub queue: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UnreserveItemsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnreserveItemsRequest {
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
    pub struct UpdateDataSourceRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::DataSource>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDataSourceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDataSourceRequest {
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
    pub struct UpdateSchemaRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "The new schema for the source."]
        #[serde(rename = "schema", default)]
        pub schema: ::std::option::Option<crate::schemas::Schema>,
        #[doc = "If true, the request will be validated without side effects."]
        #[serde(rename = "validateOnly", default)]
        pub validate_only: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for UpdateSchemaRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateSchemaRequest {
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
    pub struct UploadItemRef {
        #[doc = "Name of the content reference.\nThe maximum length is 2048 characters."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UploadItemRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UploadItemRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Value {
        #[serde(rename = "booleanValue", default)]
        pub boolean_value: ::std::option::Option<bool>,
        #[serde(rename = "dateValue", default)]
        pub date_value: ::std::option::Option<crate::schemas::Date>,
        #[serde(rename = "doubleValue", default)]
        pub double_value: ::std::option::Option<f64>,
        #[serde(rename = "integerValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub integer_value: ::std::option::Option<i64>,
        #[serde(rename = "stringValue", default)]
        pub string_value: ::std::option::Option<String>,
        #[serde(rename = "timestampValue", default)]
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
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ValueFilter {
        #[doc = "The `operator_name` applied to the query, such as *price_greater_than*.\nThe filter can work against both types of filters defined in the schema\nfor your data source:\n<br/><br/>\n\n1. `operator_name`, where the query filters results by the property\n   that matches the value.\n   <br/>\n1. `greater_than_operator_name` or `less_than_operator_name` in your\n   schema. The query filters the results for the property values that are\n   greater than or less than  the supplied value in the query."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "The value to be compared with."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::google_field_selector::FieldSelector for ValueFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueFilter {
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
    #[doc = "Actions that can be performed on the debug resource"]
    pub fn debug(&self) -> crate::resources::debug::DebugActions<A> {
        crate::resources::debug::DebugActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the indexing resource"]
    pub fn indexing(&self) -> crate::resources::indexing::IndexingActions<A> {
        crate::resources::indexing::IndexingActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the media resource"]
    pub fn media(&self) -> crate::resources::media::MediaActions<A> {
        crate::resources::media::MediaActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions<A> {
        crate::resources::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the query resource"]
    pub fn query(&self) -> crate::resources::query::QueryActions<A> {
        crate::resources::query::QueryActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the settings resource"]
    pub fn settings(&self) -> crate::resources::settings::SettingsActions<A> {
        crate::resources::settings::SettingsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the stats resource"]
    pub fn stats(&self) -> crate::resources::stats::StatsActions<A> {
        crate::resources::stats::StatsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod debug {
        pub mod params {}
        pub struct DebugActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> DebugActions<'a, A> {
            #[doc = "Actions that can be performed on the datasources resource"]
            pub fn datasources(
                &self,
            ) -> crate::resources::debug::datasources::DatasourcesActions<A> {
                crate::resources::debug::datasources::DatasourcesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the identitysources resource"]
            pub fn identitysources(
                &self,
            ) -> crate::resources::debug::identitysources::IdentitysourcesActions<A> {
                crate::resources::debug::identitysources::IdentitysourcesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod datasources {
            pub mod params {}
            pub struct DatasourcesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> DatasourcesActions<'a, A> {
                #[doc = "Actions that can be performed on the items resource"]
                pub fn items(
                    &self,
                ) -> crate::resources::debug::datasources::items::ItemsActions<A> {
                    crate::resources::debug::datasources::items::ItemsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            pub mod items {
                pub mod params {}
                pub struct ItemsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> ItemsActions<'a, A> {
                    #[doc = "Checks whether an item is accessible by specified principal."]
                    pub fn check_access(
                        &self,
                        request: crate::schemas::Principal,
                        name: impl Into<String>,
                    ) -> CheckAccessRequestBuilder<A> {
                        CheckAccessRequestBuilder {
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
                            debug_options_enable_debugging: None,
                        }
                    }
                    #[doc = "Fetches the item whose viewUrl exactly matches that of the URL provided\nin the request."]
                    pub fn search_by_view_url(
                        &self,
                        request: crate::schemas::SearchItemsByViewUrlRequest,
                        name: impl Into<String>,
                    ) -> SearchByViewUrlRequestBuilder<A> {
                        SearchByViewUrlRequestBuilder {
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
                    #[doc = "Actions that can be performed on the unmappedids resource"]pub fn unmappedids ( & self ) -> crate :: resources :: debug :: datasources :: items :: unmappedids :: UnmappedidsActions < A >{
                        crate :: resources :: debug :: datasources :: items :: unmappedids :: UnmappedidsActions { reqwest : & self . reqwest , auth : & self . auth }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CheckAccessRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::Principal,
                    name: String,
                    debug_options_enable_debugging: Option<bool>,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> CheckAccessRequestBuilder<'a, A> {
                    #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
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
                    ) -> Result<crate::schemas::CheckAccessResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::CheckAccessResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":checkAccess");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
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
                pub struct SearchByViewUrlRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::SearchItemsByViewUrlRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> SearchByViewUrlRequestBuilder<'a, A> {
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
                        crate::schemas::SearchItemsByViewUrlResponse,
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
                        crate::schemas::SearchItemsByViewUrlResponse,
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:searchByViewUrl");
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
                pub mod unmappedids {
                    pub mod params {}
                    pub struct UnmappedidsActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a A,
                    }
                    impl<'a, A: ::google_api_auth::GetAccessToken> UnmappedidsActions<'a, A> {
                        #[doc = "List all unmapped identities for a specific item."]
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
                                debug_options_enable_debugging: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a A,
                        parent: String,
                        debug_options_enable_debugging: Option<bool>,
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
                        #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                        pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                            self.debug_options_enable_debugging = Some(value);
                            self
                        }
                        #[doc = "Maximum number of items to fetch in a request.\nDefaults to 100."]
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
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                        #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                        #[doc = r" populated fields in the yielded items will be determined by the"]
                        #[doc = r" `FieldSelector` implementation."]
                        pub fn iter_unmapped_identities<T>(
                            self,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.iter_unmapped_identities_with_fields(fields)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_unmapped_identities_with_default_fields(
                            self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::UnmappedIdentity>
                        {
                            self.iter_unmapped_identities_with_fields(None::<String>)
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_unmapped_identities_with_all_fields(
                            self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::UnmappedIdentity>
                        {
                            self.iter_unmapped_identities_with_fields(Some("*"))
                        }
                        pub fn iter_unmapped_identities_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: AsRef<str>,
                        {
                            self.fields = Some({
                                let mut selector =
                                    concat!("nextPageToken,", "unmappedIdentities").to_owned();
                                let items_fields =
                                    fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                                if !items_fields.is_empty() {
                                    selector.push_str("(");
                                    selector.push_str(items_fields);
                                    selector.push_str(")");
                                }
                                selector
                            });
                            crate::iter::PageItemIter::new(self, "unmappedIdentities")
                        }
                        pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                            crate::schemas::ListUnmappedIdentitiesResponse,
                        > {
                            self.iter_with_fields(None::<&str>)
                        }
                        pub fn iter_with_all_fields(
                            self,
                        ) -> crate::iter::PageIter<
                            Self,
                            crate::schemas::ListUnmappedIdentitiesResponse,
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
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
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
                            crate::schemas::ListUnmappedIdentitiesResponse,
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
                            crate::schemas::ListUnmappedIdentitiesResponse,
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
                            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                            output.push_str("v1/debug/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/unmappedids");
                            output
                        }
                        fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                        {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[(
                                "debugOptions.enableDebugging",
                                &self.debug_options_enable_debugging,
                            )]);
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
        pub mod identitysources {
            pub mod params {}
            pub struct IdentitysourcesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> IdentitysourcesActions<'a, A> {
                #[doc = "Actions that can be performed on the items resource"]
                pub fn items(
                    &self,
                ) -> crate::resources::debug::identitysources::items::ItemsActions<A>
                {
                    crate::resources::debug::identitysources::items::ItemsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
                #[doc = "Actions that can be performed on the unmappedids resource"]
                pub fn unmappedids(
                    &self,
                ) -> crate::resources::debug::identitysources::unmappedids::UnmappedidsActions<A>
                {
                    crate::resources::debug::identitysources::unmappedids::UnmappedidsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            pub mod items {
                pub mod params {}
                pub struct ItemsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> ItemsActions<'a, A> {
                    #[doc = "Lists names of items associated with an unmapped identity."]
                    pub fn list_forunmappedidentity(
                        &self,
                        parent: impl Into<String>,
                    ) -> ListForunmappedidentityRequestBuilder<A> {
                        ListForunmappedidentityRequestBuilder {
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
                            debug_options_enable_debugging: None,
                            group_resource_name: None,
                            page_size: None,
                            page_token: None,
                            user_resource_name: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListForunmappedidentityRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    parent: String,
                    debug_options_enable_debugging: Option<bool>,
                    group_resource_name: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    user_resource_name: Option<String>,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> ListForunmappedidentityRequestBuilder<'a, A> {
                    #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = ""]
                    pub fn group_resource_name(mut self, value: impl Into<String>) -> Self {
                        self.group_resource_name = Some(value.into());
                        self
                    }
                    #[doc = "Maximum number of items to fetch in a request.\nDefaults to 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = ""]
                    pub fn user_resource_name(mut self, value: impl Into<String>) -> Self {
                        self.user_resource_name = Some(value.into());
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
                    pub fn iter_item_names<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_item_names_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_item_names_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, String> {
                        self.iter_item_names_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_item_names_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, String> {
                        self.iter_item_names_with_fields(Some("*"))
                    }
                    pub fn iter_item_names_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "itemNames").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "itemNames")
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
                        crate::schemas::ListItemNamesForUnmappedIdentityResponse,
                    > {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<
                        Self,
                        crate::schemas::ListItemNamesForUnmappedIdentityResponse,
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
                        crate::schemas::ListItemNamesForUnmappedIdentityResponse,
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
                        crate::schemas::ListItemNamesForUnmappedIdentityResponse,
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:forunmappedidentity");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
                        let req = req.query(&[("groupResourceName", &self.group_resource_name)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("userResourceName", &self.user_resource_name)]);
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
                    for ListForunmappedidentityRequestBuilder<'a, A>
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
            pub mod unmappedids {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListResolutionStatusCode {
                        CodeUnspecified,
                        IdentitySourceMisconfigured,
                        IdentitySourceNotFound,
                        InternalError,
                        NotFound,
                        TooManyMappingsFound,
                    }
                    impl ListResolutionStatusCode {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListResolutionStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                                ListResolutionStatusCode::IdentitySourceMisconfigured => {
                                    "IDENTITY_SOURCE_MISCONFIGURED"
                                }
                                ListResolutionStatusCode::IdentitySourceNotFound => {
                                    "IDENTITY_SOURCE_NOT_FOUND"
                                }
                                ListResolutionStatusCode::InternalError => "INTERNAL_ERROR",
                                ListResolutionStatusCode::NotFound => "NOT_FOUND",
                                ListResolutionStatusCode::TooManyMappingsFound => {
                                    "TOO_MANY_MAPPINGS_FOUND"
                                }
                            }
                        }
                    }
                    impl ::std::fmt::Display for ListResolutionStatusCode {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListResolutionStatusCode {
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
                    impl<'de> ::serde::Deserialize<'de> for ListResolutionStatusCode {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "CODE_UNSPECIFIED" => ListResolutionStatusCode::CodeUnspecified,
                                "IDENTITY_SOURCE_MISCONFIGURED" => {
                                    ListResolutionStatusCode::IdentitySourceMisconfigured
                                }
                                "IDENTITY_SOURCE_NOT_FOUND" => {
                                    ListResolutionStatusCode::IdentitySourceNotFound
                                }
                                "INTERNAL_ERROR" => ListResolutionStatusCode::InternalError,
                                "NOT_FOUND" => ListResolutionStatusCode::NotFound,
                                "TOO_MANY_MAPPINGS_FOUND" => {
                                    ListResolutionStatusCode::TooManyMappingsFound
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
                    impl ::google_field_selector::FieldSelector for ListResolutionStatusCode {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for ListResolutionStatusCode {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct UnmappedidsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> UnmappedidsActions<'a, A> {
                    #[doc = "Lists unmapped user identities for an identity source."]
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
                            debug_options_enable_debugging: None,
                            page_size: None,
                            page_token: None,
                            resolution_status_code: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a A , parent : String , debug_options_enable_debugging : Option < bool > , page_size : Option < i32 > , page_token : Option < String > , resolution_status_code : Option < crate :: resources :: debug :: identitysources :: unmappedids :: params :: ListResolutionStatusCode > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                    #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = "Maximum number of items to fetch in a request.\nDefaults to 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "Limit users selection to this status."]
                    pub fn resolution_status_code(
                        mut self,
                        value : crate :: resources :: debug :: identitysources :: unmappedids :: params :: ListResolutionStatusCode,
                    ) -> Self {
                        self.resolution_status_code = Some(value);
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
                    pub fn iter_unmapped_identities<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_unmapped_identities_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_unmapped_identities_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::UnmappedIdentity>
                    {
                        self.iter_unmapped_identities_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_unmapped_identities_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::UnmappedIdentity>
                    {
                        self.iter_unmapped_identities_with_fields(Some("*"))
                    }
                    pub fn iter_unmapped_identities_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector =
                                concat!("nextPageToken,", "unmappedIdentities").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "unmappedIdentities")
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
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListUnmappedIdentitiesResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListUnmappedIdentitiesResponse>
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
                        crate::schemas::ListUnmappedIdentitiesResponse,
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
                        crate::schemas::ListUnmappedIdentitiesResponse,
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/unmappedids");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req =
                            req.query(&[("resolutionStatusCode", &self.resolution_status_code)]);
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
    pub mod indexing {
        pub mod params {}
        pub struct IndexingActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> IndexingActions<'a, A> {
            #[doc = "Actions that can be performed on the datasources resource"]
            pub fn datasources(
                &self,
            ) -> crate::resources::indexing::datasources::DatasourcesActions<A> {
                crate::resources::indexing::datasources::DatasourcesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod datasources {
            pub mod params {}
            pub struct DatasourcesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> DatasourcesActions<'a, A> {
                #[doc = "Deletes the schema of a data source."]
                pub fn delete_schema(
                    &self,
                    name: impl Into<String>,
                ) -> DeleteSchemaRequestBuilder<A> {
                    DeleteSchemaRequestBuilder {
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Gets the schema of a data source."]
                pub fn get_schema(&self, name: impl Into<String>) -> GetSchemaRequestBuilder<A> {
                    GetSchemaRequestBuilder {
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Updates the schema of a data source."]
                pub fn update_schema(
                    &self,
                    request: crate::schemas::UpdateSchemaRequest,
                    name: impl Into<String>,
                ) -> UpdateSchemaRequestBuilder<A> {
                    UpdateSchemaRequestBuilder {
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
                #[doc = "Actions that can be performed on the items resource"]
                pub fn items(
                    &self,
                ) -> crate::resources::indexing::datasources::items::ItemsActions<A>
                {
                    crate::resources::indexing::datasources::items::ItemsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteSchemaRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> DeleteSchemaRequestBuilder<'a, A> {
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/schema");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
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
            pub struct GetSchemaRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> GetSchemaRequestBuilder<'a, A> {
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::Schema, Box<dyn ::std::error::Error>> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Schema, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/schema");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
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
            pub struct UpdateSchemaRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::UpdateSchemaRequest,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> UpdateSchemaRequestBuilder<'a, A> {
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/schema");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            pub mod items {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum DeleteMode {
                        Asynchronous,
                        Synchronous,
                        Unspecified,
                    }
                    impl DeleteMode {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                DeleteMode::Asynchronous => "ASYNCHRONOUS",
                                DeleteMode::Synchronous => "SYNCHRONOUS",
                                DeleteMode::Unspecified => "UNSPECIFIED",
                            }
                        }
                    }
                    impl ::std::fmt::Display for DeleteMode {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for DeleteMode {
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
                    impl<'de> ::serde::Deserialize<'de> for DeleteMode {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "ASYNCHRONOUS" => DeleteMode::Asynchronous,
                                "SYNCHRONOUS" => DeleteMode::Synchronous,
                                "UNSPECIFIED" => DeleteMode::Unspecified,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for DeleteMode {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for DeleteMode {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct ItemsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> ItemsActions<'a, A> {
                    #[doc = "Deletes Item resource for the\nspecified resource name."]
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
                            connector_name: None,
                            debug_options_enable_debugging: None,
                            mode: None,
                            version: None,
                        }
                    }
                    #[doc = "Deletes all items in a queue. This method is useful for deleting stale\nitems."]
                    pub fn delete_queue_items(
                        &self,
                        request: crate::schemas::DeleteQueueItemsRequest,
                        name: impl Into<String>,
                    ) -> DeleteQueueItemsRequestBuilder<A> {
                        DeleteQueueItemsRequestBuilder {
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
                    #[doc = "Gets Item resource by item name."]
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
                            connector_name: None,
                            debug_options_enable_debugging: None,
                        }
                    }
                    #[doc = "Updates Item ACL, metadata, and\ncontent. It will insert the Item if it\ndoes not exist.\nThis method does not support partial updates.  Fields with no provided\nvalues are cleared out in the Cloud Search index."]
                    pub fn index(
                        &self,
                        request: crate::schemas::IndexItemRequest,
                        name: impl Into<String>,
                    ) -> IndexRequestBuilder<A> {
                        IndexRequestBuilder {
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
                    #[doc = "Lists all or a subset of Item resources."]
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
                            brief: None,
                            connector_name: None,
                            debug_options_enable_debugging: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Polls for unreserved items from the indexing queue and marks a\nset as reserved, starting with items that have\nthe oldest timestamp from the highest priority\nItemStatus.\nThe priority order is as follows: <br />\nERROR\n<br />\nMODIFIED\n<br />\nNEW_ITEM\n<br />\nACCEPTED\n<br />\nReserving items ensures that polling from other threads\ncannot create overlapping sets.\n\nAfter handling the reserved items, the client should put items back\ninto the unreserved state, either by calling\nindex,\nor by calling\npush with\nthe type REQUEUE.\n\nItems automatically become available (unreserved) after 4 hours even if no\nupdate or push method is called."]
                    pub fn poll(
                        &self,
                        request: crate::schemas::PollItemsRequest,
                        name: impl Into<String>,
                    ) -> PollRequestBuilder<A> {
                        PollRequestBuilder {
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
                    #[doc = "Pushes an item onto a queue for later polling and updating."]
                    pub fn push(
                        &self,
                        request: crate::schemas::PushItemRequest,
                        name: impl Into<String>,
                    ) -> PushRequestBuilder<A> {
                        PushRequestBuilder {
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
                    #[doc = "Unreserves all items from a queue, making them all eligible to be\npolled.  This method is useful for resetting the indexing queue\nafter a connector has been restarted."]
                    pub fn unreserve(
                        &self,
                        request: crate::schemas::UnreserveItemsRequest,
                        name: impl Into<String>,
                    ) -> UnreserveRequestBuilder<A> {
                        UnreserveRequestBuilder {
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
                    #[doc = "Creates an upload session for uploading item content. For items smaller\nthan 100 KB, it's easier to embed the content\ninline within\nan index request."]
                    pub fn upload(
                        &self,
                        request: crate::schemas::StartUploadItemRequest,
                        name: impl Into<String>,
                    ) -> UploadRequestBuilder<A> {
                        UploadRequestBuilder {
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
                pub struct DeleteRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    name: String,
                    connector_name: Option<String>,
                    debug_options_enable_debugging: Option<bool>,
                    mode:
                        Option<crate::resources::indexing::datasources::items::params::DeleteMode>,
                    version: Option<crate::bytes::Bytes>,
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
                    #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
                    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
                        self.connector_name = Some(value.into());
                        self
                    }
                    #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = "Required. The RequestMode for this request."]
                    pub fn mode(
                        mut self,
                        value: crate::resources::indexing::datasources::items::params::DeleteMode,
                    ) -> Self {
                        self.mode = Some(value);
                        self
                    }
                    #[doc = "Required. The incremented version of the item to delete from the index.\nThe indexing system stores the version from the datasource as a\nbyte string and compares the Item version in the index\nto the version of the queued Item using lexical ordering.\n<br /><br />\nCloud Search Indexing won't delete any queued item with\na version value that is less than or equal to\nthe version of the currently indexed item.\nThe maximum length for this field is 1024 bytes."]
                    pub fn version(mut self, value: impl Into<Vec<u8>>) -> Self {
                        let v: Vec<u8> = value.into();
                        self.version = Some(v.into());
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
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
                        let req = req.query(&[("connectorName", &self.connector_name)]);
                        let req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
                        let req = req.query(&[("mode", &self.mode)]);
                        let req = req.query(&[("version", &self.version)]);
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
                pub struct DeleteQueueItemsRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::DeleteQueueItemsRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> DeleteQueueItemsRequestBuilder<'a, A> {
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
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:deleteQueueItems");
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
                    connector_name: Option<String>,
                    debug_options_enable_debugging: Option<bool>,
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
                    #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
                    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
                        self.connector_name = Some(value.into());
                        self
                    }
                    #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
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
                    ) -> Result<crate::schemas::Item, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Item, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
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
                        let req = req.query(&[("connectorName", &self.connector_name)]);
                        let req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
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
                pub struct IndexRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::IndexItemRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> IndexRequestBuilder<'a, A> {
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
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":index");
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
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    name: String,
                    brief: Option<bool>,
                    connector_name: Option<String>,
                    debug_options_enable_debugging: Option<bool>,
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
                    #[doc = "When set to true, the indexing system only populates the following fields:\nname,\nversion,\nmetadata.hash,\nstructured_data.hash,\ncontent.hash.\n<br />If this value is false, then all the fields are populated in Item."]
                    pub fn brief(mut self, value: bool) -> Self {
                        self.brief = Some(value);
                        self
                    }
                    #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
                    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
                        self.connector_name = Some(value.into());
                        self
                    }
                    #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = "Maximum number of items to fetch in a request.\nThe max value is 1000 when brief is true.  The max value is 10 if brief\nis false.\n<br />The default value is 10"]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_items<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_items_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_items_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Item> {
                        self.iter_items_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_items_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Item> {
                        self.iter_items_with_fields(Some("*"))
                    }
                    pub fn iter_items_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "items").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "items")
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
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListItemsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListItemsResponse>
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
                    ) -> Result<crate::schemas::ListItemsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListItemsResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("brief", &self.brief)]);
                        let req = req.query(&[("connectorName", &self.connector_name)]);
                        let req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
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
                pub struct PollRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::PollItemsRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> PollRequestBuilder<'a, A> {
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
                    ) -> Result<crate::schemas::PollItemsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::PollItemsResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:poll");
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
                pub struct PushRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::PushItemRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> PushRequestBuilder<'a, A> {
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
                    ) -> Result<crate::schemas::Item, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Item, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":push");
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
                pub struct UnreserveRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::UnreserveItemsRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> UnreserveRequestBuilder<'a, A> {
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
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:unreserve");
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
                pub struct UploadRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    request: crate::schemas::StartUploadItemRequest,
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
                impl<'a, A: ::google_api_auth::GetAccessToken> UploadRequestBuilder<'a, A> {
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
                    ) -> Result<crate::schemas::UploadItemRef, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::UploadItemRef, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":upload");
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
    }
    pub mod media {
        pub mod params {}
        pub struct MediaActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> MediaActions<'a, A> {
            #[doc = "Uploads media for indexing.\n\nThe upload endpoint supports direct and resumable upload protocols and\nis intended for large items that can not be\n[inlined during index requests](https://developers.google.com/cloud-search/docs/reference/rest/v1/indexing.datasources.items#itemcontent).\nTo index large content:\n\n1. Call\n   indexing.datasources.items.upload\n   with the resource name to begin an upload session and retrieve the\n   UploadItemRef.\n1. Call media.upload to upload the content using the same resource name from step 1.\n1. Call indexing.datasources.items.index\n   to index the item. Populate the\n   [ItemContent](/cloud-search/docs/reference/rest/v1/indexing.datasources.items#ItemContent)\n   with the UploadItemRef from step 1.\n\nFor additional information, see\n[Create a content connector using the REST API](https://developers.google.com/cloud-search/docs/guides/content-connector#rest)."]
            pub fn upload(
                &self,
                request: crate::schemas::Media,
                resource_name: impl Into<String>,
            ) -> UploadRequestBuilder<A> {
                UploadRequestBuilder {
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
                    resource_name: resource_name.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct UploadRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::Media,
            resource_name: String,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> UploadRequestBuilder<'a, A> {
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
            fn _simple_upload_path(&self) -> String {
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("upload/v1/media/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            pub fn upload<T, R>(
                mut self,
                content: R,
                mime_type: ::mime::Mime,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                R: ::std::io::Read + ::std::io::Seek + Send + 'static,
            {
                let fields = ::google_field_selector::to_string::<T>();
                self.fields = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                let req = self._request(&self._simple_upload_path())?;
                let req = req.query(&[("uploadType", "multipart")]);
                use crate::multipart::{Part, RelatedMultiPart};
                let mut multipart = RelatedMultiPart::new();
                let request_json = ::serde_json::to_vec(&self.request)?;
                multipart.new_part(Part::new(
                    ::mime::APPLICATION_JSON,
                    Box::new(::std::io::Cursor::new(request_json)),
                ));
                multipart.new_part(Part::new(mime_type, Box::new(content)));
                let req = req.header(
                    ::reqwest::header::CONTENT_TYPE,
                    format!("multipart/related; boundary={}", multipart.boundary()),
                );
                let req = req.body(reqwest::Body::new(multipart.into_reader()));
                Ok(req.send()?.error_for_status()?.json()?)
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
            ) -> Result<crate::schemas::Media, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Media, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/media/");
                {
                    let var_as_str = &self.resource_name;
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
    pub mod operations {
        pub mod params {}
        pub struct OperationsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> OperationsActions<'a, A> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
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
    }
    pub mod query {
        pub mod params {}
        pub struct QueryActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> QueryActions<'a, A> {
            #[doc = "The Cloud Search Query API provides the search method, which returns\nthe most relevant results from a user query.  The results can come from\nG Suite Apps, such as Gmail or Google Drive, or they can come from data\nthat you have indexed from a third party."]
            pub fn search(
                &self,
                request: crate::schemas::SearchRequest,
            ) -> SearchRequestBuilder<A> {
                SearchRequestBuilder {
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
            #[doc = "Provides suggestions for autocompleting the query."]
            pub fn suggest(
                &self,
                request: crate::schemas::SuggestRequest,
            ) -> SuggestRequestBuilder<A> {
                SuggestRequestBuilder {
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
            #[doc = "Actions that can be performed on the sources resource"]
            pub fn sources(&self) -> crate::resources::query::sources::SourcesActions<A> {
                crate::resources::query::sources::SourcesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::SearchRequest,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> SearchRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::SearchResponse, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SearchResponse, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/query/search");
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
        #[derive(Debug, Clone)]
        pub struct SuggestRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::SuggestRequest,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> SuggestRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::SuggestResponse, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SuggestResponse, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/query/suggest");
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
        pub mod sources {
            pub mod params {}
            pub struct SourcesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> SourcesActions<'a, A> {
                #[doc = "Returns list of sources that user can use for Search and Suggest APIs."]
                pub fn list(&self) -> ListRequestBuilder<A> {
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
                        page_token: None,
                        request_options_debug_options_enable_debugging: None,
                        request_options_language_code: None,
                        request_options_search_application_id: None,
                        request_options_time_zone: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                page_token: Option<String>,
                request_options_debug_options_enable_debugging: Option<bool>,
                request_options_language_code: Option<String>,
                request_options_search_application_id: Option<String>,
                request_options_time_zone: Option<String>,
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
                #[doc = "Number of sources to return in the response."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn request_options_debug_options_enable_debugging(
                    mut self,
                    value: bool,
                ) -> Self {
                    self.request_options_debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier.\nFor translations.\n\nWhen specified, the documents in search results are biased towards the\nspecified language.\nSuggest API does not use this parameter. It autocompletes only based on\ncharacters in the query."]
                pub fn request_options_language_code(mut self, value: impl Into<String>) -> Self {
                    self.request_options_language_code = Some(value.into());
                    self
                }
                #[doc = "Id of the application created using SearchApplicationsService."]
                pub fn request_options_search_application_id(
                    mut self,
                    value: impl Into<String>,
                ) -> Self {
                    self.request_options_search_application_id = Some(value.into());
                    self
                }
                #[doc = "Current user's time zone id, such as \"America/Los_Angeles\" or\n\"Australia/Sydney\". These IDs are defined by\n[Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/)\nproject, and currently available in the file\n[timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml)"]
                pub fn request_options_time_zone(mut self, value: impl Into<String>) -> Self {
                    self.request_options_time_zone = Some(value.into());
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
                pub fn iter_sources<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_sources_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_sources_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::QuerySource> {
                    self.iter_sources_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_sources_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::QuerySource> {
                    self.iter_sources_with_fields(Some("*"))
                }
                pub fn iter_sources_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "sources").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "sources")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListQuerySourcesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListQuerySourcesResponse>
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
                ) -> Result<crate::schemas::ListQuerySourcesResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListQuerySourcesResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/query/sources");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[(
                        "requestOptions.debugOptions.enableDebugging",
                        &self.request_options_debug_options_enable_debugging,
                    )]);
                    let req = req.query(&[(
                        "requestOptions.languageCode",
                        &self.request_options_language_code,
                    )]);
                    let req = req.query(&[(
                        "requestOptions.searchApplicationId",
                        &self.request_options_search_application_id,
                    )]);
                    let req =
                        req.query(&[("requestOptions.timeZone", &self.request_options_time_zone)]);
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
    pub mod settings {
        pub mod params {}
        pub struct SettingsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SettingsActions<'a, A> {
            #[doc = "Actions that can be performed on the datasources resource"]
            pub fn datasources(
                &self,
            ) -> crate::resources::settings::datasources::DatasourcesActions<A> {
                crate::resources::settings::datasources::DatasourcesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the searchapplications resource"]
            pub fn searchapplications(
                &self,
            ) -> crate::resources::settings::searchapplications::SearchapplicationsActions<A>
            {
                crate::resources::settings::searchapplications::SearchapplicationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod datasources {
            pub mod params {}
            pub struct DatasourcesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> DatasourcesActions<'a, A> {
                #[doc = "Creates a datasource."]
                pub fn create(
                    &self,
                    request: crate::schemas::DataSource,
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
                    }
                }
                #[doc = "Deletes a datasource."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Gets a datasource."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Lists datasources."]
                pub fn list(&self) -> ListRequestBuilder<A> {
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
                        debug_options_enable_debugging: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a datasource."]
                pub fn update(
                    &self,
                    request: crate::schemas::UpdateDataSourceRequest,
                    name: impl Into<String>,
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
                        name: name.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::DataSource,
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/datasources");
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
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
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
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::DataSource, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::DataSource, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
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
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "Maximum number of datasources to fetch in a request.\nThe max value is 100.\n<br />The default value is 10"]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Starting index of the results."]
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
                pub fn iter_sources<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_sources_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_sources_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::DataSource> {
                    self.iter_sources_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_sources_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::DataSource> {
                    self.iter_sources_with_fields(Some("*"))
                }
                pub fn iter_sources_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "sources").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "sources")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDataSourceResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListDataSourceResponse>
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
                ) -> Result<crate::schemas::ListDataSourceResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListDataSourceResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/datasources");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::UpdateDataSourceRequest,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> UpdateRequestBuilder<'a, A> {
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
        pub mod searchapplications {
            pub mod params {}
            pub struct SearchapplicationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> SearchapplicationsActions<'a, A> {
                #[doc = "Creates a search application."]
                pub fn create(
                    &self,
                    request: crate::schemas::SearchApplication,
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
                    }
                }
                #[doc = "Deletes a search application."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Gets the specified search application."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Lists all search applications."]
                pub fn list(&self) -> ListRequestBuilder<A> {
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
                        debug_options_enable_debugging: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Resets a search application to default settings. This will return an empty\nresponse."]
                pub fn reset(
                    &self,
                    request: crate::schemas::ResetSearchApplicationRequest,
                    name: impl Into<String>,
                ) -> ResetRequestBuilder<A> {
                    ResetRequestBuilder {
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
                #[doc = "Updates a search application."]
                pub fn update(
                    &self,
                    request: crate::schemas::SearchApplication,
                    name: impl Into<String>,
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
                        name: name.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::SearchApplication,
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/searchapplications");
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
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
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
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::SearchApplication, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::SearchApplication, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
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
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "The maximum number of items to return."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The next_page_token value returned from a previous List request, if any.\n<br/> The default value is 10"]
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
                pub fn iter_search_applications<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_search_applications_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_search_applications_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::SearchApplication>
                {
                    self.iter_search_applications_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_search_applications_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::SearchApplication>
                {
                    self.iter_search_applications_with_fields(Some("*"))
                }
                pub fn iter_search_applications_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector =
                            concat!("nextPageToken,", "searchApplications").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "searchApplications")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListSearchApplicationsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListSearchApplicationsResponse>
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
                    crate::schemas::ListSearchApplicationsResponse,
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
                    crate::schemas::ListSearchApplicationsResponse,
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/searchapplications");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            pub struct ResetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::ResetSearchApplicationRequest,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> ResetRequestBuilder<'a, A> {
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":reset");
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
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::SearchApplication,
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
            impl<'a, A: ::google_api_auth::GetAccessToken> UpdateRequestBuilder<'a, A> {
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
                    let req = req.json(&self.request);
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
    }
    pub mod stats {
        pub mod params {}
        pub struct StatsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> StatsActions<'a, A> {
            #[doc = "Gets indexed item statistics aggreggated across all data sources. This\nAPI only returns statistics for previous dates; it doesn't return\nstatistics for the current day."]
            pub fn get_index(&self) -> GetIndexRequestBuilder<A> {
                GetIndexRequestBuilder {
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
                    from_date_day: None,
                    from_date_month: None,
                    from_date_year: None,
                    to_date_day: None,
                    to_date_month: None,
                    to_date_year: None,
                }
            }
            #[doc = "Actions that can be performed on the index resource"]
            pub fn index(&self) -> crate::resources::stats::index::IndexActions<A> {
                crate::resources::stats::index::IndexActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetIndexRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            from_date_day: Option<i32>,
            from_date_month: Option<i32>,
            from_date_year: Option<i32>,
            to_date_day: Option<i32>,
            to_date_month: Option<i32>,
            to_date_year: Option<i32>,
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
        impl<'a, A: ::google_api_auth::GetAccessToken> GetIndexRequestBuilder<'a, A> {
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn from_date_day(mut self, value: i32) -> Self {
                self.from_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn from_date_month(mut self, value: i32) -> Self {
                self.from_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn from_date_year(mut self, value: i32) -> Self {
                self.from_date_year = Some(value);
                self
            }
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn to_date_day(mut self, value: i32) -> Self {
                self.to_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn to_date_month(mut self, value: i32) -> Self {
                self.to_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn to_date_year(mut self, value: i32) -> Self {
                self.to_date_year = Some(value);
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
            ) -> Result<crate::schemas::GetCustomerIndexStatsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetCustomerIndexStatsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/stats/index");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("fromDate.day", &self.from_date_day)]);
                let req = req.query(&[("fromDate.month", &self.from_date_month)]);
                let req = req.query(&[("fromDate.year", &self.from_date_year)]);
                let req = req.query(&[("toDate.day", &self.to_date_day)]);
                let req = req.query(&[("toDate.month", &self.to_date_month)]);
                let req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        pub mod index {
            pub mod params {}
            pub struct IndexActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> IndexActions<'a, A> {
                #[doc = "Actions that can be performed on the datasources resource"]
                pub fn datasources(
                    &self,
                ) -> crate::resources::stats::index::datasources::DatasourcesActions<A>
                {
                    crate::resources::stats::index::datasources::DatasourcesActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            pub mod datasources {
                pub mod params {}
                pub struct DatasourcesActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> DatasourcesActions<'a, A> {
                    #[doc = "Gets indexed item statistics for a single data source."]
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
                            from_date_day: None,
                            from_date_month: None,
                            from_date_year: None,
                            to_date_day: None,
                            to_date_month: None,
                            to_date_year: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    name: String,
                    from_date_day: Option<i32>,
                    from_date_month: Option<i32>,
                    from_date_year: Option<i32>,
                    to_date_day: Option<i32>,
                    to_date_month: Option<i32>,
                    to_date_year: Option<i32>,
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
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn from_date_day(mut self, value: i32) -> Self {
                        self.from_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn from_date_month(mut self, value: i32) -> Self {
                        self.from_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn from_date_year(mut self, value: i32) -> Self {
                        self.from_date_year = Some(value);
                        self
                    }
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn to_date_day(mut self, value: i32) -> Self {
                        self.to_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn to_date_month(mut self, value: i32) -> Self {
                        self.to_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn to_date_year(mut self, value: i32) -> Self {
                        self.to_date_year = Some(value);
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
                        crate::schemas::GetDataSourceIndexStatsResponse,
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
                        crate::schemas::GetDataSourceIndexStatsResponse,
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/stats/index/");
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
                        let req = req.query(&[("fromDate.day", &self.from_date_day)]);
                        let req = req.query(&[("fromDate.month", &self.from_date_month)]);
                        let req = req.query(&[("fromDate.year", &self.from_date_year)]);
                        let req = req.query(&[("toDate.day", &self.to_date_day)]);
                        let req = req.query(&[("toDate.month", &self.to_date_month)]);
                        let req = req.query(&[("toDate.year", &self.to_date_year)]);
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
