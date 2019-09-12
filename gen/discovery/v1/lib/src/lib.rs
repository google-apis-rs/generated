#![doc = "# Resources and Methods\n    * [apis](resources/apis/struct.ApisActions.html)\n      * [*getRest*](resources/apis/struct.GetRestRequestBuilder.html), [*list*](resources/apis/struct.ListRequestBuilder.html)\n"]
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
    pub struct DirectoryList {
        #[doc = "Indicate the version of the Discovery API used to generate this doc."]
        #[serde(
            rename = "discoveryVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_version: ::std::option::Option<String>,
        #[doc = "The individual directory entries. One entry per api/version pair."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::DirectoryListItemsItems>>,
        #[doc = "The kind for this response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DirectoryList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DirectoryList {
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
    pub struct DirectoryListItemsItems {
        #[doc = "The description of this API."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "A link to the discovery document."]
        #[serde(
            rename = "discoveryLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_link: ::std::option::Option<String>,
        #[doc = "The URL for the discovery REST document."]
        #[serde(
            rename = "discoveryRestUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_rest_url: ::std::option::Option<String>,
        #[doc = "A link to human readable documentation for the API."]
        #[serde(
            rename = "documentationLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation_link: ::std::option::Option<String>,
        #[doc = "Links to 16x16 and 32x32 icons representing the API."]
        #[serde(
            rename = "icons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icons: ::std::option::Option<crate::schemas::DirectoryListItemsItemsIcons>,
        #[doc = "The id of this API."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The kind for this response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Labels for the status of this API, such as labs or deprecated."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the API."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "True if this version is the preferred version to use."]
        #[serde(
            rename = "preferred",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preferred: ::std::option::Option<bool>,
        #[doc = "The title of this API."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The version of the API."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DirectoryListItemsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DirectoryListItemsItems {
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
    pub struct DirectoryListItemsItemsIcons {
        #[doc = "The URL of the 16x16 icon."]
        #[serde(
            rename = "x16",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x_16: ::std::option::Option<String>,
        #[doc = "The URL of the 32x32 icon."]
        #[serde(
            rename = "x32",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x_32: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DirectoryListItemsItemsIcons {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DirectoryListItemsItemsIcons {
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
    pub struct JsonSchema {
        #[doc = "If this is a schema for an object, this property is the schema for any additional properties with dynamic keys on this object."]
        #[serde(
            rename = "additionalProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_properties: ::std::option::Option<Box<crate::schemas::JsonSchema>>,
        #[doc = "Additional information about this property."]
        #[serde(
            rename = "annotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotations: ::std::option::Option<crate::schemas::JsonSchemaAnnotations>,
        #[doc = "The default value of this property (if one exists)."]
        #[serde(
            rename = "default",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default: ::std::option::Option<String>,
        #[doc = "A description of this object."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The descriptions for the enums. Each position maps to the corresponding value in the \"enum\" array."]
        #[serde(
            rename = "enumDescriptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enum_descriptions: ::std::option::Option<Vec<String>>,
        #[doc = "An additional regular expression or key that helps constrain the value. For more details see: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.23"]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<String>,
        #[doc = "Unique identifier for this schema."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "If this is a schema for an array, this property is the schema for each element in the array."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Box<crate::schemas::JsonSchema>>,
        #[doc = "Whether this parameter goes in the query or the path for REST requests."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The maximum value of this parameter."]
        #[serde(
            rename = "maximum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximum: ::std::option::Option<String>,
        #[doc = "The minimum value of this parameter."]
        #[serde(
            rename = "minimum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum: ::std::option::Option<String>,
        #[doc = "The regular expression this parameter must conform to. Uses Java 6 regex format: http://docs.oracle.com/javase/6/docs/api/java/util/regex/Pattern.html"]
        #[serde(
            rename = "pattern",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pattern: ::std::option::Option<String>,
        #[doc = "If this is a schema for an object, list the schema for each property of this object."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "Values this parameter may take (if it is an enum)."]
        #[serde(
            rename = "enum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#enum: ::std::option::Option<Vec<String>>,
        #[doc = "A reference to another schema. The value of this property is the \"id\" of another schema."]
        #[serde(
            rename = "$ref",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#ref: ::std::option::Option<String>,
        #[doc = "The value type for this schema. A list of values can be found here: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.1"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The value is read-only, generated by the service. The value cannot be modified by the client. If the value is included in a POST, PUT, or PATCH request, it is ignored by the service."]
        #[serde(
            rename = "readOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_only: ::std::option::Option<bool>,
        #[doc = "Whether this parameter may appear multiple times."]
        #[serde(
            rename = "repeated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repeated: ::std::option::Option<bool>,
        #[doc = "Whether the parameter is required."]
        #[serde(
            rename = "required",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required: ::std::option::Option<bool>,
        #[doc = "In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names."]
        #[serde(
            rename = "variant",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub variant: ::std::option::Option<crate::schemas::JsonSchemaVariant>,
    }
    impl ::google_field_selector::FieldSelector for JsonSchema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonSchema {
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
    pub struct JsonSchemaAnnotations {
        #[doc = "A list of methods for which this property is required on requests."]
        #[serde(
            rename = "required",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for JsonSchemaAnnotations {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonSchemaAnnotations {
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
    pub struct JsonSchemaVariant {
        #[doc = "The name of the type discriminant property."]
        #[serde(
            rename = "discriminant",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discriminant: ::std::option::Option<String>,
        #[doc = "The map of discriminant value to schema to use for parsing.."]
        #[serde(
            rename = "map",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub map: ::std::option::Option<Vec<crate::schemas::JsonSchemaVariantMapItems>>,
    }
    impl ::google_field_selector::FieldSelector for JsonSchemaVariant {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonSchemaVariant {
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
    pub struct JsonSchemaVariantMapItems {
        #[serde(
            rename = "$ref",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#ref: ::std::option::Option<String>,
        #[serde(
            rename = "type_value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JsonSchemaVariantMapItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonSchemaVariantMapItems {
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
    pub struct RestDescription {
        #[doc = "Authentication information."]
        #[serde(
            rename = "auth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auth: ::std::option::Option<crate::schemas::RestDescriptionAuth>,
        #[doc = "[DEPRECATED] The base path for REST requests."]
        #[serde(
            rename = "basePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_path: ::std::option::Option<String>,
        #[doc = "[DEPRECATED] The base URL for REST requests."]
        #[serde(
            rename = "baseUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_url: ::std::option::Option<String>,
        #[doc = "The path for REST batch requests."]
        #[serde(
            rename = "batchPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub batch_path: ::std::option::Option<String>,
        #[doc = "Indicates how the API name should be capitalized and split into various parts. Useful for generating pretty class names."]
        #[serde(
            rename = "canonicalName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub canonical_name: ::std::option::Option<String>,
        #[doc = "The description of this API."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Indicate the version of the Discovery API used to generate this doc."]
        #[serde(
            rename = "discoveryVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery_version: ::std::option::Option<String>,
        #[doc = "A link to human readable documentation for the API."]
        #[serde(
            rename = "documentationLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation_link: ::std::option::Option<String>,
        #[doc = "The ETag for this response."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Enable exponential backoff for suitable methods in the generated clients."]
        #[serde(
            rename = "exponentialBackoffDefault",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exponential_backoff_default: ::std::option::Option<bool>,
        #[doc = "A list of supported features for this API."]
        #[serde(
            rename = "features",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub features: ::std::option::Option<Vec<String>>,
        #[doc = "Links to 16x16 and 32x32 icons representing the API."]
        #[serde(
            rename = "icons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icons: ::std::option::Option<crate::schemas::RestDescriptionIcons>,
        #[doc = "The ID of this API."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The kind for this response."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Labels for the status of this API, such as labs or deprecated."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<String>>,
        #[doc = "API-level methods for this API."]
        #[serde(
            rename = "methods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub methods:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::RestMethod>>,
        #[doc = "The name of this API."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The domain of the owner of this API. Together with the ownerName and a packagePath values, this can be used to generate a library for this API which would have a unique fully qualified name."]
        #[serde(
            rename = "ownerDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owner_domain: ::std::option::Option<String>,
        #[doc = "The name of the owner of this API. See ownerDomain."]
        #[serde(
            rename = "ownerName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owner_name: ::std::option::Option<String>,
        #[doc = "The package of the owner of this API. See ownerDomain."]
        #[serde(
            rename = "packagePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_path: ::std::option::Option<String>,
        #[doc = "Common parameters that apply across all apis."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "The protocol described by this document."]
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The resources in this API."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::RestResource>,
        >,
        #[doc = "The version of this API."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<String>,
        #[doc = "The root URL under which all API services live."]
        #[serde(
            rename = "rootUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root_url: ::std::option::Option<String>,
        #[doc = "The schemas for this API."]
        #[serde(
            rename = "schemas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schemas:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "The base path for all REST requests."]
        #[serde(
            rename = "servicePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_path: ::std::option::Option<String>,
        #[doc = "The title of this API."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The version of this API."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
        #[serde(
            rename = "version_module",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_module: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for RestDescription {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestDescription {
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
    pub struct RestDescriptionAuth {
        #[doc = "OAuth 2.0 authentication information."]
        #[serde(
            rename = "oauth2",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub oauth_2: ::std::option::Option<crate::schemas::RestDescriptionAuthOauth2>,
    }
    impl ::google_field_selector::FieldSelector for RestDescriptionAuth {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestDescriptionAuth {
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
    pub struct RestDescriptionAuthOauth2 {
        #[doc = "Available OAuth 2.0 scopes."]
        #[serde(
            rename = "scopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scopes: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::RestDescriptionAuthOauth2ScopesAdditionalProperties,
            >,
        >,
    }
    impl ::google_field_selector::FieldSelector for RestDescriptionAuthOauth2 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestDescriptionAuthOauth2 {
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
    pub struct RestDescriptionAuthOauth2ScopesAdditionalProperties {
        #[doc = "Description of scope."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for RestDescriptionAuthOauth2ScopesAdditionalProperties
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestDescriptionAuthOauth2ScopesAdditionalProperties {
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
    pub struct RestDescriptionIcons {
        #[doc = "The URL of the 16x16 icon."]
        #[serde(
            rename = "x16",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x_16: ::std::option::Option<String>,
        #[doc = "The URL of the 32x32 icon."]
        #[serde(
            rename = "x32",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x_32: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestDescriptionIcons {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestDescriptionIcons {
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
    pub struct RestMethod {
        #[doc = "Description of this method."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Whether this method requires an ETag to be specified. The ETag is sent as an HTTP If-Match or If-None-Match header."]
        #[serde(
            rename = "etagRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag_required: ::std::option::Option<bool>,
        #[doc = "HTTP method used by this method."]
        #[serde(
            rename = "httpMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_method: ::std::option::Option<String>,
        #[doc = "A unique ID for this method. This property can be used to match methods between different versions of Discovery."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Media upload parameters."]
        #[serde(
            rename = "mediaUpload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub media_upload: ::std::option::Option<crate::schemas::RestMethodMediaUpload>,
        #[doc = "Ordered list of required parameters, serves as a hint to clients on how to structure their method signatures. The array is ordered such that the \"most-significant\" parameter appears first."]
        #[serde(
            rename = "parameterOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_order: ::std::option::Option<Vec<String>>,
        #[doc = "Details for all parameters in this method."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "The URI path of this REST method. Should be used in conjunction with the basePath property at the api-level."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "The schema for the request."]
        #[serde(
            rename = "request",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request: ::std::option::Option<crate::schemas::RestMethodRequest>,
        #[doc = "The schema for the response."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response: ::std::option::Option<crate::schemas::RestMethodResponse>,
        #[doc = "OAuth 2.0 scopes applicable to this method."]
        #[serde(
            rename = "scopes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scopes: ::std::option::Option<Vec<String>>,
        #[doc = "Whether this method supports media downloads."]
        #[serde(
            rename = "supportsMediaDownload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supports_media_download: ::std::option::Option<bool>,
        #[doc = "Whether this method supports media uploads."]
        #[serde(
            rename = "supportsMediaUpload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supports_media_upload: ::std::option::Option<bool>,
        #[doc = "Whether this method supports subscriptions."]
        #[serde(
            rename = "supportsSubscription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supports_subscription: ::std::option::Option<bool>,
        #[doc = "Indicates that downloads from this method should use the download service URL (i.e. \"/download\"). Only applies if the method supports media download."]
        #[serde(
            rename = "useMediaDownloadService",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_media_download_service: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for RestMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestMethod {
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
    pub struct RestMethodMediaUpload {
        #[doc = "MIME Media Ranges for acceptable media uploads to this method."]
        #[serde(
            rename = "accept",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accept: ::std::option::Option<Vec<String>>,
        #[doc = "Maximum size of a media upload, such as \"1MB\", \"2GB\" or \"3TB\"."]
        #[serde(
            rename = "maxSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_size: ::std::option::Option<String>,
        #[doc = "Supported upload protocols."]
        #[serde(
            rename = "protocols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocols: ::std::option::Option<crate::schemas::RestMethodMediaUploadProtocols>,
    }
    impl ::google_field_selector::FieldSelector for RestMethodMediaUpload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestMethodMediaUpload {
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
    pub struct RestMethodMediaUploadProtocols {
        #[doc = "Supports the Resumable Media Upload protocol."]
        #[serde(
            rename = "resumable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resumable:
            ::std::option::Option<crate::schemas::RestMethodMediaUploadProtocolsResumable>,
        #[doc = "Supports uploading as a single HTTP request."]
        #[serde(
            rename = "simple",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub simple: ::std::option::Option<crate::schemas::RestMethodMediaUploadProtocolsSimple>,
    }
    impl ::google_field_selector::FieldSelector for RestMethodMediaUploadProtocols {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestMethodMediaUploadProtocols {
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
    pub struct RestMethodMediaUploadProtocolsResumable {
        #[doc = "True if this endpoint supports uploading multipart media."]
        #[serde(
            rename = "multipart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multipart: ::std::option::Option<bool>,
        #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestMethodMediaUploadProtocolsResumable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestMethodMediaUploadProtocolsResumable {
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
    pub struct RestMethodMediaUploadProtocolsSimple {
        #[doc = "True if this endpoint supports upload multipart media."]
        #[serde(
            rename = "multipart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multipart: ::std::option::Option<bool>,
        #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestMethodMediaUploadProtocolsSimple {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestMethodMediaUploadProtocolsSimple {
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
    pub struct RestMethodRequest {
        #[doc = "parameter name."]
        #[serde(
            rename = "parameterName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameter_name: ::std::option::Option<String>,
        #[doc = "Schema ID for the request schema."]
        #[serde(
            rename = "$ref",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#ref: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestMethodRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestMethodRequest {
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
    pub struct RestMethodResponse {
        #[doc = "Schema ID for the response schema."]
        #[serde(
            rename = "$ref",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#ref: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestMethodResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestMethodResponse {
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
    pub struct RestResource {
        #[doc = "Methods on this resource."]
        #[serde(
            rename = "methods",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub methods:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::RestMethod>>,
        #[doc = "Sub-resources on this resource."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::RestResource>,
        >,
    }
    impl ::google_field_selector::FieldSelector for RestResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestResource {
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
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the apis resource"]
    pub fn apis(&self) -> crate::resources::apis::ApisActions {
        crate::resources::apis::ApisActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod apis {
        pub mod params {}
        pub struct ApisActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ApisActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieve the description of a particular version of an api."]
            pub fn get_rest(
                &self,
                api: impl Into<String>,
                version: impl Into<String>,
            ) -> GetRestRequestBuilder {
                GetRestRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    api: api.into(),
                    version: version.into(),
                }
            }
            #[doc = "Retrieve the list of APIs supported at this endpoint."]
            pub fn list(&self) -> ListRequestBuilder {
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
                    name: None,
                    preferred: None,
                }
            }
        }
        #[doc = "Created via [ApisActions::get_rest()](struct.ApisActions.html#method.get_rest)"]
        #[derive(Debug, Clone)]
        pub struct GetRestRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            api: String,
            version: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRestRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::RestDescription, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RestDescription, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/discovery/v1/".to_owned();
                output.push_str("apis/");
                {
                    let var_as_str = &self.api;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.version;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/rest");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [ApisActions::list()](struct.ApisActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: Option<String>,
            preferred: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Only include APIs with the given name."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
                self
            }
            #[doc = "Return only the preferred version of an API."]
            pub fn preferred(mut self, value: bool) -> Self {
                self.preferred = Some(value);
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
            ) -> Result<crate::schemas::DirectoryList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DirectoryList, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/discovery/v1/".to_owned();
                output.push_str("apis");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("name", &self.name)]);
                let req = req.query(&[("preferred", &self.preferred)]);
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
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest(::reqwest::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest(err) => err
                .get_ref()
                .and_then(|err| err.downcast_ref::<::serde_json::Error>()),
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest(err) => write!(f, "Reqwest Error: {}", err),
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
    fn from(err: ::reqwest::Error) -> Error {
        Error::Reqwest(err)
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
