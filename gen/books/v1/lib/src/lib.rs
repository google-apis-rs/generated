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
        #[doc = "Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty."]
        #[serde(rename = "afterSelectedText", default)]
        pub after_selected_text: ::std::option::Option<String>,
        #[doc = "Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty."]
        #[serde(rename = "beforeSelectedText", default)]
        pub before_selected_text: ::std::option::Option<String>,
        #[doc = "Selection ranges sent from the client."]
        #[serde(rename = "clientVersionRanges", default)]
        pub client_version_ranges:
            ::std::option::Option<crate::schemas::AnnotationClientVersionRanges>,
        #[doc = "Timestamp for the created time of this annotation."]
        #[serde(rename = "created", default)]
        pub created: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Selection ranges for the most recent content version."]
        #[serde(rename = "currentVersionRanges", default)]
        pub current_version_ranges:
            ::std::option::Option<crate::schemas::AnnotationCurrentVersionRanges>,
        #[doc = "User-created data for this annotation."]
        #[serde(rename = "data", default)]
        pub data: ::std::option::Option<String>,
        #[doc = "Indicates that this annotation is deleted."]
        #[serde(rename = "deleted", default)]
        pub deleted: ::std::option::Option<bool>,
        #[doc = "The highlight style for this annotation."]
        #[serde(rename = "highlightStyle", default)]
        pub highlight_style: ::std::option::Option<String>,
        #[doc = "Id of this annotation, in the form of a GUID."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The layer this annotation is for."]
        #[serde(rename = "layerId", default)]
        pub layer_id: ::std::option::Option<String>,
        #[serde(rename = "layerSummary", default)]
        pub layer_summary: ::std::option::Option<crate::schemas::AnnotationLayerSummary>,
        #[doc = "Pages that this annotation spans."]
        #[serde(rename = "pageIds", default)]
        pub page_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Excerpt from the volume."]
        #[serde(rename = "selectedText", default)]
        pub selected_text: ::std::option::Option<String>,
        #[doc = "URL to this resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Timestamp for the last time this annotation was modified."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The volume that this annotation belongs to."]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
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
    pub struct AnnotationClientVersionRanges {
        #[doc = "Range in CFI format for this annotation sent by client."]
        #[serde(rename = "cfiRange", default)]
        pub cfi_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Content version the client sent in."]
        #[serde(rename = "contentVersion", default)]
        pub content_version: ::std::option::Option<String>,
        #[doc = "Range in GB image format for this annotation sent by client."]
        #[serde(rename = "gbImageRange", default)]
        pub gb_image_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Range in GB text format for this annotation sent by client."]
        #[serde(rename = "gbTextRange", default)]
        pub gb_text_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Range in image CFI format for this annotation sent by client."]
        #[serde(rename = "imageCfiRange", default)]
        pub image_cfi_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
    }
    impl ::google_field_selector::FieldSelector for AnnotationClientVersionRanges {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnnotationClientVersionRanges {
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
    pub struct AnnotationCurrentVersionRanges {
        #[doc = "Range in CFI format for this annotation for version above."]
        #[serde(rename = "cfiRange", default)]
        pub cfi_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Content version applicable to ranges below."]
        #[serde(rename = "contentVersion", default)]
        pub content_version: ::std::option::Option<String>,
        #[doc = "Range in GB image format for this annotation for version above."]
        #[serde(rename = "gbImageRange", default)]
        pub gb_image_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Range in GB text format for this annotation for version above."]
        #[serde(rename = "gbTextRange", default)]
        pub gb_text_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Range in image CFI format for this annotation for version above."]
        #[serde(rename = "imageCfiRange", default)]
        pub image_cfi_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
    }
    impl ::google_field_selector::FieldSelector for AnnotationCurrentVersionRanges {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnnotationCurrentVersionRanges {
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
    pub struct AnnotationLayerSummary {
        #[doc = "Maximum allowed characters on this layer, especially for the \"copy\" layer."]
        #[serde(rename = "allowedCharacterCount", default)]
        pub allowed_character_count: ::std::option::Option<i32>,
        #[doc = "Type of limitation on this layer. \"limited\" or \"unlimited\" for the \"copy\" layer."]
        #[serde(rename = "limitType", default)]
        pub limit_type: ::std::option::Option<String>,
        #[doc = "Remaining allowed characters on this layer, especially for the \"copy\" layer."]
        #[serde(rename = "remainingCharacterCount", default)]
        pub remaining_character_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for AnnotationLayerSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnnotationLayerSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Annotationdata {
        #[doc = "The type of annotation this data is for."]
        #[serde(rename = "annotationType", default)]
        pub annotation_type: ::std::option::Option<String>,
        #[serde(rename = "data", default)]
        pub data: ::std::option::Option<::serde_json::Value>,
        #[doc = "Base64 encoded data for this annotation data."]
        #[serde(rename = "encoded_data", default)]
        pub encoded_data: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Unique id for this annotation data."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Resource Type"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The Layer id for this data. *"]
        #[serde(rename = "layerId", default)]
        pub layer_id: ::std::option::Option<String>,
        #[doc = "URL for this resource. *"]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format)."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The volume id for this data. *"]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Annotationdata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Annotationdata {
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
    pub struct Annotations {
        #[doc = "A list of annotations."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Annotation>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number of annotations found. This may be greater than the number of notes returned in this response if results have been paginated."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Annotations {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Annotations {
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
    pub struct AnnotationsSummary {
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[serde(rename = "layers", default)]
        pub layers: ::std::option::Option<Vec<crate::schemas::AnnotationsSummaryLayersItems>>,
    }
    impl ::google_field_selector::FieldSelector for AnnotationsSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnnotationsSummary {
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
    pub struct AnnotationsSummaryLayersItems {
        #[serde(rename = "allowedCharacterCount", default)]
        pub allowed_character_count: ::std::option::Option<i32>,
        #[serde(rename = "layerId", default)]
        pub layer_id: ::std::option::Option<String>,
        #[serde(rename = "limitType", default)]
        pub limit_type: ::std::option::Option<String>,
        #[serde(rename = "remainingCharacterCount", default)]
        pub remaining_character_count: ::std::option::Option<i32>,
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::google_field_selector::FieldSelector for AnnotationsSummaryLayersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnnotationsSummaryLayersItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Annotationsdata {
        #[doc = "A list of Annotation Data."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Annotationdata>>,
        #[doc = "Resource type"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of volume annotations found."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Annotationsdata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Annotationsdata {
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
    pub struct BooksAnnotationsRange {
        #[doc = "The offset from the ending position."]
        #[serde(rename = "endOffset", default)]
        pub end_offset: ::std::option::Option<String>,
        #[doc = "The ending position for the range."]
        #[serde(rename = "endPosition", default)]
        pub end_position: ::std::option::Option<String>,
        #[doc = "The offset from the starting position."]
        #[serde(rename = "startOffset", default)]
        pub start_offset: ::std::option::Option<String>,
        #[doc = "The starting position for the range."]
        #[serde(rename = "startPosition", default)]
        pub start_position: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BooksAnnotationsRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooksAnnotationsRange {
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
    pub struct BooksCloudloadingResource {
        #[serde(rename = "author", default)]
        pub author: ::std::option::Option<String>,
        #[serde(rename = "processingState", default)]
        pub processing_state: ::std::option::Option<String>,
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BooksCloudloadingResource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooksCloudloadingResource {
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
    pub struct BooksVolumesRecommendedRateResponse {
        #[serde(rename = "consistency_token", default)]
        pub consistency_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BooksVolumesRecommendedRateResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooksVolumesRecommendedRateResponse {
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
    pub struct Bookshelf {
        #[doc = "Whether this bookshelf is PUBLIC or PRIVATE."]
        #[serde(rename = "access", default)]
        pub access: ::std::option::Option<String>,
        #[doc = "Created time for this bookshelf (formatted UTC timestamp with millisecond resolution)."]
        #[serde(rename = "created", default)]
        pub created: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Description of this bookshelf."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Id of this bookshelf, only unique by user."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<i32>,
        #[doc = "Resource type for bookshelf metadata."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "URL to this resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Title of this bookshelf."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "Last modified time of this bookshelf (formatted UTC timestamp with millisecond resolution)."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Number of volumes in this bookshelf."]
        #[serde(rename = "volumeCount", default)]
        pub volume_count: ::std::option::Option<i32>,
        #[doc = "Last time a volume was added or removed from this bookshelf (formatted UTC timestamp with millisecond resolution)."]
        #[serde(rename = "volumesLastUpdated", default)]
        pub volumes_last_updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::google_field_selector::FieldSelector for Bookshelf {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Bookshelf {
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
    pub struct Bookshelves {
        #[doc = "A list of bookshelves."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Bookshelf>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Bookshelves {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Bookshelves {
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
    pub struct Category {
        #[doc = "A list of onboarding categories."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::CategoryItemsItems>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Category {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Category {
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
    pub struct CategoryItemsItems {
        #[serde(rename = "badgeUrl", default)]
        pub badge_url: ::std::option::Option<String>,
        #[serde(rename = "categoryId", default)]
        pub category_id: ::std::option::Option<String>,
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CategoryItemsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CategoryItemsItems {
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
    pub struct ConcurrentAccessRestriction {
        #[doc = "Whether access is granted for this (user, device, volume)."]
        #[serde(rename = "deviceAllowed", default)]
        pub device_allowed: ::std::option::Option<bool>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The maximum number of concurrent access licenses for this volume."]
        #[serde(rename = "maxConcurrentDevices", default)]
        pub max_concurrent_devices: ::std::option::Option<i32>,
        #[doc = "Error/warning message."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
        #[doc = "Client nonce for verification. Download access and client-validation only."]
        #[serde(rename = "nonce", default)]
        pub nonce: ::std::option::Option<String>,
        #[doc = "Error/warning reason code."]
        #[serde(rename = "reasonCode", default)]
        pub reason_code: ::std::option::Option<String>,
        #[doc = "Whether this volume has any concurrent access restrictions."]
        #[serde(rename = "restricted", default)]
        pub restricted: ::std::option::Option<bool>,
        #[doc = "Response signature."]
        #[serde(rename = "signature", default)]
        pub signature: ::std::option::Option<String>,
        #[doc = "Client app identifier for verification. Download access and client-validation only."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<String>,
        #[doc = "Time in seconds for license auto-expiration."]
        #[serde(rename = "timeWindowSeconds", default)]
        pub time_window_seconds: ::std::option::Option<i32>,
        #[doc = "Identifies the volume for which this entry applies."]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ConcurrentAccessRestriction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConcurrentAccessRestriction {
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
    pub struct Dictlayerdata {
        #[serde(rename = "common", default)]
        pub common: ::std::option::Option<crate::schemas::DictlayerdataCommon>,
        #[serde(rename = "dict", default)]
        pub dict: ::std::option::Option<crate::schemas::DictlayerdataDict>,
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Dictlayerdata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dictlayerdata {
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
    pub struct DictlayerdataCommon {
        #[doc = "The display title and localized canonical name to use when searching for this entity on Google search."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataCommon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataCommon {
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
    pub struct DictlayerdataDict {
        #[doc = "The source, url and attribution for this dictionary data."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::DictlayerdataDictSource>,
        #[serde(rename = "words", default)]
        pub words: ::std::option::Option<Vec<crate::schemas::DictlayerdataDictWordsItems>>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDict {
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
    pub struct DictlayerdataDictSource {
        #[serde(rename = "attribution", default)]
        pub attribution: ::std::option::Option<String>,
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictSource {
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
    pub struct DictlayerdataDictWordsItems {
        #[serde(rename = "derivatives", default)]
        pub derivatives:
            ::std::option::Option<Vec<crate::schemas::DictlayerdataDictWordsItemsDerivativesItems>>,
        #[serde(rename = "examples", default)]
        pub examples:
            ::std::option::Option<Vec<crate::schemas::DictlayerdataDictWordsItemsExamplesItems>>,
        #[serde(rename = "senses", default)]
        pub senses:
            ::std::option::Option<Vec<crate::schemas::DictlayerdataDictWordsItemsSensesItems>>,
        #[doc = "The words with different meanings but not related words, e.g. \"go\" (game) and \"go\" (verb)."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::DictlayerdataDictWordsItemsSource>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItems {
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
    pub struct DictlayerdataDictWordsItemsDerivativesItems {
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<
            crate::schemas::DictlayerdataDictWordsItemsDerivativesItemsSource,
        >,
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItemsDerivativesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsDerivativesItems {
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
    pub struct DictlayerdataDictWordsItemsDerivativesItemsSource {
        #[serde(rename = "attribution", default)]
        pub attribution: ::std::option::Option<String>,
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItemsDerivativesItemsSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsDerivativesItemsSource {
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
    pub struct DictlayerdataDictWordsItemsExamplesItems {
        #[serde(rename = "source", default)]
        pub source:
            ::std::option::Option<crate::schemas::DictlayerdataDictWordsItemsExamplesItemsSource>,
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItemsExamplesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsExamplesItems {
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
    pub struct DictlayerdataDictWordsItemsExamplesItemsSource {
        #[serde(rename = "attribution", default)]
        pub attribution: ::std::option::Option<String>,
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItemsExamplesItemsSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsExamplesItemsSource {
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
    pub struct DictlayerdataDictWordsItemsSensesItems {
        #[serde(rename = "conjugations", default)]
        pub conjugations: ::std::option::Option<
            Vec<crate::schemas::DictlayerdataDictWordsItemsSensesItemsConjugationsItems>,
        >,
        #[serde(rename = "definitions", default)]
        pub definitions: ::std::option::Option<
            Vec<crate::schemas::DictlayerdataDictWordsItemsSensesItemsDefinitionsItems>,
        >,
        #[serde(rename = "partOfSpeech", default)]
        pub part_of_speech: ::std::option::Option<String>,
        #[serde(rename = "pronunciation", default)]
        pub pronunciation: ::std::option::Option<String>,
        #[serde(rename = "pronunciationUrl", default)]
        pub pronunciation_url: ::std::option::Option<String>,
        #[serde(rename = "source", default)]
        pub source:
            ::std::option::Option<crate::schemas::DictlayerdataDictWordsItemsSensesItemsSource>,
        #[serde(rename = "syllabification", default)]
        pub syllabification: ::std::option::Option<String>,
        #[serde(rename = "synonyms", default)]
        pub synonyms: ::std::option::Option<
            Vec<crate::schemas::DictlayerdataDictWordsItemsSensesItemsSynonymsItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItemsSensesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsSensesItems {
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
    pub struct DictlayerdataDictWordsItemsSensesItemsConjugationsItems {
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for DictlayerdataDictWordsItemsSensesItemsConjugationsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for DictlayerdataDictWordsItemsSensesItemsConjugationsItems
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
    pub struct DictlayerdataDictWordsItemsSensesItemsDefinitionsItems {
        #[serde(rename = "definition", default)]
        pub definition: ::std::option::Option<String>,
        #[serde(rename = "examples", default)]
        pub examples: ::std::option::Option<
            Vec<
                crate::schemas::DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItems,
            >,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for DictlayerdataDictWordsItemsSensesItemsDefinitionsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for DictlayerdataDictWordsItemsSensesItemsDefinitionsItems
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
    pub struct DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItems { # [ serde ( rename = "source" , default ) ] pub source : :: std :: option :: Option < crate :: schemas :: DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItemsSource > , # [ serde ( rename = "text" , default ) ] pub text : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItems
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
    pub struct DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItemsSource {
        #[serde(rename = "attribution", default)]
        pub attribution: ::std::option::Option<String>,
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItemsSource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for DictlayerdataDictWordsItemsSensesItemsDefinitionsItemsExamplesItemsSource
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
    pub struct DictlayerdataDictWordsItemsSensesItemsSource {
        #[serde(rename = "attribution", default)]
        pub attribution: ::std::option::Option<String>,
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItemsSensesItemsSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsSensesItemsSource {
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
    pub struct DictlayerdataDictWordsItemsSensesItemsSynonymsItems {
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<
            crate::schemas::DictlayerdataDictWordsItemsSensesItemsSynonymsItemsSource,
        >,
        #[serde(rename = "text", default)]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for DictlayerdataDictWordsItemsSensesItemsSynonymsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsSensesItemsSynonymsItems {
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
    pub struct DictlayerdataDictWordsItemsSensesItemsSynonymsItemsSource {
        #[serde(rename = "attribution", default)]
        pub attribution: ::std::option::Option<String>,
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for DictlayerdataDictWordsItemsSensesItemsSynonymsItemsSource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for DictlayerdataDictWordsItemsSensesItemsSynonymsItemsSource
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
    pub struct DictlayerdataDictWordsItemsSource {
        #[serde(rename = "attribution", default)]
        pub attribution: ::std::option::Option<String>,
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DictlayerdataDictWordsItemsSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DictlayerdataDictWordsItemsSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Discoveryclusters {
        #[serde(rename = "clusters", default)]
        pub clusters: ::std::option::Option<Vec<crate::schemas::DiscoveryclustersClustersItems>>,
        #[doc = "Resorce type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[serde(rename = "totalClusters", default)]
        pub total_clusters: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Discoveryclusters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Discoveryclusters {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DiscoveryclustersClustersItems {
        #[serde(rename = "banner_with_content_container", default)]
        pub banner_with_content_container: ::std::option::Option<
            crate::schemas::DiscoveryclustersClustersItemsBannerWithContentContainer,
        >,
        #[serde(rename = "subTitle", default)]
        pub sub_title: ::std::option::Option<String>,
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[serde(rename = "totalVolumes", default)]
        pub total_volumes: ::std::option::Option<i32>,
        #[serde(rename = "uid", default)]
        pub uid: ::std::option::Option<String>,
        #[serde(rename = "volumes", default)]
        pub volumes: ::std::option::Option<Vec<crate::schemas::Volume>>,
    }
    impl ::google_field_selector::FieldSelector for DiscoveryclustersClustersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryclustersClustersItems {
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
    pub struct DiscoveryclustersClustersItemsBannerWithContentContainer {
        #[serde(rename = "fillColorArgb", default)]
        pub fill_color_argb: ::std::option::Option<String>,
        #[serde(rename = "imageUrl", default)]
        pub image_url: ::std::option::Option<String>,
        #[serde(rename = "maskColorArgb", default)]
        pub mask_color_argb: ::std::option::Option<String>,
        #[serde(rename = "moreButtonText", default)]
        pub more_button_text: ::std::option::Option<String>,
        #[serde(rename = "moreButtonUrl", default)]
        pub more_button_url: ::std::option::Option<String>,
        #[serde(rename = "textColorArgb", default)]
        pub text_color_argb: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for DiscoveryclustersClustersItemsBannerWithContentContainer
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for DiscoveryclustersClustersItemsBannerWithContentContainer
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
    pub struct DownloadAccessRestriction {
        #[doc = "If restricted, whether access is granted for this (user, device, volume)."]
        #[serde(rename = "deviceAllowed", default)]
        pub device_allowed: ::std::option::Option<bool>,
        #[doc = "If restricted, the number of content download licenses already acquired (including the requesting client, if licensed)."]
        #[serde(rename = "downloadsAcquired", default)]
        pub downloads_acquired: ::std::option::Option<i32>,
        #[doc = "If deviceAllowed, whether access was just acquired with this request."]
        #[serde(rename = "justAcquired", default)]
        pub just_acquired: ::std::option::Option<bool>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "If restricted, the maximum number of content download licenses for this volume."]
        #[serde(rename = "maxDownloadDevices", default)]
        pub max_download_devices: ::std::option::Option<i32>,
        #[doc = "Error/warning message."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
        #[doc = "Client nonce for verification. Download access and client-validation only."]
        #[serde(rename = "nonce", default)]
        pub nonce: ::std::option::Option<String>,
        #[doc = "Error/warning reason code. Additional codes may be added in the future. 0 OK 100 ACCESS_DENIED_PUBLISHER_LIMIT 101 ACCESS_DENIED_LIMIT 200 WARNING_USED_LAST_ACCESS"]
        #[serde(rename = "reasonCode", default)]
        pub reason_code: ::std::option::Option<String>,
        #[doc = "Whether this volume has any download access restrictions."]
        #[serde(rename = "restricted", default)]
        pub restricted: ::std::option::Option<bool>,
        #[doc = "Response signature."]
        #[serde(rename = "signature", default)]
        pub signature: ::std::option::Option<String>,
        #[doc = "Client app identifier for verification. Download access and client-validation only."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<String>,
        #[doc = "Identifies the volume for which this entry applies."]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DownloadAccessRestriction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DownloadAccessRestriction {
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
    pub struct DownloadAccesses {
        #[doc = "A list of download access responses."]
        #[serde(rename = "downloadAccessList", default)]
        pub download_access_list:
            ::std::option::Option<Vec<crate::schemas::DownloadAccessRestriction>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DownloadAccesses {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DownloadAccesses {
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
    pub struct FamilyInfo {
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Family membership info of the user that made the request."]
        #[serde(rename = "membership", default)]
        pub membership: ::std::option::Option<crate::schemas::FamilyInfoMembership>,
    }
    impl ::google_field_selector::FieldSelector for FamilyInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FamilyInfo {
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
    pub struct FamilyInfoMembership {
        #[doc = "Restrictions on user buying and acquiring content."]
        #[serde(rename = "acquirePermission", default)]
        pub acquire_permission: ::std::option::Option<String>,
        #[doc = "The age group of the user."]
        #[serde(rename = "ageGroup", default)]
        pub age_group: ::std::option::Option<String>,
        #[doc = "The maximum allowed maturity rating for the user."]
        #[serde(rename = "allowedMaturityRating", default)]
        pub allowed_maturity_rating: ::std::option::Option<String>,
        #[serde(rename = "isInFamily", default)]
        pub is_in_family: ::std::option::Option<bool>,
        #[doc = "The role of the user in the family."]
        #[serde(rename = "role", default)]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FamilyInfoMembership {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FamilyInfoMembership {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Geolayerdata {
        #[serde(rename = "common", default)]
        pub common: ::std::option::Option<crate::schemas::GeolayerdataCommon>,
        #[serde(rename = "geo", default)]
        pub geo: ::std::option::Option<crate::schemas::GeolayerdataGeo>,
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Geolayerdata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Geolayerdata {
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
    pub struct GeolayerdataCommon {
        #[doc = "The language of the information url and description."]
        #[serde(rename = "lang", default)]
        pub lang: ::std::option::Option<String>,
        #[doc = "The URL for the preview image information."]
        #[serde(rename = "previewImageUrl", default)]
        pub preview_image_url: ::std::option::Option<String>,
        #[doc = "The description for this location."]
        #[serde(rename = "snippet", default)]
        pub snippet: ::std::option::Option<String>,
        #[doc = "The URL for information for this location. Ex: wikipedia link."]
        #[serde(rename = "snippetUrl", default)]
        pub snippet_url: ::std::option::Option<String>,
        #[doc = "The display title and localized canonical name to use when searching for this entity on Google search."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GeolayerdataCommon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeolayerdataCommon {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GeolayerdataGeo {
        #[doc = "The boundary of the location as a set of loops containing pairs of latitude, longitude coordinates."]
        #[serde(rename = "boundary", default)]
        pub boundary:
            ::std::option::Option<Vec<Vec<crate::schemas::GeolayerdataGeoBoundaryItemsItems>>>,
        #[doc = "The cache policy active for this data. EX: UNRESTRICTED, RESTRICTED, NEVER"]
        #[serde(rename = "cachePolicy", default)]
        pub cache_policy: ::std::option::Option<String>,
        #[doc = "The country code of the location."]
        #[serde(rename = "countryCode", default)]
        pub country_code: ::std::option::Option<String>,
        #[doc = "The latitude of the location."]
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "The longitude of the location."]
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
        #[doc = "The type of map that should be used for this location. EX: HYBRID, ROADMAP, SATELLITE, TERRAIN"]
        #[serde(rename = "mapType", default)]
        pub map_type: ::std::option::Option<String>,
        #[doc = "The viewport for showing this location. This is a latitude, longitude rectangle."]
        #[serde(rename = "viewport", default)]
        pub viewport: ::std::option::Option<crate::schemas::GeolayerdataGeoViewport>,
        #[doc = "The Zoom level to use for the map. Zoom levels between 0 (the lowest zoom level, in which the entire world can be seen on one map) to 21+ (down to individual buildings). See: https://developers.google.com/maps/documentation/staticmaps/#Zoomlevels"]
        #[serde(rename = "zoom", default)]
        pub zoom: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GeolayerdataGeo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeolayerdataGeo {
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
    pub struct GeolayerdataGeoBoundaryItemsItems {
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<u32>,
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<u32>,
    }
    impl ::google_field_selector::FieldSelector for GeolayerdataGeoBoundaryItemsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeolayerdataGeoBoundaryItemsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GeolayerdataGeoViewport {
        #[serde(rename = "hi", default)]
        pub hi: ::std::option::Option<crate::schemas::GeolayerdataGeoViewportHi>,
        #[serde(rename = "lo", default)]
        pub lo: ::std::option::Option<crate::schemas::GeolayerdataGeoViewportLo>,
    }
    impl ::google_field_selector::FieldSelector for GeolayerdataGeoViewport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeolayerdataGeoViewport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GeolayerdataGeoViewportHi {
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GeolayerdataGeoViewportHi {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeolayerdataGeoViewportHi {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GeolayerdataGeoViewportLo {
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GeolayerdataGeoViewportLo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeolayerdataGeoViewportLo {
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
    pub struct Layersummaries {
        #[doc = "A list of layer summary items."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Layersummary>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The total number of layer summaries found."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Layersummaries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Layersummaries {
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
    pub struct Layersummary {
        #[doc = "The number of annotations for this layer."]
        #[serde(rename = "annotationCount", default)]
        pub annotation_count: ::std::option::Option<i32>,
        #[doc = "The list of annotation types contained for this layer."]
        #[serde(rename = "annotationTypes", default)]
        pub annotation_types: ::std::option::Option<Vec<String>>,
        #[doc = "Link to get data for this annotation."]
        #[serde(rename = "annotationsDataLink", default)]
        pub annotations_data_link: ::std::option::Option<String>,
        #[doc = "The link to get the annotations for this layer."]
        #[serde(rename = "annotationsLink", default)]
        pub annotations_link: ::std::option::Option<String>,
        #[doc = "The content version this resource is for."]
        #[serde(rename = "contentVersion", default)]
        pub content_version: ::std::option::Option<String>,
        #[doc = "The number of data items for this layer."]
        #[serde(rename = "dataCount", default)]
        pub data_count: ::std::option::Option<i32>,
        #[doc = "Unique id of this layer summary."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Resource Type"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The layer id for this summary."]
        #[serde(rename = "layerId", default)]
        pub layer_id: ::std::option::Option<String>,
        #[doc = "URL to this resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Timestamp for the last time an item in this layer was updated. (RFC 3339 UTC date-time format)."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately."]
        #[serde(rename = "volumeAnnotationsVersion", default)]
        pub volume_annotations_version: ::std::option::Option<String>,
        #[doc = "The volume id this resource is for."]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Layersummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Layersummary {
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
    pub struct Metadata {
        #[doc = "A list of offline dictionary metadata."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::MetadataItemsItems>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
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
    pub struct MetadataItemsItems {
        #[serde(rename = "download_url", default)]
        pub download_url: ::std::option::Option<String>,
        #[serde(rename = "encrypted_key", default)]
        pub encrypted_key: ::std::option::Option<String>,
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[serde(rename = "size", default)]
        #[serde(with = "crate::parsed_string")]
        pub size: ::std::option::Option<i64>,
        #[serde(rename = "version", default)]
        #[serde(with = "crate::parsed_string")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MetadataItemsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetadataItemsItems {
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
    pub struct Notification {
        #[serde(rename = "body", default)]
        pub body: ::std::option::Option<String>,
        #[doc = "The list of crm experiment ids."]
        #[serde(rename = "crmExperimentIds", default)]
        pub crm_experiment_ids: ::std::option::Option<Vec<i64>>,
        #[serde(rename = "doc_id", default)]
        pub doc_id: ::std::option::Option<String>,
        #[serde(rename = "doc_type", default)]
        pub doc_type: ::std::option::Option<String>,
        #[serde(rename = "dont_show_notification", default)]
        pub dont_show_notification: ::std::option::Option<bool>,
        #[serde(rename = "iconUrl", default)]
        pub icon_url: ::std::option::Option<String>,
        #[serde(rename = "is_document_mature", default)]
        pub is_document_mature: ::std::option::Option<bool>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[serde(rename = "notificationGroup", default)]
        pub notification_group: ::std::option::Option<String>,
        #[serde(rename = "notification_type", default)]
        pub notification_type: ::std::option::Option<String>,
        #[serde(rename = "pcampaign_id", default)]
        pub pcampaign_id: ::std::option::Option<String>,
        #[serde(rename = "reason", default)]
        pub reason: ::std::option::Option<String>,
        #[serde(rename = "show_notification_settings_action", default)]
        pub show_notification_settings_action: ::std::option::Option<bool>,
        #[serde(rename = "targetUrl", default)]
        pub target_url: ::std::option::Option<String>,
        #[serde(rename = "timeToExpireMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub time_to_expire_ms: ::std::option::Option<i64>,
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Notification {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Notification {
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
    pub struct Offers {
        #[doc = "A list of offers."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::OffersItemsItems>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Offers {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Offers {
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
    pub struct OffersItemsItems {
        #[serde(rename = "artUrl", default)]
        pub art_url: ::std::option::Option<String>,
        #[serde(rename = "gservicesKey", default)]
        pub gservices_key: ::std::option::Option<String>,
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::OffersItemsItemsItemsItems>>,
    }
    impl ::google_field_selector::FieldSelector for OffersItemsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OffersItemsItems {
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
    pub struct OffersItemsItemsItemsItems {
        #[serde(rename = "author", default)]
        pub author: ::std::option::Option<String>,
        #[serde(rename = "canonicalVolumeLink", default)]
        pub canonical_volume_link: ::std::option::Option<String>,
        #[serde(rename = "coverUrl", default)]
        pub cover_url: ::std::option::Option<String>,
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OffersItemsItemsItemsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OffersItemsItemsItemsItems {
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
    pub struct ReadingPosition {
        #[doc = "Position in an EPUB as a CFI."]
        #[serde(rename = "epubCfiPosition", default)]
        pub epub_cfi_position: ::std::option::Option<String>,
        #[doc = "Position in a volume for image-based content."]
        #[serde(rename = "gbImagePosition", default)]
        pub gb_image_position: ::std::option::Option<String>,
        #[doc = "Position in a volume for text-based content."]
        #[serde(rename = "gbTextPosition", default)]
        pub gb_text_position: ::std::option::Option<String>,
        #[doc = "Resource type for a reading position."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Position in a PDF file."]
        #[serde(rename = "pdfPosition", default)]
        pub pdf_position: ::std::option::Option<String>,
        #[doc = "Timestamp when this reading position was last updated (formatted UTC timestamp with millisecond resolution)."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Volume id associated with this reading position."]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReadingPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReadingPosition {
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
    pub struct RequestAccess {
        #[doc = "A concurrent access response."]
        #[serde(rename = "concurrentAccess", default)]
        pub concurrent_access: ::std::option::Option<crate::schemas::ConcurrentAccessRestriction>,
        #[doc = "A download access response."]
        #[serde(rename = "downloadAccess", default)]
        pub download_access: ::std::option::Option<crate::schemas::DownloadAccessRestriction>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RequestAccess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RequestAccess {
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
    pub struct Review {
        #[doc = "Author of this review."]
        #[serde(rename = "author", default)]
        pub author: ::std::option::Option<crate::schemas::ReviewAuthor>,
        #[doc = "Review text."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<String>,
        #[doc = "Date of this review."]
        #[serde(rename = "date", default)]
        pub date: ::std::option::Option<String>,
        #[doc = "URL for the full review text, for reviews gathered from the web."]
        #[serde(rename = "fullTextUrl", default)]
        pub full_text_url: ::std::option::Option<String>,
        #[doc = "Resource type for a review."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Source type for this review. Possible values are EDITORIAL, WEB_USER or GOOGLE_USER."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Star rating for this review. Possible values are ONE, TWO, THREE, FOUR, FIVE or NOT_RATED."]
        #[serde(rename = "rating", default)]
        pub rating: ::std::option::Option<String>,
        #[doc = "Information regarding the source of this review, when the review is not from a Google Books user."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::ReviewSource>,
        #[doc = "Title for this review."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "Volume that this review is for."]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Review {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Review {
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
    pub struct ReviewAuthor {
        #[doc = "Name of this person."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReviewAuthor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReviewAuthor {
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
    pub struct ReviewSource {
        #[doc = "Name of the source."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Extra text about the source of the review."]
        #[serde(rename = "extraDescription", default)]
        pub extra_description: ::std::option::Option<String>,
        #[doc = "URL of the source of the review."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReviewSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReviewSource {
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
    pub struct Series {
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[serde(rename = "series", default)]
        pub series: ::std::option::Option<Vec<crate::schemas::SeriesSeriesItems>>,
    }
    impl ::google_field_selector::FieldSelector for Series {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Series {
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
    pub struct SeriesSeriesItems {
        #[serde(rename = "bannerImageUrl", default)]
        pub banner_image_url: ::std::option::Option<String>,
        #[serde(rename = "imageUrl", default)]
        pub image_url: ::std::option::Option<String>,
        #[serde(rename = "seriesId", default)]
        pub series_id: ::std::option::Option<String>,
        #[serde(rename = "seriesType", default)]
        pub series_type: ::std::option::Option<String>,
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SeriesSeriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SeriesSeriesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Seriesmembership {
        #[doc = "Resorce type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[serde(rename = "member", default)]
        pub member: ::std::option::Option<Vec<crate::schemas::Volume>>,
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Seriesmembership {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Seriesmembership {
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
    pub struct Usersettings {
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "User settings in sub-objects, each for different purposes."]
        #[serde(rename = "notesExport", default)]
        pub notes_export: ::std::option::Option<crate::schemas::UsersettingsNotesExport>,
        #[serde(rename = "notification", default)]
        pub notification: ::std::option::Option<crate::schemas::UsersettingsNotification>,
    }
    impl ::google_field_selector::FieldSelector for Usersettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Usersettings {
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
    pub struct UsersettingsNotesExport {
        #[serde(rename = "folderName", default)]
        pub folder_name: ::std::option::Option<String>,
        #[serde(rename = "isEnabled", default)]
        pub is_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for UsersettingsNotesExport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersettingsNotesExport {
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
    pub struct UsersettingsNotification {
        #[serde(rename = "matchMyInterests", default)]
        pub match_my_interests:
            ::std::option::Option<crate::schemas::UsersettingsNotificationMatchMyInterests>,
        #[serde(rename = "moreFromAuthors", default)]
        pub more_from_authors:
            ::std::option::Option<crate::schemas::UsersettingsNotificationMoreFromAuthors>,
        #[serde(rename = "moreFromSeries", default)]
        pub more_from_series:
            ::std::option::Option<crate::schemas::UsersettingsNotificationMoreFromSeries>,
        #[serde(rename = "priceDrop", default)]
        pub price_drop: ::std::option::Option<crate::schemas::UsersettingsNotificationPriceDrop>,
        #[serde(rename = "rewardExpirations", default)]
        pub reward_expirations:
            ::std::option::Option<crate::schemas::UsersettingsNotificationRewardExpirations>,
    }
    impl ::google_field_selector::FieldSelector for UsersettingsNotification {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersettingsNotification {
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
    pub struct UsersettingsNotificationMatchMyInterests {
        #[serde(rename = "opted_state", default)]
        pub opted_state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsersettingsNotificationMatchMyInterests {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersettingsNotificationMatchMyInterests {
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
    pub struct UsersettingsNotificationMoreFromAuthors {
        #[serde(rename = "opted_state", default)]
        pub opted_state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsersettingsNotificationMoreFromAuthors {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersettingsNotificationMoreFromAuthors {
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
    pub struct UsersettingsNotificationMoreFromSeries {
        #[serde(rename = "opted_state", default)]
        pub opted_state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsersettingsNotificationMoreFromSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersettingsNotificationMoreFromSeries {
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
    pub struct UsersettingsNotificationPriceDrop {
        #[serde(rename = "opted_state", default)]
        pub opted_state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsersettingsNotificationPriceDrop {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersettingsNotificationPriceDrop {
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
    pub struct UsersettingsNotificationRewardExpirations {
        #[serde(rename = "opted_state", default)]
        pub opted_state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UsersettingsNotificationRewardExpirations {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsersettingsNotificationRewardExpirations {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Volume {
        #[doc = "Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.)."]
        #[serde(rename = "accessInfo", default)]
        pub access_info: ::std::option::Option<crate::schemas::VolumeAccessInfo>,
        #[doc = "Opaque identifier for a specific version of a volume resource. (In LITE projection)"]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "Unique identifier for a volume. (In LITE projection.)"]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Resource type for a volume. (In LITE projection.)"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "What layers exist in this volume and high level information about them."]
        #[serde(rename = "layerInfo", default)]
        pub layer_info: ::std::option::Option<crate::schemas::VolumeLayerInfo>,
        #[doc = "Recommendation related information for this volume."]
        #[serde(rename = "recommendedInfo", default)]
        pub recommended_info: ::std::option::Option<crate::schemas::VolumeRecommendedInfo>,
        #[doc = "Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries)."]
        #[serde(rename = "saleInfo", default)]
        pub sale_info: ::std::option::Option<crate::schemas::VolumeSaleInfo>,
        #[doc = "Search result information related to this volume."]
        #[serde(rename = "searchInfo", default)]
        pub search_info: ::std::option::Option<crate::schemas::VolumeSearchInfo>,
        #[doc = "URL to this resource. (In LITE projection.)"]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "User specific information related to this volume. (e.g. page this user last read or whether they purchased this book)"]
        #[serde(rename = "userInfo", default)]
        pub user_info: ::std::option::Option<crate::schemas::VolumeUserInfo>,
        #[doc = "General volume information."]
        #[serde(rename = "volumeInfo", default)]
        pub volume_info: ::std::option::Option<crate::schemas::VolumeVolumeInfo>,
    }
    impl ::google_field_selector::FieldSelector for Volume {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Volume {
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
    pub struct VolumeAccessInfo {
        #[doc = "Combines the access and viewability of this volume into a single status field for this user. Values can be FULL_PURCHASED, FULL_PUBLIC_DOMAIN, SAMPLE or NONE. (In LITE projection.)"]
        #[serde(rename = "accessViewStatus", default)]
        pub access_view_status: ::std::option::Option<String>,
        #[doc = "The two-letter ISO_3166-1 country code for which this access information is valid. (In LITE projection.)"]
        #[serde(rename = "country", default)]
        pub country: ::std::option::Option<String>,
        #[doc = "Information about a volume's download license access restrictions."]
        #[serde(rename = "downloadAccess", default)]
        pub download_access: ::std::option::Option<crate::schemas::DownloadAccessRestriction>,
        #[doc = "URL to the Google Drive viewer if this volume is uploaded by the user by selecting the file from Google Drive."]
        #[serde(rename = "driveImportedContentLink", default)]
        pub drive_imported_content_link: ::std::option::Option<String>,
        #[doc = "Whether this volume can be embedded in a viewport using the Embedded Viewer API."]
        #[serde(rename = "embeddable", default)]
        pub embeddable: ::std::option::Option<bool>,
        #[doc = "Information about epub content. (In LITE projection.)"]
        #[serde(rename = "epub", default)]
        pub epub: ::std::option::Option<crate::schemas::VolumeAccessInfoEpub>,
        #[doc = "Whether this volume requires that the client explicitly request offline download license rather than have it done automatically when loading the content, if the client supports it."]
        #[serde(rename = "explicitOfflineLicenseManagement", default)]
        pub explicit_offline_license_management: ::std::option::Option<bool>,
        #[doc = "Information about pdf content. (In LITE projection.)"]
        #[serde(rename = "pdf", default)]
        pub pdf: ::std::option::Option<crate::schemas::VolumeAccessInfoPdf>,
        #[doc = "Whether or not this book is public domain in the country listed above."]
        #[serde(rename = "publicDomain", default)]
        pub public_domain: ::std::option::Option<bool>,
        #[doc = "Whether quote sharing is allowed for this volume."]
        #[serde(rename = "quoteSharingAllowed", default)]
        pub quote_sharing_allowed: ::std::option::Option<bool>,
        #[doc = "Whether text-to-speech is permitted for this volume. Values can be ALLOWED, ALLOWED_FOR_ACCESSIBILITY, or NOT_ALLOWED."]
        #[serde(rename = "textToSpeechPermission", default)]
        pub text_to_speech_permission: ::std::option::Option<String>,
        #[doc = "For ordered but not yet processed orders, we give a URL that can be used to go to the appropriate Google Wallet page."]
        #[serde(rename = "viewOrderUrl", default)]
        pub view_order_url: ::std::option::Option<String>,
        #[doc = "The read access of a volume. Possible values are PARTIAL, ALL_PAGES, NO_PAGES or UNKNOWN. This value depends on the country listed above. A value of PARTIAL means that the publisher has allowed some portion of the volume to be viewed publicly, without purchase. This can apply to eBooks as well as non-eBooks. Public domain books will always have a value of ALL_PAGES."]
        #[serde(rename = "viewability", default)]
        pub viewability: ::std::option::Option<String>,
        #[doc = "URL to read this volume on the Google Books site. Link will not allow users to read non-viewable volumes."]
        #[serde(rename = "webReaderLink", default)]
        pub web_reader_link: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeAccessInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeAccessInfo {
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
    pub struct VolumeAccessInfoEpub {
        #[doc = "URL to retrieve ACS token for epub download. (In LITE projection.)"]
        #[serde(rename = "acsTokenLink", default)]
        pub acs_token_link: ::std::option::Option<String>,
        #[doc = "URL to download epub. (In LITE projection.)"]
        #[serde(rename = "downloadLink", default)]
        pub download_link: ::std::option::Option<String>,
        #[doc = "Is a flowing text epub available either as public domain or for purchase. (In LITE projection.)"]
        #[serde(rename = "isAvailable", default)]
        pub is_available: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for VolumeAccessInfoEpub {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeAccessInfoEpub {
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
    pub struct VolumeAccessInfoPdf {
        #[doc = "URL to retrieve ACS token for pdf download. (In LITE projection.)"]
        #[serde(rename = "acsTokenLink", default)]
        pub acs_token_link: ::std::option::Option<String>,
        #[doc = "URL to download pdf. (In LITE projection.)"]
        #[serde(rename = "downloadLink", default)]
        pub download_link: ::std::option::Option<String>,
        #[doc = "Is a scanned image pdf available either as public domain or for purchase. (In LITE projection.)"]
        #[serde(rename = "isAvailable", default)]
        pub is_available: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for VolumeAccessInfoPdf {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeAccessInfoPdf {
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
    pub struct VolumeLayerInfo {
        #[doc = "A layer should appear here if and only if the layer exists for this book."]
        #[serde(rename = "layers", default)]
        pub layers: ::std::option::Option<Vec<crate::schemas::VolumeLayerInfoLayersItems>>,
    }
    impl ::google_field_selector::FieldSelector for VolumeLayerInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeLayerInfo {
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
    pub struct VolumeLayerInfoLayersItems {
        #[doc = "The layer id of this layer (e.g. \"geo\")."]
        #[serde(rename = "layerId", default)]
        pub layer_id: ::std::option::Option<String>,
        #[doc = "The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately."]
        #[serde(rename = "volumeAnnotationsVersion", default)]
        pub volume_annotations_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeLayerInfoLayersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeLayerInfoLayersItems {
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
    pub struct VolumeRecommendedInfo {
        #[doc = "A text explaining why this volume is recommended."]
        #[serde(rename = "explanation", default)]
        pub explanation: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeRecommendedInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeRecommendedInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VolumeSaleInfo {
        #[doc = "URL to purchase this volume on the Google Books site. (In LITE projection)"]
        #[serde(rename = "buyLink", default)]
        pub buy_link: ::std::option::Option<String>,
        #[doc = "The two-letter ISO_3166-1 country code for which this sale information is valid. (In LITE projection.)"]
        #[serde(rename = "country", default)]
        pub country: ::std::option::Option<String>,
        #[doc = "Whether or not this volume is an eBook (can be added to the My eBooks shelf)."]
        #[serde(rename = "isEbook", default)]
        pub is_ebook: ::std::option::Option<bool>,
        #[doc = "Suggested retail price. (In LITE projection.)"]
        #[serde(rename = "listPrice", default)]
        pub list_price: ::std::option::Option<crate::schemas::VolumeSaleInfoListPrice>,
        #[doc = "Offers available for this volume (sales and rentals)."]
        #[serde(rename = "offers", default)]
        pub offers: ::std::option::Option<Vec<crate::schemas::VolumeSaleInfoOffersItems>>,
        #[doc = "The date on which this book is available for sale."]
        #[serde(rename = "onSaleDate", default)]
        pub on_sale_date: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The actual selling price of the book. This is the same as the suggested retail or list price unless there are offers or discounts on this volume. (In LITE projection.)"]
        #[serde(rename = "retailPrice", default)]
        pub retail_price: ::std::option::Option<crate::schemas::VolumeSaleInfoRetailPrice>,
        #[doc = "Whether or not this book is available for sale or offered for free in the Google eBookstore for the country listed above. Possible values are FOR_SALE, FOR_RENTAL_ONLY, FOR_SALE_AND_RENTAL, FREE, NOT_FOR_SALE, or FOR_PREORDER."]
        #[serde(rename = "saleability", default)]
        pub saleability: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSaleInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSaleInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VolumeSaleInfoListPrice {
        #[doc = "Amount in the currency listed below. (In LITE projection.)"]
        #[serde(rename = "amount", default)]
        pub amount: ::std::option::Option<f64>,
        #[doc = "An ISO 4217, three-letter currency code. (In LITE projection.)"]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSaleInfoListPrice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSaleInfoListPrice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VolumeSaleInfoOffersItems {
        #[doc = "The finsky offer type (e.g., PURCHASE=0 RENTAL=3)"]
        #[serde(rename = "finskyOfferType", default)]
        pub finsky_offer_type: ::std::option::Option<i32>,
        #[doc = "Indicates whether the offer is giftable."]
        #[serde(rename = "giftable", default)]
        pub giftable: ::std::option::Option<bool>,
        #[doc = "Offer list (=undiscounted) price in Micros."]
        #[serde(rename = "listPrice", default)]
        pub list_price: ::std::option::Option<crate::schemas::VolumeSaleInfoOffersItemsListPrice>,
        #[doc = "The rental duration (for rental offers only)."]
        #[serde(rename = "rentalDuration", default)]
        pub rental_duration:
            ::std::option::Option<crate::schemas::VolumeSaleInfoOffersItemsRentalDuration>,
        #[doc = "Offer retail (=discounted) price in Micros"]
        #[serde(rename = "retailPrice", default)]
        pub retail_price:
            ::std::option::Option<crate::schemas::VolumeSaleInfoOffersItemsRetailPrice>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSaleInfoOffersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSaleInfoOffersItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VolumeSaleInfoOffersItemsListPrice {
        #[serde(rename = "amountInMicros", default)]
        pub amount_in_micros: ::std::option::Option<f64>,
        #[serde(rename = "currencyCode", default)]
        pub currency_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSaleInfoOffersItemsListPrice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSaleInfoOffersItemsListPrice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VolumeSaleInfoOffersItemsRentalDuration {
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<f64>,
        #[serde(rename = "unit", default)]
        pub unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSaleInfoOffersItemsRentalDuration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSaleInfoOffersItemsRentalDuration {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VolumeSaleInfoOffersItemsRetailPrice {
        #[serde(rename = "amountInMicros", default)]
        pub amount_in_micros: ::std::option::Option<f64>,
        #[serde(rename = "currencyCode", default)]
        pub currency_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSaleInfoOffersItemsRetailPrice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSaleInfoOffersItemsRetailPrice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VolumeSaleInfoRetailPrice {
        #[doc = "Amount in the currency listed below. (In LITE projection.)"]
        #[serde(rename = "amount", default)]
        pub amount: ::std::option::Option<f64>,
        #[doc = "An ISO 4217, three-letter currency code. (In LITE projection.)"]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSaleInfoRetailPrice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSaleInfoRetailPrice {
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
    pub struct VolumeSearchInfo {
        #[doc = "A text snippet containing the search query."]
        #[serde(rename = "textSnippet", default)]
        pub text_snippet: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeSearchInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeSearchInfo {
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
    pub struct VolumeUserInfo {
        #[doc = "Timestamp when this volume was acquired by the user. (RFC 3339 UTC date-time format) Acquiring includes purchase, user upload, receiving family sharing, etc."]
        #[serde(rename = "acquiredTime", default)]
        pub acquired_time: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "How this volume was acquired."]
        #[serde(rename = "acquisitionType", default)]
        pub acquisition_type: ::std::option::Option<i32>,
        #[doc = "Copy/Paste accounting information."]
        #[serde(rename = "copy", default)]
        pub copy: ::std::option::Option<crate::schemas::VolumeUserInfoCopy>,
        #[doc = "Whether this volume is purchased, sample, pd download etc."]
        #[serde(rename = "entitlementType", default)]
        pub entitlement_type: ::std::option::Option<i32>,
        #[doc = "Information on the ability to share with the family."]
        #[serde(rename = "familySharing", default)]
        pub family_sharing: ::std::option::Option<crate::schemas::VolumeUserInfoFamilySharing>,
        #[doc = "Whether or not the user shared this volume with the family."]
        #[serde(rename = "isFamilySharedFromUser", default)]
        pub is_family_shared_from_user: ::std::option::Option<bool>,
        #[doc = "Whether or not the user received this volume through family sharing."]
        #[serde(rename = "isFamilySharedToUser", default)]
        pub is_family_shared_to_user: ::std::option::Option<bool>,
        #[doc = "Deprecated: Replaced by familySharing."]
        #[serde(rename = "isFamilySharingAllowed", default)]
        pub is_family_sharing_allowed: ::std::option::Option<bool>,
        #[doc = "Deprecated: Replaced by familySharing."]
        #[serde(rename = "isFamilySharingDisabledByFop", default)]
        pub is_family_sharing_disabled_by_fop: ::std::option::Option<bool>,
        #[doc = "Whether or not this volume is currently in \"my books.\""]
        #[serde(rename = "isInMyBooks", default)]
        pub is_in_my_books: ::std::option::Option<bool>,
        #[doc = "Whether or not this volume was pre-ordered by the authenticated user making the request. (In LITE projection.)"]
        #[serde(rename = "isPreordered", default)]
        pub is_preordered: ::std::option::Option<bool>,
        #[doc = "Whether or not this volume was purchased by the authenticated user making the request. (In LITE projection.)"]
        #[serde(rename = "isPurchased", default)]
        pub is_purchased: ::std::option::Option<bool>,
        #[doc = "Whether or not this volume was user uploaded."]
        #[serde(rename = "isUploaded", default)]
        pub is_uploaded: ::std::option::Option<bool>,
        #[doc = "The user's current reading position in the volume, if one is available. (In LITE projection.)"]
        #[serde(rename = "readingPosition", default)]
        pub reading_position: ::std::option::Option<crate::schemas::ReadingPosition>,
        #[doc = "Period during this book is/was a valid rental."]
        #[serde(rename = "rentalPeriod", default)]
        pub rental_period: ::std::option::Option<crate::schemas::VolumeUserInfoRentalPeriod>,
        #[doc = "Whether this book is an active or an expired rental."]
        #[serde(rename = "rentalState", default)]
        pub rental_state: ::std::option::Option<String>,
        #[doc = "This user's review of this volume, if one exists."]
        #[serde(rename = "review", default)]
        pub review: ::std::option::Option<crate::schemas::Review>,
        #[doc = "Timestamp when this volume was last modified by a user action, such as a reading position update, volume purchase or writing a review. (RFC 3339 UTC date-time format)."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[serde(rename = "userUploadedVolumeInfo", default)]
        pub user_uploaded_volume_info:
            ::std::option::Option<crate::schemas::VolumeUserInfoUserUploadedVolumeInfo>,
    }
    impl ::google_field_selector::FieldSelector for VolumeUserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeUserInfo {
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
    pub struct VolumeUserInfoCopy {
        #[serde(rename = "allowedCharacterCount", default)]
        pub allowed_character_count: ::std::option::Option<i32>,
        #[serde(rename = "limitType", default)]
        pub limit_type: ::std::option::Option<String>,
        #[serde(rename = "remainingCharacterCount", default)]
        pub remaining_character_count: ::std::option::Option<i32>,
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::google_field_selector::FieldSelector for VolumeUserInfoCopy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeUserInfoCopy {
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
    pub struct VolumeUserInfoFamilySharing {
        #[doc = "The role of the user in the family."]
        #[serde(rename = "familyRole", default)]
        pub family_role: ::std::option::Option<String>,
        #[doc = "Whether or not this volume can be shared with the family by the user. This includes sharing eligibility of both the volume and the user. If the value is true, the user can initiate a family sharing action."]
        #[serde(rename = "isSharingAllowed", default)]
        pub is_sharing_allowed: ::std::option::Option<bool>,
        #[doc = "Whether or not sharing this volume is temporarily disabled due to issues with the Family Wallet."]
        #[serde(rename = "isSharingDisabledByFop", default)]
        pub is_sharing_disabled_by_fop: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for VolumeUserInfoFamilySharing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeUserInfoFamilySharing {
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
    pub struct VolumeUserInfoRentalPeriod {
        #[serde(rename = "endUtcSec", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_utc_sec: ::std::option::Option<i64>,
        #[serde(rename = "startUtcSec", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_utc_sec: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for VolumeUserInfoRentalPeriod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeUserInfoRentalPeriod {
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
    pub struct VolumeUserInfoUserUploadedVolumeInfo {
        #[serde(rename = "processingState", default)]
        pub processing_state: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeUserInfoUserUploadedVolumeInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeUserInfoUserUploadedVolumeInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct VolumeVolumeInfo {
        #[doc = "Whether anonymous logging should be allowed."]
        #[serde(rename = "allowAnonLogging", default)]
        pub allow_anon_logging: ::std::option::Option<bool>,
        #[doc = "The names of the authors and/or editors for this volume. (In LITE projection)"]
        #[serde(rename = "authors", default)]
        pub authors: ::std::option::Option<Vec<String>>,
        #[doc = "The mean review rating for this volume. (min = 1.0, max = 5.0)"]
        #[serde(rename = "averageRating", default)]
        pub average_rating: ::std::option::Option<f64>,
        #[doc = "Canonical URL for a volume. (In LITE projection.)"]
        #[serde(rename = "canonicalVolumeLink", default)]
        pub canonical_volume_link: ::std::option::Option<String>,
        #[doc = "A list of subject categories, such as \"Fiction\", \"Suspense\", etc."]
        #[serde(rename = "categories", default)]
        pub categories: ::std::option::Option<Vec<String>>,
        #[doc = "Whether the volume has comics content."]
        #[serde(rename = "comicsContent", default)]
        pub comics_content: ::std::option::Option<bool>,
        #[doc = "An identifier for the version of the volume content (text & images). (In LITE projection)"]
        #[serde(rename = "contentVersion", default)]
        pub content_version: ::std::option::Option<String>,
        #[doc = "A synopsis of the volume. The text of the description is formatted in HTML and includes simple formatting elements, such as b, i, and br tags. (In LITE projection.)"]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Physical dimensions of this volume."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: ::std::option::Option<crate::schemas::VolumeVolumeInfoDimensions>,
        #[doc = "A list of image links for all the sizes that are available. (In LITE projection.)"]
        #[serde(rename = "imageLinks", default)]
        pub image_links: ::std::option::Option<crate::schemas::VolumeVolumeInfoImageLinks>,
        #[doc = "Industry standard identifiers for this volume."]
        #[serde(rename = "industryIdentifiers", default)]
        pub industry_identifiers:
            ::std::option::Option<Vec<crate::schemas::VolumeVolumeInfoIndustryIdentifiersItems>>,
        #[doc = "URL to view information about this volume on the Google Books site. (In LITE projection)"]
        #[serde(rename = "infoLink", default)]
        pub info_link: ::std::option::Option<String>,
        #[doc = "Best language for this volume (based on content). It is the two-letter ISO 639-1 code such as 'fr', 'en', etc."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "The main category to which this volume belongs. It will be the category from the categories list returned below that has the highest weight."]
        #[serde(rename = "mainCategory", default)]
        pub main_category: ::std::option::Option<String>,
        #[serde(rename = "maturityRating", default)]
        pub maturity_rating: ::std::option::Option<String>,
        #[doc = "Total number of pages as per publisher metadata."]
        #[serde(rename = "pageCount", default)]
        pub page_count: ::std::option::Option<i32>,
        #[doc = "A top-level summary of the panelization info in this volume."]
        #[serde(rename = "panelizationSummary", default)]
        pub panelization_summary:
            ::std::option::Option<crate::schemas::VolumeVolumeInfoPanelizationSummary>,
        #[doc = "URL to preview this volume on the Google Books site."]
        #[serde(rename = "previewLink", default)]
        pub preview_link: ::std::option::Option<String>,
        #[doc = "Type of publication of this volume. Possible values are BOOK or MAGAZINE."]
        #[serde(rename = "printType", default)]
        pub print_type: ::std::option::Option<String>,
        #[doc = "Total number of printed pages in generated pdf representation."]
        #[serde(rename = "printedPageCount", default)]
        pub printed_page_count: ::std::option::Option<i32>,
        #[doc = "Date of publication. (In LITE projection.)"]
        #[serde(rename = "publishedDate", default)]
        pub published_date: ::std::option::Option<String>,
        #[doc = "Publisher of this volume. (In LITE projection.)"]
        #[serde(rename = "publisher", default)]
        pub publisher: ::std::option::Option<String>,
        #[doc = "The number of review ratings for this volume."]
        #[serde(rename = "ratingsCount", default)]
        pub ratings_count: ::std::option::Option<i32>,
        #[doc = "The reading modes available for this volume."]
        #[serde(rename = "readingModes", default)]
        pub reading_modes: ::std::option::Option<::serde_json::Value>,
        #[doc = "Total number of sample pages as per publisher metadata."]
        #[serde(rename = "samplePageCount", default)]
        pub sample_page_count: ::std::option::Option<i32>,
        #[serde(rename = "seriesInfo", default)]
        pub series_info: ::std::option::Option<crate::schemas::Volumeseriesinfo>,
        #[doc = "Volume subtitle. (In LITE projection.)"]
        #[serde(rename = "subtitle", default)]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "Volume title. (In LITE projection.)"]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeVolumeInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeVolumeInfo {
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
    pub struct VolumeVolumeInfoDimensions {
        #[doc = "Height or length of this volume (in cm)."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<String>,
        #[doc = "Thickness of this volume (in cm)."]
        #[serde(rename = "thickness", default)]
        pub thickness: ::std::option::Option<String>,
        #[doc = "Width of this volume (in cm)."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeVolumeInfoDimensions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeVolumeInfoDimensions {
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
    pub struct VolumeVolumeInfoImageLinks {
        #[doc = "Image link for extra large size (width of ~1280 pixels). (In LITE projection)"]
        #[serde(rename = "extraLarge", default)]
        pub extra_large: ::std::option::Option<String>,
        #[doc = "Image link for large size (width of ~800 pixels). (In LITE projection)"]
        #[serde(rename = "large", default)]
        pub large: ::std::option::Option<String>,
        #[doc = "Image link for medium size (width of ~575 pixels). (In LITE projection)"]
        #[serde(rename = "medium", default)]
        pub medium: ::std::option::Option<String>,
        #[doc = "Image link for small size (width of ~300 pixels). (In LITE projection)"]
        #[serde(rename = "small", default)]
        pub small: ::std::option::Option<String>,
        #[doc = "Image link for small thumbnail size (width of ~80 pixels). (In LITE projection)"]
        #[serde(rename = "smallThumbnail", default)]
        pub small_thumbnail: ::std::option::Option<String>,
        #[doc = "Image link for thumbnail size (width of ~128 pixels). (In LITE projection)"]
        #[serde(rename = "thumbnail", default)]
        pub thumbnail: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeVolumeInfoImageLinks {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeVolumeInfoImageLinks {
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
    pub struct VolumeVolumeInfoIndustryIdentifiersItems {
        #[doc = "Industry specific volume identifier."]
        #[serde(rename = "identifier", default)]
        pub identifier: ::std::option::Option<String>,
        #[doc = "Identifier type. Possible values are ISBN_10, ISBN_13, ISSN and OTHER."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeVolumeInfoIndustryIdentifiersItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeVolumeInfoIndustryIdentifiersItems {
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
    pub struct VolumeVolumeInfoPanelizationSummary {
        #[serde(rename = "containsEpubBubbles", default)]
        pub contains_epub_bubbles: ::std::option::Option<bool>,
        #[serde(rename = "containsImageBubbles", default)]
        pub contains_image_bubbles: ::std::option::Option<bool>,
        #[serde(rename = "epubBubbleVersion", default)]
        pub epub_bubble_version: ::std::option::Option<String>,
        #[serde(rename = "imageBubbleVersion", default)]
        pub image_bubble_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeVolumeInfoPanelizationSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeVolumeInfoPanelizationSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Volume2 {
        #[doc = "A list of volumes."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Volume>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Volume2 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Volume2 {
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
    pub struct Volumeannotation {
        #[doc = "The annotation data id for this volume annotation."]
        #[serde(rename = "annotationDataId", default)]
        pub annotation_data_id: ::std::option::Option<String>,
        #[doc = "Link to get data for this annotation."]
        #[serde(rename = "annotationDataLink", default)]
        pub annotation_data_link: ::std::option::Option<String>,
        #[doc = "The type of annotation this is."]
        #[serde(rename = "annotationType", default)]
        pub annotation_type: ::std::option::Option<String>,
        #[doc = "The content ranges to identify the selected text."]
        #[serde(rename = "contentRanges", default)]
        pub content_ranges: ::std::option::Option<crate::schemas::VolumeannotationContentRanges>,
        #[doc = "Data for this annotation."]
        #[serde(rename = "data", default)]
        pub data: ::std::option::Option<String>,
        #[doc = "Indicates that this annotation is deleted."]
        #[serde(rename = "deleted", default)]
        pub deleted: ::std::option::Option<bool>,
        #[doc = "Unique id of this volume annotation."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Resource Type"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The Layer this annotation is for."]
        #[serde(rename = "layerId", default)]
        pub layer_id: ::std::option::Option<String>,
        #[doc = "Pages the annotation spans."]
        #[serde(rename = "pageIds", default)]
        pub page_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Excerpt from the volume."]
        #[serde(rename = "selectedText", default)]
        pub selected_text: ::std::option::Option<String>,
        #[doc = "URL to this resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Timestamp for the last time this anntoation was updated. (RFC 3339 UTC date-time format)."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The Volume this annotation is for."]
        #[serde(rename = "volumeId", default)]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Volumeannotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Volumeannotation {
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
    pub struct VolumeannotationContentRanges {
        #[doc = "Range in CFI format for this annotation for version above."]
        #[serde(rename = "cfiRange", default)]
        pub cfi_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Content version applicable to ranges below."]
        #[serde(rename = "contentVersion", default)]
        pub content_version: ::std::option::Option<String>,
        #[doc = "Range in GB image format for this annotation for version above."]
        #[serde(rename = "gbImageRange", default)]
        pub gb_image_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
        #[doc = "Range in GB text format for this annotation for version above."]
        #[serde(rename = "gbTextRange", default)]
        pub gb_text_range: ::std::option::Option<crate::schemas::BooksAnnotationsRange>,
    }
    impl ::google_field_selector::FieldSelector for VolumeannotationContentRanges {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeannotationContentRanges {
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
    pub struct Volumeannotations {
        #[doc = "A list of volume annotations."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Volumeannotation>>,
        #[doc = "Resource type"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token to pass in for pagination for the next page. This will not be present if this request does not have more results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of volume annotations found."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
        #[doc = "The version string for all of the volume annotations in this layer (not just the ones in this response). Note: the version string doesn't apply to the annotation data, just the information in this response (e.g. the location of annotations in the book)."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Volumeannotations {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Volumeannotations {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Volumes {
        #[doc = "A list of volumes."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Volume>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Volumes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Volumes {
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
    pub struct Volumeseriesinfo {
        #[doc = "The display number string. This should be used only for display purposes and the actual sequence should be inferred from the below orderNumber."]
        #[serde(rename = "bookDisplayNumber", default)]
        pub book_display_number: ::std::option::Option<String>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Short book title in the context of the series."]
        #[serde(rename = "shortSeriesBookTitle", default)]
        pub short_series_book_title: ::std::option::Option<String>,
        #[serde(rename = "volumeSeries", default)]
        pub volume_series:
            ::std::option::Option<Vec<crate::schemas::VolumeseriesinfoVolumeSeriesItems>>,
    }
    impl ::google_field_selector::FieldSelector for Volumeseriesinfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Volumeseriesinfo {
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
    pub struct VolumeseriesinfoVolumeSeriesItems {
        #[doc = "List of issues. Applicable only for Collection Edition and Omnibus."]
        #[serde(rename = "issue", default)]
        pub issue:
            ::std::option::Option<Vec<crate::schemas::VolumeseriesinfoVolumeSeriesItemsIssueItems>>,
        #[doc = "The book order number in the series."]
        #[serde(rename = "orderNumber", default)]
        pub order_number: ::std::option::Option<i32>,
        #[doc = "The book type in the context of series. Examples - Single Issue, Collection Edition, etc."]
        #[serde(rename = "seriesBookType", default)]
        pub series_book_type: ::std::option::Option<String>,
        #[doc = "The series id."]
        #[serde(rename = "seriesId", default)]
        pub series_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VolumeseriesinfoVolumeSeriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeseriesinfoVolumeSeriesItems {
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
    pub struct VolumeseriesinfoVolumeSeriesItemsIssueItems {
        #[serde(rename = "issueDisplayNumber", default)]
        pub issue_display_number: ::std::option::Option<String>,
        #[serde(rename = "issueOrderNumber", default)]
        pub issue_order_number: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for VolumeseriesinfoVolumeSeriesItemsIssueItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VolumeseriesinfoVolumeSeriesItemsIssueItems {
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
    #[doc = "Actions that can be performed on the bookshelves resource"]
    pub fn bookshelves(&self) -> crate::resources::bookshelves::BookshelvesActions<A> {
        crate::resources::bookshelves::BookshelvesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the cloudloading resource"]
    pub fn cloudloading(&self) -> crate::resources::cloudloading::CloudloadingActions<A> {
        crate::resources::cloudloading::CloudloadingActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the dictionary resource"]
    pub fn dictionary(&self) -> crate::resources::dictionary::DictionaryActions<A> {
        crate::resources::dictionary::DictionaryActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the familysharing resource"]
    pub fn familysharing(&self) -> crate::resources::familysharing::FamilysharingActions<A> {
        crate::resources::familysharing::FamilysharingActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the layers resource"]
    pub fn layers(&self) -> crate::resources::layers::LayersActions<A> {
        crate::resources::layers::LayersActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the myconfig resource"]
    pub fn myconfig(&self) -> crate::resources::myconfig::MyconfigActions<A> {
        crate::resources::myconfig::MyconfigActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the mylibrary resource"]
    pub fn mylibrary(&self) -> crate::resources::mylibrary::MylibraryActions<A> {
        crate::resources::mylibrary::MylibraryActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the notification resource"]
    pub fn notification(&self) -> crate::resources::notification::NotificationActions<A> {
        crate::resources::notification::NotificationActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the onboarding resource"]
    pub fn onboarding(&self) -> crate::resources::onboarding::OnboardingActions<A> {
        crate::resources::onboarding::OnboardingActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the personalizedstream resource"]
    pub fn personalizedstream(
        &self,
    ) -> crate::resources::personalizedstream::PersonalizedstreamActions<A> {
        crate::resources::personalizedstream::PersonalizedstreamActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the promooffer resource"]
    pub fn promooffer(&self) -> crate::resources::promooffer::PromoofferActions<A> {
        crate::resources::promooffer::PromoofferActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the series resource"]
    pub fn series(&self) -> crate::resources::series::SeriesActions<A> {
        crate::resources::series::SeriesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the volumes resource"]
    pub fn volumes(&self) -> crate::resources::volumes::VolumesActions<A> {
        crate::resources::volumes::VolumesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod bookshelves {
        pub mod params {}
        pub struct BookshelvesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> BookshelvesActions<'a, A> {
            #[doc = "Retrieves metadata for a specific bookshelf for the specified user."]
            pub fn get(
                &self,
                user_id: impl Into<String>,
                shelf: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                    shelf: shelf.into(),
                    source: None,
                }
            }
            #[doc = "Retrieves a list of public bookshelves for the specified user."]
            pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                    source: None,
                }
            }
            #[doc = "Actions that can be performed on the volumes resource"]
            pub fn volumes(&self) -> crate::resources::bookshelves::volumes::VolumesActions<A> {
                crate::resources::bookshelves::volumes::VolumesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            user_id: String,
            shelf: String,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::Bookshelf, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Bookshelf, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("users/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/bookshelves/");
                {
                    let var_as_str = &self.shelf;
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
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            user_id: String,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::Bookshelves, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Bookshelves, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("users/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/bookshelves");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        pub mod volumes {
            pub mod params {}
            pub struct VolumesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> VolumesActions<'a, A> {
                #[doc = "Retrieves volumes in a specific bookshelf for the specified user."]
                pub fn list(
                    &self,
                    user_id: impl Into<String>,
                    shelf: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        user_id: user_id.into(),
                        shelf: shelf.into(),
                        max_results: None,
                        show_preorders: None,
                        source: None,
                        start_index: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                user_id: String,
                shelf: String,
                max_results: Option<u32>,
                show_preorders: Option<bool>,
                source: Option<String>,
                start_index: Option<u32>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "Maximum number of results to return"]
                pub fn max_results(mut self, value: u32) -> Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "Set to true to show pre-ordered books. Defaults to false."]
                pub fn show_preorders(mut self, value: bool) -> Self {
                    self.show_preorders = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
                    self
                }
                #[doc = "Index of the first element to return (starts at 0)"]
                pub fn start_index(mut self, value: u32) -> Self {
                    self.start_index = Some(value);
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
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/bookshelves/");
                    {
                        let var_as_str = &self.shelf;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/volumes");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("showPreorders", &self.show_preorders)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("startIndex", &self.start_index)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
    }
    pub mod cloudloading {
        pub mod params {}
        pub struct CloudloadingActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> CloudloadingActions<'a, A> {
            #[doc = ""]
            pub fn add_book(&self) -> AddBookRequestBuilder<A> {
                AddBookRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    drive_document_id: None,
                    mime_type: None,
                    name: None,
                    upload_client_token: None,
                }
            }
            #[doc = "Remove the book and its contents"]
            pub fn delete_book(&self, volume_id: impl Into<String>) -> DeleteBookRequestBuilder<A> {
                DeleteBookRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    volume_id: volume_id.into(),
                }
            }
            #[doc = ""]
            pub fn update_book(
                &self,
                request: crate::schemas::BooksCloudloadingResource,
            ) -> UpdateBookRequestBuilder<A> {
                UpdateBookRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
        #[derive(Debug, Clone)]
        pub struct AddBookRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            drive_document_id: Option<String>,
            mime_type: Option<String>,
            name: Option<String>,
            upload_client_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> AddBookRequestBuilder<'a, A> {
            #[doc = "A drive document id. The upload_client_token must not be set."]
            pub fn drive_document_id(mut self, value: impl Into<String>) -> Self {
                self.drive_document_id = Some(value.into());
                self
            }
            #[doc = "The document MIME type. It can be set only if the drive_document_id is set."]
            pub fn mime_type(mut self, value: impl Into<String>) -> Self {
                self.mime_type = Some(value.into());
                self
            }
            #[doc = "The document name. It can be set only if the drive_document_id is set."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn upload_client_token(mut self, value: impl Into<String>) -> Self {
                self.upload_client_token = Some(value.into());
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
            ) -> Result<crate::schemas::BooksCloudloadingResource, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BooksCloudloadingResource, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("cloudloading/addBook");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("drive_document_id", &self.drive_document_id)]);
                let req = req.query(&[("mime_type", &self.mime_type)]);
                let req = req.query(&[("name", &self.name)]);
                let req = req.query(&[("upload_client_token", &self.upload_client_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteBookRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            volume_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> DeleteBookRequestBuilder<'a, A> {
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
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path())?;
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("cloudloading/deleteBook");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("volumeId", &self.volume_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateBookRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::BooksCloudloadingResource,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> UpdateBookRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::BooksCloudloadingResource, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BooksCloudloadingResource, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("cloudloading/updateBook");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
    }
    pub mod dictionary {
        pub mod params {}
        pub struct DictionaryActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> DictionaryActions<'a, A> {
            #[doc = "Returns a list of offline dictionary metadata available"]
            pub fn list_offline_metadata(
                &self,
                cpksver: impl Into<String>,
            ) -> ListOfflineMetadataRequestBuilder<A> {
                ListOfflineMetadataRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    cpksver: cpksver.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListOfflineMetadataRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            cpksver: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ListOfflineMetadataRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Metadata, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Metadata, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("dictionary/listOfflineMetadata");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("cpksver", &self.cpksver)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
    }
    pub mod familysharing {
        pub mod params {}
        pub struct FamilysharingActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> FamilysharingActions<'a, A> {
            #[doc = "Gets information regarding the family that the user is part of."]
            pub fn get_family_info(&self) -> GetFamilyInfoRequestBuilder<A> {
                GetFamilyInfoRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    source: None,
                }
            }
            #[doc = "Initiates sharing of the content with the user's family. Empty response indicates success."]
            pub fn share(&self) -> ShareRequestBuilder<A> {
                ShareRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    doc_id: None,
                    source: None,
                    volume_id: None,
                }
            }
            #[doc = "Initiates revoking content that has already been shared with the user's family. Empty response indicates success."]
            pub fn unshare(&self) -> UnshareRequestBuilder<A> {
                UnshareRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    doc_id: None,
                    source: None,
                    volume_id: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetFamilyInfoRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetFamilyInfoRequestBuilder<'a, A> {
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::FamilyInfo, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::FamilyInfo, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("familysharing/getFamilyInfo");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct ShareRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            doc_id: Option<String>,
            source: Option<String>,
            volume_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ShareRequestBuilder<'a, A> {
            #[doc = "The docid to share."]
            pub fn doc_id(mut self, value: impl Into<String>) -> Self {
                self.doc_id = Some(value.into());
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
                self
            }
            #[doc = "The volume to share."]
            pub fn volume_id(mut self, value: impl Into<String>) -> Self {
                self.volume_id = Some(value.into());
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
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path())?;
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("familysharing/share");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("docId", &self.doc_id)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("volumeId", &self.volume_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct UnshareRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            doc_id: Option<String>,
            source: Option<String>,
            volume_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> UnshareRequestBuilder<'a, A> {
            #[doc = "The docid to unshare."]
            pub fn doc_id(mut self, value: impl Into<String>) -> Self {
                self.doc_id = Some(value.into());
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
                self
            }
            #[doc = "The volume to unshare."]
            pub fn volume_id(mut self, value: impl Into<String>) -> Self {
                self.volume_id = Some(value.into());
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
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path())?;
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("familysharing/unshare");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("docId", &self.doc_id)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("volumeId", &self.volume_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
    }
    pub mod layers {
        pub mod params {}
        pub struct LayersActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> LayersActions<'a, A> {
            #[doc = "Gets the layer summary for a volume."]
            pub fn get(
                &self,
                volume_id: impl Into<String>,
                summary_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    volume_id: volume_id.into(),
                    summary_id: summary_id.into(),
                    content_version: None,
                    source: None,
                }
            }
            #[doc = "List the layer summaries for a volume."]
            pub fn list(&self, volume_id: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    volume_id: volume_id.into(),
                    content_version: None,
                    max_results: None,
                    page_token: None,
                    source: None,
                }
            }
            #[doc = "Actions that can be performed on the annotation_data resource"]
            pub fn annotation_data(
                &self,
            ) -> crate::resources::layers::annotation_data::AnnotationDataActions<A> {
                crate::resources::layers::annotation_data::AnnotationDataActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the volume_annotations resource"]
            pub fn volume_annotations(
                &self,
            ) -> crate::resources::layers::volume_annotations::VolumeAnnotationsActions<A>
            {
                crate::resources::layers::volume_annotations::VolumeAnnotationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            volume_id: String,
            summary_id: String,
            content_version: Option<String>,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
            #[doc = "The content version for the requested volume."]
            pub fn content_version(mut self, value: impl Into<String>) -> Self {
                self.content_version = Some(value.into());
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::Layersummary, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Layersummary, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("volumes/");
                {
                    let var_as_str = &self.volume_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/layersummary/");
                {
                    let var_as_str = &self.summary_id;
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
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("contentVersion", &self.content_version)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            volume_id: String,
            content_version: Option<String>,
            max_results: Option<u32>,
            page_token: Option<String>,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
            #[doc = "The content version for the requested volume."]
            pub fn content_version(mut self, value: impl Into<String>) -> Self {
                self.content_version = Some(value.into());
                self
            }
            #[doc = "Maximum number of results to return"]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The value of the nextToken from the previous page."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::Layersummaries, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Layersummaries, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("volumes/");
                {
                    let var_as_str = &self.volume_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/layersummary");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("contentVersion", &self.content_version)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        pub mod annotation_data {
            pub mod params {}
            pub struct AnnotationDataActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> AnnotationDataActions<'a, A> {
                #[doc = "Gets the annotation data."]
                pub fn get(
                    &self,
                    volume_id: impl Into<String>,
                    layer_id: impl Into<String>,
                    annotation_data_id: impl Into<String>,
                    content_version: impl Into<String>,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        volume_id: volume_id.into(),
                        layer_id: layer_id.into(),
                        annotation_data_id: annotation_data_id.into(),
                        content_version: content_version.into(),
                        allow_web_definitions: None,
                        h: None,
                        locale: None,
                        scale: None,
                        source: None,
                        w: None,
                    }
                }
                #[doc = "Gets the annotation data for a volume and layer."]
                pub fn list(
                    &self,
                    volume_id: impl Into<String>,
                    layer_id: impl Into<String>,
                    content_version: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        volume_id: volume_id.into(),
                        layer_id: layer_id.into(),
                        content_version: content_version.into(),
                        annotation_data_id: None,
                        h: None,
                        locale: None,
                        max_results: None,
                        page_token: None,
                        scale: None,
                        source: None,
                        updated_max: None,
                        updated_min: None,
                        w: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                volume_id: String,
                layer_id: String,
                annotation_data_id: String,
                content_version: String,
                allow_web_definitions: Option<bool>,
                h: Option<i32>,
                locale: Option<String>,
                scale: Option<i32>,
                source: Option<String>,
                w: Option<i32>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
                #[doc = "For the dictionary layer. Whether or not to allow web definitions."]
                pub fn allow_web_definitions(mut self, value: bool) -> Self {
                    self.allow_web_definitions = Some(value);
                    self
                }
                #[doc = "The requested pixel height for any images. If height is provided width must also be provided."]
                pub fn h(mut self, value: i32) -> Self {
                    self.h = Some(value);
                    self
                }
                #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "The requested scale for the image."]
                pub fn scale(mut self, value: i32) -> Self {
                    self.scale = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
                    self
                }
                #[doc = "The requested pixel width for any images. If width is provided height must also be provided."]
                pub fn w(mut self, value: i32) -> Self {
                    self.w = Some(value);
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
                ) -> Result<crate::schemas::Annotationdata, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Annotationdata, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/");
                    {
                        let var_as_str = &self.volume_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/layers/");
                    {
                        let var_as_str = &self.layer_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/data/");
                    {
                        let var_as_str = &self.annotation_data_id;
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("contentVersion", &self.content_version)]);
                    let req = req.query(&[("allowWebDefinitions", &self.allow_web_definitions)]);
                    let req = req.query(&[("h", &self.h)]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("scale", &self.scale)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("w", &self.w)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                volume_id: String,
                layer_id: String,
                content_version: String,
                annotation_data_id: Option<Vec<String>>,
                h: Option<i32>,
                locale: Option<String>,
                max_results: Option<u32>,
                page_token: Option<String>,
                scale: Option<i32>,
                source: Option<String>,
                updated_max: Option<String>,
                updated_min: Option<String>,
                w: Option<i32>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "The list of Annotation Data Ids to retrieve. Pagination is ignored if this is set."]
                pub fn annotation_data_id(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.annotation_data_id = Some(value.into());
                    self
                }
                #[doc = "The requested pixel height for any images. If height is provided width must also be provided."]
                pub fn h(mut self, value: i32) -> Self {
                    self.h = Some(value);
                    self
                }
                #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return"]
                pub fn max_results(mut self, value: u32) -> Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "The value of the nextToken from the previous page."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "The requested scale for the image."]
                pub fn scale(mut self, value: i32) -> Self {
                    self.scale = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
                    self
                }
                #[doc = "RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive)."]
                pub fn updated_max(mut self, value: impl Into<String>) -> Self {
                    self.updated_max = Some(value.into());
                    self
                }
                #[doc = "RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive)."]
                pub fn updated_min(mut self, value: impl Into<String>) -> Self {
                    self.updated_min = Some(value.into());
                    self
                }
                #[doc = "The requested pixel width for any images. If width is provided height must also be provided."]
                pub fn w(mut self, value: i32) -> Self {
                    self.w = Some(value);
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Annotationdata>
                {
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Annotationdata>
                {
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
                ) -> crate::iter::PageIter<Self, crate::schemas::Annotationsdata> {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::Annotationsdata> {
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
                ) -> Result<crate::schemas::Annotationsdata, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Annotationsdata, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/");
                    {
                        let var_as_str = &self.volume_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/layers/");
                    {
                        let var_as_str = &self.layer_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/data");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("contentVersion", &self.content_version)]);
                    let req = req.query(&[("annotationDataId", &self.annotation_data_id)]);
                    let req = req.query(&[("h", &self.h)]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("scale", &self.scale)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("updatedMax", &self.updated_max)]);
                    let req = req.query(&[("updatedMin", &self.updated_min)]);
                    let req = req.query(&[("w", &self.w)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
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
        pub mod volume_annotations {
            pub mod params {}
            pub struct VolumeAnnotationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> VolumeAnnotationsActions<'a, A> {
                #[doc = "Gets the volume annotation."]
                pub fn get(
                    &self,
                    volume_id: impl Into<String>,
                    layer_id: impl Into<String>,
                    annotation_id: impl Into<String>,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        volume_id: volume_id.into(),
                        layer_id: layer_id.into(),
                        annotation_id: annotation_id.into(),
                        locale: None,
                        source: None,
                    }
                }
                #[doc = "Gets the volume annotations for a volume and layer."]
                pub fn list(
                    &self,
                    volume_id: impl Into<String>,
                    layer_id: impl Into<String>,
                    content_version: impl Into<String>,
                ) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        volume_id: volume_id.into(),
                        layer_id: layer_id.into(),
                        content_version: content_version.into(),
                        end_offset: None,
                        end_position: None,
                        locale: None,
                        max_results: None,
                        page_token: None,
                        show_deleted: None,
                        source: None,
                        start_offset: None,
                        start_position: None,
                        updated_max: None,
                        updated_min: None,
                        volume_annotations_version: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                volume_id: String,
                layer_id: String,
                annotation_id: String,
                locale: Option<String>,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
                #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::Volumeannotation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Volumeannotation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/");
                    {
                        let var_as_str = &self.volume_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/layers/");
                    {
                        let var_as_str = &self.layer_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/annotations/");
                    {
                        let var_as_str = &self.annotation_id;
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                volume_id: String,
                layer_id: String,
                content_version: String,
                end_offset: Option<String>,
                end_position: Option<String>,
                locale: Option<String>,
                max_results: Option<u32>,
                page_token: Option<String>,
                show_deleted: Option<bool>,
                source: Option<String>,
                start_offset: Option<String>,
                start_position: Option<String>,
                updated_max: Option<String>,
                updated_min: Option<String>,
                volume_annotations_version: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "The end offset to end retrieving data from."]
                pub fn end_offset(mut self, value: impl Into<String>) -> Self {
                    self.end_offset = Some(value.into());
                    self
                }
                #[doc = "The end position to end retrieving data from."]
                pub fn end_position(mut self, value: impl Into<String>) -> Self {
                    self.end_position = Some(value.into());
                    self
                }
                #[doc = "The locale information for the data. ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return"]
                pub fn max_results(mut self, value: u32) -> Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "The value of the nextToken from the previous page."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false."]
                pub fn show_deleted(mut self, value: bool) -> Self {
                    self.show_deleted = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
                    self
                }
                #[doc = "The start offset to start retrieving data from."]
                pub fn start_offset(mut self, value: impl Into<String>) -> Self {
                    self.start_offset = Some(value.into());
                    self
                }
                #[doc = "The start position to start retrieving data from."]
                pub fn start_position(mut self, value: impl Into<String>) -> Self {
                    self.start_position = Some(value.into());
                    self
                }
                #[doc = "RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive)."]
                pub fn updated_max(mut self, value: impl Into<String>) -> Self {
                    self.updated_max = Some(value.into());
                    self
                }
                #[doc = "RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive)."]
                pub fn updated_min(mut self, value: impl Into<String>) -> Self {
                    self.updated_min = Some(value.into());
                    self
                }
                #[doc = "The version of the volume annotations that you are requesting."]
                pub fn volume_annotations_version(mut self, value: impl Into<String>) -> Self {
                    self.volume_annotations_version = Some(value.into());
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Volumeannotation>
                {
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Volumeannotation>
                {
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
                ) -> crate::iter::PageIter<Self, crate::schemas::Volumeannotations>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::Volumeannotations>
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
                ) -> Result<crate::schemas::Volumeannotations, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Volumeannotations, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/");
                    {
                        let var_as_str = &self.volume_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/layers/");
                    {
                        let var_as_str = &self.layer_id;
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("contentVersion", &self.content_version)]);
                    let req = req.query(&[("endOffset", &self.end_offset)]);
                    let req = req.query(&[("endPosition", &self.end_position)]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("showDeleted", &self.show_deleted)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("startOffset", &self.start_offset)]);
                    let req = req.query(&[("startPosition", &self.start_position)]);
                    let req = req.query(&[("updatedMax", &self.updated_max)]);
                    let req = req.query(&[("updatedMin", &self.updated_min)]);
                    let req = req
                        .query(&[("volumeAnnotationsVersion", &self.volume_annotations_version)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
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
    pub mod myconfig {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RequestAccessLicenseTypes {
                #[doc = "Both concurrent and download licenses."]
                Both,
                #[doc = "Concurrent access license."]
                Concurrent,
                #[doc = "Offline download access license."]
                Download,
            }
            impl RequestAccessLicenseTypes {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RequestAccessLicenseTypes::Both => "BOTH",
                        RequestAccessLicenseTypes::Concurrent => "CONCURRENT",
                        RequestAccessLicenseTypes::Download => "DOWNLOAD",
                    }
                }
            }
            impl ::std::fmt::Display for RequestAccessLicenseTypes {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RequestAccessLicenseTypes {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RequestAccessLicenseTypes {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "BOTH" => RequestAccessLicenseTypes::Both,
                        "CONCURRENT" => RequestAccessLicenseTypes::Concurrent,
                        "DOWNLOAD" => RequestAccessLicenseTypes::Download,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for RequestAccessLicenseTypes {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for RequestAccessLicenseTypes {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum SyncVolumeLicensesFeaturesItems {
                #[doc = "Client supports rentals."]
                Rentals,
            }
            impl SyncVolumeLicensesFeaturesItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        SyncVolumeLicensesFeaturesItems::Rentals => "RENTALS",
                    }
                }
            }
            impl ::std::fmt::Display for SyncVolumeLicensesFeaturesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for SyncVolumeLicensesFeaturesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for SyncVolumeLicensesFeaturesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "RENTALS" => SyncVolumeLicensesFeaturesItems::Rentals,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for SyncVolumeLicensesFeaturesItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for SyncVolumeLicensesFeaturesItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct MyconfigActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> MyconfigActions<'a, A> {
            #[doc = "Gets the current settings for the user."]
            pub fn get_user_settings(&self) -> GetUserSettingsRequestBuilder<A> {
                GetUserSettingsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = "Release downloaded content access restriction."]
            pub fn release_download_access(
                &self,
                volume_ids: impl Into<Vec<String>>,
                cpksver: impl Into<String>,
            ) -> ReleaseDownloadAccessRequestBuilder<A> {
                ReleaseDownloadAccessRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    volume_ids: volume_ids.into(),
                    cpksver: cpksver.into(),
                    locale: None,
                    source: None,
                }
            }
            #[doc = "Request concurrent and download access restrictions."]
            pub fn request_access(
                &self,
                source: impl Into<String>,
                volume_id: impl Into<String>,
                nonce: impl Into<String>,
                cpksver: impl Into<String>,
            ) -> RequestAccessRequestBuilder<A> {
                RequestAccessRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    source: source.into(),
                    volume_id: volume_id.into(),
                    nonce: nonce.into(),
                    cpksver: cpksver.into(),
                    license_types: None,
                    locale: None,
                }
            }
            #[doc = "Request downloaded content access for specified volumes on the My eBooks shelf."]
            pub fn sync_volume_licenses(
                &self,
                source: impl Into<String>,
                nonce: impl Into<String>,
                cpksver: impl Into<String>,
            ) -> SyncVolumeLicensesRequestBuilder<A> {
                SyncVolumeLicensesRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    source: source.into(),
                    nonce: nonce.into(),
                    cpksver: cpksver.into(),
                    features: None,
                    include_non_comics_series: None,
                    locale: None,
                    show_preorders: None,
                    volume_ids: None,
                }
            }
            #[doc = "Sets the settings for the user. If a sub-object is specified, it will overwrite the existing sub-object stored in the server. Unspecified sub-objects will retain the existing value."]
            pub fn update_user_settings(
                &self,
                request: crate::schemas::Usersettings,
            ) -> UpdateUserSettingsRequestBuilder<A> {
                UpdateUserSettingsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
        #[derive(Debug, Clone)]
        pub struct GetUserSettingsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetUserSettingsRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Usersettings, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Usersettings, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("myconfig/getUserSettings");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct ReleaseDownloadAccessRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            volume_ids: Vec<String>,
            cpksver: String,
            locale: Option<String>,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ReleaseDownloadAccessRequestBuilder<'a, A> {
            #[doc = "ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US."]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::DownloadAccesses, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DownloadAccesses, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("myconfig/releaseDownloadAccess");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("volumeIds", &self.volume_ids)]);
                let req = req.query(&[("cpksver", &self.cpksver)]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct RequestAccessRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            source: String,
            volume_id: String,
            nonce: String,
            cpksver: String,
            license_types: Option<crate::resources::myconfig::params::RequestAccessLicenseTypes>,
            locale: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> RequestAccessRequestBuilder<'a, A> {
            #[doc = "The type of access license to request. If not specified, the default is BOTH."]
            pub fn license_types(
                mut self,
                value: crate::resources::myconfig::params::RequestAccessLicenseTypes,
            ) -> Self {
                self.license_types = Some(value);
                self
            }
            #[doc = "ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US."]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
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
            ) -> Result<crate::schemas::RequestAccess, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RequestAccess, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("myconfig/requestAccess");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("volumeId", &self.volume_id)]);
                let req = req.query(&[("nonce", &self.nonce)]);
                let req = req.query(&[("cpksver", &self.cpksver)]);
                let req = req.query(&[("licenseTypes", &self.license_types)]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct SyncVolumeLicensesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            source: String,
            nonce: String,
            cpksver: String,
            features:
                Option<Vec<crate::resources::myconfig::params::SyncVolumeLicensesFeaturesItems>>,
            include_non_comics_series: Option<bool>,
            locale: Option<String>,
            show_preorders: Option<bool>,
            volume_ids: Option<Vec<String>>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SyncVolumeLicensesRequestBuilder<'a, A> {
            #[doc = "List of features supported by the client, i.e., 'RENTALS'"]
            pub fn features(
                mut self,
                value: impl Into<
                    Vec<crate::resources::myconfig::params::SyncVolumeLicensesFeaturesItems>,
                >,
            ) -> Self {
                self.features = Some(value.into());
                self
            }
            #[doc = "Set to true to include non-comics series. Defaults to false."]
            pub fn include_non_comics_series(mut self, value: bool) -> Self {
                self.include_non_comics_series = Some(value);
                self
            }
            #[doc = "ISO-639-1, ISO-3166-1 codes for message localization, i.e. en_US."]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "Set to true to show pre-ordered books. Defaults to false."]
            pub fn show_preorders(mut self, value: bool) -> Self {
                self.show_preorders = Some(value);
                self
            }
            #[doc = "The volume(s) to request download restrictions for."]
            pub fn volume_ids(mut self, value: impl Into<Vec<String>>) -> Self {
                self.volume_ids = Some(value.into());
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
            ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("myconfig/syncVolumeLicenses");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("nonce", &self.nonce)]);
                let req = req.query(&[("cpksver", &self.cpksver)]);
                let req = req.query(&[("features", &self.features)]);
                let req = req.query(&[("includeNonComicsSeries", &self.include_non_comics_series)]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("showPreorders", &self.show_preorders)]);
                let req = req.query(&[("volumeIds", &self.volume_ids)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateUserSettingsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            request: crate::schemas::Usersettings,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> UpdateUserSettingsRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Usersettings, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Usersettings, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("myconfig/updateUserSettings");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
    }
    pub mod mylibrary {
        pub mod params {}
        pub struct MylibraryActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> MylibraryActions<'a, A> {
            #[doc = "Actions that can be performed on the annotations resource"]
            pub fn annotations(
                &self,
            ) -> crate::resources::mylibrary::annotations::AnnotationsActions<A> {
                crate::resources::mylibrary::annotations::AnnotationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the bookshelves resource"]
            pub fn bookshelves(
                &self,
            ) -> crate::resources::mylibrary::bookshelves::BookshelvesActions<A> {
                crate::resources::mylibrary::bookshelves::BookshelvesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the readingpositions resource"]
            pub fn readingpositions(
                &self,
            ) -> crate::resources::mylibrary::readingpositions::ReadingpositionsActions<A>
            {
                crate::resources::mylibrary::readingpositions::ReadingpositionsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod annotations {
            pub mod params {}
            pub struct AnnotationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> AnnotationsActions<'a, A> {
                #[doc = "Deletes an annotation."]
                pub fn delete(&self, annotation_id: impl Into<String>) -> DeleteRequestBuilder<A> {
                    DeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        annotation_id: annotation_id.into(),
                        source: None,
                    }
                }
                #[doc = "Inserts a new annotation."]
                pub fn insert(
                    &self,
                    request: crate::schemas::Annotation,
                ) -> InsertRequestBuilder<A> {
                    InsertRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        annotation_id: None,
                        country: None,
                        show_only_summary_in_response: None,
                        source: None,
                    }
                }
                #[doc = "Retrieves a list of annotations, possibly filtered."]
                pub fn list(&self) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        content_version: None,
                        layer_id: None,
                        layer_ids: None,
                        max_results: None,
                        page_token: None,
                        show_deleted: None,
                        source: None,
                        updated_max: None,
                        updated_min: None,
                        volume_id: None,
                    }
                }
                #[doc = "Gets the summary of specified layers."]
                pub fn summary(
                    &self,
                    layer_ids: impl Into<Vec<String>>,
                    volume_id: impl Into<String>,
                ) -> SummaryRequestBuilder<A> {
                    SummaryRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        layer_ids: layer_ids.into(),
                        volume_id: volume_id.into(),
                    }
                }
                #[doc = "Updates an existing annotation."]
                pub fn update(
                    &self,
                    request: crate::schemas::Annotation,
                    annotation_id: impl Into<String>,
                ) -> UpdateRequestBuilder<A> {
                    UpdateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        annotation_id: annotation_id.into(),
                        source: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                annotation_id: String,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> DeleteRequestBuilder<'a, A> {
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path())?;
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/annotations/");
                    {
                        let var_as_str = &self.annotation_id;
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct InsertRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::Annotation,
                annotation_id: Option<String>,
                country: Option<String>,
                show_only_summary_in_response: Option<bool>,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> InsertRequestBuilder<'a, A> {
                #[doc = "The ID for the annotation to insert."]
                pub fn annotation_id(mut self, value: impl Into<String>) -> Self {
                    self.annotation_id = Some(value.into());
                    self
                }
                #[doc = "ISO-3166-1 code to override the IP-based location."]
                pub fn country(mut self, value: impl Into<String>) -> Self {
                    self.country = Some(value.into());
                    self
                }
                #[doc = "Requests that only the summary of the specified layer be provided in the response."]
                pub fn show_only_summary_in_response(mut self, value: bool) -> Self {
                    self.show_only_summary_in_response = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::Annotation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Annotation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/annotations");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("annotationId", &self.annotation_id)]);
                    let req = req.query(&[("country", &self.country)]);
                    let req = req.query(&[(
                        "showOnlySummaryInResponse",
                        &self.show_only_summary_in_response,
                    )]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                content_version: Option<String>,
                layer_id: Option<String>,
                layer_ids: Option<Vec<String>>,
                max_results: Option<u32>,
                page_token: Option<String>,
                show_deleted: Option<bool>,
                source: Option<String>,
                updated_max: Option<String>,
                updated_min: Option<String>,
                volume_id: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "The content version for the requested volume."]
                pub fn content_version(mut self, value: impl Into<String>) -> Self {
                    self.content_version = Some(value.into());
                    self
                }
                #[doc = "The layer ID to limit annotation by."]
                pub fn layer_id(mut self, value: impl Into<String>) -> Self {
                    self.layer_id = Some(value.into());
                    self
                }
                #[doc = "The layer ID(s) to limit annotation by."]
                pub fn layer_ids(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.layer_ids = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return"]
                pub fn max_results(mut self, value: u32) -> Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "The value of the nextToken from the previous page."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Set to true to return deleted annotations. updatedMin must be in the request to use this. Defaults to false."]
                pub fn show_deleted(mut self, value: bool) -> Self {
                    self.show_deleted = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
                    self
                }
                #[doc = "RFC 3339 timestamp to restrict to items updated prior to this timestamp (exclusive)."]
                pub fn updated_max(mut self, value: impl Into<String>) -> Self {
                    self.updated_max = Some(value.into());
                    self
                }
                #[doc = "RFC 3339 timestamp to restrict to items updated since this timestamp (inclusive)."]
                pub fn updated_min(mut self, value: impl Into<String>) -> Self {
                    self.updated_min = Some(value.into());
                    self
                }
                #[doc = "The volume to restrict annotations to."]
                pub fn volume_id(mut self, value: impl Into<String>) -> Self {
                    self.volume_id = Some(value.into());
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Annotation> {
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
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Annotation> {
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
                ) -> crate::iter::PageIter<Self, crate::schemas::Annotations> {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::Annotations> {
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
                ) -> Result<crate::schemas::Annotations, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Annotations, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/annotations");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("contentVersion", &self.content_version)]);
                    let req = req.query(&[("layerId", &self.layer_id)]);
                    let req = req.query(&[("layerIds", &self.layer_ids)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("showDeleted", &self.show_deleted)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("updatedMax", &self.updated_max)]);
                    let req = req.query(&[("updatedMin", &self.updated_min)]);
                    let req = req.query(&[("volumeId", &self.volume_id)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
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
            pub struct SummaryRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                layer_ids: Vec<String>,
                volume_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> SummaryRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::AnnotationsSummary, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::AnnotationsSummary, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/annotations/summary");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("layerIds", &self.layer_ids)]);
                    let req = req.query(&[("volumeId", &self.volume_id)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                request: crate::schemas::Annotation,
                annotation_id: String,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> UpdateRequestBuilder<'a, A> {
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::Annotation, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Annotation, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/annotations/");
                    {
                        let var_as_str = &self.annotation_id;
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
        pub mod bookshelves {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum AddVolumeReason {
                    #[doc = "Volumes added from the PREX flow on iOS."]
                    IosPrex,
                    #[doc = "Volumes added from the Search flow on iOS."]
                    IosSearch,
                    #[doc = "Volumes added from the Onboarding flow."]
                    Onboarding,
                }
                impl AddVolumeReason {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            AddVolumeReason::IosPrex => "IOS_PREX",
                            AddVolumeReason::IosSearch => "IOS_SEARCH",
                            AddVolumeReason::Onboarding => "ONBOARDING",
                        }
                    }
                }
                impl ::std::fmt::Display for AddVolumeReason {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for AddVolumeReason {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for AddVolumeReason {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "IOS_PREX" => AddVolumeReason::IosPrex,
                            "IOS_SEARCH" => AddVolumeReason::IosSearch,
                            "ONBOARDING" => AddVolumeReason::Onboarding,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for AddVolumeReason {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for AddVolumeReason {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum RemoveVolumeReason {
                    #[doc = "Samples removed from the Onboarding flow."]
                    Onboarding,
                }
                impl RemoveVolumeReason {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            RemoveVolumeReason::Onboarding => "ONBOARDING",
                        }
                    }
                }
                impl ::std::fmt::Display for RemoveVolumeReason {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for RemoveVolumeReason {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for RemoveVolumeReason {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ONBOARDING" => RemoveVolumeReason::Onboarding,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for RemoveVolumeReason {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for RemoveVolumeReason {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct BookshelvesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> BookshelvesActions<'a, A> {
                #[doc = "Adds a volume to a bookshelf."]
                pub fn add_volume(
                    &self,
                    shelf: impl Into<String>,
                    volume_id: impl Into<String>,
                ) -> AddVolumeRequestBuilder<A> {
                    AddVolumeRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        shelf: shelf.into(),
                        volume_id: volume_id.into(),
                        reason: None,
                        source: None,
                    }
                }
                #[doc = "Clears all volumes from a bookshelf."]
                pub fn clear_volumes(
                    &self,
                    shelf: impl Into<String>,
                ) -> ClearVolumesRequestBuilder<A> {
                    ClearVolumesRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        shelf: shelf.into(),
                        source: None,
                    }
                }
                #[doc = "Retrieves metadata for a specific bookshelf belonging to the authenticated user."]
                pub fn get(&self, shelf: impl Into<String>) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        shelf: shelf.into(),
                        source: None,
                    }
                }
                #[doc = "Retrieves a list of bookshelves belonging to the authenticated user."]
                pub fn list(&self) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        source: None,
                    }
                }
                #[doc = "Moves a volume within a bookshelf."]
                pub fn move_volume(
                    &self,
                    shelf: impl Into<String>,
                    volume_id: impl Into<String>,
                    volume_position: i32,
                ) -> MoveVolumeRequestBuilder<A> {
                    MoveVolumeRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        shelf: shelf.into(),
                        volume_id: volume_id.into(),
                        volume_position,
                        source: None,
                    }
                }
                #[doc = "Removes a volume from a bookshelf."]
                pub fn remove_volume(
                    &self,
                    shelf: impl Into<String>,
                    volume_id: impl Into<String>,
                ) -> RemoveVolumeRequestBuilder<A> {
                    RemoveVolumeRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        shelf: shelf.into(),
                        volume_id: volume_id.into(),
                        reason: None,
                        source: None,
                    }
                }
                #[doc = "Actions that can be performed on the volumes resource"]
                pub fn volumes(
                    &self,
                ) -> crate::resources::mylibrary::bookshelves::volumes::VolumesActions<A>
                {
                    crate::resources::mylibrary::bookshelves::volumes::VolumesActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct AddVolumeRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                shelf: String,
                volume_id: String,
                reason: Option<crate::resources::mylibrary::bookshelves::params::AddVolumeReason>,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> AddVolumeRequestBuilder<'a, A> {
                #[doc = "The reason for which the book is added to the library."]
                pub fn reason(
                    mut self,
                    value: crate::resources::mylibrary::bookshelves::params::AddVolumeReason,
                ) -> Self {
                    self.reason = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path())?;
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/bookshelves/");
                    {
                        let var_as_str = &self.shelf;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/addVolume");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("volumeId", &self.volume_id)]);
                    let req = req.query(&[("reason", &self.reason)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ClearVolumesRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                shelf: String,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ClearVolumesRequestBuilder<'a, A> {
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path())?;
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/bookshelves/");
                    {
                        let var_as_str = &self.shelf;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/clearVolumes");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                shelf: String,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::Bookshelf, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Bookshelf, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/bookshelves/");
                    {
                        let var_as_str = &self.shelf;
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::Bookshelves, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Bookshelves, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/bookshelves");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct MoveVolumeRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                shelf: String,
                volume_id: String,
                volume_position: i32,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> MoveVolumeRequestBuilder<'a, A> {
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path())?;
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/bookshelves/");
                    {
                        let var_as_str = &self.shelf;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/moveVolume");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("volumeId", &self.volume_id)]);
                    let req = req.query(&[("volumePosition", &self.volume_position)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct RemoveVolumeRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                shelf: String,
                volume_id: String,
                reason:
                    Option<crate::resources::mylibrary::bookshelves::params::RemoveVolumeReason>,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> RemoveVolumeRequestBuilder<'a, A> {
                #[doc = "The reason for which the book is removed from the library."]
                pub fn reason(
                    mut self,
                    value: crate::resources::mylibrary::bookshelves::params::RemoveVolumeReason,
                ) -> Self {
                    self.reason = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path())?;
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/bookshelves/");
                    {
                        let var_as_str = &self.shelf;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/removeVolume");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("volumeId", &self.volume_id)]);
                    let req = req.query(&[("reason", &self.reason)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            pub mod volumes {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListProjection {
                        #[doc = "Includes all volume data."]
                        Full,
                        #[doc = "Includes a subset of fields in volumeInfo and accessInfo."]
                        Lite,
                    }
                    impl ListProjection {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListProjection::Full => "full",
                                ListProjection::Lite => "lite",
                            }
                        }
                    }
                    impl ::std::fmt::Display for ListProjection {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListProjection {
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
                    impl<'de> ::serde::Deserialize<'de> for ListProjection {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "full" => ListProjection::Full,
                                "lite" => ListProjection::Lite,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for ListProjection {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for ListProjection {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct VolumesActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a A,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> VolumesActions<'a, A> {
                    #[doc = "Gets volume information for volumes on a bookshelf."]
                    pub fn list(&self, shelf: impl Into<String>) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            shelf: shelf.into(),
                            country: None,
                            max_results: None,
                            projection: None,
                            q: None,
                            show_preorders: None,
                            source: None,
                            start_index: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a A,
                    shelf: String,
                    country: Option<String>,
                    max_results: Option<u32>,
                    projection: Option<
                        crate::resources::mylibrary::bookshelves::volumes::params::ListProjection,
                    >,
                    q: Option<String>,
                    show_preorders: Option<bool>,
                    source: Option<String>,
                    start_index: Option<u32>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                    #[doc = "ISO-3166-1 code to override the IP-based location."]
                    pub fn country(mut self, value: impl Into<String>) -> Self {
                        self.country = Some(value.into());
                        self
                    }
                    #[doc = "Maximum number of results to return"]
                    pub fn max_results(mut self, value: u32) -> Self {
                        self.max_results = Some(value);
                        self
                    }
                    #[doc = "Restrict information returned to a set of selected fields."]
                    pub fn projection(
                        mut self,
                        value : crate :: resources :: mylibrary :: bookshelves :: volumes :: params :: ListProjection,
                    ) -> Self {
                        self.projection = Some(value);
                        self
                    }
                    #[doc = "Full-text search query string in this bookshelf."]
                    pub fn q(mut self, value: impl Into<String>) -> Self {
                        self.q = Some(value.into());
                        self
                    }
                    #[doc = "Set to true to show pre-ordered books. Defaults to false."]
                    pub fn show_preorders(mut self, value: bool) -> Self {
                        self.show_preorders = Some(value);
                        self
                    }
                    #[doc = "String to identify the originator of this request."]
                    pub fn source(mut self, value: impl Into<String>) -> Self {
                        self.source = Some(value.into());
                        self
                    }
                    #[doc = "Index of the first element to return (starts at 0)"]
                    pub fn start_index(mut self, value: u32) -> Self {
                        self.start_index = Some(value);
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
                    ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                        output.push_str("mylibrary/bookshelves/");
                        {
                            let var_as_str = &self.shelf;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/volumes");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                    {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("country", &self.country)]);
                        let req = req.query(&[("maxResults", &self.max_results)]);
                        let req = req.query(&[("projection", &self.projection)]);
                        let req = req.query(&[("q", &self.q)]);
                        let req = req.query(&[("showPreorders", &self.show_preorders)]);
                        let req = req.query(&[("source", &self.source)]);
                        let req = req.query(&[("startIndex", &self.start_index)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let req = req.bearer_auth(self.auth.access_token()?);
                        Ok(req)
                    }
                }
            }
        }
        pub mod readingpositions {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum SetPositionAction {
                    #[doc = "User chose bookmark within volume."]
                    Bookmark,
                    #[doc = "User selected chapter from list."]
                    Chapter,
                    #[doc = "Next page event."]
                    NextPage,
                    #[doc = "Previous page event."]
                    PrevPage,
                    #[doc = "User navigated to page."]
                    Scroll,
                    #[doc = "User chose search results within volume."]
                    Search,
                }
                impl SetPositionAction {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            SetPositionAction::Bookmark => "bookmark",
                            SetPositionAction::Chapter => "chapter",
                            SetPositionAction::NextPage => "next-page",
                            SetPositionAction::PrevPage => "prev-page",
                            SetPositionAction::Scroll => "scroll",
                            SetPositionAction::Search => "search",
                        }
                    }
                }
                impl ::std::fmt::Display for SetPositionAction {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for SetPositionAction {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for SetPositionAction {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "bookmark" => SetPositionAction::Bookmark,
                            "chapter" => SetPositionAction::Chapter,
                            "next-page" => SetPositionAction::NextPage,
                            "prev-page" => SetPositionAction::PrevPage,
                            "scroll" => SetPositionAction::Scroll,
                            "search" => SetPositionAction::Search,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for SetPositionAction {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for SetPositionAction {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ReadingpositionsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ReadingpositionsActions<'a, A> {
                #[doc = "Retrieves my reading position information for a volume."]
                pub fn get(&self, volume_id: impl Into<String>) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        volume_id: volume_id.into(),
                        content_version: None,
                        source: None,
                    }
                }
                #[doc = "Sets my reading position information for a volume."]
                pub fn set_position(
                    &self,
                    volume_id: impl Into<String>,
                    timestamp: impl Into<String>,
                    position: impl Into<String>,
                ) -> SetPositionRequestBuilder<A> {
                    SetPositionRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        volume_id: volume_id.into(),
                        timestamp: timestamp.into(),
                        position: position.into(),
                        action: None,
                        content_version: None,
                        device_cookie: None,
                        source: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                volume_id: String,
                content_version: Option<String>,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
                #[doc = "Volume content version for which this reading position is requested."]
                pub fn content_version(mut self, value: impl Into<String>) -> Self {
                    self.content_version = Some(value.into());
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::ReadingPosition, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ReadingPosition, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/readingpositions/");
                    {
                        let var_as_str = &self.volume_id;
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
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("contentVersion", &self.content_version)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct SetPositionRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                volume_id: String,
                timestamp: String,
                position: String,
                action: Option<
                    crate::resources::mylibrary::readingpositions::params::SetPositionAction,
                >,
                content_version: Option<String>,
                device_cookie: Option<String>,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> SetPositionRequestBuilder<'a, A> {
                #[doc = "Action that caused this reading position to be set."]
                pub fn action(
                    mut self,
                    value: crate::resources::mylibrary::readingpositions::params::SetPositionAction,
                ) -> Self {
                    self.action = Some(value);
                    self
                }
                #[doc = "Volume content version for which this reading position applies."]
                pub fn content_version(mut self, value: impl Into<String>) -> Self {
                    self.content_version = Some(value.into());
                    self
                }
                #[doc = "Random persistent device cookie optional on set position."]
                pub fn device_cookie(mut self, value: impl Into<String>) -> Self {
                    self.device_cookie = Some(value.into());
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path())?;
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("mylibrary/readingpositions/");
                    {
                        let var_as_str = &self.volume_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/setPosition");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("timestamp", &self.timestamp)]);
                    let req = req.query(&[("position", &self.position)]);
                    let req = req.query(&[("action", &self.action)]);
                    let req = req.query(&[("contentVersion", &self.content_version)]);
                    let req = req.query(&[("deviceCookie", &self.device_cookie)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
    }
    pub mod notification {
        pub mod params {}
        pub struct NotificationActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> NotificationActions<'a, A> {
            #[doc = "Returns notification details for a given notification id."]
            pub fn get(&self, notification_id: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    notification_id: notification_id.into(),
                    locale: None,
                    source: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            notification_id: String,
            locale: Option<String>,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating notification title and body."]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("notification/get");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("notification_id", &self.notification_id)]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
    }
    pub mod onboarding {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListCategoryVolumesMaxAllowedMaturityRating {
                #[doc = "Show books which are rated mature or lower."]
                Mature,
                #[doc = "Show books which are rated not mature."]
                NotMature,
            }
            impl ListCategoryVolumesMaxAllowedMaturityRating {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListCategoryVolumesMaxAllowedMaturityRating::Mature => "mature",
                        ListCategoryVolumesMaxAllowedMaturityRating::NotMature => "not-mature",
                    }
                }
            }
            impl ::std::fmt::Display for ListCategoryVolumesMaxAllowedMaturityRating {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListCategoryVolumesMaxAllowedMaturityRating {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListCategoryVolumesMaxAllowedMaturityRating {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "mature" => ListCategoryVolumesMaxAllowedMaturityRating::Mature,
                        "not-mature" => ListCategoryVolumesMaxAllowedMaturityRating::NotMature,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListCategoryVolumesMaxAllowedMaturityRating {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListCategoryVolumesMaxAllowedMaturityRating {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct OnboardingActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> OnboardingActions<'a, A> {
            #[doc = "List categories for onboarding experience."]
            pub fn list_categories(&self) -> ListCategoriesRequestBuilder<A> {
                ListCategoriesRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    locale: None,
                }
            }
            #[doc = "List available volumes under categories for onboarding experience."]
            pub fn list_category_volumes(&self) -> ListCategoryVolumesRequestBuilder<A> {
                ListCategoryVolumesRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    category_id: None,
                    locale: None,
                    max_allowed_maturity_rating: None,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListCategoriesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            locale: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ListCategoriesRequestBuilder<'a, A> {
            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset."]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
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
            ) -> Result<crate::schemas::Category, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Category, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("onboarding/listCategories");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListCategoryVolumesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            category_id: Option<Vec<String>>,
            locale: Option<String>,
            max_allowed_maturity_rating: Option<
                crate::resources::onboarding::params::ListCategoryVolumesMaxAllowedMaturityRating,
            >,
            page_size: Option<u32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ListCategoryVolumesRequestBuilder<'a, A> {
            #[doc = "List of category ids requested."]
            pub fn category_id(mut self, value: impl Into<Vec<String>>) -> Self {
                self.category_id = Some(value.into());
                self
            }
            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Default is en-US if unset."]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "The maximum allowed maturity rating of returned volumes. Books with a higher maturity rating are filtered out."]
            pub fn max_allowed_maturity_rating(
                mut self,
                value : crate :: resources :: onboarding :: params :: ListCategoryVolumesMaxAllowedMaturityRating,
            ) -> Self {
                self.max_allowed_maturity_rating = Some(value);
                self
            }
            #[doc = "Number of maximum results per page to be included in the response."]
            pub fn page_size(mut self, value: u32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The value of the nextToken from the previous page."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Volume> {
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Volume> {
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
            ) -> crate::iter::PageIter<Self, crate::schemas::Volume2> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::Volume2> {
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
            ) -> Result<crate::schemas::Volume2, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Volume2, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("onboarding/listCategoryVolumes");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("categoryId", &self.category_id)]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[(
                    "maxAllowedMaturityRating",
                    &self.max_allowed_maturity_rating,
                )]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> crate::iter::IterableMethod
            for ListCategoryVolumesRequestBuilder<'a, A>
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
    pub mod personalizedstream {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetMaxAllowedMaturityRating {
                #[doc = "Show books which are rated mature or lower."]
                Mature,
                #[doc = "Show books which are rated not mature."]
                NotMature,
            }
            impl GetMaxAllowedMaturityRating {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetMaxAllowedMaturityRating::Mature => "mature",
                        GetMaxAllowedMaturityRating::NotMature => "not-mature",
                    }
                }
            }
            impl ::std::fmt::Display for GetMaxAllowedMaturityRating {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetMaxAllowedMaturityRating {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetMaxAllowedMaturityRating {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "mature" => GetMaxAllowedMaturityRating::Mature,
                        "not-mature" => GetMaxAllowedMaturityRating::NotMature,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetMaxAllowedMaturityRating {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetMaxAllowedMaturityRating {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct PersonalizedstreamActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> PersonalizedstreamActions<'a, A> {
            #[doc = "Returns a stream of personalized book clusters"]
            pub fn get(&self) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    locale: None,
                    max_allowed_maturity_rating: None,
                    source: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            locale: Option<String>,
            max_allowed_maturity_rating:
                Option<crate::resources::personalizedstream::params::GetMaxAllowedMaturityRating>,
            source: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
            #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
            pub fn locale(mut self, value: impl Into<String>) -> Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
            pub fn max_allowed_maturity_rating(
                mut self,
                value: crate::resources::personalizedstream::params::GetMaxAllowedMaturityRating,
            ) -> Self {
                self.max_allowed_maturity_rating = Some(value);
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
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
            ) -> Result<crate::schemas::Discoveryclusters, Box<dyn ::std::error::Error>>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Discoveryclusters, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("personalizedstream/get");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[(
                    "maxAllowedMaturityRating",
                    &self.max_allowed_maturity_rating,
                )]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
    }
    pub mod promooffer {
        pub mod params {}
        pub struct PromoofferActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> PromoofferActions<'a, A> {
            #[doc = ""]
            pub fn accept(&self) -> AcceptRequestBuilder<A> {
                AcceptRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    android_id: None,
                    device: None,
                    manufacturer: None,
                    model: None,
                    offer_id: None,
                    product: None,
                    serial: None,
                    volume_id: None,
                }
            }
            #[doc = ""]
            pub fn dismiss(&self) -> DismissRequestBuilder<A> {
                DismissRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    android_id: None,
                    device: None,
                    manufacturer: None,
                    model: None,
                    offer_id: None,
                    product: None,
                    serial: None,
                }
            }
            #[doc = "Returns a list of promo offers available to the user"]
            pub fn get(&self) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    android_id: None,
                    device: None,
                    manufacturer: None,
                    model: None,
                    product: None,
                    serial: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct AcceptRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            android_id: Option<String>,
            device: Option<String>,
            manufacturer: Option<String>,
            model: Option<String>,
            offer_id: Option<String>,
            product: Option<String>,
            serial: Option<String>,
            volume_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> AcceptRequestBuilder<'a, A> {
            #[doc = "device android_id"]
            pub fn android_id(mut self, value: impl Into<String>) -> Self {
                self.android_id = Some(value.into());
                self
            }
            #[doc = "device device"]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = "device manufacturer"]
            pub fn manufacturer(mut self, value: impl Into<String>) -> Self {
                self.manufacturer = Some(value.into());
                self
            }
            #[doc = "device model"]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn offer_id(mut self, value: impl Into<String>) -> Self {
                self.offer_id = Some(value.into());
                self
            }
            #[doc = "device product"]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = "device serial"]
            pub fn serial(mut self, value: impl Into<String>) -> Self {
                self.serial = Some(value.into());
                self
            }
            #[doc = "Volume id to exercise the offer"]
            pub fn volume_id(mut self, value: impl Into<String>) -> Self {
                self.volume_id = Some(value.into());
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
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path())?;
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("promooffer/accept");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("androidId", &self.android_id)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("manufacturer", &self.manufacturer)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("offerId", &self.offer_id)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("serial", &self.serial)]);
                let req = req.query(&[("volumeId", &self.volume_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct DismissRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            android_id: Option<String>,
            device: Option<String>,
            manufacturer: Option<String>,
            model: Option<String>,
            offer_id: Option<String>,
            product: Option<String>,
            serial: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> DismissRequestBuilder<'a, A> {
            #[doc = "device android_id"]
            pub fn android_id(mut self, value: impl Into<String>) -> Self {
                self.android_id = Some(value.into());
                self
            }
            #[doc = "device device"]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = "device manufacturer"]
            pub fn manufacturer(mut self, value: impl Into<String>) -> Self {
                self.manufacturer = Some(value.into());
                self
            }
            #[doc = "device model"]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = "Offer to dimiss"]
            pub fn offer_id(mut self, value: impl Into<String>) -> Self {
                self.offer_id = Some(value.into());
                self
            }
            #[doc = "device product"]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = "device serial"]
            pub fn serial(mut self, value: impl Into<String>) -> Self {
                self.serial = Some(value.into());
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
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path())?;
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("promooffer/dismiss");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("androidId", &self.android_id)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("manufacturer", &self.manufacturer)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("offerId", &self.offer_id)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("serial", &self.serial)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            android_id: Option<String>,
            device: Option<String>,
            manufacturer: Option<String>,
            model: Option<String>,
            product: Option<String>,
            serial: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
            #[doc = "device android_id"]
            pub fn android_id(mut self, value: impl Into<String>) -> Self {
                self.android_id = Some(value.into());
                self
            }
            #[doc = "device device"]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = "device manufacturer"]
            pub fn manufacturer(mut self, value: impl Into<String>) -> Self {
                self.manufacturer = Some(value.into());
                self
            }
            #[doc = "device model"]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = "device product"]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = "device serial"]
            pub fn serial(mut self, value: impl Into<String>) -> Self {
                self.serial = Some(value.into());
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
            ) -> Result<crate::schemas::Offers, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Offers, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("promooffer/get");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("androidId", &self.android_id)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("manufacturer", &self.manufacturer)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("serial", &self.serial)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
    }
    pub mod series {
        pub mod params {}
        pub struct SeriesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> SeriesActions<'a, A> {
            #[doc = "Returns Series metadata for the given series ids."]
            pub fn get(&self, series_id: impl Into<Vec<String>>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    series_id: series_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the membership resource"]
            pub fn membership(&self) -> crate::resources::series::membership::MembershipActions<A> {
                crate::resources::series::membership::MembershipActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            series_id: Vec<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Series, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Series, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("series/get");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("series_id", &self.series_id)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        pub mod membership {
            pub mod params {}
            pub struct MembershipActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> MembershipActions<'a, A> {
                #[doc = "Returns Series membership data given the series id."]
                pub fn get(&self, series_id: impl Into<String>) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        series_id: series_id.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                series_id: String,
                page_size: Option<u32>,
                page_token: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
                #[doc = "Number of maximum results per page to be included in the response."]
                pub fn page_size(mut self, value: u32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The value of the nextToken from the previous page."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
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
                ) -> Result<crate::schemas::Seriesmembership, Box<dyn ::std::error::Error>>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Seriesmembership, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("series/membership/get");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("series_id", &self.series_id)]);
                    let req = req.query(&[("page_size", &self.page_size)]);
                    let req = req.query(&[("page_token", &self.page_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
    }
    pub mod volumes {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetProjection {
                #[doc = "Includes all volume data."]
                Full,
                #[doc = "Includes a subset of fields in volumeInfo and accessInfo."]
                Lite,
            }
            impl GetProjection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetProjection::Full => "full",
                        GetProjection::Lite => "lite",
                    }
                }
            }
            impl ::std::fmt::Display for GetProjection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetProjection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetProjection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "full" => GetProjection::Full,
                        "lite" => GetProjection::Lite,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetProjection {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetProjection {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListDownload {
                #[doc = "All volumes with epub."]
                Epub,
            }
            impl ListDownload {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListDownload::Epub => "epub",
                    }
                }
            }
            impl ::std::fmt::Display for ListDownload {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListDownload {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListDownload {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "epub" => ListDownload::Epub,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListDownload {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListDownload {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListFilter {
                #[doc = "All Google eBooks."]
                Ebooks,
                #[doc = "Google eBook with full volume text viewability."]
                FreeEbooks,
                #[doc = "Public can view entire volume text."]
                Full,
                #[doc = "Google eBook with a price."]
                PaidEbooks,
                #[doc = "Public able to see parts of text."]
                Partial,
            }
            impl ListFilter {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListFilter::Ebooks => "ebooks",
                        ListFilter::FreeEbooks => "free-ebooks",
                        ListFilter::Full => "full",
                        ListFilter::PaidEbooks => "paid-ebooks",
                        ListFilter::Partial => "partial",
                    }
                }
            }
            impl ::std::fmt::Display for ListFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListFilter {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListFilter {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ebooks" => ListFilter::Ebooks,
                        "free-ebooks" => ListFilter::FreeEbooks,
                        "full" => ListFilter::Full,
                        "paid-ebooks" => ListFilter::PaidEbooks,
                        "partial" => ListFilter::Partial,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListFilter {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListFilter {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListLibraryRestrict {
                #[doc = "Restrict to the user's library, any shelf."]
                MyLibrary,
                #[doc = "Do not restrict based on user's library."]
                NoRestrict,
            }
            impl ListLibraryRestrict {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListLibraryRestrict::MyLibrary => "my-library",
                        ListLibraryRestrict::NoRestrict => "no-restrict",
                    }
                }
            }
            impl ::std::fmt::Display for ListLibraryRestrict {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListLibraryRestrict {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListLibraryRestrict {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "my-library" => ListLibraryRestrict::MyLibrary,
                        "no-restrict" => ListLibraryRestrict::NoRestrict,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListLibraryRestrict {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListLibraryRestrict {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListMaxAllowedMaturityRating {
                #[doc = "Show books which are rated mature or lower."]
                Mature,
                #[doc = "Show books which are rated not mature."]
                NotMature,
            }
            impl ListMaxAllowedMaturityRating {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListMaxAllowedMaturityRating::Mature => "mature",
                        ListMaxAllowedMaturityRating::NotMature => "not-mature",
                    }
                }
            }
            impl ::std::fmt::Display for ListMaxAllowedMaturityRating {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListMaxAllowedMaturityRating {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListMaxAllowedMaturityRating {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "mature" => ListMaxAllowedMaturityRating::Mature,
                        "not-mature" => ListMaxAllowedMaturityRating::NotMature,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListMaxAllowedMaturityRating {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListMaxAllowedMaturityRating {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListOrderBy {
                #[doc = "Most recently published."]
                Newest,
                #[doc = "Relevance to search terms."]
                Relevance,
            }
            impl ListOrderBy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListOrderBy::Newest => "newest",
                        ListOrderBy::Relevance => "relevance",
                    }
                }
            }
            impl ::std::fmt::Display for ListOrderBy {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListOrderBy {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListOrderBy {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "newest" => ListOrderBy::Newest,
                        "relevance" => ListOrderBy::Relevance,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListOrderBy {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListOrderBy {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListPrintType {
                #[doc = "All volume content types."]
                All,
                #[doc = "Just books."]
                Books,
                #[doc = "Just magazines."]
                Magazines,
            }
            impl ListPrintType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListPrintType::All => "all",
                        ListPrintType::Books => "books",
                        ListPrintType::Magazines => "magazines",
                    }
                }
            }
            impl ::std::fmt::Display for ListPrintType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListPrintType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListPrintType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "all" => ListPrintType::All,
                        "books" => ListPrintType::Books,
                        "magazines" => ListPrintType::Magazines,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListPrintType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListPrintType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListProjection {
                #[doc = "Includes all volume data."]
                Full,
                #[doc = "Includes a subset of fields in volumeInfo and accessInfo."]
                Lite,
            }
            impl ListProjection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListProjection::Full => "full",
                        ListProjection::Lite => "lite",
                    }
                }
            }
            impl ::std::fmt::Display for ListProjection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListProjection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListProjection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "full" => ListProjection::Full,
                        "lite" => ListProjection::Lite,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListProjection {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListProjection {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct VolumesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a A,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> VolumesActions<'a, A> {
            #[doc = "Gets volume information for a single volume."]
            pub fn get(&self, volume_id: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    volume_id: volume_id.into(),
                    country: None,
                    include_non_comics_series: None,
                    partner: None,
                    projection: None,
                    source: None,
                    user_library_consistent_read: None,
                }
            }
            #[doc = "Performs a book search."]
            pub fn list(&self, q: impl Into<String>) -> ListRequestBuilder<A> {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    q: q.into(),
                    download: None,
                    filter: None,
                    lang_restrict: None,
                    library_restrict: None,
                    max_allowed_maturity_rating: None,
                    max_results: None,
                    order_by: None,
                    partner: None,
                    print_type: None,
                    projection: None,
                    show_preorders: None,
                    source: None,
                    start_index: None,
                }
            }
            #[doc = "Actions that can be performed on the associated resource"]
            pub fn associated(
                &self,
            ) -> crate::resources::volumes::associated::AssociatedActions<A> {
                crate::resources::volumes::associated::AssociatedActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the mybooks resource"]
            pub fn mybooks(&self) -> crate::resources::volumes::mybooks::MybooksActions<A> {
                crate::resources::volumes::mybooks::MybooksActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the recommended resource"]
            pub fn recommended(
                &self,
            ) -> crate::resources::volumes::recommended::RecommendedActions<A> {
                crate::resources::volumes::recommended::RecommendedActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the useruploaded resource"]
            pub fn useruploaded(
                &self,
            ) -> crate::resources::volumes::useruploaded::UseruploadedActions<A> {
                crate::resources::volumes::useruploaded::UseruploadedActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            volume_id: String,
            country: Option<String>,
            include_non_comics_series: Option<bool>,
            partner: Option<String>,
            projection: Option<crate::resources::volumes::params::GetProjection>,
            source: Option<String>,
            user_library_consistent_read: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> GetRequestBuilder<'a, A> {
            #[doc = "ISO-3166-1 code to override the IP-based location."]
            pub fn country(mut self, value: impl Into<String>) -> Self {
                self.country = Some(value.into());
                self
            }
            #[doc = "Set to true to include non-comics series. Defaults to false."]
            pub fn include_non_comics_series(mut self, value: bool) -> Self {
                self.include_non_comics_series = Some(value);
                self
            }
            #[doc = "Brand results for partner ID."]
            pub fn partner(mut self, value: impl Into<String>) -> Self {
                self.partner = Some(value.into());
                self
            }
            #[doc = "Restrict information returned to a set of selected fields."]
            pub fn projection(
                mut self,
                value: crate::resources::volumes::params::GetProjection,
            ) -> Self {
                self.projection = Some(value);
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn user_library_consistent_read(mut self, value: bool) -> Self {
                self.user_library_consistent_read = Some(value);
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
            ) -> Result<crate::schemas::Volume, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Volume, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("volumes/");
                {
                    let var_as_str = &self.volume_id;
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
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("country", &self.country)]);
                let req = req.query(&[("includeNonComicsSeries", &self.include_non_comics_series)]);
                let req = req.query(&[("partner", &self.partner)]);
                let req = req.query(&[("projection", &self.projection)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[(
                    "user_library_consistent_read",
                    &self.user_library_consistent_read,
                )]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a A,
            q: String,
            download: Option<crate::resources::volumes::params::ListDownload>,
            filter: Option<crate::resources::volumes::params::ListFilter>,
            lang_restrict: Option<String>,
            library_restrict: Option<crate::resources::volumes::params::ListLibraryRestrict>,
            max_allowed_maturity_rating:
                Option<crate::resources::volumes::params::ListMaxAllowedMaturityRating>,
            max_results: Option<u32>,
            order_by: Option<crate::resources::volumes::params::ListOrderBy>,
            partner: Option<String>,
            print_type: Option<crate::resources::volumes::params::ListPrintType>,
            projection: Option<crate::resources::volumes::params::ListProjection>,
            show_preorders: Option<bool>,
            source: Option<String>,
            start_index: Option<u32>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
            #[doc = "Restrict to volumes by download availability."]
            pub fn download(
                mut self,
                value: crate::resources::volumes::params::ListDownload,
            ) -> Self {
                self.download = Some(value);
                self
            }
            #[doc = "Filter search results."]
            pub fn filter(mut self, value: crate::resources::volumes::params::ListFilter) -> Self {
                self.filter = Some(value);
                self
            }
            #[doc = "Restrict results to books with this language code."]
            pub fn lang_restrict(mut self, value: impl Into<String>) -> Self {
                self.lang_restrict = Some(value.into());
                self
            }
            #[doc = "Restrict search to this user's library."]
            pub fn library_restrict(
                mut self,
                value: crate::resources::volumes::params::ListLibraryRestrict,
            ) -> Self {
                self.library_restrict = Some(value);
                self
            }
            #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
            pub fn max_allowed_maturity_rating(
                mut self,
                value: crate::resources::volumes::params::ListMaxAllowedMaturityRating,
            ) -> Self {
                self.max_allowed_maturity_rating = Some(value);
                self
            }
            #[doc = "Maximum number of results to return."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Sort search results."]
            pub fn order_by(
                mut self,
                value: crate::resources::volumes::params::ListOrderBy,
            ) -> Self {
                self.order_by = Some(value);
                self
            }
            #[doc = "Restrict and brand results for partner ID."]
            pub fn partner(mut self, value: impl Into<String>) -> Self {
                self.partner = Some(value.into());
                self
            }
            #[doc = "Restrict to books or magazines."]
            pub fn print_type(
                mut self,
                value: crate::resources::volumes::params::ListPrintType,
            ) -> Self {
                self.print_type = Some(value);
                self
            }
            #[doc = "Restrict information returned to a set of selected fields."]
            pub fn projection(
                mut self,
                value: crate::resources::volumes::params::ListProjection,
            ) -> Self {
                self.projection = Some(value);
                self
            }
            #[doc = "Set to true to show books available for preorder. Defaults to false."]
            pub fn show_preorders(mut self, value: bool) -> Self {
                self.show_preorders = Some(value);
                self
            }
            #[doc = "String to identify the originator of this request."]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
                self
            }
            #[doc = "Index of the first result to return (starts at 0)"]
            pub fn start_index(mut self, value: u32) -> Self {
                self.start_index = Some(value);
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
            ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                output.push_str("volumes");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("q", &self.q)]);
                let req = req.query(&[("download", &self.download)]);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("langRestrict", &self.lang_restrict)]);
                let req = req.query(&[("libraryRestrict", &self.library_restrict)]);
                let req = req.query(&[(
                    "maxAllowedMaturityRating",
                    &self.max_allowed_maturity_rating,
                )]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("orderBy", &self.order_by)]);
                let req = req.query(&[("partner", &self.partner)]);
                let req = req.query(&[("printType", &self.print_type)]);
                let req = req.query(&[("projection", &self.projection)]);
                let req = req.query(&[("showPreorders", &self.show_preorders)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("startIndex", &self.start_index)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let req = req.bearer_auth(self.auth.access_token()?);
                Ok(req)
            }
        }
        pub mod associated {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListAssociation {
                    #[doc = "Recommendations for display end-of-sample."]
                    EndOfSample,
                    #[doc = "Recommendations for display end-of-volume."]
                    EndOfVolume,
                    #[doc = "Related volumes for Play Store."]
                    RelatedForPlay,
                }
                impl ListAssociation {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListAssociation::EndOfSample => "end-of-sample",
                            ListAssociation::EndOfVolume => "end-of-volume",
                            ListAssociation::RelatedForPlay => "related-for-play",
                        }
                    }
                }
                impl ::std::fmt::Display for ListAssociation {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListAssociation {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListAssociation {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "end-of-sample" => ListAssociation::EndOfSample,
                            "end-of-volume" => ListAssociation::EndOfVolume,
                            "related-for-play" => ListAssociation::RelatedForPlay,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListAssociation {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListAssociation {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListMaxAllowedMaturityRating {
                    #[doc = "Show books which are rated mature or lower."]
                    Mature,
                    #[doc = "Show books which are rated not mature."]
                    NotMature,
                }
                impl ListMaxAllowedMaturityRating {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListMaxAllowedMaturityRating::Mature => "mature",
                            ListMaxAllowedMaturityRating::NotMature => "not-mature",
                        }
                    }
                }
                impl ::std::fmt::Display for ListMaxAllowedMaturityRating {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListMaxAllowedMaturityRating {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListMaxAllowedMaturityRating {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "mature" => ListMaxAllowedMaturityRating::Mature,
                            "not-mature" => ListMaxAllowedMaturityRating::NotMature,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListMaxAllowedMaturityRating {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListMaxAllowedMaturityRating {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct AssociatedActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> AssociatedActions<'a, A> {
                #[doc = "Return a list of associated books."]
                pub fn list(&self, volume_id: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        volume_id: volume_id.into(),
                        association: None,
                        locale: None,
                        max_allowed_maturity_rating: None,
                        source: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                volume_id: String,
                association: Option<crate::resources::volumes::associated::params::ListAssociation>,
                locale: Option<String>,
                max_allowed_maturity_rating: Option<
                    crate::resources::volumes::associated::params::ListMaxAllowedMaturityRating,
                >,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "Association type."]
                pub fn association(
                    mut self,
                    value: crate::resources::volumes::associated::params::ListAssociation,
                ) -> Self {
                    self.association = Some(value);
                    self
                }
                #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                pub fn max_allowed_maturity_rating(
                    mut self,
                    value : crate :: resources :: volumes :: associated :: params :: ListMaxAllowedMaturityRating,
                ) -> Self {
                    self.max_allowed_maturity_rating = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/");
                    {
                        let var_as_str = &self.volume_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/associated");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("association", &self.association)]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[(
                        "maxAllowedMaturityRating",
                        &self.max_allowed_maturity_rating,
                    )]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
        pub mod mybooks {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListAcquireMethodItems {
                    #[doc = "Books acquired via Family Sharing"]
                    FamilyShared,
                    #[doc = "Preordered books (not yet available)"]
                    Preordered,
                    #[doc = "User-rented books past their expiration time"]
                    PreviouslyRented,
                    #[doc = "Public domain books"]
                    PublicDomain,
                    #[doc = "Purchased books"]
                    Purchased,
                    #[doc = "User-rented books"]
                    Rented,
                    #[doc = "Sample books"]
                    Sample,
                    #[doc = "User uploaded books"]
                    Uploaded,
                }
                impl ListAcquireMethodItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListAcquireMethodItems::FamilyShared => "FAMILY_SHARED",
                            ListAcquireMethodItems::Preordered => "PREORDERED",
                            ListAcquireMethodItems::PreviouslyRented => "PREVIOUSLY_RENTED",
                            ListAcquireMethodItems::PublicDomain => "PUBLIC_DOMAIN",
                            ListAcquireMethodItems::Purchased => "PURCHASED",
                            ListAcquireMethodItems::Rented => "RENTED",
                            ListAcquireMethodItems::Sample => "SAMPLE",
                            ListAcquireMethodItems::Uploaded => "UPLOADED",
                        }
                    }
                }
                impl ::std::fmt::Display for ListAcquireMethodItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListAcquireMethodItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListAcquireMethodItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FAMILY_SHARED" => ListAcquireMethodItems::FamilyShared,
                            "PREORDERED" => ListAcquireMethodItems::Preordered,
                            "PREVIOUSLY_RENTED" => ListAcquireMethodItems::PreviouslyRented,
                            "PUBLIC_DOMAIN" => ListAcquireMethodItems::PublicDomain,
                            "PURCHASED" => ListAcquireMethodItems::Purchased,
                            "RENTED" => ListAcquireMethodItems::Rented,
                            "SAMPLE" => ListAcquireMethodItems::Sample,
                            "UPLOADED" => ListAcquireMethodItems::Uploaded,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListAcquireMethodItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListAcquireMethodItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListProcessingStateItems {
                    #[doc = "The volume processing hase failed."]
                    CompletedFailed,
                    #[doc = "The volume processing was completed."]
                    CompletedSuccess,
                    #[doc = "The volume processing is not completed."]
                    Running,
                }
                impl ListProcessingStateItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListProcessingStateItems::CompletedFailed => "COMPLETED_FAILED",
                            ListProcessingStateItems::CompletedSuccess => "COMPLETED_SUCCESS",
                            ListProcessingStateItems::Running => "RUNNING",
                        }
                    }
                }
                impl ::std::fmt::Display for ListProcessingStateItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListProcessingStateItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListProcessingStateItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "COMPLETED_FAILED" => ListProcessingStateItems::CompletedFailed,
                            "COMPLETED_SUCCESS" => ListProcessingStateItems::CompletedSuccess,
                            "RUNNING" => ListProcessingStateItems::Running,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListProcessingStateItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListProcessingStateItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct MybooksActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> MybooksActions<'a, A> {
                #[doc = "Return a list of books in My Library."]
                pub fn list(&self) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        acquire_method: None,
                        country: None,
                        locale: None,
                        max_results: None,
                        processing_state: None,
                        source: None,
                        start_index: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                acquire_method:
                    Option<Vec<crate::resources::volumes::mybooks::params::ListAcquireMethodItems>>,
                country: Option<String>,
                locale: Option<String>,
                max_results: Option<u32>,
                processing_state: Option<
                    Vec<crate::resources::volumes::mybooks::params::ListProcessingStateItems>,
                >,
                source: Option<String>,
                start_index: Option<u32>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "How the book was acquired"]
                pub fn acquire_method(
                    mut self,
                    value: impl Into<
                        Vec<crate::resources::volumes::mybooks::params::ListAcquireMethodItems>,
                    >,
                ) -> Self {
                    self.acquire_method = Some(value.into());
                    self
                }
                #[doc = "ISO-3166-1 code to override the IP-based location."]
                pub fn country(mut self, value: impl Into<String>) -> Self {
                    self.country = Some(value.into());
                    self
                }
                #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex:'en_US'. Used for generating recommendations."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return."]
                pub fn max_results(mut self, value: u32) -> Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "The processing state of the user uploaded volumes to be returned. Applicable only if the UPLOADED is specified in the acquireMethod."]
                pub fn processing_state(
                    mut self,
                    value: impl Into<
                        Vec<crate::resources::volumes::mybooks::params::ListProcessingStateItems>,
                    >,
                ) -> Self {
                    self.processing_state = Some(value.into());
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
                    self
                }
                #[doc = "Index of the first result to return (starts at 0)"]
                pub fn start_index(mut self, value: u32) -> Self {
                    self.start_index = Some(value);
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
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/mybooks");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("acquireMethod", &self.acquire_method)]);
                    let req = req.query(&[("country", &self.country)]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("processingState", &self.processing_state)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("startIndex", &self.start_index)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
        pub mod recommended {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListMaxAllowedMaturityRating {
                    #[doc = "Show books which are rated mature or lower."]
                    Mature,
                    #[doc = "Show books which are rated not mature."]
                    NotMature,
                }
                impl ListMaxAllowedMaturityRating {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListMaxAllowedMaturityRating::Mature => "mature",
                            ListMaxAllowedMaturityRating::NotMature => "not-mature",
                        }
                    }
                }
                impl ::std::fmt::Display for ListMaxAllowedMaturityRating {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListMaxAllowedMaturityRating {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListMaxAllowedMaturityRating {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "mature" => ListMaxAllowedMaturityRating::Mature,
                            "not-mature" => ListMaxAllowedMaturityRating::NotMature,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListMaxAllowedMaturityRating {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListMaxAllowedMaturityRating {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum RateRating {
                    #[doc = "Rating indicating a dismissal due to ownership."]
                    HaveIt,
                    #[doc = "Rating indicating a negative dismissal of a volume."]
                    NotInterested,
                }
                impl RateRating {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            RateRating::HaveIt => "HAVE_IT",
                            RateRating::NotInterested => "NOT_INTERESTED",
                        }
                    }
                }
                impl ::std::fmt::Display for RateRating {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for RateRating {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for RateRating {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "HAVE_IT" => RateRating::HaveIt,
                            "NOT_INTERESTED" => RateRating::NotInterested,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for RateRating {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for RateRating {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct RecommendedActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> RecommendedActions<'a, A> {
                #[doc = "Return a list of recommended books for the current user."]
                pub fn list(&self) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        locale: None,
                        max_allowed_maturity_rating: None,
                        source: None,
                    }
                }
                #[doc = "Rate a recommended book for the current user."]
                pub fn rate(
                    &self,
                    rating: crate::resources::volumes::recommended::params::RateRating,
                    volume_id: impl Into<String>,
                ) -> RateRequestBuilder<A> {
                    RateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        rating,
                        volume_id: volume_id.into(),
                        locale: None,
                        source: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                locale: Option<String>,
                max_allowed_maturity_rating: Option<
                    crate::resources::volumes::recommended::params::ListMaxAllowedMaturityRating,
                >,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "The maximum allowed maturity rating of returned recommendations. Books with a higher maturity rating are filtered out."]
                pub fn max_allowed_maturity_rating(
                    mut self,
                    value : crate :: resources :: volumes :: recommended :: params :: ListMaxAllowedMaturityRating,
                ) -> Self {
                    self.max_allowed_maturity_rating = Some(value);
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/recommended");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[(
                        "maxAllowedMaturityRating",
                        &self.max_allowed_maturity_rating,
                    )]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
            #[derive(Debug, Clone)]
            pub struct RateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                rating: crate::resources::volumes::recommended::params::RateRating,
                volume_id: String,
                locale: Option<String>,
                source: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> RateRequestBuilder<'a, A> {
                #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
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
                    crate::schemas::BooksVolumesRecommendedRateResponse,
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
                    crate::schemas::BooksVolumesRecommendedRateResponse,
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/recommended/rate");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("rating", &self.rating)]);
                    let req = req.query(&[("volumeId", &self.volume_id)]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
                }
            }
        }
        pub mod useruploaded {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListProcessingStateItems {
                    #[doc = "The volume processing hase failed."]
                    CompletedFailed,
                    #[doc = "The volume processing was completed."]
                    CompletedSuccess,
                    #[doc = "The volume processing is not completed."]
                    Running,
                }
                impl ListProcessingStateItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListProcessingStateItems::CompletedFailed => "COMPLETED_FAILED",
                            ListProcessingStateItems::CompletedSuccess => "COMPLETED_SUCCESS",
                            ListProcessingStateItems::Running => "RUNNING",
                        }
                    }
                }
                impl ::std::fmt::Display for ListProcessingStateItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListProcessingStateItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListProcessingStateItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "COMPLETED_FAILED" => ListProcessingStateItems::CompletedFailed,
                            "COMPLETED_SUCCESS" => ListProcessingStateItems::CompletedSuccess,
                            "RUNNING" => ListProcessingStateItems::Running,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListProcessingStateItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListProcessingStateItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct UseruploadedActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a A,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> UseruploadedActions<'a, A> {
                #[doc = "Return a list of books uploaded by the current user."]
                pub fn list(&self) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        locale: None,
                        max_results: None,
                        processing_state: None,
                        source: None,
                        start_index: None,
                        volume_id: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a A,
                locale: Option<String>,
                max_results: Option<u32>,
                processing_state: Option<
                    Vec<crate::resources::volumes::useruploaded::params::ListProcessingStateItems>,
                >,
                source: Option<String>,
                start_index: Option<u32>,
                volume_id: Option<Vec<String>>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: ::google_api_auth::GetAccessToken> ListRequestBuilder<'a, A> {
                #[doc = "ISO-639-1 language and ISO-3166-1 country code. Ex: 'en_US'. Used for generating recommendations."]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return."]
                pub fn max_results(mut self, value: u32) -> Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "The processing state of the user uploaded volumes to be returned."]
                pub fn processing_state(
                    mut self,
                    value : impl Into < Vec < crate :: resources :: volumes :: useruploaded :: params :: ListProcessingStateItems > >,
                ) -> Self {
                    self.processing_state = Some(value.into());
                    self
                }
                #[doc = "String to identify the originator of this request."]
                pub fn source(mut self, value: impl Into<String>) -> Self {
                    self.source = Some(value.into());
                    self
                }
                #[doc = "Index of the first result to return (starts at 0)"]
                pub fn start_index(mut self, value: u32) -> Self {
                    self.start_index = Some(value);
                    self
                }
                #[doc = "The ids of the volumes to be returned. If not specified all that match the processingState are returned."]
                pub fn volume_id(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.volume_id = Some(value.into());
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
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Volumes, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://www.googleapis.com/books/v1/".to_owned();
                    output.push_str("volumes/useruploaded");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, Box<dyn ::std::error::Error>>
                {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("processingState", &self.processing_state)]);
                    let req = req.query(&[("source", &self.source)]);
                    let req = req.query(&[("startIndex", &self.start_index)]);
                    let req = req.query(&[("volumeId", &self.volume_id)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let req = req.bearer_auth(self.auth.access_token()?);
                    Ok(req)
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
