#![doc = "# Resources and Methods\n    * [managed_short_links](resources/managed_short_links/struct.ManagedShortLinksActions.html)\n      * [*create*](resources/managed_short_links/struct.CreateRequestBuilder.html)\n    * [short_links](resources/short_links/struct.ShortLinksActions.html)\n      * [*create*](resources/short_links/struct.CreateRequestBuilder.html)\n    * [v_1](resources/v_1/struct.V1Actions.html)\n      * [*getLinkStats*](resources/v_1/struct.GetLinkStatsRequestBuilder.html), [*installAttribution*](resources/v_1/struct.InstallAttributionRequestBuilder.html), [*reopenAttribution*](resources/v_1/struct.ReopenAttributionRequestBuilder.html)\n"]
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
    pub struct AnalyticsInfo {
        #[doc = "Google Play Campaign Measurements."]
        #[serde(
            rename = "googlePlayAnalytics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_play_analytics: ::std::option::Option<crate::schemas::GooglePlayAnalytics>,
        #[doc = "iTunes Connect App Analytics."]
        #[serde(
            rename = "itunesConnectAnalytics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub itunes_connect_analytics: ::std::option::Option<crate::schemas::ItunesConnectAnalytics>,
    }
    impl ::google_field_selector::FieldSelector for AnalyticsInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnalyticsInfo {
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
    pub struct AndroidInfo {
        #[doc = "Link to open on Android if the app is not installed."]
        #[serde(
            rename = "androidFallbackLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_fallback_link: ::std::option::Option<String>,
        #[doc = "If specified, this overrides the \u{2018}link\u{2019} parameter on Android."]
        #[serde(
            rename = "androidLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_link: ::std::option::Option<String>,
        #[doc = "Minimum version code for the Android app. If the installed app\u{2019}s version\ncode is lower, then the user is taken to the Play Store."]
        #[serde(
            rename = "androidMinPackageVersionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_min_package_version_code: ::std::option::Option<String>,
        #[doc = "Android package name of the app."]
        #[serde(
            rename = "androidPackageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_package_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AndroidInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AndroidInfo {
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
    pub struct CreateManagedShortLinkRequest {
        #[doc = "Information about the Dynamic Link to be shortened.\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(
            rename = "dynamicLinkInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_link_info: ::std::option::Option<crate::schemas::DynamicLinkInfo>,
        #[doc = "Full long Dynamic Link URL with desired query parameters specified.\nFor example,\n\"https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample\",\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(
            rename = "longDynamicLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_dynamic_link: ::std::option::Option<String>,
        #[doc = "Link name to associate with the link. It's used for marketer to identify\nmanually-created links in the Firebase console\n(https://console.firebase.google.com/).\nLinks must be named to be tracked."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(
            rename = "sdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sdk_version: ::std::option::Option<String>,
        #[doc = "Short Dynamic Link suffix. Optional."]
        #[serde(
            rename = "suffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suffix: ::std::option::Option<crate::schemas::Suffix>,
    }
    impl ::google_field_selector::FieldSelector for CreateManagedShortLinkRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateManagedShortLinkRequest {
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
    pub struct CreateManagedShortLinkResponse {
        #[doc = "Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz"]
        #[serde(
            rename = "managedShortLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub managed_short_link: ::std::option::Option<crate::schemas::ManagedShortLink>,
        #[doc = "Preview link to show the link flow chart. (debug info.)"]
        #[serde(
            rename = "previewLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preview_link: ::std::option::Option<String>,
        #[doc = "Information about potential warnings on link creation."]
        #[serde(
            rename = "warning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning: ::std::option::Option<Vec<crate::schemas::DynamicLinkWarning>>,
    }
    impl ::google_field_selector::FieldSelector for CreateManagedShortLinkResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateManagedShortLinkResponse {
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
    pub struct CreateShortDynamicLinkRequest {
        #[doc = "Information about the Dynamic Link to be shortened.\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(
            rename = "dynamicLinkInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_link_info: ::std::option::Option<crate::schemas::DynamicLinkInfo>,
        #[doc = "Full long Dynamic Link URL with desired query parameters specified.\nFor example,\n\"https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample\",\n[Learn\nmore](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        #[serde(
            rename = "longDynamicLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_dynamic_link: ::std::option::Option<String>,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(
            rename = "sdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sdk_version: ::std::option::Option<String>,
        #[doc = "Short Dynamic Link suffix. Optional."]
        #[serde(
            rename = "suffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suffix: ::std::option::Option<crate::schemas::Suffix>,
    }
    impl ::google_field_selector::FieldSelector for CreateShortDynamicLinkRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateShortDynamicLinkRequest {
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
    pub struct CreateShortDynamicLinkResponse {
        #[doc = "Preview link to show the link flow chart. (debug info.)"]
        #[serde(
            rename = "previewLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preview_link: ::std::option::Option<String>,
        #[doc = "Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz"]
        #[serde(
            rename = "shortLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_link: ::std::option::Option<String>,
        #[doc = "Information about potential warnings on link creation."]
        #[serde(
            rename = "warning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning: ::std::option::Option<Vec<crate::schemas::DynamicLinkWarning>>,
    }
    impl ::google_field_selector::FieldSelector for CreateShortDynamicLinkResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateShortDynamicLinkResponse {
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
    pub struct DesktopInfo {
        #[doc = "Link to open on desktop."]
        #[serde(
            rename = "desktopFallbackLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub desktop_fallback_link: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DesktopInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DesktopInfo {
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
    pub struct DeviceInfo {
        #[doc = "Device model name."]
        #[serde(
            rename = "deviceModelName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_model_name: ::std::option::Option<String>,
        #[doc = "Device language code setting."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Device language code setting obtained by executing JavaScript code in\nWebView."]
        #[serde(
            rename = "languageCodeFromWebview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code_from_webview: ::std::option::Option<String>,
        #[doc = "Device language code raw setting.\niOS does returns language code in different format than iOS WebView.\nFor example WebView returns en_US, but iOS returns en-US.\nField below will return raw value returned by iOS."]
        #[serde(
            rename = "languageCodeRaw",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code_raw: ::std::option::Option<String>,
        #[doc = "Device display resolution height."]
        #[serde(
            rename = "screenResolutionHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub screen_resolution_height: ::std::option::Option<i64>,
        #[doc = "Device display resolution width."]
        #[serde(
            rename = "screenResolutionWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub screen_resolution_width: ::std::option::Option<i64>,
        #[doc = "Device timezone setting."]
        #[serde(
            rename = "timezone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timezone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeviceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceInfo {
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
    pub struct DynamicLinkEventStat {
        #[doc = "The number of times this event occurred."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Link event."]
        #[serde(
            rename = "event",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event: ::std::option::Option<crate::schemas::DynamicLinkEventStatEvent>,
        #[doc = "Requested platform."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<crate::schemas::DynamicLinkEventStatPlatform>,
    }
    impl ::google_field_selector::FieldSelector for DynamicLinkEventStat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicLinkEventStat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DynamicLinkEventStatEvent {
        #[doc = "Indicates that the app is opened for the first time after an install\ntriggered by FDLs"]
        AppFirstOpen,
        #[doc = "Indicates that an FDL triggers an app install from Play store, currently\nit's impossible to get stats from App store."]
        AppInstall,
        #[doc = "Indicates that the app is opened via an FDL for non-first time."]
        AppReOpen,
        #[doc = "Indicates that an FDL is clicked by users."]
        Click,
        #[doc = "Unspecified type."]
        DynamicLinkEventUnspecified,
        #[doc = "Indicates that an FDL redirects users to fallback link."]
        Redirect,
    }
    impl DynamicLinkEventStatEvent {
        pub fn as_str(self) -> &'static str {
            match self {
                DynamicLinkEventStatEvent::AppFirstOpen => "APP_FIRST_OPEN",
                DynamicLinkEventStatEvent::AppInstall => "APP_INSTALL",
                DynamicLinkEventStatEvent::AppReOpen => "APP_RE_OPEN",
                DynamicLinkEventStatEvent::Click => "CLICK",
                DynamicLinkEventStatEvent::DynamicLinkEventUnspecified => {
                    "DYNAMIC_LINK_EVENT_UNSPECIFIED"
                }
                DynamicLinkEventStatEvent::Redirect => "REDIRECT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DynamicLinkEventStatEvent {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DynamicLinkEventStatEvent {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DynamicLinkEventStatEvent, ()> {
            Ok(match s {
                "APP_FIRST_OPEN" => DynamicLinkEventStatEvent::AppFirstOpen,
                "APP_INSTALL" => DynamicLinkEventStatEvent::AppInstall,
                "APP_RE_OPEN" => DynamicLinkEventStatEvent::AppReOpen,
                "CLICK" => DynamicLinkEventStatEvent::Click,
                "DYNAMIC_LINK_EVENT_UNSPECIFIED" => {
                    DynamicLinkEventStatEvent::DynamicLinkEventUnspecified
                }
                "REDIRECT" => DynamicLinkEventStatEvent::Redirect,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DynamicLinkEventStatEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DynamicLinkEventStatEvent {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DynamicLinkEventStatEvent {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_FIRST_OPEN" => DynamicLinkEventStatEvent::AppFirstOpen,
                "APP_INSTALL" => DynamicLinkEventStatEvent::AppInstall,
                "APP_RE_OPEN" => DynamicLinkEventStatEvent::AppReOpen,
                "CLICK" => DynamicLinkEventStatEvent::Click,
                "DYNAMIC_LINK_EVENT_UNSPECIFIED" => {
                    DynamicLinkEventStatEvent::DynamicLinkEventUnspecified
                }
                "REDIRECT" => DynamicLinkEventStatEvent::Redirect,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DynamicLinkEventStatEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicLinkEventStatEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DynamicLinkEventStatPlatform {
        #[doc = "Represents Android platform.\nAll apps and browsers on Android are classfied in this category."]
        Android,
        #[doc = "Represents desktop."]
        Desktop,
        #[doc = "Unspecified platform."]
        DynamicLinkPlatformUnspecified,
        #[doc = "Represents iOS platform.\nAll apps and browsers on iOS are classfied in this category."]
        Ios,
        #[doc = "Platforms are not categorized as Android/iOS/Destop fall into here."]
        Other,
    }
    impl DynamicLinkEventStatPlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                DynamicLinkEventStatPlatform::Android => "ANDROID",
                DynamicLinkEventStatPlatform::Desktop => "DESKTOP",
                DynamicLinkEventStatPlatform::DynamicLinkPlatformUnspecified => {
                    "DYNAMIC_LINK_PLATFORM_UNSPECIFIED"
                }
                DynamicLinkEventStatPlatform::Ios => "IOS",
                DynamicLinkEventStatPlatform::Other => "OTHER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DynamicLinkEventStatPlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DynamicLinkEventStatPlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DynamicLinkEventStatPlatform, ()> {
            Ok(match s {
                "ANDROID" => DynamicLinkEventStatPlatform::Android,
                "DESKTOP" => DynamicLinkEventStatPlatform::Desktop,
                "DYNAMIC_LINK_PLATFORM_UNSPECIFIED" => {
                    DynamicLinkEventStatPlatform::DynamicLinkPlatformUnspecified
                }
                "IOS" => DynamicLinkEventStatPlatform::Ios,
                "OTHER" => DynamicLinkEventStatPlatform::Other,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DynamicLinkEventStatPlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DynamicLinkEventStatPlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DynamicLinkEventStatPlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => DynamicLinkEventStatPlatform::Android,
                "DESKTOP" => DynamicLinkEventStatPlatform::Desktop,
                "DYNAMIC_LINK_PLATFORM_UNSPECIFIED" => {
                    DynamicLinkEventStatPlatform::DynamicLinkPlatformUnspecified
                }
                "IOS" => DynamicLinkEventStatPlatform::Ios,
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
    impl ::google_field_selector::FieldSelector for DynamicLinkEventStatPlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicLinkEventStatPlatform {
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
    pub struct DynamicLinkInfo {
        #[doc = "Parameters used for tracking. See all tracking parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(
            rename = "analyticsInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analytics_info: ::std::option::Option<crate::schemas::AnalyticsInfo>,
        #[doc = "Android related information. See Android related parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(
            rename = "androidInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_info: ::std::option::Option<crate::schemas::AndroidInfo>,
        #[doc = "Desktop related information. See desktop related parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(
            rename = "desktopInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub desktop_info: ::std::option::Option<crate::schemas::DesktopInfo>,
        #[doc = "E.g. https://maps.app.goo.gl, https://maps.page.link, https://g.co/maps\nMore examples can be found in description of getNormalizedUriPrefix in\nj/c/g/firebase/dynamiclinks/uri/DdlDomain.java\n\nWill fallback to dynamic_link_domain is this field is missing"]
        #[serde(
            rename = "domainUriPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_uri_prefix: ::std::option::Option<String>,
        #[doc = "Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl\n[Learn\nmore](https://firebase.google.com/docs/dynamic-links/android/receive) on\nhow to set up Dynamic Link domain associated with your Firebase project.\n\nRequired if missing domain_uri_prefix."]
        #[serde(
            rename = "dynamicLinkDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dynamic_link_domain: ::std::option::Option<String>,
        #[doc = "iOS related information. See iOS related parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        #[serde(
            rename = "iosInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_info: ::std::option::Option<crate::schemas::IosInfo>,
        #[doc = "The link your app will open, You can specify any URL your app can handle.\nThis link must be a well-formatted URL, be properly URL-encoded, and use\nthe HTTP or HTTPS scheme. See 'link' parameters in the\n[documentation](https://firebase.google.com/docs/dynamic-links/create-manually).\n\nRequired."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "Information of navigation behavior of a Firebase Dynamic Links."]
        #[serde(
            rename = "navigationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub navigation_info: ::std::option::Option<crate::schemas::NavigationInfo>,
        #[doc = "Parameters for social meta tag params.\nUsed to set meta tag data for link previews on social sites."]
        #[serde(
            rename = "socialMetaTagInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub social_meta_tag_info: ::std::option::Option<crate::schemas::SocialMetaTagInfo>,
    }
    impl ::google_field_selector::FieldSelector for DynamicLinkInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicLinkInfo {
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
    pub struct DynamicLinkStats {
        #[doc = "Dynamic Link event stats."]
        #[serde(
            rename = "linkEventStats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_event_stats: ::std::option::Option<Vec<crate::schemas::DynamicLinkEventStat>>,
    }
    impl ::google_field_selector::FieldSelector for DynamicLinkStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicLinkStats {
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
    pub struct DynamicLinkWarning {
        #[doc = "The warning code."]
        #[serde(
            rename = "warningCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning_code: ::std::option::Option<crate::schemas::DynamicLinkWarningWarningCode>,
        #[doc = "The document describing the warning, and helps resolve."]
        #[serde(
            rename = "warningDocumentLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning_document_link: ::std::option::Option<String>,
        #[doc = "The warning message to help developers improve their requests."]
        #[serde(
            rename = "warningMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warning_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DynamicLinkWarning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicLinkWarning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DynamicLinkWarningWarningCode {
        #[doc = "isAd param format is incorrect."]
        BadAdParam,
        #[doc = "Debug param format is incorrect."]
        BadDebugParam,
        #[doc = "Android fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeAndroidFallbackLink,
        #[doc = "iOS fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeIosFallbackLink,
        #[doc = "iPad fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeIpadFallbackLink,
        #[doc = "Social meta tag image link has an invalid (non http/https) URI scheme."]
        BadUriSchemeSocialImageLink,
        BadUriSchemeSocialUrl,
        #[doc = "Unknown code."]
        CodeUnspecified,
        #[doc = "Indicates a certain param is deprecated."]
        DeprecatedParam,
        #[doc = "Dynamic Link URL length is too long."]
        LinkLengthTooLong,
        #[doc = "Dynamic Link URL contains fragments."]
        LinkWithFragments,
        #[doc = "The Android package does not match any in developer's DevConsole project."]
        NotInProjectAndroidPackageName,
        #[doc = "The iOS bundle ID does not match any in developer's DevConsole project."]
        NotInProjectIosBundleId,
        #[doc = "The iPad bundle ID does not match any in developer's DevConsole project."]
        NotInProjectIpadBundleId,
        #[doc = "The Android minimum version code has to be a valid integer."]
        NotIntegerAndroidPackageMinVersion,
        #[doc = "The iOS bundle ID does not match with the given iOS store ID."]
        NotMatchingIosBundleIdAndStoreId,
        #[doc = "iOS app store ID format is incorrect, e.g. not numeric."]
        NotNumericIosAppStoreId,
        #[doc = "Android fallback link is not a valid URI."]
        NotUriAndroidFallbackLink,
        #[doc = "Android link is not a valid URI."]
        NotUriAndroidLink,
        #[doc = "iOS fallback link is not a valid URI."]
        NotUriIosFallbackLink,
        #[doc = "iPad fallback link is not a valid URI."]
        NotUriIpadFallbackLink,
        #[doc = "Social meta tag image link is not a valid URI."]
        NotUriSocialImageLink,
        NotUriSocialUrl,
        #[doc = "Indicates certain paramater is too long."]
        TooLongParam,
        #[doc = "Android link param is not needed, e.g. when param 'al' and 'link' have\nthe same value.."]
        UnnecessaryAndroidLink,
        #[doc = "Android package min version param is not needed, e.g. when\n'apn' is missing."]
        UnnecessaryAndroidPackageMinVersion,
        #[doc = "iOS app store ID is not needed."]
        UnnecessaryIosAppStoreId,
        #[doc = "iOS URL scheme is not needed, e.g. when 'ibi' are 'ipbi' are all missing."]
        UnnecessaryIosUrlScheme,
        #[doc = "Indicates certain paramater is not recognized."]
        UnrecognizedParam,
    }
    impl DynamicLinkWarningWarningCode {
        pub fn as_str(self) -> &'static str {
            match self {
                DynamicLinkWarningWarningCode::BadAdParam => "BAD_AD_PARAM",
                DynamicLinkWarningWarningCode::BadDebugParam => "BAD_DEBUG_PARAM",
                DynamicLinkWarningWarningCode::BadUriSchemeAndroidFallbackLink => {
                    "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::BadUriSchemeIosFallbackLink => {
                    "BAD_URI_SCHEME_IOS_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::BadUriSchemeIpadFallbackLink => {
                    "BAD_URI_SCHEME_IPAD_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::BadUriSchemeSocialImageLink => {
                    "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK"
                }
                DynamicLinkWarningWarningCode::BadUriSchemeSocialUrl => "BAD_URI_SCHEME_SOCIAL_URL",
                DynamicLinkWarningWarningCode::CodeUnspecified => "CODE_UNSPECIFIED",
                DynamicLinkWarningWarningCode::DeprecatedParam => "DEPRECATED_PARAM",
                DynamicLinkWarningWarningCode::LinkLengthTooLong => "LINK_LENGTH_TOO_LONG",
                DynamicLinkWarningWarningCode::LinkWithFragments => "LINK_WITH_FRAGMENTS",
                DynamicLinkWarningWarningCode::NotInProjectAndroidPackageName => {
                    "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME"
                }
                DynamicLinkWarningWarningCode::NotInProjectIosBundleId => {
                    "NOT_IN_PROJECT_IOS_BUNDLE_ID"
                }
                DynamicLinkWarningWarningCode::NotInProjectIpadBundleId => {
                    "NOT_IN_PROJECT_IPAD_BUNDLE_ID"
                }
                DynamicLinkWarningWarningCode::NotIntegerAndroidPackageMinVersion => {
                    "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION"
                }
                DynamicLinkWarningWarningCode::NotMatchingIosBundleIdAndStoreId => {
                    "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID"
                }
                DynamicLinkWarningWarningCode::NotNumericIosAppStoreId => {
                    "NOT_NUMERIC_IOS_APP_STORE_ID"
                }
                DynamicLinkWarningWarningCode::NotUriAndroidFallbackLink => {
                    "NOT_URI_ANDROID_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::NotUriAndroidLink => "NOT_URI_ANDROID_LINK",
                DynamicLinkWarningWarningCode::NotUriIosFallbackLink => "NOT_URI_IOS_FALLBACK_LINK",
                DynamicLinkWarningWarningCode::NotUriIpadFallbackLink => {
                    "NOT_URI_IPAD_FALLBACK_LINK"
                }
                DynamicLinkWarningWarningCode::NotUriSocialImageLink => "NOT_URI_SOCIAL_IMAGE_LINK",
                DynamicLinkWarningWarningCode::NotUriSocialUrl => "NOT_URI_SOCIAL_URL",
                DynamicLinkWarningWarningCode::TooLongParam => "TOO_LONG_PARAM",
                DynamicLinkWarningWarningCode::UnnecessaryAndroidLink => "UNNECESSARY_ANDROID_LINK",
                DynamicLinkWarningWarningCode::UnnecessaryAndroidPackageMinVersion => {
                    "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION"
                }
                DynamicLinkWarningWarningCode::UnnecessaryIosAppStoreId => {
                    "UNNECESSARY_IOS_APP_STORE_ID"
                }
                DynamicLinkWarningWarningCode::UnnecessaryIosUrlScheme => {
                    "UNNECESSARY_IOS_URL_SCHEME"
                }
                DynamicLinkWarningWarningCode::UnrecognizedParam => "UNRECOGNIZED_PARAM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DynamicLinkWarningWarningCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DynamicLinkWarningWarningCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DynamicLinkWarningWarningCode, ()> {
            Ok(match s {
                "BAD_AD_PARAM" => DynamicLinkWarningWarningCode::BadAdParam,
                "BAD_DEBUG_PARAM" => DynamicLinkWarningWarningCode::BadDebugParam,
                "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeAndroidFallbackLink
                }
                "BAD_URI_SCHEME_IOS_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeIosFallbackLink
                }
                "BAD_URI_SCHEME_IPAD_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeIpadFallbackLink
                }
                "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeSocialImageLink
                }
                "BAD_URI_SCHEME_SOCIAL_URL" => DynamicLinkWarningWarningCode::BadUriSchemeSocialUrl,
                "CODE_UNSPECIFIED" => DynamicLinkWarningWarningCode::CodeUnspecified,
                "DEPRECATED_PARAM" => DynamicLinkWarningWarningCode::DeprecatedParam,
                "LINK_LENGTH_TOO_LONG" => DynamicLinkWarningWarningCode::LinkLengthTooLong,
                "LINK_WITH_FRAGMENTS" => DynamicLinkWarningWarningCode::LinkWithFragments,
                "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME" => {
                    DynamicLinkWarningWarningCode::NotInProjectAndroidPackageName
                }
                "NOT_IN_PROJECT_IOS_BUNDLE_ID" => {
                    DynamicLinkWarningWarningCode::NotInProjectIosBundleId
                }
                "NOT_IN_PROJECT_IPAD_BUNDLE_ID" => {
                    DynamicLinkWarningWarningCode::NotInProjectIpadBundleId
                }
                "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION" => {
                    DynamicLinkWarningWarningCode::NotIntegerAndroidPackageMinVersion
                }
                "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID" => {
                    DynamicLinkWarningWarningCode::NotMatchingIosBundleIdAndStoreId
                }
                "NOT_NUMERIC_IOS_APP_STORE_ID" => {
                    DynamicLinkWarningWarningCode::NotNumericIosAppStoreId
                }
                "NOT_URI_ANDROID_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::NotUriAndroidFallbackLink
                }
                "NOT_URI_ANDROID_LINK" => DynamicLinkWarningWarningCode::NotUriAndroidLink,
                "NOT_URI_IOS_FALLBACK_LINK" => DynamicLinkWarningWarningCode::NotUriIosFallbackLink,
                "NOT_URI_IPAD_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::NotUriIpadFallbackLink
                }
                "NOT_URI_SOCIAL_IMAGE_LINK" => DynamicLinkWarningWarningCode::NotUriSocialImageLink,
                "NOT_URI_SOCIAL_URL" => DynamicLinkWarningWarningCode::NotUriSocialUrl,
                "TOO_LONG_PARAM" => DynamicLinkWarningWarningCode::TooLongParam,
                "UNNECESSARY_ANDROID_LINK" => DynamicLinkWarningWarningCode::UnnecessaryAndroidLink,
                "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION" => {
                    DynamicLinkWarningWarningCode::UnnecessaryAndroidPackageMinVersion
                }
                "UNNECESSARY_IOS_APP_STORE_ID" => {
                    DynamicLinkWarningWarningCode::UnnecessaryIosAppStoreId
                }
                "UNNECESSARY_IOS_URL_SCHEME" => {
                    DynamicLinkWarningWarningCode::UnnecessaryIosUrlScheme
                }
                "UNRECOGNIZED_PARAM" => DynamicLinkWarningWarningCode::UnrecognizedParam,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DynamicLinkWarningWarningCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DynamicLinkWarningWarningCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DynamicLinkWarningWarningCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BAD_AD_PARAM" => DynamicLinkWarningWarningCode::BadAdParam,
                "BAD_DEBUG_PARAM" => DynamicLinkWarningWarningCode::BadDebugParam,
                "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeAndroidFallbackLink
                }
                "BAD_URI_SCHEME_IOS_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeIosFallbackLink
                }
                "BAD_URI_SCHEME_IPAD_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeIpadFallbackLink
                }
                "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK" => {
                    DynamicLinkWarningWarningCode::BadUriSchemeSocialImageLink
                }
                "BAD_URI_SCHEME_SOCIAL_URL" => DynamicLinkWarningWarningCode::BadUriSchemeSocialUrl,
                "CODE_UNSPECIFIED" => DynamicLinkWarningWarningCode::CodeUnspecified,
                "DEPRECATED_PARAM" => DynamicLinkWarningWarningCode::DeprecatedParam,
                "LINK_LENGTH_TOO_LONG" => DynamicLinkWarningWarningCode::LinkLengthTooLong,
                "LINK_WITH_FRAGMENTS" => DynamicLinkWarningWarningCode::LinkWithFragments,
                "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME" => {
                    DynamicLinkWarningWarningCode::NotInProjectAndroidPackageName
                }
                "NOT_IN_PROJECT_IOS_BUNDLE_ID" => {
                    DynamicLinkWarningWarningCode::NotInProjectIosBundleId
                }
                "NOT_IN_PROJECT_IPAD_BUNDLE_ID" => {
                    DynamicLinkWarningWarningCode::NotInProjectIpadBundleId
                }
                "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION" => {
                    DynamicLinkWarningWarningCode::NotIntegerAndroidPackageMinVersion
                }
                "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID" => {
                    DynamicLinkWarningWarningCode::NotMatchingIosBundleIdAndStoreId
                }
                "NOT_NUMERIC_IOS_APP_STORE_ID" => {
                    DynamicLinkWarningWarningCode::NotNumericIosAppStoreId
                }
                "NOT_URI_ANDROID_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::NotUriAndroidFallbackLink
                }
                "NOT_URI_ANDROID_LINK" => DynamicLinkWarningWarningCode::NotUriAndroidLink,
                "NOT_URI_IOS_FALLBACK_LINK" => DynamicLinkWarningWarningCode::NotUriIosFallbackLink,
                "NOT_URI_IPAD_FALLBACK_LINK" => {
                    DynamicLinkWarningWarningCode::NotUriIpadFallbackLink
                }
                "NOT_URI_SOCIAL_IMAGE_LINK" => DynamicLinkWarningWarningCode::NotUriSocialImageLink,
                "NOT_URI_SOCIAL_URL" => DynamicLinkWarningWarningCode::NotUriSocialUrl,
                "TOO_LONG_PARAM" => DynamicLinkWarningWarningCode::TooLongParam,
                "UNNECESSARY_ANDROID_LINK" => DynamicLinkWarningWarningCode::UnnecessaryAndroidLink,
                "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION" => {
                    DynamicLinkWarningWarningCode::UnnecessaryAndroidPackageMinVersion
                }
                "UNNECESSARY_IOS_APP_STORE_ID" => {
                    DynamicLinkWarningWarningCode::UnnecessaryIosAppStoreId
                }
                "UNNECESSARY_IOS_URL_SCHEME" => {
                    DynamicLinkWarningWarningCode::UnnecessaryIosUrlScheme
                }
                "UNRECOGNIZED_PARAM" => DynamicLinkWarningWarningCode::UnrecognizedParam,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DynamicLinkWarningWarningCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamicLinkWarningWarningCode {
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
    pub struct GetIosPostInstallAttributionRequest {
        #[doc = "App installation epoch time (https://en.wikipedia.org/wiki/Unix_time).\nThis is a client signal for a more accurate weak match."]
        #[serde(
            rename = "appInstallationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub app_installation_time: ::std::option::Option<i64>,
        #[doc = "APP bundle ID."]
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<String>,
        #[doc = "Device information."]
        #[serde(
            rename = "device",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device: ::std::option::Option<crate::schemas::DeviceInfo>,
        #[doc = "iOS version, ie: 9.3.5.\nConsider adding \"build\"."]
        #[serde(
            rename = "iosVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_version: ::std::option::Option<String>,
        #[doc = "App post install attribution retrieval information. Disambiguates\nmechanism (iSDK or developer invoked) to retrieve payload from\nclicked link."]
        #[serde(
            rename = "retrievalMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub retrieval_method: ::std::option::Option<
            crate::schemas::GetIosPostInstallAttributionRequestRetrievalMethod,
        >,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(
            rename = "sdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sdk_version: ::std::option::Option<String>,
        #[doc = "Possible unique matched link that server need to check before performing\nfingerprint match. If passed link is short server need to expand the link.\nIf link is long server need to vslidate the link."]
        #[serde(
            rename = "uniqueMatchLinkToCheck",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unique_match_link_to_check: ::std::option::Option<String>,
        #[doc = "Strong match page information. Disambiguates between default UI and\ncustom page to present when strong match succeeds/fails to find cookie."]
        #[serde(
            rename = "visualStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visual_style:
            ::std::option::Option<crate::schemas::GetIosPostInstallAttributionRequestVisualStyle>,
    }
    impl ::google_field_selector::FieldSelector for GetIosPostInstallAttributionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIosPostInstallAttributionRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionRequestRetrievalMethod {
        #[doc = "iSDK performs a strong match only if weak match is found upon a dev\nAPI call."]
        ExplicitStrongAfterWeakMatch,
        #[doc = "iSDK performs a server lookup by device fingerprint upon a dev API call."]
        ExplicitWeakMatch,
        #[doc = "iSDK performs a server lookup by device fingerprint in the background\nwhen app is first-opened; no API called by developer."]
        ImplicitWeakMatch,
        #[doc = "Unknown method."]
        UnknownPayloadRetrievalMethod,
    }
    impl GetIosPostInstallAttributionRequestRetrievalMethod {
        pub fn as_str(self) -> &'static str {
            match self { GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitStrongAfterWeakMatch => "EXPLICIT_STRONG_AFTER_WEAK_MATCH" , GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitWeakMatch => "EXPLICIT_WEAK_MATCH" , GetIosPostInstallAttributionRequestRetrievalMethod :: ImplicitWeakMatch => "IMPLICIT_WEAK_MATCH" , GetIosPostInstallAttributionRequestRetrievalMethod :: UnknownPayloadRetrievalMethod => "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD" , }
        }
    }
    impl ::std::convert::AsRef<str> for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GetIosPostInstallAttributionRequestRetrievalMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GetIosPostInstallAttributionRequestRetrievalMethod, ()> {
            Ok ( match s { "EXPLICIT_STRONG_AFTER_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitStrongAfterWeakMatch , "EXPLICIT_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitWeakMatch , "IMPLICIT_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ImplicitWeakMatch , "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD" => GetIosPostInstallAttributionRequestRetrievalMethod :: UnknownPayloadRetrievalMethod , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "EXPLICIT_STRONG_AFTER_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitStrongAfterWeakMatch , "EXPLICIT_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ExplicitWeakMatch , "IMPLICIT_WEAK_MATCH" => GetIosPostInstallAttributionRequestRetrievalMethod :: ImplicitWeakMatch , "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD" => GetIosPostInstallAttributionRequestRetrievalMethod :: UnknownPayloadRetrievalMethod , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIosPostInstallAttributionRequestRetrievalMethod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionRequestVisualStyle {
        #[doc = "Custom style."]
        CustomStyle,
        #[doc = "Default style."]
        DefaultStyle,
        #[doc = "Unknown style."]
        UnknownVisualStyle,
    }
    impl GetIosPostInstallAttributionRequestVisualStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                GetIosPostInstallAttributionRequestVisualStyle::CustomStyle => "CUSTOM_STYLE",
                GetIosPostInstallAttributionRequestVisualStyle::DefaultStyle => "DEFAULT_STYLE",
                GetIosPostInstallAttributionRequestVisualStyle::UnknownVisualStyle => {
                    "UNKNOWN_VISUAL_STYLE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GetIosPostInstallAttributionRequestVisualStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GetIosPostInstallAttributionRequestVisualStyle {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GetIosPostInstallAttributionRequestVisualStyle, ()> {
            Ok(match s {
                "CUSTOM_STYLE" => GetIosPostInstallAttributionRequestVisualStyle::CustomStyle,
                "DEFAULT_STYLE" => GetIosPostInstallAttributionRequestVisualStyle::DefaultStyle,
                "UNKNOWN_VISUAL_STYLE" => {
                    GetIosPostInstallAttributionRequestVisualStyle::UnknownVisualStyle
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionRequestVisualStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionRequestVisualStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionRequestVisualStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM_STYLE" => GetIosPostInstallAttributionRequestVisualStyle::CustomStyle,
                "DEFAULT_STYLE" => GetIosPostInstallAttributionRequestVisualStyle::DefaultStyle,
                "UNKNOWN_VISUAL_STYLE" => {
                    GetIosPostInstallAttributionRequestVisualStyle::UnknownVisualStyle
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
    impl ::google_field_selector::FieldSelector for GetIosPostInstallAttributionRequestVisualStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIosPostInstallAttributionRequestVisualStyle {
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
    pub struct GetIosPostInstallAttributionResponse {
        #[doc = "The minimum version for app, specified by dev through ?imv= parameter.\nReturn to iSDK to allow app to evaluate if current version meets this."]
        #[serde(
            rename = "appMinimumVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_minimum_version: ::std::option::Option<String>,
        #[doc = "The confidence of the returned attribution."]
        #[serde(
            rename = "attributionConfidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribution_confidence: ::std::option::Option<
            crate::schemas::GetIosPostInstallAttributionResponseAttributionConfidence,
        >,
        #[doc = "The deep-link attributed post-install via one of several techniques\n(fingerprint, copy unique)."]
        #[serde(
            rename = "deepLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deep_link: ::std::option::Option<String>,
        #[doc = "User-agent specific custom-scheme URIs for iSDK to open. This will be set\naccording to the user-agent tha the click was originally made in. There is\nno Safari-equivalent custom-scheme open URLs.\nie: googlechrome://www.example.com\nie: firefox://open-url?url=http://www.example.com\nie: opera-http://example.com"]
        #[serde(
            rename = "externalBrowserDestinationLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_browser_destination_link: ::std::option::Option<String>,
        #[doc = "The link to navigate to update the app if min version is not met.\nThis is either (in order): 1) fallback link (from ?ifl= parameter, if\nspecified by developer) or 2) AppStore URL (from ?isi= parameter, if\nspecified), or 3) the payload link (from required link= parameter)."]
        #[serde(
            rename = "fallbackLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fallback_link: ::std::option::Option<String>,
        #[doc = "Invitation ID attributed post-install via one of several techniques\n(fingerprint, copy unique)."]
        #[serde(
            rename = "invitationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invitation_id: ::std::option::Option<String>,
        #[doc = "Instruction for iSDK to attemmpt to perform strong match. For instance,\nif browser does not support/allow cookie or outside of support browsers,\nthis will be false."]
        #[serde(
            rename = "isStrongMatchExecutable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_strong_match_executable: ::std::option::Option<bool>,
        #[doc = "Describes why match failed, ie: \"discarded due to low confidence\".\nThis message will be publicly visible."]
        #[serde(
            rename = "matchMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_message: ::std::option::Option<String>,
        #[doc = "Which IP version the request was made from."]
        #[serde(
            rename = "requestIpVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_ip_version: ::std::option::Option<
            crate::schemas::GetIosPostInstallAttributionResponseRequestIpVersion,
        >,
        #[doc = "Entire FDL (short or long) attributed post-install via one of several\ntechniques (fingerprint, copy unique)."]
        #[serde(
            rename = "requestedLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_link: ::std::option::Option<String>,
        #[doc = "The entire FDL, expanded from a short link. It is the same as the\nrequested_link, if it is long. Parameters from this should not be\nused directly (ie: server can default utm_[campaign|medium|source]\nto a value when requested_link lack them, server determine the best\nfallback_link when requested_link specifies >1 fallback links)."]
        #[serde(
            rename = "resolvedLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolved_link: ::std::option::Option<String>,
        #[doc = "Scion campaign value to be propagated by iSDK to Scion at post-install."]
        #[serde(
            rename = "utmCampaign",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_campaign: ::std::option::Option<String>,
        #[doc = "Scion content value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(
            rename = "utmContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_content: ::std::option::Option<String>,
        #[doc = "Scion medium value to be propagated by iSDK to Scion at post-install."]
        #[serde(
            rename = "utmMedium",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_medium: ::std::option::Option<String>,
        #[doc = "Scion source value to be propagated by iSDK to Scion at post-install."]
        #[serde(
            rename = "utmSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_source: ::std::option::Option<String>,
        #[doc = "Scion term value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(
            rename = "utmTerm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_term: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GetIosPostInstallAttributionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIosPostInstallAttributionResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionResponseAttributionConfidence {
        #[doc = "Default confidence, match based on fingerprint"]
        Default,
        #[doc = "Unique confidence, match based on \"unique match link to check\" or other\nmeans"]
        Unique,
        #[doc = "Unset."]
        UnknownAttributionConfidence,
        #[doc = "Weak confidence, more than one matching link found or link suspected to\nbe false positive"]
        Weak,
    }
    impl GetIosPostInstallAttributionResponseAttributionConfidence {
        pub fn as_str(self) -> &'static str {
            match self { GetIosPostInstallAttributionResponseAttributionConfidence :: Default => "DEFAULT" , GetIosPostInstallAttributionResponseAttributionConfidence :: Unique => "UNIQUE" , GetIosPostInstallAttributionResponseAttributionConfidence :: UnknownAttributionConfidence => "UNKNOWN_ATTRIBUTION_CONFIDENCE" , GetIosPostInstallAttributionResponseAttributionConfidence :: Weak => "WEAK" , }
        }
    }
    impl ::std::convert::AsRef<str> for GetIosPostInstallAttributionResponseAttributionConfidence {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GetIosPostInstallAttributionResponseAttributionConfidence {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GetIosPostInstallAttributionResponseAttributionConfidence, ()>
        {
            Ok ( match s { "DEFAULT" => GetIosPostInstallAttributionResponseAttributionConfidence :: Default , "UNIQUE" => GetIosPostInstallAttributionResponseAttributionConfidence :: Unique , "UNKNOWN_ATTRIBUTION_CONFIDENCE" => GetIosPostInstallAttributionResponseAttributionConfidence :: UnknownAttributionConfidence , "WEAK" => GetIosPostInstallAttributionResponseAttributionConfidence :: Weak , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionResponseAttributionConfidence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionResponseAttributionConfidence {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionResponseAttributionConfidence {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "DEFAULT" => GetIosPostInstallAttributionResponseAttributionConfidence :: Default , "UNIQUE" => GetIosPostInstallAttributionResponseAttributionConfidence :: Unique , "UNKNOWN_ATTRIBUTION_CONFIDENCE" => GetIosPostInstallAttributionResponseAttributionConfidence :: UnknownAttributionConfidence , "WEAK" => GetIosPostInstallAttributionResponseAttributionConfidence :: Weak , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector
        for GetIosPostInstallAttributionResponseAttributionConfidence
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GetIosPostInstallAttributionResponseAttributionConfidence
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GetIosPostInstallAttributionResponseRequestIpVersion {
        #[doc = "Request made from an IPv4 IP address."]
        IpV4,
        #[doc = "Request made from an IPv6 IP address."]
        IpV6,
        #[doc = "Unset."]
        UnknownIpVersion,
    }
    impl GetIosPostInstallAttributionResponseRequestIpVersion {
        pub fn as_str(self) -> &'static str {
            match self {
                GetIosPostInstallAttributionResponseRequestIpVersion::IpV4 => "IP_V4",
                GetIosPostInstallAttributionResponseRequestIpVersion::IpV6 => "IP_V6",
                GetIosPostInstallAttributionResponseRequestIpVersion::UnknownIpVersion => {
                    "UNKNOWN_IP_VERSION"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GetIosPostInstallAttributionResponseRequestIpVersion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GetIosPostInstallAttributionResponseRequestIpVersion {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GetIosPostInstallAttributionResponseRequestIpVersion, ()>
        {
            Ok(match s {
                "IP_V4" => GetIosPostInstallAttributionResponseRequestIpVersion::IpV4,
                "IP_V6" => GetIosPostInstallAttributionResponseRequestIpVersion::IpV6,
                "UNKNOWN_IP_VERSION" => {
                    GetIosPostInstallAttributionResponseRequestIpVersion::UnknownIpVersion
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GetIosPostInstallAttributionResponseRequestIpVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GetIosPostInstallAttributionResponseRequestIpVersion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GetIosPostInstallAttributionResponseRequestIpVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IP_V4" => GetIosPostInstallAttributionResponseRequestIpVersion::IpV4,
                "IP_V6" => GetIosPostInstallAttributionResponseRequestIpVersion::IpV6,
                "UNKNOWN_IP_VERSION" => {
                    GetIosPostInstallAttributionResponseRequestIpVersion::UnknownIpVersion
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
    impl ::google_field_selector::FieldSelector
        for GetIosPostInstallAttributionResponseRequestIpVersion
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIosPostInstallAttributionResponseRequestIpVersion {
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
    pub struct GetIosReopenAttributionRequest {
        #[doc = "APP bundle ID."]
        #[serde(
            rename = "bundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bundle_id: ::std::option::Option<String>,
        #[doc = "FDL link to be verified from an app universal link open.\nThe FDL link can be one of:\n\n1. short FDL.\n   e.g. <app_code>.page.link/<ddl_id>, or\n1. long FDL.\n   e.g. <app_code>.page.link/?{query params}, or\n1. Invite FDL.\n   e.g. <app_code>.page.link/i/<invite_id_or_alias>"]
        #[serde(
            rename = "requestedLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_link: ::std::option::Option<String>,
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        #[serde(
            rename = "sdkVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sdk_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GetIosReopenAttributionRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIosReopenAttributionRequest {
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
    pub struct GetIosReopenAttributionResponse {
        #[doc = "The deep-link attributed the app universal link open. For both regular\nFDL links and invite FDL links."]
        #[serde(
            rename = "deepLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deep_link: ::std::option::Option<String>,
        #[doc = "Optional invitation ID, for only invite typed requested FDL links."]
        #[serde(
            rename = "invitationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invitation_id: ::std::option::Option<String>,
        #[doc = "FDL input value of the \"&imv=\" parameter, minimum app version to be\nreturned to Google Firebase SDK running on iOS-9."]
        #[serde(
            rename = "iosMinAppVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_min_app_version: ::std::option::Option<String>,
        #[doc = "The entire FDL, expanded from a short link. It is the same as the\nrequested_link, if it is long."]
        #[serde(
            rename = "resolvedLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolved_link: ::std::option::Option<String>,
        #[doc = "Scion campaign value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(
            rename = "utmCampaign",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_campaign: ::std::option::Option<String>,
        #[doc = "Scion content value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(
            rename = "utmContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_content: ::std::option::Option<String>,
        #[doc = "Scion medium value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(
            rename = "utmMedium",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_medium: ::std::option::Option<String>,
        #[doc = "Scion source value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(
            rename = "utmSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_source: ::std::option::Option<String>,
        #[doc = "Scion term value to be propagated by iSDK to Scion at app-reopen."]
        #[serde(
            rename = "utmTerm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_term: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GetIosReopenAttributionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIosReopenAttributionResponse {
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
    pub struct GooglePlayAnalytics {
        #[doc = "[AdWords autotagging\nparameter](https://support.google.com/analytics/answer/1033981?hl=en); used\nto measure Google AdWords ads. This value is generated dynamically and\nshould never be modified."]
        #[serde(
            rename = "gclid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gclid: ::std::option::Option<String>,
        #[doc = "Campaign name; used for keyword analysis to identify a specific product\npromotion or strategic campaign."]
        #[serde(
            rename = "utmCampaign",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_campaign: ::std::option::Option<String>,
        #[doc = "Campaign content; used for A/B testing and content-targeted ads to\ndifferentiate ads or links that point to the same URL."]
        #[serde(
            rename = "utmContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_content: ::std::option::Option<String>,
        #[doc = "Campaign medium; used to identify a medium such as email or cost-per-click."]
        #[serde(
            rename = "utmMedium",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_medium: ::std::option::Option<String>,
        #[doc = "Campaign source; used to identify a search engine, newsletter, or other\nsource."]
        #[serde(
            rename = "utmSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_source: ::std::option::Option<String>,
        #[doc = "Campaign term; used with paid search to supply the keywords for ads."]
        #[serde(
            rename = "utmTerm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub utm_term: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GooglePlayAnalytics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GooglePlayAnalytics {
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
    pub struct IosInfo {
        #[doc = "iOS App Store ID."]
        #[serde(
            rename = "iosAppStoreId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_app_store_id: ::std::option::Option<String>,
        #[doc = "iOS bundle ID of the app."]
        #[serde(
            rename = "iosBundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_bundle_id: ::std::option::Option<String>,
        #[doc = "Custom (destination) scheme to use for iOS. By default, we\u{2019}ll use the\nbundle ID as the custom scheme. Developer can override this behavior using\nthis param."]
        #[serde(
            rename = "iosCustomScheme",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_custom_scheme: ::std::option::Option<String>,
        #[doc = "Link to open on iOS if the app is not installed."]
        #[serde(
            rename = "iosFallbackLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_fallback_link: ::std::option::Option<String>,
        #[doc = "iPad bundle ID of the app."]
        #[serde(
            rename = "iosIpadBundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_ipad_bundle_id: ::std::option::Option<String>,
        #[doc = "If specified, this overrides the ios_fallback_link value on iPads."]
        #[serde(
            rename = "iosIpadFallbackLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_ipad_fallback_link: ::std::option::Option<String>,
        #[doc = "iOS minimum version."]
        #[serde(
            rename = "iosMinimumVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_minimum_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IosInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IosInfo {
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
    pub struct ItunesConnectAnalytics {
        #[doc = "Affiliate token used to create affiliate-coded links."]
        #[serde(
            rename = "at",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub at: ::std::option::Option<String>,
        #[doc = "Campaign text that developers can optionally add to any link in order to\ntrack sales from a specific marketing campaign."]
        #[serde(
            rename = "ct",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ct: ::std::option::Option<String>,
        #[doc = "iTune media types, including music, podcasts, audiobooks and so on."]
        #[serde(
            rename = "mt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mt: ::std::option::Option<String>,
        #[doc = "Provider token that enables analytics for Dynamic Links from within iTunes\nConnect."]
        #[serde(
            rename = "pt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pt: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ItunesConnectAnalytics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItunesConnectAnalytics {
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
    pub struct ManagedShortLink {
        #[doc = "Creation timestamp of the short link."]
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creation_time: ::std::option::Option<String>,
        #[doc = "Attributes that have been flagged about this short url."]
        #[serde(
            rename = "flaggedAttribute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flagged_attribute:
            ::std::option::Option<Vec<crate::schemas::ManagedShortLinkFlaggedAttributeItems>>,
        #[doc = "Full Dyamic Link info"]
        #[serde(
            rename = "info",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub info: ::std::option::Option<crate::schemas::DynamicLinkInfo>,
        #[doc = "Short durable link url, for example, \"https://sample.app.goo.gl/xyz123\".\n\nRequired."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "Link name defined by the creator.\n\nRequired."]
        #[serde(
            rename = "linkName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_name: ::std::option::Option<String>,
        #[doc = "Visibility status of link."]
        #[serde(
            rename = "visibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visibility: ::std::option::Option<crate::schemas::ManagedShortLinkVisibility>,
    }
    impl ::google_field_selector::FieldSelector for ManagedShortLink {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedShortLink {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManagedShortLinkFlaggedAttributeItems {
        Spam,
        UnspecifiedAttribute,
    }
    impl ManagedShortLinkFlaggedAttributeItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedShortLinkFlaggedAttributeItems::Spam => "SPAM",
                ManagedShortLinkFlaggedAttributeItems::UnspecifiedAttribute => {
                    "UNSPECIFIED_ATTRIBUTE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManagedShortLinkFlaggedAttributeItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManagedShortLinkFlaggedAttributeItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ManagedShortLinkFlaggedAttributeItems, ()> {
            Ok(match s {
                "SPAM" => ManagedShortLinkFlaggedAttributeItems::Spam,
                "UNSPECIFIED_ATTRIBUTE" => {
                    ManagedShortLinkFlaggedAttributeItems::UnspecifiedAttribute
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManagedShortLinkFlaggedAttributeItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedShortLinkFlaggedAttributeItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedShortLinkFlaggedAttributeItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SPAM" => ManagedShortLinkFlaggedAttributeItems::Spam,
                "UNSPECIFIED_ATTRIBUTE" => {
                    ManagedShortLinkFlaggedAttributeItems::UnspecifiedAttribute
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
    impl ::google_field_selector::FieldSelector for ManagedShortLinkFlaggedAttributeItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedShortLinkFlaggedAttributeItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManagedShortLinkVisibility {
        #[doc = "Link created in console and should not be shown in console (but can\nbe shown in the console again if it is unarchived)."]
        Archived,
        #[doc = "Link created outside of console and should never be shown in console."]
        NeverShown,
        #[doc = "Link created in console and should be shown in console."]
        Unarchived,
        #[doc = "Visibility of the link is not specified."]
        UnspecifiedVisibility,
    }
    impl ManagedShortLinkVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                ManagedShortLinkVisibility::Archived => "ARCHIVED",
                ManagedShortLinkVisibility::NeverShown => "NEVER_SHOWN",
                ManagedShortLinkVisibility::Unarchived => "UNARCHIVED",
                ManagedShortLinkVisibility::UnspecifiedVisibility => "UNSPECIFIED_VISIBILITY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManagedShortLinkVisibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManagedShortLinkVisibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ManagedShortLinkVisibility, ()> {
            Ok(match s {
                "ARCHIVED" => ManagedShortLinkVisibility::Archived,
                "NEVER_SHOWN" => ManagedShortLinkVisibility::NeverShown,
                "UNARCHIVED" => ManagedShortLinkVisibility::Unarchived,
                "UNSPECIFIED_VISIBILITY" => ManagedShortLinkVisibility::UnspecifiedVisibility,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManagedShortLinkVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManagedShortLinkVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManagedShortLinkVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHIVED" => ManagedShortLinkVisibility::Archived,
                "NEVER_SHOWN" => ManagedShortLinkVisibility::NeverShown,
                "UNARCHIVED" => ManagedShortLinkVisibility::Unarchived,
                "UNSPECIFIED_VISIBILITY" => ManagedShortLinkVisibility::UnspecifiedVisibility,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ManagedShortLinkVisibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManagedShortLinkVisibility {
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
    pub struct NavigationInfo {
        #[doc = "If this option is on, FDL click will be forced to redirect rather than\nshow an interstitial page."]
        #[serde(
            rename = "enableForcedRedirect",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_forced_redirect: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for NavigationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NavigationInfo {
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
    pub struct SocialMetaTagInfo {
        #[doc = "A short description of the link. Optional."]
        #[serde(
            rename = "socialDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub social_description: ::std::option::Option<String>,
        #[doc = "An image url string. Optional."]
        #[serde(
            rename = "socialImageLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub social_image_link: ::std::option::Option<String>,
        #[doc = "Title to be displayed. Optional."]
        #[serde(
            rename = "socialTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub social_title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SocialMetaTagInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SocialMetaTagInfo {
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
    pub struct Suffix {
        #[doc = "Only applies to Option.CUSTOM."]
        #[serde(
            rename = "customSuffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_suffix: ::std::option::Option<String>,
        #[doc = "Suffix option."]
        #[serde(
            rename = "option",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub option: ::std::option::Option<crate::schemas::SuffixOption>,
    }
    impl ::google_field_selector::FieldSelector for Suffix {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Suffix {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SuffixOption {
        #[doc = "Custom DDL suffix is a client specified string, for example,\n\"buy2get1free\".\nNOTE: custom suffix should only be available to managed short link\ncreation"]
        Custom,
        #[doc = "The suffix option is not specified, performs as UNGUESSABLE ."]
        OptionUnspecified,
        #[doc = "Short Dynamic Link suffix is a base62 [0-9A-Za-z] string starting with a\nlength of 4 chars. the length will increase when all the space is\noccupied."]
        Short,
        #[doc = "Short Dynamic Link suffix is a base62 [0-9A-Za-z] encoded string of\na random generated 96 bit random number, which has a length of 17 chars.\nFor example, \"nlAR8U4SlKRZw1cb2\".\nIt prevents other people from guessing and crawling short Dynamic Links\nthat contain personal identifiable information."]
        Unguessable,
    }
    impl SuffixOption {
        pub fn as_str(self) -> &'static str {
            match self {
                SuffixOption::Custom => "CUSTOM",
                SuffixOption::OptionUnspecified => "OPTION_UNSPECIFIED",
                SuffixOption::Short => "SHORT",
                SuffixOption::Unguessable => "UNGUESSABLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SuffixOption {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SuffixOption {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SuffixOption, ()> {
            Ok(match s {
                "CUSTOM" => SuffixOption::Custom,
                "OPTION_UNSPECIFIED" => SuffixOption::OptionUnspecified,
                "SHORT" => SuffixOption::Short,
                "UNGUESSABLE" => SuffixOption::Unguessable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SuffixOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SuffixOption {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuffixOption {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM" => SuffixOption::Custom,
                "OPTION_UNSPECIFIED" => SuffixOption::OptionUnspecified,
                "SHORT" => SuffixOption::Short,
                "UNGUESSABLE" => SuffixOption::Unguessable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SuffixOption {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuffixOption {
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
    #[doc = "Actions that can be performed on the managed_short_links resource"]
    pub fn managed_short_links(
        &self,
    ) -> crate::resources::managed_short_links::ManagedShortLinksActions {
        crate::resources::managed_short_links::ManagedShortLinksActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the short_links resource"]
    pub fn short_links(&self) -> crate::resources::short_links::ShortLinksActions {
        crate::resources::short_links::ShortLinksActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the v_1 resource"]
    pub fn v_1(&self) -> crate::resources::v_1::V1Actions {
        crate::resources::v_1::V1Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod managed_short_links {
        pub mod params {}
        pub struct ManagedShortLinksActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ManagedShortLinksActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a managed short Dynamic Link given either a valid long Dynamic Link\nor details such as Dynamic Link domain, Android and iOS app information.\nThe created short Dynamic Link will not expire.\n\nThis differs from CreateShortDynamicLink in the following ways:\n\n* The request will also contain a name for the link (non unique name\n  for the front end).\n* The response must be authenticated with an auth token (generated with\n  the admin service account).\n* The link will appear in the FDL list of links in the console front end.\n\nThe Dynamic Link domain in the request must be owned by requester's\nFirebase project."]
            pub fn create(
                &self,
                request: crate::schemas::CreateManagedShortLinkRequest,
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
        }
        #[doc = "Created via [ManagedShortLinksActions::create()](struct.ManagedShortLinksActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            ) -> Result<crate::schemas::CreateManagedShortLinkResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CreateManagedShortLinkResponse, crate::Error> {
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
                let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
                output.push_str("v1/managedShortLinks:create");
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
    pub mod short_links {
        pub mod params {}
        pub struct ShortLinksActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ShortLinksActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a short Dynamic Link given either a valid long Dynamic Link or\ndetails such as Dynamic Link domain, Android and iOS app information.\nThe created short Dynamic Link will not expire.\n\nRepeated calls with the same long Dynamic Link or Dynamic Link information\nwill produce the same short Dynamic Link.\n\nThe Dynamic Link domain in the request must be owned by requester's\nFirebase project."]
            pub fn create(
                &self,
                request: crate::schemas::CreateShortDynamicLinkRequest,
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
        }
        #[doc = "Created via [ShortLinksActions::create()](struct.ShortLinksActions.html#method.create)"]
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            ) -> Result<crate::schemas::CreateShortDynamicLinkResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CreateShortDynamicLinkResponse, crate::Error> {
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
                let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
                output.push_str("v1/shortLinks");
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
    pub mod v_1 {
        pub mod params {}
        pub struct V1Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Fetches analytics stats of a short Dynamic Link for a given\nduration. Metrics include number of clicks, redirects, installs,\napp first opens, and app reopens."]
            pub fn get_link_stats(
                &self,
                dynamic_link: impl Into<String>,
            ) -> GetLinkStatsRequestBuilder {
                GetLinkStatsRequestBuilder {
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
                    dynamic_link: dynamic_link.into(),
                    duration_days: None,
                    sdk_version: None,
                }
            }
            #[doc = "Get iOS strong/weak-match info for post-install attribution."]
            pub fn install_attribution(
                &self,
                request: crate::schemas::GetIosPostInstallAttributionRequest,
            ) -> InstallAttributionRequestBuilder {
                InstallAttributionRequestBuilder {
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
            #[doc = "Get iOS reopen attribution for app universal link open deeplinking."]
            pub fn reopen_attribution(
                &self,
                request: crate::schemas::GetIosReopenAttributionRequest,
            ) -> ReopenAttributionRequestBuilder {
                ReopenAttributionRequestBuilder {
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
        #[doc = "Created via [V1Actions::get_link_stats()](struct.V1Actions.html#method.get_link_stats)"]
        #[derive(Debug, Clone)]
        pub struct GetLinkStatsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> GetLinkStatsRequestBuilder<'a> {
            #[doc = "The span of time requested in days."]
            pub fn duration_days(mut self, value: i64) -> Self {
                self.duration_days = Some(value);
                self
            }
            #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
            pub fn sdk_version(mut self, value: impl Into<String>) -> Self {
                self.sdk_version = Some(value.into());
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
            ) -> Result<crate::schemas::DynamicLinkStats, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DynamicLinkStats, crate::Error> {
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
                let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.dynamic_link;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/linkStats");
                output
            }
            fn _request(&self, path: &str) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [V1Actions::install_attribution()](struct.V1Actions.html#method.install_attribution)"]
        #[derive(Debug, Clone)]
        pub struct InstallAttributionRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> InstallAttributionRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GetIosPostInstallAttributionResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetIosPostInstallAttributionResponse, crate::Error>
            {
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
                let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
                output.push_str("v1/installAttribution");
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
        #[doc = "Created via [V1Actions::reopen_attribution()](struct.V1Actions.html#method.reopen_attribution)"]
        #[derive(Debug, Clone)]
        pub struct ReopenAttributionRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ReopenAttributionRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GetIosReopenAttributionResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetIosReopenAttributionResponse, crate::Error> {
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
                let mut output = "https://firebasedynamiclinks.googleapis.com/".to_owned();
                output.push_str("v1/reopenAttribution");
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
