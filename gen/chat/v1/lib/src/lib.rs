#![doc = "# Resources and Methods\n    * [dms](resources/dms/struct.DmsActions.html)\n      * [*messages*](resources/dms/struct.MessagesRequestBuilder.html), [*webhooks*](resources/dms/struct.WebhooksRequestBuilder.html)\n      * [conversations](resources/dms/conversations/struct.ConversationsActions.html)\n        * [*messages*](resources/dms/conversations/struct.MessagesRequestBuilder.html)\n    * [media](resources/media/struct.MediaActions.html)\n      * [*download*](resources/media/struct.DownloadRequestBuilder.html)\n    * [rooms](resources/rooms/struct.RoomsActions.html)\n      * [*messages*](resources/rooms/struct.MessagesRequestBuilder.html), [*webhooks*](resources/rooms/struct.WebhooksRequestBuilder.html)\n      * [conversations](resources/rooms/conversations/struct.ConversationsActions.html)\n        * [*messages*](resources/rooms/conversations/struct.MessagesRequestBuilder.html)\n    * [spaces](resources/spaces/struct.SpacesActions.html)\n      * [*get*](resources/spaces/struct.GetRequestBuilder.html), [*list*](resources/spaces/struct.ListRequestBuilder.html), [*webhooks*](resources/spaces/struct.WebhooksRequestBuilder.html)\n      * [members](resources/spaces/members/struct.MembersActions.html)\n        * [*get*](resources/spaces/members/struct.GetRequestBuilder.html), [*list*](resources/spaces/members/struct.ListRequestBuilder.html)\n      * [messages](resources/spaces/messages/struct.MessagesActions.html)\n        * [*create*](resources/spaces/messages/struct.CreateRequestBuilder.html), [*delete*](resources/spaces/messages/struct.DeleteRequestBuilder.html), [*get*](resources/spaces/messages/struct.GetRequestBuilder.html), [*update*](resources/spaces/messages/struct.UpdateRequestBuilder.html)\n        * [attachments](resources/spaces/messages/attachments/struct.AttachmentsActions.html)\n          * [*get*](resources/spaces/messages/attachments/struct.GetRequestBuilder.html)\n"]
pub mod scopes {}
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
    pub struct ActionParameter {
        #[doc = "The name of the parameter for the action script."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the parameter."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActionParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ActionResponse {
        #[doc = "The type of bot response."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ActionResponseType>,
        #[doc = "URL for users to auth or config. (Only for REQUEST_CONFIG response types.)"]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ActionResponseType {
        #[doc = "Post as a new message in the topic."]
        NewMessage,
        #[doc = "Privately ask the user for additional auth or config."]
        RequestConfig,
        #[doc = "Default type; will be handled as NEW_MESSAGE."]
        TypeUnspecified,
        #[doc = "Update the bot's own message. (Only after CARD_CLICKED events.)"]
        UpdateMessage,
    }
    impl ActionResponseType {
        pub fn as_str(self) -> &'static str {
            match self {
                ActionResponseType::NewMessage => "NEW_MESSAGE",
                ActionResponseType::RequestConfig => "REQUEST_CONFIG",
                ActionResponseType::TypeUnspecified => "TYPE_UNSPECIFIED",
                ActionResponseType::UpdateMessage => "UPDATE_MESSAGE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ActionResponseType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ActionResponseType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ActionResponseType, ()> {
            Ok(match s {
                "NEW_MESSAGE" => ActionResponseType::NewMessage,
                "REQUEST_CONFIG" => ActionResponseType::RequestConfig,
                "TYPE_UNSPECIFIED" => ActionResponseType::TypeUnspecified,
                "UPDATE_MESSAGE" => ActionResponseType::UpdateMessage,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ActionResponseType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ActionResponseType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ActionResponseType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NEW_MESSAGE" => ActionResponseType::NewMessage,
                "REQUEST_CONFIG" => ActionResponseType::RequestConfig,
                "TYPE_UNSPECIFIED" => ActionResponseType::TypeUnspecified,
                "UPDATE_MESSAGE" => ActionResponseType::UpdateMessage,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ActionResponseType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionResponseType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "Length of the substring in the plain-text message body this annotation corresponds to."]
        #[serde(
            rename = "length",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub length: ::std::option::Option<i32>,
        #[doc = "The type of this annotation."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AnnotationType>,
        #[doc = "The metadata for a slash command."]
        #[serde(
            rename = "slashCommand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slash_command: ::std::option::Option<crate::schemas::SlashCommandMetadata>,
        #[doc = "Start index (0-based, inclusive) in the plain-text message body this annotation corresponds to."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The metadata of user mention."]
        #[serde(
            rename = "userMention",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_mention: ::std::option::Option<crate::schemas::UserMentionMetadata>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnnotationType {
        #[doc = "Default value for the enum. DO NOT USE."]
        AnnotationTypeUnspecified,
        #[doc = "A slash command is invoked."]
        SlashCommand,
        #[doc = "A user is mentioned."]
        UserMention,
    }
    impl AnnotationType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnnotationType::AnnotationTypeUnspecified => "ANNOTATION_TYPE_UNSPECIFIED",
                AnnotationType::SlashCommand => "SLASH_COMMAND",
                AnnotationType::UserMention => "USER_MENTION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AnnotationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AnnotationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AnnotationType, ()> {
            Ok(match s {
                "ANNOTATION_TYPE_UNSPECIFIED" => AnnotationType::AnnotationTypeUnspecified,
                "SLASH_COMMAND" => AnnotationType::SlashCommand,
                "USER_MENTION" => AnnotationType::UserMention,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AnnotationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnnotationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnnotationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNOTATION_TYPE_UNSPECIFIED" => AnnotationType::AnnotationTypeUnspecified,
                "SLASH_COMMAND" => AnnotationType::SlashCommand,
                "USER_MENTION" => AnnotationType::UserMention,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AnnotationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnnotationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Attachment {
        #[doc = "A reference to the attachment data. This is used with the media API to download the attachment data."]
        #[serde(
            rename = "attachmentDataRef",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attachment_data_ref: ::std::option::Option<crate::schemas::AttachmentDataRef>,
        #[doc = "The original file name for the content, not the full path."]
        #[serde(
            rename = "contentName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_name: ::std::option::Option<String>,
        #[doc = "The content type (MIME type) of the file."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<String>,
        #[doc = "Output only. The download URL which should be used to allow a human user to download the attachment. Bots should not use this URL to download attachment content."]
        #[serde(
            rename = "downloadUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_uri: ::std::option::Option<String>,
        #[doc = "A reference to the drive attachment. This is used with the Drive API."]
        #[serde(
            rename = "driveDataRef",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_data_ref: ::std::option::Option<crate::schemas::DriveDataRef>,
        #[doc = "Resource name of the attachment, in the form \"spaces/*/messages/*/attachments/*\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The source of the attachment."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::AttachmentSource>,
        #[doc = "Output only. The thumbnail URL which should be used to preview the attachment to a human user. Bots should not use this URL to download attachment content."]
        #[serde(
            rename = "thumbnailUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Attachment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Attachment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AttachmentSource {
        DriveFile,
        SourceUnspecified,
        UploadedContent,
    }
    impl AttachmentSource {
        pub fn as_str(self) -> &'static str {
            match self {
                AttachmentSource::DriveFile => "DRIVE_FILE",
                AttachmentSource::SourceUnspecified => "SOURCE_UNSPECIFIED",
                AttachmentSource::UploadedContent => "UPLOADED_CONTENT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AttachmentSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AttachmentSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AttachmentSource, ()> {
            Ok(match s {
                "DRIVE_FILE" => AttachmentSource::DriveFile,
                "SOURCE_UNSPECIFIED" => AttachmentSource::SourceUnspecified,
                "UPLOADED_CONTENT" => AttachmentSource::UploadedContent,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AttachmentSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AttachmentSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AttachmentSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DRIVE_FILE" => AttachmentSource::DriveFile,
                "SOURCE_UNSPECIFIED" => AttachmentSource::SourceUnspecified,
                "UPLOADED_CONTENT" => AttachmentSource::UploadedContent,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AttachmentSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttachmentSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AttachmentDataRef {
        #[doc = "The resource name of the attachment data. This is used with the media API to download the attachment data."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AttachmentDataRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttachmentDataRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Button {
        #[doc = "A button with image and onclick action."]
        #[serde(
            rename = "imageButton",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_button: ::std::option::Option<crate::schemas::ImageButton>,
        #[doc = "A button with text and onclick action."]
        #[serde(
            rename = "textButton",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_button: ::std::option::Option<crate::schemas::TextButton>,
    }
    impl ::google_field_selector::FieldSelector for Button {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Button {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Card {
        #[doc = "The actions of this card."]
        #[serde(
            rename = "cardActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub card_actions: ::std::option::Option<Vec<crate::schemas::CardAction>>,
        #[doc = "The header of the card. A header usually contains a title and an image."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::CardHeader>,
        #[doc = "Name of the card."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Sections are separated by a line divider."]
        #[serde(
            rename = "sections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sections: ::std::option::Option<Vec<crate::schemas::Section>>,
    }
    impl ::google_field_selector::FieldSelector for Card {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Card {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CardAction {
        #[doc = "The label used to be displayed in the action menu item."]
        #[serde(
            rename = "actionLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_label: ::std::option::Option<String>,
        #[doc = "The onclick action for this action item."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
    }
    impl ::google_field_selector::FieldSelector for CardAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CardAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CardHeader {
        #[doc = "The image's type (e.g. square border or circular border)."]
        #[serde(
            rename = "imageStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_style: ::std::option::Option<crate::schemas::CardHeaderImageStyle>,
        #[doc = "The URL of the image in the card header."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The subtitle of the card header."]
        #[serde(
            rename = "subtitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "The title must be specified. The header has a fixed height: if both a title and subtitle is specified, each will take up 1 line. If only the title is specified, it will take up both lines."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CardHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CardHeader {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CardHeaderImageStyle {
        #[doc = "Circular border."]
        Avatar,
        #[doc = "Square border."]
        Image,
        ImageStyleUnspecified,
    }
    impl CardHeaderImageStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                CardHeaderImageStyle::Avatar => "AVATAR",
                CardHeaderImageStyle::Image => "IMAGE",
                CardHeaderImageStyle::ImageStyleUnspecified => "IMAGE_STYLE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CardHeaderImageStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CardHeaderImageStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CardHeaderImageStyle, ()> {
            Ok(match s {
                "AVATAR" => CardHeaderImageStyle::Avatar,
                "IMAGE" => CardHeaderImageStyle::Image,
                "IMAGE_STYLE_UNSPECIFIED" => CardHeaderImageStyle::ImageStyleUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CardHeaderImageStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CardHeaderImageStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CardHeaderImageStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVATAR" => CardHeaderImageStyle::Avatar,
                "IMAGE" => CardHeaderImageStyle::Image,
                "IMAGE_STYLE_UNSPECIFIED" => CardHeaderImageStyle::ImageStyleUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CardHeaderImageStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CardHeaderImageStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DeprecatedEvent {
        #[doc = "The form action data associated with an interactive card that was clicked. Only populated for CARD_CLICKED events. See the [Interactive Cards guide](/hangouts/chat/how-tos/cards-onclick) for more information."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<crate::schemas::FormAction>,
        #[doc = "The URL the bot should redirect the user to after they have completed an authorization or configuration flow outside of Google Chat. See the [Authorizing access to 3p services guide](/hangouts/chat/how-tos/auth-3p) for more information."]
        #[serde(
            rename = "configCompleteRedirectUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_complete_redirect_url: ::std::option::Option<String>,
        #[doc = "The timestamp indicating when the event was dispatched."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "The message that triggered the event, if applicable."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<crate::schemas::Message>,
        #[doc = "The type of the event."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DeprecatedEventType>,
        #[doc = "The room or DM in which the event occurred."]
        #[serde(
            rename = "space",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space: ::std::option::Option<crate::schemas::Space>,
        #[doc = "The bot-defined key for the thread related to the event. See the thread_key field of the `spaces.message.create` request for more information."]
        #[serde(
            rename = "threadKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thread_key: ::std::option::Option<String>,
        #[doc = "A secret value that bots can use to verify if a request is from Google. The token is randomly generated by Google, remains static, and can be obtained from the Google Chat API configuration page in the Cloud Console. Developers can revoke/regenerate it if needed from the same page."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
        #[doc = "The user that triggered the event."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for DeprecatedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeprecatedEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeprecatedEventType {
        #[doc = "The bot was added to a room or DM."]
        AddedToSpace,
        #[doc = "The bot's interactive card was clicked."]
        CardClicked,
        #[doc = "A message was sent in a room or direct message."]
        Message,
        #[doc = "The bot was removed from a room or DM."]
        RemovedFromSpace,
        #[doc = "Default value for the enum. DO NOT USE."]
        Unspecified,
    }
    impl DeprecatedEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeprecatedEventType::AddedToSpace => "ADDED_TO_SPACE",
                DeprecatedEventType::CardClicked => "CARD_CLICKED",
                DeprecatedEventType::Message => "MESSAGE",
                DeprecatedEventType::RemovedFromSpace => "REMOVED_FROM_SPACE",
                DeprecatedEventType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeprecatedEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeprecatedEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeprecatedEventType, ()> {
            Ok(match s {
                "ADDED_TO_SPACE" => DeprecatedEventType::AddedToSpace,
                "CARD_CLICKED" => DeprecatedEventType::CardClicked,
                "MESSAGE" => DeprecatedEventType::Message,
                "REMOVED_FROM_SPACE" => DeprecatedEventType::RemovedFromSpace,
                "UNSPECIFIED" => DeprecatedEventType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeprecatedEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeprecatedEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeprecatedEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDED_TO_SPACE" => DeprecatedEventType::AddedToSpace,
                "CARD_CLICKED" => DeprecatedEventType::CardClicked,
                "MESSAGE" => DeprecatedEventType::Message,
                "REMOVED_FROM_SPACE" => DeprecatedEventType::RemovedFromSpace,
                "UNSPECIFIED" => DeprecatedEventType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeprecatedEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeprecatedEventType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveDataRef {
        #[doc = "The id for the drive file, for use with the Drive API."]
        #[serde(
            rename = "driveFileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_file_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DriveDataRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveDataRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct FormAction {
        #[doc = "The method name is used to identify which part of the form triggered the form submission. This information is echoed back to the bot as part of the card click event. The same method name can be used for several elements that trigger a common behavior if desired."]
        #[serde(
            rename = "actionMethodName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_method_name: ::std::option::Option<String>,
        #[doc = "List of action parameters."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<crate::schemas::ActionParameter>>,
    }
    impl ::google_field_selector::FieldSelector for FormAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FormAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Image {
        #[doc = "The aspect ratio of this image (width/height). This field allows clients to reserve the right height for the image while waiting for it to load. It's not meant to override the native aspect ratio of the image. If unset, the server fills it by prefetching the image."]
        #[serde(
            rename = "aspectRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aspect_ratio: ::std::option::Option<f64>,
        #[doc = "The URL of the image."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The onclick action."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImageButton {
        #[doc = "The icon specified by an enum that indices to an icon provided by Chat API."]
        #[serde(
            rename = "icon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon: ::std::option::Option<crate::schemas::ImageButtonIcon>,
        #[doc = "The icon specified by a URL."]
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_url: ::std::option::Option<String>,
        #[doc = "The name of this image_button which will be used for accessibility. Default value will be provided if developers don't specify."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The onclick action."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
    }
    impl ::google_field_selector::FieldSelector for ImageButton {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageButton {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageButtonIcon {
        Airplane,
        Bookmark,
        Bus,
        Car,
        Clock,
        ConfirmationNumberIcon,
        Description,
        Dollar,
        Email,
        EventPerformer,
        EventSeat,
        FlightArrival,
        FlightDeparture,
        Hotel,
        HotelRoomType,
        IconUnspecified,
        Invite,
        MapPin,
        Membership,
        MultiplePeople,
        Offer,
        Person,
        Phone,
        RestaurantIcon,
        ShoppingCart,
        Star,
        Store,
        Ticket,
        Train,
        VideoCamera,
        VideoPlay,
    }
    impl ImageButtonIcon {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageButtonIcon::Airplane => "AIRPLANE",
                ImageButtonIcon::Bookmark => "BOOKMARK",
                ImageButtonIcon::Bus => "BUS",
                ImageButtonIcon::Car => "CAR",
                ImageButtonIcon::Clock => "CLOCK",
                ImageButtonIcon::ConfirmationNumberIcon => "CONFIRMATION_NUMBER_ICON",
                ImageButtonIcon::Description => "DESCRIPTION",
                ImageButtonIcon::Dollar => "DOLLAR",
                ImageButtonIcon::Email => "EMAIL",
                ImageButtonIcon::EventPerformer => "EVENT_PERFORMER",
                ImageButtonIcon::EventSeat => "EVENT_SEAT",
                ImageButtonIcon::FlightArrival => "FLIGHT_ARRIVAL",
                ImageButtonIcon::FlightDeparture => "FLIGHT_DEPARTURE",
                ImageButtonIcon::Hotel => "HOTEL",
                ImageButtonIcon::HotelRoomType => "HOTEL_ROOM_TYPE",
                ImageButtonIcon::IconUnspecified => "ICON_UNSPECIFIED",
                ImageButtonIcon::Invite => "INVITE",
                ImageButtonIcon::MapPin => "MAP_PIN",
                ImageButtonIcon::Membership => "MEMBERSHIP",
                ImageButtonIcon::MultiplePeople => "MULTIPLE_PEOPLE",
                ImageButtonIcon::Offer => "OFFER",
                ImageButtonIcon::Person => "PERSON",
                ImageButtonIcon::Phone => "PHONE",
                ImageButtonIcon::RestaurantIcon => "RESTAURANT_ICON",
                ImageButtonIcon::ShoppingCart => "SHOPPING_CART",
                ImageButtonIcon::Star => "STAR",
                ImageButtonIcon::Store => "STORE",
                ImageButtonIcon::Ticket => "TICKET",
                ImageButtonIcon::Train => "TRAIN",
                ImageButtonIcon::VideoCamera => "VIDEO_CAMERA",
                ImageButtonIcon::VideoPlay => "VIDEO_PLAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImageButtonIcon {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImageButtonIcon {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImageButtonIcon, ()> {
            Ok(match s {
                "AIRPLANE" => ImageButtonIcon::Airplane,
                "BOOKMARK" => ImageButtonIcon::Bookmark,
                "BUS" => ImageButtonIcon::Bus,
                "CAR" => ImageButtonIcon::Car,
                "CLOCK" => ImageButtonIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => ImageButtonIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => ImageButtonIcon::Description,
                "DOLLAR" => ImageButtonIcon::Dollar,
                "EMAIL" => ImageButtonIcon::Email,
                "EVENT_PERFORMER" => ImageButtonIcon::EventPerformer,
                "EVENT_SEAT" => ImageButtonIcon::EventSeat,
                "FLIGHT_ARRIVAL" => ImageButtonIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => ImageButtonIcon::FlightDeparture,
                "HOTEL" => ImageButtonIcon::Hotel,
                "HOTEL_ROOM_TYPE" => ImageButtonIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => ImageButtonIcon::IconUnspecified,
                "INVITE" => ImageButtonIcon::Invite,
                "MAP_PIN" => ImageButtonIcon::MapPin,
                "MEMBERSHIP" => ImageButtonIcon::Membership,
                "MULTIPLE_PEOPLE" => ImageButtonIcon::MultiplePeople,
                "OFFER" => ImageButtonIcon::Offer,
                "PERSON" => ImageButtonIcon::Person,
                "PHONE" => ImageButtonIcon::Phone,
                "RESTAURANT_ICON" => ImageButtonIcon::RestaurantIcon,
                "SHOPPING_CART" => ImageButtonIcon::ShoppingCart,
                "STAR" => ImageButtonIcon::Star,
                "STORE" => ImageButtonIcon::Store,
                "TICKET" => ImageButtonIcon::Ticket,
                "TRAIN" => ImageButtonIcon::Train,
                "VIDEO_CAMERA" => ImageButtonIcon::VideoCamera,
                "VIDEO_PLAY" => ImageButtonIcon::VideoPlay,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImageButtonIcon {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageButtonIcon {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageButtonIcon {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AIRPLANE" => ImageButtonIcon::Airplane,
                "BOOKMARK" => ImageButtonIcon::Bookmark,
                "BUS" => ImageButtonIcon::Bus,
                "CAR" => ImageButtonIcon::Car,
                "CLOCK" => ImageButtonIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => ImageButtonIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => ImageButtonIcon::Description,
                "DOLLAR" => ImageButtonIcon::Dollar,
                "EMAIL" => ImageButtonIcon::Email,
                "EVENT_PERFORMER" => ImageButtonIcon::EventPerformer,
                "EVENT_SEAT" => ImageButtonIcon::EventSeat,
                "FLIGHT_ARRIVAL" => ImageButtonIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => ImageButtonIcon::FlightDeparture,
                "HOTEL" => ImageButtonIcon::Hotel,
                "HOTEL_ROOM_TYPE" => ImageButtonIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => ImageButtonIcon::IconUnspecified,
                "INVITE" => ImageButtonIcon::Invite,
                "MAP_PIN" => ImageButtonIcon::MapPin,
                "MEMBERSHIP" => ImageButtonIcon::Membership,
                "MULTIPLE_PEOPLE" => ImageButtonIcon::MultiplePeople,
                "OFFER" => ImageButtonIcon::Offer,
                "PERSON" => ImageButtonIcon::Person,
                "PHONE" => ImageButtonIcon::Phone,
                "RESTAURANT_ICON" => ImageButtonIcon::RestaurantIcon,
                "SHOPPING_CART" => ImageButtonIcon::ShoppingCart,
                "STAR" => ImageButtonIcon::Star,
                "STORE" => ImageButtonIcon::Store,
                "TICKET" => ImageButtonIcon::Ticket,
                "TRAIN" => ImageButtonIcon::Train,
                "VIDEO_CAMERA" => ImageButtonIcon::VideoCamera,
                "VIDEO_PLAY" => ImageButtonIcon::VideoPlay,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImageButtonIcon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageButtonIcon {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct KeyValue {
        #[doc = "The text of the bottom label. Formatted text supported."]
        #[serde(
            rename = "bottomLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom_label: ::std::option::Option<String>,
        #[doc = "A button that can be clicked to trigger an action."]
        #[serde(
            rename = "button",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub button: ::std::option::Option<crate::schemas::Button>,
        #[doc = "The text of the content. Formatted text supported and always required."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "If the content should be multiline."]
        #[serde(
            rename = "contentMultiline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_multiline: ::std::option::Option<bool>,
        #[doc = "An enum value that will be replaced by the Chat API with the corresponding icon image."]
        #[serde(
            rename = "icon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon: ::std::option::Option<crate::schemas::KeyValueIcon>,
        #[doc = "The icon specified by a URL."]
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_url: ::std::option::Option<String>,
        #[doc = "The onclick action. Only the top label, bottom label and content region are clickable."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
        #[doc = "The text of the top label. Formatted text supported."]
        #[serde(
            rename = "topLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for KeyValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KeyValueIcon {
        Airplane,
        Bookmark,
        Bus,
        Car,
        Clock,
        ConfirmationNumberIcon,
        Description,
        Dollar,
        Email,
        EventPerformer,
        EventSeat,
        FlightArrival,
        FlightDeparture,
        Hotel,
        HotelRoomType,
        IconUnspecified,
        Invite,
        MapPin,
        Membership,
        MultiplePeople,
        Offer,
        Person,
        Phone,
        RestaurantIcon,
        ShoppingCart,
        Star,
        Store,
        Ticket,
        Train,
        VideoCamera,
        VideoPlay,
    }
    impl KeyValueIcon {
        pub fn as_str(self) -> &'static str {
            match self {
                KeyValueIcon::Airplane => "AIRPLANE",
                KeyValueIcon::Bookmark => "BOOKMARK",
                KeyValueIcon::Bus => "BUS",
                KeyValueIcon::Car => "CAR",
                KeyValueIcon::Clock => "CLOCK",
                KeyValueIcon::ConfirmationNumberIcon => "CONFIRMATION_NUMBER_ICON",
                KeyValueIcon::Description => "DESCRIPTION",
                KeyValueIcon::Dollar => "DOLLAR",
                KeyValueIcon::Email => "EMAIL",
                KeyValueIcon::EventPerformer => "EVENT_PERFORMER",
                KeyValueIcon::EventSeat => "EVENT_SEAT",
                KeyValueIcon::FlightArrival => "FLIGHT_ARRIVAL",
                KeyValueIcon::FlightDeparture => "FLIGHT_DEPARTURE",
                KeyValueIcon::Hotel => "HOTEL",
                KeyValueIcon::HotelRoomType => "HOTEL_ROOM_TYPE",
                KeyValueIcon::IconUnspecified => "ICON_UNSPECIFIED",
                KeyValueIcon::Invite => "INVITE",
                KeyValueIcon::MapPin => "MAP_PIN",
                KeyValueIcon::Membership => "MEMBERSHIP",
                KeyValueIcon::MultiplePeople => "MULTIPLE_PEOPLE",
                KeyValueIcon::Offer => "OFFER",
                KeyValueIcon::Person => "PERSON",
                KeyValueIcon::Phone => "PHONE",
                KeyValueIcon::RestaurantIcon => "RESTAURANT_ICON",
                KeyValueIcon::ShoppingCart => "SHOPPING_CART",
                KeyValueIcon::Star => "STAR",
                KeyValueIcon::Store => "STORE",
                KeyValueIcon::Ticket => "TICKET",
                KeyValueIcon::Train => "TRAIN",
                KeyValueIcon::VideoCamera => "VIDEO_CAMERA",
                KeyValueIcon::VideoPlay => "VIDEO_PLAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for KeyValueIcon {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KeyValueIcon {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KeyValueIcon, ()> {
            Ok(match s {
                "AIRPLANE" => KeyValueIcon::Airplane,
                "BOOKMARK" => KeyValueIcon::Bookmark,
                "BUS" => KeyValueIcon::Bus,
                "CAR" => KeyValueIcon::Car,
                "CLOCK" => KeyValueIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => KeyValueIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => KeyValueIcon::Description,
                "DOLLAR" => KeyValueIcon::Dollar,
                "EMAIL" => KeyValueIcon::Email,
                "EVENT_PERFORMER" => KeyValueIcon::EventPerformer,
                "EVENT_SEAT" => KeyValueIcon::EventSeat,
                "FLIGHT_ARRIVAL" => KeyValueIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => KeyValueIcon::FlightDeparture,
                "HOTEL" => KeyValueIcon::Hotel,
                "HOTEL_ROOM_TYPE" => KeyValueIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => KeyValueIcon::IconUnspecified,
                "INVITE" => KeyValueIcon::Invite,
                "MAP_PIN" => KeyValueIcon::MapPin,
                "MEMBERSHIP" => KeyValueIcon::Membership,
                "MULTIPLE_PEOPLE" => KeyValueIcon::MultiplePeople,
                "OFFER" => KeyValueIcon::Offer,
                "PERSON" => KeyValueIcon::Person,
                "PHONE" => KeyValueIcon::Phone,
                "RESTAURANT_ICON" => KeyValueIcon::RestaurantIcon,
                "SHOPPING_CART" => KeyValueIcon::ShoppingCart,
                "STAR" => KeyValueIcon::Star,
                "STORE" => KeyValueIcon::Store,
                "TICKET" => KeyValueIcon::Ticket,
                "TRAIN" => KeyValueIcon::Train,
                "VIDEO_CAMERA" => KeyValueIcon::VideoCamera,
                "VIDEO_PLAY" => KeyValueIcon::VideoPlay,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KeyValueIcon {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KeyValueIcon {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KeyValueIcon {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AIRPLANE" => KeyValueIcon::Airplane,
                "BOOKMARK" => KeyValueIcon::Bookmark,
                "BUS" => KeyValueIcon::Bus,
                "CAR" => KeyValueIcon::Car,
                "CLOCK" => KeyValueIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => KeyValueIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => KeyValueIcon::Description,
                "DOLLAR" => KeyValueIcon::Dollar,
                "EMAIL" => KeyValueIcon::Email,
                "EVENT_PERFORMER" => KeyValueIcon::EventPerformer,
                "EVENT_SEAT" => KeyValueIcon::EventSeat,
                "FLIGHT_ARRIVAL" => KeyValueIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => KeyValueIcon::FlightDeparture,
                "HOTEL" => KeyValueIcon::Hotel,
                "HOTEL_ROOM_TYPE" => KeyValueIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => KeyValueIcon::IconUnspecified,
                "INVITE" => KeyValueIcon::Invite,
                "MAP_PIN" => KeyValueIcon::MapPin,
                "MEMBERSHIP" => KeyValueIcon::Membership,
                "MULTIPLE_PEOPLE" => KeyValueIcon::MultiplePeople,
                "OFFER" => KeyValueIcon::Offer,
                "PERSON" => KeyValueIcon::Person,
                "PHONE" => KeyValueIcon::Phone,
                "RESTAURANT_ICON" => KeyValueIcon::RestaurantIcon,
                "SHOPPING_CART" => KeyValueIcon::ShoppingCart,
                "STAR" => KeyValueIcon::Star,
                "STORE" => KeyValueIcon::Store,
                "TICKET" => KeyValueIcon::Ticket,
                "TRAIN" => KeyValueIcon::Train,
                "VIDEO_CAMERA" => KeyValueIcon::VideoCamera,
                "VIDEO_PLAY" => KeyValueIcon::VideoPlay,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for KeyValueIcon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyValueIcon {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListMembershipsResponse {
        #[doc = "List of memberships in the requested (or first) page."]
        #[serde(
            rename = "memberships",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memberships: ::std::option::Option<Vec<crate::schemas::Membership>>,
        #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListMembershipsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListMembershipsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListSpacesResponse {
        #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results. Tokens expire in an hour. An error is thrown if an expired token is passed."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of spaces in the requested (or first) page."]
        #[serde(
            rename = "spaces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spaces: ::std::option::Option<Vec<crate::schemas::Space>>,
    }
    impl ::google_field_selector::FieldSelector for ListSpacesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSpacesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Media {
        #[doc = "Name of the media resource."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Media {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Media {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Membership {
        #[doc = "The creation time of the membership a.k.a the time at which the member joined the space, if applicable."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A User in Hangout Chat"]
        #[serde(
            rename = "member",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub member: ::std::option::Option<crate::schemas::User>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "State of the membership."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::MembershipState>,
    }
    impl ::google_field_selector::FieldSelector for Membership {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Membership {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MembershipState {
        #[doc = "The user has been invited, is able to join the space, but currently has not joined."]
        Invited,
        #[doc = "The user has joined the space."]
        Joined,
        #[doc = "Default, do not use."]
        MembershipStateUnspecified,
        #[doc = "The user is not a member of the space, has not been invited and is not able to join the space."]
        NotAMember,
    }
    impl MembershipState {
        pub fn as_str(self) -> &'static str {
            match self {
                MembershipState::Invited => "INVITED",
                MembershipState::Joined => "JOINED",
                MembershipState::MembershipStateUnspecified => "MEMBERSHIP_STATE_UNSPECIFIED",
                MembershipState::NotAMember => "NOT_A_MEMBER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MembershipState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MembershipState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MembershipState, ()> {
            Ok(match s {
                "INVITED" => MembershipState::Invited,
                "JOINED" => MembershipState::Joined,
                "MEMBERSHIP_STATE_UNSPECIFIED" => MembershipState::MembershipStateUnspecified,
                "NOT_A_MEMBER" => MembershipState::NotAMember,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MembershipState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MembershipState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MembershipState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVITED" => MembershipState::Invited,
                "JOINED" => MembershipState::Joined,
                "MEMBERSHIP_STATE_UNSPECIFIED" => MembershipState::MembershipStateUnspecified,
                "NOT_A_MEMBER" => MembershipState::NotAMember,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MembershipState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Message {
        #[doc = "Input only. Parameters that a bot can use to configure how its response is posted."]
        #[serde(
            rename = "actionResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_response: ::std::option::Option<crate::schemas::ActionResponse>,
        #[doc = "Output only. Annotations associated with the text in this message."]
        #[serde(
            rename = "annotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotations: ::std::option::Option<Vec<crate::schemas::Annotation>>,
        #[doc = "Plain-text body of the message with all bot mentions stripped out."]
        #[serde(
            rename = "argumentText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub argument_text: ::std::option::Option<String>,
        #[doc = "User uploaded attachment."]
        #[serde(
            rename = "attachment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attachment: ::std::option::Option<Vec<crate::schemas::Attachment>>,
        #[doc = "Rich, formatted and interactive cards that can be used to display UI elements such as: formatted texts, buttons, clickable images. Cards are normally displayed below the plain-text body of the message."]
        #[serde(
            rename = "cards",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cards: ::std::option::Option<Vec<crate::schemas::Card>>,
        #[doc = "Output only. The time at which the message was created in Hangouts Chat server."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A plain-text description of the message's cards, used when the actual cards cannot be displayed (e.g. mobile notifications)."]
        #[serde(
            rename = "fallbackText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fallback_text: ::std::option::Option<String>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Text for generating preview chips. This text will not be displayed to the user, but any links to images, web pages, videos, etc. included here will generate preview chips."]
        #[serde(
            rename = "previewText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preview_text: ::std::option::Option<String>,
        #[doc = "The user who created the message."]
        #[serde(
            rename = "sender",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sender: ::std::option::Option<crate::schemas::User>,
        #[doc = "Slash command information, if applicable."]
        #[serde(
            rename = "slashCommand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slash_command: ::std::option::Option<crate::schemas::SlashCommand>,
        #[doc = "The space the message belongs to."]
        #[serde(
            rename = "space",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space: ::std::option::Option<crate::schemas::Space>,
        #[doc = "Plain-text body of the message."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "The thread the message belongs to."]
        #[serde(
            rename = "thread",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thread: ::std::option::Option<crate::schemas::Thread>,
    }
    impl ::google_field_selector::FieldSelector for Message {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Message {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OnClick {
        #[doc = "A form action will be triggered by this onclick if specified."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<crate::schemas::FormAction>,
        #[doc = "This onclick triggers an open link action if specified."]
        #[serde(
            rename = "openLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_link: ::std::option::Option<crate::schemas::OpenLink>,
    }
    impl ::google_field_selector::FieldSelector for OnClick {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OnClick {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OpenLink {
        #[doc = "The URL to open."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OpenLink {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OpenLink {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Section {
        #[doc = "The header of the section, text formatted supported."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<String>,
        #[doc = "A section must contain at least 1 widget."]
        #[serde(
            rename = "widgets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub widgets: ::std::option::Option<Vec<crate::schemas::WidgetMarkup>>,
    }
    impl ::google_field_selector::FieldSelector for Section {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Section {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SlashCommand {
        #[doc = "The id of the slash command invoked."]
        #[serde(
            rename = "commandId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub command_id: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SlashCommand {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlashCommand {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SlashCommandMetadata {
        #[doc = "The bot whose command was invoked."]
        #[serde(
            rename = "bot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bot: ::std::option::Option<crate::schemas::User>,
        #[doc = "The command id of the invoked slash command."]
        #[serde(
            rename = "commandId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub command_id: ::std::option::Option<i64>,
        #[doc = "The name of the invoked slash command."]
        #[serde(
            rename = "commandName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub command_name: ::std::option::Option<String>,
        #[doc = "The type of slash command."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SlashCommandMetadataType>,
        #[doc = "Indicating whether the slash command is for a dialog."]
        #[serde(
            rename = "triggersDialog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub triggers_dialog: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SlashCommandMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlashCommandMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SlashCommandMetadataType {
        #[doc = "Add bot to space."]
        Add,
        #[doc = "Invoke slash command in space."]
        Invoke,
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
    }
    impl SlashCommandMetadataType {
        pub fn as_str(self) -> &'static str {
            match self {
                SlashCommandMetadataType::Add => "ADD",
                SlashCommandMetadataType::Invoke => "INVOKE",
                SlashCommandMetadataType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SlashCommandMetadataType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SlashCommandMetadataType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SlashCommandMetadataType, ()> {
            Ok(match s {
                "ADD" => SlashCommandMetadataType::Add,
                "INVOKE" => SlashCommandMetadataType::Invoke,
                "TYPE_UNSPECIFIED" => SlashCommandMetadataType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SlashCommandMetadataType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SlashCommandMetadataType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SlashCommandMetadataType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => SlashCommandMetadataType::Add,
                "INVOKE" => SlashCommandMetadataType::Invoke,
                "TYPE_UNSPECIFIED" => SlashCommandMetadataType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SlashCommandMetadataType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlashCommandMetadataType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Space {
        #[doc = "Output only. The display name (only if the space is a room). Please note that this field might not be populated in direct messages between humans."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name of the space, in the form \"spaces/*\". Example: spaces/AAAAMpdlehYs"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The type of a space. This is deprecated. Use `single_user_bot_dm` instead."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SpaceType>,
        #[doc = "Whether the space is a DM between a bot and a single human."]
        #[serde(
            rename = "singleUserBotDm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub single_user_bot_dm: ::std::option::Option<bool>,
        #[doc = "Whether the messages are threaded in this space."]
        #[serde(
            rename = "threaded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threaded: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Space {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Space {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SpaceType {
        #[doc = "1:1 Direct Message between a human and a bot, where all messages are flat."]
        Dm,
        #[doc = "Multi-user spaces such as rooms and DMs between humans."]
        Room,
        TypeUnspecified,
    }
    impl SpaceType {
        pub fn as_str(self) -> &'static str {
            match self {
                SpaceType::Dm => "DM",
                SpaceType::Room => "ROOM",
                SpaceType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SpaceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SpaceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SpaceType, ()> {
            Ok(match s {
                "DM" => SpaceType::Dm,
                "ROOM" => SpaceType::Room,
                "TYPE_UNSPECIFIED" => SpaceType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SpaceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SpaceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SpaceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DM" => SpaceType::Dm,
                "ROOM" => SpaceType::Room,
                "TYPE_UNSPECIFIED" => SpaceType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SpaceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpaceType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TextButton {
        #[doc = "The onclick action of the button."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
        #[doc = "The text of the button."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TextButton {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextButton {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TextParagraph {
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TextParagraph {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextParagraph {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Thread {
        #[doc = "Resource name, in the form \"spaces/*/threads/*\". Example: spaces/AAAAMpdlehY/threads/UMxbHmzDlr4"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Thread {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Thread {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct User {
        #[doc = "The user's display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Obfuscated domain information."]
        #[serde(
            rename = "domainId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_id: ::std::option::Option<String>,
        #[doc = "True when the user is deleted or the user's proifle is not visible."]
        #[serde(
            rename = "isAnonymous",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_anonymous: ::std::option::Option<bool>,
        #[doc = "Resource name, in the format \"users/*\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "User type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::UserType>,
    }
    impl ::google_field_selector::FieldSelector for User {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for User {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UserType {
        #[doc = "Bot user."]
        Bot,
        #[doc = "Human user."]
        Human,
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
    }
    impl UserType {
        pub fn as_str(self) -> &'static str {
            match self {
                UserType::Bot => "BOT",
                UserType::Human => "HUMAN",
                UserType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UserType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UserType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UserType, ()> {
            Ok(match s {
                "BOT" => UserType::Bot,
                "HUMAN" => UserType::Human,
                "TYPE_UNSPECIFIED" => UserType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UserType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOT" => UserType::Bot,
                "HUMAN" => UserType::Human,
                "TYPE_UNSPECIFIED" => UserType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UserType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserMentionMetadata {
        #[doc = "The type of user mention."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::UserMentionMetadataType>,
        #[doc = "The user mentioned."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for UserMentionMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserMentionMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UserMentionMetadataType {
        #[doc = "Add user to space."]
        Add,
        #[doc = "Mention user in space."]
        Mention,
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
    }
    impl UserMentionMetadataType {
        pub fn as_str(self) -> &'static str {
            match self {
                UserMentionMetadataType::Add => "ADD",
                UserMentionMetadataType::Mention => "MENTION",
                UserMentionMetadataType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UserMentionMetadataType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UserMentionMetadataType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UserMentionMetadataType, ()> {
            Ok(match s {
                "ADD" => UserMentionMetadataType::Add,
                "MENTION" => UserMentionMetadataType::Mention,
                "TYPE_UNSPECIFIED" => UserMentionMetadataType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UserMentionMetadataType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UserMentionMetadataType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserMentionMetadataType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => UserMentionMetadataType::Add,
                "MENTION" => UserMentionMetadataType::Mention,
                "TYPE_UNSPECIFIED" => UserMentionMetadataType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UserMentionMetadataType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserMentionMetadataType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WidgetMarkup {
        #[doc = "A list of buttons. Buttons is also oneof data and only one of these fields should be set."]
        #[serde(
            rename = "buttons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buttons: ::std::option::Option<Vec<crate::schemas::Button>>,
        #[doc = "Display an image in this widget."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::Image>,
        #[doc = "Display a key value item in this widget."]
        #[serde(
            rename = "keyValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_value: ::std::option::Option<crate::schemas::KeyValue>,
        #[doc = "Display a text paragraph in this widget."]
        #[serde(
            rename = "textParagraph",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_paragraph: ::std::option::Option<crate::schemas::TextParagraph>,
    }
    impl ::google_field_selector::FieldSelector for WidgetMarkup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WidgetMarkup {
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
    #[doc = "Actions that can be performed on the dms resource"]
    pub fn dms(&self) -> crate::resources::dms::DmsActions {
        crate::resources::dms::DmsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the media resource"]
    pub fn media(&self) -> crate::resources::media::MediaActions {
        crate::resources::media::MediaActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the rooms resource"]
    pub fn rooms(&self) -> crate::resources::rooms::RoomsActions {
        crate::resources::rooms::RoomsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the spaces resource"]
    pub fn spaces(&self) -> crate::resources::spaces::SpacesActions {
        crate::resources::spaces::SpacesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod dms {
        pub mod params {}
        pub struct DmsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DmsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn messages(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> MessagesRequestBuilder {
                MessagesRequestBuilder {
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
                    parent: parent.into(),
                    thread_key: None,
                }
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn webhooks(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> WebhooksRequestBuilder {
                WebhooksRequestBuilder {
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
                    parent: parent.into(),
                    thread_key: None,
                }
            }
            #[doc = "Actions that can be performed on the conversations resource"]
            pub fn conversations(
                &self,
            ) -> crate::resources::dms::conversations::ConversationsActions {
                crate::resources::dms::conversations::ConversationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [DmsActions::messages()](struct.DmsActions.html#method.messages)"]
        #[derive(Debug, Clone)]
        pub struct MessagesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            thread_key: Option<String>,
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
        impl<'a> MessagesRequestBuilder<'a> {
            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/messages");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        #[doc = "Created via [DmsActions::webhooks()](struct.DmsActions.html#method.webhooks)"]
        #[derive(Debug, Clone)]
        pub struct WebhooksRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            thread_key: Option<String>,
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
        impl<'a> WebhooksRequestBuilder<'a> {
            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/webhooks");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        pub mod conversations {
            pub mod params {}
            pub struct ConversationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConversationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
                pub fn messages(
                    &self,
                    request: crate::schemas::Message,
                    parent: impl Into<String>,
                ) -> MessagesRequestBuilder {
                    MessagesRequestBuilder {
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
                        parent: parent.into(),
                        thread_key: None,
                    }
                }
            }
            #[doc = "Created via [ConversationsActions::messages()](struct.ConversationsActions.html#method.messages)"]
            #[derive(Debug, Clone)]
            pub struct MessagesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
                parent: String,
                thread_key: Option<String>,
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
            impl<'a> MessagesRequestBuilder<'a> {
                #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                    self.thread_key = Some(value.into());
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/messages");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("threadKey", &self.thread_key)]);
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
    pub mod media {
        pub mod params {}
        pub struct MediaActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> MediaActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Downloads media. Download is supported on the URI `/v1/media/{+name}?alt=media`."]
            pub fn download(&self, resource_name: impl Into<String>) -> DownloadRequestBuilder {
                DownloadRequestBuilder {
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
                    resource_name: resource_name.into(),
                }
            }
        }
        #[doc = "Created via [MediaActions::download()](struct.MediaActions.html#method.download)"]
        #[derive(Debug, Clone)]
        pub struct DownloadRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            resource_name: String,
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
        impl<'a> DownloadRequestBuilder<'a> {
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
            fn _download_path(&self) -> String {
                let mut output = "https://chat.googleapis.com/download/".to_owned();
                output.push_str("v1/media/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            pub async fn download<W>(mut self, output: &mut W) -> Result<u64, crate::Error>
            where
                W: futures::io::AsyncWrite + std::marker::Unpin + ?Sized,
            {
                use futures::io::AsyncWriteExt;
                self.alt = Some(crate::params::Alt::Media);
                let request = self._request(&self._path()).await?;
                let mut response = request.send().await?.error_for_status()?;
                let mut num_bytes_written: usize = 0;
                while let Some(chunk) = response.chunk().await? {
                    output.write(&chunk).await?;
                    num_bytes_written += chunk.len();
                }
                Ok(num_bytes_written as u64)
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
            ) -> Result<crate::schemas::Media, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Media, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/media/");
                {
                    let var_as_str = &self.resource_name;
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
    }
    pub mod rooms {
        pub mod params {}
        pub struct RoomsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> RoomsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn messages(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> MessagesRequestBuilder {
                MessagesRequestBuilder {
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
                    parent: parent.into(),
                    thread_key: None,
                }
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn webhooks(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> WebhooksRequestBuilder {
                WebhooksRequestBuilder {
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
                    parent: parent.into(),
                    thread_key: None,
                }
            }
            #[doc = "Actions that can be performed on the conversations resource"]
            pub fn conversations(
                &self,
            ) -> crate::resources::rooms::conversations::ConversationsActions {
                crate::resources::rooms::conversations::ConversationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [RoomsActions::messages()](struct.RoomsActions.html#method.messages)"]
        #[derive(Debug, Clone)]
        pub struct MessagesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            thread_key: Option<String>,
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
        impl<'a> MessagesRequestBuilder<'a> {
            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/messages");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        #[doc = "Created via [RoomsActions::webhooks()](struct.RoomsActions.html#method.webhooks)"]
        #[derive(Debug, Clone)]
        pub struct WebhooksRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            thread_key: Option<String>,
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
        impl<'a> WebhooksRequestBuilder<'a> {
            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/webhooks");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        pub mod conversations {
            pub mod params {}
            pub struct ConversationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConversationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
                pub fn messages(
                    &self,
                    request: crate::schemas::Message,
                    parent: impl Into<String>,
                ) -> MessagesRequestBuilder {
                    MessagesRequestBuilder {
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
                        parent: parent.into(),
                        thread_key: None,
                    }
                }
            }
            #[doc = "Created via [ConversationsActions::messages()](struct.ConversationsActions.html#method.messages)"]
            #[derive(Debug, Clone)]
            pub struct MessagesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
                parent: String,
                thread_key: Option<String>,
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
            impl<'a> MessagesRequestBuilder<'a> {
                #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                    self.thread_key = Some(value.into());
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/messages");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("threadKey", &self.thread_key)]);
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
    pub mod spaces {
        pub mod params {}
        pub struct SpacesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SpacesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns a space."]
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
            #[doc = "Lists spaces the caller is a member of."]
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
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn webhooks(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> WebhooksRequestBuilder {
                WebhooksRequestBuilder {
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
                    parent: parent.into(),
                    thread_key: None,
                }
            }
            #[doc = "Actions that can be performed on the members resource"]
            pub fn members(&self) -> crate::resources::spaces::members::MembersActions {
                crate::resources::spaces::members::MembersActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the messages resource"]
            pub fn messages(&self) -> crate::resources::spaces::messages::MessagesActions {
                crate::resources::spaces::messages::MessagesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [SpacesActions::get()](struct.SpacesActions.html#method.get)"]
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
            ) -> Result<crate::schemas::Space, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Space, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
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
        #[doc = "Created via [SpacesActions::list()](struct.SpacesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            #[doc = "Requested page size. The value is capped at 1000. Server may return fewer results than requested. If unspecified, server will default to 100."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results the server should return."]
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
            ) -> Result<crate::schemas::ListSpacesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListSpacesResponse, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/spaces");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [SpacesActions::webhooks()](struct.SpacesActions.html#method.webhooks)"]
        #[derive(Debug, Clone)]
        pub struct WebhooksRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            thread_key: Option<String>,
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
        impl<'a> WebhooksRequestBuilder<'a> {
            #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/webhooks");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        pub mod members {
            pub mod params {}
            pub struct MembersActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> MembersActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns a membership."]
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
                #[doc = "Lists human memberships in a space."]
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
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [MembersActions::get()](struct.MembersActions.html#method.get)"]
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
                ) -> Result<crate::schemas::Membership, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Membership, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
            #[doc = "Created via [MembersActions::list()](struct.MembersActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
                #[doc = "Requested page size. The value is capped at 1000. Server may return fewer results than requested. If unspecified, server will default to 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return."]
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
                ) -> Result<crate::schemas::ListMembershipsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListMembershipsResponse, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/members");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        pub mod messages {
            pub mod params {}
            pub struct MessagesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> MessagesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a message."]
                pub fn create(
                    &self,
                    request: crate::schemas::Message,
                    parent: impl Into<String>,
                ) -> CreateRequestBuilder {
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
                        parent: parent.into(),
                        thread_key: None,
                    }
                }
                #[doc = "Deletes a message."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                    DeleteRequestBuilder {
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
                #[doc = "Returns a message."]
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
                #[doc = "Updates a message."]
                pub fn update(
                    &self,
                    request: crate::schemas::Message,
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
                        update_mask: None,
                    }
                }
                #[doc = "Actions that can be performed on the attachments resource"]
                pub fn attachments(
                    &self,
                ) -> crate::resources::spaces::messages::attachments::AttachmentsActions
                {
                    crate::resources::spaces::messages::attachments::AttachmentsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [MessagesActions::create()](struct.MessagesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
                parent: String,
                thread_key: Option<String>,
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
                #[doc = "Opaque thread identifier string that can be specified to group messages into a single thread. If this is the first message with a given thread identifier, a new thread is created. Subsequent messages with the same thread identifier will be posted into the same thread. This relieves bots and webhooks from having to store the Hangouts Chat thread ID of a thread (created earlier by them) to post further updates to it. Has no effect if thread field, corresponding to an existing thread, is set in message."]
                pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                    self.thread_key = Some(value.into());
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/messages");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("threadKey", &self.thread_key)]);
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
            #[doc = "Created via [MessagesActions::delete()](struct.MessagesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
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
            impl<'a> DeleteRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
            #[doc = "Created via [MessagesActions::get()](struct.MessagesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
            #[doc = "Created via [MessagesActions::update()](struct.MessagesActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
                name: String,
                update_mask: Option<String>,
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
                #[doc = "Required. The field paths to be updated, comma separated if there are multiple. Currently supported field paths: * text * cards"]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
                    let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
                    req = req.query(&[("updateMask", &self.update_mask)]);
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
            pub mod attachments {
                pub mod params {}
                pub struct AttachmentsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> AttachmentsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets the metadata of a message attachment. The attachment data is fetched using the media API."]
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
                #[doc = "Created via [AttachmentsActions::get()](struct.AttachmentsActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::Attachment, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Attachment, crate::Error> {
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
                        let mut output = "https://chat.googleapis.com/".to_owned();
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
