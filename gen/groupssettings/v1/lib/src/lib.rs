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
    pub struct Groups {
        #[doc = "Identifies whether members external to your organization can join the group. Possible values are:\n\n* true: G Suite users external to your organization can become members of this group. \n* false: Users not belonging to the organization are not allowed to become members of this group."]
        #[serde(rename = "allowExternalMembers", default)]
        pub allow_external_members: Option<String>,
        #[doc = "Deprecated. Allows Google to contact administrator of the group.\n\n* true: Allow Google to contact managers of this group. Occasionally Google may send updates on the latest features, ask for input on new features, or ask for permission to highlight your group. \n* false: Google can not contact managers of this group."]
        #[serde(rename = "allowGoogleCommunication", default)]
        pub allow_google_communication: Option<String>,
        #[doc = "Allows posting from web. Possible values are:\n\n* true: Allows any member to post to the group forum. \n* false: Members only use Gmail to communicate with the group."]
        #[serde(rename = "allowWebPosting", default)]
        pub allow_web_posting: Option<String>,
        #[doc = "Allows the group to be archived only. Possible values are:\n\n* true: Group is archived and the group is inactive. New messages to this group are rejected. The older archived messages are browseable and searchable.\n* If true, the whoCanPostMessage property is set to NONE_CAN_POST.\n* If reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.\n* false: The group is active and can receive messages.\n* When false, updating whoCanPostMessage to NONE_CAN_POST, results in an error."]
        #[serde(rename = "archiveOnly", default)]
        pub archive_only: Option<String>,
        #[doc = "Set the content of custom footer text. The maximum number of characters is 1,000."]
        #[serde(rename = "customFooterText", default)]
        pub custom_footer_text: Option<String>,
        #[doc = "An email address used when replying to a message if the replyTo property is set to REPLY_TO_CUSTOM. This address is defined by an account administrator.\n\n* When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds a custom email address used when replying to a message. \n* If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a text value or an error is returned."]
        #[serde(rename = "customReplyTo", default)]
        pub custom_reply_to: Option<String>,
        #[doc = "Specifies whether the group has a custom role that's included in one of the settings being merged. This field is read-only and update/patch requests to it are ignored. Possible values are:\n\n* true \n* false"]
        #[serde(rename = "customRolesEnabledForSettingsToBeMerged", default)]
        pub custom_roles_enabled_for_settings_to_be_merged: Option<String>,
        #[doc = "When a message is rejected, this is text for the rejection notification sent to the message's author. By default, this property is empty and has no value in the API's response body. The maximum notification text size is 10,000 characters. Note: Requires sendMessageDenyNotification property to be true."]
        #[serde(rename = "defaultMessageDenyNotificationText", default)]
        pub default_message_deny_notification_text: Option<String>,
        #[doc = "Description of the group. This property value may be an empty string if no group description has been entered. If entered, the maximum group description is no more than 300 characters."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The group's email address. This property can be updated using the Directory API. Note: Only a group owner can change a group's email address. A group manager can't do this.\nWhen you change your group's address using the Directory API or the control panel, you are changing the address your subscribers use to send email and the web address people use to access your group. People can't reach your group by visiting the old address."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "Specifies whether a collaborative inbox will remain turned on for the group. Possible values are:\n\n* true \n* false"]
        #[serde(rename = "enableCollaborativeInbox", default)]
        pub enable_collaborative_inbox: Option<String>,
        #[doc = "Indicates if favorite replies should be displayed above other replies.\n\n* true: Favorite replies will be displayed above other replies. \n* false: Favorite replies will not be displayed above other replies."]
        #[serde(rename = "favoriteRepliesOnTop", default)]
        pub favorite_replies_on_top: Option<String>,
        #[doc = "Whether to include custom footer. Possible values are:\n\n* true \n* false"]
        #[serde(rename = "includeCustomFooter", default)]
        pub include_custom_footer: Option<String>,
        #[doc = "Enables the group to be included in the Global Address List. For more information, see the help center. Possible values are:\n\n* true: Group is included in the Global Address List. \n* false: Group is not included in the Global Address List."]
        #[serde(rename = "includeInGlobalAddressList", default)]
        pub include_in_global_address_list: Option<String>,
        #[doc = "Allows the Group contents to be archived. Possible values are:\n\n* true: Archive messages sent to the group. \n* false: Do not keep an archive of messages sent to this group. If false, previously archived messages remain in the archive."]
        #[serde(rename = "isArchived", default)]
        pub is_archived: Option<String>,
        #[doc = "The type of the resource. It is always groupsSettings#groups."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Deprecated. The maximum size of a message is 25Mb."]
        #[serde(rename = "maxMessageBytes", default)]
        pub max_message_bytes: Option<i32>,
        #[doc = "Enables members to post messages as the group. Possible values are:\n\n* true: Group member can post messages using the group's email address instead of their own email address. Message appear to originate from the group itself. Note: When true, any message moderation settings on individual users or new members do not apply to posts made on behalf of the group. \n* false: Members can not post in behalf of the group's email address."]
        #[serde(rename = "membersCanPostAsTheGroup", default)]
        pub members_can_post_as_the_group: Option<String>,
        #[doc = "Deprecated. The default message display font always has a value of \"DEFAULT_FONT\"."]
        #[serde(rename = "messageDisplayFont", default)]
        pub message_display_font: Option<String>,
        #[doc = "Moderation level of incoming messages. Possible values are:\n\n* MODERATE_ALL_MESSAGES: All messages are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n* MODERATE_NON_MEMBERS: All messages from non group members are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n* MODERATE_NEW_MEMBERS: All messages from new members are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n* MODERATE_NONE: No moderator approval is required. Messages are delivered directly to the group. Note: When the whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.\n  When memberCanPostAsTheGroup is true, any message moderation settings on individual users or new members will not apply to posts made on behalf of the group."]
        #[serde(rename = "messageModerationLevel", default)]
        pub message_moderation_level: Option<String>,
        #[doc = "Name of the group, which has a maximum size of 75 characters."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The primary language for group. For a group's primary language use the language tags from the G Suite languages found at G Suite Email Settings API Email Language Tags."]
        #[serde(rename = "primaryLanguage", default)]
        pub primary_language: Option<String>,
        #[doc = "Specifies who should the default reply go to. Possible values are:\n\n* REPLY_TO_CUSTOM: For replies to messages, use the group's custom email address.\n  When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds the custom email address used when replying to a message. If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a value. Otherwise an error is returned.\n\n* REPLY_TO_SENDER: The reply sent to author of message. \n\n* REPLY_TO_LIST: This reply message is sent to the group. \n\n* REPLY_TO_OWNER: The reply is sent to the owner(s) of the group. This does not include the group's managers. \n\n* REPLY_TO_IGNORE: Group users individually decide where the message reply is sent. \n\n* REPLY_TO_MANAGERS: This reply message is sent to the group's managers, which includes all managers and the group owner."]
        #[serde(rename = "replyTo", default)]
        pub reply_to: Option<String>,
        #[doc = "Allows a member to be notified if the member's message to the group is denied by the group owner. Possible values are:\n\n* true: When a message is rejected, send the deny message notification to the message author.\n  The defaultMessageDenyNotificationText property is dependent on the sendMessageDenyNotification property being true.\n\n* false: When a message is rejected, no notification is sent."]
        #[serde(rename = "sendMessageDenyNotification", default)]
        pub send_message_deny_notification: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanDiscoverGroup setting. Allows the group to be visible in the Groups Directory. Possible values are:\n\n* true: All groups in the account are listed in the Groups directory. \n* false: All groups in the account are not listed in the directory."]
        #[serde(rename = "showInGroupDirectory", default)]
        pub show_in_group_directory: Option<String>,
        #[doc = "Specifies moderation levels for messages detected as spam. Possible values are:\n\n* ALLOW: Post the message to the group. \n* MODERATE: Send the message to the moderation queue. This is the default. \n* SILENTLY_MODERATE: Send the message to the moderation queue, but do not send notification to moderators. \n* REJECT: Immediately reject the message."]
        #[serde(rename = "spamModerationLevel", default)]
        pub spam_moderation_level: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to add members. Possible values are:\n\n* ALL_MEMBERS_CAN_ADD: Managers and members can directly add new members. \n* ALL_MANAGERS_CAN_ADD: Only managers can directly add new members. this includes the group's owner. \n* ALL_OWNERS_CAN_ADD: Only owners can directly add new members. \n* NONE_CAN_ADD: No one can directly add new members."]
        #[serde(rename = "whoCanAdd", default)]
        pub who_can_add: Option<String>,
        #[doc = "Deprecated. This functionality is no longer supported in the Google Groups UI. The value is always \"NONE\"."]
        #[serde(rename = "whoCanAddReferences", default)]
        pub who_can_add_references: Option<String>,
        #[doc = "Specifies who can approve members who ask to join groups. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:\n\n* ALL_MEMBERS_CAN_APPROVE \n* ALL_MANAGERS_CAN_APPROVE \n* ALL_OWNERS_CAN_APPROVE \n* NONE_CAN_APPROVE"]
        #[serde(rename = "whoCanApproveMembers", default)]
        pub who_can_approve_members: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can approve pending messages in the moderation queue. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanApproveMessages", default)]
        pub who_can_approve_messages: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to assign topics in a forum to another user. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanAssignTopics", default)]
        pub who_can_assign_topics: Option<String>,
        #[doc = "Specifies who can moderate metadata. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanAssistContent", default)]
        pub who_can_assist_content: Option<String>,
        #[doc = "Specifies who can deny membership to users. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanBanUsers", default)]
        pub who_can_ban_users: Option<String>,
        #[doc = "Permission to contact owner of the group via web UI. Possible values are:\n\n* ALL_IN_DOMAIN_CAN_CONTACT \n* ALL_MANAGERS_CAN_CONTACT \n* ALL_MEMBERS_CAN_CONTACT \n* ANYONE_CAN_CONTACT"]
        #[serde(rename = "whoCanContactOwner", default)]
        pub who_can_contact_owner: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete replies to topics. (Authors can always delete their own posts). Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanDeleteAnyPost", default)]
        pub who_can_delete_any_post: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete topics. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanDeleteTopics", default)]
        pub who_can_delete_topics: Option<String>,
        #[doc = "Specifies the set of users for whom this group is discoverable. Possible values are:\n\n* ANYONE_CAN_DISCOVER \n* ALL_IN_DOMAIN_CAN_DISCOVER \n* ALL_MEMBERS_CAN_DISCOVER"]
        #[serde(rename = "whoCanDiscoverGroup", default)]
        pub who_can_discover_group: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to enter free form tags for topics in a forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanEnterFreeFormTags", default)]
        pub who_can_enter_free_form_tags: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can hide posts by reporting them as abuse. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanHideAbuse", default)]
        pub who_can_hide_abuse: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to invite new members. Possible values are:\n\n* ALL_MEMBERS_CAN_INVITE: Managers and members can invite a new member candidate. \n* ALL_MANAGERS_CAN_INVITE: Only managers can invite a new member. This includes the group's owner. \n* ALL_OWNERS_CAN_INVITE: Only owners can invite a new member. \n* NONE_CAN_INVITE: No one can invite a new member candidate."]
        #[serde(rename = "whoCanInvite", default)]
        pub who_can_invite: Option<String>,
        #[doc = "Permission to join group. Possible values are:\n\n* ANYONE_CAN_JOIN: Anyone in the account domain can join. This includes accounts with multiple domains. \n* ALL_IN_DOMAIN_CAN_JOIN: Any Internet user who is outside your domain can access your Google Groups service and view the list of groups in your Groups directory. Warning: Group owners can add external addresses, outside of the domain to their groups. They can also allow people outside your domain to join their groups. If you later disable this option, any external addresses already added to users' groups remain in those groups. \n* INVITED_CAN_JOIN: Candidates for membership can be invited to join.\n* CAN_REQUEST_TO_JOIN: Non members can request an invitation to join."]
        #[serde(rename = "whoCanJoin", default)]
        pub who_can_join: Option<String>,
        #[doc = "Permission to leave the group. Possible values are:\n\n* ALL_MANAGERS_CAN_LEAVE \n* ALL_MEMBERS_CAN_LEAVE \n* NONE_CAN_LEAVE"]
        #[serde(rename = "whoCanLeaveGroup", default)]
        pub who_can_leave_group: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can prevent users from posting replies to topics. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanLockTopics", default)]
        pub who_can_lock_topics: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can make topics appear at the top of the topic list. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanMakeTopicsSticky", default)]
        pub who_can_make_topics_sticky: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as a duplicate of another topic. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanMarkDuplicate", default)]
        pub who_can_mark_duplicate: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark any other user's post as a favorite reply. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanMarkFavoriteReplyOnAnyTopic", default)]
        pub who_can_mark_favorite_reply_on_any_topic: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a post for a topic they started as a favorite reply. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanMarkFavoriteReplyOnOwnTopic", default)]
        pub who_can_mark_favorite_reply_on_own_topic: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as not needing a response. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanMarkNoResponseNeeded", default)]
        pub who_can_mark_no_response_needed: Option<String>,
        #[doc = "Specifies who can moderate content. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanModerateContent", default)]
        pub who_can_moderate_content: Option<String>,
        #[doc = "Specifies who can manage members. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanModerateMembers", default)]
        pub who_can_moderate_members: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Specifies who can change group members' roles. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanModifyMembers", default)]
        pub who_can_modify_members: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to change tags and categories. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanModifyTagsAndCategories", default)]
        pub who_can_modify_tags_and_categories: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics into the group or forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanMoveTopicsIn", default)]
        pub who_can_move_topics_in: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics out of the group or forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanMoveTopicsOut", default)]
        pub who_can_move_topics_out: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can post announcements, a special topic type. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanPostAnnouncements", default)]
        pub who_can_post_announcements: Option<String>,
        #[doc = "Permissions to post messages. Possible values are:\n\n* NONE_CAN_POST: The group is disabled and archived. No one can post a message to this group.\n* When archiveOnly is false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. \n* If archiveOnly is reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.\n* ALL_MANAGERS_CAN_POST: Managers, including group owners, can post messages. \n* ALL_MEMBERS_CAN_POST: Any group member can post a message. \n* ALL_OWNERS_CAN_POST: Only group owners can post a message. \n* ALL_IN_DOMAIN_CAN_POST: Anyone in the account can post a message.\n* ANYONE_CAN_POST: Any Internet user who outside your account can access your Google Groups service and post a message. Note: When whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam."]
        #[serde(rename = "whoCanPostMessage", default)]
        pub who_can_post_message: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to take topics in a forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanTakeTopics", default)]
        pub who_can_take_topics: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unassign any topic in a forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanUnassignTopic", default)]
        pub who_can_unassign_topic: Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unmark any post from a favorite reply. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(rename = "whoCanUnmarkFavoriteReplyOnAnyTopic", default)]
        pub who_can_unmark_favorite_reply_on_any_topic: Option<String>,
        #[doc = "Permissions to view group messages. Possible values are:\n\n* ANYONE_CAN_VIEW: Any Internet user can view the group's messages.\n* ALL_IN_DOMAIN_CAN_VIEW: Anyone in your account can view this group's messages. \n* ALL_MEMBERS_CAN_VIEW: All group members can view the group's messages. \n* ALL_MANAGERS_CAN_VIEW: Any group manager can view this group's messages."]
        #[serde(rename = "whoCanViewGroup", default)]
        pub who_can_view_group: Option<String>,
        #[doc = "Permissions to view membership. Possible values are:\n\n* ALL_IN_DOMAIN_CAN_VIEW: Anyone in the account can view the group members list.\n  If a group already has external members, those members can still send email to this group.\n\n* ALL_MEMBERS_CAN_VIEW: The group members can view the group members list. \n\n* ALL_MANAGERS_CAN_VIEW: The group managers can view group members list."]
        #[serde(rename = "whoCanViewMembership", default)]
        pub who_can_view_membership: Option<String>,
    }
    impl ::field_selector::FieldSelector for Groups {
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
        #[doc = "Responses with Content-Type of application/atom+xml"]
        Atom,
        #[doc = "Responses with Content-Type of application/json"]
        Json,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Atom => "atom",
                Alt::Json => "json",
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
                "atom" => Alt::Atom,
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
    #[doc = "Actions that can be performed on the groups resource"]
    pub fn groups(&self) -> crate::resources::groups::GroupsActions<A> {
        crate::resources::groups::GroupsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
mod resources {
    pub mod groups {
        pub mod params {}
        pub struct GroupsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> GroupsActions<'a, A> {
            #[doc = "Gets one resource by id."]
            pub fn get(&self, group_unique_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                    group_unique_id: group_unique_id.into(),
                }
            }
            #[doc = "Updates an existing resource. This method supports patch semantics."]
            pub fn patch(
                &self,
                request: crate::schemas::Groups,
                group_unique_id: impl Into<String>,
            ) -> PatchRequestBuilder<A> {
                PatchRequestBuilder {
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
                    group_unique_id: group_unique_id.into(),
                }
            }
            #[doc = "Updates an existing resource."]
            pub fn update(
                &self,
                request: crate::schemas::Groups,
                group_unique_id: impl Into<String>,
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
                    group_unique_id: group_unique_id.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            group_unique_id: String,
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
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
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
            ) -> Result<crate::schemas::Groups, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Groups, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/groups/v1/groups/".to_owned();
                {
                    let var_as_str = &self.group_unique_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/apps.groups.settings"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Groups,
            group_unique_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
            #[doc = "Data format for the response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
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
            ) -> Result<crate::schemas::Groups, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Groups, Box<dyn ::std::error::Error>> {
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/groups/v1/groups/".to_owned();
                {
                    let var_as_str = &self.group_unique_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/apps.groups.settings"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Groups,
            group_unique_id: String,
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
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
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
            ) -> Result<crate::schemas::Groups, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Groups, Box<dyn ::std::error::Error>> {
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
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/groups/v1/groups/".to_owned();
                {
                    let var_as_str = &self.group_unique_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/apps.groups.settings"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
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
}
