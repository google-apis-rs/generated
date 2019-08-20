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
    pub struct AboutDriveThemesItems {
        #[doc = "A link to this theme's background image."]
        #[serde(rename = "backgroundImageLink", default)]
        pub background_image_link: Option<String>,
        #[doc = "The color of this theme as an RGB hex string."]
        #[serde(rename = "colorRgb", default)]
        pub color_rgb: Option<String>,
        #[doc = "The ID of the theme."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
    }
    impl ::field_selector::FieldSelector for AboutDriveThemesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AboutStorageQuota {
        #[doc = "The usage limit, if applicable. This will not be present if the user has unlimited storage."]
        #[serde(rename = "limit", default)]
        #[serde(with = "crate::parsed_string")]
        pub limit: Option<i64>,
        #[doc = "The total usage across all services."]
        #[serde(rename = "usage", default)]
        #[serde(with = "crate::parsed_string")]
        pub usage: Option<i64>,
        #[doc = "The usage by all files in Google Drive."]
        #[serde(rename = "usageInDrive", default)]
        #[serde(with = "crate::parsed_string")]
        pub usage_in_drive: Option<i64>,
        #[doc = "The usage by trashed files in Google Drive."]
        #[serde(rename = "usageInDriveTrash", default)]
        #[serde(with = "crate::parsed_string")]
        pub usage_in_drive_trash: Option<i64>,
    }
    impl ::field_selector::FieldSelector for AboutStorageQuota {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AboutTeamDriveThemesItems {
        #[doc = "Deprecated - use driveThemes/backgroundImageLink instead."]
        #[serde(rename = "backgroundImageLink", default)]
        pub background_image_link: Option<String>,
        #[doc = "Deprecated - use driveThemes/colorRgb instead."]
        #[serde(rename = "colorRgb", default)]
        pub color_rgb: Option<String>,
        #[doc = "Deprecated - use driveThemes/id instead."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
    }
    impl ::field_selector::FieldSelector for AboutTeamDriveThemesItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct About {
        #[doc = "Whether the user has installed the requesting app."]
        #[serde(rename = "appInstalled", default)]
        pub app_installed: Option<bool>,
        #[doc = "Whether the user can create shared drives."]
        #[serde(rename = "canCreateDrives", default)]
        pub can_create_drives: Option<bool>,
        #[doc = "Deprecated - use canCreateDrives instead."]
        #[serde(rename = "canCreateTeamDrives", default)]
        pub can_create_team_drives: Option<bool>,
        #[doc = "A list of themes that are supported for shared drives."]
        #[serde(rename = "driveThemes", default)]
        pub drive_themes: Option<Vec<crate::schemas::AboutDriveThemesItems>>,
        #[doc = "A map of source MIME type to possible targets for all supported exports."]
        #[serde(rename = "exportFormats", default)]
        pub export_formats: Option<::std::collections::BTreeMap<String, Vec<String>>>,
        #[doc = "The currently supported folder colors as RGB hex strings."]
        #[serde(rename = "folderColorPalette", default)]
        pub folder_color_palette: Option<Vec<String>>,
        #[doc = "A map of source MIME type to possible targets for all supported imports."]
        #[serde(rename = "importFormats", default)]
        pub import_formats: Option<::std::collections::BTreeMap<String, Vec<String>>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#about\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "A map of maximum import sizes by MIME type, in bytes."]
        #[serde(rename = "maxImportSizes", default)]
        pub max_import_sizes: Option<::std::collections::BTreeMap<String, i64>>,
        #[doc = "The maximum upload size in bytes."]
        #[serde(rename = "maxUploadSize", default)]
        #[serde(with = "crate::parsed_string")]
        pub max_upload_size: Option<i64>,
        #[doc = "The user's storage quota limits and usage. All fields are measured in bytes."]
        #[serde(rename = "storageQuota", default)]
        pub storage_quota: Option<crate::schemas::AboutStorageQuota>,
        #[doc = "Deprecated - use driveThemes instead."]
        #[serde(rename = "teamDriveThemes", default)]
        pub team_drive_themes: Option<Vec<crate::schemas::AboutTeamDriveThemesItems>>,
        #[doc = "The authenticated user."]
        #[serde(rename = "user", default)]
        pub user: Option<crate::schemas::User>,
    }
    impl ::field_selector::FieldSelector for About {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Change {
        #[doc = "The type of the change. Possible values are file and drive."]
        #[serde(rename = "changeType", default)]
        pub change_type: Option<String>,
        #[doc = "The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted."]
        #[serde(rename = "drive", default)]
        pub drive: Option<crate::schemas::Drive>,
        #[doc = "The ID of the shared drive associated with this change."]
        #[serde(rename = "driveId", default)]
        pub drive_id: Option<String>,
        #[doc = "The updated state of the file. Present if the type is file and the file has not been removed from this list of changes."]
        #[serde(rename = "file", default)]
        pub file: Option<crate::schemas::File>,
        #[doc = "The ID of the file which has changed."]
        #[serde(rename = "fileId", default)]
        pub file_id: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#change\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Deprecated - use changeType instead."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access."]
        #[serde(rename = "removed", default)]
        pub removed: Option<bool>,
        #[doc = "Deprecated - use drive instead."]
        #[serde(rename = "teamDrive", default)]
        pub team_drive: Option<crate::schemas::TeamDrive>,
        #[doc = "Deprecated - use driveId instead."]
        #[serde(rename = "teamDriveId", default)]
        pub team_drive_id: Option<String>,
        #[doc = "The time of this change (RFC 3339 date-time)."]
        #[serde(rename = "time", default)]
        pub time: Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for Change {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ChangeList {
        #[doc = "The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "changes", default)]
        pub changes: Option<Vec<crate::schemas::Change>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#changeList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The starting page token for future changes. This will be present only if the end of the current changes list has been reached."]
        #[serde(rename = "newStartPageToken", default)]
        pub new_start_page_token: Option<String>,
        #[doc = "The page token for the next page of changes. This will be absent if the end of the changes list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ChangeList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Channel {
        #[doc = "The address where notifications are delivered for this channel."]
        #[serde(rename = "address", default)]
        pub address: Option<String>,
        #[doc = "Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional."]
        #[serde(rename = "expiration", default)]
        #[serde(with = "crate::parsed_string")]
        pub expiration: Option<i64>,
        #[doc = "A UUID or similar unique string that identifies this channel."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Identifies this as a notification channel used to watch for changes to a resource, which is \"api#channel\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Additional parameters controlling delivery channel behavior. Optional."]
        #[serde(rename = "params", default)]
        pub params: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A Boolean value to indicate whether payload is wanted. Optional."]
        #[serde(rename = "payload", default)]
        pub payload: Option<bool>,
        #[doc = "The type of delivery mechanism used for this channel."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "An opaque ID that identifies the resource being watched on this channel. Stable across different API versions."]
        #[serde(rename = "resourceId", default)]
        pub resource_id: Option<String>,
        #[doc = "A version-specific identifier for the watched resource."]
        #[serde(rename = "resourceUri", default)]
        pub resource_uri: Option<String>,
        #[doc = "An arbitrary string delivered to the target address with each notification delivered over this channel. Optional."]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
    }
    impl ::field_selector::FieldSelector for Channel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CommentQuotedFileContent {
        #[doc = "The MIME type of the quoted content."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
        #[doc = "The quoted content itself. This is interpreted as plain text if set through the API."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for CommentQuotedFileContent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Comment {
        #[doc = "A region of the document represented as a JSON string. See anchor documentation for details on how to define and interpret anchor properties."]
        #[serde(rename = "anchor", default)]
        pub anchor: Option<String>,
        #[doc = "The user who created the comment."]
        #[serde(rename = "author", default)]
        pub author: Option<crate::schemas::User>,
        #[doc = "The plain text content of the comment. This field is used for setting the content, while htmlContent should be displayed."]
        #[serde(rename = "content", default)]
        pub content: Option<String>,
        #[doc = "The time at which the comment was created (RFC 3339 date-time)."]
        #[serde(rename = "createdTime", default)]
        pub created_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Whether the comment has been deleted. A deleted comment has no content."]
        #[serde(rename = "deleted", default)]
        pub deleted: Option<bool>,
        #[doc = "The content of the comment with HTML formatting."]
        #[serde(rename = "htmlContent", default)]
        pub html_content: Option<String>,
        #[doc = "The ID of the comment."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#comment\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The last time the comment or any of its replies was modified (RFC 3339 date-time)."]
        #[serde(rename = "modifiedTime", default)]
        pub modified_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment."]
        #[serde(rename = "quotedFileContent", default)]
        pub quoted_file_content: Option<crate::schemas::CommentQuotedFileContent>,
        #[doc = "The full list of replies to the comment in chronological order."]
        #[serde(rename = "replies", default)]
        pub replies: Option<Vec<crate::schemas::Reply>>,
        #[doc = "Whether the comment has been resolved by one of its replies."]
        #[serde(rename = "resolved", default)]
        pub resolved: Option<bool>,
    }
    impl ::field_selector::FieldSelector for Comment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CommentList {
        #[doc = "The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "comments", default)]
        pub comments: Option<Vec<crate::schemas::Comment>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#commentList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The page token for the next page of comments. This will be absent if the end of the comments list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for CommentList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DriveBackgroundImageFile {
        #[doc = "The ID of an image file in Google Drive to use for the background image."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high."]
        #[serde(rename = "width", default)]
        pub width: Option<f32>,
        #[doc = "The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image."]
        #[serde(rename = "xCoordinate", default)]
        pub x_coordinate: Option<f32>,
        #[doc = "The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image."]
        #[serde(rename = "yCoordinate", default)]
        pub y_coordinate: Option<f32>,
    }
    impl ::field_selector::FieldSelector for DriveBackgroundImageFile {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveCapabilities {
        #[doc = "Whether the current user can add children to folders in this shared drive."]
        #[serde(rename = "canAddChildren", default)]
        pub can_add_children: Option<bool>,
        #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this shared drive."]
        #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction", default)]
        pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
        #[doc = "Whether the current user can change the domainUsersOnly restriction of this shared drive."]
        #[serde(rename = "canChangeDomainUsersOnlyRestriction", default)]
        pub can_change_domain_users_only_restriction: Option<bool>,
        #[doc = "Whether the current user can change the background of this shared drive."]
        #[serde(rename = "canChangeDriveBackground", default)]
        pub can_change_drive_background: Option<bool>,
        #[doc = "Whether the current user can change the driveMembersOnly restriction of this shared drive."]
        #[serde(rename = "canChangeDriveMembersOnlyRestriction", default)]
        pub can_change_drive_members_only_restriction: Option<bool>,
        #[doc = "Whether the current user can comment on files in this shared drive."]
        #[serde(rename = "canComment", default)]
        pub can_comment: Option<bool>,
        #[doc = "Whether the current user can copy files in this shared drive."]
        #[serde(rename = "canCopy", default)]
        pub can_copy: Option<bool>,
        #[doc = "Whether the current user can delete children from folders in this shared drive."]
        #[serde(rename = "canDeleteChildren", default)]
        pub can_delete_children: Option<bool>,
        #[doc = "Whether the current user can delete this shared drive. Attempting to delete the shared drive may still fail if there are untrashed items inside the shared drive."]
        #[serde(rename = "canDeleteDrive", default)]
        pub can_delete_drive: Option<bool>,
        #[doc = "Whether the current user can download files in this shared drive."]
        #[serde(rename = "canDownload", default)]
        pub can_download: Option<bool>,
        #[doc = "Whether the current user can edit files in this shared drive"]
        #[serde(rename = "canEdit", default)]
        pub can_edit: Option<bool>,
        #[doc = "Whether the current user can list the children of folders in this shared drive."]
        #[serde(rename = "canListChildren", default)]
        pub can_list_children: Option<bool>,
        #[doc = "Whether the current user can add members to this shared drive or remove them or change their role."]
        #[serde(rename = "canManageMembers", default)]
        pub can_manage_members: Option<bool>,
        #[doc = "Whether the current user can read the revisions resource of files in this shared drive."]
        #[serde(rename = "canReadRevisions", default)]
        pub can_read_revisions: Option<bool>,
        #[doc = "Whether the current user can rename files or folders in this shared drive."]
        #[serde(rename = "canRename", default)]
        pub can_rename: Option<bool>,
        #[doc = "Whether the current user can rename this shared drive."]
        #[serde(rename = "canRenameDrive", default)]
        pub can_rename_drive: Option<bool>,
        #[doc = "Whether the current user can share files or folders in this shared drive."]
        #[serde(rename = "canShare", default)]
        pub can_share: Option<bool>,
        #[doc = "Whether the current user can trash children from folders in this shared drive."]
        #[serde(rename = "canTrashChildren", default)]
        pub can_trash_children: Option<bool>,
    }
    impl ::field_selector::FieldSelector for DriveCapabilities {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveRestrictions {
        #[doc = "Whether administrative privileges on this shared drive are required to modify restrictions."]
        #[serde(rename = "adminManagedRestrictions", default)]
        pub admin_managed_restrictions: Option<bool>,
        #[doc = "Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this shared drive."]
        #[serde(rename = "copyRequiresWriterPermission", default)]
        pub copy_requires_writer_permission: Option<bool>,
        #[doc = "Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive."]
        #[serde(rename = "domainUsersOnly", default)]
        pub domain_users_only: Option<bool>,
        #[doc = "Whether access to items inside this shared drive is restricted to its members."]
        #[serde(rename = "driveMembersOnly", default)]
        pub drive_members_only: Option<bool>,
    }
    impl ::field_selector::FieldSelector for DriveRestrictions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Drive {
        #[doc = "An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on drive.drives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
        #[serde(rename = "backgroundImageFile", default)]
        pub background_image_file: Option<crate::schemas::DriveBackgroundImageFile>,
        #[doc = "A short-lived link to this shared drive's background image."]
        #[serde(rename = "backgroundImageLink", default)]
        pub background_image_link: Option<String>,
        #[doc = "Capabilities the current user has on this shared drive."]
        #[serde(rename = "capabilities", default)]
        pub capabilities: Option<crate::schemas::DriveCapabilities>,
        #[doc = "The color of this shared drive as an RGB hex string. It can only be set on a drive.drives.update request that does not set themeId."]
        #[serde(rename = "colorRgb", default)]
        pub color_rgb: Option<String>,
        #[doc = "The time at which the shared drive was created (RFC 3339 date-time)."]
        #[serde(rename = "createdTime", default)]
        pub created_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Whether the shared drive is hidden from default view."]
        #[serde(rename = "hidden", default)]
        pub hidden: Option<bool>,
        #[doc = "The ID of this shared drive which is also the ID of the top level folder of this shared drive."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#drive\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The name of this shared drive."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "A set of restrictions that apply to this shared drive or items inside this shared drive."]
        #[serde(rename = "restrictions", default)]
        pub restrictions: Option<crate::schemas::DriveRestrictions>,
        #[doc = "The ID of the theme from which the background image and color will be set. The set of possible driveThemes can be retrieved from a drive.about.get response. When not specified on a drive.drives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
        #[serde(rename = "themeId", default)]
        pub theme_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Drive {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DriveList {
        #[doc = "The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "drives", default)]
        pub drives: Option<Vec<crate::schemas::Drive>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#driveList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The page token for the next page of shared drives. This will be absent if the end of the list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for DriveList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FileCapabilities {
        #[doc = "Whether the current user can add children to this folder. This is always false when the item is not a folder."]
        #[serde(rename = "canAddChildren", default)]
        pub can_add_children: Option<bool>,
        #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this file."]
        #[serde(rename = "canChangeCopyRequiresWriterPermission", default)]
        pub can_change_copy_requires_writer_permission: Option<bool>,
        #[doc = "Deprecated"]
        #[serde(rename = "canChangeViewersCanCopyContent", default)]
        pub can_change_viewers_can_copy_content: Option<bool>,
        #[doc = "Whether the current user can comment on this file."]
        #[serde(rename = "canComment", default)]
        pub can_comment: Option<bool>,
        #[doc = "Whether the current user can copy this file. For an item in a shared drive, whether the current user can copy non-folder descendants of this item, or this item itself if it is not a folder."]
        #[serde(rename = "canCopy", default)]
        pub can_copy: Option<bool>,
        #[doc = "Whether the current user can delete this file."]
        #[serde(rename = "canDelete", default)]
        pub can_delete: Option<bool>,
        #[doc = "Whether the current user can delete children of this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
        #[serde(rename = "canDeleteChildren", default)]
        pub can_delete_children: Option<bool>,
        #[doc = "Whether the current user can download this file."]
        #[serde(rename = "canDownload", default)]
        pub can_download: Option<bool>,
        #[doc = "Whether the current user can edit this file."]
        #[serde(rename = "canEdit", default)]
        pub can_edit: Option<bool>,
        #[doc = "Whether the current user can list the children of this folder. This is always false when the item is not a folder."]
        #[serde(rename = "canListChildren", default)]
        pub can_list_children: Option<bool>,
        #[doc = "Whether the current user can move children of this folder outside of the shared drive. This is false when the item is not a folder. Only populated for items in shared drives."]
        #[serde(rename = "canMoveChildrenOutOfDrive", default)]
        pub can_move_children_out_of_drive: Option<bool>,
        #[doc = "Deprecated - use canMoveChildrenOutOfDrive instead."]
        #[serde(rename = "canMoveChildrenOutOfTeamDrive", default)]
        pub can_move_children_out_of_team_drive: Option<bool>,
        #[doc = "Whether the current user can move children of this folder within the shared drive. This is false when the item is not a folder. Only populated for items in shared drives."]
        #[serde(rename = "canMoveChildrenWithinDrive", default)]
        pub can_move_children_within_drive: Option<bool>,
        #[doc = "Deprecated - use canMoveChildrenWithinDrive instead."]
        #[serde(rename = "canMoveChildrenWithinTeamDrive", default)]
        pub can_move_children_within_team_drive: Option<bool>,
        #[doc = "Deprecated - use canMoveItemOutOfDrive instead."]
        #[serde(rename = "canMoveItemIntoTeamDrive", default)]
        pub can_move_item_into_team_drive: Option<bool>,
        #[doc = "Whether the current user can move this item outside of this drive by changing its parent. Note that a request to change the parent of the item may still fail depending on the new parent that is being added."]
        #[serde(rename = "canMoveItemOutOfDrive", default)]
        pub can_move_item_out_of_drive: Option<bool>,
        #[doc = "Deprecated - use canMoveItemOutOfDrive instead."]
        #[serde(rename = "canMoveItemOutOfTeamDrive", default)]
        pub can_move_item_out_of_team_drive: Option<bool>,
        #[doc = "Whether the current user can move this item within this shared drive. Note that a request to change the parent of the item may still fail depending on the new parent that is being added. Only populated for items in shared drives."]
        #[serde(rename = "canMoveItemWithinDrive", default)]
        pub can_move_item_within_drive: Option<bool>,
        #[doc = "Deprecated - use canMoveItemWithinDrive instead."]
        #[serde(rename = "canMoveItemWithinTeamDrive", default)]
        pub can_move_item_within_team_drive: Option<bool>,
        #[doc = "Deprecated - use canMoveItemWithinDrive or canMoveItemOutOfDrive instead."]
        #[serde(rename = "canMoveTeamDriveItem", default)]
        pub can_move_team_drive_item: Option<bool>,
        #[doc = "Whether the current user can read the shared drive to which this file belongs. Only populated for items in shared drives."]
        #[serde(rename = "canReadDrive", default)]
        pub can_read_drive: Option<bool>,
        #[doc = "Whether the current user can read the revisions resource of this file. For a shared drive item, whether revisions of non-folder descendants of this item, or this item itself if it is not a folder, can be read."]
        #[serde(rename = "canReadRevisions", default)]
        pub can_read_revisions: Option<bool>,
        #[doc = "Deprecated - use canReadDrive instead."]
        #[serde(rename = "canReadTeamDrive", default)]
        pub can_read_team_drive: Option<bool>,
        #[doc = "Whether the current user can remove children from this folder. This is always false when the item is not a folder. For a folder in a shared drive, use canDeleteChildren or canTrashChildren instead."]
        #[serde(rename = "canRemoveChildren", default)]
        pub can_remove_children: Option<bool>,
        #[doc = "Whether the current user can rename this file."]
        #[serde(rename = "canRename", default)]
        pub can_rename: Option<bool>,
        #[doc = "Whether the current user can modify the sharing settings for this file."]
        #[serde(rename = "canShare", default)]
        pub can_share: Option<bool>,
        #[doc = "Whether the current user can move this file to trash."]
        #[serde(rename = "canTrash", default)]
        pub can_trash: Option<bool>,
        #[doc = "Whether the current user can trash children of this folder. This is false when the item is not a folder. Only populated for items in shared drives."]
        #[serde(rename = "canTrashChildren", default)]
        pub can_trash_children: Option<bool>,
        #[doc = "Whether the current user can restore this file from trash."]
        #[serde(rename = "canUntrash", default)]
        pub can_untrash: Option<bool>,
    }
    impl ::field_selector::FieldSelector for FileCapabilities {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FileContentHintsThumbnail {
        #[doc = "The thumbnail data encoded with URL-safe Base64 (RFC 4648 section 5)."]
        #[serde(rename = "image", default)]
        pub image: Option<Vec<u8>>,
        #[doc = "The MIME type of the thumbnail."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for FileContentHintsThumbnail {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FileContentHints {
        #[doc = "Text to be indexed for the file to improve fullText queries. This is limited to 128KB in length and may contain HTML elements."]
        #[serde(rename = "indexableText", default)]
        pub indexable_text: Option<String>,
        #[doc = "A thumbnail for the file. This will only be used if Google Drive cannot generate a standard thumbnail."]
        #[serde(rename = "thumbnail", default)]
        pub thumbnail: Option<crate::schemas::FileContentHintsThumbnail>,
    }
    impl ::field_selector::FieldSelector for FileContentHints {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FileImageMediaMetadataLocation {
        #[doc = "The altitude stored in the image."]
        #[serde(rename = "altitude", default)]
        pub altitude: Option<f64>,
        #[doc = "The latitude stored in the image."]
        #[serde(rename = "latitude", default)]
        pub latitude: Option<f64>,
        #[doc = "The longitude stored in the image."]
        #[serde(rename = "longitude", default)]
        pub longitude: Option<f64>,
    }
    impl ::field_selector::FieldSelector for FileImageMediaMetadataLocation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FileImageMediaMetadata {
        #[doc = "The aperture used to create the photo (f-number)."]
        #[serde(rename = "aperture", default)]
        pub aperture: Option<f32>,
        #[doc = "The make of the camera used to create the photo."]
        #[serde(rename = "cameraMake", default)]
        pub camera_make: Option<String>,
        #[doc = "The model of the camera used to create the photo."]
        #[serde(rename = "cameraModel", default)]
        pub camera_model: Option<String>,
        #[doc = "The color space of the photo."]
        #[serde(rename = "colorSpace", default)]
        pub color_space: Option<String>,
        #[doc = "The exposure bias of the photo (APEX value)."]
        #[serde(rename = "exposureBias", default)]
        pub exposure_bias: Option<f32>,
        #[doc = "The exposure mode used to create the photo."]
        #[serde(rename = "exposureMode", default)]
        pub exposure_mode: Option<String>,
        #[doc = "The length of the exposure, in seconds."]
        #[serde(rename = "exposureTime", default)]
        pub exposure_time: Option<f32>,
        #[doc = "Whether a flash was used to create the photo."]
        #[serde(rename = "flashUsed", default)]
        pub flash_used: Option<bool>,
        #[doc = "The focal length used to create the photo, in millimeters."]
        #[serde(rename = "focalLength", default)]
        pub focal_length: Option<f32>,
        #[doc = "The height of the image in pixels."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The ISO speed used to create the photo."]
        #[serde(rename = "isoSpeed", default)]
        pub iso_speed: Option<i32>,
        #[doc = "The lens used to create the photo."]
        #[serde(rename = "lens", default)]
        pub lens: Option<String>,
        #[doc = "Geographic location information stored in the image."]
        #[serde(rename = "location", default)]
        pub location: Option<crate::schemas::FileImageMediaMetadataLocation>,
        #[doc = "The smallest f-number of the lens at the focal length used to create the photo (APEX value)."]
        #[serde(rename = "maxApertureValue", default)]
        pub max_aperture_value: Option<f32>,
        #[doc = "The metering mode used to create the photo."]
        #[serde(rename = "meteringMode", default)]
        pub metering_mode: Option<String>,
        #[doc = "The rotation in clockwise degrees from the image's original orientation."]
        #[serde(rename = "rotation", default)]
        pub rotation: Option<i32>,
        #[doc = "The type of sensor used to create the photo."]
        #[serde(rename = "sensor", default)]
        pub sensor: Option<String>,
        #[doc = "The distance to the subject of the photo, in meters."]
        #[serde(rename = "subjectDistance", default)]
        pub subject_distance: Option<i32>,
        #[doc = "The date and time the photo was taken (EXIF DateTime)."]
        #[serde(rename = "time", default)]
        pub time: Option<String>,
        #[doc = "The white balance mode used to create the photo."]
        #[serde(rename = "whiteBalance", default)]
        pub white_balance: Option<String>,
        #[doc = "The width of the image in pixels."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for FileImageMediaMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FileVideoMediaMetadata {
        #[doc = "The duration of the video in milliseconds."]
        #[serde(rename = "durationMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub duration_millis: Option<i64>,
        #[doc = "The height of the video in pixels."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The width of the video in pixels."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for FileVideoMediaMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct File {
        #[doc = "A collection of arbitrary key-value pairs which are private to the requesting app.\nEntries with null values are cleared in update and copy requests."]
        #[serde(rename = "appProperties", default)]
        pub app_properties: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take."]
        #[serde(rename = "capabilities", default)]
        pub capabilities: Option<crate::schemas::FileCapabilities>,
        #[doc = "Additional information about the content of the file. These fields are never populated in responses."]
        #[serde(rename = "contentHints", default)]
        pub content_hints: Option<crate::schemas::FileContentHints>,
        #[doc = "Whether the options to copy, print, or download this file, should be disabled for readers and commenters."]
        #[serde(rename = "copyRequiresWriterPermission", default)]
        pub copy_requires_writer_permission: Option<bool>,
        #[doc = "The time at which the file was created (RFC 3339 date-time)."]
        #[serde(rename = "createdTime", default)]
        pub created_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "A short description of the file."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "ID of the shared drive the file resides in. Only populated for items in shared drives."]
        #[serde(rename = "driveId", default)]
        pub drive_id: Option<String>,
        #[doc = "Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder."]
        #[serde(rename = "explicitlyTrashed", default)]
        pub explicitly_trashed: Option<bool>,
        #[doc = "Links for exporting Google Docs to specific formats."]
        #[serde(rename = "exportLinks", default)]
        pub export_links: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The final component of fullFileExtension. This is only available for files with binary content in Google Drive."]
        #[serde(rename = "fileExtension", default)]
        pub file_extension: Option<String>,
        #[doc = "The color for a folder as an RGB hex string. The supported colors are published in the folderColorPalette field of the About resource.\nIf an unsupported color is specified, the closest color in the palette will be used instead."]
        #[serde(rename = "folderColorRgb", default)]
        pub folder_color_rgb: Option<String>,
        #[doc = "The full file extension extracted from the name field. May contain multiple concatenated extensions, such as \"tar.gz\". This is only available for files with binary content in Google Drive.\nThis is automatically updated when the name field changes, however it is not cleared if the new name does not contain a valid extension."]
        #[serde(rename = "fullFileExtension", default)]
        pub full_file_extension: Option<String>,
        #[doc = "Whether any users are granted file access directly on this file. This field is only populated for shared drive files."]
        #[serde(rename = "hasAugmentedPermissions", default)]
        pub has_augmented_permissions: Option<bool>,
        #[doc = "Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field."]
        #[serde(rename = "hasThumbnail", default)]
        pub has_thumbnail: Option<bool>,
        #[doc = "The ID of the file's head revision. This is currently only available for files with binary content in Google Drive."]
        #[serde(rename = "headRevisionId", default)]
        pub head_revision_id: Option<String>,
        #[doc = "A static, unauthenticated link to the file's icon."]
        #[serde(rename = "iconLink", default)]
        pub icon_link: Option<String>,
        #[doc = "The ID of the file."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Additional metadata about image media, if available."]
        #[serde(rename = "imageMediaMetadata", default)]
        pub image_media_metadata: Option<crate::schemas::FileImageMediaMetadata>,
        #[doc = "Whether the file was created or opened by the requesting app."]
        #[serde(rename = "isAppAuthorized", default)]
        pub is_app_authorized: Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#file\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The last user to modify the file."]
        #[serde(rename = "lastModifyingUser", default)]
        pub last_modifying_user: Option<crate::schemas::User>,
        #[doc = "The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive."]
        #[serde(rename = "md5Checksum", default)]
        pub md_5_checksum: Option<String>,
        #[doc = "The MIME type of the file.\nGoogle Drive will attempt to automatically detect an appropriate value from uploaded content if no value is provided. The value cannot be changed unless a new revision is uploaded.\nIf a file is created with a Google Doc MIME type, the uploaded content will be imported if possible. The supported import formats are published in the About resource."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
        #[doc = "Whether the file has been modified by this user."]
        #[serde(rename = "modifiedByMe", default)]
        pub modified_by_me: Option<bool>,
        #[doc = "The last time the file was modified by the user (RFC 3339 date-time)."]
        #[serde(rename = "modifiedByMeTime", default)]
        pub modified_by_me_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The last time the file was modified by anyone (RFC 3339 date-time).\nNote that setting modifiedTime will also update modifiedByMeTime for the user."]
        #[serde(rename = "modifiedTime", default)]
        pub modified_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The name of the file. This is not necessarily unique within a folder. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the name is constant."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The original filename of the uploaded content if available, or else the original value of the name field. This is only available for files with binary content in Google Drive."]
        #[serde(rename = "originalFilename", default)]
        pub original_filename: Option<String>,
        #[doc = "Whether the user owns the file. Not populated for items in shared drives."]
        #[serde(rename = "ownedByMe", default)]
        pub owned_by_me: Option<bool>,
        #[doc = "The owners of the file. Currently, only certain legacy files may have more than one owner. Not populated for items in shared drives."]
        #[serde(rename = "owners", default)]
        pub owners: Option<Vec<crate::schemas::User>>,
        #[doc = "The IDs of the parent folders which contain the file.\nIf not specified as part of a create request, the file will be placed directly in the user's My Drive folder. If not specified as part of a copy request, the file will inherit any discoverable parents of the source file. Update requests must use the addParents and removeParents parameters to modify the parents list."]
        #[serde(rename = "parents", default)]
        pub parents: Option<Vec<String>>,
        #[doc = "List of permission IDs for users with access to this file."]
        #[serde(rename = "permissionIds", default)]
        pub permission_ids: Option<Vec<String>>,
        #[doc = "The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives."]
        #[serde(rename = "permissions", default)]
        pub permissions: Option<Vec<crate::schemas::Permission>>,
        #[doc = "A collection of arbitrary key-value pairs which are visible to all apps.\nEntries with null values are cleared in update and copy requests."]
        #[serde(rename = "properties", default)]
        pub properties: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with keepForever enabled."]
        #[serde(rename = "quotaBytesUsed", default)]
        #[serde(with = "crate::parsed_string")]
        pub quota_bytes_used: Option<i64>,
        #[doc = "Whether the file has been shared. Not populated for items in shared drives."]
        #[serde(rename = "shared", default)]
        pub shared: Option<bool>,
        #[doc = "The time at which the file was shared with the user, if applicable (RFC 3339 date-time)."]
        #[serde(rename = "sharedWithMeTime", default)]
        pub shared_with_me_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The user who shared the file with the requesting user, if applicable."]
        #[serde(rename = "sharingUser", default)]
        pub sharing_user: Option<crate::schemas::User>,
        #[doc = "The size of the file's content in bytes. This is only applicable to files with binary content in Google Drive."]
        #[serde(rename = "size", default)]
        #[serde(with = "crate::parsed_string")]
        pub size: Option<i64>,
        #[doc = "The list of spaces which contain the file. The currently supported values are 'drive', 'appDataFolder' and 'photos'."]
        #[serde(rename = "spaces", default)]
        pub spaces: Option<Vec<String>>,
        #[doc = "Whether the user has starred the file."]
        #[serde(rename = "starred", default)]
        pub starred: Option<bool>,
        #[doc = "Deprecated - use driveId instead."]
        #[serde(rename = "teamDriveId", default)]
        pub team_drive_id: Option<String>,
        #[doc = "A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Only populated when the requesting app can access the file's content."]
        #[serde(rename = "thumbnailLink", default)]
        pub thumbnail_link: Option<String>,
        #[doc = "The thumbnail version for use in thumbnail cache invalidation."]
        #[serde(rename = "thumbnailVersion", default)]
        #[serde(with = "crate::parsed_string")]
        pub thumbnail_version: Option<i64>,
        #[doc = "Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, and other users cannot see files in the owner's trash."]
        #[serde(rename = "trashed", default)]
        pub trashed: Option<bool>,
        #[doc = "The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives."]
        #[serde(rename = "trashedTime", default)]
        pub trashed_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives."]
        #[serde(rename = "trashingUser", default)]
        pub trashing_user: Option<crate::schemas::User>,
        #[doc = "A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user."]
        #[serde(rename = "version", default)]
        #[serde(with = "crate::parsed_string")]
        pub version: Option<i64>,
        #[doc = "Additional metadata about video media. This may not be available immediately upon upload."]
        #[serde(rename = "videoMediaMetadata", default)]
        pub video_media_metadata: Option<crate::schemas::FileVideoMediaMetadata>,
        #[doc = "Whether the file has been viewed by this user."]
        #[serde(rename = "viewedByMe", default)]
        pub viewed_by_me: Option<bool>,
        #[doc = "The last time the file was viewed by the user (RFC 3339 date-time)."]
        #[serde(rename = "viewedByMeTime", default)]
        pub viewed_by_me_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Deprecated - use copyRequiresWriterPermission instead."]
        #[serde(rename = "viewersCanCopyContent", default)]
        pub viewers_can_copy_content: Option<bool>,
        #[doc = "A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive."]
        #[serde(rename = "webContentLink", default)]
        pub web_content_link: Option<String>,
        #[doc = "A link for opening the file in a relevant Google editor or viewer in a browser."]
        #[serde(rename = "webViewLink", default)]
        pub web_view_link: Option<String>,
        #[doc = "Whether users with only writer permission can modify the file's permissions. Not populated for items in shared drives."]
        #[serde(rename = "writersCanShare", default)]
        pub writers_can_share: Option<bool>,
    }
    impl ::field_selector::FieldSelector for File {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FileList {
        #[doc = "The list of files. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "files", default)]
        pub files: Option<Vec<crate::schemas::File>>,
        #[doc = "Whether the search process was incomplete. If true, then some search results may be missing, since all documents were not searched. This may occur when searching multiple drives with the \"allDrives\" corpora, but all corpora could not be searched. When this happens, it is suggested that clients narrow their query by choosing a different corpus such as \"user\" or \"drive\"."]
        #[serde(rename = "incompleteSearch", default)]
        pub incomplete_search: Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#fileList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The page token for the next page of files. This will be absent if the end of the files list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for FileList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GeneratedIds {
        #[doc = "The IDs generated for the requesting user in the specified space."]
        #[serde(rename = "ids", default)]
        pub ids: Option<Vec<String>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#generatedIds\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The type of file that can be created with these IDs."]
        #[serde(rename = "space", default)]
        pub space: Option<String>,
    }
    impl ::field_selector::FieldSelector for GeneratedIds {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PermissionPermissionDetailsItems {
        #[doc = "Whether this permission is inherited. This field is always populated. This is an output-only field."]
        #[serde(rename = "inherited", default)]
        pub inherited: Option<bool>,
        #[doc = "The ID of the item from which this permission is inherited. This is an output-only field and is only populated for members of the shared drive."]
        #[serde(rename = "inheritedFrom", default)]
        pub inherited_from: Option<String>,
        #[doc = "The permission type for this user. While new values may be added in future, the following are currently possible:  \n- file \n- member"]
        #[serde(rename = "permissionType", default)]
        pub permission_type: Option<String>,
        #[doc = "The primary role for this user. While new values may be added in the future, the following are currently possible:  \n- organizer \n- fileOrganizer \n- writer \n- commenter \n- reader"]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
    }
    impl ::field_selector::FieldSelector for PermissionPermissionDetailsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PermissionTeamDrivePermissionDetailsItems {
        #[doc = "Deprecated - use permissionDetails/inherited instead."]
        #[serde(rename = "inherited", default)]
        pub inherited: Option<bool>,
        #[doc = "Deprecated - use permissionDetails/inheritedFrom instead."]
        #[serde(rename = "inheritedFrom", default)]
        pub inherited_from: Option<String>,
        #[doc = "Deprecated - use permissionDetails/role instead."]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
        #[doc = "Deprecated - use permissionDetails/permissionType instead."]
        #[serde(rename = "teamDrivePermissionType", default)]
        pub team_drive_permission_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for PermissionTeamDrivePermissionDetailsItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Permission {
        #[doc = "Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type domain or anyone."]
        #[serde(rename = "allowFileDiscovery", default)]
        pub allow_file_discovery: Option<bool>,
        #[doc = "Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions."]
        #[serde(rename = "deleted", default)]
        pub deleted: Option<bool>,
        #[doc = "A displayable name for users, groups or domains."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The domain to which this permission refers."]
        #[serde(rename = "domain", default)]
        pub domain: Option<String>,
        #[doc = "The email address of the user or group to which this permission refers."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: Option<String>,
        #[doc = "The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions:  \n- They can only be set on user and group permissions \n- The time must be in the future \n- The time cannot be more than a year in the future"]
        #[serde(rename = "expirationTime", default)]
        pub expiration_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The ID of this permission. This is a unique identifier for the grantee, and is published in User resources as permissionId."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#permission\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Details of whether the permissions on this shared drive item are inherited or directly on this item. This is an output-only field which is present only for shared drive items."]
        #[serde(rename = "permissionDetails", default)]
        pub permission_details: Option<Vec<crate::schemas::PermissionPermissionDetailsItems>>,
        #[doc = "A link to the user's profile photo, if available."]
        #[serde(rename = "photoLink", default)]
        pub photo_link: Option<String>,
        #[doc = "The type of the grantee. Valid values are:  \n- user \n- group \n- domain \n- anyone"]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The role granted by this permission. While new values may be supported in the future, the following are currently allowed:  \n- owner \n- organizer \n- fileOrganizer \n- writer \n- commenter \n- reader"]
        #[serde(rename = "role", default)]
        pub role: Option<String>,
        #[doc = "Deprecated - use permissionDetails instead."]
        #[serde(rename = "teamDrivePermissionDetails", default)]
        pub team_drive_permission_details:
            Option<Vec<crate::schemas::PermissionTeamDrivePermissionDetailsItems>>,
    }
    impl ::field_selector::FieldSelector for Permission {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PermissionList {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#permissionList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of permissions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "permissions", default)]
        pub permissions: Option<Vec<crate::schemas::Permission>>,
    }
    impl ::field_selector::FieldSelector for PermissionList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Reply {
        #[doc = "The action the reply performed to the parent comment. Valid values are:  \n- resolve \n- reopen"]
        #[serde(rename = "action", default)]
        pub action: Option<String>,
        #[doc = "The user who created the reply."]
        #[serde(rename = "author", default)]
        pub author: Option<crate::schemas::User>,
        #[doc = "The plain text content of the reply. This field is used for setting the content, while htmlContent should be displayed. This is required on creates if no action is specified."]
        #[serde(rename = "content", default)]
        pub content: Option<String>,
        #[doc = "The time at which the reply was created (RFC 3339 date-time)."]
        #[serde(rename = "createdTime", default)]
        pub created_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Whether the reply has been deleted. A deleted reply has no content."]
        #[serde(rename = "deleted", default)]
        pub deleted: Option<bool>,
        #[doc = "The content of the reply with HTML formatting."]
        #[serde(rename = "htmlContent", default)]
        pub html_content: Option<String>,
        #[doc = "The ID of the reply."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#reply\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The last time the reply was modified (RFC 3339 date-time)."]
        #[serde(rename = "modifiedTime", default)]
        pub modified_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for Reply {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplyList {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#replyList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The page token for the next page of replies. This will be absent if the end of the replies list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of replies. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "replies", default)]
        pub replies: Option<Vec<crate::schemas::Reply>>,
    }
    impl ::field_selector::FieldSelector for ReplyList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Revision {
        #[doc = "Links for exporting Google Docs to specific formats."]
        #[serde(rename = "exportLinks", default)]
        pub export_links: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The ID of the revision."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file.\nThis field is only applicable to files with binary content in Drive."]
        #[serde(rename = "keepForever", default)]
        pub keep_forever: Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#revision\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The last user to modify this revision."]
        #[serde(rename = "lastModifyingUser", default)]
        pub last_modifying_user: Option<crate::schemas::User>,
        #[doc = "The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive."]
        #[serde(rename = "md5Checksum", default)]
        pub md_5_checksum: Option<String>,
        #[doc = "The MIME type of the revision."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
        #[doc = "The last time the revision was modified (RFC 3339 date-time)."]
        #[serde(rename = "modifiedTime", default)]
        pub modified_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The original filename used to create this revision. This is only applicable to files with binary content in Drive."]
        #[serde(rename = "originalFilename", default)]
        pub original_filename: Option<String>,
        #[doc = "Whether subsequent revisions will be automatically republished. This is only applicable to Google Docs."]
        #[serde(rename = "publishAuto", default)]
        pub publish_auto: Option<bool>,
        #[doc = "Whether this revision is published. This is only applicable to Google Docs."]
        #[serde(rename = "published", default)]
        pub published: Option<bool>,
        #[doc = "Whether this revision is published outside the domain. This is only applicable to Google Docs."]
        #[serde(rename = "publishedOutsideDomain", default)]
        pub published_outside_domain: Option<bool>,
        #[doc = "The size of the revision's content in bytes. This is only applicable to files with binary content in Drive."]
        #[serde(rename = "size", default)]
        #[serde(with = "crate::parsed_string")]
        pub size: Option<i64>,
    }
    impl ::field_selector::FieldSelector for Revision {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RevisionList {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#revisionList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "revisions", default)]
        pub revisions: Option<Vec<crate::schemas::Revision>>,
    }
    impl ::field_selector::FieldSelector for RevisionList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StartPageToken {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#startPageToken\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The starting page token for listing changes."]
        #[serde(rename = "startPageToken", default)]
        pub start_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for StartPageToken {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TeamDriveBackgroundImageFile {
        #[doc = "The ID of an image file in Drive to use for the background image."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high."]
        #[serde(rename = "width", default)]
        pub width: Option<f32>,
        #[doc = "The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image."]
        #[serde(rename = "xCoordinate", default)]
        pub x_coordinate: Option<f32>,
        #[doc = "The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image."]
        #[serde(rename = "yCoordinate", default)]
        pub y_coordinate: Option<f32>,
    }
    impl ::field_selector::FieldSelector for TeamDriveBackgroundImageFile {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TeamDriveCapabilities {
        #[doc = "Whether the current user can add children to folders in this Team Drive."]
        #[serde(rename = "canAddChildren", default)]
        pub can_add_children: Option<bool>,
        #[doc = "Whether the current user can change the copyRequiresWriterPermission restriction of this Team Drive."]
        #[serde(rename = "canChangeCopyRequiresWriterPermissionRestriction", default)]
        pub can_change_copy_requires_writer_permission_restriction: Option<bool>,
        #[doc = "Whether the current user can change the domainUsersOnly restriction of this Team Drive."]
        #[serde(rename = "canChangeDomainUsersOnlyRestriction", default)]
        pub can_change_domain_users_only_restriction: Option<bool>,
        #[doc = "Whether the current user can change the background of this Team Drive."]
        #[serde(rename = "canChangeTeamDriveBackground", default)]
        pub can_change_team_drive_background: Option<bool>,
        #[doc = "Whether the current user can change the teamMembersOnly restriction of this Team Drive."]
        #[serde(rename = "canChangeTeamMembersOnlyRestriction", default)]
        pub can_change_team_members_only_restriction: Option<bool>,
        #[doc = "Whether the current user can comment on files in this Team Drive."]
        #[serde(rename = "canComment", default)]
        pub can_comment: Option<bool>,
        #[doc = "Whether the current user can copy files in this Team Drive."]
        #[serde(rename = "canCopy", default)]
        pub can_copy: Option<bool>,
        #[doc = "Whether the current user can delete children from folders in this Team Drive."]
        #[serde(rename = "canDeleteChildren", default)]
        pub can_delete_children: Option<bool>,
        #[doc = "Whether the current user can delete this Team Drive. Attempting to delete the Team Drive may still fail if there are untrashed items inside the Team Drive."]
        #[serde(rename = "canDeleteTeamDrive", default)]
        pub can_delete_team_drive: Option<bool>,
        #[doc = "Whether the current user can download files in this Team Drive."]
        #[serde(rename = "canDownload", default)]
        pub can_download: Option<bool>,
        #[doc = "Whether the current user can edit files in this Team Drive"]
        #[serde(rename = "canEdit", default)]
        pub can_edit: Option<bool>,
        #[doc = "Whether the current user can list the children of folders in this Team Drive."]
        #[serde(rename = "canListChildren", default)]
        pub can_list_children: Option<bool>,
        #[doc = "Whether the current user can add members to this Team Drive or remove them or change their role."]
        #[serde(rename = "canManageMembers", default)]
        pub can_manage_members: Option<bool>,
        #[doc = "Whether the current user can read the revisions resource of files in this Team Drive."]
        #[serde(rename = "canReadRevisions", default)]
        pub can_read_revisions: Option<bool>,
        #[doc = "Deprecated - use canDeleteChildren or canTrashChildren instead."]
        #[serde(rename = "canRemoveChildren", default)]
        pub can_remove_children: Option<bool>,
        #[doc = "Whether the current user can rename files or folders in this Team Drive."]
        #[serde(rename = "canRename", default)]
        pub can_rename: Option<bool>,
        #[doc = "Whether the current user can rename this Team Drive."]
        #[serde(rename = "canRenameTeamDrive", default)]
        pub can_rename_team_drive: Option<bool>,
        #[doc = "Whether the current user can share files or folders in this Team Drive."]
        #[serde(rename = "canShare", default)]
        pub can_share: Option<bool>,
        #[doc = "Whether the current user can trash children from folders in this Team Drive."]
        #[serde(rename = "canTrashChildren", default)]
        pub can_trash_children: Option<bool>,
    }
    impl ::field_selector::FieldSelector for TeamDriveCapabilities {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TeamDriveRestrictions {
        #[doc = "Whether administrative privileges on this Team Drive are required to modify restrictions."]
        #[serde(rename = "adminManagedRestrictions", default)]
        pub admin_managed_restrictions: Option<bool>,
        #[doc = "Whether the options to copy, print, or download files inside this Team Drive, should be disabled for readers and commenters. When this restriction is set to true, it will override the similarly named field to true for any file inside this Team Drive."]
        #[serde(rename = "copyRequiresWriterPermission", default)]
        pub copy_requires_writer_permission: Option<bool>,
        #[doc = "Whether access to this Team Drive and items inside this Team Drive is restricted to users of the domain to which this Team Drive belongs. This restriction may be overridden by other sharing policies controlled outside of this Team Drive."]
        #[serde(rename = "domainUsersOnly", default)]
        pub domain_users_only: Option<bool>,
        #[doc = "Whether access to items inside this Team Drive is restricted to members of this Team Drive."]
        #[serde(rename = "teamMembersOnly", default)]
        pub team_members_only: Option<bool>,
    }
    impl ::field_selector::FieldSelector for TeamDriveRestrictions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TeamDrive {
        #[doc = "An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on drive.teamdrives.update requests that don't set themeId. When specified, all fields of the backgroundImageFile must be set."]
        #[serde(rename = "backgroundImageFile", default)]
        pub background_image_file: Option<crate::schemas::TeamDriveBackgroundImageFile>,
        #[doc = "A short-lived link to this Team Drive's background image."]
        #[serde(rename = "backgroundImageLink", default)]
        pub background_image_link: Option<String>,
        #[doc = "Capabilities the current user has on this Team Drive."]
        #[serde(rename = "capabilities", default)]
        pub capabilities: Option<crate::schemas::TeamDriveCapabilities>,
        #[doc = "The color of this Team Drive as an RGB hex string. It can only be set on a drive.teamdrives.update request that does not set themeId."]
        #[serde(rename = "colorRgb", default)]
        pub color_rgb: Option<String>,
        #[doc = "The time at which the Team Drive was created (RFC 3339 date-time)."]
        #[serde(rename = "createdTime", default)]
        pub created_time: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The ID of this Team Drive which is also the ID of the top level folder of this Team Drive."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#teamDrive\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The name of this Team Drive."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "A set of restrictions that apply to this Team Drive or items inside this Team Drive."]
        #[serde(rename = "restrictions", default)]
        pub restrictions: Option<crate::schemas::TeamDriveRestrictions>,
        #[doc = "The ID of the theme from which the background image and color will be set. The set of possible teamDriveThemes can be retrieved from a drive.about.get response. When not specified on a drive.teamdrives.create request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set colorRgb or backgroundImageFile."]
        #[serde(rename = "themeId", default)]
        pub theme_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for TeamDrive {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TeamDriveList {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#teamDriveList\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. If the token is rejected for any reason, it should be discarded, and pagination should be restarted from the first page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched."]
        #[serde(rename = "teamDrives", default)]
        pub team_drives: Option<Vec<crate::schemas::TeamDrive>>,
    }
    impl ::field_selector::FieldSelector for TeamDriveList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "A plain text displayable name for this user."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The email address of the user. This may not be present in certain contexts if the user has not made their email address visible to the requester."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"drive#user\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Whether this user is the requesting user."]
        #[serde(rename = "me", default)]
        pub me: Option<bool>,
        #[doc = "The user's ID as visible in Permission resources."]
        #[serde(rename = "permissionId", default)]
        pub permission_id: Option<String>,
        #[doc = "A link to the user's profile photo, if available."]
        #[serde(rename = "photoLink", default)]
        pub photo_link: Option<String>,
    }
    impl ::field_selector::FieldSelector for User {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Upload/Download media content"]
        Media,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
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
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
    #[doc = "Actions that can be performed on the about resource"]
    pub fn about(&self) -> crate::about::AboutActions<A> {
        crate::about::AboutActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the changes resource"]
    pub fn changes(&self) -> crate::changes::ChangesActions<A> {
        crate::changes::ChangesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the channels resource"]
    pub fn channels(&self) -> crate::channels::ChannelsActions<A> {
        crate::channels::ChannelsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the comments resource"]
    pub fn comments(&self) -> crate::comments::CommentsActions<A> {
        crate::comments::CommentsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the drives resource"]
    pub fn drives(&self) -> crate::drives::DrivesActions<A> {
        crate::drives::DrivesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the files resource"]
    pub fn files(&self) -> crate::files::FilesActions<A> {
        crate::files::FilesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the permissions resource"]
    pub fn permissions(&self) -> crate::permissions::PermissionsActions<A> {
        crate::permissions::PermissionsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the replies resource"]
    pub fn replies(&self) -> crate::replies::RepliesActions<A> {
        crate::replies::RepliesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the revisions resource"]
    pub fn revisions(&self) -> crate::revisions::RevisionsActions<A> {
        crate::revisions::RevisionsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the teamdrives resource"]
    pub fn teamdrives(&self) -> crate::teamdrives::TeamdrivesActions<A> {
        crate::teamdrives::TeamdrivesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod about {
    pub mod params {}
    pub struct AboutActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> AboutActions<'a, A> {
        #[doc = "Gets information about the user, the user's Drive, and system capabilities."]
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
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::About, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("about");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod changes {
    pub mod params {}
    pub struct ChangesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ChangesActions<'a, A> {
        #[doc = "Gets the starting pageToken for listing future changes."]
        pub fn get_start_page_token(&self) -> GetStartPageTokenRequestBuilder<A> {
            GetStartPageTokenRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                drive_id: None,
                supports_all_drives: None,
                supports_team_drives: None,
                team_drive_id: None,
            }
        }
        #[doc = "Lists the changes for a user or shared drive."]
        pub fn list(&self, page_token: impl Into<String>) -> ListRequestBuilder<A> {
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
                page_token: page_token.into(),
                drive_id: None,
                include_corpus_removals: None,
                include_items_from_all_drives: None,
                include_removed: None,
                include_team_drive_items: None,
                page_size: None,
                restrict_to_my_drive: None,
                spaces: None,
                supports_all_drives: None,
                supports_team_drives: None,
                team_drive_id: None,
            }
        }
        #[doc = "Subscribes to changes for a user."]
        pub fn watch(
            &self,
            request: crate::schemas::Channel,
            page_token: impl Into<String>,
        ) -> WatchRequestBuilder<A> {
            WatchRequestBuilder {
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
                page_token: page_token.into(),
                drive_id: None,
                include_corpus_removals: None,
                include_items_from_all_drives: None,
                include_removed: None,
                include_team_drive_items: None,
                page_size: None,
                restrict_to_my_drive: None,
                spaces: None,
                supports_all_drives: None,
                supports_team_drives: None,
                team_drive_id: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetStartPageTokenRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        drive_id: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        team_drive_id: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetStartPageTokenRequestBuilder<'a, A> {
        #[doc = "The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive will be returned."]
        pub fn drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.drive_id = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Deprecated use driveId instead."]
        pub fn team_drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.team_drive_id = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::StartPageToken, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("changes/startPageToken");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("driveId", &self.drive_id)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("teamDriveId", &self.team_drive_id)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        page_token: String,
        drive_id: Option<String>,
        include_corpus_removals: Option<bool>,
        include_items_from_all_drives: Option<bool>,
        include_removed: Option<bool>,
        include_team_drive_items: Option<bool>,
        page_size: Option<i32>,
        restrict_to_my_drive: Option<bool>,
        spaces: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        team_drive_id: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The shared drive from which changes will be returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier."]
        pub fn drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.drive_id = Some(value.into());
            self
        }
        #[doc = "Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file."]
        pub fn include_corpus_removals(&mut self, value: bool) -> &mut Self {
            self.include_corpus_removals = Some(value);
            self
        }
        #[doc = "Deprecated - Whether both My Drive and shared drive items should be included in results. This parameter will only be effective until June 1, 2020. Afterwards shared drive items will be included in the results."]
        pub fn include_items_from_all_drives(&mut self, value: bool) -> &mut Self {
            self.include_items_from_all_drives = Some(value);
            self
        }
        #[doc = "Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access."]
        pub fn include_removed(&mut self, value: bool) -> &mut Self {
            self.include_removed = Some(value);
            self
        }
        #[doc = "Deprecated use includeItemsFromAllDrives instead."]
        pub fn include_team_drive_items(&mut self, value: bool) -> &mut Self {
            self.include_team_drive_items = Some(value);
            self
        }
        #[doc = "The maximum number of changes to return per page."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive."]
        pub fn restrict_to_my_drive(&mut self, value: bool) -> &mut Self {
            self.restrict_to_my_drive = Some(value);
            self
        }
        #[doc = "A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'."]
        pub fn spaces(&mut self, value: impl Into<String>) -> &mut Self {
            self.spaces = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Deprecated use driveId instead."]
        pub fn team_drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.team_drive_id = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_changes<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "changes")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ChangeList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("changes");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("driveId", &self.drive_id)]);
            let req = req.query(&[("includeCorpusRemovals", &self.include_corpus_removals)]);
            let req = req.query(&[(
                "includeItemsFromAllDrives",
                &self.include_items_from_all_drives,
            )]);
            let req = req.query(&[("includeRemoved", &self.include_removed)]);
            let req = req.query(&[("includeTeamDriveItems", &self.include_team_drive_items)]);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("restrictToMyDrive", &self.restrict_to_my_drive)]);
            let req = req.query(&[("spaces", &self.spaces)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("teamDriveId", &self.team_drive_id)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct WatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Channel,
        page_token: String,
        drive_id: Option<String>,
        include_corpus_removals: Option<bool>,
        include_items_from_all_drives: Option<bool>,
        include_removed: Option<bool>,
        include_team_drive_items: Option<bool>,
        page_size: Option<i32>,
        restrict_to_my_drive: Option<bool>,
        spaces: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        team_drive_id: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> WatchRequestBuilder<'a, A> {
        #[doc = "The shared drive from which changes will be returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier."]
        pub fn drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.drive_id = Some(value.into());
            self
        }
        #[doc = "Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file."]
        pub fn include_corpus_removals(&mut self, value: bool) -> &mut Self {
            self.include_corpus_removals = Some(value);
            self
        }
        #[doc = "Deprecated - Whether both My Drive and shared drive items should be included in results. This parameter will only be effective until June 1, 2020. Afterwards shared drive items will be included in the results."]
        pub fn include_items_from_all_drives(&mut self, value: bool) -> &mut Self {
            self.include_items_from_all_drives = Some(value);
            self
        }
        #[doc = "Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access."]
        pub fn include_removed(&mut self, value: bool) -> &mut Self {
            self.include_removed = Some(value);
            self
        }
        #[doc = "Deprecated use includeItemsFromAllDrives instead."]
        pub fn include_team_drive_items(&mut self, value: bool) -> &mut Self {
            self.include_team_drive_items = Some(value);
            self
        }
        #[doc = "The maximum number of changes to return per page."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive."]
        pub fn restrict_to_my_drive(&mut self, value: bool) -> &mut Self {
            self.restrict_to_my_drive = Some(value);
            self
        }
        #[doc = "A comma-separated list of spaces to query within the user corpus. Supported values are 'drive', 'appDataFolder' and 'photos'."]
        pub fn spaces(&mut self, value: impl Into<String>) -> &mut Self {
            self.spaces = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Deprecated use driveId instead."]
        pub fn team_drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.team_drive_id = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Channel, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("changes/watch");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("driveId", &self.drive_id)]);
            let req = req.query(&[("includeCorpusRemovals", &self.include_corpus_removals)]);
            let req = req.query(&[(
                "includeItemsFromAllDrives",
                &self.include_items_from_all_drives,
            )]);
            let req = req.query(&[("includeRemoved", &self.include_removed)]);
            let req = req.query(&[("includeTeamDriveItems", &self.include_team_drive_items)]);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("restrictToMyDrive", &self.restrict_to_my_drive)]);
            let req = req.query(&[("spaces", &self.spaces)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("teamDriveId", &self.team_drive_id)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod channels {
    pub mod params {}
    pub struct ChannelsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ChannelsActions<'a, A> {
        #[doc = "Stop watching resources through this channel"]
        pub fn stop(&self, request: crate::schemas::Channel) -> StopRequestBuilder<A> {
            StopRequestBuilder {
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
    pub struct StopRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Channel,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> StopRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("channels/stop");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod comments {
    pub mod params {}
    pub struct CommentsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> CommentsActions<'a, A> {
        #[doc = "Creates a new comment on a file."]
        pub fn create(
            &self,
            request: crate::schemas::Comment,
            file_id: impl Into<String>,
        ) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
                file_id: file_id.into(),
            }
        }
        #[doc = "Deletes a comment."]
        pub fn delete(
            &self,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
            }
        }
        #[doc = "Gets a comment by ID."]
        pub fn get(
            &self,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
                include_deleted: None,
            }
        }
        #[doc = "Lists a file's comments."]
        pub fn list(&self, file_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                file_id: file_id.into(),
                include_deleted: None,
                page_size: None,
                page_token: None,
                start_modified_time: None,
            }
        }
        #[doc = "Updates a comment with patch semantics."]
        pub fn update(
            &self,
            request: crate::schemas::Comment,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Comment,
        file_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Comment, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        comment_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        comment_id: String,
        include_deleted: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Whether to return deleted comments. Deleted comments will not include their original content."]
        pub fn include_deleted(&mut self, value: bool) -> &mut Self {
            self.include_deleted = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Comment, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("includeDeleted", &self.include_deleted)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        include_deleted: Option<bool>,
        page_size: Option<i32>,
        page_token: Option<String>,
        start_modified_time: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Whether to include deleted comments. Deleted comments will not include their original content."]
        pub fn include_deleted(&mut self, value: bool) -> &mut Self {
            self.include_deleted = Some(value);
            self
        }
        #[doc = "The maximum number of comments to return per page."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time)."]
        pub fn start_modified_time(&mut self, value: impl Into<String>) -> &mut Self {
            self.start_modified_time = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_comments<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "comments")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::CommentList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("includeDeleted", &self.include_deleted)]);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("startModifiedTime", &self.start_modified_time)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Comment,
        file_id: String,
        comment_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Comment, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod drives {
    pub mod params {}
    pub struct DrivesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> DrivesActions<'a, A> {
        #[doc = "Creates a new shared drive."]
        pub fn create(
            &self,
            request: crate::schemas::Drive,
            request_id: impl Into<String>,
        ) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
                request_id: request_id.into(),
            }
        }
        #[doc = "Permanently deletes a shared drive for which the user is an organizer. The shared drive cannot contain any untrashed items."]
        pub fn delete(&self, drive_id: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                drive_id: drive_id.into(),
            }
        }
        #[doc = "Gets a shared drive's metadata by ID."]
        pub fn get(&self, drive_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                drive_id: drive_id.into(),
                use_domain_admin_access: None,
            }
        }
        #[doc = "Hides a shared drive from the default view."]
        pub fn hide(&self, drive_id: impl Into<String>) -> HideRequestBuilder<A> {
            HideRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                drive_id: drive_id.into(),
            }
        }
        #[doc = "Lists the user's shared drives."]
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
                page_size: None,
                page_token: None,
                q: None,
                use_domain_admin_access: None,
            }
        }
        #[doc = "Restores a shared drive to the default view."]
        pub fn unhide(&self, drive_id: impl Into<String>) -> UnhideRequestBuilder<A> {
            UnhideRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                drive_id: drive_id.into(),
            }
        }
        #[doc = "Updates the metadate for a shared drive."]
        pub fn update(
            &self,
            request: crate::schemas::Drive,
            drive_id: impl Into<String>,
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
                drive_id: drive_id.into(),
                use_domain_admin_access: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Drive,
        request_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Drive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("drives");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("requestId", &self.request_id)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        drive_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("drives/");
            output.push_str(&self.drive_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        drive_id: String,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Drive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("drives/");
            output.push_str(&self.drive_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct HideRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        drive_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> HideRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Drive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("drives/");
            output.push_str(&self.drive_id);
            output.push_str("/hide");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        page_size: Option<i32>,
        page_token: Option<String>,
        q: Option<String>,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Maximum number of shared drives to return."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "Page token for shared drives."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Query string for searching shared drives."]
        pub fn q(&mut self, value: impl Into<String>) -> &mut Self {
            self.q = Some(value.into());
            self
        }
        #[doc = "Issue the request as a domain administrator; if set to true, then all shared drives of the domain in which the requester is an administrator are returned."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_drives<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "drives")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::DriveList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("drives");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("q", &self.q)]);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct UnhideRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        drive_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UnhideRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Drive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("drives/");
            output.push_str(&self.drive_id);
            output.push_str("/unhide");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Drive,
        drive_id: String,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Drive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("drives/");
            output.push_str(&self.drive_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod files {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum ListCorpus {
            #[doc = "Files shared to the user's domain."]
            Domain,
            #[doc = "Files owned by or shared to the user."]
            User,
        }
        impl ListCorpus {
            pub fn as_str(self) -> &'static str {
                match self {
                    ListCorpus::Domain => "domain",
                    ListCorpus::User => "user",
                }
            }
        }
        impl ::std::fmt::Display for ListCorpus {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListCorpus {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListCorpus {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "domain" => ListCorpus::Domain,
                    "user" => ListCorpus::User,
                    _ => {
                        return Err(::serde::de::Error::custom(format!(
                            "invalid enum for #name: {}",
                            value
                        )))
                    }
                })
            }
        }
    }
    pub struct FilesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> FilesActions<'a, A> {
        #[doc = "Creates a copy of a file and applies any requested updates with patch semantics."]
        pub fn copy(
            &self,
            request: crate::schemas::File,
            file_id: impl Into<String>,
        ) -> CopyRequestBuilder<A> {
            CopyRequestBuilder {
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
                file_id: file_id.into(),
                ignore_default_visibility: None,
                keep_revision_forever: None,
                ocr_language: None,
                supports_all_drives: None,
                supports_team_drives: None,
            }
        }
        #[doc = "Creates a new file."]
        pub fn create(&self, request: crate::schemas::File) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
                ignore_default_visibility: None,
                keep_revision_forever: None,
                ocr_language: None,
                supports_all_drives: None,
                supports_team_drives: None,
                use_content_as_indexable_text: None,
            }
        }
        #[doc = "Permanently deletes a file owned by the user without moving it to the trash. If the file belongs to a shared drive the user must be an organizer on the parent. If the target is a folder, all descendants owned by the user are also deleted."]
        pub fn delete(&self, file_id: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                file_id: file_id.into(),
                supports_all_drives: None,
                supports_team_drives: None,
            }
        }
        #[doc = "Permanently deletes all of the user's trashed files."]
        pub fn empty_trash(&self) -> EmptyTrashRequestBuilder<A> {
            EmptyTrashRequestBuilder {
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
        #[doc = "Exports a Google Doc to the requested MIME type and returns the exported content. Please note that the exported content is limited to 10MB."]
        pub fn export(
            &self,
            file_id: impl Into<String>,
            mime_type: impl Into<String>,
        ) -> ExportRequestBuilder<A> {
            ExportRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                file_id: file_id.into(),
                mime_type: mime_type.into(),
            }
        }
        #[doc = "Generates a set of file IDs which can be provided in create or copy requests."]
        pub fn generate_ids(&self) -> GenerateIdsRequestBuilder<A> {
            GenerateIdsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                count: None,
                space: None,
            }
        }
        #[doc = "Gets a file's metadata or content by ID."]
        pub fn get(&self, file_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                file_id: file_id.into(),
                acknowledge_abuse: None,
                supports_all_drives: None,
                supports_team_drives: None,
            }
        }
        #[doc = "Lists or searches files."]
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
                corpora: None,
                corpus: None,
                drive_id: None,
                include_items_from_all_drives: None,
                include_team_drive_items: None,
                order_by: None,
                page_size: None,
                page_token: None,
                q: None,
                spaces: None,
                supports_all_drives: None,
                supports_team_drives: None,
                team_drive_id: None,
            }
        }
        #[doc = "Updates a file's metadata and/or content with patch semantics."]
        pub fn update(
            &self,
            request: crate::schemas::File,
            file_id: impl Into<String>,
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
                file_id: file_id.into(),
                add_parents: None,
                keep_revision_forever: None,
                ocr_language: None,
                remove_parents: None,
                supports_all_drives: None,
                supports_team_drives: None,
                use_content_as_indexable_text: None,
            }
        }
        #[doc = "Subscribes to changes to a file"]
        pub fn watch(
            &self,
            request: crate::schemas::Channel,
            file_id: impl Into<String>,
        ) -> WatchRequestBuilder<A> {
            WatchRequestBuilder {
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
                file_id: file_id.into(),
                acknowledge_abuse: None,
                supports_all_drives: None,
                supports_team_drives: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CopyRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::File,
        file_id: String,
        ignore_default_visibility: Option<bool>,
        keep_revision_forever: Option<bool>,
        ocr_language: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CopyRequestBuilder<'a, A> {
        #[doc = "Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders."]
        pub fn ignore_default_visibility(&mut self, value: bool) -> &mut Self {
            self.ignore_default_visibility = Some(value);
            self
        }
        #[doc = "Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive."]
        pub fn keep_revision_forever(&mut self, value: bool) -> &mut Self {
            self.keep_revision_forever = Some(value);
            self
        }
        #[doc = "A language hint for OCR processing during image import (ISO 639-1 code)."]
        pub fn ocr_language(&mut self, value: impl Into<String>) -> &mut Self {
            self.ocr_language = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::File, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/copy");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("ignoreDefaultVisibility", &self.ignore_default_visibility)]);
            let req = req.query(&[("keepRevisionForever", &self.keep_revision_forever)]);
            let req = req.query(&[("ocrLanguage", &self.ocr_language)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::File,
        ignore_default_visibility: Option<bool>,
        keep_revision_forever: Option<bool>,
        ocr_language: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        use_content_as_indexable_text: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
        #[doc = "Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders."]
        pub fn ignore_default_visibility(&mut self, value: bool) -> &mut Self {
            self.ignore_default_visibility = Some(value);
            self
        }
        #[doc = "Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive."]
        pub fn keep_revision_forever(&mut self, value: bool) -> &mut Self {
            self.keep_revision_forever = Some(value);
            self
        }
        #[doc = "A language hint for OCR processing during image import (ISO 639-1 code)."]
        pub fn ocr_language(&mut self, value: impl Into<String>) -> &mut Self {
            self.ocr_language = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Whether to use the uploaded content as indexable text."]
        pub fn use_content_as_indexable_text(&mut self, value: bool) -> &mut Self {
            self.use_content_as_indexable_text = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        fn _simple_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("upload/drive/v3/files");
            output
        }
        pub fn upload<T, R>(
            mut self,
            content: R,
            mime_type: ::mime::Mime,
        ) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            R: ::std::io::Read + ::std::io::Seek + Send + 'static,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._simple_upload_path());
            let req = req.query(&[("uploadType", "multipart")]);
            use crate::multipart::{Part, RelatedMultiPart};
            let mut multipart = RelatedMultiPart::new();
            let request_json = ::serde_json::to_vec(&self.request)?;
            multipart.new_part(Part::new(
                ::mime::APPLICATION_JSON,
                Box::new(::std::io::Cursor::new(request_json)),
            ));
            multipart.new_part(Part::new(mime_type, Box::new(content)));
            let req = req.header(
                ::reqwest::header::CONTENT_TYPE,
                format!("multipart/related; boundary={}", multipart.boundary()),
            );
            let req = req.body(reqwest::Body::new(multipart.into_reader()));
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _resumable_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("resumable/upload/drive/v3/files");
            output
        }
        pub fn start_resumable_upload(
            self,
            mime_type: ::mime::Mime,
        ) -> Result<crate::ResumableUpload, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._resumable_upload_path());
            let req = req.query(&[("uploadType", "resumable")]);
            let req = req.header(
                ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                mime_type.to_string(),
            );
            let req = req.json(&self.request);
            let resp = req.send()?.error_for_status()?;
            let location_header =
                resp.headers()
                    .get(::reqwest::header::LOCATION)
                    .ok_or_else(|| {
                        format!("No LOCATION header returned when initiating resumable upload")
                    })?;
            let upload_url = ::std::str::from_utf8(location_header.as_bytes())?.to_owned();
            Ok(crate::ResumableUpload::new(
                self.reqwest.clone(),
                upload_url,
            ))
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::File, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("ignoreDefaultVisibility", &self.ignore_default_visibility)]);
            let req = req.query(&[("keepRevisionForever", &self.keep_revision_forever)]);
            let req = req.query(&[("ocrLanguage", &self.ocr_language)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[(
                "useContentAsIndexableText",
                &self.use_content_as_indexable_text,
            )]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct EmptyTrashRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> EmptyTrashRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/trash");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ExportRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        mime_type: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ExportRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        fn _download_path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/download/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/export");
            output
        }
        pub fn download<W>(mut self, output: &mut W) -> Result<u64, Box<dyn ::std::error::Error>>
        where
            W: ::std::io::Write + ?Sized,
        {
            self.alt(crate::params::Alt::Media);
            Ok(self
                ._request(&self._path())
                .send()?
                .error_for_status()?
                .copy_to(output)?)
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/export");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("mimeType", &self.mime_type)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GenerateIdsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        count: Option<i32>,
        space: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GenerateIdsRequestBuilder<'a, A> {
        #[doc = "The number of IDs to return."]
        pub fn count(&mut self, value: i32) -> &mut Self {
            self.count = Some(value);
            self
        }
        #[doc = "The space in which the IDs can be used to create new files. Supported values are 'drive' and 'appDataFolder'."]
        pub fn space(&mut self, value: impl Into<String>) -> &mut Self {
            self.space = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::GeneratedIds, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/generateIds");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("count", &self.count)]);
            let req = req.query(&[("space", &self.space)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        acknowledge_abuse: Option<bool>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media."]
        pub fn acknowledge_abuse(&mut self, value: bool) -> &mut Self {
            self.acknowledge_abuse = Some(value);
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        fn _download_path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/download/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output
        }
        pub fn download<W>(mut self, output: &mut W) -> Result<u64, Box<dyn ::std::error::Error>>
        where
            W: ::std::io::Write + ?Sized,
        {
            self.alt(crate::params::Alt::Media);
            Ok(self
                ._request(&self._path())
                .send()?
                .error_for_status()?
                .copy_to(output)?)
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::File, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("acknowledgeAbuse", &self.acknowledge_abuse)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        corpora: Option<String>,
        corpus: Option<crate::files::params::ListCorpus>,
        drive_id: Option<String>,
        include_items_from_all_drives: Option<bool>,
        include_team_drive_items: Option<bool>,
        order_by: Option<String>,
        page_size: Option<i32>,
        page_token: Option<String>,
        q: Option<String>,
        spaces: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        team_drive_id: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Bodies of items (files/documents) to which the query applies. Supported bodies are 'user', 'domain', 'drive' and 'allDrives'. Prefer 'user' or 'drive' to 'allDrives' for efficiency."]
        pub fn corpora(&mut self, value: impl Into<String>) -> &mut Self {
            self.corpora = Some(value.into());
            self
        }
        #[doc = "The source of files to list. Deprecated: use 'corpora' instead."]
        pub fn corpus(&mut self, value: crate::files::params::ListCorpus) -> &mut Self {
            self.corpus = Some(value);
            self
        }
        #[doc = "ID of the shared drive to search."]
        pub fn drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.drive_id = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether both My Drive and shared drive items should be included in results. This parameter will only be effective until June 1, 2020. Afterwards shared drive items will be included in the results."]
        pub fn include_items_from_all_drives(&mut self, value: bool) -> &mut Self {
            self.include_items_from_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use includeItemsFromAllDrives instead."]
        pub fn include_team_drive_items(&mut self, value: bool) -> &mut Self {
            self.include_team_drive_items = Some(value);
            self
        }
        #[doc = "A comma-separated list of sort keys. Valid keys are 'createdTime', 'folder', 'modifiedByMeTime', 'modifiedTime', 'name', 'name_natural', 'quotaBytesUsed', 'recency', 'sharedWithMeTime', 'starred', and 'viewedByMeTime'. Each key sorts ascending by default, but may be reversed with the 'desc' modifier. Example usage: ?orderBy=folder,modifiedTime desc,name. Please note that there is a current limitation for users with approximately one million files in which the requested sort order is ignored."]
        pub fn order_by(&mut self, value: impl Into<String>) -> &mut Self {
            self.order_by = Some(value.into());
            self
        }
        #[doc = "The maximum number of files to return per page. Partial or empty result pages are possible even before the end of the files list has been reached."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "A query for filtering the file results. See the \"Search for Files\" guide for supported syntax."]
        pub fn q(&mut self, value: impl Into<String>) -> &mut Self {
            self.q = Some(value.into());
            self
        }
        #[doc = "A comma-separated list of spaces to query within the corpus. Supported values are 'drive', 'appDataFolder' and 'photos'."]
        pub fn spaces(&mut self, value: impl Into<String>) -> &mut Self {
            self.spaces = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Deprecated use driveId instead."]
        pub fn team_drive_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.team_drive_id = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_files<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "files")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::FileList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("corpora", &self.corpora)]);
            let req = req.query(&[("corpus", &self.corpus)]);
            let req = req.query(&[("driveId", &self.drive_id)]);
            let req = req.query(&[(
                "includeItemsFromAllDrives",
                &self.include_items_from_all_drives,
            )]);
            let req = req.query(&[("includeTeamDriveItems", &self.include_team_drive_items)]);
            let req = req.query(&[("orderBy", &self.order_by)]);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("q", &self.q)]);
            let req = req.query(&[("spaces", &self.spaces)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("teamDriveId", &self.team_drive_id)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::File,
        file_id: String,
        add_parents: Option<String>,
        keep_revision_forever: Option<bool>,
        ocr_language: Option<String>,
        remove_parents: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        use_content_as_indexable_text: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "A comma-separated list of parent IDs to add."]
        pub fn add_parents(&mut self, value: impl Into<String>) -> &mut Self {
            self.add_parents = Some(value.into());
            self
        }
        #[doc = "Whether to set the 'keepForever' field in the new head revision. This is only applicable to files with binary content in Google Drive."]
        pub fn keep_revision_forever(&mut self, value: bool) -> &mut Self {
            self.keep_revision_forever = Some(value);
            self
        }
        #[doc = "A language hint for OCR processing during image import (ISO 639-1 code)."]
        pub fn ocr_language(&mut self, value: impl Into<String>) -> &mut Self {
            self.ocr_language = Some(value.into());
            self
        }
        #[doc = "A comma-separated list of parent IDs to remove."]
        pub fn remove_parents(&mut self, value: impl Into<String>) -> &mut Self {
            self.remove_parents = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Whether to use the uploaded content as indexable text."]
        pub fn use_content_as_indexable_text(&mut self, value: bool) -> &mut Self {
            self.use_content_as_indexable_text = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        fn _simple_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("upload/drive/v3/files/");
            output.push_str(&self.file_id);
            output
        }
        pub fn upload<T, R>(
            mut self,
            content: R,
            mime_type: ::mime::Mime,
        ) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            R: ::std::io::Read + ::std::io::Seek + Send + 'static,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._simple_upload_path());
            let req = req.query(&[("uploadType", "multipart")]);
            use crate::multipart::{Part, RelatedMultiPart};
            let mut multipart = RelatedMultiPart::new();
            let request_json = ::serde_json::to_vec(&self.request)?;
            multipart.new_part(Part::new(
                ::mime::APPLICATION_JSON,
                Box::new(::std::io::Cursor::new(request_json)),
            ));
            multipart.new_part(Part::new(mime_type, Box::new(content)));
            let req = req.header(
                ::reqwest::header::CONTENT_TYPE,
                format!("multipart/related; boundary={}", multipart.boundary()),
            );
            let req = req.body(reqwest::Body::new(multipart.into_reader()));
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _resumable_upload_path(&self) -> String {
            let mut output = "https://www.googleapis.com/".to_owned();
            output.push_str("resumable/upload/drive/v3/files/");
            output.push_str(&self.file_id);
            output
        }
        pub fn start_resumable_upload(
            self,
            mime_type: ::mime::Mime,
        ) -> Result<crate::ResumableUpload, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._resumable_upload_path());
            let req = req.query(&[("uploadType", "resumable")]);
            let req = req.header(
                ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                mime_type.to_string(),
            );
            let req = req.json(&self.request);
            let resp = req.send()?.error_for_status()?;
            let location_header =
                resp.headers()
                    .get(::reqwest::header::LOCATION)
                    .ok_or_else(|| {
                        format!("No LOCATION header returned when initiating resumable upload")
                    })?;
            let upload_url = ::std::str::from_utf8(location_header.as_bytes())?.to_owned();
            Ok(crate::ResumableUpload::new(
                self.reqwest.clone(),
                upload_url,
            ))
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::File, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("addParents", &self.add_parents)]);
            let req = req.query(&[("keepRevisionForever", &self.keep_revision_forever)]);
            let req = req.query(&[("ocrLanguage", &self.ocr_language)]);
            let req = req.query(&[("removeParents", &self.remove_parents)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[(
                "useContentAsIndexableText",
                &self.use_content_as_indexable_text,
            )]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct WatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Channel,
        file_id: String,
        acknowledge_abuse: Option<bool>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> WatchRequestBuilder<'a, A> {
        #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media."]
        pub fn acknowledge_abuse(&mut self, value: bool) -> &mut Self {
            self.acknowledge_abuse = Some(value);
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        fn _download_path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/download/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/watch");
            output
        }
        pub fn download<W>(mut self, output: &mut W) -> Result<u64, Box<dyn ::std::error::Error>>
        where
            W: ::std::io::Write + ?Sized,
        {
            self.alt(crate::params::Alt::Media);
            Ok(self
                ._request(&self._path())
                .send()?
                .error_for_status()?
                .copy_to(output)?)
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Channel, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/watch");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("acknowledgeAbuse", &self.acknowledge_abuse)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod permissions {
    pub mod params {}
    pub struct PermissionsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PermissionsActions<'a, A> {
        #[doc = "Creates a permission for a file or shared drive."]
        pub fn create(
            &self,
            request: crate::schemas::Permission,
            file_id: impl Into<String>,
        ) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
                file_id: file_id.into(),
                email_message: None,
                send_notification_email: None,
                supports_all_drives: None,
                supports_team_drives: None,
                transfer_ownership: None,
                use_domain_admin_access: None,
            }
        }
        #[doc = "Deletes a permission."]
        pub fn delete(
            &self,
            file_id: impl Into<String>,
            permission_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
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
                file_id: file_id.into(),
                permission_id: permission_id.into(),
                supports_all_drives: None,
                supports_team_drives: None,
                use_domain_admin_access: None,
            }
        }
        #[doc = "Gets a permission by ID."]
        pub fn get(
            &self,
            file_id: impl Into<String>,
            permission_id: impl Into<String>,
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
                file_id: file_id.into(),
                permission_id: permission_id.into(),
                supports_all_drives: None,
                supports_team_drives: None,
                use_domain_admin_access: None,
            }
        }
        #[doc = "Lists a file's or shared drive's permissions."]
        pub fn list(&self, file_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                file_id: file_id.into(),
                page_size: None,
                page_token: None,
                supports_all_drives: None,
                supports_team_drives: None,
                use_domain_admin_access: None,
            }
        }
        #[doc = "Updates a permission with patch semantics."]
        pub fn update(
            &self,
            request: crate::schemas::Permission,
            file_id: impl Into<String>,
            permission_id: impl Into<String>,
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
                file_id: file_id.into(),
                permission_id: permission_id.into(),
                remove_expiration: None,
                supports_all_drives: None,
                supports_team_drives: None,
                transfer_ownership: None,
                use_domain_admin_access: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Permission,
        file_id: String,
        email_message: Option<String>,
        send_notification_email: Option<bool>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        transfer_ownership: Option<bool>,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
        #[doc = "A plain text custom message to include in the notification email."]
        pub fn email_message(&mut self, value: impl Into<String>) -> &mut Self {
            self.email_message = Some(value.into());
            self
        }
        #[doc = "Whether to send a notification email when sharing to users or groups. This defaults to true for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers."]
        pub fn send_notification_email(&mut self, value: bool) -> &mut Self {
            self.send_notification_email = Some(value);
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect."]
        pub fn transfer_ownership(&mut self, value: bool) -> &mut Self {
            self.transfer_ownership = Some(value);
            self
        }
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Permission, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/permissions");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("emailMessage", &self.email_message)]);
            let req = req.query(&[("sendNotificationEmail", &self.send_notification_email)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("transferOwnership", &self.transfer_ownership)]);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        permission_id: String,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/permissions/");
            output.push_str(&self.permission_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        permission_id: String,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Permission, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/permissions/");
            output.push_str(&self.permission_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The maximum number of permissions to return per page. When not set for files in a shared drive, at most 100 results will be returned. When not set for files that are not in a shared drive, the entire list will be returned."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_permissions<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "permissions")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::PermissionList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/permissions");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Permission,
        file_id: String,
        permission_id: String,
        remove_expiration: Option<bool>,
        supports_all_drives: Option<bool>,
        supports_team_drives: Option<bool>,
        transfer_ownership: Option<bool>,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Whether to remove the expiration date."]
        pub fn remove_expiration(&mut self, value: bool) -> &mut Self {
            self.remove_expiration = Some(value);
            self
        }
        #[doc = "Deprecated - Whether the requesting application supports both My Drives and shared drives. This parameter will only be effective until June 1, 2020. Afterwards all applications are assumed to support shared drives."]
        pub fn supports_all_drives(&mut self, value: bool) -> &mut Self {
            self.supports_all_drives = Some(value);
            self
        }
        #[doc = "Deprecated use supportsAllDrives instead."]
        pub fn supports_team_drives(&mut self, value: bool) -> &mut Self {
            self.supports_team_drives = Some(value);
            self
        }
        #[doc = "Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect."]
        pub fn transfer_ownership(&mut self, value: bool) -> &mut Self {
            self.transfer_ownership = Some(value);
            self
        }
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if the file ID parameter refers to a shared drive and the requester is an administrator of the domain to which the shared drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Permission, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/permissions/");
            output.push_str(&self.permission_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("removeExpiration", &self.remove_expiration)]);
            let req = req.query(&[("supportsAllDrives", &self.supports_all_drives)]);
            let req = req.query(&[("supportsTeamDrives", &self.supports_team_drives)]);
            let req = req.query(&[("transferOwnership", &self.transfer_ownership)]);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod replies {
    pub mod params {}
    pub struct RepliesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> RepliesActions<'a, A> {
        #[doc = "Creates a new reply to a comment."]
        pub fn create(
            &self,
            request: crate::schemas::Reply,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
        ) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
            }
        }
        #[doc = "Deletes a reply."]
        pub fn delete(
            &self,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
            reply_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
                reply_id: reply_id.into(),
            }
        }
        #[doc = "Gets a reply by ID."]
        pub fn get(
            &self,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
            reply_id: impl Into<String>,
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
                reply_id: reply_id.into(),
                include_deleted: None,
            }
        }
        #[doc = "Lists a comment's replies."]
        pub fn list(
            &self,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
                include_deleted: None,
                page_size: None,
                page_token: None,
            }
        }
        #[doc = "Updates a reply with patch semantics."]
        pub fn update(
            &self,
            request: crate::schemas::Reply,
            file_id: impl Into<String>,
            comment_id: impl Into<String>,
            reply_id: impl Into<String>,
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
                file_id: file_id.into(),
                comment_id: comment_id.into(),
                reply_id: reply_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Reply,
        file_id: String,
        comment_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Reply, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output.push_str("/replies");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        comment_id: String,
        reply_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output.push_str("/replies/");
            output.push_str(&self.reply_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        comment_id: String,
        reply_id: String,
        include_deleted: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Whether to return deleted replies. Deleted replies will not include their original content."]
        pub fn include_deleted(&mut self, value: bool) -> &mut Self {
            self.include_deleted = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Reply, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output.push_str("/replies/");
            output.push_str(&self.reply_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("includeDeleted", &self.include_deleted)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        comment_id: String,
        include_deleted: Option<bool>,
        page_size: Option<i32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Whether to include deleted replies. Deleted replies will not include their original content."]
        pub fn include_deleted(&mut self, value: bool) -> &mut Self {
            self.include_deleted = Some(value);
            self
        }
        #[doc = "The maximum number of replies to return per page."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_replies<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "replies")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::ReplyList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output.push_str("/replies");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("includeDeleted", &self.include_deleted)]);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Reply,
        file_id: String,
        comment_id: String,
        reply_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(self) -> Result<crate::schemas::Reply, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/comments/");
            output.push_str(&self.comment_id);
            output.push_str("/replies/");
            output.push_str(&self.reply_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod revisions {
    pub mod params {}
    pub struct RevisionsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> RevisionsActions<'a, A> {
        #[doc = "Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted."]
        pub fn delete(
            &self,
            file_id: impl Into<String>,
            revision_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
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
                file_id: file_id.into(),
                revision_id: revision_id.into(),
            }
        }
        #[doc = "Gets a revision's metadata or content by ID."]
        pub fn get(
            &self,
            file_id: impl Into<String>,
            revision_id: impl Into<String>,
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
                file_id: file_id.into(),
                revision_id: revision_id.into(),
                acknowledge_abuse: None,
            }
        }
        #[doc = "Lists a file's revisions."]
        pub fn list(&self, file_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                file_id: file_id.into(),
                page_size: None,
                page_token: None,
            }
        }
        #[doc = "Updates a revision with patch semantics."]
        pub fn update(
            &self,
            request: crate::schemas::Revision,
            file_id: impl Into<String>,
            revision_id: impl Into<String>,
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
                file_id: file_id.into(),
                revision_id: revision_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        revision_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/revisions/");
            output.push_str(&self.revision_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        revision_id: String,
        acknowledge_abuse: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media."]
        pub fn acknowledge_abuse(&mut self, value: bool) -> &mut Self {
            self.acknowledge_abuse = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        fn _download_path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/download/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/revisions/");
            output.push_str(&self.revision_id);
            output
        }
        pub fn download<W>(mut self, output: &mut W) -> Result<u64, Box<dyn ::std::error::Error>>
        where
            W: ::std::io::Write + ?Sized,
        {
            self.alt(crate::params::Alt::Media);
            Ok(self
                ._request(&self._path())
                .send()?
                .error_for_status()?
                .copy_to(output)?)
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Revision, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/revisions/");
            output.push_str(&self.revision_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("acknowledgeAbuse", &self.acknowledge_abuse)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        file_id: String,
        page_size: Option<i32>,
        page_token: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The maximum number of revisions to return per page."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_revisions<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "revisions")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::RevisionList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/revisions");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.metadata.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Revision,
        file_id: String,
        revision_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Revision, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("files/");
            output.push_str(&self.file_id);
            output.push_str("/revisions/");
            output.push_str(&self.revision_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod teamdrives {
    pub mod params {}
    pub struct TeamdrivesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> TeamdrivesActions<'a, A> {
        #[doc = "Deprecated use drives.create instead."]
        pub fn create(
            &self,
            request: crate::schemas::TeamDrive,
            request_id: impl Into<String>,
        ) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
                request_id: request_id.into(),
            }
        }
        #[doc = "Deprecated use drives.delete instead."]
        pub fn delete(&self, team_drive_id: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                team_drive_id: team_drive_id.into(),
            }
        }
        #[doc = "Deprecated use drives.get instead."]
        pub fn get(&self, team_drive_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                team_drive_id: team_drive_id.into(),
                use_domain_admin_access: None,
            }
        }
        #[doc = "Deprecated use drives.list instead."]
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
                page_size: None,
                page_token: None,
                q: None,
                use_domain_admin_access: None,
            }
        }
        #[doc = "Deprecated use drives.update instead"]
        pub fn update(
            &self,
            request: crate::schemas::TeamDrive,
            team_drive_id: impl Into<String>,
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
                team_drive_id: team_drive_id.into(),
                use_domain_admin_access: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::TeamDrive,
        request_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::TeamDrive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("teamdrives");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("requestId", &self.request_id)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        team_drive_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            req.send()?.error_for_status()?;
            Ok(())
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("teamdrives/");
            output.push_str(&self.team_drive_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::DELETE, path);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        team_drive_id: String,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::TeamDrive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("teamdrives/");
            output.push_str(&self.team_drive_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        page_size: Option<i32>,
        page_token: Option<String>,
        q: Option<String>,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Maximum number of Team Drives to return."]
        pub fn page_size(&mut self, value: i32) -> &mut Self {
            self.page_size = Some(value);
            self
        }
        #[doc = "Page token for Team Drives."]
        pub fn page_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.page_token = Some(value.into());
            self
        }
        #[doc = "Query string for searching Team Drives."]
        pub fn q(&mut self, value: impl Into<String>) -> &mut Self {
            self.q = Some(value.into());
            self
        }
        #[doc = "Issue the request as a domain administrator; if set to true, then all Team Drives of the domain in which the requester is an administrator are returned."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn iter_team_drives<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            struct ItemIter<'a, M, T> {
                method: &'a mut M,
                finished: bool,
                items_iter: Option<::std::vec::IntoIter<T>>,
            }
            impl<'a, M, T> Iterator for ItemIter<'a, M, T>
            where
                M: crate::IterableMethod,
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                type Item = Result<T, Box<dyn ::std::error::Error>>;
                fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
                    use ::field_selector::FieldSelector;
                    #[derive(:: serde :: Deserialize, FieldSelector)]
                    struct Resp<T>
                    where
                        T: FieldSelector,
                    {
                        #[serde(rename = "teamDrives")]
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
                        if self.finished {
                            return None;
                        }
                        let resp: Resp<T> = match self.method.execute() {
                            Ok(r) => r,
                            Err(err) => return Some(Err(err)),
                        };
                        if let Some(next_page_token) = resp.next_page_token {
                            self.method.set_page_token(next_page_token);
                        } else {
                            self.finished = true;
                        }
                        self.items_iter = resp.items.map(|i| i.into_iter());
                    }
                }
            }
            ItemIter {
                method: self,
                finished: false,
                items_iter: None,
            }
        }
        pub fn iter<T>(
            &'a mut self,
        ) -> impl Iterator<Item = Result<T, Box<dyn ::std::error::Error>>> + 'a
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector + 'a,
        {
            crate::PageIter {
                method: self,
                finished: false,
                _phantom: ::std::default::Default::default(),
            }
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::TeamDriveList, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("teamdrives");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pageSize", &self.page_size)]);
            let req = req.query(&[("pageToken", &self.page_token)]);
            let req = req.query(&[("q", &self.q)]);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
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
    #[derive(Debug, Clone)]
    pub struct UpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::TeamDrive,
        team_drive_id: String,
        use_domain_admin_access: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the Team Drive belongs."]
        pub fn use_domain_admin_access(&mut self, value: bool) -> &mut Self {
            self.use_domain_admin_access = Some(value);
            self
        }
        #[doc = "Data format for the response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Deprecated. Please use quotaUser instead."]
        pub fn user_ip(&mut self, value: impl Into<String>) -> &mut Self {
            self.user_ip = Some(value.into());
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::TeamDrive, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://www.googleapis.com/drive/v3/".to_owned();
            output.push_str("teamdrives/");
            output.push_str(&self.team_drive_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[("useDomainAdminAccess", &self.use_domain_admin_access)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
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

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
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
}
