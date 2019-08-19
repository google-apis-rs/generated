pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AnalyticsInfo {
        #[doc = "Google Play Campaign Measurements."]
        #[serde(rename = "googlePlayAnalytics", default)]
        pub google_play_analytics: Option<crate::schemas::GooglePlayAnalytics>,
        #[doc = "iTunes Connect App Analytics."]
        #[serde(rename = "itunesConnectAnalytics", default)]
        pub itunes_connect_analytics: Option<crate::schemas::ItunesConnectAnalytics>,
    }
    impl ::field_selector::FieldSelector for AnalyticsInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidInfo {
        #[doc = "Link to open on Android if the app is not installed."]
        #[serde(rename = "androidFallbackLink", default)]
        pub android_fallback_link: Option<String>,
        #[doc = "If specified, this overrides the \u{2018}link\u{2019} parameter on Android."]
        #[serde(rename = "androidLink", default)]
        pub android_link: Option<String>,
        #[doc = "Minimum version code for the Android app. If the installed app\u{2019}s version\ncode is lower, then the user is taken to the Play Store."]
        #[serde(rename = "androidMinPackageVersionCode", default)]
        pub android_min_package_version_code: Option<String>,
        #[doc = "Android package name of the app."]
        #[serde(rename = "androidPackageName", default)]
        pub android_package_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for AndroidInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateManagedShortLinkRequest {
        #[doc = "Information about the Dynamic Link to be shortened.\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(rename = "dynamicLinkInfo", default)]
        pub dynamic_link_info: Option<crate::schemas::DynamicLinkInfo>,
        #[doc = "Full long Dynamic Link URL with desired query parameters specified.\nFor example,\n\"https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample\",\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(rename = "longDynamicLink", default)]
        pub long_dynamic_link: Option<String>,
        #[doc = "Link name to associate with the link. It's used for marketer to identify\nmanually-created links in the Firebase console\n(https://console.firebase.google.com/).\nLinks must be named to be tracked."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(rename = "sdkVersion", default)]
        pub sdk_version: Option<String>,
        #[doc = "Short Dynamic Link suffix. Optional."]
        #[serde(rename = "suffix", default)]
        pub suffix: Option<crate::schemas::Suffix>,
    }
    impl ::field_selector::FieldSelector for CreateManagedShortLinkRequest {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateManagedShortLinkResponse {
        #[doc = "Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz"]
        #[serde(rename = "managedShortLink", default)]
        pub managed_short_link: Option<crate::schemas::ManagedShortLink>,
        #[doc = "Preview link to show the link flow chart. (debug info.)"]
        #[serde(rename = "previewLink", default)]
        pub preview_link: Option<String>,
        #[doc = "Information about potential warnings on link creation."]
        #[serde(rename = "warning", default)]
        pub warning: Option<Vec<crate::schemas::DynamicLinkWarning>>,
    }
    impl ::field_selector::FieldSelector for CreateManagedShortLinkResponse {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateShortDynamicLinkRequest {
        #[doc = "Information about the Dynamic Link to be shortened.\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(rename = "dynamicLinkInfo", default)]
        pub dynamic_link_info: Option<crate::schemas::DynamicLinkInfo>,
        #[doc = "Full long Dynamic Link URL with desired query parameters specified.\nFor example,\n\"https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample\",\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(rename = "longDynamicLink", default)]
        pub long_dynamic_link: Option<String>,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(rename = "sdkVersion", default)]
        pub sdk_version: Option<String>,
        #[doc = "Short Dynamic Link suffix. Optional."]
        #[serde(rename = "suffix", default)]
        pub suffix: Option<crate::schemas::Suffix>,
    }
    impl ::field_selector::FieldSelector for CreateShortDynamicLinkRequest {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateShortDynamicLinkResponse {
        #[doc = "Preview link to show the link flow chart. (debug info.)"]
        #[serde(rename = "previewLink", default)]
        pub preview_link: Option<String>,
        #[doc = "Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz"]
        #[serde(rename = "shortLink", default)]
        pub short_link: Option<String>,
        #[doc = "Information about potential warnings on link creation."]
        #[serde(rename = "warning", default)]
        pub warning: Option<Vec<crate::schemas::DynamicLinkWarning>>,
    }
    impl ::field_selector::FieldSelector for CreateShortDynamicLinkResponse {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DesktopInfo {
        #[doc = "Link to open on desktop."]
        #[serde(rename = "desktopFallbackLink", default)]
        pub desktop_fallback_link: Option<String>,
    }
    impl ::field_selector::FieldSelector for DesktopInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeviceInfo {
        #[doc = "Device model name."]
        #[serde(rename = "deviceModelName", default)]
        pub device_model_name: Option<String>,
        #[doc = "Device language code setting."]
        #[serde(rename = "languageCode", default)]
        pub language_code: Option<String>,
        #[doc = "Device language code setting obtained by executing JavaScript code in\nWebView."]
        #[serde(rename = "languageCodeFromWebview", default)]
        pub language_code_from_webview: Option<String>,
        #[doc = "Device language code raw setting.\niOS does returns language code in different format than iOS WebView.\nFor example WebView returns en_US, but iOS returns en-US.\nField below will return raw value returned by iOS."]
        #[serde(rename = "languageCodeRaw", default)]
        pub language_code_raw: Option<String>,
        #[doc = "Device display resolution height."]
        #[serde(rename = "screenResolutionHeight", default)]
        #[serde(with = "crate::parsed_string")]
        pub screen_resolution_height: Option<i64>,
        #[doc = "Device display resolution width."]
        #[serde(rename = "screenResolutionWidth", default)]
        #[serde(with = "crate::parsed_string")]
        pub screen_resolution_width: Option<i64>,
        #[doc = "Device timezone setting."]
        #[serde(rename = "timezone", default)]
        pub timezone: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeviceInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DynamicLinkEventStat {
        #[doc = "The number of times this event occurred."]
        #[serde(rename = "count", default)]
        #[serde(with = "crate::parsed_string")]
        pub count: Option<i64>,
        #[doc = "Link event."]
        #[serde(rename = "event", default)]
        pub event: Option<crate::schemas::DynamicLinkEventStatEvent>,
        #[doc = "Requested platform."]
        #[serde(rename = "platform", default)]
        pub platform: Option<crate::schemas::DynamicLinkEventStatPlatform>,
    }
    impl ::field_selector::FieldSelector for DynamicLinkEventStat {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum DynamicLinkEventStatEvent {
        #[doc = "Unspecified type."]
        DynamicLinkEventUnspecified,
        #[doc = "Indicates that an FDL is clicked by users."]
        Click,
        #[doc = "Indicates that an FDL redirects users to fallback link."]
        Redirect,
        #[doc = "Indicates that an FDL triggers an app install from Play store, currently\nit's impossible to get stats from App store."]
        AppInstall,
        #[doc = "Indicates that the app is opened for the first time after an install\ntriggered by FDLs"]
        AppFirstOpen,
        #[doc = "Indicates that the app is opened via an FDL for non-first time."]
        AppReOpen,
    }
    impl DynamicLinkEventStatEvent {
        pub fn as_str(self) -> &'static str {
            match self {
                DynamicLinkEventStatEvent::DynamicLinkEventUnspecified => {
                    "DYNAMIC_LINK_EVENT_UNSPECIFIED"
                }
                DynamicLinkEventStatEvent::Click => "CLICK",
                DynamicLinkEventStatEvent::Redirect => "REDIRECT",
                DynamicLinkEventStatEvent::AppInstall => "APP_INSTALL",
                DynamicLinkEventStatEvent::AppFirstOpen => "APP_FIRST_OPEN",
                DynamicLinkEventStatEvent::AppReOpen => "APP_RE_OPEN",
            }
        }
    }
    impl ::std::fmt::Display for DynamicLinkEventStatEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DynamicLinkEventStatEvent {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DynamicLinkEventStatEvent {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DYNAMIC_LINK_EVENT_UNSPECIFIED" => {
                    DynamicLinkEventStatEvent::DynamicLinkEventUnspecified
                }
                "CLICK" => DynamicLinkEventStatEvent::Click,
                "REDIRECT" => DynamicLinkEventStatEvent::Redirect,
                "APP_INSTALL" => DynamicLinkEventStatEvent::AppInstall,
                "APP_FIRST_OPEN" => DynamicLinkEventStatEvent::AppFirstOpen,
                "APP_RE_OPEN" => DynamicLinkEventStatEvent::AppReOpen,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum DynamicLinkEventStatPlatform {
        #[doc = "Unspecified platform."]
        DynamicLinkPlatformUnspecified,
        #[doc = "Represents Android platform.\nAll apps and browsers on Android are classfied in this category."]
        Android,
        #[doc = "Represents iOS platform.\nAll apps and browsers on iOS are classfied in this category."]
        Ios,
        #[doc = "Represents desktop."]
        Desktop,
        #[doc = "Platforms are not categorized as Android/iOS/Destop fall into here."]
        Other,
    }
    impl DynamicLinkEventStatPlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                DynamicLinkEventStatPlatform::DynamicLinkPlatformUnspecified => {
                    "DYNAMIC_LINK_PLATFORM_UNSPECIFIED"
                }
                DynamicLinkEventStatPlatform::Android => "ANDROID",
                DynamicLinkEventStatPlatform::Ios => "IOS",
                DynamicLinkEventStatPlatform::Desktop => "DESKTOP",
                DynamicLinkEventStatPlatform::Other => "OTHER",
            }
        }
    }
    impl ::std::fmt::Display for DynamicLinkEventStatPlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DynamicLinkEventStatPlatform {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DynamicLinkEventStatPlatform {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DYNAMIC_LINK_PLATFORM_UNSPECIFIED" => {
                    DynamicLinkEventStatPlatform::DynamicLinkPlatformUnspecified
                }
                "ANDROID" => DynamicLinkEventStatPlatform::Android,
                "IOS" => DynamicLinkEventStatPlatform::Ios,
                "DESKTOP" => DynamicLinkEventStatPlatform::Desktop,
                "OTHER" => DynamicLinkEventStatPlatform::Other,
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DynamicLinkInfo {
        #[doc = "Parameters used for tracking. See all tracking parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(rename = "analyticsInfo", default)]
        pub analytics_info: Option<crate::schemas::AnalyticsInfo>,
        #[doc = "Android related information. See Android related parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(rename = "androidInfo", default)]
        pub android_info: Option<crate::schemas::AndroidInfo>,
        #[doc = "Desktop related information. See desktop related parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(rename = "desktopInfo", default)]
        pub desktop_info: Option<crate::schemas::DesktopInfo>,
        #[doc = "E.g. https://maps.app.goo.gl, https://maps.page.link, https://g.co/maps\nMore examples can be found in description of getNormalizedUriPrefix in\nj/c/g/firebase/dynamiclinks/uri/DdlDomain.java\n\nWill fallback to dynamic_link_domain is this field is missing"]
        #[serde(rename = "domainUriPrefix", default)]
        pub domain_uri_prefix: Option<String>,
        #[doc = "Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl\n[Learn\nmore](https://firebase.google.com/docs/dynamic-links/android/receive) on\nhow to set up Dynamic Link domain associated with your Firebase project.\n\nRequired if missing domain_uri_prefix."]
        #[serde(rename = "dynamicLinkDomain", default)]
        pub dynamic_link_domain: Option<String>,
        #[doc = "iOS related information. See iOS related parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(rename = "iosInfo", default)]
        pub ios_info: Option<crate::schemas::IosInfo>,
        #[doc = "The link your app will open, You can specify any URL your app can handle.\nThis link must be a well-formatted URL, be properly URL-encoded, and use\nthe HTTP or HTTPS scheme. See 'link' parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually).\n\nRequired."]
        #[serde(rename = "link", default)]
        pub link: Option<String>,
        #[doc = "Information of navigation behavior of a Firebase Dynamic Links."]
        #[serde(rename = "navigationInfo", default)]
        pub navigation_info: Option<crate::schemas::NavigationInfo>,
        #[doc = "Parameters for social meta tag params.\nUsed to set meta tag data for link previews on social sites."]
        #[serde(rename = "socialMetaTagInfo", default)]
        pub social_meta_tag_info: Option<crate::schemas::SocialMetaTagInfo>,
    }
    impl ::field_selector::FieldSelector for DynamicLinkInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DynamicLinkStats {
        #[doc = "Dynamic Link event stats."]
        #[serde(rename = "linkEventStats", default)]
        pub link_event_stats: Option<Vec<crate::schemas::DynamicLinkEventStat>>,
    }
    impl ::field_selector::FieldSelector for DynamicLinkStats {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DynamicLinkWarning {
        #[doc = "The warning code."]
        #[serde(rename = "warningCode", default)]
        pub warning_code: Option<crate::schemas::DynamicLinkWarningWarningCode>,
        #[doc = "The document describing the warning, and helps resolve."]
        #[serde(rename = "warningDocumentLink", default)]
        pub warning_document_link: Option<String>,
        #[doc = "The warning message to help developers improve their requests."]
        #[serde(rename = "warningMessage", default)]
        pub warning_message: Option<String>,
    }
    impl ::field_selector::FieldSelector for DynamicLinkWarning {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum DynamicLinkWarningWarningCode {
        #[doc = "Unknown code."]
        CodeUnspecified,
        #[doc = "The Android package does not match any in developer's DevConsole project."]
        NotInProjectAndroidPackageName,
        #[doc = "The Android minimum version code has to be a valid integer."]
        NotIntegerAndroidPackageMinVersion,
        #[doc = "Android package min version param is not needed, e.g. when\n'apn' is missing."]
        UnnecessaryAndroidPackageMinVersion,
        #[doc = "Android link is not a valid URI."]
        NotUriAndroidLink,
        #[doc = "Android link param is not needed, e.g. when param 'al' and 'link' have\nthe same value.."]
        UnnecessaryAndroidLink,
        #[doc = "Android fallback link is not a valid URI."]
        NotUriAndroidFallbackLink,
        #[doc = "Android fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeAndroidFallbackLink,
        #[doc = "The iOS bundle ID does not match any in developer's DevConsole project."]
        NotInProjectIosBundleId,
        #[doc = "The iPad bundle ID does not match any in developer's DevConsole project."]
        NotInProjectIpadBundleId,
        #[doc = "iOS URL scheme is not needed, e.g. when 'ibi' are 'ipbi' are all missing."]
        UnnecessaryIosUrlScheme,
        #[doc = "iOS app store ID format is incorrect, e.g. not numeric."]
        NotNumericIosAppStoreId,
        #[doc = "iOS app store ID is not needed."]
        UnnecessaryIosAppStoreId,
        #[doc = "iOS fallback link is not a valid URI."]
        NotUriIosFallbackLink,
        #[doc = "iOS fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeIosFallbackLink,
        #[doc = "iPad fallback link is not a valid URI."]
        NotUriIpadFallbackLink,
        #[doc = "iPad fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeIpadFallbackLink,
        #[doc = "Debug param format is incorrect."]
        BadDebugParam,
        #[doc = "isAd param format is incorrect."]
        BadAdParam,
        #[doc = "Indicates a certain param is deprecated."]
        DeprecatedParam,
        #[doc = "Indicates certain paramater is not recognized."]
        UnrecognizedParam,
        #[doc = "Indicates certain paramater is too long."]
        TooLongParam,
        #[doc = "Social meta tag image link is not a valid URI."]
        NotUriSocialImageLink,
        #[doc = "Social meta tag image link has an invalid (non http/https) URI scheme."]
        BadUriSchemeSocialImageLink,
        NotUriSocialUrl,
        BadUriSchemeSocialUrl,
        #[doc = "Dynamic Link URL length is too long."]
        LinkLengthTooLong,
        #[doc = "Dynamic Link URL contains fragments."]
        LinkWithFragments,
        #[doc = "The iOS bundle ID does not match with the given iOS store ID."]
        NotMatchingIosBundleIdAndStoreId,
    }
    impl DynamicLinkWarningWarningCode {
        pub fn as_str(self) -> &'static str {
            match self {
                DynamicLinkWarningWarningCode::CodeUnspecified => "CODE_UNSPECIFIED",
                DynamicLinkWarningWarningCode::NotInProjectAndroidPackageName => {
                    "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME"
                }
                DynamicLinkWarningWarningCode::NotIntegerAndroidPackageMinVersion => {
                    "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION"
                }
                DynamicLinkWarningWarningCode::UnnecessaryAndroidPackageMinVersion => {
                    "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION"
                }
                DynamicLinkWarningWarningCode::NotUriAndroidLink => "NOT_URI_ANDROID_LINK",
                DynamicLinkWarningWarningCode::UnnecessaryAndroidLink => "UNNECESSARY_ANDROID_LINK",
                DynamicLinkWarningWarningCode::NotUriAndroidFallbackLink => {
                    "NOT_URI_ANDROID_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::BadUriSchemeAndroidFallbackLink => {
                    "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::NotInProjectIosBundleId => {
                    "NOT_IN_PROJECT_IOS_BUNDLE_ID"
                }
                DynamicLinkWarningWarningCode::NotInProjectIpadBundleId => {
                    "NOT_IN_PROJECT_IPAD_BUNDLE_ID"
                }
                DynamicLinkWarningWarningCode::UnnecessaryIosUrlScheme => {
                    "UNNECESSARY_IOS_URL_SCHEME"
                }
                DynamicLinkWarningWarningCode::NotNumericIosAppStoreId => {
                    "NOT_NUMERIC_IOS_APP_STORE_ID"
                }
                DynamicLinkWarningWarningCode::UnnecessaryIosAppStoreId => {
                    "UNNECESSARY_IOS_APP_STORE_ID"
                }
                DynamicLinkWarningWarningCode::NotUriIosFallbackLink => "NOT_URI_IOS_FALLBACK_LINK",
                DynamicLinkWarningWarningCode::BadUriSchemeIosFallbackLink => {
                    "BAD_URI_SCHEME_IOS_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::NotUriIpadFallbackLink => {
                    "NOT_URI_IPAD_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::BadUriSchemeIpadFallbackLink => {
                    "BAD_URI_SCHEME_IPAD_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::BadDebugParam => "BAD_DEBUG_PARAM",
                DynamicLinkWarningWarningCode::BadAdParam => "BAD_AD_PARAM",
                DynamicLinkWarningWarningCode::DeprecatedParam => "DEPRECATED_PARAM",
                DynamicLinkWarningWarningCode::UnrecognizedParam => "UNRECOGNIZED_PARAM",
                DynamicLinkWarningWarningCode::TooLongParam => "TOO_LONG_PARAM",
                DynamicLinkWarningWarningCode::NotUriSocialImageLink => "NOT_URI_SOCIAL_IMAGE_LINK",
                DynamicLinkWarningWarningCode::BadUriSchemeSocialImageLink => {
                    "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK"
                }
                DynamicLinkWarningWarningCode::NotUriSocialUrl => "NOT_URI_SOCIAL_URL",
                DynamicLinkWarningWarningCode::BadUriSchemeSocialUrl => "BAD_URI_SCHEME_SOCIAL_URL",
                DynamicLinkWarningWarningCode::LinkLengthTooLong => "LINK_LENGTH_TOO_LONG",
                DynamicLinkWarningWarningCode::LinkWithFragments => "LINK_WITH_FRAGMENTS",
                DynamicLinkWarningWarningCode::NotMatchingIosBundleIdAndStoreId => {
                    "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID"
                }
            }
        }
    }
    impl ::std::fmt::Display for DynamicLinkWarningWarningCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DynamicLinkWarningWarningCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DynamicLinkWarningWarningCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => DynamicLinkWarningWarningCode::CodeUnspecified,
                "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME" => {
                    DynamicLinkWarningWarningCode::NotInProjectAndroidPackageName
                }
                "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION" => {
                    DynamicLinkWarningWarningCode::NotIntegerAndroidPackageMinVersion
                }
                "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION" => {
                    DynamicLinkWarningWarningCode::UnnecessaryAndroidPackageMinVersion
                }
                "NOT_URI_ANDROID_LINK" => DynamicLinkWarningWarningCode::NotUriAndroidLink,
                "UNNECESSARY_ANDROID_LINK" => DynamicLinkWarningWarningCode::UnnecessaryAndroidLink,
                "NOT_URI_ANDROID_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::NotUriAndroidFallbackLink
                }
                "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeAndroidFallbackLink
                }
                "NOT_IN_PROJECT_IOS_BUNDLE_ID" => {
                    DynamicLinkWarningWarningCode::NotInProjectIosBundleId
                }
                "NOT_IN_PROJECT_IPAD_BUNDLE_ID" => {
                    DynamicLinkWarningWarningCode::NotInProjectIpadBundleId
                }
                "UNNECESSARY_IOS_URL_SCHEME" => {
                    DynamicLinkWarningWarningCode::UnnecessaryIosUrlScheme
                }
                "NOT_NUMERIC_IOS_APP_STORE_ID" => {
                    DynamicLinkWarningWarningCode::NotNumericIosAppStoreId
                }
                "UNNECESSARY_IOS_APP_STORE_ID" => {
                    DynamicLinkWarningWarningCode::UnnecessaryIosAppStoreId
                }
                "NOT_URI_IOS_FALLBACK_LINK" => DynamicLinkWarningWarningCode::NotUriIosFallbackLink,
                "BAD_URI_SCHEME_IOS_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeIosFallbackLink
                }
                "NOT_URI_IPAD_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::NotUriIpadFallbackLink
                }
                "BAD_URI_SCHEME_IPAD_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeIpadFallbackLink
                }
                "BAD_DEBUG_PARAM" => DynamicLinkWarningWarningCode::BadDebugParam,
                "BAD_AD_PARAM" => DynamicLinkWarningWarningCode::BadAdParam,
                "DEPRECATED_PARAM" => DynamicLinkWarningWarningCode::DeprecatedParam,
                "UNRECOGNIZED_PARAM" => DynamicLinkWarningWarningCode::UnrecognizedParam,
                "TOO_LONG_PARAM" => DynamicLinkWarningWarningCode::TooLongParam,
                "NOT_URI_SOCIAL_IMAGE_LINK" => DynamicLinkWarningWarningCode::NotUriSocialImageLink,
                "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeSocialImageLink
                }
                "NOT_URI_SOCIAL_URL" => DynamicLinkWarningWarningCode::NotUriSocialUrl,
                "BAD_URI_SCHEME_SOCIAL_URL" => DynamicLinkWarningWarningCode::BadUriSchemeSocialUrl,
                "LINK_LENGTH_TOO_LONG" => DynamicLinkWarningWarningCode::LinkLengthTooLong,
                "LINK_WITH_FRAGMENTS" => DynamicLinkWarningWarningCode::LinkWithFragments,
                "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID" => {
                    DynamicLinkWarningWarningCode::NotMatchingIosBundleIdAndStoreId
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetIosPostInstallAttributionRequest {
        #[doc = "App installation epoch time (https://en.wikipedia.org/wiki/Unix_time).\nThis is a client signal for a more accurate weak match."]
        #[serde(rename = "appInstallationTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub app_installation_time: Option<i64>,
        #[doc = "APP bundle ID."]
        #[serde(rename = "bundleId", default)]
        pub bundle_id: Option<String>,
        #[doc = "Device information."]
        #[serde(rename = "device", default)]
        pub device: Option<crate::schemas::DeviceInfo>,
        #[doc = "iOS version, ie: 9.3.5.\nConsider adding \"build\"."]
        #[serde(rename = "iosVersion", default)]
        pub ios_version: Option<String>,
        #[doc = "App post install attribution retrieval information. Disambiguates\nmechanism (iSDK or developer invoked) to retrieve payload from\nclicked link."]
        #[serde(rename = "retrievalMethod", default)]
        pub retrieval_method:
            Option<crate::schemas::GetIosPostInstallAttributionRequestRetrievalMethod>,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(rename = "sdkVersion", default)]
        pub sdk_version: Option<String>,
        #[doc = "Possible unique matched link that server need to check before performing\nfingerprint match. If passed link is short server need to expand the link.\nIf link is long server need to vslidate the link."]
        #[serde(rename = "uniqueMatchLinkToCheck", default)]
        pub unique_match_link_to_check: Option<String>,
        #[doc = "Strong match page information. Disambiguates between default UI and\ncustom page to present when strong match succeeds/fails to find cookie."]
        #[serde(rename = "visualStyle", default)]
        pub visual_style: Option<crate::schemas::GetIosPostInstallAttributionRequestVisualStyle>,
    }
    impl ::field_selector::FieldSelector for GetIosPostInstallAttributionRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionRequestRetrievalMethod {
        #[doc = "Unknown method."]
        UnknownPayloadRetrievalMethod,
        #[doc = "iSDK performs a server lookup by device fingerprint in the background\nwhen app is first-opened; no API called by developer."]
        ImplicitWeakMatch,
        #[doc = "iSDK performs a server lookup by device fingerprint upon a dev API call."]
        ExplicitWeakMatch,
        #[doc = "iSDK performs a strong match only if weak match is found upon a dev\nAPI call."]
        ExplicitStrongAfterWeakMatch,
    }
    impl GetIosPostInstallAttributionRequestRetrievalMethod {
        pub fn as_str(self) -> &'static str {
            match self { GetIosPostInstallAttributionRequestRetrievalMethod :: UnknownPayloadRetrievalMethod => "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD" , GetIosPostInstallAttributionRequestRetrievalMethod :: ImplicitWeakMatch => "IMPLICIT_WEAK_MATCH" , GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitWeakMatch => "EXPLICIT_WEAK_MATCH" , GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitStrongAfterWeakMatch => "EXPLICIT_STRONG_AFTER_WEAK_MATCH" , }
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD" => GetIosPostInstallAttributionRequestRetrievalMethod :: UnknownPayloadRetrievalMethod , "IMPLICIT_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ImplicitWeakMatch , "EXPLICIT_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitWeakMatch , "EXPLICIT_STRONG_AFTER_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitStrongAfterWeakMatch , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionRequestVisualStyle {
        #[doc = "Unknown style."]
        UnknownVisualStyle,
        #[doc = "Default style."]
        DefaultStyle,
        #[doc = "Custom style."]
        CustomStyle,
    }
    impl GetIosPostInstallAttributionRequestVisualStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                GetIosPostInstallAttributionRequestVisualStyle::UnknownVisualStyle => {
                    "UNKNOWN_VISUAL_STYLE"
                }
                GetIosPostInstallAttributionRequestVisualStyle::DefaultStyle => "DEFAULT_STYLE",
                GetIosPostInstallAttributionRequestVisualStyle::CustomStyle => "CUSTOM_STYLE",
            }
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionRequestVisualStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionRequestVisualStyle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionRequestVisualStyle {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_VISUAL_STYLE" => {
                    GetIosPostInstallAttributionRequestVisualStyle::UnknownVisualStyle
                }
                "DEFAULT_STYLE" => GetIosPostInstallAttributionRequestVisualStyle::DefaultStyle,
                "CUSTOM_STYLE" => GetIosPostInstallAttributionRequestVisualStyle::CustomStyle,
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetIosPostInstallAttributionResponse {
        #[doc = "The minimum version for app, specified by dev through ?imv= parameter.\nReturn to iSDK to allow app to evaluate if current version meets this."]
        #[serde(rename = "appMinimumVersion", default)]
        pub app_minimum_version: Option<String>,
        #[doc = "The confidence of the returned attribution."]
        #[serde(rename = "attributionConfidence", default)]
        pub attribution_confidence:
            Option<crate::schemas::GetIosPostInstallAttributionResponseAttributionConfidence>,
        #[doc = "The deep-link attributed post-install via one of several techniques\n(fingerprint, copy unique)."]
        #[serde(rename = "deepLink", default)]
        pub deep_link: Option<String>,
        #[doc = "User-agent specific custom-scheme URIs for iSDK to open. This will be set\naccording to the user-agent tha the click was originally made in. There is\nno Safari-equivalent custom-scheme open URLs.\nie: googlechrome://www.example.com\nie: firefox://open-url?url=http://www.example.com\nie: opera-http://example.com"]
        #[serde(rename = "externalBrowserDestinationLink", default)]
        pub external_browser_destination_link: Option<String>,
        #[doc = "The link to navigate to update the app if min version is not met.\nThis is either (in order): 1) fallback link (from ?ifl= parameter, if\nspecified by developer) or 2) AppStore URL (from ?isi= parameter, if\nspecified), or 3) the payload link (from required link= parameter)."]
        #[serde(rename = "fallbackLink", default)]
        pub fallback_link: Option<String>,
        #[doc = "Invitation ID attributed post-install via one of several techniques\n(fingerprint, copy unique)."]
        #[serde(rename = "invitationId", default)]
        pub invitation_id: Option<String>,
        #[doc = "Instruction for iSDK to attemmpt to perform strong match. For instance,\nif browser does not support/allow cookie or outside of support browsers,\nthis will be false."]
        #[serde(rename = "isStrongMatchExecutable", default)]
        pub is_strong_match_executable: Option<bool>,
        #[doc = "Describes why match failed, ie: \"discarded due to low confidence\".\nThis message will be publicly visible."]
        #[serde(rename = "matchMessage", default)]
        pub match_message: Option<String>,
        #[doc = "Which IP version the request was made from."]
        #[serde(rename = "requestIpVersion", default)]
        pub request_ip_version:
            Option<crate::schemas::GetIosPostInstallAttributionResponseRequestIpVersion>,
        #[doc = "Entire FDL (short or long) attributed post-install via one of several\ntechniques (fingerprint, copy unique)."]
        #[serde(rename = "requestedLink", default)]
        pub requested_link: Option<String>,
        #[doc = "The entire FDL, expanded from a short link. It is the same as the\nrequested_link, if it is long. Parameters from this should not be\nused directly (ie: server can default utm_[campaign|medium|source]\nto a value when requested_link lack them, server determine the best\nfallback_link when requested_link specifies >1 fallback links)."]
        #[serde(rename = "resolvedLink", default)]
        pub resolved_link: Option<String>,
        #[doc = "Scion campaign value to be propagated by iSDK to Scion at post-install."]
        #[serde(rename = "utmCampaign", default)]
        pub utm_campaign: Option<String>,
        #[doc = "Scion content value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(rename = "utmContent", default)]
        pub utm_content: Option<String>,
        #[doc = "Scion medium value to be propagated by iSDK to Scion at post-install."]
        #[serde(rename = "utmMedium", default)]
        pub utm_medium: Option<String>,
        #[doc = "Scion source value to be propagated by iSDK to Scion at post-install."]
        #[serde(rename = "utmSource", default)]
        pub utm_source: Option<String>,
        #[doc = "Scion term value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(rename = "utmTerm", default)]
        pub utm_term: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetIosPostInstallAttributionResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionResponseAttributionConfidence {
        #[doc = "Unset."]
        UnknownAttributionConfidence,
        #[doc = "Weak confidence, more than one matching link found or link suspected to\nbe false positive"]
        Weak,
        #[doc = "Default confidence, match based on fingerprint"]
        Default,
        #[doc = "Unique confidence, match based on \"unique match link to check\" or other\nmeans"]
        Unique,
    }
    impl GetIosPostInstallAttributionResponseAttributionConfidence {
        pub fn as_str(self) -> &'static str {
            match self { GetIosPostInstallAttributionResponseAttributionConfidence :: UnknownAttributionConfidence => "UNKNOWN_ATTRIBUTION_CONFIDENCE" , GetIosPostInstallAttributionResponseAttributionConfidence :: Weak => "WEAK" , GetIosPostInstallAttributionResponseAttributionConfidence :: Default => "DEFAULT" , GetIosPostInstallAttributionResponseAttributionConfidence :: Unique => "UNIQUE" , }
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionResponseAttributionConfidence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionResponseAttributionConfidence {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionResponseAttributionConfidence {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "UNKNOWN_ATTRIBUTION_CONFIDENCE" => GetIosPostInstallAttributionResponseAttributionConfidence :: UnknownAttributionConfidence , "WEAK" => GetIosPostInstallAttributionResponseAttributionConfidence :: Weak , "DEFAULT" => GetIosPostInstallAttributionResponseAttributionConfidence :: Default , "UNIQUE" => GetIosPostInstallAttributionResponseAttributionConfidence :: Unique , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionResponseRequestIpVersion {
        #[doc = "Unset."]
        UnknownIpVersion,
        #[doc = "Request made from an IPv4 IP address."]
        IpV4,
        #[doc = "Request made from an IPv6 IP address."]
        IpV6,
    }
    impl GetIosPostInstallAttributionResponseRequestIpVersion {
        pub fn as_str(self) -> &'static str {
            match self {
                GetIosPostInstallAttributionResponseRequestIpVersion::UnknownIpVersion => {
                    "UNKNOWN_IP_VERSION"
                }
                GetIosPostInstallAttributionResponseRequestIpVersion::IpV4 => "IP_V4",
                GetIosPostInstallAttributionResponseRequestIpVersion::IpV6 => "IP_V6",
            }
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionResponseRequestIpVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionResponseRequestIpVersion {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionResponseRequestIpVersion {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNKNOWN_IP_VERSION" => {
                    GetIosPostInstallAttributionResponseRequestIpVersion::UnknownIpVersion
                }
                "IP_V4" => GetIosPostInstallAttributionResponseRequestIpVersion::IpV4,
                "IP_V6" => GetIosPostInstallAttributionResponseRequestIpVersion::IpV6,
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetIosReopenAttributionRequest {
        #[doc = "APP bundle ID."]
        #[serde(rename = "bundleId", default)]
        pub bundle_id: Option<String>,
        #[doc = "FDL link to be verified from an app universal link open.\nThe FDL link can be one of:\n1) short FDL.\ne.g. <app_code>.page.link/<ddl_id>, or\n2) long FDL.\ne.g. <app_code>.page.link/?{query params}, or\n3) Invite FDL.\ne.g. <app_code>.page.link/i/<invite_id_or_alias>"]
        #[serde(rename = "requestedLink", default)]
        pub requested_link: Option<String>,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(rename = "sdkVersion", default)]
        pub sdk_version: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetIosReopenAttributionRequest {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GetIosReopenAttributionResponse {
        #[doc = "The deep-link attributed the app universal link open. For both regular\nFDL links and invite FDL links."]
        #[serde(rename = "deepLink", default)]
        pub deep_link: Option<String>,
        #[doc = "Optional invitation ID, for only invite typed requested FDL links."]
        #[serde(rename = "invitationId", default)]
        pub invitation_id: Option<String>,
        #[doc = "FDL input value of the \"&imv=\" parameter, minimum app version to be\nreturned to Google Firebase SDK running on iOS-9."]
        #[serde(rename = "iosMinAppVersion", default)]
        pub ios_min_app_version: Option<String>,
        #[doc = "The entire FDL, expanded from a short link. It is the same as the\nrequested_link, if it is long."]
        #[serde(rename = "resolvedLink", default)]
        pub resolved_link: Option<String>,
        #[doc = "Scion campaign value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(rename = "utmCampaign", default)]
        pub utm_campaign: Option<String>,
        #[doc = "Scion content value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(rename = "utmContent", default)]
        pub utm_content: Option<String>,
        #[doc = "Scion medium value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(rename = "utmMedium", default)]
        pub utm_medium: Option<String>,
        #[doc = "Scion source value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(rename = "utmSource", default)]
        pub utm_source: Option<String>,
        #[doc = "Scion term value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(rename = "utmTerm", default)]
        pub utm_term: Option<String>,
    }
    impl ::field_selector::FieldSelector for GetIosReopenAttributionResponse {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GooglePlayAnalytics {
        #[doc = "[AdWords autotagging\nparameter](https://support.google.com/analytics/answer/1033981?hl=en); used\nto measure Google AdWords ads. This value is generated dynamically and\nshould never be modified."]
        #[serde(rename = "gclid", default)]
        pub gclid: Option<String>,
        #[doc = "Campaign name; used for keyword analysis to identify a specific product\npromotion or strategic campaign."]
        #[serde(rename = "utmCampaign", default)]
        pub utm_campaign: Option<String>,
        #[doc = "Campaign content; used for A/B testing and content-targeted ads to\ndifferentiate ads or links that point to the same URL."]
        #[serde(rename = "utmContent", default)]
        pub utm_content: Option<String>,
        #[doc = "Campaign medium; used to identify a medium such as email or cost-per-click."]
        #[serde(rename = "utmMedium", default)]
        pub utm_medium: Option<String>,
        #[doc = "Campaign source; used to identify a search engine, newsletter, or other\nsource."]
        #[serde(rename = "utmSource", default)]
        pub utm_source: Option<String>,
        #[doc = "Campaign term; used with paid search to supply the keywords for ads."]
        #[serde(rename = "utmTerm", default)]
        pub utm_term: Option<String>,
    }
    impl ::field_selector::FieldSelector for GooglePlayAnalytics {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IosInfo {
        #[doc = "iOS App Store ID."]
        #[serde(rename = "iosAppStoreId", default)]
        pub ios_app_store_id: Option<String>,
        #[doc = "iOS bundle ID of the app."]
        #[serde(rename = "iosBundleId", default)]
        pub ios_bundle_id: Option<String>,
        #[doc = "Custom (destination) scheme to use for iOS. By default, we\u{2019}ll use the\nbundle ID as the custom scheme. Developer can override this behavior using\nthis param."]
        #[serde(rename = "iosCustomScheme", default)]
        pub ios_custom_scheme: Option<String>,
        #[doc = "Link to open on iOS if the app is not installed."]
        #[serde(rename = "iosFallbackLink", default)]
        pub ios_fallback_link: Option<String>,
        #[doc = "iPad bundle ID of the app."]
        #[serde(rename = "iosIpadBundleId", default)]
        pub ios_ipad_bundle_id: Option<String>,
        #[doc = "If specified, this overrides the ios_fallback_link value on iPads."]
        #[serde(rename = "iosIpadFallbackLink", default)]
        pub ios_ipad_fallback_link: Option<String>,
        #[doc = "iOS minimum version."]
        #[serde(rename = "iosMinimumVersion", default)]
        pub ios_minimum_version: Option<String>,
    }
    impl ::field_selector::FieldSelector for IosInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ItunesConnectAnalytics {
        #[doc = "Affiliate token used to create affiliate-coded links."]
        #[serde(rename = "at", default)]
        pub at: Option<String>,
        #[doc = "Campaign text that developers can optionally add to any link in order to\ntrack sales from a specific marketing campaign."]
        #[serde(rename = "ct", default)]
        pub ct: Option<String>,
        #[doc = "iTune media types, including music, podcasts, audiobooks and so on."]
        #[serde(rename = "mt", default)]
        pub mt: Option<String>,
        #[doc = "Provider token that enables analytics for Dynamic Links from within iTunes\nConnect."]
        #[serde(rename = "pt", default)]
        pub pt: Option<String>,
    }
    impl ::field_selector::FieldSelector for ItunesConnectAnalytics {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ManagedShortLink {
        #[doc = "Creation timestamp of the short link."]
        #[serde(rename = "creationTime", default)]
        pub creation_time: Option<String>,
        #[doc = "Attributes that have been flagged about this short url."]
        #[serde(rename = "flaggedAttribute", default)]
        pub flagged_attribute: Option<Vec<crate::schemas::ManagedShortLinkFlaggedAttributeItems>>,
        #[doc = "Full Dyamic Link info"]
        #[serde(rename = "info", default)]
        pub info: Option<crate::schemas::DynamicLinkInfo>,
        #[doc = "Short durable link url, for example, \"https://sample.app.goo.gl/xyz123\".\n\nRequired."]
        #[serde(rename = "link", default)]
        pub link: Option<String>,
        #[doc = "Link name defined by the creator.\n\nRequired."]
        #[serde(rename = "linkName", default)]
        pub link_name: Option<String>,
        #[doc = "Visibility status of link."]
        #[serde(rename = "visibility", default)]
        pub visibility: Option<crate::schemas::ManagedShortLinkVisibility>,
    }
    impl ::field_selector::FieldSelector for ManagedShortLink {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum ManagedShortLinkFlaggedAttributeItems {}
    impl ManagedShortLinkFlaggedAttributeItems {
        pub fn as_str(self) -> &'static str {
            match self {}
        }
    }
    impl ::std::fmt::Display for ManagedShortLinkFlaggedAttributeItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedShortLinkFlaggedAttributeItems {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedShortLinkFlaggedAttributeItems {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum ManagedShortLinkVisibility {
        #[doc = "Visibility of the link is not specified."]
        UnspecifiedVisibility,
        #[doc = "Link created in console and should be shown in console."]
        Unarchived,
        #[doc = "Link created in console and should not be shown in console (but can\nbe shown in the console again if it is unarchived)."]
        Archived,
        #[doc = "Link created outside of console and should never be shown in console."]
        NeverShown,
    }
    impl ManagedShortLinkVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedShortLinkVisibility::UnspecifiedVisibility => "UNSPECIFIED_VISIBILITY",
                ManagedShortLinkVisibility::Unarchived => "UNARCHIVED",
                ManagedShortLinkVisibility::Archived => "ARCHIVED",
                ManagedShortLinkVisibility::NeverShown => "NEVER_SHOWN",
            }
        }
    }
    impl ::std::fmt::Display for ManagedShortLinkVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedShortLinkVisibility {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedShortLinkVisibility {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNSPECIFIED_VISIBILITY" => ManagedShortLinkVisibility::UnspecifiedVisibility,
                "UNARCHIVED" => ManagedShortLinkVisibility::Unarchived,
                "ARCHIVED" => ManagedShortLinkVisibility::Archived,
                "NEVER_SHOWN" => ManagedShortLinkVisibility::NeverShown,
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NavigationInfo {
        #[doc = "If this option is on, FDL click will be forced to redirect rather than\nshow an interstitial page."]
        #[serde(rename = "enableForcedRedirect", default)]
        pub enable_forced_redirect: Option<bool>,
    }
    impl ::field_selector::FieldSelector for NavigationInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SocialMetaTagInfo {
        #[doc = "A short description of the link. Optional."]
        #[serde(rename = "socialDescription", default)]
        pub social_description: Option<String>,
        #[doc = "An image url string. Optional."]
        #[serde(rename = "socialImageLink", default)]
        pub social_image_link: Option<String>,
        #[doc = "Title to be displayed. Optional."]
        #[serde(rename = "socialTitle", default)]
        pub social_title: Option<String>,
    }
    impl ::field_selector::FieldSelector for SocialMetaTagInfo {
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
        PartialOrd,
        Hash,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Suffix {
        #[doc = "Only applies to Option.CUSTOM."]
        #[serde(rename = "customSuffix", default)]
        pub custom_suffix: Option<String>,
        #[doc = "Suffix option."]
        #[serde(rename = "option", default)]
        pub option: Option<crate::schemas::SuffixOption>,
    }
    impl ::field_selector::FieldSelector for Suffix {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
    pub enum SuffixOption {
        #[doc = "The suffix option is not specified, performs as UNGUESSABLE ."]
        OptionUnspecified,
        #[doc = "Short Dynamic Link suffix is a base62 [0-9A-Za-z] encoded string of\na random generated 96 bit random number, which has a length of 17 chars.\nFor example, \"nlAR8U4SlKRZw1cb2\".\nIt prevents other people from guessing and crawling short Dynamic Links\nthat contain personal identifiable information."]
        Unguessable,
        #[doc = "Short Dynamic Link suffix is a base62 [0-9A-Za-z] string starting with a\nlength of 4 chars. the length will increase when all the space is\noccupied."]
        Short,
        #[doc = "Custom DDL suffix is a client specified string, for example,\n\"buy2get1free\".\nNOTE: custom suffix should only be available to managed short link\ncreation"]
        Custom,
    }
    impl SuffixOption {
        pub fn as_str(self) -> &'static str {
            match self {
                SuffixOption::OptionUnspecified => "OPTION_UNSPECIFIED",
                SuffixOption::Unguessable => "UNGUESSABLE",
                SuffixOption::Short => "SHORT",
                SuffixOption::Custom => "CUSTOM",
            }
        }
    }
    impl ::std::fmt::Display for SuffixOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SuffixOption {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuffixOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPTION_UNSPECIFIED" => SuffixOption::OptionUnspecified,
                "UNGUESSABLE" => SuffixOption::Unguessable,
                "SHORT" => SuffixOption::Short,
                "CUSTOM" => SuffixOption::Custom,
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
pub mod params {
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
    #[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Ord, Eq, Copy)]
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
    #[doc = "Actions that can be performed on the managed_short_links resource"]
    pub fn managed_short_links(&self) -> crate::managed_short_links::ManagedShortLinksActions<A> {
        crate::managed_short_links::ManagedShortLinksActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the short_links resource"]
    pub fn short_links(&self) -> crate::short_links::ShortLinksActions<A> {
        crate::short_links::ShortLinksActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the v_1 resource"]
    pub fn v_1(&self) -> crate::v_1::V1Actions<A> {
        crate::v_1::V1Actions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod managed_short_links {
    pub mod params {}
    pub struct ManagedShortLinksActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ManagedShortLinksActions<'a, A> {
        #[doc = "Creates a managed short Dynamic Link given either a valid long Dynamic Link\nor details such as Dynamic Link domain, Android and iOS app information.\nThe created short Dynamic Link will not expire.\n\nThis differs from CreateShortDynamicLink in the following ways:\n  - The request will also contain a name for the link (non unique name\n    for the front end).\n  - The response must be authenticated with an auth token (generated with\n    the admin service account).\n  - The link will appear in the FDL list of links in the console front end.\n\nThe Dynamic Link domain in the request must be owned by requester's\nFirebase project."]
        pub fn create(
            &self,
            request: crate::schemas::CreateManagedShortLinkRequest,
        ) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::CreateManagedShortLinkRequest,
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
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::CreateManagedShortLinkResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
            output.push_str("v1/managedShortLinks:create");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/firebase"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod short_links {
    pub mod params {}
    pub struct ShortLinksActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ShortLinksActions<'a, A> {
        #[doc = "Creates a short Dynamic Link given either a valid long Dynamic Link or\ndetails such as Dynamic Link domain, Android and iOS app information.\nThe created short Dynamic Link will not expire.\n\nRepeated calls with the same long Dynamic Link or Dynamic Link information\nwill produce the same short Dynamic Link.\n\nThe Dynamic Link domain in the request must be owned by requester's\nFirebase project."]
        pub fn create(
            &self,
            request: crate::schemas::CreateShortDynamicLinkRequest,
        ) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
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
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::CreateShortDynamicLinkRequest,
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
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::CreateShortDynamicLinkResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
            output.push_str("v1/shortLinks");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/firebase"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod v_1 {
    pub mod params {}
    pub struct V1Actions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> V1Actions<'a, A> {
        #[doc = "Fetches analytics stats of a short Dynamic Link for a given\nduration. Metrics include number of clicks, redirects, installs,\napp first opens, and app reopens."]
        pub fn get_link_stats(
            &self,
            dynamic_link: impl Into<String>,
        ) -> GetLinkStatsRequestBuilder<A> {
            GetLinkStatsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
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
                dynamic_link: dynamic_link.into(),
                duration_days: None,
                sdk_version: None,
            }
        }
        #[doc = "Get iOS strong/weak-match info for post-install attribution."]
        pub fn install_attribution(
            &self,
            request: crate::schemas::GetIosPostInstallAttributionRequest,
        ) -> InstallAttributionRequestBuilder<A> {
            InstallAttributionRequestBuilder {
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
            }
        }
        #[doc = "Get iOS reopen attribution for app universal link open deeplinking."]
        pub fn reopen_attribution(
            &self,
            request: crate::schemas::GetIosReopenAttributionRequest,
        ) -> ReopenAttributionRequestBuilder<A> {
            ReopenAttributionRequestBuilder {
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
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetLinkStatsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        dynamic_link: String,
        duration_days: Option<i64>,
        sdk_version: Option<String>,
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
    impl<'a, A: yup_oauth2::GetToken> GetLinkStatsRequestBuilder<'a, A> {
        #[doc = "The span of time requested in days."]
        pub fn duration_days(&mut self, value: i64) -> &mut Self {
            self.duration_days = Some(value);
            self
        }
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        pub fn sdk_version(&mut self, value: impl Into<String>) -> &mut Self {
            self.sdk_version = Some(value.into());
            self
        }
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
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::DynamicLinkStats, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
            output.push_str("v1/");
            output.push_str(&self.dynamic_link);
            output.push_str("/linkStats");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("durationDays", &self.duration_days)]);
            let req = req.query(&[("sdkVersion", &self.sdk_version)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/firebase"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InstallAttributionRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::GetIosPostInstallAttributionRequest,
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
    impl<'a, A: yup_oauth2::GetToken> InstallAttributionRequestBuilder<'a, A> {
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
        ) -> Result<
            crate::schemas::GetIosPostInstallAttributionResponse,
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
            let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
            output.push_str("v1/installAttribution");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/firebase"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct ReopenAttributionRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::GetIosReopenAttributionRequest,
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
    impl<'a, A: yup_oauth2::GetToken> ReopenAttributionRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::GetIosReopenAttributionResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
            output.push_str("v1/reopenAttribution");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/firebase"])
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
