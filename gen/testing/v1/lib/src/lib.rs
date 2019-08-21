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
        #[doc = "An automatic google login account."]
        #[serde(rename = "googleAuto", default)]
        pub google_auto: Option<crate::schemas::GoogleAuto>,
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
    pub struct AndroidDevice {
        #[doc = "Required. The id of the Android device to be used.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "androidModelId", default)]
        pub android_model_id: Option<String>,
        #[doc = "Required. The id of the Android OS version to be used.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "androidVersionId", default)]
        pub android_version_id: Option<String>,
        #[doc = "Required. The locale the test device used for testing.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "locale", default)]
        pub locale: Option<String>,
        #[doc = "Required. How the device is oriented during the test.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "orientation", default)]
        pub orientation: Option<String>,
    }
    impl ::field_selector::FieldSelector for AndroidDevice {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AndroidDeviceCatalog {
        #[doc = "The set of supported Android device models."]
        #[serde(rename = "models", default)]
        pub models: Option<Vec<crate::schemas::AndroidModel>>,
        #[doc = "The set of supported runtime configurations."]
        #[serde(rename = "runtimeConfiguration", default)]
        pub runtime_configuration: Option<crate::schemas::AndroidRuntimeConfiguration>,
        #[doc = "The set of supported Android OS versions."]
        #[serde(rename = "versions", default)]
        pub versions: Option<Vec<crate::schemas::AndroidVersion>>,
    }
    impl ::field_selector::FieldSelector for AndroidDeviceCatalog {
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
    pub struct AndroidDeviceList {
        #[doc = "Required. A list of Android devices."]
        #[serde(rename = "androidDevices", default)]
        pub android_devices: Option<Vec<crate::schemas::AndroidDevice>>,
    }
    impl ::field_selector::FieldSelector for AndroidDeviceList {
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
    pub enum AndroidInstrumentationTestOrchestratorOption {
        #[doc = "Default value: the server will choose the mode. Currently implies that\nthe test will run without the orchestrator. In the future,\nall instrumentation tests will be run with the orchestrator.\nUsing the orchestrator is highly encouraged because of all the benefits it\noffers."]
        OrchestratorOptionUnspecified,
        #[doc = "Run test using orchestrator.\n** Only compatible with AndroidJUnitRunner version 1.0 or higher! **\nRecommended."]
        UseOrchestrator,
        #[doc = "Run test without using orchestrator."]
        DoNotUseOrchestrator,
    }
    impl AndroidInstrumentationTestOrchestratorOption {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidInstrumentationTestOrchestratorOption::OrchestratorOptionUnspecified => {
                    "ORCHESTRATOR_OPTION_UNSPECIFIED"
                }
                AndroidInstrumentationTestOrchestratorOption::UseOrchestrator => "USE_ORCHESTRATOR",
                AndroidInstrumentationTestOrchestratorOption::DoNotUseOrchestrator => {
                    "DO_NOT_USE_ORCHESTRATOR"
                }
            }
        }
    }
    impl ::std::fmt::Display for AndroidInstrumentationTestOrchestratorOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidInstrumentationTestOrchestratorOption {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidInstrumentationTestOrchestratorOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ORCHESTRATOR_OPTION_UNSPECIFIED" => {
                    AndroidInstrumentationTestOrchestratorOption::OrchestratorOptionUnspecified
                }
                "USE_ORCHESTRATOR" => AndroidInstrumentationTestOrchestratorOption::UseOrchestrator,
                "DO_NOT_USE_ORCHESTRATOR" => {
                    AndroidInstrumentationTestOrchestratorOption::DoNotUseOrchestrator
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
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AndroidInstrumentationTest {
        #[doc = "The APK for the application under test."]
        #[serde(rename = "appApk", default)]
        pub app_apk: Option<crate::schemas::FileReference>,
        #[doc = "A multi-apk app bundle for the application under test."]
        #[serde(rename = "appBundle", default)]
        pub app_bundle: Option<crate::schemas::AppBundle>,
        #[doc = "The java package for the application under test.\nThe default value is determined by examining the application's manifest."]
        #[serde(rename = "appPackageId", default)]
        pub app_package_id: Option<String>,
        #[doc = "The option of whether running each test within its own invocation of\ninstrumentation with Android Test Orchestrator or not.\n** Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or\nhigher! **\nOrchestrator offers the following benefits:\n\n* No shared state\n* Crashes are isolated\n* Logs are scoped per test\n\nSee\n[https://developer.android.com/training/testing/junit-runner.html#using-android-test-orchestrator](https://developer.android.com/training/testing/junit-runner.html#using-android-test-orchestrator)\nfor more information about Android Test Orchestrator.\n\nIf not set, the test will be run without the orchestrator."]
        #[serde(rename = "orchestratorOption", default)]
        pub orchestrator_option:
            Option<crate::schemas::AndroidInstrumentationTestOrchestratorOption>,
        #[doc = "Required. The APK containing the test code to be executed."]
        #[serde(rename = "testApk", default)]
        pub test_apk: Option<crate::schemas::FileReference>,
        #[doc = "The java package for the test to be executed.\nThe default value is determined by examining the application's manifest."]
        #[serde(rename = "testPackageId", default)]
        pub test_package_id: Option<String>,
        #[doc = "The InstrumentationTestRunner class.\nThe default value is determined by examining the application's manifest."]
        #[serde(rename = "testRunnerClass", default)]
        pub test_runner_class: Option<String>,
        #[doc = "Each target must be fully qualified with the package name or class name,\nin one of these formats:\n\n* \"package package_name\"\n* \"class package_name.class_name\"\n* \"class package_name.class_name#method_name\"\n\nIf empty, all targets in the module will be run."]
        #[serde(rename = "testTargets", default)]
        pub test_targets: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for AndroidInstrumentationTest {
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
    pub struct AndroidMatrix {
        #[doc = "Required. The ids of the set of Android device to be used.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "androidModelIds", default)]
        pub android_model_ids: Option<Vec<String>>,
        #[doc = "Required. The ids of the set of Android OS version to be used.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "androidVersionIds", default)]
        pub android_version_ids: Option<Vec<String>>,
        #[doc = "Required. The set of locales the test device will enable for testing.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "locales", default)]
        pub locales: Option<Vec<String>>,
        #[doc = "Required. The set of orientations to test with.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "orientations", default)]
        pub orientations: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for AndroidMatrix {
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
    pub enum AndroidModelForm {
        #[doc = "Do not use.  For proto versioning only."]
        DeviceFormUnspecified,
        #[doc = "A software stack that simulates the device."]
        Virtual,
        #[doc = "Actual hardware."]
        Physical,
    }
    impl AndroidModelForm {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidModelForm::DeviceFormUnspecified => "DEVICE_FORM_UNSPECIFIED",
                AndroidModelForm::Virtual => "VIRTUAL",
                AndroidModelForm::Physical => "PHYSICAL",
            }
        }
    }
    impl ::std::fmt::Display for AndroidModelForm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidModelForm {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidModelForm {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_FORM_UNSPECIFIED" => AndroidModelForm::DeviceFormUnspecified,
                "VIRTUAL" => AndroidModelForm::Virtual,
                "PHYSICAL" => AndroidModelForm::Physical,
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
    pub enum AndroidModelFormFactor {
        #[doc = "Do not use. For proto versioning only."]
        DeviceFormFactorUnspecified,
        #[doc = "This device has the shape of a phone."]
        Phone,
        #[doc = "This device has the shape of a tablet."]
        Tablet,
        #[doc = "This device has the shape of a watch or other wearable."]
        Wearable,
    }
    impl AndroidModelFormFactor {
        pub fn as_str(self) -> &'static str {
            match self {
                AndroidModelFormFactor::DeviceFormFactorUnspecified => {
                    "DEVICE_FORM_FACTOR_UNSPECIFIED"
                }
                AndroidModelFormFactor::Phone => "PHONE",
                AndroidModelFormFactor::Tablet => "TABLET",
                AndroidModelFormFactor::Wearable => "WEARABLE",
            }
        }
    }
    impl ::std::fmt::Display for AndroidModelFormFactor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AndroidModelFormFactor {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AndroidModelFormFactor {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_FORM_FACTOR_UNSPECIFIED" => {
                    AndroidModelFormFactor::DeviceFormFactorUnspecified
                }
                "PHONE" => AndroidModelFormFactor::Phone,
                "TABLET" => AndroidModelFormFactor::Tablet,
                "WEARABLE" => AndroidModelFormFactor::Wearable,
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
    pub struct AndroidModel {
        #[doc = "The company that this device is branded with.\nExample: \"Google\", \"Samsung\"."]
        #[serde(rename = "brand", default)]
        pub brand: Option<String>,
        #[doc = "The name of the industrial design.\nThis corresponds to android.os.Build.DEVICE."]
        #[serde(rename = "codename", default)]
        pub codename: Option<String>,
        #[doc = "Whether this device is virtual or physical."]
        #[serde(rename = "form", default)]
        pub form: Option<crate::schemas::AndroidModelForm>,
        #[doc = "Whether this device is a phone, tablet, wearable, etc."]
        #[serde(rename = "formFactor", default)]
        pub form_factor: Option<crate::schemas::AndroidModelFormFactor>,
        #[doc = "The unique opaque id for this model.\nUse this for invoking the TestExecutionService."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "True if and only if tests with this model are recorded by stitching\ntogether screenshots. See use_low_spec_video_recording in device config."]
        #[serde(rename = "lowFpsVideoRecording", default)]
        pub low_fps_video_recording: Option<bool>,
        #[doc = "The manufacturer of this device."]
        #[serde(rename = "manufacturer", default)]
        pub manufacturer: Option<String>,
        #[doc = "The human-readable marketing name for this device model.\nExamples: \"Nexus 5\", \"Galaxy S5\"."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Screen density in DPI.\nThis corresponds to ro.sf.lcd_density"]
        #[serde(rename = "screenDensity", default)]
        pub screen_density: Option<i32>,
        #[doc = "Screen size in the horizontal (X) dimension measured in pixels."]
        #[serde(rename = "screenX", default)]
        pub screen_x: Option<i32>,
        #[doc = "Screen size in the vertical (Y) dimension measured in pixels."]
        #[serde(rename = "screenY", default)]
        pub screen_y: Option<i32>,
        #[doc = "The list of supported ABIs for this device.\nThis corresponds to either android.os.Build.SUPPORTED_ABIS (for API level\n21 and above) or android.os.Build.CPU_ABI/CPU_ABI2.\nThe most preferred ABI is the first element in the list.\n\nElements are optionally prefixed by \"version_id:\" (where version_id is\nthe id of an AndroidVersion), denoting an ABI that is supported only on\na particular version."]
        #[serde(rename = "supportedAbis", default)]
        pub supported_abis: Option<Vec<String>>,
        #[doc = "The set of Android versions this device supports."]
        #[serde(rename = "supportedVersionIds", default)]
        pub supported_version_ids: Option<Vec<String>>,
        #[doc = "Tags for this dimension.\nExamples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for AndroidModel {
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
    pub struct AndroidRoboTest {
        #[doc = "The APK for the application under test."]
        #[serde(rename = "appApk", default)]
        pub app_apk: Option<crate::schemas::FileReference>,
        #[doc = "A multi-apk app bundle for the application under test."]
        #[serde(rename = "appBundle", default)]
        pub app_bundle: Option<crate::schemas::AppBundle>,
        #[doc = "The initial activity that should be used to start the app."]
        #[serde(rename = "appInitialActivity", default)]
        pub app_initial_activity: Option<String>,
        #[doc = "The java package for the application under test.\nThe default value is determined by examining the application's manifest."]
        #[serde(rename = "appPackageId", default)]
        pub app_package_id: Option<String>,
        #[doc = "The max depth of the traversal stack Robo can explore. Needs to be at least\n2 to make Robo explore the app beyond the first activity.\nDefault is 50."]
        #[serde(rename = "maxDepth", default)]
        pub max_depth: Option<i32>,
        #[doc = "The max number of steps Robo can execute.\nDefault is no limit."]
        #[serde(rename = "maxSteps", default)]
        pub max_steps: Option<i32>,
        #[doc = "A set of directives Robo should apply during the crawl.\nThis allows users to customize the crawl. For example, the username and\npassword for a test account can be provided."]
        #[serde(rename = "roboDirectives", default)]
        pub robo_directives: Option<Vec<crate::schemas::RoboDirective>>,
        #[doc = "A JSON file with a sequence of actions Robo should perform as a prologue\nfor the crawl."]
        #[serde(rename = "roboScript", default)]
        pub robo_script: Option<crate::schemas::FileReference>,
        #[doc = "The intents used to launch the app for the crawl.\nIf none are provided, then the main launcher activity is launched.\nIf some are provided, then only those provided are launched (the main\nlauncher activity must be provided explicitly)."]
        #[serde(rename = "startingIntents", default)]
        pub starting_intents: Option<Vec<crate::schemas::RoboStartingIntent>>,
    }
    impl ::field_selector::FieldSelector for AndroidRoboTest {
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
    pub struct AndroidRuntimeConfiguration {
        #[doc = "The set of available locales."]
        #[serde(rename = "locales", default)]
        pub locales: Option<Vec<crate::schemas::Locale>>,
        #[doc = "The set of available orientations."]
        #[serde(rename = "orientations", default)]
        pub orientations: Option<Vec<crate::schemas::Orientation>>,
    }
    impl ::field_selector::FieldSelector for AndroidRuntimeConfiguration {
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
    pub struct AndroidTestLoop {
        #[doc = "The APK for the application under test."]
        #[serde(rename = "appApk", default)]
        pub app_apk: Option<crate::schemas::FileReference>,
        #[doc = "A multi-apk app bundle for the application under test."]
        #[serde(rename = "appBundle", default)]
        pub app_bundle: Option<crate::schemas::AppBundle>,
        #[doc = "The java package for the application under test.\nThe default is determined by examining the application's manifest."]
        #[serde(rename = "appPackageId", default)]
        pub app_package_id: Option<String>,
        #[doc = "The list of scenario labels that should be run during the test.\nThe scenario labels should map to labels defined in the application's\nmanifest. For example, player_experience and\ncom.google.test.loops.player_experience add all of the loops labeled in the\nmanifest with the com.google.test.loops.player_experience name to the\nexecution.\nScenarios can also be specified in the scenarios field."]
        #[serde(rename = "scenarioLabels", default)]
        pub scenario_labels: Option<Vec<String>>,
        #[doc = "The list of scenarios that should be run during the test.\nThe default is all test loops, derived from the application's\nmanifest."]
        #[serde(rename = "scenarios", default)]
        pub scenarios: Option<Vec<i32>>,
    }
    impl ::field_selector::FieldSelector for AndroidTestLoop {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AndroidVersion {
        #[doc = "The API level for this Android version.\nExamples: 18, 19."]
        #[serde(rename = "apiLevel", default)]
        pub api_level: Option<i32>,
        #[doc = "The code name for this Android version.\nExamples: \"JellyBean\", \"KitKat\"."]
        #[serde(rename = "codeName", default)]
        pub code_name: Option<String>,
        #[doc = "Market share for this version."]
        #[serde(rename = "distribution", default)]
        pub distribution: Option<crate::schemas::Distribution>,
        #[doc = "An opaque id for this Android version.\nUse this id to invoke the TestExecutionService."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The date this Android version became available in the market."]
        #[serde(rename = "releaseDate", default)]
        pub release_date: Option<crate::schemas::Date>,
        #[doc = "Tags for this dimension.\nExamples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
        #[doc = "A string representing this version of the Android OS.\nExamples: \"4.3\", \"4.4\"."]
        #[serde(rename = "versionString", default)]
        pub version_string: Option<String>,
    }
    impl ::field_selector::FieldSelector for AndroidVersion {
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
    pub struct Apk {
        #[doc = "The path to an APK to be installed on the device before the test begins."]
        #[serde(rename = "location", default)]
        pub location: Option<crate::schemas::FileReference>,
        #[doc = "The java package for the APK to be installed.\nValue is determined by examining the application's manifest."]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for Apk {
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
    pub struct ApkDetail {
        #[serde(rename = "apkManifest", default)]
        pub apk_manifest: Option<crate::schemas::ApkManifest>,
    }
    impl ::field_selector::FieldSelector for ApkDetail {
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
    pub struct ApkManifest {
        #[doc = "User-readable name for the application."]
        #[serde(rename = "applicationLabel", default)]
        pub application_label: Option<String>,
        #[serde(rename = "intentFilters", default)]
        pub intent_filters: Option<Vec<crate::schemas::IntentFilter>>,
        #[doc = "Maximum API level on which the application is designed to run."]
        #[serde(rename = "maxSdkVersion", default)]
        pub max_sdk_version: Option<i32>,
        #[doc = "Minimum API level required for the application to run."]
        #[serde(rename = "minSdkVersion", default)]
        pub min_sdk_version: Option<i32>,
        #[doc = "Full Java-style package name for this application, e.g.\n\"com.example.foo\"."]
        #[serde(rename = "packageName", default)]
        pub package_name: Option<String>,
        #[doc = "Specifies the API Level on which the application is designed to run."]
        #[serde(rename = "targetSdkVersion", default)]
        pub target_sdk_version: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ApkManifest {
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
    pub struct AppBundle {
        #[doc = ".aab file representing the app bundle under test."]
        #[serde(rename = "bundleLocation", default)]
        pub bundle_location: Option<crate::schemas::FileReference>,
    }
    impl ::field_selector::FieldSelector for AppBundle {
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
    pub enum CancelTestMatrixResponseTestState {
        #[doc = "Do not use.  For proto versioning only."]
        TestStateUnspecified,
        #[doc = "The execution or matrix is being validated."]
        Validating,
        #[doc = "The execution or matrix is waiting for resources to become available."]
        Pending,
        #[doc = "The execution is currently being processed.\n\nCan only be set on an execution."]
        Running,
        #[doc = "The execution or matrix has terminated normally.\n\nOn a matrix this means that the matrix level processing completed normally,\nbut individual executions may be in an ERROR state."]
        Finished,
        #[doc = "The execution or matrix has stopped because it encountered an\ninfrastructure failure."]
        Error,
        #[doc = "The execution was not run because it corresponds to a unsupported\nenvironment.\n\nCan only be set on an execution."]
        UnsupportedEnvironment,
        #[doc = "The execution was not run because the provided inputs are incompatible with\nthe requested environment.\n\nExample: requested AndroidVersion is lower than APK's minSdkVersion\n\nCan only be set on an execution."]
        IncompatibleEnvironment,
        #[doc = "The execution was not run because the provided inputs are incompatible with\nthe requested architecture.\n\nExample: requested device does not support running the native code in\nthe supplied APK\n\nCan only be set on an execution."]
        IncompatibleArchitecture,
        #[doc = "The user cancelled the execution.\n\nCan only be set on an execution."]
        Cancelled,
        #[doc = "The execution or matrix was not run because the provided inputs are not\nvalid.\n\nExamples: input file is not of the expected type, is malformed/corrupt, or\nwas flagged as malware"]
        Invalid,
    }
    impl CancelTestMatrixResponseTestState {
        pub fn as_str(self) -> &'static str {
            match self {
                CancelTestMatrixResponseTestState::TestStateUnspecified => "TEST_STATE_UNSPECIFIED",
                CancelTestMatrixResponseTestState::Validating => "VALIDATING",
                CancelTestMatrixResponseTestState::Pending => "PENDING",
                CancelTestMatrixResponseTestState::Running => "RUNNING",
                CancelTestMatrixResponseTestState::Finished => "FINISHED",
                CancelTestMatrixResponseTestState::Error => "ERROR",
                CancelTestMatrixResponseTestState::UnsupportedEnvironment => {
                    "UNSUPPORTED_ENVIRONMENT"
                }
                CancelTestMatrixResponseTestState::IncompatibleEnvironment => {
                    "INCOMPATIBLE_ENVIRONMENT"
                }
                CancelTestMatrixResponseTestState::IncompatibleArchitecture => {
                    "INCOMPATIBLE_ARCHITECTURE"
                }
                CancelTestMatrixResponseTestState::Cancelled => "CANCELLED",
                CancelTestMatrixResponseTestState::Invalid => "INVALID",
            }
        }
    }
    impl ::std::fmt::Display for CancelTestMatrixResponseTestState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CancelTestMatrixResponseTestState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CancelTestMatrixResponseTestState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TEST_STATE_UNSPECIFIED" => CancelTestMatrixResponseTestState::TestStateUnspecified,
                "VALIDATING" => CancelTestMatrixResponseTestState::Validating,
                "PENDING" => CancelTestMatrixResponseTestState::Pending,
                "RUNNING" => CancelTestMatrixResponseTestState::Running,
                "FINISHED" => CancelTestMatrixResponseTestState::Finished,
                "ERROR" => CancelTestMatrixResponseTestState::Error,
                "UNSUPPORTED_ENVIRONMENT" => {
                    CancelTestMatrixResponseTestState::UnsupportedEnvironment
                }
                "INCOMPATIBLE_ENVIRONMENT" => {
                    CancelTestMatrixResponseTestState::IncompatibleEnvironment
                }
                "INCOMPATIBLE_ARCHITECTURE" => {
                    CancelTestMatrixResponseTestState::IncompatibleArchitecture
                }
                "CANCELLED" => CancelTestMatrixResponseTestState::Cancelled,
                "INVALID" => CancelTestMatrixResponseTestState::Invalid,
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
    pub struct CancelTestMatrixResponse {
        #[doc = "The current rolled-up state of the test matrix.\nIf this state is already final, then the cancelation request will\nhave no effect."]
        #[serde(rename = "testState", default)]
        pub test_state: Option<crate::schemas::CancelTestMatrixResponseTestState>,
    }
    impl ::field_selector::FieldSelector for CancelTestMatrixResponse {
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
    pub struct ClientInfo {
        #[doc = "The list of detailed information about client."]
        #[serde(rename = "clientInfoDetails", default)]
        pub client_info_details: Option<Vec<crate::schemas::ClientInfoDetail>>,
        #[doc = "Required. Client name, such as gcloud."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ClientInfo {
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
    pub struct ClientInfoDetail {
        #[doc = "Required. The key of detailed client information."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "Required. The value of detailed client information."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for ClientInfoDetail {
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
    pub struct Date {
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month, or 0\nif specifying a year by itself or a year and month where the day is not\nsignificant."]
        #[serde(rename = "day", default)]
        pub day: Option<i32>,
        #[doc = "Month of year. Must be from 1 to 12, or 0 if specifying a year without a\nmonth and day."]
        #[serde(rename = "month", default)]
        pub month: Option<i32>,
        #[doc = "Year of date. Must be from 1 to 9999, or 0 if specifying a date without\na year."]
        #[serde(rename = "year", default)]
        pub year: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Date {
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
    pub struct DeviceFile {
        #[doc = "A reference to an opaque binary blob file."]
        #[serde(rename = "obbFile", default)]
        pub obb_file: Option<crate::schemas::ObbFile>,
        #[doc = "A reference to a regular file."]
        #[serde(rename = "regularFile", default)]
        pub regular_file: Option<crate::schemas::RegularFile>,
    }
    impl ::field_selector::FieldSelector for DeviceFile {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Distribution {
        #[doc = "Output only. The estimated fraction (0-1) of the total market with this\nconfiguration."]
        #[serde(rename = "marketShare", default)]
        pub market_share: Option<f64>,
        #[doc = "Output only. The time this distribution was measured."]
        #[serde(rename = "measurementTime", default)]
        pub measurement_time: Option<String>,
    }
    impl ::field_selector::FieldSelector for Distribution {
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
    pub struct Environment {
        #[doc = "An Android device which must be used with an Android test."]
        #[serde(rename = "androidDevice", default)]
        pub android_device: Option<crate::schemas::AndroidDevice>,
        #[doc = "An iOS device which must be used with an iOS test."]
        #[serde(rename = "iosDevice", default)]
        pub ios_device: Option<crate::schemas::IosDevice>,
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
    pub struct EnvironmentMatrix {
        #[doc = "A list of Android devices; the test will be run only on the specified\ndevices."]
        #[serde(rename = "androidDeviceList", default)]
        pub android_device_list: Option<crate::schemas::AndroidDeviceList>,
        #[doc = "A matrix of Android devices."]
        #[serde(rename = "androidMatrix", default)]
        pub android_matrix: Option<crate::schemas::AndroidMatrix>,
        #[doc = "A list of iOS devices."]
        #[serde(rename = "iosDeviceList", default)]
        pub ios_device_list: Option<crate::schemas::IosDeviceList>,
    }
    impl ::field_selector::FieldSelector for EnvironmentMatrix {
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
    pub struct EnvironmentVariable {
        #[doc = "Key for the environment variable."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
        #[doc = "Value for the environment variable."]
        #[serde(rename = "value", default)]
        pub value: Option<String>,
    }
    impl ::field_selector::FieldSelector for EnvironmentVariable {
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
    pub struct FileReference {
        #[doc = "A path to a file in Google Cloud Storage.\nExample: gs://build-app-1414623860166/app-debug-unaligned.apk"]
        #[serde(rename = "gcsPath", default)]
        pub gcs_path: Option<String>,
    }
    impl ::field_selector::FieldSelector for FileReference {
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
    pub struct GetApkDetailsResponse {
        #[doc = "Details of the Android APK."]
        #[serde(rename = "apkDetail", default)]
        pub apk_detail: Option<crate::schemas::ApkDetail>,
    }
    impl ::field_selector::FieldSelector for GetApkDetailsResponse {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAuto;
    impl ::field_selector::FieldSelector for GoogleAuto {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
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
    pub struct GoogleCloudStorage {
        #[doc = "Required. The path to a directory in GCS that will\neventually contain the results for this test.\nThe requesting user must have write access on the bucket in the supplied\npath."]
        #[serde(rename = "gcsPath", default)]
        pub gcs_path: Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleCloudStorage {
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
    pub struct IntentFilter {
        #[doc = "The android:name value of the <action> tag."]
        #[serde(rename = "actionNames", default)]
        pub action_names: Option<Vec<String>>,
        #[doc = "The android:name value of the <category> tag."]
        #[serde(rename = "categoryNames", default)]
        pub category_names: Option<Vec<String>>,
        #[doc = "The android:mimeType value of the <data> tag."]
        #[serde(rename = "mimeType", default)]
        pub mime_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for IntentFilter {
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
    pub struct IosDevice {
        #[doc = "Required. The id of the iOS device to be used.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "iosModelId", default)]
        pub ios_model_id: Option<String>,
        #[doc = "Required. The id of the iOS major software version to be used.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "iosVersionId", default)]
        pub ios_version_id: Option<String>,
        #[doc = "Required. The locale the test device used for testing.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "locale", default)]
        pub locale: Option<String>,
        #[doc = "Required. How the device is oriented during the test.\nUse the TestEnvironmentDiscoveryService to get supported options."]
        #[serde(rename = "orientation", default)]
        pub orientation: Option<String>,
    }
    impl ::field_selector::FieldSelector for IosDevice {
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
    pub struct IosDeviceCatalog {
        #[doc = "The set of supported iOS device models."]
        #[serde(rename = "models", default)]
        pub models: Option<Vec<crate::schemas::IosModel>>,
        #[doc = "The set of supported runtime configurations."]
        #[serde(rename = "runtimeConfiguration", default)]
        pub runtime_configuration: Option<crate::schemas::IosRuntimeConfiguration>,
        #[doc = "The set of supported iOS software versions."]
        #[serde(rename = "versions", default)]
        pub versions: Option<Vec<crate::schemas::IosVersion>>,
        #[doc = "The set of supported Xcode versions."]
        #[serde(rename = "xcodeVersions", default)]
        pub xcode_versions: Option<Vec<crate::schemas::XcodeVersion>>,
    }
    impl ::field_selector::FieldSelector for IosDeviceCatalog {
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
    pub struct IosDeviceList {
        #[doc = "Required. A list of iOS devices."]
        #[serde(rename = "iosDevices", default)]
        pub ios_devices: Option<Vec<crate::schemas::IosDevice>>,
    }
    impl ::field_selector::FieldSelector for IosDeviceList {
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
    pub enum IosModelFormFactor {
        #[doc = "Do not use. For proto versioning only."]
        DeviceFormFactorUnspecified,
        #[doc = "This device has the shape of a phone."]
        Phone,
        #[doc = "This device has the shape of a tablet."]
        Tablet,
        #[doc = "This device has the shape of a watch or other wearable."]
        Wearable,
    }
    impl IosModelFormFactor {
        pub fn as_str(self) -> &'static str {
            match self {
                IosModelFormFactor::DeviceFormFactorUnspecified => "DEVICE_FORM_FACTOR_UNSPECIFIED",
                IosModelFormFactor::Phone => "PHONE",
                IosModelFormFactor::Tablet => "TABLET",
                IosModelFormFactor::Wearable => "WEARABLE",
            }
        }
    }
    impl ::std::fmt::Display for IosModelFormFactor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IosModelFormFactor {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IosModelFormFactor {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_FORM_FACTOR_UNSPECIFIED" => IosModelFormFactor::DeviceFormFactorUnspecified,
                "PHONE" => IosModelFormFactor::Phone,
                "TABLET" => IosModelFormFactor::Tablet,
                "WEARABLE" => IosModelFormFactor::Wearable,
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
    pub struct IosModel {
        #[doc = "Device capabilities.\nCopied from\nhttps://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html"]
        #[serde(rename = "deviceCapabilities", default)]
        pub device_capabilities: Option<Vec<String>>,
        #[doc = "Whether this device is a phone, tablet, wearable, etc."]
        #[serde(rename = "formFactor", default)]
        pub form_factor: Option<crate::schemas::IosModelFormFactor>,
        #[doc = "The unique opaque id for this model.\nUse this for invoking the TestExecutionService."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The human-readable name for this device model.\nExamples: \"iPhone 4s\", \"iPad Mini 2\"."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The set of iOS major software versions this device supports."]
        #[serde(rename = "supportedVersionIds", default)]
        pub supported_version_ids: Option<Vec<String>>,
        #[doc = "Tags for this dimension.\nExamples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for IosModel {
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
    pub struct IosRuntimeConfiguration {
        #[doc = "The set of available locales."]
        #[serde(rename = "locales", default)]
        pub locales: Option<Vec<crate::schemas::Locale>>,
        #[doc = "The set of available orientations."]
        #[serde(rename = "orientations", default)]
        pub orientations: Option<Vec<crate::schemas::Orientation>>,
    }
    impl ::field_selector::FieldSelector for IosRuntimeConfiguration {
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
    pub struct IosTestSetup {
        #[doc = "The network traffic profile used for running the test.\nAvailable network profiles can be queried by using the\nNETWORK_CONFIGURATION environment type when calling\nTestEnvironmentDiscoveryService.GetTestEnvironmentCatalog."]
        #[serde(rename = "networkProfile", default)]
        pub network_profile: Option<String>,
    }
    impl ::field_selector::FieldSelector for IosTestSetup {
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
    pub struct IosVersion {
        #[doc = "An opaque id for this iOS version.\nUse this id to invoke the TestExecutionService."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "An integer representing the major iOS version.\nExamples: \"8\", \"9\"."]
        #[serde(rename = "majorVersion", default)]
        pub major_version: Option<i32>,
        #[doc = "An integer representing the minor iOS version.\nExamples: \"1\", \"2\"."]
        #[serde(rename = "minorVersion", default)]
        pub minor_version: Option<i32>,
        #[doc = "The available Xcode versions for this version."]
        #[serde(rename = "supportedXcodeVersionIds", default)]
        pub supported_xcode_version_ids: Option<Vec<String>>,
        #[doc = "Tags for this dimension.\nExamples: \"default\", \"preview\", \"deprecated\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for IosVersion {
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
    pub struct IosXcTest {
        #[doc = "Output only. The bundle id for the application under test."]
        #[serde(rename = "appBundleId", default)]
        pub app_bundle_id: Option<String>,
        #[doc = "Required. The .zip containing the .xctestrun file and the contents of the\nDerivedData/Build/Products directory.\nThe .xctestrun file in this zip is ignored if the xctestrun field is\nspecified."]
        #[serde(rename = "testsZip", default)]
        pub tests_zip: Option<crate::schemas::FileReference>,
        #[doc = "The Xcode version that should be used for the test.\nUse the TestEnvironmentDiscoveryService to get supported options.\nDefaults to the latest Xcode version Firebase Test Lab supports."]
        #[serde(rename = "xcodeVersion", default)]
        pub xcode_version: Option<String>,
        #[doc = "An .xctestrun file that will override the .xctestrun file in the\ntests zip. Because the .xctestrun file contains environment variables along\nwith test methods to run and/or ignore, this can be useful for sharding\ntests. Default is taken from the tests zip."]
        #[serde(rename = "xctestrun", default)]
        pub xctestrun: Option<crate::schemas::FileReference>,
    }
    impl ::field_selector::FieldSelector for IosXcTest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LauncherActivityIntent;
    impl ::field_selector::FieldSelector for LauncherActivityIntent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {}
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
        #[doc = "The id for this locale.\nExample: \"en_US\"."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "A human-friendly name for this language/locale.\nExample: \"English\"."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "A human-friendly string representing the region for this\nlocale. Example: \"United States\". Not present for every locale."]
        #[serde(rename = "region", default)]
        pub region: Option<String>,
        #[doc = "Tags for this dimension.\nExample: \"default\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Locale {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NetworkConfiguration {
        #[doc = "The emulation rule applying to the download traffic."]
        #[serde(rename = "downRule", default)]
        pub down_rule: Option<crate::schemas::TrafficRule>,
        #[doc = "The unique opaque id for this network traffic configuration."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The emulation rule applying to the upload traffic."]
        #[serde(rename = "upRule", default)]
        pub up_rule: Option<crate::schemas::TrafficRule>,
    }
    impl ::field_selector::FieldSelector for NetworkConfiguration {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NetworkConfigurationCatalog {
        #[serde(rename = "configurations", default)]
        pub configurations: Option<Vec<crate::schemas::NetworkConfiguration>>,
    }
    impl ::field_selector::FieldSelector for NetworkConfigurationCatalog {
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
    pub struct ObbFile {
        #[doc = "Required. Opaque Binary Blob (OBB) file(s) to install on the device."]
        #[serde(rename = "obb", default)]
        pub obb: Option<crate::schemas::FileReference>,
        #[doc = "Required. OBB file name which must conform to the format as specified by\nAndroid\ne.g. [main|patch].0300110.com.example.android.obb\nwhich will be installed into\n<shared-storage>/Android/obb/<package-name>/\non the device."]
        #[serde(rename = "obbFileName", default)]
        pub obb_file_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ObbFile {
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
    pub struct Orientation {
        #[doc = "The id for this orientation.\nExample: \"portrait\"."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "A human-friendly name for this orientation.\nExample: \"portrait\"."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Tags for this dimension.\nExample: \"default\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Orientation {
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
    pub struct ProvidedSoftwareCatalog {
        #[doc = "A string representing the current version of Android Test\nOrchestrator that is provided by TestExecutionService.\nExample: \"1.0.2 beta\"."]
        #[serde(rename = "orchestratorVersion", default)]
        pub orchestrator_version: Option<String>,
    }
    impl ::field_selector::FieldSelector for ProvidedSoftwareCatalog {
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
    pub struct RegularFile {
        #[doc = "Required. The source file."]
        #[serde(rename = "content", default)]
        pub content: Option<crate::schemas::FileReference>,
        #[doc = "Required. Where to put the content on the device. Must be an absolute,\nwhitelisted path. If the file exists, it will be replaced.\nThe following device-side directories and any of their subdirectories are\nwhitelisted:\n\n<p>${EXTERNAL_STORAGE}, or /sdcard</p>\n<p>${ANDROID_DATA}/local/tmp, or /data/local/tmp</p>\n<p>Specifying a path outside of these directory trees is invalid.\n\n<p> The paths /sdcard and /data will be made available and treated as\nimplicit path substitutions. E.g. if /sdcard on a particular device does\nnot map to external storage, the system will replace it with the external\nstorage path prefix for that device and copy the file there.\n\n<p> It is strongly advised to use the <a href=\n\"http://developer.android.com/reference/android/os/Environment.html\">\nEnvironment API</a> in app and test code to access files on the device in a\nportable way."]
        #[serde(rename = "devicePath", default)]
        pub device_path: Option<String>,
    }
    impl ::field_selector::FieldSelector for RegularFile {
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
    pub struct ResultStorage {
        #[doc = "Required."]
        #[serde(rename = "googleCloudStorage", default)]
        pub google_cloud_storage: Option<crate::schemas::GoogleCloudStorage>,
        #[doc = "Output only. URL to the results in the Firebase Web Console."]
        #[serde(rename = "resultsUrl", default)]
        pub results_url: Option<String>,
        #[doc = "Output only. The tool results execution that results are written to."]
        #[serde(rename = "toolResultsExecution", default)]
        pub tool_results_execution: Option<crate::schemas::ToolResultsExecution>,
        #[doc = "The tool results history that contains the tool results execution that\nresults are written to.\n\nIf not provided, the service will choose an appropriate value."]
        #[serde(rename = "toolResultsHistory", default)]
        pub tool_results_history: Option<crate::schemas::ToolResultsHistory>,
    }
    impl ::field_selector::FieldSelector for ResultStorage {
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
    pub enum RoboDirectiveActionType {
        #[doc = "DO NOT USE. For proto versioning only."]
        ActionTypeUnspecified,
        #[doc = "Direct Robo to click on the specified element. No-op if specified element\nis not clickable."]
        SingleClick,
        #[doc = "Direct Robo to enter text on the specified element. No-op if specified\nelement is not enabled or does not allow text entry."]
        EnterText,
        #[doc = "Direct Robo to ignore interactions with a specific element."]
        Ignore,
    }
    impl RoboDirectiveActionType {
        pub fn as_str(self) -> &'static str {
            match self {
                RoboDirectiveActionType::ActionTypeUnspecified => "ACTION_TYPE_UNSPECIFIED",
                RoboDirectiveActionType::SingleClick => "SINGLE_CLICK",
                RoboDirectiveActionType::EnterText => "ENTER_TEXT",
                RoboDirectiveActionType::Ignore => "IGNORE",
            }
        }
    }
    impl ::std::fmt::Display for RoboDirectiveActionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RoboDirectiveActionType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RoboDirectiveActionType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTION_TYPE_UNSPECIFIED" => RoboDirectiveActionType::ActionTypeUnspecified,
                "SINGLE_CLICK" => RoboDirectiveActionType::SingleClick,
                "ENTER_TEXT" => RoboDirectiveActionType::EnterText,
                "IGNORE" => RoboDirectiveActionType::Ignore,
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
    pub struct RoboDirective {
        #[doc = "Required. The type of action that Robo should perform on the specified\nelement."]
        #[serde(rename = "actionType", default)]
        pub action_type: Option<crate::schemas::RoboDirectiveActionType>,
        #[doc = "The text that Robo is directed to set. If left empty, the directive will be\ntreated as a CLICK on the element matching the resource_name."]
        #[serde(rename = "inputText", default)]
        pub input_text: Option<String>,
        #[doc = "Required. The android resource name of the target UI element.\nFor example,\nin Java: R.string.foo\nin xml: @string/foo\nOnly the \"foo\" part is needed.\nReference doc:\nhttps://developer.android.com/guide/topics/resources/accessing-resources.html"]
        #[serde(rename = "resourceName", default)]
        pub resource_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for RoboDirective {
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
    pub struct RoboStartingIntent {
        #[doc = "An intent that starts the main launcher activity."]
        #[serde(rename = "launcherActivity", default)]
        pub launcher_activity: Option<crate::schemas::LauncherActivityIntent>,
        #[doc = "An intent that starts an activity with specific details."]
        #[serde(rename = "startActivity", default)]
        pub start_activity: Option<crate::schemas::StartActivityIntent>,
        #[doc = "Timeout in seconds for each intent."]
        #[serde(rename = "timeout", default)]
        pub timeout: Option<String>,
    }
    impl ::field_selector::FieldSelector for RoboStartingIntent {
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
    pub struct StartActivityIntent {
        #[doc = "Action name.\nRequired for START_ACTIVITY."]
        #[serde(rename = "action", default)]
        pub action: Option<String>,
        #[doc = "Intent categories to set on the intent."]
        #[serde(rename = "categories", default)]
        pub categories: Option<Vec<String>>,
        #[doc = "URI for the action."]
        #[serde(rename = "uri", default)]
        pub uri: Option<String>,
    }
    impl ::field_selector::FieldSelector for StartActivityIntent {
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
    pub struct TestDetails {
        #[doc = "Output only. If the TestState is ERROR, then this string will contain\nhuman-readable details about the error."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: Option<String>,
        #[doc = "Output only. Human-readable, detailed descriptions of the test's progress.\nFor example: \"Provisioning a device\", \"Starting Test\".\n\nDuring the course of execution new data may be appended\nto the end of progress_messages."]
        #[serde(rename = "progressMessages", default)]
        pub progress_messages: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TestDetails {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TestEnvironmentCatalog {
        #[doc = "Supported Android devices."]
        #[serde(rename = "androidDeviceCatalog", default)]
        pub android_device_catalog: Option<crate::schemas::AndroidDeviceCatalog>,
        #[doc = "Supported iOS devices."]
        #[serde(rename = "iosDeviceCatalog", default)]
        pub ios_device_catalog: Option<crate::schemas::IosDeviceCatalog>,
        #[doc = "Supported network configurations."]
        #[serde(rename = "networkConfigurationCatalog", default)]
        pub network_configuration_catalog: Option<crate::schemas::NetworkConfigurationCatalog>,
        #[doc = "The software test environment provided by TestExecutionService."]
        #[serde(rename = "softwareCatalog", default)]
        pub software_catalog: Option<crate::schemas::ProvidedSoftwareCatalog>,
    }
    impl ::field_selector::FieldSelector for TestEnvironmentCatalog {
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
    pub enum TestExecutionState {
        #[doc = "Do not use.  For proto versioning only."]
        TestStateUnspecified,
        #[doc = "The execution or matrix is being validated."]
        Validating,
        #[doc = "The execution or matrix is waiting for resources to become available."]
        Pending,
        #[doc = "The execution is currently being processed.\n\nCan only be set on an execution."]
        Running,
        #[doc = "The execution or matrix has terminated normally.\n\nOn a matrix this means that the matrix level processing completed normally,\nbut individual executions may be in an ERROR state."]
        Finished,
        #[doc = "The execution or matrix has stopped because it encountered an\ninfrastructure failure."]
        Error,
        #[doc = "The execution was not run because it corresponds to a unsupported\nenvironment.\n\nCan only be set on an execution."]
        UnsupportedEnvironment,
        #[doc = "The execution was not run because the provided inputs are incompatible with\nthe requested environment.\n\nExample: requested AndroidVersion is lower than APK's minSdkVersion\n\nCan only be set on an execution."]
        IncompatibleEnvironment,
        #[doc = "The execution was not run because the provided inputs are incompatible with\nthe requested architecture.\n\nExample: requested device does not support running the native code in\nthe supplied APK\n\nCan only be set on an execution."]
        IncompatibleArchitecture,
        #[doc = "The user cancelled the execution.\n\nCan only be set on an execution."]
        Cancelled,
        #[doc = "The execution or matrix was not run because the provided inputs are not\nvalid.\n\nExamples: input file is not of the expected type, is malformed/corrupt, or\nwas flagged as malware"]
        Invalid,
    }
    impl TestExecutionState {
        pub fn as_str(self) -> &'static str {
            match self {
                TestExecutionState::TestStateUnspecified => "TEST_STATE_UNSPECIFIED",
                TestExecutionState::Validating => "VALIDATING",
                TestExecutionState::Pending => "PENDING",
                TestExecutionState::Running => "RUNNING",
                TestExecutionState::Finished => "FINISHED",
                TestExecutionState::Error => "ERROR",
                TestExecutionState::UnsupportedEnvironment => "UNSUPPORTED_ENVIRONMENT",
                TestExecutionState::IncompatibleEnvironment => "INCOMPATIBLE_ENVIRONMENT",
                TestExecutionState::IncompatibleArchitecture => "INCOMPATIBLE_ARCHITECTURE",
                TestExecutionState::Cancelled => "CANCELLED",
                TestExecutionState::Invalid => "INVALID",
            }
        }
    }
    impl ::std::fmt::Display for TestExecutionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestExecutionState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestExecutionState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TEST_STATE_UNSPECIFIED" => TestExecutionState::TestStateUnspecified,
                "VALIDATING" => TestExecutionState::Validating,
                "PENDING" => TestExecutionState::Pending,
                "RUNNING" => TestExecutionState::Running,
                "FINISHED" => TestExecutionState::Finished,
                "ERROR" => TestExecutionState::Error,
                "UNSUPPORTED_ENVIRONMENT" => TestExecutionState::UnsupportedEnvironment,
                "INCOMPATIBLE_ENVIRONMENT" => TestExecutionState::IncompatibleEnvironment,
                "INCOMPATIBLE_ARCHITECTURE" => TestExecutionState::IncompatibleArchitecture,
                "CANCELLED" => TestExecutionState::Cancelled,
                "INVALID" => TestExecutionState::Invalid,
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
    pub struct TestExecution {
        #[doc = "Output only. How the host machine(s) are configured."]
        #[serde(rename = "environment", default)]
        pub environment: Option<crate::schemas::Environment>,
        #[doc = "Output only. Unique id set by the service."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Output only. Id of the containing TestMatrix."]
        #[serde(rename = "matrixId", default)]
        pub matrix_id: Option<String>,
        #[doc = "Output only. The cloud project that owns the test execution."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Output only. Indicates the current progress of the test execution\n(e.g., FINISHED)."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::TestExecutionState>,
        #[doc = "Output only. Additional details about the running test."]
        #[serde(rename = "testDetails", default)]
        pub test_details: Option<crate::schemas::TestDetails>,
        #[doc = "Output only. How to run the test."]
        #[serde(rename = "testSpecification", default)]
        pub test_specification: Option<crate::schemas::TestSpecification>,
        #[doc = "Output only. The time this test execution was initially created."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: Option<String>,
        #[doc = "Output only. Where the results for this execution are written."]
        #[serde(rename = "toolResultsStep", default)]
        pub tool_results_step: Option<crate::schemas::ToolResultsStep>,
    }
    impl ::field_selector::FieldSelector for TestExecution {
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
    pub enum TestMatrixInvalidMatrixDetails {
        #[doc = "Do not use. For proto versioning only."]
        InvalidMatrixDetailsUnspecified,
        #[doc = "The matrix is INVALID, but there are no further details available."]
        DetailsUnavailable,
        #[doc = "The input app APK could not be parsed."]
        MalformedApk,
        #[doc = "The input test APK could not be parsed."]
        MalformedTestApk,
        #[doc = "The AndroidManifest.xml could not be found."]
        NoManifest,
        #[doc = "The APK manifest does not declare a package name."]
        NoPackageName,
        #[doc = "The APK application ID (aka package name) is invalid.\nSee also\nhttps://developer.android.com/studio/build/application-id"]
        InvalidPackageName,
        #[doc = "The test package and app package are the same."]
        TestSameAsApp,
        #[doc = "The test apk does not declare an instrumentation."]
        NoInstrumentation,
        #[doc = "The input app apk does not have a signature."]
        NoSignature,
        #[doc = "The test runner class specified by user or in the test APK's manifest file\nis not compatible with Android Test Orchestrator.\nOrchestrator is only compatible with AndroidJUnitRunner version 1.0 or\nhigher.\nOrchestrator can be disabled by using DO_NOT_USE_ORCHESTRATOR\nOrchestratorOption."]
        InstrumentationOrchestratorIncompatible,
        #[doc = "The test APK does not contain the test runner class specified by user or in\nthe manifest file.\nThis can be caused by either of the following reasons:\n\n* the user provided a runner class name that's incorrect, or\n* the test runner isn't built into the test APK (might be in the app APK\n  instead)."]
        NoTestRunnerClass,
        #[doc = "A main launcher activity could not be found."]
        NoLauncherActivity,
        #[doc = "The app declares one or more permissions that are not allowed."]
        ForbiddenPermissions,
        #[doc = "There is a conflict in the provided robo_directives."]
        InvalidRoboDirectives,
        #[doc = "There is at least one invalid resource name in the provided\nrobo directives"]
        InvalidResourceName,
        #[doc = "Invalid definition of action in the robo directives\n(e.g. a click or ignore action includes an input text field)"]
        InvalidDirectiveAction,
        #[doc = "There is no test loop intent filter, or the one that is given is\nnot formatted correctly."]
        TestLoopIntentFilterNotFound,
        #[doc = "The request contains a scenario label that was not declared in the\nmanifest."]
        ScenarioLabelNotDeclared,
        #[doc = "There was an error when parsing a label's value."]
        ScenarioLabelMalformed,
        #[doc = "The request contains a scenario number that was not declared in the\nmanifest."]
        ScenarioNotDeclared,
        #[doc = "Device administrator applications are not allowed."]
        DeviceAdminReceiver,
        #[doc = "The zipped XCTest was malformed. The zip did not contain a single\n.xctestrun file and the contents of the DerivedData/Build/Products\ndirectory."]
        MalformedXcTestZip,
        #[doc = "The zipped XCTest was built for the iOS simulator rather than for a\nphysical device."]
        BuiltForIosSimulator,
        #[doc = "The .xctestrun file did not specify any test targets."]
        NoTestsInXcTestZip,
        #[doc = "One or more of the test targets defined in the .xctestrun file specifies\n\"UseDestinationArtifacts\", which is disallowed."]
        UseDestinationArtifacts,
        #[doc = "XC tests which run on physical devices must have\n\"IsAppHostedTestBundle\" == \"true\" in the xctestrun file."]
        TestNotAppHosted,
        #[doc = "An Info.plist file in the XCTest zip could not be parsed."]
        PlistCannotBeParsed,
        #[doc = "The APK is marked as \"testOnly\".\nDeprecated and not currently used."]
        TestOnlyApk,
        #[doc = "The input IPA could not be parsed.\nDeprecated and not currently used."]
        MalformedIpa,
        #[doc = "APK contains no code.\nSee also\nhttps://developer.android.com/guide/topics/manifest/application-element.html#code"]
        NoCodeApk,
        #[doc = "Either the provided input APK path was malformed,\nthe APK file does not exist, or the user does not have permission to\naccess the APK file."]
        InvalidInputApk,
        #[doc = "APK is built for a preview SDK which is unsupported"]
        InvalidApkPreviewSdk,
    }
    impl TestMatrixInvalidMatrixDetails {
        pub fn as_str(self) -> &'static str {
            match self {
                TestMatrixInvalidMatrixDetails::InvalidMatrixDetailsUnspecified => {
                    "INVALID_MATRIX_DETAILS_UNSPECIFIED"
                }
                TestMatrixInvalidMatrixDetails::DetailsUnavailable => "DETAILS_UNAVAILABLE",
                TestMatrixInvalidMatrixDetails::MalformedApk => "MALFORMED_APK",
                TestMatrixInvalidMatrixDetails::MalformedTestApk => "MALFORMED_TEST_APK",
                TestMatrixInvalidMatrixDetails::NoManifest => "NO_MANIFEST",
                TestMatrixInvalidMatrixDetails::NoPackageName => "NO_PACKAGE_NAME",
                TestMatrixInvalidMatrixDetails::InvalidPackageName => "INVALID_PACKAGE_NAME",
                TestMatrixInvalidMatrixDetails::TestSameAsApp => "TEST_SAME_AS_APP",
                TestMatrixInvalidMatrixDetails::NoInstrumentation => "NO_INSTRUMENTATION",
                TestMatrixInvalidMatrixDetails::NoSignature => "NO_SIGNATURE",
                TestMatrixInvalidMatrixDetails::InstrumentationOrchestratorIncompatible => {
                    "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE"
                }
                TestMatrixInvalidMatrixDetails::NoTestRunnerClass => "NO_TEST_RUNNER_CLASS",
                TestMatrixInvalidMatrixDetails::NoLauncherActivity => "NO_LAUNCHER_ACTIVITY",
                TestMatrixInvalidMatrixDetails::ForbiddenPermissions => "FORBIDDEN_PERMISSIONS",
                TestMatrixInvalidMatrixDetails::InvalidRoboDirectives => "INVALID_ROBO_DIRECTIVES",
                TestMatrixInvalidMatrixDetails::InvalidResourceName => "INVALID_RESOURCE_NAME",
                TestMatrixInvalidMatrixDetails::InvalidDirectiveAction => {
                    "INVALID_DIRECTIVE_ACTION"
                }
                TestMatrixInvalidMatrixDetails::TestLoopIntentFilterNotFound => {
                    "TEST_LOOP_INTENT_FILTER_NOT_FOUND"
                }
                TestMatrixInvalidMatrixDetails::ScenarioLabelNotDeclared => {
                    "SCENARIO_LABEL_NOT_DECLARED"
                }
                TestMatrixInvalidMatrixDetails::ScenarioLabelMalformed => {
                    "SCENARIO_LABEL_MALFORMED"
                }
                TestMatrixInvalidMatrixDetails::ScenarioNotDeclared => "SCENARIO_NOT_DECLARED",
                TestMatrixInvalidMatrixDetails::DeviceAdminReceiver => "DEVICE_ADMIN_RECEIVER",
                TestMatrixInvalidMatrixDetails::MalformedXcTestZip => "MALFORMED_XC_TEST_ZIP",
                TestMatrixInvalidMatrixDetails::BuiltForIosSimulator => "BUILT_FOR_IOS_SIMULATOR",
                TestMatrixInvalidMatrixDetails::NoTestsInXcTestZip => "NO_TESTS_IN_XC_TEST_ZIP",
                TestMatrixInvalidMatrixDetails::UseDestinationArtifacts => {
                    "USE_DESTINATION_ARTIFACTS"
                }
                TestMatrixInvalidMatrixDetails::TestNotAppHosted => "TEST_NOT_APP_HOSTED",
                TestMatrixInvalidMatrixDetails::PlistCannotBeParsed => "PLIST_CANNOT_BE_PARSED",
                TestMatrixInvalidMatrixDetails::TestOnlyApk => "TEST_ONLY_APK",
                TestMatrixInvalidMatrixDetails::MalformedIpa => "MALFORMED_IPA",
                TestMatrixInvalidMatrixDetails::NoCodeApk => "NO_CODE_APK",
                TestMatrixInvalidMatrixDetails::InvalidInputApk => "INVALID_INPUT_APK",
                TestMatrixInvalidMatrixDetails::InvalidApkPreviewSdk => "INVALID_APK_PREVIEW_SDK",
            }
        }
    }
    impl ::std::fmt::Display for TestMatrixInvalidMatrixDetails {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestMatrixInvalidMatrixDetails {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestMatrixInvalidMatrixDetails {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVALID_MATRIX_DETAILS_UNSPECIFIED" => {
                    TestMatrixInvalidMatrixDetails::InvalidMatrixDetailsUnspecified
                }
                "DETAILS_UNAVAILABLE" => TestMatrixInvalidMatrixDetails::DetailsUnavailable,
                "MALFORMED_APK" => TestMatrixInvalidMatrixDetails::MalformedApk,
                "MALFORMED_TEST_APK" => TestMatrixInvalidMatrixDetails::MalformedTestApk,
                "NO_MANIFEST" => TestMatrixInvalidMatrixDetails::NoManifest,
                "NO_PACKAGE_NAME" => TestMatrixInvalidMatrixDetails::NoPackageName,
                "INVALID_PACKAGE_NAME" => TestMatrixInvalidMatrixDetails::InvalidPackageName,
                "TEST_SAME_AS_APP" => TestMatrixInvalidMatrixDetails::TestSameAsApp,
                "NO_INSTRUMENTATION" => TestMatrixInvalidMatrixDetails::NoInstrumentation,
                "NO_SIGNATURE" => TestMatrixInvalidMatrixDetails::NoSignature,
                "INSTRUMENTATION_ORCHESTRATOR_INCOMPATIBLE" => {
                    TestMatrixInvalidMatrixDetails::InstrumentationOrchestratorIncompatible
                }
                "NO_TEST_RUNNER_CLASS" => TestMatrixInvalidMatrixDetails::NoTestRunnerClass,
                "NO_LAUNCHER_ACTIVITY" => TestMatrixInvalidMatrixDetails::NoLauncherActivity,
                "FORBIDDEN_PERMISSIONS" => TestMatrixInvalidMatrixDetails::ForbiddenPermissions,
                "INVALID_ROBO_DIRECTIVES" => TestMatrixInvalidMatrixDetails::InvalidRoboDirectives,
                "INVALID_RESOURCE_NAME" => TestMatrixInvalidMatrixDetails::InvalidResourceName,
                "INVALID_DIRECTIVE_ACTION" => {
                    TestMatrixInvalidMatrixDetails::InvalidDirectiveAction
                }
                "TEST_LOOP_INTENT_FILTER_NOT_FOUND" => {
                    TestMatrixInvalidMatrixDetails::TestLoopIntentFilterNotFound
                }
                "SCENARIO_LABEL_NOT_DECLARED" => {
                    TestMatrixInvalidMatrixDetails::ScenarioLabelNotDeclared
                }
                "SCENARIO_LABEL_MALFORMED" => {
                    TestMatrixInvalidMatrixDetails::ScenarioLabelMalformed
                }
                "SCENARIO_NOT_DECLARED" => TestMatrixInvalidMatrixDetails::ScenarioNotDeclared,
                "DEVICE_ADMIN_RECEIVER" => TestMatrixInvalidMatrixDetails::DeviceAdminReceiver,
                "MALFORMED_XC_TEST_ZIP" => TestMatrixInvalidMatrixDetails::MalformedXcTestZip,
                "BUILT_FOR_IOS_SIMULATOR" => TestMatrixInvalidMatrixDetails::BuiltForIosSimulator,
                "NO_TESTS_IN_XC_TEST_ZIP" => TestMatrixInvalidMatrixDetails::NoTestsInXcTestZip,
                "USE_DESTINATION_ARTIFACTS" => {
                    TestMatrixInvalidMatrixDetails::UseDestinationArtifacts
                }
                "TEST_NOT_APP_HOSTED" => TestMatrixInvalidMatrixDetails::TestNotAppHosted,
                "PLIST_CANNOT_BE_PARSED" => TestMatrixInvalidMatrixDetails::PlistCannotBeParsed,
                "TEST_ONLY_APK" => TestMatrixInvalidMatrixDetails::TestOnlyApk,
                "MALFORMED_IPA" => TestMatrixInvalidMatrixDetails::MalformedIpa,
                "NO_CODE_APK" => TestMatrixInvalidMatrixDetails::NoCodeApk,
                "INVALID_INPUT_APK" => TestMatrixInvalidMatrixDetails::InvalidInputApk,
                "INVALID_APK_PREVIEW_SDK" => TestMatrixInvalidMatrixDetails::InvalidApkPreviewSdk,
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
    pub enum TestMatrixOutcomeSummary {
        #[doc = "Do not use. For proto versioning only."]
        OutcomeSummaryUnspecified,
        #[doc = "The test matrix run was successful, for instance:\n\n* All the test cases passed.\n* Robo did not detect a crash of the application under test."]
        Success,
        #[doc = "A run failed, for instance:\n\n* One or more test case failed.\n* A test timed out.\n* The application under test crashed."]
        Failure,
        #[doc = "Something unexpected happened. The run should still be considered\nunsuccessful but this is likely a transient problem and re-running the\ntest might be successful."]
        Inconclusive,
        #[doc = "All tests were skipped, for instance:\n\n* All device configurations were incompatible."]
        Skipped,
    }
    impl TestMatrixOutcomeSummary {
        pub fn as_str(self) -> &'static str {
            match self {
                TestMatrixOutcomeSummary::OutcomeSummaryUnspecified => {
                    "OUTCOME_SUMMARY_UNSPECIFIED"
                }
                TestMatrixOutcomeSummary::Success => "SUCCESS",
                TestMatrixOutcomeSummary::Failure => "FAILURE",
                TestMatrixOutcomeSummary::Inconclusive => "INCONCLUSIVE",
                TestMatrixOutcomeSummary::Skipped => "SKIPPED",
            }
        }
    }
    impl ::std::fmt::Display for TestMatrixOutcomeSummary {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestMatrixOutcomeSummary {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestMatrixOutcomeSummary {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OUTCOME_SUMMARY_UNSPECIFIED" => {
                    TestMatrixOutcomeSummary::OutcomeSummaryUnspecified
                }
                "SUCCESS" => TestMatrixOutcomeSummary::Success,
                "FAILURE" => TestMatrixOutcomeSummary::Failure,
                "INCONCLUSIVE" => TestMatrixOutcomeSummary::Inconclusive,
                "SKIPPED" => TestMatrixOutcomeSummary::Skipped,
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
    pub enum TestMatrixState {
        #[doc = "Do not use.  For proto versioning only."]
        TestStateUnspecified,
        #[doc = "The execution or matrix is being validated."]
        Validating,
        #[doc = "The execution or matrix is waiting for resources to become available."]
        Pending,
        #[doc = "The execution is currently being processed.\n\nCan only be set on an execution."]
        Running,
        #[doc = "The execution or matrix has terminated normally.\n\nOn a matrix this means that the matrix level processing completed normally,\nbut individual executions may be in an ERROR state."]
        Finished,
        #[doc = "The execution or matrix has stopped because it encountered an\ninfrastructure failure."]
        Error,
        #[doc = "The execution was not run because it corresponds to a unsupported\nenvironment.\n\nCan only be set on an execution."]
        UnsupportedEnvironment,
        #[doc = "The execution was not run because the provided inputs are incompatible with\nthe requested environment.\n\nExample: requested AndroidVersion is lower than APK's minSdkVersion\n\nCan only be set on an execution."]
        IncompatibleEnvironment,
        #[doc = "The execution was not run because the provided inputs are incompatible with\nthe requested architecture.\n\nExample: requested device does not support running the native code in\nthe supplied APK\n\nCan only be set on an execution."]
        IncompatibleArchitecture,
        #[doc = "The user cancelled the execution.\n\nCan only be set on an execution."]
        Cancelled,
        #[doc = "The execution or matrix was not run because the provided inputs are not\nvalid.\n\nExamples: input file is not of the expected type, is malformed/corrupt, or\nwas flagged as malware"]
        Invalid,
    }
    impl TestMatrixState {
        pub fn as_str(self) -> &'static str {
            match self {
                TestMatrixState::TestStateUnspecified => "TEST_STATE_UNSPECIFIED",
                TestMatrixState::Validating => "VALIDATING",
                TestMatrixState::Pending => "PENDING",
                TestMatrixState::Running => "RUNNING",
                TestMatrixState::Finished => "FINISHED",
                TestMatrixState::Error => "ERROR",
                TestMatrixState::UnsupportedEnvironment => "UNSUPPORTED_ENVIRONMENT",
                TestMatrixState::IncompatibleEnvironment => "INCOMPATIBLE_ENVIRONMENT",
                TestMatrixState::IncompatibleArchitecture => "INCOMPATIBLE_ARCHITECTURE",
                TestMatrixState::Cancelled => "CANCELLED",
                TestMatrixState::Invalid => "INVALID",
            }
        }
    }
    impl ::std::fmt::Display for TestMatrixState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TestMatrixState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TestMatrixState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TEST_STATE_UNSPECIFIED" => TestMatrixState::TestStateUnspecified,
                "VALIDATING" => TestMatrixState::Validating,
                "PENDING" => TestMatrixState::Pending,
                "RUNNING" => TestMatrixState::Running,
                "FINISHED" => TestMatrixState::Finished,
                "ERROR" => TestMatrixState::Error,
                "UNSUPPORTED_ENVIRONMENT" => TestMatrixState::UnsupportedEnvironment,
                "INCOMPATIBLE_ENVIRONMENT" => TestMatrixState::IncompatibleEnvironment,
                "INCOMPATIBLE_ARCHITECTURE" => TestMatrixState::IncompatibleArchitecture,
                "CANCELLED" => TestMatrixState::Cancelled,
                "INVALID" => TestMatrixState::Invalid,
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
    pub struct TestMatrix {
        #[doc = "Information about the client which invoked the test."]
        #[serde(rename = "clientInfo", default)]
        pub client_info: Option<crate::schemas::ClientInfo>,
        #[doc = "Required. The devices the tests are being executed on."]
        #[serde(rename = "environmentMatrix", default)]
        pub environment_matrix: Option<crate::schemas::EnvironmentMatrix>,
        #[doc = "The number of times a TestExecution should be re-attempted if one or more\nof its test cases fail for any reason.\nThe maximum number of reruns allowed is 10.\n\nDefault is 0, which implies no reruns."]
        #[serde(rename = "flakyTestAttempts", default)]
        pub flaky_test_attempts: Option<i32>,
        #[doc = "Output only. Describes why the matrix is considered invalid.\nOnly useful for matrices in the INVALID state."]
        #[serde(rename = "invalidMatrixDetails", default)]
        pub invalid_matrix_details: Option<crate::schemas::TestMatrixInvalidMatrixDetails>,
        #[doc = "Output Only. The overall outcome of the test.\nOnly set when the test matrix state is FINISHED."]
        #[serde(rename = "outcomeSummary", default)]
        pub outcome_summary: Option<crate::schemas::TestMatrixOutcomeSummary>,
        #[doc = "The cloud project that owns the test matrix."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Required. Where the results for the matrix are written."]
        #[serde(rename = "resultStorage", default)]
        pub result_storage: Option<crate::schemas::ResultStorage>,
        #[doc = "Output only. Indicates the current progress of the test matrix."]
        #[serde(rename = "state", default)]
        pub state: Option<crate::schemas::TestMatrixState>,
        #[doc = "Output only. The list of test executions that the service creates for\nthis matrix."]
        #[serde(rename = "testExecutions", default)]
        pub test_executions: Option<Vec<crate::schemas::TestExecution>>,
        #[doc = "Output only. Unique id set by the service."]
        #[serde(rename = "testMatrixId", default)]
        pub test_matrix_id: Option<String>,
        #[doc = "Required. How to run the test."]
        #[serde(rename = "testSpecification", default)]
        pub test_specification: Option<crate::schemas::TestSpecification>,
        #[doc = "Output only. The time this test matrix was initially created."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: Option<String>,
    }
    impl ::field_selector::FieldSelector for TestMatrix {
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
    pub struct TestSetup {
        #[doc = "The device will be logged in on this account for the duration of the test."]
        #[serde(rename = "account", default)]
        pub account: Option<crate::schemas::Account>,
        #[doc = "APKs to install in addition to those being directly tested.\nCurrently capped at 100."]
        #[serde(rename = "additionalApks", default)]
        pub additional_apks: Option<Vec<crate::schemas::Apk>>,
        #[doc = "List of directories on the device to upload to GCS at the end of the test;\nthey must be absolute paths under /sdcard or /data/local/tmp.\nPath names are restricted to characters a-z A-Z 0-9 _ - . + and /\n\nNote: The paths /sdcard and /data will be made available and treated as\nimplicit path substitutions. E.g. if /sdcard on a particular device does\nnot map to external storage, the system will replace it with the external\nstorage path prefix for that device."]
        #[serde(rename = "directoriesToPull", default)]
        pub directories_to_pull: Option<Vec<String>>,
        #[doc = "Environment variables to set for the test (only applicable for\ninstrumentation tests)."]
        #[serde(rename = "environmentVariables", default)]
        pub environment_variables: Option<Vec<crate::schemas::EnvironmentVariable>>,
        #[doc = "List of files to push to the device before starting the test."]
        #[serde(rename = "filesToPush", default)]
        pub files_to_push: Option<Vec<crate::schemas::DeviceFile>>,
        #[doc = "The network traffic profile used for running the test.\nAvailable network profiles can be queried by using the\nNETWORK_CONFIGURATION environment type when calling\nTestEnvironmentDiscoveryService.GetTestEnvironmentCatalog."]
        #[serde(rename = "networkProfile", default)]
        pub network_profile: Option<String>,
    }
    impl ::field_selector::FieldSelector for TestSetup {
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
    pub struct TestSpecification {
        #[doc = "An Android instrumentation test."]
        #[serde(rename = "androidInstrumentationTest", default)]
        pub android_instrumentation_test: Option<crate::schemas::AndroidInstrumentationTest>,
        #[doc = "An Android robo test."]
        #[serde(rename = "androidRoboTest", default)]
        pub android_robo_test: Option<crate::schemas::AndroidRoboTest>,
        #[doc = "An Android Application with a Test Loop."]
        #[serde(rename = "androidTestLoop", default)]
        pub android_test_loop: Option<crate::schemas::AndroidTestLoop>,
        #[doc = "Disables performance metrics recording. May reduce test latency."]
        #[serde(rename = "disablePerformanceMetrics", default)]
        pub disable_performance_metrics: Option<bool>,
        #[doc = "Disables video recording. May reduce test latency."]
        #[serde(rename = "disableVideoRecording", default)]
        pub disable_video_recording: Option<bool>,
        #[doc = "Test setup requirements for iOS."]
        #[serde(rename = "iosTestSetup", default)]
        pub ios_test_setup: Option<crate::schemas::IosTestSetup>,
        #[doc = "An iOS XCTest, via an .xctestrun file."]
        #[serde(rename = "iosXcTest", default)]
        pub ios_xc_test: Option<crate::schemas::IosXcTest>,
        #[doc = "Test setup requirements for Android e.g. files to install, bootstrap\nscripts."]
        #[serde(rename = "testSetup", default)]
        pub test_setup: Option<crate::schemas::TestSetup>,
        #[doc = "Max time a test execution is allowed to run before it is\nautomatically cancelled.\nThe default value is 5 min."]
        #[serde(rename = "testTimeout", default)]
        pub test_timeout: Option<String>,
    }
    impl ::field_selector::FieldSelector for TestSpecification {
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
    pub struct ToolResultsExecution {
        #[doc = "Output only. A tool results execution ID."]
        #[serde(rename = "executionId", default)]
        pub execution_id: Option<String>,
        #[doc = "Output only. A tool results history ID."]
        #[serde(rename = "historyId", default)]
        pub history_id: Option<String>,
        #[doc = "Output only. The cloud project that owns the tool results execution."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ToolResultsExecution {
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
    pub struct ToolResultsHistory {
        #[doc = "Required. A tool results history ID."]
        #[serde(rename = "historyId", default)]
        pub history_id: Option<String>,
        #[doc = "Required. The cloud project that owns the tool results history."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ToolResultsHistory {
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
    pub struct ToolResultsStep {
        #[doc = "Output only. A tool results execution ID."]
        #[serde(rename = "executionId", default)]
        pub execution_id: Option<String>,
        #[doc = "Output only. A tool results history ID."]
        #[serde(rename = "historyId", default)]
        pub history_id: Option<String>,
        #[doc = "Output only. The cloud project that owns the tool results step."]
        #[serde(rename = "projectId", default)]
        pub project_id: Option<String>,
        #[doc = "Output only. A tool results step ID."]
        #[serde(rename = "stepId", default)]
        pub step_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ToolResultsStep {
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TrafficRule {
        #[doc = "Bandwidth in kbits/second."]
        #[serde(rename = "bandwidth", default)]
        pub bandwidth: Option<f32>,
        #[doc = "Burst size in kbits."]
        #[serde(rename = "burst", default)]
        pub burst: Option<f32>,
        #[doc = "Packet delay, must be >= 0."]
        #[serde(rename = "delay", default)]
        pub delay: Option<String>,
        #[doc = "Packet duplication ratio (0.0 - 1.0)."]
        #[serde(rename = "packetDuplicationRatio", default)]
        pub packet_duplication_ratio: Option<f32>,
        #[doc = "Packet loss ratio (0.0 - 1.0)."]
        #[serde(rename = "packetLossRatio", default)]
        pub packet_loss_ratio: Option<f32>,
    }
    impl ::field_selector::FieldSelector for TrafficRule {
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
    pub struct XcodeVersion {
        #[doc = "Tags for this Xcode version.\nExample: \"default\"."]
        #[serde(rename = "tags", default)]
        pub tags: Option<Vec<String>>,
        #[doc = "The id for this version.\nExample: \"9.2\"."]
        #[serde(rename = "version", default)]
        pub version: Option<String>,
    }
    impl ::field_selector::FieldSelector for XcodeVersion {
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
    #[doc = "Actions that can be performed on the application_detail_service resource"]
    pub fn application_detail_service(
        &self,
    ) -> crate::resources::application_detail_service::ApplicationDetailServiceActions<A> {
        crate::resources::application_detail_service::ApplicationDetailServiceActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions<A> {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the test_environment_catalog resource"]
    pub fn test_environment_catalog(
        &self,
    ) -> crate::resources::test_environment_catalog::TestEnvironmentCatalogActions<A> {
        crate::resources::test_environment_catalog::TestEnvironmentCatalogActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
mod resources {
    pub mod application_detail_service {
        pub mod params {}
        pub struct ApplicationDetailServiceActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ApplicationDetailServiceActions<'a, A> {
            #[doc = "Gets the details of an Android application APK."]
            pub fn get_apk_details(
                &self,
                request: crate::schemas::FileReference,
            ) -> GetApkDetailsRequestBuilder<A> {
                GetApkDetailsRequestBuilder {
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
        pub struct GetApkDetailsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::FileReference,
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
        impl<'a, A: yup_oauth2::GetToken> GetApkDetailsRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            ) -> Result<crate::schemas::GetApkDetailsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::GetApkDetailsResponse, Box<dyn ::std::error::Error>>
            {
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
                let mut output = "https://testing.googleapis.com/".to_owned();
                output.push_str("v1/applicationDetailService/getApkDetails");
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
    }
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
            #[doc = "Actions that can be performed on the test_matrices resource"]
            pub fn test_matrices(
                &self,
            ) -> crate::resources::projects::test_matrices::TestMatricesActions<A> {
                crate::resources::projects::test_matrices::TestMatricesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod test_matrices {
            pub mod params {}
            pub struct TestMatricesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> TestMatricesActions<'a, A> {
                #[doc = "Cancels unfinished test executions in a test matrix.\nThis call returns immediately and cancellation proceeds asychronously.\nIf the matrix is already final, this operation will have no effect.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read project\n* INVALID_ARGUMENT - if the request is malformed\n* NOT_FOUND - if the Test Matrix does not exist"]
                pub fn cancel(
                    &self,
                    project_id: impl Into<String>,
                    test_matrix_id: impl Into<String>,
                ) -> CancelRequestBuilder<A> {
                    CancelRequestBuilder {
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
                        project_id: project_id.into(),
                        test_matrix_id: test_matrix_id.into(),
                    }
                }
                #[doc = "Creates and runs a matrix of tests according to the given specifications.\nUnsupported environments will be returned in the state UNSUPPORTED.\nMatrices are limited to at most 200 supported executions.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to write to project\n* INVALID_ARGUMENT - if the request is malformed or if the matrix expands\n  to more than 200 supported executions"]
                pub fn create(
                    &self,
                    request: crate::schemas::TestMatrix,
                    project_id: impl Into<String>,
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
                        project_id: project_id.into(),
                        request_id: None,
                    }
                }
                #[doc = "Checks the status of a test matrix.\n\nMay return any of the following canonical error codes:\n\n* PERMISSION_DENIED - if the user is not authorized to read project\n* INVALID_ARGUMENT - if the request is malformed\n* NOT_FOUND - if the Test Matrix does not exist"]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    test_matrix_id: impl Into<String>,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
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
                        project_id: project_id.into(),
                        test_matrix_id: test_matrix_id.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                test_matrix_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> CancelRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(mut self, value: crate::params::Alt) -> Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                ) -> Result<crate::schemas::CancelTestMatrixResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::CancelTestMatrixResponse, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://testing.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/testMatrices/");
                    {
                        let var_as_str = &self.test_matrix_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str(":cancel");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::TestMatrix,
                project_id: String,
                request_id: Option<String>,
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
                #[doc = "A string id used to detect duplicated requests.\nIds are automatically scoped to a project, so\nusers should ensure the ID is unique per-project.\nA UUID is recommended.\n\nOptional, but strongly recommended."]
                pub fn request_id(mut self, value: impl Into<String>) -> Self {
                    self.request_id = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(mut self, value: crate::params::Alt) -> Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                ) -> Result<crate::schemas::TestMatrix, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::TestMatrix, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://testing.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/testMatrices");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("requestId", &self.request_id)]);
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                test_matrix_id: String,
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
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "Data format for response."]
                pub fn alt(mut self, value: crate::params::Alt) -> Self {
                    self.alt = Some(value);
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                ) -> Result<crate::schemas::TestMatrix, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::TestMatrix, Box<dyn ::std::error::Error>>
                {
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
                    let mut output = "https://testing.googleapis.com/".to_owned();
                    output.push_str("v1/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/testMatrices/");
                    {
                        let var_as_str = &self.test_matrix_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
        }
    }
    pub mod test_environment_catalog {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetEnvironmentType {
                EnvironmentTypeUnspecified,
                Android,
                Ios,
                NetworkConfiguration,
                ProvidedSoftware,
            }
            impl GetEnvironmentType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetEnvironmentType::EnvironmentTypeUnspecified => {
                            "ENVIRONMENT_TYPE_UNSPECIFIED"
                        }
                        GetEnvironmentType::Android => "ANDROID",
                        GetEnvironmentType::Ios => "IOS",
                        GetEnvironmentType::NetworkConfiguration => "NETWORK_CONFIGURATION",
                        GetEnvironmentType::ProvidedSoftware => "PROVIDED_SOFTWARE",
                    }
                }
            }
            impl ::std::fmt::Display for GetEnvironmentType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetEnvironmentType {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetEnvironmentType {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ENVIRONMENT_TYPE_UNSPECIFIED" => {
                            GetEnvironmentType::EnvironmentTypeUnspecified
                        }
                        "ANDROID" => GetEnvironmentType::Android,
                        "IOS" => GetEnvironmentType::Ios,
                        "NETWORK_CONFIGURATION" => GetEnvironmentType::NetworkConfiguration,
                        "PROVIDED_SOFTWARE" => GetEnvironmentType::ProvidedSoftware,
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
        pub struct TestEnvironmentCatalogActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> TestEnvironmentCatalogActions<'a, A> {
            #[doc = "Gets the catalog of supported test environments.\n\nMay return any of the following canonical error codes:\n\n* INVALID_ARGUMENT - if the request is malformed\n* NOT_FOUND - if the environment type does not exist\n* INTERNAL - if an internal error occurred"]
            pub fn get(
                &self,
                environment_type : crate :: resources :: test_environment_catalog :: params :: GetEnvironmentType,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
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
                    environment_type,
                    project_id: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            environment_type:
                crate::resources::test_environment_catalog::params::GetEnvironmentType,
            project_id: Option<String>,
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "For authorization, the cloud project requesting the TestEnvironmentCatalog."]
            pub fn project_id(mut self, value: impl Into<String>) -> Self {
                self.project_id = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(mut self, value: crate::params::Alt) -> Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            ) -> Result<crate::schemas::TestEnvironmentCatalog, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::TestEnvironmentCatalog, Box<dyn ::std::error::Error>>
            {
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
                let mut output = "https://testing.googleapis.com/".to_owned();
                output.push_str("v1/testEnvironmentCatalog/");
                {
                    let var_as_string = self.environment_type.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("projectId", &self.project_id)]);
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
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
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
