pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AndroidConfigPriority {
        #[doc = "Default priority for data messages. Normal priority messages won't open\nnetwork connections on a sleeping device, and their delivery may be\ndelayed to conserve the battery. For less time-sensitive messages, such\nas notifications of new email or other data to sync, choose normal\ndelivery priority."]
        Normal,
        #[doc = "Default priority for notification messages. FCM attempts to deliver high\npriority messages immediately, allowing the FCM service to wake a\nsleeping device when possible and open a network connection to your app\nserver. Apps with instant messaging, chat, or voice call alerts, for\nexample, generally need to open a network connection and make sure FCM\ndelivers the message to the device without delay. Set high priority if\nthe message is time-critical and requires the user's immediate\ninteraction, but beware that setting your messages to high priority\ncontributes more to battery drain compared with normal priority messages."]
        High,
    }
    impl AndroidConfigPriority {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidConfigPriority::Normal => "NORMAL",
                AndroidConfigPriority::High => "HIGH",
            }
        }
    }
    impl ::std::fmt::Display for AndroidConfigPriority {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidConfigPriority {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidConfigPriority {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NORMAL" => AndroidConfigPriority::Normal,
                "HIGH" => AndroidConfigPriority::High,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
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
    pub struct AndroidConfig {
        #[doc = "An identifier of a group of messages that can be collapsed, so that only\nthe last message gets sent when delivery can be resumed. A maximum of 4\ndifferent collapse keys is allowed at any given time."]
        #[serde(rename = "collapseKey", default)]
        pub collapse_key: Option<String>,
        #[doc = "Arbitrary key/value payload. If present, it will override\ngoogle.firebase.fcm.v1.Message.data."]
        #[serde(rename = "data", default)]
        pub data: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Options for features provided by the FCM SDK for Android."]
        #[serde(rename = "fcmOptions", default)]
        pub fcm_options: Option<crate::schemas::AndroidFcmOptions>,
        #[doc = "Notification to send to android devices."]
        #[serde(rename = "notification", default)]
        pub notification: Option<crate::schemas::AndroidNotification>,
        #[doc = "Message priority. Can take \"normal\" and \"high\" values.\nFor more information, see [Setting the priority of a\nmessage](https://goo.gl/GjONJv)."]
        #[serde(rename = "priority", default)]
        pub priority: Option<crate::schemas::AndroidConfigPriority>,
        #[doc = "Package name of the application where the registration token must match in\norder to receive the message."]
        #[serde(rename = "restrictedPackageName", default)]
        pub restricted_package_name: Option<String>,
        #[doc = "How long (in seconds) the message should be kept in FCM storage if the\ndevice is offline. The maximum time to live supported is 4 weeks, and the\ndefault value is 4 weeks if not set. Set it to 0 if want to send the\nmessage immediately.\nIn JSON format, the Duration type is encoded as a string rather than an\nobject, where the string ends in the suffix \"s\" (indicating seconds) and\nis preceded by the number of seconds, with nanoseconds expressed as\nfractional seconds. For example, 3 seconds with 0 nanoseconds should be\nencoded in JSON format as \"3s\", while 3 seconds and 1 nanosecond should\nbe expressed in JSON format as \"3.000000001s\". The ttl will be rounded down\nto the nearest second."]
        #[serde(rename = "ttl", default)]
        pub ttl: Option<String>,
    }
    impl ::field_selector::FieldSelector for AndroidConfig {
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
    pub struct AndroidFcmOptions {
        #[doc = "Label associated with the message's analytics data."]
        #[serde(rename = "analyticsLabel", default)]
        pub analytics_label: Option<String>,
    }
    impl ::field_selector::FieldSelector for AndroidFcmOptions {
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
    pub struct AndroidNotification {
        #[doc = "The notification's body text. If present, it will override\ngoogle.firebase.fcm.v1.Notification.body."]
        #[serde(rename = "body", default)]
        pub body: Option<String>,
        #[doc = "Variable string values to be used in place of the format specifiers in\nbody_loc_key to use to localize the body text to the user's current\nlocalization.\nSee [Formatting and Styling](https://goo.gl/MalYE3) for more information."]
        #[serde(rename = "bodyLocArgs", default)]
        pub body_loc_args: Option<Vec<String>>,
        #[doc = "The key to the body string in the app's string resources to use to localize\nthe body text to the user's current localization.\nSee [String Resources](https://goo.gl/NdFZGI) for more information."]
        #[serde(rename = "bodyLocKey", default)]
        pub body_loc_key: Option<String>,
        #[doc = "The [notification's channel\nid](https://developer.android.com/guide/topics/ui/notifiers/notifications#ManageChannels)\n(new in Android O). The app must create a channel with this channel ID\nbefore any notification with this channel ID is received. If you don't send\nthis channel ID in the request, or if the channel ID provided has not yet\nbeen created by the app, FCM uses the channel ID specified in the app\nmanifest."]
        #[serde(rename = "channelId", default)]
        pub channel_id: Option<String>,
        #[doc = "The action associated with a user click on the notification.\nIf specified, an activity with a matching intent filter is launched when\na user clicks on the notification."]
        #[serde(rename = "clickAction", default)]
        pub click_action: Option<String>,
        #[doc = "The notification's icon color, expressed in #rrggbb format."]
        #[serde(rename = "color", default)]
        pub color: Option<String>,
        #[doc = "The notification's icon.\nSets the notification icon to myicon for drawable resource myicon.\nIf you don't send this key in the request, FCM displays the launcher icon\nspecified in your app manifest."]
        #[serde(rename = "icon", default)]
        pub icon: Option<String>,
        #[doc = "Contains the URL of an image that is going to be displayed in a\nnotification. If present, it will override\ngoogle.firebase.fcm.v1.Notification.image."]
        #[serde(rename = "image", default)]
        pub image: Option<String>,
        #[doc = "The sound to play when the device receives the notification.\nSupports \"default\" or the filename of a sound resource bundled in the app.\nSound files must reside in /res/raw/."]
        #[serde(rename = "sound", default)]
        pub sound: Option<String>,
        #[doc = "Identifier used to replace existing notifications in the notification\ndrawer.\nIf not specified, each request creates a new notification.\nIf specified and a notification with the same tag is already being shown,\nthe new notification replaces the existing one in the notification drawer."]
        #[serde(rename = "tag", default)]
        pub tag: Option<String>,
        #[doc = "The notification's title. If present, it will override\ngoogle.firebase.fcm.v1.Notification.title."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[doc = "Variable string values to be used in place of the format specifiers in\ntitle_loc_key to use to localize the title text to the user's current\nlocalization.\nSee [Formatting and Styling](https://goo.gl/MalYE3) for more information."]
        #[serde(rename = "titleLocArgs", default)]
        pub title_loc_args: Option<Vec<String>>,
        #[doc = "The key to the title string in the app's string resources to use to\nlocalize the title text to the user's current localization.\nSee [String Resources](https://goo.gl/NdFZGI) for more information."]
        #[serde(rename = "titleLocKey", default)]
        pub title_loc_key: Option<String>,
    }
    impl ::field_selector::FieldSelector for AndroidNotification {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ApnsConfig {
        #[doc = "Options for features provided by the FCM SDK for iOS."]
        #[serde(rename = "fcmOptions", default)]
        pub fcm_options: Option<crate::schemas::ApnsFcmOptions>,
        #[doc = "HTTP request headers defined in Apple Push Notification Service. Refer to\n[APNs request headers](https://goo.gl/C6Yhia) for\nsupported headers, e.g. \"apns-priority\": \"10\"."]
        #[serde(rename = "headers", default)]
        pub headers: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "APNs payload as a JSON object, including both `aps` dictionary and custom\npayload. See [Payload Key Reference](https://goo.gl/32Pl5W).\nIf present, it overrides google.firebase.fcm.v1.Notification.title\nand google.firebase.fcm.v1.Notification.body."]
        #[serde(rename = "payload", default)]
        pub payload: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for ApnsConfig {
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
    pub struct ApnsFcmOptions {
        #[doc = "Label associated with the message's analytics data."]
        #[serde(rename = "analyticsLabel", default)]
        pub analytics_label: Option<String>,
        #[doc = "Contains the URL of an image that is going to be displayed in a\nnotification. If present, it will override\ngoogle.firebase.fcm.v1.Notification.image."]
        #[serde(rename = "image", default)]
        pub image: Option<String>,
    }
    impl ::field_selector::FieldSelector for ApnsFcmOptions {
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
    pub struct FcmOptions {
        #[doc = "Label associated with the message's analytics data."]
        #[serde(rename = "analyticsLabel", default)]
        pub analytics_label: Option<String>,
    }
    impl ::field_selector::FieldSelector for FcmOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Message {
        #[doc = "Input only. Android specific options for messages sent through\n[FCM connection server](https://goo.gl/4GLdUl)."]
        #[serde(rename = "android", default)]
        pub android: Option<crate::schemas::AndroidConfig>,
        #[doc = "Input only. [Apple Push Notification Service](https://goo.gl/MXRTPa)\nspecific options."]
        #[serde(rename = "apns", default)]
        pub apns: Option<crate::schemas::ApnsConfig>,
        #[doc = "Condition to send a message to,\ne.g. \"'foo' in topics && 'bar' in topics\"."]
        #[serde(rename = "condition", default)]
        pub condition: Option<String>,
        #[doc = "Input only. Arbitrary key/value payload."]
        #[serde(rename = "data", default)]
        pub data: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Input only. Template for FCM SDK feature options to use across all\nplatforms."]
        #[serde(rename = "fcmOptions", default)]
        pub fcm_options: Option<crate::schemas::FcmOptions>,
        #[doc = "Output Only. The identifier of the message sent, in the format of\n`projects/*/messages/{message_id}`."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Input only. Basic notification template to use across all platforms."]
        #[serde(rename = "notification", default)]
        pub notification: Option<crate::schemas::Notification>,
        #[doc = "Registration token to send a message to."]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
        #[doc = "Topic name to send a message to, e.g. \"weather\".\nNote: \"/topics/\" prefix should not be provided."]
        #[serde(rename = "topic", default)]
        pub topic: Option<String>,
        #[doc = "Input only. [Webpush protocol](https://tools.ietf.org/html/rfc8030)\noptions."]
        #[serde(rename = "webpush", default)]
        pub webpush: Option<crate::schemas::WebpushConfig>,
    }
    impl ::field_selector::FieldSelector for Message {
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
    pub struct Notification {
        #[doc = "The notification's body text."]
        #[serde(rename = "body", default)]
        pub body: Option<String>,
        #[doc = "Contains the URL of an image that is going to be downloaded on the device\nand displayed in a notification.\nJPEG, PNG, BMP have full support across platforms. Animated GIF and video\nonly work on iOS. WebP and HEIF have varying levels of support across\nplatforms and platform versions.\nAndroid has 1MB image size limit.\nQuota usage and implications/costs for hosting image on Firebase Storage:\nhttps://firebase.google.com/pricing"]
        #[serde(rename = "image", default)]
        pub image: Option<String>,
        #[doc = "The notification's title."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Notification {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SendMessageRequest {
        #[doc = "Required. Message to send."]
        #[serde(rename = "message", default)]
        pub message: Option<crate::schemas::Message>,
        #[doc = "Flag for testing the request without actually delivering the message."]
        #[serde(rename = "validateOnly", default)]
        pub validate_only: Option<bool>,
    }
    impl ::field_selector::FieldSelector for SendMessageRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WebpushConfig {
        #[doc = "Arbitrary key/value payload. If present, it will override\ngoogle.firebase.fcm.v1.Message.data."]
        #[serde(rename = "data", default)]
        pub data: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Options for features provided by the FCM SDK for Web."]
        #[serde(rename = "fcmOptions", default)]
        pub fcm_options: Option<crate::schemas::WebpushFcmOptions>,
        #[doc = "HTTP headers defined in webpush protocol. Refer to\n[Webpush protocol](https://tools.ietf.org/html/rfc8030#section-5) for\nsupported headers, e.g. \"TTL\": \"15\"."]
        #[serde(rename = "headers", default)]
        pub headers: Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Web Notification options as a JSON object. Supports Notification instance\nproperties as defined in [Web Notification\nAPI](https://developer.mozilla.org/en-US/docs/Web/API/Notification). If\npresent, \"title\" and \"body\" fields override\n[google.firebase.fcm.v1.Notification.title] and\n[google.firebase.fcm.v1.Notification.body]."]
        #[serde(rename = "notification", default)]
        pub notification: Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for WebpushConfig {
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
    pub struct WebpushFcmOptions {
        #[doc = "The link to open when the user clicks on the notification.\nFor all URL values, HTTPS is required."]
        #[serde(rename = "link", default)]
        pub link: Option<String>,
    }
    impl ::field_selector::FieldSelector for WebpushFcmOptions {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::projects::ProjectsActions<A> {
        crate::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod projects {
    pub mod params {}
    pub struct ProjectsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
        #[doc = "Actions that can be performed on the messages resource"]
        pub fn messages(&self) -> messages::MessagesActions<A> {
            messages::MessagesActions
        }
    }
    pub mod messages {
        pub mod params {}
        pub struct MessagesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> MessagesActions<'a, A> {
            #[doc = "Send a message to specified target (a registration token, topic\nor condition)."]
            pub fn send(
                &self,
                request: crate::schemas::SendMessageRequest,
                parent: impl Into<String>,
            ) -> SendRequestBuilder<A> {
                SendRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct SendRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::SendMessageRequest,
            parent: String,
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
        impl<'a, A: yup_oauth2::GetToken> SendRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::Message, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://fcm.googleapis.com/".to_owned();
                output.push_str("v1/");
                output.push_str(&self.parent);
                output.push_str("/messages:send");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
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
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/cloud-platform"])
                        .unwrap()
                        .access_token,
                );
                req
            }
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
