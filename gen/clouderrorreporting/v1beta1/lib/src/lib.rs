#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [*deleteEvents*](resources/projects/struct.DeleteEventsRequestBuilder.html)\n      * [events](resources/projects/events/struct.EventsActions.html)\n        * [*list*](resources/projects/events/struct.ListRequestBuilder.html), [*report*](resources/projects/events/struct.ReportRequestBuilder.html)\n      * [group_stats](resources/projects/group_stats/struct.GroupStatsActions.html)\n        * [*list*](resources/projects/group_stats/struct.ListRequestBuilder.html)\n      * [groups](resources/projects/groups/struct.GroupsActions.html)\n        * [*get*](resources/projects/groups/struct.GetRequestBuilder.html), [*update*](resources/projects/groups/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteEventsResponse {}
    impl ::google_field_selector::FieldSelector for DeleteEventsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteEventsResponse {
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
    pub struct ErrorContext {
        #[doc = "The HTTP request which was processed when the error was\ntriggered."]
        #[serde(
            rename = "httpRequest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_request: ::std::option::Option<crate::schemas::HttpRequestContext>,
        #[doc = "The location in the source code where the decision was made to\nreport the error, usually the place where it was logged.\nFor a logged exception this would be the source line where the\nexception is logged, usually close to the place where it was\ncaught."]
        #[serde(
            rename = "reportLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_location: ::std::option::Option<crate::schemas::SourceLocation>,
        #[doc = "Source code that was used to build the executable which has\ncaused the given error message."]
        #[serde(
            rename = "sourceReferences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_references: ::std::option::Option<Vec<crate::schemas::SourceReference>>,
        #[doc = "The user who caused or was affected by the crash.\nThis can be a user ID, an email address, or an arbitrary token that\nuniquely identifies the user.\nWhen sending an error report, leave this field empty if the user was not\nlogged in. In this case the\nError Reporting system will use other data, such as remote IP address, to\ndistinguish affected users. See `affected_users_count` in\n`ErrorGroupStats`."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ErrorContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorContext {
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
    pub struct ErrorEvent {
        #[doc = "Data about the context in which the error occurred."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::ErrorContext>,
        #[doc = "Time when the event occurred as provided in the error report.\nIf the report did not contain a timestamp, the time the error was received\nby the Error Reporting system is used."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "The stack trace that was reported or logged by the service."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
        #[doc = "The `ServiceContext` for which this error was reported."]
        #[serde(
            rename = "serviceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_context: ::std::option::Option<crate::schemas::ServiceContext>,
    }
    impl ::google_field_selector::FieldSelector for ErrorEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorEvent {
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
    pub struct ErrorGroup {
        #[doc = "Group IDs are unique for a given project. If the same kind of error\noccurs in different service contexts, it will receive the same group ID."]
        #[serde(
            rename = "groupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_id: ::std::option::Option<String>,
        #[doc = "The group resource name.\nExample: <code>projects/my-project-123/groups/CNSgkpnppqKCUw</code>"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Associated tracking issues."]
        #[serde(
            rename = "trackingIssues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracking_issues: ::std::option::Option<Vec<crate::schemas::TrackingIssue>>,
    }
    impl ::google_field_selector::FieldSelector for ErrorGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorGroup {
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
    pub struct ErrorGroupStats {
        #[doc = "Service contexts with a non-zero error count for the given filter\ncriteria. This list can be truncated if multiple services are affected.\nRefer to `num_affected_services` for the total count."]
        #[serde(
            rename = "affectedServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_services: ::std::option::Option<Vec<crate::schemas::ServiceContext>>,
        #[doc = "Approximate number of affected users in the given group that\nmatch the filter criteria.\nUsers are distinguished by data in the `ErrorContext` of the\nindividual error events, such as their login name or their remote\nIP address in case of HTTP requests.\nThe number of affected users can be zero even if the number of\nerrors is non-zero if no data was provided from which the\naffected user could be deduced.\nUsers are counted based on data in the request\ncontext that was provided in the error report. If more users are\nimplicitly affected, such as due to a crash of the whole service,\nthis is not reflected here."]
        #[serde(
            rename = "affectedUsersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub affected_users_count: ::std::option::Option<i64>,
        #[doc = "Approximate total number of events in the given group that match\nthe filter criteria."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Approximate first occurrence that was ever seen for this group\nand which matches the given filter criteria, ignoring the\ntime_range that was specified in the request."]
        #[serde(
            rename = "firstSeenTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_seen_time: ::std::option::Option<String>,
        #[doc = "Group data that is independent of the filter criteria."]
        #[serde(
            rename = "group",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<crate::schemas::ErrorGroup>,
        #[doc = "Approximate last occurrence that was ever seen for this group and\nwhich matches the given filter criteria, ignoring the time_range\nthat was specified in the request."]
        #[serde(
            rename = "lastSeenTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_seen_time: ::std::option::Option<String>,
        #[doc = "The total number of services with a non-zero error count for the given\nfilter criteria."]
        #[serde(
            rename = "numAffectedServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_affected_services: ::std::option::Option<i32>,
        #[doc = "An arbitrary event that is chosen as representative for the whole group.\nThe representative event is intended to be used as a quick preview for\nthe whole group. Events in the group are usually sufficiently similar\nto each other such that showing an arbitrary representative provides\ninsight into the characteristics of the group as a whole."]
        #[serde(
            rename = "representative",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub representative: ::std::option::Option<crate::schemas::ErrorEvent>,
        #[doc = "Approximate number of occurrences over time.\nTimed counts returned by ListGroups are guaranteed to be:\n\n* Inside the requested time interval\n* Non-overlapping, and\n* Ordered by ascending time."]
        #[serde(
            rename = "timedCounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timed_counts: ::std::option::Option<Vec<crate::schemas::TimedCount>>,
    }
    impl ::google_field_selector::FieldSelector for ErrorGroupStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorGroupStats {
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
    pub struct HttpRequestContext {
        #[doc = "The type of HTTP request, such as `GET`, `POST`, etc."]
        #[serde(
            rename = "method",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method: ::std::option::Option<String>,
        #[doc = "The referrer information that is provided with the request."]
        #[serde(
            rename = "referrer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referrer: ::std::option::Option<String>,
        #[doc = "The IP address from which the request originated.\nThis can be IPv4, IPv6, or a token which is derived from the\nIP address, depending on the data that has been provided\nin the error report."]
        #[serde(
            rename = "remoteIp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remote_ip: ::std::option::Option<String>,
        #[doc = "The HTTP response status code for the request."]
        #[serde(
            rename = "responseStatusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response_status_code: ::std::option::Option<i32>,
        #[doc = "The URL of the request."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[doc = "The user agent information that is provided with the request."]
        #[serde(
            rename = "userAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HttpRequestContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HttpRequestContext {
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
    pub struct ListEventsResponse {
        #[doc = "The error events which match the given request."]
        #[serde(
            rename = "errorEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_events: ::std::option::Option<Vec<crate::schemas::ErrorEvent>>,
        #[doc = "If non-empty, more results are available.\nPass this token, along with the same query parameters as the first\nrequest, to view the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The timestamp specifies the start time to which the request was restricted."]
        #[serde(
            rename = "timeRangeBegin",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_range_begin: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListEventsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListEventsResponse {
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
    pub struct ListGroupStatsResponse {
        #[doc = "The error group stats which match the given request."]
        #[serde(
            rename = "errorGroupStats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_group_stats: ::std::option::Option<Vec<crate::schemas::ErrorGroupStats>>,
        #[doc = "If non-empty, more results are available.\nPass this token, along with the same query parameters as the first\nrequest, to view the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The timestamp specifies the start time to which the request was restricted.\nThe start time is set based on the requested time range. It may be adjusted\nto a later time if a project has exceeded the storage quota and older data\nhas been deleted."]
        #[serde(
            rename = "timeRangeBegin",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_range_begin: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListGroupStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListGroupStatsResponse {
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
    pub struct ReportErrorEventResponse {}
    impl ::google_field_selector::FieldSelector for ReportErrorEventResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportErrorEventResponse {
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
    pub struct ReportedErrorEvent {
        #[doc = "Optional. A description of the context in which the error occurred."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::ErrorContext>,
        #[doc = "Optional. Time when the event occurred.\nIf not provided, the time when the event was received by the\nError Reporting system will be used."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "Required. The error message.\nIf no `context.reportLocation` is provided, the message must contain a\nheader (typically consisting of the exception type name and an error\nmessage) and an exception stack trace in one of the supported programming\nlanguages and formats.\nSupported languages are Java, Python, JavaScript, Ruby, C#, PHP, and Go.\nSupported stack trace formats are:\n\n* **Java**: Must be the return value of\n  [`Throwable.printStackTrace()`](https://docs.oracle.com/javase/7/docs/api/java/lang/Throwable.html#printStackTrace%28%29).\n* **Python**: Must be the return value of\n  [`traceback.format_exc()`](https://docs.python.org/2/library/traceback.html#traceback.format_exc).\n* **JavaScript**: Must be the value of\n  [`error.stack`](https://github.com/v8/v8/wiki/Stack-Trace-API) as returned\n  by V8.\n* **Ruby**: Must contain frames returned by\n  [`Exception.backtrace`](https://ruby-doc.org/core-2.2.0/Exception.html#method-i-backtrace).\n* **C#**: Must be the return value of\n  [`Exception.ToString()`](https://msdn.microsoft.com/en-us/library/system.exception.tostring.aspx).\n* **PHP**: Must start with `PHP (Notice|Parse error|Fatal error|Warning)`\n  and contain the result of\n  [`(string)$exception`](http://php.net/manual/en/exception.tostring.php).\n* **Go**: Must be the return value of\n  [`runtime.Stack()`](https://golang.org/pkg/runtime/debug/#Stack)."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
        #[doc = "Required. The service context in which this error has occurred."]
        #[serde(
            rename = "serviceContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_context: ::std::option::Option<crate::schemas::ServiceContext>,
    }
    impl ::google_field_selector::FieldSelector for ReportedErrorEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportedErrorEvent {
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
    pub struct ServiceContext {
        #[doc = "Type of the MonitoredResource. List of possible values:\nhttps://cloud.google.com/monitoring/api/resources\n\nValue is set automatically for incoming errors and must not be set when\nreporting errors."]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<String>,
        #[doc = "An identifier of the service, such as the name of the\nexecutable, job, or Google App Engine service name. This field is expected\nto have a low number of values that are relatively stable over time, as\nopposed to `version`, which can be changed whenever new code is deployed.\n\nContains the service name for error reports extracted from Google\nApp Engine logs or `default` if the App Engine default service is used."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
        #[doc = "Represents the source code version that the developer provided,\nwhich could represent a version label or a Git SHA-1 hash, for example.\nFor App Engine standard environment, the version is set to the version of\nthe app."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ServiceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceContext {
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
    pub struct SourceLocation {
        #[doc = "The source code filename, which can include a truncated relative\npath, or a full path from a production machine."]
        #[serde(
            rename = "filePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_path: ::std::option::Option<String>,
        #[doc = "Human-readable name of a function or method.\nThe value can include optional context like the class or package name.\nFor example, `my.package.MyClass.method` in case of Java."]
        #[serde(
            rename = "functionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function_name: ::std::option::Option<String>,
        #[doc = "1-based. 0 indicates that the line number is unknown."]
        #[serde(
            rename = "lineNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_number: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SourceLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceLocation {
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
    pub struct SourceReference {
        #[doc = "Optional. A URI string identifying the repository.\nExample: \"https://github.com/GoogleCloudPlatform/kubernetes.git\""]
        #[serde(
            rename = "repository",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repository: ::std::option::Option<String>,
        #[doc = "The canonical and persistent identifier of the deployed revision.\nExample (git): \"0035781c50ec7aa23385dc841529ce8a4b70db1b\""]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SourceReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceReference {
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
    pub struct TimedCount {
        #[doc = "Approximate number of occurrences in the given time period."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "End of the time period to which `count` refers (excluded)."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Start of the time period to which `count` refers (included)."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimedCount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimedCount {
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
    pub struct TrackingIssue {
        #[doc = "A URL pointing to a related entry in an issue tracking system.\nExample: `https://github.com/user/project/issues/4`"]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TrackingIssue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TrackingIssue {
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes all error events of a given project."]
            pub fn delete_events(
                &self,
                project_name: impl Into<String>,
            ) -> DeleteEventsRequestBuilder {
                DeleteEventsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
                    project_name: project_name.into(),
                }
            }
            #[doc = "Actions that can be performed on the events resource"]
            pub fn events(&self) -> crate::resources::projects::events::EventsActions {
                crate::resources::projects::events::EventsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the group_stats resource"]
            pub fn group_stats(
                &self,
            ) -> crate::resources::projects::group_stats::GroupStatsActions {
                crate::resources::projects::group_stats::GroupStatsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the groups resource"]
            pub fn groups(&self) -> crate::resources::projects::groups::GroupsActions {
                crate::resources::projects::groups::GroupsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ProjectsActions::delete_events()](struct.ProjectsActions.html#method.delete_events)"]
        #[derive(Debug, Clone)]
        pub struct DeleteEventsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            project_name: String,
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
        impl<'a> DeleteEventsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::DeleteEventsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DeleteEventsResponse, crate::Error> {
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
                let mut output = "https://clouderrorreporting.googleapis.com/".to_owned();
                output.push_str("v1beta1/");
                {
                    let var_as_str = &self.project_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/events");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        pub mod events {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListTimeRangePeriod {
                    Period1Day,
                    Period1Hour,
                    Period1Week,
                    Period30Days,
                    Period6Hours,
                    PeriodUnspecified,
                }
                impl ListTimeRangePeriod {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListTimeRangePeriod::Period1Day => "PERIOD_1_DAY",
                            ListTimeRangePeriod::Period1Hour => "PERIOD_1_HOUR",
                            ListTimeRangePeriod::Period1Week => "PERIOD_1_WEEK",
                            ListTimeRangePeriod::Period30Days => "PERIOD_30_DAYS",
                            ListTimeRangePeriod::Period6Hours => "PERIOD_6_HOURS",
                            ListTimeRangePeriod::PeriodUnspecified => "PERIOD_UNSPECIFIED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListTimeRangePeriod {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListTimeRangePeriod {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListTimeRangePeriod, ()> {
                        Ok(match s {
                            "PERIOD_1_DAY" => ListTimeRangePeriod::Period1Day,
                            "PERIOD_1_HOUR" => ListTimeRangePeriod::Period1Hour,
                            "PERIOD_1_WEEK" => ListTimeRangePeriod::Period1Week,
                            "PERIOD_30_DAYS" => ListTimeRangePeriod::Period30Days,
                            "PERIOD_6_HOURS" => ListTimeRangePeriod::Period6Hours,
                            "PERIOD_UNSPECIFIED" => ListTimeRangePeriod::PeriodUnspecified,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListTimeRangePeriod {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListTimeRangePeriod {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListTimeRangePeriod {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "PERIOD_1_DAY" => ListTimeRangePeriod::Period1Day,
                            "PERIOD_1_HOUR" => ListTimeRangePeriod::Period1Hour,
                            "PERIOD_1_WEEK" => ListTimeRangePeriod::Period1Week,
                            "PERIOD_30_DAYS" => ListTimeRangePeriod::Period30Days,
                            "PERIOD_6_HOURS" => ListTimeRangePeriod::Period6Hours,
                            "PERIOD_UNSPECIFIED" => ListTimeRangePeriod::PeriodUnspecified,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListTimeRangePeriod {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListTimeRangePeriod {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct EventsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> EventsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists the specified events."]
                pub fn list(&self, project_name: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        project_name: project_name.into(),
                        group_id: None,
                        page_size: None,
                        page_token: None,
                        service_filter_resource_type: None,
                        service_filter_service: None,
                        service_filter_version: None,
                        time_range_period: None,
                    }
                }
                #[doc = "Report an individual error event.\n\nThis endpoint accepts **either** an OAuth token,\n**or** an [API key](https://support.google.com/cloud/answer/6158862)\nfor authentication. To use an API key, append it to the URL as the value of\na `key` parameter. For example:\n\n`POST https://clouderrorreporting.googleapis.com/v1beta1/{projectName}/events:report?key=123ABC456`"]
                pub fn report(
                    &self,
                    request: crate::schemas::ReportedErrorEvent,
                    project_name: impl Into<String>,
                ) -> ReportRequestBuilder {
                    ReportRequestBuilder {
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
                        project_name: project_name.into(),
                    }
                }
            }
            #[doc = "Created via [EventsActions::list()](struct.EventsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                project_name: String,
                group_id: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                service_filter_resource_type: Option<String>,
                service_filter_service: Option<String>,
                service_filter_version: Option<String>,
                time_range_period:
                    Option<crate::resources::projects::events::params::ListTimeRangePeriod>,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Required. The group for which events shall be returned."]
                pub fn group_id(mut self, value: impl Into<String>) -> Self {
                    self.group_id = Some(value.into());
                    self
                }
                #[doc = "Optional. The maximum number of results to return per response."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A `next_page_token` provided by a previous response."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Optional. The exact value to match against\n[`ServiceContext.resource_type`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.resource_type)."]
                pub fn service_filter_resource_type(mut self, value: impl Into<String>) -> Self {
                    self.service_filter_resource_type = Some(value.into());
                    self
                }
                #[doc = "Optional. The exact value to match against\n[`ServiceContext.service`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.service)."]
                pub fn service_filter_service(mut self, value: impl Into<String>) -> Self {
                    self.service_filter_service = Some(value.into());
                    self
                }
                #[doc = "Optional. The exact value to match against\n[`ServiceContext.version`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.version)."]
                pub fn service_filter_version(mut self, value: impl Into<String>) -> Self {
                    self.service_filter_version = Some(value.into());
                    self
                }
                #[doc = "Restricts the query to the specified time range."]
                pub fn time_range_period(
                    mut self,
                    value: crate::resources::projects::events::params::ListTimeRangePeriod,
                ) -> Self {
                    self.time_range_period = Some(value);
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
                pub fn iter_error_events<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_error_events_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_error_events_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::ErrorEvent> {
                    self.iter_error_events_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_error_events_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::ErrorEvent> {
                    self.iter_error_events_with_fields(Some("*"))
                }
                pub fn iter_error_events_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "errorEvents").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "errorEvents")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListEventsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListEventsResponse>
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
                ) -> Result<crate::schemas::ListEventsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListEventsResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://clouderrorreporting.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    {
                        let var_as_str = &self.project_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/events");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("groupId", &self.group_id)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[(
                        "serviceFilter.resourceType",
                        &self.service_filter_resource_type,
                    )]);
                    let req = req.query(&[("serviceFilter.service", &self.service_filter_service)]);
                    let req = req.query(&[("serviceFilter.version", &self.service_filter_version)]);
                    let req = req.query(&[("timeRange.period", &self.time_range_period)]);
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
            #[doc = "Created via [EventsActions::report()](struct.EventsActions.html#method.report)"]
            #[derive(Debug, Clone)]
            pub struct ReportRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ReportedErrorEvent,
                project_name: String,
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
            impl<'a> ReportRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::ReportErrorEventResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ReportErrorEventResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://clouderrorreporting.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    {
                        let var_as_str = &self.project_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/events:report");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod group_stats {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListAlignment {
                    AlignmentEqualAtEnd,
                    AlignmentEqualRounded,
                    ErrorCountAlignmentUnspecified,
                }
                impl ListAlignment {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListAlignment::AlignmentEqualAtEnd => "ALIGNMENT_EQUAL_AT_END",
                            ListAlignment::AlignmentEqualRounded => "ALIGNMENT_EQUAL_ROUNDED",
                            ListAlignment::ErrorCountAlignmentUnspecified => {
                                "ERROR_COUNT_ALIGNMENT_UNSPECIFIED"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListAlignment {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListAlignment {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListAlignment, ()> {
                        Ok(match s {
                            "ALIGNMENT_EQUAL_AT_END" => ListAlignment::AlignmentEqualAtEnd,
                            "ALIGNMENT_EQUAL_ROUNDED" => ListAlignment::AlignmentEqualRounded,
                            "ERROR_COUNT_ALIGNMENT_UNSPECIFIED" => {
                                ListAlignment::ErrorCountAlignmentUnspecified
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListAlignment {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListAlignment {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListAlignment {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ALIGNMENT_EQUAL_AT_END" => ListAlignment::AlignmentEqualAtEnd,
                            "ALIGNMENT_EQUAL_ROUNDED" => ListAlignment::AlignmentEqualRounded,
                            "ERROR_COUNT_ALIGNMENT_UNSPECIFIED" => {
                                ListAlignment::ErrorCountAlignmentUnspecified
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
                impl ::google_field_selector::FieldSelector for ListAlignment {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListAlignment {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListOrder {
                    AffectedUsersDesc,
                    CountDesc,
                    CreatedDesc,
                    GroupOrderUnspecified,
                    LastSeenDesc,
                }
                impl ListOrder {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListOrder::AffectedUsersDesc => "AFFECTED_USERS_DESC",
                            ListOrder::CountDesc => "COUNT_DESC",
                            ListOrder::CreatedDesc => "CREATED_DESC",
                            ListOrder::GroupOrderUnspecified => "GROUP_ORDER_UNSPECIFIED",
                            ListOrder::LastSeenDesc => "LAST_SEEN_DESC",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListOrder {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListOrder {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListOrder, ()> {
                        Ok(match s {
                            "AFFECTED_USERS_DESC" => ListOrder::AffectedUsersDesc,
                            "COUNT_DESC" => ListOrder::CountDesc,
                            "CREATED_DESC" => ListOrder::CreatedDesc,
                            "GROUP_ORDER_UNSPECIFIED" => ListOrder::GroupOrderUnspecified,
                            "LAST_SEEN_DESC" => ListOrder::LastSeenDesc,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListOrder {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListOrder {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListOrder {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "AFFECTED_USERS_DESC" => ListOrder::AffectedUsersDesc,
                            "COUNT_DESC" => ListOrder::CountDesc,
                            "CREATED_DESC" => ListOrder::CreatedDesc,
                            "GROUP_ORDER_UNSPECIFIED" => ListOrder::GroupOrderUnspecified,
                            "LAST_SEEN_DESC" => ListOrder::LastSeenDesc,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListOrder {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListOrder {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListTimeRangePeriod {
                    Period1Day,
                    Period1Hour,
                    Period1Week,
                    Period30Days,
                    Period6Hours,
                    PeriodUnspecified,
                }
                impl ListTimeRangePeriod {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListTimeRangePeriod::Period1Day => "PERIOD_1_DAY",
                            ListTimeRangePeriod::Period1Hour => "PERIOD_1_HOUR",
                            ListTimeRangePeriod::Period1Week => "PERIOD_1_WEEK",
                            ListTimeRangePeriod::Period30Days => "PERIOD_30_DAYS",
                            ListTimeRangePeriod::Period6Hours => "PERIOD_6_HOURS",
                            ListTimeRangePeriod::PeriodUnspecified => "PERIOD_UNSPECIFIED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListTimeRangePeriod {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListTimeRangePeriod {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListTimeRangePeriod, ()> {
                        Ok(match s {
                            "PERIOD_1_DAY" => ListTimeRangePeriod::Period1Day,
                            "PERIOD_1_HOUR" => ListTimeRangePeriod::Period1Hour,
                            "PERIOD_1_WEEK" => ListTimeRangePeriod::Period1Week,
                            "PERIOD_30_DAYS" => ListTimeRangePeriod::Period30Days,
                            "PERIOD_6_HOURS" => ListTimeRangePeriod::Period6Hours,
                            "PERIOD_UNSPECIFIED" => ListTimeRangePeriod::PeriodUnspecified,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListTimeRangePeriod {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListTimeRangePeriod {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListTimeRangePeriod {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "PERIOD_1_DAY" => ListTimeRangePeriod::Period1Day,
                            "PERIOD_1_HOUR" => ListTimeRangePeriod::Period1Hour,
                            "PERIOD_1_WEEK" => ListTimeRangePeriod::Period1Week,
                            "PERIOD_30_DAYS" => ListTimeRangePeriod::Period30Days,
                            "PERIOD_6_HOURS" => ListTimeRangePeriod::Period6Hours,
                            "PERIOD_UNSPECIFIED" => ListTimeRangePeriod::PeriodUnspecified,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListTimeRangePeriod {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListTimeRangePeriod {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct GroupStatsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> GroupStatsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists the specified groups."]
                pub fn list(&self, project_name: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        project_name: project_name.into(),
                        alignment: None,
                        alignment_time: None,
                        group_id: None,
                        order: None,
                        page_size: None,
                        page_token: None,
                        service_filter_resource_type: None,
                        service_filter_service: None,
                        service_filter_version: None,
                        time_range_period: None,
                        timed_count_duration: None,
                    }
                }
            }
            #[doc = "Created via [GroupStatsActions::list()](struct.GroupStatsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                project_name: String,
                alignment: Option<crate::resources::projects::group_stats::params::ListAlignment>,
                alignment_time: Option<String>,
                group_id: Option<Vec<String>>,
                order: Option<crate::resources::projects::group_stats::params::ListOrder>,
                page_size: Option<i32>,
                page_token: Option<String>,
                service_filter_resource_type: Option<String>,
                service_filter_service: Option<String>,
                service_filter_version: Option<String>,
                time_range_period:
                    Option<crate::resources::projects::group_stats::params::ListTimeRangePeriod>,
                timed_count_duration: Option<String>,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Optional. The alignment of the timed counts to be returned.\nDefault is `ALIGNMENT_EQUAL_AT_END`."]
                pub fn alignment(
                    mut self,
                    value: crate::resources::projects::group_stats::params::ListAlignment,
                ) -> Self {
                    self.alignment = Some(value);
                    self
                }
                #[doc = "Optional. Time where the timed counts shall be aligned if rounded\nalignment is chosen. Default is 00:00 UTC."]
                pub fn alignment_time(mut self, value: impl Into<String>) -> Self {
                    self.alignment_time = Some(value.into());
                    self
                }
                #[doc = "Optional. List all <code>ErrorGroupStats</code> with these IDs."]
                pub fn group_id(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.group_id = Some(value.into());
                    self
                }
                #[doc = "Optional. The sort order in which the results are returned.\nDefault is `COUNT_DESC`."]
                pub fn order(
                    mut self,
                    value: crate::resources::projects::group_stats::params::ListOrder,
                ) -> Self {
                    self.order = Some(value);
                    self
                }
                #[doc = "Optional. The maximum number of results to return per response.\nDefault is 20."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A `next_page_token` provided by a previous response. To view\nadditional results, pass this token along with the identical query\nparameters as the first request."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Optional. The exact value to match against\n[`ServiceContext.resource_type`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.resource_type)."]
                pub fn service_filter_resource_type(mut self, value: impl Into<String>) -> Self {
                    self.service_filter_resource_type = Some(value.into());
                    self
                }
                #[doc = "Optional. The exact value to match against\n[`ServiceContext.service`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.service)."]
                pub fn service_filter_service(mut self, value: impl Into<String>) -> Self {
                    self.service_filter_service = Some(value.into());
                    self
                }
                #[doc = "Optional. The exact value to match against\n[`ServiceContext.version`](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.version)."]
                pub fn service_filter_version(mut self, value: impl Into<String>) -> Self {
                    self.service_filter_version = Some(value.into());
                    self
                }
                #[doc = "Restricts the query to the specified time range."]
                pub fn time_range_period(
                    mut self,
                    value: crate::resources::projects::group_stats::params::ListTimeRangePeriod,
                ) -> Self {
                    self.time_range_period = Some(value);
                    self
                }
                #[doc = "Optional. The preferred duration for a single returned `TimedCount`.\nIf not set, no timed counts are returned."]
                pub fn timed_count_duration(mut self, value: impl Into<String>) -> Self {
                    self.timed_count_duration = Some(value.into());
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
                pub fn iter_error_group_stats<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_error_group_stats_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_error_group_stats_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::ErrorGroupStats>
                {
                    self.iter_error_group_stats_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_error_group_stats_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::ErrorGroupStats>
                {
                    self.iter_error_group_stats_with_fields(Some("*"))
                }
                pub fn iter_error_group_stats_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "errorGroupStats").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "errorGroupStats")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListGroupStatsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListGroupStatsResponse>
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
                ) -> Result<crate::schemas::ListGroupStatsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListGroupStatsResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://clouderrorreporting.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    {
                        let var_as_str = &self.project_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/groupStats");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("alignment", &self.alignment)]);
                    let req = req.query(&[("alignmentTime", &self.alignment_time)]);
                    let req = req.query(&[("groupId", &self.group_id)]);
                    let req = req.query(&[("order", &self.order)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[(
                        "serviceFilter.resourceType",
                        &self.service_filter_resource_type,
                    )]);
                    let req = req.query(&[("serviceFilter.service", &self.service_filter_service)]);
                    let req = req.query(&[("serviceFilter.version", &self.service_filter_version)]);
                    let req = req.query(&[("timeRange.period", &self.time_range_period)]);
                    let req = req.query(&[("timedCountDuration", &self.timed_count_duration)]);
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
        }
        pub mod groups {
            pub mod params {}
            pub struct GroupsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> GroupsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Get the specified group."]
                pub fn get(&self, group_name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        group_name: group_name.into(),
                    }
                }
                #[doc = "Replace the data for the specified group.\nFails if the group does not exist."]
                pub fn update(
                    &self,
                    request: crate::schemas::ErrorGroup,
                    name: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
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
                        name: name.into(),
                    }
                }
            }
            #[doc = "Created via [GroupsActions::get()](struct.GroupsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                group_name: String,
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
            impl<'a> GetRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::ErrorGroup, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ErrorGroup, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://clouderrorreporting.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
                    {
                        let var_as_str = &self.group_name;
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
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [GroupsActions::update()](struct.GroupsActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ErrorGroup,
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
            impl<'a> UpdateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::ErrorGroup, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ErrorGroup, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    let mut output = "https://clouderrorreporting.googleapis.com/".to_owned();
                    output.push_str("v1beta1/");
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
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    let req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
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
