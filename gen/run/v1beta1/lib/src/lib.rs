#![doc = "# Resources and Methods\n    * [customresourcedefinitions](resources/customresourcedefinitions/struct.CustomresourcedefinitionsActions.html)\n      * [*list*](resources/customresourcedefinitions/struct.ListRequestBuilder.html)\n    * [namespaces](resources/namespaces/struct.NamespacesActions.html)\n      * [customresourcedefinitions](resources/namespaces/customresourcedefinitions/struct.CustomresourcedefinitionsActions.html)\n        * [*get*](resources/namespaces/customresourcedefinitions/struct.GetRequestBuilder.html)\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [locations](resources/projects/locations/struct.LocationsActions.html)\n        * [customresourcedefinitions](resources/projects/locations/customresourcedefinitions/struct.CustomresourcedefinitionsActions.html)\n          * [*get*](resources/projects/locations/customresourcedefinitions/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/customresourcedefinitions/struct.ListRequestBuilder.html)\n"]
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
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CustomResourceColumnDefinition {
        #[doc = "description is a human readable description of this column.\n+optional"]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "format is an optional OpenAPI type definition for this column. The 'name'\nformat is applied to the primary identifier column to assist in clients\nidentifying column is the resource name. See\nhttps://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types\nfor more. +optional"]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<String>,
        #[doc = "JSONPath is a simple JSON path, i.e. with array notation."]
        #[serde(
            rename = "jsonPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json_path: ::std::option::Option<String>,
        #[doc = "name is a human readable name for the column."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "priority is an integer defining the relative importance of this column\ncompared to others. Lower numbers are considered higher priority. Columns\nthat may be omitted in limited space scenarios should be given a higher\npriority. +optional"]
        #[serde(
            rename = "priority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub priority: ::std::option::Option<i32>,
        #[doc = "type is an OpenAPI type definition for this column.\nSee\nhttps://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types\nfor more."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceColumnDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceColumnDefinition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomResourceDefinition {
        #[doc = "The API version for this call such as \"k8s.apiextensions.io/v1beta1\"."]
        #[serde(
            rename = "apiVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_version: ::std::option::Option<String>,
        #[doc = "The kind of resource, in this case always \"CustomResourceDefinition\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Metadata associated with this CustomResourceDefinition."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ObjectMeta>,
        #[doc = "Spec describes how the user wants the resources to appear"]
        #[serde(
            rename = "spec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec: ::std::option::Option<crate::schemas::CustomResourceDefinitionSpec>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceDefinition {
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
    pub struct CustomResourceDefinitionNames {
        #[doc = "Categories is a list of grouped resources custom resources belong to (e.g.\n'all') +optional"]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<String>>,
        #[doc = "Kind is the serialized kind of the resource.  It is normally CamelCase and\nsingular."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "ListKind is the serialized kind of the list for this resource.  Defaults to\n<kind>List. +optional"]
        #[serde(
            rename = "listKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_kind: ::std::option::Option<String>,
        #[doc = "Plural is the plural name of the resource to serve.  It must match the name\nof the CustomResourceDefinition-registration too: plural.group and it must\nbe all lowercase."]
        #[serde(
            rename = "plural",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub plural: ::std::option::Option<String>,
        #[doc = "ShortNames are short names for the resource.  It must be all lowercase.\n+optional"]
        #[serde(
            rename = "shortNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_names: ::std::option::Option<Vec<String>>,
        #[doc = "Singular is the singular name of the resource.  It must be all lowercase\nDefaults to lowercased <kind> +optional"]
        #[serde(
            rename = "singular",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub singular: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceDefinitionNames {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceDefinitionNames {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomResourceDefinitionSpec {
        #[doc = "AdditionalPrinterColumns are additional columns shown e.g. in kubectl next\nto the name. Defaults to a created-at column. +optional"]
        #[serde(
            rename = "additionalPrinterColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_printer_columns:
            ::std::option::Option<Vec<crate::schemas::CustomResourceColumnDefinition>>,
        #[doc = "Group is the group this resource belongs in"]
        #[serde(
            rename = "group",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<String>,
        #[doc = "Names are the names used to describe this custom resource"]
        #[serde(
            rename = "names",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub names: ::std::option::Option<crate::schemas::CustomResourceDefinitionNames>,
        #[doc = "Scope indicates whether this resource is cluster or namespace scoped.\nDefault is namespaced"]
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<String>,
        #[doc = "Subresources describes the subresources for CustomResources\n+optional"]
        #[serde(
            rename = "subresources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subresources: ::std::option::Option<crate::schemas::CustomResourceSubresources>,
        #[doc = "Validation describes the validation methods for CustomResources\n+optional"]
        #[serde(
            rename = "validation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub validation: ::std::option::Option<crate::schemas::CustomResourceValidation>,
        #[doc = "Version is the version this resource belongs in\nShould be always first item in Versions field if provided.\nOptional, but at least one of Version or Versions must be set.\nDeprecated: Please use `Versions`.\n+optional"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
        #[doc = "Versions is the list of all supported versions for this resource.\nIf Version field is provided, this field is optional.\nValidation: All versions must use the same validation schema for now. i.e.,\ntop level Validation field is applied to all of these versions. Order: The\nversion name will be used to compute the order. If the version string is\n\"kube-like\", it will sort above non \"kube-like\" version strings, which are\nordered lexicographically. \"Kube-like\" versions start with a \"v\", then are\nfollowed by a number (the major version), then optionally the string\n\"alpha\" or \"beta\" and another number (the minor version). These are sorted\nfirst by GA > beta > alpha (where GA is a version with no suffix such as\nbeta or alpha), and then by comparing major version, then minor version. An\nexample sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1,\nv12alpha1, v11alpha2, foo1, foo10. +optional"]
        #[serde(
            rename = "versions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub versions: ::std::option::Option<Vec<crate::schemas::CustomResourceDefinitionVersion>>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceDefinitionSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceDefinitionSpec {
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
    pub struct CustomResourceDefinitionVersion {
        #[doc = "Name is the version name, e.g. \u{201c}v1\u{201d}, \u{201c}v2beta1\u{201d}, etc."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Served is a flag enabling/disabling this version from being served via REST\nAPIs"]
        #[serde(
            rename = "served",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub served: ::std::option::Option<bool>,
        #[doc = "Storage flags the version as storage version. There must be exactly one\nflagged as storage version."]
        #[serde(
            rename = "storage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceDefinitionVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceDefinitionVersion {
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
    pub struct CustomResourceSubresourceScale {
        #[doc = "LabelSelectorPath defines the JSON path inside of a CustomResource that\ncorresponds to Scale.Status.Selector. Only JSON paths without the array\nnotation are allowed. Must be a JSON Path under .status. Must be set to\nwork with HPA. If there is no value under the given path in the\nCustomResource, the status label selector value in the /scale subresource\nwill default to the empty string. +optional"]
        #[serde(
            rename = "labelSelectorPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_selector_path: ::std::option::Option<String>,
        #[doc = "SpecReplicasPath defines the JSON path inside of a CustomResource that\ncorresponds to Scale.Spec.Replicas. Only JSON paths without the array\nnotation are allowed. Must be a JSON Path under .spec. If there is no value\nunder the given path in the CustomResource, the /scale subresource will\nreturn an error on GET."]
        #[serde(
            rename = "specReplicasPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spec_replicas_path: ::std::option::Option<String>,
        #[doc = "StatusReplicasPath defines the JSON path inside of a CustomResource that\ncorresponds to Scale.Status.Replicas. Only JSON paths without the array\nnotation are allowed. Must be a JSON Path under .status. If there is no\nvalue under the given path in the CustomResource, the status replica value\nin the /scale subresource will default to 0."]
        #[serde(
            rename = "statusReplicasPath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_replicas_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceSubresourceScale {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceSubresourceScale {
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
    pub struct CustomResourceSubresourceStatus {}
    impl ::google_field_selector::FieldSelector for CustomResourceSubresourceStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceSubresourceStatus {
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
    pub struct CustomResourceSubresources {
        #[doc = "Scale denotes the scale subresource for CustomResources\n+optional"]
        #[serde(
            rename = "scale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scale: ::std::option::Option<crate::schemas::CustomResourceSubresourceScale>,
        #[doc = "Status denotes the status subresource for CustomResources\n+optional"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::CustomResourceSubresourceStatus>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceSubresources {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceSubresources {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomResourceValidation {
        #[doc = "OpenAPIV3Schema is the OpenAPI v3 schema to be validated against.\n+optional"]
        #[serde(
            rename = "openAPIV3Schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_apiv3_schema: ::std::option::Option<crate::schemas::JsonschemaProps>,
    }
    impl ::google_field_selector::FieldSelector for CustomResourceValidation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomResourceValidation {
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
    pub struct ExternalDocumentation {
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExternalDocumentation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExternalDocumentation {
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
    pub struct Json {
        #[serde(
            rename = "raw",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub raw: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Json {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Json {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JsonschemaProps {
        #[serde(
            rename = "additionalItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_items: ::std::option::Option<Box<crate::schemas::JsonschemaPropsOrBool>>,
        #[serde(
            rename = "additionalProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_properties:
            ::std::option::Option<Box<crate::schemas::JsonschemaPropsOrBool>>,
        #[serde(
            rename = "allOf",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_of: ::std::option::Option<Vec<crate::schemas::JsonschemaProps>>,
        #[serde(
            rename = "anyOf",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub any_of: ::std::option::Option<Vec<crate::schemas::JsonschemaProps>>,
        #[serde(
            rename = "default",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default: ::std::option::Option<crate::schemas::Json>,
        #[serde(
            rename = "definitions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub definitions: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::JsonschemaProps>,
        >,
        #[serde(
            rename = "dependencies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dependencies: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::JsonschemaPropsOrStringArray>,
        >,
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[serde(
            rename = "example",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub example: ::std::option::Option<crate::schemas::Json>,
        #[serde(
            rename = "exclusiveMaximum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusive_maximum: ::std::option::Option<bool>,
        #[serde(
            rename = "exclusiveMinimum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusive_minimum: ::std::option::Option<bool>,
        #[serde(
            rename = "externalDocs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_docs: ::std::option::Option<crate::schemas::ExternalDocumentation>,
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<String>,
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Box<crate::schemas::JsonschemaPropsOrArray>>,
        #[serde(
            rename = "maxItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_items: ::std::option::Option<i64>,
        #[serde(
            rename = "maxLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_length: ::std::option::Option<i64>,
        #[serde(
            rename = "maxProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_properties: ::std::option::Option<i64>,
        #[serde(
            rename = "maximum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximum: ::std::option::Option<f64>,
        #[serde(
            rename = "minItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_items: ::std::option::Option<i64>,
        #[serde(
            rename = "minLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_length: ::std::option::Option<i64>,
        #[serde(
            rename = "minProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_properties: ::std::option::Option<i64>,
        #[serde(
            rename = "minimum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum: ::std::option::Option<f64>,
        #[serde(
            rename = "multipleOf",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multiple_of: ::std::option::Option<f64>,
        #[serde(
            rename = "not",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub not: ::std::option::Option<Box<crate::schemas::JsonschemaProps>>,
        #[serde(
            rename = "oneOf",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub one_of: ::std::option::Option<Vec<crate::schemas::JsonschemaProps>>,
        #[serde(
            rename = "pattern",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pattern: ::std::option::Option<String>,
        #[serde(
            rename = "patternProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pattern_properties: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::JsonschemaProps>,
        >,
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::JsonschemaProps>,
        >,
        #[serde(
            rename = "enum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#enum: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "ref",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#ref: ::std::option::Option<String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[serde(
            rename = "required",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<String>,
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[serde(
            rename = "uniqueItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unique_items: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for JsonschemaProps {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonschemaProps {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JsonschemaPropsOrArray {
        #[serde(
            rename = "jsonSchemas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json_schemas: ::std::option::Option<Vec<crate::schemas::JsonschemaProps>>,
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<Box<crate::schemas::JsonschemaProps>>,
    }
    impl ::google_field_selector::FieldSelector for JsonschemaPropsOrArray {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonschemaPropsOrArray {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JsonschemaPropsOrBool {
        #[serde(
            rename = "allows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allows: ::std::option::Option<bool>,
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<Box<crate::schemas::JsonschemaProps>>,
    }
    impl ::google_field_selector::FieldSelector for JsonschemaPropsOrBool {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonschemaPropsOrBool {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JsonschemaPropsOrStringArray {
        #[serde(
            rename = "property",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::JsonschemaProps>,
    }
    impl ::google_field_selector::FieldSelector for JsonschemaPropsOrStringArray {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JsonschemaPropsOrStringArray {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListCustomResourceDefinitionsResponse {
        #[doc = "The API version for this call such as \"k8s.apiextensions.io/v1beta1\"."]
        #[serde(
            rename = "apiVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_version: ::std::option::Option<String>,
        #[doc = "List of CustomResourceDefinitions."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::CustomResourceDefinition>>,
        #[doc = "The kind of this resource, in this case \"CustomResourceDefinitionList\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Metadata associated with this CustomResourceDefinition list."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ListMeta>,
        #[doc = "Locations that could not be reached."]
        #[serde(
            rename = "unreachable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unreachable: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ListCustomResourceDefinitionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCustomResourceDefinitionsResponse {
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
    pub struct ListMeta {
        #[doc = "continue may be set if the user set a limit on the number of items\nreturned, and indicates that the server has more data available. The value\nis opaque and may be used to issue another request to the endpoint that\nserved this list to retrieve the next set of available objects. Continuing\na list may not be possible if the server configuration has changed or more\nthan a few minutes have passed. The resourceVersion field returned when\nusing this continue value will be identical to the value in the first\nresponse."]
        #[serde(
            rename = "continue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#continue: ::std::option::Option<String>,
        #[doc = "String that identifies the server's internal version of this object that\ncan be used by clients to determine when objects have changed. Value must\nbe treated as opaque by clients and passed unmodified back to the server.\nPopulated by the system.\nRead-only.\nMore info:\nhttps://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency\n+optional"]
        #[serde(
            rename = "resourceVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_version: ::std::option::Option<String>,
        #[doc = "SelfLink is a URL representing this object.\nPopulated by the system.\nRead-only.\n+optional"]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListMeta {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListMeta {
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
    pub struct ObjectMeta {
        #[doc = "(Optional)\n\nAnnotations is an unstructured key value map stored with a resource that\nmay be set by external tools to store and retrieve arbitrary metadata. They\nare not queryable and should be preserved when modifying objects. More\ninfo: http://kubernetes.io/docs/user-guide/annotations"]
        #[serde(
            rename = "annotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotations: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "(Optional)\n\nCloud Run fully managed: not supported\n\nCloud Run for Anthos: supported\n\nThe name of the cluster which the object belongs to.\nThis is used to distinguish resources with same name and namespace in\ndifferent clusters. This field is not set anywhere right now and apiserver\nis going to ignore it if set in create or update request."]
        #[serde(
            rename = "clusterName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cluster_name: ::std::option::Option<String>,
        #[doc = "(Optional)\n\nCreationTimestamp is a timestamp representing the server time when this\nobject was created. It is not guaranteed to be set in happens-before order\nacross separate operations. Clients may not set this value. It is\nrepresented in RFC3339 form and is in UTC.\n\nPopulated by the system.\nRead-only.\nNull for lists.\nMore info:\nhttps://git.k8s.io/community/contributors/devel/api-conventions.md#metadata"]
        #[serde(
            rename = "creationTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_timestamp: ::std::option::Option<String>,
        #[doc = "(Optional)\n\nCloud Run fully managed: not supported\n\nCloud Run for Anthos: supported\n\nNumber of seconds allowed for this object to gracefully terminate before\nit will be removed from the system. Only set when deletionTimestamp is also\nset. May only be shortened. Read-only."]
        #[serde(
            rename = "deletionGracePeriodSeconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deletion_grace_period_seconds: ::std::option::Option<i32>,
        #[doc = "(Optional)\n\nCloud Run fully managed: not supported\n\nCloud Run for Anthos: supported\n\nDeletionTimestamp is RFC 3339 date and time at which this resource will be\ndeleted. This field is set by the server when a graceful deletion is\nrequested by the user, and is not directly settable by a client. The\nresource is expected to be deleted (no longer visible from resource lists,\nand not reachable by name) after the time in this field, once the\nfinalizers list is empty. As long as the finalizers list contains items,\ndeletion is blocked. Once the deletionTimestamp is set, this value may not\nbe unset or be set further into the future, although it may be shortened or\nthe resource may be deleted prior to this time. For example, a user may\nrequest that a pod is deleted in 30 seconds. The Kubelet will react by\nsending a graceful termination signal to the containers in the pod. After\nthat 30 seconds, the Kubelet will send a hard termination signal (SIGKILL)\nto the container and after cleanup, remove the pod from the API. In the\npresence of network partitions, this object may still exist after this\ntimestamp, until an administrator or automated process can determine the\nresource is fully terminated.\nIf not set, graceful deletion of the object has not been requested.\n\nPopulated by the system when a graceful deletion is requested.\nRead-only.\nMore info:\nhttps://git.k8s.io/community/contributors/devel/api-conventions.md#metadata"]
        #[serde(
            rename = "deletionTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deletion_timestamp: ::std::option::Option<String>,
        #[doc = "(Optional)\n\nCloud Run fully managed: not supported\n\nCloud Run for Anthos: supported\n\nMust be empty before the object is deleted from the registry. Each entry\nis an identifier for the responsible component that will remove the entry\nfrom the list. If the deletionTimestamp of the object is non-nil, entries\nin this list can only be removed.\n+patchStrategy=merge"]
        #[serde(
            rename = "finalizers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finalizers: ::std::option::Option<Vec<String>>,
        #[doc = "(Optional)\n\nCloud Run fully managed: not supported\n\nCloud Run for Anthos: supported\n\nGenerateName is an optional prefix, used by the server, to generate a\nunique name ONLY IF the Name field has not been provided. If this field is\nused, the name returned to the client will be different than the name\npassed. This value will also be combined with a unique suffix. The provided\nvalue has the same validation rules as the Name field, and may be truncated\nby the length of the suffix required to make the value unique on the\nserver.\n\nIf this field is specified and the generated name exists, the server will\nNOT return a 409 - instead, it will either return 201 Created or 500 with\nReason ServerTimeout indicating a unique name could not be found in the\ntime allotted, and the client should retry (optionally after the time\nindicated in the Retry-After header).\n\nApplied only if Name is not specified.\nMore info:\nhttps://git.k8s.io/community/contributors/devel/api-conventions.md#idempotency\nstring generateName = 2;"]
        #[serde(
            rename = "generateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub generate_name: ::std::option::Option<String>,
        #[doc = "(Optional)\n\nA sequence number representing a specific generation of the desired state.\nPopulated by the system. Read-only."]
        #[serde(
            rename = "generation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub generation: ::std::option::Option<i32>,
        #[doc = "(Optional)\n\nMap of string keys and values that can be used to organize and categorize\n(scope and select) objects. May match selectors of replication controllers\nand routes.\nMore info: http://kubernetes.io/docs/user-guide/labels"]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Name must be unique within a namespace, within a Cloud Run region.\nIs required when creating\nresources, although some resources may allow a client to request the\ngeneration of an appropriate name automatically. Name is primarily intended\nfor creation idempotence and configuration definition. Cannot be updated.\nMore info: http://kubernetes.io/docs/user-guide/identifiers#names\n+optional"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Namespace defines the space within each name must be unique, within a\nCloud Run region. In Cloud Run the namespace must be equal to either the\nproject ID or project number."]
        #[serde(
            rename = "namespace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace: ::std::option::Option<String>,
        #[doc = "(Optional)\n\nCloud Run fully managed: not supported\n\nCloud Run for Anthos: supported\n\nList of objects that own this object. If ALL objects in the list have\nbeen deleted, this object will be garbage collected."]
        #[serde(
            rename = "ownerReferences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owner_references: ::std::option::Option<Vec<crate::schemas::OwnerReference>>,
        #[doc = "(Optional)\n\nAn opaque value that represents the internal version of this object that\ncan be used by clients to determine when objects have changed. May be used\nfor optimistic concurrency, change detection, and the watch operation on a\nresource or set of resources. Clients must treat these values as opaque and\npassed unmodified back to the server. They may only be valid for a\nparticular resource or set of resources.\n\nPopulated by the system.\nRead-only.\nValue must be treated as opaque by clients and .\nMore info:\nhttps://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency"]
        #[serde(
            rename = "resourceVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_version: ::std::option::Option<String>,
        #[doc = "(Optional)\n\nSelfLink is a URL representing this object.\nPopulated by the system.\nRead-only.\nstring selfLink = 4;"]
        #[serde(
            rename = "selfLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub self_link: ::std::option::Option<String>,
        #[doc = "(Optional)\n\nUID is the unique in time and space value for this object. It is typically\ngenerated by the server on successful creation of a resource and is not\nallowed to change on PUT operations.\n\nPopulated by the system.\nRead-only.\nMore info: http://kubernetes.io/docs/user-guide/identifiers#uids"]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ObjectMeta {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectMeta {
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
    pub struct OwnerReference {
        #[doc = "API version of the referent."]
        #[serde(
            rename = "apiVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_version: ::std::option::Option<String>,
        #[doc = "If true, AND if the owner has the \"foregroundDeletion\" finalizer, then\nthe owner cannot be deleted from the key-value store until this\nreference is removed.\nDefaults to false.\nTo set this field, a user needs \"delete\" permission of the owner,\notherwise 422 (Unprocessable Entity) will be returned.\n+optional"]
        #[serde(
            rename = "blockOwnerDeletion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub block_owner_deletion: ::std::option::Option<bool>,
        #[doc = "If true, this reference points to the managing controller.\n+optional"]
        #[serde(
            rename = "controller",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub controller: ::std::option::Option<bool>,
        #[doc = "Kind of the referent.\nMore info:\nhttps://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds"]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Name of the referent.\nMore info: http://kubernetes.io/docs/user-guide/identifiers#names"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "UID of the referent.\nMore info: http://kubernetes.io/docs/user-guide/identifiers#uids"]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OwnerReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OwnerReference {
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
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
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
    #[doc = "Actions that can be performed on the customresourcedefinitions resource"]
    pub fn customresourcedefinitions(
        &self,
    ) -> crate::resources::customresourcedefinitions::CustomresourcedefinitionsActions {
        crate::resources::customresourcedefinitions::CustomresourcedefinitionsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the namespaces resource"]
    pub fn namespaces(&self) -> crate::resources::namespaces::NamespacesActions {
        crate::resources::namespaces::NamespacesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
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
    pub mod customresourcedefinitions {
        pub mod params {}
        pub struct CustomresourcedefinitionsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CustomresourcedefinitionsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Rpc to list custom resource definitions."]
            pub fn list(&self) -> ListRequestBuilder {
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
                    field_selector: None,
                    include_uninitialized: None,
                    label_selector: None,
                    limit: None,
                    parent: None,
                    r#continue: None,
                    resource_version: None,
                    watch: None,
                }
            }
        }
        #[doc = "Created via [CustomresourcedefinitionsActions::list()](struct.CustomresourcedefinitionsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            field_selector: Option<String>,
            include_uninitialized: Option<bool>,
            label_selector: Option<String>,
            limit: Option<i32>,
            parent: Option<String>,
            r#continue: Option<String>,
            resource_version: Option<String>,
            watch: Option<bool>,
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
            #[doc = "Allows to filter resources based on a specific value for a field name.\nSend this in a query string format. i.e. 'metadata.name%3Dlorem'.\nNot currently used by Cloud Run."]
            pub fn field_selector(mut self, value: impl Into<String>) -> Self {
                self.field_selector = Some(value.into());
                self
            }
            #[doc = "Not currently used by Cloud Run."]
            pub fn include_uninitialized(mut self, value: bool) -> Self {
                self.include_uninitialized = Some(value);
                self
            }
            #[doc = "Allows to filter resources based on a label. Supported operations are\n=, !=, exists, in, and notIn."]
            pub fn label_selector(mut self, value: impl Into<String>) -> Self {
                self.label_selector = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn limit(mut self, value: i32) -> Self {
                self.limit = Some(value);
                self
            }
            #[doc = "The project ID or project number from which the storages should\nbe listed."]
            pub fn parent(mut self, value: impl Into<String>) -> Self {
                self.parent = Some(value.into());
                self
            }
            #[doc = "Optional encoded string to continue paging."]
            pub fn r#continue(mut self, value: impl Into<String>) -> Self {
                self.r#continue = Some(value.into());
                self
            }
            #[doc = "The baseline resource version from which the list or watch operation should\nstart. Not currently used by Cloud Run."]
            pub fn resource_version(mut self, value: impl Into<String>) -> Self {
                self.resource_version = Some(value.into());
                self
            }
            #[doc = "Flag that indicates that the client expects to watch this resource as well.\nNot currently used by Cloud Run."]
            pub fn watch(mut self, value: bool) -> Self {
                self.watch = Some(value);
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
            ) -> Result<crate::schemas::ListCustomResourceDefinitionsResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListCustomResourceDefinitionsResponse, crate::Error>
            {
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
                let req = self._request(&self._path())?;
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://run.googleapis.com/".to_owned();
                output.push_str("apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("fieldSelector", &self.field_selector)]);
                let req = req.query(&[("includeUninitialized", &self.include_uninitialized)]);
                let req = req.query(&[("labelSelector", &self.label_selector)]);
                let req = req.query(&[("limit", &self.limit)]);
                let req = req.query(&[("parent", &self.parent)]);
                let req = req.query(&[("continue", &self.r#continue)]);
                let req = req.query(&[("resourceVersion", &self.resource_version)]);
                let req = req.query(&[("watch", &self.watch)]);
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
    pub mod namespaces {
        pub mod params {}
        pub struct NamespacesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> NamespacesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the customresourcedefinitions resource"]pub fn customresourcedefinitions ( & self ) -> crate :: resources :: namespaces :: customresourcedefinitions :: CustomresourcedefinitionsActions{
                crate :: resources :: namespaces :: customresourcedefinitions :: CustomresourcedefinitionsActions { reqwest : & self . reqwest , auth : self . auth_ref ( ) , }
            }
        }
        pub mod customresourcedefinitions {
            pub mod params {}
            pub struct CustomresourcedefinitionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> CustomresourcedefinitionsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Rpc to get information about a CustomResourceDefinition."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                        name: name.into(),
                    }
                }
            }
            #[doc = "Created via [CustomresourcedefinitionsActions::get()](struct.CustomresourcedefinitionsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
                ) -> Result<crate::schemas::CustomResourceDefinition, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::CustomResourceDefinition, crate::Error>
                {
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
                    let req = self._request(&self._path())?;
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://run.googleapis.com/".to_owned();
                    output.push_str("apis/apiextensions.k8s.io/v1beta1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
        }
    }
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
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the customresourcedefinitions resource"]pub fn customresourcedefinitions ( & self ) -> crate :: resources :: projects :: locations :: customresourcedefinitions :: CustomresourcedefinitionsActions{
                    crate :: resources :: projects :: locations :: customresourcedefinitions :: CustomresourcedefinitionsActions { reqwest : & self . reqwest , auth : self . auth_ref ( ) , }
                }
            }
            pub mod customresourcedefinitions {
                pub mod params {}
                pub struct CustomresourcedefinitionsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> CustomresourcedefinitionsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Rpc to get information about a CustomResourceDefinition."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Rpc to list custom resource definitions."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                            parent: parent.into(),
                            field_selector: None,
                            include_uninitialized: None,
                            label_selector: None,
                            limit: None,
                            r#continue: None,
                            resource_version: None,
                            watch: None,
                        }
                    }
                }
                #[doc = "Created via [CustomresourcedefinitionsActions::get()](struct.CustomresourcedefinitionsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
                    ) -> Result<crate::schemas::CustomResourceDefinition, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::CustomResourceDefinition, crate::Error>
                    {
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
                        let req = self._request(&self._path())?;
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://run.googleapis.com/".to_owned();
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
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [CustomresourcedefinitionsActions::list()](struct.CustomresourcedefinitionsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    field_selector: Option<String>,
                    include_uninitialized: Option<bool>,
                    label_selector: Option<String>,
                    limit: Option<i32>,
                    r#continue: Option<String>,
                    resource_version: Option<String>,
                    watch: Option<bool>,
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
                    #[doc = "Allows to filter resources based on a specific value for a field name.\nSend this in a query string format. i.e. 'metadata.name%3Dlorem'.\nNot currently used by Cloud Run."]
                    pub fn field_selector(mut self, value: impl Into<String>) -> Self {
                        self.field_selector = Some(value.into());
                        self
                    }
                    #[doc = "Not currently used by Cloud Run."]
                    pub fn include_uninitialized(mut self, value: bool) -> Self {
                        self.include_uninitialized = Some(value);
                        self
                    }
                    #[doc = "Allows to filter resources based on a label. Supported operations are\n=, !=, exists, in, and notIn."]
                    pub fn label_selector(mut self, value: impl Into<String>) -> Self {
                        self.label_selector = Some(value.into());
                        self
                    }
                    #[doc = ""]
                    pub fn limit(mut self, value: i32) -> Self {
                        self.limit = Some(value);
                        self
                    }
                    #[doc = "Optional encoded string to continue paging."]
                    pub fn r#continue(mut self, value: impl Into<String>) -> Self {
                        self.r#continue = Some(value.into());
                        self
                    }
                    #[doc = "The baseline resource version from which the list or watch operation should\nstart. Not currently used by Cloud Run."]
                    pub fn resource_version(mut self, value: impl Into<String>) -> Self {
                        self.resource_version = Some(value.into());
                        self
                    }
                    #[doc = "Flag that indicates that the client expects to watch this resource as well.\nNot currently used by Cloud Run."]
                    pub fn watch(mut self, value: bool) -> Self {
                        self.watch = Some(value);
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
                    ) -> Result<crate::schemas::ListCustomResourceDefinitionsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListCustomResourceDefinitionsResponse, crate::Error>
                    {
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
                        let req = self._request(&self._path())?;
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://run.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/customresourcedefinitions");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("fieldSelector", &self.field_selector)]);
                        let req =
                            req.query(&[("includeUninitialized", &self.include_uninitialized)]);
                        let req = req.query(&[("labelSelector", &self.label_selector)]);
                        let req = req.query(&[("limit", &self.limit)]);
                        let req = req.query(&[("continue", &self.r#continue)]);
                        let req = req.query(&[("resourceVersion", &self.resource_version)]);
                        let req = req.query(&[("watch", &self.watch)]);
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
