#![doc = "# Resources and Methods\n    * [activities](resources/activities/struct.ActivitiesActions.html)\n      * [*list*](resources/activities/struct.ListRequestBuilder.html), [*watch*](resources/activities/struct.WatchRequestBuilder.html)\n    * [channels](resources/channels/struct.ChannelsActions.html)\n      * [*stop*](resources/channels/struct.StopRequestBuilder.html)\n    * [customer_usage_reports](resources/customer_usage_reports/struct.CustomerUsageReportsActions.html)\n      * [*get*](resources/customer_usage_reports/struct.GetRequestBuilder.html)\n    * [entity_usage_reports](resources/entity_usage_reports/struct.EntityUsageReportsActions.html)\n      * [*get*](resources/entity_usage_reports/struct.GetRequestBuilder.html)\n    * [user_usage_report](resources/user_usage_report/struct.UserUsageReportActions.html)\n      * [*get*](resources/user_usage_report/struct.GetRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View audit reports for your G Suite domain\n\n`https://www.googleapis.com/auth/admin.reports.audit.readonly`"]
    pub const ADMIN_REPORTS_AUDIT_READONLY: &str =
        "https://www.googleapis.com/auth/admin.reports.audit.readonly";
    #[doc = "View usage reports for your G Suite domain\n\n`https://www.googleapis.com/auth/admin.reports.usage.readonly`"]
    pub const ADMIN_REPORTS_USAGE_READONLY: &str =
        "https://www.googleapis.com/auth/admin.reports.usage.readonly";
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
    pub struct Activities {
        #[doc = "ETag of the resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Each activity record in the response."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Activity>>,
        #[doc = "The type of API resource. For an activity report, the value is reports#activities."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token for retrieving the follow-on next page of the report. The nextPageToken value is used in the request's pageToken query string."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Activities {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Activities {
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
    pub struct Activity {
        #[doc = "User doing the action."]
        #[serde(
            rename = "actor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actor: ::std::option::Option<crate::schemas::ActivityActor>,
        #[doc = "ETag of the entry."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Activity events in the report."]
        #[serde(
            rename = "events",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub events: ::std::option::Option<Vec<crate::schemas::ActivityEventsItems>>,
        #[doc = "Unique identifier for each activity record."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<crate::schemas::ActivityId>,
        #[doc = "IP address of the user doing the action. This is the Internet Protocol (IP) address of the user when logging into G Suite which may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. The API supports IPv4 and IPv6."]
        #[serde(
            rename = "ipAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_address: ::std::option::Option<String>,
        #[doc = "The type of API resource. For an activity report, the value is audit#activity."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "This is the domain that is affected by the report's event. For example domain of Admin console or the Drive application's document owner."]
        #[serde(
            rename = "ownerDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owner_domain: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Activity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Activity {
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
    pub struct ActivityActor {
        #[doc = "The type of actor."]
        #[serde(
            rename = "callerType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub caller_type: ::std::option::Option<String>,
        #[doc = "The primary email address of the actor. May be absent if there is no email address associated with the actor."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Only present when callerType is KEY. Can be the consumer_key of the requestor for OAuth 2LO API requests or an identifier for robot accounts."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The unique G Suite profile ID of the actor. May be absent if the actor is not a G Suite user."]
        #[serde(
            rename = "profileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActivityActor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActivityActor {
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
    pub struct ActivityEventsItems {
        #[doc = "Name of the event. This is the specific name of the activity reported by the API. And each eventName is related to a specific G Suite service or feature which the API organizes into types of events.\nFor eventName request parameters in general:\n\n* If no eventName is given, the report returns all possible instances of an eventName.\n* When you request an eventName, the API's response returns all activities which contain that eventName. It is possible that the returned activities will have other eventName properties in addition to the one requested.  \n  For more information about eventName properties, see the list of event names for various applications above in applicationName."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Parameter value pairs for various applications. For more information about eventName parameters, see the list of event names for various applications above in applicationName."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters:
            ::std::option::Option<Vec<crate::schemas::ActivityEventsItemsParametersItems>>,
        #[doc = "Type of event. The G Suite service or feature that an administrator changes is identified in the type property which identifies an event using the eventName property. For a full list of the API's type categories, see the list of event names for various applications above in applicationName."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActivityEventsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActivityEventsItems {
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
    pub struct ActivityEventsItemsParametersItems {
        #[doc = "Boolean value of the parameter."]
        #[serde(
            rename = "boolValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "Integer value of the parameter."]
        #[serde(
            rename = "intValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub int_value: ::std::option::Option<i64>,
        #[doc = "Nested parameter value pairs associated with this parameter. Complex value type for a parameter are returned as a list of parameter values. For example, the address parameter may have a value as [{parameter: [{name: city, value: abc}]}]"]
        #[serde(
            rename = "messageValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_value:
            ::std::option::Option<crate::schemas::ActivityEventsItemsParametersItemsMessageValue>,
        #[doc = "Integer values of the parameter."]
        #[serde(
            rename = "multiIntValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_int_value: ::std::option::Option<Vec<i64>>,
        #[doc = "List of messageValue objects."]
        #[serde(
            rename = "multiMessageValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_message_value: ::std::option::Option<
            Vec<crate::schemas::ActivityEventsItemsParametersItemsMultiMessageValueItems>,
        >,
        #[doc = "String values of the parameter."]
        #[serde(
            rename = "multiValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_value: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the parameter."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "String value of the parameter."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActivityEventsItemsParametersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActivityEventsItemsParametersItems {
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
    pub struct ActivityEventsItemsParametersItemsMessageValue {
        #[doc = "Parameter values"]
        #[serde(
            rename = "parameter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter: ::std::option::Option<Vec<crate::schemas::NestedParameter>>,
    }
    impl ::google_field_selector::FieldSelector for ActivityEventsItemsParametersItemsMessageValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActivityEventsItemsParametersItemsMessageValue {
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
    pub struct ActivityEventsItemsParametersItemsMultiMessageValueItems {
        #[doc = "Parameter values"]
        #[serde(
            rename = "parameter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter: ::std::option::Option<Vec<crate::schemas::NestedParameter>>,
    }
    impl ::google_field_selector::FieldSelector
        for ActivityEventsItemsParametersItemsMultiMessageValueItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ActivityEventsItemsParametersItemsMultiMessageValueItems
    {
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
    pub struct ActivityId {
        #[doc = "Application name to which the event belongs. For possible values see the list of applications above in applicationName."]
        #[serde(
            rename = "applicationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub application_name: ::std::option::Option<String>,
        #[doc = "The unique identifier for a G suite account."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
        #[doc = "Time of occurrence of the activity. This is in UNIX epoch time in seconds."]
        #[serde(
            rename = "time",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Unique qualifier if multiple events have the same time."]
        #[serde(
            rename = "uniqueQualifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub unique_qualifier: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ActivityId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActivityId {
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
    pub struct Channel {
        #[doc = "The address where notifications are delivered for this channel."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<String>,
        #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
        #[serde(
            rename = "expiration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub expiration: ::std::option::Option<i64>,
        #[doc = "A UUID or similar unique string that identifies this channel."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
        #[serde(
            rename = "params",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub params: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
        #[serde(
            rename = "payload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload: ::std::option::Option<bool>,
        #[doc = "The type of delivery mechanism used for this channel."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<String>,
        #[doc = "A version-specific identifier for the watched resource."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
        #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Channel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Channel {
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
    pub struct NestedParameter {
        #[doc = "Boolean value of the parameter."]
        #[serde(
            rename = "boolValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "Integer value of the parameter."]
        #[serde(
            rename = "intValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub int_value: ::std::option::Option<i64>,
        #[doc = "Multiple boolean values of the parameter."]
        #[serde(
            rename = "multiBoolValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_bool_value: ::std::option::Option<Vec<bool>>,
        #[doc = "Multiple integer values of the parameter."]
        #[serde(
            rename = "multiIntValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_int_value: ::std::option::Option<Vec<i64>>,
        #[doc = "Multiple string values of the parameter."]
        #[serde(
            rename = "multiValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_value: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the parameter."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "String value of the parameter."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NestedParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NestedParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct UsageReport {
        #[doc = "The date of the report request."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<String>,
        #[doc = "Information about the type of the item."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity: ::std::option::Option<crate::schemas::UsageReportEntity>,
        #[doc = "ETag of the resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The type of API resource. For a usage report, the value is admin#reports#usageReport."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Parameter value pairs for various applications. For the Customers usage report parameters and values, see the customer usage parameters reference."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<crate::schemas::UsageReportParametersItems>>,
    }
    impl ::google_field_selector::FieldSelector for UsageReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageReport {
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
    pub struct UsageReportEntity {
        #[doc = "The unique identifier of the customer's account."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
        #[doc = "Object key. Only relevant if entity.type = \"OBJECT\" Note: external-facing name of report is \"Entities\" rather than \"Objects\"."]
        #[serde(
            rename = "entityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "The user's immutable G Suite profile identifier."]
        #[serde(
            rename = "profileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_id: ::std::option::Option<String>,
        #[doc = "The type of item. The value is customer."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The user's email address. Only relevant if entity.type = \"USER\""]
        #[serde(
            rename = "userEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsageReportEntity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageReportEntity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct UsageReportParametersItems {
        #[doc = "Boolean value of the parameter."]
        #[serde(
            rename = "boolValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "The RFC 3339 formatted value of the parameter, for example 2010-10-28T10:26:35.000Z."]
        #[serde(
            rename = "datetimeValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub datetime_value: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Integer value of the parameter."]
        #[serde(
            rename = "intValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub int_value: ::std::option::Option<i64>,
        #[doc = "Nested message value of the parameter."]
        #[serde(
            rename = "msgValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub msg_value:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "String value of the parameter."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsageReportParametersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageReportParametersItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct UsageReports {
        #[doc = "ETag of the resource."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The type of API resource. For a usage report, the value is admin#reports#usageReports."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token to specify next page. A report with multiple pages has a nextPageToken property in the response. For your follow-on requests getting all of the report's pages, enter the nextPageToken value in the pageToken query string."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Various application parameter records."]
        #[serde(
            rename = "usageReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage_reports: ::std::option::Option<Vec<crate::schemas::UsageReport>>,
        #[doc = "Warnings, if any."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warnings: ::std::option::Option<Vec<crate::schemas::UsageReportsWarningsItems>>,
    }
    impl ::google_field_selector::FieldSelector for UsageReports {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageReports {
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
    pub struct UsageReportsWarningsItems {
        #[doc = "Machine readable code or warning type. The warning code value is 200."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<String>,
        #[doc = "Key-value pairs to give detailed information on the warning."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<Vec<crate::schemas::UsageReportsWarningsItemsDataItems>>,
        #[doc = "The human readable messages for a warning are:\n\n* Data is not available warning - Sorry, data for date yyyy-mm-dd for application \"application name\" is not available.\n* Partial data is available warning - Data for date yyyy-mm-dd for application \"application name\" is not available right now, please try again after a few hours."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsageReportsWarningsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageReportsWarningsItems {
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
    pub struct UsageReportsWarningsItemsDataItems {
        #[doc = "Key associated with a key-value pair to give detailed information on the warning."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Value associated with a key-value pair to give detailed information on the warning."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsageReportsWarningsItemsDataItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageReportsWarningsItemsDataItems {
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
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
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
    #[doc = "Actions that can be performed on the activities resource"]
    pub fn activities(&self) -> crate::resources::activities::ActivitiesActions {
        crate::resources::activities::ActivitiesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the channels resource"]
    pub fn channels(&self) -> crate::resources::channels::ChannelsActions {
        crate::resources::channels::ChannelsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the customer_usage_reports resource"]
    pub fn customer_usage_reports(
        &self,
    ) -> crate::resources::customer_usage_reports::CustomerUsageReportsActions {
        crate::resources::customer_usage_reports::CustomerUsageReportsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the entity_usage_reports resource"]
    pub fn entity_usage_reports(
        &self,
    ) -> crate::resources::entity_usage_reports::EntityUsageReportsActions {
        crate::resources::entity_usage_reports::EntityUsageReportsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the user_usage_report resource"]
    pub fn user_usage_report(&self) -> crate::resources::user_usage_report::UserUsageReportActions {
        crate::resources::user_usage_report::UserUsageReportActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod activities {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListApplicationName {
                #[doc = "The G Suite Access Transparency activity reports return information about different types of Access Transparency activity events."]
                AccessTransparency,
                #[doc = "The Admin console application's activity reports return account information about different types of administrator activity events."]
                Admin,
                #[doc = "The G Suite Calendar application's activity reports return information about various Calendar activity events."]
                Calendar,
                #[doc = "The Chat activity reports return information about various Chat activity events."]
                Chat,
                #[doc = "The Google Drive application's activity reports return information about various Google Drive activity events. The Drive activity report is only available for G Suite Business customers."]
                Drive,
                #[doc = "The Google Cloud Platform application's activity reports return information about various GCP activity events."]
                Gcp,
                #[doc = "The Google+ application's activity reports return information about various Google+ activity events."]
                Gplus,
                #[doc = "The Google Groups application's activity reports return information about various Groups activity events."]
                Groups,
                #[doc = "The Enterprise Groups activity reports return information about various Enterprise group activity events."]
                GroupsEnterprise,
                #[doc = "The Jamboard activity reports return information about various Jamboard activity events."]
                Jamboard,
                #[doc = "The Login application's activity reports return account information about different types of Login activity events."]
                Login,
                #[doc = "The Meet Audit activity report return information about different types of Meet Audit activity events."]
                Meet,
                #[doc = "The Mobile Audit activity report return information about different types of Mobile Audit activity events."]
                Mobile,
                #[doc = "The Rules activity report return information about different types of Rules activity events."]
                Rules,
                #[doc = "The SAML activity report return information about different types of SAML activity events."]
                Saml,
                #[doc = "The Token application's activity reports return account information about different types of Token activity events."]
                Token,
                #[doc = "The User Accounts application's activity reports return account information about different types of User Accounts activity events."]
                UserAccounts,
            }
            impl ListApplicationName {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListApplicationName::AccessTransparency => "access_transparency",
                        ListApplicationName::Admin => "admin",
                        ListApplicationName::Calendar => "calendar",
                        ListApplicationName::Chat => "chat",
                        ListApplicationName::Drive => "drive",
                        ListApplicationName::Gcp => "gcp",
                        ListApplicationName::Gplus => "gplus",
                        ListApplicationName::Groups => "groups",
                        ListApplicationName::GroupsEnterprise => "groups_enterprise",
                        ListApplicationName::Jamboard => "jamboard",
                        ListApplicationName::Login => "login",
                        ListApplicationName::Meet => "meet",
                        ListApplicationName::Mobile => "mobile",
                        ListApplicationName::Rules => "rules",
                        ListApplicationName::Saml => "saml",
                        ListApplicationName::Token => "token",
                        ListApplicationName::UserAccounts => "user_accounts",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListApplicationName {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListApplicationName {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListApplicationName, ()> {
                    Ok(match s {
                        "access_transparency" => ListApplicationName::AccessTransparency,
                        "admin" => ListApplicationName::Admin,
                        "calendar" => ListApplicationName::Calendar,
                        "chat" => ListApplicationName::Chat,
                        "drive" => ListApplicationName::Drive,
                        "gcp" => ListApplicationName::Gcp,
                        "gplus" => ListApplicationName::Gplus,
                        "groups" => ListApplicationName::Groups,
                        "groups_enterprise" => ListApplicationName::GroupsEnterprise,
                        "jamboard" => ListApplicationName::Jamboard,
                        "login" => ListApplicationName::Login,
                        "meet" => ListApplicationName::Meet,
                        "mobile" => ListApplicationName::Mobile,
                        "rules" => ListApplicationName::Rules,
                        "saml" => ListApplicationName::Saml,
                        "token" => ListApplicationName::Token,
                        "user_accounts" => ListApplicationName::UserAccounts,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListApplicationName {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListApplicationName {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListApplicationName {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "access_transparency" => ListApplicationName::AccessTransparency,
                        "admin" => ListApplicationName::Admin,
                        "calendar" => ListApplicationName::Calendar,
                        "chat" => ListApplicationName::Chat,
                        "drive" => ListApplicationName::Drive,
                        "gcp" => ListApplicationName::Gcp,
                        "gplus" => ListApplicationName::Gplus,
                        "groups" => ListApplicationName::Groups,
                        "groups_enterprise" => ListApplicationName::GroupsEnterprise,
                        "jamboard" => ListApplicationName::Jamboard,
                        "login" => ListApplicationName::Login,
                        "meet" => ListApplicationName::Meet,
                        "mobile" => ListApplicationName::Mobile,
                        "rules" => ListApplicationName::Rules,
                        "saml" => ListApplicationName::Saml,
                        "token" => ListApplicationName::Token,
                        "user_accounts" => ListApplicationName::UserAccounts,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListApplicationName {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListApplicationName {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum WatchApplicationName {
                #[doc = "The G Suite Access Transparency activity reports return information about different types of Access Transparency activity events."]
                AccessTransparency,
                #[doc = "The Admin console application's activity reports return account information about different types of administrator activity events."]
                Admin,
                #[doc = "The G Suite Calendar application's activity reports return information about various Calendar activity events."]
                Calendar,
                #[doc = "The Chat activity reports return information about various Chat activity events."]
                Chat,
                #[doc = "The Google Drive application's activity reports return information about various Google Drive activity events. The Drive activity report is only available for G Suite Business customers."]
                Drive,
                #[doc = "The Google Cloud Platform application's activity reports return information about various GCP activity events."]
                Gcp,
                #[doc = "The Google+ application's activity reports return information about various Google+ activity events."]
                Gplus,
                #[doc = "The Google Groups application's activity reports return information about various Groups activity events."]
                Groups,
                #[doc = "The Enterprise Groups activity reports return information about various Enterprise group activity events."]
                GroupsEnterprise,
                #[doc = "The Jamboard activity reports return information about various Jamboard activity events."]
                Jamboard,
                #[doc = "The Login application's activity reports return account information about different types of Login activity events."]
                Login,
                #[doc = "The Meet Audit activity report return information about different types of Meet Audit activity events."]
                Meet,
                #[doc = "The Mobile Audit activity report return information about different types of Mobile Audit activity events."]
                Mobile,
                #[doc = "The Rules activity report return information about different types of Rules activity events."]
                Rules,
                #[doc = "The SAML activity report return information about different types of SAML activity events."]
                Saml,
                #[doc = "The Token application's activity reports return account information about different types of Token activity events."]
                Token,
                #[doc = "The User Accounts application's activity reports return account information about different types of User Accounts activity events."]
                UserAccounts,
            }
            impl WatchApplicationName {
                pub fn as_str(self) -> &'static str {
                    match self {
                        WatchApplicationName::AccessTransparency => "access_transparency",
                        WatchApplicationName::Admin => "admin",
                        WatchApplicationName::Calendar => "calendar",
                        WatchApplicationName::Chat => "chat",
                        WatchApplicationName::Drive => "drive",
                        WatchApplicationName::Gcp => "gcp",
                        WatchApplicationName::Gplus => "gplus",
                        WatchApplicationName::Groups => "groups",
                        WatchApplicationName::GroupsEnterprise => "groups_enterprise",
                        WatchApplicationName::Jamboard => "jamboard",
                        WatchApplicationName::Login => "login",
                        WatchApplicationName::Meet => "meet",
                        WatchApplicationName::Mobile => "mobile",
                        WatchApplicationName::Rules => "rules",
                        WatchApplicationName::Saml => "saml",
                        WatchApplicationName::Token => "token",
                        WatchApplicationName::UserAccounts => "user_accounts",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for WatchApplicationName {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for WatchApplicationName {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<WatchApplicationName, ()> {
                    Ok(match s {
                        "access_transparency" => WatchApplicationName::AccessTransparency,
                        "admin" => WatchApplicationName::Admin,
                        "calendar" => WatchApplicationName::Calendar,
                        "chat" => WatchApplicationName::Chat,
                        "drive" => WatchApplicationName::Drive,
                        "gcp" => WatchApplicationName::Gcp,
                        "gplus" => WatchApplicationName::Gplus,
                        "groups" => WatchApplicationName::Groups,
                        "groups_enterprise" => WatchApplicationName::GroupsEnterprise,
                        "jamboard" => WatchApplicationName::Jamboard,
                        "login" => WatchApplicationName::Login,
                        "meet" => WatchApplicationName::Meet,
                        "mobile" => WatchApplicationName::Mobile,
                        "rules" => WatchApplicationName::Rules,
                        "saml" => WatchApplicationName::Saml,
                        "token" => WatchApplicationName::Token,
                        "user_accounts" => WatchApplicationName::UserAccounts,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for WatchApplicationName {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for WatchApplicationName {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for WatchApplicationName {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "access_transparency" => WatchApplicationName::AccessTransparency,
                        "admin" => WatchApplicationName::Admin,
                        "calendar" => WatchApplicationName::Calendar,
                        "chat" => WatchApplicationName::Chat,
                        "drive" => WatchApplicationName::Drive,
                        "gcp" => WatchApplicationName::Gcp,
                        "gplus" => WatchApplicationName::Gplus,
                        "groups" => WatchApplicationName::Groups,
                        "groups_enterprise" => WatchApplicationName::GroupsEnterprise,
                        "jamboard" => WatchApplicationName::Jamboard,
                        "login" => WatchApplicationName::Login,
                        "meet" => WatchApplicationName::Meet,
                        "mobile" => WatchApplicationName::Mobile,
                        "rules" => WatchApplicationName::Rules,
                        "saml" => WatchApplicationName::Saml,
                        "token" => WatchApplicationName::Token,
                        "user_accounts" => WatchApplicationName::UserAccounts,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for WatchApplicationName {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for WatchApplicationName {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ActivitiesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ActivitiesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves a list of activities for a specific customer's account and application such as the Admin console application or the Google Drive application. For more information, see the guides for administrator and Google Drive activity reports. For more information about the activity report's parameters, see the activity parameters reference guides."]
            pub fn list(
                &self,
                user_key: impl Into<String>,
                application_name: crate::resources::activities::params::ListApplicationName,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_key: user_key.into(),
                    application_name,
                    actor_ip_address: None,
                    customer_id: None,
                    end_time: None,
                    event_name: None,
                    filters: None,
                    max_results: None,
                    org_unit_id: None,
                    page_token: None,
                    start_time: None,
                }
            }
            #[doc = "Start receiving notifications for account activities. For more information, see Receiving Push Notifications."]
            pub fn watch(
                &self,
                request: crate::schemas::Channel,
                user_key: impl Into<String>,
                application_name: crate::resources::activities::params::WatchApplicationName,
            ) -> WatchRequestBuilder {
                WatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_key: user_key.into(),
                    application_name,
                    actor_ip_address: None,
                    customer_id: None,
                    end_time: None,
                    event_name: None,
                    filters: None,
                    max_results: None,
                    org_unit_id: None,
                    page_token: None,
                    start_time: None,
                }
            }
        }
        #[doc = "Created via [ActivitiesActions::list()](struct.ActivitiesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            user_key: String,
            application_name: crate::resources::activities::params::ListApplicationName,
            actor_ip_address: Option<String>,
            customer_id: Option<String>,
            end_time: Option<String>,
            event_name: Option<String>,
            filters: Option<String>,
            max_results: Option<i32>,
            org_unit_id: Option<String>,
            page_token: Option<String>,
            start_time: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "The Internet Protocol (IP) Address of host where the event was performed. This is an additional way to filter a report's summary using the IP address of the user whose activity is being reported. This IP address may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. This parameter supports both IPv4 and IPv6 address versions."]
            pub fn actor_ip_address(mut self, value: impl Into<String>) -> Self {
                self.actor_ip_address = Some(value.into());
                self
            }
            #[doc = "The unique ID of the customer to retrieve data for."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
                self
            }
            #[doc = "Sets the end of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The default value is the approximate time of the API request. An API report has three basic time concepts:\n\n* Date of the API's request for a report: When the API created and retrieved the report. \n* Report's start time: The beginning of the timespan shown in the report. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error. \n* Report's end time: The end of the timespan shown in the report. For example, the timespan of events summarized in a report can start in April and end in May. The report itself can be requested in August.  If the endTime is not specified, the report returns all activities from the startTime until the current time or the most recent 180 days if the startTime is more than 180 days in the past."]
            pub fn end_time(mut self, value: impl Into<String>) -> Self {
                self.end_time = Some(value.into());
                self
            }
            #[doc = "The name of the event being queried by the API. Each eventName is related to a specific G Suite service or feature which the API organizes into types of events. An example is the Google Calendar events in the Admin console application's reports. The Calendar Settings type structure has all of the Calendar eventName activities reported by the API. When an administrator changes a Calendar setting, the API reports this activity in the Calendar Settings type and eventName parameters. For more information about eventName query strings and parameters, see the list of event names for various applications above in applicationName."]
            pub fn event_name(mut self, value: impl Into<String>) -> Self {
                self.event_name = Some(value.into());
                self
            }
            #[doc = "The filters query string is a comma-separated list. The list is composed of event parameters that are manipulated by relational operators. Event parameters are in the form [parameter1 name][relational operator][parameter1 value],[parameter2 name][relational operator][parameter2 value],... \nThese event parameters are associated with a specific eventName. An empty report is returned if the filtered request's parameter does not belong to the eventName. For more information about eventName parameters, see the list of event names for various applications above in applicationName.\n\nIn the following Admin Activity example, the <> operator is URL-encoded in the request's query string (%3C%3E):\nGET...&eventName=CHANGE_CALENDAR_SETTING &filters=NEW_VALUE%3C%3EREAD_ONLY_ACCESS\n\nIn the following Drive example, the list can be a view or edit event's doc_id parameter with a value that is manipulated by an 'equal to' (==) or 'not equal to' (<>) relational operator. In the first example, the report returns each edited document's doc_id. In the second example, the report returns each viewed document's doc_id that equals the value 12345 and does not return any viewed document's which have a doc_id value of 98765. The <> operator is URL-encoded in the request's query string (%3C%3E):\n\nGET...&eventName=edit&filters=doc_id GET...&eventName=view&filters=doc_id==12345,doc_id%3C%3E98765\n\nThe relational operators include:\n\n* == - 'equal to'. \n* <> - 'not equal to'. It is URL-encoded (%3C%3E). \n* < - 'less than'. It is URL-encoded (%3C). \n* <= - 'less than or equal to'. It is URL-encoded (%3C=). \n* \n   > \n   > * 'greater than'. It is URL-encoded (%3E). \n\n* \n   > \n   > = - 'greater than or equal to'. It is URL-encoded (%3E=).  \n   > Note: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.\n   > In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. If no parameters are requested, all parameters are returned."]
            pub fn filters(mut self, value: impl Into<String>) -> Self {
                self.filters = Some(value.into());
                self
            }
            #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page. The maxResults query string is optional in the request. The default value is 1000."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "ID of the organizational unit to report on. Activity records will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results."]
            pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                self.org_unit_id = Some(value.into());
                self
            }
            #[doc = "The token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Sets the beginning of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The report returns all activities from startTime until endTime. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error."]
            pub fn start_time(mut self, value: impl Into<String>) -> Self {
                self.start_time = Some(value.into());
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Activity> {
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Activity> {
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
            ) -> crate::iter::PageIter<Self, crate::schemas::Activities> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::Activities> {
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
            ) -> Result<crate::schemas::Activities, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Activities, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/admin/reports/v1/".to_owned();
                output.push_str("activity/users/");
                {
                    let var_as_str = &self.user_key;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/applications/");
                {
                    let var_as_string = self.application_name.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("actorIpAddress", &self.actor_ip_address)]);
                let req = req.query(&[("customerId", &self.customer_id)]);
                let req = req.query(&[("endTime", &self.end_time)]);
                let req = req.query(&[("eventName", &self.event_name)]);
                let req = req.query(&[("filters", &self.filters)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("orgUnitID", &self.org_unit_id)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("startTime", &self.start_time)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
        #[doc = "Created via [ActivitiesActions::watch()](struct.ActivitiesActions.html#method.watch)"]
        #[derive(Debug, Clone)]
        pub struct WatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Channel,
            user_key: String,
            application_name: crate::resources::activities::params::WatchApplicationName,
            actor_ip_address: Option<String>,
            customer_id: Option<String>,
            end_time: Option<String>,
            event_name: Option<String>,
            filters: Option<String>,
            max_results: Option<i32>,
            org_unit_id: Option<String>,
            page_token: Option<String>,
            start_time: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> WatchRequestBuilder<'a> {
            #[doc = "The Internet Protocol (IP) Address of host where the event was performed. This is an additional way to filter a report's summary using the IP address of the user whose activity is being reported. This IP address may or may not reflect the user's physical location. For example, the IP address can be the user's proxy server's address or a virtual private network (VPN) address. This parameter supports both IPv4 and IPv6 address versions."]
            pub fn actor_ip_address(mut self, value: impl Into<String>) -> Self {
                self.actor_ip_address = Some(value.into());
                self
            }
            #[doc = "The unique ID of the customer to retrieve data for."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
                self
            }
            #[doc = "Sets the end of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The default value is the approximate time of the API request. An API report has three basic time concepts:\n\n* Date of the API's request for a report: When the API created and retrieved the report. \n* Report's start time: The beginning of the timespan shown in the report. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error. \n* Report's end time: The end of the timespan shown in the report. For example, the timespan of events summarized in a report can start in April and end in May. The report itself can be requested in August.  If the endTime is not specified, the report returns all activities from the startTime until the current time or the most recent 180 days if the startTime is more than 180 days in the past."]
            pub fn end_time(mut self, value: impl Into<String>) -> Self {
                self.end_time = Some(value.into());
                self
            }
            #[doc = "The name of the event being queried by the API. Each eventName is related to a specific G Suite service or feature which the API organizes into types of events. An example is the Google Calendar events in the Admin console application's reports. The Calendar Settings type structure has all of the Calendar eventName activities reported by the API. When an administrator changes a Calendar setting, the API reports this activity in the Calendar Settings type and eventName parameters. For more information about eventName query strings and parameters, see the list of event names for various applications above in applicationName."]
            pub fn event_name(mut self, value: impl Into<String>) -> Self {
                self.event_name = Some(value.into());
                self
            }
            #[doc = "The filters query string is a comma-separated list. The list is composed of event parameters that are manipulated by relational operators. Event parameters are in the form [parameter1 name][relational operator][parameter1 value],[parameter2 name][relational operator][parameter2 value],... \nThese event parameters are associated with a specific eventName. An empty report is returned if the filtered request's parameter does not belong to the eventName. For more information about eventName parameters, see the list of event names for various applications above in applicationName.\n\nIn the following Admin Activity example, the <> operator is URL-encoded in the request's query string (%3C%3E):\nGET...&eventName=CHANGE_CALENDAR_SETTING &filters=NEW_VALUE%3C%3EREAD_ONLY_ACCESS\n\nIn the following Drive example, the list can be a view or edit event's doc_id parameter with a value that is manipulated by an 'equal to' (==) or 'not equal to' (<>) relational operator. In the first example, the report returns each edited document's doc_id. In the second example, the report returns each viewed document's doc_id that equals the value 12345 and does not return any viewed document's which have a doc_id value of 98765. The <> operator is URL-encoded in the request's query string (%3C%3E):\n\nGET...&eventName=edit&filters=doc_id GET...&eventName=view&filters=doc_id==12345,doc_id%3C%3E98765\n\nThe relational operators include:\n\n* == - 'equal to'. \n* <> - 'not equal to'. It is URL-encoded (%3C%3E). \n* < - 'less than'. It is URL-encoded (%3C). \n* <= - 'less than or equal to'. It is URL-encoded (%3C=). \n* \n   > \n   > * 'greater than'. It is URL-encoded (%3E). \n\n* \n   > \n   > = - 'greater than or equal to'. It is URL-encoded (%3E=).  \n   > Note: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.\n   > In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters. If no parameters are requested, all parameters are returned."]
            pub fn filters(mut self, value: impl Into<String>) -> Self {
                self.filters = Some(value.into());
                self
            }
            #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page. The maxResults query string is optional in the request. The default value is 1000."]
            pub fn max_results(mut self, value: i32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "ID of the organizational unit to report on. Activity records will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results."]
            pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                self.org_unit_id = Some(value.into());
                self
            }
            #[doc = "The token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Sets the beginning of the range of time shown in the report. The date is in the RFC 3339 format, for example 2010-10-28T10:26:35.000Z. The report returns all activities from startTime until endTime. The startTime must be before the endTime (if specified) and the current time when the request is made, or the API returns an error."]
            pub fn start_time(mut self, value: impl Into<String>) -> Self {
                self.start_time = Some(value.into());
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
            ) -> Result<crate::schemas::Channel, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Channel, crate::Error> {
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
                let mut output = "https://www.googleapis.com/admin/reports/v1/".to_owned();
                output.push_str("activity/users/");
                {
                    let var_as_str = &self.user_key;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/applications/");
                {
                    let var_as_string = self.application_name.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/watch");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("actorIpAddress", &self.actor_ip_address)]);
                let req = req.query(&[("customerId", &self.customer_id)]);
                let req = req.query(&[("endTime", &self.end_time)]);
                let req = req.query(&[("eventName", &self.event_name)]);
                let req = req.query(&[("filters", &self.filters)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("orgUnitID", &self.org_unit_id)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("startTime", &self.start_time)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod channels {
        pub mod params {}
        pub struct ChannelsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ChannelsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Stop watching resources through this channel"]
            pub fn stop(&self, request: crate::schemas::Channel) -> StopRequestBuilder {
                StopRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
        }
        #[doc = "Created via [ChannelsActions::stop()](struct.ChannelsActions.html#method.stop)"]
        #[derive(Debug, Clone)]
        pub struct StopRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Channel,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> StopRequestBuilder<'a> {
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/admin/reports/v1/".to_owned();
                output.push_str("/admin/reports_v1/channels/stop");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod customer_usage_reports {
        pub mod params {}
        pub struct CustomerUsageReportsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CustomerUsageReportsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves a report which is a collection of properties and statistics for a specific customer's account. For more information, see the Customers Usage Report guide. For more information about the customer report's parameters, see the Customers Usage parameters reference guides."]
            pub fn get(&self, date: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    date: date.into(),
                    customer_id: None,
                    page_token: None,
                    parameters: None,
                }
            }
        }
        #[doc = "Created via [CustomerUsageReportsActions::get()](struct.CustomerUsageReportsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            date: String,
            customer_id: Option<String>,
            page_token: Option<String>,
            parameters: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The unique ID of the customer to retrieve data for."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
                self
            }
            #[doc = "Token to specify next page. A report with multiple pages has a nextPageToken property in the response. For your follow-on requests getting all of the report's pages, enter the nextPageToken value in the pageToken query string."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "The parameters query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Customers usage report include accounts, app_maker, apps_scripts, calendar, classroom, cros, docs, gmail, gplus, device_management, meet, and sites.\nA parameters query string is in the CSV form of app_name1:param_name1, app_name2:param_name2.\nNote: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.\nIn addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters.\n\nAn example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned."]
            pub fn parameters(mut self, value: impl Into<String>) -> Self {
                self.parameters = Some(value.into());
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
            pub fn iter_usage_reports<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_usage_reports_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_usage_reports_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReport> {
                self.iter_usage_reports_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_usage_reports_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReport> {
                self.iter_usage_reports_with_fields(Some("*"))
            }
            pub fn iter_usage_reports_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "usageReports").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "usageReports")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_warnings<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_warnings_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_warnings_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReportsWarningsItems>
            {
                self.iter_warnings_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_warnings_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReportsWarningsItems>
            {
                self.iter_warnings_with_fields(Some("*"))
            }
            pub fn iter_warnings_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "warnings").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "warnings")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::UsageReports> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::UsageReports> {
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
            ) -> Result<crate::schemas::UsageReports, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::UsageReports, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/admin/reports/v1/".to_owned();
                output.push_str("usage/dates/");
                {
                    let var_as_str = &self.date;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("customerId", &self.customer_id)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("parameters", &self.parameters)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for GetRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
    }
    pub mod entity_usage_reports {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetEntityType {
                #[doc = "Returns a report on Google+ communities."]
                GplusCommunities,
            }
            impl GetEntityType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetEntityType::GplusCommunities => "gplus_communities",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetEntityType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetEntityType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetEntityType, ()> {
                    Ok(match s {
                        "gplus_communities" => GetEntityType::GplusCommunities,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetEntityType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetEntityType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetEntityType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "gplus_communities" => GetEntityType::GplusCommunities,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetEntityType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetEntityType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetEntityKey {
                #[doc = "Returns activity events for all users."]
                All,
                #[doc = "Represents an app-specific identifier for the entity. For details on how to obtain the entityKey for a particular entityType, see the Entities Usage parameters reference guides."]
                EntityKey,
            }
            impl GetEntityKey {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetEntityKey::All => "all",
                        GetEntityKey::EntityKey => "entityKey",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetEntityKey {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetEntityKey {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetEntityKey, ()> {
                    Ok(match s {
                        "all" => GetEntityKey::All,
                        "entityKey" => GetEntityKey::EntityKey,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetEntityKey {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetEntityKey {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetEntityKey {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "all" => GetEntityKey::All,
                        "entityKey" => GetEntityKey::EntityKey,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetEntityKey {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetEntityKey {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct EntityUsageReportsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> EntityUsageReportsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves a report which is a collection of properties and statistics for entities used by users within the account. For more information, see the Entities Usage Report guide. For more information about the entities report's parameters, see the Entities Usage parameters reference guides."]
            pub fn get(
                &self,
                entity_type: crate::resources::entity_usage_reports::params::GetEntityType,
                entity_key: crate::resources::entity_usage_reports::params::GetEntityKey,
                date: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    entity_type,
                    entity_key,
                    date: date.into(),
                    customer_id: None,
                    filters: None,
                    max_results: None,
                    page_token: None,
                    parameters: None,
                }
            }
        }
        #[doc = "Created via [EntityUsageReportsActions::get()](struct.EntityUsageReportsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            entity_type: crate::resources::entity_usage_reports::params::GetEntityType,
            entity_key: crate::resources::entity_usage_reports::params::GetEntityKey,
            date: String,
            customer_id: Option<String>,
            filters: Option<String>,
            max_results: Option<u32>,
            page_token: Option<String>,
            parameters: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The unique ID of the customer to retrieve data for."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
                self
            }
            #[doc = "The filters query string is a comma-separated list of an application's event parameters where the parameter's value is manipulated by a relational operator. The filters query string includes the name of the application whose usage is returned in the report. The application values for the Entities usage report include accounts, docs, and gmail.\nFilters are in the form [application name]:[parameter name][relational operator][parameter value],....\n\nIn this example, the <> 'not equal to' operator is URL-encoded in the request's query string (%3C%3E):\nGET https://www.googleapis.com/admin/reports/v1/usage/gplus_communities/all/dates/2017-12-01 ?parameters=gplus:community_name,gplus:num_total_members &filters=gplus:num_total_members>0\n\nThe relational operators include:\n\n* == - 'equal to'. \n* <> - 'not equal to'. It is URL-encoded (%3C%3E). \n* < - 'less than'. It is URL-encoded (%3C). \n* <= - 'less than or equal to'. It is URL-encoded (%3C=). \n* \n   > \n   > * 'greater than'. It is URL-encoded (%3E). \n\n* \n   > \n   > = - 'greater than or equal to'. It is URL-encoded (%3E=).  Filters can only be applied to numeric parameters."]
            pub fn filters(mut self, value: impl Into<String>) -> Self {
                self.filters = Some(value.into());
                self
            }
            #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "The parameters query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Entities usage report are only gplus.\nA parameter query string is in the CSV form of [app_name1:param_name1], [app_name2:param_name2]....\nNote: The API doesn't accept multiple values of a parameter. If a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter.\nIn addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters.\n\nAn example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned."]
            pub fn parameters(mut self, value: impl Into<String>) -> Self {
                self.parameters = Some(value.into());
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
            pub fn iter_usage_reports<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_usage_reports_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_usage_reports_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReport> {
                self.iter_usage_reports_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_usage_reports_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReport> {
                self.iter_usage_reports_with_fields(Some("*"))
            }
            pub fn iter_usage_reports_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "usageReports").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "usageReports")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_warnings<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_warnings_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_warnings_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReportsWarningsItems>
            {
                self.iter_warnings_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_warnings_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReportsWarningsItems>
            {
                self.iter_warnings_with_fields(Some("*"))
            }
            pub fn iter_warnings_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "warnings").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "warnings")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::UsageReports> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::UsageReports> {
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
            ) -> Result<crate::schemas::UsageReports, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::UsageReports, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/admin/reports/v1/".to_owned();
                output.push_str("usage/");
                {
                    let var_as_string = self.entity_type.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.entity_key.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/dates/");
                {
                    let var_as_str = &self.date;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("customerId", &self.customer_id)]);
                let req = req.query(&[("filters", &self.filters)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("parameters", &self.parameters)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for GetRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
    }
    pub mod user_usage_report {
        pub mod params {}
        pub struct UserUsageReportActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> UserUsageReportActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves a report which is a collection of properties and statistics for a set of users with the account. For more information, see the User Usage Report guide. For more information about the user report's parameters, see the Users Usage parameters reference guides."]
            pub fn get(
                &self,
                user_key: impl Into<String>,
                date: impl Into<String>,
            ) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_key: user_key.into(),
                    date: date.into(),
                    customer_id: None,
                    filters: None,
                    max_results: None,
                    org_unit_id: None,
                    page_token: None,
                    parameters: None,
                }
            }
        }
        #[doc = "Created via [UserUsageReportActions::get()](struct.UserUsageReportActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            user_key: String,
            date: String,
            customer_id: Option<String>,
            filters: Option<String>,
            max_results: Option<u32>,
            org_unit_id: Option<String>,
            page_token: Option<String>,
            parameters: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
            #[doc = "The unique ID of the customer to retrieve data for."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
                self
            }
            #[doc = "The filters query string is a comma-separated list of an application's event parameters where the parameter's value is manipulated by a relational operator. The filters query string includes the name of the application whose usage is returned in the report. The application values for the Users Usage Report include accounts, docs, and gmail.\nFilters are in the form [application name]:[parameter name][relational operator][parameter value],....\n\nIn this example, the <> 'not equal to' operator is URL-encoded in the request's query string (%3C%3E):\nGET https://www.googleapis.com/admin/reports/v1/usage/users/all/dates/2013-03-03 ?parameters=accounts:last_login_time &filters=accounts:last_login_time>2010-10-28T10:26:35.000Z\n\nThe relational operators include:\n\n* == - 'equal to'. \n* <> - 'not equal to'. It is URL-encoded (%3C%3E). \n* < - 'less than'. It is URL-encoded (%3C). \n* <= - 'less than or equal to'. It is URL-encoded (%3C=). \n* \n   > \n   > * 'greater than'. It is URL-encoded (%3E). \n\n* \n   > \n   > = - 'greater than or equal to'. It is URL-encoded (%3E=)."]
            pub fn filters(mut self, value: impl Into<String>) -> Self {
                self.filters = Some(value.into());
                self
            }
            #[doc = "Determines how many activity records are shown on each response page. For example, if the request sets maxResults=1 and the report has two activities, the report has two pages. The response's nextPageToken property has the token to the second page.\nThe maxResults query string is optional."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "ID of the organizational unit to report on. User activity will be shown only for users who belong to the specified organizational unit. Data before Dec 17, 2018 doesn't appear in the filtered results."]
            pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                self.org_unit_id = Some(value.into());
                self
            }
            #[doc = "Token to specify next page. A report with multiple pages has a nextPageToken property in the response. In your follow-on request getting the next page of the report, enter the nextPageToken value in the pageToken query string."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "The parameters query string is a comma-separated list of event parameters that refine a report's results. The parameter is associated with a specific application. The application values for the Customers usage report include accounts, app_maker, apps_scripts, calendar, classroom, cros, docs, gmail, gplus, device_management, meet, and sites.\nA parameters query string is in the CSV form of app_name1:param_name1, app_name2:param_name2.\nNote: The API doesn't accept multiple values of a parameter.\nIf a particular parameter is supplied more than once in the API request, the API only accepts the last value of that request parameter. In addition, if an invalid request parameter is supplied in the API request, the API ignores that request parameter and returns the response corresponding to the remaining valid request parameters.\n\nAn example of an invalid request parameter is one that does not belong to the application. If no parameters are requested, all parameters are returned."]
            pub fn parameters(mut self, value: impl Into<String>) -> Self {
                self.parameters = Some(value.into());
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
            pub fn iter_usage_reports<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_usage_reports_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_usage_reports_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReport> {
                self.iter_usage_reports_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_usage_reports_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReport> {
                self.iter_usage_reports_with_fields(Some("*"))
            }
            pub fn iter_usage_reports_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "usageReports").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "usageReports")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_warnings<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_warnings_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_warnings_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReportsWarningsItems>
            {
                self.iter_warnings_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_warnings_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UsageReportsWarningsItems>
            {
                self.iter_warnings_with_fields(Some("*"))
            }
            pub fn iter_warnings_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "warnings").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "warnings")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::UsageReports> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::UsageReports> {
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
            ) -> Result<crate::schemas::UsageReports, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::UsageReports, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/admin/reports/v1/".to_owned();
                output.push_str("usage/users/");
                {
                    let var_as_str = &self.user_key;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/dates/");
                {
                    let var_as_str = &self.date;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("customerId", &self.customer_id)]);
                let req = req.query(&[("filters", &self.filters)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("orgUnitID", &self.org_unit_id)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("parameters", &self.parameters)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for GetRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
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
