#![doc = "# Resources and Methods\n    * [presentations](resources/presentations/struct.PresentationsActions.html)\n      * [*batchUpdate*](resources/presentations/struct.BatchUpdateRequestBuilder.html), [*create*](resources/presentations/struct.CreateRequestBuilder.html), [*get*](resources/presentations/struct.GetRequestBuilder.html)\n      * [pages](resources/presentations/pages/struct.PagesActions.html)\n        * [*get*](resources/presentations/pages/struct.GetRequestBuilder.html), [*getThumbnail*](resources/presentations/pages/struct.GetThumbnailRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, create, and delete all of your Google Drive files\n\n`https://www.googleapis.com/auth/drive`"]
    pub const DRIVE: &str = "https://www.googleapis.com/auth/drive";
    #[doc = "View and manage Google Drive files and folders that you have opened or created with this app\n\n`https://www.googleapis.com/auth/drive.file`"]
    pub const DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
    #[doc = "See and download all your Google Drive files\n\n`https://www.googleapis.com/auth/drive.readonly`"]
    pub const DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
    #[doc = "View and manage your Google Slides presentations\n\n`https://www.googleapis.com/auth/presentations`"]
    pub const PRESENTATIONS: &str = "https://www.googleapis.com/auth/presentations";
    #[doc = "View your Google Slides presentations\n\n`https://www.googleapis.com/auth/presentations.readonly`"]
    pub const PRESENTATIONS_READONLY: &str =
        "https://www.googleapis.com/auth/presentations.readonly";
    #[doc = "See, edit, create, and delete your spreadsheets in Google Drive\n\n`https://www.googleapis.com/auth/spreadsheets`"]
    pub const SPREADSHEETS: &str = "https://www.googleapis.com/auth/spreadsheets";
    #[doc = "View your Google Spreadsheets\n\n`https://www.googleapis.com/auth/spreadsheets.readonly`"]
    pub const SPREADSHEETS_READONLY: &str = "https://www.googleapis.com/auth/spreadsheets.readonly";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AffineTransform {
        #[doc = "The X coordinate scaling element."]
        #[serde(
            rename = "scaleX",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scale_x: ::std::option::Option<f64>,
        #[doc = "The Y coordinate scaling element."]
        #[serde(
            rename = "scaleY",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scale_y: ::std::option::Option<f64>,
        #[doc = "The X coordinate shearing element."]
        #[serde(
            rename = "shearX",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shear_x: ::std::option::Option<f64>,
        #[doc = "The Y coordinate shearing element."]
        #[serde(
            rename = "shearY",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shear_y: ::std::option::Option<f64>,
        #[doc = "The X coordinate translation element."]
        #[serde(
            rename = "translateX",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub translate_x: ::std::option::Option<f64>,
        #[doc = "The Y coordinate translation element."]
        #[serde(
            rename = "translateY",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub translate_y: ::std::option::Option<f64>,
        #[doc = "The units for translate elements."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<crate::schemas::AffineTransformUnit>,
    }
    impl ::google_field_selector::FieldSelector for AffineTransform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AffineTransform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AffineTransformUnit {
        #[doc = "An English Metric Unit (EMU) is defined as 1/360,000 of a centimeter\nand thus there are 914,400 EMUs per inch, and 12,700 EMUs per point."]
        Emu,
        #[doc = "A point, 1/72 of an inch."]
        Pt,
        #[doc = "The units are unknown."]
        UnitUnspecified,
    }
    impl AffineTransformUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                AffineTransformUnit::Emu => "EMU",
                AffineTransformUnit::Pt => "PT",
                AffineTransformUnit::UnitUnspecified => "UNIT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AffineTransformUnit {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AffineTransformUnit {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AffineTransformUnit, ()> {
            Ok(match s {
                "EMU" => AffineTransformUnit::Emu,
                "PT" => AffineTransformUnit::Pt,
                "UNIT_UNSPECIFIED" => AffineTransformUnit::UnitUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AffineTransformUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AffineTransformUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AffineTransformUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMU" => AffineTransformUnit::Emu,
                "PT" => AffineTransformUnit::Pt,
                "UNIT_UNSPECIFIED" => AffineTransformUnit::UnitUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AffineTransformUnit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AffineTransformUnit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AutoText {
        #[doc = "The rendered content of this auto text, if available."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The type of this auto text."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AutoTextType>,
        #[doc = "The styling applied to this auto text."]
        #[serde(
            rename = "style",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for AutoText {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AutoText {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AutoTextType {
        #[doc = "Type for autotext that represents the current slide number."]
        SlideNumber,
        #[doc = "An unspecified autotext type."]
        TypeUnspecified,
    }
    impl AutoTextType {
        pub fn as_str(self) -> &'static str {
            match self {
                AutoTextType::SlideNumber => "SLIDE_NUMBER",
                AutoTextType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AutoTextType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AutoTextType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AutoTextType, ()> {
            Ok(match s {
                "SLIDE_NUMBER" => AutoTextType::SlideNumber,
                "TYPE_UNSPECIFIED" => AutoTextType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AutoTextType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AutoTextType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AutoTextType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SLIDE_NUMBER" => AutoTextType::SlideNumber,
                "TYPE_UNSPECIFIED" => AutoTextType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AutoTextType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AutoTextType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchUpdatePresentationRequest {
        #[doc = "A list of updates to apply to the presentation."]
        #[serde(
            rename = "requests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requests: ::std::option::Option<Vec<crate::schemas::Request>>,
        #[doc = "Provides control over how write requests are executed."]
        #[serde(
            rename = "writeControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_control: ::std::option::Option<crate::schemas::WriteControl>,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdatePresentationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdatePresentationRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BatchUpdatePresentationResponse {
        #[doc = "The presentation the updates were applied to."]
        #[serde(
            rename = "presentationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub presentation_id: ::std::option::Option<String>,
        #[doc = "The reply of the updates.  This maps 1:1 with the updates, although\nreplies to some requests may be empty."]
        #[serde(
            rename = "replies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replies: ::std::option::Option<Vec<crate::schemas::Response>>,
        #[doc = "The updated write control after applying the request."]
        #[serde(
            rename = "writeControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_control: ::std::option::Option<crate::schemas::WriteControl>,
    }
    impl ::google_field_selector::FieldSelector for BatchUpdatePresentationResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdatePresentationResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Bullet {
        #[doc = "The paragraph specific text style applied to this bullet."]
        #[serde(
            rename = "bulletStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet_style: ::std::option::Option<crate::schemas::TextStyle>,
        #[doc = "The rendered bullet glyph for this paragraph."]
        #[serde(
            rename = "glyph",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub glyph: ::std::option::Option<String>,
        #[doc = "The ID of the list this paragraph belongs to."]
        #[serde(
            rename = "listId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_id: ::std::option::Option<String>,
        #[doc = "The nesting level of this paragraph in the list."]
        #[serde(
            rename = "nestingLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nesting_level: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Bullet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Bullet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColorScheme {
        #[doc = "The ThemeColorType and corresponding concrete color pairs."]
        #[serde(
            rename = "colors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub colors: ::std::option::Option<Vec<crate::schemas::ThemeColorPair>>,
    }
    impl ::google_field_selector::FieldSelector for ColorScheme {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ColorScheme {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColorStop {
        #[doc = "The alpha value of this color in the gradient band. Defaults to 1.0,\nfully opaque."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The color of the gradient stop."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::OpaqueColor>,
        #[doc = "The relative position of the color stop in the gradient band measured\nin percentage. The value should be in the interval [0.0, 1.0]."]
        #[serde(
            rename = "position",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for ColorStop {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ColorStop {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateImageRequest {
        #[doc = "The element properties for the image.\n\nWhen the aspect ratio of the provided size does not match the image aspect\nratio, the image is scaled and centered with respect to the size in order\nto maintain aspect ratio. The provided transform is applied after this\noperation.\n\nThe PageElementProperties.size property is\noptional. If you don't specify the size, the default size of the image is\nused.\n\nThe PageElementProperties.transform property is\noptional. If you don't specify a transform, the image will be placed at the\ntop left corner of the page."]
        #[serde(
            rename = "elementProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element_properties: ::std::option::Option<crate::schemas::PageElementProperties>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The image URL.\n\nThe image is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Images must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length. The URL itself is saved\nwith the image, and exposed via the Image.source_url field."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateImageRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateImageRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateImageResponse {
        #[doc = "The object ID of the created image."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateImageResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateImageResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateLineRequest {
        #[doc = "The category of the line to be created.\n\nThe exact line type created is\ndetermined based on the category and how it's routed to connect to other\npage elements.\n\nIf you specify both a `category` and a `line_category`, the `category`\ntakes precedence.\n\nIf you do not specify a value for `category`, but specify a value for\n`line_category`, then the specified `line_category` value is used.\n\nIf you do not specify either, then STRAIGHT is used."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<crate::schemas::CreateLineRequestCategory>,
        #[doc = "The element properties for the line."]
        #[serde(
            rename = "elementProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element_properties: ::std::option::Option<crate::schemas::PageElementProperties>,
        #[doc = "The category of the line to be created.\n\n<b>Deprecated</b>: use `category` instead.\n\nThe exact line type created is\ndetermined based on the category and how it's routed to connect to other\npage elements.\n\nIf you specify both a `category` and a `line_category`, the `category`\ntakes precedence."]
        #[serde(
            rename = "lineCategory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_category: ::std::option::Option<crate::schemas::CreateLineRequestLineCategory>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateLineRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateLineRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateLineRequestCategory {
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
    }
    impl CreateLineRequestCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateLineRequestCategory::Bent => "BENT",
                CreateLineRequestCategory::Curved => "CURVED",
                CreateLineRequestCategory::LineCategoryUnspecified => "LINE_CATEGORY_UNSPECIFIED",
                CreateLineRequestCategory::Straight => "STRAIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateLineRequestCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateLineRequestCategory {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreateLineRequestCategory, ()> {
            Ok(match s {
                "BENT" => CreateLineRequestCategory::Bent,
                "CURVED" => CreateLineRequestCategory::Curved,
                "LINE_CATEGORY_UNSPECIFIED" => CreateLineRequestCategory::LineCategoryUnspecified,
                "STRAIGHT" => CreateLineRequestCategory::Straight,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateLineRequestCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateLineRequestCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateLineRequestCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BENT" => CreateLineRequestCategory::Bent,
                "CURVED" => CreateLineRequestCategory::Curved,
                "LINE_CATEGORY_UNSPECIFIED" => CreateLineRequestCategory::LineCategoryUnspecified,
                "STRAIGHT" => CreateLineRequestCategory::Straight,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CreateLineRequestCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateLineRequestCategory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateLineRequestLineCategory {
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
        #[doc = "Straight connectors, including straight connector 1. The is the default\ncategory when one is not specified."]
        Straight,
    }
    impl CreateLineRequestLineCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateLineRequestLineCategory::Bent => "BENT",
                CreateLineRequestLineCategory::Curved => "CURVED",
                CreateLineRequestLineCategory::Straight => "STRAIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateLineRequestLineCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateLineRequestLineCategory {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreateLineRequestLineCategory, ()> {
            Ok(match s {
                "BENT" => CreateLineRequestLineCategory::Bent,
                "CURVED" => CreateLineRequestLineCategory::Curved,
                "STRAIGHT" => CreateLineRequestLineCategory::Straight,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateLineRequestLineCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateLineRequestLineCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateLineRequestLineCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BENT" => CreateLineRequestLineCategory::Bent,
                "CURVED" => CreateLineRequestLineCategory::Curved,
                "STRAIGHT" => CreateLineRequestLineCategory::Straight,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CreateLineRequestLineCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateLineRequestLineCategory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateLineResponse {
        #[doc = "The object ID of the created line."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateLineResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateLineResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateParagraphBulletsRequest {
        #[doc = "The kinds of bullet glyphs to be used. Defaults to the\n`BULLET_DISC_CIRCLE_SQUARE` preset."]
        #[serde(
            rename = "bulletPreset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet_preset:
            ::std::option::Option<crate::schemas::CreateParagraphBulletsRequestBulletPreset>,
        #[doc = "The optional table cell location if the text to be modified is in a table\ncell. If present, the object_id must refer to a table."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The object ID of the shape or table containing the text to add bullets to."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The range of text to apply the bullet presets to, based on TextElement indexes."]
        #[serde(
            rename = "textRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_range: ::std::option::Option<crate::schemas::Range>,
    }
    impl ::google_field_selector::FieldSelector for CreateParagraphBulletsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateParagraphBulletsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateParagraphBulletsRequestBulletPreset {
        #[doc = "A bulleted list with a `ARROW3D`, `CIRCLE` and `SQUARE` bullet glyph for\nthe first 3 list nesting levels."]
        BulletArrow3DCircleSquare,
        #[doc = "A bulleted list with a `ARROW`, `DIAMOND` and `DISC` bullet glyph for\nthe first 3 list nesting levels."]
        BulletArrowDiamondDisc,
        #[doc = "A bulleted list with `CHECKBOX` bullet glyphs for all list nesting levels."]
        BulletCheckbox,
        #[doc = "A bulleted list with a `DIAMOND`, `CIRCLE` and `SQUARE` bullet glyph\nfor the first 3 list nesting levels."]
        BulletDiamondCircleSquare,
        #[doc = "A bulleted list with a `DIAMONDX`, `ARROW3D` and `SQUARE` bullet glyph for\nthe first 3 list nesting levels."]
        BulletDiamondxArrow3DSquare,
        #[doc = "A bulleted list with a `DIAMONDX`, `HOLLOWDIAMOND` and `SQUARE` bullet\nglyph for the first 3 list nesting levels."]
        BulletDiamondxHollowdiamondSquare,
        #[doc = "A bulleted list with a `DISC`, `CIRCLE` and `SQUARE` bullet glyph for the\nfirst 3 list nesting levels."]
        BulletDiscCircleSquare,
        #[doc = "A bulleted list with a `LEFTTRIANGLE`, `DIAMOND` and `DISC` bullet glyph\nfor the first 3 list nesting levels."]
        BulletLefttriangleDiamondDisc,
        #[doc = "A bulleted list with a `STAR`, `CIRCLE` and `SQUARE` bullet glyph for\nthe first 3 list nesting levels."]
        BulletStarCircleSquare,
        #[doc = "A numbered list with `DIGIT`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by periods."]
        NumberedDigitAlphaRoman,
        #[doc = "A numbered list with `DIGIT`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by parenthesis."]
        NumberedDigitAlphaRomanParens,
        #[doc = "A numbered list with `DIGIT` numeric glyphs separated by periods, where\neach nesting level uses the previous nesting level's glyph as a prefix.\nFor example: '1.', '1.1.', '2.', '2.2.'."]
        NumberedDigitNested,
        #[doc = "A numbered list with `UPPERALPHA`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by periods."]
        NumberedUpperalphaAlphaRoman,
        #[doc = "A numbered list with `UPPERROMAN`, `UPPERALPHA` and `DIGIT` numeric glyphs\nfor the first 3 list nesting levels, followed by periods."]
        NumberedUpperromanUpperalphaDigit,
        #[doc = "A numbered list with `ZERODIGIT`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by periods."]
        NumberedZerodigitAlphaRoman,
    }
    impl CreateParagraphBulletsRequestBulletPreset {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateParagraphBulletsRequestBulletPreset::BulletArrow3DCircleSquare => {
                    "BULLET_ARROW3D_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletArrowDiamondDisc => {
                    "BULLET_ARROW_DIAMOND_DISC"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletCheckbox => "BULLET_CHECKBOX",
                CreateParagraphBulletsRequestBulletPreset::BulletDiamondCircleSquare => {
                    "BULLET_DIAMOND_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletDiamondxArrow3DSquare => {
                    "BULLET_DIAMONDX_ARROW3D_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletDiamondxHollowdiamondSquare => {
                    "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletDiscCircleSquare => {
                    "BULLET_DISC_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc => {
                    "BULLET_LEFTTRIANGLE_DIAMOND_DISC"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare => {
                    "BULLET_STAR_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRoman => {
                    "NUMBERED_DIGIT_ALPHA_ROMAN"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRomanParens => {
                    "NUMBERED_DIGIT_ALPHA_ROMAN_PARENS"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDigitNested => {
                    "NUMBERED_DIGIT_NESTED"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman => {
                    "NUMBERED_UPPERALPHA_ALPHA_ROMAN"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDigit => {
                    "NUMBERED_UPPERROMAN_UPPERALPHA_DIGIT"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedZerodigitAlphaRoman => {
                    "NUMBERED_ZERODIGIT_ALPHA_ROMAN"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateParagraphBulletsRequestBulletPreset {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateParagraphBulletsRequestBulletPreset {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CreateParagraphBulletsRequestBulletPreset, ()> {
            Ok(match s {
                "BULLET_ARROW3D_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletArrow3DCircleSquare
                }
                "BULLET_ARROW_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletArrowDiamondDisc
                }
                "BULLET_CHECKBOX" => CreateParagraphBulletsRequestBulletPreset::BulletCheckbox,
                "BULLET_DIAMOND_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondCircleSquare
                }
                "BULLET_DIAMONDX_ARROW3D_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondxArrow3DSquare
                }
                "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondxHollowdiamondSquare
                }
                "BULLET_DISC_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiscCircleSquare
                }
                "BULLET_LEFTTRIANGLE_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc
                }
                "BULLET_STAR_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare
                }
                "NUMBERED_DIGIT_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRoman
                }
                "NUMBERED_DIGIT_ALPHA_ROMAN_PARENS" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRomanParens
                }
                "NUMBERED_DIGIT_NESTED" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitNested
                }
                "NUMBERED_UPPERALPHA_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman
                }
                "NUMBERED_UPPERROMAN_UPPERALPHA_DIGIT" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDigit
                }
                "NUMBERED_ZERODIGIT_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedZerodigitAlphaRoman
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateParagraphBulletsRequestBulletPreset {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateParagraphBulletsRequestBulletPreset {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateParagraphBulletsRequestBulletPreset {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BULLET_ARROW3D_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletArrow3DCircleSquare
                }
                "BULLET_ARROW_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletArrowDiamondDisc
                }
                "BULLET_CHECKBOX" => CreateParagraphBulletsRequestBulletPreset::BulletCheckbox,
                "BULLET_DIAMOND_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondCircleSquare
                }
                "BULLET_DIAMONDX_ARROW3D_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondxArrow3DSquare
                }
                "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondxHollowdiamondSquare
                }
                "BULLET_DISC_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiscCircleSquare
                }
                "BULLET_LEFTTRIANGLE_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc
                }
                "BULLET_STAR_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare
                }
                "NUMBERED_DIGIT_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRoman
                }
                "NUMBERED_DIGIT_ALPHA_ROMAN_PARENS" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRomanParens
                }
                "NUMBERED_DIGIT_NESTED" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitNested
                }
                "NUMBERED_UPPERALPHA_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman
                }
                "NUMBERED_UPPERROMAN_UPPERALPHA_DIGIT" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDigit
                }
                "NUMBERED_ZERODIGIT_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedZerodigitAlphaRoman
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
    impl ::google_field_selector::FieldSelector for CreateParagraphBulletsRequestBulletPreset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateParagraphBulletsRequestBulletPreset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateShapeRequest {
        #[doc = "The element properties for the shape."]
        #[serde(
            rename = "elementProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element_properties: ::std::option::Option<crate::schemas::PageElementProperties>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\nIf empty, a unique identifier will be generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The shape type."]
        #[serde(
            rename = "shapeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shape_type: ::std::option::Option<crate::schemas::CreateShapeRequestShapeType>,
    }
    impl ::google_field_selector::FieldSelector for CreateShapeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateShapeRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateShapeRequestShapeType {
        #[doc = "Curved arc shape. Corresponds to ECMA-376 ST_ShapeType 'arc'"]
        Arc,
        #[doc = "East arrow shape."]
        ArrowEast,
        #[doc = "North arrow shape."]
        ArrowNorth,
        #[doc = "Northeast arrow shape."]
        ArrowNorthEast,
        #[doc = "Bent arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentArrow'"]
        BentArrow,
        #[doc = "Bent up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentUpArrow'"]
        BentUpArrow,
        #[doc = "Bevel shape. Corresponds to ECMA-376 ST_ShapeType 'bevel'"]
        Bevel,
        #[doc = "Block arc shape. Corresponds to ECMA-376 ST_ShapeType 'blockArc'"]
        BlockArc,
        #[doc = "Brace pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracePair'"]
        BracePair,
        #[doc = "Bracket pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracketPair'"]
        BracketPair,
        #[doc = "Can shape. Corresponds to ECMA-376 ST_ShapeType 'can'"]
        Can,
        #[doc = "Chevron shape. Corresponds to ECMA-376 ST_ShapeType 'chevron'"]
        Chevron,
        #[doc = "Chord shape. Corresponds to ECMA-376 ST_ShapeType 'chord'"]
        Chord,
        #[doc = "Cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloud'"]
        Cloud,
        #[doc = "Callout cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloudCallout'"]
        CloudCallout,
        #[doc = "Corner shape. Corresponds to ECMA-376 ST_ShapeType 'corner'"]
        Corner,
        #[doc = "Cube shape. Corresponds to ECMA-376 ST_ShapeType 'cube'"]
        Cube,
        #[doc = "Curved down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedDownArrow'"]
        CurvedDownArrow,
        #[doc = "Curved left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedLeftArrow'"]
        CurvedLeftArrow,
        #[doc = "Curved right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedRightArrow'"]
        CurvedRightArrow,
        #[doc = "Curved up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedUpArrow'"]
        CurvedUpArrow,
        #[doc = "Custom shape."]
        Custom,
        #[doc = "Decagon shape. Corresponds to ECMA-376 ST_ShapeType 'decagon'"]
        Decagon,
        #[doc = "Diagonal stripe shape. Corresponds to ECMA-376 ST_ShapeType 'diagStripe'"]
        DiagonalStripe,
        #[doc = "Diamond shape. Corresponds to ECMA-376 ST_ShapeType 'diamond'"]
        Diamond,
        #[doc = "Dodecagon shape. Corresponds to ECMA-376 ST_ShapeType 'dodecagon'"]
        Dodecagon,
        #[doc = "Donut shape. Corresponds to ECMA-376 ST_ShapeType 'donut'"]
        Donut,
        #[doc = "Double wave shape. Corresponds to ECMA-376 ST_ShapeType 'doubleWave'"]
        DoubleWave,
        #[doc = "Down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrow'"]
        DownArrow,
        #[doc = "Callout down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'downArrowCallout'"]
        DownArrowCallout,
        #[doc = "Ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'ellipse'"]
        Ellipse,
        #[doc = "Ellipse ribbon shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon'"]
        EllipseRibbon,
        #[doc = "Ellipse ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon2'"]
        EllipseRibbon2,
        #[doc = "Alternate process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartAlternateProcess'"]
        FlowChartAlternateProcess,
        #[doc = "Collate flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartCollate'"]
        FlowChartCollate,
        #[doc = "Connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartConnector'"]
        FlowChartConnector,
        #[doc = "Decision flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDecision'"]
        FlowChartDecision,
        #[doc = "Delay flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDelay'"]
        FlowChartDelay,
        #[doc = "Display flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDisplay'"]
        FlowChartDisplay,
        #[doc = "Document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDocument'"]
        FlowChartDocument,
        #[doc = "Extract flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartExtract'"]
        FlowChartExtract,
        #[doc = "Input output flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInputOutput'"]
        FlowChartInputOutput,
        #[doc = "Internal storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInternalStorage'"]
        FlowChartInternalStorage,
        #[doc = "Magnetic disk flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDisk'"]
        FlowChartMagneticDisk,
        #[doc = "Magnetic drum flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDrum'"]
        FlowChartMagneticDrum,
        #[doc = "Magnetic tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticTape'"]
        FlowChartMagneticTape,
        #[doc = "Manual input flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualInput'"]
        FlowChartManualInput,
        #[doc = "Manual operation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualOperation'"]
        FlowChartManualOperation,
        #[doc = "Merge flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMerge'"]
        FlowChartMerge,
        #[doc = "Multi-document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMultidocument'"]
        FlowChartMultidocument,
        #[doc = "Offline storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOfflineStorage'"]
        FlowChartOfflineStorage,
        #[doc = "Off-page connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOffpageConnector'"]
        FlowChartOffpageConnector,
        #[doc = "Online storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOnlineStorage'"]
        FlowChartOnlineStorage,
        #[doc = "Or flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOr'"]
        FlowChartOr,
        #[doc = "Predefined process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPredefinedProcess'"]
        FlowChartPredefinedProcess,
        #[doc = "Preparation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPreparation'"]
        FlowChartPreparation,
        #[doc = "Process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartProcess'"]
        FlowChartProcess,
        #[doc = "Punched card flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedCard'"]
        FlowChartPunchedCard,
        #[doc = "Punched tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedTape'"]
        FlowChartPunchedTape,
        #[doc = "Sort flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSort'"]
        FlowChartSort,
        #[doc = "Summing junction flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartSummingJunction'"]
        FlowChartSummingJunction,
        #[doc = "Terminator flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartTerminator'"]
        FlowChartTerminator,
        #[doc = "Folded corner shape. Corresponds to ECMA-376 ST_ShapeType 'foldedCorner'"]
        FoldedCorner,
        #[doc = "Frame shape. Corresponds to ECMA-376 ST_ShapeType 'frame'"]
        Frame,
        #[doc = "Half frame shape. Corresponds to ECMA-376 ST_ShapeType 'halfFrame'"]
        HalfFrame,
        #[doc = "Heart shape. Corresponds to ECMA-376 ST_ShapeType 'heart'"]
        Heart,
        #[doc = "Heptagon shape. Corresponds to ECMA-376 ST_ShapeType 'heptagon'"]
        Heptagon,
        #[doc = "Hexagon shape. Corresponds to ECMA-376 ST_ShapeType 'hexagon'"]
        Hexagon,
        #[doc = "Home plate shape. Corresponds to ECMA-376 ST_ShapeType 'homePlate'"]
        HomePlate,
        #[doc = "Horizontal scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'horizontalScroll'"]
        HorizontalScroll,
        #[doc = "Irregular seal 1 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal1'"]
        IrregularSeal1,
        #[doc = "Irregular seal 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal2'"]
        IrregularSeal2,
        #[doc = "Left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrow'"]
        LeftArrow,
        #[doc = "Callout left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftArrowCallout'"]
        LeftArrowCallout,
        #[doc = "Left brace shape. Corresponds to ECMA-376 ST_ShapeType 'leftBrace'"]
        LeftBrace,
        #[doc = "Left bracket shape. Corresponds to ECMA-376 ST_ShapeType 'leftBracket'"]
        LeftBracket,
        #[doc = "Left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrow'"]
        LeftRightArrow,
        #[doc = "Callout left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrowCallout'"]
        LeftRightArrowCallout,
        #[doc = "Left right up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightUpArrow'"]
        LeftRightUpArrow,
        #[doc = "Left up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftUpArrow'"]
        LeftUpArrow,
        #[doc = "Lightning bolt shape. Corresponds to ECMA-376 ST_ShapeType\n'lightningBolt'"]
        LightningBolt,
        #[doc = "Divide math shape. Corresponds to ECMA-376 ST_ShapeType 'mathDivide'"]
        MathDivide,
        #[doc = "Equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathEqual'"]
        MathEqual,
        #[doc = "Minus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMinus'"]
        MathMinus,
        #[doc = "Multiply math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMultiply'"]
        MathMultiply,
        #[doc = "Not equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathNotEqual'"]
        MathNotEqual,
        #[doc = "Plus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathPlus'"]
        MathPlus,
        #[doc = "Moon shape. Corresponds to ECMA-376 ST_ShapeType 'moon'"]
        Moon,
        #[doc = "No smoking shape. Corresponds to ECMA-376 ST_ShapeType 'noSmoking'"]
        NoSmoking,
        #[doc = "Notched right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'notchedRightArrow'"]
        NotchedRightArrow,
        #[doc = "Octagon shape. Corresponds to ECMA-376 ST_ShapeType 'octagon'"]
        Octagon,
        #[doc = "Parallelogram shape. Corresponds to ECMA-376 ST_ShapeType 'parallelogram'"]
        Parallelogram,
        #[doc = "Pentagon shape. Corresponds to ECMA-376 ST_ShapeType 'pentagon'"]
        Pentagon,
        #[doc = "Pie shape. Corresponds to ECMA-376 ST_ShapeType 'pie'"]
        Pie,
        #[doc = "Plaque shape. Corresponds to ECMA-376 ST_ShapeType 'plaque'"]
        Plaque,
        #[doc = "Plus shape. Corresponds to ECMA-376 ST_ShapeType 'plus'"]
        Plus,
        #[doc = "Quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrow'"]
        QuadArrow,
        #[doc = "Callout quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'quadArrowCallout'"]
        QuadArrowCallout,
        #[doc = "Rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'rect'."]
        Rectangle,
        #[doc = "Ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon'"]
        Ribbon,
        #[doc = "Ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon2'"]
        Ribbon2,
        #[doc = "Right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrow'"]
        RightArrow,
        #[doc = "Callout right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'rightArrowCallout'"]
        RightArrowCallout,
        #[doc = "Right brace shape. Corresponds to ECMA-376 ST_ShapeType 'rightBrace'"]
        RightBrace,
        #[doc = "Right bracket shape. Corresponds to ECMA-376 ST_ShapeType 'rightBracket'"]
        RightBracket,
        #[doc = "Right triangle shape. Corresponds to ECMA-376 ST_ShapeType 'rtTriangle'"]
        RightTriangle,
        #[doc = "One round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'round1Rect'"]
        Round1Rectangle,
        #[doc = "Two diagonal round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2DiagRect'"]
        Round2DiagonalRectangle,
        #[doc = "Two same-side round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2SameRect'"]
        Round2SameRectangle,
        #[doc = "Round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'roundRect'"]
        RoundRectangle,
        #[doc = "Smiley face shape. Corresponds to ECMA-376 ST_ShapeType 'smileyFace'"]
        SmileyFace,
        #[doc = "One snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'snip1Rect'"]
        Snip1Rectangle,
        #[doc = "Two diagonal snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2DiagRect'"]
        Snip2DiagonalRectangle,
        #[doc = "Two same-side snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2SameRect'"]
        Snip2SameRectangle,
        #[doc = "One snip one round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snipRoundRect'"]
        SnipRoundRectangle,
        #[doc = "Speech shape."]
        Speech,
        #[doc = "Ten pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star10'"]
        Star10,
        #[doc = "Twelve pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star12'"]
        Star12,
        #[doc = "Sixteen pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star16'"]
        Star16,
        #[doc = "Twenty four pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star24'"]
        Star24,
        #[doc = "Thirty two pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star32'"]
        Star32,
        #[doc = "Four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star4'"]
        Star4,
        #[doc = "Five pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star5'"]
        Star5,
        #[doc = "Six pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star6'"]
        Star6,
        #[doc = "Seven pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star7'"]
        Star7,
        #[doc = "Eight pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star8'"]
        Star8,
        #[doc = "Star burst shape."]
        Starburst,
        #[doc = "Striped right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'stripedRightArrow'"]
        StripedRightArrow,
        #[doc = "Sun shape. Corresponds to ECMA-376 ST_ShapeType 'sun'"]
        Sun,
        #[doc = "Teardrop shape. Corresponds to ECMA-376 ST_ShapeType 'teardrop'"]
        Teardrop,
        #[doc = "Text box shape."]
        TextBox,
        #[doc = "Trapezoid shape. Corresponds to ECMA-376 ST_ShapeType 'trapezoid'"]
        Trapezoid,
        #[doc = "Triangle shape. Corresponds to ECMA-376 ST_ShapeType 'triangle'"]
        Triangle,
        #[doc = "The shape type that is not predefined."]
        TypeUnspecified,
        #[doc = "Up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrow'"]
        UpArrow,
        #[doc = "Callout up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'upArrowCallout'"]
        UpArrowCallout,
        #[doc = "Up down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upDownArrow'"]
        UpDownArrow,
        #[doc = "U-turn arrow shape. Corresponds to ECMA-376 ST_ShapeType 'uturnArrow'"]
        UturnArrow,
        #[doc = "Vertical scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'verticalScroll'"]
        VerticalScroll,
        #[doc = "Wave shape. Corresponds to ECMA-376 ST_ShapeType 'wave'"]
        Wave,
        #[doc = "Callout wedge ellipse shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeEllipseCallout'"]
        WedgeEllipseCallout,
        #[doc = "Callout wedge rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRectCallout'"]
        WedgeRectangleCallout,
        #[doc = "Callout wedge round rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRoundRectCallout'"]
        WedgeRoundRectangleCallout,
    }
    impl CreateShapeRequestShapeType {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateShapeRequestShapeType::Arc => "ARC",
                CreateShapeRequestShapeType::ArrowEast => "ARROW_EAST",
                CreateShapeRequestShapeType::ArrowNorth => "ARROW_NORTH",
                CreateShapeRequestShapeType::ArrowNorthEast => "ARROW_NORTH_EAST",
                CreateShapeRequestShapeType::BentArrow => "BENT_ARROW",
                CreateShapeRequestShapeType::BentUpArrow => "BENT_UP_ARROW",
                CreateShapeRequestShapeType::Bevel => "BEVEL",
                CreateShapeRequestShapeType::BlockArc => "BLOCK_ARC",
                CreateShapeRequestShapeType::BracePair => "BRACE_PAIR",
                CreateShapeRequestShapeType::BracketPair => "BRACKET_PAIR",
                CreateShapeRequestShapeType::Can => "CAN",
                CreateShapeRequestShapeType::Chevron => "CHEVRON",
                CreateShapeRequestShapeType::Chord => "CHORD",
                CreateShapeRequestShapeType::Cloud => "CLOUD",
                CreateShapeRequestShapeType::CloudCallout => "CLOUD_CALLOUT",
                CreateShapeRequestShapeType::Corner => "CORNER",
                CreateShapeRequestShapeType::Cube => "CUBE",
                CreateShapeRequestShapeType::CurvedDownArrow => "CURVED_DOWN_ARROW",
                CreateShapeRequestShapeType::CurvedLeftArrow => "CURVED_LEFT_ARROW",
                CreateShapeRequestShapeType::CurvedRightArrow => "CURVED_RIGHT_ARROW",
                CreateShapeRequestShapeType::CurvedUpArrow => "CURVED_UP_ARROW",
                CreateShapeRequestShapeType::Custom => "CUSTOM",
                CreateShapeRequestShapeType::Decagon => "DECAGON",
                CreateShapeRequestShapeType::DiagonalStripe => "DIAGONAL_STRIPE",
                CreateShapeRequestShapeType::Diamond => "DIAMOND",
                CreateShapeRequestShapeType::Dodecagon => "DODECAGON",
                CreateShapeRequestShapeType::Donut => "DONUT",
                CreateShapeRequestShapeType::DoubleWave => "DOUBLE_WAVE",
                CreateShapeRequestShapeType::DownArrow => "DOWN_ARROW",
                CreateShapeRequestShapeType::DownArrowCallout => "DOWN_ARROW_CALLOUT",
                CreateShapeRequestShapeType::Ellipse => "ELLIPSE",
                CreateShapeRequestShapeType::EllipseRibbon => "ELLIPSE_RIBBON",
                CreateShapeRequestShapeType::EllipseRibbon2 => "ELLIPSE_RIBBON_2",
                CreateShapeRequestShapeType::FlowChartAlternateProcess => {
                    "FLOW_CHART_ALTERNATE_PROCESS"
                }
                CreateShapeRequestShapeType::FlowChartCollate => "FLOW_CHART_COLLATE",
                CreateShapeRequestShapeType::FlowChartConnector => "FLOW_CHART_CONNECTOR",
                CreateShapeRequestShapeType::FlowChartDecision => "FLOW_CHART_DECISION",
                CreateShapeRequestShapeType::FlowChartDelay => "FLOW_CHART_DELAY",
                CreateShapeRequestShapeType::FlowChartDisplay => "FLOW_CHART_DISPLAY",
                CreateShapeRequestShapeType::FlowChartDocument => "FLOW_CHART_DOCUMENT",
                CreateShapeRequestShapeType::FlowChartExtract => "FLOW_CHART_EXTRACT",
                CreateShapeRequestShapeType::FlowChartInputOutput => "FLOW_CHART_INPUT_OUTPUT",
                CreateShapeRequestShapeType::FlowChartInternalStorage => {
                    "FLOW_CHART_INTERNAL_STORAGE"
                }
                CreateShapeRequestShapeType::FlowChartMagneticDisk => "FLOW_CHART_MAGNETIC_DISK",
                CreateShapeRequestShapeType::FlowChartMagneticDrum => "FLOW_CHART_MAGNETIC_DRUM",
                CreateShapeRequestShapeType::FlowChartMagneticTape => "FLOW_CHART_MAGNETIC_TAPE",
                CreateShapeRequestShapeType::FlowChartManualInput => "FLOW_CHART_MANUAL_INPUT",
                CreateShapeRequestShapeType::FlowChartManualOperation => {
                    "FLOW_CHART_MANUAL_OPERATION"
                }
                CreateShapeRequestShapeType::FlowChartMerge => "FLOW_CHART_MERGE",
                CreateShapeRequestShapeType::FlowChartMultidocument => "FLOW_CHART_MULTIDOCUMENT",
                CreateShapeRequestShapeType::FlowChartOfflineStorage => {
                    "FLOW_CHART_OFFLINE_STORAGE"
                }
                CreateShapeRequestShapeType::FlowChartOffpageConnector => {
                    "FLOW_CHART_OFFPAGE_CONNECTOR"
                }
                CreateShapeRequestShapeType::FlowChartOnlineStorage => "FLOW_CHART_ONLINE_STORAGE",
                CreateShapeRequestShapeType::FlowChartOr => "FLOW_CHART_OR",
                CreateShapeRequestShapeType::FlowChartPredefinedProcess => {
                    "FLOW_CHART_PREDEFINED_PROCESS"
                }
                CreateShapeRequestShapeType::FlowChartPreparation => "FLOW_CHART_PREPARATION",
                CreateShapeRequestShapeType::FlowChartProcess => "FLOW_CHART_PROCESS",
                CreateShapeRequestShapeType::FlowChartPunchedCard => "FLOW_CHART_PUNCHED_CARD",
                CreateShapeRequestShapeType::FlowChartPunchedTape => "FLOW_CHART_PUNCHED_TAPE",
                CreateShapeRequestShapeType::FlowChartSort => "FLOW_CHART_SORT",
                CreateShapeRequestShapeType::FlowChartSummingJunction => {
                    "FLOW_CHART_SUMMING_JUNCTION"
                }
                CreateShapeRequestShapeType::FlowChartTerminator => "FLOW_CHART_TERMINATOR",
                CreateShapeRequestShapeType::FoldedCorner => "FOLDED_CORNER",
                CreateShapeRequestShapeType::Frame => "FRAME",
                CreateShapeRequestShapeType::HalfFrame => "HALF_FRAME",
                CreateShapeRequestShapeType::Heart => "HEART",
                CreateShapeRequestShapeType::Heptagon => "HEPTAGON",
                CreateShapeRequestShapeType::Hexagon => "HEXAGON",
                CreateShapeRequestShapeType::HomePlate => "HOME_PLATE",
                CreateShapeRequestShapeType::HorizontalScroll => "HORIZONTAL_SCROLL",
                CreateShapeRequestShapeType::IrregularSeal1 => "IRREGULAR_SEAL_1",
                CreateShapeRequestShapeType::IrregularSeal2 => "IRREGULAR_SEAL_2",
                CreateShapeRequestShapeType::LeftArrow => "LEFT_ARROW",
                CreateShapeRequestShapeType::LeftArrowCallout => "LEFT_ARROW_CALLOUT",
                CreateShapeRequestShapeType::LeftBrace => "LEFT_BRACE",
                CreateShapeRequestShapeType::LeftBracket => "LEFT_BRACKET",
                CreateShapeRequestShapeType::LeftRightArrow => "LEFT_RIGHT_ARROW",
                CreateShapeRequestShapeType::LeftRightArrowCallout => "LEFT_RIGHT_ARROW_CALLOUT",
                CreateShapeRequestShapeType::LeftRightUpArrow => "LEFT_RIGHT_UP_ARROW",
                CreateShapeRequestShapeType::LeftUpArrow => "LEFT_UP_ARROW",
                CreateShapeRequestShapeType::LightningBolt => "LIGHTNING_BOLT",
                CreateShapeRequestShapeType::MathDivide => "MATH_DIVIDE",
                CreateShapeRequestShapeType::MathEqual => "MATH_EQUAL",
                CreateShapeRequestShapeType::MathMinus => "MATH_MINUS",
                CreateShapeRequestShapeType::MathMultiply => "MATH_MULTIPLY",
                CreateShapeRequestShapeType::MathNotEqual => "MATH_NOT_EQUAL",
                CreateShapeRequestShapeType::MathPlus => "MATH_PLUS",
                CreateShapeRequestShapeType::Moon => "MOON",
                CreateShapeRequestShapeType::NoSmoking => "NO_SMOKING",
                CreateShapeRequestShapeType::NotchedRightArrow => "NOTCHED_RIGHT_ARROW",
                CreateShapeRequestShapeType::Octagon => "OCTAGON",
                CreateShapeRequestShapeType::Parallelogram => "PARALLELOGRAM",
                CreateShapeRequestShapeType::Pentagon => "PENTAGON",
                CreateShapeRequestShapeType::Pie => "PIE",
                CreateShapeRequestShapeType::Plaque => "PLAQUE",
                CreateShapeRequestShapeType::Plus => "PLUS",
                CreateShapeRequestShapeType::QuadArrow => "QUAD_ARROW",
                CreateShapeRequestShapeType::QuadArrowCallout => "QUAD_ARROW_CALLOUT",
                CreateShapeRequestShapeType::Rectangle => "RECTANGLE",
                CreateShapeRequestShapeType::Ribbon => "RIBBON",
                CreateShapeRequestShapeType::Ribbon2 => "RIBBON_2",
                CreateShapeRequestShapeType::RightArrow => "RIGHT_ARROW",
                CreateShapeRequestShapeType::RightArrowCallout => "RIGHT_ARROW_CALLOUT",
                CreateShapeRequestShapeType::RightBrace => "RIGHT_BRACE",
                CreateShapeRequestShapeType::RightBracket => "RIGHT_BRACKET",
                CreateShapeRequestShapeType::RightTriangle => "RIGHT_TRIANGLE",
                CreateShapeRequestShapeType::Round1Rectangle => "ROUND_1_RECTANGLE",
                CreateShapeRequestShapeType::Round2DiagonalRectangle => {
                    "ROUND_2_DIAGONAL_RECTANGLE"
                }
                CreateShapeRequestShapeType::Round2SameRectangle => "ROUND_2_SAME_RECTANGLE",
                CreateShapeRequestShapeType::RoundRectangle => "ROUND_RECTANGLE",
                CreateShapeRequestShapeType::SmileyFace => "SMILEY_FACE",
                CreateShapeRequestShapeType::Snip1Rectangle => "SNIP_1_RECTANGLE",
                CreateShapeRequestShapeType::Snip2DiagonalRectangle => "SNIP_2_DIAGONAL_RECTANGLE",
                CreateShapeRequestShapeType::Snip2SameRectangle => "SNIP_2_SAME_RECTANGLE",
                CreateShapeRequestShapeType::SnipRoundRectangle => "SNIP_ROUND_RECTANGLE",
                CreateShapeRequestShapeType::Speech => "SPEECH",
                CreateShapeRequestShapeType::Star10 => "STAR_10",
                CreateShapeRequestShapeType::Star12 => "STAR_12",
                CreateShapeRequestShapeType::Star16 => "STAR_16",
                CreateShapeRequestShapeType::Star24 => "STAR_24",
                CreateShapeRequestShapeType::Star32 => "STAR_32",
                CreateShapeRequestShapeType::Star4 => "STAR_4",
                CreateShapeRequestShapeType::Star5 => "STAR_5",
                CreateShapeRequestShapeType::Star6 => "STAR_6",
                CreateShapeRequestShapeType::Star7 => "STAR_7",
                CreateShapeRequestShapeType::Star8 => "STAR_8",
                CreateShapeRequestShapeType::Starburst => "STARBURST",
                CreateShapeRequestShapeType::StripedRightArrow => "STRIPED_RIGHT_ARROW",
                CreateShapeRequestShapeType::Sun => "SUN",
                CreateShapeRequestShapeType::Teardrop => "TEARDROP",
                CreateShapeRequestShapeType::TextBox => "TEXT_BOX",
                CreateShapeRequestShapeType::Trapezoid => "TRAPEZOID",
                CreateShapeRequestShapeType::Triangle => "TRIANGLE",
                CreateShapeRequestShapeType::TypeUnspecified => "TYPE_UNSPECIFIED",
                CreateShapeRequestShapeType::UpArrow => "UP_ARROW",
                CreateShapeRequestShapeType::UpArrowCallout => "UP_ARROW_CALLOUT",
                CreateShapeRequestShapeType::UpDownArrow => "UP_DOWN_ARROW",
                CreateShapeRequestShapeType::UturnArrow => "UTURN_ARROW",
                CreateShapeRequestShapeType::VerticalScroll => "VERTICAL_SCROLL",
                CreateShapeRequestShapeType::Wave => "WAVE",
                CreateShapeRequestShapeType::WedgeEllipseCallout => "WEDGE_ELLIPSE_CALLOUT",
                CreateShapeRequestShapeType::WedgeRectangleCallout => "WEDGE_RECTANGLE_CALLOUT",
                CreateShapeRequestShapeType::WedgeRoundRectangleCallout => {
                    "WEDGE_ROUND_RECTANGLE_CALLOUT"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateShapeRequestShapeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateShapeRequestShapeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreateShapeRequestShapeType, ()> {
            Ok(match s {
                "ARC" => CreateShapeRequestShapeType::Arc,
                "ARROW_EAST" => CreateShapeRequestShapeType::ArrowEast,
                "ARROW_NORTH" => CreateShapeRequestShapeType::ArrowNorth,
                "ARROW_NORTH_EAST" => CreateShapeRequestShapeType::ArrowNorthEast,
                "BENT_ARROW" => CreateShapeRequestShapeType::BentArrow,
                "BENT_UP_ARROW" => CreateShapeRequestShapeType::BentUpArrow,
                "BEVEL" => CreateShapeRequestShapeType::Bevel,
                "BLOCK_ARC" => CreateShapeRequestShapeType::BlockArc,
                "BRACE_PAIR" => CreateShapeRequestShapeType::BracePair,
                "BRACKET_PAIR" => CreateShapeRequestShapeType::BracketPair,
                "CAN" => CreateShapeRequestShapeType::Can,
                "CHEVRON" => CreateShapeRequestShapeType::Chevron,
                "CHORD" => CreateShapeRequestShapeType::Chord,
                "CLOUD" => CreateShapeRequestShapeType::Cloud,
                "CLOUD_CALLOUT" => CreateShapeRequestShapeType::CloudCallout,
                "CORNER" => CreateShapeRequestShapeType::Corner,
                "CUBE" => CreateShapeRequestShapeType::Cube,
                "CURVED_DOWN_ARROW" => CreateShapeRequestShapeType::CurvedDownArrow,
                "CURVED_LEFT_ARROW" => CreateShapeRequestShapeType::CurvedLeftArrow,
                "CURVED_RIGHT_ARROW" => CreateShapeRequestShapeType::CurvedRightArrow,
                "CURVED_UP_ARROW" => CreateShapeRequestShapeType::CurvedUpArrow,
                "CUSTOM" => CreateShapeRequestShapeType::Custom,
                "DECAGON" => CreateShapeRequestShapeType::Decagon,
                "DIAGONAL_STRIPE" => CreateShapeRequestShapeType::DiagonalStripe,
                "DIAMOND" => CreateShapeRequestShapeType::Diamond,
                "DODECAGON" => CreateShapeRequestShapeType::Dodecagon,
                "DONUT" => CreateShapeRequestShapeType::Donut,
                "DOUBLE_WAVE" => CreateShapeRequestShapeType::DoubleWave,
                "DOWN_ARROW" => CreateShapeRequestShapeType::DownArrow,
                "DOWN_ARROW_CALLOUT" => CreateShapeRequestShapeType::DownArrowCallout,
                "ELLIPSE" => CreateShapeRequestShapeType::Ellipse,
                "ELLIPSE_RIBBON" => CreateShapeRequestShapeType::EllipseRibbon,
                "ELLIPSE_RIBBON_2" => CreateShapeRequestShapeType::EllipseRibbon2,
                "FLOW_CHART_ALTERNATE_PROCESS" => {
                    CreateShapeRequestShapeType::FlowChartAlternateProcess
                }
                "FLOW_CHART_COLLATE" => CreateShapeRequestShapeType::FlowChartCollate,
                "FLOW_CHART_CONNECTOR" => CreateShapeRequestShapeType::FlowChartConnector,
                "FLOW_CHART_DECISION" => CreateShapeRequestShapeType::FlowChartDecision,
                "FLOW_CHART_DELAY" => CreateShapeRequestShapeType::FlowChartDelay,
                "FLOW_CHART_DISPLAY" => CreateShapeRequestShapeType::FlowChartDisplay,
                "FLOW_CHART_DOCUMENT" => CreateShapeRequestShapeType::FlowChartDocument,
                "FLOW_CHART_EXTRACT" => CreateShapeRequestShapeType::FlowChartExtract,
                "FLOW_CHART_INPUT_OUTPUT" => CreateShapeRequestShapeType::FlowChartInputOutput,
                "FLOW_CHART_INTERNAL_STORAGE" => {
                    CreateShapeRequestShapeType::FlowChartInternalStorage
                }
                "FLOW_CHART_MAGNETIC_DISK" => CreateShapeRequestShapeType::FlowChartMagneticDisk,
                "FLOW_CHART_MAGNETIC_DRUM" => CreateShapeRequestShapeType::FlowChartMagneticDrum,
                "FLOW_CHART_MAGNETIC_TAPE" => CreateShapeRequestShapeType::FlowChartMagneticTape,
                "FLOW_CHART_MANUAL_INPUT" => CreateShapeRequestShapeType::FlowChartManualInput,
                "FLOW_CHART_MANUAL_OPERATION" => {
                    CreateShapeRequestShapeType::FlowChartManualOperation
                }
                "FLOW_CHART_MERGE" => CreateShapeRequestShapeType::FlowChartMerge,
                "FLOW_CHART_MULTIDOCUMENT" => CreateShapeRequestShapeType::FlowChartMultidocument,
                "FLOW_CHART_OFFLINE_STORAGE" => {
                    CreateShapeRequestShapeType::FlowChartOfflineStorage
                }
                "FLOW_CHART_OFFPAGE_CONNECTOR" => {
                    CreateShapeRequestShapeType::FlowChartOffpageConnector
                }
                "FLOW_CHART_ONLINE_STORAGE" => CreateShapeRequestShapeType::FlowChartOnlineStorage,
                "FLOW_CHART_OR" => CreateShapeRequestShapeType::FlowChartOr,
                "FLOW_CHART_PREDEFINED_PROCESS" => {
                    CreateShapeRequestShapeType::FlowChartPredefinedProcess
                }
                "FLOW_CHART_PREPARATION" => CreateShapeRequestShapeType::FlowChartPreparation,
                "FLOW_CHART_PROCESS" => CreateShapeRequestShapeType::FlowChartProcess,
                "FLOW_CHART_PUNCHED_CARD" => CreateShapeRequestShapeType::FlowChartPunchedCard,
                "FLOW_CHART_PUNCHED_TAPE" => CreateShapeRequestShapeType::FlowChartPunchedTape,
                "FLOW_CHART_SORT" => CreateShapeRequestShapeType::FlowChartSort,
                "FLOW_CHART_SUMMING_JUNCTION" => {
                    CreateShapeRequestShapeType::FlowChartSummingJunction
                }
                "FLOW_CHART_TERMINATOR" => CreateShapeRequestShapeType::FlowChartTerminator,
                "FOLDED_CORNER" => CreateShapeRequestShapeType::FoldedCorner,
                "FRAME" => CreateShapeRequestShapeType::Frame,
                "HALF_FRAME" => CreateShapeRequestShapeType::HalfFrame,
                "HEART" => CreateShapeRequestShapeType::Heart,
                "HEPTAGON" => CreateShapeRequestShapeType::Heptagon,
                "HEXAGON" => CreateShapeRequestShapeType::Hexagon,
                "HOME_PLATE" => CreateShapeRequestShapeType::HomePlate,
                "HORIZONTAL_SCROLL" => CreateShapeRequestShapeType::HorizontalScroll,
                "IRREGULAR_SEAL_1" => CreateShapeRequestShapeType::IrregularSeal1,
                "IRREGULAR_SEAL_2" => CreateShapeRequestShapeType::IrregularSeal2,
                "LEFT_ARROW" => CreateShapeRequestShapeType::LeftArrow,
                "LEFT_ARROW_CALLOUT" => CreateShapeRequestShapeType::LeftArrowCallout,
                "LEFT_BRACE" => CreateShapeRequestShapeType::LeftBrace,
                "LEFT_BRACKET" => CreateShapeRequestShapeType::LeftBracket,
                "LEFT_RIGHT_ARROW" => CreateShapeRequestShapeType::LeftRightArrow,
                "LEFT_RIGHT_ARROW_CALLOUT" => CreateShapeRequestShapeType::LeftRightArrowCallout,
                "LEFT_RIGHT_UP_ARROW" => CreateShapeRequestShapeType::LeftRightUpArrow,
                "LEFT_UP_ARROW" => CreateShapeRequestShapeType::LeftUpArrow,
                "LIGHTNING_BOLT" => CreateShapeRequestShapeType::LightningBolt,
                "MATH_DIVIDE" => CreateShapeRequestShapeType::MathDivide,
                "MATH_EQUAL" => CreateShapeRequestShapeType::MathEqual,
                "MATH_MINUS" => CreateShapeRequestShapeType::MathMinus,
                "MATH_MULTIPLY" => CreateShapeRequestShapeType::MathMultiply,
                "MATH_NOT_EQUAL" => CreateShapeRequestShapeType::MathNotEqual,
                "MATH_PLUS" => CreateShapeRequestShapeType::MathPlus,
                "MOON" => CreateShapeRequestShapeType::Moon,
                "NO_SMOKING" => CreateShapeRequestShapeType::NoSmoking,
                "NOTCHED_RIGHT_ARROW" => CreateShapeRequestShapeType::NotchedRightArrow,
                "OCTAGON" => CreateShapeRequestShapeType::Octagon,
                "PARALLELOGRAM" => CreateShapeRequestShapeType::Parallelogram,
                "PENTAGON" => CreateShapeRequestShapeType::Pentagon,
                "PIE" => CreateShapeRequestShapeType::Pie,
                "PLAQUE" => CreateShapeRequestShapeType::Plaque,
                "PLUS" => CreateShapeRequestShapeType::Plus,
                "QUAD_ARROW" => CreateShapeRequestShapeType::QuadArrow,
                "QUAD_ARROW_CALLOUT" => CreateShapeRequestShapeType::QuadArrowCallout,
                "RECTANGLE" => CreateShapeRequestShapeType::Rectangle,
                "RIBBON" => CreateShapeRequestShapeType::Ribbon,
                "RIBBON_2" => CreateShapeRequestShapeType::Ribbon2,
                "RIGHT_ARROW" => CreateShapeRequestShapeType::RightArrow,
                "RIGHT_ARROW_CALLOUT" => CreateShapeRequestShapeType::RightArrowCallout,
                "RIGHT_BRACE" => CreateShapeRequestShapeType::RightBrace,
                "RIGHT_BRACKET" => CreateShapeRequestShapeType::RightBracket,
                "RIGHT_TRIANGLE" => CreateShapeRequestShapeType::RightTriangle,
                "ROUND_1_RECTANGLE" => CreateShapeRequestShapeType::Round1Rectangle,
                "ROUND_2_DIAGONAL_RECTANGLE" => {
                    CreateShapeRequestShapeType::Round2DiagonalRectangle
                }
                "ROUND_2_SAME_RECTANGLE" => CreateShapeRequestShapeType::Round2SameRectangle,
                "ROUND_RECTANGLE" => CreateShapeRequestShapeType::RoundRectangle,
                "SMILEY_FACE" => CreateShapeRequestShapeType::SmileyFace,
                "SNIP_1_RECTANGLE" => CreateShapeRequestShapeType::Snip1Rectangle,
                "SNIP_2_DIAGONAL_RECTANGLE" => CreateShapeRequestShapeType::Snip2DiagonalRectangle,
                "SNIP_2_SAME_RECTANGLE" => CreateShapeRequestShapeType::Snip2SameRectangle,
                "SNIP_ROUND_RECTANGLE" => CreateShapeRequestShapeType::SnipRoundRectangle,
                "SPEECH" => CreateShapeRequestShapeType::Speech,
                "STAR_10" => CreateShapeRequestShapeType::Star10,
                "STAR_12" => CreateShapeRequestShapeType::Star12,
                "STAR_16" => CreateShapeRequestShapeType::Star16,
                "STAR_24" => CreateShapeRequestShapeType::Star24,
                "STAR_32" => CreateShapeRequestShapeType::Star32,
                "STAR_4" => CreateShapeRequestShapeType::Star4,
                "STAR_5" => CreateShapeRequestShapeType::Star5,
                "STAR_6" => CreateShapeRequestShapeType::Star6,
                "STAR_7" => CreateShapeRequestShapeType::Star7,
                "STAR_8" => CreateShapeRequestShapeType::Star8,
                "STARBURST" => CreateShapeRequestShapeType::Starburst,
                "STRIPED_RIGHT_ARROW" => CreateShapeRequestShapeType::StripedRightArrow,
                "SUN" => CreateShapeRequestShapeType::Sun,
                "TEARDROP" => CreateShapeRequestShapeType::Teardrop,
                "TEXT_BOX" => CreateShapeRequestShapeType::TextBox,
                "TRAPEZOID" => CreateShapeRequestShapeType::Trapezoid,
                "TRIANGLE" => CreateShapeRequestShapeType::Triangle,
                "TYPE_UNSPECIFIED" => CreateShapeRequestShapeType::TypeUnspecified,
                "UP_ARROW" => CreateShapeRequestShapeType::UpArrow,
                "UP_ARROW_CALLOUT" => CreateShapeRequestShapeType::UpArrowCallout,
                "UP_DOWN_ARROW" => CreateShapeRequestShapeType::UpDownArrow,
                "UTURN_ARROW" => CreateShapeRequestShapeType::UturnArrow,
                "VERTICAL_SCROLL" => CreateShapeRequestShapeType::VerticalScroll,
                "WAVE" => CreateShapeRequestShapeType::Wave,
                "WEDGE_ELLIPSE_CALLOUT" => CreateShapeRequestShapeType::WedgeEllipseCallout,
                "WEDGE_RECTANGLE_CALLOUT" => CreateShapeRequestShapeType::WedgeRectangleCallout,
                "WEDGE_ROUND_RECTANGLE_CALLOUT" => {
                    CreateShapeRequestShapeType::WedgeRoundRectangleCallout
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateShapeRequestShapeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateShapeRequestShapeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateShapeRequestShapeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARC" => CreateShapeRequestShapeType::Arc,
                "ARROW_EAST" => CreateShapeRequestShapeType::ArrowEast,
                "ARROW_NORTH" => CreateShapeRequestShapeType::ArrowNorth,
                "ARROW_NORTH_EAST" => CreateShapeRequestShapeType::ArrowNorthEast,
                "BENT_ARROW" => CreateShapeRequestShapeType::BentArrow,
                "BENT_UP_ARROW" => CreateShapeRequestShapeType::BentUpArrow,
                "BEVEL" => CreateShapeRequestShapeType::Bevel,
                "BLOCK_ARC" => CreateShapeRequestShapeType::BlockArc,
                "BRACE_PAIR" => CreateShapeRequestShapeType::BracePair,
                "BRACKET_PAIR" => CreateShapeRequestShapeType::BracketPair,
                "CAN" => CreateShapeRequestShapeType::Can,
                "CHEVRON" => CreateShapeRequestShapeType::Chevron,
                "CHORD" => CreateShapeRequestShapeType::Chord,
                "CLOUD" => CreateShapeRequestShapeType::Cloud,
                "CLOUD_CALLOUT" => CreateShapeRequestShapeType::CloudCallout,
                "CORNER" => CreateShapeRequestShapeType::Corner,
                "CUBE" => CreateShapeRequestShapeType::Cube,
                "CURVED_DOWN_ARROW" => CreateShapeRequestShapeType::CurvedDownArrow,
                "CURVED_LEFT_ARROW" => CreateShapeRequestShapeType::CurvedLeftArrow,
                "CURVED_RIGHT_ARROW" => CreateShapeRequestShapeType::CurvedRightArrow,
                "CURVED_UP_ARROW" => CreateShapeRequestShapeType::CurvedUpArrow,
                "CUSTOM" => CreateShapeRequestShapeType::Custom,
                "DECAGON" => CreateShapeRequestShapeType::Decagon,
                "DIAGONAL_STRIPE" => CreateShapeRequestShapeType::DiagonalStripe,
                "DIAMOND" => CreateShapeRequestShapeType::Diamond,
                "DODECAGON" => CreateShapeRequestShapeType::Dodecagon,
                "DONUT" => CreateShapeRequestShapeType::Donut,
                "DOUBLE_WAVE" => CreateShapeRequestShapeType::DoubleWave,
                "DOWN_ARROW" => CreateShapeRequestShapeType::DownArrow,
                "DOWN_ARROW_CALLOUT" => CreateShapeRequestShapeType::DownArrowCallout,
                "ELLIPSE" => CreateShapeRequestShapeType::Ellipse,
                "ELLIPSE_RIBBON" => CreateShapeRequestShapeType::EllipseRibbon,
                "ELLIPSE_RIBBON_2" => CreateShapeRequestShapeType::EllipseRibbon2,
                "FLOW_CHART_ALTERNATE_PROCESS" => {
                    CreateShapeRequestShapeType::FlowChartAlternateProcess
                }
                "FLOW_CHART_COLLATE" => CreateShapeRequestShapeType::FlowChartCollate,
                "FLOW_CHART_CONNECTOR" => CreateShapeRequestShapeType::FlowChartConnector,
                "FLOW_CHART_DECISION" => CreateShapeRequestShapeType::FlowChartDecision,
                "FLOW_CHART_DELAY" => CreateShapeRequestShapeType::FlowChartDelay,
                "FLOW_CHART_DISPLAY" => CreateShapeRequestShapeType::FlowChartDisplay,
                "FLOW_CHART_DOCUMENT" => CreateShapeRequestShapeType::FlowChartDocument,
                "FLOW_CHART_EXTRACT" => CreateShapeRequestShapeType::FlowChartExtract,
                "FLOW_CHART_INPUT_OUTPUT" => CreateShapeRequestShapeType::FlowChartInputOutput,
                "FLOW_CHART_INTERNAL_STORAGE" => {
                    CreateShapeRequestShapeType::FlowChartInternalStorage
                }
                "FLOW_CHART_MAGNETIC_DISK" => CreateShapeRequestShapeType::FlowChartMagneticDisk,
                "FLOW_CHART_MAGNETIC_DRUM" => CreateShapeRequestShapeType::FlowChartMagneticDrum,
                "FLOW_CHART_MAGNETIC_TAPE" => CreateShapeRequestShapeType::FlowChartMagneticTape,
                "FLOW_CHART_MANUAL_INPUT" => CreateShapeRequestShapeType::FlowChartManualInput,
                "FLOW_CHART_MANUAL_OPERATION" => {
                    CreateShapeRequestShapeType::FlowChartManualOperation
                }
                "FLOW_CHART_MERGE" => CreateShapeRequestShapeType::FlowChartMerge,
                "FLOW_CHART_MULTIDOCUMENT" => CreateShapeRequestShapeType::FlowChartMultidocument,
                "FLOW_CHART_OFFLINE_STORAGE" => {
                    CreateShapeRequestShapeType::FlowChartOfflineStorage
                }
                "FLOW_CHART_OFFPAGE_CONNECTOR" => {
                    CreateShapeRequestShapeType::FlowChartOffpageConnector
                }
                "FLOW_CHART_ONLINE_STORAGE" => CreateShapeRequestShapeType::FlowChartOnlineStorage,
                "FLOW_CHART_OR" => CreateShapeRequestShapeType::FlowChartOr,
                "FLOW_CHART_PREDEFINED_PROCESS" => {
                    CreateShapeRequestShapeType::FlowChartPredefinedProcess
                }
                "FLOW_CHART_PREPARATION" => CreateShapeRequestShapeType::FlowChartPreparation,
                "FLOW_CHART_PROCESS" => CreateShapeRequestShapeType::FlowChartProcess,
                "FLOW_CHART_PUNCHED_CARD" => CreateShapeRequestShapeType::FlowChartPunchedCard,
                "FLOW_CHART_PUNCHED_TAPE" => CreateShapeRequestShapeType::FlowChartPunchedTape,
                "FLOW_CHART_SORT" => CreateShapeRequestShapeType::FlowChartSort,
                "FLOW_CHART_SUMMING_JUNCTION" => {
                    CreateShapeRequestShapeType::FlowChartSummingJunction
                }
                "FLOW_CHART_TERMINATOR" => CreateShapeRequestShapeType::FlowChartTerminator,
                "FOLDED_CORNER" => CreateShapeRequestShapeType::FoldedCorner,
                "FRAME" => CreateShapeRequestShapeType::Frame,
                "HALF_FRAME" => CreateShapeRequestShapeType::HalfFrame,
                "HEART" => CreateShapeRequestShapeType::Heart,
                "HEPTAGON" => CreateShapeRequestShapeType::Heptagon,
                "HEXAGON" => CreateShapeRequestShapeType::Hexagon,
                "HOME_PLATE" => CreateShapeRequestShapeType::HomePlate,
                "HORIZONTAL_SCROLL" => CreateShapeRequestShapeType::HorizontalScroll,
                "IRREGULAR_SEAL_1" => CreateShapeRequestShapeType::IrregularSeal1,
                "IRREGULAR_SEAL_2" => CreateShapeRequestShapeType::IrregularSeal2,
                "LEFT_ARROW" => CreateShapeRequestShapeType::LeftArrow,
                "LEFT_ARROW_CALLOUT" => CreateShapeRequestShapeType::LeftArrowCallout,
                "LEFT_BRACE" => CreateShapeRequestShapeType::LeftBrace,
                "LEFT_BRACKET" => CreateShapeRequestShapeType::LeftBracket,
                "LEFT_RIGHT_ARROW" => CreateShapeRequestShapeType::LeftRightArrow,
                "LEFT_RIGHT_ARROW_CALLOUT" => CreateShapeRequestShapeType::LeftRightArrowCallout,
                "LEFT_RIGHT_UP_ARROW" => CreateShapeRequestShapeType::LeftRightUpArrow,
                "LEFT_UP_ARROW" => CreateShapeRequestShapeType::LeftUpArrow,
                "LIGHTNING_BOLT" => CreateShapeRequestShapeType::LightningBolt,
                "MATH_DIVIDE" => CreateShapeRequestShapeType::MathDivide,
                "MATH_EQUAL" => CreateShapeRequestShapeType::MathEqual,
                "MATH_MINUS" => CreateShapeRequestShapeType::MathMinus,
                "MATH_MULTIPLY" => CreateShapeRequestShapeType::MathMultiply,
                "MATH_NOT_EQUAL" => CreateShapeRequestShapeType::MathNotEqual,
                "MATH_PLUS" => CreateShapeRequestShapeType::MathPlus,
                "MOON" => CreateShapeRequestShapeType::Moon,
                "NO_SMOKING" => CreateShapeRequestShapeType::NoSmoking,
                "NOTCHED_RIGHT_ARROW" => CreateShapeRequestShapeType::NotchedRightArrow,
                "OCTAGON" => CreateShapeRequestShapeType::Octagon,
                "PARALLELOGRAM" => CreateShapeRequestShapeType::Parallelogram,
                "PENTAGON" => CreateShapeRequestShapeType::Pentagon,
                "PIE" => CreateShapeRequestShapeType::Pie,
                "PLAQUE" => CreateShapeRequestShapeType::Plaque,
                "PLUS" => CreateShapeRequestShapeType::Plus,
                "QUAD_ARROW" => CreateShapeRequestShapeType::QuadArrow,
                "QUAD_ARROW_CALLOUT" => CreateShapeRequestShapeType::QuadArrowCallout,
                "RECTANGLE" => CreateShapeRequestShapeType::Rectangle,
                "RIBBON" => CreateShapeRequestShapeType::Ribbon,
                "RIBBON_2" => CreateShapeRequestShapeType::Ribbon2,
                "RIGHT_ARROW" => CreateShapeRequestShapeType::RightArrow,
                "RIGHT_ARROW_CALLOUT" => CreateShapeRequestShapeType::RightArrowCallout,
                "RIGHT_BRACE" => CreateShapeRequestShapeType::RightBrace,
                "RIGHT_BRACKET" => CreateShapeRequestShapeType::RightBracket,
                "RIGHT_TRIANGLE" => CreateShapeRequestShapeType::RightTriangle,
                "ROUND_1_RECTANGLE" => CreateShapeRequestShapeType::Round1Rectangle,
                "ROUND_2_DIAGONAL_RECTANGLE" => {
                    CreateShapeRequestShapeType::Round2DiagonalRectangle
                }
                "ROUND_2_SAME_RECTANGLE" => CreateShapeRequestShapeType::Round2SameRectangle,
                "ROUND_RECTANGLE" => CreateShapeRequestShapeType::RoundRectangle,
                "SMILEY_FACE" => CreateShapeRequestShapeType::SmileyFace,
                "SNIP_1_RECTANGLE" => CreateShapeRequestShapeType::Snip1Rectangle,
                "SNIP_2_DIAGONAL_RECTANGLE" => CreateShapeRequestShapeType::Snip2DiagonalRectangle,
                "SNIP_2_SAME_RECTANGLE" => CreateShapeRequestShapeType::Snip2SameRectangle,
                "SNIP_ROUND_RECTANGLE" => CreateShapeRequestShapeType::SnipRoundRectangle,
                "SPEECH" => CreateShapeRequestShapeType::Speech,
                "STAR_10" => CreateShapeRequestShapeType::Star10,
                "STAR_12" => CreateShapeRequestShapeType::Star12,
                "STAR_16" => CreateShapeRequestShapeType::Star16,
                "STAR_24" => CreateShapeRequestShapeType::Star24,
                "STAR_32" => CreateShapeRequestShapeType::Star32,
                "STAR_4" => CreateShapeRequestShapeType::Star4,
                "STAR_5" => CreateShapeRequestShapeType::Star5,
                "STAR_6" => CreateShapeRequestShapeType::Star6,
                "STAR_7" => CreateShapeRequestShapeType::Star7,
                "STAR_8" => CreateShapeRequestShapeType::Star8,
                "STARBURST" => CreateShapeRequestShapeType::Starburst,
                "STRIPED_RIGHT_ARROW" => CreateShapeRequestShapeType::StripedRightArrow,
                "SUN" => CreateShapeRequestShapeType::Sun,
                "TEARDROP" => CreateShapeRequestShapeType::Teardrop,
                "TEXT_BOX" => CreateShapeRequestShapeType::TextBox,
                "TRAPEZOID" => CreateShapeRequestShapeType::Trapezoid,
                "TRIANGLE" => CreateShapeRequestShapeType::Triangle,
                "TYPE_UNSPECIFIED" => CreateShapeRequestShapeType::TypeUnspecified,
                "UP_ARROW" => CreateShapeRequestShapeType::UpArrow,
                "UP_ARROW_CALLOUT" => CreateShapeRequestShapeType::UpArrowCallout,
                "UP_DOWN_ARROW" => CreateShapeRequestShapeType::UpDownArrow,
                "UTURN_ARROW" => CreateShapeRequestShapeType::UturnArrow,
                "VERTICAL_SCROLL" => CreateShapeRequestShapeType::VerticalScroll,
                "WAVE" => CreateShapeRequestShapeType::Wave,
                "WEDGE_ELLIPSE_CALLOUT" => CreateShapeRequestShapeType::WedgeEllipseCallout,
                "WEDGE_RECTANGLE_CALLOUT" => CreateShapeRequestShapeType::WedgeRectangleCallout,
                "WEDGE_ROUND_RECTANGLE_CALLOUT" => {
                    CreateShapeRequestShapeType::WedgeRoundRectangleCallout
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
    impl ::google_field_selector::FieldSelector for CreateShapeRequestShapeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateShapeRequestShapeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateShapeResponse {
        #[doc = "The object ID of the created shape."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateShapeResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateShapeResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateSheetsChartRequest {
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet."]
        #[serde(
            rename = "chartId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_id: ::std::option::Option<i32>,
        #[doc = "The element properties for the chart.\n\nWhen the aspect ratio of the provided size does not match the chart aspect\nratio, the chart is scaled and centered with respect to the size in order\nto maintain aspect ratio. The provided transform is applied after this\noperation."]
        #[serde(
            rename = "elementProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element_properties: ::std::option::Option<crate::schemas::PageElementProperties>,
        #[doc = "The mode with which the chart is linked to the source spreadsheet. When\nnot specified, the chart will be an image that is not linked."]
        #[serde(
            rename = "linkingMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linking_mode:
            ::std::option::Option<crate::schemas::CreateSheetsChartRequestLinkingMode>,
        #[doc = "A user-supplied object ID.\n\nIf specified, the ID must be unique among all pages and page elements in\nthe presentation. The ID should start with a word character [a-zA-Z0-9_]\nand then followed by any number of the following characters [a-zA-Z0-9_-:].\nThe length of the ID should not be less than 5 or greater than 50.\nIf empty, a unique identifier will be generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The ID of the Google Sheets spreadsheet that contains the chart."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateSheetsChartRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateSheetsChartRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateSheetsChartRequestLinkingMode {
        #[doc = "Linking the chart allows it to be updated, and other collaborators will\nsee a link to the spreadsheet."]
        Linked,
        #[doc = "The chart is not associated with the source spreadsheet and cannot be\nupdated. A chart that is not linked will be inserted as an image."]
        NotLinkedImage,
    }
    impl CreateSheetsChartRequestLinkingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateSheetsChartRequestLinkingMode::Linked => "LINKED",
                CreateSheetsChartRequestLinkingMode::NotLinkedImage => "NOT_LINKED_IMAGE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateSheetsChartRequestLinkingMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateSheetsChartRequestLinkingMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreateSheetsChartRequestLinkingMode, ()> {
            Ok(match s {
                "LINKED" => CreateSheetsChartRequestLinkingMode::Linked,
                "NOT_LINKED_IMAGE" => CreateSheetsChartRequestLinkingMode::NotLinkedImage,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateSheetsChartRequestLinkingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateSheetsChartRequestLinkingMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateSheetsChartRequestLinkingMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LINKED" => CreateSheetsChartRequestLinkingMode::Linked,
                "NOT_LINKED_IMAGE" => CreateSheetsChartRequestLinkingMode::NotLinkedImage,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CreateSheetsChartRequestLinkingMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateSheetsChartRequestLinkingMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateSheetsChartResponse {
        #[doc = "The object ID of the created chart."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateSheetsChartResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateSheetsChartResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateSlideRequest {
        #[doc = "The optional zero-based index indicating where to insert the slides.\n\nIf you don't specify an index, the new slide is created at the end."]
        #[serde(
            rename = "insertionIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insertion_index: ::std::option::Option<i32>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "An optional list of object ID mappings from the placeholder(s) on the layout to the placeholder(s)\nthat will be created on the new slide from that specified layout. Can only\nbe used when `slide_layout_reference` is specified."]
        #[serde(
            rename = "placeholderIdMappings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub placeholder_id_mappings:
            ::std::option::Option<Vec<crate::schemas::LayoutPlaceholderIdMapping>>,
        #[doc = "Layout reference of the slide to be inserted, based on the *current\nmaster*, which is one of the following:\n\n* The master of the previous slide index.\n* The master of the first slide, if the insertion_index is zero.\n* The first master in the presentation, if there are no slides.\n\nIf the LayoutReference is not found in the current master, a 400 bad\nrequest error is returned.\n\nIf you don't specify a layout reference, then the new slide will use the\npredefined layout `BLANK`."]
        #[serde(
            rename = "slideLayoutReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slide_layout_reference: ::std::option::Option<crate::schemas::LayoutReference>,
    }
    impl ::google_field_selector::FieldSelector for CreateSlideRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateSlideRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateSlideResponse {
        #[doc = "The object ID of the created slide."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateSlideResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateSlideResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateTableRequest {
        #[doc = "Number of columns in the table."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<i32>,
        #[doc = "The element properties for the table.\n\nThe table will be created at the provided size, subject to a minimum size.\nIf no size is provided, the table will be automatically sized.\n\nTable transforms must have a scale of 1 and no shear components. If no\ntransform is provided, the table will be centered on the page."]
        #[serde(
            rename = "elementProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element_properties: ::std::option::Option<crate::schemas::PageElementProperties>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "Number of rows in the table."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for CreateTableRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateTableRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateTableResponse {
        #[doc = "The object ID of the created table."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateTableResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateTableResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateVideoRequest {
        #[doc = "The element properties for the video.\n\nThe PageElementProperties.size property is\noptional. If you don't specify a size, a default size is chosen by the\nserver.\n\nThe PageElementProperties.transform property is\noptional. The transform must not have shear components.\nIf you don't specify a transform, the video will be placed at the top left\ncorner of the page."]
        #[serde(
            rename = "elementProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element_properties: ::std::option::Option<crate::schemas::PageElementProperties>,
        #[doc = "The video source's unique identifier for this video.\n\ne.g. For YouTube video https://www.youtube.com/watch?v=7U3axjORYZ0,\nthe ID is 7U3axjORYZ0. For a Google Drive video\nhttps://drive.google.com/file/d/1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q the ID\nis 1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The video source."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::CreateVideoRequestSource>,
    }
    impl ::google_field_selector::FieldSelector for CreateVideoRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateVideoRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateVideoRequestSource {
        #[doc = "The video source is Google Drive."]
        Drive,
        #[doc = "The video source is unspecified."]
        SourceUnspecified,
        #[doc = "The video source is YouTube."]
        Youtube,
    }
    impl CreateVideoRequestSource {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateVideoRequestSource::Drive => "DRIVE",
                CreateVideoRequestSource::SourceUnspecified => "SOURCE_UNSPECIFIED",
                CreateVideoRequestSource::Youtube => "YOUTUBE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateVideoRequestSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateVideoRequestSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreateVideoRequestSource, ()> {
            Ok(match s {
                "DRIVE" => CreateVideoRequestSource::Drive,
                "SOURCE_UNSPECIFIED" => CreateVideoRequestSource::SourceUnspecified,
                "YOUTUBE" => CreateVideoRequestSource::Youtube,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateVideoRequestSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateVideoRequestSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateVideoRequestSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DRIVE" => CreateVideoRequestSource::Drive,
                "SOURCE_UNSPECIFIED" => CreateVideoRequestSource::SourceUnspecified,
                "YOUTUBE" => CreateVideoRequestSource::Youtube,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CreateVideoRequestSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateVideoRequestSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateVideoResponse {
        #[doc = "The object ID of the created video."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateVideoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateVideoResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CropProperties {
        #[doc = "The rotation angle of the crop window around its center, in radians.\nRotation angle is applied after the offset."]
        #[serde(
            rename = "angle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub angle: ::std::option::Option<f32>,
        #[doc = "The offset specifies the bottom edge of the crop rectangle that is located\nabove the original bounding rectangle bottom edge, relative to the object's\noriginal height."]
        #[serde(
            rename = "bottomOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom_offset: ::std::option::Option<f32>,
        #[doc = "The offset specifies the left edge of the crop rectangle that is located to\nthe right of the original bounding rectangle left edge, relative to the\nobject's original width."]
        #[serde(
            rename = "leftOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left_offset: ::std::option::Option<f32>,
        #[doc = "The offset specifies the right edge of the crop rectangle that is located\nto the left of the original bounding rectangle right edge, relative to the\nobject's original width."]
        #[serde(
            rename = "rightOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right_offset: ::std::option::Option<f32>,
        #[doc = "The offset specifies the top edge of the crop rectangle that is located\nbelow the original bounding rectangle top edge, relative to the object's\noriginal height."]
        #[serde(
            rename = "topOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_offset: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for CropProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CropProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteObjectRequest {
        #[doc = "The object ID of the page or page element to delete.\n\nIf after a delete operation a group contains\nonly 1 or no page elements, the group is also deleted.\n\nIf a placeholder is deleted on a layout, any empty inheriting shapes are\nalso deleted."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteObjectRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteObjectRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteParagraphBulletsRequest {
        #[doc = "The optional table cell location if the text to be modified is in a table\ncell. If present, the object_id must refer to a table."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The object ID of the shape or table containing the text to delete bullets\nfrom."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The range of text to delete bullets from, based on TextElement indexes."]
        #[serde(
            rename = "textRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_range: ::std::option::Option<crate::schemas::Range>,
    }
    impl ::google_field_selector::FieldSelector for DeleteParagraphBulletsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteParagraphBulletsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteTableColumnRequest {
        #[doc = "The reference table cell location from which a column will be deleted.\n\nThe column this cell spans will be deleted. If this is a merged cell,\nmultiple columns will be deleted. If no columns remain in the table after\nthis deletion, the whole table is deleted."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The table to delete columns from."]
        #[serde(
            rename = "tableObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteTableColumnRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteTableColumnRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteTableRowRequest {
        #[doc = "The reference table cell location from which a row will be deleted.\n\nThe row this cell spans will be deleted. If this is a merged cell, multiple\nrows will be deleted. If no rows remain in the table after this deletion,\nthe whole table is deleted."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The table to delete rows from."]
        #[serde(
            rename = "tableObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteTableRowRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteTableRowRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteTextRequest {
        #[doc = "The optional table cell location if the text is to be deleted from a table\ncell. If present, the object_id must refer to a table."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The object ID of the shape or table from which the text will be deleted."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The range of text to delete, based on TextElement indexes.\n\nThere is always an implicit newline character at the end of a shape's or\ntable cell's text that cannot be deleted. `Range.Type.ALL` will use the\ncorrect bounds, but care must be taken when specifying explicit bounds for\nrange types `FROM_START_INDEX` and `FIXED_RANGE`. For example, if the text\nis \"ABC\", followed by an implicit newline, then the maximum value is 2 for\n`text_range.start_index` and 3 for `text_range.end_index`.\n\nDeleting text that crosses a paragraph boundary may result in changes\nto paragraph styles and lists as the two paragraphs are merged.\n\nRanges that include only one code unit of a surrogate pair are expanded to\ninclude both code units."]
        #[serde(
            rename = "textRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_range: ::std::option::Option<crate::schemas::Range>,
    }
    impl ::google_field_selector::FieldSelector for DeleteTextRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteTextRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Dimension {
        #[doc = "The magnitude."]
        #[serde(
            rename = "magnitude",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub magnitude: ::std::option::Option<f64>,
        #[doc = "The units for magnitude."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<crate::schemas::DimensionUnit>,
    }
    impl ::google_field_selector::FieldSelector for Dimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DimensionUnit {
        #[doc = "An English Metric Unit (EMU) is defined as 1/360,000 of a centimeter\nand thus there are 914,400 EMUs per inch, and 12,700 EMUs per point."]
        Emu,
        #[doc = "A point, 1/72 of an inch."]
        Pt,
        #[doc = "The units are unknown."]
        UnitUnspecified,
    }
    impl DimensionUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                DimensionUnit::Emu => "EMU",
                DimensionUnit::Pt => "PT",
                DimensionUnit::UnitUnspecified => "UNIT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DimensionUnit {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DimensionUnit {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DimensionUnit, ()> {
            Ok(match s {
                "EMU" => DimensionUnit::Emu,
                "PT" => DimensionUnit::Pt,
                "UNIT_UNSPECIFIED" => DimensionUnit::UnitUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DimensionUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DimensionUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DimensionUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMU" => DimensionUnit::Emu,
                "PT" => DimensionUnit::Pt,
                "UNIT_UNSPECIFIED" => DimensionUnit::UnitUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DimensionUnit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionUnit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DuplicateObjectRequest {
        #[doc = "The ID of the object to duplicate."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The object being duplicated may contain other objects, for example when\nduplicating a slide or a group page element. This map defines how the IDs\nof duplicated objects are generated: the keys are the IDs of the original\nobjects and its values are the IDs that will be assigned to the\ncorresponding duplicate object. The ID of the source object's duplicate\nmay be specified in this map as well, using the same value of the\n`object_id` field as a key and the newly desired ID as the value.\n\nAll keys must correspond to existing IDs in the presentation. All values\nmust be unique in the presentation and must start with an alphanumeric\ncharacter or an underscore (matches regex `[a-zA-Z0-9_]`); remaining\ncharacters may include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`). The length of the new ID must not be less than 5 or\ngreater than 50.\n\nIf any IDs of source objects are omitted from the map, a new random ID will\nbe assigned. If the map is empty or unset, all duplicate objects will\nreceive a new random ID."]
        #[serde(
            rename = "objectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_ids: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for DuplicateObjectRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DuplicateObjectRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DuplicateObjectResponse {
        #[doc = "The ID of the new duplicate object."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DuplicateObjectResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DuplicateObjectResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Group {
        #[doc = "The collection of elements in the group. The minimum size of a group is 2."]
        #[serde(
            rename = "children",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub children: ::std::option::Option<Vec<crate::schemas::PageElement>>,
    }
    impl ::google_field_selector::FieldSelector for Group {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Group {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GroupObjectsRequest {
        #[doc = "The object IDs of the objects to group.\n\nOnly page elements can be grouped. There should be at least two page\nelements on the same page that are not already in another group. Some page\nelements, such as videos, tables and placeholder shapes cannot be grouped."]
        #[serde(
            rename = "childrenObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub children_object_ids: ::std::option::Option<Vec<String>>,
        #[doc = "A user-supplied object ID for the group to be created.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(
            rename = "groupObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GroupObjectsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GroupObjectsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GroupObjectsResponse {
        #[doc = "The object ID of the created group."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GroupObjectsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GroupObjectsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Image {
        #[doc = "An URL to an image with a default lifetime of 30 minutes.\nThis URL is tagged with the account of the requester. Anyone with the URL\neffectively accesses the image as the original requester. Access to the\nimage may be lost if the presentation's sharing settings change."]
        #[serde(
            rename = "contentUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_url: ::std::option::Option<String>,
        #[doc = "The properties of the image."]
        #[serde(
            rename = "imageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_properties: ::std::option::Option<crate::schemas::ImageProperties>,
        #[doc = "The source URL is the URL used to insert the image. The source URL can be\nempty."]
        #[serde(
            rename = "sourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Image {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ImageProperties {
        #[doc = "The brightness effect of the image. The value should be in the interval\n[-1.0, 1.0], where 0 means no effect. This property is read-only."]
        #[serde(
            rename = "brightness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brightness: ::std::option::Option<f32>,
        #[doc = "The contrast effect of the image. The value should be in the interval\n[-1.0, 1.0], where 0 means no effect. This property is read-only."]
        #[serde(
            rename = "contrast",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contrast: ::std::option::Option<f32>,
        #[doc = "The crop properties of the image. If not set, the image is not cropped.\nThis property is read-only."]
        #[serde(
            rename = "cropProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crop_properties: ::std::option::Option<crate::schemas::CropProperties>,
        #[doc = "The hyperlink destination of the image. If unset, there is no link."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<crate::schemas::Link>,
        #[doc = "The outline of the image. If not set, the image has no outline."]
        #[serde(
            rename = "outline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outline: ::std::option::Option<crate::schemas::Outline>,
        #[doc = "The recolor effect of the image. If not set, the image is not recolored.\nThis property is read-only."]
        #[serde(
            rename = "recolor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recolor: ::std::option::Option<crate::schemas::Recolor>,
        #[doc = "The shadow of the image. If not set, the image has no shadow. This property\nis read-only."]
        #[serde(
            rename = "shadow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shadow: ::std::option::Option<crate::schemas::Shadow>,
        #[doc = "The transparency effect of the image. The value should be in the interval\n[0.0, 1.0], where 0 means no effect and 1 means completely transparent.\nThis property is read-only."]
        #[serde(
            rename = "transparency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transparency: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for ImageProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InsertTableColumnsRequest {
        #[doc = "The reference table cell location from which columns will be inserted.\n\nA new column will be inserted to the left (or right) of the column where\nthe reference cell is. If the reference cell is a merged cell, a new\ncolumn will be inserted to the left (or right) of the merged cell."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "Whether to insert new columns to the right of the reference cell location.\n\n* `True`: insert to the right.\n* `False`: insert to the left."]
        #[serde(
            rename = "insertRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_right: ::std::option::Option<bool>,
        #[doc = "The number of columns to be inserted. Maximum 20 per request."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
        #[doc = "The table to insert columns into."]
        #[serde(
            rename = "tableObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InsertTableColumnsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertTableColumnsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InsertTableRowsRequest {
        #[doc = "The reference table cell location from which rows will be inserted.\n\nA new row will be inserted above (or below) the row where the reference\ncell is. If the reference cell is a merged cell, a new row will be\ninserted above (or below) the merged cell."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "Whether to insert new rows below the reference cell location.\n\n* `True`: insert below the cell.\n* `False`: insert above the cell."]
        #[serde(
            rename = "insertBelow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_below: ::std::option::Option<bool>,
        #[doc = "The number of rows to be inserted. Maximum 20 per request."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<i32>,
        #[doc = "The table to insert rows into."]
        #[serde(
            rename = "tableObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InsertTableRowsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertTableRowsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InsertTextRequest {
        #[doc = "The optional table cell location if the text is to be inserted into a table\ncell. If present, the object_id must refer to a table."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The index where the text will be inserted, in Unicode code units, based\non TextElement indexes.\n\nThe index is zero-based and is computed from the start of the string.\nThe index may be adjusted to prevent insertions inside Unicode grapheme\nclusters. In these cases, the text will be inserted immediately after the\ngrapheme cluster."]
        #[serde(
            rename = "insertionIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insertion_index: ::std::option::Option<i32>,
        #[doc = "The object ID of the shape or table where the text will be inserted."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The text to be inserted.\n\nInserting a newline character will implicitly create a new\nParagraphMarker at that index.\nThe paragraph style of the new paragraph will be copied from the paragraph\nat the current insertion index, including lists and bullets.\n\nText styles for inserted text will be determined automatically, generally\npreserving the styling of neighboring text. In most cases, the text will be\nadded to the TextRun that exists at the\ninsertion index.\n\nSome control characters (U+0000-U+0008, U+000C-U+001F) and characters\nfrom the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF)\nwill be stripped out of the inserted text."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InsertTextRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertTextRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LayoutPlaceholderIdMapping {
        #[doc = "The placeholder on a layout that will be applied to a slide. Only type and index are needed. For example, a\npredefined `TITLE_AND_BODY` layout may usually have a TITLE placeholder\nwith index 0 and a BODY placeholder with index 0."]
        #[serde(
            rename = "layoutPlaceholder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_placeholder: ::std::option::Option<crate::schemas::Placeholder>,
        #[doc = "The object ID of the placeholder on a layout that will be applied\nto a slide."]
        #[serde(
            rename = "layoutPlaceholderObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_placeholder_object_id: ::std::option::Option<String>,
        #[doc = "A user-supplied object ID for the placeholder identified above that to be\ncreated onto a slide.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LayoutPlaceholderIdMapping {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LayoutPlaceholderIdMapping {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LayoutProperties {
        #[doc = "The human-readable name of the layout."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The object ID of the master that this layout is based on."]
        #[serde(
            rename = "masterObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_object_id: ::std::option::Option<String>,
        #[doc = "The name of the layout."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LayoutProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LayoutProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LayoutReference {
        #[doc = "Layout ID: the object ID of one of the layouts in the presentation."]
        #[serde(
            rename = "layoutId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_id: ::std::option::Option<String>,
        #[doc = "Predefined layout."]
        #[serde(
            rename = "predefinedLayout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predefined_layout:
            ::std::option::Option<crate::schemas::LayoutReferencePredefinedLayout>,
    }
    impl ::google_field_selector::FieldSelector for LayoutReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LayoutReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LayoutReferencePredefinedLayout {
        #[doc = "Layout with a big number heading."]
        BigNumber,
        #[doc = "Blank layout, with no placeholders."]
        Blank,
        #[doc = "Layout with a caption at the bottom."]
        CaptionOnly,
        #[doc = "Layout with a main point."]
        MainPoint,
        #[doc = "Layout with one title and one body, arranged in a single column."]
        OneColumnText,
        #[doc = "Unspecified layout."]
        PredefinedLayoutUnspecified,
        #[doc = "Layout with a section title."]
        SectionHeader,
        #[doc = "Layout with a title and subtitle on one side and description on the other."]
        SectionTitleAndDescription,
        #[doc = "Layout with a title and a subtitle."]
        Title,
        #[doc = "Layout with a title and body."]
        TitleAndBody,
        #[doc = "Layout with a title and two columns."]
        TitleAndTwoColumns,
        #[doc = "Layout with only a title."]
        TitleOnly,
    }
    impl LayoutReferencePredefinedLayout {
        pub fn as_str(self) -> &'static str {
            match self {
                LayoutReferencePredefinedLayout::BigNumber => "BIG_NUMBER",
                LayoutReferencePredefinedLayout::Blank => "BLANK",
                LayoutReferencePredefinedLayout::CaptionOnly => "CAPTION_ONLY",
                LayoutReferencePredefinedLayout::MainPoint => "MAIN_POINT",
                LayoutReferencePredefinedLayout::OneColumnText => "ONE_COLUMN_TEXT",
                LayoutReferencePredefinedLayout::PredefinedLayoutUnspecified => {
                    "PREDEFINED_LAYOUT_UNSPECIFIED"
                }
                LayoutReferencePredefinedLayout::SectionHeader => "SECTION_HEADER",
                LayoutReferencePredefinedLayout::SectionTitleAndDescription => {
                    "SECTION_TITLE_AND_DESCRIPTION"
                }
                LayoutReferencePredefinedLayout::Title => "TITLE",
                LayoutReferencePredefinedLayout::TitleAndBody => "TITLE_AND_BODY",
                LayoutReferencePredefinedLayout::TitleAndTwoColumns => "TITLE_AND_TWO_COLUMNS",
                LayoutReferencePredefinedLayout::TitleOnly => "TITLE_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LayoutReferencePredefinedLayout {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LayoutReferencePredefinedLayout {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LayoutReferencePredefinedLayout, ()> {
            Ok(match s {
                "BIG_NUMBER" => LayoutReferencePredefinedLayout::BigNumber,
                "BLANK" => LayoutReferencePredefinedLayout::Blank,
                "CAPTION_ONLY" => LayoutReferencePredefinedLayout::CaptionOnly,
                "MAIN_POINT" => LayoutReferencePredefinedLayout::MainPoint,
                "ONE_COLUMN_TEXT" => LayoutReferencePredefinedLayout::OneColumnText,
                "PREDEFINED_LAYOUT_UNSPECIFIED" => {
                    LayoutReferencePredefinedLayout::PredefinedLayoutUnspecified
                }
                "SECTION_HEADER" => LayoutReferencePredefinedLayout::SectionHeader,
                "SECTION_TITLE_AND_DESCRIPTION" => {
                    LayoutReferencePredefinedLayout::SectionTitleAndDescription
                }
                "TITLE" => LayoutReferencePredefinedLayout::Title,
                "TITLE_AND_BODY" => LayoutReferencePredefinedLayout::TitleAndBody,
                "TITLE_AND_TWO_COLUMNS" => LayoutReferencePredefinedLayout::TitleAndTwoColumns,
                "TITLE_ONLY" => LayoutReferencePredefinedLayout::TitleOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LayoutReferencePredefinedLayout {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LayoutReferencePredefinedLayout {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LayoutReferencePredefinedLayout {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BIG_NUMBER" => LayoutReferencePredefinedLayout::BigNumber,
                "BLANK" => LayoutReferencePredefinedLayout::Blank,
                "CAPTION_ONLY" => LayoutReferencePredefinedLayout::CaptionOnly,
                "MAIN_POINT" => LayoutReferencePredefinedLayout::MainPoint,
                "ONE_COLUMN_TEXT" => LayoutReferencePredefinedLayout::OneColumnText,
                "PREDEFINED_LAYOUT_UNSPECIFIED" => {
                    LayoutReferencePredefinedLayout::PredefinedLayoutUnspecified
                }
                "SECTION_HEADER" => LayoutReferencePredefinedLayout::SectionHeader,
                "SECTION_TITLE_AND_DESCRIPTION" => {
                    LayoutReferencePredefinedLayout::SectionTitleAndDescription
                }
                "TITLE" => LayoutReferencePredefinedLayout::Title,
                "TITLE_AND_BODY" => LayoutReferencePredefinedLayout::TitleAndBody,
                "TITLE_AND_TWO_COLUMNS" => LayoutReferencePredefinedLayout::TitleAndTwoColumns,
                "TITLE_ONLY" => LayoutReferencePredefinedLayout::TitleOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LayoutReferencePredefinedLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LayoutReferencePredefinedLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Line {
        #[doc = "The category of the line.\n\nIt matches the `category` specified in CreateLineRequest, and can be updated with\nUpdateLineCategoryRequest."]
        #[serde(
            rename = "lineCategory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_category: ::std::option::Option<crate::schemas::LineLineCategory>,
        #[doc = "The properties of the line."]
        #[serde(
            rename = "lineProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_properties: ::std::option::Option<crate::schemas::LineProperties>,
        #[doc = "The type of the line."]
        #[serde(
            rename = "lineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_type: ::std::option::Option<crate::schemas::LineLineType>,
    }
    impl ::google_field_selector::FieldSelector for Line {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Line {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LineLineCategory {
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
    }
    impl LineLineCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                LineLineCategory::Bent => "BENT",
                LineLineCategory::Curved => "CURVED",
                LineLineCategory::LineCategoryUnspecified => "LINE_CATEGORY_UNSPECIFIED",
                LineLineCategory::Straight => "STRAIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LineLineCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LineLineCategory {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LineLineCategory, ()> {
            Ok(match s {
                "BENT" => LineLineCategory::Bent,
                "CURVED" => LineLineCategory::Curved,
                "LINE_CATEGORY_UNSPECIFIED" => LineLineCategory::LineCategoryUnspecified,
                "STRAIGHT" => LineLineCategory::Straight,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LineLineCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LineLineCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LineLineCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BENT" => LineLineCategory::Bent,
                "CURVED" => LineLineCategory::Curved,
                "LINE_CATEGORY_UNSPECIFIED" => LineLineCategory::LineCategoryUnspecified,
                "STRAIGHT" => LineLineCategory::Straight,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LineLineCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LineLineCategory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LineLineType {
        #[doc = "Bent connector 2 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector2'."]
        BentConnector2,
        #[doc = "Bent connector 3 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector3'."]
        BentConnector3,
        #[doc = "Bent connector 4 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector4'."]
        BentConnector4,
        #[doc = "Bent connector 5 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector5'."]
        BentConnector5,
        #[doc = "Curved connector 2 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector2'."]
        CurvedConnector2,
        #[doc = "Curved connector 3 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector3'."]
        CurvedConnector3,
        #[doc = "Curved connector 4 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector4'."]
        CurvedConnector4,
        #[doc = "Curved connector 5 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector5'."]
        CurvedConnector5,
        #[doc = "Straight connector 1 form. Corresponds to ECMA-376 ST_ShapeType\n'straightConnector1'."]
        StraightConnector1,
        #[doc = "Straight line. Corresponds to ECMA-376 ST_ShapeType 'line'. This line\ntype is not a connector."]
        StraightLine,
        #[doc = "An unspecified line type."]
        TypeUnspecified,
    }
    impl LineLineType {
        pub fn as_str(self) -> &'static str {
            match self {
                LineLineType::BentConnector2 => "BENT_CONNECTOR_2",
                LineLineType::BentConnector3 => "BENT_CONNECTOR_3",
                LineLineType::BentConnector4 => "BENT_CONNECTOR_4",
                LineLineType::BentConnector5 => "BENT_CONNECTOR_5",
                LineLineType::CurvedConnector2 => "CURVED_CONNECTOR_2",
                LineLineType::CurvedConnector3 => "CURVED_CONNECTOR_3",
                LineLineType::CurvedConnector4 => "CURVED_CONNECTOR_4",
                LineLineType::CurvedConnector5 => "CURVED_CONNECTOR_5",
                LineLineType::StraightConnector1 => "STRAIGHT_CONNECTOR_1",
                LineLineType::StraightLine => "STRAIGHT_LINE",
                LineLineType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LineLineType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LineLineType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LineLineType, ()> {
            Ok(match s {
                "BENT_CONNECTOR_2" => LineLineType::BentConnector2,
                "BENT_CONNECTOR_3" => LineLineType::BentConnector3,
                "BENT_CONNECTOR_4" => LineLineType::BentConnector4,
                "BENT_CONNECTOR_5" => LineLineType::BentConnector5,
                "CURVED_CONNECTOR_2" => LineLineType::CurvedConnector2,
                "CURVED_CONNECTOR_3" => LineLineType::CurvedConnector3,
                "CURVED_CONNECTOR_4" => LineLineType::CurvedConnector4,
                "CURVED_CONNECTOR_5" => LineLineType::CurvedConnector5,
                "STRAIGHT_CONNECTOR_1" => LineLineType::StraightConnector1,
                "STRAIGHT_LINE" => LineLineType::StraightLine,
                "TYPE_UNSPECIFIED" => LineLineType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LineLineType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LineLineType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LineLineType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BENT_CONNECTOR_2" => LineLineType::BentConnector2,
                "BENT_CONNECTOR_3" => LineLineType::BentConnector3,
                "BENT_CONNECTOR_4" => LineLineType::BentConnector4,
                "BENT_CONNECTOR_5" => LineLineType::BentConnector5,
                "CURVED_CONNECTOR_2" => LineLineType::CurvedConnector2,
                "CURVED_CONNECTOR_3" => LineLineType::CurvedConnector3,
                "CURVED_CONNECTOR_4" => LineLineType::CurvedConnector4,
                "CURVED_CONNECTOR_5" => LineLineType::CurvedConnector5,
                "STRAIGHT_CONNECTOR_1" => LineLineType::StraightConnector1,
                "STRAIGHT_LINE" => LineLineType::StraightLine,
                "TYPE_UNSPECIFIED" => LineLineType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LineLineType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LineLineType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LineConnection {
        #[doc = "The object ID of the connected page element.\n\nSome page elements, such as groups, tables, and lines\ndo not have connection sites and therefore cannot be connected to a\nconnector line."]
        #[serde(
            rename = "connectedObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connected_object_id: ::std::option::Option<String>,
        #[doc = "The index of the connection site on the connected page element.\n\nIn most cases, it corresponds to the predefined connection site index from\nthe ECMA-376 standard. More information on those connection sites can be\nfound in the description of the \"cnx\" attribute in section 20.1.9.9 and\nAnnex H. \"Predefined DrawingML Shape and Text Geometries\" of \"Office Open\nXML File Formats-Fundamentals and Markup Language Reference\", part 1 of\n[ECMA-376 5th edition]\n(http://www.ecma-international.org/publications/standards/Ecma-376.htm).\n\nThe position of each connection site can also be viewed from Slides editor."]
        #[serde(
            rename = "connectionSiteIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connection_site_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for LineConnection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LineConnection {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LineFill {
        #[doc = "Solid color fill."]
        #[serde(
            rename = "solidFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub solid_fill: ::std::option::Option<crate::schemas::SolidFill>,
    }
    impl ::google_field_selector::FieldSelector for LineFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LineFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LineProperties {
        #[doc = "The dash style of the line."]
        #[serde(
            rename = "dashStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dash_style: ::std::option::Option<crate::schemas::LinePropertiesDashStyle>,
        #[doc = "The style of the arrow at the end of the line."]
        #[serde(
            rename = "endArrow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_arrow: ::std::option::Option<crate::schemas::LinePropertiesEndArrow>,
        #[doc = "The connection at the end of the line. If unset, there is no connection.\n\nOnly lines with a Type indicating it is\na \"connector\" can have an `end_connection`."]
        #[serde(
            rename = "endConnection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_connection: ::std::option::Option<crate::schemas::LineConnection>,
        #[doc = "The fill of the line. The default line fill matches the defaults for new\nlines created in the Slides editor."]
        #[serde(
            rename = "lineFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_fill: ::std::option::Option<crate::schemas::LineFill>,
        #[doc = "The hyperlink destination of the line. If unset, there is no link."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<crate::schemas::Link>,
        #[doc = "The style of the arrow at the beginning of the line."]
        #[serde(
            rename = "startArrow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_arrow: ::std::option::Option<crate::schemas::LinePropertiesStartArrow>,
        #[doc = "The connection at the beginning of the line. If unset, there is no\nconnection.\n\nOnly lines with a Type indicating it is\na \"connector\" can have a `start_connection`."]
        #[serde(
            rename = "startConnection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_connection: ::std::option::Option<crate::schemas::LineConnection>,
        #[doc = "The thickness of the line."]
        #[serde(
            rename = "weight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weight: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for LineProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LineProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinePropertiesDashStyle {
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'dashDot'."]
        DashDot,
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'lgDash'."]
        LongDash,
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376\nST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'.\nThis is the default dash style."]
        Solid,
    }
    impl LinePropertiesDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                LinePropertiesDashStyle::Dash => "DASH",
                LinePropertiesDashStyle::DashDot => "DASH_DOT",
                LinePropertiesDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                LinePropertiesDashStyle::Dot => "DOT",
                LinePropertiesDashStyle::LongDash => "LONG_DASH",
                LinePropertiesDashStyle::LongDashDot => "LONG_DASH_DOT",
                LinePropertiesDashStyle::Solid => "SOLID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LinePropertiesDashStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LinePropertiesDashStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LinePropertiesDashStyle, ()> {
            Ok(match s {
                "DASH" => LinePropertiesDashStyle::Dash,
                "DASH_DOT" => LinePropertiesDashStyle::DashDot,
                "DASH_STYLE_UNSPECIFIED" => LinePropertiesDashStyle::DashStyleUnspecified,
                "DOT" => LinePropertiesDashStyle::Dot,
                "LONG_DASH" => LinePropertiesDashStyle::LongDash,
                "LONG_DASH_DOT" => LinePropertiesDashStyle::LongDashDot,
                "SOLID" => LinePropertiesDashStyle::Solid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LinePropertiesDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinePropertiesDashStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinePropertiesDashStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH" => LinePropertiesDashStyle::Dash,
                "DASH_DOT" => LinePropertiesDashStyle::DashDot,
                "DASH_STYLE_UNSPECIFIED" => LinePropertiesDashStyle::DashStyleUnspecified,
                "DOT" => LinePropertiesDashStyle::Dot,
                "LONG_DASH" => LinePropertiesDashStyle::LongDash,
                "LONG_DASH_DOT" => LinePropertiesDashStyle::LongDashDot,
                "SOLID" => LinePropertiesDashStyle::Solid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LinePropertiesDashStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LinePropertiesDashStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinePropertiesEndArrow {
        #[doc = "An unspecified arrow style."]
        ArrowStyleUnspecified,
        #[doc = "Filled arrow. Corresponds to ECMA-376 ST_LineEndType value 'triangle'."]
        FillArrow,
        #[doc = "Filled circle. Corresponds to ECMA-376 ST_LineEndType value 'oval'."]
        FillCircle,
        #[doc = "Filled diamond. Corresponds to ECMA-376 ST_LineEndType value 'diamond'."]
        FillDiamond,
        #[doc = "Filled square."]
        FillSquare,
        #[doc = "No arrow."]
        None,
        #[doc = "Hollow arrow."]
        OpenArrow,
        #[doc = "Hollow circle."]
        OpenCircle,
        #[doc = "Hollow diamond."]
        OpenDiamond,
        #[doc = "Hollow square."]
        OpenSquare,
        #[doc = "Arrow with notched back. Corresponds to ECMA-376 ST_LineEndType value\n'stealth'."]
        StealthArrow,
    }
    impl LinePropertiesEndArrow {
        pub fn as_str(self) -> &'static str {
            match self {
                LinePropertiesEndArrow::ArrowStyleUnspecified => "ARROW_STYLE_UNSPECIFIED",
                LinePropertiesEndArrow::FillArrow => "FILL_ARROW",
                LinePropertiesEndArrow::FillCircle => "FILL_CIRCLE",
                LinePropertiesEndArrow::FillDiamond => "FILL_DIAMOND",
                LinePropertiesEndArrow::FillSquare => "FILL_SQUARE",
                LinePropertiesEndArrow::None => "NONE",
                LinePropertiesEndArrow::OpenArrow => "OPEN_ARROW",
                LinePropertiesEndArrow::OpenCircle => "OPEN_CIRCLE",
                LinePropertiesEndArrow::OpenDiamond => "OPEN_DIAMOND",
                LinePropertiesEndArrow::OpenSquare => "OPEN_SQUARE",
                LinePropertiesEndArrow::StealthArrow => "STEALTH_ARROW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LinePropertiesEndArrow {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LinePropertiesEndArrow {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LinePropertiesEndArrow, ()> {
            Ok(match s {
                "ARROW_STYLE_UNSPECIFIED" => LinePropertiesEndArrow::ArrowStyleUnspecified,
                "FILL_ARROW" => LinePropertiesEndArrow::FillArrow,
                "FILL_CIRCLE" => LinePropertiesEndArrow::FillCircle,
                "FILL_DIAMOND" => LinePropertiesEndArrow::FillDiamond,
                "FILL_SQUARE" => LinePropertiesEndArrow::FillSquare,
                "NONE" => LinePropertiesEndArrow::None,
                "OPEN_ARROW" => LinePropertiesEndArrow::OpenArrow,
                "OPEN_CIRCLE" => LinePropertiesEndArrow::OpenCircle,
                "OPEN_DIAMOND" => LinePropertiesEndArrow::OpenDiamond,
                "OPEN_SQUARE" => LinePropertiesEndArrow::OpenSquare,
                "STEALTH_ARROW" => LinePropertiesEndArrow::StealthArrow,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LinePropertiesEndArrow {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinePropertiesEndArrow {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinePropertiesEndArrow {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARROW_STYLE_UNSPECIFIED" => LinePropertiesEndArrow::ArrowStyleUnspecified,
                "FILL_ARROW" => LinePropertiesEndArrow::FillArrow,
                "FILL_CIRCLE" => LinePropertiesEndArrow::FillCircle,
                "FILL_DIAMOND" => LinePropertiesEndArrow::FillDiamond,
                "FILL_SQUARE" => LinePropertiesEndArrow::FillSquare,
                "NONE" => LinePropertiesEndArrow::None,
                "OPEN_ARROW" => LinePropertiesEndArrow::OpenArrow,
                "OPEN_CIRCLE" => LinePropertiesEndArrow::OpenCircle,
                "OPEN_DIAMOND" => LinePropertiesEndArrow::OpenDiamond,
                "OPEN_SQUARE" => LinePropertiesEndArrow::OpenSquare,
                "STEALTH_ARROW" => LinePropertiesEndArrow::StealthArrow,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LinePropertiesEndArrow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LinePropertiesEndArrow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinePropertiesStartArrow {
        #[doc = "An unspecified arrow style."]
        ArrowStyleUnspecified,
        #[doc = "Filled arrow. Corresponds to ECMA-376 ST_LineEndType value 'triangle'."]
        FillArrow,
        #[doc = "Filled circle. Corresponds to ECMA-376 ST_LineEndType value 'oval'."]
        FillCircle,
        #[doc = "Filled diamond. Corresponds to ECMA-376 ST_LineEndType value 'diamond'."]
        FillDiamond,
        #[doc = "Filled square."]
        FillSquare,
        #[doc = "No arrow."]
        None,
        #[doc = "Hollow arrow."]
        OpenArrow,
        #[doc = "Hollow circle."]
        OpenCircle,
        #[doc = "Hollow diamond."]
        OpenDiamond,
        #[doc = "Hollow square."]
        OpenSquare,
        #[doc = "Arrow with notched back. Corresponds to ECMA-376 ST_LineEndType value\n'stealth'."]
        StealthArrow,
    }
    impl LinePropertiesStartArrow {
        pub fn as_str(self) -> &'static str {
            match self {
                LinePropertiesStartArrow::ArrowStyleUnspecified => "ARROW_STYLE_UNSPECIFIED",
                LinePropertiesStartArrow::FillArrow => "FILL_ARROW",
                LinePropertiesStartArrow::FillCircle => "FILL_CIRCLE",
                LinePropertiesStartArrow::FillDiamond => "FILL_DIAMOND",
                LinePropertiesStartArrow::FillSquare => "FILL_SQUARE",
                LinePropertiesStartArrow::None => "NONE",
                LinePropertiesStartArrow::OpenArrow => "OPEN_ARROW",
                LinePropertiesStartArrow::OpenCircle => "OPEN_CIRCLE",
                LinePropertiesStartArrow::OpenDiamond => "OPEN_DIAMOND",
                LinePropertiesStartArrow::OpenSquare => "OPEN_SQUARE",
                LinePropertiesStartArrow::StealthArrow => "STEALTH_ARROW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LinePropertiesStartArrow {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LinePropertiesStartArrow {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LinePropertiesStartArrow, ()> {
            Ok(match s {
                "ARROW_STYLE_UNSPECIFIED" => LinePropertiesStartArrow::ArrowStyleUnspecified,
                "FILL_ARROW" => LinePropertiesStartArrow::FillArrow,
                "FILL_CIRCLE" => LinePropertiesStartArrow::FillCircle,
                "FILL_DIAMOND" => LinePropertiesStartArrow::FillDiamond,
                "FILL_SQUARE" => LinePropertiesStartArrow::FillSquare,
                "NONE" => LinePropertiesStartArrow::None,
                "OPEN_ARROW" => LinePropertiesStartArrow::OpenArrow,
                "OPEN_CIRCLE" => LinePropertiesStartArrow::OpenCircle,
                "OPEN_DIAMOND" => LinePropertiesStartArrow::OpenDiamond,
                "OPEN_SQUARE" => LinePropertiesStartArrow::OpenSquare,
                "STEALTH_ARROW" => LinePropertiesStartArrow::StealthArrow,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LinePropertiesStartArrow {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinePropertiesStartArrow {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinePropertiesStartArrow {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARROW_STYLE_UNSPECIFIED" => LinePropertiesStartArrow::ArrowStyleUnspecified,
                "FILL_ARROW" => LinePropertiesStartArrow::FillArrow,
                "FILL_CIRCLE" => LinePropertiesStartArrow::FillCircle,
                "FILL_DIAMOND" => LinePropertiesStartArrow::FillDiamond,
                "FILL_SQUARE" => LinePropertiesStartArrow::FillSquare,
                "NONE" => LinePropertiesStartArrow::None,
                "OPEN_ARROW" => LinePropertiesStartArrow::OpenArrow,
                "OPEN_CIRCLE" => LinePropertiesStartArrow::OpenCircle,
                "OPEN_DIAMOND" => LinePropertiesStartArrow::OpenDiamond,
                "OPEN_SQUARE" => LinePropertiesStartArrow::OpenSquare,
                "STEALTH_ARROW" => LinePropertiesStartArrow::StealthArrow,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LinePropertiesStartArrow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LinePropertiesStartArrow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "If set, indicates this is a link to the specific page in this\npresentation with this ID. A page with this ID may not exist."]
        #[serde(
            rename = "pageObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_object_id: ::std::option::Option<String>,
        #[doc = "If set, indicates this is a link to a slide in this presentation,\naddressed by its position."]
        #[serde(
            rename = "relativeLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relative_link: ::std::option::Option<crate::schemas::LinkRelativeLink>,
        #[doc = "If set, indicates this is a link to the slide at this zero-based index\nin the presentation. There may not be a slide at this index."]
        #[serde(
            rename = "slideIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slide_index: ::std::option::Option<i32>,
        #[doc = "If set, indicates this is a link to the external web page at this URL."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
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
    pub enum LinkRelativeLink {
        #[doc = "A link to the first slide in the presentation."]
        FirstSlide,
        #[doc = "A link to the last slide in the presentation."]
        LastSlide,
        #[doc = "A link to the next slide."]
        NextSlide,
        #[doc = "A link to the previous slide."]
        PreviousSlide,
        #[doc = "An unspecified relative slide link."]
        RelativeSlideLinkUnspecified,
    }
    impl LinkRelativeLink {
        pub fn as_str(self) -> &'static str {
            match self {
                LinkRelativeLink::FirstSlide => "FIRST_SLIDE",
                LinkRelativeLink::LastSlide => "LAST_SLIDE",
                LinkRelativeLink::NextSlide => "NEXT_SLIDE",
                LinkRelativeLink::PreviousSlide => "PREVIOUS_SLIDE",
                LinkRelativeLink::RelativeSlideLinkUnspecified => "RELATIVE_SLIDE_LINK_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LinkRelativeLink {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LinkRelativeLink {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LinkRelativeLink, ()> {
            Ok(match s {
                "FIRST_SLIDE" => LinkRelativeLink::FirstSlide,
                "LAST_SLIDE" => LinkRelativeLink::LastSlide,
                "NEXT_SLIDE" => LinkRelativeLink::NextSlide,
                "PREVIOUS_SLIDE" => LinkRelativeLink::PreviousSlide,
                "RELATIVE_SLIDE_LINK_UNSPECIFIED" => LinkRelativeLink::RelativeSlideLinkUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LinkRelativeLink {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinkRelativeLink {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinkRelativeLink {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FIRST_SLIDE" => LinkRelativeLink::FirstSlide,
                "LAST_SLIDE" => LinkRelativeLink::LastSlide,
                "NEXT_SLIDE" => LinkRelativeLink::NextSlide,
                "PREVIOUS_SLIDE" => LinkRelativeLink::PreviousSlide,
                "RELATIVE_SLIDE_LINK_UNSPECIFIED" => LinkRelativeLink::RelativeSlideLinkUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LinkRelativeLink {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LinkRelativeLink {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct List {
        #[doc = "The ID of the list."]
        #[serde(
            rename = "listId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_id: ::std::option::Option<String>,
        #[doc = "A map of nesting levels to the properties of bullets at the associated\nlevel. A list has at most nine levels of nesting, so the possible values\nfor the keys of this map are 0 through 8, inclusive."]
        #[serde(
            rename = "nestingLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nesting_level: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::NestingLevel>,
        >,
    }
    impl ::google_field_selector::FieldSelector for List {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for List {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MasterProperties {
        #[doc = "The human-readable name of the master."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MasterProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MasterProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MergeTableCellsRequest {
        #[doc = "The object ID of the table."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The table range specifying which cells of the table to merge.\n\nAny text in the cells being merged will be concatenated and stored in the\nupper-left (\"head\") cell of the range. If the range is non-rectangular\n(which can occur in some cases where the range covers cells that are\nalready merged), a 400 bad request error is returned."]
        #[serde(
            rename = "tableRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_range: ::std::option::Option<crate::schemas::TableRange>,
    }
    impl ::google_field_selector::FieldSelector for MergeTableCellsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MergeTableCellsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NestingLevel {
        #[doc = "The style of a bullet at this level of nesting."]
        #[serde(
            rename = "bulletStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for NestingLevel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NestingLevel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NotesProperties {
        #[doc = "The object ID of the shape on this notes page that contains the speaker\nnotes for the corresponding slide.\nThe actual shape may not always exist on the notes page. Inserting text\nusing this object ID will automatically create the shape. In this case, the\nactual shape may have different object ID. The `GetPresentation` or\n`GetPage` action will always return the latest object ID."]
        #[serde(
            rename = "speakerNotesObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speaker_notes_object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NotesProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NotesProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OpaqueColor {
        #[doc = "An opaque RGB color."]
        #[serde(
            rename = "rgbColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rgb_color: ::std::option::Option<crate::schemas::RgbColor>,
        #[doc = "An opaque theme color."]
        #[serde(
            rename = "themeColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub theme_color: ::std::option::Option<crate::schemas::OpaqueColorThemeColor>,
    }
    impl ::google_field_selector::FieldSelector for OpaqueColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OpaqueColor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OpaqueColorThemeColor {
        #[doc = "Represents the first accent color."]
        Accent1,
        #[doc = "Represents the second accent color."]
        Accent2,
        #[doc = "Represents the third accent color."]
        Accent3,
        #[doc = "Represents the fourth accent color."]
        Accent4,
        #[doc = "Represents the fifth accent color."]
        Accent5,
        #[doc = "Represents the sixth accent color."]
        Accent6,
        #[doc = "Represents the first background color."]
        Background1,
        #[doc = "Represents the second background color."]
        Background2,
        #[doc = "Represents the first dark color."]
        Dark1,
        #[doc = "Represents the second dark color."]
        Dark2,
        #[doc = "Represents the color to use for visited hyperlinks."]
        FollowedHyperlink,
        #[doc = "Represents the color to use for hyperlinks."]
        Hyperlink,
        #[doc = "Represents the first light color."]
        Light1,
        #[doc = "Represents the second light color."]
        Light2,
        #[doc = "Represents the first text color."]
        Text1,
        #[doc = "Represents the second text color."]
        Text2,
        #[doc = "Unspecified theme color. This value should not be used."]
        ThemeColorTypeUnspecified,
    }
    impl OpaqueColorThemeColor {
        pub fn as_str(self) -> &'static str {
            match self {
                OpaqueColorThemeColor::Accent1 => "ACCENT1",
                OpaqueColorThemeColor::Accent2 => "ACCENT2",
                OpaqueColorThemeColor::Accent3 => "ACCENT3",
                OpaqueColorThemeColor::Accent4 => "ACCENT4",
                OpaqueColorThemeColor::Accent5 => "ACCENT5",
                OpaqueColorThemeColor::Accent6 => "ACCENT6",
                OpaqueColorThemeColor::Background1 => "BACKGROUND1",
                OpaqueColorThemeColor::Background2 => "BACKGROUND2",
                OpaqueColorThemeColor::Dark1 => "DARK1",
                OpaqueColorThemeColor::Dark2 => "DARK2",
                OpaqueColorThemeColor::FollowedHyperlink => "FOLLOWED_HYPERLINK",
                OpaqueColorThemeColor::Hyperlink => "HYPERLINK",
                OpaqueColorThemeColor::Light1 => "LIGHT1",
                OpaqueColorThemeColor::Light2 => "LIGHT2",
                OpaqueColorThemeColor::Text1 => "TEXT1",
                OpaqueColorThemeColor::Text2 => "TEXT2",
                OpaqueColorThemeColor::ThemeColorTypeUnspecified => "THEME_COLOR_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OpaqueColorThemeColor {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OpaqueColorThemeColor {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OpaqueColorThemeColor, ()> {
            Ok(match s {
                "ACCENT1" => OpaqueColorThemeColor::Accent1,
                "ACCENT2" => OpaqueColorThemeColor::Accent2,
                "ACCENT3" => OpaqueColorThemeColor::Accent3,
                "ACCENT4" => OpaqueColorThemeColor::Accent4,
                "ACCENT5" => OpaqueColorThemeColor::Accent5,
                "ACCENT6" => OpaqueColorThemeColor::Accent6,
                "BACKGROUND1" => OpaqueColorThemeColor::Background1,
                "BACKGROUND2" => OpaqueColorThemeColor::Background2,
                "DARK1" => OpaqueColorThemeColor::Dark1,
                "DARK2" => OpaqueColorThemeColor::Dark2,
                "FOLLOWED_HYPERLINK" => OpaqueColorThemeColor::FollowedHyperlink,
                "HYPERLINK" => OpaqueColorThemeColor::Hyperlink,
                "LIGHT1" => OpaqueColorThemeColor::Light1,
                "LIGHT2" => OpaqueColorThemeColor::Light2,
                "TEXT1" => OpaqueColorThemeColor::Text1,
                "TEXT2" => OpaqueColorThemeColor::Text2,
                "THEME_COLOR_TYPE_UNSPECIFIED" => OpaqueColorThemeColor::ThemeColorTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OpaqueColorThemeColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OpaqueColorThemeColor {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OpaqueColorThemeColor {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCENT1" => OpaqueColorThemeColor::Accent1,
                "ACCENT2" => OpaqueColorThemeColor::Accent2,
                "ACCENT3" => OpaqueColorThemeColor::Accent3,
                "ACCENT4" => OpaqueColorThemeColor::Accent4,
                "ACCENT5" => OpaqueColorThemeColor::Accent5,
                "ACCENT6" => OpaqueColorThemeColor::Accent6,
                "BACKGROUND1" => OpaqueColorThemeColor::Background1,
                "BACKGROUND2" => OpaqueColorThemeColor::Background2,
                "DARK1" => OpaqueColorThemeColor::Dark1,
                "DARK2" => OpaqueColorThemeColor::Dark2,
                "FOLLOWED_HYPERLINK" => OpaqueColorThemeColor::FollowedHyperlink,
                "HYPERLINK" => OpaqueColorThemeColor::Hyperlink,
                "LIGHT1" => OpaqueColorThemeColor::Light1,
                "LIGHT2" => OpaqueColorThemeColor::Light2,
                "TEXT1" => OpaqueColorThemeColor::Text1,
                "TEXT2" => OpaqueColorThemeColor::Text2,
                "THEME_COLOR_TYPE_UNSPECIFIED" => OpaqueColorThemeColor::ThemeColorTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OpaqueColorThemeColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OpaqueColorThemeColor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OptionalColor {
        #[doc = "If set, this will be used as an opaque color. If unset, this represents\na transparent color."]
        #[serde(
            rename = "opaqueColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub opaque_color: ::std::option::Option<crate::schemas::OpaqueColor>,
    }
    impl ::google_field_selector::FieldSelector for OptionalColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OptionalColor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Outline {
        #[doc = "The dash style of the outline."]
        #[serde(
            rename = "dashStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dash_style: ::std::option::Option<crate::schemas::OutlineDashStyle>,
        #[doc = "The fill of the outline."]
        #[serde(
            rename = "outlineFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outline_fill: ::std::option::Option<crate::schemas::OutlineFill>,
        #[doc = "The outline property state.\n\nUpdating the outline on a page element will implicitly update this field\nto `RENDERED`, unless another value is specified in the same request. To\nhave no outline on a page element, set this field to `NOT_RENDERED`. In\nthis case, any other outline fields set in the same request will be\nignored."]
        #[serde(
            rename = "propertyState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_state: ::std::option::Option<crate::schemas::OutlinePropertyState>,
        #[doc = "The thickness of the outline."]
        #[serde(
            rename = "weight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weight: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for Outline {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Outline {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OutlineDashStyle {
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'dashDot'."]
        DashDot,
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'lgDash'."]
        LongDash,
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376\nST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'.\nThis is the default dash style."]
        Solid,
    }
    impl OutlineDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                OutlineDashStyle::Dash => "DASH",
                OutlineDashStyle::DashDot => "DASH_DOT",
                OutlineDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                OutlineDashStyle::Dot => "DOT",
                OutlineDashStyle::LongDash => "LONG_DASH",
                OutlineDashStyle::LongDashDot => "LONG_DASH_DOT",
                OutlineDashStyle::Solid => "SOLID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OutlineDashStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OutlineDashStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OutlineDashStyle, ()> {
            Ok(match s {
                "DASH" => OutlineDashStyle::Dash,
                "DASH_DOT" => OutlineDashStyle::DashDot,
                "DASH_STYLE_UNSPECIFIED" => OutlineDashStyle::DashStyleUnspecified,
                "DOT" => OutlineDashStyle::Dot,
                "LONG_DASH" => OutlineDashStyle::LongDash,
                "LONG_DASH_DOT" => OutlineDashStyle::LongDashDot,
                "SOLID" => OutlineDashStyle::Solid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OutlineDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OutlineDashStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OutlineDashStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH" => OutlineDashStyle::Dash,
                "DASH_DOT" => OutlineDashStyle::DashDot,
                "DASH_STYLE_UNSPECIFIED" => OutlineDashStyle::DashStyleUnspecified,
                "DOT" => OutlineDashStyle::Dot,
                "LONG_DASH" => OutlineDashStyle::LongDash,
                "LONG_DASH_DOT" => OutlineDashStyle::LongDashDot,
                "SOLID" => OutlineDashStyle::Solid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OutlineDashStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OutlineDashStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OutlinePropertyState {
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
    }
    impl OutlinePropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                OutlinePropertyState::Inherit => "INHERIT",
                OutlinePropertyState::NotRendered => "NOT_RENDERED",
                OutlinePropertyState::Rendered => "RENDERED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OutlinePropertyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OutlinePropertyState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OutlinePropertyState, ()> {
            Ok(match s {
                "INHERIT" => OutlinePropertyState::Inherit,
                "NOT_RENDERED" => OutlinePropertyState::NotRendered,
                "RENDERED" => OutlinePropertyState::Rendered,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OutlinePropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OutlinePropertyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OutlinePropertyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INHERIT" => OutlinePropertyState::Inherit,
                "NOT_RENDERED" => OutlinePropertyState::NotRendered,
                "RENDERED" => OutlinePropertyState::Rendered,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OutlinePropertyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OutlinePropertyState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OutlineFill {
        #[doc = "Solid color fill."]
        #[serde(
            rename = "solidFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub solid_fill: ::std::option::Option<crate::schemas::SolidFill>,
    }
    impl ::google_field_selector::FieldSelector for OutlineFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OutlineFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Page {
        #[doc = "Layout specific properties. Only set if page_type = LAYOUT."]
        #[serde(
            rename = "layoutProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_properties: ::std::option::Option<crate::schemas::LayoutProperties>,
        #[doc = "Master specific properties. Only set if page_type = MASTER."]
        #[serde(
            rename = "masterProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_properties: ::std::option::Option<crate::schemas::MasterProperties>,
        #[doc = "Notes specific properties. Only set if page_type = NOTES."]
        #[serde(
            rename = "notesProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes_properties: ::std::option::Option<crate::schemas::NotesProperties>,
        #[doc = "The object ID for this page. Object IDs used by\nPage and\nPageElement share the same namespace."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The page elements rendered on the page."]
        #[serde(
            rename = "pageElements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_elements: ::std::option::Option<Vec<crate::schemas::PageElement>>,
        #[doc = "The properties of the page."]
        #[serde(
            rename = "pageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_properties: ::std::option::Option<crate::schemas::PageProperties>,
        #[doc = "The type of the page."]
        #[serde(
            rename = "pageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_type: ::std::option::Option<crate::schemas::PagePageType>,
        #[doc = "The revision ID of the presentation containing this page. Can be used in\nupdate requests to assert that the presentation revision hasn't changed\nsince the last read operation. Only populated if the user has edit access\nto the presentation.\n\nThe format of the revision ID may change over time, so it should be treated\nopaquely. A returned revision ID is only guaranteed to be valid for 24\nhours after it has been returned and cannot be shared across users. If the\nrevision ID is unchanged between calls, then the presentation has not\nchanged. Conversely, a changed ID (for the same presentation and user)\nusually means the presentation has been updated; however, a changed ID can\nalso be due to internal factors such as ID format changes."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Slide specific properties. Only set if page_type = SLIDE."]
        #[serde(
            rename = "slideProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slide_properties: ::std::option::Option<Box<crate::schemas::SlideProperties>>,
    }
    impl ::google_field_selector::FieldSelector for Page {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Page {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PagePageType {
        #[doc = "A layout page."]
        Layout,
        #[doc = "A master slide page."]
        Master,
        #[doc = "A notes page."]
        Notes,
        #[doc = "A notes master page."]
        NotesMaster,
        #[doc = "A slide page."]
        Slide,
    }
    impl PagePageType {
        pub fn as_str(self) -> &'static str {
            match self {
                PagePageType::Layout => "LAYOUT",
                PagePageType::Master => "MASTER",
                PagePageType::Notes => "NOTES",
                PagePageType::NotesMaster => "NOTES_MASTER",
                PagePageType::Slide => "SLIDE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PagePageType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PagePageType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PagePageType, ()> {
            Ok(match s {
                "LAYOUT" => PagePageType::Layout,
                "MASTER" => PagePageType::Master,
                "NOTES" => PagePageType::Notes,
                "NOTES_MASTER" => PagePageType::NotesMaster,
                "SLIDE" => PagePageType::Slide,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PagePageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PagePageType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PagePageType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LAYOUT" => PagePageType::Layout,
                "MASTER" => PagePageType::Master,
                "NOTES" => PagePageType::Notes,
                "NOTES_MASTER" => PagePageType::NotesMaster,
                "SLIDE" => PagePageType::Slide,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PagePageType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PagePageType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageBackgroundFill {
        #[doc = "The background fill property state.\n\nUpdating the fill on a page will implicitly update this field to\n`RENDERED`, unless another value is specified in the same request. To\nhave no fill on a page, set this field to `NOT_RENDERED`. In this case,\nany other fill fields set in the same request will be ignored."]
        #[serde(
            rename = "propertyState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_state: ::std::option::Option<crate::schemas::PageBackgroundFillPropertyState>,
        #[doc = "Solid color fill."]
        #[serde(
            rename = "solidFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub solid_fill: ::std::option::Option<crate::schemas::SolidFill>,
        #[doc = "Stretched picture fill."]
        #[serde(
            rename = "stretchedPictureFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stretched_picture_fill: ::std::option::Option<crate::schemas::StretchedPictureFill>,
    }
    impl ::google_field_selector::FieldSelector for PageBackgroundFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PageBackgroundFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PageBackgroundFillPropertyState {
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
    }
    impl PageBackgroundFillPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                PageBackgroundFillPropertyState::Inherit => "INHERIT",
                PageBackgroundFillPropertyState::NotRendered => "NOT_RENDERED",
                PageBackgroundFillPropertyState::Rendered => "RENDERED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PageBackgroundFillPropertyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PageBackgroundFillPropertyState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PageBackgroundFillPropertyState, ()> {
            Ok(match s {
                "INHERIT" => PageBackgroundFillPropertyState::Inherit,
                "NOT_RENDERED" => PageBackgroundFillPropertyState::NotRendered,
                "RENDERED" => PageBackgroundFillPropertyState::Rendered,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PageBackgroundFillPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PageBackgroundFillPropertyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PageBackgroundFillPropertyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INHERIT" => PageBackgroundFillPropertyState::Inherit,
                "NOT_RENDERED" => PageBackgroundFillPropertyState::NotRendered,
                "RENDERED" => PageBackgroundFillPropertyState::Rendered,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PageBackgroundFillPropertyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PageBackgroundFillPropertyState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageElement {
        #[doc = "The description of the page element. Combined with title to display alt\ntext."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "A collection of page elements joined as a single unit."]
        #[serde(
            rename = "elementGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub element_group: ::std::option::Option<crate::schemas::Group>,
        #[doc = "An image page element."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::Image>,
        #[doc = "A line page element."]
        #[serde(
            rename = "line",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line: ::std::option::Option<crate::schemas::Line>,
        #[doc = "The object ID for this page element. Object IDs used by\ngoogle.apps.slides.v1.Page and\ngoogle.apps.slides.v1.PageElement share the same namespace."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "A generic shape."]
        #[serde(
            rename = "shape",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shape: ::std::option::Option<crate::schemas::Shape>,
        #[doc = "A linked chart embedded from Google Sheets. Unlinked charts are\nrepresented as images."]
        #[serde(
            rename = "sheetsChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheets_chart: ::std::option::Option<crate::schemas::SheetsChart>,
        #[doc = "The size of the page element."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<crate::schemas::Size>,
        #[doc = "A table page element."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<crate::schemas::Table>,
        #[doc = "The title of the page element. Combined with description to display alt\ntext."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The transform of the page element.\n\nThe visual appearance of the page element is determined by its absolute\ntransform. To compute the absolute transform, preconcatenate a page\nelement's transform with the transforms of all of its parent groups. If the\npage element is not in a group, its absolute transform is the same as the\nvalue in this field.\n\nThe initial transform for the newly created Group is always the identity transform."]
        #[serde(
            rename = "transform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transform: ::std::option::Option<crate::schemas::AffineTransform>,
        #[doc = "A video page element."]
        #[serde(
            rename = "video",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video: ::std::option::Option<crate::schemas::Video>,
        #[doc = "A word art page element."]
        #[serde(
            rename = "wordArt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub word_art: ::std::option::Option<crate::schemas::WordArt>,
    }
    impl ::google_field_selector::FieldSelector for PageElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PageElement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageElementProperties {
        #[doc = "The object ID of the page where the element is located."]
        #[serde(
            rename = "pageObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_object_id: ::std::option::Option<String>,
        #[doc = "The size of the element."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<crate::schemas::Size>,
        #[doc = "The transform for the element."]
        #[serde(
            rename = "transform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transform: ::std::option::Option<crate::schemas::AffineTransform>,
    }
    impl ::google_field_selector::FieldSelector for PageElementProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PageElementProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageProperties {
        #[doc = "The color scheme of the page. If unset, the color scheme is inherited from\na parent page. If the page has no parent, the color scheme uses a default\nSlides color scheme, matching the defaults in the Slides editor.\n\nOnly the concrete colors of the first 12 ThemeColorTypes are editable. In addition, only\nthe color scheme on `Master` pages can be updated. To update the field, a\ncolor scheme containing mappings from all the first 12 ThemeColorTypes to\ntheir concrete colors must be provided. Colors for the remaining\nThemeColorTypes will be ignored."]
        #[serde(
            rename = "colorScheme",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_scheme: ::std::option::Option<crate::schemas::ColorScheme>,
        #[doc = "The background fill of the page. If unset, the background fill is inherited\nfrom a parent page if it exists. If the page has no parent, then the\nbackground fill defaults to the corresponding fill in the Slides editor."]
        #[serde(
            rename = "pageBackgroundFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_background_fill: ::std::option::Option<crate::schemas::PageBackgroundFill>,
    }
    impl ::google_field_selector::FieldSelector for PageProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PageProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ParagraphMarker {
        #[doc = "The bullet for this paragraph. If not present, the paragraph does not\nbelong to a list."]
        #[serde(
            rename = "bullet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet: ::std::option::Option<crate::schemas::Bullet>,
        #[doc = "The paragraph's style"]
        #[serde(
            rename = "style",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub style: ::std::option::Option<crate::schemas::ParagraphStyle>,
    }
    impl ::google_field_selector::FieldSelector for ParagraphMarker {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphMarker {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ParagraphStyle {
        #[doc = "The text alignment for this paragraph."]
        #[serde(
            rename = "alignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alignment: ::std::option::Option<crate::schemas::ParagraphStyleAlignment>,
        #[doc = "The text direction of this paragraph. If unset, the value defaults to\nLEFT_TO_RIGHT since\ntext direction is not inherited."]
        #[serde(
            rename = "direction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direction: ::std::option::Option<crate::schemas::ParagraphStyleDirection>,
        #[doc = "The amount indentation for the paragraph on the side that corresponds to\nthe end of the text, based on the current text direction. If unset, the\nvalue is inherited from the parent."]
        #[serde(
            rename = "indentEnd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_end: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of indentation for the start of the first line of the paragraph.\nIf unset, the value is inherited from the parent."]
        #[serde(
            rename = "indentFirstLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_first_line: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount indentation for the paragraph on the side that corresponds to\nthe start of the text, based on the current text direction. If unset, the\nvalue is inherited from the parent."]
        #[serde(
            rename = "indentStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_start: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of space between lines, as a percentage of normal, where normal\nis represented as 100.0. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "lineSpacing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_spacing: ::std::option::Option<f32>,
        #[doc = "The amount of extra space above the paragraph. If unset, the value is\ninherited from the parent."]
        #[serde(
            rename = "spaceAbove",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_above: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of extra space below the paragraph. If unset, the value is\ninherited from the parent."]
        #[serde(
            rename = "spaceBelow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_below: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The spacing mode for the paragraph."]
        #[serde(
            rename = "spacingMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spacing_mode: ::std::option::Option<crate::schemas::ParagraphStyleSpacingMode>,
    }
    impl ::google_field_selector::FieldSelector for ParagraphStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParagraphStyleAlignment {
        #[doc = "The paragraph alignment is inherited from the parent."]
        AlignmentUnspecified,
        #[doc = "The paragraph is centered."]
        Center,
        #[doc = "The paragraph is aligned to the end of the line. Right-aligned for\nLTR text, left-aligned otherwise."]
        End,
        #[doc = "The paragraph is justified."]
        Justified,
        #[doc = "The paragraph is aligned to the start of the line. Left-aligned for\nLTR text, right-aligned otherwise."]
        Start,
    }
    impl ParagraphStyleAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleAlignment::AlignmentUnspecified => "ALIGNMENT_UNSPECIFIED",
                ParagraphStyleAlignment::Center => "CENTER",
                ParagraphStyleAlignment::End => "END",
                ParagraphStyleAlignment::Justified => "JUSTIFIED",
                ParagraphStyleAlignment::Start => "START",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ParagraphStyleAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParagraphStyleAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParagraphStyleAlignment, ()> {
            Ok(match s {
                "ALIGNMENT_UNSPECIFIED" => ParagraphStyleAlignment::AlignmentUnspecified,
                "CENTER" => ParagraphStyleAlignment::Center,
                "END" => ParagraphStyleAlignment::End,
                "JUSTIFIED" => ParagraphStyleAlignment::Justified,
                "START" => ParagraphStyleAlignment::Start,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ParagraphStyleAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphStyleAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphStyleAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALIGNMENT_UNSPECIFIED" => ParagraphStyleAlignment::AlignmentUnspecified,
                "CENTER" => ParagraphStyleAlignment::Center,
                "END" => ParagraphStyleAlignment::End,
                "JUSTIFIED" => ParagraphStyleAlignment::Justified,
                "START" => ParagraphStyleAlignment::Start,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ParagraphStyleAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphStyleAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParagraphStyleDirection {
        #[doc = "The text goes from left to right."]
        LeftToRight,
        #[doc = "The text goes from right to left."]
        RightToLeft,
        #[doc = "The text direction is inherited from the parent."]
        TextDirectionUnspecified,
    }
    impl ParagraphStyleDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleDirection::LeftToRight => "LEFT_TO_RIGHT",
                ParagraphStyleDirection::RightToLeft => "RIGHT_TO_LEFT",
                ParagraphStyleDirection::TextDirectionUnspecified => "TEXT_DIRECTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ParagraphStyleDirection {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParagraphStyleDirection {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParagraphStyleDirection, ()> {
            Ok(match s {
                "LEFT_TO_RIGHT" => ParagraphStyleDirection::LeftToRight,
                "RIGHT_TO_LEFT" => ParagraphStyleDirection::RightToLeft,
                "TEXT_DIRECTION_UNSPECIFIED" => ParagraphStyleDirection::TextDirectionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ParagraphStyleDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphStyleDirection {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphStyleDirection {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LEFT_TO_RIGHT" => ParagraphStyleDirection::LeftToRight,
                "RIGHT_TO_LEFT" => ParagraphStyleDirection::RightToLeft,
                "TEXT_DIRECTION_UNSPECIFIED" => ParagraphStyleDirection::TextDirectionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ParagraphStyleDirection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphStyleDirection {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParagraphStyleSpacingMode {
        #[doc = "Paragraph spacing is skipped between list elements."]
        CollapseLists,
        #[doc = "Paragraph spacing is always rendered."]
        NeverCollapse,
        #[doc = "The spacing mode is inherited from the parent."]
        SpacingModeUnspecified,
    }
    impl ParagraphStyleSpacingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleSpacingMode::CollapseLists => "COLLAPSE_LISTS",
                ParagraphStyleSpacingMode::NeverCollapse => "NEVER_COLLAPSE",
                ParagraphStyleSpacingMode::SpacingModeUnspecified => "SPACING_MODE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ParagraphStyleSpacingMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParagraphStyleSpacingMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParagraphStyleSpacingMode, ()> {
            Ok(match s {
                "COLLAPSE_LISTS" => ParagraphStyleSpacingMode::CollapseLists,
                "NEVER_COLLAPSE" => ParagraphStyleSpacingMode::NeverCollapse,
                "SPACING_MODE_UNSPECIFIED" => ParagraphStyleSpacingMode::SpacingModeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ParagraphStyleSpacingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphStyleSpacingMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphStyleSpacingMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COLLAPSE_LISTS" => ParagraphStyleSpacingMode::CollapseLists,
                "NEVER_COLLAPSE" => ParagraphStyleSpacingMode::NeverCollapse,
                "SPACING_MODE_UNSPECIFIED" => ParagraphStyleSpacingMode::SpacingModeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ParagraphStyleSpacingMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphStyleSpacingMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Placeholder {
        #[doc = "The index of the placeholder. If the same placeholder types are present in\nthe same page, they would have different index values."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The object ID of this shape's parent placeholder.\nIf unset, the parent placeholder shape does not exist, so the shape does\nnot inherit properties from any other shape."]
        #[serde(
            rename = "parentObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent_object_id: ::std::option::Option<String>,
        #[doc = "The type of the placeholder."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::PlaceholderType>,
    }
    impl ::google_field_selector::FieldSelector for Placeholder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Placeholder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlaceholderType {
        #[doc = "Body text."]
        Body,
        #[doc = "Title centered."]
        CenteredTitle,
        #[doc = "Chart or graph."]
        Chart,
        #[doc = "Clip art image."]
        ClipArt,
        #[doc = "Date and time."]
        DateAndTime,
        #[doc = "Diagram."]
        Diagram,
        #[doc = "Footer text."]
        Footer,
        #[doc = "Header text."]
        Header,
        #[doc = "Multimedia."]
        Media,
        #[doc = "Default value, signifies it is not a placeholder."]
        None,
        #[doc = "Any content type."]
        Object,
        #[doc = "Picture."]
        Picture,
        #[doc = "Slide image."]
        SlideImage,
        #[doc = "Number of a slide."]
        SlideNumber,
        #[doc = "Subtitle."]
        Subtitle,
        #[doc = "Table."]
        Table,
        #[doc = "Slide title."]
        Title,
    }
    impl PlaceholderType {
        pub fn as_str(self) -> &'static str {
            match self {
                PlaceholderType::Body => "BODY",
                PlaceholderType::CenteredTitle => "CENTERED_TITLE",
                PlaceholderType::Chart => "CHART",
                PlaceholderType::ClipArt => "CLIP_ART",
                PlaceholderType::DateAndTime => "DATE_AND_TIME",
                PlaceholderType::Diagram => "DIAGRAM",
                PlaceholderType::Footer => "FOOTER",
                PlaceholderType::Header => "HEADER",
                PlaceholderType::Media => "MEDIA",
                PlaceholderType::None => "NONE",
                PlaceholderType::Object => "OBJECT",
                PlaceholderType::Picture => "PICTURE",
                PlaceholderType::SlideImage => "SLIDE_IMAGE",
                PlaceholderType::SlideNumber => "SLIDE_NUMBER",
                PlaceholderType::Subtitle => "SUBTITLE",
                PlaceholderType::Table => "TABLE",
                PlaceholderType::Title => "TITLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PlaceholderType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PlaceholderType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PlaceholderType, ()> {
            Ok(match s {
                "BODY" => PlaceholderType::Body,
                "CENTERED_TITLE" => PlaceholderType::CenteredTitle,
                "CHART" => PlaceholderType::Chart,
                "CLIP_ART" => PlaceholderType::ClipArt,
                "DATE_AND_TIME" => PlaceholderType::DateAndTime,
                "DIAGRAM" => PlaceholderType::Diagram,
                "FOOTER" => PlaceholderType::Footer,
                "HEADER" => PlaceholderType::Header,
                "MEDIA" => PlaceholderType::Media,
                "NONE" => PlaceholderType::None,
                "OBJECT" => PlaceholderType::Object,
                "PICTURE" => PlaceholderType::Picture,
                "SLIDE_IMAGE" => PlaceholderType::SlideImage,
                "SLIDE_NUMBER" => PlaceholderType::SlideNumber,
                "SUBTITLE" => PlaceholderType::Subtitle,
                "TABLE" => PlaceholderType::Table,
                "TITLE" => PlaceholderType::Title,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PlaceholderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlaceholderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlaceholderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BODY" => PlaceholderType::Body,
                "CENTERED_TITLE" => PlaceholderType::CenteredTitle,
                "CHART" => PlaceholderType::Chart,
                "CLIP_ART" => PlaceholderType::ClipArt,
                "DATE_AND_TIME" => PlaceholderType::DateAndTime,
                "DIAGRAM" => PlaceholderType::Diagram,
                "FOOTER" => PlaceholderType::Footer,
                "HEADER" => PlaceholderType::Header,
                "MEDIA" => PlaceholderType::Media,
                "NONE" => PlaceholderType::None,
                "OBJECT" => PlaceholderType::Object,
                "PICTURE" => PlaceholderType::Picture,
                "SLIDE_IMAGE" => PlaceholderType::SlideImage,
                "SLIDE_NUMBER" => PlaceholderType::SlideNumber,
                "SUBTITLE" => PlaceholderType::Subtitle,
                "TABLE" => PlaceholderType::Table,
                "TITLE" => PlaceholderType::Title,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PlaceholderType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PlaceholderType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Presentation {
        #[doc = "The layouts in the presentation. A layout is a template that determines\nhow content is arranged and styled on the slides that inherit from that\nlayout."]
        #[serde(
            rename = "layouts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layouts: ::std::option::Option<Vec<crate::schemas::Page>>,
        #[doc = "The locale of the presentation, as an IETF BCP 47 language tag."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "The slide masters in the presentation. A slide master contains all common\npage elements and the common properties for a set of layouts. They serve\nthree purposes:\n\n* Placeholder shapes on a master contain the default text styles and shape\n  properties of all placeholder shapes on pages that use that master.\n* The master page properties define the common page properties inherited by\n  its layouts.\n* Any other shapes on the master slide appear on all slides using that\n  master, regardless of their layout."]
        #[serde(
            rename = "masters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub masters: ::std::option::Option<Vec<crate::schemas::Page>>,
        #[doc = "The notes master in the presentation. It serves three purposes:\n\n* Placeholder shapes on a notes master contain the default text styles and\n  shape properties of all placeholder shapes on notes pages. Specifically,\n  a `SLIDE_IMAGE` placeholder shape contains the slide thumbnail, and a\n  `BODY` placeholder shape contains the speaker notes.\n* The notes master page properties define the common page properties\n  inherited by all notes pages.\n* Any other shapes on the notes master appears on all notes pages.\n\nThe notes master is read-only."]
        #[serde(
            rename = "notesMaster",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes_master: ::std::option::Option<crate::schemas::Page>,
        #[doc = "The size of pages in the presentation."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<crate::schemas::Size>,
        #[doc = "The ID of the presentation."]
        #[serde(
            rename = "presentationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub presentation_id: ::std::option::Option<String>,
        #[doc = "The revision ID of the presentation. Can be used in update requests\nto assert that the presentation revision hasn't changed since the last\nread operation. Only populated if the user has edit access to the\npresentation.\n\nThe format of the revision ID may change over time, so it should be treated\nopaquely. A returned revision ID is only guaranteed to be valid for 24\nhours after it has been returned and cannot be shared across users. If the\nrevision ID is unchanged between calls, then the presentation has not\nchanged. Conversely, a changed ID (for the same presentation and user)\nusually means the presentation has been updated; however, a changed ID can\nalso be due to internal factors such as ID format changes."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "The slides in the presentation.\nA slide inherits properties from a slide layout."]
        #[serde(
            rename = "slides",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slides: ::std::option::Option<Vec<crate::schemas::Page>>,
        #[doc = "The title of the presentation."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Presentation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Presentation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Range {
        #[doc = "The optional zero-based index of the end of the collection.\nRequired for `FIXED_RANGE` ranges."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The type of range."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::RangeType>,
        #[doc = "The optional zero-based index of the beginning of the collection.\nRequired for `FIXED_RANGE` and `FROM_START_INDEX` ranges."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Range {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Range {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RangeType {
        #[doc = "Sets the range to be the whole length of the collection. Both the\n`start_index` and the `end_index` must not be\nspecified."]
        All,
        #[doc = "A fixed range. Both the `start_index` and\n`end_index` must be specified."]
        FixedRange,
        #[doc = "Starts the range at `start_index` and continues until the\nend of the collection. The `end_index` must not be specified."]
        FromStartIndex,
        #[doc = "Unspecified range type. This value must not be used."]
        RangeTypeUnspecified,
    }
    impl RangeType {
        pub fn as_str(self) -> &'static str {
            match self {
                RangeType::All => "ALL",
                RangeType::FixedRange => "FIXED_RANGE",
                RangeType::FromStartIndex => "FROM_START_INDEX",
                RangeType::RangeTypeUnspecified => "RANGE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RangeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RangeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RangeType, ()> {
            Ok(match s {
                "ALL" => RangeType::All,
                "FIXED_RANGE" => RangeType::FixedRange,
                "FROM_START_INDEX" => RangeType::FromStartIndex,
                "RANGE_TYPE_UNSPECIFIED" => RangeType::RangeTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RangeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RangeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RangeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL" => RangeType::All,
                "FIXED_RANGE" => RangeType::FixedRange,
                "FROM_START_INDEX" => RangeType::FromStartIndex,
                "RANGE_TYPE_UNSPECIFIED" => RangeType::RangeTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RangeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RangeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Recolor {
        #[doc = "The name of the recolor effect.\n\nThe name is determined from the `recolor_stops` by matching the gradient\nagainst the colors in the page's current color scheme. This property is\nread-only."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<crate::schemas::RecolorName>,
        #[doc = "The recolor effect is represented by a gradient, which is a list of color\nstops.\n\nThe colors in the gradient will replace the corresponding colors at\nthe same position in the color palette and apply to the image. This\nproperty is read-only."]
        #[serde(
            rename = "recolorStops",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recolor_stops: ::std::option::Option<Vec<crate::schemas::ColorStop>>,
    }
    impl ::google_field_selector::FieldSelector for Recolor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Recolor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RecolorName {
        #[doc = "Custom recolor effect. Refer to `recolor_stops` for the concrete\ngradient."]
        Custom,
        #[doc = "A recolor effect that darkens the image using the page's first available\ncolor from its color scheme."]
        Dark1,
        #[doc = "A recolor effect that darkens the image using the page's tenth available\ncolor from its color scheme."]
        Dark10,
        #[doc = "A recolor effect that darkens the image using the page's second available\ncolor from its color scheme."]
        Dark2,
        #[doc = "A recolor effect that darkens the image using the page's third available\ncolor from its color scheme."]
        Dark3,
        #[doc = "A recolor effect that darkens the image using the page's fourth available\ncolor from its color scheme."]
        Dark4,
        #[doc = "A recolor effect that darkens the image using the page's fifth available\ncolor from its color scheme."]
        Dark5,
        #[doc = "A recolor effect that darkens the image using the page's sixth available\ncolor from its color scheme."]
        Dark6,
        #[doc = "A recolor effect that darkens the image using the page's seventh\navailable color from its color scheme."]
        Dark7,
        #[doc = "A recolor effect that darkens the image using the page's eighth available\ncolor from its color scheme."]
        Dark8,
        #[doc = "A recolor effect that darkens the image using the page's ninth available\ncolor from its color scheme."]
        Dark9,
        #[doc = "A recolor effect that recolors the image to grayscale."]
        Grayscale,
        #[doc = "A recolor effect that lightens the image using the page's first available\ncolor from its color scheme."]
        Light1,
        #[doc = "A recolor effect that lightens the image using the page's tenth available\ncolor from its color scheme."]
        Light10,
        #[doc = "A recolor effect that lightens the image using the page's second\navailable color from its color scheme."]
        Light2,
        #[doc = "A recolor effect that lightens the image using the page's third available\ncolor from its color scheme."]
        Light3,
        #[doc = "A recolor effect that lightens the image using the page's forth available\ncolor from its color scheme."]
        Light4,
        #[doc = "A recolor effect that lightens the image using the page's fifth available\ncolor from its color scheme."]
        Light5,
        #[doc = "A recolor effect that lightens the image using the page's sixth available\ncolor from its color scheme."]
        Light6,
        #[doc = "A recolor effect that lightens the image using the page's seventh\navailable color from its color scheme."]
        Light7,
        #[doc = "A recolor effect that lightens the image using the page's eighth\navailable color from its color scheme."]
        Light8,
        #[doc = "A recolor effect that lightens the image using the page's ninth available\ncolor from its color scheme."]
        Light9,
        #[doc = "A recolor effect that recolors the image to negative grayscale."]
        Negative,
        #[doc = "No recolor effect. The default value."]
        None,
        #[doc = "A recolor effect that recolors the image using the sepia color."]
        Sepia,
    }
    impl RecolorName {
        pub fn as_str(self) -> &'static str {
            match self {
                RecolorName::Custom => "CUSTOM",
                RecolorName::Dark1 => "DARK1",
                RecolorName::Dark10 => "DARK10",
                RecolorName::Dark2 => "DARK2",
                RecolorName::Dark3 => "DARK3",
                RecolorName::Dark4 => "DARK4",
                RecolorName::Dark5 => "DARK5",
                RecolorName::Dark6 => "DARK6",
                RecolorName::Dark7 => "DARK7",
                RecolorName::Dark8 => "DARK8",
                RecolorName::Dark9 => "DARK9",
                RecolorName::Grayscale => "GRAYSCALE",
                RecolorName::Light1 => "LIGHT1",
                RecolorName::Light10 => "LIGHT10",
                RecolorName::Light2 => "LIGHT2",
                RecolorName::Light3 => "LIGHT3",
                RecolorName::Light4 => "LIGHT4",
                RecolorName::Light5 => "LIGHT5",
                RecolorName::Light6 => "LIGHT6",
                RecolorName::Light7 => "LIGHT7",
                RecolorName::Light8 => "LIGHT8",
                RecolorName::Light9 => "LIGHT9",
                RecolorName::Negative => "NEGATIVE",
                RecolorName::None => "NONE",
                RecolorName::Sepia => "SEPIA",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RecolorName {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RecolorName {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RecolorName, ()> {
            Ok(match s {
                "CUSTOM" => RecolorName::Custom,
                "DARK1" => RecolorName::Dark1,
                "DARK10" => RecolorName::Dark10,
                "DARK2" => RecolorName::Dark2,
                "DARK3" => RecolorName::Dark3,
                "DARK4" => RecolorName::Dark4,
                "DARK5" => RecolorName::Dark5,
                "DARK6" => RecolorName::Dark6,
                "DARK7" => RecolorName::Dark7,
                "DARK8" => RecolorName::Dark8,
                "DARK9" => RecolorName::Dark9,
                "GRAYSCALE" => RecolorName::Grayscale,
                "LIGHT1" => RecolorName::Light1,
                "LIGHT10" => RecolorName::Light10,
                "LIGHT2" => RecolorName::Light2,
                "LIGHT3" => RecolorName::Light3,
                "LIGHT4" => RecolorName::Light4,
                "LIGHT5" => RecolorName::Light5,
                "LIGHT6" => RecolorName::Light6,
                "LIGHT7" => RecolorName::Light7,
                "LIGHT8" => RecolorName::Light8,
                "LIGHT9" => RecolorName::Light9,
                "NEGATIVE" => RecolorName::Negative,
                "NONE" => RecolorName::None,
                "SEPIA" => RecolorName::Sepia,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RecolorName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RecolorName {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecolorName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM" => RecolorName::Custom,
                "DARK1" => RecolorName::Dark1,
                "DARK10" => RecolorName::Dark10,
                "DARK2" => RecolorName::Dark2,
                "DARK3" => RecolorName::Dark3,
                "DARK4" => RecolorName::Dark4,
                "DARK5" => RecolorName::Dark5,
                "DARK6" => RecolorName::Dark6,
                "DARK7" => RecolorName::Dark7,
                "DARK8" => RecolorName::Dark8,
                "DARK9" => RecolorName::Dark9,
                "GRAYSCALE" => RecolorName::Grayscale,
                "LIGHT1" => RecolorName::Light1,
                "LIGHT10" => RecolorName::Light10,
                "LIGHT2" => RecolorName::Light2,
                "LIGHT3" => RecolorName::Light3,
                "LIGHT4" => RecolorName::Light4,
                "LIGHT5" => RecolorName::Light5,
                "LIGHT6" => RecolorName::Light6,
                "LIGHT7" => RecolorName::Light7,
                "LIGHT8" => RecolorName::Light8,
                "LIGHT9" => RecolorName::Light9,
                "NEGATIVE" => RecolorName::Negative,
                "NONE" => RecolorName::None,
                "SEPIA" => RecolorName::Sepia,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RecolorName {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RecolorName {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RefreshSheetsChartRequest {
        #[doc = "The object ID of the chart to refresh."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RefreshSheetsChartRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RefreshSheetsChartRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithImageRequest {
        #[doc = "If set, this request will replace all of the shapes that contain the\ngiven text."]
        #[serde(
            rename = "containsText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contains_text: ::std::option::Option<crate::schemas::SubstringMatchCriteria>,
        #[doc = "The image replace method.\n\nIf you specify both a `replace_method` and an `image_replace_method`, the\n`image_replace_method` takes precedence.\n\nIf you do not specify a value for `image_replace_method`, but specify a\nvalue for `replace_method`, then the specified `replace_method` value is\nused.\n\nIf you do not specify either, then CENTER_INSIDE is used."]
        #[serde(
            rename = "imageReplaceMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_replace_method: ::std::option::Option<
            crate::schemas::ReplaceAllShapesWithImageRequestImageReplaceMethod,
        >,
        #[doc = "The image URL.\n\nThe image is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Images must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length. The URL itself is saved\nwith the image, and exposed via the Image.source_url field."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "If non-empty, limits the matches to page elements only on the given pages.\n\nReturns a 400 bad request error if given the page object ID of a\nnotes page or a\nnotes master, or if a\npage with that object ID doesn't exist in the presentation."]
        #[serde(
            rename = "pageObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_object_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The replace method.\n\n<b>Deprecated</b>: use `image_replace_method` instead.\n\nIf you specify both a `replace_method` and an `image_replace_method`, the\n`image_replace_method` takes precedence."]
        #[serde(
            rename = "replaceMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_method:
            ::std::option::Option<crate::schemas::ReplaceAllShapesWithImageRequestReplaceMethod>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllShapesWithImageRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllShapesWithImageRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceAllShapesWithImageRequestImageReplaceMethod {
        #[doc = "Scales and centers the image to fill the bounds of the original shape.\nThe image may be cropped in order to fill the shape. The rendered size of\nthe image will be the same as that of the original shape."]
        CenterCrop,
        #[doc = "Scales and centers the image to fit within the bounds of the original\nshape and maintains the image's aspect ratio. The rendered size of the\nimage may be smaller than the size of the shape. This is the default\nmethod when one is not specified."]
        CenterInside,
        #[doc = "Unspecified image replace method. This value must not be used."]
        ImageReplaceMethodUnspecified,
    }
    impl ReplaceAllShapesWithImageRequestImageReplaceMethod {
        pub fn as_str(self) -> &'static str {
            match self { ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterCrop => "CENTER_CROP" , ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterInside => "CENTER_INSIDE" , ReplaceAllShapesWithImageRequestImageReplaceMethod :: ImageReplaceMethodUnspecified => "IMAGE_REPLACE_METHOD_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ReplaceAllShapesWithImageRequestImageReplaceMethod, ()> {
            Ok ( match s { "CENTER_CROP" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterCrop , "CENTER_INSIDE" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterInside , "IMAGE_REPLACE_METHOD_UNSPECIFIED" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: ImageReplaceMethodUnspecified , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "CENTER_CROP" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterCrop , "CENTER_INSIDE" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterInside , "IMAGE_REPLACE_METHOD_UNSPECIFIED" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: ImageReplaceMethodUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceAllShapesWithImageRequestReplaceMethod {
        #[doc = "Scales and centers the image to fill the bounds of the original shape.\nThe image may be cropped in order to fill the shape. The rendered size of\nthe image will be the same as that of the original shape."]
        CenterCrop,
        #[doc = "Scales and centers the image to fit within the bounds of the original\nshape and maintains the image's aspect ratio. The rendered size of the\nimage may be smaller than the size of the shape. This is the default\nmethod when one is not specified."]
        CenterInside,
    }
    impl ReplaceAllShapesWithImageRequestReplaceMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                ReplaceAllShapesWithImageRequestReplaceMethod::CenterCrop => "CENTER_CROP",
                ReplaceAllShapesWithImageRequestReplaceMethod::CenterInside => "CENTER_INSIDE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReplaceAllShapesWithImageRequestReplaceMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ReplaceAllShapesWithImageRequestReplaceMethod, ()> {
            Ok(match s {
                "CENTER_CROP" => ReplaceAllShapesWithImageRequestReplaceMethod::CenterCrop,
                "CENTER_INSIDE" => ReplaceAllShapesWithImageRequestReplaceMethod::CenterInside,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER_CROP" => ReplaceAllShapesWithImageRequestReplaceMethod::CenterCrop,
                "CENTER_INSIDE" => ReplaceAllShapesWithImageRequestReplaceMethod::CenterInside,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithImageResponse {
        #[doc = "The number of shapes replaced with images."]
        #[serde(
            rename = "occurrencesChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences_changed: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllShapesWithImageResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllShapesWithImageResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithSheetsChartRequest {
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet."]
        #[serde(
            rename = "chartId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_id: ::std::option::Option<i32>,
        #[doc = "The criteria that the shapes must match in order to be replaced. The\nrequest will replace all of the shapes that contain the given text."]
        #[serde(
            rename = "containsText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contains_text: ::std::option::Option<crate::schemas::SubstringMatchCriteria>,
        #[doc = "The mode with which the chart is linked to the source spreadsheet. When\nnot specified, the chart will be an image that is not linked."]
        #[serde(
            rename = "linkingMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linking_mode: ::std::option::Option<
            crate::schemas::ReplaceAllShapesWithSheetsChartRequestLinkingMode,
        >,
        #[doc = "If non-empty, limits the matches to page elements only on the given pages.\n\nReturns a 400 bad request error if given the page object ID of a\nnotes page or a\nnotes master, or if a\npage with that object ID doesn't exist in the presentation."]
        #[serde(
            rename = "pageObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_object_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The ID of the Google Sheets spreadsheet that contains the chart."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllShapesWithSheetsChartRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllShapesWithSheetsChartRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        #[doc = "Linking the chart allows it to be updated, and other collaborators will\nsee a link to the spreadsheet."]
        Linked,
        #[doc = "The chart is not associated with the source spreadsheet and cannot be\nupdated. A chart that is not linked will be inserted as an image."]
        NotLinkedImage,
    }
    impl ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ReplaceAllShapesWithSheetsChartRequestLinkingMode::Linked => "LINKED",
                ReplaceAllShapesWithSheetsChartRequestLinkingMode::NotLinkedImage => {
                    "NOT_LINKED_IMAGE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ReplaceAllShapesWithSheetsChartRequestLinkingMode, ()> {
            Ok(match s {
                "LINKED" => ReplaceAllShapesWithSheetsChartRequestLinkingMode::Linked,
                "NOT_LINKED_IMAGE" => {
                    ReplaceAllShapesWithSheetsChartRequestLinkingMode::NotLinkedImage
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LINKED" => ReplaceAllShapesWithSheetsChartRequestLinkingMode::Linked,
                "NOT_LINKED_IMAGE" => {
                    ReplaceAllShapesWithSheetsChartRequestLinkingMode::NotLinkedImage
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
    impl ::google_field_selector::FieldSelector for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithSheetsChartResponse {
        #[doc = "The number of shapes replaced with charts."]
        #[serde(
            rename = "occurrencesChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences_changed: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllShapesWithSheetsChartResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllShapesWithSheetsChartResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllTextRequest {
        #[doc = "Finds text in a shape matching this substring."]
        #[serde(
            rename = "containsText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contains_text: ::std::option::Option<crate::schemas::SubstringMatchCriteria>,
        #[doc = "If non-empty, limits the matches to page elements only on the given pages.\n\nReturns a 400 bad request error if given the page object ID of a\nnotes master,\nor if a page with that object ID doesn't exist in the presentation."]
        #[serde(
            rename = "pageObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_object_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The text that will replace the matched text."]
        #[serde(
            rename = "replaceText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllTextRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllTextRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllTextResponse {
        #[doc = "The number of occurrences changed by replacing all text."]
        #[serde(
            rename = "occurrencesChanged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences_changed: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceAllTextResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceAllTextResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceImageRequest {
        #[doc = "The ID of the existing image that will be replaced."]
        #[serde(
            rename = "imageObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_object_id: ::std::option::Option<String>,
        #[doc = "The replacement method."]
        #[serde(
            rename = "imageReplaceMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_replace_method:
            ::std::option::Option<crate::schemas::ReplaceImageRequestImageReplaceMethod>,
        #[doc = "The image URL.\n\nThe image is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Images must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length. The URL itself is saved\nwith the image, and exposed via the Image.source_url field."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceImageRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceImageRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceImageRequestImageReplaceMethod {
        #[doc = "Scales and centers the image to fill the bounds of the original shape.\nThe image may be cropped in order to fill the shape. The rendered size of\nthe image will be the same as that of the original shape."]
        CenterCrop,
        #[doc = "Scales and centers the image to fit within the bounds of the original\nshape and maintains the image's aspect ratio. The rendered size of the\nimage may be smaller than the size of the shape. This is the default\nmethod when one is not specified."]
        CenterInside,
        #[doc = "Unspecified image replace method. This value must not be used."]
        ImageReplaceMethodUnspecified,
    }
    impl ReplaceImageRequestImageReplaceMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                ReplaceImageRequestImageReplaceMethod::CenterCrop => "CENTER_CROP",
                ReplaceImageRequestImageReplaceMethod::CenterInside => "CENTER_INSIDE",
                ReplaceImageRequestImageReplaceMethod::ImageReplaceMethodUnspecified => {
                    "IMAGE_REPLACE_METHOD_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReplaceImageRequestImageReplaceMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReplaceImageRequestImageReplaceMethod {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ReplaceImageRequestImageReplaceMethod, ()> {
            Ok(match s {
                "CENTER_CROP" => ReplaceImageRequestImageReplaceMethod::CenterCrop,
                "CENTER_INSIDE" => ReplaceImageRequestImageReplaceMethod::CenterInside,
                "IMAGE_REPLACE_METHOD_UNSPECIFIED" => {
                    ReplaceImageRequestImageReplaceMethod::ImageReplaceMethodUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReplaceImageRequestImageReplaceMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceImageRequestImageReplaceMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceImageRequestImageReplaceMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER_CROP" => ReplaceImageRequestImageReplaceMethod::CenterCrop,
                "CENTER_INSIDE" => ReplaceImageRequestImageReplaceMethod::CenterInside,
                "IMAGE_REPLACE_METHOD_UNSPECIFIED" => {
                    ReplaceImageRequestImageReplaceMethod::ImageReplaceMethodUnspecified
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
    impl ::google_field_selector::FieldSelector for ReplaceImageRequestImageReplaceMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceImageRequestImageReplaceMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Request {
        #[doc = "Creates an image."]
        #[serde(
            rename = "createImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_image: ::std::option::Option<crate::schemas::CreateImageRequest>,
        #[doc = "Creates a line."]
        #[serde(
            rename = "createLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_line: ::std::option::Option<crate::schemas::CreateLineRequest>,
        #[doc = "Creates bullets for paragraphs."]
        #[serde(
            rename = "createParagraphBullets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_paragraph_bullets:
            ::std::option::Option<crate::schemas::CreateParagraphBulletsRequest>,
        #[doc = "Creates a new shape."]
        #[serde(
            rename = "createShape",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_shape: ::std::option::Option<crate::schemas::CreateShapeRequest>,
        #[doc = "Creates an embedded Google Sheets chart."]
        #[serde(
            rename = "createSheetsChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_sheets_chart: ::std::option::Option<crate::schemas::CreateSheetsChartRequest>,
        #[doc = "Creates a new slide."]
        #[serde(
            rename = "createSlide",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_slide: ::std::option::Option<crate::schemas::CreateSlideRequest>,
        #[doc = "Creates a new table."]
        #[serde(
            rename = "createTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_table: ::std::option::Option<crate::schemas::CreateTableRequest>,
        #[doc = "Creates a video."]
        #[serde(
            rename = "createVideo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_video: ::std::option::Option<crate::schemas::CreateVideoRequest>,
        #[doc = "Deletes a page or page element from the presentation."]
        #[serde(
            rename = "deleteObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_object: ::std::option::Option<crate::schemas::DeleteObjectRequest>,
        #[doc = "Deletes bullets from paragraphs."]
        #[serde(
            rename = "deleteParagraphBullets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_paragraph_bullets:
            ::std::option::Option<crate::schemas::DeleteParagraphBulletsRequest>,
        #[doc = "Deletes a column from a table."]
        #[serde(
            rename = "deleteTableColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_table_column: ::std::option::Option<crate::schemas::DeleteTableColumnRequest>,
        #[doc = "Deletes a row from a table."]
        #[serde(
            rename = "deleteTableRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_table_row: ::std::option::Option<crate::schemas::DeleteTableRowRequest>,
        #[doc = "Deletes text from a shape or a table cell."]
        #[serde(
            rename = "deleteText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_text: ::std::option::Option<crate::schemas::DeleteTextRequest>,
        #[doc = "Duplicates a slide or page element."]
        #[serde(
            rename = "duplicateObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duplicate_object: ::std::option::Option<crate::schemas::DuplicateObjectRequest>,
        #[doc = "Groups objects, such as page elements."]
        #[serde(
            rename = "groupObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_objects: ::std::option::Option<crate::schemas::GroupObjectsRequest>,
        #[doc = "Inserts columns into a table."]
        #[serde(
            rename = "insertTableColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_table_columns: ::std::option::Option<crate::schemas::InsertTableColumnsRequest>,
        #[doc = "Inserts rows into a table."]
        #[serde(
            rename = "insertTableRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_table_rows: ::std::option::Option<crate::schemas::InsertTableRowsRequest>,
        #[doc = "Inserts text into a shape or table cell."]
        #[serde(
            rename = "insertText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_text: ::std::option::Option<crate::schemas::InsertTextRequest>,
        #[doc = "Merges cells in a Table."]
        #[serde(
            rename = "mergeTableCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub merge_table_cells: ::std::option::Option<crate::schemas::MergeTableCellsRequest>,
        #[doc = "Refreshes a Google Sheets chart."]
        #[serde(
            rename = "refreshSheetsChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_sheets_chart: ::std::option::Option<crate::schemas::RefreshSheetsChartRequest>,
        #[doc = "Replaces all shapes matching some criteria with an image."]
        #[serde(
            rename = "replaceAllShapesWithImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_all_shapes_with_image:
            ::std::option::Option<crate::schemas::ReplaceAllShapesWithImageRequest>,
        #[doc = "Replaces all shapes matching some criteria with a Google Sheets chart."]
        #[serde(
            rename = "replaceAllShapesWithSheetsChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_all_shapes_with_sheets_chart:
            ::std::option::Option<crate::schemas::ReplaceAllShapesWithSheetsChartRequest>,
        #[doc = "Replaces all instances of specified text."]
        #[serde(
            rename = "replaceAllText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_all_text: ::std::option::Option<crate::schemas::ReplaceAllTextRequest>,
        #[doc = "Replaces an existing image with a new image."]
        #[serde(
            rename = "replaceImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_image: ::std::option::Option<crate::schemas::ReplaceImageRequest>,
        #[doc = "Reroutes a line such that it's connected\nat the two closest connection sites on the connected page elements."]
        #[serde(
            rename = "rerouteLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reroute_line: ::std::option::Option<crate::schemas::RerouteLineRequest>,
        #[doc = "Ungroups objects, such as groups."]
        #[serde(
            rename = "ungroupObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ungroup_objects: ::std::option::Option<crate::schemas::UngroupObjectsRequest>,
        #[doc = "Unmerges cells in a Table."]
        #[serde(
            rename = "unmergeTableCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unmerge_table_cells: ::std::option::Option<crate::schemas::UnmergeTableCellsRequest>,
        #[doc = "Updates the properties of an Image."]
        #[serde(
            rename = "updateImageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_image_properties:
            ::std::option::Option<crate::schemas::UpdateImagePropertiesRequest>,
        #[doc = "Updates the category of a line."]
        #[serde(
            rename = "updateLineCategory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_line_category: ::std::option::Option<crate::schemas::UpdateLineCategoryRequest>,
        #[doc = "Updates the properties of a Line."]
        #[serde(
            rename = "updateLineProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_line_properties:
            ::std::option::Option<crate::schemas::UpdateLinePropertiesRequest>,
        #[doc = "Updates the alt text title and/or description of a\npage element."]
        #[serde(
            rename = "updatePageElementAltText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_page_element_alt_text:
            ::std::option::Option<crate::schemas::UpdatePageElementAltTextRequest>,
        #[doc = "Updates the transform of a page element."]
        #[serde(
            rename = "updatePageElementTransform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_page_element_transform:
            ::std::option::Option<crate::schemas::UpdatePageElementTransformRequest>,
        #[doc = "Updates the Z-order of page elements."]
        #[serde(
            rename = "updatePageElementsZOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_page_elements_z_order:
            ::std::option::Option<crate::schemas::UpdatePageElementsZOrderRequest>,
        #[doc = "Updates the properties of a Page."]
        #[serde(
            rename = "updatePageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_page_properties:
            ::std::option::Option<crate::schemas::UpdatePagePropertiesRequest>,
        #[doc = "Updates the styling of paragraphs within a Shape or Table."]
        #[serde(
            rename = "updateParagraphStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_paragraph_style:
            ::std::option::Option<crate::schemas::UpdateParagraphStyleRequest>,
        #[doc = "Updates the properties of a Shape."]
        #[serde(
            rename = "updateShapeProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_shape_properties:
            ::std::option::Option<crate::schemas::UpdateShapePropertiesRequest>,
        #[doc = "Updates the position of a set of slides in the presentation."]
        #[serde(
            rename = "updateSlidesPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_slides_position:
            ::std::option::Option<crate::schemas::UpdateSlidesPositionRequest>,
        #[doc = "Updates the properties of the table borders in a Table."]
        #[serde(
            rename = "updateTableBorderProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_table_border_properties:
            ::std::option::Option<crate::schemas::UpdateTableBorderPropertiesRequest>,
        #[doc = "Updates the properties of a TableCell."]
        #[serde(
            rename = "updateTableCellProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_table_cell_properties:
            ::std::option::Option<crate::schemas::UpdateTableCellPropertiesRequest>,
        #[doc = "Updates the properties of a Table\ncolumn."]
        #[serde(
            rename = "updateTableColumnProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_table_column_properties:
            ::std::option::Option<crate::schemas::UpdateTableColumnPropertiesRequest>,
        #[doc = "Updates the properties of a Table row."]
        #[serde(
            rename = "updateTableRowProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_table_row_properties:
            ::std::option::Option<crate::schemas::UpdateTableRowPropertiesRequest>,
        #[doc = "Updates the styling of text within a Shape or Table."]
        #[serde(
            rename = "updateTextStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_text_style: ::std::option::Option<crate::schemas::UpdateTextStyleRequest>,
        #[doc = "Updates the properties of a Video."]
        #[serde(
            rename = "updateVideoProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_video_properties:
            ::std::option::Option<crate::schemas::UpdateVideoPropertiesRequest>,
    }
    impl ::google_field_selector::FieldSelector for Request {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Request {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RerouteLineRequest {
        #[doc = "The object ID of the line to reroute.\n\nOnly a line with a category\nindicating it is a \"connector\" can be rerouted. The start and end\nconnections of the line must be on different page elements."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RerouteLineRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RerouteLineRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Response {
        #[doc = "The result of creating an image."]
        #[serde(
            rename = "createImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_image: ::std::option::Option<crate::schemas::CreateImageResponse>,
        #[doc = "The result of creating a line."]
        #[serde(
            rename = "createLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_line: ::std::option::Option<crate::schemas::CreateLineResponse>,
        #[doc = "The result of creating a shape."]
        #[serde(
            rename = "createShape",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_shape: ::std::option::Option<crate::schemas::CreateShapeResponse>,
        #[doc = "The result of creating a Google Sheets chart."]
        #[serde(
            rename = "createSheetsChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_sheets_chart: ::std::option::Option<crate::schemas::CreateSheetsChartResponse>,
        #[doc = "The result of creating a slide."]
        #[serde(
            rename = "createSlide",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_slide: ::std::option::Option<crate::schemas::CreateSlideResponse>,
        #[doc = "The result of creating a table."]
        #[serde(
            rename = "createTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_table: ::std::option::Option<crate::schemas::CreateTableResponse>,
        #[doc = "The result of creating a video."]
        #[serde(
            rename = "createVideo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_video: ::std::option::Option<crate::schemas::CreateVideoResponse>,
        #[doc = "The result of duplicating an object."]
        #[serde(
            rename = "duplicateObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duplicate_object: ::std::option::Option<crate::schemas::DuplicateObjectResponse>,
        #[doc = "The result of grouping objects."]
        #[serde(
            rename = "groupObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_objects: ::std::option::Option<crate::schemas::GroupObjectsResponse>,
        #[doc = "The result of replacing all shapes matching some criteria with an\nimage."]
        #[serde(
            rename = "replaceAllShapesWithImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_all_shapes_with_image:
            ::std::option::Option<crate::schemas::ReplaceAllShapesWithImageResponse>,
        #[doc = "The result of replacing all shapes matching some criteria with a Google\nSheets chart."]
        #[serde(
            rename = "replaceAllShapesWithSheetsChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_all_shapes_with_sheets_chart:
            ::std::option::Option<crate::schemas::ReplaceAllShapesWithSheetsChartResponse>,
        #[doc = "The result of replacing text."]
        #[serde(
            rename = "replaceAllText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_all_text: ::std::option::Option<crate::schemas::ReplaceAllTextResponse>,
    }
    impl ::google_field_selector::FieldSelector for Response {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Response {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RgbColor {
        #[doc = "The blue component of the color, from 0.0 to 1.0."]
        #[serde(
            rename = "blue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blue: ::std::option::Option<f32>,
        #[doc = "The green component of the color, from 0.0 to 1.0."]
        #[serde(
            rename = "green",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub green: ::std::option::Option<f32>,
        #[doc = "The red component of the color, from 0.0 to 1.0."]
        #[serde(
            rename = "red",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub red: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for RgbColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RgbColor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Shadow {
        #[doc = "The alignment point of the shadow, that sets the origin for translate,\nscale and skew of the shadow. This property is read-only."]
        #[serde(
            rename = "alignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alignment: ::std::option::Option<crate::schemas::ShadowAlignment>,
        #[doc = "The alpha of the shadow's color, from 0.0 to 1.0."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The radius of the shadow blur. The larger the radius, the more diffuse the\nshadow becomes."]
        #[serde(
            rename = "blurRadius",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blur_radius: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The shadow color value."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::OpaqueColor>,
        #[doc = "The shadow property state.\n\nUpdating the shadow on a page element will implicitly update this field to\n`RENDERED`, unless another value is specified in the same request. To have\nno shadow on a page element, set this field to `NOT_RENDERED`. In this\ncase, any other shadow fields set in the same request will be ignored."]
        #[serde(
            rename = "propertyState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_state: ::std::option::Option<crate::schemas::ShadowPropertyState>,
        #[doc = "The type of the shadow. This property is read-only."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ShadowType>,
        #[doc = "Whether the shadow should rotate with the shape. This property is\nread-only."]
        #[serde(
            rename = "rotateWithShape",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotate_with_shape: ::std::option::Option<bool>,
        #[doc = "Transform that encodes the translate, scale, and skew of the shadow,\nrelative to the alignment position."]
        #[serde(
            rename = "transform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transform: ::std::option::Option<crate::schemas::AffineTransform>,
    }
    impl ::google_field_selector::FieldSelector for Shadow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Shadow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShadowAlignment {
        #[doc = "Bottom center."]
        BottomCenter,
        #[doc = "Bottom left."]
        BottomLeft,
        #[doc = "Bottom right."]
        BottomRight,
        #[doc = "Center."]
        Center,
        #[doc = "Left center."]
        LeftCenter,
        #[doc = "Unspecified."]
        RectanglePositionUnspecified,
        #[doc = "Right center."]
        RightCenter,
        #[doc = "Top center."]
        TopCenter,
        #[doc = "Top left."]
        TopLeft,
        #[doc = "Top right."]
        TopRight,
    }
    impl ShadowAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                ShadowAlignment::BottomCenter => "BOTTOM_CENTER",
                ShadowAlignment::BottomLeft => "BOTTOM_LEFT",
                ShadowAlignment::BottomRight => "BOTTOM_RIGHT",
                ShadowAlignment::Center => "CENTER",
                ShadowAlignment::LeftCenter => "LEFT_CENTER",
                ShadowAlignment::RectanglePositionUnspecified => "RECTANGLE_POSITION_UNSPECIFIED",
                ShadowAlignment::RightCenter => "RIGHT_CENTER",
                ShadowAlignment::TopCenter => "TOP_CENTER",
                ShadowAlignment::TopLeft => "TOP_LEFT",
                ShadowAlignment::TopRight => "TOP_RIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ShadowAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ShadowAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ShadowAlignment, ()> {
            Ok(match s {
                "BOTTOM_CENTER" => ShadowAlignment::BottomCenter,
                "BOTTOM_LEFT" => ShadowAlignment::BottomLeft,
                "BOTTOM_RIGHT" => ShadowAlignment::BottomRight,
                "CENTER" => ShadowAlignment::Center,
                "LEFT_CENTER" => ShadowAlignment::LeftCenter,
                "RECTANGLE_POSITION_UNSPECIFIED" => ShadowAlignment::RectanglePositionUnspecified,
                "RIGHT_CENTER" => ShadowAlignment::RightCenter,
                "TOP_CENTER" => ShadowAlignment::TopCenter,
                "TOP_LEFT" => ShadowAlignment::TopLeft,
                "TOP_RIGHT" => ShadowAlignment::TopRight,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ShadowAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShadowAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShadowAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM_CENTER" => ShadowAlignment::BottomCenter,
                "BOTTOM_LEFT" => ShadowAlignment::BottomLeft,
                "BOTTOM_RIGHT" => ShadowAlignment::BottomRight,
                "CENTER" => ShadowAlignment::Center,
                "LEFT_CENTER" => ShadowAlignment::LeftCenter,
                "RECTANGLE_POSITION_UNSPECIFIED" => ShadowAlignment::RectanglePositionUnspecified,
                "RIGHT_CENTER" => ShadowAlignment::RightCenter,
                "TOP_CENTER" => ShadowAlignment::TopCenter,
                "TOP_LEFT" => ShadowAlignment::TopLeft,
                "TOP_RIGHT" => ShadowAlignment::TopRight,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ShadowAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShadowAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShadowPropertyState {
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
    }
    impl ShadowPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                ShadowPropertyState::Inherit => "INHERIT",
                ShadowPropertyState::NotRendered => "NOT_RENDERED",
                ShadowPropertyState::Rendered => "RENDERED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ShadowPropertyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ShadowPropertyState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ShadowPropertyState, ()> {
            Ok(match s {
                "INHERIT" => ShadowPropertyState::Inherit,
                "NOT_RENDERED" => ShadowPropertyState::NotRendered,
                "RENDERED" => ShadowPropertyState::Rendered,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ShadowPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShadowPropertyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShadowPropertyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INHERIT" => ShadowPropertyState::Inherit,
                "NOT_RENDERED" => ShadowPropertyState::NotRendered,
                "RENDERED" => ShadowPropertyState::Rendered,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ShadowPropertyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShadowPropertyState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShadowType {
        #[doc = "Outer shadow."]
        Outer,
        #[doc = "Unspecified shadow type."]
        ShadowTypeUnspecified,
    }
    impl ShadowType {
        pub fn as_str(self) -> &'static str {
            match self {
                ShadowType::Outer => "OUTER",
                ShadowType::ShadowTypeUnspecified => "SHADOW_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ShadowType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ShadowType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ShadowType, ()> {
            Ok(match s {
                "OUTER" => ShadowType::Outer,
                "SHADOW_TYPE_UNSPECIFIED" => ShadowType::ShadowTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ShadowType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShadowType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShadowType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OUTER" => ShadowType::Outer,
                "SHADOW_TYPE_UNSPECIFIED" => ShadowType::ShadowTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ShadowType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShadowType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Shape {
        #[doc = "Placeholders are shapes that are inherit from corresponding placeholders on\nlayouts and masters.\n\nIf set, the shape is a placeholder shape and any inherited properties\ncan be resolved by looking at the parent placeholder identified by the\nPlaceholder.parent_object_id field."]
        #[serde(
            rename = "placeholder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub placeholder: ::std::option::Option<crate::schemas::Placeholder>,
        #[doc = "The properties of the shape."]
        #[serde(
            rename = "shapeProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shape_properties: ::std::option::Option<crate::schemas::ShapeProperties>,
        #[doc = "The type of the shape."]
        #[serde(
            rename = "shapeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shape_type: ::std::option::Option<crate::schemas::ShapeShapeType>,
        #[doc = "The text content of the shape."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<crate::schemas::TextContent>,
    }
    impl ::google_field_selector::FieldSelector for Shape {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Shape {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShapeShapeType {
        #[doc = "Curved arc shape. Corresponds to ECMA-376 ST_ShapeType 'arc'"]
        Arc,
        #[doc = "East arrow shape."]
        ArrowEast,
        #[doc = "North arrow shape."]
        ArrowNorth,
        #[doc = "Northeast arrow shape."]
        ArrowNorthEast,
        #[doc = "Bent arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentArrow'"]
        BentArrow,
        #[doc = "Bent up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentUpArrow'"]
        BentUpArrow,
        #[doc = "Bevel shape. Corresponds to ECMA-376 ST_ShapeType 'bevel'"]
        Bevel,
        #[doc = "Block arc shape. Corresponds to ECMA-376 ST_ShapeType 'blockArc'"]
        BlockArc,
        #[doc = "Brace pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracePair'"]
        BracePair,
        #[doc = "Bracket pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracketPair'"]
        BracketPair,
        #[doc = "Can shape. Corresponds to ECMA-376 ST_ShapeType 'can'"]
        Can,
        #[doc = "Chevron shape. Corresponds to ECMA-376 ST_ShapeType 'chevron'"]
        Chevron,
        #[doc = "Chord shape. Corresponds to ECMA-376 ST_ShapeType 'chord'"]
        Chord,
        #[doc = "Cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloud'"]
        Cloud,
        #[doc = "Callout cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloudCallout'"]
        CloudCallout,
        #[doc = "Corner shape. Corresponds to ECMA-376 ST_ShapeType 'corner'"]
        Corner,
        #[doc = "Cube shape. Corresponds to ECMA-376 ST_ShapeType 'cube'"]
        Cube,
        #[doc = "Curved down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedDownArrow'"]
        CurvedDownArrow,
        #[doc = "Curved left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedLeftArrow'"]
        CurvedLeftArrow,
        #[doc = "Curved right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedRightArrow'"]
        CurvedRightArrow,
        #[doc = "Curved up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedUpArrow'"]
        CurvedUpArrow,
        #[doc = "Custom shape."]
        Custom,
        #[doc = "Decagon shape. Corresponds to ECMA-376 ST_ShapeType 'decagon'"]
        Decagon,
        #[doc = "Diagonal stripe shape. Corresponds to ECMA-376 ST_ShapeType 'diagStripe'"]
        DiagonalStripe,
        #[doc = "Diamond shape. Corresponds to ECMA-376 ST_ShapeType 'diamond'"]
        Diamond,
        #[doc = "Dodecagon shape. Corresponds to ECMA-376 ST_ShapeType 'dodecagon'"]
        Dodecagon,
        #[doc = "Donut shape. Corresponds to ECMA-376 ST_ShapeType 'donut'"]
        Donut,
        #[doc = "Double wave shape. Corresponds to ECMA-376 ST_ShapeType 'doubleWave'"]
        DoubleWave,
        #[doc = "Down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrow'"]
        DownArrow,
        #[doc = "Callout down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'downArrowCallout'"]
        DownArrowCallout,
        #[doc = "Ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'ellipse'"]
        Ellipse,
        #[doc = "Ellipse ribbon shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon'"]
        EllipseRibbon,
        #[doc = "Ellipse ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon2'"]
        EllipseRibbon2,
        #[doc = "Alternate process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartAlternateProcess'"]
        FlowChartAlternateProcess,
        #[doc = "Collate flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartCollate'"]
        FlowChartCollate,
        #[doc = "Connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartConnector'"]
        FlowChartConnector,
        #[doc = "Decision flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDecision'"]
        FlowChartDecision,
        #[doc = "Delay flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDelay'"]
        FlowChartDelay,
        #[doc = "Display flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDisplay'"]
        FlowChartDisplay,
        #[doc = "Document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDocument'"]
        FlowChartDocument,
        #[doc = "Extract flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartExtract'"]
        FlowChartExtract,
        #[doc = "Input output flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInputOutput'"]
        FlowChartInputOutput,
        #[doc = "Internal storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInternalStorage'"]
        FlowChartInternalStorage,
        #[doc = "Magnetic disk flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDisk'"]
        FlowChartMagneticDisk,
        #[doc = "Magnetic drum flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDrum'"]
        FlowChartMagneticDrum,
        #[doc = "Magnetic tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticTape'"]
        FlowChartMagneticTape,
        #[doc = "Manual input flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualInput'"]
        FlowChartManualInput,
        #[doc = "Manual operation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualOperation'"]
        FlowChartManualOperation,
        #[doc = "Merge flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMerge'"]
        FlowChartMerge,
        #[doc = "Multi-document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMultidocument'"]
        FlowChartMultidocument,
        #[doc = "Offline storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOfflineStorage'"]
        FlowChartOfflineStorage,
        #[doc = "Off-page connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOffpageConnector'"]
        FlowChartOffpageConnector,
        #[doc = "Online storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOnlineStorage'"]
        FlowChartOnlineStorage,
        #[doc = "Or flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOr'"]
        FlowChartOr,
        #[doc = "Predefined process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPredefinedProcess'"]
        FlowChartPredefinedProcess,
        #[doc = "Preparation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPreparation'"]
        FlowChartPreparation,
        #[doc = "Process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartProcess'"]
        FlowChartProcess,
        #[doc = "Punched card flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedCard'"]
        FlowChartPunchedCard,
        #[doc = "Punched tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedTape'"]
        FlowChartPunchedTape,
        #[doc = "Sort flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSort'"]
        FlowChartSort,
        #[doc = "Summing junction flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartSummingJunction'"]
        FlowChartSummingJunction,
        #[doc = "Terminator flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartTerminator'"]
        FlowChartTerminator,
        #[doc = "Folded corner shape. Corresponds to ECMA-376 ST_ShapeType 'foldedCorner'"]
        FoldedCorner,
        #[doc = "Frame shape. Corresponds to ECMA-376 ST_ShapeType 'frame'"]
        Frame,
        #[doc = "Half frame shape. Corresponds to ECMA-376 ST_ShapeType 'halfFrame'"]
        HalfFrame,
        #[doc = "Heart shape. Corresponds to ECMA-376 ST_ShapeType 'heart'"]
        Heart,
        #[doc = "Heptagon shape. Corresponds to ECMA-376 ST_ShapeType 'heptagon'"]
        Heptagon,
        #[doc = "Hexagon shape. Corresponds to ECMA-376 ST_ShapeType 'hexagon'"]
        Hexagon,
        #[doc = "Home plate shape. Corresponds to ECMA-376 ST_ShapeType 'homePlate'"]
        HomePlate,
        #[doc = "Horizontal scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'horizontalScroll'"]
        HorizontalScroll,
        #[doc = "Irregular seal 1 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal1'"]
        IrregularSeal1,
        #[doc = "Irregular seal 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal2'"]
        IrregularSeal2,
        #[doc = "Left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrow'"]
        LeftArrow,
        #[doc = "Callout left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftArrowCallout'"]
        LeftArrowCallout,
        #[doc = "Left brace shape. Corresponds to ECMA-376 ST_ShapeType 'leftBrace'"]
        LeftBrace,
        #[doc = "Left bracket shape. Corresponds to ECMA-376 ST_ShapeType 'leftBracket'"]
        LeftBracket,
        #[doc = "Left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrow'"]
        LeftRightArrow,
        #[doc = "Callout left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrowCallout'"]
        LeftRightArrowCallout,
        #[doc = "Left right up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightUpArrow'"]
        LeftRightUpArrow,
        #[doc = "Left up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftUpArrow'"]
        LeftUpArrow,
        #[doc = "Lightning bolt shape. Corresponds to ECMA-376 ST_ShapeType\n'lightningBolt'"]
        LightningBolt,
        #[doc = "Divide math shape. Corresponds to ECMA-376 ST_ShapeType 'mathDivide'"]
        MathDivide,
        #[doc = "Equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathEqual'"]
        MathEqual,
        #[doc = "Minus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMinus'"]
        MathMinus,
        #[doc = "Multiply math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMultiply'"]
        MathMultiply,
        #[doc = "Not equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathNotEqual'"]
        MathNotEqual,
        #[doc = "Plus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathPlus'"]
        MathPlus,
        #[doc = "Moon shape. Corresponds to ECMA-376 ST_ShapeType 'moon'"]
        Moon,
        #[doc = "No smoking shape. Corresponds to ECMA-376 ST_ShapeType 'noSmoking'"]
        NoSmoking,
        #[doc = "Notched right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'notchedRightArrow'"]
        NotchedRightArrow,
        #[doc = "Octagon shape. Corresponds to ECMA-376 ST_ShapeType 'octagon'"]
        Octagon,
        #[doc = "Parallelogram shape. Corresponds to ECMA-376 ST_ShapeType 'parallelogram'"]
        Parallelogram,
        #[doc = "Pentagon shape. Corresponds to ECMA-376 ST_ShapeType 'pentagon'"]
        Pentagon,
        #[doc = "Pie shape. Corresponds to ECMA-376 ST_ShapeType 'pie'"]
        Pie,
        #[doc = "Plaque shape. Corresponds to ECMA-376 ST_ShapeType 'plaque'"]
        Plaque,
        #[doc = "Plus shape. Corresponds to ECMA-376 ST_ShapeType 'plus'"]
        Plus,
        #[doc = "Quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrow'"]
        QuadArrow,
        #[doc = "Callout quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'quadArrowCallout'"]
        QuadArrowCallout,
        #[doc = "Rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'rect'."]
        Rectangle,
        #[doc = "Ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon'"]
        Ribbon,
        #[doc = "Ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon2'"]
        Ribbon2,
        #[doc = "Right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrow'"]
        RightArrow,
        #[doc = "Callout right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'rightArrowCallout'"]
        RightArrowCallout,
        #[doc = "Right brace shape. Corresponds to ECMA-376 ST_ShapeType 'rightBrace'"]
        RightBrace,
        #[doc = "Right bracket shape. Corresponds to ECMA-376 ST_ShapeType 'rightBracket'"]
        RightBracket,
        #[doc = "Right triangle shape. Corresponds to ECMA-376 ST_ShapeType 'rtTriangle'"]
        RightTriangle,
        #[doc = "One round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'round1Rect'"]
        Round1Rectangle,
        #[doc = "Two diagonal round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2DiagRect'"]
        Round2DiagonalRectangle,
        #[doc = "Two same-side round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2SameRect'"]
        Round2SameRectangle,
        #[doc = "Round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'roundRect'"]
        RoundRectangle,
        #[doc = "Smiley face shape. Corresponds to ECMA-376 ST_ShapeType 'smileyFace'"]
        SmileyFace,
        #[doc = "One snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'snip1Rect'"]
        Snip1Rectangle,
        #[doc = "Two diagonal snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2DiagRect'"]
        Snip2DiagonalRectangle,
        #[doc = "Two same-side snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2SameRect'"]
        Snip2SameRectangle,
        #[doc = "One snip one round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snipRoundRect'"]
        SnipRoundRectangle,
        #[doc = "Speech shape."]
        Speech,
        #[doc = "Ten pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star10'"]
        Star10,
        #[doc = "Twelve pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star12'"]
        Star12,
        #[doc = "Sixteen pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star16'"]
        Star16,
        #[doc = "Twenty four pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star24'"]
        Star24,
        #[doc = "Thirty two pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star32'"]
        Star32,
        #[doc = "Four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star4'"]
        Star4,
        #[doc = "Five pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star5'"]
        Star5,
        #[doc = "Six pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star6'"]
        Star6,
        #[doc = "Seven pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star7'"]
        Star7,
        #[doc = "Eight pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star8'"]
        Star8,
        #[doc = "Star burst shape."]
        Starburst,
        #[doc = "Striped right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'stripedRightArrow'"]
        StripedRightArrow,
        #[doc = "Sun shape. Corresponds to ECMA-376 ST_ShapeType 'sun'"]
        Sun,
        #[doc = "Teardrop shape. Corresponds to ECMA-376 ST_ShapeType 'teardrop'"]
        Teardrop,
        #[doc = "Text box shape."]
        TextBox,
        #[doc = "Trapezoid shape. Corresponds to ECMA-376 ST_ShapeType 'trapezoid'"]
        Trapezoid,
        #[doc = "Triangle shape. Corresponds to ECMA-376 ST_ShapeType 'triangle'"]
        Triangle,
        #[doc = "The shape type that is not predefined."]
        TypeUnspecified,
        #[doc = "Up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrow'"]
        UpArrow,
        #[doc = "Callout up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'upArrowCallout'"]
        UpArrowCallout,
        #[doc = "Up down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upDownArrow'"]
        UpDownArrow,
        #[doc = "U-turn arrow shape. Corresponds to ECMA-376 ST_ShapeType 'uturnArrow'"]
        UturnArrow,
        #[doc = "Vertical scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'verticalScroll'"]
        VerticalScroll,
        #[doc = "Wave shape. Corresponds to ECMA-376 ST_ShapeType 'wave'"]
        Wave,
        #[doc = "Callout wedge ellipse shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeEllipseCallout'"]
        WedgeEllipseCallout,
        #[doc = "Callout wedge rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRectCallout'"]
        WedgeRectangleCallout,
        #[doc = "Callout wedge round rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRoundRectCallout'"]
        WedgeRoundRectangleCallout,
    }
    impl ShapeShapeType {
        pub fn as_str(self) -> &'static str {
            match self {
                ShapeShapeType::Arc => "ARC",
                ShapeShapeType::ArrowEast => "ARROW_EAST",
                ShapeShapeType::ArrowNorth => "ARROW_NORTH",
                ShapeShapeType::ArrowNorthEast => "ARROW_NORTH_EAST",
                ShapeShapeType::BentArrow => "BENT_ARROW",
                ShapeShapeType::BentUpArrow => "BENT_UP_ARROW",
                ShapeShapeType::Bevel => "BEVEL",
                ShapeShapeType::BlockArc => "BLOCK_ARC",
                ShapeShapeType::BracePair => "BRACE_PAIR",
                ShapeShapeType::BracketPair => "BRACKET_PAIR",
                ShapeShapeType::Can => "CAN",
                ShapeShapeType::Chevron => "CHEVRON",
                ShapeShapeType::Chord => "CHORD",
                ShapeShapeType::Cloud => "CLOUD",
                ShapeShapeType::CloudCallout => "CLOUD_CALLOUT",
                ShapeShapeType::Corner => "CORNER",
                ShapeShapeType::Cube => "CUBE",
                ShapeShapeType::CurvedDownArrow => "CURVED_DOWN_ARROW",
                ShapeShapeType::CurvedLeftArrow => "CURVED_LEFT_ARROW",
                ShapeShapeType::CurvedRightArrow => "CURVED_RIGHT_ARROW",
                ShapeShapeType::CurvedUpArrow => "CURVED_UP_ARROW",
                ShapeShapeType::Custom => "CUSTOM",
                ShapeShapeType::Decagon => "DECAGON",
                ShapeShapeType::DiagonalStripe => "DIAGONAL_STRIPE",
                ShapeShapeType::Diamond => "DIAMOND",
                ShapeShapeType::Dodecagon => "DODECAGON",
                ShapeShapeType::Donut => "DONUT",
                ShapeShapeType::DoubleWave => "DOUBLE_WAVE",
                ShapeShapeType::DownArrow => "DOWN_ARROW",
                ShapeShapeType::DownArrowCallout => "DOWN_ARROW_CALLOUT",
                ShapeShapeType::Ellipse => "ELLIPSE",
                ShapeShapeType::EllipseRibbon => "ELLIPSE_RIBBON",
                ShapeShapeType::EllipseRibbon2 => "ELLIPSE_RIBBON_2",
                ShapeShapeType::FlowChartAlternateProcess => "FLOW_CHART_ALTERNATE_PROCESS",
                ShapeShapeType::FlowChartCollate => "FLOW_CHART_COLLATE",
                ShapeShapeType::FlowChartConnector => "FLOW_CHART_CONNECTOR",
                ShapeShapeType::FlowChartDecision => "FLOW_CHART_DECISION",
                ShapeShapeType::FlowChartDelay => "FLOW_CHART_DELAY",
                ShapeShapeType::FlowChartDisplay => "FLOW_CHART_DISPLAY",
                ShapeShapeType::FlowChartDocument => "FLOW_CHART_DOCUMENT",
                ShapeShapeType::FlowChartExtract => "FLOW_CHART_EXTRACT",
                ShapeShapeType::FlowChartInputOutput => "FLOW_CHART_INPUT_OUTPUT",
                ShapeShapeType::FlowChartInternalStorage => "FLOW_CHART_INTERNAL_STORAGE",
                ShapeShapeType::FlowChartMagneticDisk => "FLOW_CHART_MAGNETIC_DISK",
                ShapeShapeType::FlowChartMagneticDrum => "FLOW_CHART_MAGNETIC_DRUM",
                ShapeShapeType::FlowChartMagneticTape => "FLOW_CHART_MAGNETIC_TAPE",
                ShapeShapeType::FlowChartManualInput => "FLOW_CHART_MANUAL_INPUT",
                ShapeShapeType::FlowChartManualOperation => "FLOW_CHART_MANUAL_OPERATION",
                ShapeShapeType::FlowChartMerge => "FLOW_CHART_MERGE",
                ShapeShapeType::FlowChartMultidocument => "FLOW_CHART_MULTIDOCUMENT",
                ShapeShapeType::FlowChartOfflineStorage => "FLOW_CHART_OFFLINE_STORAGE",
                ShapeShapeType::FlowChartOffpageConnector => "FLOW_CHART_OFFPAGE_CONNECTOR",
                ShapeShapeType::FlowChartOnlineStorage => "FLOW_CHART_ONLINE_STORAGE",
                ShapeShapeType::FlowChartOr => "FLOW_CHART_OR",
                ShapeShapeType::FlowChartPredefinedProcess => "FLOW_CHART_PREDEFINED_PROCESS",
                ShapeShapeType::FlowChartPreparation => "FLOW_CHART_PREPARATION",
                ShapeShapeType::FlowChartProcess => "FLOW_CHART_PROCESS",
                ShapeShapeType::FlowChartPunchedCard => "FLOW_CHART_PUNCHED_CARD",
                ShapeShapeType::FlowChartPunchedTape => "FLOW_CHART_PUNCHED_TAPE",
                ShapeShapeType::FlowChartSort => "FLOW_CHART_SORT",
                ShapeShapeType::FlowChartSummingJunction => "FLOW_CHART_SUMMING_JUNCTION",
                ShapeShapeType::FlowChartTerminator => "FLOW_CHART_TERMINATOR",
                ShapeShapeType::FoldedCorner => "FOLDED_CORNER",
                ShapeShapeType::Frame => "FRAME",
                ShapeShapeType::HalfFrame => "HALF_FRAME",
                ShapeShapeType::Heart => "HEART",
                ShapeShapeType::Heptagon => "HEPTAGON",
                ShapeShapeType::Hexagon => "HEXAGON",
                ShapeShapeType::HomePlate => "HOME_PLATE",
                ShapeShapeType::HorizontalScroll => "HORIZONTAL_SCROLL",
                ShapeShapeType::IrregularSeal1 => "IRREGULAR_SEAL_1",
                ShapeShapeType::IrregularSeal2 => "IRREGULAR_SEAL_2",
                ShapeShapeType::LeftArrow => "LEFT_ARROW",
                ShapeShapeType::LeftArrowCallout => "LEFT_ARROW_CALLOUT",
                ShapeShapeType::LeftBrace => "LEFT_BRACE",
                ShapeShapeType::LeftBracket => "LEFT_BRACKET",
                ShapeShapeType::LeftRightArrow => "LEFT_RIGHT_ARROW",
                ShapeShapeType::LeftRightArrowCallout => "LEFT_RIGHT_ARROW_CALLOUT",
                ShapeShapeType::LeftRightUpArrow => "LEFT_RIGHT_UP_ARROW",
                ShapeShapeType::LeftUpArrow => "LEFT_UP_ARROW",
                ShapeShapeType::LightningBolt => "LIGHTNING_BOLT",
                ShapeShapeType::MathDivide => "MATH_DIVIDE",
                ShapeShapeType::MathEqual => "MATH_EQUAL",
                ShapeShapeType::MathMinus => "MATH_MINUS",
                ShapeShapeType::MathMultiply => "MATH_MULTIPLY",
                ShapeShapeType::MathNotEqual => "MATH_NOT_EQUAL",
                ShapeShapeType::MathPlus => "MATH_PLUS",
                ShapeShapeType::Moon => "MOON",
                ShapeShapeType::NoSmoking => "NO_SMOKING",
                ShapeShapeType::NotchedRightArrow => "NOTCHED_RIGHT_ARROW",
                ShapeShapeType::Octagon => "OCTAGON",
                ShapeShapeType::Parallelogram => "PARALLELOGRAM",
                ShapeShapeType::Pentagon => "PENTAGON",
                ShapeShapeType::Pie => "PIE",
                ShapeShapeType::Plaque => "PLAQUE",
                ShapeShapeType::Plus => "PLUS",
                ShapeShapeType::QuadArrow => "QUAD_ARROW",
                ShapeShapeType::QuadArrowCallout => "QUAD_ARROW_CALLOUT",
                ShapeShapeType::Rectangle => "RECTANGLE",
                ShapeShapeType::Ribbon => "RIBBON",
                ShapeShapeType::Ribbon2 => "RIBBON_2",
                ShapeShapeType::RightArrow => "RIGHT_ARROW",
                ShapeShapeType::RightArrowCallout => "RIGHT_ARROW_CALLOUT",
                ShapeShapeType::RightBrace => "RIGHT_BRACE",
                ShapeShapeType::RightBracket => "RIGHT_BRACKET",
                ShapeShapeType::RightTriangle => "RIGHT_TRIANGLE",
                ShapeShapeType::Round1Rectangle => "ROUND_1_RECTANGLE",
                ShapeShapeType::Round2DiagonalRectangle => "ROUND_2_DIAGONAL_RECTANGLE",
                ShapeShapeType::Round2SameRectangle => "ROUND_2_SAME_RECTANGLE",
                ShapeShapeType::RoundRectangle => "ROUND_RECTANGLE",
                ShapeShapeType::SmileyFace => "SMILEY_FACE",
                ShapeShapeType::Snip1Rectangle => "SNIP_1_RECTANGLE",
                ShapeShapeType::Snip2DiagonalRectangle => "SNIP_2_DIAGONAL_RECTANGLE",
                ShapeShapeType::Snip2SameRectangle => "SNIP_2_SAME_RECTANGLE",
                ShapeShapeType::SnipRoundRectangle => "SNIP_ROUND_RECTANGLE",
                ShapeShapeType::Speech => "SPEECH",
                ShapeShapeType::Star10 => "STAR_10",
                ShapeShapeType::Star12 => "STAR_12",
                ShapeShapeType::Star16 => "STAR_16",
                ShapeShapeType::Star24 => "STAR_24",
                ShapeShapeType::Star32 => "STAR_32",
                ShapeShapeType::Star4 => "STAR_4",
                ShapeShapeType::Star5 => "STAR_5",
                ShapeShapeType::Star6 => "STAR_6",
                ShapeShapeType::Star7 => "STAR_7",
                ShapeShapeType::Star8 => "STAR_8",
                ShapeShapeType::Starburst => "STARBURST",
                ShapeShapeType::StripedRightArrow => "STRIPED_RIGHT_ARROW",
                ShapeShapeType::Sun => "SUN",
                ShapeShapeType::Teardrop => "TEARDROP",
                ShapeShapeType::TextBox => "TEXT_BOX",
                ShapeShapeType::Trapezoid => "TRAPEZOID",
                ShapeShapeType::Triangle => "TRIANGLE",
                ShapeShapeType::TypeUnspecified => "TYPE_UNSPECIFIED",
                ShapeShapeType::UpArrow => "UP_ARROW",
                ShapeShapeType::UpArrowCallout => "UP_ARROW_CALLOUT",
                ShapeShapeType::UpDownArrow => "UP_DOWN_ARROW",
                ShapeShapeType::UturnArrow => "UTURN_ARROW",
                ShapeShapeType::VerticalScroll => "VERTICAL_SCROLL",
                ShapeShapeType::Wave => "WAVE",
                ShapeShapeType::WedgeEllipseCallout => "WEDGE_ELLIPSE_CALLOUT",
                ShapeShapeType::WedgeRectangleCallout => "WEDGE_RECTANGLE_CALLOUT",
                ShapeShapeType::WedgeRoundRectangleCallout => "WEDGE_ROUND_RECTANGLE_CALLOUT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ShapeShapeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ShapeShapeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ShapeShapeType, ()> {
            Ok(match s {
                "ARC" => ShapeShapeType::Arc,
                "ARROW_EAST" => ShapeShapeType::ArrowEast,
                "ARROW_NORTH" => ShapeShapeType::ArrowNorth,
                "ARROW_NORTH_EAST" => ShapeShapeType::ArrowNorthEast,
                "BENT_ARROW" => ShapeShapeType::BentArrow,
                "BENT_UP_ARROW" => ShapeShapeType::BentUpArrow,
                "BEVEL" => ShapeShapeType::Bevel,
                "BLOCK_ARC" => ShapeShapeType::BlockArc,
                "BRACE_PAIR" => ShapeShapeType::BracePair,
                "BRACKET_PAIR" => ShapeShapeType::BracketPair,
                "CAN" => ShapeShapeType::Can,
                "CHEVRON" => ShapeShapeType::Chevron,
                "CHORD" => ShapeShapeType::Chord,
                "CLOUD" => ShapeShapeType::Cloud,
                "CLOUD_CALLOUT" => ShapeShapeType::CloudCallout,
                "CORNER" => ShapeShapeType::Corner,
                "CUBE" => ShapeShapeType::Cube,
                "CURVED_DOWN_ARROW" => ShapeShapeType::CurvedDownArrow,
                "CURVED_LEFT_ARROW" => ShapeShapeType::CurvedLeftArrow,
                "CURVED_RIGHT_ARROW" => ShapeShapeType::CurvedRightArrow,
                "CURVED_UP_ARROW" => ShapeShapeType::CurvedUpArrow,
                "CUSTOM" => ShapeShapeType::Custom,
                "DECAGON" => ShapeShapeType::Decagon,
                "DIAGONAL_STRIPE" => ShapeShapeType::DiagonalStripe,
                "DIAMOND" => ShapeShapeType::Diamond,
                "DODECAGON" => ShapeShapeType::Dodecagon,
                "DONUT" => ShapeShapeType::Donut,
                "DOUBLE_WAVE" => ShapeShapeType::DoubleWave,
                "DOWN_ARROW" => ShapeShapeType::DownArrow,
                "DOWN_ARROW_CALLOUT" => ShapeShapeType::DownArrowCallout,
                "ELLIPSE" => ShapeShapeType::Ellipse,
                "ELLIPSE_RIBBON" => ShapeShapeType::EllipseRibbon,
                "ELLIPSE_RIBBON_2" => ShapeShapeType::EllipseRibbon2,
                "FLOW_CHART_ALTERNATE_PROCESS" => ShapeShapeType::FlowChartAlternateProcess,
                "FLOW_CHART_COLLATE" => ShapeShapeType::FlowChartCollate,
                "FLOW_CHART_CONNECTOR" => ShapeShapeType::FlowChartConnector,
                "FLOW_CHART_DECISION" => ShapeShapeType::FlowChartDecision,
                "FLOW_CHART_DELAY" => ShapeShapeType::FlowChartDelay,
                "FLOW_CHART_DISPLAY" => ShapeShapeType::FlowChartDisplay,
                "FLOW_CHART_DOCUMENT" => ShapeShapeType::FlowChartDocument,
                "FLOW_CHART_EXTRACT" => ShapeShapeType::FlowChartExtract,
                "FLOW_CHART_INPUT_OUTPUT" => ShapeShapeType::FlowChartInputOutput,
                "FLOW_CHART_INTERNAL_STORAGE" => ShapeShapeType::FlowChartInternalStorage,
                "FLOW_CHART_MAGNETIC_DISK" => ShapeShapeType::FlowChartMagneticDisk,
                "FLOW_CHART_MAGNETIC_DRUM" => ShapeShapeType::FlowChartMagneticDrum,
                "FLOW_CHART_MAGNETIC_TAPE" => ShapeShapeType::FlowChartMagneticTape,
                "FLOW_CHART_MANUAL_INPUT" => ShapeShapeType::FlowChartManualInput,
                "FLOW_CHART_MANUAL_OPERATION" => ShapeShapeType::FlowChartManualOperation,
                "FLOW_CHART_MERGE" => ShapeShapeType::FlowChartMerge,
                "FLOW_CHART_MULTIDOCUMENT" => ShapeShapeType::FlowChartMultidocument,
                "FLOW_CHART_OFFLINE_STORAGE" => ShapeShapeType::FlowChartOfflineStorage,
                "FLOW_CHART_OFFPAGE_CONNECTOR" => ShapeShapeType::FlowChartOffpageConnector,
                "FLOW_CHART_ONLINE_STORAGE" => ShapeShapeType::FlowChartOnlineStorage,
                "FLOW_CHART_OR" => ShapeShapeType::FlowChartOr,
                "FLOW_CHART_PREDEFINED_PROCESS" => ShapeShapeType::FlowChartPredefinedProcess,
                "FLOW_CHART_PREPARATION" => ShapeShapeType::FlowChartPreparation,
                "FLOW_CHART_PROCESS" => ShapeShapeType::FlowChartProcess,
                "FLOW_CHART_PUNCHED_CARD" => ShapeShapeType::FlowChartPunchedCard,
                "FLOW_CHART_PUNCHED_TAPE" => ShapeShapeType::FlowChartPunchedTape,
                "FLOW_CHART_SORT" => ShapeShapeType::FlowChartSort,
                "FLOW_CHART_SUMMING_JUNCTION" => ShapeShapeType::FlowChartSummingJunction,
                "FLOW_CHART_TERMINATOR" => ShapeShapeType::FlowChartTerminator,
                "FOLDED_CORNER" => ShapeShapeType::FoldedCorner,
                "FRAME" => ShapeShapeType::Frame,
                "HALF_FRAME" => ShapeShapeType::HalfFrame,
                "HEART" => ShapeShapeType::Heart,
                "HEPTAGON" => ShapeShapeType::Heptagon,
                "HEXAGON" => ShapeShapeType::Hexagon,
                "HOME_PLATE" => ShapeShapeType::HomePlate,
                "HORIZONTAL_SCROLL" => ShapeShapeType::HorizontalScroll,
                "IRREGULAR_SEAL_1" => ShapeShapeType::IrregularSeal1,
                "IRREGULAR_SEAL_2" => ShapeShapeType::IrregularSeal2,
                "LEFT_ARROW" => ShapeShapeType::LeftArrow,
                "LEFT_ARROW_CALLOUT" => ShapeShapeType::LeftArrowCallout,
                "LEFT_BRACE" => ShapeShapeType::LeftBrace,
                "LEFT_BRACKET" => ShapeShapeType::LeftBracket,
                "LEFT_RIGHT_ARROW" => ShapeShapeType::LeftRightArrow,
                "LEFT_RIGHT_ARROW_CALLOUT" => ShapeShapeType::LeftRightArrowCallout,
                "LEFT_RIGHT_UP_ARROW" => ShapeShapeType::LeftRightUpArrow,
                "LEFT_UP_ARROW" => ShapeShapeType::LeftUpArrow,
                "LIGHTNING_BOLT" => ShapeShapeType::LightningBolt,
                "MATH_DIVIDE" => ShapeShapeType::MathDivide,
                "MATH_EQUAL" => ShapeShapeType::MathEqual,
                "MATH_MINUS" => ShapeShapeType::MathMinus,
                "MATH_MULTIPLY" => ShapeShapeType::MathMultiply,
                "MATH_NOT_EQUAL" => ShapeShapeType::MathNotEqual,
                "MATH_PLUS" => ShapeShapeType::MathPlus,
                "MOON" => ShapeShapeType::Moon,
                "NO_SMOKING" => ShapeShapeType::NoSmoking,
                "NOTCHED_RIGHT_ARROW" => ShapeShapeType::NotchedRightArrow,
                "OCTAGON" => ShapeShapeType::Octagon,
                "PARALLELOGRAM" => ShapeShapeType::Parallelogram,
                "PENTAGON" => ShapeShapeType::Pentagon,
                "PIE" => ShapeShapeType::Pie,
                "PLAQUE" => ShapeShapeType::Plaque,
                "PLUS" => ShapeShapeType::Plus,
                "QUAD_ARROW" => ShapeShapeType::QuadArrow,
                "QUAD_ARROW_CALLOUT" => ShapeShapeType::QuadArrowCallout,
                "RECTANGLE" => ShapeShapeType::Rectangle,
                "RIBBON" => ShapeShapeType::Ribbon,
                "RIBBON_2" => ShapeShapeType::Ribbon2,
                "RIGHT_ARROW" => ShapeShapeType::RightArrow,
                "RIGHT_ARROW_CALLOUT" => ShapeShapeType::RightArrowCallout,
                "RIGHT_BRACE" => ShapeShapeType::RightBrace,
                "RIGHT_BRACKET" => ShapeShapeType::RightBracket,
                "RIGHT_TRIANGLE" => ShapeShapeType::RightTriangle,
                "ROUND_1_RECTANGLE" => ShapeShapeType::Round1Rectangle,
                "ROUND_2_DIAGONAL_RECTANGLE" => ShapeShapeType::Round2DiagonalRectangle,
                "ROUND_2_SAME_RECTANGLE" => ShapeShapeType::Round2SameRectangle,
                "ROUND_RECTANGLE" => ShapeShapeType::RoundRectangle,
                "SMILEY_FACE" => ShapeShapeType::SmileyFace,
                "SNIP_1_RECTANGLE" => ShapeShapeType::Snip1Rectangle,
                "SNIP_2_DIAGONAL_RECTANGLE" => ShapeShapeType::Snip2DiagonalRectangle,
                "SNIP_2_SAME_RECTANGLE" => ShapeShapeType::Snip2SameRectangle,
                "SNIP_ROUND_RECTANGLE" => ShapeShapeType::SnipRoundRectangle,
                "SPEECH" => ShapeShapeType::Speech,
                "STAR_10" => ShapeShapeType::Star10,
                "STAR_12" => ShapeShapeType::Star12,
                "STAR_16" => ShapeShapeType::Star16,
                "STAR_24" => ShapeShapeType::Star24,
                "STAR_32" => ShapeShapeType::Star32,
                "STAR_4" => ShapeShapeType::Star4,
                "STAR_5" => ShapeShapeType::Star5,
                "STAR_6" => ShapeShapeType::Star6,
                "STAR_7" => ShapeShapeType::Star7,
                "STAR_8" => ShapeShapeType::Star8,
                "STARBURST" => ShapeShapeType::Starburst,
                "STRIPED_RIGHT_ARROW" => ShapeShapeType::StripedRightArrow,
                "SUN" => ShapeShapeType::Sun,
                "TEARDROP" => ShapeShapeType::Teardrop,
                "TEXT_BOX" => ShapeShapeType::TextBox,
                "TRAPEZOID" => ShapeShapeType::Trapezoid,
                "TRIANGLE" => ShapeShapeType::Triangle,
                "TYPE_UNSPECIFIED" => ShapeShapeType::TypeUnspecified,
                "UP_ARROW" => ShapeShapeType::UpArrow,
                "UP_ARROW_CALLOUT" => ShapeShapeType::UpArrowCallout,
                "UP_DOWN_ARROW" => ShapeShapeType::UpDownArrow,
                "UTURN_ARROW" => ShapeShapeType::UturnArrow,
                "VERTICAL_SCROLL" => ShapeShapeType::VerticalScroll,
                "WAVE" => ShapeShapeType::Wave,
                "WEDGE_ELLIPSE_CALLOUT" => ShapeShapeType::WedgeEllipseCallout,
                "WEDGE_RECTANGLE_CALLOUT" => ShapeShapeType::WedgeRectangleCallout,
                "WEDGE_ROUND_RECTANGLE_CALLOUT" => ShapeShapeType::WedgeRoundRectangleCallout,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ShapeShapeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShapeShapeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShapeShapeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARC" => ShapeShapeType::Arc,
                "ARROW_EAST" => ShapeShapeType::ArrowEast,
                "ARROW_NORTH" => ShapeShapeType::ArrowNorth,
                "ARROW_NORTH_EAST" => ShapeShapeType::ArrowNorthEast,
                "BENT_ARROW" => ShapeShapeType::BentArrow,
                "BENT_UP_ARROW" => ShapeShapeType::BentUpArrow,
                "BEVEL" => ShapeShapeType::Bevel,
                "BLOCK_ARC" => ShapeShapeType::BlockArc,
                "BRACE_PAIR" => ShapeShapeType::BracePair,
                "BRACKET_PAIR" => ShapeShapeType::BracketPair,
                "CAN" => ShapeShapeType::Can,
                "CHEVRON" => ShapeShapeType::Chevron,
                "CHORD" => ShapeShapeType::Chord,
                "CLOUD" => ShapeShapeType::Cloud,
                "CLOUD_CALLOUT" => ShapeShapeType::CloudCallout,
                "CORNER" => ShapeShapeType::Corner,
                "CUBE" => ShapeShapeType::Cube,
                "CURVED_DOWN_ARROW" => ShapeShapeType::CurvedDownArrow,
                "CURVED_LEFT_ARROW" => ShapeShapeType::CurvedLeftArrow,
                "CURVED_RIGHT_ARROW" => ShapeShapeType::CurvedRightArrow,
                "CURVED_UP_ARROW" => ShapeShapeType::CurvedUpArrow,
                "CUSTOM" => ShapeShapeType::Custom,
                "DECAGON" => ShapeShapeType::Decagon,
                "DIAGONAL_STRIPE" => ShapeShapeType::DiagonalStripe,
                "DIAMOND" => ShapeShapeType::Diamond,
                "DODECAGON" => ShapeShapeType::Dodecagon,
                "DONUT" => ShapeShapeType::Donut,
                "DOUBLE_WAVE" => ShapeShapeType::DoubleWave,
                "DOWN_ARROW" => ShapeShapeType::DownArrow,
                "DOWN_ARROW_CALLOUT" => ShapeShapeType::DownArrowCallout,
                "ELLIPSE" => ShapeShapeType::Ellipse,
                "ELLIPSE_RIBBON" => ShapeShapeType::EllipseRibbon,
                "ELLIPSE_RIBBON_2" => ShapeShapeType::EllipseRibbon2,
                "FLOW_CHART_ALTERNATE_PROCESS" => ShapeShapeType::FlowChartAlternateProcess,
                "FLOW_CHART_COLLATE" => ShapeShapeType::FlowChartCollate,
                "FLOW_CHART_CONNECTOR" => ShapeShapeType::FlowChartConnector,
                "FLOW_CHART_DECISION" => ShapeShapeType::FlowChartDecision,
                "FLOW_CHART_DELAY" => ShapeShapeType::FlowChartDelay,
                "FLOW_CHART_DISPLAY" => ShapeShapeType::FlowChartDisplay,
                "FLOW_CHART_DOCUMENT" => ShapeShapeType::FlowChartDocument,
                "FLOW_CHART_EXTRACT" => ShapeShapeType::FlowChartExtract,
                "FLOW_CHART_INPUT_OUTPUT" => ShapeShapeType::FlowChartInputOutput,
                "FLOW_CHART_INTERNAL_STORAGE" => ShapeShapeType::FlowChartInternalStorage,
                "FLOW_CHART_MAGNETIC_DISK" => ShapeShapeType::FlowChartMagneticDisk,
                "FLOW_CHART_MAGNETIC_DRUM" => ShapeShapeType::FlowChartMagneticDrum,
                "FLOW_CHART_MAGNETIC_TAPE" => ShapeShapeType::FlowChartMagneticTape,
                "FLOW_CHART_MANUAL_INPUT" => ShapeShapeType::FlowChartManualInput,
                "FLOW_CHART_MANUAL_OPERATION" => ShapeShapeType::FlowChartManualOperation,
                "FLOW_CHART_MERGE" => ShapeShapeType::FlowChartMerge,
                "FLOW_CHART_MULTIDOCUMENT" => ShapeShapeType::FlowChartMultidocument,
                "FLOW_CHART_OFFLINE_STORAGE" => ShapeShapeType::FlowChartOfflineStorage,
                "FLOW_CHART_OFFPAGE_CONNECTOR" => ShapeShapeType::FlowChartOffpageConnector,
                "FLOW_CHART_ONLINE_STORAGE" => ShapeShapeType::FlowChartOnlineStorage,
                "FLOW_CHART_OR" => ShapeShapeType::FlowChartOr,
                "FLOW_CHART_PREDEFINED_PROCESS" => ShapeShapeType::FlowChartPredefinedProcess,
                "FLOW_CHART_PREPARATION" => ShapeShapeType::FlowChartPreparation,
                "FLOW_CHART_PROCESS" => ShapeShapeType::FlowChartProcess,
                "FLOW_CHART_PUNCHED_CARD" => ShapeShapeType::FlowChartPunchedCard,
                "FLOW_CHART_PUNCHED_TAPE" => ShapeShapeType::FlowChartPunchedTape,
                "FLOW_CHART_SORT" => ShapeShapeType::FlowChartSort,
                "FLOW_CHART_SUMMING_JUNCTION" => ShapeShapeType::FlowChartSummingJunction,
                "FLOW_CHART_TERMINATOR" => ShapeShapeType::FlowChartTerminator,
                "FOLDED_CORNER" => ShapeShapeType::FoldedCorner,
                "FRAME" => ShapeShapeType::Frame,
                "HALF_FRAME" => ShapeShapeType::HalfFrame,
                "HEART" => ShapeShapeType::Heart,
                "HEPTAGON" => ShapeShapeType::Heptagon,
                "HEXAGON" => ShapeShapeType::Hexagon,
                "HOME_PLATE" => ShapeShapeType::HomePlate,
                "HORIZONTAL_SCROLL" => ShapeShapeType::HorizontalScroll,
                "IRREGULAR_SEAL_1" => ShapeShapeType::IrregularSeal1,
                "IRREGULAR_SEAL_2" => ShapeShapeType::IrregularSeal2,
                "LEFT_ARROW" => ShapeShapeType::LeftArrow,
                "LEFT_ARROW_CALLOUT" => ShapeShapeType::LeftArrowCallout,
                "LEFT_BRACE" => ShapeShapeType::LeftBrace,
                "LEFT_BRACKET" => ShapeShapeType::LeftBracket,
                "LEFT_RIGHT_ARROW" => ShapeShapeType::LeftRightArrow,
                "LEFT_RIGHT_ARROW_CALLOUT" => ShapeShapeType::LeftRightArrowCallout,
                "LEFT_RIGHT_UP_ARROW" => ShapeShapeType::LeftRightUpArrow,
                "LEFT_UP_ARROW" => ShapeShapeType::LeftUpArrow,
                "LIGHTNING_BOLT" => ShapeShapeType::LightningBolt,
                "MATH_DIVIDE" => ShapeShapeType::MathDivide,
                "MATH_EQUAL" => ShapeShapeType::MathEqual,
                "MATH_MINUS" => ShapeShapeType::MathMinus,
                "MATH_MULTIPLY" => ShapeShapeType::MathMultiply,
                "MATH_NOT_EQUAL" => ShapeShapeType::MathNotEqual,
                "MATH_PLUS" => ShapeShapeType::MathPlus,
                "MOON" => ShapeShapeType::Moon,
                "NO_SMOKING" => ShapeShapeType::NoSmoking,
                "NOTCHED_RIGHT_ARROW" => ShapeShapeType::NotchedRightArrow,
                "OCTAGON" => ShapeShapeType::Octagon,
                "PARALLELOGRAM" => ShapeShapeType::Parallelogram,
                "PENTAGON" => ShapeShapeType::Pentagon,
                "PIE" => ShapeShapeType::Pie,
                "PLAQUE" => ShapeShapeType::Plaque,
                "PLUS" => ShapeShapeType::Plus,
                "QUAD_ARROW" => ShapeShapeType::QuadArrow,
                "QUAD_ARROW_CALLOUT" => ShapeShapeType::QuadArrowCallout,
                "RECTANGLE" => ShapeShapeType::Rectangle,
                "RIBBON" => ShapeShapeType::Ribbon,
                "RIBBON_2" => ShapeShapeType::Ribbon2,
                "RIGHT_ARROW" => ShapeShapeType::RightArrow,
                "RIGHT_ARROW_CALLOUT" => ShapeShapeType::RightArrowCallout,
                "RIGHT_BRACE" => ShapeShapeType::RightBrace,
                "RIGHT_BRACKET" => ShapeShapeType::RightBracket,
                "RIGHT_TRIANGLE" => ShapeShapeType::RightTriangle,
                "ROUND_1_RECTANGLE" => ShapeShapeType::Round1Rectangle,
                "ROUND_2_DIAGONAL_RECTANGLE" => ShapeShapeType::Round2DiagonalRectangle,
                "ROUND_2_SAME_RECTANGLE" => ShapeShapeType::Round2SameRectangle,
                "ROUND_RECTANGLE" => ShapeShapeType::RoundRectangle,
                "SMILEY_FACE" => ShapeShapeType::SmileyFace,
                "SNIP_1_RECTANGLE" => ShapeShapeType::Snip1Rectangle,
                "SNIP_2_DIAGONAL_RECTANGLE" => ShapeShapeType::Snip2DiagonalRectangle,
                "SNIP_2_SAME_RECTANGLE" => ShapeShapeType::Snip2SameRectangle,
                "SNIP_ROUND_RECTANGLE" => ShapeShapeType::SnipRoundRectangle,
                "SPEECH" => ShapeShapeType::Speech,
                "STAR_10" => ShapeShapeType::Star10,
                "STAR_12" => ShapeShapeType::Star12,
                "STAR_16" => ShapeShapeType::Star16,
                "STAR_24" => ShapeShapeType::Star24,
                "STAR_32" => ShapeShapeType::Star32,
                "STAR_4" => ShapeShapeType::Star4,
                "STAR_5" => ShapeShapeType::Star5,
                "STAR_6" => ShapeShapeType::Star6,
                "STAR_7" => ShapeShapeType::Star7,
                "STAR_8" => ShapeShapeType::Star8,
                "STARBURST" => ShapeShapeType::Starburst,
                "STRIPED_RIGHT_ARROW" => ShapeShapeType::StripedRightArrow,
                "SUN" => ShapeShapeType::Sun,
                "TEARDROP" => ShapeShapeType::Teardrop,
                "TEXT_BOX" => ShapeShapeType::TextBox,
                "TRAPEZOID" => ShapeShapeType::Trapezoid,
                "TRIANGLE" => ShapeShapeType::Triangle,
                "TYPE_UNSPECIFIED" => ShapeShapeType::TypeUnspecified,
                "UP_ARROW" => ShapeShapeType::UpArrow,
                "UP_ARROW_CALLOUT" => ShapeShapeType::UpArrowCallout,
                "UP_DOWN_ARROW" => ShapeShapeType::UpDownArrow,
                "UTURN_ARROW" => ShapeShapeType::UturnArrow,
                "VERTICAL_SCROLL" => ShapeShapeType::VerticalScroll,
                "WAVE" => ShapeShapeType::Wave,
                "WEDGE_ELLIPSE_CALLOUT" => ShapeShapeType::WedgeEllipseCallout,
                "WEDGE_RECTANGLE_CALLOUT" => ShapeShapeType::WedgeRectangleCallout,
                "WEDGE_ROUND_RECTANGLE_CALLOUT" => ShapeShapeType::WedgeRoundRectangleCallout,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ShapeShapeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShapeShapeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ShapeBackgroundFill {
        #[doc = "The background fill property state.\n\nUpdating the fill on a shape will implicitly update this field to\n`RENDERED`, unless another value is specified in the same request. To\nhave no fill on a shape, set this field to `NOT_RENDERED`. In this case,\nany other fill fields set in the same request will be ignored."]
        #[serde(
            rename = "propertyState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_state: ::std::option::Option<crate::schemas::ShapeBackgroundFillPropertyState>,
        #[doc = "Solid color fill."]
        #[serde(
            rename = "solidFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub solid_fill: ::std::option::Option<crate::schemas::SolidFill>,
    }
    impl ::google_field_selector::FieldSelector for ShapeBackgroundFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShapeBackgroundFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShapeBackgroundFillPropertyState {
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
    }
    impl ShapeBackgroundFillPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                ShapeBackgroundFillPropertyState::Inherit => "INHERIT",
                ShapeBackgroundFillPropertyState::NotRendered => "NOT_RENDERED",
                ShapeBackgroundFillPropertyState::Rendered => "RENDERED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ShapeBackgroundFillPropertyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ShapeBackgroundFillPropertyState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ShapeBackgroundFillPropertyState, ()> {
            Ok(match s {
                "INHERIT" => ShapeBackgroundFillPropertyState::Inherit,
                "NOT_RENDERED" => ShapeBackgroundFillPropertyState::NotRendered,
                "RENDERED" => ShapeBackgroundFillPropertyState::Rendered,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ShapeBackgroundFillPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShapeBackgroundFillPropertyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShapeBackgroundFillPropertyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INHERIT" => ShapeBackgroundFillPropertyState::Inherit,
                "NOT_RENDERED" => ShapeBackgroundFillPropertyState::NotRendered,
                "RENDERED" => ShapeBackgroundFillPropertyState::Rendered,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ShapeBackgroundFillPropertyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShapeBackgroundFillPropertyState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ShapeProperties {
        #[doc = "The alignment of the content in the shape. If unspecified,\nthe alignment is inherited from a parent placeholder if it exists. If the\nshape has no parent, the default alignment matches the alignment for new\nshapes created in the Slides editor."]
        #[serde(
            rename = "contentAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_alignment:
            ::std::option::Option<crate::schemas::ShapePropertiesContentAlignment>,
        #[doc = "The hyperlink destination of the shape. If unset, there is no link. Links\nare not inherited from parent placeholders."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<crate::schemas::Link>,
        #[doc = "The outline of the shape. If unset, the outline is inherited from a\nparent placeholder if it exists. If the shape has no parent, then the\ndefault outline depends on the shape type, matching the defaults for\nnew shapes created in the Slides editor."]
        #[serde(
            rename = "outline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outline: ::std::option::Option<crate::schemas::Outline>,
        #[doc = "The shadow properties of the shape. If unset, the shadow is inherited from\na parent placeholder if it exists. If the shape has no parent, then the\ndefault shadow matches the defaults for new shapes created in the Slides\neditor. This property is read-only."]
        #[serde(
            rename = "shadow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shadow: ::std::option::Option<crate::schemas::Shadow>,
        #[doc = "The background fill of the shape. If unset, the background fill is\ninherited from a parent placeholder if it exists. If the shape has no\nparent, then the default background fill depends on the shape type,\nmatching the defaults for new shapes created in the Slides editor."]
        #[serde(
            rename = "shapeBackgroundFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shape_background_fill: ::std::option::Option<crate::schemas::ShapeBackgroundFill>,
    }
    impl ::google_field_selector::FieldSelector for ShapeProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShapeProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShapePropertiesContentAlignment {
        #[doc = "An alignment that aligns the content to the bottom of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
        Bottom,
        #[doc = "An unspecified content alignment. The content alignment is inherited from\nthe parent if it exists."]
        ContentAlignmentUnspecified,
        #[doc = "An unsupported content alignment."]
        ContentAlignmentUnsupported,
        #[doc = "An alignment that aligns the content to the middle of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
        Middle,
        #[doc = "An alignment that aligns the content to the top of the content holder.\nCorresponds to ECMA-376 ST_TextAnchoringType 't'."]
        Top,
    }
    impl ShapePropertiesContentAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                ShapePropertiesContentAlignment::Bottom => "BOTTOM",
                ShapePropertiesContentAlignment::ContentAlignmentUnspecified => {
                    "CONTENT_ALIGNMENT_UNSPECIFIED"
                }
                ShapePropertiesContentAlignment::ContentAlignmentUnsupported => {
                    "CONTENT_ALIGNMENT_UNSUPPORTED"
                }
                ShapePropertiesContentAlignment::Middle => "MIDDLE",
                ShapePropertiesContentAlignment::Top => "TOP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ShapePropertiesContentAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ShapePropertiesContentAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ShapePropertiesContentAlignment, ()> {
            Ok(match s {
                "BOTTOM" => ShapePropertiesContentAlignment::Bottom,
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    ShapePropertiesContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    ShapePropertiesContentAlignment::ContentAlignmentUnsupported
                }
                "MIDDLE" => ShapePropertiesContentAlignment::Middle,
                "TOP" => ShapePropertiesContentAlignment::Top,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ShapePropertiesContentAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShapePropertiesContentAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShapePropertiesContentAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM" => ShapePropertiesContentAlignment::Bottom,
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    ShapePropertiesContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    ShapePropertiesContentAlignment::ContentAlignmentUnsupported
                }
                "MIDDLE" => ShapePropertiesContentAlignment::Middle,
                "TOP" => ShapePropertiesContentAlignment::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ShapePropertiesContentAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShapePropertiesContentAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SheetsChart {
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet that is\nembedded."]
        #[serde(
            rename = "chartId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_id: ::std::option::Option<i32>,
        #[doc = "The URL of an image of the embedded chart, with a default lifetime of 30\nminutes. This URL is tagged with the account of the requester. Anyone with\nthe URL effectively accesses the image as the original requester. Access to\nthe image may be lost if the presentation's sharing settings change."]
        #[serde(
            rename = "contentUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_url: ::std::option::Option<String>,
        #[doc = "The properties of the Sheets chart."]
        #[serde(
            rename = "sheetsChartProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheets_chart_properties: ::std::option::Option<crate::schemas::SheetsChartProperties>,
        #[doc = "The ID of the Google Sheets spreadsheet that contains the source chart."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SheetsChart {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SheetsChart {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SheetsChartProperties {
        #[doc = "The properties of the embedded chart image."]
        #[serde(
            rename = "chartImageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_image_properties: ::std::option::Option<crate::schemas::ImageProperties>,
    }
    impl ::google_field_selector::FieldSelector for SheetsChartProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SheetsChartProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Size {
        #[doc = "The height of the object."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The width of the object."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for Size {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Size {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SlideProperties {
        #[doc = "The object ID of the layout that this slide is based on. This property is\nread-only."]
        #[serde(
            rename = "layoutObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_object_id: ::std::option::Option<String>,
        #[doc = "The object ID of the master that this slide is based on. This property is\nread-only."]
        #[serde(
            rename = "masterObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub master_object_id: ::std::option::Option<String>,
        #[doc = "The notes page that this slide is associated with. It defines the visual\nappearance of a notes page when printing or exporting slides with speaker\nnotes. A notes page inherits properties from the\nnotes master.\nThe placeholder shape with type BODY on the notes page contains the speaker\nnotes for this slide. The ID of this shape is identified by the\nspeakerNotesObjectId field.\nThe notes page is read-only except for the text content and styles of the\nspeaker notes shape. This property is read-only."]
        #[serde(
            rename = "notesPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes_page: ::std::option::Option<Box<crate::schemas::Page>>,
    }
    impl ::google_field_selector::FieldSelector for SlideProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlideProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SolidFill {
        #[doc = "The fraction of this `color` that should be applied to the pixel.\nThat is, the final pixel color is defined by the equation:\n\npixel color = alpha * (color) + (1.0 - alpha) * (background color)\n\nThis means that a value of 1.0 corresponds to a solid color, whereas\na value of 0.0 corresponds to a completely transparent color."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The color value of the solid fill."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::OpaqueColor>,
    }
    impl ::google_field_selector::FieldSelector for SolidFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SolidFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StretchedPictureFill {
        #[doc = "Reading the content_url:\n\nAn URL to a picture with a default lifetime of 30 minutes.\nThis URL is tagged with the account of the requester. Anyone with the URL\neffectively accesses the picture as the original requester. Access to the\npicture may be lost if the presentation's sharing settings change.\n\nWriting the content_url:\n\nThe picture is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Pictures must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length."]
        #[serde(
            rename = "contentUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_url: ::std::option::Option<String>,
        #[doc = "The original size of the picture fill. This field is read-only."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<crate::schemas::Size>,
    }
    impl ::google_field_selector::FieldSelector for StretchedPictureFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StretchedPictureFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SubstringMatchCriteria {
        #[doc = "Indicates whether the search should respect case:\n\n* `True`: the search is case sensitive.\n* `False`: the search is case insensitive."]
        #[serde(
            rename = "matchCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_case: ::std::option::Option<bool>,
        #[doc = "The text to search for in the shape or table."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SubstringMatchCriteria {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SubstringMatchCriteria {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Table {
        #[doc = "Number of columns in the table."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<i32>,
        #[doc = "Properties of horizontal cell borders.\n\nA table's horizontal cell borders are represented as a grid. The grid has\none more row than the number of rows in the table and the same number of\ncolumns as the table. For example, if the table is 3 x 3, its horizontal\nborders will be represented as a grid with 4 rows and 3 columns."]
        #[serde(
            rename = "horizontalBorderRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub horizontal_border_rows: ::std::option::Option<Vec<crate::schemas::TableBorderRow>>,
        #[doc = "Number of rows in the table."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<i32>,
        #[doc = "Properties of each column."]
        #[serde(
            rename = "tableColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_columns: ::std::option::Option<Vec<crate::schemas::TableColumnProperties>>,
        #[doc = "Properties and contents of each row.\n\nCells that span multiple rows are contained in only one of these rows and\nhave a row_span greater\nthan 1."]
        #[serde(
            rename = "tableRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_rows: ::std::option::Option<Vec<crate::schemas::TableRow>>,
        #[doc = "Properties of vertical cell borders.\n\nA table's vertical cell borders are represented as a grid. The grid has the\nsame number of rows as the table and one more column than the number of\ncolumns in the table. For example, if the table is 3 x 3, its vertical\nborders will be represented as a grid with 3 rows and 4 columns."]
        #[serde(
            rename = "verticalBorderRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertical_border_rows: ::std::option::Option<Vec<crate::schemas::TableBorderRow>>,
    }
    impl ::google_field_selector::FieldSelector for Table {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Table {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderCell {
        #[doc = "The location of the border within the border table."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The border properties."]
        #[serde(
            rename = "tableBorderProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_border_properties: ::std::option::Option<crate::schemas::TableBorderProperties>,
    }
    impl ::google_field_selector::FieldSelector for TableBorderCell {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableBorderCell {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderFill {
        #[doc = "Solid fill."]
        #[serde(
            rename = "solidFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub solid_fill: ::std::option::Option<crate::schemas::SolidFill>,
    }
    impl ::google_field_selector::FieldSelector for TableBorderFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableBorderFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderProperties {
        #[doc = "The dash style of the border."]
        #[serde(
            rename = "dashStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dash_style: ::std::option::Option<crate::schemas::TableBorderPropertiesDashStyle>,
        #[doc = "The fill of the table border."]
        #[serde(
            rename = "tableBorderFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_border_fill: ::std::option::Option<crate::schemas::TableBorderFill>,
        #[doc = "The thickness of the border."]
        #[serde(
            rename = "weight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weight: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for TableBorderProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableBorderProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableBorderPropertiesDashStyle {
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'dashDot'."]
        DashDot,
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'lgDash'."]
        LongDash,
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376\nST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'.\nThis is the default dash style."]
        Solid,
    }
    impl TableBorderPropertiesDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                TableBorderPropertiesDashStyle::Dash => "DASH",
                TableBorderPropertiesDashStyle::DashDot => "DASH_DOT",
                TableBorderPropertiesDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                TableBorderPropertiesDashStyle::Dot => "DOT",
                TableBorderPropertiesDashStyle::LongDash => "LONG_DASH",
                TableBorderPropertiesDashStyle::LongDashDot => "LONG_DASH_DOT",
                TableBorderPropertiesDashStyle::Solid => "SOLID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TableBorderPropertiesDashStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TableBorderPropertiesDashStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TableBorderPropertiesDashStyle, ()> {
            Ok(match s {
                "DASH" => TableBorderPropertiesDashStyle::Dash,
                "DASH_DOT" => TableBorderPropertiesDashStyle::DashDot,
                "DASH_STYLE_UNSPECIFIED" => TableBorderPropertiesDashStyle::DashStyleUnspecified,
                "DOT" => TableBorderPropertiesDashStyle::Dot,
                "LONG_DASH" => TableBorderPropertiesDashStyle::LongDash,
                "LONG_DASH_DOT" => TableBorderPropertiesDashStyle::LongDashDot,
                "SOLID" => TableBorderPropertiesDashStyle::Solid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TableBorderPropertiesDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableBorderPropertiesDashStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableBorderPropertiesDashStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH" => TableBorderPropertiesDashStyle::Dash,
                "DASH_DOT" => TableBorderPropertiesDashStyle::DashDot,
                "DASH_STYLE_UNSPECIFIED" => TableBorderPropertiesDashStyle::DashStyleUnspecified,
                "DOT" => TableBorderPropertiesDashStyle::Dot,
                "LONG_DASH" => TableBorderPropertiesDashStyle::LongDash,
                "LONG_DASH_DOT" => TableBorderPropertiesDashStyle::LongDashDot,
                "SOLID" => TableBorderPropertiesDashStyle::Solid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TableBorderPropertiesDashStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableBorderPropertiesDashStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderRow {
        #[doc = "Properties of each border cell. When a border's adjacent table cells are\nmerged, it is not included in the response."]
        #[serde(
            rename = "tableBorderCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_border_cells: ::std::option::Option<Vec<crate::schemas::TableBorderCell>>,
    }
    impl ::google_field_selector::FieldSelector for TableBorderRow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableBorderRow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableCell {
        #[doc = "Column span of the cell."]
        #[serde(
            rename = "columnSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_span: ::std::option::Option<i32>,
        #[doc = "The location of the cell within the table."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "Row span of the cell."]
        #[serde(
            rename = "rowSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span: ::std::option::Option<i32>,
        #[doc = "The properties of the table cell."]
        #[serde(
            rename = "tableCellProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_properties: ::std::option::Option<crate::schemas::TableCellProperties>,
        #[doc = "The text content of the cell."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<crate::schemas::TextContent>,
    }
    impl ::google_field_selector::FieldSelector for TableCell {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCell {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableCellBackgroundFill {
        #[doc = "The background fill property state.\n\nUpdating the fill on a table cell will implicitly update this field\nto `RENDERED`, unless another value is specified in the same request. To\nhave no fill on a table cell, set this field to `NOT_RENDERED`. In this\ncase, any other fill fields set in the same request will be ignored."]
        #[serde(
            rename = "propertyState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_state:
            ::std::option::Option<crate::schemas::TableCellBackgroundFillPropertyState>,
        #[doc = "Solid color fill."]
        #[serde(
            rename = "solidFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub solid_fill: ::std::option::Option<crate::schemas::SolidFill>,
    }
    impl ::google_field_selector::FieldSelector for TableCellBackgroundFill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellBackgroundFill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableCellBackgroundFillPropertyState {
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
    }
    impl TableCellBackgroundFillPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                TableCellBackgroundFillPropertyState::Inherit => "INHERIT",
                TableCellBackgroundFillPropertyState::NotRendered => "NOT_RENDERED",
                TableCellBackgroundFillPropertyState::Rendered => "RENDERED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TableCellBackgroundFillPropertyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TableCellBackgroundFillPropertyState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TableCellBackgroundFillPropertyState, ()> {
            Ok(match s {
                "INHERIT" => TableCellBackgroundFillPropertyState::Inherit,
                "NOT_RENDERED" => TableCellBackgroundFillPropertyState::NotRendered,
                "RENDERED" => TableCellBackgroundFillPropertyState::Rendered,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TableCellBackgroundFillPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableCellBackgroundFillPropertyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableCellBackgroundFillPropertyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INHERIT" => TableCellBackgroundFillPropertyState::Inherit,
                "NOT_RENDERED" => TableCellBackgroundFillPropertyState::NotRendered,
                "RENDERED" => TableCellBackgroundFillPropertyState::Rendered,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TableCellBackgroundFillPropertyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellBackgroundFillPropertyState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TableCellLocation {
        #[doc = "The 0-based column index."]
        #[serde(
            rename = "columnIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_index: ::std::option::Option<i32>,
        #[doc = "The 0-based row index."]
        #[serde(
            rename = "rowIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TableCellLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableCellProperties {
        #[doc = "The alignment of the content in the table cell. The default alignment\nmatches the alignment for newly created table cells in the Slides editor."]
        #[serde(
            rename = "contentAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_alignment:
            ::std::option::Option<crate::schemas::TableCellPropertiesContentAlignment>,
        #[doc = "The background fill of the table cell. The default fill matches the fill\nfor newly created table cells in the Slides editor."]
        #[serde(
            rename = "tableCellBackgroundFill",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_background_fill:
            ::std::option::Option<crate::schemas::TableCellBackgroundFill>,
    }
    impl ::google_field_selector::FieldSelector for TableCellProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableCellPropertiesContentAlignment {
        #[doc = "An alignment that aligns the content to the bottom of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
        Bottom,
        #[doc = "An unspecified content alignment. The content alignment is inherited from\nthe parent if it exists."]
        ContentAlignmentUnspecified,
        #[doc = "An unsupported content alignment."]
        ContentAlignmentUnsupported,
        #[doc = "An alignment that aligns the content to the middle of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
        Middle,
        #[doc = "An alignment that aligns the content to the top of the content holder.\nCorresponds to ECMA-376 ST_TextAnchoringType 't'."]
        Top,
    }
    impl TableCellPropertiesContentAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                TableCellPropertiesContentAlignment::Bottom => "BOTTOM",
                TableCellPropertiesContentAlignment::ContentAlignmentUnspecified => {
                    "CONTENT_ALIGNMENT_UNSPECIFIED"
                }
                TableCellPropertiesContentAlignment::ContentAlignmentUnsupported => {
                    "CONTENT_ALIGNMENT_UNSUPPORTED"
                }
                TableCellPropertiesContentAlignment::Middle => "MIDDLE",
                TableCellPropertiesContentAlignment::Top => "TOP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TableCellPropertiesContentAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TableCellPropertiesContentAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TableCellPropertiesContentAlignment, ()> {
            Ok(match s {
                "BOTTOM" => TableCellPropertiesContentAlignment::Bottom,
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    TableCellPropertiesContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    TableCellPropertiesContentAlignment::ContentAlignmentUnsupported
                }
                "MIDDLE" => TableCellPropertiesContentAlignment::Middle,
                "TOP" => TableCellPropertiesContentAlignment::Top,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TableCellPropertiesContentAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableCellPropertiesContentAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableCellPropertiesContentAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM" => TableCellPropertiesContentAlignment::Bottom,
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    TableCellPropertiesContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    TableCellPropertiesContentAlignment::ContentAlignmentUnsupported
                }
                "MIDDLE" => TableCellPropertiesContentAlignment::Middle,
                "TOP" => TableCellPropertiesContentAlignment::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TableCellPropertiesContentAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellPropertiesContentAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableColumnProperties {
        #[doc = "Width of a column."]
        #[serde(
            rename = "columnWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_width: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for TableColumnProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableColumnProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TableRange {
        #[doc = "The column span of the table range."]
        #[serde(
            rename = "columnSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_span: ::std::option::Option<i32>,
        #[doc = "The starting location of the table range."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The row span of the table range."]
        #[serde(
            rename = "rowSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TableRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableRow {
        #[doc = "Height of a row."]
        #[serde(
            rename = "rowHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_height: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "Properties and contents of each cell.\n\nCells that span multiple columns are represented only once with a\ncolumn_span greater\nthan 1. As a result, the length of this collection does not always match\nthe number of columns of the entire table."]
        #[serde(
            rename = "tableCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cells: ::std::option::Option<Vec<crate::schemas::TableCell>>,
        #[doc = "Properties of the row."]
        #[serde(
            rename = "tableRowProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_row_properties: ::std::option::Option<crate::schemas::TableRowProperties>,
    }
    impl ::google_field_selector::FieldSelector for TableRow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableRow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableRowProperties {
        #[doc = "Minimum height of the row. The row will be rendered in the Slides editor at\na height equal to or greater than this value in order to show all the text\nin the row's cell(s)."]
        #[serde(
            rename = "minRowHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_row_height: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for TableRowProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableRowProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextContent {
        #[doc = "The bulleted lists contained in this text, keyed by list ID."]
        #[serde(
            rename = "lists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lists:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::List>>,
        #[doc = "The text contents broken down into its component parts, including styling\ninformation. This property is read-only."]
        #[serde(
            rename = "textElements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_elements: ::std::option::Option<Vec<crate::schemas::TextElement>>,
    }
    impl ::google_field_selector::FieldSelector for TextContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextContent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextElement {
        #[doc = "A TextElement representing a spot in the text that is dynamically\nreplaced with content that can change over time."]
        #[serde(
            rename = "autoText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_text: ::std::option::Option<crate::schemas::AutoText>,
        #[doc = "The zero-based end index of this text element, exclusive, in Unicode code\nunits."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "A marker representing the beginning of a new paragraph.\n\nThe `start_index` and `end_index` of this TextElement represent the\nrange of the paragraph. Other TextElements with an index range contained\ninside this paragraph's range are considered to be part of this\nparagraph. The range of indices of two separate paragraphs will never\noverlap."]
        #[serde(
            rename = "paragraphMarker",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph_marker: ::std::option::Option<crate::schemas::ParagraphMarker>,
        #[doc = "The zero-based start index of this text element, in Unicode code units."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "A TextElement representing a run of text where all of the characters\nin the run have the same TextStyle.\n\nThe `start_index` and `end_index` of TextRuns will always be fully\ncontained in the index range of a single `paragraph_marker` TextElement.\nIn other words, a TextRun will never span multiple paragraphs."]
        #[serde(
            rename = "textRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_run: ::std::option::Option<crate::schemas::TextRun>,
    }
    impl ::google_field_selector::FieldSelector for TextElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextElement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextRun {
        #[doc = "The text of this run."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The styling applied to this run."]
        #[serde(
            rename = "style",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for TextRun {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextRun {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextStyle {
        #[doc = "The background color of the text. If set, the color is either opaque or\ntransparent, depending on if the `opaque_color` field in it is set."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::OptionalColor>,
        #[doc = "The text's vertical offset from its normal position.\n\nText with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically\nrendered in a smaller font size, computed based on the `font_size` field.\nThe `font_size` itself is not affected by changes in this field."]
        #[serde(
            rename = "baselineOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub baseline_offset: ::std::option::Option<crate::schemas::TextStyleBaselineOffset>,
        #[doc = "Whether or not the text is rendered as bold."]
        #[serde(
            rename = "bold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bold: ::std::option::Option<bool>,
        #[doc = "The font family of the text.\n\nThe font family can be any font from the Font menu in Slides or from\n[Google Fonts] (https://fonts.google.com/). If the font name is\nunrecognized, the text is rendered in `Arial`.\n\nSome fonts can affect the weight of the text. If an update request\nspecifies values for both `font_family` and `bold`, the explicitly-set\n`bold` value is used."]
        #[serde(
            rename = "fontFamily",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_family: ::std::option::Option<String>,
        #[doc = "The size of the text's font. When read, the `font_size` will specified in\npoints."]
        #[serde(
            rename = "fontSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_size: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The color of the text itself. If set, the color is either opaque or\ntransparent, depending on if the `opaque_color` field in it is set."]
        #[serde(
            rename = "foregroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub foreground_color: ::std::option::Option<crate::schemas::OptionalColor>,
        #[doc = "Whether or not the text is italicized."]
        #[serde(
            rename = "italic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub italic: ::std::option::Option<bool>,
        #[doc = "The hyperlink destination of the text. If unset, there is no link. Links\nare not inherited from parent text.\n\nChanging the link in an update request causes some other changes to the\ntext style of the range:\n\n* When setting a link, the text foreground color will be set to\n  ThemeColorType.HYPERLINK and the text will\n  be underlined. If these fields are modified in the same\n  request, those values will be used instead of the link defaults.\n* Setting a link on a text range that overlaps with an existing link will\n  also update the existing link to point to the new URL.\n* Links are not settable on newline characters. As a result, setting a link\n  on a text range that crosses a paragraph boundary, such as `\"ABC\\n123\"`,\n  will separate the newline character(s) into their own text runs. The\n  link will be applied separately to the runs before and after the newline.\n* Removing a link will update the text style of the range to match the\n  style of the preceding text (or the default text styles if the preceding\n  text is another link) unless different styles are being set in the same\n  request."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<crate::schemas::Link>,
        #[doc = "Whether or not the text is in small capital letters."]
        #[serde(
            rename = "smallCaps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub small_caps: ::std::option::Option<bool>,
        #[doc = "Whether or not the text is struck through."]
        #[serde(
            rename = "strikethrough",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub strikethrough: ::std::option::Option<bool>,
        #[doc = "Whether or not the text is underlined."]
        #[serde(
            rename = "underline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub underline: ::std::option::Option<bool>,
        #[doc = "The font family and rendered weight of the text.\n\nThis field is an extension of `font_family` meant to support explicit font\nweights without breaking backwards compatibility. As such, when reading the\nstyle of a range of text, the value of `weighted_font_family#font_family`\nwill always be equal to that of `font_family`. However, when writing, if\nboth fields are included in the field mask (either explicitly or through\nthe wildcard `\"*\"`), their values are reconciled as follows:\n\n* If `font_family` is set and `weighted_font_family` is not, the value of\n  `font_family` is applied with weight `400` (\"normal\").\n* If both fields are set, the value of `font_family` must match that of\n  `weighted_font_family#font_family`. If so, the font family and weight of\n  `weighted_font_family` is applied. Otherwise, a 400 bad request error is\n  returned.\n* If `weighted_font_family` is set and `font_family` is not, the font\n  family and weight of `weighted_font_family` is applied.\n* If neither field is set, the font family and weight of the text inherit\n  from the parent. Note that these properties cannot inherit separately\n  from each other.\n\nIf an update request specifies values for both `weighted_font_family` and\n`bold`, the `weighted_font_family` is applied first, then `bold`.\n\nIf `weighted_font_family#weight` is not set, it defaults to `400`.\n\nIf `weighted_font_family` is set, then `weighted_font_family#font_family`\nmust also be set with a non-empty value. Otherwise, a 400 bad request error\nis returned."]
        #[serde(
            rename = "weightedFontFamily",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weighted_font_family: ::std::option::Option<crate::schemas::WeightedFontFamily>,
    }
    impl ::google_field_selector::FieldSelector for TextStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TextStyleBaselineOffset {
        #[doc = "The text's baseline offset is inherited from the parent."]
        BaselineOffsetUnspecified,
        #[doc = "The text is not vertically offset."]
        None,
        #[doc = "The text is vertically offset downwards (subscript)."]
        Subscript,
        #[doc = "The text is vertically offset upwards (superscript)."]
        Superscript,
    }
    impl TextStyleBaselineOffset {
        pub fn as_str(self) -> &'static str {
            match self {
                TextStyleBaselineOffset::BaselineOffsetUnspecified => "BASELINE_OFFSET_UNSPECIFIED",
                TextStyleBaselineOffset::None => "NONE",
                TextStyleBaselineOffset::Subscript => "SUBSCRIPT",
                TextStyleBaselineOffset::Superscript => "SUPERSCRIPT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TextStyleBaselineOffset {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TextStyleBaselineOffset {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TextStyleBaselineOffset, ()> {
            Ok(match s {
                "BASELINE_OFFSET_UNSPECIFIED" => TextStyleBaselineOffset::BaselineOffsetUnspecified,
                "NONE" => TextStyleBaselineOffset::None,
                "SUBSCRIPT" => TextStyleBaselineOffset::Subscript,
                "SUPERSCRIPT" => TextStyleBaselineOffset::Superscript,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TextStyleBaselineOffset {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TextStyleBaselineOffset {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TextStyleBaselineOffset {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASELINE_OFFSET_UNSPECIFIED" => TextStyleBaselineOffset::BaselineOffsetUnspecified,
                "NONE" => TextStyleBaselineOffset::None,
                "SUBSCRIPT" => TextStyleBaselineOffset::Subscript,
                "SUPERSCRIPT" => TextStyleBaselineOffset::Superscript,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TextStyleBaselineOffset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextStyleBaselineOffset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ThemeColorPair {
        #[doc = "The concrete color corresponding to the theme color type above."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::RgbColor>,
        #[doc = "The type of the theme color."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ThemeColorPairType>,
    }
    impl ::google_field_selector::FieldSelector for ThemeColorPair {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThemeColorPair {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThemeColorPairType {
        #[doc = "Represents the first accent color."]
        Accent1,
        #[doc = "Represents the second accent color."]
        Accent2,
        #[doc = "Represents the third accent color."]
        Accent3,
        #[doc = "Represents the fourth accent color."]
        Accent4,
        #[doc = "Represents the fifth accent color."]
        Accent5,
        #[doc = "Represents the sixth accent color."]
        Accent6,
        #[doc = "Represents the first background color."]
        Background1,
        #[doc = "Represents the second background color."]
        Background2,
        #[doc = "Represents the first dark color."]
        Dark1,
        #[doc = "Represents the second dark color."]
        Dark2,
        #[doc = "Represents the color to use for visited hyperlinks."]
        FollowedHyperlink,
        #[doc = "Represents the color to use for hyperlinks."]
        Hyperlink,
        #[doc = "Represents the first light color."]
        Light1,
        #[doc = "Represents the second light color."]
        Light2,
        #[doc = "Represents the first text color."]
        Text1,
        #[doc = "Represents the second text color."]
        Text2,
        #[doc = "Unspecified theme color. This value should not be used."]
        ThemeColorTypeUnspecified,
    }
    impl ThemeColorPairType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThemeColorPairType::Accent1 => "ACCENT1",
                ThemeColorPairType::Accent2 => "ACCENT2",
                ThemeColorPairType::Accent3 => "ACCENT3",
                ThemeColorPairType::Accent4 => "ACCENT4",
                ThemeColorPairType::Accent5 => "ACCENT5",
                ThemeColorPairType::Accent6 => "ACCENT6",
                ThemeColorPairType::Background1 => "BACKGROUND1",
                ThemeColorPairType::Background2 => "BACKGROUND2",
                ThemeColorPairType::Dark1 => "DARK1",
                ThemeColorPairType::Dark2 => "DARK2",
                ThemeColorPairType::FollowedHyperlink => "FOLLOWED_HYPERLINK",
                ThemeColorPairType::Hyperlink => "HYPERLINK",
                ThemeColorPairType::Light1 => "LIGHT1",
                ThemeColorPairType::Light2 => "LIGHT2",
                ThemeColorPairType::Text1 => "TEXT1",
                ThemeColorPairType::Text2 => "TEXT2",
                ThemeColorPairType::ThemeColorTypeUnspecified => "THEME_COLOR_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ThemeColorPairType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ThemeColorPairType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ThemeColorPairType, ()> {
            Ok(match s {
                "ACCENT1" => ThemeColorPairType::Accent1,
                "ACCENT2" => ThemeColorPairType::Accent2,
                "ACCENT3" => ThemeColorPairType::Accent3,
                "ACCENT4" => ThemeColorPairType::Accent4,
                "ACCENT5" => ThemeColorPairType::Accent5,
                "ACCENT6" => ThemeColorPairType::Accent6,
                "BACKGROUND1" => ThemeColorPairType::Background1,
                "BACKGROUND2" => ThemeColorPairType::Background2,
                "DARK1" => ThemeColorPairType::Dark1,
                "DARK2" => ThemeColorPairType::Dark2,
                "FOLLOWED_HYPERLINK" => ThemeColorPairType::FollowedHyperlink,
                "HYPERLINK" => ThemeColorPairType::Hyperlink,
                "LIGHT1" => ThemeColorPairType::Light1,
                "LIGHT2" => ThemeColorPairType::Light2,
                "TEXT1" => ThemeColorPairType::Text1,
                "TEXT2" => ThemeColorPairType::Text2,
                "THEME_COLOR_TYPE_UNSPECIFIED" => ThemeColorPairType::ThemeColorTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ThemeColorPairType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThemeColorPairType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThemeColorPairType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCENT1" => ThemeColorPairType::Accent1,
                "ACCENT2" => ThemeColorPairType::Accent2,
                "ACCENT3" => ThemeColorPairType::Accent3,
                "ACCENT4" => ThemeColorPairType::Accent4,
                "ACCENT5" => ThemeColorPairType::Accent5,
                "ACCENT6" => ThemeColorPairType::Accent6,
                "BACKGROUND1" => ThemeColorPairType::Background1,
                "BACKGROUND2" => ThemeColorPairType::Background2,
                "DARK1" => ThemeColorPairType::Dark1,
                "DARK2" => ThemeColorPairType::Dark2,
                "FOLLOWED_HYPERLINK" => ThemeColorPairType::FollowedHyperlink,
                "HYPERLINK" => ThemeColorPairType::Hyperlink,
                "LIGHT1" => ThemeColorPairType::Light1,
                "LIGHT2" => ThemeColorPairType::Light2,
                "TEXT1" => ThemeColorPairType::Text1,
                "TEXT2" => ThemeColorPairType::Text2,
                "THEME_COLOR_TYPE_UNSPECIFIED" => ThemeColorPairType::ThemeColorTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ThemeColorPairType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThemeColorPairType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Thumbnail {
        #[doc = "The content URL of the thumbnail image.\n\nThe URL to the image has a default lifetime of 30 minutes.\nThis URL is tagged with the account of the requester. Anyone with the URL\neffectively accesses the image as the original requester. Access to the\nimage may be lost if the presentation's sharing settings change.\nThe mime type of the thumbnail image is the same as specified in the\n`GetPageThumbnailRequest`."]
        #[serde(
            rename = "contentUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_url: ::std::option::Option<String>,
        #[doc = "The positive height in pixels of the thumbnail image."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The positive width in pixels of the thumbnail image."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Thumbnail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Thumbnail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UngroupObjectsRequest {
        #[doc = "The object IDs of the objects to ungroup.\n\nOnly groups that are not inside other\ngroups can be ungrouped. All the groups\nshould be on the same page. The group itself is deleted. The visual sizes\nand positions of all the children are preserved."]
        #[serde(
            rename = "objectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for UngroupObjectsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UngroupObjectsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UnmergeTableCellsRequest {
        #[doc = "The object ID of the table."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The table range specifying which cells of the table to unmerge.\n\nAll merged cells in this range will be unmerged, and cells that are already\nunmerged will not be affected. If the range has no merged cells, the\nrequest will do nothing. If there is text in any of the merged cells, the\ntext will remain in the upper-left (\"head\") cell of the resulting block of\nunmerged cells."]
        #[serde(
            rename = "tableRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_range: ::std::option::Option<crate::schemas::TableRange>,
    }
    impl ::google_field_selector::FieldSelector for UnmergeTableCellsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnmergeTableCellsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateImagePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `imageProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the image outline color, set `fields` to\n`\"outline.outlineFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The image properties to update."]
        #[serde(
            rename = "imageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_properties: ::std::option::Option<crate::schemas::ImageProperties>,
        #[doc = "The object ID of the image the updates are applied to."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateImagePropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateImagePropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateLineCategoryRequest {
        #[doc = "The line category to update to.\n\nThe exact line type is determined based\non the category to update to and how it's routed to connect to other page\nelements."]
        #[serde(
            rename = "lineCategory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_category:
            ::std::option::Option<crate::schemas::UpdateLineCategoryRequestLineCategory>,
        #[doc = "The object ID of the line the update is applied to.\n\nOnly a line with a category\nindicating it is a \"connector\" can be updated.\n\nThe line may be rerouted after updating its category."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateLineCategoryRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateLineCategoryRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdateLineCategoryRequestLineCategory {
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
    }
    impl UpdateLineCategoryRequestLineCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdateLineCategoryRequestLineCategory::Bent => "BENT",
                UpdateLineCategoryRequestLineCategory::Curved => "CURVED",
                UpdateLineCategoryRequestLineCategory::LineCategoryUnspecified => {
                    "LINE_CATEGORY_UNSPECIFIED"
                }
                UpdateLineCategoryRequestLineCategory::Straight => "STRAIGHT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UpdateLineCategoryRequestLineCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UpdateLineCategoryRequestLineCategory {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UpdateLineCategoryRequestLineCategory, ()> {
            Ok(match s {
                "BENT" => UpdateLineCategoryRequestLineCategory::Bent,
                "CURVED" => UpdateLineCategoryRequestLineCategory::Curved,
                "LINE_CATEGORY_UNSPECIFIED" => {
                    UpdateLineCategoryRequestLineCategory::LineCategoryUnspecified
                }
                "STRAIGHT" => UpdateLineCategoryRequestLineCategory::Straight,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UpdateLineCategoryRequestLineCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdateLineCategoryRequestLineCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateLineCategoryRequestLineCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BENT" => UpdateLineCategoryRequestLineCategory::Bent,
                "CURVED" => UpdateLineCategoryRequestLineCategory::Curved,
                "LINE_CATEGORY_UNSPECIFIED" => {
                    UpdateLineCategoryRequestLineCategory::LineCategoryUnspecified
                }
                "STRAIGHT" => UpdateLineCategoryRequestLineCategory::Straight,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UpdateLineCategoryRequestLineCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateLineCategoryRequestLineCategory {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateLinePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `lineProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the line solid fill color, set `fields` to\n`\"lineFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The line properties to update."]
        #[serde(
            rename = "lineProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_properties: ::std::option::Option<crate::schemas::LineProperties>,
        #[doc = "The object ID of the line the update is applied to."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateLinePropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateLinePropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdatePageElementAltTextRequest {
        #[doc = "The updated alt text description of the page element. If unset the existing\nvalue will be maintained. The description is exposed to screen readers\nand other accessibility interfaces. Only use human readable values related\nto the content of the page element."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The object ID of the page element the updates are applied to."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The updated alt text title of the page element. If unset the\nexisting value will be maintained. The title is exposed to screen readers\nand other accessibility interfaces. Only use human readable values related\nto the content of the page element."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdatePageElementAltTextRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdatePageElementAltTextRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdatePageElementTransformRequest {
        #[doc = "The apply mode of the transform update."]
        #[serde(
            rename = "applyMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apply_mode:
            ::std::option::Option<crate::schemas::UpdatePageElementTransformRequestApplyMode>,
        #[doc = "The object ID of the page element to update."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The input transform matrix used to update the page element."]
        #[serde(
            rename = "transform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transform: ::std::option::Option<crate::schemas::AffineTransform>,
    }
    impl ::google_field_selector::FieldSelector for UpdatePageElementTransformRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdatePageElementTransformRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdatePageElementTransformRequestApplyMode {
        #[doc = "Replaces the existing AffineTransform matrix with the new one."]
        Absolute,
        #[doc = "Unspecified mode."]
        ApplyModeUnspecified,
        #[doc = "Applies the new AffineTransform matrix to the existing one, and\nreplaces the existing one with the resulting concatenation."]
        Relative,
    }
    impl UpdatePageElementTransformRequestApplyMode {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdatePageElementTransformRequestApplyMode::Absolute => "ABSOLUTE",
                UpdatePageElementTransformRequestApplyMode::ApplyModeUnspecified => {
                    "APPLY_MODE_UNSPECIFIED"
                }
                UpdatePageElementTransformRequestApplyMode::Relative => "RELATIVE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UpdatePageElementTransformRequestApplyMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UpdatePageElementTransformRequestApplyMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<UpdatePageElementTransformRequestApplyMode, ()> {
            Ok(match s {
                "ABSOLUTE" => UpdatePageElementTransformRequestApplyMode::Absolute,
                "APPLY_MODE_UNSPECIFIED" => {
                    UpdatePageElementTransformRequestApplyMode::ApplyModeUnspecified
                }
                "RELATIVE" => UpdatePageElementTransformRequestApplyMode::Relative,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UpdatePageElementTransformRequestApplyMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdatePageElementTransformRequestApplyMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdatePageElementTransformRequestApplyMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABSOLUTE" => UpdatePageElementTransformRequestApplyMode::Absolute,
                "APPLY_MODE_UNSPECIFIED" => {
                    UpdatePageElementTransformRequestApplyMode::ApplyModeUnspecified
                }
                "RELATIVE" => UpdatePageElementTransformRequestApplyMode::Relative,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UpdatePageElementTransformRequestApplyMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdatePageElementTransformRequestApplyMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdatePageElementsZOrderRequest {
        #[doc = "The Z-order operation to apply on the page elements.\n\nWhen applying the operation on multiple page elements, the relative\nZ-orders within these page elements before the operation is maintained."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation:
            ::std::option::Option<crate::schemas::UpdatePageElementsZOrderRequestOperation>,
        #[doc = "The object IDs of the page elements to update.\n\nAll the page elements must be on the same page and must not be grouped."]
        #[serde(
            rename = "pageElementObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_element_object_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for UpdatePageElementsZOrderRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdatePageElementsZOrderRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdatePageElementsZOrderRequestOperation {
        #[doc = "Brings the page elements forward on the page by one element relative to the\nforwardmost one in the specified page elements."]
        BringForward,
        #[doc = "Brings the page elements to the front of the page."]
        BringToFront,
        #[doc = "Sends the page elements backward on the page by one element relative to the\nfurthest behind one in the specified page elements."]
        SendBackward,
        #[doc = "Sends the page elements to the back of the page."]
        SendToBack,
        #[doc = "Unspecified operation."]
        ZOrderOperationUnspecified,
    }
    impl UpdatePageElementsZOrderRequestOperation {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdatePageElementsZOrderRequestOperation::BringForward => "BRING_FORWARD",
                UpdatePageElementsZOrderRequestOperation::BringToFront => "BRING_TO_FRONT",
                UpdatePageElementsZOrderRequestOperation::SendBackward => "SEND_BACKWARD",
                UpdatePageElementsZOrderRequestOperation::SendToBack => "SEND_TO_BACK",
                UpdatePageElementsZOrderRequestOperation::ZOrderOperationUnspecified => {
                    "Z_ORDER_OPERATION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for UpdatePageElementsZOrderRequestOperation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UpdatePageElementsZOrderRequestOperation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<UpdatePageElementsZOrderRequestOperation, ()> {
            Ok(match s {
                "BRING_FORWARD" => UpdatePageElementsZOrderRequestOperation::BringForward,
                "BRING_TO_FRONT" => UpdatePageElementsZOrderRequestOperation::BringToFront,
                "SEND_BACKWARD" => UpdatePageElementsZOrderRequestOperation::SendBackward,
                "SEND_TO_BACK" => UpdatePageElementsZOrderRequestOperation::SendToBack,
                "Z_ORDER_OPERATION_UNSPECIFIED" => {
                    UpdatePageElementsZOrderRequestOperation::ZOrderOperationUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UpdatePageElementsZOrderRequestOperation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdatePageElementsZOrderRequestOperation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdatePageElementsZOrderRequestOperation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BRING_FORWARD" => UpdatePageElementsZOrderRequestOperation::BringForward,
                "BRING_TO_FRONT" => UpdatePageElementsZOrderRequestOperation::BringToFront,
                "SEND_BACKWARD" => UpdatePageElementsZOrderRequestOperation::SendBackward,
                "SEND_TO_BACK" => UpdatePageElementsZOrderRequestOperation::SendToBack,
                "Z_ORDER_OPERATION_UNSPECIFIED" => {
                    UpdatePageElementsZOrderRequestOperation::ZOrderOperationUnspecified
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
    impl ::google_field_selector::FieldSelector for UpdatePageElementsZOrderRequestOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdatePageElementsZOrderRequestOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdatePagePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `pageProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the page background solid fill color, set `fields`\nto `\"pageBackgroundFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the page the update is applied to."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The page properties to update."]
        #[serde(
            rename = "pageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_properties: ::std::option::Option<crate::schemas::PageProperties>,
    }
    impl ::google_field_selector::FieldSelector for UpdatePagePropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdatePagePropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateParagraphStyleRequest {
        #[doc = "The location of the cell in the table containing the paragraph(s) to\nstyle. If `object_id` refers to a table, `cell_location` must have a value.\nOtherwise, it must not."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `style` is implied and\nshould not be specified. A single `\"*\"` can be used as short-hand for\nlisting every field.\n\nFor example, to update the paragraph alignment, set `fields` to\n`\"alignment\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the shape or table with the text to be styled."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The paragraph's style."]
        #[serde(
            rename = "style",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub style: ::std::option::Option<crate::schemas::ParagraphStyle>,
        #[doc = "The range of text containing the paragraph(s) to style."]
        #[serde(
            rename = "textRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_range: ::std::option::Option<crate::schemas::Range>,
    }
    impl ::google_field_selector::FieldSelector for UpdateParagraphStyleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateParagraphStyleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateShapePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `shapeProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the shape background solid fill color, set `fields`\nto `\"shapeBackgroundFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the shape the updates are applied to."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The shape properties to update."]
        #[serde(
            rename = "shapeProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shape_properties: ::std::option::Option<crate::schemas::ShapeProperties>,
    }
    impl ::google_field_selector::FieldSelector for UpdateShapePropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateShapePropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateSlidesPositionRequest {
        #[doc = "The index where the slides should be inserted, based on the slide\narrangement before the move takes place. Must be between zero and the\nnumber of slides in the presentation, inclusive."]
        #[serde(
            rename = "insertionIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insertion_index: ::std::option::Option<i32>,
        #[doc = "The IDs of the slides in the presentation that should be moved.\nThe slides in this list must be in existing presentation order, without\nduplicates."]
        #[serde(
            rename = "slideObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slide_object_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for UpdateSlidesPositionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateSlidesPositionRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableBorderPropertiesRequest {
        #[doc = "The border position in the table range the updates should apply to. If a\nborder position is not specified, the updates will apply to all borders in\nthe table range."]
        #[serde(
            rename = "borderPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_position:
            ::std::option::Option<crate::schemas::UpdateTableBorderPropertiesRequestBorderPosition>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableBorderProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the table border solid fill color, set\n`fields` to `\"tableBorderFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The table border properties to update."]
        #[serde(
            rename = "tableBorderProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_border_properties: ::std::option::Option<crate::schemas::TableBorderProperties>,
        #[doc = "The table range representing the subset of the table to which the updates\nare applied. If a table range is not specified, the updates will apply to\nthe entire table."]
        #[serde(
            rename = "tableRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_range: ::std::option::Option<crate::schemas::TableRange>,
    }
    impl ::google_field_selector::FieldSelector for UpdateTableBorderPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTableBorderPropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdateTableBorderPropertiesRequestBorderPosition {
        #[doc = "All borders in the range."]
        All,
        #[doc = "Borders at the bottom of the range."]
        Bottom,
        #[doc = "Borders on the inside of the range."]
        Inner,
        #[doc = "Horizontal borders on the inside of the range."]
        InnerHorizontal,
        #[doc = "Vertical borders on the inside of the range."]
        InnerVertical,
        #[doc = "Borders at the left of the range."]
        Left,
        #[doc = "Borders along the outside of the range."]
        Outer,
        #[doc = "Borders at the right of the range."]
        Right,
        #[doc = "Borders at the top of the range."]
        Top,
    }
    impl UpdateTableBorderPropertiesRequestBorderPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdateTableBorderPropertiesRequestBorderPosition::All => "ALL",
                UpdateTableBorderPropertiesRequestBorderPosition::Bottom => "BOTTOM",
                UpdateTableBorderPropertiesRequestBorderPosition::Inner => "INNER",
                UpdateTableBorderPropertiesRequestBorderPosition::InnerHorizontal => {
                    "INNER_HORIZONTAL"
                }
                UpdateTableBorderPropertiesRequestBorderPosition::InnerVertical => "INNER_VERTICAL",
                UpdateTableBorderPropertiesRequestBorderPosition::Left => "LEFT",
                UpdateTableBorderPropertiesRequestBorderPosition::Outer => "OUTER",
                UpdateTableBorderPropertiesRequestBorderPosition::Right => "RIGHT",
                UpdateTableBorderPropertiesRequestBorderPosition::Top => "TOP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UpdateTableBorderPropertiesRequestBorderPosition {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UpdateTableBorderPropertiesRequestBorderPosition {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<UpdateTableBorderPropertiesRequestBorderPosition, ()> {
            Ok(match s {
                "ALL" => UpdateTableBorderPropertiesRequestBorderPosition::All,
                "BOTTOM" => UpdateTableBorderPropertiesRequestBorderPosition::Bottom,
                "INNER" => UpdateTableBorderPropertiesRequestBorderPosition::Inner,
                "INNER_HORIZONTAL" => {
                    UpdateTableBorderPropertiesRequestBorderPosition::InnerHorizontal
                }
                "INNER_VERTICAL" => UpdateTableBorderPropertiesRequestBorderPosition::InnerVertical,
                "LEFT" => UpdateTableBorderPropertiesRequestBorderPosition::Left,
                "OUTER" => UpdateTableBorderPropertiesRequestBorderPosition::Outer,
                "RIGHT" => UpdateTableBorderPropertiesRequestBorderPosition::Right,
                "TOP" => UpdateTableBorderPropertiesRequestBorderPosition::Top,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UpdateTableBorderPropertiesRequestBorderPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdateTableBorderPropertiesRequestBorderPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateTableBorderPropertiesRequestBorderPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL" => UpdateTableBorderPropertiesRequestBorderPosition::All,
                "BOTTOM" => UpdateTableBorderPropertiesRequestBorderPosition::Bottom,
                "INNER" => UpdateTableBorderPropertiesRequestBorderPosition::Inner,
                "INNER_HORIZONTAL" => {
                    UpdateTableBorderPropertiesRequestBorderPosition::InnerHorizontal
                }
                "INNER_VERTICAL" => UpdateTableBorderPropertiesRequestBorderPosition::InnerVertical,
                "LEFT" => UpdateTableBorderPropertiesRequestBorderPosition::Left,
                "OUTER" => UpdateTableBorderPropertiesRequestBorderPosition::Outer,
                "RIGHT" => UpdateTableBorderPropertiesRequestBorderPosition::Right,
                "TOP" => UpdateTableBorderPropertiesRequestBorderPosition::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UpdateTableBorderPropertiesRequestBorderPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTableBorderPropertiesRequestBorderPosition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableCellPropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableCellProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the table cell background solid fill color, set\n`fields` to `\"tableCellBackgroundFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The table cell properties to update."]
        #[serde(
            rename = "tableCellProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_properties: ::std::option::Option<crate::schemas::TableCellProperties>,
        #[doc = "The table range representing the subset of the table to which the updates\nare applied. If a table range is not specified, the updates will apply to\nthe entire table."]
        #[serde(
            rename = "tableRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_range: ::std::option::Option<crate::schemas::TableRange>,
    }
    impl ::google_field_selector::FieldSelector for UpdateTableCellPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTableCellPropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableColumnPropertiesRequest {
        #[doc = "The list of zero-based indices specifying which columns to update. If no\nindices are provided, all columns in the table will be updated."]
        #[serde(
            rename = "columnIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_indices: ::std::option::Option<Vec<i32>>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableColumnProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the column width, set `fields` to `\"column_width\"`.\n\nIf '\"column_width\"' is included in the field mask but the property is left\nunset, the column width will default to 406,400 EMU (32 points)."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The table column properties to update.\n\nIf the value of `table_column_properties#column_width` in the request is\nless than 406,400 EMU (32 points), a 400 bad request error is returned."]
        #[serde(
            rename = "tableColumnProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_column_properties: ::std::option::Option<crate::schemas::TableColumnProperties>,
    }
    impl ::google_field_selector::FieldSelector for UpdateTableColumnPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTableColumnPropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableRowPropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableRowProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the minimum row height, set `fields` to\n`\"min_row_height\"`.\n\nIf '\"min_row_height\"' is included in the field mask but the property is\nleft unset, the minimum row height will default to 0."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The list of zero-based indices specifying which rows to update. If no\nindices are provided, all rows in the table will be updated."]
        #[serde(
            rename = "rowIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_indices: ::std::option::Option<Vec<i32>>,
        #[doc = "The table row properties to update."]
        #[serde(
            rename = "tableRowProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_row_properties: ::std::option::Option<crate::schemas::TableRowProperties>,
    }
    impl ::google_field_selector::FieldSelector for UpdateTableRowPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTableRowPropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTextStyleRequest {
        #[doc = "The location of the cell in the table containing the text to style. If\n`object_id` refers to a table, `cell_location` must have a value.\nOtherwise, it must not."]
        #[serde(
            rename = "cellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `style` is implied and\nshould not be specified. A single `\"*\"` can be used as short-hand for\nlisting every field.\n\nFor example, to update the text style to bold, set `fields` to `\"bold\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the shape or table with the text to be styled."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The style(s) to set on the text.\n\nIf the value for a particular style matches that of the parent, that style\nwill be set to inherit.\n\nCertain text style changes may cause other changes meant to mirror the\nbehavior of the Slides editor. See the documentation of\nTextStyle for more information."]
        #[serde(
            rename = "style",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub style: ::std::option::Option<crate::schemas::TextStyle>,
        #[doc = "The range of text to style.\n\nThe range may be extended to include adjacent newlines.\n\nIf the range fully contains a paragraph belonging to a list, the\nparagraph's bullet is also updated with the matching text style."]
        #[serde(
            rename = "textRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_range: ::std::option::Option<crate::schemas::Range>,
    }
    impl ::google_field_selector::FieldSelector for UpdateTextStyleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTextStyleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateVideoPropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `videoProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the video outline color, set `fields` to\n`\"outline.outlineFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The object ID of the video the updates are applied to."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The video properties to update."]
        #[serde(
            rename = "videoProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_properties: ::std::option::Option<crate::schemas::VideoProperties>,
    }
    impl ::google_field_selector::FieldSelector for UpdateVideoPropertiesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateVideoPropertiesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Video {
        #[doc = "The video source's unique identifier for this video."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The video source."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::VideoSource>,
        #[doc = "An URL to a video. The URL is valid as long as the source video exists and\nsharing settings do not change."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[doc = "The properties of the video."]
        #[serde(
            rename = "videoProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_properties: ::std::option::Option<crate::schemas::VideoProperties>,
    }
    impl ::google_field_selector::FieldSelector for Video {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Video {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VideoSource {
        #[doc = "The video source is Google Drive."]
        Drive,
        #[doc = "The video source is unspecified."]
        SourceUnspecified,
        #[doc = "The video source is YouTube."]
        Youtube,
    }
    impl VideoSource {
        pub fn as_str(self) -> &'static str {
            match self {
                VideoSource::Drive => "DRIVE",
                VideoSource::SourceUnspecified => "SOURCE_UNSPECIFIED",
                VideoSource::Youtube => "YOUTUBE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VideoSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VideoSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VideoSource, ()> {
            Ok(match s {
                "DRIVE" => VideoSource::Drive,
                "SOURCE_UNSPECIFIED" => VideoSource::SourceUnspecified,
                "YOUTUBE" => VideoSource::Youtube,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VideoSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VideoSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VideoSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DRIVE" => VideoSource::Drive,
                "SOURCE_UNSPECIFIED" => VideoSource::SourceUnspecified,
                "YOUTUBE" => VideoSource::Youtube,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VideoSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VideoSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VideoProperties {
        #[doc = "Whether to enable video autoplay when the page is displayed in present\nmode. Defaults to false."]
        #[serde(
            rename = "autoPlay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_play: ::std::option::Option<bool>,
        #[doc = "The time at which to end playback, measured in seconds from the beginning\nof the video.\nIf set, the end time should be after the start time.\nIf not set or if you set this to a value that exceeds the video's length,\nthe video will be played until its end."]
        #[serde(
            rename = "end",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end: ::std::option::Option<u32>,
        #[doc = "Whether to mute the audio during video playback. Defaults to false."]
        #[serde(
            rename = "mute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mute: ::std::option::Option<bool>,
        #[doc = "The outline of the video. The default outline matches the defaults for new\nvideos created in the Slides editor."]
        #[serde(
            rename = "outline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub outline: ::std::option::Option<crate::schemas::Outline>,
        #[doc = "The time at which to start playback, measured in seconds from the beginning\nof the video.\nIf set, the start time should be before the end time.\nIf you set this to a value that exceeds the video's length in seconds, the\nvideo will be played from the last second.\nIf not set, the video will be played from the beginning."]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start: ::std::option::Option<u32>,
    }
    impl ::google_field_selector::FieldSelector for VideoProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VideoProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WeightedFontFamily {
        #[doc = "The font family of the text.\n\nThe font family can be any font from the Font menu in Slides or from\n[Google Fonts] (https://fonts.google.com/). If the font name is\nunrecognized, the text is rendered in `Arial`."]
        #[serde(
            rename = "fontFamily",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_family: ::std::option::Option<String>,
        #[doc = "The rendered weight of the text. This field can have any value that is a\nmultiple of `100` between `100` and `900`, inclusive. This range\ncorresponds to the numerical values described in the CSS 2.1\nSpecification,\n[section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness),\nwith non-numerical values disallowed. Weights greater than or equal to\n`700` are considered bold, and weights less than `700`are not bold. The\ndefault value is `400` (\"normal\")."]
        #[serde(
            rename = "weight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weight: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for WeightedFontFamily {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WeightedFontFamily {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WordArt {
        #[doc = "The text rendered as word art."]
        #[serde(
            rename = "renderedText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rendered_text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WordArt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WordArt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WriteControl {
        #[doc = "The revision ID of the presentation required for the write request. If\nspecified and the `required_revision_id` doesn't exactly match the\npresentation's current `revision_id`, the request will not be processed and\nwill return a 400 bad request error."]
        #[serde(
            rename = "requiredRevisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required_revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WriteControl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WriteControl {
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
    #[doc = "Actions that can be performed on the presentations resource"]
    pub fn presentations(&self) -> crate::resources::presentations::PresentationsActions {
        crate::resources::presentations::PresentationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod presentations {
        pub mod params {}
        pub struct PresentationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PresentationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Applies one or more updates to the presentation.\n\nEach request is validated before\nbeing applied. If any request is not valid, then the entire request will\nfail and nothing will be applied.\n\nSome requests have replies to\ngive you some information about how they are applied. Other requests do\nnot need to return information; these each return an empty reply.\nThe order of replies matches that of the requests.\n\nFor example, suppose you call batchUpdate with four updates, and only the\nthird one returns information. The response would have two empty replies:\nthe reply to the third request, and another empty reply, in that order.\n\nBecause other users may be editing the presentation, the presentation\nmight not exactly reflect your changes: your changes may\nbe altered with respect to collaborator changes. If there are no\ncollaborators, the presentation should reflect your changes. In any case,\nthe updates in your request are guaranteed to be applied together\natomically."]
            pub fn batch_update(
                &self,
                request: crate::schemas::BatchUpdatePresentationRequest,
                presentation_id: impl Into<String>,
            ) -> BatchUpdateRequestBuilder {
                BatchUpdateRequestBuilder {
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
                    presentation_id: presentation_id.into(),
                }
            }
            #[doc = "Creates a blank presentation using the title given in the request. If a\n`presentationId` is provided, it is used as the ID of the new presentation.\nOtherwise, a new ID is generated. Other fields in the request, including\nany provided content, are ignored.\nReturns the created presentation."]
            pub fn create(&self, request: crate::schemas::Presentation) -> CreateRequestBuilder {
                CreateRequestBuilder {
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
                }
            }
            #[doc = "Gets the latest version of the specified presentation."]
            pub fn get(&self, presentation_id: impl Into<String>) -> GetRequestBuilder {
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
                    presentation_id: presentation_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the pages resource"]
            pub fn pages(&self) -> crate::resources::presentations::pages::PagesActions {
                crate::resources::presentations::pages::PagesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [PresentationsActions::batch_update()](struct.PresentationsActions.html#method.batch_update)"]
        #[derive(Debug, Clone)]
        pub struct BatchUpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchUpdatePresentationRequest,
            presentation_id: String,
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
        impl<'a> BatchUpdateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::BatchUpdatePresentationResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchUpdatePresentationResponse, crate::Error> {
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
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://slides.googleapis.com/".to_owned();
                output.push_str("v1/presentations/");
                {
                    let var_as_str = &self.presentation_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":batchUpdate");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
        #[doc = "Created via [PresentationsActions::create()](struct.PresentationsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Presentation,
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
        impl<'a> CreateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Presentation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Presentation, crate::Error> {
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
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://slides.googleapis.com/".to_owned();
                output.push_str("v1/presentations");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
        #[doc = "Created via [PresentationsActions::get()](struct.PresentationsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            presentation_id: String,
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
            ) -> Result<crate::schemas::Presentation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Presentation, crate::Error> {
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
                let mut output = "https://slides.googleapis.com/".to_owned();
                output.push_str("v1/presentations/");
                {
                    let var_as_str = &self.presentation_id;
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
        pub mod pages {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetThumbnailThumbnailPropertiesMimeType {
                    Png,
                }
                impl GetThumbnailThumbnailPropertiesMimeType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetThumbnailThumbnailPropertiesMimeType::Png => "PNG",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetThumbnailThumbnailPropertiesMimeType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetThumbnailThumbnailPropertiesMimeType {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<GetThumbnailThumbnailPropertiesMimeType, ()>
                    {
                        Ok(match s {
                            "PNG" => GetThumbnailThumbnailPropertiesMimeType::Png,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetThumbnailThumbnailPropertiesMimeType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetThumbnailThumbnailPropertiesMimeType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetThumbnailThumbnailPropertiesMimeType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "PNG" => GetThumbnailThumbnailPropertiesMimeType::Png,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetThumbnailThumbnailPropertiesMimeType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetThumbnailThumbnailPropertiesMimeType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetThumbnailThumbnailPropertiesThumbnailSize {
                    Large,
                    Medium,
                    Small,
                    ThumbnailSizeUnspecified,
                }
                impl GetThumbnailThumbnailPropertiesThumbnailSize {
                    pub fn as_str(self) -> &'static str {
                        match self { GetThumbnailThumbnailPropertiesThumbnailSize :: Large => "LARGE" , GetThumbnailThumbnailPropertiesThumbnailSize :: Medium => "MEDIUM" , GetThumbnailThumbnailPropertiesThumbnailSize :: Small => "SMALL" , GetThumbnailThumbnailPropertiesThumbnailSize :: ThumbnailSizeUnspecified => "THUMBNAIL_SIZE_UNSPECIFIED" , }
                    }
                }
                impl ::std::convert::AsRef<str> for GetThumbnailThumbnailPropertiesThumbnailSize {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetThumbnailThumbnailPropertiesThumbnailSize {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<GetThumbnailThumbnailPropertiesThumbnailSize, ()>
                    {
                        Ok ( match s { "LARGE" => GetThumbnailThumbnailPropertiesThumbnailSize :: Large , "MEDIUM" => GetThumbnailThumbnailPropertiesThumbnailSize :: Medium , "SMALL" => GetThumbnailThumbnailPropertiesThumbnailSize :: Small , "THUMBNAIL_SIZE_UNSPECIFIED" => GetThumbnailThumbnailPropertiesThumbnailSize :: ThumbnailSizeUnspecified , _ => return Err ( ( ) ) , } )
                    }
                }
                impl ::std::fmt::Display for GetThumbnailThumbnailPropertiesThumbnailSize {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetThumbnailThumbnailPropertiesThumbnailSize {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetThumbnailThumbnailPropertiesThumbnailSize {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok ( match value { "LARGE" => GetThumbnailThumbnailPropertiesThumbnailSize :: Large , "MEDIUM" => GetThumbnailThumbnailPropertiesThumbnailSize :: Medium , "SMALL" => GetThumbnailThumbnailPropertiesThumbnailSize :: Small , "THUMBNAIL_SIZE_UNSPECIFIED" => GetThumbnailThumbnailPropertiesThumbnailSize :: ThumbnailSizeUnspecified , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
                    }
                }
                impl ::google_field_selector::FieldSelector for GetThumbnailThumbnailPropertiesThumbnailSize {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetThumbnailThumbnailPropertiesThumbnailSize {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct PagesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PagesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets the latest version of the specified page in the presentation."]
                pub fn get(
                    &self,
                    presentation_id: impl Into<String>,
                    page_object_id: impl Into<String>,
                ) -> GetRequestBuilder {
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
                        presentation_id: presentation_id.into(),
                        page_object_id: page_object_id.into(),
                    }
                }
                #[doc = "Generates a thumbnail of the latest version of the specified page in the\npresentation and returns a URL to the thumbnail image.\n\nThis request counts as an [expensive read request](/slides/limits) for\nquota purposes."]
                pub fn get_thumbnail(
                    &self,
                    presentation_id: impl Into<String>,
                    page_object_id: impl Into<String>,
                ) -> GetThumbnailRequestBuilder {
                    GetThumbnailRequestBuilder {
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
                        presentation_id: presentation_id.into(),
                        page_object_id: page_object_id.into(),
                        thumbnail_properties_mime_type: None,
                        thumbnail_properties_thumbnail_size: None,
                    }
                }
            }
            #[doc = "Created via [PagesActions::get()](struct.PagesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                presentation_id: String,
                page_object_id: String,
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
                ) -> Result<crate::schemas::Page, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Page, crate::Error> {
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
                    let mut output = "https://slides.googleapis.com/".to_owned();
                    output.push_str("v1/presentations/");
                    {
                        let var_as_str = &self.presentation_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/pages/");
                    {
                        let var_as_str = &self.page_object_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
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
            #[doc = "Created via [PagesActions::get_thumbnail()](struct.PagesActions.html#method.get_thumbnail)"]
            #[derive(Debug, Clone)]
            pub struct GetThumbnailRequestBuilder < 'a > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a dyn :: google_api_auth :: GetAccessToken , presentation_id : String , page_object_id : String , thumbnail_properties_mime_type : Option < crate :: resources :: presentations :: pages :: params :: GetThumbnailThumbnailPropertiesMimeType > , thumbnail_properties_thumbnail_size : Option < crate :: resources :: presentations :: pages :: params :: GetThumbnailThumbnailPropertiesThumbnailSize > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
            impl<'a> GetThumbnailRequestBuilder<'a> {
                #[doc = "The optional mime type of the thumbnail image.\n\nIf you don't specify the mime type, the mime type defaults to PNG."]
                pub fn thumbnail_properties_mime_type(
                    mut self,
                    value : crate :: resources :: presentations :: pages :: params :: GetThumbnailThumbnailPropertiesMimeType,
                ) -> Self {
                    self.thumbnail_properties_mime_type = Some(value);
                    self
                }
                #[doc = "The optional thumbnail image size.\n\nIf you don't specify the size, the server chooses a default size of the\nimage."]
                pub fn thumbnail_properties_thumbnail_size(
                    mut self,
                    value : crate :: resources :: presentations :: pages :: params :: GetThumbnailThumbnailPropertiesThumbnailSize,
                ) -> Self {
                    self.thumbnail_properties_thumbnail_size = Some(value);
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
                ) -> Result<crate::schemas::Thumbnail, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Thumbnail, crate::Error> {
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
                    let mut output = "https://slides.googleapis.com/".to_owned();
                    output.push_str("v1/presentations/");
                    {
                        let var_as_str = &self.presentation_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/pages/");
                    {
                        let var_as_str = &self.page_object_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/thumbnail");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "thumbnailProperties.mimeType",
                        &self.thumbnail_properties_mime_type,
                    )]);
                    let req = req.query(&[(
                        "thumbnailProperties.thumbnailSize",
                        &self.thumbnail_properties_thumbnail_size,
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
