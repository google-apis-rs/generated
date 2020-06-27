#![doc = "# Resources and Methods\n    * [groups](resources/groups/struct.GroupsActions.html)\n      * [*get*](resources/groups/struct.GetRequestBuilder.html), [*patch*](resources/groups/struct.PatchRequestBuilder.html), [*update*](resources/groups/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage the settings of a G Suite group\n\n`https://www.googleapis.com/auth/apps.groups.settings`"]
    pub const APPS_GROUPS_SETTINGS: &str = "https://www.googleapis.com/auth/apps.groups.settings";
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
    pub struct Groups {
        #[doc = "Identifies whether members external to your organization can join the group. Possible values are:\n\n* true: G Suite users external to your organization can become members of this group. \n* false: Users not belonging to the organization are not allowed to become members of this group."]
        #[serde(
            rename = "allowExternalMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_external_members: ::std::option::Option<String>,
        #[doc = "Deprecated. Allows Google to contact administrator of the group.\n\n* true: Allow Google to contact managers of this group. Occasionally Google may send updates on the latest features, ask for input on new features, or ask for permission to highlight your group. \n* false: Google can not contact managers of this group."]
        #[serde(
            rename = "allowGoogleCommunication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_google_communication: ::std::option::Option<String>,
        #[doc = "Allows posting from web. Possible values are:\n\n* true: Allows any member to post to the group forum. \n* false: Members only use Gmail to communicate with the group."]
        #[serde(
            rename = "allowWebPosting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_web_posting: ::std::option::Option<String>,
        #[doc = "Allows the group to be archived only. Possible values are:\n\n* true: Group is archived and the group is inactive. New messages to this group are rejected. The older archived messages are browseable and searchable.\n* If true, the whoCanPostMessage property is set to NONE_CAN_POST.\n* If reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.\n* false: The group is active and can receive messages.\n* When false, updating whoCanPostMessage to NONE_CAN_POST, results in an error."]
        #[serde(
            rename = "archiveOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub archive_only: ::std::option::Option<String>,
        #[doc = "Set the content of custom footer text. The maximum number of characters is 1,000."]
        #[serde(
            rename = "customFooterText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_footer_text: ::std::option::Option<String>,
        #[doc = "An email address used when replying to a message if the replyTo property is set to REPLY_TO_CUSTOM. This address is defined by an account administrator.\n\n* When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds a custom email address used when replying to a message. \n* If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a text value or an error is returned."]
        #[serde(
            rename = "customReplyTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_reply_to: ::std::option::Option<String>,
        #[doc = "Specifies whether the group has a custom role that's included in one of the settings being merged. This field is read-only and update/patch requests to it are ignored. Possible values are:\n\n* true \n* false"]
        #[serde(
            rename = "customRolesEnabledForSettingsToBeMerged",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_roles_enabled_for_settings_to_be_merged: ::std::option::Option<String>,
        #[doc = "When a message is rejected, this is text for the rejection notification sent to the message's author. By default, this property is empty and has no value in the API's response body. The maximum notification text size is 10,000 characters. Note: Requires sendMessageDenyNotification property to be true."]
        #[serde(
            rename = "defaultMessageDenyNotificationText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_message_deny_notification_text: ::std::option::Option<String>,
        #[doc = "Description of the group. This property value may be an empty string if no group description has been entered. If entered, the maximum group description is no more than 300 characters."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The group's email address. This property can be updated using the Directory API. Note: Only a group owner can change a group's email address. A group manager can't do this.\nWhen you change your group's address using the Directory API or the control panel, you are changing the address your subscribers use to send email and the web address people use to access your group. People can't reach your group by visiting the old address."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Specifies whether a collaborative inbox will remain turned on for the group. Possible values are:\n\n* true \n* false"]
        #[serde(
            rename = "enableCollaborativeInbox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_collaborative_inbox: ::std::option::Option<String>,
        #[doc = "Indicates if favorite replies should be displayed above other replies.\n\n* true: Favorite replies will be displayed above other replies. \n* false: Favorite replies will not be displayed above other replies."]
        #[serde(
            rename = "favoriteRepliesOnTop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub favorite_replies_on_top: ::std::option::Option<String>,
        #[doc = "Whether to include custom footer. Possible values are:\n\n* true \n* false"]
        #[serde(
            rename = "includeCustomFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_custom_footer: ::std::option::Option<String>,
        #[doc = "Enables the group to be included in the Global Address List. For more information, see the help center. Possible values are:\n\n* true: Group is included in the Global Address List. \n* false: Group is not included in the Global Address List."]
        #[serde(
            rename = "includeInGlobalAddressList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_in_global_address_list: ::std::option::Option<String>,
        #[doc = "Allows the Group contents to be archived. Possible values are:\n\n* true: Archive messages sent to the group. \n* false: Do not keep an archive of messages sent to this group. If false, previously archived messages remain in the archive."]
        #[serde(
            rename = "isArchived",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_archived: ::std::option::Option<String>,
        #[doc = "The type of the resource. It is always groupsSettings#groups."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Deprecated. The maximum size of a message is 25Mb."]
        #[serde(
            rename = "maxMessageBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_message_bytes: ::std::option::Option<i32>,
        #[doc = "Enables members to post messages as the group. Possible values are:\n\n* true: Group member can post messages using the group's email address instead of their own email address. Message appear to originate from the group itself. Note: When true, any message moderation settings on individual users or new members do not apply to posts made on behalf of the group. \n* false: Members can not post in behalf of the group's email address."]
        #[serde(
            rename = "membersCanPostAsTheGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members_can_post_as_the_group: ::std::option::Option<String>,
        #[doc = "Deprecated. The default message display font always has a value of \"DEFAULT_FONT\"."]
        #[serde(
            rename = "messageDisplayFont",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_display_font: ::std::option::Option<String>,
        #[doc = "Moderation level of incoming messages. Possible values are:\n\n* MODERATE_ALL_MESSAGES: All messages are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n* MODERATE_NON_MEMBERS: All messages from non group members are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n* MODERATE_NEW_MEMBERS: All messages from new members are sent to the group owner's email address for approval. If approved, the message is sent to the group. \n* MODERATE_NONE: No moderator approval is required. Messages are delivered directly to the group. Note: When the whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.\n  When memberCanPostAsTheGroup is true, any message moderation settings on individual users or new members will not apply to posts made on behalf of the group."]
        #[serde(
            rename = "messageModerationLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_moderation_level: ::std::option::Option<String>,
        #[doc = "Name of the group, which has a maximum size of 75 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The primary language for group. For a group's primary language use the language tags from the G Suite languages found at G Suite Email Settings API Email Language Tags."]
        #[serde(
            rename = "primaryLanguage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_language: ::std::option::Option<String>,
        #[doc = "Specifies who should the default reply go to. Possible values are:\n\n* REPLY_TO_CUSTOM: For replies to messages, use the group's custom email address.\n  When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds the custom email address used when replying to a message. If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a value. Otherwise an error is returned.\n\n* REPLY_TO_SENDER: The reply sent to author of message. \n\n* REPLY_TO_LIST: This reply message is sent to the group. \n\n* REPLY_TO_OWNER: The reply is sent to the owner(s) of the group. This does not include the group's managers. \n\n* REPLY_TO_IGNORE: Group users individually decide where the message reply is sent. \n\n* REPLY_TO_MANAGERS: This reply message is sent to the group's managers, which includes all managers and the group owner."]
        #[serde(
            rename = "replyTo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reply_to: ::std::option::Option<String>,
        #[doc = "Allows a member to be notified if the member's message to the group is denied by the group owner. Possible values are:\n\n* true: When a message is rejected, send the deny message notification to the message author.\n  The defaultMessageDenyNotificationText property is dependent on the sendMessageDenyNotification property being true.\n\n* false: When a message is rejected, no notification is sent."]
        #[serde(
            rename = "sendMessageDenyNotification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub send_message_deny_notification: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanDiscoverGroup setting. Allows the group to be visible in the Groups Directory. Possible values are:\n\n* true: All groups in the account are listed in the Groups directory. \n* false: All groups in the account are not listed in the directory."]
        #[serde(
            rename = "showInGroupDirectory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub show_in_group_directory: ::std::option::Option<String>,
        #[doc = "Specifies moderation levels for messages detected as spam. Possible values are:\n\n* ALLOW: Post the message to the group. \n* MODERATE: Send the message to the moderation queue. This is the default. \n* SILENTLY_MODERATE: Send the message to the moderation queue, but do not send notification to moderators. \n* REJECT: Immediately reject the message."]
        #[serde(
            rename = "spamModerationLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spam_moderation_level: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to add members. Possible values are:\n\n* ALL_MEMBERS_CAN_ADD: Managers and members can directly add new members. \n* ALL_MANAGERS_CAN_ADD: Only managers can directly add new members. this includes the group's owner. \n* ALL_OWNERS_CAN_ADD: Only owners can directly add new members. \n* NONE_CAN_ADD: No one can directly add new members."]
        #[serde(
            rename = "whoCanAdd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_add: ::std::option::Option<String>,
        #[doc = "Deprecated. This functionality is no longer supported in the Google Groups UI. The value is always \"NONE\"."]
        #[serde(
            rename = "whoCanAddReferences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_add_references: ::std::option::Option<String>,
        #[doc = "Specifies who can approve members who ask to join groups. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:\n\n* ALL_MEMBERS_CAN_APPROVE \n* ALL_MANAGERS_CAN_APPROVE \n* ALL_OWNERS_CAN_APPROVE \n* NONE_CAN_APPROVE"]
        #[serde(
            rename = "whoCanApproveMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_approve_members: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can approve pending messages in the moderation queue. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanApproveMessages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_approve_messages: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to assign topics in a forum to another user. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanAssignTopics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_assign_topics: ::std::option::Option<String>,
        #[doc = "Specifies who can moderate metadata. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanAssistContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_assist_content: ::std::option::Option<String>,
        #[doc = "Specifies who can deny membership to users. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanBanUsers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_ban_users: ::std::option::Option<String>,
        #[doc = "Permission to contact owner of the group via web UI. Possible values are:\n\n* ALL_IN_DOMAIN_CAN_CONTACT \n* ALL_MANAGERS_CAN_CONTACT \n* ALL_MEMBERS_CAN_CONTACT \n* ANYONE_CAN_CONTACT"]
        #[serde(
            rename = "whoCanContactOwner",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_contact_owner: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete replies to topics. (Authors can always delete their own posts). Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanDeleteAnyPost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_delete_any_post: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete topics. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanDeleteTopics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_delete_topics: ::std::option::Option<String>,
        #[doc = "Specifies the set of users for whom this group is discoverable. Possible values are:\n\n* ANYONE_CAN_DISCOVER \n* ALL_IN_DOMAIN_CAN_DISCOVER \n* ALL_MEMBERS_CAN_DISCOVER"]
        #[serde(
            rename = "whoCanDiscoverGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_discover_group: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to enter free form tags for topics in a forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanEnterFreeFormTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_enter_free_form_tags: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can hide posts by reporting them as abuse. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanHideAbuse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_hide_abuse: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to invite new members. Possible values are:\n\n* ALL_MEMBERS_CAN_INVITE: Managers and members can invite a new member candidate. \n* ALL_MANAGERS_CAN_INVITE: Only managers can invite a new member. This includes the group's owner. \n* ALL_OWNERS_CAN_INVITE: Only owners can invite a new member. \n* NONE_CAN_INVITE: No one can invite a new member candidate."]
        #[serde(
            rename = "whoCanInvite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_invite: ::std::option::Option<String>,
        #[doc = "Permission to join group. Possible values are:\n\n* ANYONE_CAN_JOIN: Anyone in the account domain can join. This includes accounts with multiple domains. \n* ALL_IN_DOMAIN_CAN_JOIN: Any Internet user who is outside your domain can access your Google Groups service and view the list of groups in your Groups directory. Warning: Group owners can add external addresses, outside of the domain to their groups. They can also allow people outside your domain to join their groups. If you later disable this option, any external addresses already added to users' groups remain in those groups. \n* INVITED_CAN_JOIN: Candidates for membership can be invited to join.\n* CAN_REQUEST_TO_JOIN: Non members can request an invitation to join."]
        #[serde(
            rename = "whoCanJoin",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_join: ::std::option::Option<String>,
        #[doc = "Permission to leave the group. Possible values are:\n\n* ALL_MANAGERS_CAN_LEAVE \n* ALL_MEMBERS_CAN_LEAVE \n* NONE_CAN_LEAVE"]
        #[serde(
            rename = "whoCanLeaveGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_leave_group: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can prevent users from posting replies to topics. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanLockTopics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_lock_topics: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can make topics appear at the top of the topic list. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanMakeTopicsSticky",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_make_topics_sticky: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as a duplicate of another topic. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanMarkDuplicate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_mark_duplicate: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark any other user's post as a favorite reply. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanMarkFavoriteReplyOnAnyTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_mark_favorite_reply_on_any_topic: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a post for a topic they started as a favorite reply. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanMarkFavoriteReplyOnOwnTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_mark_favorite_reply_on_own_topic: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as not needing a response. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanMarkNoResponseNeeded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_mark_no_response_needed: ::std::option::Option<String>,
        #[doc = "Specifies who can moderate content. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanModerateContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_moderate_content: ::std::option::Option<String>,
        #[doc = "Specifies who can manage members. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanModerateMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_moderate_members: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateMembers setting. Specifies who can change group members' roles. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanModifyMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_modify_members: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to change tags and categories. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanModifyTagsAndCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_modify_tags_and_categories: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics into the group or forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanMoveTopicsIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_move_topics_in: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics out of the group or forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanMoveTopicsOut",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_move_topics_out: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can post announcements, a special topic type. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanPostAnnouncements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_post_announcements: ::std::option::Option<String>,
        #[doc = "Permissions to post messages. Possible values are:\n\n* NONE_CAN_POST: The group is disabled and archived. No one can post a message to this group.\n* When archiveOnly is false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. \n* If archiveOnly is reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.\n* ALL_MANAGERS_CAN_POST: Managers, including group owners, can post messages. \n* ALL_MEMBERS_CAN_POST: Any group member can post a message. \n* ALL_OWNERS_CAN_POST: Only group owners can post a message. \n* ALL_IN_DOMAIN_CAN_POST: Anyone in the account can post a message.\n* ANYONE_CAN_POST: Any Internet user who outside your account can access your Google Groups service and post a message. Note: When whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam."]
        #[serde(
            rename = "whoCanPostMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_post_message: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to take topics in a forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanTakeTopics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_take_topics: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unassign any topic in a forum. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanUnassignTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_unassign_topic: ::std::option::Option<String>,
        #[doc = "Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unmark any post from a favorite reply. Possible values are:\n\n* ALL_MEMBERS \n* OWNERS_AND_MANAGERS \n* MANAGERS_ONLY \n* OWNERS_ONLY \n* NONE"]
        #[serde(
            rename = "whoCanUnmarkFavoriteReplyOnAnyTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_unmark_favorite_reply_on_any_topic: ::std::option::Option<String>,
        #[doc = "Permissions to view group messages. Possible values are:\n\n* ANYONE_CAN_VIEW: Any Internet user can view the group's messages.\n* ALL_IN_DOMAIN_CAN_VIEW: Anyone in your account can view this group's messages. \n* ALL_MEMBERS_CAN_VIEW: All group members can view the group's messages. \n* ALL_MANAGERS_CAN_VIEW: Any group manager can view this group's messages."]
        #[serde(
            rename = "whoCanViewGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_view_group: ::std::option::Option<String>,
        #[doc = "Permissions to view membership. Possible values are:\n\n* ALL_IN_DOMAIN_CAN_VIEW: Anyone in the account can view the group members list.\n  If a group already has external members, those members can still send email to this group.\n\n* ALL_MEMBERS_CAN_VIEW: The group members can view the group members list. \n\n* ALL_MANAGERS_CAN_VIEW: The group managers can view group members list."]
        #[serde(
            rename = "whoCanViewMembership",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub who_can_view_membership: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Groups {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Groups {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
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
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "atom" => Alt::Atom,
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
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
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
    #[doc = "Actions that can be performed on the groups resource"]
    pub fn groups(&self) -> crate::resources::groups::GroupsActions {
        crate::resources::groups::GroupsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod groups {
        pub mod params {}
        pub struct GroupsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> GroupsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets one resource by id."]
            pub fn get(&self, group_unique_id: impl Into<String>) -> GetRequestBuilder {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            ) -> PatchRequestBuilder {
                PatchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            ) -> UpdateRequestBuilder {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        #[doc = "Created via [GroupsActions::get()](struct.GroupsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            group_unique_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> GetRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Groups, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Groups, crate::Error> {
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [GroupsActions::patch()](struct.GroupsActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> PatchRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Groups, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Groups, crate::Error> {
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
        #[doc = "Created via [GroupsActions::update()](struct.GroupsActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> UpdateRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Groups, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Groups, crate::Error> {
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
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
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

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
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
