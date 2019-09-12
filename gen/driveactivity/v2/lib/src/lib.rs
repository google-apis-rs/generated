#![doc = "# Resources and Methods\n    * [activity](resources/activity/struct.ActivityActions.html)\n      * [*query*](resources/activity/struct.QueryRequestBuilder.html)\n"]
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
    pub struct Action {
        #[doc = "The actor responsible for this action (or empty if all actors are\nresponsible)."]
        #[serde(
            rename = "actor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actor: ::std::option::Option<crate::schemas::Actor>,
        #[doc = "The type and detailed information about the action."]
        #[serde(
            rename = "detail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail: ::std::option::Option<crate::schemas::ActionDetail>,
        #[doc = "The target this action affects (or empty if affecting all targets). This\nrepresents the state of the target immediately after this action occurred."]
        #[serde(
            rename = "target",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target: ::std::option::Option<crate::schemas::Target>,
        #[doc = "The action occurred over this time range."]
        #[serde(
            rename = "timeRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_range: ::std::option::Option<crate::schemas::TimeRange>,
        #[doc = "The action occurred at this specific time."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Action {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Action {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ActionDetail {
        #[doc = "A change about comments was made."]
        #[serde(
            rename = "comment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comment: ::std::option::Option<crate::schemas::Comment>,
        #[doc = "An object was created."]
        #[serde(
            rename = "create",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create: ::std::option::Option<crate::schemas::Create>,
        #[doc = "An object was deleted."]
        #[serde(
            rename = "delete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete: ::std::option::Option<crate::schemas::Delete>,
        #[doc = "A change happened in data leak prevention status."]
        #[serde(
            rename = "dlpChange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dlp_change: ::std::option::Option<crate::schemas::DataLeakPreventionChange>,
        #[doc = "An object was edited."]
        #[serde(
            rename = "edit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub edit: ::std::option::Option<crate::schemas::Edit>,
        #[doc = "The permission on an object was changed."]
        #[serde(
            rename = "permissionChange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission_change: ::std::option::Option<crate::schemas::PermissionChange>,
        #[doc = "An object was moved."]
        #[serde(
            rename = "move",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#move: ::std::option::Option<crate::schemas::Move>,
        #[doc = "An object was referenced in an application outside of Drive/Docs."]
        #[serde(
            rename = "reference",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reference: ::std::option::Option<crate::schemas::ApplicationReference>,
        #[doc = "An object was renamed."]
        #[serde(
            rename = "rename",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rename: ::std::option::Option<crate::schemas::Rename>,
        #[doc = "A deleted object was restored."]
        #[serde(
            rename = "restore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restore: ::std::option::Option<crate::schemas::Restore>,
        #[doc = "Settings were changed."]
        #[serde(
            rename = "settingsChange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub settings_change: ::std::option::Option<crate::schemas::SettingsChange>,
    }
    impl ::google_field_selector::FieldSelector for ActionDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Actor {
        #[doc = "An administrator."]
        #[serde(
            rename = "administrator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub administrator: ::std::option::Option<crate::schemas::Administrator>,
        #[doc = "An anonymous user."]
        #[serde(
            rename = "anonymous",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub anonymous: ::std::option::Option<crate::schemas::AnonymousUser>,
        #[doc = "An account acting on behalf of another."]
        #[serde(
            rename = "impersonation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impersonation: ::std::option::Option<crate::schemas::Impersonation>,
        #[doc = "A non-user actor (i.e. system triggered)."]
        #[serde(
            rename = "system",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system: ::std::option::Option<crate::schemas::SystemEvent>,
        #[doc = "An end user."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for Actor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Actor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct Administrator;
    impl ::google_field_selector::FieldSelector for Administrator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Administrator {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct AnonymousUser;
    impl ::google_field_selector::FieldSelector for AnonymousUser {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnonymousUser {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct Anyone;
    impl ::google_field_selector::FieldSelector for Anyone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Anyone {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ApplicationReference {
        #[doc = "The reference type corresponding to this event."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ApplicationReferenceType>,
    }
    impl ::google_field_selector::FieldSelector for ApplicationReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ApplicationReferenceType {
        #[doc = "Comments were made regarding a Drive item."]
        Discuss,
        #[doc = "The links of one or more Drive items were posted."]
        Link,
        #[doc = "The type is not available."]
        UnspecifiedReferenceType,
    }
    impl ApplicationReferenceType {
        pub fn as_str(self) -> &'static str {
            match self {
                ApplicationReferenceType::Discuss => "DISCUSS",
                ApplicationReferenceType::Link => "LINK",
                ApplicationReferenceType::UnspecifiedReferenceType => "UNSPECIFIED_REFERENCE_TYPE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ApplicationReferenceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ApplicationReferenceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ApplicationReferenceType, ()> {
            Ok(match s {
                "DISCUSS" => ApplicationReferenceType::Discuss,
                "LINK" => ApplicationReferenceType::Link,
                "UNSPECIFIED_REFERENCE_TYPE" => ApplicationReferenceType::UnspecifiedReferenceType,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ApplicationReferenceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ApplicationReferenceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ApplicationReferenceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISCUSS" => ApplicationReferenceType::Discuss,
                "LINK" => ApplicationReferenceType::Link,
                "UNSPECIFIED_REFERENCE_TYPE" => ApplicationReferenceType::UnspecifiedReferenceType,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ApplicationReferenceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ApplicationReferenceType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Assignment {
        #[doc = "The sub-type of this event."]
        #[serde(
            rename = "subtype",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtype: ::std::option::Option<crate::schemas::AssignmentSubtype>,
    }
    impl ::google_field_selector::FieldSelector for Assignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Assignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssignmentSubtype {
        #[doc = "An assignment was added."]
        Added,
        #[doc = "An assignment was deleted."]
        Deleted,
        #[doc = "An assignment was reassigned."]
        Reassigned,
        #[doc = "A resolved assignment was reopened."]
        Reopened,
        #[doc = "An assignment reply was added."]
        ReplyAdded,
        #[doc = "An assignment reply was deleted."]
        ReplyDeleted,
        #[doc = "An assignment was resolved."]
        Resolved,
        #[doc = "Subtype not available."]
        SubtypeUnspecified,
    }
    impl AssignmentSubtype {
        pub fn as_str(self) -> &'static str {
            match self {
                AssignmentSubtype::Added => "ADDED",
                AssignmentSubtype::Deleted => "DELETED",
                AssignmentSubtype::Reassigned => "REASSIGNED",
                AssignmentSubtype::Reopened => "REOPENED",
                AssignmentSubtype::ReplyAdded => "REPLY_ADDED",
                AssignmentSubtype::ReplyDeleted => "REPLY_DELETED",
                AssignmentSubtype::Resolved => "RESOLVED",
                AssignmentSubtype::SubtypeUnspecified => "SUBTYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AssignmentSubtype {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AssignmentSubtype {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AssignmentSubtype, ()> {
            Ok(match s {
                "ADDED" => AssignmentSubtype::Added,
                "DELETED" => AssignmentSubtype::Deleted,
                "REASSIGNED" => AssignmentSubtype::Reassigned,
                "REOPENED" => AssignmentSubtype::Reopened,
                "REPLY_ADDED" => AssignmentSubtype::ReplyAdded,
                "REPLY_DELETED" => AssignmentSubtype::ReplyDeleted,
                "RESOLVED" => AssignmentSubtype::Resolved,
                "SUBTYPE_UNSPECIFIED" => AssignmentSubtype::SubtypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AssignmentSubtype {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssignmentSubtype {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssignmentSubtype {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDED" => AssignmentSubtype::Added,
                "DELETED" => AssignmentSubtype::Deleted,
                "REASSIGNED" => AssignmentSubtype::Reassigned,
                "REOPENED" => AssignmentSubtype::Reopened,
                "REPLY_ADDED" => AssignmentSubtype::ReplyAdded,
                "REPLY_DELETED" => AssignmentSubtype::ReplyDeleted,
                "RESOLVED" => AssignmentSubtype::Resolved,
                "SUBTYPE_UNSPECIFIED" => AssignmentSubtype::SubtypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AssignmentSubtype {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AssignmentSubtype {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "A change on an assignment."]
        #[serde(
            rename = "assignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assignment: ::std::option::Option<crate::schemas::Assignment>,
        #[doc = "Users who are mentioned in this comment."]
        #[serde(
            rename = "mentionedUsers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mentioned_users: ::std::option::Option<Vec<crate::schemas::User>>,
        #[doc = "A change on a regular posted comment."]
        #[serde(
            rename = "post",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub post: ::std::option::Option<crate::schemas::Post>,
        #[doc = "A change on a suggestion."]
        #[serde(
            rename = "suggestion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggestion: ::std::option::Option<crate::schemas::Suggestion>,
    }
    impl ::google_field_selector::FieldSelector for Comment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Comment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ConsolidationStrategy {
        #[doc = "The individual activities are consolidated using the legacy strategy."]
        #[serde(
            rename = "legacy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy: ::std::option::Option<crate::schemas::Legacy>,
        #[doc = "The individual activities are not consolidated."]
        #[serde(
            rename = "none",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub none: ::std::option::Option<crate::schemas::NoConsolidation>,
    }
    impl ::google_field_selector::FieldSelector for ConsolidationStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConsolidationStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Copy {
        #[doc = "The the original object."]
        #[serde(
            rename = "originalObject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub original_object: ::std::option::Option<crate::schemas::TargetReference>,
    }
    impl ::google_field_selector::FieldSelector for Copy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Copy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Create {
        #[doc = "If present, indicates the object was created by copying an existing Drive\nobject."]
        #[serde(
            rename = "copy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub copy: ::std::option::Option<crate::schemas::Copy>,
        #[doc = "If present, indicates the object was newly created (e.g. as a blank\ndocument), not derived from a Drive object or external object."]
        #[serde(
            rename = "new",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new: ::std::option::Option<crate::schemas::New>,
        #[doc = "If present, indicates the object originated externally and was uploaded\nto Drive."]
        #[serde(
            rename = "upload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upload: ::std::option::Option<crate::schemas::Upload>,
    }
    impl ::google_field_selector::FieldSelector for Create {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Create {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DataLeakPreventionChange {
        #[doc = "The type of Data Leak Prevention (DLP) change."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DataLeakPreventionChangeType>,
    }
    impl ::google_field_selector::FieldSelector for DataLeakPreventionChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataLeakPreventionChange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataLeakPreventionChangeType {
        #[doc = "Document is no longer flagged as containing sensitive content."]
        Cleared,
        #[doc = "Document has been flagged as containing sensitive content."]
        Flagged,
        #[doc = "An update to the DLP state that is neither FLAGGED or CLEARED."]
        TypeUnspecified,
    }
    impl DataLeakPreventionChangeType {
        pub fn as_str(self) -> &'static str {
            match self {
                DataLeakPreventionChangeType::Cleared => "CLEARED",
                DataLeakPreventionChangeType::Flagged => "FLAGGED",
                DataLeakPreventionChangeType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DataLeakPreventionChangeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DataLeakPreventionChangeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DataLeakPreventionChangeType, ()> {
            Ok(match s {
                "CLEARED" => DataLeakPreventionChangeType::Cleared,
                "FLAGGED" => DataLeakPreventionChangeType::Flagged,
                "TYPE_UNSPECIFIED" => DataLeakPreventionChangeType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DataLeakPreventionChangeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataLeakPreventionChangeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataLeakPreventionChangeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLEARED" => DataLeakPreventionChangeType::Cleared,
                "FLAGGED" => DataLeakPreventionChangeType::Flagged,
                "TYPE_UNSPECIFIED" => DataLeakPreventionChangeType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DataLeakPreventionChangeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataLeakPreventionChangeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Delete {
        #[doc = "The type of delete action taken."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DeleteType>,
    }
    impl ::google_field_selector::FieldSelector for Delete {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Delete {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeleteType {
        #[doc = "An object was deleted permanently."]
        PermanentDelete,
        #[doc = "An object was put into the trash."]
        Trash,
        #[doc = "Deletion type is not available."]
        TypeUnspecified,
    }
    impl DeleteType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeleteType::PermanentDelete => "PERMANENT_DELETE",
                DeleteType::Trash => "TRASH",
                DeleteType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeleteType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeleteType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeleteType, ()> {
            Ok(match s {
                "PERMANENT_DELETE" => DeleteType::PermanentDelete,
                "TRASH" => DeleteType::Trash,
                "TYPE_UNSPECIFIED" => DeleteType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeleteType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeleteType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeleteType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PERMANENT_DELETE" => DeleteType::PermanentDelete,
                "TRASH" => DeleteType::Trash,
                "TYPE_UNSPECIFIED" => DeleteType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeleteType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct DeletedUser;
    impl ::google_field_selector::FieldSelector for DeletedUser {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeletedUser {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Domain {
        #[doc = "An opaque string used to identify this domain."]
        #[serde(
            rename = "legacyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy_id: ::std::option::Option<String>,
        #[doc = "The name of the domain, e.g. \"google.com\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Domain {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Domain {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Drive {
        #[doc = "The resource name of the shared drive. The format is\n\"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection\nID for this resource name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The root of this shared drive."]
        #[serde(
            rename = "root",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root: ::std::option::Option<crate::schemas::DriveItem>,
        #[doc = "The title of the shared drive."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Drive {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Drive {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveActivity {
        #[doc = "Details on all actions in this activity."]
        #[serde(
            rename = "actions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actions: ::std::option::Option<Vec<crate::schemas::Action>>,
        #[doc = "All actor(s) responsible for the activity."]
        #[serde(
            rename = "actors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub actors: ::std::option::Option<Vec<crate::schemas::Actor>>,
        #[doc = "Key information about the primary action for this activity. This is either\nrepresentative, or the most important, of all actions in the activity,\naccording to the ConsolidationStrategy in the request."]
        #[serde(
            rename = "primaryActionDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_action_detail: ::std::option::Option<crate::schemas::ActionDetail>,
        #[doc = "All Google Drive objects this activity is about (e.g. file, folder, drive).\nThis represents the state of the target immediately after the actions\noccurred."]
        #[serde(
            rename = "targets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub targets: ::std::option::Option<Vec<crate::schemas::Target>>,
        #[doc = "The activity occurred over this time range."]
        #[serde(
            rename = "timeRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_range: ::std::option::Option<crate::schemas::TimeRange>,
        #[doc = "The activity occurred at this specific time."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DriveActivity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveActivity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct DriveFile;
    impl ::google_field_selector::FieldSelector for DriveFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveFolder {
        #[doc = "The type of Drive folder."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DriveFolderType>,
    }
    impl ::google_field_selector::FieldSelector for DriveFolder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveFolder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveFolderType {
        #[doc = "The folder is the root of a user's MyDrive."]
        MyDriveRoot,
        #[doc = "The folder is the root of a shared drive."]
        SharedDriveRoot,
        #[doc = "The folder is a standard, non-root, folder."]
        StandardFolder,
        #[doc = "The folder type is unknown."]
        TypeUnspecified,
    }
    impl DriveFolderType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveFolderType::MyDriveRoot => "MY_DRIVE_ROOT",
                DriveFolderType::SharedDriveRoot => "SHARED_DRIVE_ROOT",
                DriveFolderType::StandardFolder => "STANDARD_FOLDER",
                DriveFolderType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DriveFolderType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DriveFolderType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DriveFolderType, ()> {
            Ok(match s {
                "MY_DRIVE_ROOT" => DriveFolderType::MyDriveRoot,
                "SHARED_DRIVE_ROOT" => DriveFolderType::SharedDriveRoot,
                "STANDARD_FOLDER" => DriveFolderType::StandardFolder,
                "TYPE_UNSPECIFIED" => DriveFolderType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DriveFolderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveFolderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveFolderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MY_DRIVE_ROOT" => DriveFolderType::MyDriveRoot,
                "SHARED_DRIVE_ROOT" => DriveFolderType::SharedDriveRoot,
                "STANDARD_FOLDER" => DriveFolderType::StandardFolder,
                "TYPE_UNSPECIFIED" => DriveFolderType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveFolderType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveFolderType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveItem {
        #[doc = "The Drive item is a file."]
        #[serde(
            rename = "driveFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_file: ::std::option::Option<crate::schemas::DriveFile>,
        #[doc = "The Drive item is a folder. Includes information about the type of\nfolder."]
        #[serde(
            rename = "driveFolder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_folder: ::std::option::Option<crate::schemas::DriveFolder>,
        #[doc = "This field is deprecated; please use the `driveFile` field instead."]
        #[serde(
            rename = "file",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file: ::std::option::Option<crate::schemas::File>,
        #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
        #[serde(
            rename = "folder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folder: ::std::option::Option<crate::schemas::Folder>,
        #[doc = "The MIME type of the Drive item.  See\nhttps://developers.google.com/drive/v3/web/mime-types."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Information about the owner of this Drive item."]
        #[serde(
            rename = "owner",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owner: ::std::option::Option<crate::schemas::Owner>,
        #[doc = "The title of the Drive item."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DriveItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveItemReference {
        #[doc = "The Drive item is a file."]
        #[serde(
            rename = "driveFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_file: ::std::option::Option<crate::schemas::DriveFile>,
        #[doc = "The Drive item is a folder. Includes information about the type of\nfolder."]
        #[serde(
            rename = "driveFolder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_folder: ::std::option::Option<crate::schemas::DriveFolder>,
        #[doc = "This field is deprecated; please use the `driveFile` field instead."]
        #[serde(
            rename = "file",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file: ::std::option::Option<crate::schemas::File>,
        #[doc = "This field is deprecated; please use the `driveFolder` field instead."]
        #[serde(
            rename = "folder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folder: ::std::option::Option<crate::schemas::Folder>,
        #[doc = "The target Drive item. The format is \"items/ITEM_ID\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The title of the Drive item."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DriveItemReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveItemReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveReference {
        #[doc = "The resource name of the shared drive. The format is\n\"COLLECTION_ID/DRIVE_ID\". Clients should not assume a specific collection\nID for this resource name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The title of the shared drive."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DriveReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct Edit;
    impl ::google_field_selector::FieldSelector for Edit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Edit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct File;
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
    pub struct FileComment {
        #[doc = "The comment in the discussion thread. This identifier is an opaque string\ncompatible with the Drive API; see\nhttps://developers.google.com/drive/v3/reference/comments/get"]
        #[serde(
            rename = "legacyCommentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy_comment_id: ::std::option::Option<String>,
        #[doc = "The discussion thread to which the comment was added. This identifier is an\nopaque string compatible with the Drive API and references the first\ncomment in a discussion; see\nhttps://developers.google.com/drive/v3/reference/comments/get"]
        #[serde(
            rename = "legacyDiscussionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy_discussion_id: ::std::option::Option<String>,
        #[doc = "The link to the discussion thread containing this comment, for example,\n\"https://docs.google.com/DOCUMENT_ID/edit?disco=THREAD_ID\"."]
        #[serde(
            rename = "linkToDiscussion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_to_discussion: ::std::option::Option<String>,
        #[doc = "The Drive item containing this comment."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<crate::schemas::DriveItem>,
    }
    impl ::google_field_selector::FieldSelector for FileComment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileComment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Folder {
        #[doc = "This field is deprecated; please see `DriveFolder.type` instead."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::FolderType>,
    }
    impl ::google_field_selector::FieldSelector for Folder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Folder {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FolderType {
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        MyDriveRoot,
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        StandardFolder,
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        TeamDriveRoot,
        #[doc = "This item is deprecated; please see `DriveFolder.Type` instead."]
        TypeUnspecified,
    }
    impl FolderType {
        pub fn as_str(self) -> &'static str {
            match self {
                FolderType::MyDriveRoot => "MY_DRIVE_ROOT",
                FolderType::StandardFolder => "STANDARD_FOLDER",
                FolderType::TeamDriveRoot => "TEAM_DRIVE_ROOT",
                FolderType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FolderType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FolderType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FolderType, ()> {
            Ok(match s {
                "MY_DRIVE_ROOT" => FolderType::MyDriveRoot,
                "STANDARD_FOLDER" => FolderType::StandardFolder,
                "TEAM_DRIVE_ROOT" => FolderType::TeamDriveRoot,
                "TYPE_UNSPECIFIED" => FolderType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FolderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FolderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FolderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MY_DRIVE_ROOT" => FolderType::MyDriveRoot,
                "STANDARD_FOLDER" => FolderType::StandardFolder,
                "TEAM_DRIVE_ROOT" => FolderType::TeamDriveRoot,
                "TYPE_UNSPECIFIED" => FolderType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FolderType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FolderType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Group {
        #[doc = "The email address of the group."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The title of the group."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
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
    pub struct Impersonation {
        #[doc = "The impersonated user."]
        #[serde(
            rename = "impersonatedUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impersonated_user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for Impersonation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Impersonation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct KnownUser {
        #[doc = "True if this is the user making the request."]
        #[serde(
            rename = "isCurrentUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_current_user: ::std::option::Option<bool>,
        #[doc = "The identifier for this user that can be used with the People API to get\nmore information. The format is \"people/ACCOUNT_ID\". See\nhttps://developers.google.com/people/."]
        #[serde(
            rename = "personName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for KnownUser {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KnownUser {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct Legacy;
    impl ::google_field_selector::FieldSelector for Legacy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Legacy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The added parent object(s)."]
        #[serde(
            rename = "addedParents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub added_parents: ::std::option::Option<Vec<crate::schemas::TargetReference>>,
        #[doc = "The removed parent object(s)."]
        #[serde(
            rename = "removedParents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub removed_parents: ::std::option::Option<Vec<crate::schemas::TargetReference>>,
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct New;
    impl ::google_field_selector::FieldSelector for New {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for New {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct NoConsolidation;
    impl ::google_field_selector::FieldSelector for NoConsolidation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NoConsolidation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Owner {
        #[doc = "The domain of the Drive item owner."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<crate::schemas::Domain>,
        #[doc = "The drive that owns the item."]
        #[serde(
            rename = "drive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive: ::std::option::Option<crate::schemas::DriveReference>,
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        #[serde(
            rename = "teamDrive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub team_drive: ::std::option::Option<crate::schemas::TeamDriveReference>,
        #[doc = "The user that owns the Drive item."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for Owner {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Owner {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "If true, the item can be discovered (e.g. in the user's \"Shared with me\"\ncollection) without needing a link to the item."]
        #[serde(
            rename = "allowDiscovery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_discovery: ::std::option::Option<bool>,
        #[doc = "If set, this permission applies to anyone, even logged out users."]
        #[serde(
            rename = "anyone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub anyone: ::std::option::Option<crate::schemas::Anyone>,
        #[doc = "The domain to whom this permission applies."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<crate::schemas::Domain>,
        #[doc = "The group to whom this permission applies."]
        #[serde(
            rename = "group",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group: ::std::option::Option<crate::schemas::Group>,
        #[doc = "Indicates the\n<a href=\"/drive/web/manage-sharing#roles\">Google Drive permissions\nrole</a>. The role determines a user's ability to read, write, and\ncomment on items."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<crate::schemas::PermissionRole>,
        #[doc = "The user to whom this permission applies."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
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
    pub enum PermissionRole {
        #[doc = "A role granting the ability to view and comment on content."]
        Commenter,
        #[doc = "A role granting the ability to contribute content. This role is sometimes\nalso known as \"writer\"."]
        Editor,
        #[doc = "A role granting the ability to contribute and manage content."]
        FileOrganizer,
        #[doc = "A role granting the ability to manage people and settings."]
        Organizer,
        #[doc = "A role granting full access."]
        Owner,
        #[doc = "A role granting the ability to view content only after it has been\npublished to the web. This role is sometimes also known as \"published\nreader\". See https://support.google.com/sites/answer/6372880 for more\ninformation."]
        PublishedViewer,
        #[doc = "The role is not available."]
        RoleUnspecified,
        #[doc = "A role granting the ability to view content. This role is sometimes also\nknown as \"reader\"."]
        Viewer,
    }
    impl PermissionRole {
        pub fn as_str(self) -> &'static str {
            match self {
                PermissionRole::Commenter => "COMMENTER",
                PermissionRole::Editor => "EDITOR",
                PermissionRole::FileOrganizer => "FILE_ORGANIZER",
                PermissionRole::Organizer => "ORGANIZER",
                PermissionRole::Owner => "OWNER",
                PermissionRole::PublishedViewer => "PUBLISHED_VIEWER",
                PermissionRole::RoleUnspecified => "ROLE_UNSPECIFIED",
                PermissionRole::Viewer => "VIEWER",
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
                "COMMENTER" => PermissionRole::Commenter,
                "EDITOR" => PermissionRole::Editor,
                "FILE_ORGANIZER" => PermissionRole::FileOrganizer,
                "ORGANIZER" => PermissionRole::Organizer,
                "OWNER" => PermissionRole::Owner,
                "PUBLISHED_VIEWER" => PermissionRole::PublishedViewer,
                "ROLE_UNSPECIFIED" => PermissionRole::RoleUnspecified,
                "VIEWER" => PermissionRole::Viewer,
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
                "COMMENTER" => PermissionRole::Commenter,
                "EDITOR" => PermissionRole::Editor,
                "FILE_ORGANIZER" => PermissionRole::FileOrganizer,
                "ORGANIZER" => PermissionRole::Organizer,
                "OWNER" => PermissionRole::Owner,
                "PUBLISHED_VIEWER" => PermissionRole::PublishedViewer,
                "ROLE_UNSPECIFIED" => PermissionRole::RoleUnspecified,
                "VIEWER" => PermissionRole::Viewer,
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
        #[doc = "The set of permissions added by this change."]
        #[serde(
            rename = "addedPermissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub added_permissions: ::std::option::Option<Vec<crate::schemas::Permission>>,
        #[doc = "The set of permissions removed by this change."]
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
    pub struct Post {
        #[doc = "The sub-type of this event."]
        #[serde(
            rename = "subtype",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtype: ::std::option::Option<crate::schemas::PostSubtype>,
    }
    impl ::google_field_selector::FieldSelector for Post {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Post {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PostSubtype {
        #[doc = "A post was added."]
        Added,
        #[doc = "A post was deleted."]
        Deleted,
        #[doc = "A posted comment was reopened."]
        Reopened,
        #[doc = "A reply was added."]
        ReplyAdded,
        #[doc = "A reply was deleted."]
        ReplyDeleted,
        #[doc = "A posted comment was resolved."]
        Resolved,
        #[doc = "Subtype not available."]
        SubtypeUnspecified,
    }
    impl PostSubtype {
        pub fn as_str(self) -> &'static str {
            match self {
                PostSubtype::Added => "ADDED",
                PostSubtype::Deleted => "DELETED",
                PostSubtype::Reopened => "REOPENED",
                PostSubtype::ReplyAdded => "REPLY_ADDED",
                PostSubtype::ReplyDeleted => "REPLY_DELETED",
                PostSubtype::Resolved => "RESOLVED",
                PostSubtype::SubtypeUnspecified => "SUBTYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PostSubtype {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PostSubtype {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PostSubtype, ()> {
            Ok(match s {
                "ADDED" => PostSubtype::Added,
                "DELETED" => PostSubtype::Deleted,
                "REOPENED" => PostSubtype::Reopened,
                "REPLY_ADDED" => PostSubtype::ReplyAdded,
                "REPLY_DELETED" => PostSubtype::ReplyDeleted,
                "RESOLVED" => PostSubtype::Resolved,
                "SUBTYPE_UNSPECIFIED" => PostSubtype::SubtypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PostSubtype {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PostSubtype {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PostSubtype {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDED" => PostSubtype::Added,
                "DELETED" => PostSubtype::Deleted,
                "REOPENED" => PostSubtype::Reopened,
                "REPLY_ADDED" => PostSubtype::ReplyAdded,
                "REPLY_DELETED" => PostSubtype::ReplyDeleted,
                "RESOLVED" => PostSubtype::Resolved,
                "SUBTYPE_UNSPECIFIED" => PostSubtype::SubtypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PostSubtype {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PostSubtype {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct QueryDriveActivityRequest {
        #[doc = "Return activities for this Drive folder and all children and descendants.\nThe format is \"items/ITEM_ID\"."]
        #[serde(
            rename = "ancestorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ancestor_name: ::std::option::Option<String>,
        #[doc = "Details on how to consolidate related actions that make up the activity. If\nnot set, then related actions will not be consolidated."]
        #[serde(
            rename = "consolidationStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consolidation_strategy: ::std::option::Option<crate::schemas::ConsolidationStrategy>,
        #[doc = "The filtering for items returned from this query request. The format of the\nfilter string is a sequence of expressions, joined by an optional \"AND\",\nwhere each expression is of the form \"field operator value\".\n\nSupported fields:\n\n* <tt>time</tt>: Uses numerical operators on date values either in\n  terms of milliseconds since Jan 1, 1970 or in RFC 3339 format.\n  Examples:\n  \n  * <tt>time > 1452409200000 AND time <= 1492812924310</tt>\n  * <tt>time >= \"2016-01-10T01:02:03-05:00\"</tt>\n* <tt>detail.action_detail_case</tt>: Uses the \"has\" operator (:) and\n  either a singular value or a list of allowed action types enclosed in\n  parentheses.\n  Examples:\n  \n  * <tt>detail.action_detail_case: RENAME</tt>\n  * <tt>detail.action_detail_case:(CREATE UPLOAD)</tt>\n  * <tt>-detail.action_detail_case:MOVE</tt>"]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Return activities for this Drive item. The format is\n\"items/ITEM_ID\"."]
        #[serde(
            rename = "itemName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_name: ::std::option::Option<String>,
        #[doc = "The requested number of activity to return. If not set, a default value\nwill be used."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "The next_page_token value returned from a previous QueryDriveActivity\nrequest, if any."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for QueryDriveActivityRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryDriveActivityRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct QueryDriveActivityResponse {
        #[doc = "List of activity requested."]
        #[serde(
            rename = "activities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activities: ::std::option::Option<Vec<crate::schemas::DriveActivity>>,
        #[doc = "Token to retrieve the next page of results, or\nempty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for QueryDriveActivityResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryDriveActivityResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The new title of the drive object."]
        #[serde(
            rename = "newTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_title: ::std::option::Option<String>,
        #[doc = "The previous title of the drive object."]
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
    pub struct Restore {
        #[doc = "The type of restore action taken."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::RestoreType>,
    }
    impl ::google_field_selector::FieldSelector for Restore {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Restore {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RestoreType {
        #[doc = "The type is not available."]
        TypeUnspecified,
        #[doc = "An object was restored from the trash."]
        Untrash,
    }
    impl RestoreType {
        pub fn as_str(self) -> &'static str {
            match self {
                RestoreType::TypeUnspecified => "TYPE_UNSPECIFIED",
                RestoreType::Untrash => "UNTRASH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RestoreType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RestoreType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RestoreType, ()> {
            Ok(match s {
                "TYPE_UNSPECIFIED" => RestoreType::TypeUnspecified,
                "UNTRASH" => RestoreType::Untrash,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RestoreType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RestoreType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RestoreType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNSPECIFIED" => RestoreType::TypeUnspecified,
                "UNTRASH" => RestoreType::Untrash,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RestoreType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestoreType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RestrictionChange {
        #[doc = "The feature which had a change in restriction policy."]
        #[serde(
            rename = "feature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature: ::std::option::Option<crate::schemas::RestrictionChangeFeature>,
        #[doc = "The restriction in place after the change."]
        #[serde(
            rename = "newRestriction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_restriction: ::std::option::Option<crate::schemas::RestrictionChangeNewRestriction>,
    }
    impl ::google_field_selector::FieldSelector for RestrictionChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestrictionChange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RestrictionChangeFeature {
        #[doc = "When restricted, this prevents direct sharing of individual items."]
        DirectSharing,
        #[doc = "When restricted, this prevents use of Drive File Stream."]
        DriveFileStream,
        #[doc = "The feature which changed restriction settings was not available."]
        FeatureUnspecified,
        #[doc = "When restricted, this prevents actions like copy, download, and print\nthat might result in uncontrolled duplicates of items."]
        ItemDuplication,
        #[doc = "When restricted, this prevents items from being shared outside the\ndomain."]
        SharingOutsideDomain,
    }
    impl RestrictionChangeFeature {
        pub fn as_str(self) -> &'static str {
            match self {
                RestrictionChangeFeature::DirectSharing => "DIRECT_SHARING",
                RestrictionChangeFeature::DriveFileStream => "DRIVE_FILE_STREAM",
                RestrictionChangeFeature::FeatureUnspecified => "FEATURE_UNSPECIFIED",
                RestrictionChangeFeature::ItemDuplication => "ITEM_DUPLICATION",
                RestrictionChangeFeature::SharingOutsideDomain => "SHARING_OUTSIDE_DOMAIN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RestrictionChangeFeature {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RestrictionChangeFeature {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RestrictionChangeFeature, ()> {
            Ok(match s {
                "DIRECT_SHARING" => RestrictionChangeFeature::DirectSharing,
                "DRIVE_FILE_STREAM" => RestrictionChangeFeature::DriveFileStream,
                "FEATURE_UNSPECIFIED" => RestrictionChangeFeature::FeatureUnspecified,
                "ITEM_DUPLICATION" => RestrictionChangeFeature::ItemDuplication,
                "SHARING_OUTSIDE_DOMAIN" => RestrictionChangeFeature::SharingOutsideDomain,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RestrictionChangeFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RestrictionChangeFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RestrictionChangeFeature {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIRECT_SHARING" => RestrictionChangeFeature::DirectSharing,
                "DRIVE_FILE_STREAM" => RestrictionChangeFeature::DriveFileStream,
                "FEATURE_UNSPECIFIED" => RestrictionChangeFeature::FeatureUnspecified,
                "ITEM_DUPLICATION" => RestrictionChangeFeature::ItemDuplication,
                "SHARING_OUTSIDE_DOMAIN" => RestrictionChangeFeature::SharingOutsideDomain,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RestrictionChangeFeature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestrictionChangeFeature {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RestrictionChangeNewRestriction {
        #[doc = "The use of this feature is fully restricted."]
        FullyRestricted,
        #[doc = "The type of restriction is not available."]
        RestrictionUnspecified,
        #[doc = "The feature is available without restriction."]
        Unrestricted,
    }
    impl RestrictionChangeNewRestriction {
        pub fn as_str(self) -> &'static str {
            match self {
                RestrictionChangeNewRestriction::FullyRestricted => "FULLY_RESTRICTED",
                RestrictionChangeNewRestriction::RestrictionUnspecified => {
                    "RESTRICTION_UNSPECIFIED"
                }
                RestrictionChangeNewRestriction::Unrestricted => "UNRESTRICTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RestrictionChangeNewRestriction {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RestrictionChangeNewRestriction {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RestrictionChangeNewRestriction, ()> {
            Ok(match s {
                "FULLY_RESTRICTED" => RestrictionChangeNewRestriction::FullyRestricted,
                "RESTRICTION_UNSPECIFIED" => {
                    RestrictionChangeNewRestriction::RestrictionUnspecified
                }
                "UNRESTRICTED" => RestrictionChangeNewRestriction::Unrestricted,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RestrictionChangeNewRestriction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RestrictionChangeNewRestriction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RestrictionChangeNewRestriction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FULLY_RESTRICTED" => RestrictionChangeNewRestriction::FullyRestricted,
                "RESTRICTION_UNSPECIFIED" => {
                    RestrictionChangeNewRestriction::RestrictionUnspecified
                }
                "UNRESTRICTED" => RestrictionChangeNewRestriction::Unrestricted,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RestrictionChangeNewRestriction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestrictionChangeNewRestriction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SettingsChange {
        #[doc = "The set of changes made to restrictions."]
        #[serde(
            rename = "restrictionChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restriction_changes: ::std::option::Option<Vec<crate::schemas::RestrictionChange>>,
    }
    impl ::google_field_selector::FieldSelector for SettingsChange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SettingsChange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Suggestion {
        #[doc = "The sub-type of this event."]
        #[serde(
            rename = "subtype",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtype: ::std::option::Option<crate::schemas::SuggestionSubtype>,
    }
    impl ::google_field_selector::FieldSelector for Suggestion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Suggestion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SuggestionSubtype {
        #[doc = "An accepted suggestion was deleted."]
        AcceptDeleted,
        #[doc = "A suggestion was accepted."]
        Accepted,
        #[doc = "A suggestion was added."]
        Added,
        #[doc = "A suggestion was deleted."]
        Deleted,
        #[doc = "A rejected suggestion was deleted."]
        RejectDeleted,
        #[doc = "A suggestion was rejected."]
        Rejected,
        #[doc = "A suggestion reply was added."]
        ReplyAdded,
        #[doc = "A suggestion reply was deleted."]
        ReplyDeleted,
        #[doc = "Subtype not available."]
        SubtypeUnspecified,
    }
    impl SuggestionSubtype {
        pub fn as_str(self) -> &'static str {
            match self {
                SuggestionSubtype::AcceptDeleted => "ACCEPT_DELETED",
                SuggestionSubtype::Accepted => "ACCEPTED",
                SuggestionSubtype::Added => "ADDED",
                SuggestionSubtype::Deleted => "DELETED",
                SuggestionSubtype::RejectDeleted => "REJECT_DELETED",
                SuggestionSubtype::Rejected => "REJECTED",
                SuggestionSubtype::ReplyAdded => "REPLY_ADDED",
                SuggestionSubtype::ReplyDeleted => "REPLY_DELETED",
                SuggestionSubtype::SubtypeUnspecified => "SUBTYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SuggestionSubtype {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SuggestionSubtype {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SuggestionSubtype, ()> {
            Ok(match s {
                "ACCEPT_DELETED" => SuggestionSubtype::AcceptDeleted,
                "ACCEPTED" => SuggestionSubtype::Accepted,
                "ADDED" => SuggestionSubtype::Added,
                "DELETED" => SuggestionSubtype::Deleted,
                "REJECT_DELETED" => SuggestionSubtype::RejectDeleted,
                "REJECTED" => SuggestionSubtype::Rejected,
                "REPLY_ADDED" => SuggestionSubtype::ReplyAdded,
                "REPLY_DELETED" => SuggestionSubtype::ReplyDeleted,
                "SUBTYPE_UNSPECIFIED" => SuggestionSubtype::SubtypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SuggestionSubtype {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SuggestionSubtype {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuggestionSubtype {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPT_DELETED" => SuggestionSubtype::AcceptDeleted,
                "ACCEPTED" => SuggestionSubtype::Accepted,
                "ADDED" => SuggestionSubtype::Added,
                "DELETED" => SuggestionSubtype::Deleted,
                "REJECT_DELETED" => SuggestionSubtype::RejectDeleted,
                "REJECTED" => SuggestionSubtype::Rejected,
                "REPLY_ADDED" => SuggestionSubtype::ReplyAdded,
                "REPLY_DELETED" => SuggestionSubtype::ReplyDeleted,
                "SUBTYPE_UNSPECIFIED" => SuggestionSubtype::SubtypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SuggestionSubtype {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestionSubtype {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SystemEvent {
        #[doc = "The type of the system event that may triggered activity."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SystemEventType>,
    }
    impl ::google_field_selector::FieldSelector for SystemEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SystemEventType {
        #[doc = "The event is due to the system automatically purging trash."]
        TrashAutoPurge,
        #[doc = "The event type is unspecified."]
        TypeUnspecified,
        #[doc = "The event is a consequence of a user account being deleted."]
        UserDeletion,
    }
    impl SystemEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                SystemEventType::TrashAutoPurge => "TRASH_AUTO_PURGE",
                SystemEventType::TypeUnspecified => "TYPE_UNSPECIFIED",
                SystemEventType::UserDeletion => "USER_DELETION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SystemEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SystemEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SystemEventType, ()> {
            Ok(match s {
                "TRASH_AUTO_PURGE" => SystemEventType::TrashAutoPurge,
                "TYPE_UNSPECIFIED" => SystemEventType::TypeUnspecified,
                "USER_DELETION" => SystemEventType::UserDeletion,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SystemEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SystemEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SystemEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TRASH_AUTO_PURGE" => SystemEventType::TrashAutoPurge,
                "TYPE_UNSPECIFIED" => SystemEventType::TypeUnspecified,
                "USER_DELETION" => SystemEventType::UserDeletion,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SystemEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SystemEventType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The target is a shared drive."]
        #[serde(
            rename = "drive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive: ::std::option::Option<crate::schemas::Drive>,
        #[doc = "The target is a Drive item."]
        #[serde(
            rename = "driveItem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_item: ::std::option::Option<crate::schemas::DriveItem>,
        #[doc = "The target is a comment on a Drive file."]
        #[serde(
            rename = "fileComment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_comment: ::std::option::Option<crate::schemas::FileComment>,
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        #[serde(
            rename = "teamDrive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub team_drive: ::std::option::Option<crate::schemas::TeamDrive>,
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
    pub struct TargetReference {
        #[doc = "The target is a shared drive."]
        #[serde(
            rename = "drive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive: ::std::option::Option<crate::schemas::DriveReference>,
        #[doc = "The target is a Drive item."]
        #[serde(
            rename = "driveItem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_item: ::std::option::Option<crate::schemas::DriveItemReference>,
        #[doc = "This field is deprecated; please use the `drive` field instead."]
        #[serde(
            rename = "teamDrive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub team_drive: ::std::option::Option<crate::schemas::TeamDriveReference>,
    }
    impl ::google_field_selector::FieldSelector for TargetReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TeamDrive {
        #[doc = "This field is deprecated; please see `Drive.name` instead."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "This field is deprecated; please see `Drive.root` instead."]
        #[serde(
            rename = "root",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub root: ::std::option::Option<crate::schemas::DriveItem>,
        #[doc = "This field is deprecated; please see `Drive.title` instead."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TeamDrive {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TeamDrive {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TeamDriveReference {
        #[doc = "This field is deprecated; please see `DriveReference.name` instead."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "This field is deprecated; please see `DriveReference.title` instead."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TeamDriveReference {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TeamDriveReference {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeRange {
        #[doc = "The end of the time range."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "The start of the time range."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimeRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct UnknownUser;
    impl ::google_field_selector::FieldSelector for UnknownUser {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnknownUser {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct Upload;
    impl ::google_field_selector::FieldSelector for Upload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Upload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "A user whose account has since been deleted."]
        #[serde(
            rename = "deletedUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted_user: ::std::option::Option<crate::schemas::DeletedUser>,
        #[doc = "A known user."]
        #[serde(
            rename = "knownUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub known_user: ::std::option::Option<crate::schemas::KnownUser>,
        #[doc = "A user about whom nothing is currently known."]
        #[serde(
            rename = "unknownUser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unknown_user: ::std::option::Option<crate::schemas::UnknownUser>,
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
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the activity resource"]
    pub fn activity(&self) -> crate::resources::activity::ActivityActions {
        crate::resources::activity::ActivityActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod activity {
        pub mod params {}
        pub struct ActivityActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ActivityActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Query past activity in Google Drive."]
            pub fn query(
                &self,
                request: crate::schemas::QueryDriveActivityRequest,
            ) -> QueryRequestBuilder {
                QueryRequestBuilder {
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
        }
        #[doc = "Created via [ActivityActions::query()](struct.ActivityActions.html#method.query)"]
        #[derive(Debug, Clone)]
        pub struct QueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::QueryDriveActivityRequest,
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
        impl<'a> QueryRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::QueryDriveActivityResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::QueryDriveActivityResponse, crate::Error> {
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
                let req = req.json(&self.request);
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://driveactivity.googleapis.com/".to_owned();
                output.push_str("v2/activity:query");
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
            Error::Reqwest { reqwest_err, .. } => reqwest_err
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

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(mut response: ::reqwest::Response) -> Result<::reqwest::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
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
