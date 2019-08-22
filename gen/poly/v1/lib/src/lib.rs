pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetLicense {
        #[doc = "Unknown license value."]
        Unknown,
        #[doc = "Creative Commons CC-BY 3.0. https://creativecommons.org/licenses/by/3.0/"]
        CreativeCommonsBy,
        #[doc = "Unlicensed: All Rights Reserved by the author. Unlicensed assets are\n**not** returned by List Assets."]
        AllRightsReserved,
    }
    impl AssetLicense {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetLicense::Unknown => "UNKNOWN",
                AssetLicense::CreativeCommonsBy => "CREATIVE_COMMONS_BY",
                AssetLicense::AllRightsReserved => "ALL_RIGHTS_RESERVED",
            }
        }
    }
    impl ::std::fmt::Display for AssetLicense {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetLicense {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetLicense {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => AssetLicense::Unknown,
                "CREATIVE_COMMONS_BY" => AssetLicense::CreativeCommonsBy,
                "ALL_RIGHTS_RESERVED" => AssetLicense::AllRightsReserved,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AssetLicense {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetVisibility {
        #[doc = "Unknown (and invalid) visibility."]
        VisibilityUnspecified,
        #[doc = "Access to the asset and its underlying files and resources is restricted to\nthe author.\n**Authentication:** You must supply an OAuth token that corresponds to the\nauthor's account."]
        Private,
        #[doc = "Access to the asset and its underlying files and resources is available to\nanyone with the asset's name. Unlisted assets are **not**\nreturned by List Assets."]
        Unlisted,
        #[doc = "Access to the asset and its underlying files and resources is available\nto anyone."]
        Public,
    }
    impl AssetVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetVisibility::VisibilityUnspecified => "VISIBILITY_UNSPECIFIED",
                AssetVisibility::Private => "PRIVATE",
                AssetVisibility::Unlisted => "UNLISTED",
                AssetVisibility::Public => "PUBLIC",
            }
        }
    }
    impl ::std::fmt::Display for AssetVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetVisibility {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetVisibility {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "VISIBILITY_UNSPECIFIED" => AssetVisibility::VisibilityUnspecified,
                "PRIVATE" => AssetVisibility::Private,
                "UNLISTED" => AssetVisibility::Unlisted,
                "PUBLIC" => AssetVisibility::Public,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AssetVisibility {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Asset {
        #[doc = "The author's publicly visible name. Use this name when giving credit to the\nauthor. For more information, see [Licensing](/poly/discover/licensing)."]
        #[serde(rename = "authorName", default)]
        pub author_name: ::std::option::Option<String>,
        #[doc = "For published assets, the time when the asset was published.\nFor unpublished assets, the time when the asset was created."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The human-readable description, set by the asset's author."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The human-readable name, set by the asset's author."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A list of Formats where each\nformat describes one representation of the asset."]
        #[serde(rename = "formats", default)]
        pub formats: ::std::option::Option<Vec<crate::schemas::Format>>,
        #[doc = "Whether this asset has been curated by the Poly team."]
        #[serde(rename = "isCurated", default)]
        pub is_curated: ::std::option::Option<bool>,
        #[doc = "The license under which the author has made the asset available\nfor use, if any."]
        #[serde(rename = "license", default)]
        pub license: ::std::option::Option<crate::schemas::AssetLicense>,
        #[doc = "Application-defined opaque metadata for this asset. This field is only\nreturned when querying for the signed-in user's own assets, not for public\nassets. This string is limited to 1K chars. It is up to the creator of\nthe asset to define the format for this string (for example, JSON)."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<String>,
        #[doc = "The unique identifier for the asset in the form:\n`assets/{ASSET_ID}`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Hints for displaying the asset. Note that these parameters are not\nimmutable; the author of an asset may change them post-publication."]
        #[serde(rename = "presentationParams", default)]
        pub presentation_params: ::std::option::Option<crate::schemas::PresentationParams>,
        #[doc = "The remix info for the asset."]
        #[serde(rename = "remixInfo", default)]
        pub remix_info: ::std::option::Option<crate::schemas::RemixInfo>,
        #[doc = "The thumbnail image for the asset."]
        #[serde(rename = "thumbnail", default)]
        pub thumbnail: ::std::option::Option<crate::schemas::File>,
        #[doc = "The time when the asset was last modified. For published assets, whose\ncontents are immutable, the update time changes only when metadata\nproperties, such as visibility, are updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
        #[doc = "The visibility of the asset and who can access it."]
        #[serde(rename = "visibility", default)]
        pub visibility: ::std::option::Option<crate::schemas::AssetVisibility>,
    }
    impl ::field_selector::FieldSelector for Asset {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetImportMessageCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "The asset import did not include any file that we can import (i.e. an OBJ\nfile)."]
        NoImportableFile,
        #[doc = "When generating the preview for the import, no geometry was found."]
        EmptyModel,
        #[doc = "A problem was encountered while parsing the OBJ file. The converter makes\na 'best effort' attempt to continue when encountering such issues. In\nsome cases the resulting preview model may still be acceptable. The\ndetails can be found in the parse error message."]
        ObjParseError,
        #[doc = "The importer was not able to import the model before the expiration time."]
        Expired,
        #[doc = "The importer encountered a problem reading an image file."]
        ImageError,
        #[doc = "Multiple files were encountered in addition to a ZIP archive. When\nuploading an archive only one file is permitted."]
        ExtraFilesWithArchive,
        #[doc = "Default materials are used in the model. This means that one or more\nfaces is using default materials either because no usemtl statement was\nspecified or because the requested material was not found due to a\nmissing material file or bad material name. This does not cover the case\nof missing textures."]
        DefaultMaterials,
        #[doc = "The importer encountered a fatal error and was unable to import the\nmodel."]
        FatalError,
        #[doc = "The import includes a file of an unsupported element type. The file path\nis specified."]
        InvalidElementType,
    }
    impl AssetImportMessageCode {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetImportMessageCode::CodeUnspecified => "CODE_UNSPECIFIED",
                AssetImportMessageCode::NoImportableFile => "NO_IMPORTABLE_FILE",
                AssetImportMessageCode::EmptyModel => "EMPTY_MODEL",
                AssetImportMessageCode::ObjParseError => "OBJ_PARSE_ERROR",
                AssetImportMessageCode::Expired => "EXPIRED",
                AssetImportMessageCode::ImageError => "IMAGE_ERROR",
                AssetImportMessageCode::ExtraFilesWithArchive => "EXTRA_FILES_WITH_ARCHIVE",
                AssetImportMessageCode::DefaultMaterials => "DEFAULT_MATERIALS",
                AssetImportMessageCode::FatalError => "FATAL_ERROR",
                AssetImportMessageCode::InvalidElementType => "INVALID_ELEMENT_TYPE",
            }
        }
    }
    impl ::std::fmt::Display for AssetImportMessageCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetImportMessageCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetImportMessageCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => AssetImportMessageCode::CodeUnspecified,
                "NO_IMPORTABLE_FILE" => AssetImportMessageCode::NoImportableFile,
                "EMPTY_MODEL" => AssetImportMessageCode::EmptyModel,
                "OBJ_PARSE_ERROR" => AssetImportMessageCode::ObjParseError,
                "EXPIRED" => AssetImportMessageCode::Expired,
                "IMAGE_ERROR" => AssetImportMessageCode::ImageError,
                "EXTRA_FILES_WITH_ARCHIVE" => AssetImportMessageCode::ExtraFilesWithArchive,
                "DEFAULT_MATERIALS" => AssetImportMessageCode::DefaultMaterials,
                "FATAL_ERROR" => AssetImportMessageCode::FatalError,
                "INVALID_ELEMENT_TYPE" => AssetImportMessageCode::InvalidElementType,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AssetImportMessageCode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::AssetImportMessageCode>,
        #[doc = "An optional file path. Only present for those error codes that specify it."]
        #[serde(rename = "filePath", default)]
        pub file_path: ::std::option::Option<String>,
        #[doc = "An optional image error. Only present for INVALID_IMAGE_FILE."]
        #[serde(rename = "imageError", default)]
        pub image_error: ::std::option::Option<crate::schemas::ImageError>,
        #[doc = "An optional OBJ parse error. Only present for OBJ_PARSE_ERROR."]
        #[serde(rename = "objParseError", default)]
        pub obj_parse_error: ::std::option::Option<crate::schemas::ObjParseError>,
    }
    impl ::field_selector::FieldSelector for AssetImportMessage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The MIME content-type, such as `image/png`.\nFor more information, see\n[MIME\ntypes](//developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)."]
        #[serde(rename = "contentType", default)]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The path of the resource file relative to the\nroot file. For root or thumbnail files,\nthis is just the filename."]
        #[serde(rename = "relativePath", default)]
        pub relative_path: ::std::option::Option<String>,
        #[doc = "The URL where the file data can be retrieved."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for File {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[serde(rename = "formatComplexity", default)]
        pub format_complexity: ::std::option::Option<crate::schemas::FormatComplexity>,
        #[doc = "A short string that identifies the format type of this representation.\nPossible values are: `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`."]
        #[serde(rename = "formatType", default)]
        pub format_type: ::std::option::Option<String>,
        #[doc = "A list of dependencies of the root element. May include, but is not\nlimited to, materials, textures, and shader programs."]
        #[serde(rename = "resources", default)]
        pub resources: ::std::option::Option<Vec<crate::schemas::File>>,
        #[doc = "The root of the file hierarchy. This will always be populated.\nFor some format_types - such as `TILT`, which are\nself-contained - this is all of the data.\n\nOther types - such as `OBJ` - often reference other data elements.\nThese are contained in the resources field."]
        #[serde(rename = "root", default)]
        pub root: ::std::option::Option<crate::schemas::File>,
    }
    impl ::field_selector::FieldSelector for Format {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "A non-negative integer that represents the level of detail (LOD) of this\nformat relative to other formats of the same asset with the same\nformat_type.\nThis hint allows you to sort formats from the most-detailed (0) to\nleast-detailed (integers greater than 0)."]
        #[serde(rename = "lodHint", default)]
        pub lod_hint: ::std::option::Option<i32>,
        #[doc = "The estimated number of triangles."]
        #[serde(rename = "triangleCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub triangle_count: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for FormatComplexity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageErrorCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "We were unable to read the image file."]
        InvalidImage,
        #[doc = "The image size is too large."]
        ImageTooBig,
        #[doc = "The image data does not match the expected MIME type of the image."]
        WrongImageType,
    }
    impl ImageErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ImageErrorCode::InvalidImage => "INVALID_IMAGE",
                ImageErrorCode::ImageTooBig => "IMAGE_TOO_BIG",
                ImageErrorCode::WrongImageType => "WRONG_IMAGE_TYPE",
            }
        }
    }
    impl ::std::fmt::Display for ImageErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ImageErrorCode::CodeUnspecified,
                "INVALID_IMAGE" => ImageErrorCode::InvalidImage,
                "IMAGE_TOO_BIG" => ImageErrorCode::ImageTooBig,
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
    impl ::field_selector::FieldSelector for ImageErrorCode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::ImageErrorCode>,
        #[doc = "The file path in the import of the image that was rejected."]
        #[serde(rename = "filePath", default)]
        pub file_path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ImageError {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListAssetsResponse {
        #[doc = "A list of assets that match the criteria specified in the request."]
        #[serde(rename = "assets", default)]
        pub assets: ::std::option::Option<Vec<crate::schemas::Asset>>,
        #[doc = "The continuation token for retrieving the next page. If empty,\nindicates that there are no more pages. To get the next page, submit the\nsame request specifying this value as the\npage_token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(rename = "totalSize", default)]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ListAssetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListLikedAssetsResponse {
        #[doc = "A list of assets that match the criteria specified in the request."]
        #[serde(rename = "assets", default)]
        pub assets: ::std::option::Option<Vec<crate::schemas::Asset>>,
        #[doc = "The continuation token for retrieving the next page. If empty,\nindicates that there are no more pages. To get the next page, submit the\nsame request specifying this value as the\npage_token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(rename = "totalSize", default)]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ListLikedAssetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListUserAssetsResponse {
        #[doc = "The continuation token for retrieving the next page. If empty,\nindicates that there are no more pages. To get the next page, submit the\nsame request specifying this value as the\npage_token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(rename = "totalSize", default)]
        pub total_size: ::std::option::Option<i32>,
        #[doc = "A list of UserAssets matching the request."]
        #[serde(rename = "userAssets", default)]
        pub user_assets: ::std::option::Option<Vec<crate::schemas::UserAsset>>,
    }
    impl ::field_selector::FieldSelector for ListUserAssetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ObjParseErrorCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "Vertex references are specified in an inconsistent style for a face (e.g.\nsome vertices specify texture vertices but some don't)."]
        InconsistentVertexRefs,
        #[doc = "The command is invalid."]
        InvalidCommand,
        #[doc = "A invalid number was specified."]
        InvalidNumber,
        #[doc = "An invalid vertex reference was specified."]
        InvalidVertexRef,
        #[doc = "A vertex reference does not specify a geometric vertex."]
        MissingGeometricVertex,
        #[doc = "An expected token was not found."]
        MissingToken,
        #[doc = "The vertex specified too few dimensions for its usage."]
        TooFewDimensions,
        #[doc = "The face specified too few vertices."]
        TooFewVertices,
        #[doc = "The vertex specified too many dimensions for its usage."]
        TooManyDimensions,
        #[doc = "This command is a valid OBJ command but is not supported. This error is\nonly generated for the first instance of such a command."]
        UnsupportedCommand,
        #[doc = "This line ended with unparsed token characters."]
        UnusedTokens,
        #[doc = "The specified vertex was not found."]
        VertexNotFound,
        #[doc = "The specified number was too large or small for its usage."]
        NumberOutOfRange,
        #[doc = "The specified parameter value was not recognized."]
        InvalidValue,
        #[doc = "The specified texture option is not valid."]
        InvalidTextureOption,
        #[doc = "The maximum number of problems to report was reached. Parsing continues,\nbut further problems will be ignored."]
        TooManyProblems,
        #[doc = "An expected file name was not specified."]
        MissingFileName,
        #[doc = "The specified file was not found in the import."]
        FileNotFound,
        #[doc = "The specified material was not found in any material definition in the\nimport."]
        UnknownMaterial,
        #[doc = "Material parameters were specified before the first material definition."]
        NoMaterialDefined,
        #[doc = "The smoothing group is not valid."]
        InvalidSmoothingGroup,
        #[doc = "Vertex colors were specified for only some vertices of a face."]
        MissingVertexColors,
        #[doc = "A missing file was found at a different file path."]
        FileSubstitution,
        #[doc = "A line in an OBJ or MTL file exceeded the maximum line length."]
        LineTooLong,
        #[doc = "The file path was invalid. Only relative paths are supported."]
        InvalidFilePath,
    }
    impl ObjParseErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ObjParseErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ObjParseErrorCode::InconsistentVertexRefs => "INCONSISTENT_VERTEX_REFS",
                ObjParseErrorCode::InvalidCommand => "INVALID_COMMAND",
                ObjParseErrorCode::InvalidNumber => "INVALID_NUMBER",
                ObjParseErrorCode::InvalidVertexRef => "INVALID_VERTEX_REF",
                ObjParseErrorCode::MissingGeometricVertex => "MISSING_GEOMETRIC_VERTEX",
                ObjParseErrorCode::MissingToken => "MISSING_TOKEN",
                ObjParseErrorCode::TooFewDimensions => "TOO_FEW_DIMENSIONS",
                ObjParseErrorCode::TooFewVertices => "TOO_FEW_VERTICES",
                ObjParseErrorCode::TooManyDimensions => "TOO_MANY_DIMENSIONS",
                ObjParseErrorCode::UnsupportedCommand => "UNSUPPORTED_COMMAND",
                ObjParseErrorCode::UnusedTokens => "UNUSED_TOKENS",
                ObjParseErrorCode::VertexNotFound => "VERTEX_NOT_FOUND",
                ObjParseErrorCode::NumberOutOfRange => "NUMBER_OUT_OF_RANGE",
                ObjParseErrorCode::InvalidValue => "INVALID_VALUE",
                ObjParseErrorCode::InvalidTextureOption => "INVALID_TEXTURE_OPTION",
                ObjParseErrorCode::TooManyProblems => "TOO_MANY_PROBLEMS",
                ObjParseErrorCode::MissingFileName => "MISSING_FILE_NAME",
                ObjParseErrorCode::FileNotFound => "FILE_NOT_FOUND",
                ObjParseErrorCode::UnknownMaterial => "UNKNOWN_MATERIAL",
                ObjParseErrorCode::NoMaterialDefined => "NO_MATERIAL_DEFINED",
                ObjParseErrorCode::InvalidSmoothingGroup => "INVALID_SMOOTHING_GROUP",
                ObjParseErrorCode::MissingVertexColors => "MISSING_VERTEX_COLORS",
                ObjParseErrorCode::FileSubstitution => "FILE_SUBSTITUTION",
                ObjParseErrorCode::LineTooLong => "LINE_TOO_LONG",
                ObjParseErrorCode::InvalidFilePath => "INVALID_FILE_PATH",
            }
        }
    }
    impl ::std::fmt::Display for ObjParseErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ObjParseErrorCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ObjParseErrorCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ObjParseErrorCode::CodeUnspecified,
                "INCONSISTENT_VERTEX_REFS" => ObjParseErrorCode::InconsistentVertexRefs,
                "INVALID_COMMAND" => ObjParseErrorCode::InvalidCommand,
                "INVALID_NUMBER" => ObjParseErrorCode::InvalidNumber,
                "INVALID_VERTEX_REF" => ObjParseErrorCode::InvalidVertexRef,
                "MISSING_GEOMETRIC_VERTEX" => ObjParseErrorCode::MissingGeometricVertex,
                "MISSING_TOKEN" => ObjParseErrorCode::MissingToken,
                "TOO_FEW_DIMENSIONS" => ObjParseErrorCode::TooFewDimensions,
                "TOO_FEW_VERTICES" => ObjParseErrorCode::TooFewVertices,
                "TOO_MANY_DIMENSIONS" => ObjParseErrorCode::TooManyDimensions,
                "UNSUPPORTED_COMMAND" => ObjParseErrorCode::UnsupportedCommand,
                "UNUSED_TOKENS" => ObjParseErrorCode::UnusedTokens,
                "VERTEX_NOT_FOUND" => ObjParseErrorCode::VertexNotFound,
                "NUMBER_OUT_OF_RANGE" => ObjParseErrorCode::NumberOutOfRange,
                "INVALID_VALUE" => ObjParseErrorCode::InvalidValue,
                "INVALID_TEXTURE_OPTION" => ObjParseErrorCode::InvalidTextureOption,
                "TOO_MANY_PROBLEMS" => ObjParseErrorCode::TooManyProblems,
                "MISSING_FILE_NAME" => ObjParseErrorCode::MissingFileName,
                "FILE_NOT_FOUND" => ObjParseErrorCode::FileNotFound,
                "UNKNOWN_MATERIAL" => ObjParseErrorCode::UnknownMaterial,
                "NO_MATERIAL_DEFINED" => ObjParseErrorCode::NoMaterialDefined,
                "INVALID_SMOOTHING_GROUP" => ObjParseErrorCode::InvalidSmoothingGroup,
                "MISSING_VERTEX_COLORS" => ObjParseErrorCode::MissingVertexColors,
                "FILE_SUBSTITUTION" => ObjParseErrorCode::FileSubstitution,
                "LINE_TOO_LONG" => ObjParseErrorCode::LineTooLong,
                "INVALID_FILE_PATH" => ObjParseErrorCode::InvalidFilePath,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ObjParseErrorCode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::ObjParseErrorCode>,
        #[doc = "The ending character index at which the problem was found."]
        #[serde(rename = "endIndex", default)]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The file path in which the problem was found."]
        #[serde(rename = "filePath", default)]
        pub file_path: ::std::option::Option<String>,
        #[doc = "The text of the line. Note that this may be truncated if the line was very\nlong. This may not include the error if it occurs after line truncation."]
        #[serde(rename = "line", default)]
        pub line: ::std::option::Option<String>,
        #[doc = "Line number at which the problem was found."]
        #[serde(rename = "lineNumber", default)]
        pub line_number: ::std::option::Option<i32>,
        #[doc = "The starting character index at which the problem was found."]
        #[serde(rename = "startIndex", default)]
        pub start_index: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ObjParseError {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PresentationParamsColorSpace {
        #[doc = "Invalid color value."]
        Unknown,
        #[doc = "Linear color values. Default."]
        Linear,
        #[doc = "Colors should be converted to linear by assuming gamma = 2.0."]
        Gamma,
    }
    impl PresentationParamsColorSpace {
        pub fn as_str(self) -> &'static str {
            match self {
                PresentationParamsColorSpace::Unknown => "UNKNOWN",
                PresentationParamsColorSpace::Linear => "LINEAR",
                PresentationParamsColorSpace::Gamma => "GAMMA",
            }
        }
    }
    impl ::std::fmt::Display for PresentationParamsColorSpace {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PresentationParamsColorSpace {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PresentationParamsColorSpace {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN" => PresentationParamsColorSpace::Unknown,
                "LINEAR" => PresentationParamsColorSpace::Linear,
                "GAMMA" => PresentationParamsColorSpace::Gamma,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PresentationParamsColorSpace {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PresentationParams {
        #[doc = "A background color which could be used for displaying the 3D asset in a\n'thumbnail' or 'palette' style view. Authors have the option to set this\nbackground color when publishing or editing their asset.\n\nThis is represented as a six-digit hexademical triplet specifying the\nRGB components of the background color, e.g. #FF0000 for Red."]
        #[serde(rename = "backgroundColor", default)]
        pub background_color: ::std::option::Option<String>,
        #[doc = "The materials' diffuse/albedo color. This does not apply to vertex colors\nor texture maps."]
        #[serde(rename = "colorSpace", default)]
        pub color_space: ::std::option::Option<crate::schemas::PresentationParamsColorSpace>,
        #[doc = "A rotation that should be applied to the object root to make it upright.\nMore precisely, this quaternion transforms from \"object space\" (the space\nin which the object is defined) to \"presentation space\", a coordinate\nsystem where +Y is up, +X is right, -Z is forward. For example, if\nthe object is the Eiffel Tower, in its local coordinate system the\nobject might be laid out such that the base of the tower is on the\nYZ plane and the tip of the tower is towards positive X. In this case\nthis quaternion would specify a rotation (of 90 degrees about the Z\naxis) such that in the presentation space the base of the tower is\naligned with the XZ plane, and the tip of the tower lies towards +Y.\n\nThis rotation is unrelated to the object's pose in the web preview,\nwhich is just a camera position setting and is *not* reflected in this\nrotation.\n\nPlease note: this is applicable only to the gLTF."]
        #[serde(rename = "orientingRotation", default)]
        pub orienting_rotation: ::std::option::Option<crate::schemas::Quaternion>,
    }
    impl ::field_selector::FieldSelector for PresentationParams {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Quaternion {
        #[doc = "The scalar component."]
        #[serde(rename = "w", default)]
        pub w: ::std::option::Option<f64>,
        #[doc = "The x component."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f64>,
        #[doc = "The y component."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f64>,
        #[doc = "The z component."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for Quaternion {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Resource ids for the sources of this remix, of the form:\n`assets/{ASSET_ID}`"]
        #[serde(rename = "sourceAsset", default)]
        pub source_asset: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for RemixInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The id of newly created asset. If this is empty when the operation is\ncomplete it means the import failed. Please refer to the\nassetImportMessages field to understand what went wrong."]
        #[serde(rename = "assetId", default)]
        pub asset_id: ::std::option::Option<String>,
        #[doc = "The id of the asset import."]
        #[serde(rename = "assetImportId", default)]
        pub asset_import_id: ::std::option::Option<String>,
        #[doc = "The message from the asset import. This will contain any warnings\n(or - in the case of failure - errors) that occurred during import."]
        #[serde(rename = "assetImportMessages", default)]
        pub asset_import_messages: ::std::option::Option<Vec<crate::schemas::AssetImportMessage>>,
        #[doc = "The publish URL for the asset."]
        #[serde(rename = "publishUrl", default)]
        pub publish_url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StartAssetImportResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UserAsset {
        #[doc = "An Asset."]
        #[serde(rename = "asset", default)]
        pub asset: ::std::option::Option<crate::schemas::Asset>,
    }
    impl ::field_selector::FieldSelector for UserAsset {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Alt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    impl ::field_selector::FieldSelector for Xgafv {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
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
    #[doc = "Actions that can be performed on the assets resource"]
    pub fn assets(&self) -> crate::resources::assets::AssetsActions<A> {
        crate::resources::assets::AssetsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the users resource"]
    pub fn users(&self) -> crate::resources::users::UsersActions<A> {
        crate::resources::users::UsersActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
mod resources {
    pub mod assets {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListMaxComplexity {
                ComplexityUnspecified,
                Complex,
                Medium,
                Simple,
            }
            impl ListMaxComplexity {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListMaxComplexity::ComplexityUnspecified => "COMPLEXITY_UNSPECIFIED",
                        ListMaxComplexity::Complex => "COMPLEX",
                        ListMaxComplexity::Medium => "MEDIUM",
                        ListMaxComplexity::Simple => "SIMPLE",
                    }
                }
            }
            impl ::std::fmt::Display for ListMaxComplexity {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListMaxComplexity {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListMaxComplexity {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "COMPLEXITY_UNSPECIFIED" => ListMaxComplexity::ComplexityUnspecified,
                        "COMPLEX" => ListMaxComplexity::Complex,
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
            impl ::field_selector::FieldSelector for ListMaxComplexity {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct AssetsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AssetsActions<'a, A> {
            #[doc = "Returns detailed information about an asset given its name.\nPRIVATE assets are returned only if\nthe currently authenticated user (via OAuth token) is the author of the\nasset."]
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
            #[doc = "Lists all public, remixable assets. These are assets with an access level\nof PUBLIC and published under the\nCC-By license."]
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
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::Asset, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Asset, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
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
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Filter assets based on the specified category. Supported values are:\n`animals`, `architecture`, `art`, `food`, `nature`, `objects`, `people`,\n`scenes`, `technology`, and `transport`."]
            pub fn category(mut self, value: impl Into<String>) -> Self {
                self.category = Some(value.into());
                self
            }
            #[doc = "Return only assets that have been curated by the Poly team."]
            pub fn curated(mut self, value: bool) -> Self {
                self.curated = Some(value);
                self
            }
            #[doc = "Return only assets with the matching format. Acceptable values are:\n`BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`."]
            pub fn format(mut self, value: impl Into<String>) -> Self {
                self.format = Some(value.into());
                self
            }
            #[doc = "One or more search terms to be matched against all text that Poly has\nindexed for assets, which includes display_name,\ndescription, and tags. Multiple keywords should be\nseparated by spaces."]
            pub fn keywords(mut self, value: impl Into<String>) -> Self {
                self.keywords = Some(value.into());
                self
            }
            #[doc = "Returns assets that are of the specified complexity or less. Defaults to\nCOMPLEX. For example, a request for\nMEDIUM assets also includes\nSIMPLE assets."]
            pub fn max_complexity(
                mut self,
                value: crate::resources::assets::params::ListMaxComplexity,
            ) -> Self {
                self.max_complexity = Some(value);
                self
            }
            #[doc = "Specifies an ordering for assets. Acceptable values are:\n`BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets\nbased on a combination of popularity and other features."]
            pub fn order_by(mut self, value: impl Into<String>) -> Self {
                self.order_by = Some(value.into());
                self
            }
            #[doc = "The maximum number of assets to be returned. This value must be between `1`\nand `100`. Defaults to `20`."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Specifies a continuation token from a previous search whose results were\nsplit into multiple pages. To get the next page, submit the same request\nspecifying the value from\nnext_page_token."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(mut self, value: impl Into<String>) -> Self {
                self.fields = Some(value.into());
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
            pub fn iter_assets<T>(self) -> ListAssetsIter<'a, A, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                ListAssetsIter {
                    method: self,
                    last_page_reached: false,
                    items_iter: None,
                }
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_assets_standard(mut self) -> ListAssetsIter<'a, A, crate::schemas::Asset> {
                self.fields = Some(concat!("nextPageToken,", "assets").to_owned());
                ListAssetsIter {
                    method: self,
                    last_page_reached: false,
                    items_iter: None,
                }
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_assets_debug(mut self) -> ListAssetsIter<'a, A, crate::schemas::Asset> {
                self.fields = Some(concat!("nextPageToken,", "assets", "(*)").to_owned());
                ListAssetsIter {
                    method: self,
                    last_page_reached: false,
                    items_iter: None,
                }
            }
            #[doc = r" Return an iterator that"]
            pub fn iter<T>(
                self,
            ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error + 'static>>> + 'a
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
            {
                crate::PageIter {
                    method: self,
                    finished: false,
                    _phantom: ::std::default::Default::default(),
                }
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
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::ListAssetsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListAssetsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
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
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://poly.googleapis.com/".to_owned();
                output.push_str("v1/assets");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("category", &self.category)]);
                let req = req.query(&[("curated", &self.curated)]);
                let req = req.query(&[("format", &self.format)]);
                let req = req.query(&[("keywords", &self.keywords)]);
                let req = req.query(&[("maxComplexity", &self.max_complexity)]);
                let req = req.query(&[("orderBy", &self.order_by)]);
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
                req
            }
        }
        pub struct ListAssetsIter<'a, A, T> {
            method: ListRequestBuilder<'a, A>,
            last_page_reached: bool,
            items_iter: Option<::std::vec::IntoIter<T>>,
        }
        impl<'a, A, T> Iterator for ListAssetsIter<'a, A, T>
        where
            A: ::yup_oauth2::GetToken,
            T: ::serde::de::DeserializeOwned,
        {
            type Item = Result<T, Box<dyn ::std::error::Error>>;
            fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                #[derive(:: serde :: Deserialize)]
                struct Resp<T> {
                    #[serde(rename = "assets")]
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
                    if self.last_page_reached {
                        return None;
                    }
                    let resp: Resp<T> = match self.method._execute() {
                        Ok(r) => r,
                        Err(err) => return Some(Err(err)),
                    };
                    self.last_page_reached = resp.next_page_token.as_ref().is_none();
                    self.method.page_token = resp.next_page_token;
                    self.items_iter = resp.items.map(|i| i.into_iter());
                }
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
    pub mod users {
        pub mod params {}
        pub struct UsersActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UsersActions<'a, A> {
            #[doc = "Actions that can be performed on the assets resource"]
            pub fn assets(&self) -> crate::resources::users::assets::AssetsActions<A> {
                crate::resources::users::assets::AssetsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the likedassets resource"]
            pub fn likedassets(
                &self,
            ) -> crate::resources::users::likedassets::LikedassetsActions<A> {
                crate::resources::users::likedassets::LikedassetsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod assets {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListVisibility {
                    VisibilityUnspecified,
                    Published,
                    Private,
                }
                impl ListVisibility {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListVisibility::VisibilityUnspecified => "VISIBILITY_UNSPECIFIED",
                            ListVisibility::Published => "PUBLISHED",
                            ListVisibility::Private => "PRIVATE",
                        }
                    }
                }
                impl ::std::fmt::Display for ListVisibility {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListVisibility {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListVisibility {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "VISIBILITY_UNSPECIFIED" => ListVisibility::VisibilityUnspecified,
                            "PUBLISHED" => ListVisibility::Published,
                            "PRIVATE" => ListVisibility::Private,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for ListVisibility {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
            }
            pub struct AssetsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> AssetsActions<'a, A> {
                #[doc = "Lists assets authored by the given user. Only the value 'me', representing\nthe currently-authenticated user, is supported. May include assets with an\naccess level of PRIVATE or\nUNLISTED and assets which are\nAll Rights Reserved for the\ncurrently-authenticated user."]
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
                        format: None,
                        order_by: None,
                        page_size: None,
                        page_token: None,
                        visibility: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Return only assets with the matching format. Acceptable values are:\n`BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`."]
                pub fn format(mut self, value: impl Into<String>) -> Self {
                    self.format = Some(value.into());
                    self
                }
                #[doc = "Specifies an ordering for assets. Acceptable values are:\n`BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets\nbased on a combination of popularity and other features."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The maximum number of assets to be returned. This value must be between `1`\nand `100`. Defaults to `20`."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Specifies a continuation token from a previous search whose results were\nsplit into multiple pages. To get the next page, submit the same request\nspecifying the value from\nnext_page_token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "The visibility of the assets to be returned.\nDefaults to\nVISIBILITY_UNSPECIFIED\nwhich returns all assets."]
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
                #[doc = "Data format for response."]
                pub fn alt(mut self, value: crate::params::Alt) -> Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(mut self, value: impl Into<String>) -> Self {
                    self.fields = Some(value.into());
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
                pub fn iter_user_assets<T>(self) -> ListUserAssetsIter<'a, A, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    ListUserAssetsIter {
                        method: self,
                        last_page_reached: false,
                        items_iter: None,
                    }
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_user_assets_standard(
                    mut self,
                ) -> ListUserAssetsIter<'a, A, crate::schemas::UserAsset> {
                    self.fields = Some(concat!("nextPageToken,", "userAssets").to_owned());
                    ListUserAssetsIter {
                        method: self,
                        last_page_reached: false,
                        items_iter: None,
                    }
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_user_assets_debug(
                    mut self,
                ) -> ListUserAssetsIter<'a, A, crate::schemas::UserAsset> {
                    self.fields = Some(concat!("nextPageToken,", "userAssets", "(*)").to_owned());
                    ListUserAssetsIter {
                        method: self,
                        last_page_reached: false,
                        items_iter: None,
                    }
                }
                #[doc = r" Return an iterator that"]
                pub fn iter<T>(
                    self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error + 'static>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
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
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let fields = T::field_selector();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_standard(
                    self,
                ) -> Result<crate::schemas::ListUserAssetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListUserAssetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_fields<T, F>(
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
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
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
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("format", &self.format)]);
                    let req = req.query(&[("orderBy", &self.order_by)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("visibility", &self.visibility)]);
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
                    req
                }
            }
            pub struct ListUserAssetsIter<'a, A, T> {
                method: ListRequestBuilder<'a, A>,
                last_page_reached: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, A, T> Iterator for ListUserAssetsIter<'a, A, T>
            where
                A: ::yup_oauth2::GetToken,
                T: ::serde::de::DeserializeOwned,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    #[derive(:: serde :: Deserialize)]
                    struct Resp<T> {
                        #[serde(rename = "userAssets")]
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
                        if self.last_page_reached {
                            return None;
                        }
                        let resp: Resp<T> = match self.method._execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        self.last_page_reached = resp.next_page_token.as_ref().is_none();
                        self.method.page_token = resp.next_page_token;
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
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
        pub mod likedassets {
            pub mod params {}
            pub struct LikedassetsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> LikedassetsActions<'a, A> {
                #[doc = "Lists assets that the user has liked. Only the value 'me', representing\nthe currently-authenticated user, is supported. May include assets with an\naccess level of UNLISTED."]
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
                        format: None,
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
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
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "Return only assets with the matching format. Acceptable values are:\n`BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`."]
                pub fn format(mut self, value: impl Into<String>) -> Self {
                    self.format = Some(value.into());
                    self
                }
                #[doc = "Specifies an ordering for assets. Acceptable values are:\n`BEST`, `NEWEST`, `OLDEST`, 'LIKED_TIME'. Defaults to `LIKED_TIME`, which\nranks assets based on how recently they were liked."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The maximum number of assets to be returned. This value must be between `1`\nand `100`. Defaults to `20`."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Specifies a continuation token from a previous search whose results were\nsplit into multiple pages. To get the next page, submit the same request\nspecifying the value from\nnext_page_token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(mut self, value: crate::params::Alt) -> Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
                    self
                }
                #[doc = "Selector specifying which fields to include in a partial response."]
                pub fn fields(mut self, value: impl Into<String>) -> Self {
                    self.fields = Some(value.into());
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
                pub fn iter_assets<T>(self) -> ListAssetsIter<'a, A, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    ListAssetsIter {
                        method: self,
                        last_page_reached: false,
                        items_iter: None,
                    }
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_assets_standard(
                    mut self,
                ) -> ListAssetsIter<'a, A, crate::schemas::Asset> {
                    self.fields = Some(concat!("nextPageToken,", "assets").to_owned());
                    ListAssetsIter {
                        method: self,
                        last_page_reached: false,
                        items_iter: None,
                    }
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_assets_debug(mut self) -> ListAssetsIter<'a, A, crate::schemas::Asset> {
                    self.fields = Some(concat!("nextPageToken,", "assets", "(*)").to_owned());
                    ListAssetsIter {
                        method: self,
                        last_page_reached: false,
                        items_iter: None,
                    }
                }
                #[doc = r" Return an iterator that"]
                pub fn iter<T>(
                    self,
                ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error + 'static>>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
                {
                    crate::PageIter {
                        method: self,
                        finished: false,
                        _phantom: ::std::default::Default::default(),
                    }
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
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let fields = T::field_selector();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_standard(
                    self,
                ) -> Result<crate::schemas::ListLikedAssetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListLikedAssetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_fields<T, F>(
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
                    let req = self._request(&self._path());
                    Ok(req.send()?.error_for_status()?.json()?)
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
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("format", &self.format)]);
                    let req = req.query(&[("orderBy", &self.order_by)]);
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
                    req
                }
            }
            pub struct ListAssetsIter<'a, A, T> {
                method: ListRequestBuilder<'a, A>,
                last_page_reached: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, A, T> Iterator for ListAssetsIter<'a, A, T>
            where
                A: ::yup_oauth2::GetToken,
                T: ::serde::de::DeserializeOwned,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    #[derive(:: serde :: Deserialize)]
                    struct Resp<T> {
                        #[serde(rename = "assets")]
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
                        if self.last_page_reached {
                            return None;
                        }
                        let resp: Resp<T> = match self.method._execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        self.last_page_reached = resp.next_page_token.as_ref().is_none();
                        self.method.page_token = resp.next_page_token;
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
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
#[allow(dead_code)]
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

#[allow(dead_code)]
struct PageIter<M, T> {
    method: M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<M, T> Iterator for PageIter<M, T>
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
} // Bytes in google apis are represented as urlsafe base64 encoded strings.
  // This defines a Bytes type that is a simple wrapper around a Vec<u8> used
  // internally to handle byte fields in google apis.
#[allow(dead_code)]
mod bytes {
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
