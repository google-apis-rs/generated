#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [traces](resources/projects/traces/struct.TracesActions.html)\n        * [*batchWrite*](resources/projects/traces/struct.BatchWriteRequestBuilder.html)\n        * [spans](resources/projects/traces/spans/struct.SpansActions.html)\n          * [*createSpan*](resources/projects/traces/spans/struct.CreateSpanRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your data across Google Cloud Platform services\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "Write Trace data for a project or application\n\n`https://www.googleapis.com/auth/trace.append`"]
    pub const TRACE_APPEND: &str = "https://www.googleapis.com/auth/trace.append";
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
    pub struct Annotation {
        #[doc = "A set of attributes on the annotation. You can have up to 4 attributes\nper Annotation."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<crate::schemas::Attributes>,
        #[doc = "A user-supplied message describing the event. The maximum length for\nthe description is 256 bytes."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<crate::schemas::TruncatableString>,
    }
    impl ::google_field_selector::FieldSelector for Annotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Annotation {
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
    pub struct AttributeValue {
        #[doc = "A Boolean value represented by `true` or `false`."]
        #[serde(
            rename = "boolValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "A 64-bit signed integer."]
        #[serde(
            rename = "intValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub int_value: ::std::option::Option<i64>,
        #[doc = "A string up to 256 bytes long."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<crate::schemas::TruncatableString>,
    }
    impl ::google_field_selector::FieldSelector for AttributeValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttributeValue {
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
    pub struct Attributes {
        #[doc = "The set of attributes. Each attribute's key can be up to 128 bytes\nlong. The value can be a string up to 256 bytes, a signed 64-bit integer,\nor the Boolean values `true` and `false`. For example:\n\n````text\n\"/instance_id\": { \"string_value\": { \"value\": \"my-instance\" } }\n\"/http/request_bytes\": { \"int_value\": 300 }\n\"abc.com/myattribute\": { \"bool_value\": false }````"]
        #[serde(
            rename = "attributeMap",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribute_map: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::AttributeValue>,
        >,
        #[doc = "The number of attributes that were discarded. Attributes can be discarded\nbecause their keys are too long or because there are too many attributes.\nIf this value is 0 then all attributes are valid."]
        #[serde(
            rename = "droppedAttributesCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dropped_attributes_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Attributes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Attributes {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchWriteSpansRequest {
        #[doc = "Required. A list of new spans. The span names must not match existing\nspans, or the results are undefined."]
        #[serde(
            rename = "spans",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spans: ::std::option::Option<Vec<crate::schemas::Span>>,
    }
    impl ::google_field_selector::FieldSelector for BatchWriteSpansRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchWriteSpansRequest {
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
    pub struct Empty {}
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Link {
        #[doc = "A set of attributes on the link. You have have up to  32 attributes per\nlink."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<crate::schemas::Attributes>,
        #[doc = "The relationship of the current span relative to the linked span."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::LinkType>,
        #[doc = "The [SPAN_ID] for a span within a trace."]
        #[serde(
            rename = "spanId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub span_id: ::std::option::Option<String>,
        #[doc = "The [TRACE_ID] for a trace within a project."]
        #[serde(
            rename = "traceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trace_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Link {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Link {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinkType {
        #[doc = "The linked span is a child of the current span."]
        ChildLinkedSpan,
        #[doc = "The linked span is a parent of the current span."]
        ParentLinkedSpan,
        #[doc = "The relationship of the two spans is unknown."]
        TypeUnspecified,
    }
    impl LinkType {
        pub fn as_str(self) -> &'static str {
            match self {
                LinkType::ChildLinkedSpan => "CHILD_LINKED_SPAN",
                LinkType::ParentLinkedSpan => "PARENT_LINKED_SPAN",
                LinkType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LinkType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LinkType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LinkType, ()> {
            Ok(match s {
                "CHILD_LINKED_SPAN" => LinkType::ChildLinkedSpan,
                "PARENT_LINKED_SPAN" => LinkType::ParentLinkedSpan,
                "TYPE_UNSPECIFIED" => LinkType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LinkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHILD_LINKED_SPAN" => LinkType::ChildLinkedSpan,
                "PARENT_LINKED_SPAN" => LinkType::ParentLinkedSpan,
                "TYPE_UNSPECIFIED" => LinkType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LinkType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LinkType {
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
    pub struct Links {
        #[doc = "The number of dropped links after the maximum size was enforced. If\nthis value is 0, then no links were dropped."]
        #[serde(
            rename = "droppedLinksCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dropped_links_count: ::std::option::Option<i32>,
        #[doc = "A collection of links."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<Vec<crate::schemas::Link>>,
    }
    impl ::google_field_selector::FieldSelector for Links {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Links {
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
    pub struct MessageEvent {
        #[doc = "The number of compressed bytes sent or received. If missing assumed to\nbe the same size as uncompressed."]
        #[serde(
            rename = "compressedSizeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub compressed_size_bytes: ::std::option::Option<i64>,
        #[doc = "An identifier for the MessageEvent's message that can be used to match\nSENT and RECEIVED MessageEvents. It is recommended to be unique within\na Span."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "Type of MessageEvent. Indicates whether the message was sent or\nreceived."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::MessageEventType>,
        #[doc = "The number of uncompressed bytes sent or received."]
        #[serde(
            rename = "uncompressedSizeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub uncompressed_size_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MessageEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MessageEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MessageEventType {
        #[doc = "Indicates a received message."]
        Received,
        #[doc = "Indicates a sent message."]
        Sent,
        #[doc = "Unknown event type."]
        TypeUnspecified,
    }
    impl MessageEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                MessageEventType::Received => "RECEIVED",
                MessageEventType::Sent => "SENT",
                MessageEventType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MessageEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MessageEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MessageEventType, ()> {
            Ok(match s {
                "RECEIVED" => MessageEventType::Received,
                "SENT" => MessageEventType::Sent,
                "TYPE_UNSPECIFIED" => MessageEventType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MessageEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MessageEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MessageEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RECEIVED" => MessageEventType::Received,
                "SENT" => MessageEventType::Sent,
                "TYPE_UNSPECIFIED" => MessageEventType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MessageEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MessageEventType {
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
    pub struct Module {
        #[doc = "A unique identifier for the module, usually a hash of its\ncontents (up to 128 bytes)."]
        #[serde(
            rename = "buildId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_id: ::std::option::Option<crate::schemas::TruncatableString>,
        #[doc = "For example: main binary, kernel modules, and dynamic libraries\nsuch as libc.so, sharedlib.so (up to 256 bytes)."]
        #[serde(
            rename = "module",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub module: ::std::option::Option<crate::schemas::TruncatableString>,
    }
    impl ::google_field_selector::FieldSelector for Module {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Module {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Span {
        #[doc = "A set of attributes on the span. You can have up to 32 attributes per\nspan."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<crate::schemas::Attributes>,
        #[doc = "Optional. The number of child spans that were generated while this span\nwas active. If set, allows implementation to detect missing child spans."]
        #[serde(
            rename = "childSpanCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub child_span_count: ::std::option::Option<i32>,
        #[doc = "Required. A description of the span's operation (up to 128 bytes).\nStackdriver Trace displays the description in the\nGoogle Cloud Platform Console.\nFor example, the display name can be a qualified method name or a file name\nand a line number where the operation is called. A best practice is to use\nthe same display name within an application and at the same call point.\nThis makes it easier to correlate spans in different traces."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<crate::schemas::TruncatableString>,
        #[doc = "Required. The end time of the span. On the client side, this is the time kept by\nthe local machine where the span execution ends. On the server side, this\nis the time when the server application handler stops running."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Links associated with the span. You can have up to 128 links per Span."]
        #[serde(
            rename = "links",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub links: ::std::option::Option<crate::schemas::Links>,
        #[doc = "The resource name of the span in the following format:\n\n````text\nprojects/[PROJECT_ID]/traces/[TRACE_ID]/spans/SPAN_ID is a unique identifier for a trace within a project;\n````\n\nit is a 32-character hexadecimal encoding of a 16-byte array.\n\n[SPAN_ID] is a unique identifier for a span within a trace; it\nis a 16-character hexadecimal encoding of an 8-byte array."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The [SPAN_ID] of this span's parent span. If this is a root span,\nthen this field must be empty."]
        #[serde(
            rename = "parentSpanId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_span_id: ::std::option::Option<String>,
        #[doc = "Optional. Set this parameter to indicate whether this span is in\nthe same process as its parent. If you do not set this parameter,\nStackdriver Trace is unable to take advantage of this helpful\ninformation."]
        #[serde(
            rename = "sameProcessAsParentSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub same_process_as_parent_span: ::std::option::Option<bool>,
        #[doc = "Required. The [SPAN_ID] portion of the span's resource name."]
        #[serde(
            rename = "spanId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub span_id: ::std::option::Option<String>,
        #[doc = "Distinguishes between spans generated in a particular context. For example,\ntwo spans with the same name may be distinguished using `CLIENT` (caller)\nand `SERVER` (callee) to identify an RPC call."]
        #[serde(
            rename = "spanKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub span_kind: ::std::option::Option<crate::schemas::SpanSpanKind>,
        #[doc = "Stack trace captured at the start of the span."]
        #[serde(
            rename = "stackTrace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_trace: ::std::option::Option<crate::schemas::StackTrace>,
        #[doc = "Required. The start time of the span. On the client side, this is the time kept by\nthe local machine where the span execution starts. On the server side, this\nis the time when the server's application handler starts running."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Optional. The final status for this span."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::Status>,
        #[doc = "A set of time events. You can have up to 32 annotations and 128 message\nevents per span."]
        #[serde(
            rename = "timeEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_events: ::std::option::Option<crate::schemas::TimeEvents>,
    }
    impl ::google_field_selector::FieldSelector for Span {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Span {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SpanSpanKind {
        #[doc = "Indicates that the span covers the client-side wrapper around an RPC or\nother remote request."]
        Client,
        #[doc = "Indicates that the span describes consumer recieving a message from a\nbroker. Unlike client and  server, there is no direct critical path\nlatency relationship between producer and consumer spans (e.g. receiving\na message from a pubsub service subscription)."]
        Consumer,
        #[doc = "Indicates that the span is used internally. Default value."]
        Internal,
        #[doc = "Indicates that the span describes producer sending a message to a broker.\nUnlike client and  server, there is no direct critical path latency\nrelationship between producer and consumer spans (e.g. publishing a\nmessage to a pubsub service)."]
        Producer,
        #[doc = "Indicates that the span covers server-side handling of an RPC or other\nremote network request."]
        Server,
        #[doc = "Unspecified. Do NOT use as default.\nImplementations MAY assume SpanKind.INTERNAL to be default."]
        SpanKindUnspecified,
    }
    impl SpanSpanKind {
        pub fn as_str(self) -> &'static str {
            match self {
                SpanSpanKind::Client => "CLIENT",
                SpanSpanKind::Consumer => "CONSUMER",
                SpanSpanKind::Internal => "INTERNAL",
                SpanSpanKind::Producer => "PRODUCER",
                SpanSpanKind::Server => "SERVER",
                SpanSpanKind::SpanKindUnspecified => "SPAN_KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SpanSpanKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SpanSpanKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SpanSpanKind, ()> {
            Ok(match s {
                "CLIENT" => SpanSpanKind::Client,
                "CONSUMER" => SpanSpanKind::Consumer,
                "INTERNAL" => SpanSpanKind::Internal,
                "PRODUCER" => SpanSpanKind::Producer,
                "SERVER" => SpanSpanKind::Server,
                "SPAN_KIND_UNSPECIFIED" => SpanSpanKind::SpanKindUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SpanSpanKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SpanSpanKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SpanSpanKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLIENT" => SpanSpanKind::Client,
                "CONSUMER" => SpanSpanKind::Consumer,
                "INTERNAL" => SpanSpanKind::Internal,
                "PRODUCER" => SpanSpanKind::Producer,
                "SERVER" => SpanSpanKind::Server,
                "SPAN_KIND_UNSPECIFIED" => SpanSpanKind::SpanKindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SpanSpanKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpanSpanKind {
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
    pub struct StackFrame {
        #[doc = "The column number where the function call appears, if available.\nThis is important in JavaScript because of its anonymous functions."]
        #[serde(
            rename = "columnNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub column_number: ::std::option::Option<i64>,
        #[doc = "The name of the source file where the function call appears (up to 256\nbytes)."]
        #[serde(
            rename = "fileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_name: ::std::option::Option<crate::schemas::TruncatableString>,
        #[doc = "The fully-qualified name that uniquely identifies the function or\nmethod that is active in this frame (up to 1024 bytes)."]
        #[serde(
            rename = "functionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function_name: ::std::option::Option<crate::schemas::TruncatableString>,
        #[doc = "The line number in `file_name` where the function call appears."]
        #[serde(
            rename = "lineNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub line_number: ::std::option::Option<i64>,
        #[doc = "The binary module from where the code was loaded."]
        #[serde(
            rename = "loadModule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub load_module: ::std::option::Option<crate::schemas::Module>,
        #[doc = "An un-mangled function name, if `function_name` is\n[mangled](http://www.avabodh.com/cxxin/namemangling.html). The name can\nbe fully-qualified (up to 1024 bytes)."]
        #[serde(
            rename = "originalFunctionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub original_function_name: ::std::option::Option<crate::schemas::TruncatableString>,
        #[doc = "The version of the deployed source code (up to 128 bytes)."]
        #[serde(
            rename = "sourceVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_version: ::std::option::Option<crate::schemas::TruncatableString>,
    }
    impl ::google_field_selector::FieldSelector for StackFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StackFrame {
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
    pub struct StackFrames {
        #[doc = "The number of stack frames that were dropped because there\nwere too many stack frames.\nIf this value is 0, then no stack frames were dropped."]
        #[serde(
            rename = "droppedFramesCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dropped_frames_count: ::std::option::Option<i32>,
        #[doc = "Stack frames in this call stack."]
        #[serde(
            rename = "frame",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame: ::std::option::Option<Vec<crate::schemas::StackFrame>>,
    }
    impl ::google_field_selector::FieldSelector for StackFrames {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StackFrames {
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
    pub struct StackTrace {
        #[doc = "Stack frames in this stack trace. A maximum of 128 frames are allowed."]
        #[serde(
            rename = "stackFrames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stack_frames: ::std::option::Option<crate::schemas::StackFrames>,
        #[doc = "The hash ID is used to conserve network bandwidth for duplicate\nstack traces within a single trace.\n\nOften multiple spans will have identical stack traces.\nThe first occurrence of a stack trace should contain both the\n`stackFrame` content and a value in `stackTraceHashId`.\n\nSubsequent spans within the same request can refer\nto that stack trace by only setting `stackTraceHashId`."]
        #[serde(
            rename = "stackTraceHashId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub stack_trace_hash_id: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for StackTrace {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StackTrace {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
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
    pub struct TimeEvent {
        #[doc = "Text annotation with a set of attributes."]
        #[serde(
            rename = "annotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation: ::std::option::Option<crate::schemas::Annotation>,
        #[doc = "An event describing a message sent/received between Spans."]
        #[serde(
            rename = "messageEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_event: ::std::option::Option<crate::schemas::MessageEvent>,
        #[doc = "The timestamp indicating the time the event occurred."]
        #[serde(
            rename = "time",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimeEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeEvent {
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
    pub struct TimeEvents {
        #[doc = "The number of dropped annotations in all the included time events.\nIf the value is 0, then no annotations were dropped."]
        #[serde(
            rename = "droppedAnnotationsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dropped_annotations_count: ::std::option::Option<i32>,
        #[doc = "The number of dropped message events in all the included time events.\nIf the value is 0, then no message events were dropped."]
        #[serde(
            rename = "droppedMessageEventsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dropped_message_events_count: ::std::option::Option<i32>,
        #[doc = "A collection of `TimeEvent`s."]
        #[serde(
            rename = "timeEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_event: ::std::option::Option<Vec<crate::schemas::TimeEvent>>,
    }
    impl ::google_field_selector::FieldSelector for TimeEvents {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeEvents {
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
    pub struct TruncatableString {
        #[doc = "The number of bytes removed from the original string. If this\nvalue is 0, then the string was not shortened."]
        #[serde(
            rename = "truncatedByteCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub truncated_byte_count: ::std::option::Option<i32>,
        #[doc = "The shortened string. For example, if the original string is 500\nbytes long and the limit of the string is 128 bytes, then\n`value` contains the first 128 bytes of the 500-byte string.\n\nTruncation always happens on a UTF8 character boundary. If there\nare multi-byte characters in the string, then the length of the\nshortened string might be less than the size limit."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TruncatableString {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TruncatableString {
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
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
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
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest,
            auth: auth.into(),
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
            #[doc = "Actions that can be performed on the traces resource"]
            pub fn traces(&self) -> crate::resources::projects::traces::TracesActions {
                crate::resources::projects::traces::TracesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod traces {
            pub mod params {}
            pub struct TracesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> TracesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Sends new spans to new or existing traces. You cannot update\nexisting spans.\nIn this case, writing traces is not considered an active developer\nmethod since traces are machine generated."]
                pub fn batch_write(
                    &self,
                    request: crate::schemas::BatchWriteSpansRequest,
                    name: impl Into<String>,
                ) -> BatchWriteRequestBuilder {
                    BatchWriteRequestBuilder {
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
                #[doc = "Actions that can be performed on the spans resource"]
                pub fn spans(&self) -> crate::resources::projects::traces::spans::SpansActions {
                    crate::resources::projects::traces::spans::SpansActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [TracesActions::batch_write()](struct.TracesActions.html#method.batch_write)"]
            #[derive(Debug, Clone)]
            pub struct BatchWriteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchWriteSpansRequest,
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
            impl<'a> BatchWriteRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    let mut output = "https://cloudtrace.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/traces:batchWrite");
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
            pub mod spans {
                pub mod params {}
                pub struct SpansActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> SpansActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates a new span.\nIn this case, writing traces is not considered an active developer\nmethod since traces are machine generated."]
                    pub fn create_span(
                        &self,
                        request: crate::schemas::Span,
                        name: impl Into<String>,
                    ) -> CreateSpanRequestBuilder {
                        CreateSpanRequestBuilder {
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
                #[doc = "Created via [SpansActions::create_span()](struct.SpansActions.html#method.create_span)"]
                #[derive(Debug, Clone)]
                pub struct CreateSpanRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Span,
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
                impl<'a> CreateSpanRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::Span, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Span, crate::Error> {
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
                        let mut output = "https://cloudtrace.googleapis.com/".to_owned();
                        output.push_str("v2/");
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
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
