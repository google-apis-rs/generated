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
    pub struct Account {
        #[doc = "The Account ID uniquely identifies the GTM Account."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "Account display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "GTM Account's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Whether the account shares data anonymously with Google and others. This flag enables benchmarking by sharing your data in an anonymous form. Google will remove all identifiable information about your website, combine the data with hundreds of other anonymous sites and report aggregate trends in the benchmarking service."]
        #[serde(rename = "shareData", default)]
        pub share_data: Option<bool>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for Account {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AccountAccessPermission {
        AccountPermissionUnspecified,
        Admin,
        NoAccess,
        User,
    }
    impl AccountAccessPermission {
        pub fn as_str(self) -> &'static str {
            match self {
                AccountAccessPermission::AccountPermissionUnspecified => {
                    "accountPermissionUnspecified"
                }
                AccountAccessPermission::Admin => "admin",
                AccountAccessPermission::NoAccess => "noAccess",
                AccountAccessPermission::User => "user",
            }
        }
    }
    impl ::std::fmt::Display for AccountAccessPermission {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AccountAccessPermission {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AccountAccessPermission {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "accountPermissionUnspecified" => {
                    AccountAccessPermission::AccountPermissionUnspecified
                }
                "admin" => AccountAccessPermission::Admin,
                "noAccess" => AccountAccessPermission::NoAccess,
                "user" => AccountAccessPermission::User,
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
    pub struct AccountAccess {
        #[doc = "Whether the user has no access, user access, or admin access to an account."]
        #[serde(rename = "permission", default)]
        pub permission: Option<crate::schemas::AccountAccessPermission>,
    }
    impl ::field_selector::FieldSelector for AccountAccess {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BuiltInVariableType {
        AdvertiserId,
        AdvertisingTrackingEnabled,
        AmpBrowserLanguage,
        AmpCanonicalHost,
        AmpCanonicalPath,
        AmpCanonicalUrl,
        AmpClientId,
        AmpClientMaxScrollX,
        AmpClientMaxScrollY,
        AmpClientScreenHeight,
        AmpClientScreenWidth,
        AmpClientScrollX,
        AmpClientScrollY,
        AmpClientTimestamp,
        AmpClientTimezone,
        AmpGtmEvent,
        AmpPageDownloadTime,
        AmpPageLoadTime,
        AmpPageViewId,
        AmpReferrer,
        AmpTitle,
        AmpTotalEngagedTime,
        AppId,
        AppName,
        AppVersionCode,
        AppVersionName,
        BuiltInVariableTypeUnspecified,
        ClickClasses,
        ClickElement,
        ClickId,
        ClickTarget,
        ClickText,
        ClickUrl,
        ContainerId,
        ContainerVersion,
        DebugMode,
        DeviceName,
        ElementVisibilityFirstTime,
        ElementVisibilityRatio,
        ElementVisibilityRecentTime,
        ElementVisibilityTime,
        EnvironmentName,
        ErrorLine,
        ErrorMessage,
        ErrorUrl,
        Event,
        EventName,
        FirebaseEventParameterCampaign,
        FirebaseEventParameterCampaignAclid,
        FirebaseEventParameterCampaignAnid,
        FirebaseEventParameterCampaignClickTimestamp,
        FirebaseEventParameterCampaignContent,
        FirebaseEventParameterCampaignCp1,
        FirebaseEventParameterCampaignGclid,
        FirebaseEventParameterCampaignSource,
        FirebaseEventParameterCampaignTerm,
        FirebaseEventParameterCurrency,
        FirebaseEventParameterDynamicLinkAcceptTime,
        FirebaseEventParameterDynamicLinkLinkid,
        FirebaseEventParameterNotificationMessageDeviceTime,
        FirebaseEventParameterNotificationMessageId,
        FirebaseEventParameterNotificationMessageName,
        FirebaseEventParameterNotificationMessageTime,
        FirebaseEventParameterNotificationTopic,
        FirebaseEventParameterPreviousAppVersion,
        FirebaseEventParameterPreviousOsVersion,
        FirebaseEventParameterPrice,
        FirebaseEventParameterProductId,
        FirebaseEventParameterQuantity,
        FirebaseEventParameterValue,
        FormClasses,
        FormElement,
        FormId,
        FormTarget,
        FormText,
        FormUrl,
        HistorySource,
        HtmlId,
        Language,
        NewHistoryFragment,
        NewHistoryState,
        NewHistoryUrl,
        OldHistoryFragment,
        OldHistoryState,
        OldHistoryUrl,
        OsVersion,
        PageHostname,
        PagePath,
        PageUrl,
        Platform,
        RandomNumber,
        Referrer,
        Resolution,
        ScrollDepthDirection,
        ScrollDepthThreshold,
        ScrollDepthUnits,
        SdkVersion,
        VideoCurrentTime,
        VideoDuration,
        VideoPercent,
        VideoProvider,
        VideoStatus,
        VideoTitle,
        VideoUrl,
        VideoVisible,
    }
    impl BuiltInVariableType {
        pub fn as_str(self) -> &'static str {
            match self {
                BuiltInVariableType::AdvertiserId => "advertiserId",
                BuiltInVariableType::AdvertisingTrackingEnabled => "advertisingTrackingEnabled",
                BuiltInVariableType::AmpBrowserLanguage => "ampBrowserLanguage",
                BuiltInVariableType::AmpCanonicalHost => "ampCanonicalHost",
                BuiltInVariableType::AmpCanonicalPath => "ampCanonicalPath",
                BuiltInVariableType::AmpCanonicalUrl => "ampCanonicalUrl",
                BuiltInVariableType::AmpClientId => "ampClientId",
                BuiltInVariableType::AmpClientMaxScrollX => "ampClientMaxScrollX",
                BuiltInVariableType::AmpClientMaxScrollY => "ampClientMaxScrollY",
                BuiltInVariableType::AmpClientScreenHeight => "ampClientScreenHeight",
                BuiltInVariableType::AmpClientScreenWidth => "ampClientScreenWidth",
                BuiltInVariableType::AmpClientScrollX => "ampClientScrollX",
                BuiltInVariableType::AmpClientScrollY => "ampClientScrollY",
                BuiltInVariableType::AmpClientTimestamp => "ampClientTimestamp",
                BuiltInVariableType::AmpClientTimezone => "ampClientTimezone",
                BuiltInVariableType::AmpGtmEvent => "ampGtmEvent",
                BuiltInVariableType::AmpPageDownloadTime => "ampPageDownloadTime",
                BuiltInVariableType::AmpPageLoadTime => "ampPageLoadTime",
                BuiltInVariableType::AmpPageViewId => "ampPageViewId",
                BuiltInVariableType::AmpReferrer => "ampReferrer",
                BuiltInVariableType::AmpTitle => "ampTitle",
                BuiltInVariableType::AmpTotalEngagedTime => "ampTotalEngagedTime",
                BuiltInVariableType::AppId => "appId",
                BuiltInVariableType::AppName => "appName",
                BuiltInVariableType::AppVersionCode => "appVersionCode",
                BuiltInVariableType::AppVersionName => "appVersionName",
                BuiltInVariableType::BuiltInVariableTypeUnspecified => {
                    "builtInVariableTypeUnspecified"
                }
                BuiltInVariableType::ClickClasses => "clickClasses",
                BuiltInVariableType::ClickElement => "clickElement",
                BuiltInVariableType::ClickId => "clickId",
                BuiltInVariableType::ClickTarget => "clickTarget",
                BuiltInVariableType::ClickText => "clickText",
                BuiltInVariableType::ClickUrl => "clickUrl",
                BuiltInVariableType::ContainerId => "containerId",
                BuiltInVariableType::ContainerVersion => "containerVersion",
                BuiltInVariableType::DebugMode => "debugMode",
                BuiltInVariableType::DeviceName => "deviceName",
                BuiltInVariableType::ElementVisibilityFirstTime => "elementVisibilityFirstTime",
                BuiltInVariableType::ElementVisibilityRatio => "elementVisibilityRatio",
                BuiltInVariableType::ElementVisibilityRecentTime => "elementVisibilityRecentTime",
                BuiltInVariableType::ElementVisibilityTime => "elementVisibilityTime",
                BuiltInVariableType::EnvironmentName => "environmentName",
                BuiltInVariableType::ErrorLine => "errorLine",
                BuiltInVariableType::ErrorMessage => "errorMessage",
                BuiltInVariableType::ErrorUrl => "errorUrl",
                BuiltInVariableType::Event => "event",
                BuiltInVariableType::EventName => "eventName",
                BuiltInVariableType::FirebaseEventParameterCampaign => {
                    "firebaseEventParameterCampaign"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignAclid => {
                    "firebaseEventParameterCampaignAclid"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignAnid => {
                    "firebaseEventParameterCampaignAnid"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignClickTimestamp => {
                    "firebaseEventParameterCampaignClickTimestamp"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignContent => {
                    "firebaseEventParameterCampaignContent"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignCp1 => {
                    "firebaseEventParameterCampaignCp1"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignGclid => {
                    "firebaseEventParameterCampaignGclid"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignSource => {
                    "firebaseEventParameterCampaignSource"
                }
                BuiltInVariableType::FirebaseEventParameterCampaignTerm => {
                    "firebaseEventParameterCampaignTerm"
                }
                BuiltInVariableType::FirebaseEventParameterCurrency => {
                    "firebaseEventParameterCurrency"
                }
                BuiltInVariableType::FirebaseEventParameterDynamicLinkAcceptTime => {
                    "firebaseEventParameterDynamicLinkAcceptTime"
                }
                BuiltInVariableType::FirebaseEventParameterDynamicLinkLinkid => {
                    "firebaseEventParameterDynamicLinkLinkid"
                }
                BuiltInVariableType::FirebaseEventParameterNotificationMessageDeviceTime => {
                    "firebaseEventParameterNotificationMessageDeviceTime"
                }
                BuiltInVariableType::FirebaseEventParameterNotificationMessageId => {
                    "firebaseEventParameterNotificationMessageId"
                }
                BuiltInVariableType::FirebaseEventParameterNotificationMessageName => {
                    "firebaseEventParameterNotificationMessageName"
                }
                BuiltInVariableType::FirebaseEventParameterNotificationMessageTime => {
                    "firebaseEventParameterNotificationMessageTime"
                }
                BuiltInVariableType::FirebaseEventParameterNotificationTopic => {
                    "firebaseEventParameterNotificationTopic"
                }
                BuiltInVariableType::FirebaseEventParameterPreviousAppVersion => {
                    "firebaseEventParameterPreviousAppVersion"
                }
                BuiltInVariableType::FirebaseEventParameterPreviousOsVersion => {
                    "firebaseEventParameterPreviousOsVersion"
                }
                BuiltInVariableType::FirebaseEventParameterPrice => "firebaseEventParameterPrice",
                BuiltInVariableType::FirebaseEventParameterProductId => {
                    "firebaseEventParameterProductId"
                }
                BuiltInVariableType::FirebaseEventParameterQuantity => {
                    "firebaseEventParameterQuantity"
                }
                BuiltInVariableType::FirebaseEventParameterValue => "firebaseEventParameterValue",
                BuiltInVariableType::FormClasses => "formClasses",
                BuiltInVariableType::FormElement => "formElement",
                BuiltInVariableType::FormId => "formId",
                BuiltInVariableType::FormTarget => "formTarget",
                BuiltInVariableType::FormText => "formText",
                BuiltInVariableType::FormUrl => "formUrl",
                BuiltInVariableType::HistorySource => "historySource",
                BuiltInVariableType::HtmlId => "htmlId",
                BuiltInVariableType::Language => "language",
                BuiltInVariableType::NewHistoryFragment => "newHistoryFragment",
                BuiltInVariableType::NewHistoryState => "newHistoryState",
                BuiltInVariableType::NewHistoryUrl => "newHistoryUrl",
                BuiltInVariableType::OldHistoryFragment => "oldHistoryFragment",
                BuiltInVariableType::OldHistoryState => "oldHistoryState",
                BuiltInVariableType::OldHistoryUrl => "oldHistoryUrl",
                BuiltInVariableType::OsVersion => "osVersion",
                BuiltInVariableType::PageHostname => "pageHostname",
                BuiltInVariableType::PagePath => "pagePath",
                BuiltInVariableType::PageUrl => "pageUrl",
                BuiltInVariableType::Platform => "platform",
                BuiltInVariableType::RandomNumber => "randomNumber",
                BuiltInVariableType::Referrer => "referrer",
                BuiltInVariableType::Resolution => "resolution",
                BuiltInVariableType::ScrollDepthDirection => "scrollDepthDirection",
                BuiltInVariableType::ScrollDepthThreshold => "scrollDepthThreshold",
                BuiltInVariableType::ScrollDepthUnits => "scrollDepthUnits",
                BuiltInVariableType::SdkVersion => "sdkVersion",
                BuiltInVariableType::VideoCurrentTime => "videoCurrentTime",
                BuiltInVariableType::VideoDuration => "videoDuration",
                BuiltInVariableType::VideoPercent => "videoPercent",
                BuiltInVariableType::VideoProvider => "videoProvider",
                BuiltInVariableType::VideoStatus => "videoStatus",
                BuiltInVariableType::VideoTitle => "videoTitle",
                BuiltInVariableType::VideoUrl => "videoUrl",
                BuiltInVariableType::VideoVisible => "videoVisible",
            }
        }
    }
    impl ::std::fmt::Display for BuiltInVariableType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuiltInVariableType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuiltInVariableType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "advertiserId" => BuiltInVariableType::AdvertiserId,
                "advertisingTrackingEnabled" => BuiltInVariableType::AdvertisingTrackingEnabled,
                "ampBrowserLanguage" => BuiltInVariableType::AmpBrowserLanguage,
                "ampCanonicalHost" => BuiltInVariableType::AmpCanonicalHost,
                "ampCanonicalPath" => BuiltInVariableType::AmpCanonicalPath,
                "ampCanonicalUrl" => BuiltInVariableType::AmpCanonicalUrl,
                "ampClientId" => BuiltInVariableType::AmpClientId,
                "ampClientMaxScrollX" => BuiltInVariableType::AmpClientMaxScrollX,
                "ampClientMaxScrollY" => BuiltInVariableType::AmpClientMaxScrollY,
                "ampClientScreenHeight" => BuiltInVariableType::AmpClientScreenHeight,
                "ampClientScreenWidth" => BuiltInVariableType::AmpClientScreenWidth,
                "ampClientScrollX" => BuiltInVariableType::AmpClientScrollX,
                "ampClientScrollY" => BuiltInVariableType::AmpClientScrollY,
                "ampClientTimestamp" => BuiltInVariableType::AmpClientTimestamp,
                "ampClientTimezone" => BuiltInVariableType::AmpClientTimezone,
                "ampGtmEvent" => BuiltInVariableType::AmpGtmEvent,
                "ampPageDownloadTime" => BuiltInVariableType::AmpPageDownloadTime,
                "ampPageLoadTime" => BuiltInVariableType::AmpPageLoadTime,
                "ampPageViewId" => BuiltInVariableType::AmpPageViewId,
                "ampReferrer" => BuiltInVariableType::AmpReferrer,
                "ampTitle" => BuiltInVariableType::AmpTitle,
                "ampTotalEngagedTime" => BuiltInVariableType::AmpTotalEngagedTime,
                "appId" => BuiltInVariableType::AppId,
                "appName" => BuiltInVariableType::AppName,
                "appVersionCode" => BuiltInVariableType::AppVersionCode,
                "appVersionName" => BuiltInVariableType::AppVersionName,
                "builtInVariableTypeUnspecified" => {
                    BuiltInVariableType::BuiltInVariableTypeUnspecified
                }
                "clickClasses" => BuiltInVariableType::ClickClasses,
                "clickElement" => BuiltInVariableType::ClickElement,
                "clickId" => BuiltInVariableType::ClickId,
                "clickTarget" => BuiltInVariableType::ClickTarget,
                "clickText" => BuiltInVariableType::ClickText,
                "clickUrl" => BuiltInVariableType::ClickUrl,
                "containerId" => BuiltInVariableType::ContainerId,
                "containerVersion" => BuiltInVariableType::ContainerVersion,
                "debugMode" => BuiltInVariableType::DebugMode,
                "deviceName" => BuiltInVariableType::DeviceName,
                "elementVisibilityFirstTime" => BuiltInVariableType::ElementVisibilityFirstTime,
                "elementVisibilityRatio" => BuiltInVariableType::ElementVisibilityRatio,
                "elementVisibilityRecentTime" => BuiltInVariableType::ElementVisibilityRecentTime,
                "elementVisibilityTime" => BuiltInVariableType::ElementVisibilityTime,
                "environmentName" => BuiltInVariableType::EnvironmentName,
                "errorLine" => BuiltInVariableType::ErrorLine,
                "errorMessage" => BuiltInVariableType::ErrorMessage,
                "errorUrl" => BuiltInVariableType::ErrorUrl,
                "event" => BuiltInVariableType::Event,
                "eventName" => BuiltInVariableType::EventName,
                "firebaseEventParameterCampaign" => {
                    BuiltInVariableType::FirebaseEventParameterCampaign
                }
                "firebaseEventParameterCampaignAclid" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignAclid
                }
                "firebaseEventParameterCampaignAnid" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignAnid
                }
                "firebaseEventParameterCampaignClickTimestamp" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignClickTimestamp
                }
                "firebaseEventParameterCampaignContent" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignContent
                }
                "firebaseEventParameterCampaignCp1" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignCp1
                }
                "firebaseEventParameterCampaignGclid" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignGclid
                }
                "firebaseEventParameterCampaignSource" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignSource
                }
                "firebaseEventParameterCampaignTerm" => {
                    BuiltInVariableType::FirebaseEventParameterCampaignTerm
                }
                "firebaseEventParameterCurrency" => {
                    BuiltInVariableType::FirebaseEventParameterCurrency
                }
                "firebaseEventParameterDynamicLinkAcceptTime" => {
                    BuiltInVariableType::FirebaseEventParameterDynamicLinkAcceptTime
                }
                "firebaseEventParameterDynamicLinkLinkid" => {
                    BuiltInVariableType::FirebaseEventParameterDynamicLinkLinkid
                }
                "firebaseEventParameterNotificationMessageDeviceTime" => {
                    BuiltInVariableType::FirebaseEventParameterNotificationMessageDeviceTime
                }
                "firebaseEventParameterNotificationMessageId" => {
                    BuiltInVariableType::FirebaseEventParameterNotificationMessageId
                }
                "firebaseEventParameterNotificationMessageName" => {
                    BuiltInVariableType::FirebaseEventParameterNotificationMessageName
                }
                "firebaseEventParameterNotificationMessageTime" => {
                    BuiltInVariableType::FirebaseEventParameterNotificationMessageTime
                }
                "firebaseEventParameterNotificationTopic" => {
                    BuiltInVariableType::FirebaseEventParameterNotificationTopic
                }
                "firebaseEventParameterPreviousAppVersion" => {
                    BuiltInVariableType::FirebaseEventParameterPreviousAppVersion
                }
                "firebaseEventParameterPreviousOsVersion" => {
                    BuiltInVariableType::FirebaseEventParameterPreviousOsVersion
                }
                "firebaseEventParameterPrice" => BuiltInVariableType::FirebaseEventParameterPrice,
                "firebaseEventParameterProductId" => {
                    BuiltInVariableType::FirebaseEventParameterProductId
                }
                "firebaseEventParameterQuantity" => {
                    BuiltInVariableType::FirebaseEventParameterQuantity
                }
                "firebaseEventParameterValue" => BuiltInVariableType::FirebaseEventParameterValue,
                "formClasses" => BuiltInVariableType::FormClasses,
                "formElement" => BuiltInVariableType::FormElement,
                "formId" => BuiltInVariableType::FormId,
                "formTarget" => BuiltInVariableType::FormTarget,
                "formText" => BuiltInVariableType::FormText,
                "formUrl" => BuiltInVariableType::FormUrl,
                "historySource" => BuiltInVariableType::HistorySource,
                "htmlId" => BuiltInVariableType::HtmlId,
                "language" => BuiltInVariableType::Language,
                "newHistoryFragment" => BuiltInVariableType::NewHistoryFragment,
                "newHistoryState" => BuiltInVariableType::NewHistoryState,
                "newHistoryUrl" => BuiltInVariableType::NewHistoryUrl,
                "oldHistoryFragment" => BuiltInVariableType::OldHistoryFragment,
                "oldHistoryState" => BuiltInVariableType::OldHistoryState,
                "oldHistoryUrl" => BuiltInVariableType::OldHistoryUrl,
                "osVersion" => BuiltInVariableType::OsVersion,
                "pageHostname" => BuiltInVariableType::PageHostname,
                "pagePath" => BuiltInVariableType::PagePath,
                "pageUrl" => BuiltInVariableType::PageUrl,
                "platform" => BuiltInVariableType::Platform,
                "randomNumber" => BuiltInVariableType::RandomNumber,
                "referrer" => BuiltInVariableType::Referrer,
                "resolution" => BuiltInVariableType::Resolution,
                "scrollDepthDirection" => BuiltInVariableType::ScrollDepthDirection,
                "scrollDepthThreshold" => BuiltInVariableType::ScrollDepthThreshold,
                "scrollDepthUnits" => BuiltInVariableType::ScrollDepthUnits,
                "sdkVersion" => BuiltInVariableType::SdkVersion,
                "videoCurrentTime" => BuiltInVariableType::VideoCurrentTime,
                "videoDuration" => BuiltInVariableType::VideoDuration,
                "videoPercent" => BuiltInVariableType::VideoPercent,
                "videoProvider" => BuiltInVariableType::VideoProvider,
                "videoStatus" => BuiltInVariableType::VideoStatus,
                "videoTitle" => BuiltInVariableType::VideoTitle,
                "videoUrl" => BuiltInVariableType::VideoUrl,
                "videoVisible" => BuiltInVariableType::VideoVisible,
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
    pub struct BuiltInVariable {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "Name of the built-in variable to be used to refer to the built-in variable."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "GTM BuiltInVariable's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Type of built-in variable."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::BuiltInVariableType>,
        #[doc = "GTM Workspace ID."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for BuiltInVariable {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ConditionType {
        ConditionTypeUnspecified,
        Contains,
        CssSelector,
        EndsWith,
        Equals,
        Greater,
        GreaterOrEquals,
        Less,
        LessOrEquals,
        MatchRegex,
        StartsWith,
        UrlMatches,
    }
    impl ConditionType {
        pub fn as_str(self) -> &'static str {
            match self {
                ConditionType::ConditionTypeUnspecified => "conditionTypeUnspecified",
                ConditionType::Contains => "contains",
                ConditionType::CssSelector => "cssSelector",
                ConditionType::EndsWith => "endsWith",
                ConditionType::Equals => "equals",
                ConditionType::Greater => "greater",
                ConditionType::GreaterOrEquals => "greaterOrEquals",
                ConditionType::Less => "less",
                ConditionType::LessOrEquals => "lessOrEquals",
                ConditionType::MatchRegex => "matchRegex",
                ConditionType::StartsWith => "startsWith",
                ConditionType::UrlMatches => "urlMatches",
            }
        }
    }
    impl ::std::fmt::Display for ConditionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ConditionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ConditionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "conditionTypeUnspecified" => ConditionType::ConditionTypeUnspecified,
                "contains" => ConditionType::Contains,
                "cssSelector" => ConditionType::CssSelector,
                "endsWith" => ConditionType::EndsWith,
                "equals" => ConditionType::Equals,
                "greater" => ConditionType::Greater,
                "greaterOrEquals" => ConditionType::GreaterOrEquals,
                "less" => ConditionType::Less,
                "lessOrEquals" => ConditionType::LessOrEquals,
                "matchRegex" => ConditionType::MatchRegex,
                "startsWith" => ConditionType::StartsWith,
                "urlMatches" => ConditionType::UrlMatches,
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
    pub struct Condition {
        #[doc = "A list of named parameters (key/value), depending on the condition's type. Notes: \n- For binary operators, include parameters named arg0 and arg1 for specifying the left and right operands, respectively. \n- At this time, the left operand (arg0) must be a reference to a variable. \n- For case-insensitive Regex matching, include a boolean parameter named ignore_case that is set to true. If not specified or set to any other value, the matching will be case sensitive. \n- To negate an operator, include a boolean parameter named negate boolean parameter that is set to true."]
        #[serde(rename = "parameter", default)]
        pub parameter: Option<Vec<crate::schemas::Parameter>>,
        #[doc = "The type of operator for this condition."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::ConditionType>,
    }
    impl ::field_selector::FieldSelector for Condition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContainerUsageContextItems {
        Amp,
        Android,
        AndroidSdk5,
        Ios,
        IosSdk5,
        UsageContextUnspecified,
        Web,
    }
    impl ContainerUsageContextItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ContainerUsageContextItems::Amp => "amp",
                ContainerUsageContextItems::Android => "android",
                ContainerUsageContextItems::AndroidSdk5 => "androidSdk5",
                ContainerUsageContextItems::Ios => "ios",
                ContainerUsageContextItems::IosSdk5 => "iosSdk5",
                ContainerUsageContextItems::UsageContextUnspecified => "usageContextUnspecified",
                ContainerUsageContextItems::Web => "web",
            }
        }
    }
    impl ::std::fmt::Display for ContainerUsageContextItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContainerUsageContextItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContainerUsageContextItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "amp" => ContainerUsageContextItems::Amp,
                "android" => ContainerUsageContextItems::Android,
                "androidSdk5" => ContainerUsageContextItems::AndroidSdk5,
                "ios" => ContainerUsageContextItems::Ios,
                "iosSdk5" => ContainerUsageContextItems::IosSdk5,
                "usageContextUnspecified" => ContainerUsageContextItems::UsageContextUnspecified,
                "web" => ContainerUsageContextItems::Web,
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
    pub struct Container {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "The Container ID uniquely identifies the GTM Container."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "List of domain names associated with the Container."]
        #[serde(rename = "domainName", default)]
        pub domain_name: Option<Vec<String>>,
        #[doc = "The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "Container display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Container Notes."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
        #[doc = "GTM Container's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Container Public ID."]
        #[serde(rename = "publicId", default)]
        pub public_id: Option<String>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "List of Usage Contexts for the Container. Valid values include: web, android, or ios."]
        #[serde(rename = "usageContext", default)]
        pub usage_context: Option<Vec<crate::schemas::ContainerUsageContextItems>>,
    }
    impl ::field_selector::FieldSelector for Container {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContainerAccessPermission {
        Approve,
        ContainerPermissionUnspecified,
        Edit,
        NoAccess,
        Publish,
        Read,
    }
    impl ContainerAccessPermission {
        pub fn as_str(self) -> &'static str {
            match self {
                ContainerAccessPermission::Approve => "approve",
                ContainerAccessPermission::ContainerPermissionUnspecified => {
                    "containerPermissionUnspecified"
                }
                ContainerAccessPermission::Edit => "edit",
                ContainerAccessPermission::NoAccess => "noAccess",
                ContainerAccessPermission::Publish => "publish",
                ContainerAccessPermission::Read => "read",
            }
        }
    }
    impl ::std::fmt::Display for ContainerAccessPermission {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContainerAccessPermission {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContainerAccessPermission {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "approve" => ContainerAccessPermission::Approve,
                "containerPermissionUnspecified" => {
                    ContainerAccessPermission::ContainerPermissionUnspecified
                }
                "edit" => ContainerAccessPermission::Edit,
                "noAccess" => ContainerAccessPermission::NoAccess,
                "publish" => ContainerAccessPermission::Publish,
                "read" => ContainerAccessPermission::Read,
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
    pub struct ContainerAccess {
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "List of Container permissions."]
        #[serde(rename = "permission", default)]
        pub permission: Option<crate::schemas::ContainerAccessPermission>,
    }
    impl ::field_selector::FieldSelector for ContainerAccess {
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
    pub struct ContainerVersion {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "The built-in variables in the container that this version was taken from."]
        #[serde(rename = "builtInVariable", default)]
        pub built_in_variable: Option<Vec<crate::schemas::BuiltInVariable>>,
        #[doc = "The container that this version was taken from."]
        #[serde(rename = "container", default)]
        pub container: Option<crate::schemas::Container>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "The Container Version ID uniquely identifies the GTM Container Version."]
        #[serde(rename = "containerVersionId", default)]
        pub container_version_id: Option<String>,
        #[doc = "The custom templates in the container that this version was taken from."]
        #[serde(rename = "customTemplate", default)]
        pub custom_template: Option<Vec<crate::schemas::CustomTemplate>>,
        #[doc = "A value of true indicates this container version has been deleted."]
        #[serde(rename = "deleted", default)]
        pub deleted: Option<bool>,
        #[doc = "Container version description."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "The folders in the container that this version was taken from."]
        #[serde(rename = "folder", default)]
        pub folder: Option<Vec<crate::schemas::Folder>>,
        #[doc = "Container version display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "GTM ContainerVersions's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "The tags in the container that this version was taken from."]
        #[serde(rename = "tag", default)]
        pub tag: Option<Vec<crate::schemas::Tag>>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "The triggers in the container that this version was taken from."]
        #[serde(rename = "trigger", default)]
        pub trigger: Option<Vec<crate::schemas::Trigger>>,
        #[doc = "The variables in the container that this version was taken from."]
        #[serde(rename = "variable", default)]
        pub variable: Option<Vec<crate::schemas::Variable>>,
        #[doc = "The zones in the container that this version was taken from."]
        #[serde(rename = "zone", default)]
        pub zone: Option<Vec<crate::schemas::Zone>>,
    }
    impl ::field_selector::FieldSelector for ContainerVersion {
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
    pub struct ContainerVersionHeader {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "The Container Version ID uniquely identifies the GTM Container Version."]
        #[serde(rename = "containerVersionId", default)]
        pub container_version_id: Option<String>,
        #[doc = "A value of true indicates this container version has been deleted."]
        #[serde(rename = "deleted", default)]
        pub deleted: Option<bool>,
        #[doc = "Container version display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Number of custom templates in the container version."]
        #[serde(rename = "numCustomTemplates", default)]
        pub num_custom_templates: Option<String>,
        #[doc = "Number of macros in the container version."]
        #[serde(rename = "numMacros", default)]
        pub num_macros: Option<String>,
        #[doc = "Number of rules in the container version."]
        #[serde(rename = "numRules", default)]
        pub num_rules: Option<String>,
        #[doc = "Number of tags in the container version."]
        #[serde(rename = "numTags", default)]
        pub num_tags: Option<String>,
        #[doc = "Number of triggers in the container version."]
        #[serde(rename = "numTriggers", default)]
        pub num_triggers: Option<String>,
        #[doc = "Number of variables in the container version."]
        #[serde(rename = "numVariables", default)]
        pub num_variables: Option<String>,
        #[doc = "Number of zones in the container version."]
        #[serde(rename = "numZones", default)]
        pub num_zones: Option<String>,
        #[doc = "GTM Container Versions's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
    }
    impl ::field_selector::FieldSelector for ContainerVersionHeader {
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
    pub struct CreateBuiltInVariableResponse {
        #[doc = "List of created built-in variables."]
        #[serde(rename = "builtInVariable", default)]
        pub built_in_variable: Option<Vec<crate::schemas::BuiltInVariable>>,
    }
    impl ::field_selector::FieldSelector for CreateBuiltInVariableResponse {
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
    pub struct CreateContainerVersionRequestVersionOptions {
        #[doc = "The name of the container version to be created."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The notes of the container version to be created."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateContainerVersionRequestVersionOptions {
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
    pub struct CreateContainerVersionResponse {
        #[doc = "Compiler errors or not."]
        #[serde(rename = "compilerError", default)]
        pub compiler_error: Option<bool>,
        #[doc = "The container version created."]
        #[serde(rename = "containerVersion", default)]
        pub container_version: Option<crate::schemas::ContainerVersion>,
        #[doc = "Auto generated workspace path created as a result of version creation. This field should only be populated if the created version was not a quick preview."]
        #[serde(rename = "newWorkspacePath", default)]
        pub new_workspace_path: Option<String>,
        #[doc = "Whether version creation failed when syncing the workspace to the latest container version."]
        #[serde(rename = "syncStatus", default)]
        pub sync_status: Option<crate::schemas::SyncStatus>,
    }
    impl ::field_selector::FieldSelector for CreateContainerVersionResponse {
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
    pub struct CustomTemplate {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "The fingerprint of the GTM Custom Template as computed at storage time. This value is recomputed whenever the template is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "Custom Template display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "GTM Custom Template's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "The custom template in text format."]
        #[serde(rename = "templateData", default)]
        pub template_data: Option<String>,
        #[doc = "The Custom Template ID uniquely identifies the GTM custom template."]
        #[serde(rename = "templateId", default)]
        pub template_id: Option<String>,
        #[doc = "GTM Workspace ID."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomTemplate {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EntityChangeStatus {
        Added,
        ChangeStatusUnspecified,
        Deleted,
        None,
        Updated,
    }
    impl EntityChangeStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                EntityChangeStatus::Added => "added",
                EntityChangeStatus::ChangeStatusUnspecified => "changeStatusUnspecified",
                EntityChangeStatus::Deleted => "deleted",
                EntityChangeStatus::None => "none",
                EntityChangeStatus::Updated => "updated",
            }
        }
    }
    impl ::std::fmt::Display for EntityChangeStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EntityChangeStatus {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EntityChangeStatus {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "added" => EntityChangeStatus::Added,
                "changeStatusUnspecified" => EntityChangeStatus::ChangeStatusUnspecified,
                "deleted" => EntityChangeStatus::Deleted,
                "none" => EntityChangeStatus::None,
                "updated" => EntityChangeStatus::Updated,
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
    pub struct Entity {
        #[doc = "Represents how the entity has been changed in the workspace."]
        #[serde(rename = "changeStatus", default)]
        pub change_status: Option<crate::schemas::EntityChangeStatus>,
        #[doc = "The Folder being represented by the entity."]
        #[serde(rename = "folder", default)]
        pub folder: Option<crate::schemas::Folder>,
        #[doc = "The tag being represented by the entity."]
        #[serde(rename = "tag", default)]
        pub tag: Option<crate::schemas::Tag>,
        #[doc = "The trigger being represented by the entity."]
        #[serde(rename = "trigger", default)]
        pub trigger: Option<crate::schemas::Trigger>,
        #[doc = "The variable being represented by the entity."]
        #[serde(rename = "variable", default)]
        pub variable: Option<crate::schemas::Variable>,
    }
    impl ::field_selector::FieldSelector for Entity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnvironmentType {
        Latest,
        Live,
        User,
        Workspace,
    }
    impl EnvironmentType {
        pub fn as_str(self) -> &'static str {
            match self {
                EnvironmentType::Latest => "latest",
                EnvironmentType::Live => "live",
                EnvironmentType::User => "user",
                EnvironmentType::Workspace => "workspace",
            }
        }
    }
    impl ::std::fmt::Display for EnvironmentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnvironmentType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnvironmentType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "latest" => EnvironmentType::Latest,
                "live" => EnvironmentType::Live,
                "user" => EnvironmentType::User,
                "workspace" => EnvironmentType::Workspace,
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
    pub struct Environment {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "The environment authorization code."]
        #[serde(rename = "authorizationCode", default)]
        pub authorization_code: Option<String>,
        #[doc = "The last update time-stamp for the authorization code."]
        #[serde(rename = "authorizationTimestamp", default)]
        pub authorization_timestamp: Option<crate::schemas::Timestamp>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "Represents a link to a container version."]
        #[serde(rename = "containerVersionId", default)]
        pub container_version_id: Option<String>,
        #[doc = "The environment description. Can be set or changed only on USER type environments."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Whether or not to enable debug by default for the environment."]
        #[serde(rename = "enableDebug", default)]
        pub enable_debug: Option<bool>,
        #[doc = "GTM Environment ID uniquely identifies the GTM Environment."]
        #[serde(rename = "environmentId", default)]
        pub environment_id: Option<String>,
        #[doc = "The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "The environment display name. Can be set or changed only on USER type environments."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "GTM Environment's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "The type of this environment."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::EnvironmentType>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "Default preview page url for the environment."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
        #[doc = "Represents a link to a quick preview of a workspace."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Environment {
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
    pub struct Folder {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "The Folder ID uniquely identifies the GTM Folder."]
        #[serde(rename = "folderId", default)]
        pub folder_id: Option<String>,
        #[doc = "Folder display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User notes on how to apply this folder in the container."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
        #[doc = "GTM Folder's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "GTM Workspace ID."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Folder {
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
    pub struct FolderEntities {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "The list of tags inside the folder."]
        #[serde(rename = "tag", default)]
        pub tag: Option<Vec<crate::schemas::Tag>>,
        #[doc = "The list of triggers inside the folder."]
        #[serde(rename = "trigger", default)]
        pub trigger: Option<Vec<crate::schemas::Trigger>>,
        #[doc = "The list of variables inside the folder."]
        #[serde(rename = "variable", default)]
        pub variable: Option<Vec<crate::schemas::Variable>>,
    }
    impl ::field_selector::FieldSelector for FolderEntities {
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
    pub struct GetWorkspaceStatusResponse {
        #[doc = "The merge conflict after sync."]
        #[serde(rename = "mergeConflict", default)]
        pub merge_conflict: Option<Vec<crate::schemas::MergeConflict>>,
        #[doc = "Entities that have been changed in the workspace."]
        #[serde(rename = "workspaceChange", default)]
        pub workspace_change: Option<Vec<crate::schemas::Entity>>,
    }
    impl ::field_selector::FieldSelector for GetWorkspaceStatusResponse {
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
    pub struct ListAccountsResponse {
        #[doc = "List of GTM Accounts that a user has access to."]
        #[serde(rename = "account", default)]
        pub account: Option<Vec<crate::schemas::Account>>,
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListAccountsResponse {
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
    pub struct ListContainerVersionsResponse {
        #[doc = "All container version headers of a GTM Container."]
        #[serde(rename = "containerVersionHeader", default)]
        pub container_version_header: Option<Vec<crate::schemas::ContainerVersionHeader>>,
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListContainerVersionsResponse {
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
    pub struct ListContainersResponse {
        #[doc = "All Containers of a GTM Account."]
        #[serde(rename = "container", default)]
        pub container: Option<Vec<crate::schemas::Container>>,
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListContainersResponse {
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
    pub struct ListEnabledBuiltInVariablesResponse {
        #[doc = "All GTM BuiltInVariables of a GTM container."]
        #[serde(rename = "builtInVariable", default)]
        pub built_in_variable: Option<Vec<crate::schemas::BuiltInVariable>>,
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListEnabledBuiltInVariablesResponse {
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
    pub struct ListEnvironmentsResponse {
        #[doc = "All Environments of a GTM Container."]
        #[serde(rename = "environment", default)]
        pub environment: Option<Vec<crate::schemas::Environment>>,
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListEnvironmentsResponse {
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
    pub struct ListFoldersResponse {
        #[doc = "All GTM Folders of a GTM Container."]
        #[serde(rename = "folder", default)]
        pub folder: Option<Vec<crate::schemas::Folder>>,
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for ListFoldersResponse {
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
    pub struct ListTagsResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "All GTM Tags of a GTM Container."]
        #[serde(rename = "tag", default)]
        pub tag: Option<Vec<crate::schemas::Tag>>,
    }
    impl ::field_selector::FieldSelector for ListTagsResponse {
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
    pub struct ListTemplatesResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "All GTM Custom Templates of a GTM Container."]
        #[serde(rename = "template", default)]
        pub template: Option<Vec<crate::schemas::CustomTemplate>>,
    }
    impl ::field_selector::FieldSelector for ListTemplatesResponse {
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
    pub struct ListTriggersResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "All GTM Triggers of a GTM Container."]
        #[serde(rename = "trigger", default)]
        pub trigger: Option<Vec<crate::schemas::Trigger>>,
    }
    impl ::field_selector::FieldSelector for ListTriggersResponse {
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
    pub struct ListUserPermissionsResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "All GTM UserPermissions of a GTM Account."]
        #[serde(rename = "userPermission", default)]
        pub user_permission: Option<Vec<crate::schemas::UserPermission>>,
    }
    impl ::field_selector::FieldSelector for ListUserPermissionsResponse {
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
    pub struct ListVariablesResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "All GTM Variables of a GTM Container."]
        #[serde(rename = "variable", default)]
        pub variable: Option<Vec<crate::schemas::Variable>>,
    }
    impl ::field_selector::FieldSelector for ListVariablesResponse {
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
    pub struct ListWorkspacesResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "All Workspaces of a GTM Container."]
        #[serde(rename = "workspace", default)]
        pub workspace: Option<Vec<crate::schemas::Workspace>>,
    }
    impl ::field_selector::FieldSelector for ListWorkspacesResponse {
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
    pub struct ListZonesResponse {
        #[doc = "Continuation token for fetching the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
        #[doc = "All GTM Zones of a GTM Container."]
        #[serde(rename = "zone", default)]
        pub zone: Option<Vec<crate::schemas::Zone>>,
    }
    impl ::field_selector::FieldSelector for ListZonesResponse {
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
    pub struct MergeConflict {
        #[doc = "The base version entity (since the latest sync operation) that has conflicting changes compared to the workspace. If this field is missing, it means the workspace entity is deleted from the base version."]
        #[serde(rename = "entityInBaseVersion", default)]
        pub entity_in_base_version: Option<crate::schemas::Entity>,
        #[doc = "The workspace entity that has conflicting changes compared to the base version. If an entity is deleted in a workspace, it will still appear with a deleted change status."]
        #[serde(rename = "entityInWorkspace", default)]
        pub entity_in_workspace: Option<crate::schemas::Entity>,
    }
    impl ::field_selector::FieldSelector for MergeConflict {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParameterType {
        Boolean,
        Integer,
        List,
        Map,
        Template,
        TriggerReference,
        TypeUnspecified,
    }
    impl ParameterType {
        pub fn as_str(self) -> &'static str {
            match self {
                ParameterType::Boolean => "boolean",
                ParameterType::Integer => "integer",
                ParameterType::List => "list",
                ParameterType::Map => "map",
                ParameterType::Template => "template",
                ParameterType::TriggerReference => "triggerReference",
                ParameterType::TypeUnspecified => "typeUnspecified",
            }
        }
    }
    impl ::std::fmt::Display for ParameterType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParameterType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParameterType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "boolean" => ParameterType::Boolean,
                "integer" => ParameterType::Integer,
                "list" => ParameterType::List,
                "map" => ParameterType::Map,
                "template" => ParameterType::Template,
                "triggerReference" => ParameterType::TriggerReference,
                "typeUnspecified" => ParameterType::TypeUnspecified,
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
    pub struct Parameter {
        #[doc = "The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "This list parameter's parameters (keys will be ignored)."]
        #[serde(rename = "list", default)]
        pub list: Option<Vec<crate::schemas::Parameter>>,
        #[doc = "This map parameter's parameters (must have keys; keys must be unique)."]
        #[serde(rename = "map", default)]
        pub map: Option<Vec<crate::schemas::Parameter>>,
        #[doc = "The parameter type. Valid values are: \n- boolean: The value represents a boolean, represented as 'true' or 'false' \n- integer: The value represents a 64-bit signed integer value, in base 10 \n- list: A list of parameters should be specified \n- map: A map of parameters should be specified \n- template: The value represents any text; this can include variable references (even variable references that might return non-string types) \n- trigger_reference: The value represents a trigger, represented as the trigger id"]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::ParameterType>,
        #[doc = "A parameter's value (may contain variable references such as \"{{myVariable}}\") as appropriate to the specified type."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for Parameter {
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
    pub struct PublishContainerVersionResponse {
        #[doc = "Compiler errors or not."]
        #[serde(rename = "compilerError", default)]
        pub compiler_error: Option<bool>,
        #[doc = "The container version created."]
        #[serde(rename = "containerVersion", default)]
        pub container_version: Option<crate::schemas::ContainerVersion>,
    }
    impl ::field_selector::FieldSelector for PublishContainerVersionResponse {
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
    pub struct QuickPreviewResponse {
        #[doc = "Were there compiler errors or not."]
        #[serde(rename = "compilerError", default)]
        pub compiler_error: Option<bool>,
        #[doc = "The quick previewed container version."]
        #[serde(rename = "containerVersion", default)]
        pub container_version: Option<crate::schemas::ContainerVersion>,
        #[doc = "Whether quick previewing failed when syncing the workspace to the latest container version."]
        #[serde(rename = "syncStatus", default)]
        pub sync_status: Option<crate::schemas::SyncStatus>,
    }
    impl ::field_selector::FieldSelector for QuickPreviewResponse {
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
    pub struct RevertBuiltInVariableResponse {
        #[doc = "Whether the built-in variable is enabled after reversion."]
        #[serde(rename = "enabled", default)]
        pub enabled: Option<bool>,
    }
    impl ::field_selector::FieldSelector for RevertBuiltInVariableResponse {
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
    pub struct RevertFolderResponse {
        #[doc = "Folder as it appears in the latest container version since the last workspace synchronization operation. If no folder is present, that means the folder was deleted in the latest container version."]
        #[serde(rename = "folder", default)]
        pub folder: Option<crate::schemas::Folder>,
    }
    impl ::field_selector::FieldSelector for RevertFolderResponse {
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
    pub struct RevertTagResponse {
        #[doc = "Tag as it appears in the latest container version since the last workspace synchronization operation. If no tag is present, that means the tag was deleted in the latest container version."]
        #[serde(rename = "tag", default)]
        pub tag: Option<crate::schemas::Tag>,
    }
    impl ::field_selector::FieldSelector for RevertTagResponse {
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
    pub struct RevertTemplateResponse {
        #[doc = "Template as it appears in the latest container version since the last workspace synchronization operation. If no template is present, that means the template was deleted in the latest container version."]
        #[serde(rename = "template", default)]
        pub template: Option<crate::schemas::CustomTemplate>,
    }
    impl ::field_selector::FieldSelector for RevertTemplateResponse {
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
    pub struct RevertTriggerResponse {
        #[doc = "Trigger as it appears in the latest container version since the last workspace synchronization operation. If no trigger is present, that means the trigger was deleted in the latest container version."]
        #[serde(rename = "trigger", default)]
        pub trigger: Option<crate::schemas::Trigger>,
    }
    impl ::field_selector::FieldSelector for RevertTriggerResponse {
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
    pub struct RevertVariableResponse {
        #[doc = "Variable as it appears in the latest container version since the last workspace synchronization operation. If no variable is present, that means the variable was deleted in the latest container version."]
        #[serde(rename = "variable", default)]
        pub variable: Option<crate::schemas::Variable>,
    }
    impl ::field_selector::FieldSelector for RevertVariableResponse {
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
    pub struct RevertZoneResponse {
        #[doc = "Zone as it appears in the latest container version since the last workspace synchronization operation. If no zone is present, that means the zone was deleted in the latest container version."]
        #[serde(rename = "zone", default)]
        pub zone: Option<crate::schemas::Zone>,
    }
    impl ::field_selector::FieldSelector for RevertZoneResponse {
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
    pub struct SetupTag {
        #[doc = "If true, fire the main tag if and only if the setup tag fires successfully. If false, fire the main tag regardless of setup tag firing status."]
        #[serde(rename = "stopOnSetupFailure", default)]
        pub stop_on_setup_failure: Option<bool>,
        #[doc = "The name of the setup tag."]
        #[serde(rename = "tagName", default)]
        pub tag_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for SetupTag {
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
    pub struct SyncStatus {
        #[doc = "Synchornization operation detected a merge conflict."]
        #[serde(rename = "mergeConflict", default)]
        pub merge_conflict: Option<bool>,
        #[doc = "An error occurred during the synchronization operation."]
        #[serde(rename = "syncError", default)]
        pub sync_error: Option<bool>,
    }
    impl ::field_selector::FieldSelector for SyncStatus {
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
    pub struct SyncWorkspaceResponse {
        #[doc = "The merge conflict after sync. If this field is not empty, the sync is still treated as successful. But a version cannot be created until all conflicts are resolved."]
        #[serde(rename = "mergeConflict", default)]
        pub merge_conflict: Option<Vec<crate::schemas::MergeConflict>>,
        #[doc = "Indicates whether synchronization caused a merge conflict or sync error."]
        #[serde(rename = "syncStatus", default)]
        pub sync_status: Option<crate::schemas::SyncStatus>,
    }
    impl ::field_selector::FieldSelector for SyncWorkspaceResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TagTagFiringOption {
        OncePerEvent,
        OncePerLoad,
        TagFiringOptionUnspecified,
        Unlimited,
    }
    impl TagTagFiringOption {
        pub fn as_str(self) -> &'static str {
            match self {
                TagTagFiringOption::OncePerEvent => "oncePerEvent",
                TagTagFiringOption::OncePerLoad => "oncePerLoad",
                TagTagFiringOption::TagFiringOptionUnspecified => "tagFiringOptionUnspecified",
                TagTagFiringOption::Unlimited => "unlimited",
            }
        }
    }
    impl ::std::fmt::Display for TagTagFiringOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TagTagFiringOption {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TagTagFiringOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "oncePerEvent" => TagTagFiringOption::OncePerEvent,
                "oncePerLoad" => TagTagFiringOption::OncePerLoad,
                "tagFiringOptionUnspecified" => TagTagFiringOption::TagFiringOptionUnspecified,
                "unlimited" => TagTagFiringOption::Unlimited,
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
    pub struct Tag {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire."]
        #[serde(rename = "blockingRuleId", default)]
        pub blocking_rule_id: Option<Vec<String>>,
        #[doc = "Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire."]
        #[serde(rename = "blockingTriggerId", default)]
        pub blocking_trigger_id: Option<Vec<String>>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false."]
        #[serde(rename = "firingRuleId", default)]
        pub firing_rule_id: Option<Vec<String>>,
        #[doc = "Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false."]
        #[serde(rename = "firingTriggerId", default)]
        pub firing_trigger_id: Option<Vec<String>>,
        #[doc = "If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode)."]
        #[serde(rename = "liveOnly", default)]
        pub live_only: Option<bool>,
        #[doc = "Tag display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User notes on how to apply this tag in the container."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
        #[doc = "The tag's parameters."]
        #[serde(rename = "parameter", default)]
        pub parameter: Option<Vec<crate::schemas::Parameter>>,
        #[doc = "Parent folder id."]
        #[serde(rename = "parentFolderId", default)]
        pub parent_folder_id: Option<String>,
        #[doc = "GTM Tag's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Indicates whether the tag is paused, which prevents the tag from firing."]
        #[serde(rename = "paused", default)]
        pub paused: Option<bool>,
        #[doc = "User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0."]
        #[serde(rename = "priority", default)]
        pub priority: Option<crate::schemas::Parameter>,
        #[doc = "GTM Tag Type."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The end timestamp in milliseconds to schedule a tag."]
        #[serde(rename = "scheduleEndMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub schedule_end_ms: Option<i64>,
        #[doc = "The start timestamp in milliseconds to schedule a tag."]
        #[serde(rename = "scheduleStartMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub schedule_start_ms: Option<i64>,
        #[doc = "The list of setup tags. Currently we only allow one."]
        #[serde(rename = "setupTag", default)]
        pub setup_tag: Option<Vec<crate::schemas::SetupTag>>,
        #[doc = "Option to fire this tag."]
        #[serde(rename = "tagFiringOption", default)]
        pub tag_firing_option: Option<crate::schemas::TagTagFiringOption>,
        #[doc = "The Tag ID uniquely identifies the GTM Tag."]
        #[serde(rename = "tagId", default)]
        pub tag_id: Option<String>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "The list of teardown tags. Currently we only allow one."]
        #[serde(rename = "teardownTag", default)]
        pub teardown_tag: Option<Vec<crate::schemas::TeardownTag>>,
        #[doc = "GTM Workspace ID."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Tag {
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
    pub struct TeardownTag {
        #[doc = "If true, fire the teardown tag if and only if the main tag fires successfully. If false, fire the teardown tag regardless of main tag firing status."]
        #[serde(rename = "stopTeardownOnFailure", default)]
        pub stop_teardown_on_failure: Option<bool>,
        #[doc = "The name of the teardown tag."]
        #[serde(rename = "tagName", default)]
        pub tag_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for TeardownTag {
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
    pub struct Timestamp {
        #[doc = "Non-negative fractions of a second at nanosecond resolution. Negative second values with fractions must still have non-negative nanos values that count forward in time. Must be from 0 to 999,999,999 inclusive."]
        #[serde(rename = "nanos", default)]
        pub nanos: Option<i32>,
        #[doc = "Represents seconds of UTC time since Unix epoch 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to 9999-12-31T23:59:59Z inclusive."]
        #[serde(rename = "seconds", default)]
        #[serde(with = "crate::parsed_string")]
        pub seconds: Option<i64>,
    }
    impl ::field_selector::FieldSelector for Timestamp {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TriggerType {
        Always,
        AmpClick,
        AmpScroll,
        AmpTimer,
        AmpVisibility,
        Click,
        CustomEvent,
        DomReady,
        ElementVisibility,
        EventTypeUnspecified,
        FirebaseAppException,
        FirebaseAppUpdate,
        FirebaseCampaign,
        FirebaseFirstOpen,
        FirebaseInAppPurchase,
        FirebaseNotificationDismiss,
        FirebaseNotificationForeground,
        FirebaseNotificationOpen,
        FirebaseNotificationReceive,
        FirebaseOsUpdate,
        FirebaseSessionStart,
        FirebaseUserEngagement,
        FormSubmission,
        HistoryChange,
        JsError,
        LinkClick,
        Pageview,
        ScrollDepth,
        Timer,
        TriggerGroup,
        WindowLoaded,
        YouTubeVideo,
    }
    impl TriggerType {
        pub fn as_str(self) -> &'static str {
            match self {
                TriggerType::Always => "always",
                TriggerType::AmpClick => "ampClick",
                TriggerType::AmpScroll => "ampScroll",
                TriggerType::AmpTimer => "ampTimer",
                TriggerType::AmpVisibility => "ampVisibility",
                TriggerType::Click => "click",
                TriggerType::CustomEvent => "customEvent",
                TriggerType::DomReady => "domReady",
                TriggerType::ElementVisibility => "elementVisibility",
                TriggerType::EventTypeUnspecified => "eventTypeUnspecified",
                TriggerType::FirebaseAppException => "firebaseAppException",
                TriggerType::FirebaseAppUpdate => "firebaseAppUpdate",
                TriggerType::FirebaseCampaign => "firebaseCampaign",
                TriggerType::FirebaseFirstOpen => "firebaseFirstOpen",
                TriggerType::FirebaseInAppPurchase => "firebaseInAppPurchase",
                TriggerType::FirebaseNotificationDismiss => "firebaseNotificationDismiss",
                TriggerType::FirebaseNotificationForeground => "firebaseNotificationForeground",
                TriggerType::FirebaseNotificationOpen => "firebaseNotificationOpen",
                TriggerType::FirebaseNotificationReceive => "firebaseNotificationReceive",
                TriggerType::FirebaseOsUpdate => "firebaseOsUpdate",
                TriggerType::FirebaseSessionStart => "firebaseSessionStart",
                TriggerType::FirebaseUserEngagement => "firebaseUserEngagement",
                TriggerType::FormSubmission => "formSubmission",
                TriggerType::HistoryChange => "historyChange",
                TriggerType::JsError => "jsError",
                TriggerType::LinkClick => "linkClick",
                TriggerType::Pageview => "pageview",
                TriggerType::ScrollDepth => "scrollDepth",
                TriggerType::Timer => "timer",
                TriggerType::TriggerGroup => "triggerGroup",
                TriggerType::WindowLoaded => "windowLoaded",
                TriggerType::YouTubeVideo => "youTubeVideo",
            }
        }
    }
    impl ::std::fmt::Display for TriggerType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TriggerType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TriggerType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "always" => TriggerType::Always,
                "ampClick" => TriggerType::AmpClick,
                "ampScroll" => TriggerType::AmpScroll,
                "ampTimer" => TriggerType::AmpTimer,
                "ampVisibility" => TriggerType::AmpVisibility,
                "click" => TriggerType::Click,
                "customEvent" => TriggerType::CustomEvent,
                "domReady" => TriggerType::DomReady,
                "elementVisibility" => TriggerType::ElementVisibility,
                "eventTypeUnspecified" => TriggerType::EventTypeUnspecified,
                "firebaseAppException" => TriggerType::FirebaseAppException,
                "firebaseAppUpdate" => TriggerType::FirebaseAppUpdate,
                "firebaseCampaign" => TriggerType::FirebaseCampaign,
                "firebaseFirstOpen" => TriggerType::FirebaseFirstOpen,
                "firebaseInAppPurchase" => TriggerType::FirebaseInAppPurchase,
                "firebaseNotificationDismiss" => TriggerType::FirebaseNotificationDismiss,
                "firebaseNotificationForeground" => TriggerType::FirebaseNotificationForeground,
                "firebaseNotificationOpen" => TriggerType::FirebaseNotificationOpen,
                "firebaseNotificationReceive" => TriggerType::FirebaseNotificationReceive,
                "firebaseOsUpdate" => TriggerType::FirebaseOsUpdate,
                "firebaseSessionStart" => TriggerType::FirebaseSessionStart,
                "firebaseUserEngagement" => TriggerType::FirebaseUserEngagement,
                "formSubmission" => TriggerType::FormSubmission,
                "historyChange" => TriggerType::HistoryChange,
                "jsError" => TriggerType::JsError,
                "linkClick" => TriggerType::LinkClick,
                "pageview" => TriggerType::Pageview,
                "scrollDepth" => TriggerType::ScrollDepth,
                "timer" => TriggerType::Timer,
                "triggerGroup" => TriggerType::TriggerGroup,
                "windowLoaded" => TriggerType::WindowLoaded,
                "youTubeVideo" => TriggerType::YouTubeVideo,
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
    pub struct Trigger {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "Used in the case of auto event tracking."]
        #[serde(rename = "autoEventFilter", default)]
        pub auto_event_filter: Option<Vec<crate::schemas::Condition>>,
        #[doc = "Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers."]
        #[serde(rename = "checkValidation", default)]
        pub check_validation: Option<crate::schemas::Parameter>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger."]
        #[serde(rename = "continuousTimeMinMilliseconds", default)]
        pub continuous_time_min_milliseconds: Option<crate::schemas::Parameter>,
        #[doc = "Used in the case of custom event, which is fired iff all Conditions are true."]
        #[serde(rename = "customEventFilter", default)]
        pub custom_event_filter: Option<Vec<crate::schemas::Condition>>,
        #[doc = "Name of the GTM event that is fired. Only valid for Timer triggers."]
        #[serde(rename = "eventName", default)]
        pub event_name: Option<crate::schemas::Parameter>,
        #[doc = "The trigger will only fire iff all Conditions are true."]
        #[serde(rename = "filter", default)]
        pub filter: Option<Vec<crate::schemas::Condition>>,
        #[doc = "The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers."]
        #[serde(rename = "horizontalScrollPercentageList", default)]
        pub horizontal_scroll_percentage_list: Option<crate::schemas::Parameter>,
        #[doc = "Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers."]
        #[serde(rename = "interval", default)]
        pub interval: Option<crate::schemas::Parameter>,
        #[doc = "Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger."]
        #[serde(rename = "intervalSeconds", default)]
        pub interval_seconds: Option<crate::schemas::Parameter>,
        #[doc = "Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers."]
        #[serde(rename = "limit", default)]
        pub limit: Option<crate::schemas::Parameter>,
        #[doc = "Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger."]
        #[serde(rename = "maxTimerLengthSeconds", default)]
        pub max_timer_length_seconds: Option<crate::schemas::Parameter>,
        #[doc = "Trigger display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User notes on how to apply this trigger in the container."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
        #[doc = "Additional parameters."]
        #[serde(rename = "parameter", default)]
        pub parameter: Option<Vec<crate::schemas::Parameter>>,
        #[doc = "Parent folder id."]
        #[serde(rename = "parentFolderId", default)]
        pub parent_folder_id: Option<String>,
        #[doc = "GTM Trigger's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Defines the data layer event that causes this trigger."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::TriggerType>,
        #[doc = "A click trigger CSS selector (i.e. \"a\", \"button\" etc.). Only valid for AMP Click trigger."]
        #[serde(rename = "selector", default)]
        pub selector: Option<crate::schemas::Parameter>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger."]
        #[serde(rename = "totalTimeMinMilliseconds", default)]
        pub total_time_min_milliseconds: Option<crate::schemas::Parameter>,
        #[doc = "The Trigger ID uniquely identifies the GTM Trigger."]
        #[serde(rename = "triggerId", default)]
        pub trigger_id: Option<String>,
        #[doc = "Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers."]
        #[serde(rename = "uniqueTriggerId", default)]
        pub unique_trigger_id: Option<crate::schemas::Parameter>,
        #[doc = "List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers."]
        #[serde(rename = "verticalScrollPercentageList", default)]
        pub vertical_scroll_percentage_list: Option<crate::schemas::Parameter>,
        #[doc = "A visibility trigger CSS selector (i.e. \"#id\"). Only valid for AMP Visibility trigger."]
        #[serde(rename = "visibilitySelector", default)]
        pub visibility_selector: Option<crate::schemas::Parameter>,
        #[doc = "A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger."]
        #[serde(rename = "visiblePercentageMax", default)]
        pub visible_percentage_max: Option<crate::schemas::Parameter>,
        #[doc = "A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger."]
        #[serde(rename = "visiblePercentageMin", default)]
        pub visible_percentage_min: Option<crate::schemas::Parameter>,
        #[doc = "Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers."]
        #[serde(rename = "waitForTags", default)]
        pub wait_for_tags: Option<crate::schemas::Parameter>,
        #[doc = "How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers."]
        #[serde(rename = "waitForTagsTimeout", default)]
        pub wait_for_tags_timeout: Option<crate::schemas::Parameter>,
        #[doc = "GTM Workspace ID."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Trigger {
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
    pub struct UserPermission {
        #[doc = "GTM Account access permissions."]
        #[serde(rename = "accountAccess", default)]
        pub account_access: Option<crate::schemas::AccountAccess>,
        #[doc = "The Account ID uniquely identifies the GTM Account."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "GTM Container access permissions."]
        #[serde(rename = "containerAccess", default)]
        pub container_access: Option<Vec<crate::schemas::ContainerAccess>>,
        #[doc = "User's email address."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: Option<String>,
        #[doc = "GTM UserPermission's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
    }
    impl ::field_selector::FieldSelector for UserPermission {
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
    pub struct Variable {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set."]
        #[serde(rename = "disablingTriggerId", default)]
        pub disabling_trigger_id: Option<Vec<String>>,
        #[doc = "For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set."]
        #[serde(rename = "enablingTriggerId", default)]
        pub enabling_trigger_id: Option<Vec<String>>,
        #[doc = "The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "Option to convert a variable value to other value."]
        #[serde(rename = "formatValue", default)]
        pub format_value: Option<crate::schemas::VariableFormatValue>,
        #[doc = "Variable display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User notes on how to apply this variable in the container."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
        #[doc = "The variable's parameters."]
        #[serde(rename = "parameter", default)]
        pub parameter: Option<Vec<crate::schemas::Parameter>>,
        #[doc = "Parent folder id."]
        #[serde(rename = "parentFolderId", default)]
        pub parent_folder_id: Option<String>,
        #[doc = "GTM Variable's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "GTM Variable Type."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The end timestamp in milliseconds to schedule a variable."]
        #[serde(rename = "scheduleEndMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub schedule_end_ms: Option<i64>,
        #[doc = "The start timestamp in milliseconds to schedule a variable."]
        #[serde(rename = "scheduleStartMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub schedule_start_ms: Option<i64>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "The Variable ID uniquely identifies the GTM Variable."]
        #[serde(rename = "variableId", default)]
        pub variable_id: Option<String>,
        #[doc = "GTM Workspace ID."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Variable {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VariableFormatValueCaseConversionType {
        Lowercase,
        None,
        Uppercase,
    }
    impl VariableFormatValueCaseConversionType {
        pub fn as_str(self) -> &'static str {
            match self {
                VariableFormatValueCaseConversionType::Lowercase => "lowercase",
                VariableFormatValueCaseConversionType::None => "none",
                VariableFormatValueCaseConversionType::Uppercase => "uppercase",
            }
        }
    }
    impl ::std::fmt::Display for VariableFormatValueCaseConversionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VariableFormatValueCaseConversionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VariableFormatValueCaseConversionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "lowercase" => VariableFormatValueCaseConversionType::Lowercase,
                "none" => VariableFormatValueCaseConversionType::None,
                "uppercase" => VariableFormatValueCaseConversionType::Uppercase,
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
    pub struct VariableFormatValue {
        #[doc = "The option to convert a string-type variable value to either lowercase or uppercase."]
        #[serde(rename = "caseConversionType", default)]
        pub case_conversion_type: Option<crate::schemas::VariableFormatValueCaseConversionType>,
        #[doc = "The value to convert if a variable value is false."]
        #[serde(rename = "convertFalseToValue", default)]
        pub convert_false_to_value: Option<crate::schemas::Parameter>,
        #[doc = "The value to convert if a variable value is null."]
        #[serde(rename = "convertNullToValue", default)]
        pub convert_null_to_value: Option<crate::schemas::Parameter>,
        #[doc = "The value to convert if a variable value is true."]
        #[serde(rename = "convertTrueToValue", default)]
        pub convert_true_to_value: Option<crate::schemas::Parameter>,
        #[doc = "The value to convert if a variable value is undefined."]
        #[serde(rename = "convertUndefinedToValue", default)]
        pub convert_undefined_to_value: Option<crate::schemas::Parameter>,
    }
    impl ::field_selector::FieldSelector for VariableFormatValue {
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
    pub struct Workspace {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "Workspace description."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The fingerprint of the GTM Workspace as computed at storage time. This value is recomputed whenever the workspace is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "Workspace display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "GTM Workspace's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "The Workspace ID uniquely identifies the GTM Workspace."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Workspace {
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
    pub struct Zone {
        #[doc = "GTM Account ID."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "This Zone's boundary."]
        #[serde(rename = "boundary", default)]
        pub boundary: Option<crate::schemas::ZoneBoundary>,
        #[doc = "Containers that are children of this Zone."]
        #[serde(rename = "childContainer", default)]
        pub child_container: Option<Vec<crate::schemas::ZoneChildContainer>>,
        #[doc = "GTM Container ID."]
        #[serde(rename = "containerId", default)]
        pub container_id: Option<String>,
        #[doc = "The fingerprint of the GTM Zone as computed at storage time. This value is recomputed whenever the zone is modified."]
        #[serde(rename = "fingerprint", default)]
        pub fingerprint: Option<String>,
        #[doc = "Zone display name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "User notes on how to apply this zone in the container."]
        #[serde(rename = "notes", default)]
        pub notes: Option<String>,
        #[doc = "GTM Zone's API relative path."]
        #[serde(rename = "path", default)]
        pub path: Option<String>,
        #[doc = "Auto generated link to the tag manager UI"]
        #[serde(rename = "tagManagerUrl", default)]
        pub tag_manager_url: Option<String>,
        #[doc = "This Zone's type restrictions."]
        #[serde(rename = "typeRestriction", default)]
        pub type_restriction: Option<crate::schemas::ZoneTypeRestriction>,
        #[doc = "GTM Workspace ID."]
        #[serde(rename = "workspaceId", default)]
        pub workspace_id: Option<String>,
        #[doc = "The Zone ID uniquely identifies the GTM Zone."]
        #[serde(rename = "zoneId", default)]
        pub zone_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Zone {
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
    pub struct ZoneBoundary {
        #[doc = "The conditions that, when conjoined, make up the boundary."]
        #[serde(rename = "condition", default)]
        pub condition: Option<Vec<crate::schemas::Condition>>,
        #[doc = "Custom evaluation trigger IDs. A zone will evaluate its boundary conditions when any of the listed triggers are true."]
        #[serde(rename = "customEvaluationTriggerId", default)]
        pub custom_evaluation_trigger_id: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ZoneBoundary {
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
    pub struct ZoneChildContainer {
        #[doc = "The zone's nickname for the child container."]
        #[serde(rename = "nickname", default)]
        pub nickname: Option<String>,
        #[doc = "The child container's public id."]
        #[serde(rename = "publicId", default)]
        pub public_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ZoneChildContainer {
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
    pub struct ZoneTypeRestriction {
        #[doc = "True if type restrictions have been enabled for this Zone."]
        #[serde(rename = "enable", default)]
        pub enable: Option<bool>,
        #[doc = "List of type public ids that have been whitelisted for use in this Zone."]
        #[serde(rename = "whitelistedTypeId", default)]
        pub whitelisted_type_id: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ZoneTypeRestriction {
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
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
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
    #[doc = "Actions that can be performed on the accounts resource"]
    pub fn accounts(&self) -> crate::accounts::AccountsActions<A> {
        crate::accounts::AccountsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod accounts {
    pub mod params {}
    pub struct AccountsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> AccountsActions<'a, A> {
        #[doc = "Gets a GTM Account."]
        pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                path: path.into(),
            }
        }
        #[doc = "Lists all GTM Accounts that a user has access to."]
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
                page_token: None,
            }
        }
        #[doc = "Updates a GTM Account."]
        pub fn update(
            &self,
            request: crate::schemas::Account,
            path: impl Into<String>,
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
                path: path.into(),
                fingerprint: None,
            }
        }
        #[doc = "Actions that can be performed on the containers resource"]
        pub fn containers(&self) -> containers::ContainersActions<A> {
            containers::ContainersActions
        }
        #[doc = "Actions that can be performed on the user_permissions resource"]
        pub fn user_permissions(&self) -> user_permissions::UserPermissionsActions<A> {
            user_permissions::UserPermissionsActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        path: String,
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
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Account, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
            output.push_str(&self.path);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/tagmanager.readonly"])
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
        #[doc = "Continuation token for fetching the next page of results."]
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
        pub fn iter_account<T>(
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
                        #[serde(rename = "account")]
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
        ) -> Result<crate::schemas::ListAccountsResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
            output.push_str("accounts");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/tagmanager.readonly"])
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
        request: crate::schemas::Account,
        path: String,
        fingerprint: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "When provided, this fingerprint must match the fingerprint of the account in storage."]
        pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
            self.fingerprint = Some(value.into());
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
        ) -> Result<crate::schemas::Account, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
            output.push_str(&self.path);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[("fingerprint", &self.fingerprint)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&[
                    "https://www.googleapis.com/auth/tagmanager.manage.accounts",
                ])
                .unwrap()
                .access_token,
            );
            req
        }
    }
    pub mod containers {
        pub mod params {}
        pub struct ContainersActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ContainersActions<'a, A> {
            #[doc = "Creates a Container."]
            pub fn create(
                &self,
                request: crate::schemas::Container,
                parent: impl Into<String>,
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
                    parent: parent.into(),
                }
            }
            #[doc = "Deletes a Container."]
            pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                    path: path.into(),
                }
            }
            #[doc = "Gets a Container."]
            pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                    path: path.into(),
                }
            }
            #[doc = "Lists all Containers that belongs to a GTM Account."]
            pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                    parent: parent.into(),
                    page_token: None,
                }
            }
            #[doc = "Updates a Container."]
            pub fn update(
                &self,
                request: crate::schemas::Container,
                path: impl Into<String>,
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
                    path: path.into(),
                    fingerprint: None,
                }
            }
            #[doc = "Actions that can be performed on the environments resource"]
            pub fn environments(&self) -> environments::EnvironmentsActions<A> {
                environments::EnvironmentsActions
            }
            #[doc = "Actions that can be performed on the version_headers resource"]
            pub fn version_headers(&self) -> version_headers::VersionHeadersActions<A> {
                version_headers::VersionHeadersActions
            }
            #[doc = "Actions that can be performed on the versions resource"]
            pub fn versions(&self) -> versions::VersionsActions<A> {
                versions::VersionsActions
            }
            #[doc = "Actions that can be performed on the workspaces resource"]
            pub fn workspaces(&self) -> workspaces::WorkspacesActions<A> {
                workspaces::WorkspacesActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Container,
            parent: String,
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
            ) -> Result<crate::schemas::Container, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.parent);
                output.push_str("/containers");
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
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.edit.containers",
                    ])
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
            path: String,
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.path);
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
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.delete.containers",
                    ])
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
            path: String,
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
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Container, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.path);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/tagmanager.readonly"])
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
            parent: String,
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
            #[doc = "Continuation token for fetching the next page of results."]
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
            pub fn iter_container<T>(
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
                            #[serde(rename = "container")]
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
            ) -> Result<crate::schemas::ListContainersResponse, Box<dyn ::std::error::Error>>
            {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.parent);
                output.push_str("/containers");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/tagmanager.readonly"])
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
            request: crate::schemas::Container,
            path: String,
            fingerprint: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = "When provided, this fingerprint must match the fingerprint of the container in storage."]
            pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                self.fingerprint = Some(value.into());
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
            ) -> Result<crate::schemas::Container, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.path);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("fingerprint", &self.fingerprint)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.edit.containers",
                    ])
                    .unwrap()
                    .access_token,
                );
                req
            }
        }
        pub mod environments {
            pub mod params {}
            pub struct EnvironmentsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> EnvironmentsActions<'a, A> {
                #[doc = "Creates a GTM Environment."]
                pub fn create(
                    &self,
                    request: crate::schemas::Environment,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Deletes a GTM Environment."]
                pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                        path: path.into(),
                    }
                }
                #[doc = "Gets a GTM Environment."]
                pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                        path: path.into(),
                    }
                }
                #[doc = "Lists all GTM Environments of a GTM Container."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                        parent: parent.into(),
                        page_token: None,
                    }
                }
                #[doc = "Re-generates the authorization code for a GTM Environment."]
                pub fn reauthorize(
                    &self,
                    request: crate::schemas::Environment,
                    path: impl Into<String>,
                ) -> ReauthorizeRequestBuilder<A> {
                    ReauthorizeRequestBuilder {
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
                        path: path.into(),
                    }
                }
                #[doc = "Updates a GTM Environment."]
                pub fn update(
                    &self,
                    request: crate::schemas::Environment,
                    path: impl Into<String>,
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
                        path: path.into(),
                        fingerprint: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Environment,
                parent: String,
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
                ) -> Result<crate::schemas::Environment, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.parent);
                    output.push_str("/environments");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
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
                path: String,
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
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
                path: String,
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
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Environment, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
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
                parent: String,
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
                #[doc = "Continuation token for fetching the next page of results."]
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
                pub fn iter_environment<T>(
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
                                #[serde(rename = "environment")]
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
                ) -> Result<crate::schemas::ListEnvironmentsResponse, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.parent);
                    output.push_str("/environments");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
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
            pub struct ReauthorizeRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Environment,
                path: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ReauthorizeRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Environment, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":reauthorize");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.publish",
                        ])
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
                request: crate::schemas::Environment,
                path: String,
                fingerprint: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                #[doc = "When provided, this fingerprint must match the fingerprint of the environment in storage."]
                pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fingerprint = Some(value.into());
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
                ) -> Result<crate::schemas::Environment, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("fingerprint", &self.fingerprint)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
        }
        pub mod version_headers {
            pub mod params {}
            pub struct VersionHeadersActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> VersionHeadersActions<'a, A> {
                #[doc = "Gets the latest container version header"]
                pub fn latest(&self, parent: impl Into<String>) -> LatestRequestBuilder<A> {
                    LatestRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Lists all Container Versions of a GTM Container."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                        parent: parent.into(),
                        include_deleted: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct LatestRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                parent: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> LatestRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::ContainerVersionHeader, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.parent);
                    output.push_str("/version_headers:latest");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
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
                parent: String,
                include_deleted: Option<bool>,
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
                #[doc = "Also retrieve deleted (archived) versions when true."]
                pub fn include_deleted(&mut self, value: bool) -> &mut Self {
                    self.include_deleted = Some(value);
                    self
                }
                #[doc = "Continuation token for fetching the next page of results."]
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
                pub fn iter_container_version_header<T>(
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
                                #[serde(rename = "containerVersionHeader")]
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
                ) -> Result<
                    crate::schemas::ListContainerVersionsResponse,
                    Box<dyn ::std::error::Error>,
                > {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.parent);
                    output.push_str("/version_headers");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("includeDeleted", &self.include_deleted)]);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
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
        }
        pub mod versions {
            pub mod params {}
            pub struct VersionsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> VersionsActions<'a, A> {
                #[doc = "Deletes a Container Version."]
                pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                        path: path.into(),
                    }
                }
                #[doc = "Gets a Container Version."]
                pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                        path: path.into(),
                        container_version_id: None,
                    }
                }
                #[doc = "Gets the live (i.e. published) container version"]
                pub fn live(&self, parent: impl Into<String>) -> LiveRequestBuilder<A> {
                    LiveRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        parent: parent.into(),
                    }
                }
                #[doc = "Publishes a Container Version."]
                pub fn publish(&self, path: impl Into<String>) -> PublishRequestBuilder<A> {
                    PublishRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        path: path.into(),
                        fingerprint: None,
                    }
                }
                #[doc = "Sets the latest version used for synchronization of workspaces when detecting conflicts and errors."]
                pub fn set_latest(&self, path: impl Into<String>) -> SetLatestRequestBuilder<A> {
                    SetLatestRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        path: path.into(),
                    }
                }
                #[doc = "Undeletes a Container Version."]
                pub fn undelete(&self, path: impl Into<String>) -> UndeleteRequestBuilder<A> {
                    UndeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        path: path.into(),
                    }
                }
                #[doc = "Updates a Container Version."]
                pub fn update(
                    &self,
                    request: crate::schemas::ContainerVersion,
                    path: impl Into<String>,
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
                        path: path.into(),
                        fingerprint: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                path: String,
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
                        ])
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
                path: String,
                container_version_id: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "The GTM ContainerVersion ID. Specify published to retrieve the currently published version."]
                pub fn container_version_id(&mut self, value: impl Into<String>) -> &mut Self {
                    self.container_version_id = Some(value.into());
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
                ) -> Result<crate::schemas::ContainerVersion, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("containerVersionId", &self.container_version_id)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct LiveRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                parent: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> LiveRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::ContainerVersion, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.parent);
                    output.push_str("/versions:live");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct PublishRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                path: String,
                fingerprint: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> PublishRequestBuilder<'a, A> {
                #[doc = "When provided, this fingerprint must match the fingerprint of the container version in storage."]
                pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fingerprint = Some(value.into());
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
                ) -> Result<
                    crate::schemas::PublishContainerVersionResponse,
                    Box<dyn ::std::error::Error>,
                > {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":publish");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("fingerprint", &self.fingerprint)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.publish",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct SetLatestRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                path: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> SetLatestRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::ContainerVersion, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":set_latest");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct UndeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                path: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UndeleteRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::ContainerVersion, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":undelete");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
                        ])
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
                request: crate::schemas::ContainerVersion,
                path: String,
                fingerprint: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                #[doc = "When provided, this fingerprint must match the fingerprint of the container version in storage."]
                pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fingerprint = Some(value.into());
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
                ) -> Result<crate::schemas::ContainerVersion, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("fingerprint", &self.fingerprint)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
        }
        pub mod workspaces {
            pub mod params {}
            pub struct WorkspacesActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> WorkspacesActions<'a, A> {
                #[doc = "Creates a Workspace."]
                pub fn create(
                    &self,
                    request: crate::schemas::Workspace,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Creates a Container Version from the entities present in the workspace, deletes the workspace, and sets the base container version to the newly created version."]
                pub fn create_version(
                    &self,
                    request: crate::schemas::CreateContainerVersionRequestVersionOptions,
                    path: impl Into<String>,
                ) -> CreateVersionRequestBuilder<A> {
                    CreateVersionRequestBuilder {
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
                        path: path.into(),
                    }
                }
                #[doc = "Deletes a Workspace."]
                pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                        path: path.into(),
                    }
                }
                #[doc = "Gets a Workspace."]
                pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                        path: path.into(),
                    }
                }
                #[doc = "Finds conflicting and modified entities in the workspace."]
                pub fn get_status(&self, path: impl Into<String>) -> GetStatusRequestBuilder<A> {
                    GetStatusRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        path: path.into(),
                    }
                }
                #[doc = "Lists all Workspaces that belong to a GTM Container."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                        parent: parent.into(),
                        page_token: None,
                    }
                }
                #[doc = "Quick previews a workspace by creating a fake container version from all entities in the provided workspace."]
                pub fn quick_preview(
                    &self,
                    path: impl Into<String>,
                ) -> QuickPreviewRequestBuilder<A> {
                    QuickPreviewRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        path: path.into(),
                    }
                }
                #[doc = "Resolves a merge conflict for a workspace entity by updating it to the resolved entity passed in the request."]
                pub fn resolve_conflict(
                    &self,
                    request: crate::schemas::Entity,
                    path: impl Into<String>,
                ) -> ResolveConflictRequestBuilder<A> {
                    ResolveConflictRequestBuilder {
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
                        path: path.into(),
                        fingerprint: None,
                    }
                }
                #[doc = "Syncs a workspace to the latest container version by updating all unmodified workspace entities and displaying conflicts for modified entities."]
                pub fn sync(&self, path: impl Into<String>) -> SyncRequestBuilder<A> {
                    SyncRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        path: path.into(),
                    }
                }
                #[doc = "Updates a Workspace."]
                pub fn update(
                    &self,
                    request: crate::schemas::Workspace,
                    path: impl Into<String>,
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
                        path: path.into(),
                        fingerprint: None,
                    }
                }
                #[doc = "Actions that can be performed on the built_in_variables resource"]
                pub fn built_in_variables(&self) -> built_in_variables::BuiltInVariablesActions<A> {
                    built_in_variables::BuiltInVariablesActions
                }
                #[doc = "Actions that can be performed on the folders resource"]
                pub fn folders(&self) -> folders::FoldersActions<A> {
                    folders::FoldersActions
                }
                #[doc = "Actions that can be performed on the tags resource"]
                pub fn tags(&self) -> tags::TagsActions<A> {
                    tags::TagsActions
                }
                #[doc = "Actions that can be performed on the templates resource"]
                pub fn templates(&self) -> templates::TemplatesActions<A> {
                    templates::TemplatesActions
                }
                #[doc = "Actions that can be performed on the triggers resource"]
                pub fn triggers(&self) -> triggers::TriggersActions<A> {
                    triggers::TriggersActions
                }
                #[doc = "Actions that can be performed on the variables resource"]
                pub fn variables(&self) -> variables::VariablesActions<A> {
                    variables::VariablesActions
                }
                #[doc = "Actions that can be performed on the zones resource"]
                pub fn zones(&self) -> zones::ZonesActions<A> {
                    zones::ZonesActions
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Workspace,
                parent: String,
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
                ) -> Result<crate::schemas::Workspace, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.parent);
                    output.push_str("/workspaces");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateVersionRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::CreateContainerVersionRequestVersionOptions,
                path: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateVersionRequestBuilder<'a, A> {
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
                ) -> Result<
                    crate::schemas::CreateContainerVersionResponse,
                    Box<dyn ::std::error::Error>,
                > {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":create_version");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
                        ])
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
                path: String,
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.delete.containers",
                        ])
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
                path: String,
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
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Workspace, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetStatusRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                path: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetStatusRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::GetWorkspaceStatusResponse, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str("/status");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
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
                parent: String,
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
                #[doc = "Continuation token for fetching the next page of results."]
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
                pub fn iter_workspace<T>(
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
                                #[serde(rename = "workspace")]
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
                ) -> Result<crate::schemas::ListWorkspacesResponse, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.parent);
                    output.push_str("/workspaces");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.readonly",
                        ])
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
            pub struct QuickPreviewRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                path: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> QuickPreviewRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::QuickPreviewResponse, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":quick_preview");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ResolveConflictRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Entity,
                path: String,
                fingerprint: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ResolveConflictRequestBuilder<'a, A> {
                #[doc = "When provided, this fingerprint must match the fingerprint of the entity_in_workspace in the merge conflict."]
                pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fingerprint = Some(value.into());
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
                    let req = req.json(&self.request);
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":resolve_conflict");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("fingerprint", &self.fingerprint)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct SyncRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                path: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> SyncRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::SyncWorkspaceResponse, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output.push_str(":sync");
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
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
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
                request: crate::schemas::Workspace,
                path: String,
                fingerprint: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                #[doc = "When provided, this fingerprint must match the fingerprint of the workspace in storage."]
                pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                    self.fingerprint = Some(value.into());
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
                ) -> Result<crate::schemas::Workspace, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                    output.push_str(&self.path);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("fingerprint", &self.fingerprint)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("userIp", &self.user_ip)]);
                    let mut auth = self.auth.lock().unwrap();
                    let req = req.bearer_auth(
                        auth.token::<_, &str>(&[
                            "https://www.googleapis.com/auth/tagmanager.edit.containers",
                        ])
                        .unwrap()
                        .access_token,
                    );
                    req
                }
            }
            pub mod built_in_variables {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum CreateType {
                        AdvertiserId,
                        AdvertisingTrackingEnabled,
                        AmpBrowserLanguage,
                        AmpCanonicalHost,
                        AmpCanonicalPath,
                        AmpCanonicalUrl,
                        AmpClientId,
                        AmpClientMaxScrollX,
                        AmpClientMaxScrollY,
                        AmpClientScreenHeight,
                        AmpClientScreenWidth,
                        AmpClientScrollX,
                        AmpClientScrollY,
                        AmpClientTimestamp,
                        AmpClientTimezone,
                        AmpGtmEvent,
                        AmpPageDownloadTime,
                        AmpPageLoadTime,
                        AmpPageViewId,
                        AmpReferrer,
                        AmpTitle,
                        AmpTotalEngagedTime,
                        AppId,
                        AppName,
                        AppVersionCode,
                        AppVersionName,
                        BuiltInVariableTypeUnspecified,
                        ClickClasses,
                        ClickElement,
                        ClickId,
                        ClickTarget,
                        ClickText,
                        ClickUrl,
                        ContainerId,
                        ContainerVersion,
                        DebugMode,
                        DeviceName,
                        ElementVisibilityFirstTime,
                        ElementVisibilityRatio,
                        ElementVisibilityRecentTime,
                        ElementVisibilityTime,
                        EnvironmentName,
                        ErrorLine,
                        ErrorMessage,
                        ErrorUrl,
                        Event,
                        EventName,
                        FirebaseEventParameterCampaign,
                        FirebaseEventParameterCampaignAclid,
                        FirebaseEventParameterCampaignAnid,
                        FirebaseEventParameterCampaignClickTimestamp,
                        FirebaseEventParameterCampaignContent,
                        FirebaseEventParameterCampaignCp1,
                        FirebaseEventParameterCampaignGclid,
                        FirebaseEventParameterCampaignSource,
                        FirebaseEventParameterCampaignTerm,
                        FirebaseEventParameterCurrency,
                        FirebaseEventParameterDynamicLinkAcceptTime,
                        FirebaseEventParameterDynamicLinkLinkid,
                        FirebaseEventParameterNotificationMessageDeviceTime,
                        FirebaseEventParameterNotificationMessageId,
                        FirebaseEventParameterNotificationMessageName,
                        FirebaseEventParameterNotificationMessageTime,
                        FirebaseEventParameterNotificationTopic,
                        FirebaseEventParameterPreviousAppVersion,
                        FirebaseEventParameterPreviousOsVersion,
                        FirebaseEventParameterPrice,
                        FirebaseEventParameterProductId,
                        FirebaseEventParameterQuantity,
                        FirebaseEventParameterValue,
                        FormClasses,
                        FormElement,
                        FormId,
                        FormTarget,
                        FormText,
                        FormUrl,
                        HistorySource,
                        HtmlId,
                        Language,
                        NewHistoryFragment,
                        NewHistoryState,
                        NewHistoryUrl,
                        OldHistoryFragment,
                        OldHistoryState,
                        OldHistoryUrl,
                        OsVersion,
                        PageHostname,
                        PagePath,
                        PageUrl,
                        Platform,
                        RandomNumber,
                        Referrer,
                        Resolution,
                        ScrollDepthDirection,
                        ScrollDepthThreshold,
                        ScrollDepthUnits,
                        SdkVersion,
                        VideoCurrentTime,
                        VideoDuration,
                        VideoPercent,
                        VideoProvider,
                        VideoStatus,
                        VideoTitle,
                        VideoUrl,
                        VideoVisible,
                    }
                    impl CreateType {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                CreateType::AdvertiserId => "advertiserId",
                                CreateType::AdvertisingTrackingEnabled => {
                                    "advertisingTrackingEnabled"
                                }
                                CreateType::AmpBrowserLanguage => "ampBrowserLanguage",
                                CreateType::AmpCanonicalHost => "ampCanonicalHost",
                                CreateType::AmpCanonicalPath => "ampCanonicalPath",
                                CreateType::AmpCanonicalUrl => "ampCanonicalUrl",
                                CreateType::AmpClientId => "ampClientId",
                                CreateType::AmpClientMaxScrollX => "ampClientMaxScrollX",
                                CreateType::AmpClientMaxScrollY => "ampClientMaxScrollY",
                                CreateType::AmpClientScreenHeight => "ampClientScreenHeight",
                                CreateType::AmpClientScreenWidth => "ampClientScreenWidth",
                                CreateType::AmpClientScrollX => "ampClientScrollX",
                                CreateType::AmpClientScrollY => "ampClientScrollY",
                                CreateType::AmpClientTimestamp => "ampClientTimestamp",
                                CreateType::AmpClientTimezone => "ampClientTimezone",
                                CreateType::AmpGtmEvent => "ampGtmEvent",
                                CreateType::AmpPageDownloadTime => "ampPageDownloadTime",
                                CreateType::AmpPageLoadTime => "ampPageLoadTime",
                                CreateType::AmpPageViewId => "ampPageViewId",
                                CreateType::AmpReferrer => "ampReferrer",
                                CreateType::AmpTitle => "ampTitle",
                                CreateType::AmpTotalEngagedTime => "ampTotalEngagedTime",
                                CreateType::AppId => "appId",
                                CreateType::AppName => "appName",
                                CreateType::AppVersionCode => "appVersionCode",
                                CreateType::AppVersionName => "appVersionName",
                                CreateType::BuiltInVariableTypeUnspecified => {
                                    "builtInVariableTypeUnspecified"
                                }
                                CreateType::ClickClasses => "clickClasses",
                                CreateType::ClickElement => "clickElement",
                                CreateType::ClickId => "clickId",
                                CreateType::ClickTarget => "clickTarget",
                                CreateType::ClickText => "clickText",
                                CreateType::ClickUrl => "clickUrl",
                                CreateType::ContainerId => "containerId",
                                CreateType::ContainerVersion => "containerVersion",
                                CreateType::DebugMode => "debugMode",
                                CreateType::DeviceName => "deviceName",
                                CreateType::ElementVisibilityFirstTime => {
                                    "elementVisibilityFirstTime"
                                }
                                CreateType::ElementVisibilityRatio => "elementVisibilityRatio",
                                CreateType::ElementVisibilityRecentTime => {
                                    "elementVisibilityRecentTime"
                                }
                                CreateType::ElementVisibilityTime => "elementVisibilityTime",
                                CreateType::EnvironmentName => "environmentName",
                                CreateType::ErrorLine => "errorLine",
                                CreateType::ErrorMessage => "errorMessage",
                                CreateType::ErrorUrl => "errorUrl",
                                CreateType::Event => "event",
                                CreateType::EventName => "eventName",
                                CreateType::FirebaseEventParameterCampaign => {
                                    "firebaseEventParameterCampaign"
                                }
                                CreateType::FirebaseEventParameterCampaignAclid => {
                                    "firebaseEventParameterCampaignAclid"
                                }
                                CreateType::FirebaseEventParameterCampaignAnid => {
                                    "firebaseEventParameterCampaignAnid"
                                }
                                CreateType::FirebaseEventParameterCampaignClickTimestamp => {
                                    "firebaseEventParameterCampaignClickTimestamp"
                                }
                                CreateType::FirebaseEventParameterCampaignContent => {
                                    "firebaseEventParameterCampaignContent"
                                }
                                CreateType::FirebaseEventParameterCampaignCp1 => {
                                    "firebaseEventParameterCampaignCp1"
                                }
                                CreateType::FirebaseEventParameterCampaignGclid => {
                                    "firebaseEventParameterCampaignGclid"
                                }
                                CreateType::FirebaseEventParameterCampaignSource => {
                                    "firebaseEventParameterCampaignSource"
                                }
                                CreateType::FirebaseEventParameterCampaignTerm => {
                                    "firebaseEventParameterCampaignTerm"
                                }
                                CreateType::FirebaseEventParameterCurrency => {
                                    "firebaseEventParameterCurrency"
                                }
                                CreateType::FirebaseEventParameterDynamicLinkAcceptTime => {
                                    "firebaseEventParameterDynamicLinkAcceptTime"
                                }
                                CreateType::FirebaseEventParameterDynamicLinkLinkid => {
                                    "firebaseEventParameterDynamicLinkLinkid"
                                }
                                CreateType::FirebaseEventParameterNotificationMessageDeviceTime => {
                                    "firebaseEventParameterNotificationMessageDeviceTime"
                                }
                                CreateType::FirebaseEventParameterNotificationMessageId => {
                                    "firebaseEventParameterNotificationMessageId"
                                }
                                CreateType::FirebaseEventParameterNotificationMessageName => {
                                    "firebaseEventParameterNotificationMessageName"
                                }
                                CreateType::FirebaseEventParameterNotificationMessageTime => {
                                    "firebaseEventParameterNotificationMessageTime"
                                }
                                CreateType::FirebaseEventParameterNotificationTopic => {
                                    "firebaseEventParameterNotificationTopic"
                                }
                                CreateType::FirebaseEventParameterPreviousAppVersion => {
                                    "firebaseEventParameterPreviousAppVersion"
                                }
                                CreateType::FirebaseEventParameterPreviousOsVersion => {
                                    "firebaseEventParameterPreviousOsVersion"
                                }
                                CreateType::FirebaseEventParameterPrice => {
                                    "firebaseEventParameterPrice"
                                }
                                CreateType::FirebaseEventParameterProductId => {
                                    "firebaseEventParameterProductId"
                                }
                                CreateType::FirebaseEventParameterQuantity => {
                                    "firebaseEventParameterQuantity"
                                }
                                CreateType::FirebaseEventParameterValue => {
                                    "firebaseEventParameterValue"
                                }
                                CreateType::FormClasses => "formClasses",
                                CreateType::FormElement => "formElement",
                                CreateType::FormId => "formId",
                                CreateType::FormTarget => "formTarget",
                                CreateType::FormText => "formText",
                                CreateType::FormUrl => "formUrl",
                                CreateType::HistorySource => "historySource",
                                CreateType::HtmlId => "htmlId",
                                CreateType::Language => "language",
                                CreateType::NewHistoryFragment => "newHistoryFragment",
                                CreateType::NewHistoryState => "newHistoryState",
                                CreateType::NewHistoryUrl => "newHistoryUrl",
                                CreateType::OldHistoryFragment => "oldHistoryFragment",
                                CreateType::OldHistoryState => "oldHistoryState",
                                CreateType::OldHistoryUrl => "oldHistoryUrl",
                                CreateType::OsVersion => "osVersion",
                                CreateType::PageHostname => "pageHostname",
                                CreateType::PagePath => "pagePath",
                                CreateType::PageUrl => "pageUrl",
                                CreateType::Platform => "platform",
                                CreateType::RandomNumber => "randomNumber",
                                CreateType::Referrer => "referrer",
                                CreateType::Resolution => "resolution",
                                CreateType::ScrollDepthDirection => "scrollDepthDirection",
                                CreateType::ScrollDepthThreshold => "scrollDepthThreshold",
                                CreateType::ScrollDepthUnits => "scrollDepthUnits",
                                CreateType::SdkVersion => "sdkVersion",
                                CreateType::VideoCurrentTime => "videoCurrentTime",
                                CreateType::VideoDuration => "videoDuration",
                                CreateType::VideoPercent => "videoPercent",
                                CreateType::VideoProvider => "videoProvider",
                                CreateType::VideoStatus => "videoStatus",
                                CreateType::VideoTitle => "videoTitle",
                                CreateType::VideoUrl => "videoUrl",
                                CreateType::VideoVisible => "videoVisible",
                            }
                        }
                    }
                    impl ::std::fmt::Display for CreateType {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for CreateType {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for CreateType {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "advertiserId" => CreateType::AdvertiserId,
                                "advertisingTrackingEnabled" => {
                                    CreateType::AdvertisingTrackingEnabled
                                }
                                "ampBrowserLanguage" => CreateType::AmpBrowserLanguage,
                                "ampCanonicalHost" => CreateType::AmpCanonicalHost,
                                "ampCanonicalPath" => CreateType::AmpCanonicalPath,
                                "ampCanonicalUrl" => CreateType::AmpCanonicalUrl,
                                "ampClientId" => CreateType::AmpClientId,
                                "ampClientMaxScrollX" => CreateType::AmpClientMaxScrollX,
                                "ampClientMaxScrollY" => CreateType::AmpClientMaxScrollY,
                                "ampClientScreenHeight" => CreateType::AmpClientScreenHeight,
                                "ampClientScreenWidth" => CreateType::AmpClientScreenWidth,
                                "ampClientScrollX" => CreateType::AmpClientScrollX,
                                "ampClientScrollY" => CreateType::AmpClientScrollY,
                                "ampClientTimestamp" => CreateType::AmpClientTimestamp,
                                "ampClientTimezone" => CreateType::AmpClientTimezone,
                                "ampGtmEvent" => CreateType::AmpGtmEvent,
                                "ampPageDownloadTime" => CreateType::AmpPageDownloadTime,
                                "ampPageLoadTime" => CreateType::AmpPageLoadTime,
                                "ampPageViewId" => CreateType::AmpPageViewId,
                                "ampReferrer" => CreateType::AmpReferrer,
                                "ampTitle" => CreateType::AmpTitle,
                                "ampTotalEngagedTime" => CreateType::AmpTotalEngagedTime,
                                "appId" => CreateType::AppId,
                                "appName" => CreateType::AppName,
                                "appVersionCode" => CreateType::AppVersionCode,
                                "appVersionName" => CreateType::AppVersionName,
                                "builtInVariableTypeUnspecified" => {
                                    CreateType::BuiltInVariableTypeUnspecified
                                }
                                "clickClasses" => CreateType::ClickClasses,
                                "clickElement" => CreateType::ClickElement,
                                "clickId" => CreateType::ClickId,
                                "clickTarget" => CreateType::ClickTarget,
                                "clickText" => CreateType::ClickText,
                                "clickUrl" => CreateType::ClickUrl,
                                "containerId" => CreateType::ContainerId,
                                "containerVersion" => CreateType::ContainerVersion,
                                "debugMode" => CreateType::DebugMode,
                                "deviceName" => CreateType::DeviceName,
                                "elementVisibilityFirstTime" => {
                                    CreateType::ElementVisibilityFirstTime
                                }
                                "elementVisibilityRatio" => CreateType::ElementVisibilityRatio,
                                "elementVisibilityRecentTime" => {
                                    CreateType::ElementVisibilityRecentTime
                                }
                                "elementVisibilityTime" => CreateType::ElementVisibilityTime,
                                "environmentName" => CreateType::EnvironmentName,
                                "errorLine" => CreateType::ErrorLine,
                                "errorMessage" => CreateType::ErrorMessage,
                                "errorUrl" => CreateType::ErrorUrl,
                                "event" => CreateType::Event,
                                "eventName" => CreateType::EventName,
                                "firebaseEventParameterCampaign" => {
                                    CreateType::FirebaseEventParameterCampaign
                                }
                                "firebaseEventParameterCampaignAclid" => {
                                    CreateType::FirebaseEventParameterCampaignAclid
                                }
                                "firebaseEventParameterCampaignAnid" => {
                                    CreateType::FirebaseEventParameterCampaignAnid
                                }
                                "firebaseEventParameterCampaignClickTimestamp" => {
                                    CreateType::FirebaseEventParameterCampaignClickTimestamp
                                }
                                "firebaseEventParameterCampaignContent" => {
                                    CreateType::FirebaseEventParameterCampaignContent
                                }
                                "firebaseEventParameterCampaignCp1" => {
                                    CreateType::FirebaseEventParameterCampaignCp1
                                }
                                "firebaseEventParameterCampaignGclid" => {
                                    CreateType::FirebaseEventParameterCampaignGclid
                                }
                                "firebaseEventParameterCampaignSource" => {
                                    CreateType::FirebaseEventParameterCampaignSource
                                }
                                "firebaseEventParameterCampaignTerm" => {
                                    CreateType::FirebaseEventParameterCampaignTerm
                                }
                                "firebaseEventParameterCurrency" => {
                                    CreateType::FirebaseEventParameterCurrency
                                }
                                "firebaseEventParameterDynamicLinkAcceptTime" => {
                                    CreateType::FirebaseEventParameterDynamicLinkAcceptTime
                                }
                                "firebaseEventParameterDynamicLinkLinkid" => {
                                    CreateType::FirebaseEventParameterDynamicLinkLinkid
                                }
                                "firebaseEventParameterNotificationMessageDeviceTime" => {
                                    CreateType::FirebaseEventParameterNotificationMessageDeviceTime
                                }
                                "firebaseEventParameterNotificationMessageId" => {
                                    CreateType::FirebaseEventParameterNotificationMessageId
                                }
                                "firebaseEventParameterNotificationMessageName" => {
                                    CreateType::FirebaseEventParameterNotificationMessageName
                                }
                                "firebaseEventParameterNotificationMessageTime" => {
                                    CreateType::FirebaseEventParameterNotificationMessageTime
                                }
                                "firebaseEventParameterNotificationTopic" => {
                                    CreateType::FirebaseEventParameterNotificationTopic
                                }
                                "firebaseEventParameterPreviousAppVersion" => {
                                    CreateType::FirebaseEventParameterPreviousAppVersion
                                }
                                "firebaseEventParameterPreviousOsVersion" => {
                                    CreateType::FirebaseEventParameterPreviousOsVersion
                                }
                                "firebaseEventParameterPrice" => {
                                    CreateType::FirebaseEventParameterPrice
                                }
                                "firebaseEventParameterProductId" => {
                                    CreateType::FirebaseEventParameterProductId
                                }
                                "firebaseEventParameterQuantity" => {
                                    CreateType::FirebaseEventParameterQuantity
                                }
                                "firebaseEventParameterValue" => {
                                    CreateType::FirebaseEventParameterValue
                                }
                                "formClasses" => CreateType::FormClasses,
                                "formElement" => CreateType::FormElement,
                                "formId" => CreateType::FormId,
                                "formTarget" => CreateType::FormTarget,
                                "formText" => CreateType::FormText,
                                "formUrl" => CreateType::FormUrl,
                                "historySource" => CreateType::HistorySource,
                                "htmlId" => CreateType::HtmlId,
                                "language" => CreateType::Language,
                                "newHistoryFragment" => CreateType::NewHistoryFragment,
                                "newHistoryState" => CreateType::NewHistoryState,
                                "newHistoryUrl" => CreateType::NewHistoryUrl,
                                "oldHistoryFragment" => CreateType::OldHistoryFragment,
                                "oldHistoryState" => CreateType::OldHistoryState,
                                "oldHistoryUrl" => CreateType::OldHistoryUrl,
                                "osVersion" => CreateType::OsVersion,
                                "pageHostname" => CreateType::PageHostname,
                                "pagePath" => CreateType::PagePath,
                                "pageUrl" => CreateType::PageUrl,
                                "platform" => CreateType::Platform,
                                "randomNumber" => CreateType::RandomNumber,
                                "referrer" => CreateType::Referrer,
                                "resolution" => CreateType::Resolution,
                                "scrollDepthDirection" => CreateType::ScrollDepthDirection,
                                "scrollDepthThreshold" => CreateType::ScrollDepthThreshold,
                                "scrollDepthUnits" => CreateType::ScrollDepthUnits,
                                "sdkVersion" => CreateType::SdkVersion,
                                "videoCurrentTime" => CreateType::VideoCurrentTime,
                                "videoDuration" => CreateType::VideoDuration,
                                "videoPercent" => CreateType::VideoPercent,
                                "videoProvider" => CreateType::VideoProvider,
                                "videoStatus" => CreateType::VideoStatus,
                                "videoTitle" => CreateType::VideoTitle,
                                "videoUrl" => CreateType::VideoUrl,
                                "videoVisible" => CreateType::VideoVisible,
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
                    pub enum DeleteType {
                        AdvertiserId,
                        AdvertisingTrackingEnabled,
                        AmpBrowserLanguage,
                        AmpCanonicalHost,
                        AmpCanonicalPath,
                        AmpCanonicalUrl,
                        AmpClientId,
                        AmpClientMaxScrollX,
                        AmpClientMaxScrollY,
                        AmpClientScreenHeight,
                        AmpClientScreenWidth,
                        AmpClientScrollX,
                        AmpClientScrollY,
                        AmpClientTimestamp,
                        AmpClientTimezone,
                        AmpGtmEvent,
                        AmpPageDownloadTime,
                        AmpPageLoadTime,
                        AmpPageViewId,
                        AmpReferrer,
                        AmpTitle,
                        AmpTotalEngagedTime,
                        AppId,
                        AppName,
                        AppVersionCode,
                        AppVersionName,
                        BuiltInVariableTypeUnspecified,
                        ClickClasses,
                        ClickElement,
                        ClickId,
                        ClickTarget,
                        ClickText,
                        ClickUrl,
                        ContainerId,
                        ContainerVersion,
                        DebugMode,
                        DeviceName,
                        ElementVisibilityFirstTime,
                        ElementVisibilityRatio,
                        ElementVisibilityRecentTime,
                        ElementVisibilityTime,
                        EnvironmentName,
                        ErrorLine,
                        ErrorMessage,
                        ErrorUrl,
                        Event,
                        EventName,
                        FirebaseEventParameterCampaign,
                        FirebaseEventParameterCampaignAclid,
                        FirebaseEventParameterCampaignAnid,
                        FirebaseEventParameterCampaignClickTimestamp,
                        FirebaseEventParameterCampaignContent,
                        FirebaseEventParameterCampaignCp1,
                        FirebaseEventParameterCampaignGclid,
                        FirebaseEventParameterCampaignSource,
                        FirebaseEventParameterCampaignTerm,
                        FirebaseEventParameterCurrency,
                        FirebaseEventParameterDynamicLinkAcceptTime,
                        FirebaseEventParameterDynamicLinkLinkid,
                        FirebaseEventParameterNotificationMessageDeviceTime,
                        FirebaseEventParameterNotificationMessageId,
                        FirebaseEventParameterNotificationMessageName,
                        FirebaseEventParameterNotificationMessageTime,
                        FirebaseEventParameterNotificationTopic,
                        FirebaseEventParameterPreviousAppVersion,
                        FirebaseEventParameterPreviousOsVersion,
                        FirebaseEventParameterPrice,
                        FirebaseEventParameterProductId,
                        FirebaseEventParameterQuantity,
                        FirebaseEventParameterValue,
                        FormClasses,
                        FormElement,
                        FormId,
                        FormTarget,
                        FormText,
                        FormUrl,
                        HistorySource,
                        HtmlId,
                        Language,
                        NewHistoryFragment,
                        NewHistoryState,
                        NewHistoryUrl,
                        OldHistoryFragment,
                        OldHistoryState,
                        OldHistoryUrl,
                        OsVersion,
                        PageHostname,
                        PagePath,
                        PageUrl,
                        Platform,
                        RandomNumber,
                        Referrer,
                        Resolution,
                        ScrollDepthDirection,
                        ScrollDepthThreshold,
                        ScrollDepthUnits,
                        SdkVersion,
                        VideoCurrentTime,
                        VideoDuration,
                        VideoPercent,
                        VideoProvider,
                        VideoStatus,
                        VideoTitle,
                        VideoUrl,
                        VideoVisible,
                    }
                    impl DeleteType {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                DeleteType::AdvertiserId => "advertiserId",
                                DeleteType::AdvertisingTrackingEnabled => {
                                    "advertisingTrackingEnabled"
                                }
                                DeleteType::AmpBrowserLanguage => "ampBrowserLanguage",
                                DeleteType::AmpCanonicalHost => "ampCanonicalHost",
                                DeleteType::AmpCanonicalPath => "ampCanonicalPath",
                                DeleteType::AmpCanonicalUrl => "ampCanonicalUrl",
                                DeleteType::AmpClientId => "ampClientId",
                                DeleteType::AmpClientMaxScrollX => "ampClientMaxScrollX",
                                DeleteType::AmpClientMaxScrollY => "ampClientMaxScrollY",
                                DeleteType::AmpClientScreenHeight => "ampClientScreenHeight",
                                DeleteType::AmpClientScreenWidth => "ampClientScreenWidth",
                                DeleteType::AmpClientScrollX => "ampClientScrollX",
                                DeleteType::AmpClientScrollY => "ampClientScrollY",
                                DeleteType::AmpClientTimestamp => "ampClientTimestamp",
                                DeleteType::AmpClientTimezone => "ampClientTimezone",
                                DeleteType::AmpGtmEvent => "ampGtmEvent",
                                DeleteType::AmpPageDownloadTime => "ampPageDownloadTime",
                                DeleteType::AmpPageLoadTime => "ampPageLoadTime",
                                DeleteType::AmpPageViewId => "ampPageViewId",
                                DeleteType::AmpReferrer => "ampReferrer",
                                DeleteType::AmpTitle => "ampTitle",
                                DeleteType::AmpTotalEngagedTime => "ampTotalEngagedTime",
                                DeleteType::AppId => "appId",
                                DeleteType::AppName => "appName",
                                DeleteType::AppVersionCode => "appVersionCode",
                                DeleteType::AppVersionName => "appVersionName",
                                DeleteType::BuiltInVariableTypeUnspecified => {
                                    "builtInVariableTypeUnspecified"
                                }
                                DeleteType::ClickClasses => "clickClasses",
                                DeleteType::ClickElement => "clickElement",
                                DeleteType::ClickId => "clickId",
                                DeleteType::ClickTarget => "clickTarget",
                                DeleteType::ClickText => "clickText",
                                DeleteType::ClickUrl => "clickUrl",
                                DeleteType::ContainerId => "containerId",
                                DeleteType::ContainerVersion => "containerVersion",
                                DeleteType::DebugMode => "debugMode",
                                DeleteType::DeviceName => "deviceName",
                                DeleteType::ElementVisibilityFirstTime => {
                                    "elementVisibilityFirstTime"
                                }
                                DeleteType::ElementVisibilityRatio => "elementVisibilityRatio",
                                DeleteType::ElementVisibilityRecentTime => {
                                    "elementVisibilityRecentTime"
                                }
                                DeleteType::ElementVisibilityTime => "elementVisibilityTime",
                                DeleteType::EnvironmentName => "environmentName",
                                DeleteType::ErrorLine => "errorLine",
                                DeleteType::ErrorMessage => "errorMessage",
                                DeleteType::ErrorUrl => "errorUrl",
                                DeleteType::Event => "event",
                                DeleteType::EventName => "eventName",
                                DeleteType::FirebaseEventParameterCampaign => {
                                    "firebaseEventParameterCampaign"
                                }
                                DeleteType::FirebaseEventParameterCampaignAclid => {
                                    "firebaseEventParameterCampaignAclid"
                                }
                                DeleteType::FirebaseEventParameterCampaignAnid => {
                                    "firebaseEventParameterCampaignAnid"
                                }
                                DeleteType::FirebaseEventParameterCampaignClickTimestamp => {
                                    "firebaseEventParameterCampaignClickTimestamp"
                                }
                                DeleteType::FirebaseEventParameterCampaignContent => {
                                    "firebaseEventParameterCampaignContent"
                                }
                                DeleteType::FirebaseEventParameterCampaignCp1 => {
                                    "firebaseEventParameterCampaignCp1"
                                }
                                DeleteType::FirebaseEventParameterCampaignGclid => {
                                    "firebaseEventParameterCampaignGclid"
                                }
                                DeleteType::FirebaseEventParameterCampaignSource => {
                                    "firebaseEventParameterCampaignSource"
                                }
                                DeleteType::FirebaseEventParameterCampaignTerm => {
                                    "firebaseEventParameterCampaignTerm"
                                }
                                DeleteType::FirebaseEventParameterCurrency => {
                                    "firebaseEventParameterCurrency"
                                }
                                DeleteType::FirebaseEventParameterDynamicLinkAcceptTime => {
                                    "firebaseEventParameterDynamicLinkAcceptTime"
                                }
                                DeleteType::FirebaseEventParameterDynamicLinkLinkid => {
                                    "firebaseEventParameterDynamicLinkLinkid"
                                }
                                DeleteType::FirebaseEventParameterNotificationMessageDeviceTime => {
                                    "firebaseEventParameterNotificationMessageDeviceTime"
                                }
                                DeleteType::FirebaseEventParameterNotificationMessageId => {
                                    "firebaseEventParameterNotificationMessageId"
                                }
                                DeleteType::FirebaseEventParameterNotificationMessageName => {
                                    "firebaseEventParameterNotificationMessageName"
                                }
                                DeleteType::FirebaseEventParameterNotificationMessageTime => {
                                    "firebaseEventParameterNotificationMessageTime"
                                }
                                DeleteType::FirebaseEventParameterNotificationTopic => {
                                    "firebaseEventParameterNotificationTopic"
                                }
                                DeleteType::FirebaseEventParameterPreviousAppVersion => {
                                    "firebaseEventParameterPreviousAppVersion"
                                }
                                DeleteType::FirebaseEventParameterPreviousOsVersion => {
                                    "firebaseEventParameterPreviousOsVersion"
                                }
                                DeleteType::FirebaseEventParameterPrice => {
                                    "firebaseEventParameterPrice"
                                }
                                DeleteType::FirebaseEventParameterProductId => {
                                    "firebaseEventParameterProductId"
                                }
                                DeleteType::FirebaseEventParameterQuantity => {
                                    "firebaseEventParameterQuantity"
                                }
                                DeleteType::FirebaseEventParameterValue => {
                                    "firebaseEventParameterValue"
                                }
                                DeleteType::FormClasses => "formClasses",
                                DeleteType::FormElement => "formElement",
                                DeleteType::FormId => "formId",
                                DeleteType::FormTarget => "formTarget",
                                DeleteType::FormText => "formText",
                                DeleteType::FormUrl => "formUrl",
                                DeleteType::HistorySource => "historySource",
                                DeleteType::HtmlId => "htmlId",
                                DeleteType::Language => "language",
                                DeleteType::NewHistoryFragment => "newHistoryFragment",
                                DeleteType::NewHistoryState => "newHistoryState",
                                DeleteType::NewHistoryUrl => "newHistoryUrl",
                                DeleteType::OldHistoryFragment => "oldHistoryFragment",
                                DeleteType::OldHistoryState => "oldHistoryState",
                                DeleteType::OldHistoryUrl => "oldHistoryUrl",
                                DeleteType::OsVersion => "osVersion",
                                DeleteType::PageHostname => "pageHostname",
                                DeleteType::PagePath => "pagePath",
                                DeleteType::PageUrl => "pageUrl",
                                DeleteType::Platform => "platform",
                                DeleteType::RandomNumber => "randomNumber",
                                DeleteType::Referrer => "referrer",
                                DeleteType::Resolution => "resolution",
                                DeleteType::ScrollDepthDirection => "scrollDepthDirection",
                                DeleteType::ScrollDepthThreshold => "scrollDepthThreshold",
                                DeleteType::ScrollDepthUnits => "scrollDepthUnits",
                                DeleteType::SdkVersion => "sdkVersion",
                                DeleteType::VideoCurrentTime => "videoCurrentTime",
                                DeleteType::VideoDuration => "videoDuration",
                                DeleteType::VideoPercent => "videoPercent",
                                DeleteType::VideoProvider => "videoProvider",
                                DeleteType::VideoStatus => "videoStatus",
                                DeleteType::VideoTitle => "videoTitle",
                                DeleteType::VideoUrl => "videoUrl",
                                DeleteType::VideoVisible => "videoVisible",
                            }
                        }
                    }
                    impl ::std::fmt::Display for DeleteType {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for DeleteType {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for DeleteType {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "advertiserId" => DeleteType::AdvertiserId,
                                "advertisingTrackingEnabled" => {
                                    DeleteType::AdvertisingTrackingEnabled
                                }
                                "ampBrowserLanguage" => DeleteType::AmpBrowserLanguage,
                                "ampCanonicalHost" => DeleteType::AmpCanonicalHost,
                                "ampCanonicalPath" => DeleteType::AmpCanonicalPath,
                                "ampCanonicalUrl" => DeleteType::AmpCanonicalUrl,
                                "ampClientId" => DeleteType::AmpClientId,
                                "ampClientMaxScrollX" => DeleteType::AmpClientMaxScrollX,
                                "ampClientMaxScrollY" => DeleteType::AmpClientMaxScrollY,
                                "ampClientScreenHeight" => DeleteType::AmpClientScreenHeight,
                                "ampClientScreenWidth" => DeleteType::AmpClientScreenWidth,
                                "ampClientScrollX" => DeleteType::AmpClientScrollX,
                                "ampClientScrollY" => DeleteType::AmpClientScrollY,
                                "ampClientTimestamp" => DeleteType::AmpClientTimestamp,
                                "ampClientTimezone" => DeleteType::AmpClientTimezone,
                                "ampGtmEvent" => DeleteType::AmpGtmEvent,
                                "ampPageDownloadTime" => DeleteType::AmpPageDownloadTime,
                                "ampPageLoadTime" => DeleteType::AmpPageLoadTime,
                                "ampPageViewId" => DeleteType::AmpPageViewId,
                                "ampReferrer" => DeleteType::AmpReferrer,
                                "ampTitle" => DeleteType::AmpTitle,
                                "ampTotalEngagedTime" => DeleteType::AmpTotalEngagedTime,
                                "appId" => DeleteType::AppId,
                                "appName" => DeleteType::AppName,
                                "appVersionCode" => DeleteType::AppVersionCode,
                                "appVersionName" => DeleteType::AppVersionName,
                                "builtInVariableTypeUnspecified" => {
                                    DeleteType::BuiltInVariableTypeUnspecified
                                }
                                "clickClasses" => DeleteType::ClickClasses,
                                "clickElement" => DeleteType::ClickElement,
                                "clickId" => DeleteType::ClickId,
                                "clickTarget" => DeleteType::ClickTarget,
                                "clickText" => DeleteType::ClickText,
                                "clickUrl" => DeleteType::ClickUrl,
                                "containerId" => DeleteType::ContainerId,
                                "containerVersion" => DeleteType::ContainerVersion,
                                "debugMode" => DeleteType::DebugMode,
                                "deviceName" => DeleteType::DeviceName,
                                "elementVisibilityFirstTime" => {
                                    DeleteType::ElementVisibilityFirstTime
                                }
                                "elementVisibilityRatio" => DeleteType::ElementVisibilityRatio,
                                "elementVisibilityRecentTime" => {
                                    DeleteType::ElementVisibilityRecentTime
                                }
                                "elementVisibilityTime" => DeleteType::ElementVisibilityTime,
                                "environmentName" => DeleteType::EnvironmentName,
                                "errorLine" => DeleteType::ErrorLine,
                                "errorMessage" => DeleteType::ErrorMessage,
                                "errorUrl" => DeleteType::ErrorUrl,
                                "event" => DeleteType::Event,
                                "eventName" => DeleteType::EventName,
                                "firebaseEventParameterCampaign" => {
                                    DeleteType::FirebaseEventParameterCampaign
                                }
                                "firebaseEventParameterCampaignAclid" => {
                                    DeleteType::FirebaseEventParameterCampaignAclid
                                }
                                "firebaseEventParameterCampaignAnid" => {
                                    DeleteType::FirebaseEventParameterCampaignAnid
                                }
                                "firebaseEventParameterCampaignClickTimestamp" => {
                                    DeleteType::FirebaseEventParameterCampaignClickTimestamp
                                }
                                "firebaseEventParameterCampaignContent" => {
                                    DeleteType::FirebaseEventParameterCampaignContent
                                }
                                "firebaseEventParameterCampaignCp1" => {
                                    DeleteType::FirebaseEventParameterCampaignCp1
                                }
                                "firebaseEventParameterCampaignGclid" => {
                                    DeleteType::FirebaseEventParameterCampaignGclid
                                }
                                "firebaseEventParameterCampaignSource" => {
                                    DeleteType::FirebaseEventParameterCampaignSource
                                }
                                "firebaseEventParameterCampaignTerm" => {
                                    DeleteType::FirebaseEventParameterCampaignTerm
                                }
                                "firebaseEventParameterCurrency" => {
                                    DeleteType::FirebaseEventParameterCurrency
                                }
                                "firebaseEventParameterDynamicLinkAcceptTime" => {
                                    DeleteType::FirebaseEventParameterDynamicLinkAcceptTime
                                }
                                "firebaseEventParameterDynamicLinkLinkid" => {
                                    DeleteType::FirebaseEventParameterDynamicLinkLinkid
                                }
                                "firebaseEventParameterNotificationMessageDeviceTime" => {
                                    DeleteType::FirebaseEventParameterNotificationMessageDeviceTime
                                }
                                "firebaseEventParameterNotificationMessageId" => {
                                    DeleteType::FirebaseEventParameterNotificationMessageId
                                }
                                "firebaseEventParameterNotificationMessageName" => {
                                    DeleteType::FirebaseEventParameterNotificationMessageName
                                }
                                "firebaseEventParameterNotificationMessageTime" => {
                                    DeleteType::FirebaseEventParameterNotificationMessageTime
                                }
                                "firebaseEventParameterNotificationTopic" => {
                                    DeleteType::FirebaseEventParameterNotificationTopic
                                }
                                "firebaseEventParameterPreviousAppVersion" => {
                                    DeleteType::FirebaseEventParameterPreviousAppVersion
                                }
                                "firebaseEventParameterPreviousOsVersion" => {
                                    DeleteType::FirebaseEventParameterPreviousOsVersion
                                }
                                "firebaseEventParameterPrice" => {
                                    DeleteType::FirebaseEventParameterPrice
                                }
                                "firebaseEventParameterProductId" => {
                                    DeleteType::FirebaseEventParameterProductId
                                }
                                "firebaseEventParameterQuantity" => {
                                    DeleteType::FirebaseEventParameterQuantity
                                }
                                "firebaseEventParameterValue" => {
                                    DeleteType::FirebaseEventParameterValue
                                }
                                "formClasses" => DeleteType::FormClasses,
                                "formElement" => DeleteType::FormElement,
                                "formId" => DeleteType::FormId,
                                "formTarget" => DeleteType::FormTarget,
                                "formText" => DeleteType::FormText,
                                "formUrl" => DeleteType::FormUrl,
                                "historySource" => DeleteType::HistorySource,
                                "htmlId" => DeleteType::HtmlId,
                                "language" => DeleteType::Language,
                                "newHistoryFragment" => DeleteType::NewHistoryFragment,
                                "newHistoryState" => DeleteType::NewHistoryState,
                                "newHistoryUrl" => DeleteType::NewHistoryUrl,
                                "oldHistoryFragment" => DeleteType::OldHistoryFragment,
                                "oldHistoryState" => DeleteType::OldHistoryState,
                                "oldHistoryUrl" => DeleteType::OldHistoryUrl,
                                "osVersion" => DeleteType::OsVersion,
                                "pageHostname" => DeleteType::PageHostname,
                                "pagePath" => DeleteType::PagePath,
                                "pageUrl" => DeleteType::PageUrl,
                                "platform" => DeleteType::Platform,
                                "randomNumber" => DeleteType::RandomNumber,
                                "referrer" => DeleteType::Referrer,
                                "resolution" => DeleteType::Resolution,
                                "scrollDepthDirection" => DeleteType::ScrollDepthDirection,
                                "scrollDepthThreshold" => DeleteType::ScrollDepthThreshold,
                                "scrollDepthUnits" => DeleteType::ScrollDepthUnits,
                                "sdkVersion" => DeleteType::SdkVersion,
                                "videoCurrentTime" => DeleteType::VideoCurrentTime,
                                "videoDuration" => DeleteType::VideoDuration,
                                "videoPercent" => DeleteType::VideoPercent,
                                "videoProvider" => DeleteType::VideoProvider,
                                "videoStatus" => DeleteType::VideoStatus,
                                "videoTitle" => DeleteType::VideoTitle,
                                "videoUrl" => DeleteType::VideoUrl,
                                "videoVisible" => DeleteType::VideoVisible,
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
                    pub enum RevertType {
                        AdvertiserId,
                        AdvertisingTrackingEnabled,
                        AmpBrowserLanguage,
                        AmpCanonicalHost,
                        AmpCanonicalPath,
                        AmpCanonicalUrl,
                        AmpClientId,
                        AmpClientMaxScrollX,
                        AmpClientMaxScrollY,
                        AmpClientScreenHeight,
                        AmpClientScreenWidth,
                        AmpClientScrollX,
                        AmpClientScrollY,
                        AmpClientTimestamp,
                        AmpClientTimezone,
                        AmpGtmEvent,
                        AmpPageDownloadTime,
                        AmpPageLoadTime,
                        AmpPageViewId,
                        AmpReferrer,
                        AmpTitle,
                        AmpTotalEngagedTime,
                        AppId,
                        AppName,
                        AppVersionCode,
                        AppVersionName,
                        BuiltInVariableTypeUnspecified,
                        ClickClasses,
                        ClickElement,
                        ClickId,
                        ClickTarget,
                        ClickText,
                        ClickUrl,
                        ContainerId,
                        ContainerVersion,
                        DebugMode,
                        DeviceName,
                        ElementVisibilityFirstTime,
                        ElementVisibilityRatio,
                        ElementVisibilityRecentTime,
                        ElementVisibilityTime,
                        EnvironmentName,
                        ErrorLine,
                        ErrorMessage,
                        ErrorUrl,
                        Event,
                        EventName,
                        FirebaseEventParameterCampaign,
                        FirebaseEventParameterCampaignAclid,
                        FirebaseEventParameterCampaignAnid,
                        FirebaseEventParameterCampaignClickTimestamp,
                        FirebaseEventParameterCampaignContent,
                        FirebaseEventParameterCampaignCp1,
                        FirebaseEventParameterCampaignGclid,
                        FirebaseEventParameterCampaignSource,
                        FirebaseEventParameterCampaignTerm,
                        FirebaseEventParameterCurrency,
                        FirebaseEventParameterDynamicLinkAcceptTime,
                        FirebaseEventParameterDynamicLinkLinkid,
                        FirebaseEventParameterNotificationMessageDeviceTime,
                        FirebaseEventParameterNotificationMessageId,
                        FirebaseEventParameterNotificationMessageName,
                        FirebaseEventParameterNotificationMessageTime,
                        FirebaseEventParameterNotificationTopic,
                        FirebaseEventParameterPreviousAppVersion,
                        FirebaseEventParameterPreviousOsVersion,
                        FirebaseEventParameterPrice,
                        FirebaseEventParameterProductId,
                        FirebaseEventParameterQuantity,
                        FirebaseEventParameterValue,
                        FormClasses,
                        FormElement,
                        FormId,
                        FormTarget,
                        FormText,
                        FormUrl,
                        HistorySource,
                        HtmlId,
                        Language,
                        NewHistoryFragment,
                        NewHistoryState,
                        NewHistoryUrl,
                        OldHistoryFragment,
                        OldHistoryState,
                        OldHistoryUrl,
                        OsVersion,
                        PageHostname,
                        PagePath,
                        PageUrl,
                        Platform,
                        RandomNumber,
                        Referrer,
                        Resolution,
                        ScrollDepthDirection,
                        ScrollDepthThreshold,
                        ScrollDepthUnits,
                        SdkVersion,
                        VideoCurrentTime,
                        VideoDuration,
                        VideoPercent,
                        VideoProvider,
                        VideoStatus,
                        VideoTitle,
                        VideoUrl,
                        VideoVisible,
                    }
                    impl RevertType {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                RevertType::AdvertiserId => "advertiserId",
                                RevertType::AdvertisingTrackingEnabled => {
                                    "advertisingTrackingEnabled"
                                }
                                RevertType::AmpBrowserLanguage => "ampBrowserLanguage",
                                RevertType::AmpCanonicalHost => "ampCanonicalHost",
                                RevertType::AmpCanonicalPath => "ampCanonicalPath",
                                RevertType::AmpCanonicalUrl => "ampCanonicalUrl",
                                RevertType::AmpClientId => "ampClientId",
                                RevertType::AmpClientMaxScrollX => "ampClientMaxScrollX",
                                RevertType::AmpClientMaxScrollY => "ampClientMaxScrollY",
                                RevertType::AmpClientScreenHeight => "ampClientScreenHeight",
                                RevertType::AmpClientScreenWidth => "ampClientScreenWidth",
                                RevertType::AmpClientScrollX => "ampClientScrollX",
                                RevertType::AmpClientScrollY => "ampClientScrollY",
                                RevertType::AmpClientTimestamp => "ampClientTimestamp",
                                RevertType::AmpClientTimezone => "ampClientTimezone",
                                RevertType::AmpGtmEvent => "ampGtmEvent",
                                RevertType::AmpPageDownloadTime => "ampPageDownloadTime",
                                RevertType::AmpPageLoadTime => "ampPageLoadTime",
                                RevertType::AmpPageViewId => "ampPageViewId",
                                RevertType::AmpReferrer => "ampReferrer",
                                RevertType::AmpTitle => "ampTitle",
                                RevertType::AmpTotalEngagedTime => "ampTotalEngagedTime",
                                RevertType::AppId => "appId",
                                RevertType::AppName => "appName",
                                RevertType::AppVersionCode => "appVersionCode",
                                RevertType::AppVersionName => "appVersionName",
                                RevertType::BuiltInVariableTypeUnspecified => {
                                    "builtInVariableTypeUnspecified"
                                }
                                RevertType::ClickClasses => "clickClasses",
                                RevertType::ClickElement => "clickElement",
                                RevertType::ClickId => "clickId",
                                RevertType::ClickTarget => "clickTarget",
                                RevertType::ClickText => "clickText",
                                RevertType::ClickUrl => "clickUrl",
                                RevertType::ContainerId => "containerId",
                                RevertType::ContainerVersion => "containerVersion",
                                RevertType::DebugMode => "debugMode",
                                RevertType::DeviceName => "deviceName",
                                RevertType::ElementVisibilityFirstTime => {
                                    "elementVisibilityFirstTime"
                                }
                                RevertType::ElementVisibilityRatio => "elementVisibilityRatio",
                                RevertType::ElementVisibilityRecentTime => {
                                    "elementVisibilityRecentTime"
                                }
                                RevertType::ElementVisibilityTime => "elementVisibilityTime",
                                RevertType::EnvironmentName => "environmentName",
                                RevertType::ErrorLine => "errorLine",
                                RevertType::ErrorMessage => "errorMessage",
                                RevertType::ErrorUrl => "errorUrl",
                                RevertType::Event => "event",
                                RevertType::EventName => "eventName",
                                RevertType::FirebaseEventParameterCampaign => {
                                    "firebaseEventParameterCampaign"
                                }
                                RevertType::FirebaseEventParameterCampaignAclid => {
                                    "firebaseEventParameterCampaignAclid"
                                }
                                RevertType::FirebaseEventParameterCampaignAnid => {
                                    "firebaseEventParameterCampaignAnid"
                                }
                                RevertType::FirebaseEventParameterCampaignClickTimestamp => {
                                    "firebaseEventParameterCampaignClickTimestamp"
                                }
                                RevertType::FirebaseEventParameterCampaignContent => {
                                    "firebaseEventParameterCampaignContent"
                                }
                                RevertType::FirebaseEventParameterCampaignCp1 => {
                                    "firebaseEventParameterCampaignCp1"
                                }
                                RevertType::FirebaseEventParameterCampaignGclid => {
                                    "firebaseEventParameterCampaignGclid"
                                }
                                RevertType::FirebaseEventParameterCampaignSource => {
                                    "firebaseEventParameterCampaignSource"
                                }
                                RevertType::FirebaseEventParameterCampaignTerm => {
                                    "firebaseEventParameterCampaignTerm"
                                }
                                RevertType::FirebaseEventParameterCurrency => {
                                    "firebaseEventParameterCurrency"
                                }
                                RevertType::FirebaseEventParameterDynamicLinkAcceptTime => {
                                    "firebaseEventParameterDynamicLinkAcceptTime"
                                }
                                RevertType::FirebaseEventParameterDynamicLinkLinkid => {
                                    "firebaseEventParameterDynamicLinkLinkid"
                                }
                                RevertType::FirebaseEventParameterNotificationMessageDeviceTime => {
                                    "firebaseEventParameterNotificationMessageDeviceTime"
                                }
                                RevertType::FirebaseEventParameterNotificationMessageId => {
                                    "firebaseEventParameterNotificationMessageId"
                                }
                                RevertType::FirebaseEventParameterNotificationMessageName => {
                                    "firebaseEventParameterNotificationMessageName"
                                }
                                RevertType::FirebaseEventParameterNotificationMessageTime => {
                                    "firebaseEventParameterNotificationMessageTime"
                                }
                                RevertType::FirebaseEventParameterNotificationTopic => {
                                    "firebaseEventParameterNotificationTopic"
                                }
                                RevertType::FirebaseEventParameterPreviousAppVersion => {
                                    "firebaseEventParameterPreviousAppVersion"
                                }
                                RevertType::FirebaseEventParameterPreviousOsVersion => {
                                    "firebaseEventParameterPreviousOsVersion"
                                }
                                RevertType::FirebaseEventParameterPrice => {
                                    "firebaseEventParameterPrice"
                                }
                                RevertType::FirebaseEventParameterProductId => {
                                    "firebaseEventParameterProductId"
                                }
                                RevertType::FirebaseEventParameterQuantity => {
                                    "firebaseEventParameterQuantity"
                                }
                                RevertType::FirebaseEventParameterValue => {
                                    "firebaseEventParameterValue"
                                }
                                RevertType::FormClasses => "formClasses",
                                RevertType::FormElement => "formElement",
                                RevertType::FormId => "formId",
                                RevertType::FormTarget => "formTarget",
                                RevertType::FormText => "formText",
                                RevertType::FormUrl => "formUrl",
                                RevertType::HistorySource => "historySource",
                                RevertType::HtmlId => "htmlId",
                                RevertType::Language => "language",
                                RevertType::NewHistoryFragment => "newHistoryFragment",
                                RevertType::NewHistoryState => "newHistoryState",
                                RevertType::NewHistoryUrl => "newHistoryUrl",
                                RevertType::OldHistoryFragment => "oldHistoryFragment",
                                RevertType::OldHistoryState => "oldHistoryState",
                                RevertType::OldHistoryUrl => "oldHistoryUrl",
                                RevertType::OsVersion => "osVersion",
                                RevertType::PageHostname => "pageHostname",
                                RevertType::PagePath => "pagePath",
                                RevertType::PageUrl => "pageUrl",
                                RevertType::Platform => "platform",
                                RevertType::RandomNumber => "randomNumber",
                                RevertType::Referrer => "referrer",
                                RevertType::Resolution => "resolution",
                                RevertType::ScrollDepthDirection => "scrollDepthDirection",
                                RevertType::ScrollDepthThreshold => "scrollDepthThreshold",
                                RevertType::ScrollDepthUnits => "scrollDepthUnits",
                                RevertType::SdkVersion => "sdkVersion",
                                RevertType::VideoCurrentTime => "videoCurrentTime",
                                RevertType::VideoDuration => "videoDuration",
                                RevertType::VideoPercent => "videoPercent",
                                RevertType::VideoProvider => "videoProvider",
                                RevertType::VideoStatus => "videoStatus",
                                RevertType::VideoTitle => "videoTitle",
                                RevertType::VideoUrl => "videoUrl",
                                RevertType::VideoVisible => "videoVisible",
                            }
                        }
                    }
                    impl ::std::fmt::Display for RevertType {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for RevertType {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for RevertType {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "advertiserId" => RevertType::AdvertiserId,
                                "advertisingTrackingEnabled" => {
                                    RevertType::AdvertisingTrackingEnabled
                                }
                                "ampBrowserLanguage" => RevertType::AmpBrowserLanguage,
                                "ampCanonicalHost" => RevertType::AmpCanonicalHost,
                                "ampCanonicalPath" => RevertType::AmpCanonicalPath,
                                "ampCanonicalUrl" => RevertType::AmpCanonicalUrl,
                                "ampClientId" => RevertType::AmpClientId,
                                "ampClientMaxScrollX" => RevertType::AmpClientMaxScrollX,
                                "ampClientMaxScrollY" => RevertType::AmpClientMaxScrollY,
                                "ampClientScreenHeight" => RevertType::AmpClientScreenHeight,
                                "ampClientScreenWidth" => RevertType::AmpClientScreenWidth,
                                "ampClientScrollX" => RevertType::AmpClientScrollX,
                                "ampClientScrollY" => RevertType::AmpClientScrollY,
                                "ampClientTimestamp" => RevertType::AmpClientTimestamp,
                                "ampClientTimezone" => RevertType::AmpClientTimezone,
                                "ampGtmEvent" => RevertType::AmpGtmEvent,
                                "ampPageDownloadTime" => RevertType::AmpPageDownloadTime,
                                "ampPageLoadTime" => RevertType::AmpPageLoadTime,
                                "ampPageViewId" => RevertType::AmpPageViewId,
                                "ampReferrer" => RevertType::AmpReferrer,
                                "ampTitle" => RevertType::AmpTitle,
                                "ampTotalEngagedTime" => RevertType::AmpTotalEngagedTime,
                                "appId" => RevertType::AppId,
                                "appName" => RevertType::AppName,
                                "appVersionCode" => RevertType::AppVersionCode,
                                "appVersionName" => RevertType::AppVersionName,
                                "builtInVariableTypeUnspecified" => {
                                    RevertType::BuiltInVariableTypeUnspecified
                                }
                                "clickClasses" => RevertType::ClickClasses,
                                "clickElement" => RevertType::ClickElement,
                                "clickId" => RevertType::ClickId,
                                "clickTarget" => RevertType::ClickTarget,
                                "clickText" => RevertType::ClickText,
                                "clickUrl" => RevertType::ClickUrl,
                                "containerId" => RevertType::ContainerId,
                                "containerVersion" => RevertType::ContainerVersion,
                                "debugMode" => RevertType::DebugMode,
                                "deviceName" => RevertType::DeviceName,
                                "elementVisibilityFirstTime" => {
                                    RevertType::ElementVisibilityFirstTime
                                }
                                "elementVisibilityRatio" => RevertType::ElementVisibilityRatio,
                                "elementVisibilityRecentTime" => {
                                    RevertType::ElementVisibilityRecentTime
                                }
                                "elementVisibilityTime" => RevertType::ElementVisibilityTime,
                                "environmentName" => RevertType::EnvironmentName,
                                "errorLine" => RevertType::ErrorLine,
                                "errorMessage" => RevertType::ErrorMessage,
                                "errorUrl" => RevertType::ErrorUrl,
                                "event" => RevertType::Event,
                                "eventName" => RevertType::EventName,
                                "firebaseEventParameterCampaign" => {
                                    RevertType::FirebaseEventParameterCampaign
                                }
                                "firebaseEventParameterCampaignAclid" => {
                                    RevertType::FirebaseEventParameterCampaignAclid
                                }
                                "firebaseEventParameterCampaignAnid" => {
                                    RevertType::FirebaseEventParameterCampaignAnid
                                }
                                "firebaseEventParameterCampaignClickTimestamp" => {
                                    RevertType::FirebaseEventParameterCampaignClickTimestamp
                                }
                                "firebaseEventParameterCampaignContent" => {
                                    RevertType::FirebaseEventParameterCampaignContent
                                }
                                "firebaseEventParameterCampaignCp1" => {
                                    RevertType::FirebaseEventParameterCampaignCp1
                                }
                                "firebaseEventParameterCampaignGclid" => {
                                    RevertType::FirebaseEventParameterCampaignGclid
                                }
                                "firebaseEventParameterCampaignSource" => {
                                    RevertType::FirebaseEventParameterCampaignSource
                                }
                                "firebaseEventParameterCampaignTerm" => {
                                    RevertType::FirebaseEventParameterCampaignTerm
                                }
                                "firebaseEventParameterCurrency" => {
                                    RevertType::FirebaseEventParameterCurrency
                                }
                                "firebaseEventParameterDynamicLinkAcceptTime" => {
                                    RevertType::FirebaseEventParameterDynamicLinkAcceptTime
                                }
                                "firebaseEventParameterDynamicLinkLinkid" => {
                                    RevertType::FirebaseEventParameterDynamicLinkLinkid
                                }
                                "firebaseEventParameterNotificationMessageDeviceTime" => {
                                    RevertType::FirebaseEventParameterNotificationMessageDeviceTime
                                }
                                "firebaseEventParameterNotificationMessageId" => {
                                    RevertType::FirebaseEventParameterNotificationMessageId
                                }
                                "firebaseEventParameterNotificationMessageName" => {
                                    RevertType::FirebaseEventParameterNotificationMessageName
                                }
                                "firebaseEventParameterNotificationMessageTime" => {
                                    RevertType::FirebaseEventParameterNotificationMessageTime
                                }
                                "firebaseEventParameterNotificationTopic" => {
                                    RevertType::FirebaseEventParameterNotificationTopic
                                }
                                "firebaseEventParameterPreviousAppVersion" => {
                                    RevertType::FirebaseEventParameterPreviousAppVersion
                                }
                                "firebaseEventParameterPreviousOsVersion" => {
                                    RevertType::FirebaseEventParameterPreviousOsVersion
                                }
                                "firebaseEventParameterPrice" => {
                                    RevertType::FirebaseEventParameterPrice
                                }
                                "firebaseEventParameterProductId" => {
                                    RevertType::FirebaseEventParameterProductId
                                }
                                "firebaseEventParameterQuantity" => {
                                    RevertType::FirebaseEventParameterQuantity
                                }
                                "firebaseEventParameterValue" => {
                                    RevertType::FirebaseEventParameterValue
                                }
                                "formClasses" => RevertType::FormClasses,
                                "formElement" => RevertType::FormElement,
                                "formId" => RevertType::FormId,
                                "formTarget" => RevertType::FormTarget,
                                "formText" => RevertType::FormText,
                                "formUrl" => RevertType::FormUrl,
                                "historySource" => RevertType::HistorySource,
                                "htmlId" => RevertType::HtmlId,
                                "language" => RevertType::Language,
                                "newHistoryFragment" => RevertType::NewHistoryFragment,
                                "newHistoryState" => RevertType::NewHistoryState,
                                "newHistoryUrl" => RevertType::NewHistoryUrl,
                                "oldHistoryFragment" => RevertType::OldHistoryFragment,
                                "oldHistoryState" => RevertType::OldHistoryState,
                                "oldHistoryUrl" => RevertType::OldHistoryUrl,
                                "osVersion" => RevertType::OsVersion,
                                "pageHostname" => RevertType::PageHostname,
                                "pagePath" => RevertType::PagePath,
                                "pageUrl" => RevertType::PageUrl,
                                "platform" => RevertType::Platform,
                                "randomNumber" => RevertType::RandomNumber,
                                "referrer" => RevertType::Referrer,
                                "resolution" => RevertType::Resolution,
                                "scrollDepthDirection" => RevertType::ScrollDepthDirection,
                                "scrollDepthThreshold" => RevertType::ScrollDepthThreshold,
                                "scrollDepthUnits" => RevertType::ScrollDepthUnits,
                                "sdkVersion" => RevertType::SdkVersion,
                                "videoCurrentTime" => RevertType::VideoCurrentTime,
                                "videoDuration" => RevertType::VideoDuration,
                                "videoPercent" => RevertType::VideoPercent,
                                "videoProvider" => RevertType::VideoProvider,
                                "videoStatus" => RevertType::VideoStatus,
                                "videoTitle" => RevertType::VideoTitle,
                                "videoUrl" => RevertType::VideoUrl,
                                "videoVisible" => RevertType::VideoVisible,
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
                pub struct BuiltInVariablesActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> BuiltInVariablesActions<'a, A> {
                    #[doc = "Creates one or more GTM Built-In Variables."]
                    pub fn create(&self, parent: impl Into<String>) -> CreateRequestBuilder<A> {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            parent: parent.into(),
                            r#type: None,
                        }
                    }
                    #[doc = "Deletes one or more GTM Built-In Variables."]
                    pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            path: path.into(),
                            r#type: None,
                        }
                    }
                    #[doc = "Lists all the enabled Built-In Variables of a GTM Container."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Reverts changes to a GTM Built-In Variables in a GTM Workspace."]
                    pub fn revert(&self, path: impl Into<String>) -> RevertRequestBuilder<A> {
                        RevertRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            r#type: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    parent: String,
                    r#type: Option<crate::built_in_variables::params::CreateType>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                    #[doc = "The types of built-in variables to enable."]
                    pub fn r_type(
                        &mut self,
                        value: crate::built_in_variables::params::CreateType,
                    ) -> &mut Self {
                        self.r#type = Some(value);
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
                    ) -> Result<
                        crate::schemas::CreateBuiltInVariableResponse,
                        Box<dyn ::std::error::Error>,
                    > {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/built_in_variables");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("type", &self.r#type)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
                    r#type: Option<crate::built_in_variables::params::DeleteType>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                    #[doc = "The types of built-in variables to delete."]
                    pub fn r_type(
                        &mut self,
                        value: crate::built_in_variables::params::DeleteType,
                    ) -> &mut Self {
                        self.r#type = Some(value);
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("type", &self.r#type)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    parent: String,
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
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_built_in_variable<T>(
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
                                    #[serde(rename = "builtInVariable")]
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
                    ) -> Result<
                        crate::schemas::ListEnabledBuiltInVariablesResponse,
                        Box<dyn ::std::error::Error>,
                    > {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/built_in_variables");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                pub struct RevertRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    r#type: Option<crate::built_in_variables::params::RevertType>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> RevertRequestBuilder<'a, A> {
                    #[doc = "The type of built-in variable to revert."]
                    pub fn r_type(
                        &mut self,
                        value: crate::built_in_variables::params::RevertType,
                    ) -> &mut Self {
                        self.r#type = Some(value);
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
                    ) -> Result<
                        crate::schemas::RevertBuiltInVariableResponse,
                        Box<dyn ::std::error::Error>,
                    > {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str("/built_in_variables:revert");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("type", &self.r#type)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod folders {
                pub mod params {}
                pub struct FoldersActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> FoldersActions<'a, A> {
                    #[doc = "Creates a GTM Folder."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Folder,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a GTM Folder."]
                    pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "List all entities in a GTM Folder."]
                    pub fn entities(&self, path: impl Into<String>) -> EntitiesRequestBuilder<A> {
                        EntitiesRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Gets a GTM Folder."]
                    pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Lists all GTM Folders of a Container."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Moves entities to a GTM Folder."]
                    pub fn move_entities_to_folder(
                        &self,
                        request: crate::schemas::Folder,
                        path: impl Into<String>,
                    ) -> MoveEntitiesToFolderRequestBuilder<A> {
                        MoveEntitiesToFolderRequestBuilder {
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
                            path: path.into(),
                            tag_id: None,
                            trigger_id: None,
                            variable_id: None,
                        }
                    }
                    #[doc = "Reverts changes to a GTM Folder in a GTM Workspace."]
                    pub fn revert(&self, path: impl Into<String>) -> RevertRequestBuilder<A> {
                        RevertRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                    #[doc = "Updates a GTM Folder."]
                    pub fn update(
                        &self,
                        request: crate::schemas::Folder,
                        path: impl Into<String>,
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
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Folder,
                    parent: String,
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
                    ) -> Result<crate::schemas::Folder, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/folders");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct EntitiesRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    page_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> EntitiesRequestBuilder<'a, A> {
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_tag<T>(
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
                                    #[serde(rename = "tag")]
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
                    pub fn iter_trigger<T>(
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
                                    #[serde(rename = "trigger")]
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
                    pub fn iter_variable<T>(
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
                                    #[serde(rename = "variable")]
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
                    ) -> Result<crate::schemas::FolderEntities, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":entities");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::IterableMethod for EntitiesRequestBuilder<'a, A> {
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
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
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
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Folder, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                    parent: String,
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
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_folder<T>(
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
                                    #[serde(rename = "folder")]
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
                    ) -> Result<crate::schemas::ListFoldersResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/folders");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                pub struct MoveEntitiesToFolderRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Folder,
                    path: String,
                    tag_id: Option<String>,
                    trigger_id: Option<String>,
                    variable_id: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> MoveEntitiesToFolderRequestBuilder<'a, A> {
                    #[doc = "The tags to be moved to the folder."]
                    pub fn tag_id(&mut self, value: impl Into<String>) -> &mut Self {
                        self.tag_id = Some(value.into());
                        self
                    }
                    #[doc = "The triggers to be moved to the folder."]
                    pub fn trigger_id(&mut self, value: impl Into<String>) -> &mut Self {
                        self.trigger_id = Some(value.into());
                        self
                    }
                    #[doc = "The variables to be moved to the folder."]
                    pub fn variable_id(&mut self, value: impl Into<String>) -> &mut Self {
                        self.variable_id = Some(value.into());
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
                        let req = req.json(&self.request);
                        req.send()?.error_for_status()?;
                        Ok(())
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":move_entities_to_folder");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("tagId", &self.tag_id)]);
                        let req = req.query(&[("triggerId", &self.trigger_id)]);
                        let req = req.query(&[("variableId", &self.variable_id)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct RevertRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> RevertRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the tag in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::RevertFolderResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":revert");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    request: crate::schemas::Folder,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the folder in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::Folder, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod tags {
                pub mod params {}
                pub struct TagsActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> TagsActions<'a, A> {
                    #[doc = "Creates a GTM Tag."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Tag,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a GTM Tag."]
                    pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Gets a GTM Tag."]
                    pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Lists all GTM Tags of a Container."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Reverts changes to a GTM Tag in a GTM Workspace."]
                    pub fn revert(&self, path: impl Into<String>) -> RevertRequestBuilder<A> {
                        RevertRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                    #[doc = "Updates a GTM Tag."]
                    pub fn update(
                        &self,
                        request: crate::schemas::Tag,
                        path: impl Into<String>,
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
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Tag,
                    parent: String,
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
                    ) -> Result<crate::schemas::Tag, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/tags");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Tag, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                    parent: String,
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
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_tag<T>(
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
                                    #[serde(rename = "tag")]
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
                    ) -> Result<crate::schemas::ListTagsResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/tags");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                pub struct RevertRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> RevertRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of thetag in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::RevertTagResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":revert");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    request: crate::schemas::Tag,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the tag in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::Tag, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod templates {
                pub mod params {}
                pub struct TemplatesActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> TemplatesActions<'a, A> {
                    #[doc = "Creates a GTM Custom Template."]
                    pub fn create(
                        &self,
                        request: crate::schemas::CustomTemplate,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a GTM Template."]
                    pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Gets a GTM Template."]
                    pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Lists all GTM Templates of a GTM container workspace."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Reverts changes to a GTM Template in a GTM Workspace."]
                    pub fn revert(&self, path: impl Into<String>) -> RevertRequestBuilder<A> {
                        RevertRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                    #[doc = "Updates a GTM Template."]
                    pub fn update(
                        &self,
                        request: crate::schemas::CustomTemplate,
                        path: impl Into<String>,
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
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::CustomTemplate,
                    parent: String,
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
                    ) -> Result<crate::schemas::CustomTemplate, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/templates");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::CustomTemplate, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                    parent: String,
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
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_template<T>(
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
                                    #[serde(rename = "template")]
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
                    ) -> Result<crate::schemas::ListTemplatesResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/templates");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                pub struct RevertRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> RevertRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the template in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::RevertTemplateResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":revert");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    request: crate::schemas::CustomTemplate,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the templates in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::CustomTemplate, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod triggers {
                pub mod params {}
                pub struct TriggersActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> TriggersActions<'a, A> {
                    #[doc = "Creates a GTM Trigger."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Trigger,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a GTM Trigger."]
                    pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Gets a GTM Trigger."]
                    pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Lists all GTM Triggers of a Container."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Reverts changes to a GTM Trigger in a GTM Workspace."]
                    pub fn revert(&self, path: impl Into<String>) -> RevertRequestBuilder<A> {
                        RevertRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                    #[doc = "Updates a GTM Trigger."]
                    pub fn update(
                        &self,
                        request: crate::schemas::Trigger,
                        path: impl Into<String>,
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
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Trigger,
                    parent: String,
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
                    ) -> Result<crate::schemas::Trigger, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/triggers");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Trigger, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                    parent: String,
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
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_trigger<T>(
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
                                    #[serde(rename = "trigger")]
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
                    ) -> Result<crate::schemas::ListTriggersResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/triggers");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                pub struct RevertRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> RevertRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the trigger in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::RevertTriggerResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":revert");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    request: crate::schemas::Trigger,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the trigger in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::Trigger, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod variables {
                pub mod params {}
                pub struct VariablesActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> VariablesActions<'a, A> {
                    #[doc = "Creates a GTM Variable."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Variable,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a GTM Variable."]
                    pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Gets a GTM Variable."]
                    pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Lists all GTM Variables of a Container."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Reverts changes to a GTM Variable in a GTM Workspace."]
                    pub fn revert(&self, path: impl Into<String>) -> RevertRequestBuilder<A> {
                        RevertRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                    #[doc = "Updates a GTM Variable."]
                    pub fn update(
                        &self,
                        request: crate::schemas::Variable,
                        path: impl Into<String>,
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
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Variable,
                    parent: String,
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
                    ) -> Result<crate::schemas::Variable, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/variables");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Variable, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                    parent: String,
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
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_variable<T>(
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
                                    #[serde(rename = "variable")]
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
                    ) -> Result<crate::schemas::ListVariablesResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/variables");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                pub struct RevertRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> RevertRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the variable in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::RevertVariableResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":revert");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    request: crate::schemas::Variable,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the variable in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::Variable, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
            pub mod zones {
                pub mod params {}
                pub struct ZonesActions<'a, A> {
                    pub(super) reqwest: &'a reqwest::Client,
                    pub(super) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> ZonesActions<'a, A> {
                    #[doc = "Creates a GTM Zone."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Zone,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes a GTM Zone."]
                    pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Gets a GTM Zone."]
                    pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                            path: path.into(),
                        }
                    }
                    #[doc = "Lists all GTM Zones of a GTM container workspace."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            page_token: None,
                        }
                    }
                    #[doc = "Reverts changes to a GTM Zone in a GTM Workspace."]
                    pub fn revert(&self, path: impl Into<String>) -> RevertRequestBuilder<A> {
                        RevertRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                    #[doc = "Updates a GTM Zone."]
                    pub fn update(
                        &self,
                        request: crate::schemas::Zone,
                        path: impl Into<String>,
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
                            path: path.into(),
                            fingerprint: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Zone,
                    parent: String,
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
                    ) -> Result<crate::schemas::Zone, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/zones");
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    path: String,
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
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Zone, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                    parent: String,
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
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub fn iter_zone<T>(
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
                                    #[serde(rename = "zone")]
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
                    ) -> Result<crate::schemas::ListZonesResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.parent);
                        output.push_str("/zones");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.readonly",
                            ])
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
                pub struct RevertRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> RevertRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the zone in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::RevertZoneResponse, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output.push_str(":revert");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
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
                    request: crate::schemas::Zone,
                    path: String,
                    fingerprint: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                    #[doc = "When provided, this fingerprint must match the fingerprint of the zone in storage."]
                    pub fn fingerprint(&mut self, value: impl Into<String>) -> &mut Self {
                        self.fingerprint = Some(value.into());
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
                    ) -> Result<crate::schemas::Zone, Box<dyn ::std::error::Error>>
                    {
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
                        let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                        output.push_str(&self.path);
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
                        let req = req.query(&[("fingerprint", &self.fingerprint)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        let mut auth = self.auth.lock().unwrap();
                        let req = req.bearer_auth(
                            auth.token::<_, &str>(&[
                                "https://www.googleapis.com/auth/tagmanager.edit.containers",
                            ])
                            .unwrap()
                            .access_token,
                        );
                        req
                    }
                }
            }
        }
    }
    pub mod user_permissions {
        pub mod params {}
        pub struct UserPermissionsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UserPermissionsActions<'a, A> {
            #[doc = "Creates a user's Account & Container access."]
            pub fn create(
                &self,
                request: crate::schemas::UserPermission,
                parent: impl Into<String>,
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
                    parent: parent.into(),
                }
            }
            #[doc = "Removes a user from the account, revoking access to it and all of its containers."]
            pub fn delete(&self, path: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                    path: path.into(),
                }
            }
            #[doc = "Gets a user's Account & Container access."]
            pub fn get(&self, path: impl Into<String>) -> GetRequestBuilder<A> {
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
                    path: path.into(),
                }
            }
            #[doc = "List all users that have access to the account along with Account and Container user access granted to each of them."]
            pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                    parent: parent.into(),
                    page_token: None,
                }
            }
            #[doc = "Updates a user's Account & Container access."]
            pub fn update(
                &self,
                request: crate::schemas::UserPermission,
                path: impl Into<String>,
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
                    path: path.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::UserPermission,
            parent: String,
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
            ) -> Result<crate::schemas::UserPermission, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.parent);
                output.push_str("/user_permissions");
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
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.manage.users",
                    ])
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
            path: String,
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.path);
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
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.manage.users",
                    ])
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
            path: String,
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
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::UserPermission, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.path);
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
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.manage.users",
                    ])
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
            parent: String,
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
            #[doc = "Continuation token for fetching the next page of results."]
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
            pub fn iter_user_permission<T>(
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
                            #[serde(rename = "userPermission")]
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
            ) -> Result<crate::schemas::ListUserPermissionsResponse, Box<dyn ::std::error::Error>>
            {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.parent);
                output.push_str("/user_permissions");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.manage.users",
                    ])
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
            request: crate::schemas::UserPermission,
            path: String,
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
            ) -> Result<crate::schemas::UserPermission, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/tagmanager/v2/".to_owned();
                output.push_str(&self.path);
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
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&[
                        "https://www.googleapis.com/auth/tagmanager.manage.users",
                    ])
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
