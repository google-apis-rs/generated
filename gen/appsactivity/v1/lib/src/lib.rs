#![doc = "# Resources and Methods\n    * [activities](resources/activities/struct.ActivitiesActions.html)\n      * [*list*](resources/activities/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View the activity history of your Google apps\n\n`https://www.googleapis.com/auth/activity`"]
    pub const ACTIVITY: &str = "https://www.googleapis.com/auth/activity";
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
    pub struct Activity {
        #[doc = "The fields common to all of the singleEvents that make up the Activity."]
        #[serde(
            rename = "combinedEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub combined_event: ::std::option::Option<crate::schemas::Event>,
        #[doc = "A list of all the Events that make up the Activity."]
        #[serde(
            rename = "singleEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub single_events: ::std::option::Option<Vec<crate::schemas::Event>>,
    }
    impl ::google_field_selector::FieldSelector for Activity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Activity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Event {
        #[doc = "Additional event types. Some events may have multiple types when multiple actions are part of a single event. For example, creating a document, renaming it, and sharing it may be part of a single file-creation event."]
        #[serde(
            rename = "additionalEventTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_event_types:
            ::std::option::Option<Vec<crate::schemas::EventAdditionalEventTypesItems>>,
        #[doc = "The time at which the event occurred formatted as Unix time in milliseconds."]
        #[serde(
            rename = "eventTimeMillis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub event_time_millis: ::std::option::Option<u64>,
        #[doc = "Whether this event is caused by a user being deleted."]
        #[serde(
            rename = "fromUserDeletion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub from_user_deletion: ::std::option::Option<bool>,
        #[doc = "Extra information for permissionChange type events, such as the user or group the new permission applies to."]
        #[serde(
            rename = "permissionChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_changes: ::std::option::Option<Vec<crate::schemas::PermissionChange>>,
        #[doc = "The main type of event that occurred."]
        #[serde(
            rename = "primaryEventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_event_type: ::std::option::Option<crate::schemas::EventPrimaryEventType>,
        #[doc = "Extra information for move type events, such as changes in an object's parents."]
        #[serde(
            rename = "move",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#move: ::std::option::Option<crate::schemas::Move>,
        #[doc = "Extra information for rename type events, such as the old and new names."]
        #[serde(
            rename = "rename",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rename: ::std::option::Option<crate::schemas::Rename>,
        #[doc = "Information specific to the Target object modified by the event."]
        #[serde(
            rename = "target",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target: ::std::option::Option<crate::schemas::Target>,
        #[doc = "Represents the user responsible for the event."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for Event {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Event {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EventAdditionalEventTypesItems {
        Comment,
        Create,
        Edit,
        EmptyTrash,
        Move,
        PermissionChange,
        Rename,
        Trash,
        Unknown,
        Untrash,
        Upload,
    }
    impl EventAdditionalEventTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                EventAdditionalEventTypesItems::Comment => "comment",
                EventAdditionalEventTypesItems::Create => "create",
                EventAdditionalEventTypesItems::Edit => "edit",
                EventAdditionalEventTypesItems::EmptyTrash => "emptyTrash",
                EventAdditionalEventTypesItems::Move => "move",
                EventAdditionalEventTypesItems::PermissionChange => "permissionChange",
                EventAdditionalEventTypesItems::Rename => "rename",
                EventAdditionalEventTypesItems::Trash => "trash",
                EventAdditionalEventTypesItems::Unknown => "unknown",
                EventAdditionalEventTypesItems::Untrash => "untrash",
                EventAdditionalEventTypesItems::Upload => "upload",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EventAdditionalEventTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EventAdditionalEventTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EventAdditionalEventTypesItems, ()> {
            Ok(match s {
                "comment" => EventAdditionalEventTypesItems::Comment,
                "create" => EventAdditionalEventTypesItems::Create,
                "edit" => EventAdditionalEventTypesItems::Edit,
                "emptyTrash" => EventAdditionalEventTypesItems::EmptyTrash,
                "move" => EventAdditionalEventTypesItems::Move,
                "permissionChange" => EventAdditionalEventTypesItems::PermissionChange,
                "rename" => EventAdditionalEventTypesItems::Rename,
                "trash" => EventAdditionalEventTypesItems::Trash,
                "unknown" => EventAdditionalEventTypesItems::Unknown,
                "untrash" => EventAdditionalEventTypesItems::Untrash,
                "upload" => EventAdditionalEventTypesItems::Upload,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EventAdditionalEventTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EventAdditionalEventTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EventAdditionalEventTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "comment" => EventAdditionalEventTypesItems::Comment,
                "create" => EventAdditionalEventTypesItems::Create,
                "edit" => EventAdditionalEventTypesItems::Edit,
                "emptyTrash" => EventAdditionalEventTypesItems::EmptyTrash,
                "move" => EventAdditionalEventTypesItems::Move,
                "permissionChange" => EventAdditionalEventTypesItems::PermissionChange,
                "rename" => EventAdditionalEventTypesItems::Rename,
                "trash" => EventAdditionalEventTypesItems::Trash,
                "unknown" => EventAdditionalEventTypesItems::Unknown,
                "untrash" => EventAdditionalEventTypesItems::Untrash,
                "upload" => EventAdditionalEventTypesItems::Upload,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EventAdditionalEventTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventAdditionalEventTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EventPrimaryEventType {
        Comment,
        Create,
        Edit,
        EmptyTrash,
        Move,
        PermissionChange,
        Rename,
        Trash,
        Unknown,
        Untrash,
        Upload,
    }
    impl EventPrimaryEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                EventPrimaryEventType::Comment => "comment",
                EventPrimaryEventType::Create => "create",
                EventPrimaryEventType::Edit => "edit",
                EventPrimaryEventType::EmptyTrash => "emptyTrash",
                EventPrimaryEventType::Move => "move",
                EventPrimaryEventType::PermissionChange => "permissionChange",
                EventPrimaryEventType::Rename => "rename",
                EventPrimaryEventType::Trash => "trash",
                EventPrimaryEventType::Unknown => "unknown",
                EventPrimaryEventType::Untrash => "untrash",
                EventPrimaryEventType::Upload => "upload",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EventPrimaryEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EventPrimaryEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EventPrimaryEventType, ()> {
            Ok(match s {
                "comment" => EventPrimaryEventType::Comment,
                "create" => EventPrimaryEventType::Create,
                "edit" => EventPrimaryEventType::Edit,
                "emptyTrash" => EventPrimaryEventType::EmptyTrash,
                "move" => EventPrimaryEventType::Move,
                "permissionChange" => EventPrimaryEventType::PermissionChange,
                "rename" => EventPrimaryEventType::Rename,
                "trash" => EventPrimaryEventType::Trash,
                "unknown" => EventPrimaryEventType::Unknown,
                "untrash" => EventPrimaryEventType::Untrash,
                "upload" => EventPrimaryEventType::Upload,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EventPrimaryEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EventPrimaryEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EventPrimaryEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "comment" => EventPrimaryEventType::Comment,
                "create" => EventPrimaryEventType::Create,
                "edit" => EventPrimaryEventType::Edit,
                "emptyTrash" => EventPrimaryEventType::EmptyTrash,
                "move" => EventPrimaryEventType::Move,
                "permissionChange" => EventPrimaryEventType::PermissionChange,
                "rename" => EventPrimaryEventType::Rename,
                "trash" => EventPrimaryEventType::Trash,
                "unknown" => EventPrimaryEventType::Unknown,
                "untrash" => EventPrimaryEventType::Untrash,
                "upload" => EventPrimaryEventType::Upload,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EventPrimaryEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventPrimaryEventType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListActivitiesResponse {
        #[doc = "List of activities."]
        #[serde(
            rename = "activities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activities: ::std::option::Option<Vec<crate::schemas::Activity>>,
        #[doc = "Token for the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListActivitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListActivitiesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Move {
        #[doc = "The added parent(s)."]
        #[serde(
            rename = "addedParents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub added_parents: ::std::option::Option<Vec<crate::schemas::Parent>>,
        #[doc = "The removed parent(s)."]
        #[serde(
            rename = "removedParents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub removed_parents: ::std::option::Option<Vec<crate::schemas::Parent>>,
    }
    impl ::google_field_selector::FieldSelector for Move {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Move {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Parent {
        #[doc = "The parent's ID."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Whether this is the root folder."]
        #[serde(
            rename = "isRoot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_root: ::std::option::Option<bool>,
        #[doc = "The parent's title."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Parent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Parent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The name of the user or group the permission applies to."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID for this permission. Corresponds to the Drive API's permission ID returned as part of the Drive Permissions resource."]
        #[serde(
            rename = "permissionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_id: ::std::option::Option<String>,
        #[doc = "Indicates how widely permissions are granted."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::PermissionType>,
        #[doc = "Indicates the Google Drive permissions role. The role determines a user's ability to read, write, or comment on the file."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<crate::schemas::PermissionRole>,
        #[doc = "The user's information if the type is USER."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
        #[doc = "Whether the permission requires a link to the file."]
        #[serde(
            rename = "withLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub with_link: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Permission {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Permission {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PermissionType {
        Anyone,
        Domain,
        Group,
        User,
    }
    impl PermissionType {
        pub fn as_str(self) -> &'static str {
            match self {
                PermissionType::Anyone => "anyone",
                PermissionType::Domain => "domain",
                PermissionType::Group => "group",
                PermissionType::User => "user",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PermissionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PermissionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PermissionType, ()> {
            Ok(match s {
                "anyone" => PermissionType::Anyone,
                "domain" => PermissionType::Domain,
                "group" => PermissionType::Group,
                "user" => PermissionType::User,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PermissionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PermissionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PermissionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "anyone" => PermissionType::Anyone,
                "domain" => PermissionType::Domain,
                "group" => PermissionType::Group,
                "user" => PermissionType::User,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PermissionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PermissionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PermissionRole {
        Commenter,
        FileOrganizer,
        Owner,
        PublishedReader,
        Reader,
        Writer,
    }
    impl PermissionRole {
        pub fn as_str(self) -> &'static str {
            match self {
                PermissionRole::Commenter => "commenter",
                PermissionRole::FileOrganizer => "fileOrganizer",
                PermissionRole::Owner => "owner",
                PermissionRole::PublishedReader => "publishedReader",
                PermissionRole::Reader => "reader",
                PermissionRole::Writer => "writer",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PermissionRole {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PermissionRole {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PermissionRole, ()> {
            Ok(match s {
                "commenter" => PermissionRole::Commenter,
                "fileOrganizer" => PermissionRole::FileOrganizer,
                "owner" => PermissionRole::Owner,
                "publishedReader" => PermissionRole::PublishedReader,
                "reader" => PermissionRole::Reader,
                "writer" => PermissionRole::Writer,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PermissionRole {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PermissionRole {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PermissionRole {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "commenter" => PermissionRole::Commenter,
                "fileOrganizer" => PermissionRole::FileOrganizer,
                "owner" => PermissionRole::Owner,
                "publishedReader" => PermissionRole::PublishedReader,
                "reader" => PermissionRole::Reader,
                "writer" => PermissionRole::Writer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PermissionRole {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PermissionRole {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PermissionChange {
        #[doc = "Lists all Permission objects added."]
        #[serde(
            rename = "addedPermissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub added_permissions: ::std::option::Option<Vec<crate::schemas::Permission>>,
        #[doc = "Lists all Permission objects removed."]
        #[serde(
            rename = "removedPermissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub removed_permissions: ::std::option::Option<Vec<crate::schemas::Permission>>,
    }
    impl ::google_field_selector::FieldSelector for PermissionChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PermissionChange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Photo {
        #[doc = "The URL of the photo."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Photo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Photo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Rename {
        #[doc = "The new title."]
        #[serde(
            rename = "newTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_title: ::std::option::Option<String>,
        #[doc = "The old title."]
        #[serde(
            rename = "oldTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Rename {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Rename {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Target {
        #[doc = "The ID of the target. For example, in Google Drive, this is the file or folder ID."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The MIME type of the target."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The name of the target. For example, in Google Drive, this is the title of the file."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Target {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Target {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "A boolean which indicates whether the specified User was deleted. If true, name, photo and permission_id will be omitted."]
        #[serde(
            rename = "isDeleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_deleted: ::std::option::Option<bool>,
        #[doc = "Whether the user is the authenticated user."]
        #[serde(
            rename = "isMe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_me: ::std::option::Option<bool>,
        #[doc = "The displayable name of the user."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The permission ID associated with this user. Equivalent to the Drive API's permission ID for this user, returned as part of the Drive Permissions resource."]
        #[serde(
            rename = "permissionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_id: ::std::option::Option<String>,
        #[doc = "The profile photo of the user. Not present if the user has no profile photo."]
        #[serde(
            rename = "photo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo: ::std::option::Option<crate::schemas::Photo>,
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
    #[doc = "Actions that can be performed on the activities resource"]
    pub fn activities(&self) -> crate::resources::activities::ActivitiesActions {
        crate::resources::activities::ActivitiesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod activities {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListGroupingStrategy {
                DriveUi,
                None,
            }
            impl ListGroupingStrategy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListGroupingStrategy::DriveUi => "driveUi",
                        ListGroupingStrategy::None => "none",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListGroupingStrategy {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListGroupingStrategy {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListGroupingStrategy, ()> {
                    Ok(match s {
                        "driveUi" => ListGroupingStrategy::DriveUi,
                        "none" => ListGroupingStrategy::None,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListGroupingStrategy {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListGroupingStrategy {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListGroupingStrategy {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "driveUi" => ListGroupingStrategy::DriveUi,
                        "none" => ListGroupingStrategy::None,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListGroupingStrategy {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListGroupingStrategy {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ActivitiesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ActivitiesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns a list of activities visible to the current logged in user. Visible activities are determined by the visibility settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter."]
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
                    drive_ancestor_id: None,
                    drive_file_id: None,
                    grouping_strategy: None,
                    page_size: None,
                    page_token: None,
                    source: None,
                    user_id: None,
                }
            }
        }
        #[doc = "Created via [ActivitiesActions::list()](struct.ActivitiesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            drive_ancestor_id: Option<String>,
            drive_file_id: Option<String>,
            grouping_strategy: Option<crate::resources::activities::params::ListGroupingStrategy>,
            page_size: Option<i32>,
            page_token: Option<String>,
            source: Option<String>,
            user_id: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Identifies the Drive folder containing the items for which to return activities."]
            pub fn drive_ancestor_id(mut self, value: impl Into<String>) -> Self {
                self.drive_ancestor_id = Some(value.into());
                self
            }
            #[doc = "Identifies the Drive item to return activities for."]
            pub fn drive_file_id(mut self, value: impl Into<String>) -> Self {
                self.drive_file_id = Some(value.into());
                self
            }
            #[doc = "Indicates the strategy to use when grouping singleEvents items in the associated combinedEvent object."]
            pub fn grouping_strategy(
                mut self,
                value: crate::resources::activities::params::ListGroupingStrategy,
            ) -> Self {
                self.grouping_strategy = Some(value);
                self
            }
            #[doc = "The maximum number of events to return on a page. The response includes a continuation token if there are more events."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token to retrieve a specific page of results."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "The Google service from which to return activities. Possible values of source are: \n\n* drive.google.com"]
            pub fn source(mut self, value: impl Into<String>) -> Self {
                self.source = Some(value.into());
                self
            }
            #[doc = "The ID used for ACL checks (does not filter the resulting event list by the assigned value). Use the special value me to indicate the currently authenticated user."]
            pub fn user_id(mut self, value: impl Into<String>) -> Self {
                self.user_id = Some(value.into());
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
            pub fn iter_activities<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_activities_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_activities_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Activity> {
                self.iter_activities_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_activities_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Activity> {
                self.iter_activities_with_fields(Some("*"))
            }
            pub fn iter_activities_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "activities").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "activities")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListActivitiesResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListActivitiesResponse> {
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
            ) -> Result<crate::schemas::ListActivitiesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListActivitiesResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/appsactivity/v1/".to_owned();
                output.push_str("activities");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("drive.ancestorId", &self.drive_ancestor_id)]);
                let req = req.query(&[("drive.fileId", &self.drive_file_id)]);
                let req = req.query(&[("groupingStrategy", &self.grouping_strategy)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("source", &self.source)]);
                let req = req.query(&[("userId", &self.user_id)]);
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
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                todo!("implement async `execute` method for `IterableMethod` trait")
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
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
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
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
}
