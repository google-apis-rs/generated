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
        #[doc = "Indicates the operator name required in the query in order to isolate the\nboolean property. For example, if operatorName is *closed* and the\nproperty's name is *isClosed*, then queries like\n*closed:&lt;value&gt;* will show results only where the value of the\nproperty named *isClosed* matches *&lt;value&gt;*. By contrast, a\nsearch that uses the same *&lt;value&gt;* without an operator will return\nall items where *&lt;value&gt;* matches the value of any\nString properties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for BooleanOperatorOptions {
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
    pub struct BooleanPropertyOptions {
        #[doc = "If set, describes how the boolean should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::BooleanOperatorOptions>,
    }
    impl ::field_selector::FieldSelector for BooleanPropertyOptions {
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
    pub struct CheckAccessResponse {
        #[doc = "Returns true if principal has access.  Returns false otherwise."]
        #[serde(rename = "hasAccess", default)]
        pub has_access: Option<bool>,
    }
    impl ::field_selector::FieldSelector for CheckAccessResponse {
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
    pub enum CompositeFilterLogicOperator {
        #[doc = "Logical operators, which can only be applied to sub filters."]
        And,
        Or,
        #[doc = "NOT can only be applied on a single sub filter."]
        Not,
    }
    impl CompositeFilterLogicOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                CompositeFilterLogicOperator::And => "AND",
                CompositeFilterLogicOperator::Or => "OR",
                CompositeFilterLogicOperator::Not => "NOT",
            }
        }
    }
    impl ::std::fmt::Display for CompositeFilterLogicOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompositeFilterLogicOperator {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompositeFilterLogicOperator {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => CompositeFilterLogicOperator::And,
                "OR" => CompositeFilterLogicOperator::Or,
                "NOT" => CompositeFilterLogicOperator::Not,
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
    pub struct CompositeFilter {
        #[doc = "The logic operator of the sub filter."]
        #[serde(rename = "logicOperator", default)]
        pub logic_operator: Option<crate::schemas::CompositeFilterLogicOperator>,
        #[doc = "Sub filters."]
        #[serde(rename = "subFilters", default)]
        pub sub_filters: Option<Vec<crate::schemas::Filter>>,
    }
    impl ::field_selector::FieldSelector for CompositeFilter {
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
    pub struct CustomerIndexStats {
        #[doc = "Date for which statistics were calculated."]
        #[serde(rename = "date", default)]
        pub date: Option<crate::schemas::Date>,
        #[doc = "Number of items aggregrated by status code."]
        #[serde(rename = "itemCountByStatus", default)]
        pub item_count_by_status: Option<Vec<crate::schemas::ItemCountByStatus>>,
    }
    impl ::field_selector::FieldSelector for CustomerIndexStats {
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
    pub struct DataSource {
        #[doc = "If true, Indexing API rejects any modification calls to this datasource\nsuch as create, update, and delete.\nDisabling this does not imply halting process of previously\naccepted data."]
        #[serde(rename = "disableModifications", default)]
        pub disable_modifications: Option<bool>,
        #[doc = "Disable serving any search or assist results."]
        #[serde(rename = "disableServing", default)]
        pub disable_serving: Option<bool>,
        #[doc = "Required. Display name of the datasource\nThe maximum length is 300 characters."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "List of service accounts that have indexing access."]
        #[serde(rename = "indexingServiceAccounts", default)]
        pub indexing_service_accounts: Option<Vec<String>>,
        #[doc = "This field restricts visibility to items at the datasource level. Items\nwithin the datasource are restricted to the union of users and groups\nincluded in this field. Note that, this does not ensure access to a\nspecific item, as users need to have ACL permissions on the contained\nitems. This ensures a high level access on the entire datasource, and\nthat the individual items are not shared outside this visibility."]
        #[serde(rename = "itemsVisibility", default)]
        pub items_visibility: Option<Vec<crate::schemas::GsuitePrincipal>>,
        #[doc = "Name of the datasource resource.\nFormat: datasources/{source_id}.\n<br />The name is ignored when creating a datasource."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this\nschema."]
        #[serde(rename = "operationIds", default)]
        pub operation_ids: Option<Vec<String>>,
        #[doc = "A short name or alias for the source.  This value will be used to match the\n'source' operator. For example, if the short name is *&lt;value&gt;* then\nqueries like *source:&lt;value&gt;* will only return results for this\nsource. The value must be unique across all datasources. The value must\nonly contain alphanumeric characters (a-zA-Z0-9). The value cannot start\nwith 'google' and cannot be one of the following: mail, gmail, docs, drive,\ngroups, sites, calendar, hangouts, gplus, keep, people, teams.\nIts maximum length is 32 characters."]
        #[serde(rename = "shortName", default)]
        pub short_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for DataSource {
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
    pub struct DataSourceIndexStats {
        #[doc = "Date for which index stats were calculated. If the date of request is not\nthe current date then stats calculated on the next day are returned. Stats\nare calculated close to mid night in this case. If date of request is\ncurrent date, then real time stats are returned."]
        #[serde(rename = "date", default)]
        pub date: Option<crate::schemas::Date>,
        #[doc = "Number of items aggregrated by status code."]
        #[serde(rename = "itemCountByStatus", default)]
        pub item_count_by_status: Option<Vec<crate::schemas::ItemCountByStatus>>,
    }
    impl ::field_selector::FieldSelector for DataSourceIndexStats {
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
    pub struct DataSourceRestriction {
        #[doc = "Filter options restricting the results. If multiple filters\nare present, they are grouped by object type before joining.\nFilters with the same object type are joined conjunctively, then\nthe resulting expressions are joined disjunctively.\n\nThe maximum number of elements is 20.\n\nNOTE: Suggest API supports only few filters at the moment:\n  \"objecttype\", \"type\" and \"mimetype\".\nFor now, schema specific filters cannot be used to filter suggestions."]
        #[serde(rename = "filterOptions", default)]
        pub filter_options: Option<Vec<crate::schemas::FilterOptions>>,
        #[doc = "The source of restriction."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for DataSourceRestriction {
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
    pub struct Date {
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
        #[serde(rename = "day", default)]
        pub day: Option<i32>,
        #[doc = "Month of date. Must be from 1 to 12."]
        #[serde(rename = "month", default)]
        pub month: Option<i32>,
        #[doc = "Year of date. Must be from 1 to 9999."]
        #[serde(rename = "year", default)]
        pub year: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Date {
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
    pub struct DateOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\ndate property using the greater-than operator. For example, if\ngreaterThanOperatorName is *closedafter* and the property's name is\n*closeDate*, then queries like *closedafter:&lt;value&gt;* will\nshow results only where the value of the property named *closeDate* is\nlater than *&lt;value&gt;*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ndate property using the less-than operator. For example, if\nlessThanOperatorName is *closedbefore* and the property's name is\n*closeDate*, then queries like *closedbefore:&lt;value&gt;* will\nshow results only where the value of the property named *closeDate* is\nearlier than *&lt;value&gt;*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: Option<String>,
        #[doc = "Indicates the actual string required in the query in order to isolate the\ndate property. For example, suppose an issue tracking schema object\nhas a property named *closeDate* that specifies an operator with an\noperatorName of *closedon*. For searches on that data, queries like\n*closedon:&lt;value&gt;* will show results only where the value of the\n*closeDate* property matches *&lt;value&gt;*. By contrast, a\nsearch that uses the same *&lt;value&gt;* without an operator will return\nall items where *&lt;value&gt;* matches the value of any String\nproperties or text within the content field for the indexed datasource.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for DateOperatorOptions {
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
    pub struct DatePropertyOptions {
        #[doc = "If set, describes how the date should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::DateOperatorOptions>,
    }
    impl ::field_selector::FieldSelector for DatePropertyOptions {
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
    pub struct DateValues {
        #[serde(rename = "values", default)]
        pub values: Option<Vec<crate::schemas::Date>>,
    }
    impl ::field_selector::FieldSelector for DateValues {
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
    pub struct DebugOptions {
        #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
        #[serde(rename = "enableDebugging", default)]
        pub enable_debugging: Option<bool>,
    }
    impl ::field_selector::FieldSelector for DebugOptions {
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
    pub struct DeleteQueueItemsRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[doc = "Name of a queue to delete items from."]
        #[serde(rename = "queue", default)]
        pub queue: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeleteQueueItemsRequest {
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
    pub struct DisplayedProperty {
        #[doc = "The name of the top-level property as defined in a property definition\nfor the object. If the name is not a defined property in the schema, an\nerror will be given when attempting to update the schema."]
        #[serde(rename = "propertyName", default)]
        pub property_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for DisplayedProperty {
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
    pub struct DoubleOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to use the\ndouble property in sorting or as a facet.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for DoubleOperatorOptions {
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
    pub struct DoublePropertyOptions {
        #[doc = "If set, describes how the double should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::DoubleOperatorOptions>,
    }
    impl ::field_selector::FieldSelector for DoublePropertyOptions {
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
    pub struct DoubleValues {
        #[serde(rename = "values", default)]
        pub values: Option<Vec<f64>>,
    }
    impl ::field_selector::FieldSelector for DoubleValues {
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
    pub enum DriveFollowUpRestrictType {
        Unspecified,
        FollowupSuggestions,
        FollowupActionItems,
    }
    impl DriveFollowUpRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveFollowUpRestrictType::Unspecified => "UNSPECIFIED",
                DriveFollowUpRestrictType::FollowupSuggestions => "FOLLOWUP_SUGGESTIONS",
                DriveFollowUpRestrictType::FollowupActionItems => "FOLLOWUP_ACTION_ITEMS",
            }
        }
    }
    impl ::std::fmt::Display for DriveFollowUpRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveFollowUpRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveFollowUpRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => DriveFollowUpRestrictType::Unspecified,
                "FOLLOWUP_SUGGESTIONS" => DriveFollowUpRestrictType::FollowupSuggestions,
                "FOLLOWUP_ACTION_ITEMS" => DriveFollowUpRestrictType::FollowupActionItems,
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
    pub struct DriveFollowUpRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::DriveFollowUpRestrictType>,
    }
    impl ::field_selector::FieldSelector for DriveFollowUpRestrict {
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
    pub enum DriveLocationRestrictType {
        Unspecified,
        Trashed,
        Starred,
    }
    impl DriveLocationRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveLocationRestrictType::Unspecified => "UNSPECIFIED",
                DriveLocationRestrictType::Trashed => "TRASHED",
                DriveLocationRestrictType::Starred => "STARRED",
            }
        }
    }
    impl ::std::fmt::Display for DriveLocationRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveLocationRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveLocationRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => DriveLocationRestrictType::Unspecified,
                "TRASHED" => DriveLocationRestrictType::Trashed,
                "STARRED" => DriveLocationRestrictType::Starred,
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
    pub struct DriveLocationRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::DriveLocationRestrictType>,
    }
    impl ::field_selector::FieldSelector for DriveLocationRestrict {
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
    pub enum DriveMimeTypeRestrictType {
        Unspecified,
        Pdf,
        Document,
        Presentation,
        Spreadsheet,
        Form,
        Drawing,
        Script,
        Map,
        Image,
        Audio,
        Video,
        Folder,
        Archive,
        Site,
    }
    impl DriveMimeTypeRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveMimeTypeRestrictType::Unspecified => "UNSPECIFIED",
                DriveMimeTypeRestrictType::Pdf => "PDF",
                DriveMimeTypeRestrictType::Document => "DOCUMENT",
                DriveMimeTypeRestrictType::Presentation => "PRESENTATION",
                DriveMimeTypeRestrictType::Spreadsheet => "SPREADSHEET",
                DriveMimeTypeRestrictType::Form => "FORM",
                DriveMimeTypeRestrictType::Drawing => "DRAWING",
                DriveMimeTypeRestrictType::Script => "SCRIPT",
                DriveMimeTypeRestrictType::Map => "MAP",
                DriveMimeTypeRestrictType::Image => "IMAGE",
                DriveMimeTypeRestrictType::Audio => "AUDIO",
                DriveMimeTypeRestrictType::Video => "VIDEO",
                DriveMimeTypeRestrictType::Folder => "FOLDER",
                DriveMimeTypeRestrictType::Archive => "ARCHIVE",
                DriveMimeTypeRestrictType::Site => "SITE",
            }
        }
    }
    impl ::std::fmt::Display for DriveMimeTypeRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveMimeTypeRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveMimeTypeRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => DriveMimeTypeRestrictType::Unspecified,
                "PDF" => DriveMimeTypeRestrictType::Pdf,
                "DOCUMENT" => DriveMimeTypeRestrictType::Document,
                "PRESENTATION" => DriveMimeTypeRestrictType::Presentation,
                "SPREADSHEET" => DriveMimeTypeRestrictType::Spreadsheet,
                "FORM" => DriveMimeTypeRestrictType::Form,
                "DRAWING" => DriveMimeTypeRestrictType::Drawing,
                "SCRIPT" => DriveMimeTypeRestrictType::Script,
                "MAP" => DriveMimeTypeRestrictType::Map,
                "IMAGE" => DriveMimeTypeRestrictType::Image,
                "AUDIO" => DriveMimeTypeRestrictType::Audio,
                "VIDEO" => DriveMimeTypeRestrictType::Video,
                "FOLDER" => DriveMimeTypeRestrictType::Folder,
                "ARCHIVE" => DriveMimeTypeRestrictType::Archive,
                "SITE" => DriveMimeTypeRestrictType::Site,
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
    pub struct DriveMimeTypeRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::DriveMimeTypeRestrictType>,
    }
    impl ::field_selector::FieldSelector for DriveMimeTypeRestrict {
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
    pub enum DriveTimeSpanRestrictType {
        Unspecified,
        Today,
        Yesterday,
        Last7Days,
        #[doc = "Not Enabled"]
        Last30Days,
        #[doc = "Not Enabled"]
        Last90Days,
    }
    impl DriveTimeSpanRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveTimeSpanRestrictType::Unspecified => "UNSPECIFIED",
                DriveTimeSpanRestrictType::Today => "TODAY",
                DriveTimeSpanRestrictType::Yesterday => "YESTERDAY",
                DriveTimeSpanRestrictType::Last7Days => "LAST_7_DAYS",
                DriveTimeSpanRestrictType::Last30Days => "LAST_30_DAYS",
                DriveTimeSpanRestrictType::Last90Days => "LAST_90_DAYS",
            }
        }
    }
    impl ::std::fmt::Display for DriveTimeSpanRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveTimeSpanRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveTimeSpanRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => DriveTimeSpanRestrictType::Unspecified,
                "TODAY" => DriveTimeSpanRestrictType::Today,
                "YESTERDAY" => DriveTimeSpanRestrictType::Yesterday,
                "LAST_7_DAYS" => DriveTimeSpanRestrictType::Last7Days,
                "LAST_30_DAYS" => DriveTimeSpanRestrictType::Last30Days,
                "LAST_90_DAYS" => DriveTimeSpanRestrictType::Last90Days,
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
    pub struct DriveTimeSpanRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::DriveTimeSpanRestrictType>,
    }
    impl ::field_selector::FieldSelector for DriveTimeSpanRestrict {
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
    pub struct EmailAddress {
        #[doc = "The email address."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: Option<String>,
    }
    impl ::field_selector::FieldSelector for EmailAddress {
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
    pub struct EnumOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\nenum property. For example, if operatorName is *priority* and the\nproperty's name is *priorityVal*, then queries like\n*priority:&lt;value&gt;* will show results only where the value of the\nproperty named *priorityVal* matches *&lt;value&gt;*. By contrast, a\nsearch that uses the same *&lt;value&gt;* without an operator will return\nall items where *&lt;value&gt;* matches the value of any String\nproperties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for EnumOperatorOptions {
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
    pub enum EnumPropertyOptionsOrderedRanking {
        #[doc = "There is no ranking order for the property. Results will not be adjusted\nby this property's value."]
        NoOrder,
        #[doc = "This property is ranked in ascending order. Lower values indicate lower\nranking."]
        Ascending,
        #[doc = "This property is ranked in descending order. Lower values indicate\nhigher ranking."]
        Descending,
    }
    impl EnumPropertyOptionsOrderedRanking {
        pub fn as_str(self) -> &'static str {
            match self {
                EnumPropertyOptionsOrderedRanking::NoOrder => "NO_ORDER",
                EnumPropertyOptionsOrderedRanking::Ascending => "ASCENDING",
                EnumPropertyOptionsOrderedRanking::Descending => "DESCENDING",
            }
        }
    }
    impl ::std::fmt::Display for EnumPropertyOptionsOrderedRanking {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnumPropertyOptionsOrderedRanking {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnumPropertyOptionsOrderedRanking {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NO_ORDER" => EnumPropertyOptionsOrderedRanking::NoOrder,
                "ASCENDING" => EnumPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => EnumPropertyOptionsOrderedRanking::Descending,
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
    pub struct EnumPropertyOptions {
        #[doc = "If set, describes how the enum should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::EnumOperatorOptions>,
        #[doc = "Used to specify the ordered ranking for the enumeration that determines how\nthe integer values provided in the possible EnumValuePairs are used to rank\nresults. If specified, integer values must be provided for all possible\nEnumValuePair values given for this property. Can only be used if\nisRepeatable\nis false."]
        #[serde(rename = "orderedRanking", default)]
        pub ordered_ranking: Option<crate::schemas::EnumPropertyOptionsOrderedRanking>,
        #[doc = "The list of possible values for the enumeration property. All\nEnumValuePairs must provide a string value. If you specify an integer value\nfor one EnumValuePair, then all possible EnumValuePairs must provide an\ninteger value. Both the string value and integer value must be unique over\nall possible values. Once set, possible values cannot be removed or\nmodified. If you supply an ordered ranking and think you might insert\nadditional enum values in the future, leave gaps in the initial integer\nvalues to allow adding a value in between previously registered values.\nThe maximum number of elements is 100."]
        #[serde(rename = "possibleValues", default)]
        pub possible_values: Option<Vec<crate::schemas::EnumValuePair>>,
    }
    impl ::field_selector::FieldSelector for EnumPropertyOptions {
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
    pub struct EnumValuePair {
        #[doc = "The integer value of the EnumValuePair which must be non-negative.\nOptional."]
        #[serde(rename = "integerValue", default)]
        pub integer_value: Option<i32>,
        #[doc = "The string value of the EnumValuePair.\nThe maximum length is 32 characters."]
        #[serde(rename = "stringValue", default)]
        pub string_value: Option<String>,
    }
    impl ::field_selector::FieldSelector for EnumValuePair {
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
    pub struct EnumValues {
        #[doc = "The maximum allowable length for string values is 32 characters."]
        #[serde(rename = "values", default)]
        pub values: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for EnumValues {
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
    pub struct ErrorInfo {
        #[serde(rename = "errorMessages", default)]
        pub error_messages: Option<Vec<crate::schemas::ErrorMessage>>,
    }
    impl ::field_selector::FieldSelector for ErrorInfo {
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
    pub struct ErrorMessage {
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for ErrorMessage {
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
    pub struct FacetBucket {
        #[doc = "Number of results that match the bucket value. Counts are only returned\nfor searches when count accuracy is ensured. Can be empty."]
        #[serde(rename = "count", default)]
        pub count: Option<i32>,
        #[doc = "Percent of results that match the bucket value. This value is between\n(0-100]. Percentages are returned for all searches, but are an estimate.\nBecause percentages are always returned, you should render percentages\ninstead of counts."]
        #[serde(rename = "percentage", default)]
        pub percentage: Option<i32>,
        #[serde(rename = "value", default)]
        pub value: Option<crate::schemas::Value>,
    }
    impl ::field_selector::FieldSelector for FacetBucket {
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
    pub struct FacetOptions {
        #[doc = "Maximum number of facet buckets that should be returned for this facet.\nDefaults to 10.\nMaximum value is 100."]
        #[serde(rename = "numFacetBuckets", default)]
        pub num_facet_buckets: Option<i32>,
        #[doc = "If object_type is set, only those objects of that type will be used to\ncompute facets. If empty, then all objects will be used to compute facets."]
        #[serde(rename = "objectType", default)]
        pub object_type: Option<String>,
        #[doc = "Name of the operator chosen for faceting. @see\ncloudsearch.SchemaPropertyOptions"]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
        #[doc = "Source name to facet on. Format: datasources/{source_id}\nIf empty, all data sources will be used."]
        #[serde(rename = "sourceName", default)]
        pub source_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for FacetOptions {
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
    pub struct FacetResult {
        #[doc = "FacetBuckets for values in response containing at least a single result."]
        #[serde(rename = "buckets", default)]
        pub buckets: Option<Vec<crate::schemas::FacetBucket>>,
        #[doc = "Object type for which facet results are returned. Can be empty."]
        #[serde(rename = "objectType", default)]
        pub object_type: Option<String>,
        #[doc = "Name of the operator chosen for faceting. @see\ncloudsearch.SchemaPropertyOptions"]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
        #[doc = "Source name for which facet results are returned. Will not be empty."]
        #[serde(rename = "sourceName", default)]
        pub source_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for FacetResult {
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
    pub struct FieldViolation {
        #[doc = "Description of the error."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Path of field with violation."]
        #[serde(rename = "field", default)]
        pub field: Option<String>,
    }
    impl ::field_selector::FieldSelector for FieldViolation {
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
    pub struct Filter {
        #[serde(rename = "compositeFilter", default)]
        pub composite_filter: Option<crate::schemas::CompositeFilter>,
        #[serde(rename = "valueFilter", default)]
        pub value_filter: Option<crate::schemas::ValueFilter>,
    }
    impl ::field_selector::FieldSelector for Filter {
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
    pub struct FilterOptions {
        #[doc = "Generic filter to restrict the search, such as `lang:en`, `site:xyz`."]
        #[serde(rename = "filter", default)]
        pub filter: Option<crate::schemas::Filter>,
        #[doc = "If object_type is set, only objects of that type are returned. This should\ncorrespond to the name of the object that was registered within the\ndefinition of schema. The maximum length is 256 characters."]
        #[serde(rename = "objectType", default)]
        pub object_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for FilterOptions {
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
    pub struct FreshnessOptions {
        #[doc = "The duration after which an object should be considered\nstale. The default value is 180 days (in seconds)."]
        #[serde(rename = "freshnessDuration", default)]
        pub freshness_duration: Option<String>,
        #[doc = "This property indicates the freshness level of the object in the index.\nIf set, this property must be a top-level property within the\nproperty definitions\nand it must be a\ntimestamp type\nor\ndate type.\nOtherwise, the Indexing API uses\nupdateTime\nas the freshness indicator.\nThe maximum length is 256 characters.\n\nWhen a property is used to calculate fresheness, the value defaults\nto 2 years from the current time."]
        #[serde(rename = "freshnessProperty", default)]
        pub freshness_property: Option<String>,
    }
    impl ::field_selector::FieldSelector for FreshnessOptions {
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
    pub struct GetCustomerIndexStatsResponse {
        #[doc = "Summary of indexed item counts, one for each day in the requested range."]
        #[serde(rename = "stats", default)]
        pub stats: Option<Vec<crate::schemas::CustomerIndexStats>>,
    }
    impl ::field_selector::FieldSelector for GetCustomerIndexStatsResponse {
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
    pub struct GetDataSourceIndexStatsResponse {
        #[doc = "Summary of indexed item counts, one for each day in the requested range."]
        #[serde(rename = "stats", default)]
        pub stats: Option<Vec<crate::schemas::DataSourceIndexStats>>,
    }
    impl ::field_selector::FieldSelector for GetDataSourceIndexStatsResponse {
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
    pub enum GmailActionRestrictType {
        Unspecified,
        #[doc = "is:unread"]
        Unread,
        #[doc = "is:read"]
        Read,
        #[doc = "label:^io_re"]
        RepliedTo,
        #[doc = "label:mute"]
        Muted,
    }
    impl GmailActionRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailActionRestrictType::Unspecified => "UNSPECIFIED",
                GmailActionRestrictType::Unread => "UNREAD",
                GmailActionRestrictType::Read => "READ",
                GmailActionRestrictType::RepliedTo => "REPLIED_TO",
                GmailActionRestrictType::Muted => "MUTED",
            }
        }
    }
    impl ::std::fmt::Display for GmailActionRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailActionRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailActionRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => GmailActionRestrictType::Unspecified,
                "UNREAD" => GmailActionRestrictType::Unread,
                "READ" => GmailActionRestrictType::Read,
                "REPLIED_TO" => GmailActionRestrictType::RepliedTo,
                "MUTED" => GmailActionRestrictType::Muted,
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
    pub struct GmailActionRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::GmailActionRestrictType>,
    }
    impl ::field_selector::FieldSelector for GmailActionRestrict {
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
    pub enum GmailAttachmentRestrictType {
        Unspecified,
        #[doc = "has:attachment"]
        HasAttachment,
        #[doc = "has photos (changes to filename:(jpg OR jpeg OR png)  when typed)"]
        HasPhoto,
        #[doc = "has:drive"]
        HasDrive,
        #[doc = "has:document"]
        HasDocument,
        #[doc = "has:spreadsheet"]
        HasSpreadsheet,
        #[doc = "has:presentation"]
        HasPresentation,
        #[doc = "has:youtube"]
        HasYoutube,
        #[doc = "filename:pdf"]
        HasPdf,
    }
    impl GmailAttachmentRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailAttachmentRestrictType::Unspecified => "UNSPECIFIED",
                GmailAttachmentRestrictType::HasAttachment => "HAS_ATTACHMENT",
                GmailAttachmentRestrictType::HasPhoto => "HAS_PHOTO",
                GmailAttachmentRestrictType::HasDrive => "HAS_DRIVE",
                GmailAttachmentRestrictType::HasDocument => "HAS_DOCUMENT",
                GmailAttachmentRestrictType::HasSpreadsheet => "HAS_SPREADSHEET",
                GmailAttachmentRestrictType::HasPresentation => "HAS_PRESENTATION",
                GmailAttachmentRestrictType::HasYoutube => "HAS_YOUTUBE",
                GmailAttachmentRestrictType::HasPdf => "HAS_PDF",
            }
        }
    }
    impl ::std::fmt::Display for GmailAttachmentRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailAttachmentRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailAttachmentRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => GmailAttachmentRestrictType::Unspecified,
                "HAS_ATTACHMENT" => GmailAttachmentRestrictType::HasAttachment,
                "HAS_PHOTO" => GmailAttachmentRestrictType::HasPhoto,
                "HAS_DRIVE" => GmailAttachmentRestrictType::HasDrive,
                "HAS_DOCUMENT" => GmailAttachmentRestrictType::HasDocument,
                "HAS_SPREADSHEET" => GmailAttachmentRestrictType::HasSpreadsheet,
                "HAS_PRESENTATION" => GmailAttachmentRestrictType::HasPresentation,
                "HAS_YOUTUBE" => GmailAttachmentRestrictType::HasYoutube,
                "HAS_PDF" => GmailAttachmentRestrictType::HasPdf,
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
    pub struct GmailAttachmentRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::GmailAttachmentRestrictType>,
    }
    impl ::field_selector::FieldSelector for GmailAttachmentRestrict {
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
    pub enum GmailFolderRestrictType {
        Unspecified,
        #[doc = "in:sent"]
        InSent,
        #[doc = "in:draft"]
        InDraft,
        #[doc = "label:chats"]
        Chats,
        #[doc = "in:trash"]
        InTrash,
        #[doc = "label:<user generated>"]
        UserGeneratedLabel,
    }
    impl GmailFolderRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailFolderRestrictType::Unspecified => "UNSPECIFIED",
                GmailFolderRestrictType::InSent => "IN_SENT",
                GmailFolderRestrictType::InDraft => "IN_DRAFT",
                GmailFolderRestrictType::Chats => "CHATS",
                GmailFolderRestrictType::InTrash => "IN_TRASH",
                GmailFolderRestrictType::UserGeneratedLabel => "USER_GENERATED_LABEL",
            }
        }
    }
    impl ::std::fmt::Display for GmailFolderRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailFolderRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailFolderRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => GmailFolderRestrictType::Unspecified,
                "IN_SENT" => GmailFolderRestrictType::InSent,
                "IN_DRAFT" => GmailFolderRestrictType::InDraft,
                "CHATS" => GmailFolderRestrictType::Chats,
                "IN_TRASH" => GmailFolderRestrictType::InTrash,
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
    #[derive(
        Debug,
        Clone,
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
        pub r#type: Option<crate::schemas::GmailFolderRestrictType>,
    }
    impl ::field_selector::FieldSelector for GmailFolderRestrict {
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
    pub enum GmailIntelligentRestrictType {
        Unspecified,
        #[doc = "category:social"]
        Social,
        #[doc = "category:updates"]
        Updates,
        #[doc = "category:forums"]
        Forums,
        #[doc = "category:promotions"]
        Promotions,
        #[doc = "is:important"]
        Important,
        #[doc = "label:^cob_sm_flightreservation"]
        FlightReservation,
        #[doc = "label:^cob_sm_lodgingreservation"]
        LodgingReservation,
        #[doc = "label:^cob_sm_rentalcarreservation"]
        CarReservation,
        #[doc = "label:^cob_sm_busreservation"]
        BusReservation,
    }
    impl GmailIntelligentRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailIntelligentRestrictType::Unspecified => "UNSPECIFIED",
                GmailIntelligentRestrictType::Social => "SOCIAL",
                GmailIntelligentRestrictType::Updates => "UPDATES",
                GmailIntelligentRestrictType::Forums => "FORUMS",
                GmailIntelligentRestrictType::Promotions => "PROMOTIONS",
                GmailIntelligentRestrictType::Important => "IMPORTANT",
                GmailIntelligentRestrictType::FlightReservation => "FLIGHT_RESERVATION",
                GmailIntelligentRestrictType::LodgingReservation => "LODGING_RESERVATION",
                GmailIntelligentRestrictType::CarReservation => "CAR_RESERVATION",
                GmailIntelligentRestrictType::BusReservation => "BUS_RESERVATION",
            }
        }
    }
    impl ::std::fmt::Display for GmailIntelligentRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailIntelligentRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailIntelligentRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => GmailIntelligentRestrictType::Unspecified,
                "SOCIAL" => GmailIntelligentRestrictType::Social,
                "UPDATES" => GmailIntelligentRestrictType::Updates,
                "FORUMS" => GmailIntelligentRestrictType::Forums,
                "PROMOTIONS" => GmailIntelligentRestrictType::Promotions,
                "IMPORTANT" => GmailIntelligentRestrictType::Important,
                "FLIGHT_RESERVATION" => GmailIntelligentRestrictType::FlightReservation,
                "LODGING_RESERVATION" => GmailIntelligentRestrictType::LodgingReservation,
                "CAR_RESERVATION" => GmailIntelligentRestrictType::CarReservation,
                "BUS_RESERVATION" => GmailIntelligentRestrictType::BusReservation,
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
    pub struct GmailIntelligentRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::GmailIntelligentRestrictType>,
    }
    impl ::field_selector::FieldSelector for GmailIntelligentRestrict {
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
    pub enum GmailTimeRestrictType {
        Unspecified,
        #[doc = "newer_than:7d"]
        FromThisWeek,
        #[doc = "older_than:1y"]
        OlderThanOneYear,
        #[doc = "newer_than:1d"]
        FromToday,
        #[doc = "newer_than:2d older_than:1d"]
        FromYesterday,
        #[doc = "newer_than:30d"]
        FromThisMonth,
        #[doc = "This will read as something like \"From November\" and will have operator\nbefore:X after:Y"]
        FromCertainMonth,
        #[doc = "older_than:1d"]
        OlderThanToday,
        #[doc = "older_than:2d"]
        OlderThanYesterday,
        #[doc = "older_than:7d"]
        OlderThanAWeek,
        #[doc = "older_than:30d"]
        OlderThanAMonth,
    }
    impl GmailTimeRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                GmailTimeRestrictType::Unspecified => "UNSPECIFIED",
                GmailTimeRestrictType::FromThisWeek => "FROM_THIS_WEEK",
                GmailTimeRestrictType::OlderThanOneYear => "OLDER_THAN_ONE_YEAR",
                GmailTimeRestrictType::FromToday => "FROM_TODAY",
                GmailTimeRestrictType::FromYesterday => "FROM_YESTERDAY",
                GmailTimeRestrictType::FromThisMonth => "FROM_THIS_MONTH",
                GmailTimeRestrictType::FromCertainMonth => "FROM_CERTAIN_MONTH",
                GmailTimeRestrictType::OlderThanToday => "OLDER_THAN_TODAY",
                GmailTimeRestrictType::OlderThanYesterday => "OLDER_THAN_YESTERDAY",
                GmailTimeRestrictType::OlderThanAWeek => "OLDER_THAN_A_WEEK",
                GmailTimeRestrictType::OlderThanAMonth => "OLDER_THAN_A_MONTH",
            }
        }
    }
    impl ::std::fmt::Display for GmailTimeRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GmailTimeRestrictType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GmailTimeRestrictType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => GmailTimeRestrictType::Unspecified,
                "FROM_THIS_WEEK" => GmailTimeRestrictType::FromThisWeek,
                "OLDER_THAN_ONE_YEAR" => GmailTimeRestrictType::OlderThanOneYear,
                "FROM_TODAY" => GmailTimeRestrictType::FromToday,
                "FROM_YESTERDAY" => GmailTimeRestrictType::FromYesterday,
                "FROM_THIS_MONTH" => GmailTimeRestrictType::FromThisMonth,
                "FROM_CERTAIN_MONTH" => GmailTimeRestrictType::FromCertainMonth,
                "OLDER_THAN_TODAY" => GmailTimeRestrictType::OlderThanToday,
                "OLDER_THAN_YESTERDAY" => GmailTimeRestrictType::OlderThanYesterday,
                "OLDER_THAN_A_WEEK" => GmailTimeRestrictType::OlderThanAWeek,
                "OLDER_THAN_A_MONTH" => GmailTimeRestrictType::OlderThanAMonth,
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
    pub struct GmailTimeRestrict {
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::GmailTimeRestrictType>,
    }
    impl ::field_selector::FieldSelector for GmailTimeRestrict {
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
    pub struct GsuitePrincipal {
        #[doc = "This principal represents all users of the G Suite domain of the\ncustomer."]
        #[serde(rename = "gsuiteDomain", default)]
        pub gsuite_domain: Option<bool>,
        #[doc = "This principal references a G Suite group account"]
        #[serde(rename = "gsuiteGroupEmail", default)]
        pub gsuite_group_email: Option<String>,
        #[doc = "This principal references a G Suite user account"]
        #[serde(rename = "gsuiteUserEmail", default)]
        pub gsuite_user_email: Option<String>,
    }
    impl ::field_selector::FieldSelector for GsuitePrincipal {
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
    pub struct HtmlOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\nhtml property. For example, if operatorName is *subject* and the\nproperty's name is *subjectLine*, then queries like\n*subject:&lt;value&gt;* will show results only where the value of the\nproperty named *subjectLine* matches *&lt;value&gt;*. By contrast, a\nsearch that uses the same *&lt;value&gt;* without an operator will return\nall items where *&lt;value&gt;* matches the value of any\nhtml properties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for HtmlOperatorOptions {
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
    pub struct HtmlPropertyOptions {
        #[doc = "If set, describes how the property should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::HtmlOperatorOptions>,
        #[doc = "Indicates the search quality importance of the tokens within the\nfield when used for retrieval. Can only be set to DEFAULT or NONE."]
        #[serde(rename = "retrievalImportance", default)]
        pub retrieval_importance: Option<crate::schemas::RetrievalImportance>,
    }
    impl ::field_selector::FieldSelector for HtmlPropertyOptions {
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
    pub struct HtmlValues {
        #[doc = "The maximum allowable length for html values is 2048 characters."]
        #[serde(rename = "values", default)]
        pub values: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for HtmlValues {
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
    pub struct IndexItemOptions {
        #[doc = "Specifies if the index request should allow gsuite principals that do not\nexist or are deleted in the index request."]
        #[serde(rename = "allowUnknownGsuitePrincipals", default)]
        pub allow_unknown_gsuite_principals: Option<bool>,
    }
    impl ::field_selector::FieldSelector for IndexItemOptions {
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
    pub enum IndexItemRequestMode {
        #[doc = "Priority is not specified in the update request.\nLeaving priority unspecified results in an update failure."]
        Unspecified,
        #[doc = "For real-time updates."]
        Synchronous,
        #[doc = "For changes that are executed after the response is sent back to the\ncaller."]
        Asynchronous,
    }
    impl IndexItemRequestMode {
        pub fn as_str(self) -> &'static str {
            match self {
                IndexItemRequestMode::Unspecified => "UNSPECIFIED",
                IndexItemRequestMode::Synchronous => "SYNCHRONOUS",
                IndexItemRequestMode::Asynchronous => "ASYNCHRONOUS",
            }
        }
    }
    impl ::std::fmt::Display for IndexItemRequestMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IndexItemRequestMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IndexItemRequestMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => IndexItemRequestMode::Unspecified,
                "SYNCHRONOUS" => IndexItemRequestMode::Synchronous,
                "ASYNCHRONOUS" => IndexItemRequestMode::Asynchronous,
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
    pub struct IndexItemRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[serde(rename = "indexItemOptions", default)]
        pub index_item_options: Option<crate::schemas::IndexItemOptions>,
        #[doc = "Name of the item.  Format:\ndatasources/{source_id}/items/{item_id}"]
        #[serde(rename = "item", default)]
        pub item: Option<crate::schemas::Item>,
        #[doc = "Required. The RequestMode for this request."]
        #[serde(rename = "mode", default)]
        pub mode: Option<crate::schemas::IndexItemRequestMode>,
    }
    impl ::field_selector::FieldSelector for IndexItemRequest {
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
    pub struct IntegerOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\ninteger property using the greater-than operator. For example, if\ngreaterThanOperatorName is *priorityabove* and the property's name is\n*priorityVal*, then queries like *priorityabove:&lt;value&gt;* will\nshow results only where the value of the property named *priorityVal* is\ngreater than *&lt;value&gt;*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ninteger property using the less-than operator. For example, if\nlessThanOperatorName is *prioritybelow* and the property's name is\n*priorityVal*, then queries like *prioritybelow:&lt;value&gt;* will\nshow results only where the value of the property named *priorityVal* is\nless than *&lt;value&gt;*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ninteger property. For example, if operatorName is *priority* and the\nproperty's name is *priorityVal*, then queries like\n*priority:&lt;value&gt;* will show results only where the value of the\nproperty named *priorityVal* matches *&lt;value&gt;*. By contrast, a\nsearch that uses the same *&lt;value&gt;* without an operator will return\nall items where *&lt;value&gt;* matches the value of any String\nproperties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for IntegerOperatorOptions {
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
    pub enum IntegerPropertyOptionsOrderedRanking {
        #[doc = "There is no ranking order for the property. Results will not be adjusted\nby this property's value."]
        NoOrder,
        #[doc = "This property is ranked in ascending order. Lower values indicate lower\nranking."]
        Ascending,
        #[doc = "This property is ranked in descending order. Lower values indicate\nhigher ranking."]
        Descending,
    }
    impl IntegerPropertyOptionsOrderedRanking {
        pub fn as_str(self) -> &'static str {
            match self {
                IntegerPropertyOptionsOrderedRanking::NoOrder => "NO_ORDER",
                IntegerPropertyOptionsOrderedRanking::Ascending => "ASCENDING",
                IntegerPropertyOptionsOrderedRanking::Descending => "DESCENDING",
            }
        }
    }
    impl ::std::fmt::Display for IntegerPropertyOptionsOrderedRanking {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IntegerPropertyOptionsOrderedRanking {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IntegerPropertyOptionsOrderedRanking {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NO_ORDER" => IntegerPropertyOptionsOrderedRanking::NoOrder,
                "ASCENDING" => IntegerPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => IntegerPropertyOptionsOrderedRanking::Descending,
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
    pub struct IntegerPropertyOptions {
        #[doc = "The maximum value of the property. The minimum and maximum values for the\nproperty are used to rank results according to the\nordered ranking.\nIndexing requests with values greater than the maximum are accepted and\nranked with the same weight as items indexed with the maximum value."]
        #[serde(rename = "maximumValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub maximum_value: Option<i64>,
        #[doc = "The minimum value of the property. The minimum and maximum values for the\nproperty are used to rank results according to the\nordered ranking.\nIndexing requests with values less than the minimum are accepted and\nranked with the same weight as items indexed with the minimum value."]
        #[serde(rename = "minimumValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub minimum_value: Option<i64>,
        #[doc = "If set, describes how the integer should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::IntegerOperatorOptions>,
        #[doc = "Used to specify the ordered ranking for the integer. Can only be used if\nisRepeatable\nis false."]
        #[serde(rename = "orderedRanking", default)]
        pub ordered_ranking: Option<crate::schemas::IntegerPropertyOptionsOrderedRanking>,
    }
    impl ::field_selector::FieldSelector for IntegerPropertyOptions {
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
    pub struct IntegerValues {
        #[serde(rename = "values", default)]
        pub values: Option<Vec<i64>>,
    }
    impl ::field_selector::FieldSelector for IntegerValues {
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
    pub enum InteractionType {
        #[doc = "Invalid value."]
        Unspecified,
        #[doc = "This interaction indicates the user viewed the item."]
        View,
        #[doc = "This interaction indicates the user edited the item."]
        Edit,
    }
    impl InteractionType {
        pub fn as_str(self) -> &'static str {
            match self {
                InteractionType::Unspecified => "UNSPECIFIED",
                InteractionType::View => "VIEW",
                InteractionType::Edit => "EDIT",
            }
        }
    }
    impl ::std::fmt::Display for InteractionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InteractionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InteractionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => InteractionType::Unspecified,
                "VIEW" => InteractionType::View,
                "EDIT" => InteractionType::Edit,
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
    pub struct Interaction {
        #[doc = "The time when the user acted on the item.  If multiple actions of the same\ntype exist for a single user, only the most recent action is recorded."]
        #[serde(rename = "interactionTime", default)]
        pub interaction_time: Option<String>,
        #[doc = "The user that acted on the item."]
        #[serde(rename = "principal", default)]
        pub principal: Option<crate::schemas::Principal>,
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::InteractionType>,
    }
    impl ::field_selector::FieldSelector for Interaction {
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
    pub enum ItemItemType {
        Unspecified,
        #[doc = "An item that is indexed for the only purpose of serving information.\nThese items cannot be referred in\ncontainerName\nor inheritAclFrom\nfields."]
        ContentItem,
        #[doc = "An item that gets indexed and whose purpose is to supply other items\nwith ACLs and/or contain other items."]
        ContainerItem,
        #[doc = "An item that does not get indexed, but otherwise has the same purpose\nas CONTAINER_ITEM."]
        VirtualContainerItem,
    }
    impl ItemItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemItemType::Unspecified => "UNSPECIFIED",
                ItemItemType::ContentItem => "CONTENT_ITEM",
                ItemItemType::ContainerItem => "CONTAINER_ITEM",
                ItemItemType::VirtualContainerItem => "VIRTUAL_CONTAINER_ITEM",
            }
        }
    }
    impl ::std::fmt::Display for ItemItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemItemType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemItemType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => ItemItemType::Unspecified,
                "CONTENT_ITEM" => ItemItemType::ContentItem,
                "CONTAINER_ITEM" => ItemItemType::ContainerItem,
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
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Item {
        #[doc = "Access control list for this item."]
        #[serde(rename = "acl", default)]
        pub acl: Option<crate::schemas::ItemAcl>,
        #[doc = "Item content to be indexed and made text searchable."]
        #[serde(rename = "content", default)]
        pub content: Option<crate::schemas::ItemContent>,
        #[doc = "Type for this item."]
        #[serde(rename = "itemType", default)]
        pub item_type: Option<crate::schemas::ItemItemType>,
        #[doc = "Metadata information."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::ItemMetadata>,
        #[doc = "Name of the Item. Format:\ndatasources/{source_id}/items/{item_id}\n<br />This is a required field.\nThe maximum length is 1536 characters."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Additional state connector can store for this item.\nThe maximum length is 10000 bytes."]
        #[serde(rename = "payload", default)]
        pub payload: Option<Vec<u8>>,
        #[doc = "Queue this item belongs to.\nThe maximum length is 100 characters."]
        #[serde(rename = "queue", default)]
        pub queue: Option<String>,
        #[doc = "Status of the item.\nOutput only field."]
        #[serde(rename = "status", default)]
        pub status: Option<crate::schemas::ItemStatus>,
        #[doc = "The structured data for the item that should conform to a registered\nobject definition in the schema for the data source."]
        #[serde(rename = "structuredData", default)]
        pub structured_data: Option<crate::schemas::ItemStructuredData>,
        #[doc = "Required. The indexing system stores the version from the datasource as a\nbyte string and compares the Item version in the index\nto the version of the queued Item using lexical ordering.\n<br /><br />\nCloud Search Indexing won't index or delete any queued item with\na version value that is less than or equal to the version of the\ncurrently indexed item.\nThe maximum length for this field is 1024 bytes."]
        #[serde(rename = "version", default)]
        pub version: Option<Vec<u8>>,
    }
    impl ::field_selector::FieldSelector for Item {
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
    pub enum ItemAclAclInheritanceType {
        #[doc = "The default value when this item does not inherit an ACL.\nUse NOT_APPLICABLE when\ninheritAclFrom\nis empty.  An item without ACL inheritance can still have ACLs supplied\nby its own readers and\ndeniedReaders fields."]
        NotApplicable,
        #[doc = "During an authorization conflict, the ACL of the child item determines\nits read access."]
        ChildOverride,
        #[doc = "During an authorization conflict, the ACL of the parent item\nspecified in the\ninheritAclFrom\nfield determines read access."]
        ParentOverride,
        #[doc = "Access is granted only if this item and the parent item specified in\nthe inheritAclFrom\nfield both permit read access."]
        BothPermit,
    }
    impl ItemAclAclInheritanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemAclAclInheritanceType::NotApplicable => "NOT_APPLICABLE",
                ItemAclAclInheritanceType::ChildOverride => "CHILD_OVERRIDE",
                ItemAclAclInheritanceType::ParentOverride => "PARENT_OVERRIDE",
                ItemAclAclInheritanceType::BothPermit => "BOTH_PERMIT",
            }
        }
    }
    impl ::std::fmt::Display for ItemAclAclInheritanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemAclAclInheritanceType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemAclAclInheritanceType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOT_APPLICABLE" => ItemAclAclInheritanceType::NotApplicable,
                "CHILD_OVERRIDE" => ItemAclAclInheritanceType::ChildOverride,
                "PARENT_OVERRIDE" => ItemAclAclInheritanceType::ParentOverride,
                "BOTH_PERMIT" => ItemAclAclInheritanceType::BothPermit,
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
    pub struct ItemAcl {
        #[doc = "Sets the type of access rules to apply when an item inherits its ACL from a\nparent. This should always be set in tandem with the\ninheritAclFrom\nfield. Also, when the\ninheritAclFrom field\nis set, this field should be set to a valid AclInheritanceType."]
        #[serde(rename = "aclInheritanceType", default)]
        pub acl_inheritance_type: Option<crate::schemas::ItemAclAclInheritanceType>,
        #[doc = "List of principals who are explicitly denied access to the item in search\nresults. While principals are denied access by default, use denied readers\nto handle exceptions and override the list allowed readers.\nThe maximum number of elements is 100."]
        #[serde(rename = "deniedReaders", default)]
        pub denied_readers: Option<Vec<crate::schemas::Principal>>,
        #[doc = "Name of the item to inherit the Access Permission List (ACL) from.\nNote: ACL inheritance *only* provides access permissions\nto child items and does not define structural relationships, nor does it\nprovide convenient ways to delete large groups of items.\nDeleting an ACL parent from the index only alters the access permissions of\nchild items that reference the parent in the\ninheritAclFrom\nfield. The item is still in the index, but may not\nvisible in search results. By contrast, deletion of a container item\nalso deletes all items that reference the container via the\ncontainerName\nfield.\nThe maximum length for this field is 1536 characters."]
        #[serde(rename = "inheritAclFrom", default)]
        pub inherit_acl_from: Option<String>,
        #[doc = "Optional. List of owners for the item. This field has no bearing on\ndocument access permissions. It does, however, offer\na slight ranking boosts items where the querying user is an owner.\nThe maximum number of elements is 5."]
        #[serde(rename = "owners", default)]
        pub owners: Option<Vec<crate::schemas::Principal>>,
        #[doc = "List of principals who are allowed to see the item in search results.\nOptional if inheriting permissions from another item or if the item\nis not intended to be visible, such as\nvirtual\ncontainers.\nThe maximum number of elements is 1000."]
        #[serde(rename = "readers", default)]
        pub readers: Option<Vec<crate::schemas::Principal>>,
    }
    impl ::field_selector::FieldSelector for ItemAcl {
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
    pub enum ItemContentContentFormat {
        #[doc = "Invalid value."]
        Unspecified,
        #[doc = "contentFormat is HTML."]
        Html,
        #[doc = "contentFormat is free text."]
        Text,
        #[doc = "contentFormat is raw bytes."]
        Raw,
    }
    impl ItemContentContentFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemContentContentFormat::Unspecified => "UNSPECIFIED",
                ItemContentContentFormat::Html => "HTML",
                ItemContentContentFormat::Text => "TEXT",
                ItemContentContentFormat::Raw => "RAW",
            }
        }
    }
    impl ::std::fmt::Display for ItemContentContentFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemContentContentFormat {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemContentContentFormat {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => ItemContentContentFormat::Unspecified,
                "HTML" => ItemContentContentFormat::Html,
                "TEXT" => ItemContentContentFormat::Text,
                "RAW" => ItemContentContentFormat::Raw,
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
    pub struct ItemContent {
        #[doc = "Upload reference ID of a previously uploaded content via write method."]
        #[serde(rename = "contentDataRef", default)]
        pub content_data_ref: Option<crate::schemas::UploadItemRef>,
        #[serde(rename = "contentFormat", default)]
        pub content_format: Option<crate::schemas::ItemContentContentFormat>,
        #[doc = "Hashing info calculated and provided by the API client for content.\nCan be used with the items.push method to calculate modified state.\nThe maximum length is 2048 characters."]
        #[serde(rename = "hash", default)]
        pub hash: Option<String>,
        #[doc = "Content that is supplied inlined within the update method.\nThe maximum length is 102400 bytes (100 KiB)."]
        #[serde(rename = "inlineContent", default)]
        pub inline_content: Option<Vec<u8>>,
    }
    impl ::field_selector::FieldSelector for ItemContent {
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
    pub enum ItemCountByStatusStatusCode {
        #[doc = "Input-only value.  Used with\nItems.list\nto list all items in the queue, regardless of status."]
        CodeUnspecified,
        #[doc = "Error encountered by Cloud Search while processing this item.\nDetails of the error are in\nrepositoryError."]
        Error,
        #[doc = "Item has been modified in the repository, and is out of date with\nthe version previously accepted into Cloud Search."]
        Modified,
        #[doc = "Item is known to exist in the repository, but is not yet accepted by\nCloud Search.\nAn item can be in this state when\nItems.push\nhas been called for\nan item of this name that did not exist previously."]
        NewItem,
        #[doc = "API has accepted the up-to-date data of this item."]
        Accepted,
    }
    impl ItemCountByStatusStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemCountByStatusStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ItemCountByStatusStatusCode::Error => "ERROR",
                ItemCountByStatusStatusCode::Modified => "MODIFIED",
                ItemCountByStatusStatusCode::NewItem => "NEW_ITEM",
                ItemCountByStatusStatusCode::Accepted => "ACCEPTED",
            }
        }
    }
    impl ::std::fmt::Display for ItemCountByStatusStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemCountByStatusStatusCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemCountByStatusStatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ItemCountByStatusStatusCode::CodeUnspecified,
                "ERROR" => ItemCountByStatusStatusCode::Error,
                "MODIFIED" => ItemCountByStatusStatusCode::Modified,
                "NEW_ITEM" => ItemCountByStatusStatusCode::NewItem,
                "ACCEPTED" => ItemCountByStatusStatusCode::Accepted,
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
    pub struct ItemCountByStatus {
        #[doc = "Number of items matching the status code."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: Option<i64>,
        #[doc = "Status of the items."]
        #[serde(rename = "statusCode", default)]
        pub status_code: Option<crate::schemas::ItemCountByStatusStatusCode>,
    }
    impl ::field_selector::FieldSelector for ItemCountByStatus {
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
    pub struct ItemMetadata {
        #[doc = "The name of the container for this item.\nDeletion of the container item leads to automatic deletion of this\nitem.  Note: ACLs are not inherited from a container item.\nTo provide ACL inheritance for an item, use the\ninheritAclFrom\nfield. The maximum length is 1536 characters."]
        #[serde(rename = "containerName", default)]
        pub container_name: Option<String>,
        #[doc = "The BCP-47 language code for the item, such as \"en-US\" or \"sr-Latn\". For\nmore information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier.\nThe maximum length is 32 characters."]
        #[serde(rename = "contentLanguage", default)]
        pub content_language: Option<String>,
        #[doc = "The time when the item was created in the source repository."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Hashing value provided by the API caller.\nThis can be used with the\nitems.push\nmethod to calculate modified state.\nThe maximum length is 2048 characters."]
        #[serde(rename = "hash", default)]
        pub hash: Option<String>,
        #[doc = "A list of interactions for the item.  Interactions are used to improve\nSearch quality, but are not exposed to end users.\nThe maximum number of elements is 1000."]
        #[serde(rename = "interactions", default)]
        pub interactions: Option<Vec<crate::schemas::Interaction>>,
        #[doc = "Additional keywords or phrases that should match the item.\nUsed internally for user generated content.\nThe maximum number of elements is 100.\nThe maximum length is 8192 characters."]
        #[serde(rename = "keywords", default)]
        pub keywords: Option<Vec<String>>,
        #[doc = "The original mime-type of\nItemContent.content\nin the source repository.\nThe maximum length is 256 characters."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
        #[doc = "The type of the item.  This should correspond to the name of an object\ndefinition in the schema registered for the data source.  For example, if\nthe schema for the data source contains an object definition with name\n'document', then item indexing requests for objects of that type should set\nobjectType to 'document'.\nThe maximum length is 256 characters."]
        #[serde(rename = "objectType", default)]
        pub object_type: Option<String>,
        #[doc = "Additional search quality metadata of the item"]
        #[serde(rename = "searchQualityMetadata", default)]
        pub search_quality_metadata: Option<crate::schemas::SearchQualityMetadata>,
        #[doc = "Link to the source repository serving the data.  &#83;earch results apply\nthis link to the title.\nThe maximum length is 2048 characters."]
        #[serde(rename = "sourceRepositoryUrl", default)]
        pub source_repository_url: Option<String>,
        #[doc = "The title of the item.  If given, this will be the displayed title of the\nSearch result.\nThe maximum length is 2048 characters."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[doc = "The time when the item was last modified in the source repository."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for ItemMetadata {
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
    pub enum ItemStatusCode {
        #[doc = "Input-only value.  Used with\nItems.list\nto list all items in the queue, regardless of status."]
        CodeUnspecified,
        #[doc = "Error encountered by Cloud Search while processing this item.\nDetails of the error are in\nrepositoryError."]
        Error,
        #[doc = "Item has been modified in the repository, and is out of date with\nthe version previously accepted into Cloud Search."]
        Modified,
        #[doc = "Item is known to exist in the repository, but is not yet accepted by\nCloud Search.\nAn item can be in this state when\nItems.push\nhas been called for\nan item of this name that did not exist previously."]
        NewItem,
        #[doc = "API has accepted the up-to-date data of this item."]
        Accepted,
    }
    impl ItemStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ItemStatusCode::Error => "ERROR",
                ItemStatusCode::Modified => "MODIFIED",
                ItemStatusCode::NewItem => "NEW_ITEM",
                ItemStatusCode::Accepted => "ACCEPTED",
            }
        }
    }
    impl ::std::fmt::Display for ItemStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemStatusCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemStatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ItemStatusCode::CodeUnspecified,
                "ERROR" => ItemStatusCode::Error,
                "MODIFIED" => ItemStatusCode::Modified,
                "NEW_ITEM" => ItemStatusCode::NewItem,
                "ACCEPTED" => ItemStatusCode::Accepted,
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
    pub struct ItemStatus {
        #[doc = "Status code."]
        #[serde(rename = "code", default)]
        pub code: Option<crate::schemas::ItemStatusCode>,
        #[doc = "Error details in case the item is in ERROR state."]
        #[serde(rename = "processingErrors", default)]
        pub processing_errors: Option<Vec<crate::schemas::ProcessingError>>,
        #[doc = "Repository error reported by connector."]
        #[serde(rename = "repositoryErrors", default)]
        pub repository_errors: Option<Vec<crate::schemas::RepositoryError>>,
    }
    impl ::field_selector::FieldSelector for ItemStatus {
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
    pub struct ItemStructuredData {
        #[doc = "Hashing value provided by the API caller.\nThis can be used with the\nitems.push\nmethod to calculate modified state.\nThe maximum length is 2048 characters."]
        #[serde(rename = "hash", default)]
        pub hash: Option<String>,
        #[doc = "The structured data object that should conform to a registered object\ndefinition in the schema for the data source."]
        #[serde(rename = "object", default)]
        pub object: Option<crate::schemas::StructuredDataObject>,
    }
    impl ::field_selector::FieldSelector for ItemStructuredData {
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
    pub struct ListDataSourceResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[serde(rename = "sources", default)]
        pub sources: Option<Vec<crate::schemas::DataSource>>,
    }
    impl ::field_selector::FieldSelector for ListDataSourceResponse {
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
    pub struct ListItemNamesForUnmappedIdentityResponse {
        #[serde(rename = "itemNames", default)]
        pub item_names: Option<Vec<String>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListItemNamesForUnmappedIdentityResponse {
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
    pub struct ListItemsResponse {
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Item>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListItemsResponse {
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
    pub struct ListQuerySourcesResponse {
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[serde(rename = "sources", default)]
        pub sources: Option<Vec<crate::schemas::QuerySource>>,
    }
    impl ::field_selector::FieldSelector for ListQuerySourcesResponse {
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
    pub struct ListSearchApplicationsResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[serde(rename = "searchApplications", default)]
        pub search_applications: Option<Vec<crate::schemas::SearchApplication>>,
    }
    impl ::field_selector::FieldSelector for ListSearchApplicationsResponse {
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
    pub struct ListUnmappedIdentitiesResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[serde(rename = "unmappedIdentities", default)]
        pub unmapped_identities: Option<Vec<crate::schemas::UnmappedIdentity>>,
    }
    impl ::field_selector::FieldSelector for ListUnmappedIdentitiesResponse {
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
    pub struct MatchRange {
        #[doc = "End of the match in the snippet."]
        #[serde(rename = "end", default)]
        pub end: Option<i32>,
        #[doc = "Starting position of the match in the snippet."]
        #[serde(rename = "start", default)]
        pub start: Option<i32>,
    }
    impl ::field_selector::FieldSelector for MatchRange {
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
    pub struct Media {
        #[doc = "Name of the media resource."]
        #[serde(rename = "resourceName", default)]
        pub resource_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Media {
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
    pub struct Metadata {
        #[doc = "The creation time for this document or object in the search result."]
        #[serde(rename = "createTime", default)]
        pub create_time: Option<String>,
        #[doc = "Options that specify how to display a structured data search result."]
        #[serde(rename = "displayOptions", default)]
        pub display_options: Option<crate::schemas::ResultDisplayMetadata>,
        #[doc = "Indexed fields in structured data, returned as a generic named property."]
        #[serde(rename = "fields", default)]
        pub fields: Option<Vec<crate::schemas::NamedProperty>>,
        #[doc = "Mime type of the search result."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
        #[doc = "Object type of the search result."]
        #[serde(rename = "objectType", default)]
        pub object_type: Option<String>,
        #[doc = "Owner (usually creator) of the document or object of the search result."]
        #[serde(rename = "owner", default)]
        pub owner: Option<crate::schemas::Person>,
        #[doc = "The named source for the result, such as Gmail."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
        #[doc = "The last modified date for the object in the search result. If not\nset in the item, the value returned here is empty. When\n`updateTime` is used for calculating freshness and is not set, this\nvalue defaults to 2 years from the current time."]
        #[serde(rename = "updateTime", default)]
        pub update_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for Metadata {
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
    pub struct Metaline {
        #[doc = "The list of displayed properties for the metaline. The maxiumum number of\nproperties is 5."]
        #[serde(rename = "properties", default)]
        pub properties: Option<Vec<crate::schemas::DisplayedProperty>>,
    }
    impl ::field_selector::FieldSelector for Metaline {
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
    pub struct Name {
        #[doc = "The read-only display name formatted according to the locale specified by\nthe viewer's account or the <code>Accept-Language</code> HTTP header."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Name {
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
    pub struct NamedProperty {
        #[serde(rename = "booleanValue", default)]
        pub boolean_value: Option<bool>,
        #[serde(rename = "dateValues", default)]
        pub date_values: Option<crate::schemas::DateValues>,
        #[serde(rename = "doubleValues", default)]
        pub double_values: Option<crate::schemas::DoubleValues>,
        #[serde(rename = "enumValues", default)]
        pub enum_values: Option<crate::schemas::EnumValues>,
        #[serde(rename = "htmlValues", default)]
        pub html_values: Option<crate::schemas::HtmlValues>,
        #[serde(rename = "integerValues", default)]
        pub integer_values: Option<crate::schemas::IntegerValues>,
        #[doc = "The name of the property.  This name should correspond to the name of the\nproperty that was registered for object definition in the schema.\nThe maximum allowable length for this property is 256 characters."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[serde(rename = "objectValues", default)]
        pub object_values: Option<crate::schemas::ObjectValues>,
        #[serde(rename = "textValues", default)]
        pub text_values: Option<crate::schemas::TextValues>,
        #[serde(rename = "timestampValues", default)]
        pub timestamp_values: Option<crate::schemas::TimestampValues>,
    }
    impl ::field_selector::FieldSelector for NamedProperty {
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
    pub struct ObjectDefinition {
        #[doc = "Name for the object, which then defines its type. Item indexing requests\nshould set the\nobjectType field\nequal to this value. For example, if *name* is *Document*, then indexing\nrequests for items of type Document should set\nobjectType equal to\n*Document*. Each object definition must be uniquely named within a schema.\nThe name must start with a letter and can only contain letters (A-Z, a-z)\nor numbers (0-9).\nThe maximum length is 256 characters."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The optional object-specific options."]
        #[serde(rename = "options", default)]
        pub options: Option<crate::schemas::ObjectOptions>,
        #[doc = "The property definitions for the object.\nThe maximum number of elements is 1000."]
        #[serde(rename = "propertyDefinitions", default)]
        pub property_definitions: Option<Vec<crate::schemas::PropertyDefinition>>,
    }
    impl ::field_selector::FieldSelector for ObjectDefinition {
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
    pub struct ObjectDisplayOptions {
        #[doc = "Defines the properties that will be displayed in the metalines of the\nsearch results. The property values will be displayed in the order given\nhere. If a property holds multiple values, all of the values will be\ndiplayed before the next properties. For this reason, it is a good practice\nto specify singular properties before repeated properties in this list. All\nof the properties must set\nis_returnable\nto true. The maximum number of metalines is 3."]
        #[serde(rename = "metalines", default)]
        pub metalines: Option<Vec<crate::schemas::Metaline>>,
        #[doc = "The user friendly label to display in the search result to inidicate the\ntype of the item. This is OPTIONAL; if not given, an object label will not\nbe displayed on the context line of the search results. The maximum length\nis 32 characters."]
        #[serde(rename = "objectDisplayLabel", default)]
        pub object_display_label: Option<String>,
    }
    impl ::field_selector::FieldSelector for ObjectDisplayOptions {
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
    pub struct ObjectOptions {
        #[doc = "Options that determine how the object is displayed in the Cloud Search\nresults page."]
        #[serde(rename = "displayOptions", default)]
        pub display_options: Option<crate::schemas::ObjectDisplayOptions>,
        #[doc = "The freshness options for an object."]
        #[serde(rename = "freshnessOptions", default)]
        pub freshness_options: Option<crate::schemas::FreshnessOptions>,
    }
    impl ::field_selector::FieldSelector for ObjectOptions {
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
    pub struct ObjectPropertyOptions {
        #[doc = "The properties of the sub-object. These properties represent a nested\nobject. For example, if this property represents a postal address, the\nsubobjectProperties might be named *street*, *city*, and *state*.\nThe maximum number of elements is 1000."]
        #[serde(rename = "subobjectProperties", default)]
        pub subobject_properties: Option<Vec<crate::schemas::PropertyDefinition>>,
    }
    impl ::field_selector::FieldSelector for ObjectPropertyOptions {
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
    pub struct ObjectValues {
        #[serde(rename = "values", default)]
        pub values: Option<Vec<crate::schemas::StructuredDataObject>>,
    }
    impl ::field_selector::FieldSelector for ObjectValues {
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
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Operation {
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
    pub struct PeopleSuggestion {
        #[doc = "Suggested person. All fields of the person object might not be populated."]
        #[serde(rename = "person", default)]
        pub person: Option<crate::schemas::Person>,
    }
    impl ::field_selector::FieldSelector for PeopleSuggestion {
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
    pub struct Person {
        #[doc = "The person's email addresses"]
        #[serde(rename = "emailAddresses", default)]
        pub email_addresses: Option<Vec<crate::schemas::EmailAddress>>,
        #[doc = "The resource name of the person to provide information about.\nSee <a href=\"https://developers.google.com/people/api/rest/v1/people/get\">\nPeople.get</a> from Google People API."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Obfuscated ID of a person."]
        #[serde(rename = "obfuscatedId", default)]
        pub obfuscated_id: Option<String>,
        #[doc = "The person's name"]
        #[serde(rename = "personNames", default)]
        pub person_names: Option<Vec<crate::schemas::Name>>,
        #[doc = "A person's read-only photo. A picture shown next to the person's name to\nhelp others recognize the person in search results."]
        #[serde(rename = "photos", default)]
        pub photos: Option<Vec<crate::schemas::Photo>>,
    }
    impl ::field_selector::FieldSelector for Person {
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
    pub struct Photo {
        #[doc = "The URL of the photo."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for Photo {
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
    pub enum PollItemsRequestStatusCodesItems {}
    impl PollItemsRequestStatusCodesItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for PollItemsRequestStatusCodesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PollItemsRequestStatusCodesItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PollItemsRequestStatusCodesItems {
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
    #[derive(
        Debug,
        Clone,
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
        pub connector_name: Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[doc = "Maximum number of items to return.\n<br />The maximum and the default value is 1000"]
        #[serde(rename = "limit", default)]
        pub limit: Option<i32>,
        #[doc = "Queue name to fetch items from.  If unspecified, PollItems will\nfetch from 'default' queue.\nThe maximum length is 100 characters."]
        #[serde(rename = "queue", default)]
        pub queue: Option<String>,
        #[doc = "Limit the items polled to the ones with these statuses."]
        #[serde(rename = "statusCodes", default)]
        pub status_codes: Option<Vec<crate::schemas::PollItemsRequestStatusCodesItems>>,
    }
    impl ::field_selector::FieldSelector for PollItemsRequest {
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
    pub struct PollItemsResponse {
        #[doc = "Set of items from the queue available for connector to process.\n<br />These items have the following subset of fields populated: <br />\n<br />version\n<br />metadata.hash\n<br />structured_data.hash\n<br />content.hash\n<br />payload\n<br />status\n<br />queue"]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Item>>,
    }
    impl ::field_selector::FieldSelector for PollItemsResponse {
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
    pub struct Principal {
        #[doc = "This principal is a group identified using an external identity.\nThe name field must specify the group resource name with this format:\nidentitysources/{source_id}/groups/{ID}"]
        #[serde(rename = "groupResourceName", default)]
        pub group_resource_name: Option<String>,
        #[doc = "This principal is a GSuite user, group or domain."]
        #[serde(rename = "gsuitePrincipal", default)]
        pub gsuite_principal: Option<crate::schemas::GsuitePrincipal>,
        #[doc = "This principal is a user identified using an external identity.\nThe name field must specify the user resource name with this format:\nidentitysources/{source_id}/users/{ID}"]
        #[serde(rename = "userResourceName", default)]
        pub user_resource_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Principal {
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
    pub enum ProcessingErrorCode {
        #[doc = "Input only value.  Use this value in Items."]
        ProcessingErrorCodeUnspecified,
        #[doc = "Item's ACL, metadata, or content is malformed or in invalid state.\nFieldViolations contains more details on where the problem is."]
        MalformedRequest,
        #[doc = "Countent format is unsupported."]
        UnsupportedContentFormat,
        #[doc = "Items with incomplete ACL information due to inheriting other\nitems with broken ACL or having groups with unmapped descendants."]
        IndirectBrokenAcl,
        #[doc = "ACL inheritance graph formed a cycle."]
        AclCycle,
    }
    impl ProcessingErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ProcessingErrorCode::ProcessingErrorCodeUnspecified => {
                    "PROCESSING_ERROR_CODE_UNSPECIFIED"
                }
                ProcessingErrorCode::MalformedRequest => "MALFORMED_REQUEST",
                ProcessingErrorCode::UnsupportedContentFormat => "UNSUPPORTED_CONTENT_FORMAT",
                ProcessingErrorCode::IndirectBrokenAcl => "INDIRECT_BROKEN_ACL",
                ProcessingErrorCode::AclCycle => "ACL_CYCLE",
            }
        }
    }
    impl ::std::fmt::Display for ProcessingErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProcessingErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProcessingErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PROCESSING_ERROR_CODE_UNSPECIFIED" => {
                    ProcessingErrorCode::ProcessingErrorCodeUnspecified
                }
                "MALFORMED_REQUEST" => ProcessingErrorCode::MalformedRequest,
                "UNSUPPORTED_CONTENT_FORMAT" => ProcessingErrorCode::UnsupportedContentFormat,
                "INDIRECT_BROKEN_ACL" => ProcessingErrorCode::IndirectBrokenAcl,
                "ACL_CYCLE" => ProcessingErrorCode::AclCycle,
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
    pub struct ProcessingError {
        #[doc = "Error code indicating the nature of the error."]
        #[serde(rename = "code", default)]
        pub code: Option<crate::schemas::ProcessingErrorCode>,
        #[doc = "Description of the error."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
        #[doc = "In case the item fields are invalid, this field contains the details\nabout the validation errors."]
        #[serde(rename = "fieldViolations", default)]
        pub field_violations: Option<Vec<crate::schemas::FieldViolation>>,
    }
    impl ::field_selector::FieldSelector for ProcessingError {
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
    pub struct PropertyDefinition {
        #[serde(rename = "booleanPropertyOptions", default)]
        pub boolean_property_options: Option<crate::schemas::BooleanPropertyOptions>,
        #[serde(rename = "datePropertyOptions", default)]
        pub date_property_options: Option<crate::schemas::DatePropertyOptions>,
        #[doc = "Options that determine how the property is displayed in the Cloud Search\nresults page if it is specified to be displayed in the object's\ndisplay options\n."]
        #[serde(rename = "displayOptions", default)]
        pub display_options: Option<crate::schemas::PropertyDisplayOptions>,
        #[serde(rename = "doublePropertyOptions", default)]
        pub double_property_options: Option<crate::schemas::DoublePropertyOptions>,
        #[serde(rename = "enumPropertyOptions", default)]
        pub enum_property_options: Option<crate::schemas::EnumPropertyOptions>,
        #[serde(rename = "htmlPropertyOptions", default)]
        pub html_property_options: Option<crate::schemas::HtmlPropertyOptions>,
        #[serde(rename = "integerPropertyOptions", default)]
        pub integer_property_options: Option<crate::schemas::IntegerPropertyOptions>,
        #[doc = "Indicates that the property can be used for generating facets. Cannot be\ntrue for properties whose type is object. IsReturnable must be true to set\nthis option.\nOnly supported for Boolean, Enum, and Text properties."]
        #[serde(rename = "isFacetable", default)]
        pub is_facetable: Option<bool>,
        #[doc = "Indicates that multiple values are allowed for the property. For example, a\ndocument only has one description but can have multiple comments. Cannot be\ntrue for properties whose type is a boolean.\nIf set to false, properties that contain more than one value will cause the\nindexing request for that item to be rejected."]
        #[serde(rename = "isRepeatable", default)]
        pub is_repeatable: Option<bool>,
        #[doc = "Indicates that the property identifies data that should be returned in\nsearch results via the Query API. If set to *true*, indicates that Query\nAPI users can use matching property fields in results. However, storing\nfields requires more space allocation and uses more bandwidth for search\nqueries, which impacts performance over large datasets. Set to *true* here\nonly if the field is needed for search results. Cannot be true for\nproperties whose type is an object."]
        #[serde(rename = "isReturnable", default)]
        pub is_returnable: Option<bool>,
        #[doc = "Indicates that the property can be used for sorting. Cannot be true for\nproperties that are repeatable. Cannot be true for properties whose type\nis object or user identifier. IsReturnable must be true to set this option.\nOnly supported for Boolean, Date, Double, Integer, and Timestamp\nproperties."]
        #[serde(rename = "isSortable", default)]
        pub is_sortable: Option<bool>,
        #[doc = "Indicates that users can perform wildcard search for this\nproperty. Only supported for Text properties. IsReturnable must be true to\nset this option. In a given datasource maximum of 5 properties can be\nmarked as is_wildcard_searchable.\n\nNote: This is an alpha feature and is enabled for whitelisted users only."]
        #[serde(rename = "isWildcardSearchable", default)]
        pub is_wildcard_searchable: Option<bool>,
        #[doc = "The name of the property. Item indexing requests sent to the Indexing API\nshould set the property name\nequal to this value. For example, if name is *subject_line*, then indexing\nrequests for document items with subject fields should set the\nname for that field equal to\n*subject_line*. Use the name as the identifier for the object property.\nOnce registered as a property for an object, you cannot re-use this name\nfor another property within that object.\nThe name must start with a letter and can only contain letters (A-Z, a-z)\nor numbers (0-9).\nThe maximum length is 256 characters."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[serde(rename = "objectPropertyOptions", default)]
        pub object_property_options: Option<crate::schemas::ObjectPropertyOptions>,
        #[serde(rename = "textPropertyOptions", default)]
        pub text_property_options: Option<crate::schemas::TextPropertyOptions>,
        #[serde(rename = "timestampPropertyOptions", default)]
        pub timestamp_property_options: Option<crate::schemas::TimestampPropertyOptions>,
    }
    impl ::field_selector::FieldSelector for PropertyDefinition {
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
    pub struct PropertyDisplayOptions {
        #[doc = "The user friendly label for the property that will be used if the property\nis specified to be displayed in ObjectDisplayOptions. If given, the display\nlabel will be shown in front of the property values when the property is\npart of the object display options. For example, if the property value is\n'1', the value by itself may not be useful context for the user. If the\ndisplay name given was 'priority', then the user will see 'priority : 1' in\nthe search results which provides clear conext to search users. This is\nOPTIONAL; if not given, only the property values will be displayed.\nThe maximum length is 32 characters."]
        #[serde(rename = "displayLabel", default)]
        pub display_label: Option<String>,
    }
    impl ::field_selector::FieldSelector for PropertyDisplayOptions {
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
    pub enum PushItemType {
        #[doc = "Default UNSPECIFIED.  Specifies that the push operation should not modify\nItemStatus"]
        Unspecified,
        #[doc = "Indicates that the repository document has been modified or updated since\nthe previous\nupdate\ncall. This changes status to\nMODIFIED state for\nan existing item. If this is called on a non existing item, the status is\nchanged to\nNEW_ITEM."]
        Modified,
        #[doc = "Item in the repository has not been modified since the last update\ncall.  This push operation will set status to\nACCEPTED state."]
        NotModified,
        #[doc = "Connector is facing a repository error regarding this item.  Change\nstatus to\nREPOSITORY_ERROR\nstate. Item is unreserved and rescheduled at a future time determined by\nexponential backoff."]
        RepositoryError,
        #[doc = "Call push with REQUEUE only for items that have been reserved.\nThis action unreserves the item and resets its available time to the\nwall clock time."]
        Requeue,
    }
    impl PushItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                PushItemType::Unspecified => "UNSPECIFIED",
                PushItemType::Modified => "MODIFIED",
                PushItemType::NotModified => "NOT_MODIFIED",
                PushItemType::RepositoryError => "REPOSITORY_ERROR",
                PushItemType::Requeue => "REQUEUE",
            }
        }
    }
    impl ::std::fmt::Display for PushItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PushItemType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PushItemType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED" => PushItemType::Unspecified,
                "MODIFIED" => PushItemType::Modified,
                "NOT_MODIFIED" => PushItemType::NotModified,
                "REPOSITORY_ERROR" => PushItemType::RepositoryError,
                "REQUEUE" => PushItemType::Requeue,
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
    pub struct PushItem {
        #[doc = "Content hash of the item according to the repository. If specified, this is\nused to determine how to modify this\nitem's status. Setting this field and the\ntype field results in argument\nerror.\nThe maximum length is 2048 characters."]
        #[serde(rename = "contentHash", default)]
        pub content_hash: Option<String>,
        #[doc = "Metadata hash of the item according to the repository. If specified, this\nis used to determine how to modify this\nitem's status. Setting this field and the\ntype field results in argument\nerror.\nThe maximum length is 2048 characters."]
        #[serde(rename = "metadataHash", default)]
        pub metadata_hash: Option<String>,
        #[doc = "Provides additional document state information for the connector,\nsuch as an alternate repository ID and other metadata.\nThe maximum length is 8192 bytes."]
        #[serde(rename = "payload", default)]
        pub payload: Option<Vec<u8>>,
        #[doc = "Queue to which this item belongs to.  The <code>default</code> queue is\nchosen if this field is not specified. The maximum length is\n512 characters."]
        #[serde(rename = "queue", default)]
        pub queue: Option<String>,
        #[doc = "The type of the push operation that defines the push behavior."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::PushItemType>,
        #[doc = "Populate this field to store Connector or repository error details.\nThis information is displayed in the Admin Console.\nThis field may only be populated when the\nType is\nREPOSITORY_ERROR."]
        #[serde(rename = "repositoryError", default)]
        pub repository_error: Option<crate::schemas::RepositoryError>,
        #[doc = "Structured data hash of the item according to the repository. If specified,\nthis is used to determine how to modify this item's status. Setting this\nfield and the type field\nresults in argument error.\nThe maximum length is 2048 characters."]
        #[serde(rename = "structuredDataHash", default)]
        pub structured_data_hash: Option<String>,
    }
    impl ::field_selector::FieldSelector for PushItem {
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
    pub struct PushItemRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[doc = "Item to push onto the queue."]
        #[serde(rename = "item", default)]
        pub item: Option<crate::schemas::PushItem>,
    }
    impl ::field_selector::FieldSelector for PushItemRequest {
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
    pub enum QueryInterpretationInterpretationType {
        #[doc = "No natural language interpretation or the natural language interpretation\nis not used to fetch the search results."]
        None,
        #[doc = "The natural language results is mixed with results from original query."]
        Blend,
        #[doc = "The results only contain natural language results."]
        Replace,
    }
    impl QueryInterpretationInterpretationType {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryInterpretationInterpretationType::None => "NONE",
                QueryInterpretationInterpretationType::Blend => "BLEND",
                QueryInterpretationInterpretationType::Replace => "REPLACE",
            }
        }
    }
    impl ::std::fmt::Display for QueryInterpretationInterpretationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryInterpretationInterpretationType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryInterpretationInterpretationType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => QueryInterpretationInterpretationType::None,
                "BLEND" => QueryInterpretationInterpretationType::Blend,
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
    #[derive(
        Debug,
        Clone,
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
        pub interpretation_type: Option<crate::schemas::QueryInterpretationInterpretationType>,
        #[doc = "The interpretation of the query used in search. For example, query \"email\nfrom john\" will be interpreted as \"from:john source:mail\""]
        #[serde(rename = "interpretedQuery", default)]
        pub interpreted_query: Option<String>,
    }
    impl ::field_selector::FieldSelector for QueryInterpretation {
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
    pub struct QueryInterpretationOptions {
        #[doc = "Flag to disable natural language (NL) interpretation of queries. Default is\nfalse, Set to true to disable natural language interpretation. NL\ninterpretation only applies to predefined datasources."]
        #[serde(rename = "disableNlInterpretation", default)]
        pub disable_nl_interpretation: Option<bool>,
    }
    impl ::field_selector::FieldSelector for QueryInterpretationOptions {
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
    pub struct QueryItem {
        #[doc = "True if the text was generated by means other than a previous user search."]
        #[serde(rename = "isSynthetic", default)]
        pub is_synthetic: Option<bool>,
    }
    impl ::field_selector::FieldSelector for QueryItem {
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
    pub enum QueryOperatorType {
        #[doc = "Invalid value."]
        Unknown,
        Integer,
        Double,
        Timestamp,
        Boolean,
        Enum,
        Date,
        Text,
        Html,
    }
    impl QueryOperatorType {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryOperatorType::Unknown => "UNKNOWN",
                QueryOperatorType::Integer => "INTEGER",
                QueryOperatorType::Double => "DOUBLE",
                QueryOperatorType::Timestamp => "TIMESTAMP",
                QueryOperatorType::Boolean => "BOOLEAN",
                QueryOperatorType::Enum => "ENUM",
                QueryOperatorType::Date => "DATE",
                QueryOperatorType::Text => "TEXT",
                QueryOperatorType::Html => "HTML",
            }
        }
    }
    impl ::std::fmt::Display for QueryOperatorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryOperatorType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryOperatorType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => QueryOperatorType::Unknown,
                "INTEGER" => QueryOperatorType::Integer,
                "DOUBLE" => QueryOperatorType::Double,
                "TIMESTAMP" => QueryOperatorType::Timestamp,
                "BOOLEAN" => QueryOperatorType::Boolean,
                "ENUM" => QueryOperatorType::Enum,
                "DATE" => QueryOperatorType::Date,
                "TEXT" => QueryOperatorType::Text,
                "HTML" => QueryOperatorType::Html,
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
    pub struct QueryOperator {
        #[doc = "Display name of the operator"]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "Potential list of values for the opeatror field. This field is only filled\nwhen we can safely enumerate all the possible values of this operator."]
        #[serde(rename = "enumValues", default)]
        pub enum_values: Option<Vec<String>>,
        #[doc = "Indicates the operator name that can be used to  isolate the property using\nthe greater-than operator."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: Option<String>,
        #[doc = "Can this operator be used to get facets."]
        #[serde(rename = "isFacetable", default)]
        pub is_facetable: Option<bool>,
        #[doc = "Indicates if multiple values can be set for this property."]
        #[serde(rename = "isRepeatable", default)]
        pub is_repeatable: Option<bool>,
        #[doc = "Will the property associated with this facet be returned as part of search\nresults."]
        #[serde(rename = "isReturnable", default)]
        pub is_returnable: Option<bool>,
        #[doc = "Can this operator be used to sort results."]
        #[serde(rename = "isSortable", default)]
        pub is_sortable: Option<bool>,
        #[doc = "Can get suggestions for this field."]
        #[serde(rename = "isSuggestable", default)]
        pub is_suggestable: Option<bool>,
        #[doc = "Indicates the operator name that can be used to  isolate the property using\nthe less-than operator."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: Option<String>,
        #[doc = "The name of the operator."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
        #[doc = "Type of the operator."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::QueryOperatorType>,
    }
    impl ::field_selector::FieldSelector for QueryOperator {
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
    pub struct QuerySource {
        #[doc = "Display name of the data source."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "List of all operators applicable for this source."]
        #[serde(rename = "operators", default)]
        pub operators: Option<Vec<crate::schemas::QueryOperator>>,
        #[doc = "A short name or alias for the source.  This value can be used with the\n'source' operator."]
        #[serde(rename = "shortName", default)]
        pub short_name: Option<String>,
        #[doc = "Name of the source"]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for QuerySource {
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
    pub struct QuerySuggestion;
    impl ::field_selector::FieldSelector for QuerySuggestion {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RepositoryErrorType {
        #[doc = "Unknown error."]
        Unknown,
        #[doc = "Unknown or unreachable host."]
        NetworkError,
        #[doc = "DNS problem, such as the DNS server is not responding."]
        DnsError,
        #[doc = "Cannot connect to the repository server."]
        ConnectionError,
        #[doc = "Failed authentication due to incorrect credentials."]
        AuthenticationError,
        #[doc = "Service account is not authorized for the repository."]
        AuthorizationError,
        #[doc = "Repository server error."]
        ServerError,
        #[doc = "Quota exceeded."]
        QuotaExceeded,
        #[doc = "Server temporarily unavailable."]
        ServiceUnavailable,
        #[doc = "Client-related error, such as an invalid request from the connector to\nthe repository server."]
        ClientError,
    }
    impl RepositoryErrorType {
        pub fn as_str(self) -> &'static str {
            match self {
                RepositoryErrorType::Unknown => "UNKNOWN",
                RepositoryErrorType::NetworkError => "NETWORK_ERROR",
                RepositoryErrorType::DnsError => "DNS_ERROR",
                RepositoryErrorType::ConnectionError => "CONNECTION_ERROR",
                RepositoryErrorType::AuthenticationError => "AUTHENTICATION_ERROR",
                RepositoryErrorType::AuthorizationError => "AUTHORIZATION_ERROR",
                RepositoryErrorType::ServerError => "SERVER_ERROR",
                RepositoryErrorType::QuotaExceeded => "QUOTA_EXCEEDED",
                RepositoryErrorType::ServiceUnavailable => "SERVICE_UNAVAILABLE",
                RepositoryErrorType::ClientError => "CLIENT_ERROR",
            }
        }
    }
    impl ::std::fmt::Display for RepositoryErrorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RepositoryErrorType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RepositoryErrorType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => RepositoryErrorType::Unknown,
                "NETWORK_ERROR" => RepositoryErrorType::NetworkError,
                "DNS_ERROR" => RepositoryErrorType::DnsError,
                "CONNECTION_ERROR" => RepositoryErrorType::ConnectionError,
                "AUTHENTICATION_ERROR" => RepositoryErrorType::AuthenticationError,
                "AUTHORIZATION_ERROR" => RepositoryErrorType::AuthorizationError,
                "SERVER_ERROR" => RepositoryErrorType::ServerError,
                "QUOTA_EXCEEDED" => RepositoryErrorType::QuotaExceeded,
                "SERVICE_UNAVAILABLE" => RepositoryErrorType::ServiceUnavailable,
                "CLIENT_ERROR" => RepositoryErrorType::ClientError,
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
    pub struct RepositoryError {
        #[doc = "Message that describes the error. The maximum allowable length\nof the message is 8192 characters."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
        #[doc = "Error codes.  Matches the definition of HTTP status codes."]
        #[serde(rename = "httpStatusCode", default)]
        pub http_status_code: Option<i32>,
        #[doc = "Type of error."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::RepositoryErrorType>,
    }
    impl ::field_selector::FieldSelector for RepositoryError {
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
    pub struct RequestOptions {
        #[doc = "Debug options of the request"]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier.\nFor translations.\n\nWhen specified, the documents in search results are biased towards the\nspecified language.\nSuggest API does not use this parameter. It autocompletes only based on\ncharacters in the query."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
        #[doc = "Id of the application created using SearchApplicationsService."]
        #[serde(rename = "searchApplicationId", default)]
        pub search_application_id: Option<String>,
        #[doc = "Current user's time zone id, such as \"America/Los_Angeles\" or\n\"Australia/Sydney\". These IDs are defined by\n[Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/)\nproject, and currently available in the file\n[timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml)"]
        #[serde(rename = "timeZone", default)]
        pub time_zone: Option<String>,
    }
    impl ::field_selector::FieldSelector for RequestOptions {
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
    pub struct ResetSearchApplicationRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
    }
    impl ::field_selector::FieldSelector for ResetSearchApplicationRequest {
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
    pub struct ResponseDebugInfo {
        #[doc = "General debug info formatted for display."]
        #[serde(rename = "formattedDebugInfo", default)]
        pub formatted_debug_info: Option<String>,
    }
    impl ::field_selector::FieldSelector for ResponseDebugInfo {
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
    pub struct RestrictItem {
        #[doc = "LINT.ThenChange(//depot/google3/java/com/google/apps/search/quality/itemsuggest/utils/SubtypeRerankingUtils.java)"]
        #[serde(rename = "driveFollowUpRestrict", default)]
        pub drive_follow_up_restrict: Option<crate::schemas::DriveFollowUpRestrict>,
        #[serde(rename = "driveLocationRestrict", default)]
        pub drive_location_restrict: Option<crate::schemas::DriveLocationRestrict>,
        #[doc = "LINT.IfChange\nDrive Types."]
        #[serde(rename = "driveMimeTypeRestrict", default)]
        pub drive_mime_type_restrict: Option<crate::schemas::DriveMimeTypeRestrict>,
        #[serde(rename = "driveTimeSpanRestrict", default)]
        pub drive_time_span_restrict: Option<crate::schemas::DriveTimeSpanRestrict>,
        #[serde(rename = "gmailActionRestrict", default)]
        pub gmail_action_restrict: Option<crate::schemas::GmailActionRestrict>,
        #[serde(rename = "gmailAttachmentRestrict", default)]
        pub gmail_attachment_restrict: Option<crate::schemas::GmailAttachmentRestrict>,
        #[doc = "Gmail Types."]
        #[serde(rename = "gmailFolderRestrict", default)]
        pub gmail_folder_restrict: Option<crate::schemas::GmailFolderRestrict>,
        #[serde(rename = "gmailIntelligentRestrict", default)]
        pub gmail_intelligent_restrict: Option<crate::schemas::GmailIntelligentRestrict>,
        #[serde(rename = "gmailTimeRestrict", default)]
        pub gmail_time_restrict: Option<crate::schemas::GmailTimeRestrict>,
        #[doc = "The search restrict (e.g. \"after:2017-09-11 before:2017-09-12\")."]
        #[serde(rename = "searchOperator", default)]
        pub search_operator: Option<String>,
    }
    impl ::field_selector::FieldSelector for RestrictItem {
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
    pub struct ResultCounts {
        #[doc = "Result count information for each source with results."]
        #[serde(rename = "sourceResultCounts", default)]
        pub source_result_counts: Option<Vec<crate::schemas::SourceResultCount>>,
    }
    impl ::field_selector::FieldSelector for ResultCounts {
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
    pub struct ResultDebugInfo {
        #[doc = "General debug info formatted for display."]
        #[serde(rename = "formattedDebugInfo", default)]
        pub formatted_debug_info: Option<String>,
    }
    impl ::field_selector::FieldSelector for ResultDebugInfo {
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
    pub struct ResultDisplayField {
        #[doc = "The display label for the property."]
        #[serde(rename = "label", default)]
        pub label: Option<String>,
        #[doc = "The operator name of the property."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
        #[doc = "The name value pair for the property."]
        #[serde(rename = "property", default)]
        pub property: Option<crate::schemas::NamedProperty>,
    }
    impl ::field_selector::FieldSelector for ResultDisplayField {
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
    pub struct ResultDisplayLine {
        #[serde(rename = "fields", default)]
        pub fields: Option<Vec<crate::schemas::ResultDisplayField>>,
    }
    impl ::field_selector::FieldSelector for ResultDisplayLine {
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
    pub struct ResultDisplayMetadata {
        #[doc = "The metalines content to be displayed with the result."]
        #[serde(rename = "metalines", default)]
        pub metalines: Option<Vec<crate::schemas::ResultDisplayLine>>,
        #[doc = "The display label for the object."]
        #[serde(rename = "objectTypeLabel", default)]
        pub object_type_label: Option<String>,
    }
    impl ::field_selector::FieldSelector for ResultDisplayMetadata {
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
    pub enum RetrievalImportanceImportance {
        #[doc = "Treat the match like a body text match."]
        Default,
        #[doc = "Treat the match like a match against title of the item."]
        Highest,
        #[doc = "Treat the match with higher importance than body text."]
        High,
        #[doc = "Treat the match with lower importance than body text."]
        Low,
        #[doc = "Do not match against this field during retrieval. The property can still\nbe used for operator matching, faceting, and suggest if\ndesired."]
        None,
    }
    impl RetrievalImportanceImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                RetrievalImportanceImportance::Default => "DEFAULT",
                RetrievalImportanceImportance::Highest => "HIGHEST",
                RetrievalImportanceImportance::High => "HIGH",
                RetrievalImportanceImportance::Low => "LOW",
                RetrievalImportanceImportance::None => "NONE",
            }
        }
    }
    impl ::std::fmt::Display for RetrievalImportanceImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RetrievalImportanceImportance {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RetrievalImportanceImportance {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => RetrievalImportanceImportance::Default,
                "HIGHEST" => RetrievalImportanceImportance::Highest,
                "HIGH" => RetrievalImportanceImportance::High,
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
    #[derive(
        Debug,
        Clone,
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
        pub importance: Option<crate::schemas::RetrievalImportanceImportance>,
    }
    impl ::field_selector::FieldSelector for RetrievalImportance {
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
    pub struct Schema {
        #[doc = "The list of top-level objects for the data source.\nThe maximum number of elements is 10."]
        #[serde(rename = "objectDefinitions", default)]
        pub object_definitions: Option<Vec<crate::schemas::ObjectDefinition>>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this\nschema. After modifying the schema, wait for operations to complete\nbefore indexing additional content."]
        #[serde(rename = "operationIds", default)]
        pub operation_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Schema {
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
    pub struct ScoringConfig {
        #[doc = "Whether to use freshness as a ranking signal. By default, freshness is used\nas a ranking signal. Note that this setting is not available in the Admin\nUI."]
        #[serde(rename = "disableFreshness", default)]
        pub disable_freshness: Option<bool>,
        #[doc = "Whether to personalize the results. By default, personal signals will\nbe used to boost results."]
        #[serde(rename = "disablePersonalization", default)]
        pub disable_personalization: Option<bool>,
    }
    impl ::field_selector::FieldSelector for ScoringConfig {
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
    pub struct SearchApplication {
        #[doc = "Retrictions applied to the configurations.\nThe maximum number of elements is 10."]
        #[serde(rename = "dataSourceRestrictions", default)]
        pub data_source_restrictions: Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[doc = "The default fields for returning facet results.\nThe sources specified here also have been included in\ndata_source_restrictions\nabove."]
        #[serde(rename = "defaultFacetOptions", default)]
        pub default_facet_options: Option<Vec<crate::schemas::FacetOptions>>,
        #[doc = "The default options for sorting the search results"]
        #[serde(rename = "defaultSortOptions", default)]
        pub default_sort_options: Option<crate::schemas::SortOptions>,
        #[doc = "Display name of the Search Application.\nThe maximum length is 300 characters."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "Name of the Search Application.\n<br />Format: searchapplications/{application_id}."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this\nschema. Output only field."]
        #[serde(rename = "operationIds", default)]
        pub operation_ids: Option<Vec<String>>,
        #[doc = "Configuration for ranking results."]
        #[serde(rename = "scoringConfig", default)]
        pub scoring_config: Option<crate::schemas::ScoringConfig>,
        #[doc = "Configuration for a sources specified in data_source_restrictions."]
        #[serde(rename = "sourceConfig", default)]
        pub source_config: Option<Vec<crate::schemas::SourceConfig>>,
    }
    impl ::field_selector::FieldSelector for SearchApplication {
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
    pub struct SearchItemsByViewUrlRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[doc = "The next_page_token value returned from a previous request, if any."]
        #[serde(rename = "pageToken", default)]
        pub page_token: Option<String>,
        #[doc = "Specify the full view URL to find the corresponding item.\nThe maximum length is 2048 characters."]
        #[serde(rename = "viewUrl", default)]
        pub view_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for SearchItemsByViewUrlRequest {
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
    pub struct SearchItemsByViewUrlResponse {
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Item>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no\nmore results in the list."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for SearchItemsByViewUrlResponse {
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
    pub struct SearchQualityMetadata {
        #[doc = "An indication of the quality of the item, used to influence search quality.\nValue should be between 0.0 (lowest quality) and 1.0 (highest quality). The\ndefault value is 0.0."]
        #[serde(rename = "quality", default)]
        pub quality: Option<f64>,
    }
    impl ::field_selector::FieldSelector for SearchQualityMetadata {
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
    pub struct SearchRequest {
        #[doc = "The sources to use for querying. If not specified, all data sources\nfrom the current search application are used."]
        #[serde(rename = "dataSourceRestrictions", default)]
        pub data_source_restrictions: Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[serde(rename = "facetOptions", default)]
        pub facet_options: Option<Vec<crate::schemas::FacetOptions>>,
        #[doc = "Maximum number of search results to return in one page.\nValid values are between 1 and 100, inclusive.\nDefault value is 10."]
        #[serde(rename = "pageSize", default)]
        pub page_size: Option<i32>,
        #[doc = "The raw query string.\nSee supported search operators in the [Cloud search\nCheat\nSheet](https://gsuite.google.com/learning-center/products/cloudsearch/cheat-sheet/)"]
        #[serde(rename = "query", default)]
        pub query: Option<String>,
        #[doc = "Options to interpret the user query."]
        #[serde(rename = "queryInterpretationOptions", default)]
        pub query_interpretation_options: Option<crate::schemas::QueryInterpretationOptions>,
        #[doc = "Request options, such as the search application and user timezone."]
        #[serde(rename = "requestOptions", default)]
        pub request_options: Option<crate::schemas::RequestOptions>,
        #[doc = "The options for sorting the search results"]
        #[serde(rename = "sortOptions", default)]
        pub sort_options: Option<crate::schemas::SortOptions>,
        #[doc = "Starting index of the results."]
        #[serde(rename = "start", default)]
        pub start: Option<i32>,
    }
    impl ::field_selector::FieldSelector for SearchRequest {
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
    pub struct SearchResponse {
        #[doc = "Debugging information about the response."]
        #[serde(rename = "debugInfo", default)]
        pub debug_info: Option<crate::schemas::ResponseDebugInfo>,
        #[doc = "Error information about the response."]
        #[serde(rename = "errorInfo", default)]
        pub error_info: Option<crate::schemas::ErrorInfo>,
        #[doc = "Repeated facet results."]
        #[serde(rename = "facetResults", default)]
        pub facet_results: Option<Vec<crate::schemas::FacetResult>>,
        #[doc = "Whether there are more search results matching the query."]
        #[serde(rename = "hasMoreResults", default)]
        pub has_more_results: Option<bool>,
        #[doc = "Query interpretation result for user query. Empty if query interpretation\nis disabled."]
        #[serde(rename = "queryInterpretation", default)]
        pub query_interpretation: Option<crate::schemas::QueryInterpretation>,
        #[doc = "The estimated result count for this query."]
        #[serde(rename = "resultCountEstimate", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_estimate: Option<i64>,
        #[doc = "The exact result count for this query."]
        #[serde(rename = "resultCountExact", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_exact: Option<i64>,
        #[doc = "Expanded result count information."]
        #[serde(rename = "resultCounts", default)]
        pub result_counts: Option<crate::schemas::ResultCounts>,
        #[doc = "Results from a search query."]
        #[serde(rename = "results", default)]
        pub results: Option<Vec<crate::schemas::SearchResult>>,
        #[doc = "Suggested spelling for the query."]
        #[serde(rename = "spellResults", default)]
        pub spell_results: Option<Vec<crate::schemas::SpellResult>>,
        #[doc = "Structured results for the user query. These results are not counted\nagainst the page_size."]
        #[serde(rename = "structuredResults", default)]
        pub structured_results: Option<Vec<crate::schemas::StructuredResult>>,
    }
    impl ::field_selector::FieldSelector for SearchResponse {
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
    pub struct SearchResult {
        #[doc = "If source is clustered, provide list of clustered results. There will only\nbe one level of clustered results. If current source is not enabled for\nclustering, this field will be empty."]
        #[serde(rename = "clusteredResults", default)]
        pub clustered_results: Option<Vec<crate::schemas::SearchResult>>,
        #[doc = "Debugging information about this search result."]
        #[serde(rename = "debugInfo", default)]
        pub debug_info: Option<crate::schemas::ResultDebugInfo>,
        #[doc = "Metadata of the search result."]
        #[serde(rename = "metadata", default)]
        pub metadata: Option<crate::schemas::Metadata>,
        #[doc = "The concatenation of all snippets (summaries) available for this result."]
        #[serde(rename = "snippet", default)]
        pub snippet: Option<crate::schemas::Snippet>,
        #[doc = "Title of the search result."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[doc = "The URL of the search result. The URL contains a Google redirect to the\nactual item. This URL is signed and shouldn't be changed."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for SearchResult {
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
    pub struct Snippet {
        #[doc = "The matched ranges in the snippet."]
        #[serde(rename = "matchRanges", default)]
        pub match_ranges: Option<Vec<crate::schemas::MatchRange>>,
        #[doc = "The snippet of the document.\nThe snippet of the document. May contain escaped HTML character that\nshould be unescaped prior to rendering."]
        #[serde(rename = "snippet", default)]
        pub snippet: Option<String>,
    }
    impl ::field_selector::FieldSelector for Snippet {
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SortOptionsSortOrder {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SortOptionsSortOrder {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    #[derive(
        Debug,
        Clone,
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
        pub operator_name: Option<String>,
        #[doc = "Ascending is the default sort order"]
        #[serde(rename = "sortOrder", default)]
        pub sort_order: Option<crate::schemas::SortOptionsSortOrder>,
    }
    impl ::field_selector::FieldSelector for SortOptions {
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
    pub enum SourcePredefinedSource {
        None,
        #[doc = "Suggests queries issued by the user in the past. Only valid when used\nwith the suggest API. Ignored when used in the query API."]
        QueryHistory,
        #[doc = "Suggests people in the organization. Only valid when used\nwith the suggest API. Results in an error when used in the query API."]
        Person,
        GoogleDrive,
        GoogleGmail,
        GoogleSites,
        GoogleGroups,
        GoogleCalendar,
        GoogleKeep,
    }
    impl SourcePredefinedSource {
        pub fn as_str(self) -> &'static str {
            match self {
                SourcePredefinedSource::None => "NONE",
                SourcePredefinedSource::QueryHistory => "QUERY_HISTORY",
                SourcePredefinedSource::Person => "PERSON",
                SourcePredefinedSource::GoogleDrive => "GOOGLE_DRIVE",
                SourcePredefinedSource::GoogleGmail => "GOOGLE_GMAIL",
                SourcePredefinedSource::GoogleSites => "GOOGLE_SITES",
                SourcePredefinedSource::GoogleGroups => "GOOGLE_GROUPS",
                SourcePredefinedSource::GoogleCalendar => "GOOGLE_CALENDAR",
                SourcePredefinedSource::GoogleKeep => "GOOGLE_KEEP",
            }
        }
    }
    impl ::std::fmt::Display for SourcePredefinedSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourcePredefinedSource {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourcePredefinedSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => SourcePredefinedSource::None,
                "QUERY_HISTORY" => SourcePredefinedSource::QueryHistory,
                "PERSON" => SourcePredefinedSource::Person,
                "GOOGLE_DRIVE" => SourcePredefinedSource::GoogleDrive,
                "GOOGLE_GMAIL" => SourcePredefinedSource::GoogleGmail,
                "GOOGLE_SITES" => SourcePredefinedSource::GoogleSites,
                "GOOGLE_GROUPS" => SourcePredefinedSource::GoogleGroups,
                "GOOGLE_CALENDAR" => SourcePredefinedSource::GoogleCalendar,
                "GOOGLE_KEEP" => SourcePredefinedSource::GoogleKeep,
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
    pub struct Source {
        #[doc = "Source name for content indexed by the\nIndexing API."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Predefined content source for Google Apps."]
        #[serde(rename = "predefinedSource", default)]
        pub predefined_source: Option<crate::schemas::SourcePredefinedSource>,
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
    pub struct SourceConfig {
        #[doc = "The crowding configuration for the source."]
        #[serde(rename = "crowdingConfig", default)]
        pub crowding_config: Option<crate::schemas::SourceCrowdingConfig>,
        #[doc = "The scoring configuration for the source."]
        #[serde(rename = "scoringConfig", default)]
        pub scoring_config: Option<crate::schemas::SourceScoringConfig>,
        #[doc = "The source for which this configuration is to be used."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceConfig {
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
    pub struct SourceCrowdingConfig {
        #[doc = "Maximum number of results allowed from a source.\nNo limits will be set on results if this value is less than or equal to 0."]
        #[serde(rename = "numResults", default)]
        pub num_results: Option<i32>,
        #[doc = "Maximum number of suggestions allowed from a source.\nNo limits will be set on results if this value is less than or equal to 0."]
        #[serde(rename = "numSuggestions", default)]
        pub num_suggestions: Option<i32>,
    }
    impl ::field_selector::FieldSelector for SourceCrowdingConfig {
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
    pub struct SourceResultCount {
        #[doc = "Whether there are more search results for this source."]
        #[serde(rename = "hasMoreResults", default)]
        pub has_more_results: Option<bool>,
        #[doc = "The estimated result count for this source."]
        #[serde(rename = "resultCountEstimate", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_estimate: Option<i64>,
        #[doc = "The exact result count for this source."]
        #[serde(rename = "resultCountExact", default)]
        #[serde(with = "crate::parsed_string")]
        pub result_count_exact: Option<i64>,
        #[doc = "The source the result count information is associated with."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceResultCount {
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
    pub enum SourceScoringConfigSourceImportance {
        Default,
        Low,
        High,
    }
    impl SourceScoringConfigSourceImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceScoringConfigSourceImportance::Default => "DEFAULT",
                SourceScoringConfigSourceImportance::Low => "LOW",
                SourceScoringConfigSourceImportance::High => "HIGH",
            }
        }
    }
    impl ::std::fmt::Display for SourceScoringConfigSourceImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceScoringConfigSourceImportance {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceScoringConfigSourceImportance {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => SourceScoringConfigSourceImportance::Default,
                "LOW" => SourceScoringConfigSourceImportance::Low,
                "HIGH" => SourceScoringConfigSourceImportance::High,
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
    pub struct SourceScoringConfig {
        #[doc = "Importance of the source."]
        #[serde(rename = "sourceImportance", default)]
        pub source_importance: Option<crate::schemas::SourceScoringConfigSourceImportance>,
    }
    impl ::field_selector::FieldSelector for SourceScoringConfig {
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
    pub struct SpellResult {
        #[doc = "The suggested spelling of the query."]
        #[serde(rename = "suggestedQuery", default)]
        pub suggested_query: Option<String>,
    }
    impl ::field_selector::FieldSelector for SpellResult {
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
    pub struct StartUploadItemRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
    }
    impl ::field_selector::FieldSelector for StartUploadItemRequest {
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
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StructuredDataObject {
        #[doc = "The properties for the object.\nThe maximum number of elements is 1000."]
        #[serde(rename = "properties", default)]
        pub properties: Option<Vec<crate::schemas::NamedProperty>>,
    }
    impl ::field_selector::FieldSelector for StructuredDataObject {
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
    pub struct StructuredResult {
        #[doc = "Representation of a person"]
        #[serde(rename = "person", default)]
        pub person: Option<crate::schemas::Person>,
    }
    impl ::field_selector::FieldSelector for StructuredResult {
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
    pub struct SuggestRequest {
        #[doc = "The sources to use for suggestions. If not specified, all data sources\nfrom the current search application are used.\nSuggestions are based on Gmail titles. Suggestions from third party sources\nare not available."]
        #[serde(rename = "dataSourceRestrictions", default)]
        pub data_source_restrictions: Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[doc = "Partial query for which autocomplete suggestions will be shown.\nFor example, if the query is \"sea\", then the server might return\n\"season\", \"search\", \"seagull\" and so on."]
        #[serde(rename = "query", default)]
        pub query: Option<String>,
        #[doc = "Request options, such as the search application and user timezone."]
        #[serde(rename = "requestOptions", default)]
        pub request_options: Option<crate::schemas::RequestOptions>,
    }
    impl ::field_selector::FieldSelector for SuggestRequest {
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
    pub struct SuggestResponse {
        #[doc = "List of suggestions."]
        #[serde(rename = "suggestResults", default)]
        pub suggest_results: Option<Vec<crate::schemas::SuggestResult>>,
    }
    impl ::field_selector::FieldSelector for SuggestResponse {
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
    pub struct SuggestResult {
        #[doc = "This is present when the suggestion indicates a person. It\ncontains more information about the person - like their email ID,\nname etc."]
        #[serde(rename = "peopleSuggestion", default)]
        pub people_suggestion: Option<crate::schemas::PeopleSuggestion>,
        #[doc = "This field will be present if the suggested query is a word/phrase\ncompletion."]
        #[serde(rename = "querySuggestion", default)]
        pub query_suggestion: Option<crate::schemas::QuerySuggestion>,
        #[doc = "The source of the suggestion."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::Source>,
        #[doc = "The suggested query that will be used for search, when the user\nclicks on the suggestion"]
        #[serde(rename = "suggestedQuery", default)]
        pub suggested_query: Option<String>,
    }
    impl ::field_selector::FieldSelector for SuggestResult {
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
    pub struct TextOperatorOptions {
        #[doc = "If true, the text value will be tokenized as one atomic value in\noperator searches and facet matches. For example, if the operator name is\n\"genre\" and the value is \"science-fiction\" the query restrictions\n\"genre:science\" and \"genre:fiction\" will not match the item;\n\"genre:science-fiction\" will. Value matching is case-sensitive\nand does not remove special characters.\nIf false, the text will be tokenized. For example, if the value is\n\"science-fiction\" the queries \"genre:science\" and \"genre:fiction\" will\nmatch the item."]
        #[serde(rename = "exactMatchWithOperator", default)]
        pub exact_match_with_operator: Option<bool>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntext property. For example, if operatorName is *subject* and the\nproperty's name is *subjectLine*, then queries like\n*subject:&lt;value&gt;* will show results only where the value of the\nproperty named *subjectLine* matches *&lt;value&gt;*. By contrast, a\nsearch that uses the same *&lt;value&gt;* without an operator will return\nall items where *&lt;value&gt;* matches the value of any\ntext properties or text within the content field for the item.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for TextOperatorOptions {
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
    pub struct TextPropertyOptions {
        #[doc = "If set, describes how the property should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::TextOperatorOptions>,
        #[doc = "Indicates the search quality importance of the tokens within the\nfield when used for retrieval."]
        #[serde(rename = "retrievalImportance", default)]
        pub retrieval_importance: Option<crate::schemas::RetrievalImportance>,
    }
    impl ::field_selector::FieldSelector for TextPropertyOptions {
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
    pub struct TextValues {
        #[doc = "The maximum allowable length for text values is 2048 characters."]
        #[serde(rename = "values", default)]
        pub values: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TextValues {
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
    pub struct TimestampOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntimestamp property using the greater-than operator. For example, if\ngreaterThanOperatorName is *closedafter* and the property's name is\n*closeDate*, then queries like *closedafter:&lt;value&gt;* will\nshow results only where the value of the property named *closeDate* is\nlater than *&lt;value&gt;*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "greaterThanOperatorName", default)]
        pub greater_than_operator_name: Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntimestamp property using the less-than operator. For example, if\nlessThanOperatorName is *closedbefore* and the property's name is\n*closeDate*, then queries like *closedbefore:&lt;value&gt;* will\nshow results only where the value of the property named *closeDate* is\nearlier than *&lt;value&gt;*.\nThe operator name can only contain lowercase letters (a-z).\nThe maximum length is 32 characters."]
        #[serde(rename = "lessThanOperatorName", default)]
        pub less_than_operator_name: Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the\ntimestamp property. For example, if operatorName is *closedon* and the\nproperty's name is *closeDate*, then queries like\n*closedon:&lt;value&gt;* will show results only where the value of the\nproperty named *closeDate* matches *&lt;value&gt;*. By contrast, a\nsearch that uses the same *&lt;value&gt;* without an operator will return\nall items where *&lt;value&gt;* matches the value of any String\nproperties or text within the content field for the item. The operator\nname can only contain lowercase letters (a-z). The maximum length is 32\ncharacters."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for TimestampOperatorOptions {
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
    pub struct TimestampPropertyOptions {
        #[doc = "If set, describes how the timestamp should be used as a search operator."]
        #[serde(rename = "operatorOptions", default)]
        pub operator_options: Option<crate::schemas::TimestampOperatorOptions>,
    }
    impl ::field_selector::FieldSelector for TimestampPropertyOptions {
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
    pub struct TimestampValues {
        #[serde(rename = "values", default)]
        pub values: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TimestampValues {
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
    pub enum UnmappedIdentityResolutionStatusCode {
        #[doc = "Input-only value.  Used to list all unmapped identities regardless of\nstatus."]
        CodeUnspecified,
        #[doc = "The unmapped identity was not found in IDaaS, and needs to be provided by\nthe user."]
        NotFound,
        #[doc = "The identity source associated with the identity was either not found or\ndeleted."]
        IdentitySourceNotFound,
        #[doc = "IDaaS does not understand the identity source, probably because the\nschema was modified in a non compatible way."]
        IdentitySourceMisconfigured,
        #[doc = "The number of users associated with the external identity is too large."]
        TooManyMappingsFound,
        #[doc = "Internal error."]
        InternalError,
    }
    impl UnmappedIdentityResolutionStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                UnmappedIdentityResolutionStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                UnmappedIdentityResolutionStatusCode::NotFound => "NOT_FOUND",
                UnmappedIdentityResolutionStatusCode::IdentitySourceNotFound => {
                    "IDENTITY_SOURCE_NOT_FOUND"
                }
                UnmappedIdentityResolutionStatusCode::IdentitySourceMisconfigured => {
                    "IDENTITY_SOURCE_MISCONFIGURED"
                }
                UnmappedIdentityResolutionStatusCode::TooManyMappingsFound => {
                    "TOO_MANY_MAPPINGS_FOUND"
                }
                UnmappedIdentityResolutionStatusCode::InternalError => "INTERNAL_ERROR",
            }
        }
    }
    impl ::std::fmt::Display for UnmappedIdentityResolutionStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UnmappedIdentityResolutionStatusCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UnmappedIdentityResolutionStatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => UnmappedIdentityResolutionStatusCode::CodeUnspecified,
                "NOT_FOUND" => UnmappedIdentityResolutionStatusCode::NotFound,
                "IDENTITY_SOURCE_NOT_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceNotFound
                }
                "IDENTITY_SOURCE_MISCONFIGURED" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceMisconfigured
                }
                "TOO_MANY_MAPPINGS_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::TooManyMappingsFound
                }
                "INTERNAL_ERROR" => UnmappedIdentityResolutionStatusCode::InternalError,
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
    pub struct UnmappedIdentity {
        #[doc = "The resource name for an external user."]
        #[serde(rename = "externalIdentity", default)]
        pub external_identity: Option<crate::schemas::Principal>,
        #[doc = "The resolution status for the external identity."]
        #[serde(rename = "resolutionStatusCode", default)]
        pub resolution_status_code: Option<crate::schemas::UnmappedIdentityResolutionStatusCode>,
    }
    impl ::field_selector::FieldSelector for UnmappedIdentity {
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
    pub struct UnreserveItemsRequest {
        #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(rename = "connectorName", default)]
        pub connector_name: Option<String>,
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[doc = "Name of a queue to unreserve items from."]
        #[serde(rename = "queue", default)]
        pub queue: Option<String>,
    }
    impl ::field_selector::FieldSelector for UnreserveItemsRequest {
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
    pub struct UpdateDataSourceRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::DataSource>,
    }
    impl ::field_selector::FieldSelector for UpdateDataSourceRequest {
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
    pub struct UpdateSchemaRequest {
        #[doc = "Common debug options."]
        #[serde(rename = "debugOptions", default)]
        pub debug_options: Option<crate::schemas::DebugOptions>,
        #[doc = "The new schema for the source."]
        #[serde(rename = "schema", default)]
        pub schema: Option<crate::schemas::Schema>,
        #[doc = "If true, the request will be validated without side effects."]
        #[serde(rename = "validateOnly", default)]
        pub validate_only: Option<bool>,
    }
    impl ::field_selector::FieldSelector for UpdateSchemaRequest {
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
    pub struct UploadItemRef {
        #[doc = "Name of the content reference.\nThe maximum length is 2048 characters."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for UploadItemRef {
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
    pub struct Value {
        #[serde(rename = "booleanValue", default)]
        pub boolean_value: Option<bool>,
        #[serde(rename = "dateValue", default)]
        pub date_value: Option<crate::schemas::Date>,
        #[serde(rename = "doubleValue", default)]
        pub double_value: Option<f64>,
        #[serde(rename = "integerValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub integer_value: Option<i64>,
        #[serde(rename = "stringValue", default)]
        pub string_value: Option<String>,
        #[serde(rename = "timestampValue", default)]
        pub timestamp_value: Option<String>,
    }
    impl ::field_selector::FieldSelector for Value {
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
    pub struct ValueFilter {
        #[doc = "The `operator_name` applied to the query, such as *price_greater_than*.\nThe filter can work against both types of filters defined in the schema\nfor your data source:\n<br/><br/>\n1. `operator_name`, where the query filters results by the property\nthat matches the value.\n<br/>\n2. `greater_than_operator_name` or `less_than_operator_name` in your\nschema. The query filters the results for the property values that are\ngreater than or less than  the supplied value in the query."]
        #[serde(rename = "operatorName", default)]
        pub operator_name: Option<String>,
        #[doc = "The value to be compared with."]
        #[serde(rename = "value", default)]
        pub value: Option<crate::schemas::Value>,
    }
    impl ::field_selector::FieldSelector for ValueFilter {
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
    #[doc = "Actions that can be performed on the debug resource"]
    pub fn debug(&self) -> crate::debug::DebugActions<A> {
        crate::debug::DebugActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the indexing resource"]
    pub fn indexing(&self) -> crate::indexing::IndexingActions<A> {
        crate::indexing::IndexingActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the media resource"]
    pub fn media(&self) -> crate::media::MediaActions<A> {
        crate::media::MediaActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::operations::OperationsActions<A> {
        crate::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the query resource"]
    pub fn query(&self) -> crate::query::QueryActions<A> {
        crate::query::QueryActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the settings resource"]
    pub fn settings(&self) -> crate::settings::SettingsActions<A> {
        crate::settings::SettingsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the stats resource"]
    pub fn stats(&self) -> crate::stats::StatsActions<A> {
        crate::stats::StatsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod debug {
    pub mod params {}
    pub struct DebugActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> DebugActions<'a, A> {
        #[doc = "Actions that can be performed on the datasources resource"]
        pub fn datasources(&self) -> datasources::DatasourcesActions<A> {
            datasources::DatasourcesActions
        }
        #[doc = "Actions that can be performed on the identitysources resource"]
        pub fn identitysources(&self) -> identitysources::IdentitysourcesActions<A> {
            identitysources::IdentitysourcesActions
        }
    }
    pub mod datasources {
        pub mod params {}
        pub struct DatasourcesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> DatasourcesActions<'a, A> {
            #[doc = "Actions that can be performed on the items resource"]
            pub fn items(&self) -> items::ItemsActions<A> {
                items::ItemsActions
            }
        }
        pub mod items {
            pub mod params {}
            pub struct ItemsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> ItemsActions<'a, A> {
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
                #[doc = "Actions that can be performed on the unmappedids resource"]
                pub fn unmappedids(&self) -> unmappedids::UnmappedidsActions<A> {
                    unmappedids::UnmappedidsActions
                }
            }
            #[derive(Debug, Clone)]
            pub struct CheckAccessRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> CheckAccessRequestBuilder<'a, A> {
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::CheckAccessResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/debug/");
                    output.push_str(&self.name);
                    output.push_str(":checkAccess");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct SearchByViewUrlRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> SearchByViewUrlRequestBuilder<'a, A> {
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
                    crate::schemas::SearchItemsByViewUrlResponse,
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/debug/");
                    output.push_str(&self.name);
                    output.push_str("/items:searchByViewUrl");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            pub mod unmappedids {
                pub mod params {}
                pub struct UnmappedidsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> UnmappedidsActions<'a, A> {
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
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = "Maximum number of items to fetch in a request.\nDefaults to 100."]
                    pub fn page_size(&mut self, value: i32) -> &mut Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
                    pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                        self.page_token = Some(value.into());
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
                    pub fn iter_unmapped_identities<T>(
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
                                    #[serde(rename = "unmappedIdentities")]
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
                    ) -> Result<
                        crate::schemas::ListUnmappedIdentitiesResponse,
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
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        output.push_str(&self.parent);
                        output.push_str("/unmappedids");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/cloud_search",
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
        }
    }
    pub mod identitysources {
        pub mod params {}
        pub struct IdentitysourcesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> IdentitysourcesActions<'a, A> {
            #[doc = "Actions that can be performed on the items resource"]
            pub fn items(&self) -> items::ItemsActions<A> {
                items::ItemsActions
            }
            #[doc = "Actions that can be performed on the unmappedids resource"]
            pub fn unmappedids(&self) -> unmappedids::UnmappedidsActions<A> {
                unmappedids::UnmappedidsActions
            }
        }
        pub mod items {
            pub mod params {}
            pub struct ItemsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> ItemsActions<'a, A> {
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
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> ListForunmappedidentityRequestBuilder<'a, A> {
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = ""]
                pub fn group_resource_name(&mut self, value: impl Into<String>) -> &mut Self {
                    self.group_resource_name = Some(value.into());
                    self
                }
                #[doc = "Maximum number of items to fetch in a request.\nDefaults to 100."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The next_page_token value returned from a previous List request, if any."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = ""]
                pub fn user_resource_name(&mut self, value: impl Into<String>) -> &mut Self {
                    self.user_resource_name = Some(value.into());
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
                pub fn iter_item_names<T>(
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
                                #[serde(rename = "itemNames")]
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
                ) -> Result<
                    crate::schemas::ListItemNamesForUnmappedIdentityResponse,
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/debug/");
                    output.push_str(&self.parent);
                    output.push_str("/items:forunmappedidentity");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod
                for ListForunmappedidentityRequestBuilder<'a, A>
            {
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
        pub mod unmappedids {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListResolutionStatusCode {}
                impl ListResolutionStatusCode {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for ListResolutionStatusCode {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListResolutionStatusCode {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListResolutionStatusCode {
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
            pub struct UnmappedidsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> UnmappedidsActions<'a, A> {
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
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                parent: String,
                debug_options_enable_debugging: Option<bool>,
                page_size: Option<i32>,
                page_token: Option<String>,
                resolution_status_code:
                    Option<crate::unmappedids::params::ListResolutionStatusCode>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "Maximum number of items to fetch in a request.\nDefaults to 100."]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The next_page_token value returned from a previous List request, if any."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Limit users selection to this status."]
                pub fn resolution_status_code(
                    &mut self,
                    value: crate::unmappedids::params::ListResolutionStatusCode,
                ) -> &mut Self {
                    self.resolution_status_code = Some(value);
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
                pub fn iter_unmapped_identities<T>(
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
                                #[serde(rename = "unmappedIdentities")]
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
                ) -> Result<
                    crate::schemas::ListUnmappedIdentitiesResponse,
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/debug/");
                    output.push_str(&self.parent);
                    output.push_str("/unmappedids");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("resolutionStatusCode", &self.resolution_status_code)]);
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
    }
}
pub mod indexing {
    pub mod params {}
    pub struct IndexingActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> IndexingActions<'a, A> {
        #[doc = "Actions that can be performed on the datasources resource"]
        pub fn datasources(&self) -> datasources::DatasourcesActions<A> {
            datasources::DatasourcesActions
        }
    }
    pub mod datasources {
        pub mod params {}
        pub struct DatasourcesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> DatasourcesActions<'a, A> {
            #[doc = "Deletes the schema of a data source."]
            pub fn delete_schema(&self, name: impl Into<String>) -> DeleteSchemaRequestBuilder<A> {
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
            pub fn items(&self) -> items::ItemsActions<A> {
                items::ItemsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteSchemaRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> DeleteSchemaRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/indexing/");
                output.push_str(&self.name);
                output.push_str("/schema");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetSchemaRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> GetSchemaRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
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
            ) -> Result<crate::schemas::Schema, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/indexing/");
                output.push_str(&self.name);
                output.push_str("/schema");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateSchemaRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> UpdateSchemaRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/indexing/");
                output.push_str(&self.name);
                output.push_str("/schema");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        pub mod items {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum DeleteMode {}
                impl DeleteMode {
                    pub fn as_str(self) -> &'static str {
                        match self {}
                    }
                }
                impl ::std::fmt::Display for DeleteMode {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for DeleteMode {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for DeleteMode {
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
            pub struct ItemsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> ItemsActions<'a, A> {
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
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
                connector_name: Option<String>,
                debug_options_enable_debugging: Option<bool>,
                mode: Option<crate::items::params::DeleteMode>,
                version: Option<Vec<u8>>,
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
                #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
                pub fn connector_name(&mut self, value: impl Into<String>) -> &mut Self {
                    self.connector_name = Some(value.into());
                    self
                }
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "Required. The RequestMode for this request."]
                pub fn mode(&mut self, value: crate::items::params::DeleteMode) -> &mut Self {
                    self.mode = Some(value);
                    self
                }
                #[doc = "Required. The incremented version of the item to delete from the index.\nThe indexing system stores the version from the datasource as a\nbyte string and compares the Item version in the index\nto the version of the queued Item using lexical ordering.\n<br /><br />\nCloud Search Indexing won't delete any queued item with\na version value that is less than or equal to\nthe version of the currently indexed item.\nThe maximum length for this field is 1024 bytes."]
                pub fn version(&mut self, value: Vec<u8>) -> &mut Self {
                    self.version = Some(value);
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
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteQueueItemsRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> DeleteQueueItemsRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output.push_str("/items:deleteQueueItems");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
                pub fn connector_name(&mut self, value: impl Into<String>) -> &mut Self {
                    self.connector_name = Some(value.into());
                    self
                }
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::Item, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct IndexRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> IndexRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output.push_str(":index");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "When set to true, the indexing system only populates the following fields:\nname,\nversion,\nmetadata.hash,\nstructured_data.hash,\ncontent.hash.\n<br />If this value is false, then all the fields are populated in Item."]
                pub fn brief(&mut self, value: bool) -> &mut Self {
                    self.brief = Some(value);
                    self
                }
                #[doc = "Name of connector making this call.\n<br />Format: datasources/{source_id}/connectors/{ID}"]
                pub fn connector_name(&mut self, value: impl Into<String>) -> &mut Self {
                    self.connector_name = Some(value.into());
                    self
                }
                #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
                pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "Maximum number of items to fetch in a request.\nThe max value is 1000 when brief is true.  The max value is 10 if brief\nis false.\n<br />The default value is 10"]
                pub fn page_size(&mut self, value: i32) -> &mut Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The next_page_token value returned from a previous List request, if any."]
                pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                    self.page_token = Some(value.into());
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
                pub fn iter_items<T>(
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
                                #[serde(rename = "items")]
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
                ) -> Result<crate::schemas::ListItemsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output.push_str("/items");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
            pub struct PollRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> PollRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::PollItemsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output.push_str("/items:poll");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct PushRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> PushRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Item, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output.push_str(":push");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct UnreserveRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> UnreserveRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output.push_str("/items:unreserve");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct UploadRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> UploadRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::UploadItemRef, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    output.push_str(&self.name);
                    output.push_str(":upload");
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
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
            }
        }
    }
}
pub mod media {
    pub mod params {}
    pub struct MediaActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> MediaActions<'a, A> {
        #[doc = "Uploads media for indexing.\n\nThe upload endpoint supports direct and resumable upload protocols and\nis intended for large items that can not be\n[inlined during index requests](https://developers.google.com/cloud-search/docs/reference/rest/v1/indexing.datasources.items#itemcontent).\nTo index large content:\n\n1. Call\n   indexing.datasources.items.upload\n   with the resource name to begin an upload session and retrieve the\n   UploadItemRef.\n1. Call media.upload to upload the content using the same resource name from step 1.\n1. Call indexing.datasources.items.index\n   to index the item. Populate the\n   [ItemContent](/cloud-search/docs/reference/rest/v1/indexing.datasources.items#ItemContent)\n   with the UploadItemRef from step 1.\n\n\nFor additional information, see\n[Create a content connector using the REST API](https://developers.google.com/cloud-search/docs/guides/content-connector#rest)."]
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
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
    impl<'a, A: yup_oauth2::GetToken> UploadRequestBuilder<'a, A> {
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
        fn _simple_upload_path(&self) -> String {
            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
            output.push_str("upload/v1/media/");
            output.push_str(&self.resource_name);
            output
        }
        pub fn upload<T, R>(
            mut self,
            content: R,
            mime_type: ::mime::Mime,
        ) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            R: ::std::io::Read + ::std::io::Seek + Send + 'static,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._simple_upload_path());
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
        pub fn execute_debug(self) -> Result<crate::schemas::Media, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
            output.push_str("v1/media/");
            output.push_str(&self.resource_name);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod operations {
    pub mod params {}
    pub struct OperationsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> OperationsActions<'a, A> {
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
        ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
            output.push_str("v1/");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod query {
    pub mod params {}
    pub struct QueryActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> QueryActions<'a, A> {
        #[doc = "The Cloud Search Query API provides the search method, which returns\nthe most relevant results from a user query.  The results can come from\nG Suite Apps, such as Gmail or Google Drive, or they can come from data\nthat you have indexed from a third party."]
        pub fn search(&self, request: crate::schemas::SearchRequest) -> SearchRequestBuilder<A> {
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
        pub fn suggest(&self, request: crate::schemas::SuggestRequest) -> SuggestRequestBuilder<A> {
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
        pub fn sources(&self) -> sources::SourcesActions<A> {
            sources::SourcesActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct SearchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
    impl<'a, A: yup_oauth2::GetToken> SearchRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::SearchResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
            output.push_str("v1/query/search");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SuggestRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
    impl<'a, A: yup_oauth2::GetToken> SuggestRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::SuggestResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
            output.push_str("v1/query/suggest");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    pub mod sources {
        pub mod params {}
        pub struct SourcesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> SourcesActions<'a, A> {
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
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Number of sources to return in the response."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn request_options_debug_options_enable_debugging(
                &mut self,
                value: bool,
            ) -> &mut Self {
                self.request_options_debug_options_enable_debugging = Some(value);
                self
            }
            #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\".\nFor more information, see\nhttp://www.unicode.org/reports/tr35/#Unicode_locale_identifier.\nFor translations.\n\nWhen specified, the documents in search results are biased towards the\nspecified language.\nSuggest API does not use this parameter. It autocompletes only based on\ncharacters in the query."]
            pub fn request_options_language_code(&mut self, value: impl Into<String>) -> &mut Self {
                self.request_options_language_code = Some(value.into());
                self
            }
            #[doc = "Id of the application created using SearchApplicationsService."]
            pub fn request_options_search_application_id(
                &mut self,
                value: impl Into<String>,
            ) -> &mut Self {
                self.request_options_search_application_id = Some(value.into());
                self
            }
            #[doc = "Current user's time zone id, such as \"America/Los_Angeles\" or\n\"Australia/Sydney\". These IDs are defined by\n[Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/)\nproject, and currently available in the file\n[timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml)"]
            pub fn request_options_time_zone(&mut self, value: impl Into<String>) -> &mut Self {
                self.request_options_time_zone = Some(value.into());
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
            pub fn iter_sources<T>(
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
                            #[serde(rename = "sources")]
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
            ) -> Result<crate::schemas::ListQuerySourcesResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/query/sources");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
}
pub mod settings {
    pub mod params {}
    pub struct SettingsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> SettingsActions<'a, A> {
        #[doc = "Actions that can be performed on the datasources resource"]
        pub fn datasources(&self) -> datasources::DatasourcesActions<A> {
            datasources::DatasourcesActions
        }
        #[doc = "Actions that can be performed on the searchapplications resource"]
        pub fn searchapplications(&self) -> searchapplications::SearchapplicationsActions<A> {
            searchapplications::SearchapplicationsActions
        }
    }
    pub mod datasources {
        pub mod params {}
        pub struct DatasourcesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> DatasourcesActions<'a, A> {
            #[doc = "Creates a datasource."]
            pub fn create(&self, request: crate::schemas::DataSource) -> CreateRequestBuilder<A> {
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
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/datasources");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
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
            ) -> Result<crate::schemas::DataSource, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
                self
            }
            #[doc = "Maximum number of datasources to fetch in a request.\nThe max value is 100.\n<br />The default value is 10"]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Starting index of the results."]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
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
            pub fn iter_sources<T>(
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
                            #[serde(rename = "sources")]
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
            ) -> Result<crate::schemas::ListDataSourceResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/datasources");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        pub struct UpdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/");
                output.push_str(&self.name);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod searchapplications {
        pub mod params {}
        pub struct SearchapplicationsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> SearchapplicationsActions<'a, A> {
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
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/searchapplications");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
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
            ) -> Result<crate::schemas::SearchApplication, Box<dyn ::std::error::Error>>
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/");
                output.push_str(&self.name);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "If you are asked by Google to help with debugging, set this field.\nOtherwise, ignore this field."]
            pub fn debug_options_enable_debugging(&mut self, value: bool) -> &mut Self {
                self.debug_options_enable_debugging = Some(value);
                self
            }
            #[doc = "The maximum number of items to return."]
            pub fn page_size(&mut self, value: i32) -> &mut Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The next_page_token value returned from a previous List request, if any.\n<br/> The default value is 10"]
            pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.page_token = Some(value.into());
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
            pub fn iter_search_applications<T>(
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
                            #[serde(rename = "searchApplications")]
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
            ) -> Result<crate::schemas::ListSearchApplicationsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/searchapplications");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
        pub struct ResetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> ResetRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/");
                output.push_str(&self.name);
                output.push_str(":reset");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
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
            ) -> Result<crate::schemas::Operation, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/");
                output.push_str(&self.name);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
}
pub mod stats {
    pub mod params {}
    pub struct StatsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> StatsActions<'a, A> {
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
        pub fn index(&self) -> index::IndexActions<A> {
            index::IndexActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetIndexRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
    impl<'a, A: yup_oauth2::GetToken> GetIndexRequestBuilder<'a, A> {
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
        pub fn from_date_day(&mut self, value: i32) -> &mut Self {
            self.from_date_day = Some(value);
            self
        }
        #[doc = "Month of date. Must be from 1 to 12."]
        pub fn from_date_month(&mut self, value: i32) -> &mut Self {
            self.from_date_month = Some(value);
            self
        }
        #[doc = "Year of date. Must be from 1 to 9999."]
        pub fn from_date_year(&mut self, value: i32) -> &mut Self {
            self.from_date_year = Some(value);
            self
        }
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
        pub fn to_date_day(&mut self, value: i32) -> &mut Self {
            self.to_date_day = Some(value);
            self
        }
        #[doc = "Month of date. Must be from 1 to 12."]
        pub fn to_date_month(&mut self, value: i32) -> &mut Self {
            self.to_date_month = Some(value);
            self
        }
        #[doc = "Year of date. Must be from 1 to 9999."]
        pub fn to_date_year(&mut self, value: i32) -> &mut Self {
            self.to_date_year = Some(value);
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
        ) -> Result<crate::schemas::GetCustomerIndexStatsResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
            output.push_str("v1/stats/index");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    pub mod index {
        pub mod params {}
        pub struct IndexActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> IndexActions<'a, A> {
            #[doc = "Actions that can be performed on the datasources resource"]
            pub fn datasources(&self) -> datasources::DatasourcesActions<A> {
                datasources::DatasourcesActions
            }
        }
        pub mod datasources {
            pub mod params {}
            pub struct DatasourcesActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> DatasourcesActions<'a, A> {
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
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                pub fn from_date_day(&mut self, value: i32) -> &mut Self {
                    self.from_date_day = Some(value);
                    self
                }
                #[doc = "Month of date. Must be from 1 to 12."]
                pub fn from_date_month(&mut self, value: i32) -> &mut Self {
                    self.from_date_month = Some(value);
                    self
                }
                #[doc = "Year of date. Must be from 1 to 9999."]
                pub fn from_date_year(&mut self, value: i32) -> &mut Self {
                    self.from_date_year = Some(value);
                    self
                }
                #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                pub fn to_date_day(&mut self, value: i32) -> &mut Self {
                    self.to_date_day = Some(value);
                    self
                }
                #[doc = "Month of date. Must be from 1 to 12."]
                pub fn to_date_month(&mut self, value: i32) -> &mut Self {
                    self.to_date_month = Some(value);
                    self
                }
                #[doc = "Year of date. Must be from 1 to 9999."]
                pub fn to_date_year(&mut self, value: i32) -> &mut Self {
                    self.to_date_year = Some(value);
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
                ) -> Result<
                    crate::schemas::GetDataSourceIndexStatsResponse,
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/stats/index/");
                    output.push_str(&self.name);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud_search"])
                            .unwrap()
                            .access_token,
                    );
                    req
                }
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
