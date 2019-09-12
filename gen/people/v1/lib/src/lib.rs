#![doc = "# Resources and Methods\n    * [contact_groups](resources/contact_groups/struct.ContactGroupsActions.html)\n      * [*batchGet*](resources/contact_groups/struct.BatchGetRequestBuilder.html), [*create*](resources/contact_groups/struct.CreateRequestBuilder.html), [*delete*](resources/contact_groups/struct.DeleteRequestBuilder.html), [*get*](resources/contact_groups/struct.GetRequestBuilder.html), [*list*](resources/contact_groups/struct.ListRequestBuilder.html), [*update*](resources/contact_groups/struct.UpdateRequestBuilder.html)\n      * [members](resources/contact_groups/members/struct.MembersActions.html)\n        * [*modify*](resources/contact_groups/members/struct.ModifyRequestBuilder.html)\n    * [people](resources/people/struct.PeopleActions.html)\n      * [*createContact*](resources/people/struct.CreateContactRequestBuilder.html), [*deleteContact*](resources/people/struct.DeleteContactRequestBuilder.html), [*deleteContactPhoto*](resources/people/struct.DeleteContactPhotoRequestBuilder.html), [*get*](resources/people/struct.GetRequestBuilder.html), [*getBatchGet*](resources/people/struct.GetBatchGetRequestBuilder.html), [*updateContact*](resources/people/struct.UpdateContactRequestBuilder.html), [*updateContactPhoto*](resources/people/struct.UpdateContactPhotoRequestBuilder.html)\n      * [connections](resources/people/connections/struct.ConnectionsActions.html)\n        * [*list*](resources/people/connections/struct.ListRequestBuilder.html)\n"]
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
    pub struct Address {
        #[doc = "The city of the address."]
        #[serde(
            rename = "city",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub city: ::std::option::Option<String>,
        #[doc = "The country of the address."]
        #[serde(
            rename = "country",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub country: ::std::option::Option<String>,
        #[doc = "The [ISO 3166-1 alpha-2](http://www.iso.org/iso/country_codes.htm) country\ncode of the address."]
        #[serde(
            rename = "countryCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub country_code: ::std::option::Option<String>,
        #[doc = "The extended address of the address; for example, the apartment number."]
        #[serde(
            rename = "extendedAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extended_address: ::std::option::Option<String>,
        #[doc = "The read-only type of the address translated and formatted in the viewer's\naccount locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "The unstructured value of the address. If this is not set by the user it\nwill be automatically constructed from structured values."]
        #[serde(
            rename = "formattedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_value: ::std::option::Option<String>,
        #[doc = "Metadata about the address."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The P.O. box of the address."]
        #[serde(
            rename = "poBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub po_box: ::std::option::Option<String>,
        #[doc = "The postal code of the address."]
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_code: ::std::option::Option<String>,
        #[doc = "The type of the address. The type can be custom or one of these predefined\nvalues:\n\n* `home`\n* `work`\n* `other`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The region of the address; for example, the state or province."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "The street address."]
        #[serde(
            rename = "streetAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub street_address: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Address {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Address {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AgeRangeType {
        #[doc = "The age range."]
        #[serde(
            rename = "ageRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub age_range: ::std::option::Option<crate::schemas::AgeRangeTypeAgeRange>,
        #[doc = "Metadata about the age range."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
    }
    impl ::google_field_selector::FieldSelector for AgeRangeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AgeRangeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AgeRangeTypeAgeRange {
        #[doc = "Unspecified."]
        AgeRangeUnspecified,
        #[doc = "Between eighteen and twenty."]
        EighteenToTwenty,
        #[doc = "Younger than eighteen."]
        LessThanEighteen,
        #[doc = "Twenty-one and older."]
        TwentyOneOrOlder,
    }
    impl AgeRangeTypeAgeRange {
        pub fn as_str(self) -> &'static str {
            match self {
                AgeRangeTypeAgeRange::AgeRangeUnspecified => "AGE_RANGE_UNSPECIFIED",
                AgeRangeTypeAgeRange::EighteenToTwenty => "EIGHTEEN_TO_TWENTY",
                AgeRangeTypeAgeRange::LessThanEighteen => "LESS_THAN_EIGHTEEN",
                AgeRangeTypeAgeRange::TwentyOneOrOlder => "TWENTY_ONE_OR_OLDER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AgeRangeTypeAgeRange {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AgeRangeTypeAgeRange {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AgeRangeTypeAgeRange, ()> {
            Ok(match s {
                "AGE_RANGE_UNSPECIFIED" => AgeRangeTypeAgeRange::AgeRangeUnspecified,
                "EIGHTEEN_TO_TWENTY" => AgeRangeTypeAgeRange::EighteenToTwenty,
                "LESS_THAN_EIGHTEEN" => AgeRangeTypeAgeRange::LessThanEighteen,
                "TWENTY_ONE_OR_OLDER" => AgeRangeTypeAgeRange::TwentyOneOrOlder,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AgeRangeTypeAgeRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AgeRangeTypeAgeRange {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AgeRangeTypeAgeRange {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AGE_RANGE_UNSPECIFIED" => AgeRangeTypeAgeRange::AgeRangeUnspecified,
                "EIGHTEEN_TO_TWENTY" => AgeRangeTypeAgeRange::EighteenToTwenty,
                "LESS_THAN_EIGHTEEN" => AgeRangeTypeAgeRange::LessThanEighteen,
                "TWENTY_ONE_OR_OLDER" => AgeRangeTypeAgeRange::TwentyOneOrOlder,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AgeRangeTypeAgeRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AgeRangeTypeAgeRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchGetContactGroupsResponse {
        #[doc = "The list of responses for each requested contact group resource."]
        #[serde(
            rename = "responses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responses: ::std::option::Option<Vec<crate::schemas::ContactGroupResponse>>,
    }
    impl ::google_field_selector::FieldSelector for BatchGetContactGroupsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchGetContactGroupsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Biography {
        #[doc = "The content type of the biography."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<crate::schemas::BiographyContentType>,
        #[doc = "Metadata about the biography."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The short biography."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Biography {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Biography {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BiographyContentType {
        #[doc = "Unspecified."]
        ContentTypeUnspecified,
        #[doc = "HTML text."]
        TextHtml,
        #[doc = "Plain text."]
        TextPlain,
    }
    impl BiographyContentType {
        pub fn as_str(self) -> &'static str {
            match self {
                BiographyContentType::ContentTypeUnspecified => "CONTENT_TYPE_UNSPECIFIED",
                BiographyContentType::TextHtml => "TEXT_HTML",
                BiographyContentType::TextPlain => "TEXT_PLAIN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BiographyContentType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BiographyContentType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BiographyContentType, ()> {
            Ok(match s {
                "CONTENT_TYPE_UNSPECIFIED" => BiographyContentType::ContentTypeUnspecified,
                "TEXT_HTML" => BiographyContentType::TextHtml,
                "TEXT_PLAIN" => BiographyContentType::TextPlain,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BiographyContentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BiographyContentType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BiographyContentType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_TYPE_UNSPECIFIED" => BiographyContentType::ContentTypeUnspecified,
                "TEXT_HTML" => BiographyContentType::TextHtml,
                "TEXT_PLAIN" => BiographyContentType::TextPlain,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BiographyContentType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BiographyContentType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Birthday {
        #[doc = "The date of the birthday."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Metadata about the birthday."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "A free-form string representing the user's birthday."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Birthday {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Birthday {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BraggingRights {
        #[doc = "Metadata about the bragging rights."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The bragging rights; for example, `climbed mount everest`."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BraggingRights {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BraggingRights {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContactGroup {
        #[doc = "The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the\nresource. Used for web cache validation."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The read-only name translated and formatted in the viewer's account locale\nor the `Accept-Language` HTTP header locale for system groups names.\nGroup names set by the owner are the same as name."]
        #[serde(
            rename = "formattedName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_name: ::std::option::Option<String>,
        #[doc = "The read-only contact group type."]
        #[serde(
            rename = "groupType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_type: ::std::option::Option<crate::schemas::ContactGroupGroupType>,
        #[doc = "The total number of contacts in the group irrespective of max members in\nspecified in the request."]
        #[serde(
            rename = "memberCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub member_count: ::std::option::Option<i32>,
        #[doc = "The list of contact person resource names that are members of the contact\ngroup. The field is not populated for LIST requests and can only be updated\nthrough the\n[ModifyContactGroupMembers](/people/api/rest/v1/contactgroups/members/modify)."]
        #[serde(
            rename = "memberResourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub member_resource_names: ::std::option::Option<Vec<String>>,
        #[doc = "Metadata about the contact group."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ContactGroupMetadata>,
        #[doc = "The contact group name set by the group owner or a system provided name\nfor system groups."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The resource name for the contact group, assigned by the server. An ASCII\nstring, in the form of `contactGroups/`<var>contact_group_id</var>."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContactGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContactGroup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContactGroupGroupType {
        #[doc = "Unspecified."]
        GroupTypeUnspecified,
        #[doc = "System defined contact group."]
        SystemContactGroup,
        #[doc = "User defined contact group."]
        UserContactGroup,
    }
    impl ContactGroupGroupType {
        pub fn as_str(self) -> &'static str {
            match self {
                ContactGroupGroupType::GroupTypeUnspecified => "GROUP_TYPE_UNSPECIFIED",
                ContactGroupGroupType::SystemContactGroup => "SYSTEM_CONTACT_GROUP",
                ContactGroupGroupType::UserContactGroup => "USER_CONTACT_GROUP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContactGroupGroupType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContactGroupGroupType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ContactGroupGroupType, ()> {
            Ok(match s {
                "GROUP_TYPE_UNSPECIFIED" => ContactGroupGroupType::GroupTypeUnspecified,
                "SYSTEM_CONTACT_GROUP" => ContactGroupGroupType::SystemContactGroup,
                "USER_CONTACT_GROUP" => ContactGroupGroupType::UserContactGroup,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContactGroupGroupType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContactGroupGroupType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContactGroupGroupType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GROUP_TYPE_UNSPECIFIED" => ContactGroupGroupType::GroupTypeUnspecified,
                "SYSTEM_CONTACT_GROUP" => ContactGroupGroupType::SystemContactGroup,
                "USER_CONTACT_GROUP" => ContactGroupGroupType::UserContactGroup,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ContactGroupGroupType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContactGroupGroupType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContactGroupMembership {
        #[doc = "The read-only contact group ID for the contact group membership."]
        #[serde(
            rename = "contactGroupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_group_id: ::std::option::Option<String>,
        #[doc = "The resource name for the contact group, assigned by the server. An ASCII\nstring, in the form of `contactGroups/`<var>contact_group_id</var>.\nOnly contact_group_resource_name can be used for modifying memberships.\nAny contact group membership can be removed, but only user group or\n\"myContacts\" or \"starred\" system groups memberships can be added. A\ncontact must always have at least one contact group membership."]
        #[serde(
            rename = "contactGroupResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_group_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContactGroupMembership {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContactGroupMembership {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContactGroupMetadata {
        #[doc = "True if the contact group resource has been deleted. Populated only for\n[`ListContactGroups`](/people/api/rest/v1/contactgroups/list) requests\nthat include a sync token."]
        #[serde(
            rename = "deleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted: ::std::option::Option<bool>,
        #[doc = "The time the group was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContactGroupMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContactGroupMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ContactGroupResponse {
        #[doc = "The contact group."]
        #[serde(
            rename = "contactGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_group: ::std::option::Option<crate::schemas::ContactGroup>,
        #[doc = "The original requested resource name."]
        #[serde(
            rename = "requestedResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_resource_name: ::std::option::Option<String>,
        #[doc = "The status of the response."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::google_field_selector::FieldSelector for ContactGroupResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContactGroupResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CoverPhoto {
        #[doc = "True if the cover photo is the default cover photo;\nfalse if the cover photo is a user-provided cover photo."]
        #[serde(
            rename = "default",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default: ::std::option::Option<bool>,
        #[doc = "Metadata about the cover photo."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The URL of the cover photo."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CoverPhoto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CoverPhoto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateContactGroupRequest {
        #[doc = "The contact group to create."]
        #[serde(
            rename = "contactGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_group: ::std::option::Option<crate::schemas::ContactGroup>,
    }
    impl ::google_field_selector::FieldSelector for CreateContactGroupRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateContactGroupRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Date {
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month, or 0\nif specifying a year by itself or a year and month where the day is not\nsignificant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of year. Must be from 1 to 12, or 0 if specifying a year without a\nmonth and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of date. Must be from 1 to 9999, or 0 if specifying a date without\na year."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Date {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Date {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteContactPhotoResponse {
        #[doc = "The updated person, if person_fields is set in the\nDeleteContactPhotoRequest; otherwise this will be unset."]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<crate::schemas::Person>,
    }
    impl ::google_field_selector::FieldSelector for DeleteContactPhotoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteContactPhotoResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DomainMembership {
        #[doc = "True if the person is in the viewer's G Suite domain."]
        #[serde(
            rename = "inViewerDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub in_viewer_domain: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DomainMembership {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DomainMembership {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EmailAddress {
        #[doc = "The display name of the email."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The read-only type of the email address translated and formatted in the\nviewer's account locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "Metadata about the email address."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The type of the email address. The type can be custom or one of these\npredefined values:\n\n* `home`\n* `work`\n* `other`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The email address."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EmailAddress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmailAddress {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct Empty;
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
    pub struct Event {
        #[doc = "The date of the event."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The read-only type of the event translated and formatted in the\nviewer's account locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "Metadata about the event."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The type of the event. The type can be custom or one of these predefined\nvalues:\n\n* `anniversary`\n* `other`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FieldMetadata {
        #[doc = "True if the field is the primary field; false if the field is a secondary\nfield."]
        #[serde(
            rename = "primary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary: ::std::option::Option<bool>,
        #[doc = "The source of the field."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
        #[doc = "True if the field is verified; false if the field is unverified. A\nverified field is typically a name, email address, phone number, or\nwebsite that has been confirmed to be owned by the person."]
        #[serde(
            rename = "verified",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verified: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for FieldMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Gender {
        #[doc = "The read-only value of the gender translated and formatted in the viewer's\naccount locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_value: ::std::option::Option<String>,
        #[doc = "Metadata about the gender."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The gender for the person. The gender can be custom or one of these\npredefined values:\n\n* `male`\n* `female`\n* `other`\n* `unknown`"]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Gender {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Gender {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GetPeopleResponse {
        #[doc = "The response for each requested resource name."]
        #[serde(
            rename = "responses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub responses: ::std::option::Option<Vec<crate::schemas::PersonResponse>>,
    }
    impl ::google_field_selector::FieldSelector for GetPeopleResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetPeopleResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImClient {
        #[doc = "The read-only protocol of the IM client formatted in the viewer's account\nlocale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedProtocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_protocol: ::std::option::Option<String>,
        #[doc = "The read-only type of the IM client translated and formatted in the\nviewer's account locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "Metadata about the IM client."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The protocol of the IM client. The protocol can be custom or one of these\npredefined values:\n\n* `aim`\n* `msn`\n* `yahoo`\n* `skype`\n* `qq`\n* `googleTalk`\n* `icq`\n* `jabber`\n* `netMeeting`"]
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The type of the IM client. The type can be custom or one of these\npredefined values:\n\n* `home`\n* `work`\n* `other`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The user name used in the IM client."]
        #[serde(
            rename = "username",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub username: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ImClient {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImClient {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Interest {
        #[doc = "Metadata about the interest."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The interest; for example, `stargazing`."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Interest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Interest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListConnectionsResponse {
        #[doc = "The list of people that the requestor is connected to."]
        #[serde(
            rename = "connections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connections: ::std::option::Option<Vec<crate::schemas::Person>>,
        #[doc = "The token that can be used to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The token that can be used to retrieve changes since the last request."]
        #[serde(
            rename = "nextSyncToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_sync_token: ::std::option::Option<String>,
        #[doc = "The total number of items in the list without pagination."]
        #[serde(
            rename = "totalItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_items: ::std::option::Option<i32>,
        #[doc = "**DEPRECATED** (Please use totalItems)\nThe total number of people in the list without pagination."]
        #[serde(
            rename = "totalPeople",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_people: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListConnectionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListConnectionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListContactGroupsResponse {
        #[doc = "The list of contact groups. Members of the contact groups are not\npopulated."]
        #[serde(
            rename = "contactGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_groups: ::std::option::Option<Vec<crate::schemas::ContactGroup>>,
        #[doc = "The token that can be used to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The token that can be used to retrieve changes since the last request."]
        #[serde(
            rename = "nextSyncToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_sync_token: ::std::option::Option<String>,
        #[doc = "The total number of items in the list without pagination."]
        #[serde(
            rename = "totalItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ListContactGroupsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListContactGroupsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Locale {
        #[doc = "Metadata about the locale."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The well-formed [IETF BCP 47](https://tools.ietf.org/html/bcp47)\nlanguage tag representing the locale."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Locale {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Locale {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The contact group membership."]
        #[serde(
            rename = "contactGroupMembership",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_group_membership: ::std::option::Option<crate::schemas::ContactGroupMembership>,
        #[doc = "The read-only domain membership."]
        #[serde(
            rename = "domainMembership",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_membership: ::std::option::Option<crate::schemas::DomainMembership>,
        #[doc = "Metadata about the membership."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ModifyContactGroupMembersRequest {
        #[doc = "The resource names of the contact people to add in the form of in the form\n`people/`<var>person_id</var>."]
        #[serde(
            rename = "resourceNamesToAdd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_names_to_add: ::std::option::Option<Vec<String>>,
        #[doc = "The resource names of the contact people to remove in the form of in the\nform of `people/`<var>person_id</var>."]
        #[serde(
            rename = "resourceNamesToRemove",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_names_to_remove: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ModifyContactGroupMembersRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ModifyContactGroupMembersRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ModifyContactGroupMembersResponse {
        #[doc = "The contact people resource names that cannot be removed from their\nlast contact group."]
        #[serde(
            rename = "canNotRemoveLastContactGroupResourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_not_remove_last_contact_group_resource_names: ::std::option::Option<Vec<String>>,
        #[doc = "The contact people resource names that were not found."]
        #[serde(
            rename = "notFoundResourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub not_found_resource_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ModifyContactGroupMembersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ModifyContactGroupMembersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Name {
        #[doc = "The read-only display name formatted according to the locale specified by\nthe viewer's account or the `Accept-Language` HTTP header."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The read-only display name with the last name first formatted according to\nthe locale specified by the viewer's account or the\n`Accept-Language` HTTP header."]
        #[serde(
            rename = "displayNameLastFirst",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name_last_first: ::std::option::Option<String>,
        #[doc = "The family name."]
        #[serde(
            rename = "familyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub family_name: ::std::option::Option<String>,
        #[doc = "The given name."]
        #[serde(
            rename = "givenName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub given_name: ::std::option::Option<String>,
        #[doc = "The honorific prefixes, such as `Mrs.` or `Dr.`"]
        #[serde(
            rename = "honorificPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub honorific_prefix: ::std::option::Option<String>,
        #[doc = "The honorific suffixes, such as `Jr.`"]
        #[serde(
            rename = "honorificSuffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub honorific_suffix: ::std::option::Option<String>,
        #[doc = "Metadata about the name."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The middle name(s)."]
        #[serde(
            rename = "middleName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub middle_name: ::std::option::Option<String>,
        #[doc = "The family name spelled as it sounds."]
        #[serde(
            rename = "phoneticFamilyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phonetic_family_name: ::std::option::Option<String>,
        #[doc = "The full name spelled as it sounds."]
        #[serde(
            rename = "phoneticFullName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phonetic_full_name: ::std::option::Option<String>,
        #[doc = "The given name spelled as it sounds."]
        #[serde(
            rename = "phoneticGivenName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phonetic_given_name: ::std::option::Option<String>,
        #[doc = "The honorific prefixes spelled as they sound."]
        #[serde(
            rename = "phoneticHonorificPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phonetic_honorific_prefix: ::std::option::Option<String>,
        #[doc = "The honorific suffixes spelled as they sound."]
        #[serde(
            rename = "phoneticHonorificSuffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phonetic_honorific_suffix: ::std::option::Option<String>,
        #[doc = "The middle name(s) spelled as they sound."]
        #[serde(
            rename = "phoneticMiddleName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phonetic_middle_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Name {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Name {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Nickname {
        #[doc = "Metadata about the nickname."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The type of the nickname."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::NicknameType>,
        #[doc = "The nickname."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Nickname {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Nickname {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NicknameType {
        #[doc = "Generic nickname."]
        Default,
        #[doc = "Google+ profile nickname."]
        Gplus,
        #[doc = "Initials."]
        Initials,
        #[doc = "Maiden name or birth family name. Used when the person's family name has\nchanged as a result of marriage."]
        MaidenName,
        #[doc = "A professional affiliation or other name; for example, `Dr. Smith.`"]
        OtherName,
    }
    impl NicknameType {
        pub fn as_str(self) -> &'static str {
            match self {
                NicknameType::Default => "DEFAULT",
                NicknameType::Gplus => "GPLUS",
                NicknameType::Initials => "INITIALS",
                NicknameType::MaidenName => "MAIDEN_NAME",
                NicknameType::OtherName => "OTHER_NAME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NicknameType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NicknameType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NicknameType, ()> {
            Ok(match s {
                "DEFAULT" => NicknameType::Default,
                "GPLUS" => NicknameType::Gplus,
                "INITIALS" => NicknameType::Initials,
                "MAIDEN_NAME" => NicknameType::MaidenName,
                "OTHER_NAME" => NicknameType::OtherName,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NicknameType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NicknameType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NicknameType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => NicknameType::Default,
                "GPLUS" => NicknameType::Gplus,
                "INITIALS" => NicknameType::Initials,
                "MAIDEN_NAME" => NicknameType::MaidenName,
                "OTHER_NAME" => NicknameType::OtherName,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NicknameType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NicknameType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Occupation {
        #[doc = "Metadata about the occupation."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The occupation; for example, `carpenter`."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Occupation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Occupation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Organization {
        #[doc = "True if the organization is the person's current organization;\nfalse if the organization is a past organization."]
        #[serde(
            rename = "current",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current: ::std::option::Option<bool>,
        #[doc = "The person's department at the organization."]
        #[serde(
            rename = "department",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub department: ::std::option::Option<String>,
        #[doc = "The domain name associated with the organization; for example,\n`google.com`."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "The end date when the person left the organization."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The read-only type of the organization translated and formatted in the\nviewer's account locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "The person's job description at the organization."]
        #[serde(
            rename = "jobDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_description: ::std::option::Option<String>,
        #[doc = "The location of the organization office the person works at."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Metadata about the organization."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The name of the organization."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The phonetic name of the organization."]
        #[serde(
            rename = "phoneticName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phonetic_name: ::std::option::Option<String>,
        #[doc = "The type of the organization. The type can be custom or  one of these\npredefined values:\n\n* `work`\n* `school`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The start date when the person joined the organization."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The symbol associated with the organization; for example, a stock ticker\nsymbol, abbreviation, or acronym."]
        #[serde(
            rename = "symbol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub symbol: ::std::option::Option<String>,
        #[doc = "The person's job title at the organization."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Organization {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Organization {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Person {
        #[doc = "The person's street addresses."]
        #[serde(
            rename = "addresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub addresses: ::std::option::Option<Vec<crate::schemas::Address>>,
        #[doc = "**DEPRECATED** (Please use `person.ageRanges` instead)**\n\nThe person's read-only age range."]
        #[serde(
            rename = "ageRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub age_range: ::std::option::Option<crate::schemas::PersonAgeRange>,
        #[doc = "The person's read-only age ranges."]
        #[serde(
            rename = "ageRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub age_ranges: ::std::option::Option<Vec<crate::schemas::AgeRangeType>>,
        #[doc = "The person's biographies."]
        #[serde(
            rename = "biographies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub biographies: ::std::option::Option<Vec<crate::schemas::Biography>>,
        #[doc = "The person's birthdays."]
        #[serde(
            rename = "birthdays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub birthdays: ::std::option::Option<Vec<crate::schemas::Birthday>>,
        #[doc = "The person's bragging rights."]
        #[serde(
            rename = "braggingRights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bragging_rights: ::std::option::Option<Vec<crate::schemas::BraggingRights>>,
        #[doc = "The person's read-only cover photos."]
        #[serde(
            rename = "coverPhotos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cover_photos: ::std::option::Option<Vec<crate::schemas::CoverPhoto>>,
        #[doc = "The person's email addresses."]
        #[serde(
            rename = "emailAddresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_addresses: ::std::option::Option<Vec<crate::schemas::EmailAddress>>,
        #[doc = "The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the\nresource. Used for web cache validation."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The person's events."]
        #[serde(
            rename = "events",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub events: ::std::option::Option<Vec<crate::schemas::Event>>,
        #[doc = "The person's genders."]
        #[serde(
            rename = "genders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub genders: ::std::option::Option<Vec<crate::schemas::Gender>>,
        #[doc = "The person's instant messaging clients."]
        #[serde(
            rename = "imClients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub im_clients: ::std::option::Option<Vec<crate::schemas::ImClient>>,
        #[doc = "The person's interests."]
        #[serde(
            rename = "interests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interests: ::std::option::Option<Vec<crate::schemas::Interest>>,
        #[doc = "The person's locale preferences."]
        #[serde(
            rename = "locales",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locales: ::std::option::Option<Vec<crate::schemas::Locale>>,
        #[doc = "The person's group memberships."]
        #[serde(
            rename = "memberships",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memberships: ::std::option::Option<Vec<crate::schemas::Membership>>,
        #[doc = "Read-only metadata about the person."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::PersonMetadata>,
        #[doc = "The person's names."]
        #[serde(
            rename = "names",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub names: ::std::option::Option<Vec<crate::schemas::Name>>,
        #[doc = "The person's nicknames."]
        #[serde(
            rename = "nicknames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nicknames: ::std::option::Option<Vec<crate::schemas::Nickname>>,
        #[doc = "The person's occupations."]
        #[serde(
            rename = "occupations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occupations: ::std::option::Option<Vec<crate::schemas::Occupation>>,
        #[doc = "The person's past or current organizations."]
        #[serde(
            rename = "organizations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub organizations: ::std::option::Option<Vec<crate::schemas::Organization>>,
        #[doc = "The person's phone numbers."]
        #[serde(
            rename = "phoneNumbers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_numbers: ::std::option::Option<Vec<crate::schemas::PhoneNumber>>,
        #[doc = "The person's read-only photos."]
        #[serde(
            rename = "photos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photos: ::std::option::Option<Vec<crate::schemas::Photo>>,
        #[doc = "The person's relations."]
        #[serde(
            rename = "relations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relations: ::std::option::Option<Vec<crate::schemas::Relation>>,
        #[doc = "The person's read-only relationship interests."]
        #[serde(
            rename = "relationshipInterests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_interests:
            ::std::option::Option<Vec<crate::schemas::RelationshipInterest>>,
        #[doc = "The person's read-only relationship statuses."]
        #[serde(
            rename = "relationshipStatuses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub relationship_statuses: ::std::option::Option<Vec<crate::schemas::RelationshipStatus>>,
        #[doc = "The person's residences."]
        #[serde(
            rename = "residences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub residences: ::std::option::Option<Vec<crate::schemas::Residence>>,
        #[doc = "The resource name for the person, assigned by the server. An ASCII string\nwith a max length of 27 characters, in the form of\n`people/`<var>person_id</var>."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "The person's SIP addresses."]
        #[serde(
            rename = "sipAddresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sip_addresses: ::std::option::Option<Vec<crate::schemas::SipAddress>>,
        #[doc = "The person's skills."]
        #[serde(
            rename = "skills",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skills: ::std::option::Option<Vec<crate::schemas::Skill>>,
        #[doc = "The person's read-only taglines."]
        #[serde(
            rename = "taglines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub taglines: ::std::option::Option<Vec<crate::schemas::Tagline>>,
        #[doc = "The person's associated URLs."]
        #[serde(
            rename = "urls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub urls: ::std::option::Option<Vec<crate::schemas::Url>>,
        #[doc = "The person's user defined data."]
        #[serde(
            rename = "userDefined",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_defined: ::std::option::Option<Vec<crate::schemas::UserDefined>>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PersonAgeRange {
        #[doc = "Unspecified."]
        AgeRangeUnspecified,
        #[doc = "Between eighteen and twenty."]
        EighteenToTwenty,
        #[doc = "Younger than eighteen."]
        LessThanEighteen,
        #[doc = "Twenty-one and older."]
        TwentyOneOrOlder,
    }
    impl PersonAgeRange {
        pub fn as_str(self) -> &'static str {
            match self {
                PersonAgeRange::AgeRangeUnspecified => "AGE_RANGE_UNSPECIFIED",
                PersonAgeRange::EighteenToTwenty => "EIGHTEEN_TO_TWENTY",
                PersonAgeRange::LessThanEighteen => "LESS_THAN_EIGHTEEN",
                PersonAgeRange::TwentyOneOrOlder => "TWENTY_ONE_OR_OLDER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PersonAgeRange {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PersonAgeRange {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PersonAgeRange, ()> {
            Ok(match s {
                "AGE_RANGE_UNSPECIFIED" => PersonAgeRange::AgeRangeUnspecified,
                "EIGHTEEN_TO_TWENTY" => PersonAgeRange::EighteenToTwenty,
                "LESS_THAN_EIGHTEEN" => PersonAgeRange::LessThanEighteen,
                "TWENTY_ONE_OR_OLDER" => PersonAgeRange::TwentyOneOrOlder,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PersonAgeRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PersonAgeRange {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PersonAgeRange {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AGE_RANGE_UNSPECIFIED" => PersonAgeRange::AgeRangeUnspecified,
                "EIGHTEEN_TO_TWENTY" => PersonAgeRange::EighteenToTwenty,
                "LESS_THAN_EIGHTEEN" => PersonAgeRange::LessThanEighteen,
                "TWENTY_ONE_OR_OLDER" => PersonAgeRange::TwentyOneOrOlder,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PersonAgeRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonAgeRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PersonMetadata {
        #[doc = "True if the person resource has been deleted. Populated only for\n[`connections.list`](/people/api/rest/v1/people.connections/list) requests\nthat include a sync token."]
        #[serde(
            rename = "deleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted: ::std::option::Option<bool>,
        #[doc = "Resource names of people linked to this resource."]
        #[serde(
            rename = "linkedPeopleResourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linked_people_resource_names: ::std::option::Option<Vec<String>>,
        #[doc = "**DEPRECATED** (Please use\n`person.metadata.sources.profileMetadata.objectType` instead)\n\nThe type of the person object."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<crate::schemas::PersonMetadataObjectType>,
        #[doc = "Any former resource names this person has had. Populated only for\n[`connections.list`](/people/api/rest/v1/people.connections/list) requests\nthat include a sync token.\n\nThe resource name may change when adding or removing fields that link a\ncontact and profile such as a verified email, verified phone number, or\nprofile URL."]
        #[serde(
            rename = "previousResourceNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub previous_resource_names: ::std::option::Option<Vec<String>>,
        #[doc = "The sources of data for the person."]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
    }
    impl ::google_field_selector::FieldSelector for PersonMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PersonMetadataObjectType {
        #[doc = "Unspecified."]
        ObjectTypeUnspecified,
        #[doc = "[Google+ Page.](http://www.google.com/+/brands/)"]
        Page,
        #[doc = "Person."]
        Person,
    }
    impl PersonMetadataObjectType {
        pub fn as_str(self) -> &'static str {
            match self {
                PersonMetadataObjectType::ObjectTypeUnspecified => "OBJECT_TYPE_UNSPECIFIED",
                PersonMetadataObjectType::Page => "PAGE",
                PersonMetadataObjectType::Person => "PERSON",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PersonMetadataObjectType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PersonMetadataObjectType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PersonMetadataObjectType, ()> {
            Ok(match s {
                "OBJECT_TYPE_UNSPECIFIED" => PersonMetadataObjectType::ObjectTypeUnspecified,
                "PAGE" => PersonMetadataObjectType::Page,
                "PERSON" => PersonMetadataObjectType::Person,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PersonMetadataObjectType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PersonMetadataObjectType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PersonMetadataObjectType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OBJECT_TYPE_UNSPECIFIED" => PersonMetadataObjectType::ObjectTypeUnspecified,
                "PAGE" => PersonMetadataObjectType::Page,
                "PERSON" => PersonMetadataObjectType::Person,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PersonMetadataObjectType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonMetadataObjectType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct PersonResponse {
        #[doc = "**DEPRECATED** (Please use status instead)\n\n[HTTP 1.1 status code]\n(http://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html)."]
        #[serde(
            rename = "httpStatusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_status_code: ::std::option::Option<i32>,
        #[doc = "The person."]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<crate::schemas::Person>,
        #[doc = "The original requested resource name. May be different than the resource\nname on the returned person.\n\nThe resource name can change when adding or removing fields that link a\ncontact and profile such as a verified email, verified phone number, or a\nprofile URL."]
        #[serde(
            rename = "requestedResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_resource_name: ::std::option::Option<String>,
        #[doc = "The status of the response."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::google_field_selector::FieldSelector for PersonResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersonResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PhoneNumber {
        #[doc = "The read-only canonicalized [ITU-T\nE.164](https://law.resource.org/pub/us/cfr/ibr/004/itu-t.E.164.1.2008.pdf)\nform of the phone number."]
        #[serde(
            rename = "canonicalForm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub canonical_form: ::std::option::Option<String>,
        #[doc = "The read-only type of the phone number translated and formatted in the\nviewer's account locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "Metadata about the phone number."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The type of the phone number. The type can be custom or one of these\npredefined values:\n\n* `home`\n* `work`\n* `mobile`\n* `homeFax`\n* `workFax`\n* `otherFax`\n* `pager`\n* `workMobile`\n* `workPager`\n* `main`\n* `googleVoice`\n* `other`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The phone number."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PhoneNumber {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PhoneNumber {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "True if the photo is a default photo;\nfalse if the photo is a user-provided photo."]
        #[serde(
            rename = "default",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default: ::std::option::Option<bool>,
        #[doc = "Metadata about the photo."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The URL of the photo. You can change the desired size by appending a query\nparameter `sz=`<var>size</var> at the end of the url. Example:\n`https://lh3.googleusercontent.com/-T_wVWLlmg7w/AAAAAAAAAAI/AAAAAAAABa8/00gzXvDBYqw/s100/photo.jpg?sz=50`"]
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
    pub struct ProfileMetadata {
        #[doc = "The profile object type."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<crate::schemas::ProfileMetadataObjectType>,
        #[doc = "The user types."]
        #[serde(
            rename = "userTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_types: ::std::option::Option<Vec<crate::schemas::ProfileMetadataUserTypesItems>>,
    }
    impl ::google_field_selector::FieldSelector for ProfileMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProfileMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileMetadataObjectType {
        #[doc = "Unspecified."]
        ObjectTypeUnspecified,
        #[doc = "[Google+ Page.](http://www.google.com/+/brands/)"]
        Page,
        #[doc = "Person."]
        Person,
    }
    impl ProfileMetadataObjectType {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileMetadataObjectType::ObjectTypeUnspecified => "OBJECT_TYPE_UNSPECIFIED",
                ProfileMetadataObjectType::Page => "PAGE",
                ProfileMetadataObjectType::Person => "PERSON",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ProfileMetadataObjectType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ProfileMetadataObjectType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ProfileMetadataObjectType, ()> {
            Ok(match s {
                "OBJECT_TYPE_UNSPECIFIED" => ProfileMetadataObjectType::ObjectTypeUnspecified,
                "PAGE" => ProfileMetadataObjectType::Page,
                "PERSON" => ProfileMetadataObjectType::Person,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ProfileMetadataObjectType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileMetadataObjectType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileMetadataObjectType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OBJECT_TYPE_UNSPECIFIED" => ProfileMetadataObjectType::ObjectTypeUnspecified,
                "PAGE" => ProfileMetadataObjectType::Page,
                "PERSON" => ProfileMetadataObjectType::Person,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ProfileMetadataObjectType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProfileMetadataObjectType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileMetadataUserTypesItems {
        GoogleAppsUser,
        GoogleUser,
        GplusUser,
        UserTypeUnknown,
    }
    impl ProfileMetadataUserTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileMetadataUserTypesItems::GoogleAppsUser => "GOOGLE_APPS_USER",
                ProfileMetadataUserTypesItems::GoogleUser => "GOOGLE_USER",
                ProfileMetadataUserTypesItems::GplusUser => "GPLUS_USER",
                ProfileMetadataUserTypesItems::UserTypeUnknown => "USER_TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ProfileMetadataUserTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ProfileMetadataUserTypesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ProfileMetadataUserTypesItems, ()> {
            Ok(match s {
                "GOOGLE_APPS_USER" => ProfileMetadataUserTypesItems::GoogleAppsUser,
                "GOOGLE_USER" => ProfileMetadataUserTypesItems::GoogleUser,
                "GPLUS_USER" => ProfileMetadataUserTypesItems::GplusUser,
                "USER_TYPE_UNKNOWN" => ProfileMetadataUserTypesItems::UserTypeUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ProfileMetadataUserTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileMetadataUserTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileMetadataUserTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GOOGLE_APPS_USER" => ProfileMetadataUserTypesItems::GoogleAppsUser,
                "GOOGLE_USER" => ProfileMetadataUserTypesItems::GoogleUser,
                "GPLUS_USER" => ProfileMetadataUserTypesItems::GplusUser,
                "USER_TYPE_UNKNOWN" => ProfileMetadataUserTypesItems::UserTypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ProfileMetadataUserTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProfileMetadataUserTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Relation {
        #[doc = "The type of the relation translated and formatted in the viewer's account\nlocale or the locale specified in the Accept-Language HTTP header."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "Metadata about the relation."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The name of the other person this relation refers to."]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<String>,
        #[doc = "The person's relation to the other person. The type can be custom or one of\nthese predefined values:\n\n* `spouse`\n* `child`\n* `mother`\n* `father`\n* `parent`\n* `brother`\n* `sister`\n* `friend`\n* `relative`\n* `domesticPartner`\n* `manager`\n* `assistant`\n* `referredBy`\n* `partner`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Relation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Relation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RelationshipInterest {
        #[doc = "The value of the relationship interest translated and formatted in the\nviewer's account locale or the locale specified in the Accept-Language\nHTTP header."]
        #[serde(
            rename = "formattedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_value: ::std::option::Option<String>,
        #[doc = "Metadata about the relationship interest."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The kind of relationship the person is looking for. The value can be custom\nor one of these predefined values:\n\n* `friend`\n* `date`\n* `relationship`\n* `networking`"]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelationshipInterest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationshipInterest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RelationshipStatus {
        #[doc = "The read-only value of the relationship status translated and formatted in\nthe viewer's account locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_value: ::std::option::Option<String>,
        #[doc = "Metadata about the relationship status."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The relationship status. The value can be custom or one of these\npredefined values:\n\n* `single`\n* `inARelationship`\n* `engaged`\n* `married`\n* `itsComplicated`\n* `openRelationship`\n* `widowed`\n* `inDomesticPartnership`\n* `inCivilUnion`"]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelationshipStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationshipStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Residence {
        #[doc = "True if the residence is the person's current residence;\nfalse if the residence is a past residence."]
        #[serde(
            rename = "current",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current: ::std::option::Option<bool>,
        #[doc = "Metadata about the residence."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The address of the residence."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Residence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Residence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SipAddress {
        #[doc = "The read-only type of the SIP address translated and formatted in the\nviewer's account locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "Metadata about the SIP address."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The type of the SIP address. The type can be custom or or one of these\npredefined values:\n\n* `home`\n* `work`\n* `mobile`\n* `other`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The SIP address in the\n[RFC 3261 19.1](https://tools.ietf.org/html/rfc3261#section-19.1) SIP URI\nformat."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SipAddress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SipAddress {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Skill {
        #[doc = "Metadata about the skill."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The skill; for example, `underwater basket weaving`."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Skill {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Skill {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Source {
        #[doc = "**Only populated in `person.metadata.sources`.**\n\nThe [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the\nsource. Used for web cache validation."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The unique identifier within the source type generated by the server."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "**Only populated in `person.metadata.sources`.**\n\nMetadata about a source of type PROFILE."]
        #[serde(
            rename = "profileMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_metadata: ::std::option::Option<crate::schemas::ProfileMetadata>,
        #[doc = "The source type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SourceType>,
        #[doc = "**Only populated in `person.metadata.sources`.**\n\nLast update timestamp of this source."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Source {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Source {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourceType {
        #[doc = "[Google Account](https://accounts.google.com)."]
        Account,
        #[doc = "[Google contact](https://contacts.google.com). You can view the\ncontact at https://contact.google.com/<var>id</var> where <var>id</var>\nis the source id."]
        Contact,
        #[doc = "[G Suite domain profile](https://support.google.com/a/answer/1628008)."]
        DomainProfile,
        #[doc = "[Google profile](https://profiles.google.com). You can view the\nprofile at https://profiles.google.com/<var>id</var> where\n<var>id</var> is the source id."]
        Profile,
        #[doc = "Unspecified."]
        SourceTypeUnspecified,
    }
    impl SourceType {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceType::Account => "ACCOUNT",
                SourceType::Contact => "CONTACT",
                SourceType::DomainProfile => "DOMAIN_PROFILE",
                SourceType::Profile => "PROFILE",
                SourceType::SourceTypeUnspecified => "SOURCE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SourceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SourceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SourceType, ()> {
            Ok(match s {
                "ACCOUNT" => SourceType::Account,
                "CONTACT" => SourceType::Contact,
                "DOMAIN_PROFILE" => SourceType::DomainProfile,
                "PROFILE" => SourceType::Profile,
                "SOURCE_TYPE_UNSPECIFIED" => SourceType::SourceTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNT" => SourceType::Account,
                "CONTACT" => SourceType::Contact,
                "DOMAIN_PROFILE" => SourceType::DomainProfile,
                "PROFILE" => SourceType::Profile,
                "SOURCE_TYPE_UNSPECIFIED" => SourceType::SourceTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SourceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Tagline {
        #[doc = "Metadata about the tagline."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The tagline."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Tagline {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Tagline {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateContactGroupRequest {
        #[doc = "The contact group to update."]
        #[serde(
            rename = "contactGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contact_group: ::std::option::Option<crate::schemas::ContactGroup>,
    }
    impl ::google_field_selector::FieldSelector for UpdateContactGroupRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateContactGroupRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateContactPhotoRequest {
        #[doc = "**Optional.** Not specifying any fields will skip the post mutate read.\nA field mask to restrict which fields on the person are\nreturned. Multiple fields can be specified by separating them with commas.\nValid values are:\n\n* addresses\n* ageRanges\n* biographies\n* birthdays\n* braggingRights\n* coverPhotos\n* emailAddresses\n* events\n* genders\n* imClients\n* interests\n* locales\n* memberships\n* metadata\n* names\n* nicknames\n* occupations\n* organizations\n* phoneNumbers\n* photos\n* relations\n* relationshipInterests\n* relationshipStatuses\n* residences\n* sipAddresses\n* skills\n* taglines\n* urls\n* userDefined"]
        #[serde(
            rename = "personFields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_fields: ::std::option::Option<String>,
        #[doc = "Raw photo bytes"]
        #[serde(
            rename = "photoBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_bytes: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for UpdateContactPhotoRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateContactPhotoRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateContactPhotoResponse {
        #[doc = "The updated person, if person_fields is set in the\nUpdateContactPhotoRequest; otherwise this will be unset."]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<crate::schemas::Person>,
    }
    impl ::google_field_selector::FieldSelector for UpdateContactPhotoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateContactPhotoResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Url {
        #[doc = "The read-only type of the URL translated and formatted in the viewer's\naccount locale or the `Accept-Language` HTTP header locale."]
        #[serde(
            rename = "formattedType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_type: ::std::option::Option<String>,
        #[doc = "Metadata about the URL."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The type of the URL. The type can be custom or one of these predefined\nvalues:\n\n* `home`\n* `work`\n* `blog`\n* `profile`\n* `homePage`\n* `ftp`\n* `reservations`\n* `appInstallPage`: website for a Google+ application.\n* `other`"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The URL."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Url {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Url {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserDefined {
        #[doc = "The end user specified key of the user defined data."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Metadata about the user defined data."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadata>,
        #[doc = "The end user specified value of the user defined data."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UserDefined {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserDefined {
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
    #[doc = "Actions that can be performed on the contact_groups resource"]
    pub fn contact_groups(&self) -> crate::resources::contact_groups::ContactGroupsActions {
        crate::resources::contact_groups::ContactGroupsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the people resource"]
    pub fn people(&self) -> crate::resources::people::PeopleActions {
        crate::resources::people::PeopleActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod contact_groups {
        pub mod params {}
        pub struct ContactGroupsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ContactGroupsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Get a list of contact groups owned by the authenticated user by specifying\na list of contact group resource names."]
            pub fn batch_get(&self) -> BatchGetRequestBuilder {
                BatchGetRequestBuilder {
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
                    max_members: None,
                    resource_names: None,
                }
            }
            #[doc = "Create a new contact group owned by the authenticated user."]
            pub fn create(
                &self,
                request: crate::schemas::CreateContactGroupRequest,
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
                }
            }
            #[doc = "Delete an existing contact group owned by the authenticated user by\nspecifying a contact group resource name."]
            pub fn delete(&self, resource_name: impl Into<String>) -> DeleteRequestBuilder {
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
                    resource_name: resource_name.into(),
                    delete_contacts: None,
                }
            }
            #[doc = "Get a specific contact group owned by the authenticated user by specifying\na contact group resource name."]
            pub fn get(&self, resource_name: impl Into<String>) -> GetRequestBuilder {
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
                    resource_name: resource_name.into(),
                    max_members: None,
                }
            }
            #[doc = "List all contact groups owned by the authenticated user. Members of the\ncontact groups are not populated."]
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
                    sync_token: None,
                }
            }
            #[doc = "Update the name of an existing contact group owned by the authenticated\nuser."]
            pub fn update(
                &self,
                request: crate::schemas::UpdateContactGroupRequest,
                resource_name: impl Into<String>,
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
                    resource_name: resource_name.into(),
                }
            }
            #[doc = "Actions that can be performed on the members resource"]
            pub fn members(&self) -> crate::resources::contact_groups::members::MembersActions {
                crate::resources::contact_groups::members::MembersActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [ContactGroupsActions::batch_get()](struct.ContactGroupsActions.html#method.batch_get)"]
        #[derive(Debug, Clone)]
        pub struct BatchGetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            max_members: Option<i32>,
            resource_names: Option<Vec<String>>,
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
        impl<'a> BatchGetRequestBuilder<'a> {
            #[doc = "Specifies the maximum number of members to return for each group."]
            pub fn max_members(mut self, value: i32) -> Self {
                self.max_members = Some(value);
                self
            }
            #[doc = "The resource names of the contact groups to get."]
            pub fn resource_names(mut self, value: impl Into<Vec<String>>) -> Self {
                self.resource_names = Some(value.into());
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
            ) -> Result<crate::schemas::BatchGetContactGroupsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchGetContactGroupsResponse, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/contactGroups:batchGet");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxMembers", &self.max_members)]);
                let req = req.query(&[("resourceNames", &self.resource_names)]);
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
        #[doc = "Created via [ContactGroupsActions::create()](struct.ContactGroupsActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CreateContactGroupRequest,
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
            ) -> Result<crate::schemas::ContactGroup, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ContactGroup, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/contactGroups");
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
        #[doc = "Created via [ContactGroupsActions::delete()](struct.ContactGroupsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            resource_name: String,
            delete_contacts: Option<bool>,
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
            #[doc = "Set to true to also delete the contacts in the specified group."]
            pub fn delete_contacts(mut self, value: bool) -> Self {
                self.delete_contacts = Some(value);
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
            ) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Empty, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("deleteContacts", &self.delete_contacts)]);
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
        #[doc = "Created via [ContactGroupsActions::get()](struct.ContactGroupsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            resource_name: String,
            max_members: Option<i32>,
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
            #[doc = "Specifies the maximum number of members to return."]
            pub fn max_members(mut self, value: i32) -> Self {
                self.max_members = Some(value);
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
            ) -> Result<crate::schemas::ContactGroup, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ContactGroup, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxMembers", &self.max_members)]);
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
        #[doc = "Created via [ContactGroupsActions::list()](struct.ContactGroupsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            page_size: Option<i32>,
            page_token: Option<String>,
            sync_token: Option<String>,
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
            #[doc = "The maximum number of resources to return."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The next_page_token value returned from a previous call to\n[ListContactGroups](/people/api/rest/v1/contactgroups/list).\nRequests the next page of resources."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "A sync token, returned by a previous call to `contactgroups.list`.\nOnly resources changed since the sync token was created will be returned."]
            pub fn sync_token(mut self, value: impl Into<String>) -> Self {
                self.sync_token = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_contact_groups<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_contact_groups_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_contact_groups_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ContactGroup> {
                self.iter_contact_groups_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_contact_groups_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::ContactGroup> {
                self.iter_contact_groups_with_fields(Some("*"))
            }
            pub fn iter_contact_groups_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "contactGroups").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "contactGroups")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListContactGroupsResponse>
            {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListContactGroupsResponse>
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
            ) -> Result<crate::schemas::ListContactGroupsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListContactGroupsResponse, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/contactGroups");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("syncToken", &self.sync_token)]);
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
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
        #[doc = "Created via [ContactGroupsActions::update()](struct.ContactGroupsActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::UpdateContactGroupRequest,
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
        impl<'a> UpdateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ContactGroup, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ContactGroup, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
                #[doc = "Modify the members of a contact group owned by the authenticated user.\n<br>\nThe only system contact groups that can have members added are\n`contactGroups/myContacts` and `contactGroups/starred`. Other system\ncontact groups are deprecated and can only have contacts removed."]
                pub fn modify(
                    &self,
                    request: crate::schemas::ModifyContactGroupMembersRequest,
                    resource_name: impl Into<String>,
                ) -> ModifyRequestBuilder {
                    ModifyRequestBuilder {
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
                        resource_name: resource_name.into(),
                    }
                }
            }
            #[doc = "Created via [MembersActions::modify()](struct.MembersActions.html#method.modify)"]
            #[derive(Debug, Clone)]
            pub struct ModifyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ModifyContactGroupMembersRequest,
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
            impl<'a> ModifyRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::ModifyContactGroupMembersResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ModifyContactGroupMembersResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://people.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/members:modify");
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
    pub mod people {
        pub mod params {}
        pub struct PeopleActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PeopleActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Create a new contact and return the person resource for that contact."]
            pub fn create_contact(
                &self,
                request: crate::schemas::Person,
            ) -> CreateContactRequestBuilder {
                CreateContactRequestBuilder {
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
                    parent: None,
                }
            }
            #[doc = "Delete a contact person. Any non-contact data will not be deleted."]
            pub fn delete_contact(
                &self,
                resource_name: impl Into<String>,
            ) -> DeleteContactRequestBuilder {
                DeleteContactRequestBuilder {
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
            #[doc = "Delete a contact's photo."]
            pub fn delete_contact_photo(
                &self,
                resource_name: impl Into<String>,
            ) -> DeleteContactPhotoRequestBuilder {
                DeleteContactPhotoRequestBuilder {
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
                    person_fields: None,
                }
            }
            #[doc = "Provides information about a person by specifying a resource name. Use\n`people/me` to indicate the authenticated user.\n<br>\nThe request throws a 400 error if 'personFields' is not specified."]
            pub fn get(&self, resource_name: impl Into<String>) -> GetRequestBuilder {
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
                    resource_name: resource_name.into(),
                    person_fields: None,
                    request_mask_include_field: None,
                }
            }
            #[doc = "Provides information about a list of specific people by specifying a list\nof requested resource names. Use `people/me` to indicate the authenticated\nuser.\n<br>\nThe request throws a 400 error if 'personFields' is not specified."]
            pub fn get_batch_get(&self) -> GetBatchGetRequestBuilder {
                GetBatchGetRequestBuilder {
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
                    person_fields: None,
                    request_mask_include_field: None,
                    resource_names: None,
                }
            }
            #[doc = "Update contact data for an existing contact person. Any non-contact data\nwill not be modified.\n\nThe request throws a 400 error if `updatePersonFields` is not specified.\n<br>\nThe request throws a 400 error if `person.metadata.sources` is not\nspecified for the contact to be updated.\n<br>\nThe request throws a 400 error with an error with reason\n`\"failedPrecondition\"` if `person.metadata.sources.etag` is different than\nthe contact's etag, which indicates the contact has changed since its data\nwas read. Clients should get the latest person and re-apply their updates\nto the latest person."]
            pub fn update_contact(
                &self,
                request: crate::schemas::Person,
                resource_name: impl Into<String>,
            ) -> UpdateContactRequestBuilder {
                UpdateContactRequestBuilder {
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
                    resource_name: resource_name.into(),
                    update_person_fields: None,
                }
            }
            #[doc = "Update a contact's photo."]
            pub fn update_contact_photo(
                &self,
                request: crate::schemas::UpdateContactPhotoRequest,
                resource_name: impl Into<String>,
            ) -> UpdateContactPhotoRequestBuilder {
                UpdateContactPhotoRequestBuilder {
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
                    resource_name: resource_name.into(),
                }
            }
            #[doc = "Actions that can be performed on the connections resource"]
            pub fn connections(&self) -> crate::resources::people::connections::ConnectionsActions {
                crate::resources::people::connections::ConnectionsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [PeopleActions::create_contact()](struct.PeopleActions.html#method.create_contact)"]
        #[derive(Debug, Clone)]
        pub struct CreateContactRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Person,
            parent: Option<String>,
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
        impl<'a> CreateContactRequestBuilder<'a> {
            #[doc = "The resource name of the owning person resource."]
            pub fn parent(mut self, value: impl Into<String>) -> Self {
                self.parent = Some(value.into());
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
            ) -> Result<crate::schemas::Person, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Person, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/people:createContact");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("parent", &self.parent)]);
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
        #[doc = "Created via [PeopleActions::delete_contact()](struct.PeopleActions.html#method.delete_contact)"]
        #[derive(Debug, Clone)]
        pub struct DeleteContactRequestBuilder<'a> {
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
        impl<'a> DeleteContactRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Empty, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":deleteContact");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
        #[doc = "Created via [PeopleActions::delete_contact_photo()](struct.PeopleActions.html#method.delete_contact_photo)"]
        #[derive(Debug, Clone)]
        pub struct DeleteContactPhotoRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            resource_name: String,
            person_fields: Option<String>,
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
        impl<'a> DeleteContactPhotoRequestBuilder<'a> {
            #[doc = "**Optional.** Not specifying any fields will skip the post mutate read.\nA field mask to restrict which fields on the person are\nreturned. Multiple fields can be specified by separating them with commas.\nValid values are:\n\n* addresses\n* ageRanges\n* biographies\n* birthdays\n* braggingRights\n* coverPhotos\n* emailAddresses\n* events\n* genders\n* imClients\n* interests\n* locales\n* memberships\n* metadata\n* names\n* nicknames\n* occupations\n* organizations\n* phoneNumbers\n* photos\n* relations\n* relationshipInterests\n* relationshipStatuses\n* residences\n* sipAddresses\n* skills\n* taglines\n* urls\n* userDefined"]
            pub fn person_fields(mut self, value: impl Into<String>) -> Self {
                self.person_fields = Some(value.into());
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
            ) -> Result<crate::schemas::DeleteContactPhotoResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DeleteContactPhotoResponse, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":deleteContactPhoto");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("personFields", &self.person_fields)]);
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
        #[doc = "Created via [PeopleActions::get()](struct.PeopleActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            resource_name: String,
            person_fields: Option<String>,
            request_mask_include_field: Option<String>,
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
            #[doc = "**Required.** A field mask to restrict which fields on the person are\nreturned. Multiple fields can be specified by separating them with commas.\nValid values are:\n\n* addresses\n* ageRanges\n* biographies\n* birthdays\n* braggingRights\n* coverPhotos\n* emailAddresses\n* events\n* genders\n* imClients\n* interests\n* locales\n* memberships\n* metadata\n* names\n* nicknames\n* occupations\n* organizations\n* phoneNumbers\n* photos\n* relations\n* relationshipInterests\n* relationshipStatuses\n* residences\n* sipAddresses\n* skills\n* taglines\n* urls\n* userDefined"]
            pub fn person_fields(mut self, value: impl Into<String>) -> Self {
                self.person_fields = Some(value.into());
                self
            }
            #[doc = "**Required.** Comma-separated list of person fields to be included in the\nresponse. Each path should start with `person.`: for example,\n`person.names` or `person.photos`."]
            pub fn request_mask_include_field(mut self, value: impl Into<String>) -> Self {
                self.request_mask_include_field = Some(value.into());
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
            ) -> Result<crate::schemas::Person, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Person, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("personFields", &self.person_fields)]);
                let req =
                    req.query(&[("requestMask.includeField", &self.request_mask_include_field)]);
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
        #[doc = "Created via [PeopleActions::get_batch_get()](struct.PeopleActions.html#method.get_batch_get)"]
        #[derive(Debug, Clone)]
        pub struct GetBatchGetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            person_fields: Option<String>,
            request_mask_include_field: Option<String>,
            resource_names: Option<Vec<String>>,
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
        impl<'a> GetBatchGetRequestBuilder<'a> {
            #[doc = "**Required.** A field mask to restrict which fields on each person are\nreturned. Multiple fields can be specified by separating them with commas.\nValid values are:\n\n* addresses\n* ageRanges\n* biographies\n* birthdays\n* braggingRights\n* coverPhotos\n* emailAddresses\n* events\n* genders\n* imClients\n* interests\n* locales\n* memberships\n* metadata\n* names\n* nicknames\n* occupations\n* organizations\n* phoneNumbers\n* photos\n* relations\n* relationshipInterests\n* relationshipStatuses\n* residences\n* sipAddresses\n* skills\n* taglines\n* urls\n* userDefined"]
            pub fn person_fields(mut self, value: impl Into<String>) -> Self {
                self.person_fields = Some(value.into());
                self
            }
            #[doc = "**Required.** Comma-separated list of person fields to be included in the\nresponse. Each path should start with `person.`: for example,\n`person.names` or `person.photos`."]
            pub fn request_mask_include_field(mut self, value: impl Into<String>) -> Self {
                self.request_mask_include_field = Some(value.into());
                self
            }
            #[doc = "The resource names of the people to provide information about.\n\n* To get information about the authenticated user, specify `people/me`.\n* To get information about a google account, specify\n  `people/`<var>account_id</var>.\n* To get information about a contact, specify the resource name that\n  identifies the contact as returned by\n  [`people.connections.list`](/people/api/rest/v1/people.connections/list).\n\nYou can include up to 50 resource names in one request."]
            pub fn resource_names(mut self, value: impl Into<Vec<String>>) -> Self {
                self.resource_names = Some(value.into());
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
            ) -> Result<crate::schemas::GetPeopleResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetPeopleResponse, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/people:batchGet");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("personFields", &self.person_fields)]);
                let req =
                    req.query(&[("requestMask.includeField", &self.request_mask_include_field)]);
                let req = req.query(&[("resourceNames", &self.resource_names)]);
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
        #[doc = "Created via [PeopleActions::update_contact()](struct.PeopleActions.html#method.update_contact)"]
        #[derive(Debug, Clone)]
        pub struct UpdateContactRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Person,
            resource_name: String,
            update_person_fields: Option<String>,
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
        impl<'a> UpdateContactRequestBuilder<'a> {
            #[doc = "**Required.** A field mask to restrict which fields on the person are\nupdated. Multiple fields can be specified by separating them with commas.\nAll updated fields will be replaced. Valid values are:\n\n* addresses\n* biographies\n* birthdays\n* emailAddresses\n* events\n* genders\n* imClients\n* interests\n* locales\n* memberships\n* names\n* nicknames\n* occupations\n* organizations\n* phoneNumbers\n* relations\n* residences\n* sipAddresses\n* urls\n* userDefined"]
            pub fn update_person_fields(mut self, value: impl Into<String>) -> Self {
                self.update_person_fields = Some(value.into());
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
            ) -> Result<crate::schemas::Person, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Person, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":updateContact");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
                let req = req.query(&[("updatePersonFields", &self.update_person_fields)]);
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
        #[doc = "Created via [PeopleActions::update_contact_photo()](struct.PeopleActions.html#method.update_contact_photo)"]
        #[derive(Debug, Clone)]
        pub struct UpdateContactPhotoRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::UpdateContactPhotoRequest,
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
        impl<'a> UpdateContactPhotoRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::UpdateContactPhotoResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::UpdateContactPhotoResponse, crate::Error> {
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
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://people.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":updateContactPhoto");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
        pub mod connections {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListSortOrder {
                    FirstNameAscending,
                    LastModifiedAscending,
                    LastModifiedDescending,
                    LastNameAscending,
                }
                impl ListSortOrder {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSortOrder::FirstNameAscending => "FIRST_NAME_ASCENDING",
                            ListSortOrder::LastModifiedAscending => "LAST_MODIFIED_ASCENDING",
                            ListSortOrder::LastModifiedDescending => "LAST_MODIFIED_DESCENDING",
                            ListSortOrder::LastNameAscending => "LAST_NAME_ASCENDING",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListSortOrder {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListSortOrder {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListSortOrder, ()> {
                        Ok(match s {
                            "FIRST_NAME_ASCENDING" => ListSortOrder::FirstNameAscending,
                            "LAST_MODIFIED_ASCENDING" => ListSortOrder::LastModifiedAscending,
                            "LAST_MODIFIED_DESCENDING" => ListSortOrder::LastModifiedDescending,
                            "LAST_NAME_ASCENDING" => ListSortOrder::LastNameAscending,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListSortOrder {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListSortOrder {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListSortOrder {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "FIRST_NAME_ASCENDING" => ListSortOrder::FirstNameAscending,
                            "LAST_MODIFIED_ASCENDING" => ListSortOrder::LastModifiedAscending,
                            "LAST_MODIFIED_DESCENDING" => ListSortOrder::LastModifiedDescending,
                            "LAST_NAME_ASCENDING" => ListSortOrder::LastNameAscending,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListSortOrder {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListSortOrder {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ConnectionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConnectionsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Provides a list of the authenticated user's contacts merged with any\nconnected profiles.\n<br>\nThe request throws a 400 error if 'personFields' is not specified."]
                pub fn list(&self, resource_name: impl Into<String>) -> ListRequestBuilder {
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
                        resource_name: resource_name.into(),
                        page_size: None,
                        page_token: None,
                        person_fields: None,
                        request_mask_include_field: None,
                        request_sync_token: None,
                        sort_order: None,
                        sync_token: None,
                    }
                }
            }
            #[doc = "Created via [ConnectionsActions::list()](struct.ConnectionsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                resource_name: String,
                page_size: Option<i32>,
                page_token: Option<String>,
                person_fields: Option<String>,
                request_mask_include_field: Option<String>,
                request_sync_token: Option<bool>,
                sort_order: Option<crate::resources::people::connections::params::ListSortOrder>,
                sync_token: Option<String>,
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
                #[doc = "The number of connections to include in the response. Valid values are\nbetween 1 and 2000, inclusive. Defaults to 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The token of the page to be returned."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "**Required.** A field mask to restrict which fields on each person are\nreturned. Multiple fields can be specified by separating them with commas.\nValid values are:\n\n* addresses\n* ageRanges\n* biographies\n* birthdays\n* braggingRights\n* coverPhotos\n* emailAddresses\n* events\n* genders\n* imClients\n* interests\n* locales\n* memberships\n* metadata\n* names\n* nicknames\n* occupations\n* organizations\n* phoneNumbers\n* photos\n* relations\n* relationshipInterests\n* relationshipStatuses\n* residences\n* sipAddresses\n* skills\n* taglines\n* urls\n* userDefined"]
                pub fn person_fields(mut self, value: impl Into<String>) -> Self {
                    self.person_fields = Some(value.into());
                    self
                }
                #[doc = "**Required.** Comma-separated list of person fields to be included in the\nresponse. Each path should start with `person.`: for example,\n`person.names` or `person.photos`."]
                pub fn request_mask_include_field(mut self, value: impl Into<String>) -> Self {
                    self.request_mask_include_field = Some(value.into());
                    self
                }
                #[doc = "Whether the response should include a sync token, which can be used to get\nall changes since the last request. For subsequent sync requests use the\n`sync_token` param instead. Initial sync requests that specify\n`request_sync_token` have an additional rate limit."]
                pub fn request_sync_token(mut self, value: bool) -> Self {
                    self.request_sync_token = Some(value);
                    self
                }
                #[doc = "The order in which the connections should be sorted. Defaults to\n`LAST_MODIFIED_ASCENDING`."]
                pub fn sort_order(
                    mut self,
                    value: crate::resources::people::connections::params::ListSortOrder,
                ) -> Self {
                    self.sort_order = Some(value);
                    self
                }
                #[doc = "A sync token returned by a previous call to `people.connections.list`.\nOnly resources changed since the sync token was created will be returned.\nSync requests that specify `sync_token` have an additional rate limit."]
                pub fn sync_token(mut self, value: impl Into<String>) -> Self {
                    self.sync_token = Some(value.into());
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_connections<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_connections_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_connections_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Person> {
                    self.iter_connections_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_connections_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Person> {
                    self.iter_connections_with_fields(Some("*"))
                }
                pub fn iter_connections_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "connections").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "connections")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListConnectionsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListConnectionsResponse>
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
                ) -> Result<crate::schemas::ListConnectionsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListConnectionsResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://people.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource_name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/connections");
                    output
                }
                fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("personFields", &self.person_fields)]);
                    let req = req
                        .query(&[("requestMask.includeField", &self.request_mask_include_field)]);
                    let req = req.query(&[("requestSyncToken", &self.request_sync_token)]);
                    let req = req.query(&[("sortOrder", &self.sort_order)]);
                    let req = req.query(&[("syncToken", &self.sync_token)]);
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
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest(::reqwest::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest(err) => err
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
            Error::Reqwest(err) => write!(f, "Reqwest Error: {}", err),
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
    fn from(err: ::reqwest::Error) -> Error {
        Error::Reqwest(err)
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
