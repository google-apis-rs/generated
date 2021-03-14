#![doc = "# Resources and Methods\n    * [documents](resources/documents/struct.DocumentsActions.html)\n      * [*batchUpdate*](resources/documents/struct.BatchUpdateRequestBuilder.html), [*create*](resources/documents/struct.CreateRequestBuilder.html), [*get*](resources/documents/struct.GetRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, create, and edit all Google Docs documents you have access to\n\n`https://www.googleapis.com/auth/documents`"]
    pub const DOCUMENTS: &str = "https://www.googleapis.com/auth/documents";
    #[doc = "View your Google Docs documents\n\n`https://www.googleapis.com/auth/documents.readonly`"]
    pub const DOCUMENTS_READONLY: &str = "https://www.googleapis.com/auth/documents.readonly";
    #[doc = "See, edit, create, and delete all of your Google Drive files\n\n`https://www.googleapis.com/auth/drive`"]
    pub const DRIVE: &str = "https://www.googleapis.com/auth/drive";
    #[doc = "View and manage Google Drive files and folders that you have opened or created with this app\n\n`https://www.googleapis.com/auth/drive.file`"]
    pub const DRIVE_FILE: &str = "https://www.googleapis.com/auth/drive.file";
    #[doc = "See and download all your Google Drive files\n\n`https://www.googleapis.com/auth/drive.readonly`"]
    pub const DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AutoText {
        #[doc = "The type of this auto text."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AutoTextType>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. An AutoText may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this AutoText, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this AutoText."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
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
        #[doc = "Type for auto text that represents the total number of pages in the document."]
        PageCount,
        #[doc = "Type for auto text that represents the current page number."]
        PageNumber,
        #[doc = "An unspecified auto text type."]
        TypeUnspecified,
    }
    impl AutoTextType {
        pub fn as_str(self) -> &'static str {
            match self {
                AutoTextType::PageCount => "PAGE_COUNT",
                AutoTextType::PageNumber => "PAGE_NUMBER",
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
                "PAGE_COUNT" => AutoTextType::PageCount,
                "PAGE_NUMBER" => AutoTextType::PageNumber,
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
                "PAGE_COUNT" => AutoTextType::PageCount,
                "PAGE_NUMBER" => AutoTextType::PageNumber,
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
    pub struct Background {
        #[doc = "The background color."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::OptionalColor>,
    }
    impl ::google_field_selector::FieldSelector for Background {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Background {
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
    pub struct BackgroundSuggestionState {
        #[doc = "Indicates whether the current background color has been modified in this suggestion."]
        #[serde(
            rename = "backgroundColorSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for BackgroundSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BackgroundSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchUpdateDocumentRequest {
        #[doc = "A list of updates to apply to the document."]
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
    impl ::google_field_selector::FieldSelector for BatchUpdateDocumentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateDocumentRequest {
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
    pub struct BatchUpdateDocumentResponse {
        #[doc = "The ID of the document to which the updates were applied to."]
        #[serde(
            rename = "documentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_id: ::std::option::Option<String>,
        #[doc = "The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty."]
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
    impl ::google_field_selector::FieldSelector for BatchUpdateDocumentResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUpdateDocumentResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Body {
        #[doc = "The contents of the body. The indexes for the body's content begin at zero."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<Vec<crate::schemas::StructuralElement>>,
    }
    impl ::google_field_selector::FieldSelector for Body {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Body {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Bullet {
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
        #[doc = "The paragraph specific text style applied to this bullet."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BulletSuggestionState {
        #[doc = "Indicates if there was a suggested change to the list_id."]
        #[serde(
            rename = "listIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to the nesting_level."]
        #[serde(
            rename = "nestingLevelSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nesting_level_suggested: ::std::option::Option<bool>,
        #[doc = "A mask that indicates which of the fields in text style have been changed in this suggestion."]
        #[serde(
            rename = "textStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style_suggestion_state:
            ::std::option::Option<crate::schemas::TextStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for BulletSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BulletSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Color {
        #[doc = "The RGB color value."]
        #[serde(
            rename = "rgbColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rgb_color: ::std::option::Option<crate::schemas::RgbColor>,
    }
    impl ::google_field_selector::FieldSelector for Color {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Color {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColumnBreak {
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A ColumnBreak may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this ColumnBreak, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this ColumnBreak. Similar to text content, like text runs and footnote references, the text style of a column break can affect content layout as well as the styling of text inserted adjacent to it."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for ColumnBreak {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ColumnBreak {
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
    pub struct CreateFooterRequest {
        #[doc = "The type of footer to create."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CreateFooterRequestType>,
        #[doc = "The location of the SectionBreak immediately preceding the section whose SectionStyle this footer should belong to. If this is unset or refers to the first section break in the document, the footer applies to the document style."]
        #[serde(
            rename = "sectionBreakLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_break_location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::google_field_selector::FieldSelector for CreateFooterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateFooterRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateFooterRequestType {
        #[doc = "A default header/footer."]
        Default,
        #[doc = "The header/footer type is unspecified."]
        HeaderFooterTypeUnspecified,
    }
    impl CreateFooterRequestType {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateFooterRequestType::Default => "DEFAULT",
                CreateFooterRequestType::HeaderFooterTypeUnspecified => {
                    "HEADER_FOOTER_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateFooterRequestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateFooterRequestType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreateFooterRequestType, ()> {
            Ok(match s {
                "DEFAULT" => CreateFooterRequestType::Default,
                "HEADER_FOOTER_TYPE_UNSPECIFIED" => {
                    CreateFooterRequestType::HeaderFooterTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateFooterRequestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateFooterRequestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateFooterRequestType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => CreateFooterRequestType::Default,
                "HEADER_FOOTER_TYPE_UNSPECIFIED" => {
                    CreateFooterRequestType::HeaderFooterTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for CreateFooterRequestType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateFooterRequestType {
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
    pub struct CreateFooterResponse {
        #[doc = "The ID of the created footer."]
        #[serde(
            rename = "footerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footer_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateFooterResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateFooterResponse {
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
    pub struct CreateFootnoteRequest {
        #[doc = "Inserts the footnote reference at the end of the document body. Footnote references cannot be inserted inside a header, footer or footnote. Since footnote references can only be inserted in the body, the segment ID field must be empty."]
        #[serde(
            rename = "endOfSegmentLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_of_segment_location: ::std::option::Option<crate::schemas::EndOfSegmentLocation>,
        #[doc = "Inserts the footnote reference at a specific index in the document. The footnote reference must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Footnote references cannot be inserted inside an equation, header, footer or footnote. Since footnote references can only be inserted in the body, the segment ID field must be empty."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::google_field_selector::FieldSelector for CreateFootnoteRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateFootnoteRequest {
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
    pub struct CreateFootnoteResponse {
        #[doc = "The ID of the created footnote."]
        #[serde(
            rename = "footnoteId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footnote_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateFootnoteResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateFootnoteResponse {
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
    pub struct CreateHeaderRequest {
        #[doc = "The type of header to create."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::CreateHeaderRequestType>,
        #[doc = "The location of the SectionBreak which begins the section this header should belong to. If `section_break_location' is unset or if it refers to the first section break in the document body, the header applies to the DocumentStyle"]
        #[serde(
            rename = "sectionBreakLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_break_location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::google_field_selector::FieldSelector for CreateHeaderRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateHeaderRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateHeaderRequestType {
        #[doc = "A default header/footer."]
        Default,
        #[doc = "The header/footer type is unspecified."]
        HeaderFooterTypeUnspecified,
    }
    impl CreateHeaderRequestType {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateHeaderRequestType::Default => "DEFAULT",
                CreateHeaderRequestType::HeaderFooterTypeUnspecified => {
                    "HEADER_FOOTER_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreateHeaderRequestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreateHeaderRequestType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreateHeaderRequestType, ()> {
            Ok(match s {
                "DEFAULT" => CreateHeaderRequestType::Default,
                "HEADER_FOOTER_TYPE_UNSPECIFIED" => {
                    CreateHeaderRequestType::HeaderFooterTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreateHeaderRequestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateHeaderRequestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateHeaderRequestType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => CreateHeaderRequestType::Default,
                "HEADER_FOOTER_TYPE_UNSPECIFIED" => {
                    CreateHeaderRequestType::HeaderFooterTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for CreateHeaderRequestType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateHeaderRequestType {
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
    pub struct CreateHeaderResponse {
        #[doc = "The ID of the created header."]
        #[serde(
            rename = "headerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateHeaderResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateHeaderResponse {
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
    pub struct CreateNamedRangeRequest {
        #[doc = "The name of the NamedRange. Names do not need to be unique. Names must be at least 1 character and no more than 256 characters, measured in UTF-16 code units."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The range to apply the name to."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::Range>,
    }
    impl ::google_field_selector::FieldSelector for CreateNamedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateNamedRangeRequest {
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
    pub struct CreateNamedRangeResponse {
        #[doc = "The ID of the created named range."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateNamedRangeResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateNamedRangeResponse {
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
        #[doc = "The kinds of bullet glyphs to be used."]
        #[serde(
            rename = "bulletPreset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet_preset:
            ::std::option::Option<crate::schemas::CreateParagraphBulletsRequestBulletPreset>,
        #[doc = "The range to apply the bullet preset to."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::Range>,
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
        #[doc = "A bulleted list with a `ARROW3D`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletArrow3DCircleSquare,
        #[doc = "A bulleted list with a `ARROW`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels."]
        BulletArrowDiamondDisc,
        #[doc = "A bulleted list with `CHECKBOX` bullet glyphs for all list nesting levels."]
        BulletCheckbox,
        #[doc = "A bulleted list with a `DIAMOND`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiamondCircleSquare,
        #[doc = "A bulleted list with a `DIAMONDX`, `ARROW3D` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiamondxArrow3DSquare,
        #[doc = "A bulleted list with a `DIAMONDX`, `HOLLOWDIAMOND` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiamondxHollowdiamondSquare,
        #[doc = "A bulleted list with a `DISC`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletDiscCircleSquare,
        #[doc = "The bullet glyph preset is unspecified."]
        BulletGlyphPresetUnspecified,
        #[doc = "A bulleted list with a `LEFTTRIANGLE`, `DIAMOND` and `DISC` bullet glyph for the first 3 list nesting levels."]
        BulletLefttriangleDiamondDisc,
        #[doc = "A bulleted list with a `STAR`, `CIRCLE` and `SQUARE` bullet glyph for the first 3 list nesting levels."]
        BulletStarCircleSquare,
        #[doc = "A numbered list with `DECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedDecimalAlphaRoman,
        #[doc = "A numbered list with `DECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by parenthesis."]
        NumberedDecimalAlphaRomanParens,
        #[doc = "A numbered list with `DECIMAL` numeric glyphs separated by periods, where each nesting level uses the previous nesting level's glyph as a prefix. For example: '1.', '1.1.', '2.', '2.2.'."]
        NumberedDecimalNested,
        #[doc = "A numbered list with `UPPERALPHA`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedUpperalphaAlphaRoman,
        #[doc = "A numbered list with `UPPERROMAN`, `UPPERALPHA` and `DECIMAL` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedUpperromanUpperalphaDecimal,
        #[doc = "A numbered list with `ZERODECIMAL`, `ALPHA` and `ROMAN` numeric glyphs for the first 3 list nesting levels, followed by periods."]
        NumberedZerodecimalAlphaRoman,
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
                CreateParagraphBulletsRequestBulletPreset::BulletGlyphPresetUnspecified => {
                    "BULLET_GLYPH_PRESET_UNSPECIFIED"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc => {
                    "BULLET_LEFTTRIANGLE_DIAMOND_DISC"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare => {
                    "BULLET_STAR_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDecimalAlphaRoman => {
                    "NUMBERED_DECIMAL_ALPHA_ROMAN"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDecimalAlphaRomanParens => {
                    "NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDecimalNested => {
                    "NUMBERED_DECIMAL_NESTED"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman => {
                    "NUMBERED_UPPERALPHA_ALPHA_ROMAN"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDecimal => {
                    "NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedZerodecimalAlphaRoman => {
                    "NUMBERED_ZERODECIMAL_ALPHA_ROMAN"
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
                "BULLET_GLYPH_PRESET_UNSPECIFIED" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletGlyphPresetUnspecified
                }
                "BULLET_LEFTTRIANGLE_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc
                }
                "BULLET_STAR_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare
                }
                "NUMBERED_DECIMAL_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDecimalAlphaRoman
                }
                "NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDecimalAlphaRomanParens
                }
                "NUMBERED_DECIMAL_NESTED" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDecimalNested
                }
                "NUMBERED_UPPERALPHA_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman
                }
                "NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDecimal
                }
                "NUMBERED_ZERODECIMAL_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedZerodecimalAlphaRoman
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
                "BULLET_GLYPH_PRESET_UNSPECIFIED" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletGlyphPresetUnspecified
                }
                "BULLET_LEFTTRIANGLE_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc
                }
                "BULLET_STAR_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare
                }
                "NUMBERED_DECIMAL_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDecimalAlphaRoman
                }
                "NUMBERED_DECIMAL_ALPHA_ROMAN_PARENS" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDecimalAlphaRomanParens
                }
                "NUMBERED_DECIMAL_NESTED" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDecimalNested
                }
                "NUMBERED_UPPERALPHA_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman
                }
                "NUMBERED_UPPERROMAN_UPPERALPHA_DECIMAL" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDecimal
                }
                "NUMBERED_ZERODECIMAL_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedZerodecimalAlphaRoman
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
    pub struct CropProperties {
        #[doc = "The clockwise rotation angle of the crop rectangle around its center, in radians. Rotation is applied after the offsets."]
        #[serde(
            rename = "angle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub angle: ::std::option::Option<f32>,
        #[doc = "The offset specifies how far inwards the bottom edge of the crop rectangle is from the bottom edge of the original content as a fraction of the original content's height."]
        #[serde(
            rename = "offsetBottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_bottom: ::std::option::Option<f32>,
        #[doc = "The offset specifies how far inwards the left edge of the crop rectangle is from the left edge of the original content as a fraction of the original content's width."]
        #[serde(
            rename = "offsetLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_left: ::std::option::Option<f32>,
        #[doc = "The offset specifies how far inwards the right edge of the crop rectangle is from the right edge of the original content as a fraction of the original content's width."]
        #[serde(
            rename = "offsetRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_right: ::std::option::Option<f32>,
        #[doc = "The offset specifies how far inwards the top edge of the crop rectangle is from the top edge of the original content as a fraction of the original content's height."]
        #[serde(
            rename = "offsetTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_top: ::std::option::Option<f32>,
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
    pub struct CropPropertiesSuggestionState {
        #[doc = "Indicates if there was a suggested change to angle."]
        #[serde(
            rename = "angleSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub angle_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to offset_bottom."]
        #[serde(
            rename = "offsetBottomSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_bottom_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to offset_left."]
        #[serde(
            rename = "offsetLeftSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_left_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to offset_right."]
        #[serde(
            rename = "offsetRightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_right_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to offset_top."]
        #[serde(
            rename = "offsetTopSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset_top_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CropPropertiesSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CropPropertiesSuggestionState {
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
    pub struct DeleteContentRangeRequest {
        #[doc = "The range of content to delete. Deleting text that crosses a paragraph boundary may result in changes to paragraph styles, lists, positioned objects and bookmarks as the two paragraphs are merged. Attempting to delete certain ranges can result in an invalid document structure in which case a 400 bad request error is returned. Some examples of invalid delete requests include: * Deleting one code unit of a surrogate pair. * Deleting the last newline character of a Body, Header, Footer, Footnote, TableCell or TableOfContents. * Deleting the start or end of a Table, TableOfContents or Equation without deleting the entire element. * Deleting the newline character before a Table, TableOfContents or SectionBreak without deleting the element. * Deleting individual rows or cells of a table. Deleting the content within a table cell is allowed."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::Range>,
    }
    impl ::google_field_selector::FieldSelector for DeleteContentRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteContentRangeRequest {
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
    pub struct DeleteFooterRequest {
        #[doc = "The id of the footer to delete. If this footer is defined on DocumentStyle, the reference to this footer is removed, resulting in no footer of that type for the first section of the document. If this footer is defined on a SectionStyle, the reference to this footer is removed and the footer of that type is now continued from the previous section."]
        #[serde(
            rename = "footerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footer_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteFooterRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteFooterRequest {
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
    pub struct DeleteHeaderRequest {
        #[doc = "The id of the header to delete. If this header is defined on DocumentStyle, the reference to this header is removed, resulting in no header of that type for the first section of the document. If this header is defined on a SectionStyle, the reference to this header is removed and the header of that type is now continued from the previous section."]
        #[serde(
            rename = "headerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteHeaderRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteHeaderRequest {
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
    pub struct DeleteNamedRangeRequest {
        #[doc = "The name of the range(s) to delete. All named ranges with the given name will be deleted."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID of the named range to delete."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteNamedRangeRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteNamedRangeRequest {
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
        #[doc = "The range to delete bullets from."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::Range>,
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
    pub struct DeletePositionedObjectRequest {
        #[doc = "The ID of the positioned object to delete."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeletePositionedObjectRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeletePositionedObjectRequest {
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
        #[doc = "The reference table cell location from which the column will be deleted. The column this cell spans will be deleted. If this is a merged cell that spans multiple columns, all columns that the cell spans will be deleted. If no columns remain in the table after this deletion, the whole table is deleted."]
        #[serde(
            rename = "tableCellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
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
        #[doc = "The reference table cell location from which the row will be deleted. The row this cell spans will be deleted. If this is a merged cell that spans multiple rows, all rows that the cell spans will be deleted. If no rows remain in the table after this deletion, the whole table is deleted."]
        #[serde(
            rename = "tableCellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
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
        #[doc = "A point, 1/72 of an inch."]
        Pt,
        #[doc = "The units are unknown."]
        UnitUnspecified,
    }
    impl DimensionUnit {
        pub fn as_str(self) -> &'static str {
            match self {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Document {
        #[doc = "Output only. The main body of the document."]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<crate::schemas::Body>,
        #[doc = "Output only. The ID of the document."]
        #[serde(
            rename = "documentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_id: ::std::option::Option<String>,
        #[doc = "Output only. The style of the document."]
        #[serde(
            rename = "documentStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_style: ::std::option::Option<crate::schemas::DocumentStyle>,
        #[doc = "Output only. The footers in the document, keyed by footer ID."]
        #[serde(
            rename = "footers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footers:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Footer>>,
        #[doc = "Output only. The footnotes in the document, keyed by footnote ID."]
        #[serde(
            rename = "footnotes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footnotes:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Footnote>>,
        #[doc = "Output only. The headers in the document, keyed by header ID."]
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Header>>,
        #[doc = "Output only. The inline objects in the document, keyed by object ID."]
        #[serde(
            rename = "inlineObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_objects: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::InlineObject>,
        >,
        #[doc = "Output only. The lists in the document, keyed by list ID."]
        #[serde(
            rename = "lists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lists:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::List>>,
        #[doc = "Output only. The named ranges in the document, keyed by name."]
        #[serde(
            rename = "namedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_ranges: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::NamedRanges>,
        >,
        #[doc = "Output only. The named styles of the document."]
        #[serde(
            rename = "namedStyles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_styles: ::std::option::Option<crate::schemas::NamedStyles>,
        #[doc = "Output only. The positioned objects in the document, keyed by object ID."]
        #[serde(
            rename = "positionedObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positioned_objects: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::PositionedObject>,
        >,
        #[doc = "Output only. The revision ID of the document. Can be used in update requests to specify which revision of a document to apply updates to and how the request should behave if the document has been edited since that revision. Only populated if the user has edit access to the document. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the document has not changed. Conversely, a changed ID (for the same document and user) usually means the document has been updated; however, a changed ID can also be due to internal factors such as ID format changes."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Output only. The suggested changes to the style of the document, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedDocumentStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_document_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedDocumentStyle>,
        >,
        #[doc = "Output only. The suggested changes to the named styles of the document, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedNamedStylesChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_named_styles_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedNamedStyles>,
        >,
        #[doc = "Output only. The suggestions view mode applied to the document. Note: When editing a document, changes must be based on a document with SUGGESTIONS_INLINE."]
        #[serde(
            rename = "suggestionsViewMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggestions_view_mode:
            ::std::option::Option<crate::schemas::DocumentSuggestionsViewMode>,
        #[doc = "The title of the document."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Document {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Document {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DocumentSuggestionsViewMode {
        #[doc = "The SuggestionsViewMode applied to the returned document depends on the user's current access level. If the user only has view access, PREVIEW_WITHOUT_SUGGESTIONS is applied. Otherwise, SUGGESTIONS_INLINE is applied. This is the default suggestions view mode."]
        DefaultForCurrentAccess,
        #[doc = "The returned document is a preview with all suggested changes accepted. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes."]
        PreviewSuggestionsAccepted,
        #[doc = "The returned document is a preview with all suggested changes rejected if there are any suggestions in the document."]
        PreviewWithoutSuggestions,
        #[doc = "The returned document has suggestions inline. Suggested changes will be differentiated from base content within the document. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes."]
        SuggestionsInline,
    }
    impl DocumentSuggestionsViewMode {
        pub fn as_str(self) -> &'static str {
            match self {
                DocumentSuggestionsViewMode::DefaultForCurrentAccess => {
                    "DEFAULT_FOR_CURRENT_ACCESS"
                }
                DocumentSuggestionsViewMode::PreviewSuggestionsAccepted => {
                    "PREVIEW_SUGGESTIONS_ACCEPTED"
                }
                DocumentSuggestionsViewMode::PreviewWithoutSuggestions => {
                    "PREVIEW_WITHOUT_SUGGESTIONS"
                }
                DocumentSuggestionsViewMode::SuggestionsInline => "SUGGESTIONS_INLINE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DocumentSuggestionsViewMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DocumentSuggestionsViewMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DocumentSuggestionsViewMode, ()> {
            Ok(match s {
                "DEFAULT_FOR_CURRENT_ACCESS" => {
                    DocumentSuggestionsViewMode::DefaultForCurrentAccess
                }
                "PREVIEW_SUGGESTIONS_ACCEPTED" => {
                    DocumentSuggestionsViewMode::PreviewSuggestionsAccepted
                }
                "PREVIEW_WITHOUT_SUGGESTIONS" => {
                    DocumentSuggestionsViewMode::PreviewWithoutSuggestions
                }
                "SUGGESTIONS_INLINE" => DocumentSuggestionsViewMode::SuggestionsInline,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DocumentSuggestionsViewMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DocumentSuggestionsViewMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DocumentSuggestionsViewMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT_FOR_CURRENT_ACCESS" => {
                    DocumentSuggestionsViewMode::DefaultForCurrentAccess
                }
                "PREVIEW_SUGGESTIONS_ACCEPTED" => {
                    DocumentSuggestionsViewMode::PreviewSuggestionsAccepted
                }
                "PREVIEW_WITHOUT_SUGGESTIONS" => {
                    DocumentSuggestionsViewMode::PreviewWithoutSuggestions
                }
                "SUGGESTIONS_INLINE" => DocumentSuggestionsViewMode::SuggestionsInline,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DocumentSuggestionsViewMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DocumentSuggestionsViewMode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DocumentStyle {
        #[doc = "The background of the document. Documents cannot have a transparent background color."]
        #[serde(
            rename = "background",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background: ::std::option::Option<crate::schemas::Background>,
        #[doc = "The ID of the default footer. If not set, there is no default footer. This property is read-only."]
        #[serde(
            rename = "defaultFooterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_footer_id: ::std::option::Option<String>,
        #[doc = "The ID of the default header. If not set, there is no default header. This property is read-only."]
        #[serde(
            rename = "defaultHeaderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_header_id: ::std::option::Option<String>,
        #[doc = "The ID of the footer used only for even pages. The value of use_even_page_header_footer determines whether to use the default_footer_id or this value for the footer on even pages. If not set, there is no even page footer. This property is read-only."]
        #[serde(
            rename = "evenPageFooterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub even_page_footer_id: ::std::option::Option<String>,
        #[doc = "The ID of the header used only for even pages. The value of use_even_page_header_footer determines whether to use the default_header_id or this value for the header on even pages. If not set, there is no even page header. This property is read-only."]
        #[serde(
            rename = "evenPageHeaderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub even_page_header_id: ::std::option::Option<String>,
        #[doc = "The ID of the footer used only for the first page. If not set then a unique footer for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_footer_id or this value for the footer on the first page. If not set, there is no first page footer. This property is read-only."]
        #[serde(
            rename = "firstPageFooterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_page_footer_id: ::std::option::Option<String>,
        #[doc = "The ID of the header used only for the first page. If not set then a unique header for the first page does not exist. The value of use_first_page_header_footer determines whether to use the default_header_id or this value for the header on the first page. If not set, there is no first page header. This property is read-only."]
        #[serde(
            rename = "firstPageHeaderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_page_header_id: ::std::option::Option<String>,
        #[doc = "The bottom page margin. Updating the bottom page margin on the document style clears the bottom page margin on all section styles."]
        #[serde(
            rename = "marginBottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_bottom: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of space between the bottom of the page and the contents of the footer."]
        #[serde(
            rename = "marginFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_footer: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of space between the top of the page and the contents of the header."]
        #[serde(
            rename = "marginHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_header: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The left page margin. Updating the left page margin on the document style clears the left page margin on all section styles. It may also cause columns to resize in all sections."]
        #[serde(
            rename = "marginLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_left: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The right page margin. Updating the right page margin on the document style clears the right page margin on all section styles. It may also cause columns to resize in all sections."]
        #[serde(
            rename = "marginRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_right: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The top page margin. Updating the top page margin on the document style clears the top page margin on all section styles."]
        #[serde(
            rename = "marginTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_top: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The page number from which to start counting the number of pages."]
        #[serde(
            rename = "pageNumberStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_number_start: ::std::option::Option<i32>,
        #[doc = "The size of a page in the document."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<crate::schemas::Size>,
        #[doc = "Indicates whether DocumentStyle margin_header, SectionStyle margin_header and DocumentStyle margin_footer, SectionStyle margin_footer are respected. When false, the default values in the Docs editor for header and footer margin are used. This property is read-only."]
        #[serde(
            rename = "useCustomHeaderFooterMargins",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_custom_header_footer_margins: ::std::option::Option<bool>,
        #[doc = "Indicates whether to use the even page header / footer IDs for the even pages."]
        #[serde(
            rename = "useEvenPageHeaderFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_even_page_header_footer: ::std::option::Option<bool>,
        #[doc = "Indicates whether to use the first page header / footer IDs for the first page."]
        #[serde(
            rename = "useFirstPageHeaderFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_first_page_header_footer: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DocumentStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DocumentStyle {
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
    pub struct DocumentStyleSuggestionState {
        #[doc = "A mask that indicates which of the fields in background have been changed in this suggestion."]
        #[serde(
            rename = "backgroundSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_suggestion_state:
            ::std::option::Option<crate::schemas::BackgroundSuggestionState>,
        #[doc = "Indicates if there was a suggested change to default_footer_id."]
        #[serde(
            rename = "defaultFooterIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_footer_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to default_header_id."]
        #[serde(
            rename = "defaultHeaderIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_header_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to even_page_footer_id."]
        #[serde(
            rename = "evenPageFooterIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub even_page_footer_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to even_page_header_id."]
        #[serde(
            rename = "evenPageHeaderIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub even_page_header_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to first_page_footer_id."]
        #[serde(
            rename = "firstPageFooterIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_page_footer_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to first_page_header_id."]
        #[serde(
            rename = "firstPageHeaderIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_page_header_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_bottom."]
        #[serde(
            rename = "marginBottomSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_bottom_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_footer."]
        #[serde(
            rename = "marginFooterSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_footer_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_header."]
        #[serde(
            rename = "marginHeaderSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_header_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_left."]
        #[serde(
            rename = "marginLeftSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_left_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_right."]
        #[serde(
            rename = "marginRightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_right_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_top."]
        #[serde(
            rename = "marginTopSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_top_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to page_number_start."]
        #[serde(
            rename = "pageNumberStartSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_number_start_suggested: ::std::option::Option<bool>,
        #[doc = "A mask that indicates which of the fields in size have been changed in this suggestion."]
        #[serde(
            rename = "pageSizeSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size_suggestion_state: ::std::option::Option<crate::schemas::SizeSuggestionState>,
        #[doc = "Indicates if there was a suggested change to use_custom_header_footer_margins."]
        #[serde(
            rename = "useCustomHeaderFooterMarginsSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_custom_header_footer_margins_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to use_even_page_header_footer."]
        #[serde(
            rename = "useEvenPageHeaderFooterSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_even_page_header_footer_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to use_first_page_header_footer."]
        #[serde(
            rename = "useFirstPageHeaderFooterSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_first_page_header_footer_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DocumentStyleSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DocumentStyleSuggestionState {
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
    pub struct EmbeddedDrawingProperties {}
    impl ::google_field_selector::FieldSelector for EmbeddedDrawingProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedDrawingProperties {
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
    pub struct EmbeddedDrawingPropertiesSuggestionState {}
    impl ::google_field_selector::FieldSelector for EmbeddedDrawingPropertiesSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedDrawingPropertiesSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EmbeddedObject {
        #[doc = "The description of the embedded object. The `title` and `description` are both combined to display alt text."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The properties of an embedded drawing."]
        #[serde(
            rename = "embeddedDrawingProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_drawing_properties:
            ::std::option::Option<crate::schemas::EmbeddedDrawingProperties>,
        #[doc = "The border of the embedded object."]
        #[serde(
            rename = "embeddedObjectBorder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_object_border: ::std::option::Option<crate::schemas::EmbeddedObjectBorder>,
        #[doc = "The properties of an image."]
        #[serde(
            rename = "imageProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_properties: ::std::option::Option<crate::schemas::ImageProperties>,
        #[doc = "A reference to the external linked source content. For example, it contains a reference to the source Sheets chart when the embedded object is a linked chart. If unset, then the embedded object is not linked."]
        #[serde(
            rename = "linkedContentReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linked_content_reference: ::std::option::Option<crate::schemas::LinkedContentReference>,
        #[doc = "The bottom margin of the embedded object."]
        #[serde(
            rename = "marginBottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_bottom: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The left margin of the embedded object."]
        #[serde(
            rename = "marginLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_left: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The right margin of the embedded object."]
        #[serde(
            rename = "marginRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_right: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The top margin of the embedded object."]
        #[serde(
            rename = "marginTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_top: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The visible size of the image after cropping."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<crate::schemas::Size>,
        #[doc = "The title of the embedded object. The `title` and `description` are both combined to display alt text."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EmbeddedObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedObject {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EmbeddedObjectBorder {
        #[doc = "The color of the border."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::OptionalColor>,
        #[doc = "The dash style of the border."]
        #[serde(
            rename = "dashStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dash_style: ::std::option::Option<crate::schemas::EmbeddedObjectBorderDashStyle>,
        #[doc = "The property state of the border property."]
        #[serde(
            rename = "propertyState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_state:
            ::std::option::Option<crate::schemas::EmbeddedObjectBorderPropertyState>,
        #[doc = "The width of the border."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for EmbeddedObjectBorder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedObjectBorder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EmbeddedObjectBorderDashStyle {
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
        Solid,
    }
    impl EmbeddedObjectBorderDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                EmbeddedObjectBorderDashStyle::Dash => "DASH",
                EmbeddedObjectBorderDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                EmbeddedObjectBorderDashStyle::Dot => "DOT",
                EmbeddedObjectBorderDashStyle::Solid => "SOLID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EmbeddedObjectBorderDashStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EmbeddedObjectBorderDashStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EmbeddedObjectBorderDashStyle, ()> {
            Ok(match s {
                "DASH" => EmbeddedObjectBorderDashStyle::Dash,
                "DASH_STYLE_UNSPECIFIED" => EmbeddedObjectBorderDashStyle::DashStyleUnspecified,
                "DOT" => EmbeddedObjectBorderDashStyle::Dot,
                "SOLID" => EmbeddedObjectBorderDashStyle::Solid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EmbeddedObjectBorderDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EmbeddedObjectBorderDashStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EmbeddedObjectBorderDashStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH" => EmbeddedObjectBorderDashStyle::Dash,
                "DASH_STYLE_UNSPECIFIED" => EmbeddedObjectBorderDashStyle::DashStyleUnspecified,
                "DOT" => EmbeddedObjectBorderDashStyle::Dot,
                "SOLID" => EmbeddedObjectBorderDashStyle::Solid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EmbeddedObjectBorderDashStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedObjectBorderDashStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EmbeddedObjectBorderPropertyState {
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the corresponding property when rendered in the document."]
        NotRendered,
        #[doc = "If a property's state is RENDERED, then the element has the corresponding property when rendered in the document. This is the default value."]
        Rendered,
    }
    impl EmbeddedObjectBorderPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                EmbeddedObjectBorderPropertyState::NotRendered => "NOT_RENDERED",
                EmbeddedObjectBorderPropertyState::Rendered => "RENDERED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EmbeddedObjectBorderPropertyState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EmbeddedObjectBorderPropertyState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EmbeddedObjectBorderPropertyState, ()> {
            Ok(match s {
                "NOT_RENDERED" => EmbeddedObjectBorderPropertyState::NotRendered,
                "RENDERED" => EmbeddedObjectBorderPropertyState::Rendered,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EmbeddedObjectBorderPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EmbeddedObjectBorderPropertyState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EmbeddedObjectBorderPropertyState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOT_RENDERED" => EmbeddedObjectBorderPropertyState::NotRendered,
                "RENDERED" => EmbeddedObjectBorderPropertyState::Rendered,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EmbeddedObjectBorderPropertyState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedObjectBorderPropertyState {
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
    pub struct EmbeddedObjectBorderSuggestionState {
        #[doc = "Indicates if there was a suggested change to color."]
        #[serde(
            rename = "colorSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to dash_style."]
        #[serde(
            rename = "dashStyleSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dash_style_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to property_state."]
        #[serde(
            rename = "propertyStateSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_state_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to width."]
        #[serde(
            rename = "widthSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for EmbeddedObjectBorderSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedObjectBorderSuggestionState {
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
    pub struct EmbeddedObjectSuggestionState {
        #[doc = "Indicates if there was a suggested change to description."]
        #[serde(
            rename = "descriptionSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description_suggested: ::std::option::Option<bool>,
        #[doc = "A mask that indicates which of the fields in embedded_drawing_properties have been changed in this suggestion."]
        #[serde(
            rename = "embeddedDrawingPropertiesSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_drawing_properties_suggestion_state:
            ::std::option::Option<crate::schemas::EmbeddedDrawingPropertiesSuggestionState>,
        #[doc = "A mask that indicates which of the fields in embedded_object_border have been changed in this suggestion."]
        #[serde(
            rename = "embeddedObjectBorderSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_object_border_suggestion_state:
            ::std::option::Option<crate::schemas::EmbeddedObjectBorderSuggestionState>,
        #[doc = "A mask that indicates which of the fields in image_properties have been changed in this suggestion."]
        #[serde(
            rename = "imagePropertiesSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_properties_suggestion_state:
            ::std::option::Option<crate::schemas::ImagePropertiesSuggestionState>,
        #[doc = "A mask that indicates which of the fields in linked_content_reference have been changed in this suggestion."]
        #[serde(
            rename = "linkedContentReferenceSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linked_content_reference_suggestion_state:
            ::std::option::Option<crate::schemas::LinkedContentReferenceSuggestionState>,
        #[doc = "Indicates if there was a suggested change to margin_bottom."]
        #[serde(
            rename = "marginBottomSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_bottom_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_left."]
        #[serde(
            rename = "marginLeftSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_left_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_right."]
        #[serde(
            rename = "marginRightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_right_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to margin_top."]
        #[serde(
            rename = "marginTopSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_top_suggested: ::std::option::Option<bool>,
        #[doc = "A mask that indicates which of the fields in size have been changed in this suggestion."]
        #[serde(
            rename = "sizeSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size_suggestion_state: ::std::option::Option<crate::schemas::SizeSuggestionState>,
        #[doc = "Indicates if there was a suggested change to title."]
        #[serde(
            rename = "titleSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for EmbeddedObjectSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmbeddedObjectSuggestionState {
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
    pub struct EndOfSegmentLocation {
        #[doc = "The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body."]
        #[serde(
            rename = "segmentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EndOfSegmentLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EndOfSegmentLocation {
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
    pub struct Equation {
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A Equation may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Equation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Equation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Footer {
        #[doc = "The contents of the footer. The indexes for a footer's content begin at zero."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<Vec<crate::schemas::StructuralElement>>,
        #[doc = "The ID of the footer."]
        #[serde(
            rename = "footerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footer_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Footer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Footer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Footnote {
        #[doc = "The contents of the footnote. The indexes for a footnote's content begin at zero."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<Vec<crate::schemas::StructuralElement>>,
        #[doc = "The ID of the footnote."]
        #[serde(
            rename = "footnoteId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footnote_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Footnote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Footnote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FootnoteReference {
        #[doc = "The ID of the footnote that contains the content of this footnote reference."]
        #[serde(
            rename = "footnoteId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footnote_id: ::std::option::Option<String>,
        #[doc = "The rendered number of this footnote."]
        #[serde(
            rename = "footnoteNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footnote_number: ::std::option::Option<String>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A FootnoteReference may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this FootnoteReference, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this FootnoteReference."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for FootnoteReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FootnoteReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Header {
        #[doc = "The contents of the header. The indexes for a header's content begin at zero."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<Vec<crate::schemas::StructuralElement>>,
        #[doc = "The ID of the header."]
        #[serde(
            rename = "headerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Header {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Header {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct HorizontalRule {
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A HorizontalRule may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this HorizontalRule, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this HorizontalRule. Similar to text content, like text runs and footnote references, the text style of a horizontal rule can affect content layout as well as the styling of text inserted adjacent to it."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for HorizontalRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HorizontalRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ImageProperties {
        #[doc = "The clockwise rotation angle of the image, in radians."]
        #[serde(
            rename = "angle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub angle: ::std::option::Option<f32>,
        #[doc = "The brightness effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect."]
        #[serde(
            rename = "brightness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brightness: ::std::option::Option<f32>,
        #[doc = "A URI to the image with a default lifetime of 30 minutes. This URI is tagged with the account of the requester. Anyone with the URI effectively accesses the image as the original requester. Access to the image may be lost if the document's sharing settings change."]
        #[serde(
            rename = "contentUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_uri: ::std::option::Option<String>,
        #[doc = "The contrast effect of the image. The value should be in the interval [-1.0, 1.0], where 0 means no effect."]
        #[serde(
            rename = "contrast",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contrast: ::std::option::Option<f32>,
        #[doc = "The crop properties of the image."]
        #[serde(
            rename = "cropProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crop_properties: ::std::option::Option<crate::schemas::CropProperties>,
        #[doc = "The source URI is the URI used to insert the image. The source URI can be empty."]
        #[serde(
            rename = "sourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_uri: ::std::option::Option<String>,
        #[doc = "The transparency effect of the image. The value should be in the interval [0.0, 1.0], where 0 means no effect and 1 means completely transparent."]
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
    pub struct ImagePropertiesSuggestionState {
        #[doc = "Indicates if there was a suggested change to angle."]
        #[serde(
            rename = "angleSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub angle_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to brightness."]
        #[serde(
            rename = "brightnessSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brightness_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to content_uri."]
        #[serde(
            rename = "contentUriSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_uri_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to contrast."]
        #[serde(
            rename = "contrastSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contrast_suggested: ::std::option::Option<bool>,
        #[doc = "A mask that indicates which of the fields in crop_properties have been changed in this suggestion."]
        #[serde(
            rename = "cropPropertiesSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crop_properties_suggestion_state:
            ::std::option::Option<crate::schemas::CropPropertiesSuggestionState>,
        #[doc = "Indicates if there was a suggested change to source_uri."]
        #[serde(
            rename = "sourceUriSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_uri_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to transparency."]
        #[serde(
            rename = "transparencySuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transparency_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ImagePropertiesSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImagePropertiesSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct InlineObject {
        #[doc = "The properties of this inline object."]
        #[serde(
            rename = "inlineObjectProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_object_properties: ::std::option::Option<crate::schemas::InlineObjectProperties>,
        #[doc = "The ID of this inline object."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested changes to the inline object properties, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedInlineObjectPropertiesChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_inline_object_properties_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedInlineObjectProperties>,
        >,
        #[doc = "The suggested insertion ID. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InlineObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InlineObject {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct InlineObjectElement {
        #[doc = "The ID of the InlineObject this element contains."]
        #[serde(
            rename = "inlineObjectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_object_id: ::std::option::Option<String>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. An InlineObjectElement may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this InlineObject, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this InlineObjectElement. Similar to text content, like text runs and footnote references, the text style of an inline object element can affect content layout as well as the styling of text inserted adjacent to it."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for InlineObjectElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InlineObjectElement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct InlineObjectProperties {
        #[doc = "The embedded object of this inline object."]
        #[serde(
            rename = "embeddedObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_object: ::std::option::Option<crate::schemas::EmbeddedObject>,
    }
    impl ::google_field_selector::FieldSelector for InlineObjectProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InlineObjectProperties {
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
    pub struct InlineObjectPropertiesSuggestionState {
        #[doc = "A mask that indicates which of the fields in embedded_object have been changed in this suggestion."]
        #[serde(
            rename = "embeddedObjectSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_object_suggestion_state:
            ::std::option::Option<crate::schemas::EmbeddedObjectSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for InlineObjectPropertiesSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InlineObjectPropertiesSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct InsertInlineImageRequest {
        #[doc = "Inserts the text at the end of a header, footer or the document body. Inline images cannot be inserted inside a footnote."]
        #[serde(
            rename = "endOfSegmentLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_of_segment_location: ::std::option::Option<crate::schemas::EndOfSegmentLocation>,
        #[doc = "Inserts the image at a specific index in the document. The image must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Inline images cannot be inserted inside a footnote or equation."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::Location>,
        #[doc = "The size that the image should appear as in the document. This property is optional and the final size of the image in the document is determined by the following rules: * If neither width nor height is specified, then a default size of the image is calculated based on its resolution. * If one dimension is specified then the other dimension is calculated to preserve the aspect ratio of the image. * If both width and height are specified, the image is scaled to fit within the provided dimensions while maintaining its aspect ratio."]
        #[serde(
            rename = "objectSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_size: ::std::option::Option<crate::schemas::Size>,
        #[doc = "The image URI. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URI can be at most 2 kB in length. The URI itself is saved with the image, and exposed via the ImageProperties.content_uri field."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InsertInlineImageRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertInlineImageRequest {
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
    pub struct InsertInlineImageResponse {
        #[doc = "The ID of the created InlineObject."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InsertInlineImageResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertInlineImageResponse {
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
    pub struct InsertInlineSheetsChartResponse {
        #[doc = "The object ID of the inserted chart."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for InsertInlineSheetsChartResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertInlineSheetsChartResponse {
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
    pub struct InsertPageBreakRequest {
        #[doc = "Inserts the page break at the end of the document body. Page breaks cannot be inserted inside a footnote, header or footer. Since page breaks can only be inserted inside the body, the segment ID field must be empty."]
        #[serde(
            rename = "endOfSegmentLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_of_segment_location: ::std::option::Option<crate::schemas::EndOfSegmentLocation>,
        #[doc = "Inserts the page break at a specific index in the document. The page break must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Page breaks cannot be inserted inside a table, equation, footnote, header or footer. Since page breaks can only be inserted inside the body, the segment ID field must be empty."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::google_field_selector::FieldSelector for InsertPageBreakRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertPageBreakRequest {
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
    pub struct InsertSectionBreakRequest {
        #[doc = "Inserts a newline and a section break at the end of the document body. Section breaks cannot be inserted inside a footnote, header or footer. Because section breaks can only be inserted inside the body, the segment ID field must be empty."]
        #[serde(
            rename = "endOfSegmentLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_of_segment_location: ::std::option::Option<crate::schemas::EndOfSegmentLocation>,
        #[doc = "Inserts a newline and a section break at a specific index in the document. The section break must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). Section breaks cannot be inserted inside a table, equation, footnote, header, or footer. Since section breaks can only be inserted inside the body, the segment ID field must be empty."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::Location>,
        #[doc = "The type of section to insert."]
        #[serde(
            rename = "sectionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_type:
            ::std::option::Option<crate::schemas::InsertSectionBreakRequestSectionType>,
    }
    impl ::google_field_selector::FieldSelector for InsertSectionBreakRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertSectionBreakRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InsertSectionBreakRequestSectionType {
        #[doc = "The section starts immediately after the last paragraph of the previous section."]
        Continuous,
        #[doc = "The section starts on the next page."]
        NextPage,
        #[doc = "The section type is unspecified."]
        SectionTypeUnspecified,
    }
    impl InsertSectionBreakRequestSectionType {
        pub fn as_str(self) -> &'static str {
            match self {
                InsertSectionBreakRequestSectionType::Continuous => "CONTINUOUS",
                InsertSectionBreakRequestSectionType::NextPage => "NEXT_PAGE",
                InsertSectionBreakRequestSectionType::SectionTypeUnspecified => {
                    "SECTION_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for InsertSectionBreakRequestSectionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InsertSectionBreakRequestSectionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InsertSectionBreakRequestSectionType, ()> {
            Ok(match s {
                "CONTINUOUS" => InsertSectionBreakRequestSectionType::Continuous,
                "NEXT_PAGE" => InsertSectionBreakRequestSectionType::NextPage,
                "SECTION_TYPE_UNSPECIFIED" => {
                    InsertSectionBreakRequestSectionType::SectionTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InsertSectionBreakRequestSectionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InsertSectionBreakRequestSectionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InsertSectionBreakRequestSectionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTINUOUS" => InsertSectionBreakRequestSectionType::Continuous,
                "NEXT_PAGE" => InsertSectionBreakRequestSectionType::NextPage,
                "SECTION_TYPE_UNSPECIFIED" => {
                    InsertSectionBreakRequestSectionType::SectionTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for InsertSectionBreakRequestSectionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertSectionBreakRequestSectionType {
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
    pub struct InsertTableColumnRequest {
        #[doc = "Whether to insert new column to the right of the reference cell location. - `True`: insert to the right. - `False`: insert to the left."]
        #[serde(
            rename = "insertRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_right: ::std::option::Option<bool>,
        #[doc = "The reference table cell location from which columns will be inserted. A new column will be inserted to the left (or right) of the column where the reference cell is. If the reference cell is a merged cell, a new column will be inserted to the left (or right) of the merged cell."]
        #[serde(
            rename = "tableCellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
    }
    impl ::google_field_selector::FieldSelector for InsertTableColumnRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertTableColumnRequest {
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
    pub struct InsertTableRequest {
        #[doc = "The number of columns in the table."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<i32>,
        #[doc = "Inserts the table at the end of the given header, footer or document body. A newline character will be inserted before the inserted table. Tables cannot be inserted inside a footnote."]
        #[serde(
            rename = "endOfSegmentLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_of_segment_location: ::std::option::Option<crate::schemas::EndOfSegmentLocation>,
        #[doc = "Inserts the table at a specific model index. A newline character will be inserted before the inserted table, therefore the table start index will be at the specified location index + 1. The table must be inserted inside the bounds of an existing Paragraph. For instance, it cannot be inserted at a table's start index (i.e. between an existing table and its preceding paragraph). Tables cannot be inserted inside a footnote or equation."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::Location>,
        #[doc = "The number of rows in the table."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for InsertTableRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertTableRequest {
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
    pub struct InsertTableRowRequest {
        #[doc = "Whether to insert new row below the reference cell location. - `True`: insert below the cell. - `False`: insert above the cell."]
        #[serde(
            rename = "insertBelow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_below: ::std::option::Option<bool>,
        #[doc = "The reference table cell location from which rows will be inserted. A new row will be inserted above (or below) the row where the reference cell is. If the reference cell is a merged cell, a new row will be inserted above (or below) the merged cell."]
        #[serde(
            rename = "tableCellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
    }
    impl ::google_field_selector::FieldSelector for InsertTableRowRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsertTableRowRequest {
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
        #[doc = "Inserts the text at the end of a header, footer, footnote or the document body."]
        #[serde(
            rename = "endOfSegmentLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_of_segment_location: ::std::option::Option<crate::schemas::EndOfSegmentLocation>,
        #[doc = "Inserts the text at a specific index in the document. Text must be inserted inside the bounds of an existing Paragraph. For instance, text cannot be inserted at a table's start index (i.e. between the table and its preceding paragraph). The text must be inserted in the preceding paragraph."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<crate::schemas::Location>,
        #[doc = "The text to be inserted. Inserting a newline character will implicitly create a new Paragraph at that index. The paragraph style of the new paragraph will be copied from the paragraph at the current insertion index, including lists and bullets. Text styles for inserted text will be determined automatically, generally preserving the styling of neighboring text. In most cases, the text style for the inserted text will match the text immediately before the insertion index. Some control characters (U+0000-U+0008, U+000C-U+001F) and characters from the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF) will be stripped out of the inserted text."]
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
    pub struct Link {
        #[doc = "The ID of a bookmark in this document."]
        #[serde(
            rename = "bookmarkId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bookmark_id: ::std::option::Option<String>,
        #[doc = "The ID of a heading in this document."]
        #[serde(
            rename = "headingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub heading_id: ::std::option::Option<String>,
        #[doc = "An external URL."]
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LinkedContentReference {
        #[doc = "A reference to the linked chart."]
        #[serde(
            rename = "sheetsChartReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheets_chart_reference: ::std::option::Option<crate::schemas::SheetsChartReference>,
    }
    impl ::google_field_selector::FieldSelector for LinkedContentReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LinkedContentReference {
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
    pub struct LinkedContentReferenceSuggestionState {
        #[doc = "A mask that indicates which of the fields in sheets_chart_reference have been changed in this suggestion."]
        #[serde(
            rename = "sheetsChartReferenceSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sheets_chart_reference_suggestion_state:
            ::std::option::Option<crate::schemas::SheetsChartReferenceSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for LinkedContentReferenceSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LinkedContentReferenceSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct List {
        #[doc = "The properties of the list."]
        #[serde(
            rename = "listProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_properties: ::std::option::Option<crate::schemas::ListProperties>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this list."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion ID. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_id: ::std::option::Option<String>,
        #[doc = "The suggested changes to the list properties, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedListPropertiesChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_list_properties_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedListProperties>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListProperties {
        #[doc = "Describes the properties of the bullets at the associated level. A list has at most nine levels of nesting with nesting level 0 corresponding to the top-most level and nesting level 8 corresponding to the most nested level. The nesting levels are returned in ascending order with the least nested returned first."]
        #[serde(
            rename = "nestingLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nesting_levels: ::std::option::Option<Vec<crate::schemas::NestingLevel>>,
    }
    impl ::google_field_selector::FieldSelector for ListProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListProperties {
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
    pub struct ListPropertiesSuggestionState {
        #[doc = "A mask that indicates which of the fields on the corresponding NestingLevel in nesting_levels have been changed in this suggestion. The nesting level suggestion states are returned in ascending order of the nesting level with the least nested returned first."]
        #[serde(
            rename = "nestingLevelsSuggestionStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nesting_levels_suggestion_states:
            ::std::option::Option<Vec<crate::schemas::NestingLevelSuggestionState>>,
    }
    impl ::google_field_selector::FieldSelector for ListPropertiesSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPropertiesSuggestionState {
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
    pub struct Location {
        #[doc = "The zero-based index, in UTF-16 code units. The index is relative to the beginning of the segment specified by segment_id."]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
        #[doc = "The ID of the header, footer or footnote the location is in. An empty segment ID signifies the document's body."]
        #[serde(
            rename = "segmentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Location {
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
        #[doc = "The table range specifying which cells of the table to merge. Any text in the cells being merged will be concatenated and stored in the \"head\" cell of the range. This is the upper-left cell of the range when the content direction is left to right, and the upper-right cell of the range otherwise. If the range is non-rectangular (which can occur in some cases where the range covers cells that are already merged or where the table is non-rectangular), a 400 bad request error is returned."]
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NamedRange {
        #[doc = "The name of the named range."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID of the named range."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
        #[doc = "The ranges that belong to this named range."]
        #[serde(
            rename = "ranges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ranges: ::std::option::Option<Vec<crate::schemas::Range>>,
    }
    impl ::google_field_selector::FieldSelector for NamedRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedRange {
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
    pub struct NamedRanges {
        #[doc = "The name that all the named ranges share."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The NamedRanges that share the same name."]
        #[serde(
            rename = "namedRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_ranges: ::std::option::Option<Vec<crate::schemas::NamedRange>>,
    }
    impl ::google_field_selector::FieldSelector for NamedRanges {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedRanges {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NamedStyle {
        #[doc = "The type of this named style."]
        #[serde(
            rename = "namedStyleType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_style_type: ::std::option::Option<crate::schemas::NamedStyleNamedStyleType>,
        #[doc = "The paragraph style of this named style."]
        #[serde(
            rename = "paragraphStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph_style: ::std::option::Option<crate::schemas::ParagraphStyle>,
        #[doc = "The text style of this named style."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for NamedStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NamedStyleNamedStyleType {
        #[doc = "Heading 1."]
        Heading1,
        #[doc = "Heading 2."]
        Heading2,
        #[doc = "Heading 3."]
        Heading3,
        #[doc = "Heading 4."]
        Heading4,
        #[doc = "Heading 5."]
        Heading5,
        #[doc = "Heading 6."]
        Heading6,
        #[doc = "The type of named style is unspecified."]
        NamedStyleTypeUnspecified,
        #[doc = "Normal text."]
        NormalText,
        #[doc = "Subtitle."]
        Subtitle,
        #[doc = "Title."]
        Title,
    }
    impl NamedStyleNamedStyleType {
        pub fn as_str(self) -> &'static str {
            match self {
                NamedStyleNamedStyleType::Heading1 => "HEADING_1",
                NamedStyleNamedStyleType::Heading2 => "HEADING_2",
                NamedStyleNamedStyleType::Heading3 => "HEADING_3",
                NamedStyleNamedStyleType::Heading4 => "HEADING_4",
                NamedStyleNamedStyleType::Heading5 => "HEADING_5",
                NamedStyleNamedStyleType::Heading6 => "HEADING_6",
                NamedStyleNamedStyleType::NamedStyleTypeUnspecified => {
                    "NAMED_STYLE_TYPE_UNSPECIFIED"
                }
                NamedStyleNamedStyleType::NormalText => "NORMAL_TEXT",
                NamedStyleNamedStyleType::Subtitle => "SUBTITLE",
                NamedStyleNamedStyleType::Title => "TITLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NamedStyleNamedStyleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NamedStyleNamedStyleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NamedStyleNamedStyleType, ()> {
            Ok(match s {
                "HEADING_1" => NamedStyleNamedStyleType::Heading1,
                "HEADING_2" => NamedStyleNamedStyleType::Heading2,
                "HEADING_3" => NamedStyleNamedStyleType::Heading3,
                "HEADING_4" => NamedStyleNamedStyleType::Heading4,
                "HEADING_5" => NamedStyleNamedStyleType::Heading5,
                "HEADING_6" => NamedStyleNamedStyleType::Heading6,
                "NAMED_STYLE_TYPE_UNSPECIFIED" => {
                    NamedStyleNamedStyleType::NamedStyleTypeUnspecified
                }
                "NORMAL_TEXT" => NamedStyleNamedStyleType::NormalText,
                "SUBTITLE" => NamedStyleNamedStyleType::Subtitle,
                "TITLE" => NamedStyleNamedStyleType::Title,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NamedStyleNamedStyleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NamedStyleNamedStyleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NamedStyleNamedStyleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HEADING_1" => NamedStyleNamedStyleType::Heading1,
                "HEADING_2" => NamedStyleNamedStyleType::Heading2,
                "HEADING_3" => NamedStyleNamedStyleType::Heading3,
                "HEADING_4" => NamedStyleNamedStyleType::Heading4,
                "HEADING_5" => NamedStyleNamedStyleType::Heading5,
                "HEADING_6" => NamedStyleNamedStyleType::Heading6,
                "NAMED_STYLE_TYPE_UNSPECIFIED" => {
                    NamedStyleNamedStyleType::NamedStyleTypeUnspecified
                }
                "NORMAL_TEXT" => NamedStyleNamedStyleType::NormalText,
                "SUBTITLE" => NamedStyleNamedStyleType::Subtitle,
                "TITLE" => NamedStyleNamedStyleType::Title,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NamedStyleNamedStyleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedStyleNamedStyleType {
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
    pub struct NamedStyleSuggestionState {
        #[doc = "The named style type that this suggestion state corresponds to. This field is provided as a convenience for matching the NamedStyleSuggestionState with its corresponding NamedStyle."]
        #[serde(
            rename = "namedStyleType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_style_type:
            ::std::option::Option<crate::schemas::NamedStyleSuggestionStateNamedStyleType>,
        #[doc = "A mask that indicates which of the fields in paragraph style have been changed in this suggestion."]
        #[serde(
            rename = "paragraphStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph_style_suggestion_state:
            ::std::option::Option<crate::schemas::ParagraphStyleSuggestionState>,
        #[doc = "A mask that indicates which of the fields in text style have been changed in this suggestion."]
        #[serde(
            rename = "textStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style_suggestion_state:
            ::std::option::Option<crate::schemas::TextStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for NamedStyleSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedStyleSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NamedStyleSuggestionStateNamedStyleType {
        #[doc = "Heading 1."]
        Heading1,
        #[doc = "Heading 2."]
        Heading2,
        #[doc = "Heading 3."]
        Heading3,
        #[doc = "Heading 4."]
        Heading4,
        #[doc = "Heading 5."]
        Heading5,
        #[doc = "Heading 6."]
        Heading6,
        #[doc = "The type of named style is unspecified."]
        NamedStyleTypeUnspecified,
        #[doc = "Normal text."]
        NormalText,
        #[doc = "Subtitle."]
        Subtitle,
        #[doc = "Title."]
        Title,
    }
    impl NamedStyleSuggestionStateNamedStyleType {
        pub fn as_str(self) -> &'static str {
            match self {
                NamedStyleSuggestionStateNamedStyleType::Heading1 => "HEADING_1",
                NamedStyleSuggestionStateNamedStyleType::Heading2 => "HEADING_2",
                NamedStyleSuggestionStateNamedStyleType::Heading3 => "HEADING_3",
                NamedStyleSuggestionStateNamedStyleType::Heading4 => "HEADING_4",
                NamedStyleSuggestionStateNamedStyleType::Heading5 => "HEADING_5",
                NamedStyleSuggestionStateNamedStyleType::Heading6 => "HEADING_6",
                NamedStyleSuggestionStateNamedStyleType::NamedStyleTypeUnspecified => {
                    "NAMED_STYLE_TYPE_UNSPECIFIED"
                }
                NamedStyleSuggestionStateNamedStyleType::NormalText => "NORMAL_TEXT",
                NamedStyleSuggestionStateNamedStyleType::Subtitle => "SUBTITLE",
                NamedStyleSuggestionStateNamedStyleType::Title => "TITLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NamedStyleSuggestionStateNamedStyleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NamedStyleSuggestionStateNamedStyleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NamedStyleSuggestionStateNamedStyleType, ()> {
            Ok(match s {
                "HEADING_1" => NamedStyleSuggestionStateNamedStyleType::Heading1,
                "HEADING_2" => NamedStyleSuggestionStateNamedStyleType::Heading2,
                "HEADING_3" => NamedStyleSuggestionStateNamedStyleType::Heading3,
                "HEADING_4" => NamedStyleSuggestionStateNamedStyleType::Heading4,
                "HEADING_5" => NamedStyleSuggestionStateNamedStyleType::Heading5,
                "HEADING_6" => NamedStyleSuggestionStateNamedStyleType::Heading6,
                "NAMED_STYLE_TYPE_UNSPECIFIED" => {
                    NamedStyleSuggestionStateNamedStyleType::NamedStyleTypeUnspecified
                }
                "NORMAL_TEXT" => NamedStyleSuggestionStateNamedStyleType::NormalText,
                "SUBTITLE" => NamedStyleSuggestionStateNamedStyleType::Subtitle,
                "TITLE" => NamedStyleSuggestionStateNamedStyleType::Title,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NamedStyleSuggestionStateNamedStyleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NamedStyleSuggestionStateNamedStyleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NamedStyleSuggestionStateNamedStyleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HEADING_1" => NamedStyleSuggestionStateNamedStyleType::Heading1,
                "HEADING_2" => NamedStyleSuggestionStateNamedStyleType::Heading2,
                "HEADING_3" => NamedStyleSuggestionStateNamedStyleType::Heading3,
                "HEADING_4" => NamedStyleSuggestionStateNamedStyleType::Heading4,
                "HEADING_5" => NamedStyleSuggestionStateNamedStyleType::Heading5,
                "HEADING_6" => NamedStyleSuggestionStateNamedStyleType::Heading6,
                "NAMED_STYLE_TYPE_UNSPECIFIED" => {
                    NamedStyleSuggestionStateNamedStyleType::NamedStyleTypeUnspecified
                }
                "NORMAL_TEXT" => NamedStyleSuggestionStateNamedStyleType::NormalText,
                "SUBTITLE" => NamedStyleSuggestionStateNamedStyleType::Subtitle,
                "TITLE" => NamedStyleSuggestionStateNamedStyleType::Title,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NamedStyleSuggestionStateNamedStyleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedStyleSuggestionStateNamedStyleType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NamedStyles {
        #[doc = "The named styles. There is an entry for each of the possible named style types."]
        #[serde(
            rename = "styles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub styles: ::std::option::Option<Vec<crate::schemas::NamedStyle>>,
    }
    impl ::google_field_selector::FieldSelector for NamedStyles {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedStyles {
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
    pub struct NamedStylesSuggestionState {
        #[doc = "A mask that indicates which of the fields on the corresponding NamedStyle in styles have been changed in this suggestion. The order of these named style suggestion states match the order of the corresponding named style within the named styles suggestion."]
        #[serde(
            rename = "stylesSuggestionStates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub styles_suggestion_states:
            ::std::option::Option<Vec<crate::schemas::NamedStyleSuggestionState>>,
    }
    impl ::google_field_selector::FieldSelector for NamedStylesSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedStylesSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NestingLevel {
        #[doc = "The alignment of the bullet within the space allotted for rendering the bullet."]
        #[serde(
            rename = "bulletAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet_alignment: ::std::option::Option<crate::schemas::NestingLevelBulletAlignment>,
        #[doc = "The format string used by bullets at this level of nesting. The glyph format contains one or more placeholders, and these placeholder are replaced with the appropriate values depending on the glyph_type or glyph_symbol. The placeholders follow the pattern `%[nesting_level]`. Furthermore, placeholders can have prefixes and suffixes. Thus, the glyph format follows the pattern `%[nesting_level]`. Note that the prefix and suffix are optional and can be arbitrary strings. For example, the glyph format `%0.` indicates that the rendered glyph will replace the placeholder with the corresponding glyph for nesting level 0 followed by a period as the suffix. So a list with a glyph type of UPPER_ALPHA and glyph format `%0.` at nesting level 0 will result in a list with rendered glyphs `A.` `B.` `C.` The glyph format can contain placeholders for the current nesting level as well as placeholders for parent nesting levels. For example, a list can have a glyph format of `%0.` at nesting level 0 and a glyph format of `%0.%1.` at nesting level 1. Assuming both nesting levels have DECIMAL glyph types, this would result in a list with rendered glyphs `1.` `2.` ` 2.1.` ` 2.2.` `3.` For nesting levels that are ordered, the string that replaces a placeholder in the glyph format for a particular paragraph depends on the paragraph's order within the list."]
        #[serde(
            rename = "glyphFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub glyph_format: ::std::option::Option<String>,
        #[doc = "A custom glyph symbol used by bullets when paragraphs at this level of nesting are unordered. The glyph symbol replaces placeholders within the glyph_format. For example, if the glyph_symbol is the solid circle corresponding to Unicode U+25cf code point and the glyph_format is `%0`, the rendered glyph would be the solid circle."]
        #[serde(
            rename = "glyphSymbol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub glyph_symbol: ::std::option::Option<String>,
        #[doc = "The type of glyph used by bullets when paragraphs at this level of nesting are ordered. The glyph type determines the type of glyph used to replace placeholders within the glyph_format when paragraphs at this level of nesting are ordered. For example, if the nesting level is 0, the glyph_format is `%0.` and the glyph type is DECIMAL, then the rendered glyph would replace the placeholder `%0` in the glyph format with a number corresponding to list item's order within the list."]
        #[serde(
            rename = "glyphType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub glyph_type: ::std::option::Option<crate::schemas::NestingLevelGlyphType>,
        #[doc = "The amount of indentation for the first line of paragraphs at this level of nesting."]
        #[serde(
            rename = "indentFirstLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_first_line: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of indentation for paragraphs at this level of nesting. Applied to the side that corresponds to the start of the text, based on the paragraph's content direction."]
        #[serde(
            rename = "indentStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_start: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The number of the first list item at this nesting level. A value of 0 is treated as a value of 1 for lettered lists and roman numeraled lists, i.e. for values of both 0 and 1, lettered and roman numeraled lists will begin at `a` and `i` respectively. This value is ignored for nesting levels with unordered glyphs."]
        #[serde(
            rename = "startNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_number: ::std::option::Option<i32>,
        #[doc = "The text style of bullets at this level of nesting."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NestingLevelBulletAlignment {
        #[doc = "The bullet alignment is unspecified."]
        BulletAlignmentUnspecified,
        #[doc = "The bullet is aligned to the center of the space allotted for rendering the bullet."]
        Center,
        #[doc = "The bullet is aligned to the end of the space allotted for rendering the bullet. Right-aligned for LTR text, left-aligned otherwise."]
        End,
        #[doc = "The bullet is aligned to the start of the space allotted for rendering the bullet. Left-aligned for LTR text, right-aligned otherwise."]
        Start,
    }
    impl NestingLevelBulletAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                NestingLevelBulletAlignment::BulletAlignmentUnspecified => {
                    "BULLET_ALIGNMENT_UNSPECIFIED"
                }
                NestingLevelBulletAlignment::Center => "CENTER",
                NestingLevelBulletAlignment::End => "END",
                NestingLevelBulletAlignment::Start => "START",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NestingLevelBulletAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NestingLevelBulletAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NestingLevelBulletAlignment, ()> {
            Ok(match s {
                "BULLET_ALIGNMENT_UNSPECIFIED" => {
                    NestingLevelBulletAlignment::BulletAlignmentUnspecified
                }
                "CENTER" => NestingLevelBulletAlignment::Center,
                "END" => NestingLevelBulletAlignment::End,
                "START" => NestingLevelBulletAlignment::Start,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NestingLevelBulletAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NestingLevelBulletAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NestingLevelBulletAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BULLET_ALIGNMENT_UNSPECIFIED" => {
                    NestingLevelBulletAlignment::BulletAlignmentUnspecified
                }
                "CENTER" => NestingLevelBulletAlignment::Center,
                "END" => NestingLevelBulletAlignment::End,
                "START" => NestingLevelBulletAlignment::Start,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NestingLevelBulletAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NestingLevelBulletAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NestingLevelGlyphType {
        #[doc = "A lowercase letter, like `a`, `b`, or `c`."]
        Alpha,
        #[doc = "A number, like `1`, `2`, or `3`."]
        Decimal,
        #[doc = "The glyph type is unspecified or unsupported."]
        GlyphTypeUnspecified,
        #[doc = "An empty string."]
        None,
        #[doc = "A lowercase Roman numeral, like `i`, `ii`, or `iii`."]
        Roman,
        #[doc = "An uppercase letter, like `A`, `B`, or `C`."]
        UpperAlpha,
        #[doc = "An uppercase Roman numeral, like `I`, `II`, or `III`."]
        UpperRoman,
        #[doc = "A number where single digit numbers are prefixed with a zero, like `01`, `02`, or `03`. Numbers with more than one digit are not prefixed with a zero."]
        ZeroDecimal,
    }
    impl NestingLevelGlyphType {
        pub fn as_str(self) -> &'static str {
            match self {
                NestingLevelGlyphType::Alpha => "ALPHA",
                NestingLevelGlyphType::Decimal => "DECIMAL",
                NestingLevelGlyphType::GlyphTypeUnspecified => "GLYPH_TYPE_UNSPECIFIED",
                NestingLevelGlyphType::None => "NONE",
                NestingLevelGlyphType::Roman => "ROMAN",
                NestingLevelGlyphType::UpperAlpha => "UPPER_ALPHA",
                NestingLevelGlyphType::UpperRoman => "UPPER_ROMAN",
                NestingLevelGlyphType::ZeroDecimal => "ZERO_DECIMAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NestingLevelGlyphType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NestingLevelGlyphType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NestingLevelGlyphType, ()> {
            Ok(match s {
                "ALPHA" => NestingLevelGlyphType::Alpha,
                "DECIMAL" => NestingLevelGlyphType::Decimal,
                "GLYPH_TYPE_UNSPECIFIED" => NestingLevelGlyphType::GlyphTypeUnspecified,
                "NONE" => NestingLevelGlyphType::None,
                "ROMAN" => NestingLevelGlyphType::Roman,
                "UPPER_ALPHA" => NestingLevelGlyphType::UpperAlpha,
                "UPPER_ROMAN" => NestingLevelGlyphType::UpperRoman,
                "ZERO_DECIMAL" => NestingLevelGlyphType::ZeroDecimal,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NestingLevelGlyphType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NestingLevelGlyphType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NestingLevelGlyphType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHA" => NestingLevelGlyphType::Alpha,
                "DECIMAL" => NestingLevelGlyphType::Decimal,
                "GLYPH_TYPE_UNSPECIFIED" => NestingLevelGlyphType::GlyphTypeUnspecified,
                "NONE" => NestingLevelGlyphType::None,
                "ROMAN" => NestingLevelGlyphType::Roman,
                "UPPER_ALPHA" => NestingLevelGlyphType::UpperAlpha,
                "UPPER_ROMAN" => NestingLevelGlyphType::UpperRoman,
                "ZERO_DECIMAL" => NestingLevelGlyphType::ZeroDecimal,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NestingLevelGlyphType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NestingLevelGlyphType {
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
    pub struct NestingLevelSuggestionState {
        #[doc = "Indicates if there was a suggested change to bullet_alignment."]
        #[serde(
            rename = "bulletAlignmentSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet_alignment_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to glyph_format."]
        #[serde(
            rename = "glyphFormatSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub glyph_format_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to glyph_symbol."]
        #[serde(
            rename = "glyphSymbolSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub glyph_symbol_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to glyph_type."]
        #[serde(
            rename = "glyphTypeSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub glyph_type_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to indent_first_line."]
        #[serde(
            rename = "indentFirstLineSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_first_line_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to indent_start."]
        #[serde(
            rename = "indentStartSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_start_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to start_number."]
        #[serde(
            rename = "startNumberSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_number_suggested: ::std::option::Option<bool>,
        #[doc = "A mask that indicates which of the fields in text style have been changed in this suggestion."]
        #[serde(
            rename = "textStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style_suggestion_state:
            ::std::option::Option<crate::schemas::TextStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for NestingLevelSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NestingLevelSuggestionState {
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
    pub struct ObjectReferences {
        #[doc = "The object IDs."]
        #[serde(
            rename = "objectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ObjectReferences {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectReferences {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OptionalColor {
        #[doc = "If set, this will be used as an opaque color. If unset, this represents a transparent color."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::Color>,
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
    pub struct PageBreak {
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A PageBreak may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this PageBreak, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this PageBreak. Similar to text content, like text runs and footnote references, the text style of a page break can affect content layout as well as the styling of text inserted adjacent to it."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for PageBreak {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PageBreak {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Paragraph {
        #[doc = "The bullet for this paragraph. If not present, the paragraph does not belong to a list."]
        #[serde(
            rename = "bullet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet: ::std::option::Option<crate::schemas::Bullet>,
        #[doc = "The content of the paragraph broken down into its component parts."]
        #[serde(
            rename = "elements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub elements: ::std::option::Option<Vec<crate::schemas::ParagraphElement>>,
        #[doc = "The style of this paragraph."]
        #[serde(
            rename = "paragraphStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph_style: ::std::option::Option<crate::schemas::ParagraphStyle>,
        #[doc = "The IDs of the positioned objects tethered to this paragraph."]
        #[serde(
            rename = "positionedObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positioned_object_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested changes to this paragraph's bullet."]
        #[serde(
            rename = "suggestedBulletChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_bullet_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedBullet>,
        >,
        #[doc = "The suggested paragraph style changes to this paragraph, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedParagraphStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_paragraph_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedParagraphStyle>,
        >,
        #[doc = "The IDs of the positioned objects that are suggested to be attached to this paragraph, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedPositionedObjectIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_positioned_object_ids: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::ObjectReferences>,
        >,
    }
    impl ::google_field_selector::FieldSelector for Paragraph {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Paragraph {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ParagraphBorder {
        #[doc = "The color of the border."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::OptionalColor>,
        #[doc = "The dash style of the border."]
        #[serde(
            rename = "dashStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dash_style: ::std::option::Option<crate::schemas::ParagraphBorderDashStyle>,
        #[doc = "The padding of the border."]
        #[serde(
            rename = "padding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The width of the border."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for ParagraphBorder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphBorder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParagraphBorderDashStyle {
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
        Solid,
    }
    impl ParagraphBorderDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphBorderDashStyle::Dash => "DASH",
                ParagraphBorderDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                ParagraphBorderDashStyle::Dot => "DOT",
                ParagraphBorderDashStyle::Solid => "SOLID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ParagraphBorderDashStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParagraphBorderDashStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParagraphBorderDashStyle, ()> {
            Ok(match s {
                "DASH" => ParagraphBorderDashStyle::Dash,
                "DASH_STYLE_UNSPECIFIED" => ParagraphBorderDashStyle::DashStyleUnspecified,
                "DOT" => ParagraphBorderDashStyle::Dot,
                "SOLID" => ParagraphBorderDashStyle::Solid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ParagraphBorderDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphBorderDashStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphBorderDashStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH" => ParagraphBorderDashStyle::Dash,
                "DASH_STYLE_UNSPECIFIED" => ParagraphBorderDashStyle::DashStyleUnspecified,
                "DOT" => ParagraphBorderDashStyle::Dot,
                "SOLID" => ParagraphBorderDashStyle::Solid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ParagraphBorderDashStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphBorderDashStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ParagraphElement {
        #[doc = "An auto text paragraph element."]
        #[serde(
            rename = "autoText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_text: ::std::option::Option<crate::schemas::AutoText>,
        #[doc = "A column break paragraph element."]
        #[serde(
            rename = "columnBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_break: ::std::option::Option<crate::schemas::ColumnBreak>,
        #[doc = "The zero-base end index of this paragraph element, exclusive, in UTF-16 code units."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "An equation paragraph element."]
        #[serde(
            rename = "equation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub equation: ::std::option::Option<crate::schemas::Equation>,
        #[doc = "A footnote reference paragraph element."]
        #[serde(
            rename = "footnoteReference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footnote_reference: ::std::option::Option<crate::schemas::FootnoteReference>,
        #[doc = "A horizontal rule paragraph element."]
        #[serde(
            rename = "horizontalRule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub horizontal_rule: ::std::option::Option<crate::schemas::HorizontalRule>,
        #[doc = "An inline object paragraph element."]
        #[serde(
            rename = "inlineObjectElement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_object_element: ::std::option::Option<crate::schemas::InlineObjectElement>,
        #[doc = "A page break paragraph element."]
        #[serde(
            rename = "pageBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_break: ::std::option::Option<crate::schemas::PageBreak>,
        #[doc = "A paragraph element that links to a person or email address."]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<crate::schemas::Person>,
        #[doc = "The zero-based start index of this paragraph element, in UTF-16 code units."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "A text run paragraph element."]
        #[serde(
            rename = "textRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_run: ::std::option::Option<crate::schemas::TextRun>,
    }
    impl ::google_field_selector::FieldSelector for ParagraphElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphElement {
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
        #[doc = "Whether to avoid widows and orphans for the paragraph. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "avoidWidowAndOrphan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub avoid_widow_and_orphan: ::std::option::Option<bool>,
        #[doc = "The border between this paragraph and the next and previous paragraphs. If unset, the value is inherited from the parent. The between border is rendered when the adjacent paragraph has the same border and indent properties. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
        #[serde(
            rename = "borderBetween",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_between: ::std::option::Option<crate::schemas::ParagraphBorder>,
        #[doc = "The border at the bottom of this paragraph. If unset, the value is inherited from the parent. The bottom border is rendered when the paragraph below has different border and indent properties. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
        #[serde(
            rename = "borderBottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_bottom: ::std::option::Option<crate::schemas::ParagraphBorder>,
        #[doc = "The border to the left of this paragraph. If unset, the value is inherited from the parent. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
        #[serde(
            rename = "borderLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_left: ::std::option::Option<crate::schemas::ParagraphBorder>,
        #[doc = "The border to the right of this paragraph. If unset, the value is inherited from the parent. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
        #[serde(
            rename = "borderRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_right: ::std::option::Option<crate::schemas::ParagraphBorder>,
        #[doc = "The border at the top of this paragraph. If unset, the value is inherited from the parent. The top border is rendered when the paragraph above has different border and indent properties. Paragraph borders cannot be partially updated. When making changes to a paragraph border the new border must be specified in its entirety."]
        #[serde(
            rename = "borderTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_top: ::std::option::Option<crate::schemas::ParagraphBorder>,
        #[doc = "The text direction of this paragraph. If unset, the value defaults to LEFT_TO_RIGHT since paragraph direction is not inherited."]
        #[serde(
            rename = "direction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direction: ::std::option::Option<crate::schemas::ParagraphStyleDirection>,
        #[doc = "The heading ID of the paragraph. If empty, then this paragraph is not a heading. This property is read-only."]
        #[serde(
            rename = "headingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub heading_id: ::std::option::Option<String>,
        #[doc = "The amount of indentation for the paragraph on the side that corresponds to the end of the text, based on the current paragraph direction. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "indentEnd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_end: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of indentation for the first line of the paragraph. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "indentFirstLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_first_line: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of indentation for the paragraph on the side that corresponds to the start of the text, based on the current paragraph direction. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "indentStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_start: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "Whether all lines of the paragraph should be laid out on the same page or column if possible. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "keepLinesTogether",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keep_lines_together: ::std::option::Option<bool>,
        #[doc = "Whether at least a part of this paragraph should be laid out on the same page or column as the next paragraph if possible. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "keepWithNext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keep_with_next: ::std::option::Option<bool>,
        #[doc = "The amount of space between lines, as a percentage of normal, where normal is represented as 100.0. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "lineSpacing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_spacing: ::std::option::Option<f32>,
        #[doc = "The named style type of the paragraph. Since updating the named style type affects other properties within ParagraphStyle, the named style type is applied before the other properties are updated."]
        #[serde(
            rename = "namedStyleType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_style_type: ::std::option::Option<crate::schemas::ParagraphStyleNamedStyleType>,
        #[doc = "The shading of the paragraph. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "shading",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shading: ::std::option::Option<crate::schemas::Shading>,
        #[doc = "The amount of extra space above the paragraph. If unset, the value is inherited from the parent."]
        #[serde(
            rename = "spaceAbove",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_above: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The amount of extra space below the paragraph. If unset, the value is inherited from the parent."]
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
        #[doc = "A list of the tab stops for this paragraph. The list of tab stops is not inherited. This property is read-only."]
        #[serde(
            rename = "tabStops",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tab_stops: ::std::option::Option<Vec<crate::schemas::TabStop>>,
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
        #[doc = "The paragraph is aligned to the end of the line. Right-aligned for LTR text, left-aligned otherwise."]
        End,
        #[doc = "The paragraph is justified."]
        Justified,
        #[doc = "The paragraph is aligned to the start of the line. Left-aligned for LTR text, right-aligned otherwise."]
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
        #[doc = "The content direction is unspecified."]
        ContentDirectionUnspecified,
        #[doc = "The content goes from left to right."]
        LeftToRight,
        #[doc = "The content goes from right to left."]
        RightToLeft,
    }
    impl ParagraphStyleDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleDirection::ContentDirectionUnspecified => {
                    "CONTENT_DIRECTION_UNSPECIFIED"
                }
                ParagraphStyleDirection::LeftToRight => "LEFT_TO_RIGHT",
                ParagraphStyleDirection::RightToLeft => "RIGHT_TO_LEFT",
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
                "CONTENT_DIRECTION_UNSPECIFIED" => {
                    ParagraphStyleDirection::ContentDirectionUnspecified
                }
                "LEFT_TO_RIGHT" => ParagraphStyleDirection::LeftToRight,
                "RIGHT_TO_LEFT" => ParagraphStyleDirection::RightToLeft,
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
                "CONTENT_DIRECTION_UNSPECIFIED" => {
                    ParagraphStyleDirection::ContentDirectionUnspecified
                }
                "LEFT_TO_RIGHT" => ParagraphStyleDirection::LeftToRight,
                "RIGHT_TO_LEFT" => ParagraphStyleDirection::RightToLeft,
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
    pub enum ParagraphStyleNamedStyleType {
        #[doc = "Heading 1."]
        Heading1,
        #[doc = "Heading 2."]
        Heading2,
        #[doc = "Heading 3."]
        Heading3,
        #[doc = "Heading 4."]
        Heading4,
        #[doc = "Heading 5."]
        Heading5,
        #[doc = "Heading 6."]
        Heading6,
        #[doc = "The type of named style is unspecified."]
        NamedStyleTypeUnspecified,
        #[doc = "Normal text."]
        NormalText,
        #[doc = "Subtitle."]
        Subtitle,
        #[doc = "Title."]
        Title,
    }
    impl ParagraphStyleNamedStyleType {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleNamedStyleType::Heading1 => "HEADING_1",
                ParagraphStyleNamedStyleType::Heading2 => "HEADING_2",
                ParagraphStyleNamedStyleType::Heading3 => "HEADING_3",
                ParagraphStyleNamedStyleType::Heading4 => "HEADING_4",
                ParagraphStyleNamedStyleType::Heading5 => "HEADING_5",
                ParagraphStyleNamedStyleType::Heading6 => "HEADING_6",
                ParagraphStyleNamedStyleType::NamedStyleTypeUnspecified => {
                    "NAMED_STYLE_TYPE_UNSPECIFIED"
                }
                ParagraphStyleNamedStyleType::NormalText => "NORMAL_TEXT",
                ParagraphStyleNamedStyleType::Subtitle => "SUBTITLE",
                ParagraphStyleNamedStyleType::Title => "TITLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ParagraphStyleNamedStyleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParagraphStyleNamedStyleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParagraphStyleNamedStyleType, ()> {
            Ok(match s {
                "HEADING_1" => ParagraphStyleNamedStyleType::Heading1,
                "HEADING_2" => ParagraphStyleNamedStyleType::Heading2,
                "HEADING_3" => ParagraphStyleNamedStyleType::Heading3,
                "HEADING_4" => ParagraphStyleNamedStyleType::Heading4,
                "HEADING_5" => ParagraphStyleNamedStyleType::Heading5,
                "HEADING_6" => ParagraphStyleNamedStyleType::Heading6,
                "NAMED_STYLE_TYPE_UNSPECIFIED" => {
                    ParagraphStyleNamedStyleType::NamedStyleTypeUnspecified
                }
                "NORMAL_TEXT" => ParagraphStyleNamedStyleType::NormalText,
                "SUBTITLE" => ParagraphStyleNamedStyleType::Subtitle,
                "TITLE" => ParagraphStyleNamedStyleType::Title,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ParagraphStyleNamedStyleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphStyleNamedStyleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphStyleNamedStyleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HEADING_1" => ParagraphStyleNamedStyleType::Heading1,
                "HEADING_2" => ParagraphStyleNamedStyleType::Heading2,
                "HEADING_3" => ParagraphStyleNamedStyleType::Heading3,
                "HEADING_4" => ParagraphStyleNamedStyleType::Heading4,
                "HEADING_5" => ParagraphStyleNamedStyleType::Heading5,
                "HEADING_6" => ParagraphStyleNamedStyleType::Heading6,
                "NAMED_STYLE_TYPE_UNSPECIFIED" => {
                    ParagraphStyleNamedStyleType::NamedStyleTypeUnspecified
                }
                "NORMAL_TEXT" => ParagraphStyleNamedStyleType::NormalText,
                "SUBTITLE" => ParagraphStyleNamedStyleType::Subtitle,
                "TITLE" => ParagraphStyleNamedStyleType::Title,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ParagraphStyleNamedStyleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphStyleNamedStyleType {
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
    pub struct ParagraphStyleSuggestionState {
        #[doc = "Indicates if there was a suggested change to alignment."]
        #[serde(
            rename = "alignmentSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alignment_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to avoid_widow_and_orphan."]
        #[serde(
            rename = "avoidWidowAndOrphanSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub avoid_widow_and_orphan_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_between."]
        #[serde(
            rename = "borderBetweenSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_between_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_bottom."]
        #[serde(
            rename = "borderBottomSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_bottom_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_left."]
        #[serde(
            rename = "borderLeftSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_left_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_right."]
        #[serde(
            rename = "borderRightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_right_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_top."]
        #[serde(
            rename = "borderTopSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_top_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to direction."]
        #[serde(
            rename = "directionSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direction_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to heading_id."]
        #[serde(
            rename = "headingIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub heading_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to indent_end."]
        #[serde(
            rename = "indentEndSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_end_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to indent_first_line."]
        #[serde(
            rename = "indentFirstLineSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_first_line_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to indent_start."]
        #[serde(
            rename = "indentStartSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indent_start_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to keep_lines_together."]
        #[serde(
            rename = "keepLinesTogetherSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keep_lines_together_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to keep_with_next."]
        #[serde(
            rename = "keepWithNextSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keep_with_next_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to line_spacing."]
        #[serde(
            rename = "lineSpacingSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_spacing_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to named_style_type."]
        #[serde(
            rename = "namedStyleTypeSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_style_type_suggested: ::std::option::Option<bool>,
        #[doc = "A mask that indicates which of the fields in shading have been changed in this suggestion."]
        #[serde(
            rename = "shadingSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shading_suggestion_state: ::std::option::Option<crate::schemas::ShadingSuggestionState>,
        #[doc = "Indicates if there was a suggested change to space_above."]
        #[serde(
            rename = "spaceAboveSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_above_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to space_below."]
        #[serde(
            rename = "spaceBelowSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_below_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to spacing_mode."]
        #[serde(
            rename = "spacingModeSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spacing_mode_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ParagraphStyleSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParagraphStyleSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Person {
        #[doc = "Output only. The unique ID of this link."]
        #[serde(
            rename = "personId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_id: ::std::option::Option<String>,
        #[doc = "Output only. The properties of this Person. This field is always present."]
        #[serde(
            rename = "personProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_properties: ::std::option::Option<crate::schemas::PersonProperties>,
        #[doc = "IDs for suggestions that remove this person link from the document. A Person might have multiple deletion IDs if, for example, multiple users suggest to delete it. If empty, then this person link isn't suggested for deletion."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "IDs for suggestions that insert this person link into the document. A Person might have multiple insertion IDs if it is a nested suggested change (a suggestion within a suggestion made by a different user, for example). If empty, then this person link isn't a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this Person, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this Person."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
    }
    impl ::google_field_selector::FieldSelector for Person {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Person {
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
    pub struct PersonProperties {
        #[doc = "Output only. The email address linked to this Person. This field is always present."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Output only. The name of the person if it is displayed in the link text instead of the person's email address."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PersonProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PositionedObject {
        #[doc = "The ID of this positioned object."]
        #[serde(
            rename = "objectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_id: ::std::option::Option<String>,
        #[doc = "The properties of this positioned object."]
        #[serde(
            rename = "positionedObjectProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positioned_object_properties:
            ::std::option::Option<crate::schemas::PositionedObjectProperties>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion ID. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_id: ::std::option::Option<String>,
        #[doc = "The suggested changes to the positioned object properties, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedPositionedObjectPropertiesChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_positioned_object_properties_changes: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::SuggestedPositionedObjectProperties,
            >,
        >,
    }
    impl ::google_field_selector::FieldSelector for PositionedObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PositionedObject {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PositionedObjectPositioning {
        #[doc = "The layout of this positioned object."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout: ::std::option::Option<crate::schemas::PositionedObjectPositioningLayout>,
        #[doc = "The offset of the left edge of the positioned object relative to the beginning of the Paragraph it is tethered to. The exact positioning of the object can depend on other content in the document and the document's styling."]
        #[serde(
            rename = "leftOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left_offset: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The offset of the top edge of the positioned object relative to the beginning of the Paragraph it is tethered to. The exact positioning of the object can depend on other content in the document and the document's styling."]
        #[serde(
            rename = "topOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_offset: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for PositionedObjectPositioning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PositionedObjectPositioning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PositionedObjectPositioningLayout {
        #[doc = "Breaks text such that the positioned object is on the left and text is on the right."]
        BreakLeft,
        #[doc = "Breaks text such that there is no text on the left or right of the positioned object."]
        BreakLeftRight,
        #[doc = "Breaks text such that the positioned object is on the right and text is on the left."]
        BreakRight,
        #[doc = "The positioned object is in front of the text."]
        InFrontOfText,
        #[doc = "The layout is unspecified."]
        PositionedObjectLayoutUnspecified,
        #[doc = "The text wraps around the positioned object."]
        WrapText,
    }
    impl PositionedObjectPositioningLayout {
        pub fn as_str(self) -> &'static str {
            match self {
                PositionedObjectPositioningLayout::BreakLeft => "BREAK_LEFT",
                PositionedObjectPositioningLayout::BreakLeftRight => "BREAK_LEFT_RIGHT",
                PositionedObjectPositioningLayout::BreakRight => "BREAK_RIGHT",
                PositionedObjectPositioningLayout::InFrontOfText => "IN_FRONT_OF_TEXT",
                PositionedObjectPositioningLayout::PositionedObjectLayoutUnspecified => {
                    "POSITIONED_OBJECT_LAYOUT_UNSPECIFIED"
                }
                PositionedObjectPositioningLayout::WrapText => "WRAP_TEXT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PositionedObjectPositioningLayout {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PositionedObjectPositioningLayout {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PositionedObjectPositioningLayout, ()> {
            Ok(match s {
                "BREAK_LEFT" => PositionedObjectPositioningLayout::BreakLeft,
                "BREAK_LEFT_RIGHT" => PositionedObjectPositioningLayout::BreakLeftRight,
                "BREAK_RIGHT" => PositionedObjectPositioningLayout::BreakRight,
                "IN_FRONT_OF_TEXT" => PositionedObjectPositioningLayout::InFrontOfText,
                "POSITIONED_OBJECT_LAYOUT_UNSPECIFIED" => {
                    PositionedObjectPositioningLayout::PositionedObjectLayoutUnspecified
                }
                "WRAP_TEXT" => PositionedObjectPositioningLayout::WrapText,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PositionedObjectPositioningLayout {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PositionedObjectPositioningLayout {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PositionedObjectPositioningLayout {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BREAK_LEFT" => PositionedObjectPositioningLayout::BreakLeft,
                "BREAK_LEFT_RIGHT" => PositionedObjectPositioningLayout::BreakLeftRight,
                "BREAK_RIGHT" => PositionedObjectPositioningLayout::BreakRight,
                "IN_FRONT_OF_TEXT" => PositionedObjectPositioningLayout::InFrontOfText,
                "POSITIONED_OBJECT_LAYOUT_UNSPECIFIED" => {
                    PositionedObjectPositioningLayout::PositionedObjectLayoutUnspecified
                }
                "WRAP_TEXT" => PositionedObjectPositioningLayout::WrapText,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PositionedObjectPositioningLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PositionedObjectPositioningLayout {
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
    pub struct PositionedObjectPositioningSuggestionState {
        #[doc = "Indicates if there was a suggested change to layout."]
        #[serde(
            rename = "layoutSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to left_offset."]
        #[serde(
            rename = "leftOffsetSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left_offset_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to top_offset."]
        #[serde(
            rename = "topOffsetSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_offset_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for PositionedObjectPositioningSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PositionedObjectPositioningSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PositionedObjectProperties {
        #[doc = "The embedded object of this positioned object."]
        #[serde(
            rename = "embeddedObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_object: ::std::option::Option<crate::schemas::EmbeddedObject>,
        #[doc = "The positioning of this positioned object relative to the newline of the Paragraph that references this positioned object."]
        #[serde(
            rename = "positioning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positioning: ::std::option::Option<crate::schemas::PositionedObjectPositioning>,
    }
    impl ::google_field_selector::FieldSelector for PositionedObjectProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PositionedObjectProperties {
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
    pub struct PositionedObjectPropertiesSuggestionState {
        #[doc = "A mask that indicates which of the fields in embedded_object have been changed in this suggestion."]
        #[serde(
            rename = "embeddedObjectSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub embedded_object_suggestion_state:
            ::std::option::Option<crate::schemas::EmbeddedObjectSuggestionState>,
        #[doc = "A mask that indicates which of the fields in positioning have been changed in this suggestion."]
        #[serde(
            rename = "positioningSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positioning_suggestion_state:
            ::std::option::Option<crate::schemas::PositionedObjectPositioningSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for PositionedObjectPropertiesSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PositionedObjectPropertiesSuggestionState {
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
        #[doc = "The zero-based end index of this range, exclusive, in UTF-16 code units. In all current uses, an end index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The ID of the header, footer or footnote that this range is contained in. An empty segment ID signifies the document's body."]
        #[serde(
            rename = "segmentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_id: ::std::option::Option<String>,
        #[doc = "The zero-based start index of this range, in UTF-16 code units. In all current uses, a start index must be provided. This field is an Int32Value in order to accommodate future use cases with open-ended ranges."]
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
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Finds text in the document matching this substring."]
        #[serde(
            rename = "containsText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contains_text: ::std::option::Option<crate::schemas::SubstringMatchCriteria>,
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
        #[doc = "The URI of the new image. The image is fetched once at insertion time and a copy is stored for display inside the document. Images must be less than 50MB in size, cannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF format. The provided URI can be at most 2 kB in length. The URI itself is saved with the image, and exposed via the ImageProperties.source_uri field."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
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
        #[doc = "Scales and centers the image to fill the bounds of the original image. The image may be cropped in order to fill the original image's bounds. The rendered size of the image will be the same as that of the original image."]
        CenterCrop,
        #[doc = "Unspecified image replace method. This value must not be used."]
        ImageReplaceMethodUnspecified,
    }
    impl ReplaceImageRequestImageReplaceMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                ReplaceImageRequestImageReplaceMethod::CenterCrop => "CENTER_CROP",
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceNamedRangeContentRequest {
        #[doc = "The ID of the named range whose content will be replaced. If there is no named range with the given ID a 400 bad request error is returned."]
        #[serde(
            rename = "namedRangeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_id: ::std::option::Option<String>,
        #[doc = "The name of the NamedRanges whose content will be replaced. If there are multiple named ranges with the given name, then the content of each one will be replaced. If there are no named ranges with the given name, then the request will be a no-op."]
        #[serde(
            rename = "namedRangeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_range_name: ::std::option::Option<String>,
        #[doc = "Replaces the content of the specified named range(s) with the given text."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReplaceNamedRangeContentRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReplaceNamedRangeContentRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Request {
        #[doc = "Creates a footer."]
        #[serde(
            rename = "createFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_footer: ::std::option::Option<crate::schemas::CreateFooterRequest>,
        #[doc = "Creates a footnote."]
        #[serde(
            rename = "createFootnote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_footnote: ::std::option::Option<crate::schemas::CreateFootnoteRequest>,
        #[doc = "Creates a header."]
        #[serde(
            rename = "createHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_header: ::std::option::Option<crate::schemas::CreateHeaderRequest>,
        #[doc = "Creates a named range."]
        #[serde(
            rename = "createNamedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_named_range: ::std::option::Option<crate::schemas::CreateNamedRangeRequest>,
        #[doc = "Creates bullets for paragraphs."]
        #[serde(
            rename = "createParagraphBullets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_paragraph_bullets:
            ::std::option::Option<crate::schemas::CreateParagraphBulletsRequest>,
        #[doc = "Deletes content from the document."]
        #[serde(
            rename = "deleteContentRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_content_range: ::std::option::Option<crate::schemas::DeleteContentRangeRequest>,
        #[doc = "Deletes a footer from the document."]
        #[serde(
            rename = "deleteFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_footer: ::std::option::Option<crate::schemas::DeleteFooterRequest>,
        #[doc = "Deletes a header from the document."]
        #[serde(
            rename = "deleteHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_header: ::std::option::Option<crate::schemas::DeleteHeaderRequest>,
        #[doc = "Deletes a named range."]
        #[serde(
            rename = "deleteNamedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_named_range: ::std::option::Option<crate::schemas::DeleteNamedRangeRequest>,
        #[doc = "Deletes bullets from paragraphs."]
        #[serde(
            rename = "deleteParagraphBullets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_paragraph_bullets:
            ::std::option::Option<crate::schemas::DeleteParagraphBulletsRequest>,
        #[doc = "Deletes a positioned object from the document."]
        #[serde(
            rename = "deletePositionedObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_positioned_object:
            ::std::option::Option<crate::schemas::DeletePositionedObjectRequest>,
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
        #[doc = "Inserts an inline image at the specified location."]
        #[serde(
            rename = "insertInlineImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_inline_image: ::std::option::Option<crate::schemas::InsertInlineImageRequest>,
        #[doc = "Inserts a page break at the specified location."]
        #[serde(
            rename = "insertPageBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_page_break: ::std::option::Option<crate::schemas::InsertPageBreakRequest>,
        #[doc = "Inserts a section break at the specified location."]
        #[serde(
            rename = "insertSectionBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_section_break: ::std::option::Option<crate::schemas::InsertSectionBreakRequest>,
        #[doc = "Inserts a table at the specified location."]
        #[serde(
            rename = "insertTable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_table: ::std::option::Option<crate::schemas::InsertTableRequest>,
        #[doc = "Inserts an empty column into a table."]
        #[serde(
            rename = "insertTableColumn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_table_column: ::std::option::Option<crate::schemas::InsertTableColumnRequest>,
        #[doc = "Inserts an empty row into a table."]
        #[serde(
            rename = "insertTableRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_table_row: ::std::option::Option<crate::schemas::InsertTableRowRequest>,
        #[doc = "Inserts text at the specified location."]
        #[serde(
            rename = "insertText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_text: ::std::option::Option<crate::schemas::InsertTextRequest>,
        #[doc = "Merges cells in a table."]
        #[serde(
            rename = "mergeTableCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub merge_table_cells: ::std::option::Option<crate::schemas::MergeTableCellsRequest>,
        #[doc = "Replaces all instances of the specified text."]
        #[serde(
            rename = "replaceAllText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_all_text: ::std::option::Option<crate::schemas::ReplaceAllTextRequest>,
        #[doc = "Replaces an image in the document."]
        #[serde(
            rename = "replaceImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_image: ::std::option::Option<crate::schemas::ReplaceImageRequest>,
        #[doc = "Replaces the content in a named range."]
        #[serde(
            rename = "replaceNamedRangeContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub replace_named_range_content:
            ::std::option::Option<crate::schemas::ReplaceNamedRangeContentRequest>,
        #[doc = "Unmerges cells in a table."]
        #[serde(
            rename = "unmergeTableCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unmerge_table_cells: ::std::option::Option<crate::schemas::UnmergeTableCellsRequest>,
        #[doc = "Updates the style of the document."]
        #[serde(
            rename = "updateDocumentStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_document_style:
            ::std::option::Option<crate::schemas::UpdateDocumentStyleRequest>,
        #[doc = "Updates the paragraph style at the specified range."]
        #[serde(
            rename = "updateParagraphStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_paragraph_style:
            ::std::option::Option<crate::schemas::UpdateParagraphStyleRequest>,
        #[doc = "Updates the section style of the specified range."]
        #[serde(
            rename = "updateSectionStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_section_style: ::std::option::Option<crate::schemas::UpdateSectionStyleRequest>,
        #[doc = "Updates the style of table cells."]
        #[serde(
            rename = "updateTableCellStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_table_cell_style:
            ::std::option::Option<crate::schemas::UpdateTableCellStyleRequest>,
        #[doc = "Updates the properties of columns in a table."]
        #[serde(
            rename = "updateTableColumnProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_table_column_properties:
            ::std::option::Option<crate::schemas::UpdateTableColumnPropertiesRequest>,
        #[doc = "Updates the row style in a table."]
        #[serde(
            rename = "updateTableRowStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_table_row_style:
            ::std::option::Option<crate::schemas::UpdateTableRowStyleRequest>,
        #[doc = "Updates the text style at the specified range."]
        #[serde(
            rename = "updateTextStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_text_style: ::std::option::Option<crate::schemas::UpdateTextStyleRequest>,
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
    pub struct Response {
        #[doc = "The result of creating a footer."]
        #[serde(
            rename = "createFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_footer: ::std::option::Option<crate::schemas::CreateFooterResponse>,
        #[doc = "The result of creating a footnote."]
        #[serde(
            rename = "createFootnote",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_footnote: ::std::option::Option<crate::schemas::CreateFootnoteResponse>,
        #[doc = "The result of creating a header."]
        #[serde(
            rename = "createHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_header: ::std::option::Option<crate::schemas::CreateHeaderResponse>,
        #[doc = "The result of creating a named range."]
        #[serde(
            rename = "createNamedRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_named_range: ::std::option::Option<crate::schemas::CreateNamedRangeResponse>,
        #[doc = "The result of inserting an inline image."]
        #[serde(
            rename = "insertInlineImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_inline_image: ::std::option::Option<crate::schemas::InsertInlineImageResponse>,
        #[doc = "The result of inserting an inline Google Sheets chart."]
        #[serde(
            rename = "insertInlineSheetsChart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_inline_sheets_chart:
            ::std::option::Option<crate::schemas::InsertInlineSheetsChartResponse>,
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
    pub struct SectionBreak {
        #[doc = "The style of the section after this section break."]
        #[serde(
            rename = "sectionStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_style: ::std::option::Option<crate::schemas::SectionStyle>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A SectionBreak may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for SectionBreak {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SectionBreak {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SectionColumnProperties {
        #[doc = "The padding at the end of the column."]
        #[serde(
            rename = "paddingEnd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_end: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "Output only. The width of the column."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for SectionColumnProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SectionColumnProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SectionStyle {
        #[doc = "The section's columns properties. If empty, the section contains one column with the default properties in the Docs editor. A section can be updated to have no more than three columns. When updating this property, setting a concrete value is required. Unsetting this property will result in a 400 bad request error."]
        #[serde(
            rename = "columnProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_properties: ::std::option::Option<Vec<crate::schemas::SectionColumnProperties>>,
        #[doc = "The style of column separators. This style can be set even when there is one column in the section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "columnSeparatorStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_separator_style:
            ::std::option::Option<crate::schemas::SectionStyleColumnSeparatorStyle>,
        #[doc = "The content direction of this section. If unset, the value defaults to LEFT_TO_RIGHT. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "contentDirection",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_direction: ::std::option::Option<crate::schemas::SectionStyleContentDirection>,
        #[doc = "The ID of the default footer. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_footer_id. This property is read-only."]
        #[serde(
            rename = "defaultFooterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_footer_id: ::std::option::Option<String>,
        #[doc = "The ID of the default header. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's default_header_id. This property is read-only."]
        #[serde(
            rename = "defaultHeaderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_header_id: ::std::option::Option<String>,
        #[doc = "The ID of the footer used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the footers on even pages in the section. If it is false, the footers on even pages uses the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_footer_id. This property is read-only."]
        #[serde(
            rename = "evenPageFooterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub even_page_footer_id: ::std::option::Option<String>,
        #[doc = "The ID of the header used only for even pages. If the value of DocumentStyle's use_even_page_header_footer is true, this value is used for the headers on even pages in the section. If it is false, the headers on even pages uses the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's even_page_header_id. This property is read-only."]
        #[serde(
            rename = "evenPageHeaderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub even_page_header_id: ::std::option::Option<String>,
        #[doc = "The ID of the footer used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the footer on the first page of the section. If it is false, the footer on the first page of the section uses the default_footer_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_footer_id. This property is read-only."]
        #[serde(
            rename = "firstPageFooterId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_page_footer_id: ::std::option::Option<String>,
        #[doc = "The ID of the header used only for the first page of the section. If use_first_page_header_footer is true, this value is used for the header on the first page of the section. If it is false, the header on the first page of the section uses the default_header_id. If unset, the value inherits from the previous SectionBreak's SectionStyle. If the value is unset in the first SectionBreak, it inherits from DocumentStyle's first_page_header_id. This property is read-only."]
        #[serde(
            rename = "firstPageHeaderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_page_header_id: ::std::option::Option<String>,
        #[doc = "The bottom page margin of the section. If unset, uses margin_bottom from DocumentStyle. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "marginBottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_bottom: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The footer margin of the section. If unset, uses margin_footer from DocumentStyle. If updated, use_custom_header_footer_margins is set to true on DocumentStyle. The value of use_custom_header_footer_margins on DocumentStyle indicates if a footer margin is being respected for this section When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "marginFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_footer: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The header margin of the section. If unset, uses margin_header from DocumentStyle. If updated, use_custom_header_footer_margins is set to true on DocumentStyle. The value of use_custom_header_footer_margins on DocumentStyle indicates if a header margin is being respected for this section. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "marginHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_header: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The left page margin of the section. If unset, uses margin_left from DocumentStyle. Updating left margin causes columns in this section to resize. Since the margin affects column width, it is applied before column properties. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "marginLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_left: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The right page margin of the section. If unset, uses margin_right from DocumentStyle. Updating right margin causes columns in this section to resize. Since the margin affects column width, it is applied before column properties. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "marginRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_right: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The top page margin of the section. If unset, uses margin_top from DocumentStyle. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "marginTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub margin_top: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The page number from which to start counting the number of pages for this section. If unset, page numbering continues from the previous section. If the value is unset in the first SectionBreak, refer to DocumentStyle's page_number_start. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "pageNumberStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_number_start: ::std::option::Option<i32>,
        #[doc = "Output only. The type of section."]
        #[serde(
            rename = "sectionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_type: ::std::option::Option<crate::schemas::SectionStyleSectionType>,
        #[doc = "Indicates whether to use the first page header / footer IDs for the first page of the section. If unset, it inherits from DocumentStyle's use_first_page_header_footer for the first section. If the value is unset for subsequent sectors, it should be interpreted as false. When updating this property, setting a concrete value is required. Unsetting this property results in a 400 bad request error."]
        #[serde(
            rename = "useFirstPageHeaderFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_first_page_header_footer: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SectionStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SectionStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SectionStyleColumnSeparatorStyle {
        #[doc = "Renders a column separator line between each column."]
        BetweenEachColumn,
        #[doc = "An unspecified column separator style."]
        ColumnSeparatorStyleUnspecified,
        #[doc = "No column separator lines between columns."]
        None,
    }
    impl SectionStyleColumnSeparatorStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                SectionStyleColumnSeparatorStyle::BetweenEachColumn => "BETWEEN_EACH_COLUMN",
                SectionStyleColumnSeparatorStyle::ColumnSeparatorStyleUnspecified => {
                    "COLUMN_SEPARATOR_STYLE_UNSPECIFIED"
                }
                SectionStyleColumnSeparatorStyle::None => "NONE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SectionStyleColumnSeparatorStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SectionStyleColumnSeparatorStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SectionStyleColumnSeparatorStyle, ()> {
            Ok(match s {
                "BETWEEN_EACH_COLUMN" => SectionStyleColumnSeparatorStyle::BetweenEachColumn,
                "COLUMN_SEPARATOR_STYLE_UNSPECIFIED" => {
                    SectionStyleColumnSeparatorStyle::ColumnSeparatorStyleUnspecified
                }
                "NONE" => SectionStyleColumnSeparatorStyle::None,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SectionStyleColumnSeparatorStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SectionStyleColumnSeparatorStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SectionStyleColumnSeparatorStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BETWEEN_EACH_COLUMN" => SectionStyleColumnSeparatorStyle::BetweenEachColumn,
                "COLUMN_SEPARATOR_STYLE_UNSPECIFIED" => {
                    SectionStyleColumnSeparatorStyle::ColumnSeparatorStyleUnspecified
                }
                "NONE" => SectionStyleColumnSeparatorStyle::None,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SectionStyleColumnSeparatorStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SectionStyleColumnSeparatorStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SectionStyleContentDirection {
        #[doc = "The content direction is unspecified."]
        ContentDirectionUnspecified,
        #[doc = "The content goes from left to right."]
        LeftToRight,
        #[doc = "The content goes from right to left."]
        RightToLeft,
    }
    impl SectionStyleContentDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                SectionStyleContentDirection::ContentDirectionUnspecified => {
                    "CONTENT_DIRECTION_UNSPECIFIED"
                }
                SectionStyleContentDirection::LeftToRight => "LEFT_TO_RIGHT",
                SectionStyleContentDirection::RightToLeft => "RIGHT_TO_LEFT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SectionStyleContentDirection {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SectionStyleContentDirection {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SectionStyleContentDirection, ()> {
            Ok(match s {
                "CONTENT_DIRECTION_UNSPECIFIED" => {
                    SectionStyleContentDirection::ContentDirectionUnspecified
                }
                "LEFT_TO_RIGHT" => SectionStyleContentDirection::LeftToRight,
                "RIGHT_TO_LEFT" => SectionStyleContentDirection::RightToLeft,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SectionStyleContentDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SectionStyleContentDirection {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SectionStyleContentDirection {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_DIRECTION_UNSPECIFIED" => {
                    SectionStyleContentDirection::ContentDirectionUnspecified
                }
                "LEFT_TO_RIGHT" => SectionStyleContentDirection::LeftToRight,
                "RIGHT_TO_LEFT" => SectionStyleContentDirection::RightToLeft,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SectionStyleContentDirection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SectionStyleContentDirection {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SectionStyleSectionType {
        #[doc = "The section starts immediately after the last paragraph of the previous section."]
        Continuous,
        #[doc = "The section starts on the next page."]
        NextPage,
        #[doc = "The section type is unspecified."]
        SectionTypeUnspecified,
    }
    impl SectionStyleSectionType {
        pub fn as_str(self) -> &'static str {
            match self {
                SectionStyleSectionType::Continuous => "CONTINUOUS",
                SectionStyleSectionType::NextPage => "NEXT_PAGE",
                SectionStyleSectionType::SectionTypeUnspecified => "SECTION_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SectionStyleSectionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SectionStyleSectionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SectionStyleSectionType, ()> {
            Ok(match s {
                "CONTINUOUS" => SectionStyleSectionType::Continuous,
                "NEXT_PAGE" => SectionStyleSectionType::NextPage,
                "SECTION_TYPE_UNSPECIFIED" => SectionStyleSectionType::SectionTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SectionStyleSectionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SectionStyleSectionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SectionStyleSectionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTINUOUS" => SectionStyleSectionType::Continuous,
                "NEXT_PAGE" => SectionStyleSectionType::NextPage,
                "SECTION_TYPE_UNSPECIFIED" => SectionStyleSectionType::SectionTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SectionStyleSectionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SectionStyleSectionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Shading {
        #[doc = "The background color of this paragraph shading."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::OptionalColor>,
    }
    impl ::google_field_selector::FieldSelector for Shading {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Shading {
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
    pub struct ShadingSuggestionState {
        #[doc = "Indicates if there was a suggested change to the Shading."]
        #[serde(
            rename = "backgroundColorSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ShadingSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShadingSuggestionState {
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
    pub struct SheetsChartReference {
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet that is embedded."]
        #[serde(
            rename = "chartId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_id: ::std::option::Option<i32>,
        #[doc = "The ID of the Google Sheets spreadsheet that contains the source chart."]
        #[serde(
            rename = "spreadsheetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SheetsChartReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SheetsChartReference {
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
    pub struct SheetsChartReferenceSuggestionState {
        #[doc = "Indicates if there was a suggested change to chart_id."]
        #[serde(
            rename = "chartIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chart_id_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to spreadsheet_id."]
        #[serde(
            rename = "spreadsheetIdSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spreadsheet_id_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SheetsChartReferenceSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SheetsChartReferenceSuggestionState {
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SizeSuggestionState {
        #[doc = "Indicates if there was a suggested change to height."]
        #[serde(
            rename = "heightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to width."]
        #[serde(
            rename = "widthSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SizeSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SizeSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StructuralElement {
        #[doc = "The zero-based end index of this structural element, exclusive, in UTF-16 code units."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "A paragraph type of structural element."]
        #[serde(
            rename = "paragraph",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph: ::std::option::Option<crate::schemas::Paragraph>,
        #[doc = "A section break type of structural element."]
        #[serde(
            rename = "sectionBreak",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_break: ::std::option::Option<crate::schemas::SectionBreak>,
        #[doc = "The zero-based start index of this structural element, in UTF-16 code units."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "A table type of structural element."]
        #[serde(
            rename = "table",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table: ::std::option::Option<crate::schemas::Table>,
        #[doc = "A table of contents type of structural element."]
        #[serde(
            rename = "tableOfContents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_of_contents: ::std::option::Option<crate::schemas::TableOfContents>,
    }
    impl ::google_field_selector::FieldSelector for StructuralElement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StructuralElement {
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
        #[doc = "Indicates whether the search should respect case: - `True`: the search is case sensitive. - `False`: the search is case insensitive."]
        #[serde(
            rename = "matchCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_case: ::std::option::Option<bool>,
        #[doc = "The text to search for in the document."]
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
    pub struct SuggestedBullet {
        #[doc = "A Bullet that only includes the changes made in this suggestion. This can be used along with the bullet_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "bullet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet: ::std::option::Option<crate::schemas::Bullet>,
        #[doc = "A mask that indicates which of the fields on the base Bullet have been changed in this suggestion."]
        #[serde(
            rename = "bulletSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bullet_suggestion_state: ::std::option::Option<crate::schemas::BulletSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedBullet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedBullet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedDocumentStyle {
        #[doc = "A DocumentStyle that only includes the changes made in this suggestion. This can be used along with the document_style_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "documentStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_style: ::std::option::Option<crate::schemas::DocumentStyle>,
        #[doc = "A mask that indicates which of the fields on the base DocumentStyle have been changed in this suggestion."]
        #[serde(
            rename = "documentStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_style_suggestion_state:
            ::std::option::Option<crate::schemas::DocumentStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedDocumentStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedDocumentStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedInlineObjectProperties {
        #[doc = "An InlineObjectProperties that only includes the changes made in this suggestion. This can be used along with the inline_object_properties_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "inlineObjectProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_object_properties: ::std::option::Option<crate::schemas::InlineObjectProperties>,
        #[doc = "A mask that indicates which of the fields on the base InlineObjectProperties have been changed in this suggestion."]
        #[serde(
            rename = "inlineObjectPropertiesSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_object_properties_suggestion_state:
            ::std::option::Option<crate::schemas::InlineObjectPropertiesSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedInlineObjectProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedInlineObjectProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedListProperties {
        #[doc = "A ListProperties that only includes the changes made in this suggestion. This can be used along with the list_properties_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "listProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_properties: ::std::option::Option<crate::schemas::ListProperties>,
        #[doc = "A mask that indicates which of the fields on the base ListProperties have been changed in this suggestion."]
        #[serde(
            rename = "listPropertiesSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_properties_suggestion_state:
            ::std::option::Option<crate::schemas::ListPropertiesSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedListProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedListProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedNamedStyles {
        #[doc = "A NamedStyles that only includes the changes made in this suggestion. This can be used along with the named_styles_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "namedStyles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_styles: ::std::option::Option<crate::schemas::NamedStyles>,
        #[doc = "A mask that indicates which of the fields on the base NamedStyles have been changed in this suggestion."]
        #[serde(
            rename = "namedStylesSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub named_styles_suggestion_state:
            ::std::option::Option<crate::schemas::NamedStylesSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedNamedStyles {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedNamedStyles {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedParagraphStyle {
        #[doc = "A ParagraphStyle that only includes the changes made in this suggestion. This can be used along with the paragraph_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "paragraphStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph_style: ::std::option::Option<crate::schemas::ParagraphStyle>,
        #[doc = "A mask that indicates which of the fields on the base ParagraphStyle have been changed in this suggestion."]
        #[serde(
            rename = "paragraphStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph_style_suggestion_state:
            ::std::option::Option<crate::schemas::ParagraphStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedParagraphStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedParagraphStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedPositionedObjectProperties {
        #[doc = "A PositionedObjectProperties that only includes the changes made in this suggestion. This can be used along with the positioned_object_properties_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "positionedObjectProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positioned_object_properties:
            ::std::option::Option<crate::schemas::PositionedObjectProperties>,
        #[doc = "A mask that indicates which of the fields on the base PositionedObjectProperties have been changed in this suggestion."]
        #[serde(
            rename = "positionedObjectPropertiesSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub positioned_object_properties_suggestion_state:
            ::std::option::Option<crate::schemas::PositionedObjectPropertiesSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedPositionedObjectProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedPositionedObjectProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedTableCellStyle {
        #[doc = "A TableCellStyle that only includes the changes made in this suggestion. This can be used along with the table_cell_style_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "tableCellStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_style: ::std::option::Option<crate::schemas::TableCellStyle>,
        #[doc = "A mask that indicates which of the fields on the base TableCellStyle have been changed in this suggestion."]
        #[serde(
            rename = "tableCellStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_style_suggestion_state:
            ::std::option::Option<crate::schemas::TableCellStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedTableCellStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedTableCellStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedTableRowStyle {
        #[doc = "A TableRowStyle that only includes the changes made in this suggestion. This can be used along with the table_row_style_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "tableRowStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_row_style: ::std::option::Option<crate::schemas::TableRowStyle>,
        #[doc = "A mask that indicates which of the fields on the base TableRowStyle have been changed in this suggestion."]
        #[serde(
            rename = "tableRowStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_row_style_suggestion_state:
            ::std::option::Option<crate::schemas::TableRowStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedTableRowStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedTableRowStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestedTextStyle {
        #[doc = "A TextStyle that only includes the changes made in this suggestion. This can be used along with the text_style_suggestion_state to see which fields have changed and their new values."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
        #[doc = "A mask that indicates which of the fields on the base TextStyle have been changed in this suggestion."]
        #[serde(
            rename = "textStyleSuggestionState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style_suggestion_state:
            ::std::option::Option<crate::schemas::TextStyleSuggestionState>,
    }
    impl ::google_field_selector::FieldSelector for SuggestedTextStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestedTextStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TabStop {
        #[doc = "The alignment of this tab stop. If unset, the value defaults to START."]
        #[serde(
            rename = "alignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alignment: ::std::option::Option<crate::schemas::TabStopAlignment>,
        #[doc = "The offset between this tab stop and the start margin."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for TabStop {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TabStop {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TabStopAlignment {
        #[doc = "The tab stop is aligned to the center of the line."]
        Center,
        #[doc = "The tab stop is aligned to the end of the line."]
        End,
        #[doc = "The tab stop is aligned to the start of the line. This is the default."]
        Start,
        #[doc = "The tab stop alignment is unspecified."]
        TabStopAlignmentUnspecified,
    }
    impl TabStopAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                TabStopAlignment::Center => "CENTER",
                TabStopAlignment::End => "END",
                TabStopAlignment::Start => "START",
                TabStopAlignment::TabStopAlignmentUnspecified => "TAB_STOP_ALIGNMENT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TabStopAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TabStopAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TabStopAlignment, ()> {
            Ok(match s {
                "CENTER" => TabStopAlignment::Center,
                "END" => TabStopAlignment::End,
                "START" => TabStopAlignment::Start,
                "TAB_STOP_ALIGNMENT_UNSPECIFIED" => TabStopAlignment::TabStopAlignmentUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TabStopAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TabStopAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TabStopAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER" => TabStopAlignment::Center,
                "END" => TabStopAlignment::End,
                "START" => TabStopAlignment::Start,
                "TAB_STOP_ALIGNMENT_UNSPECIFIED" => TabStopAlignment::TabStopAlignmentUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TabStopAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TabStopAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Table {
        #[doc = "Number of columns in the table. It is possible for a table to be non-rectangular, so some rows may have a different number of cells."]
        #[serde(
            rename = "columns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub columns: ::std::option::Option<i32>,
        #[doc = "Number of rows in the table."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<i32>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A Table may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The contents and style of each row."]
        #[serde(
            rename = "tableRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_rows: ::std::option::Option<Vec<crate::schemas::TableRow>>,
        #[doc = "The style of the table."]
        #[serde(
            rename = "tableStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_style: ::std::option::Option<crate::schemas::TableStyle>,
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
    pub struct TableCell {
        #[doc = "The content of the cell."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<Vec<crate::schemas::StructuralElement>>,
        #[doc = "The zero-based end index of this cell, exclusive, in UTF-16 code units."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The zero-based start index of this cell, in UTF-16 code units."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A TableCell may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested changes to the table cell style, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTableCellStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_table_cell_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTableCellStyle>,
        >,
        #[doc = "The style of the cell."]
        #[serde(
            rename = "tableCellStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_style: ::std::option::Option<crate::schemas::TableCellStyle>,
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
    pub struct TableCellBorder {
        #[doc = "The color of the border. This color cannot be transparent."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::OptionalColor>,
        #[doc = "The dash style of the border."]
        #[serde(
            rename = "dashStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dash_style: ::std::option::Option<crate::schemas::TableCellBorderDashStyle>,
        #[doc = "The width of the border."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for TableCellBorder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellBorder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableCellBorderDashStyle {
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'. This is the default dash style."]
        Solid,
    }
    impl TableCellBorderDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                TableCellBorderDashStyle::Dash => "DASH",
                TableCellBorderDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                TableCellBorderDashStyle::Dot => "DOT",
                TableCellBorderDashStyle::Solid => "SOLID",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TableCellBorderDashStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TableCellBorderDashStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TableCellBorderDashStyle, ()> {
            Ok(match s {
                "DASH" => TableCellBorderDashStyle::Dash,
                "DASH_STYLE_UNSPECIFIED" => TableCellBorderDashStyle::DashStyleUnspecified,
                "DOT" => TableCellBorderDashStyle::Dot,
                "SOLID" => TableCellBorderDashStyle::Solid,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TableCellBorderDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableCellBorderDashStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableCellBorderDashStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH" => TableCellBorderDashStyle::Dash,
                "DASH_STYLE_UNSPECIFIED" => TableCellBorderDashStyle::DashStyleUnspecified,
                "DOT" => TableCellBorderDashStyle::Dot,
                "SOLID" => TableCellBorderDashStyle::Solid,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TableCellBorderDashStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellBorderDashStyle {
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
        #[doc = "The zero-based column index. For example, the second column in the table has a column index of 1."]
        #[serde(
            rename = "columnIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_index: ::std::option::Option<i32>,
        #[doc = "The zero-based row index. For example, the second row in the table has a row index of 1."]
        #[serde(
            rename = "rowIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_index: ::std::option::Option<i32>,
        #[doc = "The location where the table starts in the document."]
        #[serde(
            rename = "tableStartLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_start_location: ::std::option::Option<crate::schemas::Location>,
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
    pub struct TableCellStyle {
        #[doc = "The background color of the cell."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::OptionalColor>,
        #[doc = "The bottom border of the cell."]
        #[serde(
            rename = "borderBottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_bottom: ::std::option::Option<crate::schemas::TableCellBorder>,
        #[doc = "The left border of the cell."]
        #[serde(
            rename = "borderLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_left: ::std::option::Option<crate::schemas::TableCellBorder>,
        #[doc = "The right border of the cell."]
        #[serde(
            rename = "borderRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_right: ::std::option::Option<crate::schemas::TableCellBorder>,
        #[doc = "The top border of the cell."]
        #[serde(
            rename = "borderTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_top: ::std::option::Option<crate::schemas::TableCellBorder>,
        #[doc = "The column span of the cell. This property is read-only."]
        #[serde(
            rename = "columnSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_span: ::std::option::Option<i32>,
        #[doc = "The alignment of the content in the table cell. The default alignment matches the alignment for newly created table cells in the Docs editor."]
        #[serde(
            rename = "contentAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_alignment:
            ::std::option::Option<crate::schemas::TableCellStyleContentAlignment>,
        #[doc = "The bottom padding of the cell."]
        #[serde(
            rename = "paddingBottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_bottom: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The left padding of the cell."]
        #[serde(
            rename = "paddingLeft",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_left: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The right padding of the cell."]
        #[serde(
            rename = "paddingRight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_right: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The top padding of the cell."]
        #[serde(
            rename = "paddingTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_top: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The row span of the cell. This property is read-only."]
        #[serde(
            rename = "rowSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TableCellStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableCellStyleContentAlignment {
        #[doc = "An alignment that aligns the content to the bottom of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
        Bottom,
        #[doc = "An unspecified content alignment. The content alignment is inherited from the parent if one exists."]
        ContentAlignmentUnspecified,
        #[doc = "An unsupported content alignment."]
        ContentAlignmentUnsupported,
        #[doc = "An alignment that aligns the content to the middle of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
        Middle,
        #[doc = "An alignment that aligns the content to the top of the content holder. Corresponds to ECMA-376 ST_TextAnchoringType 't'."]
        Top,
    }
    impl TableCellStyleContentAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                TableCellStyleContentAlignment::Bottom => "BOTTOM",
                TableCellStyleContentAlignment::ContentAlignmentUnspecified => {
                    "CONTENT_ALIGNMENT_UNSPECIFIED"
                }
                TableCellStyleContentAlignment::ContentAlignmentUnsupported => {
                    "CONTENT_ALIGNMENT_UNSUPPORTED"
                }
                TableCellStyleContentAlignment::Middle => "MIDDLE",
                TableCellStyleContentAlignment::Top => "TOP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TableCellStyleContentAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TableCellStyleContentAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TableCellStyleContentAlignment, ()> {
            Ok(match s {
                "BOTTOM" => TableCellStyleContentAlignment::Bottom,
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    TableCellStyleContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    TableCellStyleContentAlignment::ContentAlignmentUnsupported
                }
                "MIDDLE" => TableCellStyleContentAlignment::Middle,
                "TOP" => TableCellStyleContentAlignment::Top,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TableCellStyleContentAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableCellStyleContentAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableCellStyleContentAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTTOM" => TableCellStyleContentAlignment::Bottom,
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    TableCellStyleContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    TableCellStyleContentAlignment::ContentAlignmentUnsupported
                }
                "MIDDLE" => TableCellStyleContentAlignment::Middle,
                "TOP" => TableCellStyleContentAlignment::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TableCellStyleContentAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellStyleContentAlignment {
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
    pub struct TableCellStyleSuggestionState {
        #[doc = "Indicates if there was a suggested change to background_color."]
        #[serde(
            rename = "backgroundColorSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_bottom."]
        #[serde(
            rename = "borderBottomSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_bottom_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_left."]
        #[serde(
            rename = "borderLeftSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_left_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_right."]
        #[serde(
            rename = "borderRightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_right_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to border_top."]
        #[serde(
            rename = "borderTopSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_top_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to column_span."]
        #[serde(
            rename = "columnSpanSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_span_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to content_alignment."]
        #[serde(
            rename = "contentAlignmentSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_alignment_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to padding_bottom."]
        #[serde(
            rename = "paddingBottomSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_bottom_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to padding_left."]
        #[serde(
            rename = "paddingLeftSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_left_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to padding_right."]
        #[serde(
            rename = "paddingRightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_right_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to padding_top."]
        #[serde(
            rename = "paddingTopSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub padding_top_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to row_span."]
        #[serde(
            rename = "rowSpanSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TableCellStyleSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableCellStyleSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableColumnProperties {
        #[doc = "The width of the column. Set when the column's `width_type` is FIXED_WIDTH."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The width type of the column."]
        #[serde(
            rename = "widthType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_type: ::std::option::Option<crate::schemas::TableColumnPropertiesWidthType>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableColumnPropertiesWidthType {
        #[doc = "The column width is evenly distributed among the other evenly distrubted columns. The width of the column is automatically determined and will have an equal portion of the width remaining for the table after accounting for all columns with specified widths."]
        EvenlyDistributed,
        #[doc = "A fixed column width. The width property contains the column's width."]
        FixedWidth,
        #[doc = "The column width type is unspecified."]
        WidthTypeUnspecified,
    }
    impl TableColumnPropertiesWidthType {
        pub fn as_str(self) -> &'static str {
            match self {
                TableColumnPropertiesWidthType::EvenlyDistributed => "EVENLY_DISTRIBUTED",
                TableColumnPropertiesWidthType::FixedWidth => "FIXED_WIDTH",
                TableColumnPropertiesWidthType::WidthTypeUnspecified => "WIDTH_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for TableColumnPropertiesWidthType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for TableColumnPropertiesWidthType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<TableColumnPropertiesWidthType, ()> {
            Ok(match s {
                "EVENLY_DISTRIBUTED" => TableColumnPropertiesWidthType::EvenlyDistributed,
                "FIXED_WIDTH" => TableColumnPropertiesWidthType::FixedWidth,
                "WIDTH_TYPE_UNSPECIFIED" => TableColumnPropertiesWidthType::WidthTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for TableColumnPropertiesWidthType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableColumnPropertiesWidthType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableColumnPropertiesWidthType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EVENLY_DISTRIBUTED" => TableColumnPropertiesWidthType::EvenlyDistributed,
                "FIXED_WIDTH" => TableColumnPropertiesWidthType::FixedWidth,
                "WIDTH_TYPE_UNSPECIFIED" => TableColumnPropertiesWidthType::WidthTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for TableColumnPropertiesWidthType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableColumnPropertiesWidthType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableOfContents {
        #[doc = "The content of the table of contents."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<Vec<crate::schemas::StructuralElement>>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A TableOfContents may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TableOfContents {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableOfContents {
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
        #[doc = "The row span of the table range."]
        #[serde(
            rename = "rowSpan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_span: ::std::option::Option<i32>,
        #[doc = "The cell location where the table range starts."]
        #[serde(
            rename = "tableCellLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_location: ::std::option::Option<crate::schemas::TableCellLocation>,
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
        #[doc = "The zero-based end index of this row, exclusive, in UTF-16 code units."]
        #[serde(
            rename = "endIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The zero-based start index of this row, in UTF-16 code units."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A TableRow may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested style changes to this row, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTableRowStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_table_row_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTableRowStyle>,
        >,
        #[doc = "The contents and style of each cell in this row. It is possible for a table to be non-rectangular, so some rows may have a different number of cells than other rows in the same table."]
        #[serde(
            rename = "tableCells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cells: ::std::option::Option<Vec<crate::schemas::TableCell>>,
        #[doc = "The style of the table row."]
        #[serde(
            rename = "tableRowStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_row_style: ::std::option::Option<crate::schemas::TableRowStyle>,
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
    pub struct TableRowStyle {
        #[doc = "The minimum height of the row. The row will be rendered in the Docs editor at a height equal to or greater than this value in order to show all the content in the row's cells."]
        #[serde(
            rename = "minRowHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_row_height: ::std::option::Option<crate::schemas::Dimension>,
    }
    impl ::google_field_selector::FieldSelector for TableRowStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableRowStyle {
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
    pub struct TableRowStyleSuggestionState {
        #[doc = "Indicates if there was a suggested change to min_row_height."]
        #[serde(
            rename = "minRowHeightSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_row_height_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TableRowStyleSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableRowStyleSuggestionState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableStyle {
        #[doc = "The properties of each column. Note that in Docs, tables contain rows and rows contain cells, similar to HTML. So the properties for a row can be found on the row's table_row_style."]
        #[serde(
            rename = "tableColumnProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_column_properties:
            ::std::option::Option<Vec<crate::schemas::TableColumnProperties>>,
    }
    impl ::google_field_selector::FieldSelector for TableStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TableStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextRun {
        #[doc = "The text of this run. Any non-text elements in the run are replaced with the Unicode character U+E907."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "The suggested deletion IDs. If empty, then there are no suggested deletions of this content."]
        #[serde(
            rename = "suggestedDeletionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_deletion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested insertion IDs. A TextRun may have multiple insertion IDs if it is a nested suggested change. If empty, then this is not a suggested insertion."]
        #[serde(
            rename = "suggestedInsertionIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_insertion_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested text style changes to this run, keyed by suggestion ID."]
        #[serde(
            rename = "suggestedTextStyleChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_text_style_changes: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::SuggestedTextStyle>,
        >,
        #[doc = "The text style of this run."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
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
        #[doc = "The background color of the text. If set, the color is either an RGB color or transparent, depending on the `color` field."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::OptionalColor>,
        #[doc = "The text's vertical offset from its normal position. Text with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically rendered in a smaller font size, computed based on the `font_size` field. The `font_size` itself is not affected by changes in this field."]
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
        #[doc = "The size of the text's font."]
        #[serde(
            rename = "fontSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_size: ::std::option::Option<crate::schemas::Dimension>,
        #[doc = "The foreground color of the text. If set, the color is either an RGB color or transparent, depending on the `color` field."]
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
        #[doc = "The hyperlink destination of the text. If unset, there is no link. Links are not inherited from parent text. Changing the link in an update request causes some other changes to the text style of the range: * When setting a link, the text foreground color will be updated to the default link color and the text will be underlined. If these fields are modified in the same request, those values will be used instead of the link defaults. * Setting a link on a text range that overlaps with an existing link will also update the existing link to point to the new URL. * Links are not settable on newline characters. As a result, setting a link on a text range that crosses a paragraph boundary, such as `\"ABC\\n123\"`, will separate the newline character(s) into their own text runs. The link will be applied separately to the runs before and after the newline. * Removing a link will update the text style of the range to match the style of the preceding text (or the default text styles if the preceding text is another link) unless different styles are being set in the same request."]
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
        #[doc = "The font family and rendered weight of the text. If an update request specifies values for both `weighted_font_family` and `bold`, the `weighted_font_family` is applied first, then `bold`. If `weighted_font_family#weight` is not set, it defaults to `400`. If `weighted_font_family` is set, then `weighted_font_family#font_family` must also be set with a non-empty value. Otherwise, a 400 bad request error is returned."]
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TextStyleSuggestionState {
        #[doc = "Indicates if there was a suggested change to background_color."]
        #[serde(
            rename = "backgroundColorSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to baseline_offset."]
        #[serde(
            rename = "baselineOffsetSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub baseline_offset_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to bold."]
        #[serde(
            rename = "boldSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bold_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to font_size."]
        #[serde(
            rename = "fontSizeSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_size_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to foreground_color."]
        #[serde(
            rename = "foregroundColorSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub foreground_color_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to italic."]
        #[serde(
            rename = "italicSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub italic_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to link."]
        #[serde(
            rename = "linkSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to small_caps."]
        #[serde(
            rename = "smallCapsSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub small_caps_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to strikethrough."]
        #[serde(
            rename = "strikethroughSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub strikethrough_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to underline."]
        #[serde(
            rename = "underlineSuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub underline_suggested: ::std::option::Option<bool>,
        #[doc = "Indicates if there was a suggested change to weighted_font_family."]
        #[serde(
            rename = "weightedFontFamilySuggested",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub weighted_font_family_suggested: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for TextStyleSuggestionState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextStyleSuggestionState {
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
        #[doc = "The table range specifying which cells of the table to unmerge. All merged cells in this range will be unmerged, and cells that are already unmerged will not be affected. If the range has no merged cells, the request will do nothing. If there is text in any of the merged cells, the text will remain in the \"head\" cell of the resulting block of unmerged cells. The \"head\" cell is the upper-left cell when the content direction is from left to right, and the upper-right otherwise."]
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
    pub struct UpdateDocumentStyleRequest {
        #[doc = "The styles to set on the document. Certain document style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of DocumentStyle for more information."]
        #[serde(
            rename = "documentStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_style: ::std::option::Option<crate::schemas::DocumentStyle>,
        #[doc = "The fields that should be updated. At least one field must be specified. The root `document_style` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the background, set `fields` to `\"background\"`."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDocumentStyleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDocumentStyleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateParagraphStyleRequest {
        #[doc = "The fields that should be updated. At least one field must be specified. The root `paragraph_style` is implied and should not be specified. For example, to update the paragraph style's alignment property, set `fields` to `\"alignment\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The styles to set on the paragraphs. Certain paragraph style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of ParagraphStyle for more information."]
        #[serde(
            rename = "paragraphStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paragraph_style: ::std::option::Option<crate::schemas::ParagraphStyle>,
        #[doc = "The range overlapping the paragraphs to style."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::Range>,
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
    pub struct UpdateSectionStyleRequest {
        #[doc = "The fields that should be updated. At least one field must be specified. The root `section_style` is implied and must not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the left margin, set `fields` to `\"margin_left\"`."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The range overlapping the sections to style. Because section breaks can only be inserted inside the body, the segment ID field must be empty."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::Range>,
        #[doc = "The styles to be set on the section. Certain section style changes may cause other changes in order to mirror the behavior of the Docs editor. See the documentation of SectionStyle for more information."]
        #[serde(
            rename = "sectionStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_style: ::std::option::Option<crate::schemas::SectionStyle>,
    }
    impl ::google_field_selector::FieldSelector for UpdateSectionStyleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateSectionStyleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableCellStyleRequest {
        #[doc = "The fields that should be updated. At least one field must be specified. The root `tableCellStyle` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the table cell background color, set `fields` to `\"backgroundColor\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The style to set on the table cells. When updating borders, if a cell shares a border with an adjacent cell, the corresponding border property of the adjacent cell is updated as well. Borders that are merged and invisible are not updated. Since updating a border shared by adjacent cells in the same request can cause conflicting border updates, border updates are applied in the following order: - `border_right` - `border_left` - `border_bottom` - `border_top`"]
        #[serde(
            rename = "tableCellStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_cell_style: ::std::option::Option<crate::schemas::TableCellStyle>,
        #[doc = "The table range representing the subset of the table to which the updates are applied."]
        #[serde(
            rename = "tableRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_range: ::std::option::Option<crate::schemas::TableRange>,
        #[doc = "The location where the table starts in the document. When specified, the updates are applied to all the cells in the table."]
        #[serde(
            rename = "tableStartLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_start_location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::google_field_selector::FieldSelector for UpdateTableCellStyleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTableCellStyleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableColumnPropertiesRequest {
        #[doc = "The list of zero-based column indices whose property should be updated. If no indices are specified, all columns will be updated."]
        #[serde(
            rename = "columnIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_indices: ::std::option::Option<Vec<i32>>,
        #[doc = "The fields that should be updated. At least one field must be specified. The root `tableColumnProperties` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the column width, set `fields` to `\"width\"`."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The table column properties to update. If the value of `table_column_properties#width` is less than 5 points (5/72 inch), a 400 bad request error is returned."]
        #[serde(
            rename = "tableColumnProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_column_properties: ::std::option::Option<crate::schemas::TableColumnProperties>,
        #[doc = "The location where the table starts in the document."]
        #[serde(
            rename = "tableStartLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_start_location: ::std::option::Option<crate::schemas::Location>,
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
    pub struct UpdateTableRowStyleRequest {
        #[doc = "The fields that should be updated. At least one field must be specified. The root `tableRowStyle` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example to update the minimum row height, set `fields` to `\"min_row_height\"`."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The list of zero-based row indices whose style should be updated. If no indices are specified, all rows will be updated."]
        #[serde(
            rename = "rowIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_indices: ::std::option::Option<Vec<i32>>,
        #[doc = "The styles to be set on the rows."]
        #[serde(
            rename = "tableRowStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_row_style: ::std::option::Option<crate::schemas::TableRowStyle>,
        #[doc = "The location where the table starts in the document."]
        #[serde(
            rename = "tableStartLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_start_location: ::std::option::Option<crate::schemas::Location>,
    }
    impl ::google_field_selector::FieldSelector for UpdateTableRowStyleRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateTableRowStyleRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTextStyleRequest {
        #[doc = "The fields that should be updated. At least one field must be specified. The root `text_style` is implied and should not be specified. A single `\"*\"` can be used as short-hand for listing every field. For example, to update the text style to bold, set `fields` to `\"bold\"`. To reset a property to its default value, include its field name in the field mask but leave the field itself unset."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<String>,
        #[doc = "The range of text to style. The range may be extended to include adjacent newlines. If the range fully contains a paragraph belonging to a list, the paragraph's bullet is also updated with the matching text style. Ranges cannot be inserted inside a relative UpdateTextStyleRequest."]
        #[serde(
            rename = "range",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub range: ::std::option::Option<crate::schemas::Range>,
        #[doc = "The styles to set on the text. If the value for a particular style matches that of the parent, that style will be set to inherit. Certain text style changes may cause other changes in order to to mirror the behavior of the Docs editor. See the documentation of TextStyle for more information."]
        #[serde(
            rename = "textStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_style: ::std::option::Option<crate::schemas::TextStyle>,
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
        Debug,
        Clone,
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
        #[doc = "The font family of the text. The font family can be any font from the Font menu in Docs or from [Google Fonts] (https://fonts.google.com/). If the font name is unrecognized, the text is rendered in `Arial`."]
        #[serde(
            rename = "fontFamily",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub font_family: ::std::option::Option<String>,
        #[doc = "The weight of the font. This field can have any value that is a multiple of `100` between `100` and `900`, inclusive. This range corresponds to the numerical values described in the CSS 2.1 Specification, [section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness), with non-numerical values disallowed. The default value is `400` (\"normal\"). The font weight makes up just one component of the rendered font weight. The rendered weight is determined by a combination of the `weight` and the text style's resolved `bold` value, after accounting for inheritance: * If the text is bold and the weight is less than `400`, the rendered weight is 400. * If the text is bold and the weight is greater than or equal to `400` but is less than `700`, the rendered weight is `700`. * If the weight is greater than or equal to `700`, the rendered weight is equal to the weight. * If the text is not bold, the rendered weight is equal to the weight."]
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
    pub struct WriteControl {
        #[doc = "The revision ID of the document that the write request will be applied to. If this is not the latest revision of the document, the request will not be processed and will return a 400 bad request error. When a required revision ID is returned in a response, it indicates the revision ID of the document after the request was applied."]
        #[serde(
            rename = "requiredRevisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required_revision_id: ::std::option::Option<String>,
        #[doc = "The target revision ID of the document that the write request will be applied to. If collaborator changes have occurred after the document was read using the API, the changes produced by this write request will be transformed against the collaborator changes. This results in a new revision of the document which incorporates both the changes in the request and the collaborator changes, and the Docs server will resolve conflicting changes. When using `target_revision_id`, the API client can be thought of as another collaborator of the document. The target revision ID may only be used to write to recent versions of a document. If the target revision is too far behind the latest revision, the request will not be processed and will return a 400 bad request error and the request should be retried after reading the latest version of the document. In most cases a `revision_id` will remain valid for use as a target revision for several minutes after it is read, but for frequently-edited documents this window may be shorter."]
        #[serde(
            rename = "targetRevisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_revision_id: ::std::option::Option<String>,
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
    #[doc = "Actions that can be performed on the documents resource"]
    pub fn documents(&self) -> crate::resources::documents::DocumentsActions {
        crate::resources::documents::DocumentsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod documents {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetSuggestionsViewMode {
                #[doc = "The SuggestionsViewMode applied to the returned document depends on the user's current access level. If the user only has view access, PREVIEW_WITHOUT_SUGGESTIONS is applied. Otherwise, SUGGESTIONS_INLINE is applied. This is the default suggestions view mode."]
                DefaultForCurrentAccess,
                #[doc = "The returned document is a preview with all suggested changes accepted. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes."]
                PreviewSuggestionsAccepted,
                #[doc = "The returned document is a preview with all suggested changes rejected if there are any suggestions in the document."]
                PreviewWithoutSuggestions,
                #[doc = "The returned document has suggestions inline. Suggested changes will be differentiated from base content within the document. Requests to retrieve a document using this mode will return a 403 error if the user does not have permission to view suggested changes."]
                SuggestionsInline,
            }
            impl GetSuggestionsViewMode {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetSuggestionsViewMode::DefaultForCurrentAccess => {
                            "DEFAULT_FOR_CURRENT_ACCESS"
                        }
                        GetSuggestionsViewMode::PreviewSuggestionsAccepted => {
                            "PREVIEW_SUGGESTIONS_ACCEPTED"
                        }
                        GetSuggestionsViewMode::PreviewWithoutSuggestions => {
                            "PREVIEW_WITHOUT_SUGGESTIONS"
                        }
                        GetSuggestionsViewMode::SuggestionsInline => "SUGGESTIONS_INLINE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetSuggestionsViewMode {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetSuggestionsViewMode {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetSuggestionsViewMode, ()> {
                    Ok(match s {
                        "DEFAULT_FOR_CURRENT_ACCESS" => {
                            GetSuggestionsViewMode::DefaultForCurrentAccess
                        }
                        "PREVIEW_SUGGESTIONS_ACCEPTED" => {
                            GetSuggestionsViewMode::PreviewSuggestionsAccepted
                        }
                        "PREVIEW_WITHOUT_SUGGESTIONS" => {
                            GetSuggestionsViewMode::PreviewWithoutSuggestions
                        }
                        "SUGGESTIONS_INLINE" => GetSuggestionsViewMode::SuggestionsInline,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetSuggestionsViewMode {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetSuggestionsViewMode {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetSuggestionsViewMode {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "DEFAULT_FOR_CURRENT_ACCESS" => {
                            GetSuggestionsViewMode::DefaultForCurrentAccess
                        }
                        "PREVIEW_SUGGESTIONS_ACCEPTED" => {
                            GetSuggestionsViewMode::PreviewSuggestionsAccepted
                        }
                        "PREVIEW_WITHOUT_SUGGESTIONS" => {
                            GetSuggestionsViewMode::PreviewWithoutSuggestions
                        }
                        "SUGGESTIONS_INLINE" => GetSuggestionsViewMode::SuggestionsInline,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetSuggestionsViewMode {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetSuggestionsViewMode {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct DocumentsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DocumentsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Applies one or more updates to the document. Each request is validated before being applied. If any request is not valid, then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. Other requests do not need to return information; these each return an empty reply. The order of replies matches that of the requests. For example, suppose you call batchUpdate with four updates, and only the third one returns information. The response would have two empty replies, the reply to the third request, and another empty reply, in that order. Because other users may be editing the document, the document might not exactly reflect your changes: your changes may be altered with respect to collaborator changes. If there are no collaborators, the document should reflect your changes. In any case, the updates in your request are guaranteed to be applied together atomically."]
            pub fn batch_update(
                &self,
                request: crate::schemas::BatchUpdateDocumentRequest,
                document_id: impl Into<String>,
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
                    document_id: document_id.into(),
                }
            }
            #[doc = "Creates a blank document using the title given in the request. Other fields in the request, including any provided content, are ignored. Returns the created document."]
            pub fn create(&self, request: crate::schemas::Document) -> CreateRequestBuilder {
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
            #[doc = "Gets the latest version of the specified document."]
            pub fn get(&self, document_id: impl Into<String>) -> GetRequestBuilder {
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
                    document_id: document_id.into(),
                    suggestions_view_mode: None,
                }
            }
        }
        #[doc = "Created via [DocumentsActions::batch_update()](struct.DocumentsActions.html#method.batch_update)"]
        #[derive(Debug, Clone)]
        pub struct BatchUpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchUpdateDocumentRequest,
            document_id: String,
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
            ) -> Result<crate::schemas::BatchUpdateDocumentResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchUpdateDocumentResponse, crate::Error> {
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
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://docs.googleapis.com/".to_owned();
                output.push_str("v1/documents/");
                {
                    let var_as_str = &self.document_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":batchUpdate");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [DocumentsActions::create()](struct.DocumentsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Document,
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
            ) -> Result<crate::schemas::Document, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Document, crate::Error> {
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
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://docs.googleapis.com/".to_owned();
                output.push_str("v1/documents");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [DocumentsActions::get()](struct.DocumentsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            document_id: String,
            suggestions_view_mode:
                Option<crate::resources::documents::params::GetSuggestionsViewMode>,
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
            #[doc = "The suggestions view mode to apply to the document. This allows viewing the document with all suggestions inline, accepted or rejected. If one is not specified, DEFAULT_FOR_CURRENT_ACCESS is used."]
            pub fn suggestions_view_mode(
                mut self,
                value: crate::resources::documents::params::GetSuggestionsViewMode,
            ) -> Self {
                self.suggestions_view_mode = Some(value);
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
            ) -> Result<crate::schemas::Document, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Document, crate::Error> {
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
                let mut output = "https://docs.googleapis.com/".to_owned();
                output.push_str("v1/documents/");
                {
                    let var_as_str = &self.document_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("suggestionsViewMode", &self.suggestions_view_mode)]);
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
