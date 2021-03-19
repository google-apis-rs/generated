#![doc = "# Resources and Methods\n    * [customers](resources/customers/struct.CustomersActions.html)\n      * [reports](resources/customers/reports/struct.ReportsActions.html)\n        * [*countChromeVersions*](resources/customers/reports/struct.CountChromeVersionsRequestBuilder.html), [*countInstalledApps*](resources/customers/reports/struct.CountInstalledAppsRequestBuilder.html), [*findInstalledAppDevices*](resources/customers/reports/struct.FindInstalledAppDevicesRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See reports about devices and Chrome browsers managed within your organization\n\n`https://www.googleapis.com/auth/chrome.management.reports.readonly`"]
    pub const CHROME_MANAGEMENT_REPORTS_READONLY: &str =
        "https://www.googleapis.com/auth/chrome.management.reports.readonly";
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
    pub struct GoogleChromeManagementV1BrowserVersion {
        #[doc = "Output only. The release channel of the installed browser."]
        #[serde(
            rename = "channel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1BrowserVersionChannel>,
        #[doc = "Output only. Count grouped by device_system and major version"]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Output only. Version of the system-specified operating system."]
        #[serde(
            rename = "deviceOsVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_os_version: ::std::option::Option<String>,
        #[doc = "Output only. The device operating system."]
        #[serde(
            rename = "system",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1BrowserVersionSystem>,
        #[doc = "Output only. The full version of the installed browser."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BrowserVersionChannel {
        #[doc = "Beta release channel."]
        Beta,
        #[doc = "Canary release channel."]
        Canary,
        #[doc = "Dev release channel."]
        Dev,
        #[doc = "No release channel specified."]
        ReleaseChannelUnspecified,
        #[doc = "Stable release channel."]
        Stable,
    }
    impl GoogleChromeManagementV1BrowserVersionChannel {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1BrowserVersionChannel::Beta => "BETA",
                GoogleChromeManagementV1BrowserVersionChannel::Canary => "CANARY",
                GoogleChromeManagementV1BrowserVersionChannel::Dev => "DEV",
                GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified => {
                    "RELEASE_CHANNEL_UNSPECIFIED"
                }
                GoogleChromeManagementV1BrowserVersionChannel::Stable => "STABLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BrowserVersionChannel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BrowserVersionChannel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BrowserVersionChannel, ()> {
            Ok(match s {
                "BETA" => GoogleChromeManagementV1BrowserVersionChannel::Beta,
                "CANARY" => GoogleChromeManagementV1BrowserVersionChannel::Canary,
                "DEV" => GoogleChromeManagementV1BrowserVersionChannel::Dev,
                "RELEASE_CHANNEL_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified
                }
                "STABLE" => GoogleChromeManagementV1BrowserVersionChannel::Stable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BrowserVersionChannel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BrowserVersionChannel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BrowserVersionChannel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BETA" => GoogleChromeManagementV1BrowserVersionChannel::Beta,
                "CANARY" => GoogleChromeManagementV1BrowserVersionChannel::Canary,
                "DEV" => GoogleChromeManagementV1BrowserVersionChannel::Dev,
                "RELEASE_CHANNEL_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified
                }
                "STABLE" => GoogleChromeManagementV1BrowserVersionChannel::Stable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersionChannel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersionChannel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BrowserVersionSystem {
        #[doc = "No operating system specified."]
        DeviceSystemUnspecified,
        #[doc = "Android operating system."]
        SystemAndroid,
        #[doc = "Chrome OS operating system."]
        SystemCros,
        #[doc = "Apple iOS operating system."]
        SystemIos,
        #[doc = "Linux operating system."]
        SystemLinux,
        #[doc = "Apple macOS operating system."]
        SystemMac,
        #[doc = "Other operating system."]
        SystemOther,
        #[doc = "Microsoft Windows operating system."]
        SystemWindows,
    }
    impl GoogleChromeManagementV1BrowserVersionSystem {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified => {
                    "DEVICE_SYSTEM_UNSPECIFIED"
                }
                GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid => "SYSTEM_ANDROID",
                GoogleChromeManagementV1BrowserVersionSystem::SystemCros => "SYSTEM_CROS",
                GoogleChromeManagementV1BrowserVersionSystem::SystemIos => "SYSTEM_IOS",
                GoogleChromeManagementV1BrowserVersionSystem::SystemLinux => "SYSTEM_LINUX",
                GoogleChromeManagementV1BrowserVersionSystem::SystemMac => "SYSTEM_MAC",
                GoogleChromeManagementV1BrowserVersionSystem::SystemOther => "SYSTEM_OTHER",
                GoogleChromeManagementV1BrowserVersionSystem::SystemWindows => "SYSTEM_WINDOWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BrowserVersionSystem {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BrowserVersionSystem {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BrowserVersionSystem, ()> {
            Ok(match s {
                "DEVICE_SYSTEM_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified
                }
                "SYSTEM_ANDROID" => GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid,
                "SYSTEM_CROS" => GoogleChromeManagementV1BrowserVersionSystem::SystemCros,
                "SYSTEM_IOS" => GoogleChromeManagementV1BrowserVersionSystem::SystemIos,
                "SYSTEM_LINUX" => GoogleChromeManagementV1BrowserVersionSystem::SystemLinux,
                "SYSTEM_MAC" => GoogleChromeManagementV1BrowserVersionSystem::SystemMac,
                "SYSTEM_OTHER" => GoogleChromeManagementV1BrowserVersionSystem::SystemOther,
                "SYSTEM_WINDOWS" => GoogleChromeManagementV1BrowserVersionSystem::SystemWindows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BrowserVersionSystem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BrowserVersionSystem {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BrowserVersionSystem {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_SYSTEM_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified
                }
                "SYSTEM_ANDROID" => GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid,
                "SYSTEM_CROS" => GoogleChromeManagementV1BrowserVersionSystem::SystemCros,
                "SYSTEM_IOS" => GoogleChromeManagementV1BrowserVersionSystem::SystemIos,
                "SYSTEM_LINUX" => GoogleChromeManagementV1BrowserVersionSystem::SystemLinux,
                "SYSTEM_MAC" => GoogleChromeManagementV1BrowserVersionSystem::SystemMac,
                "SYSTEM_OTHER" => GoogleChromeManagementV1BrowserVersionSystem::SystemOther,
                "SYSTEM_WINDOWS" => GoogleChromeManagementV1BrowserVersionSystem::SystemWindows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersionSystem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersionSystem {
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
    pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
        #[doc = "List of all browser versions and their install counts."]
        #[serde(
            rename = "browserVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub browser_versions:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BrowserVersion>>,
        #[doc = "Token to specify the next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number browser versions matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeVersionsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CountChromeVersionsResponse {
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
    pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
        #[doc = "List of installed apps matching request."]
        #[serde(
            rename = "installedApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installed_apps:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1InstalledApp>>,
        #[doc = "Token to specify next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number of installed apps matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CountInstalledAppsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CountInstalledAppsResponse {
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
    pub struct GoogleChromeManagementV1Device {
        #[doc = "Output only. The ID of the device that reported this Chrome browser information."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "Output only. The name of the machine within its local network."]
        #[serde(
            rename = "machine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1Device {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1Device {
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
    pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
        #[doc = "A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field."]
        #[serde(
            rename = "devices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub devices: ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1Device>>,
        #[doc = "Token to specify the next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number of devices matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1FindInstalledAppDevicesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1FindInstalledAppDevicesResponse
    {
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
    pub struct GoogleChromeManagementV1InstalledApp {
        #[doc = "Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Output only. How the app was installed."]
        #[serde(
            rename = "appInstallType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_install_type: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1InstalledAppAppInstallType,
        >,
        #[doc = "Output only. Source of the installed app."]
        #[serde(
            rename = "appSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_source:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1InstalledAppAppSource>,
        #[doc = "Output only. Type of the app."]
        #[serde(
            rename = "appType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_type:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1InstalledAppAppType>,
        #[doc = "Output only. Count of browser devices with this app installed."]
        #[serde(
            rename = "browserDeviceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub browser_device_count: ::std::option::Option<i64>,
        #[doc = "Output only. Description of the installed app."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. Whether the app is disabled."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "Output only. Name of the installed app."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. Homepage uri of the installed app."]
        #[serde(
            rename = "homepageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub homepage_uri: ::std::option::Option<String>,
        #[doc = "Output only. Count of ChromeOS users with this app installed."]
        #[serde(
            rename = "osUserCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub os_user_count: ::std::option::Option<i64>,
        #[doc = "Output only. Permissions of the installed app."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledApp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledApp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppInstallType {
        #[doc = "Administrator app install type."]
        Admin,
        #[doc = "Application install type not specified."]
        AppInstallTypeUnspecified,
        #[doc = "Development app install type."]
        Development,
        #[doc = "Multiple app install types."]
        Multiple,
        #[doc = "Normal app install type."]
        Normal,
        #[doc = "Other app install type."]
        Other,
        #[doc = "Sideloaded app install type."]
        Sideload,
    }
    impl GoogleChromeManagementV1InstalledAppAppInstallType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppInstallType::Admin => "ADMIN",
                GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified => {
                    "APP_INSTALL_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppInstallType::Development => "DEVELOPMENT",
                GoogleChromeManagementV1InstalledAppAppInstallType::Multiple => "MULTIPLE",
                GoogleChromeManagementV1InstalledAppAppInstallType::Normal => "NORMAL",
                GoogleChromeManagementV1InstalledAppAppInstallType::Other => "OTHER",
                GoogleChromeManagementV1InstalledAppAppInstallType::Sideload => "SIDELOAD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppInstallType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppInstallType, ()> {
            Ok(match s {
                "ADMIN" => GoogleChromeManagementV1InstalledAppAppInstallType::Admin,
                "APP_INSTALL_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified
                }
                "DEVELOPMENT" => GoogleChromeManagementV1InstalledAppAppInstallType::Development,
                "MULTIPLE" => GoogleChromeManagementV1InstalledAppAppInstallType::Multiple,
                "NORMAL" => GoogleChromeManagementV1InstalledAppAppInstallType::Normal,
                "OTHER" => GoogleChromeManagementV1InstalledAppAppInstallType::Other,
                "SIDELOAD" => GoogleChromeManagementV1InstalledAppAppInstallType::Sideload,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN" => GoogleChromeManagementV1InstalledAppAppInstallType::Admin,
                "APP_INSTALL_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified
                }
                "DEVELOPMENT" => GoogleChromeManagementV1InstalledAppAppInstallType::Development,
                "MULTIPLE" => GoogleChromeManagementV1InstalledAppAppInstallType::Multiple,
                "NORMAL" => GoogleChromeManagementV1InstalledAppAppInstallType::Normal,
                "OTHER" => GoogleChromeManagementV1InstalledAppAppInstallType::Other,
                "SIDELOAD" => GoogleChromeManagementV1InstalledAppAppInstallType::Sideload,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppSource {
        #[doc = "Application source not specified."]
        AppSourceUnspecified,
        #[doc = "Generally for extensions and Chrome apps."]
        ChromeWebstore,
        #[doc = "Play Store app."]
        PlayStore,
    }
    impl GoogleChromeManagementV1InstalledAppAppSource {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified => {
                    "APP_SOURCE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore => "CHROME_WEBSTORE",
                GoogleChromeManagementV1InstalledAppAppSource::PlayStore => "PLAY_STORE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppSource {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppSource, ()> {
            Ok(match s {
                "APP_SOURCE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified
                }
                "CHROME_WEBSTORE" => GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore,
                "PLAY_STORE" => GoogleChromeManagementV1InstalledAppAppSource::PlayStore,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_SOURCE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified
                }
                "CHROME_WEBSTORE" => GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore,
                "PLAY_STORE" => GoogleChromeManagementV1InstalledAppAppSource::PlayStore,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppType {
        #[doc = "ARC++ app."]
        AndroidApp,
        #[doc = "Chrome app."]
        App,
        #[doc = "App type not specified."]
        AppTypeUnspecified,
        #[doc = "Chrome extension."]
        Extension,
        #[doc = "Chrome hosted app."]
        HostedApp,
        #[doc = "Chrome theme."]
        Theme,
    }
    impl GoogleChromeManagementV1InstalledAppAppType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppType::AndroidApp => "ANDROID_APP",
                GoogleChromeManagementV1InstalledAppAppType::App => "APP",
                GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified => {
                    "APP_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppType::Extension => "EXTENSION",
                GoogleChromeManagementV1InstalledAppAppType::HostedApp => "HOSTED_APP",
                GoogleChromeManagementV1InstalledAppAppType::Theme => "THEME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppType, ()> {
            Ok(match s {
                "ANDROID_APP" => GoogleChromeManagementV1InstalledAppAppType::AndroidApp,
                "APP" => GoogleChromeManagementV1InstalledAppAppType::App,
                "APP_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified
                }
                "EXTENSION" => GoogleChromeManagementV1InstalledAppAppType::Extension,
                "HOSTED_APP" => GoogleChromeManagementV1InstalledAppAppType::HostedApp,
                "THEME" => GoogleChromeManagementV1InstalledAppAppType::Theme,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID_APP" => GoogleChromeManagementV1InstalledAppAppType::AndroidApp,
                "APP" => GoogleChromeManagementV1InstalledAppAppType::App,
                "APP_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified
                }
                "EXTENSION" => GoogleChromeManagementV1InstalledAppAppType::Extension,
                "HOSTED_APP" => GoogleChromeManagementV1InstalledAppAppType::HostedApp,
                "THEME" => GoogleChromeManagementV1InstalledAppAppType::Theme,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppType {
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
    #[doc = "Actions that can be performed on the customers resource"]
    pub fn customers(&self) -> crate::resources::customers::CustomersActions {
        crate::resources::customers::CustomersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod customers {
        pub mod params {}
        pub struct CustomersActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CustomersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the reports resource"]
            pub fn reports(&self) -> crate::resources::customers::reports::ReportsActions {
                crate::resources::customers::reports::ReportsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod reports {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum FindInstalledAppDevicesAppType {
                    #[doc = "ARC++ app."]
                    AndroidApp,
                    #[doc = "Chrome app."]
                    App,
                    #[doc = "App type not specified."]
                    AppTypeUnspecified,
                    #[doc = "Chrome extension."]
                    Extension,
                    #[doc = "Chrome hosted app."]
                    HostedApp,
                    #[doc = "Chrome theme."]
                    Theme,
                }
                impl FindInstalledAppDevicesAppType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            FindInstalledAppDevicesAppType::AndroidApp => "ANDROID_APP",
                            FindInstalledAppDevicesAppType::App => "APP",
                            FindInstalledAppDevicesAppType::AppTypeUnspecified => {
                                "APP_TYPE_UNSPECIFIED"
                            }
                            FindInstalledAppDevicesAppType::Extension => "EXTENSION",
                            FindInstalledAppDevicesAppType::HostedApp => "HOSTED_APP",
                            FindInstalledAppDevicesAppType::Theme => "THEME",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for FindInstalledAppDevicesAppType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for FindInstalledAppDevicesAppType {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<FindInstalledAppDevicesAppType, ()>
                    {
                        Ok(match s {
                            "ANDROID_APP" => FindInstalledAppDevicesAppType::AndroidApp,
                            "APP" => FindInstalledAppDevicesAppType::App,
                            "APP_TYPE_UNSPECIFIED" => {
                                FindInstalledAppDevicesAppType::AppTypeUnspecified
                            }
                            "EXTENSION" => FindInstalledAppDevicesAppType::Extension,
                            "HOSTED_APP" => FindInstalledAppDevicesAppType::HostedApp,
                            "THEME" => FindInstalledAppDevicesAppType::Theme,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for FindInstalledAppDevicesAppType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for FindInstalledAppDevicesAppType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for FindInstalledAppDevicesAppType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ANDROID_APP" => FindInstalledAppDevicesAppType::AndroidApp,
                            "APP" => FindInstalledAppDevicesAppType::App,
                            "APP_TYPE_UNSPECIFIED" => {
                                FindInstalledAppDevicesAppType::AppTypeUnspecified
                            }
                            "EXTENSION" => FindInstalledAppDevicesAppType::Extension,
                            "HOSTED_APP" => FindInstalledAppDevicesAppType::HostedApp,
                            "THEME" => FindInstalledAppDevicesAppType::Theme,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for FindInstalledAppDevicesAppType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for FindInstalledAppDevicesAppType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ReportsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ReportsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Generate report of installed Chrome versions."]
                pub fn count_chrome_versions(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeVersionsRequestBuilder {
                    CountChromeVersionsRequestBuilder {
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
                        customer: customer.into(),
                        filter: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Generate report of app installations."]
                pub fn count_installed_apps(
                    &self,
                    customer: impl Into<String>,
                ) -> CountInstalledAppsRequestBuilder {
                    CountInstalledAppsRequestBuilder {
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
                        customer: customer.into(),
                        filter: None,
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Generate report of devices that have a specified app installed."]
                pub fn find_installed_app_devices(
                    &self,
                    customer: impl Into<String>,
                ) -> FindInstalledAppDevicesRequestBuilder {
                    FindInstalledAppDevicesRequestBuilder {
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
                        customer: customer.into(),
                        app_id: None,
                        app_type: None,
                        filter: None,
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [ReportsActions::count_chrome_versions()](struct.ReportsActions.html#method.count_chrome_versions)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeVersionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                filter: Option<String>,
                org_unit_id: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
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
            impl<'a> CountChromeVersionsRequestBuilder<'a> {
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the next page in the list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
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
                pub fn iter_browser_versions<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_browser_versions_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_browser_versions_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1BrowserVersion,
                > {
                    self.iter_browser_versions_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_browser_versions_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1BrowserVersion,
                > {
                    self.iter_browser_versions_with_fields(Some("*"))
                }
                pub fn iter_browser_versions_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "browserVersions").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "browserVersions")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
                > {
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
                ) -> Result<
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
                    crate::Error,
                > {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countChromeVersions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for CountChromeVersionsRequestBuilder<'a> {
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
            #[doc = "Created via [ReportsActions::count_installed_apps()](struct.ReportsActions.html#method.count_installed_apps)"]
            #[derive(Debug, Clone)]
            pub struct CountInstalledAppsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                filter: Option<String>,
                order_by: Option<String>,
                org_unit_id: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
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
            impl<'a> CountInstalledAppsRequestBuilder<'a> {
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * latest_profile_active_date * permission_name"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Field used to order results. Supported order by fields: * app_name * app_type * install_type * number_of_permissions * total_install_count"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify next page in the list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
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
                pub fn iter_installed_apps<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_installed_apps_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_installed_apps_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1InstalledApp,
                > {
                    self.iter_installed_apps_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_installed_apps_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1InstalledApp,
                > {
                    self.iter_installed_apps_with_fields(Some("*"))
                }
                pub fn iter_installed_apps_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "installedApps").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "installedApps")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
                > {
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
                ) -> Result<
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
                    crate::Error,
                > {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countInstalledApps");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for CountInstalledAppsRequestBuilder<'a> {
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
            #[doc = "Created via [ReportsActions::find_installed_app_devices()](struct.ReportsActions.html#method.find_installed_app_devices)"]
            #[derive(Debug, Clone)]
            pub struct FindInstalledAppDevicesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                app_id: Option<String>,
                app_type: Option<
                    crate::resources::customers::reports::params::FindInstalledAppDevicesAppType,
                >,
                filter: Option<String>,
                order_by: Option<String>,
                org_unit_id: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
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
            impl<'a> FindInstalledAppDevicesRequestBuilder<'a> {
                #[doc = "Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
                pub fn app_id(mut self, value: impl Into<String>) -> Self {
                    self.app_id = Some(value.into());
                    self
                }
                #[doc = "Type of the app."]
                pub fn app_type(
                    mut self,
                    value : crate :: resources :: customers :: reports :: params :: FindInstalledAppDevicesAppType,
                ) -> Self {
                    self.app_type = Some(value);
                    self
                }
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Field used to order results. Supported order by fields: * machine_name * device_id"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the next page in the list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
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
                pub fn iter_devices<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_devices_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_devices_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleChromeManagementV1Device>
                {
                    self.iter_devices_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_devices_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::GoogleChromeManagementV1Device>
                {
                    self.iter_devices_with_fields(Some("*"))
                }
                pub fn iter_devices_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "devices").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "devices")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
                > {
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
                ) -> Result<
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
                    crate::Error,
                > {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:findInstalledAppDevices");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("appId", &self.app_id)]);
                    req = req.query(&[("appType", &self.app_type)]);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("access_token", &self.access_token)]);
                    req = req.query(&[("alt", &self.alt)]);
                    req = req.query(&[("callback", &self.callback)]);
                    req = req.query(&[("fields", &self.fields)]);
                    req = req.query(&[("key", &self.key)]);
                    req = req.query(&[("oauth_token", &self.oauth_token)]);
                    req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    req = req.query(&[("quotaUser", &self.quota_user)]);
                    req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    req = req.query(&[("uploadType", &self.upload_type)]);
                    req = req.query(&[("$.xgafv", &self.xgafv)]);
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for FindInstalledAppDevicesRequestBuilder<'a> {
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
