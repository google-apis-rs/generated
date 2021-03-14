#![doc = "# Resources and Methods\n    * [assets](resources/assets/struct.AssetsActions.html)\n      * [*get*](resources/assets/struct.GetRequestBuilder.html), [*list*](resources/assets/struct.ListRequestBuilder.html)\n    * [users](resources/users/struct.UsersActions.html)\n      * [assets](resources/users/assets/struct.AssetsActions.html)\n        * [*list*](resources/users/assets/struct.ListRequestBuilder.html)\n      * [likedassets](resources/users/likedassets/struct.LikedassetsActions.html)\n        * [*list*](resources/users/likedassets/struct.ListRequestBuilder.html)\n"]
pub mod scopes {}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Asset {
        #[doc = "The author's publicly visible name. Use this name when giving credit to the author. For more information, see [Licensing](/poly/discover/licensing)."]
        #[serde(
            rename = "authorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub author_name: ::std::option::Option<String>,
        #[doc = "For published assets, the time when the asset was published. For unpublished assets, the time when the asset was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The human-readable description, set by the asset's author."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The human-readable name, set by the asset's author."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A list of Formats where each format describes one representation of the asset."]
        #[serde(
            rename = "formats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formats: ::std::option::Option<Vec<crate::schemas::Format>>,
        #[doc = "Whether this asset has been curated by the Poly team."]
        #[serde(
            rename = "isCurated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_curated: ::std::option::Option<bool>,
        #[doc = "The license under which the author has made the asset available for use, if any."]
        #[serde(
            rename = "license",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub license: ::std::option::Option<crate::schemas::AssetLicense>,
        #[doc = "Application-defined opaque metadata for this asset. This field is only returned when querying for the signed-in user's own assets, not for public assets. This string is limited to 1K chars. It is up to the creator of the asset to define the format for this string (for example, JSON)."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<String>,
        #[doc = "The unique identifier for the asset in the form: `assets/{ASSET_ID}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Hints for displaying the asset. Note that these parameters are not immutable; the author of an asset may change them post-publication."]
        #[serde(
            rename = "presentationParams",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub presentation_params: ::std::option::Option<crate::schemas::PresentationParams>,
        #[doc = "The remix info for the asset."]
        #[serde(
            rename = "remixInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remix_info: ::std::option::Option<crate::schemas::RemixInfo>,
        #[doc = "The thumbnail image for the asset."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<crate::schemas::File>,
        #[doc = "The time when the asset was last modified. For published assets, whose contents are immutable, the update time changes only when metadata properties, such as visibility, are updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "The visibility of the asset and who can access it."]
        #[serde(
            rename = "visibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visibility: ::std::option::Option<crate::schemas::AssetVisibility>,
    }
    impl ::google_field_selector::FieldSelector for Asset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Asset {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetLicense {
        #[doc = "Unlicensed: All Rights Reserved by the author. Unlicensed assets are **not** returned by List Assets."]
        AllRightsReserved,
        #[doc = "Creative Commons CC-BY 3.0. https://creativecommons.org/licenses/by/3.0/"]
        CreativeCommonsBy,
        #[doc = "Unknown license value."]
        Unknown,
    }
    impl AssetLicense {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetLicense::AllRightsReserved => "ALL_RIGHTS_RESERVED",
                AssetLicense::CreativeCommonsBy => "CREATIVE_COMMONS_BY",
                AssetLicense::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AssetLicense {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AssetLicense {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AssetLicense, ()> {
            Ok(match s {
                "ALL_RIGHTS_RESERVED" => AssetLicense::AllRightsReserved,
                "CREATIVE_COMMONS_BY" => AssetLicense::CreativeCommonsBy,
                "UNKNOWN" => AssetLicense::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AssetLicense {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetLicense {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetLicense {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_RIGHTS_RESERVED" => AssetLicense::AllRightsReserved,
                "CREATIVE_COMMONS_BY" => AssetLicense::CreativeCommonsBy,
                "UNKNOWN" => AssetLicense::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AssetLicense {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AssetLicense {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetVisibility {
        #[doc = "Access to the asset and its underlying files and resources is restricted to the author. **Authentication:** You must supply an OAuth token that corresponds to the author's account."]
        Private,
        #[doc = "Access to the asset and its underlying files and resources is available to anyone."]
        Public,
        #[doc = "Access to the asset and its underlying files and resources is available to anyone with the asset's name. Unlisted assets are **not** returned by List Assets."]
        Unlisted,
        #[doc = "Unknown (and invalid) visibility."]
        VisibilityUnspecified,
    }
    impl AssetVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetVisibility::Private => "PRIVATE",
                AssetVisibility::Public => "PUBLIC",
                AssetVisibility::Unlisted => "UNLISTED",
                AssetVisibility::VisibilityUnspecified => "VISIBILITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AssetVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AssetVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AssetVisibility, ()> {
            Ok(match s {
                "PRIVATE" => AssetVisibility::Private,
                "PUBLIC" => AssetVisibility::Public,
                "UNLISTED" => AssetVisibility::Unlisted,
                "VISIBILITY_UNSPECIFIED" => AssetVisibility::VisibilityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AssetVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PRIVATE" => AssetVisibility::Private,
                "PUBLIC" => AssetVisibility::Public,
                "UNLISTED" => AssetVisibility::Unlisted,
                "VISIBILITY_UNSPECIFIED" => AssetVisibility::VisibilityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AssetVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AssetVisibility {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AssetImportMessage {
        #[doc = "The code associated with this message."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::AssetImportMessageCode>,
        #[doc = "An optional file path. Only present for those error codes that specify it."]
        #[serde(
            rename = "filePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_path: ::std::option::Option<String>,
        #[doc = "An optional image error. Only present for INVALID_IMAGE_FILE."]
        #[serde(
            rename = "imageError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_error: ::std::option::Option<crate::schemas::ImageError>,
        #[doc = "An optional OBJ parse error. Only present for OBJ_PARSE_ERROR."]
        #[serde(
            rename = "objParseError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub obj_parse_error: ::std::option::Option<crate::schemas::ObjParseError>,
    }
    impl ::google_field_selector::FieldSelector for AssetImportMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AssetImportMessage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetImportMessageCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "Default materials are used in the model. This means that one or more faces is using default materials either because no usemtl statement was specified or because the requested material was not found due to a missing material file or bad material name. This does not cover the case of missing textures."]
        DefaultMaterials,
        #[doc = "When generating the preview for the import, no geometry was found."]
        EmptyModel,
        #[doc = "The importer was not able to import the model before the expiration time."]
        Expired,
        #[doc = "Multiple files were encountered in addition to a ZIP archive. When uploading an archive only one file is permitted."]
        ExtraFilesWithArchive,
        #[doc = "The importer encountered a fatal error and was unable to import the model."]
        FatalError,
        #[doc = "The importer encountered a problem reading an image file."]
        ImageError,
        #[doc = "The import includes a file of an unsupported element type. The file path is specified."]
        InvalidElementType,
        #[doc = "The asset import did not include any file that we can import (i.e. an OBJ file)."]
        NoImportableFile,
        #[doc = "A problem was encountered while parsing the OBJ file. The converter makes a 'best effort' attempt to continue when encountering such issues. In some cases the resulting preview model may still be acceptable. The details can be found in the parse error message."]
        ObjParseError,
    }
    impl AssetImportMessageCode {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetImportMessageCode::CodeUnspecified => "CODE_UNSPECIFIED",
                AssetImportMessageCode::DefaultMaterials => "DEFAULT_MATERIALS",
                AssetImportMessageCode::EmptyModel => "EMPTY_MODEL",
                AssetImportMessageCode::Expired => "EXPIRED",
                AssetImportMessageCode::ExtraFilesWithArchive => "EXTRA_FILES_WITH_ARCHIVE",
                AssetImportMessageCode::FatalError => "FATAL_ERROR",
                AssetImportMessageCode::ImageError => "IMAGE_ERROR",
                AssetImportMessageCode::InvalidElementType => "INVALID_ELEMENT_TYPE",
                AssetImportMessageCode::NoImportableFile => "NO_IMPORTABLE_FILE",
                AssetImportMessageCode::ObjParseError => "OBJ_PARSE_ERROR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AssetImportMessageCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AssetImportMessageCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AssetImportMessageCode, ()> {
            Ok(match s {
                "CODE_UNSPECIFIED" => AssetImportMessageCode::CodeUnspecified,
                "DEFAULT_MATERIALS" => AssetImportMessageCode::DefaultMaterials,
                "EMPTY_MODEL" => AssetImportMessageCode::EmptyModel,
                "EXPIRED" => AssetImportMessageCode::Expired,
                "EXTRA_FILES_WITH_ARCHIVE" => AssetImportMessageCode::ExtraFilesWithArchive,
                "FATAL_ERROR" => AssetImportMessageCode::FatalError,
                "IMAGE_ERROR" => AssetImportMessageCode::ImageError,
                "INVALID_ELEMENT_TYPE" => AssetImportMessageCode::InvalidElementType,
                "NO_IMPORTABLE_FILE" => AssetImportMessageCode::NoImportableFile,
                "OBJ_PARSE_ERROR" => AssetImportMessageCode::ObjParseError,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AssetImportMessageCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetImportMessageCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetImportMessageCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => AssetImportMessageCode::CodeUnspecified,
                "DEFAULT_MATERIALS" => AssetImportMessageCode::DefaultMaterials,
                "EMPTY_MODEL" => AssetImportMessageCode::EmptyModel,
                "EXPIRED" => AssetImportMessageCode::Expired,
                "EXTRA_FILES_WITH_ARCHIVE" => AssetImportMessageCode::ExtraFilesWithArchive,
                "FATAL_ERROR" => AssetImportMessageCode::FatalError,
                "IMAGE_ERROR" => AssetImportMessageCode::ImageError,
                "INVALID_ELEMENT_TYPE" => AssetImportMessageCode::InvalidElementType,
                "NO_IMPORTABLE_FILE" => AssetImportMessageCode::NoImportableFile,
                "OBJ_PARSE_ERROR" => AssetImportMessageCode::ObjParseError,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AssetImportMessageCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AssetImportMessageCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct File {
        #[doc = "The MIME content-type, such as `image/png`. For more information, see [MIME types](//developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The path of the resource file relative to the root file. For root or thumbnail files, this is just the filename."]
        #[serde(
            rename = "relativePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relative_path: ::std::option::Option<String>,
        #[doc = "The URL where the file data can be retrieved."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for File {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for File {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Format {
        #[doc = "Complexity stats about this representation of the asset."]
        #[serde(
            rename = "formatComplexity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format_complexity: ::std::option::Option<crate::schemas::FormatComplexity>,
        #[doc = "A short string that identifies the format type of this representation. Possible values are: `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`."]
        #[serde(
            rename = "formatType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format_type: ::std::option::Option<String>,
        #[doc = "A list of dependencies of the root element. May include, but is not limited to, materials, textures, and shader programs."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<crate::schemas::File>>,
        #[doc = "The root of the file hierarchy. This will always be populated. For some format_types - such as `TILT`, which are self-contained - this is all of the data. Other types - such as `OBJ` - often reference other data elements. These are contained in the resources field."]
        #[serde(
            rename = "root",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root: ::std::option::Option<crate::schemas::File>,
    }
    impl ::google_field_selector::FieldSelector for Format {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Format {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FormatComplexity {
        #[doc = "A non-negative integer that represents the level of detail (LOD) of this format relative to other formats of the same asset with the same format_type. This hint allows you to sort formats from the most-detailed (0) to least-detailed (integers greater than 0)."]
        #[serde(
            rename = "lodHint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lod_hint: ::std::option::Option<i32>,
        #[doc = "The estimated number of triangles."]
        #[serde(
            rename = "triangleCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub triangle_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for FormatComplexity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FormatComplexity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImageError {
        #[doc = "The type of image error encountered. Optional for older image errors."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ImageErrorCode>,
        #[doc = "The file path in the import of the image that was rejected."]
        #[serde(
            rename = "filePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ImageError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageErrorCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "The image size is too large."]
        ImageTooBig,
        #[doc = "We were unable to read the image file."]
        InvalidImage,
        #[doc = "The image data does not match the expected MIME type of the image."]
        WrongImageType,
    }
    impl ImageErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ImageErrorCode::ImageTooBig => "IMAGE_TOO_BIG",
                ImageErrorCode::InvalidImage => "INVALID_IMAGE",
                ImageErrorCode::WrongImageType => "WRONG_IMAGE_TYPE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImageErrorCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImageErrorCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImageErrorCode, ()> {
            Ok(match s {
                "CODE_UNSPECIFIED" => ImageErrorCode::CodeUnspecified,
                "IMAGE_TOO_BIG" => ImageErrorCode::ImageTooBig,
                "INVALID_IMAGE" => ImageErrorCode::InvalidImage,
                "WRONG_IMAGE_TYPE" => ImageErrorCode::WrongImageType,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImageErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ImageErrorCode::CodeUnspecified,
                "IMAGE_TOO_BIG" => ImageErrorCode::ImageTooBig,
                "INVALID_IMAGE" => ImageErrorCode::InvalidImage,
                "WRONG_IMAGE_TYPE" => ImageErrorCode::WrongImageType,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImageErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageErrorCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListAssetsResponse {
        #[doc = "A list of assets that match the criteria specified in the request."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets: ::std::option::Option<Vec<crate::schemas::Asset>>,
        #[doc = "The continuation token for retrieving the next page. If empty, indicates that there are no more pages. To get the next page, submit the same request specifying this value as the page_token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListAssetsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAssetsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListLikedAssetsResponse {
        #[doc = "A list of assets that match the criteria specified in the request."]
        #[serde(
            rename = "assets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assets: ::std::option::Option<Vec<crate::schemas::Asset>>,
        #[doc = "The continuation token for retrieving the next page. If empty, indicates that there are no more pages. To get the next page, submit the same request specifying this value as the page_token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListLikedAssetsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListLikedAssetsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListUserAssetsResponse {
        #[doc = "The continuation token for retrieving the next page. If empty, indicates that there are no more pages. To get the next page, submit the same request specifying this value as the page_token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
        #[doc = "A list of UserAssets matching the request."]
        #[serde(
            rename = "userAssets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_assets: ::std::option::Option<Vec<crate::schemas::UserAsset>>,
    }
    impl ::google_field_selector::FieldSelector for ListUserAssetsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUserAssetsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ObjParseError {
        #[doc = "The type of problem found (required)."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ObjParseErrorCode>,
        #[doc = "The ending character index at which the problem was found."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The file path in which the problem was found."]
        #[serde(
            rename = "filePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_path: ::std::option::Option<String>,
        #[doc = "The text of the line. Note that this may be truncated if the line was very long. This may not include the error if it occurs after line truncation."]
        #[serde(
            rename = "line",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line: ::std::option::Option<String>,
        #[doc = "Line number at which the problem was found."]
        #[serde(
            rename = "lineNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_number: ::std::option::Option<i32>,
        #[doc = "The starting character index at which the problem was found."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ObjParseError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjParseError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ObjParseErrorCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "The specified file was not found in the import."]
        FileNotFound,
        #[doc = "A missing file was found at a different file path."]
        FileSubstitution,
        #[doc = "Vertex references are specified in an inconsistent style for a face (e.g. some vertices specify texture vertices but some don't)."]
        InconsistentVertexRefs,
        #[doc = "The command is invalid."]
        InvalidCommand,
        #[doc = "The file path was invalid. Only relative paths are supported."]
        InvalidFilePath,
        #[doc = "A invalid number was specified."]
        InvalidNumber,
        #[doc = "The smoothing group is not valid."]
        InvalidSmoothingGroup,
        #[doc = "The specified texture option is not valid."]
        InvalidTextureOption,
        #[doc = "The specified parameter value was not recognized."]
        InvalidValue,
        #[doc = "An invalid vertex reference was specified."]
        InvalidVertexRef,
        #[doc = "A line in an OBJ or MTL file exceeded the maximum line length."]
        LineTooLong,
        #[doc = "An expected file name was not specified."]
        MissingFileName,
        #[doc = "A vertex reference does not specify a geometric vertex."]
        MissingGeometricVertex,
        #[doc = "An expected token was not found."]
        MissingToken,
        #[doc = "Vertex colors were specified for only some vertices of a face."]
        MissingVertexColors,
        #[doc = "Material parameters were specified before the first material definition."]
        NoMaterialDefined,
        #[doc = "The specified number was too large or small for its usage."]
        NumberOutOfRange,
        #[doc = "The vertex specified too few dimensions for its usage."]
        TooFewDimensions,
        #[doc = "The face specified too few vertices."]
        TooFewVertices,
        #[doc = "The vertex specified too many dimensions for its usage."]
        TooManyDimensions,
        #[doc = "The maximum number of problems to report was reached. Parsing continues, but further problems will be ignored."]
        TooManyProblems,
        #[doc = "The specified material was not found in any material definition in the import."]
        UnknownMaterial,
        #[doc = "This command is a valid OBJ command but is not supported. This error is only generated for the first instance of such a command."]
        UnsupportedCommand,
        #[doc = "This line ended with unparsed token characters."]
        UnusedTokens,
        #[doc = "The specified vertex was not found."]
        VertexNotFound,
    }
    impl ObjParseErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ObjParseErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ObjParseErrorCode::FileNotFound => "FILE_NOT_FOUND",
                ObjParseErrorCode::FileSubstitution => "FILE_SUBSTITUTION",
                ObjParseErrorCode::InconsistentVertexRefs => "INCONSISTENT_VERTEX_REFS",
                ObjParseErrorCode::InvalidCommand => "INVALID_COMMAND",
                ObjParseErrorCode::InvalidFilePath => "INVALID_FILE_PATH",
                ObjParseErrorCode::InvalidNumber => "INVALID_NUMBER",
                ObjParseErrorCode::InvalidSmoothingGroup => "INVALID_SMOOTHING_GROUP",
                ObjParseErrorCode::InvalidTextureOption => "INVALID_TEXTURE_OPTION",
                ObjParseErrorCode::InvalidValue => "INVALID_VALUE",
                ObjParseErrorCode::InvalidVertexRef => "INVALID_VERTEX_REF",
                ObjParseErrorCode::LineTooLong => "LINE_TOO_LONG",
                ObjParseErrorCode::MissingFileName => "MISSING_FILE_NAME",
                ObjParseErrorCode::MissingGeometricVertex => "MISSING_GEOMETRIC_VERTEX",
                ObjParseErrorCode::MissingToken => "MISSING_TOKEN",
                ObjParseErrorCode::MissingVertexColors => "MISSING_VERTEX_COLORS",
                ObjParseErrorCode::NoMaterialDefined => "NO_MATERIAL_DEFINED",
                ObjParseErrorCode::NumberOutOfRange => "NUMBER_OUT_OF_RANGE",
                ObjParseErrorCode::TooFewDimensions => "TOO_FEW_DIMENSIONS",
                ObjParseErrorCode::TooFewVertices => "TOO_FEW_VERTICES",
                ObjParseErrorCode::TooManyDimensions => "TOO_MANY_DIMENSIONS",
                ObjParseErrorCode::TooManyProblems => "TOO_MANY_PROBLEMS",
                ObjParseErrorCode::UnknownMaterial => "UNKNOWN_MATERIAL",
                ObjParseErrorCode::UnsupportedCommand => "UNSUPPORTED_COMMAND",
                ObjParseErrorCode::UnusedTokens => "UNUSED_TOKENS",
                ObjParseErrorCode::VertexNotFound => "VERTEX_NOT_FOUND",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ObjParseErrorCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ObjParseErrorCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ObjParseErrorCode, ()> {
            Ok(match s {
                "CODE_UNSPECIFIED" => ObjParseErrorCode::CodeUnspecified,
                "FILE_NOT_FOUND" => ObjParseErrorCode::FileNotFound,
                "FILE_SUBSTITUTION" => ObjParseErrorCode::FileSubstitution,
                "INCONSISTENT_VERTEX_REFS" => ObjParseErrorCode::InconsistentVertexRefs,
                "INVALID_COMMAND" => ObjParseErrorCode::InvalidCommand,
                "INVALID_FILE_PATH" => ObjParseErrorCode::InvalidFilePath,
                "INVALID_NUMBER" => ObjParseErrorCode::InvalidNumber,
                "INVALID_SMOOTHING_GROUP" => ObjParseErrorCode::InvalidSmoothingGroup,
                "INVALID_TEXTURE_OPTION" => ObjParseErrorCode::InvalidTextureOption,
                "INVALID_VALUE" => ObjParseErrorCode::InvalidValue,
                "INVALID_VERTEX_REF" => ObjParseErrorCode::InvalidVertexRef,
                "LINE_TOO_LONG" => ObjParseErrorCode::LineTooLong,
                "MISSING_FILE_NAME" => ObjParseErrorCode::MissingFileName,
                "MISSING_GEOMETRIC_VERTEX" => ObjParseErrorCode::MissingGeometricVertex,
                "MISSING_TOKEN" => ObjParseErrorCode::MissingToken,
                "MISSING_VERTEX_COLORS" => ObjParseErrorCode::MissingVertexColors,
                "NO_MATERIAL_DEFINED" => ObjParseErrorCode::NoMaterialDefined,
                "NUMBER_OUT_OF_RANGE" => ObjParseErrorCode::NumberOutOfRange,
                "TOO_FEW_DIMENSIONS" => ObjParseErrorCode::TooFewDimensions,
                "TOO_FEW_VERTICES" => ObjParseErrorCode::TooFewVertices,
                "TOO_MANY_DIMENSIONS" => ObjParseErrorCode::TooManyDimensions,
                "TOO_MANY_PROBLEMS" => ObjParseErrorCode::TooManyProblems,
                "UNKNOWN_MATERIAL" => ObjParseErrorCode::UnknownMaterial,
                "UNSUPPORTED_COMMAND" => ObjParseErrorCode::UnsupportedCommand,
                "UNUSED_TOKENS" => ObjParseErrorCode::UnusedTokens,
                "VERTEX_NOT_FOUND" => ObjParseErrorCode::VertexNotFound,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ObjParseErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ObjParseErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ObjParseErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ObjParseErrorCode::CodeUnspecified,
                "FILE_NOT_FOUND" => ObjParseErrorCode::FileNotFound,
                "FILE_SUBSTITUTION" => ObjParseErrorCode::FileSubstitution,
                "INCONSISTENT_VERTEX_REFS" => ObjParseErrorCode::InconsistentVertexRefs,
                "INVALID_COMMAND" => ObjParseErrorCode::InvalidCommand,
                "INVALID_FILE_PATH" => ObjParseErrorCode::InvalidFilePath,
                "INVALID_NUMBER" => ObjParseErrorCode::InvalidNumber,
                "INVALID_SMOOTHING_GROUP" => ObjParseErrorCode::InvalidSmoothingGroup,
                "INVALID_TEXTURE_OPTION" => ObjParseErrorCode::InvalidTextureOption,
                "INVALID_VALUE" => ObjParseErrorCode::InvalidValue,
                "INVALID_VERTEX_REF" => ObjParseErrorCode::InvalidVertexRef,
                "LINE_TOO_LONG" => ObjParseErrorCode::LineTooLong,
                "MISSING_FILE_NAME" => ObjParseErrorCode::MissingFileName,
                "MISSING_GEOMETRIC_VERTEX" => ObjParseErrorCode::MissingGeometricVertex,
                "MISSING_TOKEN" => ObjParseErrorCode::MissingToken,
                "MISSING_VERTEX_COLORS" => ObjParseErrorCode::MissingVertexColors,
                "NO_MATERIAL_DEFINED" => ObjParseErrorCode::NoMaterialDefined,
                "NUMBER_OUT_OF_RANGE" => ObjParseErrorCode::NumberOutOfRange,
                "TOO_FEW_DIMENSIONS" => ObjParseErrorCode::TooFewDimensions,
                "TOO_FEW_VERTICES" => ObjParseErrorCode::TooFewVertices,
                "TOO_MANY_DIMENSIONS" => ObjParseErrorCode::TooManyDimensions,
                "TOO_MANY_PROBLEMS" => ObjParseErrorCode::TooManyProblems,
                "UNKNOWN_MATERIAL" => ObjParseErrorCode::UnknownMaterial,
                "UNSUPPORTED_COMMAND" => ObjParseErrorCode::UnsupportedCommand,
                "UNUSED_TOKENS" => ObjParseErrorCode::UnusedTokens,
                "VERTEX_NOT_FOUND" => ObjParseErrorCode::VertexNotFound,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ObjParseErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjParseErrorCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PresentationParams {
        #[doc = "A background color which could be used for displaying the 3D asset in a 'thumbnail' or 'palette' style view. Authors have the option to set this background color when publishing or editing their asset. This is represented as a six-digit hexademical triplet specifying the RGB components of the background color, e.g. #FF0000 for Red."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<String>,
        #[doc = "The materials' diffuse/albedo color. This does not apply to vertex colors or texture maps."]
        #[serde(
            rename = "colorSpace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_space: ::std::option::Option<crate::schemas::PresentationParamsColorSpace>,
        #[doc = "A rotation that should be applied to the object root to make it upright. More precisely, this quaternion transforms from \"object space\" (the space in which the object is defined) to \"presentation space\", a coordinate system where +Y is up, +X is right, -Z is forward. For example, if the object is the Eiffel Tower, in its local coordinate system the object might be laid out such that the base of the tower is on the YZ plane and the tip of the tower is towards positive X. In this case this quaternion would specify a rotation (of 90 degrees about the Z axis) such that in the presentation space the base of the tower is aligned with the XZ plane, and the tip of the tower lies towards +Y. This rotation is unrelated to the object's pose in the web preview, which is just a camera position setting and is *not* reflected in this rotation. Please note: this is applicable only to the gLTF."]
        #[serde(
            rename = "orientingRotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub orienting_rotation: ::std::option::Option<crate::schemas::Quaternion>,
    }
    impl ::google_field_selector::FieldSelector for PresentationParams {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PresentationParams {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PresentationParamsColorSpace {
        #[doc = "Colors should be converted to linear by assuming gamma = 2.0."]
        Gamma,
        #[doc = "Linear color values. Default."]
        Linear,
        #[doc = "Invalid color value."]
        Unknown,
    }
    impl PresentationParamsColorSpace {
        pub fn as_str(self) -> &'static str {
            match self {
                PresentationParamsColorSpace::Gamma => "GAMMA",
                PresentationParamsColorSpace::Linear => "LINEAR",
                PresentationParamsColorSpace::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PresentationParamsColorSpace {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PresentationParamsColorSpace {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PresentationParamsColorSpace, ()> {
            Ok(match s {
                "GAMMA" => PresentationParamsColorSpace::Gamma,
                "LINEAR" => PresentationParamsColorSpace::Linear,
                "UNKNOWN" => PresentationParamsColorSpace::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PresentationParamsColorSpace {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PresentationParamsColorSpace {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PresentationParamsColorSpace {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GAMMA" => PresentationParamsColorSpace::Gamma,
                "LINEAR" => PresentationParamsColorSpace::Linear,
                "UNKNOWN" => PresentationParamsColorSpace::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PresentationParamsColorSpace {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PresentationParamsColorSpace {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Quaternion {
        #[doc = "The scalar component."]
        #[serde(
            rename = "w",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub w: ::std::option::Option<f64>,
        #[doc = "The x component."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f64>,
        #[doc = "The y component."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f64>,
        #[doc = "The z component."]
        #[serde(
            rename = "z",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub z: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Quaternion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Quaternion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RemixInfo {
        #[doc = "Resource ids for the sources of this remix, of the form: `assets/{ASSET_ID}`"]
        #[serde(
            rename = "sourceAsset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_asset: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for RemixInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RemixInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StartAssetImportResponse {
        #[doc = "The id of newly created asset. If this is empty when the operation is complete it means the import failed. Please refer to the assetImportMessages field to understand what went wrong."]
        #[serde(
            rename = "assetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_id: ::std::option::Option<String>,
        #[doc = "The id of the asset import."]
        #[serde(
            rename = "assetImportId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_import_id: ::std::option::Option<String>,
        #[doc = "The message from the asset import. This will contain any warnings (or - in the case of failure - errors) that occurred during import."]
        #[serde(
            rename = "assetImportMessages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset_import_messages: ::std::option::Option<Vec<crate::schemas::AssetImportMessage>>,
        #[doc = "The publish URL for the asset."]
        #[serde(
            rename = "publishUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StartAssetImportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StartAssetImportResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UserAsset {
        #[doc = "An Asset."]
        #[serde(
            rename = "asset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub asset: ::std::option::Option<crate::schemas::Asset>,
    }
    impl ::google_field_selector::FieldSelector for UserAsset {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserAsset {
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
    #[doc = "Actions that can be performed on the assets resource"]
    pub fn assets(&self) -> crate::resources::assets::AssetsActions {
        crate::resources::assets::AssetsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the users resource"]
    pub fn users(&self) -> crate::resources::users::UsersActions {
        crate::resources::users::UsersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod assets {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListMaxComplexity {
                #[doc = "Highly-complex."]
                Complex,
                #[doc = "No complexity specified. This is equivalent to omitting the filter."]
                ComplexityUnspecified,
                #[doc = "Averagely-complex."]
                Medium,
                #[doc = "Simple."]
                Simple,
            }
            impl ListMaxComplexity {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListMaxComplexity::Complex => "COMPLEX",
                        ListMaxComplexity::ComplexityUnspecified => "COMPLEXITY_UNSPECIFIED",
                        ListMaxComplexity::Medium => "MEDIUM",
                        ListMaxComplexity::Simple => "SIMPLE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListMaxComplexity {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListMaxComplexity {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListMaxComplexity, ()> {
                    Ok(match s {
                        "COMPLEX" => ListMaxComplexity::Complex,
                        "COMPLEXITY_UNSPECIFIED" => ListMaxComplexity::ComplexityUnspecified,
                        "MEDIUM" => ListMaxComplexity::Medium,
                        "SIMPLE" => ListMaxComplexity::Simple,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListMaxComplexity {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListMaxComplexity {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListMaxComplexity {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "COMPLEX" => ListMaxComplexity::Complex,
                        "COMPLEXITY_UNSPECIFIED" => ListMaxComplexity::ComplexityUnspecified,
                        "MEDIUM" => ListMaxComplexity::Medium,
                        "SIMPLE" => ListMaxComplexity::Simple,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListMaxComplexity {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListMaxComplexity {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct AssetsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AssetsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns detailed information about an asset given its name. PRIVATE assets are returned only if the currently authenticated user (via OAuth token) is the author of the asset."]
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
            #[doc = "Lists all public, remixable assets. These are assets with an access level of PUBLIC and published under the CC-By license."]
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
                    category: None,
                    curated: None,
                    format: None,
                    keywords: None,
                    max_complexity: None,
                    order_by: None,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [AssetsActions::get()](struct.AssetsActions.html#method.get)"]
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
            ) -> Result<crate::schemas::Asset, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Asset, crate::Error> {
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://poly.googleapis.com/".to_owned();
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
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [AssetsActions::list()](struct.AssetsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            category: Option<String>,
            curated: Option<bool>,
            format: Option<String>,
            keywords: Option<String>,
            max_complexity: Option<crate::resources::assets::params::ListMaxComplexity>,
            order_by: Option<String>,
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
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Filter assets based on the specified category. Supported values are: `animals`, `architecture`, `art`, `food`, `nature`, `objects`, `people`, `scenes`, `technology`, and `transport`."]
            pub fn category(mut self, value: impl Into<String>) -> Self {
                self.category = Some(value.into());
                self
            }
            #[doc = "Return only assets that have been curated by the Poly team."]
            pub fn curated(mut self, value: bool) -> Self {
                self.curated = Some(value);
                self
            }
            #[doc = "Return only assets with the matching format. Acceptable values are: `BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`."]
            pub fn format(mut self, value: impl Into<String>) -> Self {
                self.format = Some(value.into());
                self
            }
            #[doc = "One or more search terms to be matched against all text that Poly has indexed for assets, which includes display_name, description, and tags. Multiple keywords should be separated by spaces."]
            pub fn keywords(mut self, value: impl Into<String>) -> Self {
                self.keywords = Some(value.into());
                self
            }
            #[doc = "Returns assets that are of the specified complexity or less. Defaults to COMPLEX. For example, a request for MEDIUM assets also includes SIMPLE assets."]
            pub fn max_complexity(
                mut self,
                value: crate::resources::assets::params::ListMaxComplexity,
            ) -> Self {
                self.max_complexity = Some(value);
                self
            }
            #[doc = "Specifies an ordering for assets. Acceptable values are: `BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets based on a combination of popularity and other features."]
            pub fn order_by(mut self, value: impl Into<String>) -> Self {
                self.order_by = Some(value.into());
                self
            }
            #[doc = "The maximum number of assets to be returned. This value must be between `1` and `100`. Defaults to `20`."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Specifies a continuation token from a previous search whose results were split into multiple pages. To get the next page, submit the same request specifying the value from next_page_token."]
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
            ) -> Result<crate::schemas::ListAssetsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListAssetsResponse, crate::Error> {
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
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://poly.googleapis.com/".to_owned();
                output.push_str("v1/assets");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("category", &self.category)]);
                req = req.query(&[("curated", &self.curated)]);
                req = req.query(&[("format", &self.format)]);
                req = req.query(&[("keywords", &self.keywords)]);
                req = req.query(&[("maxComplexity", &self.max_complexity)]);
                req = req.query(&[("orderBy", &self.order_by)]);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
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
    pub mod users {
        pub mod params {}
        pub struct UsersActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> UsersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the assets resource"]
            pub fn assets(&self) -> crate::resources::users::assets::AssetsActions {
                crate::resources::users::assets::AssetsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the likedassets resource"]
            pub fn likedassets(&self) -> crate::resources::users::likedassets::LikedassetsActions {
                crate::resources::users::likedassets::LikedassetsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod assets {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListVisibility {
                    #[doc = "Returns only private assets."]
                    Private,
                    #[doc = "Returns only published assets."]
                    Published,
                    #[doc = "No visibility specified. Returns all assets."]
                    VisibilityUnspecified,
                }
                impl ListVisibility {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListVisibility::Private => "PRIVATE",
                            ListVisibility::Published => "PUBLISHED",
                            ListVisibility::VisibilityUnspecified => "VISIBILITY_UNSPECIFIED",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListVisibility {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListVisibility {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListVisibility, ()> {
                        Ok(match s {
                            "PRIVATE" => ListVisibility::Private,
                            "PUBLISHED" => ListVisibility::Published,
                            "VISIBILITY_UNSPECIFIED" => ListVisibility::VisibilityUnspecified,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListVisibility {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListVisibility {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListVisibility {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "PRIVATE" => ListVisibility::Private,
                            "PUBLISHED" => ListVisibility::Published,
                            "VISIBILITY_UNSPECIFIED" => ListVisibility::VisibilityUnspecified,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListVisibility {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListVisibility {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct AssetsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AssetsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists assets authored by the given user. Only the value 'me', representing the currently-authenticated user, is supported. May include assets with an access level of PRIVATE or UNLISTED and assets which are All Rights Reserved for the currently-authenticated user."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
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
                        name: name.into(),
                        format: None,
                        order_by: None,
                        page_size: None,
                        page_token: None,
                        visibility: None,
                    }
                }
            }
            #[doc = "Created via [AssetsActions::list()](struct.AssetsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                format: Option<String>,
                order_by: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                visibility: Option<crate::resources::users::assets::params::ListVisibility>,
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
                #[doc = "Return only assets with the matching format. Acceptable values are: `BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`."]
                pub fn format(mut self, value: impl Into<String>) -> Self {
                    self.format = Some(value.into());
                    self
                }
                #[doc = "Specifies an ordering for assets. Acceptable values are: `BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets based on a combination of popularity and other features."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The maximum number of assets to be returned. This value must be between `1` and `100`. Defaults to `20`."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Specifies a continuation token from a previous search whose results were split into multiple pages. To get the next page, submit the same request specifying the value from next_page_token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "The visibility of the assets to be returned. Defaults to VISIBILITY_UNSPECIFIED which returns all assets."]
                pub fn visibility(
                    mut self,
                    value: crate::resources::users::assets::params::ListVisibility,
                ) -> Self {
                    self.visibility = Some(value);
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
                ) -> Result<crate::schemas::ListUserAssetsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListUserAssetsResponse, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://poly.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/assets");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("format", &self.format)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("visibility", &self.visibility)]);
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
        pub mod likedassets {
            pub mod params {}
            pub struct LikedassetsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LikedassetsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists assets that the user has liked. Only the value 'me', representing the currently-authenticated user, is supported. May include assets with an access level of UNLISTED."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
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
                        name: name.into(),
                        format: None,
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [LikedassetsActions::list()](struct.LikedassetsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                format: Option<String>,
                order_by: Option<String>,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "Return only assets with the matching format. Acceptable values are: `BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`."]
                pub fn format(mut self, value: impl Into<String>) -> Self {
                    self.format = Some(value.into());
                    self
                }
                #[doc = "Specifies an ordering for assets. Acceptable values are: `BEST`, `NEWEST`, `OLDEST`, 'LIKED_TIME'. Defaults to `LIKED_TIME`, which ranks assets based on how recently they were liked."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The maximum number of assets to be returned. This value must be between `1` and `100`. Defaults to `20`."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Specifies a continuation token from a previous search whose results were split into multiple pages. To get the next page, submit the same request specifying the value from next_page_token."]
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
                ) -> Result<crate::schemas::ListLikedAssetsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListLikedAssetsResponse, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://poly.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/likedassets");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("format", &self.format)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
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
