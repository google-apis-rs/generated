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
        #[serde(rename = "creation_time", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time: Option<i64>,
        #[doc = "Unique identifier of this account."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#account."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Name of this account."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Whether this account is premium."]
        #[serde(rename = "premium", default)]
        pub premium: Option<bool>,
        #[doc = "Sub accounts of the this account."]
        #[serde(rename = "subAccounts", default)]
        pub sub_accounts: Option<Vec<crate::schemas::Account>>,
        #[doc = "AdSense timezone of this account."]
        #[serde(rename = "timezone", default)]
        pub timezone: Option<String>,
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
    pub struct Accounts {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The accounts returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Account>>,
        #[doc = "Kind of list this is, in this case adsense#accounts."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through accounts. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for Accounts {
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
    pub struct AdClient {
        #[doc = "Whether this ad client is opted in to ARC."]
        #[serde(rename = "arcOptIn", default)]
        pub arc_opt_in: Option<bool>,
        #[doc = "Unique identifier of this ad client."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#adClient."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "This ad client's product code, which corresponds to the PRODUCT_CODE report dimension."]
        #[serde(rename = "productCode", default)]
        pub product_code: Option<String>,
        #[doc = "Whether this ad client supports being reported on."]
        #[serde(rename = "supportsReporting", default)]
        pub supports_reporting: Option<bool>,
    }
    impl ::field_selector::FieldSelector for AdClient {
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
    pub struct AdClients {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The ad clients returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::AdClient>>,
        #[doc = "Kind of list this is, in this case adsense#adClients."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through ad clients. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdClients {
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
    pub struct AdCode {
        #[doc = "The Auto ad code snippet. The ad code snippet."]
        #[serde(rename = "adCode", default)]
        pub ad_code: Option<String>,
        #[doc = "The AMP Auto ad code snippet that goes in the body of an AMP page."]
        #[serde(rename = "ampBody", default)]
        pub amp_body: Option<String>,
        #[doc = "The AMP Auto ad code snippet that goes in the head of an AMP page."]
        #[serde(rename = "ampHead", default)]
        pub amp_head: Option<String>,
        #[doc = "Kind this is, in this case adsense#adCode."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdCode {
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
    pub struct AdStyleColors {
        #[doc = "The color of the ad background."]
        #[serde(rename = "background", default)]
        pub background: Option<String>,
        #[doc = "The color of the ad border."]
        #[serde(rename = "border", default)]
        pub border: Option<String>,
        #[doc = "The color of the ad text."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
        #[doc = "The color of the ad title."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[doc = "The color of the ad url."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdStyleColors {
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
    pub struct AdStyleFont {
        #[doc = "The family of the font."]
        #[serde(rename = "family", default)]
        pub family: Option<String>,
        #[doc = "The size of the font."]
        #[serde(rename = "size", default)]
        pub size: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdStyleFont {
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
    pub struct AdStyle {
        #[doc = "The colors which are included in the style. These are represented as six hexadecimal characters, similar to HTML color codes, but without the leading hash."]
        #[serde(rename = "colors", default)]
        pub colors: Option<crate::schemas::AdStyleColors>,
        #[doc = "The style of the corners in the ad (deprecated: never populated, ignored)."]
        #[serde(rename = "corners", default)]
        pub corners: Option<String>,
        #[doc = "The font which is included in the style."]
        #[serde(rename = "font", default)]
        pub font: Option<crate::schemas::AdStyleFont>,
        #[doc = "Kind this is, in this case adsense#adStyle."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdStyle {
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
    pub struct AdUnitContentAdsSettingsBackupOption {
        #[doc = "Color to use when type is set to COLOR."]
        #[serde(rename = "color", default)]
        pub color: Option<String>,
        #[doc = "Type of the backup option. Possible values are BLANK, COLOR and URL."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "URL to use when type is set to URL."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdUnitContentAdsSettingsBackupOption {
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
    pub struct AdUnitContentAdsSettings {
        #[doc = "The backup option to be used in instances where no ad is available."]
        #[serde(rename = "backupOption", default)]
        pub backup_option: Option<crate::schemas::AdUnitContentAdsSettingsBackupOption>,
        #[doc = "Type of this ad unit."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "Size of this ad unit."]
        #[serde(rename = "size", default)]
        pub size: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdUnitContentAdsSettings {
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
    pub struct AdUnitFeedAdsSettings {
        #[doc = "The position of the ads relative to the feed entries."]
        #[serde(rename = "adPosition", default)]
        pub ad_position: Option<String>,
        #[doc = "The frequency at which ads should appear in the feed (i.e. every N entries)."]
        #[serde(rename = "frequency", default)]
        pub frequency: Option<i32>,
        #[doc = "The minimum length an entry should be in order to have attached ads."]
        #[serde(rename = "minimumWordCount", default)]
        pub minimum_word_count: Option<i32>,
        #[doc = "The type of ads which should appear."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdUnitFeedAdsSettings {
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
    pub struct AdUnitMobileContentAdsSettings {
        #[doc = "The markup language to use for this ad unit."]
        #[serde(rename = "markupLanguage", default)]
        pub markup_language: Option<String>,
        #[doc = "Type of this ad unit."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The scripting language to use for this ad unit."]
        #[serde(rename = "scriptingLanguage", default)]
        pub scripting_language: Option<String>,
        #[doc = "Size of this ad unit."]
        #[serde(rename = "size", default)]
        pub size: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdUnitMobileContentAdsSettings {
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
    pub struct AdUnit {
        #[doc = "Identity code of this ad unit, not necessarily unique across ad clients."]
        #[serde(rename = "code", default)]
        pub code: Option<String>,
        #[doc = "Settings specific to content ads (AFC) and highend mobile content ads (AFMC - deprecated)."]
        #[serde(rename = "contentAdsSettings", default)]
        pub content_ads_settings: Option<crate::schemas::AdUnitContentAdsSettings>,
        #[doc = "Custom style information specific to this ad unit."]
        #[serde(rename = "customStyle", default)]
        pub custom_style: Option<crate::schemas::AdStyle>,
        #[doc = "Settings specific to feed ads (AFF) - deprecated."]
        #[serde(rename = "feedAdsSettings", default)]
        pub feed_ads_settings: Option<crate::schemas::AdUnitFeedAdsSettings>,
        #[doc = "Unique identifier of this ad unit. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#adUnit."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Settings specific to WAP mobile content ads (AFMC) - deprecated."]
        #[serde(rename = "mobileContentAdsSettings", default)]
        pub mobile_content_ads_settings: Option<crate::schemas::AdUnitMobileContentAdsSettings>,
        #[doc = "Name of this ad unit."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "ID of the saved ad style which holds this ad unit's style information."]
        #[serde(rename = "savedStyleId", default)]
        pub saved_style_id: Option<String>,
        #[doc = "Status of this ad unit. Possible values are:\nNEW: Indicates that the ad unit was created within the last seven days and does not yet have any activity associated with it.\n\nACTIVE: Indicates that there has been activity on this ad unit in the last seven days.\n\nINACTIVE: Indicates that there has been no activity on this ad unit in the last seven days."]
        #[serde(rename = "status", default)]
        pub status: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdUnit {
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
    pub struct AdUnits {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The ad units returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::AdUnit>>,
        #[doc = "Kind of list this is, in this case adsense#adUnits."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through ad units. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdUnits {
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
    pub struct AdsenseReportsGenerateResponseHeadersItems {
        #[doc = "The currency of this column. Only present if the header type is METRIC_CURRENCY."]
        #[serde(rename = "currency", default)]
        pub currency: Option<String>,
        #[doc = "The name of the header."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The type of the header; one of DIMENSION, METRIC_TALLY, METRIC_RATIO, or METRIC_CURRENCY."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
    }
    impl ::field_selector::FieldSelector for AdsenseReportsGenerateResponseHeadersItems {
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
    pub struct AdsenseReportsGenerateResponse {
        #[doc = "The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
        #[serde(rename = "averages", default)]
        pub averages: Option<Vec<String>>,
        #[doc = "The requested end date in yyyy-mm-dd format."]
        #[serde(rename = "endDate", default)]
        pub end_date: Option<String>,
        #[doc = "The header information of the columns requested in the report. This is a list of headers; one for each dimension in the request, followed by one for each metric in the request."]
        #[serde(rename = "headers", default)]
        pub headers: Option<Vec<crate::schemas::AdsenseReportsGenerateResponseHeadersItems>>,
        #[doc = "Kind this is, in this case adsense#report."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request. The dimension cells contain strings, and the metric cells contain numbers."]
        #[serde(rename = "rows", default)]
        pub rows: Option<Vec<Vec<String>>>,
        #[doc = "The requested start date in yyyy-mm-dd format."]
        #[serde(rename = "startDate", default)]
        pub start_date: Option<String>,
        #[doc = "The total number of rows matched by the report request. Fewer rows may be returned in the response due to being limited by the row count requested or the report row limit."]
        #[serde(rename = "totalMatchedRows", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_matched_rows: Option<i64>,
        #[doc = "The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
        #[serde(rename = "totals", default)]
        pub totals: Option<Vec<String>>,
        #[doc = "Any warnings associated with generation of the report."]
        #[serde(rename = "warnings", default)]
        pub warnings: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for AdsenseReportsGenerateResponse {
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
    pub struct Alert {
        #[doc = "Unique identifier of this alert. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Whether this alert can be dismissed."]
        #[serde(rename = "isDismissible", default)]
        pub is_dismissible: Option<bool>,
        #[doc = "Kind of resource this is, in this case adsense#alert."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The localized alert message."]
        #[serde(rename = "message", default)]
        pub message: Option<String>,
        #[doc = "Type of this alert. Possible values: SELF_HOLD, MIGRATED_TO_BILLING3, ADDRESS_PIN_VERIFICATION, PHONE_PIN_VERIFICATION, CORPORATE_ENTITY, GRAYLISTED_PUBLISHER, API_HOLD."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "Severity of this alert. Possible values: INFO, WARNING, SEVERE."]
        #[serde(rename = "severity", default)]
        pub severity: Option<String>,
    }
    impl ::field_selector::FieldSelector for Alert {
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
    pub struct Alerts {
        #[doc = "The alerts returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Alert>>,
        #[doc = "Kind of list this is, in this case adsense#alerts."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for Alerts {
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
    pub struct CustomChannelTargetingInfo {
        #[doc = "The name used to describe this channel externally."]
        #[serde(rename = "adsAppearOn", default)]
        pub ads_appear_on: Option<String>,
        #[doc = "The external description of the channel."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The locations in which ads appear. (Only valid for content and mobile content ads (deprecated)). Acceptable values for content ads are: TOP_LEFT, TOP_CENTER, TOP_RIGHT, MIDDLE_LEFT, MIDDLE_CENTER, MIDDLE_RIGHT, BOTTOM_LEFT, BOTTOM_CENTER, BOTTOM_RIGHT, MULTIPLE_LOCATIONS. Acceptable values for mobile content ads (deprecated) are: TOP, MIDDLE, BOTTOM, MULTIPLE_LOCATIONS."]
        #[serde(rename = "location", default)]
        pub location: Option<String>,
        #[doc = "The language of the sites ads will be displayed on."]
        #[serde(rename = "siteLanguage", default)]
        pub site_language: Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomChannelTargetingInfo {
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
    pub struct CustomChannel {
        #[doc = "Code of this custom channel, not necessarily unique across ad clients."]
        #[serde(rename = "code", default)]
        pub code: Option<String>,
        #[doc = "Unique identifier of this custom channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#customChannel."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Name of this custom channel."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The targeting information of this custom channel, if activated."]
        #[serde(rename = "targetingInfo", default)]
        pub targeting_info: Option<crate::schemas::CustomChannelTargetingInfo>,
    }
    impl ::field_selector::FieldSelector for CustomChannel {
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
    pub struct CustomChannels {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The custom channels returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::CustomChannel>>,
        #[doc = "Kind of list this is, in this case adsense#customChannels."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through custom channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for CustomChannels {
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
    pub struct Metadata {
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::ReportingMetadataEntry>>,
        #[doc = "Kind of list this is, in this case adsense#metadata."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for Metadata {
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
    pub struct Payment {
        #[doc = "Unique identifier of this Payment."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#payment."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The amount to be paid."]
        #[serde(rename = "paymentAmount", default)]
        pub payment_amount: Option<String>,
        #[doc = "The currency code for the amount to be paid."]
        #[serde(rename = "paymentAmountCurrencyCode", default)]
        pub payment_amount_currency_code: Option<String>,
        #[doc = "The date this payment was/will be credited to the user, or none if the payment threshold has not been met."]
        #[serde(rename = "paymentDate", default)]
        pub payment_date: Option<String>,
    }
    impl ::field_selector::FieldSelector for Payment {
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
    pub struct Payments {
        #[doc = "The list of Payments for the account. One or both of a) the account's most recent payment; and b) the account's upcoming payment."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Payment>>,
        #[doc = "Kind of list this is, in this case adsense#payments."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for Payments {
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
    pub struct ReportingMetadataEntry {
        #[doc = "For metrics this is a list of dimension IDs which the metric is compatible with, for dimensions it is a list of compatibility groups the dimension belongs to."]
        #[serde(rename = "compatibleDimensions", default)]
        pub compatible_dimensions: Option<Vec<String>>,
        #[doc = "The names of the metrics the dimension or metric this reporting metadata entry describes is compatible with."]
        #[serde(rename = "compatibleMetrics", default)]
        pub compatible_metrics: Option<Vec<String>>,
        #[doc = "Unique identifier of this reporting metadata entry, corresponding to the name of the appropriate dimension or metric."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#reportingMetadataEntry."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The names of the dimensions which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted."]
        #[serde(rename = "requiredDimensions", default)]
        pub required_dimensions: Option<Vec<String>>,
        #[doc = "The names of the metrics which the dimension or metric this reporting metadata entry describes requires to also be present in order for the report to be valid. Omitting these will not cause an error or warning, but may result in data which cannot be correctly interpreted."]
        #[serde(rename = "requiredMetrics", default)]
        pub required_metrics: Option<Vec<String>>,
        #[doc = "The codes of the projects supported by the dimension or metric this reporting metadata entry describes."]
        #[serde(rename = "supportedProducts", default)]
        pub supported_products: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ReportingMetadataEntry {
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
    pub struct SavedAdStyle {
        #[doc = "The AdStyle itself."]
        #[serde(rename = "adStyle", default)]
        pub ad_style: Option<crate::schemas::AdStyle>,
        #[doc = "Unique identifier of this saved ad style. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#savedAdStyle."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The user selected name of this SavedAdStyle."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for SavedAdStyle {
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
    pub struct SavedAdStyles {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The saved ad styles returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::SavedAdStyle>>,
        #[doc = "Kind of list this is, in this case adsense#savedAdStyles."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through ad units. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for SavedAdStyles {
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
    pub struct SavedReport {
        #[doc = "Unique identifier of this saved report."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#savedReport."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "This saved report's name."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for SavedReport {
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
    pub struct SavedReports {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The saved reports returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::SavedReport>>,
        #[doc = "Kind of list this is, in this case adsense#savedReports."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through saved reports. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for SavedReports {
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
    pub struct UrlChannel {
        #[doc = "Unique identifier of this URL channel. This should be considered an opaque identifier; it is not safe to rely on it being in any particular format."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "Kind of resource this is, in this case adsense#urlChannel."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "URL Pattern of this URL channel. Does not include \"http://\" or \"https://\". Example: www.example.com/home"]
        #[serde(rename = "urlPattern", default)]
        pub url_pattern: Option<String>,
    }
    impl ::field_selector::FieldSelector for UrlChannel {
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
    pub struct UrlChannels {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: Option<String>,
        #[doc = "The URL channels returned in this list response."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::UrlChannel>>,
        #[doc = "Kind of list this is, in this case adsense#urlChannels."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through URL channels. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for UrlChannels {
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
        #[doc = "Responses with Content-Type of text/csv"]
        Csv,
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Upload/Download media content"]
        Media,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Csv => "csv",
                Alt::Json => "json",
                Alt::Media => "media",
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
                "csv" => Alt::Csv,
                "json" => Alt::Json,
                "media" => Alt::Media,
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
    #[doc = "Actions that can be performed on the adclients resource"]
    pub fn adclients(&self) -> crate::adclients::AdclientsActions<A> {
        crate::adclients::AdclientsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the adunits resource"]
    pub fn adunits(&self) -> crate::adunits::AdunitsActions<A> {
        crate::adunits::AdunitsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the alerts resource"]
    pub fn alerts(&self) -> crate::alerts::AlertsActions<A> {
        crate::alerts::AlertsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the customchannels resource"]
    pub fn customchannels(&self) -> crate::customchannels::CustomchannelsActions<A> {
        crate::customchannels::CustomchannelsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the metadata resource"]
    pub fn metadata(&self) -> crate::metadata::MetadataActions<A> {
        crate::metadata::MetadataActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the payments resource"]
    pub fn payments(&self) -> crate::payments::PaymentsActions<A> {
        crate::payments::PaymentsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the reports resource"]
    pub fn reports(&self) -> crate::reports::ReportsActions<A> {
        crate::reports::ReportsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the savedadstyles resource"]
    pub fn savedadstyles(&self) -> crate::savedadstyles::SavedadstylesActions<A> {
        crate::savedadstyles::SavedadstylesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the urlchannels resource"]
    pub fn urlchannels(&self) -> crate::urlchannels::UrlchannelsActions<A> {
        crate::urlchannels::UrlchannelsActions {
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
        #[doc = "Get information about the selected AdSense account."]
        pub fn get(&self, account_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                account_id: account_id.into(),
                tree: None,
            }
        }
        #[doc = "List all accounts available to this AdSense account."]
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
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Actions that can be performed on the adclients resource"]
        pub fn adclients(&self) -> adclients::AdclientsActions<A> {
            adclients::AdclientsActions
        }
        #[doc = "Actions that can be performed on the adunits resource"]
        pub fn adunits(&self) -> adunits::AdunitsActions<A> {
            adunits::AdunitsActions
        }
        #[doc = "Actions that can be performed on the alerts resource"]
        pub fn alerts(&self) -> alerts::AlertsActions<A> {
            alerts::AlertsActions
        }
        #[doc = "Actions that can be performed on the customchannels resource"]
        pub fn customchannels(&self) -> customchannels::CustomchannelsActions<A> {
            customchannels::CustomchannelsActions
        }
        #[doc = "Actions that can be performed on the payments resource"]
        pub fn payments(&self) -> payments::PaymentsActions<A> {
            payments::PaymentsActions
        }
        #[doc = "Actions that can be performed on the reports resource"]
        pub fn reports(&self) -> reports::ReportsActions<A> {
            reports::ReportsActions
        }
        #[doc = "Actions that can be performed on the savedadstyles resource"]
        pub fn savedadstyles(&self) -> savedadstyles::SavedadstylesActions<A> {
            savedadstyles::SavedadstylesActions
        }
        #[doc = "Actions that can be performed on the urlchannels resource"]
        pub fn urlchannels(&self) -> urlchannels::UrlchannelsActions<A> {
            urlchannels::UrlchannelsActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: String,
        tree: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "Whether the tree of sub accounts should be returned."]
        pub fn tree(&mut self, value: bool) -> &mut Self {
            self.tree = Some(value);
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("accounts/");
            output.push_str(&self.account_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("tree", &self.tree)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
        max_results: Option<i32>,
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
        #[doc = "The maximum number of accounts to include in the response, used for paging."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through accounts. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
        pub fn iter_items<T>(
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
                        #[serde(rename = "items")]
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
        ) -> Result<crate::schemas::Accounts, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("accounts");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
    pub mod adclients {
        pub mod params {}
        pub struct AdclientsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AdclientsActions<'a, A> {
            #[doc = "Get Auto ad code for a given ad client."]
            pub fn get_ad_code(
                &self,
                account_id: impl Into<String>,
                ad_client_id: impl Into<String>,
            ) -> GetAdCodeRequestBuilder<A> {
                GetAdCodeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    account_id: account_id.into(),
                    ad_client_id: ad_client_id.into(),
                }
            }
            #[doc = "List all ad clients in the specified account."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetAdCodeRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            ad_client_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetAdCodeRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::AdCode, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/adcode");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
            account_id: String,
            max_results: Option<i32>,
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
            #[doc = "The maximum number of ad clients to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through ad clients. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::AdClients, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
    pub mod adunits {
        pub mod params {}
        pub struct AdunitsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AdunitsActions<'a, A> {
            #[doc = "Gets the specified ad unit in the specified ad client for the specified account."]
            pub fn get(
                &self,
                account_id: impl Into<String>,
                ad_client_id: impl Into<String>,
                ad_unit_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    ad_client_id: ad_client_id.into(),
                    ad_unit_id: ad_unit_id.into(),
                }
            }
            #[doc = "Get ad code for the specified ad unit."]
            pub fn get_ad_code(
                &self,
                account_id: impl Into<String>,
                ad_client_id: impl Into<String>,
                ad_unit_id: impl Into<String>,
            ) -> GetAdCodeRequestBuilder<A> {
                GetAdCodeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    account_id: account_id.into(),
                    ad_client_id: ad_client_id.into(),
                    ad_unit_id: ad_unit_id.into(),
                }
            }
            #[doc = "List all ad units in the specified ad client for the specified account."]
            pub fn list(
                &self,
                account_id: impl Into<String>,
                ad_client_id: impl Into<String>,
            ) -> ListRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    ad_client_id: ad_client_id.into(),
                    include_inactive: None,
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Actions that can be performed on the customchannels resource"]
            pub fn customchannels(&self) -> customchannels::CustomchannelsActions<A> {
                customchannels::CustomchannelsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            ad_client_id: String,
            ad_unit_id: String,
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
            ) -> Result<crate::schemas::AdUnit, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/adunits/");
                output.push_str(&self.ad_unit_id);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetAdCodeRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            ad_client_id: String,
            ad_unit_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetAdCodeRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::AdCode, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/adunits/");
                output.push_str(&self.ad_unit_id);
                output.push_str("/adcode");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
            account_id: String,
            ad_client_id: String,
            include_inactive: Option<bool>,
            max_results: Option<i32>,
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
            #[doc = "Whether to include inactive ad units. Default: true."]
            pub fn include_inactive(&mut self, value: bool) -> &mut Self {
                self.include_inactive = Some(value);
                self
            }
            #[doc = "The maximum number of ad units to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::AdUnits, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/adunits");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("includeInactive", &self.include_inactive)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
        pub mod customchannels {
            pub mod params {}
            pub struct CustomchannelsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> CustomchannelsActions<'a, A> {
                #[doc = "List all custom channels which the specified ad unit belongs to."]
                pub fn list(
                    &self,
                    account_id: impl Into<String>,
                    ad_client_id: impl Into<String>,
                    ad_unit_id: impl Into<String>,
                ) -> ListRequestBuilder<A> {
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
                        account_id: account_id.into(),
                        ad_client_id: ad_client_id.into(),
                        ad_unit_id: ad_unit_id.into(),
                        max_results: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: String,
                ad_client_id: String,
                ad_unit_id: String,
                max_results: Option<i32>,
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
                #[doc = "The maximum number of custom channels to include in the response, used for paging."]
                pub fn max_results(&mut self, value: i32) -> &mut Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
                pub fn iter_items<T>(
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
                                #[serde(rename = "items")]
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
                ) -> Result<crate::schemas::CustomChannels, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                    output.push_str("accounts/");
                    output.push_str(&self.account_id);
                    output.push_str("/adclients/");
                    output.push_str(&self.ad_client_id);
                    output.push_str("/adunits/");
                    output.push_str(&self.ad_unit_id);
                    output.push_str("/customchannels");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("maxResults", &self.max_results)]);
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
                            "https://www.googleapis.com/auth/adsense.readonly",
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
    }
    pub mod alerts {
        pub mod params {}
        pub struct AlertsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AlertsActions<'a, A> {
            #[doc = "Dismiss (delete) the specified alert from the specified publisher AdSense account."]
            pub fn delete(
                &self,
                account_id: impl Into<String>,
                alert_id: impl Into<String>,
            ) -> DeleteRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    alert_id: alert_id.into(),
                }
            }
            #[doc = "List the alerts for the specified AdSense account."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    locale: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            alert_id: String,
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/alerts/");
                output.push_str(&self.alert_id);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense"])
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
            account_id: String,
            locale: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "The locale to use for translating alert messages. The account locale will be used if this is not supplied. The AdSense default (English) will be used if the supplied locale is invalid or unsupported."]
            pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
                self.locale = Some(value.into());
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
            ) -> Result<crate::schemas::Alerts, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/alerts");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod customchannels {
        pub mod params {}
        pub struct CustomchannelsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> CustomchannelsActions<'a, A> {
            #[doc = "Get the specified custom channel from the specified ad client for the specified account."]
            pub fn get(
                &self,
                account_id: impl Into<String>,
                ad_client_id: impl Into<String>,
                custom_channel_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    ad_client_id: ad_client_id.into(),
                    custom_channel_id: custom_channel_id.into(),
                }
            }
            #[doc = "List all custom channels in the specified ad client for the specified account."]
            pub fn list(
                &self,
                account_id: impl Into<String>,
                ad_client_id: impl Into<String>,
            ) -> ListRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    ad_client_id: ad_client_id.into(),
                    max_results: None,
                    page_token: None,
                }
            }
            #[doc = "Actions that can be performed on the adunits resource"]
            pub fn adunits(&self) -> adunits::AdunitsActions<A> {
                adunits::AdunitsActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            ad_client_id: String,
            custom_channel_id: String,
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
            ) -> Result<crate::schemas::CustomChannel, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/customchannels/");
                output.push_str(&self.custom_channel_id);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
            account_id: String,
            ad_client_id: String,
            max_results: Option<i32>,
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
            #[doc = "The maximum number of custom channels to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::CustomChannels, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/customchannels");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
        pub mod adunits {
            pub mod params {}
            pub struct AdunitsActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> AdunitsActions<'a, A> {
                #[doc = "List all ad units in the specified custom channel."]
                pub fn list(
                    &self,
                    account_id: impl Into<String>,
                    ad_client_id: impl Into<String>,
                    custom_channel_id: impl Into<String>,
                ) -> ListRequestBuilder<A> {
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
                        account_id: account_id.into(),
                        ad_client_id: ad_client_id.into(),
                        custom_channel_id: custom_channel_id.into(),
                        include_inactive: None,
                        max_results: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: String,
                ad_client_id: String,
                custom_channel_id: String,
                include_inactive: Option<bool>,
                max_results: Option<i32>,
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
                #[doc = "Whether to include inactive ad units. Default: true."]
                pub fn include_inactive(&mut self, value: bool) -> &mut Self {
                    self.include_inactive = Some(value);
                    self
                }
                #[doc = "The maximum number of ad units to include in the response, used for paging."]
                pub fn max_results(&mut self, value: i32) -> &mut Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
                pub fn iter_items<T>(
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
                                #[serde(rename = "items")]
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
                ) -> Result<crate::schemas::AdUnits, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                    output.push_str("accounts/");
                    output.push_str(&self.account_id);
                    output.push_str("/adclients/");
                    output.push_str(&self.ad_client_id);
                    output.push_str("/customchannels/");
                    output.push_str(&self.custom_channel_id);
                    output.push_str("/adunits");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("includeInactive", &self.include_inactive)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
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
                            "https://www.googleapis.com/auth/adsense.readonly",
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
    }
    pub mod payments {
        pub mod params {}
        pub struct PaymentsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PaymentsActions<'a, A> {
            #[doc = "List the payments for the specified AdSense account."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                    account_id: account_id.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Payments, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/payments");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod reports {
        pub mod params {}
        pub struct ReportsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ReportsActions<'a, A> {
            #[doc = "Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify \"alt=csv\" as a query parameter."]
            pub fn generate(
                &self,
                account_id: impl Into<String>,
                start_date: impl Into<String>,
                end_date: impl Into<String>,
            ) -> GenerateRequestBuilder<A> {
                GenerateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    account_id: account_id.into(),
                    start_date: start_date.into(),
                    end_date: end_date.into(),
                    currency: None,
                    dimension: None,
                    filter: None,
                    locale: None,
                    max_results: None,
                    metric: None,
                    sort: None,
                    start_index: None,
                    use_timezone_reporting: None,
                }
            }
            #[doc = "Actions that can be performed on the saved resource"]
            pub fn saved(&self) -> saved::SavedActions<A> {
                saved::SavedActions
            }
        }
        #[derive(Debug, Clone)]
        pub struct GenerateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            start_date: String,
            end_date: String,
            currency: Option<String>,
            dimension: Option<String>,
            filter: Option<String>,
            locale: Option<String>,
            max_results: Option<i32>,
            metric: Option<String>,
            sort: Option<String>,
            start_index: Option<i32>,
            use_timezone_reporting: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GenerateRequestBuilder<'a, A> {
            #[doc = "Optional currency to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
            pub fn currency(&mut self, value: impl Into<String>) -> &mut Self {
                self.currency = Some(value.into());
                self
            }
            #[doc = "Dimensions to base the report on."]
            pub fn dimension(&mut self, value: impl Into<String>) -> &mut Self {
                self.dimension = Some(value.into());
                self
            }
            #[doc = "Filters to be run on the report."]
            pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
            pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "The maximum number of rows of report data to return."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Numeric columns to include in the report."]
            pub fn metric(&mut self, value: impl Into<String>) -> &mut Self {
                self.metric = Some(value.into());
                self
            }
            #[doc = "The name of a dimension or metric to sort the resulting report on, optionally prefixed with \"+\" to sort ascending or \"-\" to sort descending. If no prefix is specified, the column is sorted ascending."]
            pub fn sort(&mut self, value: impl Into<String>) -> &mut Self {
                self.sort = Some(value.into());
                self
            }
            #[doc = "Index of the first row of report data to return."]
            pub fn start_index(&mut self, value: i32) -> &mut Self {
                self.start_index = Some(value);
                self
            }
            #[doc = "Whether the report should be generated in the AdSense account's local timezone. If false default PST/PDT timezone will be used."]
            pub fn use_timezone_reporting(&mut self, value: bool) -> &mut Self {
                self.use_timezone_reporting = Some(value);
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
            fn _download_path(&self) -> String {
                let mut output = "https://www.googleapis.com/adsense/v1.4/download/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/reports");
                output
            }
            pub fn download<W>(
                mut self,
                output: &mut W,
            ) -> Result<u64, Box<dyn ::std::error::Error>>
            where
                W: ::std::io::Write + ?Sized,
            {
                self.alt(crate::params::Alt::Media);
                Ok(self
                    ._request(&self._path())
                    .send()?
                    .error_for_status()?
                    .copy_to(output)?)
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
            ) -> Result<crate::schemas::AdsenseReportsGenerateResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/reports");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("startDate", &self.start_date)]);
                let req = req.query(&[("endDate", &self.end_date)]);
                let req = req.query(&[("currency", &self.currency)]);
                let req = req.query(&[("dimension", &self.dimension)]);
                let req = req.query(&[("filter", &self.filter)]);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("metric", &self.metric)]);
                let req = req.query(&[("sort", &self.sort)]);
                let req = req.query(&[("startIndex", &self.start_index)]);
                let req = req.query(&[("useTimezoneReporting", &self.use_timezone_reporting)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        pub mod saved {
            pub mod params {}
            pub struct SavedActions<'a, A> {
                pub(super) reqwest: &'a reqwest::Client,
                pub(super) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> SavedActions<'a, A> {
                #[doc = "Generate an AdSense report based on the saved report ID sent in the query parameters."]
                pub fn generate(
                    &self,
                    account_id: impl Into<String>,
                    saved_report_id: impl Into<String>,
                ) -> GenerateRequestBuilder<A> {
                    GenerateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        alt: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        user_ip: None,
                        account_id: account_id.into(),
                        saved_report_id: saved_report_id.into(),
                        locale: None,
                        max_results: None,
                        start_index: None,
                    }
                }
                #[doc = "List all saved reports in the specified AdSense account."]
                pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                        account_id: account_id.into(),
                        max_results: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GenerateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                account_id: String,
                saved_report_id: String,
                locale: Option<String>,
                max_results: Option<i32>,
                start_index: Option<i32>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> GenerateRequestBuilder<'a, A> {
                #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
                pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = "The maximum number of rows of report data to return."]
                pub fn max_results(&mut self, value: i32) -> &mut Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "Index of the first row of report data to return."]
                pub fn start_index(&mut self, value: i32) -> &mut Self {
                    self.start_index = Some(value);
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
                    crate::schemas::AdsenseReportsGenerateResponse,
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
                    let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                    output.push_str("accounts/");
                    output.push_str(&self.account_id);
                    output.push_str("/reports/");
                    output.push_str(&self.saved_report_id);
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("maxResults", &self.max_results)]);
                    let req = req.query(&[("startIndex", &self.start_index)]);
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
                            "https://www.googleapis.com/auth/adsense.readonly",
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
                account_id: String,
                max_results: Option<i32>,
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
                #[doc = "The maximum number of saved reports to include in the response, used for paging."]
                pub fn max_results(&mut self, value: i32) -> &mut Self {
                    self.max_results = Some(value);
                    self
                }
                #[doc = "A continuation token, used to page through saved reports. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
                pub fn iter_items<T>(
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
                                #[serde(rename = "items")]
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
                ) -> Result<crate::schemas::SavedReports, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                    output.push_str("accounts/");
                    output.push_str(&self.account_id);
                    output.push_str("/reports/saved");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("maxResults", &self.max_results)]);
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
                            "https://www.googleapis.com/auth/adsense.readonly",
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
    }
    pub mod savedadstyles {
        pub mod params {}
        pub struct SavedadstylesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> SavedadstylesActions<'a, A> {
            #[doc = "List a specific saved ad style for the specified account."]
            pub fn get(
                &self,
                account_id: impl Into<String>,
                saved_ad_style_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    saved_ad_style_id: saved_ad_style_id.into(),
                }
            }
            #[doc = "List all saved ad styles in the specified account."]
            pub fn list(&self, account_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            saved_ad_style_id: String,
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
            ) -> Result<crate::schemas::SavedAdStyle, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/savedadstyles/");
                output.push_str(&self.saved_ad_style_id);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
            account_id: String,
            max_results: Option<i32>,
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
            #[doc = "The maximum number of saved ad styles to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through saved ad styles. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::SavedAdStyles, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/savedadstyles");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
    pub mod urlchannels {
        pub mod params {}
        pub struct UrlchannelsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UrlchannelsActions<'a, A> {
            #[doc = "List all URL channels in the specified ad client for the specified account."]
            pub fn list(
                &self,
                account_id: impl Into<String>,
                ad_client_id: impl Into<String>,
            ) -> ListRequestBuilder<A> {
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
                    account_id: account_id.into(),
                    ad_client_id: ad_client_id.into(),
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            account_id: String,
            ad_client_id: String,
            max_results: Option<i32>,
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
            #[doc = "The maximum number of URL channels to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through URL channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::UrlChannels, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("accounts/");
                output.push_str(&self.account_id);
                output.push_str("/adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/urlchannels");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
}
pub mod adclients {
    pub mod params {}
    pub struct AdclientsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> AdclientsActions<'a, A> {
        #[doc = "List all ad clients in this AdSense account."]
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
                max_results: None,
                page_token: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        max_results: Option<i32>,
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
        #[doc = "The maximum number of ad clients to include in the response, used for paging."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through ad clients. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
        pub fn iter_items<T>(
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
                        #[serde(rename = "items")]
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
        ) -> Result<crate::schemas::AdClients, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("adclients");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
pub mod adunits {
    pub mod params {}
    pub struct AdunitsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> AdunitsActions<'a, A> {
        #[doc = "Gets the specified ad unit in the specified ad client."]
        pub fn get(
            &self,
            ad_client_id: impl Into<String>,
            ad_unit_id: impl Into<String>,
        ) -> GetRequestBuilder<A> {
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
                ad_client_id: ad_client_id.into(),
                ad_unit_id: ad_unit_id.into(),
            }
        }
        #[doc = "Get ad code for the specified ad unit."]
        pub fn get_ad_code(
            &self,
            ad_client_id: impl Into<String>,
            ad_unit_id: impl Into<String>,
        ) -> GetAdCodeRequestBuilder<A> {
            GetAdCodeRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                ad_client_id: ad_client_id.into(),
                ad_unit_id: ad_unit_id.into(),
            }
        }
        #[doc = "List all ad units in the specified ad client for this AdSense account."]
        pub fn list(&self, ad_client_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                ad_client_id: ad_client_id.into(),
                include_inactive: None,
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Actions that can be performed on the customchannels resource"]
        pub fn customchannels(&self) -> customchannels::CustomchannelsActions<A> {
            customchannels::CustomchannelsActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        ad_client_id: String,
        ad_unit_id: String,
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
        pub fn execute_debug(self) -> Result<crate::schemas::AdUnit, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("adclients/");
            output.push_str(&self.ad_client_id);
            output.push_str("/adunits/");
            output.push_str(&self.ad_unit_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetAdCodeRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        ad_client_id: String,
        ad_unit_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GetAdCodeRequestBuilder<'a, A> {
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
        pub fn execute_debug(self) -> Result<crate::schemas::AdCode, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("adclients/");
            output.push_str(&self.ad_client_id);
            output.push_str("/adunits/");
            output.push_str(&self.ad_unit_id);
            output.push_str("/adcode");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
        ad_client_id: String,
        include_inactive: Option<bool>,
        max_results: Option<i32>,
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
        #[doc = "Whether to include inactive ad units. Default: true."]
        pub fn include_inactive(&mut self, value: bool) -> &mut Self {
            self.include_inactive = Some(value);
            self
        }
        #[doc = "The maximum number of ad units to include in the response, used for paging."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
        pub fn iter_items<T>(
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
                        #[serde(rename = "items")]
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
        ) -> Result<crate::schemas::AdUnits, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("adclients/");
            output.push_str(&self.ad_client_id);
            output.push_str("/adunits");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("includeInactive", &self.include_inactive)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
    pub mod customchannels {
        pub mod params {}
        pub struct CustomchannelsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> CustomchannelsActions<'a, A> {
            #[doc = "List all custom channels which the specified ad unit belongs to."]
            pub fn list(
                &self,
                ad_client_id: impl Into<String>,
                ad_unit_id: impl Into<String>,
            ) -> ListRequestBuilder<A> {
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
                    ad_client_id: ad_client_id.into(),
                    ad_unit_id: ad_unit_id.into(),
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            ad_client_id: String,
            ad_unit_id: String,
            max_results: Option<i32>,
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
            #[doc = "The maximum number of custom channels to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::CustomChannels, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/adunits/");
                output.push_str(&self.ad_unit_id);
                output.push_str("/customchannels");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
}
pub mod alerts {
    pub mod params {}
    pub struct AlertsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> AlertsActions<'a, A> {
        #[doc = "Dismiss (delete) the specified alert from the publisher's AdSense account."]
        pub fn delete(&self, alert_id: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                alert_id: alert_id.into(),
            }
        }
        #[doc = "List the alerts for this AdSense account."]
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
                locale: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        alert_id: String,
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("alerts/");
            output.push_str(&self.alert_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense"])
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
        locale: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "The locale to use for translating alert messages. The account locale will be used if this is not supplied. The AdSense default (English) will be used if the supplied locale is invalid or unsupported."]
        pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
            self.locale = Some(value.into());
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
        pub fn execute_debug(self) -> Result<crate::schemas::Alerts, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("alerts");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("locale", &self.locale)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod customchannels {
    pub mod params {}
    pub struct CustomchannelsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> CustomchannelsActions<'a, A> {
        #[doc = "Get the specified custom channel from the specified ad client."]
        pub fn get(
            &self,
            ad_client_id: impl Into<String>,
            custom_channel_id: impl Into<String>,
        ) -> GetRequestBuilder<A> {
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
                ad_client_id: ad_client_id.into(),
                custom_channel_id: custom_channel_id.into(),
            }
        }
        #[doc = "List all custom channels in the specified ad client for this AdSense account."]
        pub fn list(&self, ad_client_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                ad_client_id: ad_client_id.into(),
                max_results: None,
                page_token: None,
            }
        }
        #[doc = "Actions that can be performed on the adunits resource"]
        pub fn adunits(&self) -> adunits::AdunitsActions<A> {
            adunits::AdunitsActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        ad_client_id: String,
        custom_channel_id: String,
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
        ) -> Result<crate::schemas::CustomChannel, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("adclients/");
            output.push_str(&self.ad_client_id);
            output.push_str("/customchannels/");
            output.push_str(&self.custom_channel_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
        ad_client_id: String,
        max_results: Option<i32>,
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
        #[doc = "The maximum number of custom channels to include in the response, used for paging."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through custom channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
        pub fn iter_items<T>(
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
                        #[serde(rename = "items")]
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
        ) -> Result<crate::schemas::CustomChannels, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("adclients/");
            output.push_str(&self.ad_client_id);
            output.push_str("/customchannels");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
    pub mod adunits {
        pub mod params {}
        pub struct AdunitsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AdunitsActions<'a, A> {
            #[doc = "List all ad units in the specified custom channel."]
            pub fn list(
                &self,
                ad_client_id: impl Into<String>,
                custom_channel_id: impl Into<String>,
            ) -> ListRequestBuilder<A> {
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
                    ad_client_id: ad_client_id.into(),
                    custom_channel_id: custom_channel_id.into(),
                    include_inactive: None,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            ad_client_id: String,
            custom_channel_id: String,
            include_inactive: Option<bool>,
            max_results: Option<i32>,
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
            #[doc = "Whether to include inactive ad units. Default: true."]
            pub fn include_inactive(&mut self, value: bool) -> &mut Self {
                self.include_inactive = Some(value);
                self
            }
            #[doc = "The maximum number of ad units to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through ad units. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::AdUnits, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("adclients/");
                output.push_str(&self.ad_client_id);
                output.push_str("/customchannels/");
                output.push_str(&self.custom_channel_id);
                output.push_str("/adunits");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("includeInactive", &self.include_inactive)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
}
pub mod metadata {
    pub mod params {}
    pub struct MetadataActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> MetadataActions<'a, A> {
        #[doc = "Actions that can be performed on the dimensions resource"]
        pub fn dimensions(&self) -> dimensions::DimensionsActions<A> {
            dimensions::DimensionsActions
        }
        #[doc = "Actions that can be performed on the metrics resource"]
        pub fn metrics(&self) -> metrics::MetricsActions<A> {
            metrics::MetricsActions
        }
    }
    pub mod dimensions {
        pub mod params {}
        pub struct DimensionsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> DimensionsActions<'a, A> {
            #[doc = "List the metadata for the dimensions available to this AdSense account."]
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Metadata, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("metadata/dimensions");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
    pub mod metrics {
        pub mod params {}
        pub struct MetricsActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> MetricsActions<'a, A> {
            #[doc = "List the metadata for the metrics available to this AdSense account."]
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Metadata, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("metadata/metrics");
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
}
pub mod payments {
    pub mod params {}
    pub struct PaymentsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PaymentsActions<'a, A> {
        #[doc = "List the payments for this AdSense account."]
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
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::Payments, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("payments");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod reports {
    pub mod params {}
    pub struct ReportsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ReportsActions<'a, A> {
        #[doc = "Generate an AdSense report based on the report request sent in the query parameters. Returns the result as JSON; to retrieve output in CSV format specify \"alt=csv\" as a query parameter."]
        pub fn generate(
            &self,
            start_date: impl Into<String>,
            end_date: impl Into<String>,
        ) -> GenerateRequestBuilder<A> {
            GenerateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                start_date: start_date.into(),
                end_date: end_date.into(),
                account_id: None,
                currency: None,
                dimension: None,
                filter: None,
                locale: None,
                max_results: None,
                metric: None,
                sort: None,
                start_index: None,
                use_timezone_reporting: None,
            }
        }
        #[doc = "Actions that can be performed on the saved resource"]
        pub fn saved(&self) -> saved::SavedActions<A> {
            saved::SavedActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct GenerateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        start_date: String,
        end_date: String,
        account_id: Option<String>,
        currency: Option<String>,
        dimension: Option<String>,
        filter: Option<String>,
        locale: Option<String>,
        max_results: Option<i32>,
        metric: Option<String>,
        sort: Option<String>,
        start_index: Option<i32>,
        use_timezone_reporting: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> GenerateRequestBuilder<'a, A> {
        #[doc = "Accounts upon which to report."]
        pub fn account_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.account_id = Some(value.into());
            self
        }
        #[doc = "Optional currency to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
        pub fn currency(&mut self, value: impl Into<String>) -> &mut Self {
            self.currency = Some(value.into());
            self
        }
        #[doc = "Dimensions to base the report on."]
        pub fn dimension(&mut self, value: impl Into<String>) -> &mut Self {
            self.dimension = Some(value.into());
            self
        }
        #[doc = "Filters to be run on the report."]
        pub fn filter(&mut self, value: impl Into<String>) -> &mut Self {
            self.filter = Some(value.into());
            self
        }
        #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
        pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
            self.locale = Some(value.into());
            self
        }
        #[doc = "The maximum number of rows of report data to return."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "Numeric columns to include in the report."]
        pub fn metric(&mut self, value: impl Into<String>) -> &mut Self {
            self.metric = Some(value.into());
            self
        }
        #[doc = "The name of a dimension or metric to sort the resulting report on, optionally prefixed with \"+\" to sort ascending or \"-\" to sort descending. If no prefix is specified, the column is sorted ascending."]
        pub fn sort(&mut self, value: impl Into<String>) -> &mut Self {
            self.sort = Some(value.into());
            self
        }
        #[doc = "Index of the first row of report data to return."]
        pub fn start_index(&mut self, value: i32) -> &mut Self {
            self.start_index = Some(value);
            self
        }
        #[doc = "Whether the report should be generated in the AdSense account's local timezone. If false default PST/PDT timezone will be used."]
        pub fn use_timezone_reporting(&mut self, value: bool) -> &mut Self {
            self.use_timezone_reporting = Some(value);
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
        fn _download_path(&self) -> String {
            let mut output = "https://www.googleapis.com/adsense/v1.4/download/".to_owned();
            output.push_str("reports");
            output
        }
        pub fn download<W>(mut self, output: &mut W) -> Result<u64, Box<dyn ::std::error::Error>>
        where
            W: ::std::io::Write + ?Sized,
        {
            self.alt(crate::params::Alt::Media);
            Ok(self
                ._request(&self._path())
                .send()?
                .error_for_status()?
                .copy_to(output)?)
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
        ) -> Result<crate::schemas::AdsenseReportsGenerateResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("reports");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("startDate", &self.start_date)]);
            let req = req.query(&[("endDate", &self.end_date)]);
            let req = req.query(&[("accountId", &self.account_id)]);
            let req = req.query(&[("currency", &self.currency)]);
            let req = req.query(&[("dimension", &self.dimension)]);
            let req = req.query(&[("filter", &self.filter)]);
            let req = req.query(&[("locale", &self.locale)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("metric", &self.metric)]);
            let req = req.query(&[("sort", &self.sort)]);
            let req = req.query(&[("startIndex", &self.start_index)]);
            let req = req.query(&[("useTimezoneReporting", &self.use_timezone_reporting)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    pub mod saved {
        pub mod params {}
        pub struct SavedActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> SavedActions<'a, A> {
            #[doc = "Generate an AdSense report based on the saved report ID sent in the query parameters."]
            pub fn generate(
                &self,
                saved_report_id: impl Into<String>,
            ) -> GenerateRequestBuilder<A> {
                GenerateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    saved_report_id: saved_report_id.into(),
                    locale: None,
                    max_results: None,
                    start_index: None,
                }
            }
            #[doc = "List all saved reports in this AdSense account."]
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
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GenerateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            saved_report_id: String,
            locale: Option<String>,
            max_results: Option<i32>,
            start_index: Option<i32>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GenerateRequestBuilder<'a, A> {
            #[doc = "Optional locale to use for translating report output to a local language. Defaults to \"en_US\" if not specified."]
            pub fn locale(&mut self, value: impl Into<String>) -> &mut Self {
                self.locale = Some(value.into());
                self
            }
            #[doc = "The maximum number of rows of report data to return."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Index of the first row of report data to return."]
            pub fn start_index(&mut self, value: i32) -> &mut Self {
                self.start_index = Some(value);
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
            ) -> Result<crate::schemas::AdsenseReportsGenerateResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("reports/");
                output.push_str(&self.saved_report_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("locale", &self.locale)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("startIndex", &self.start_index)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
            max_results: Option<i32>,
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
            #[doc = "The maximum number of saved reports to include in the response, used for paging."]
            pub fn max_results(&mut self, value: i32) -> &mut Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through saved reports. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(
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
                            #[serde(rename = "items")]
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
            ) -> Result<crate::schemas::SavedReports, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
                output.push_str("reports/saved");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
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
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
}
pub mod savedadstyles {
    pub mod params {}
    pub struct SavedadstylesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> SavedadstylesActions<'a, A> {
        #[doc = "Get a specific saved ad style from the user's account."]
        pub fn get(&self, saved_ad_style_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                saved_ad_style_id: saved_ad_style_id.into(),
            }
        }
        #[doc = "List all saved ad styles in the user's account."]
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
                max_results: None,
                page_token: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        saved_ad_style_id: String,
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
        ) -> Result<crate::schemas::SavedAdStyle, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("savedadstyles/");
            output.push_str(&self.saved_ad_style_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
        max_results: Option<i32>,
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
        #[doc = "The maximum number of saved ad styles to include in the response, used for paging."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through saved ad styles. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
        pub fn iter_items<T>(
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
                        #[serde(rename = "items")]
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
        ) -> Result<crate::schemas::SavedAdStyles, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("savedadstyles");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
pub mod urlchannels {
    pub mod params {}
    pub struct UrlchannelsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> UrlchannelsActions<'a, A> {
        #[doc = "List all URL channels in the specified ad client for this AdSense account."]
        pub fn list(&self, ad_client_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                ad_client_id: ad_client_id.into(),
                max_results: None,
                page_token: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        ad_client_id: String,
        max_results: Option<i32>,
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
        #[doc = "The maximum number of URL channels to include in the response, used for paging."]
        pub fn max_results(&mut self, value: i32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through URL channels. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
        pub fn iter_items<T>(
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
                        #[serde(rename = "items")]
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
        ) -> Result<crate::schemas::UrlChannels, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adsense/v1.4/".to_owned();
            output.push_str("adclients/");
            output.push_str(&self.ad_client_id);
            output.push_str("/urlchannels");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("maxResults", &self.max_results)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adsense.readonly"])
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
