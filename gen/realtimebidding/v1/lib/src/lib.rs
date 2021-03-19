#![doc = "# Resources and Methods\n    * [bidders](resources/bidders/struct.BiddersActions.html)\n      * [*get*](resources/bidders/struct.GetRequestBuilder.html), [*list*](resources/bidders/struct.ListRequestBuilder.html)\n      * [creatives](resources/bidders/creatives/struct.CreativesActions.html)\n        * [*list*](resources/bidders/creatives/struct.ListRequestBuilder.html), [*watch*](resources/bidders/creatives/struct.WatchRequestBuilder.html)\n      * [endpoints](resources/bidders/endpoints/struct.EndpointsActions.html)\n        * [*get*](resources/bidders/endpoints/struct.GetRequestBuilder.html), [*list*](resources/bidders/endpoints/struct.ListRequestBuilder.html)\n      * [pretargeting_configs](resources/bidders/pretargeting_configs/struct.PretargetingConfigsActions.html)\n        * [*activate*](resources/bidders/pretargeting_configs/struct.ActivateRequestBuilder.html), [*addTargetedApps*](resources/bidders/pretargeting_configs/struct.AddTargetedAppsRequestBuilder.html), [*addTargetedPublishers*](resources/bidders/pretargeting_configs/struct.AddTargetedPublishersRequestBuilder.html), [*addTargetedSites*](resources/bidders/pretargeting_configs/struct.AddTargetedSitesRequestBuilder.html), [*create*](resources/bidders/pretargeting_configs/struct.CreateRequestBuilder.html), [*delete*](resources/bidders/pretargeting_configs/struct.DeleteRequestBuilder.html), [*get*](resources/bidders/pretargeting_configs/struct.GetRequestBuilder.html), [*list*](resources/bidders/pretargeting_configs/struct.ListRequestBuilder.html), [*patch*](resources/bidders/pretargeting_configs/struct.PatchRequestBuilder.html), [*removeTargetedApps*](resources/bidders/pretargeting_configs/struct.RemoveTargetedAppsRequestBuilder.html), [*removeTargetedPublishers*](resources/bidders/pretargeting_configs/struct.RemoveTargetedPublishersRequestBuilder.html), [*removeTargetedSites*](resources/bidders/pretargeting_configs/struct.RemoveTargetedSitesRequestBuilder.html), [*suspend*](resources/bidders/pretargeting_configs/struct.SuspendRequestBuilder.html)\n    * [buyers](resources/buyers/struct.BuyersActions.html)\n      * [*get*](resources/buyers/struct.GetRequestBuilder.html), [*getRemarketingTag*](resources/buyers/struct.GetRemarketingTagRequestBuilder.html), [*list*](resources/buyers/struct.ListRequestBuilder.html)\n      * [creatives](resources/buyers/creatives/struct.CreativesActions.html)\n        * [*create*](resources/buyers/creatives/struct.CreateRequestBuilder.html), [*get*](resources/buyers/creatives/struct.GetRequestBuilder.html), [*list*](resources/buyers/creatives/struct.ListRequestBuilder.html), [*patch*](resources/buyers/creatives/struct.PatchRequestBuilder.html)\n      * [user_lists](resources/buyers/user_lists/struct.UserListsActions.html)\n        * [*close*](resources/buyers/user_lists/struct.CloseRequestBuilder.html), [*create*](resources/buyers/user_lists/struct.CreateRequestBuilder.html), [*get*](resources/buyers/user_lists/struct.GetRequestBuilder.html), [*getRemarketingTag*](resources/buyers/user_lists/struct.GetRemarketingTagRequestBuilder.html), [*list*](resources/buyers/user_lists/struct.ListRequestBuilder.html), [*open*](resources/buyers/user_lists/struct.OpenRequestBuilder.html), [*update*](resources/buyers/user_lists/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, create, edit, and delete your Authorized Buyers and Open Bidding account entities\n\n`https://www.googleapis.com/auth/realtime-bidding`"]
    pub const REALTIME_BIDDING: &str = "https://www.googleapis.com/auth/realtime-bidding";
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ActivatePretargetingConfigRequest {}
    impl ::google_field_selector::FieldSelector for ActivatePretargetingConfigRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActivatePretargetingConfigRequest {
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
    pub struct AdTechnologyProviders {
        #[doc = "The detected IAB Global Vendor List (GVL) IDs for this creative. See the IAB Global Vendor List at https://vendorlist.consensu.org/v2/vendor-list.json for details about the vendors."]
        #[serde(
            rename = "detectedGvlIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_gvl_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "The detected [Google Ad Tech Providers (ATP)](https://support.google.com/admanager/answer/9012903) for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider."]
        #[serde(
            rename = "detectedProviderIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_provider_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "Domains of detected unidentified ad technology providers (if any). You must ensure that the creatives used in bids placed for inventory that will serve to EEA or UK users does not contain unidentified ad technology providers. Google reserves the right to filter non-compliant bids."]
        #[serde(
            rename = "unidentifiedProviderDomains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unidentified_provider_domains: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for AdTechnologyProviders {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdTechnologyProviders {
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
    pub struct AddTargetedAppsRequest {
        #[doc = "A list of app IDs to target in the pretargeting configuration. These values will be added to the list of targeted app IDs in PretargetingConfig.appTargeting.mobileAppTargeting.values."]
        #[serde(
            rename = "appIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The targeting mode that should be applied to the list of app IDs. If there are existing targeted app IDs, must be equal to the existing PretargetingConfig.appTargeting.mobileAppTargeting.targetingMode or a 400 bad request error will be returned."]
        #[serde(
            rename = "targetingMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub targeting_mode:
            ::std::option::Option<crate::schemas::AddTargetedAppsRequestTargetingMode>,
    }
    impl ::google_field_selector::FieldSelector for AddTargetedAppsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddTargetedAppsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AddTargetedAppsRequestTargetingMode {
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
    }
    impl AddTargetedAppsRequestTargetingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                AddTargetedAppsRequestTargetingMode::Exclusive => "EXCLUSIVE",
                AddTargetedAppsRequestTargetingMode::Inclusive => "INCLUSIVE",
                AddTargetedAppsRequestTargetingMode::TargetingModeUnspecified => {
                    "TARGETING_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for AddTargetedAppsRequestTargetingMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AddTargetedAppsRequestTargetingMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AddTargetedAppsRequestTargetingMode, ()> {
            Ok(match s {
                "EXCLUSIVE" => AddTargetedAppsRequestTargetingMode::Exclusive,
                "INCLUSIVE" => AddTargetedAppsRequestTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    AddTargetedAppsRequestTargetingMode::TargetingModeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AddTargetedAppsRequestTargetingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AddTargetedAppsRequestTargetingMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AddTargetedAppsRequestTargetingMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXCLUSIVE" => AddTargetedAppsRequestTargetingMode::Exclusive,
                "INCLUSIVE" => AddTargetedAppsRequestTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    AddTargetedAppsRequestTargetingMode::TargetingModeUnspecified
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
    impl ::google_field_selector::FieldSelector for AddTargetedAppsRequestTargetingMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddTargetedAppsRequestTargetingMode {
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
    pub struct AddTargetedPublishersRequest {
        #[doc = "A list of publisher IDs to target in the pretargeting configuration. These values will be added to the list of targeted publisher IDs in PretargetingConfig.publisherTargeting.values. Publishers are identified by their publisher ID from ads.txt / app-ads.txt. See https://iabtechlab.com/ads-txt/ and https://iabtechlab.com/app-ads-txt/ for more details."]
        #[serde(
            rename = "publisherIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The targeting mode that should be applied to the list of publisher IDs. If are existing publisher IDs, must be equal to the existing PretargetingConfig.publisherTargeting.targetingMode or a 400 bad request error will be returned."]
        #[serde(
            rename = "targetingMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub targeting_mode:
            ::std::option::Option<crate::schemas::AddTargetedPublishersRequestTargetingMode>,
    }
    impl ::google_field_selector::FieldSelector for AddTargetedPublishersRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddTargetedPublishersRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AddTargetedPublishersRequestTargetingMode {
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
    }
    impl AddTargetedPublishersRequestTargetingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                AddTargetedPublishersRequestTargetingMode::Exclusive => "EXCLUSIVE",
                AddTargetedPublishersRequestTargetingMode::Inclusive => "INCLUSIVE",
                AddTargetedPublishersRequestTargetingMode::TargetingModeUnspecified => {
                    "TARGETING_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for AddTargetedPublishersRequestTargetingMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AddTargetedPublishersRequestTargetingMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<AddTargetedPublishersRequestTargetingMode, ()> {
            Ok(match s {
                "EXCLUSIVE" => AddTargetedPublishersRequestTargetingMode::Exclusive,
                "INCLUSIVE" => AddTargetedPublishersRequestTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    AddTargetedPublishersRequestTargetingMode::TargetingModeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AddTargetedPublishersRequestTargetingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AddTargetedPublishersRequestTargetingMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AddTargetedPublishersRequestTargetingMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXCLUSIVE" => AddTargetedPublishersRequestTargetingMode::Exclusive,
                "INCLUSIVE" => AddTargetedPublishersRequestTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    AddTargetedPublishersRequestTargetingMode::TargetingModeUnspecified
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
    impl ::google_field_selector::FieldSelector for AddTargetedPublishersRequestTargetingMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddTargetedPublishersRequestTargetingMode {
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
    pub struct AddTargetedSitesRequest {
        #[doc = "A list of site URLs to target in the pretargeting configuration. These values will be added to the list of targeted URLs in PretargetingConfig.webTargeting.values."]
        #[serde(
            rename = "sites",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sites: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The targeting mode that should be applied to the list of site URLs. If there are existing targeted sites, must be equal to the existing PretargetingConfig.webTargeting.targetingMode or a 400 bad request error will be returned."]
        #[serde(
            rename = "targetingMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub targeting_mode:
            ::std::option::Option<crate::schemas::AddTargetedSitesRequestTargetingMode>,
    }
    impl ::google_field_selector::FieldSelector for AddTargetedSitesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddTargetedSitesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AddTargetedSitesRequestTargetingMode {
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
    }
    impl AddTargetedSitesRequestTargetingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                AddTargetedSitesRequestTargetingMode::Exclusive => "EXCLUSIVE",
                AddTargetedSitesRequestTargetingMode::Inclusive => "INCLUSIVE",
                AddTargetedSitesRequestTargetingMode::TargetingModeUnspecified => {
                    "TARGETING_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for AddTargetedSitesRequestTargetingMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AddTargetedSitesRequestTargetingMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AddTargetedSitesRequestTargetingMode, ()> {
            Ok(match s {
                "EXCLUSIVE" => AddTargetedSitesRequestTargetingMode::Exclusive,
                "INCLUSIVE" => AddTargetedSitesRequestTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    AddTargetedSitesRequestTargetingMode::TargetingModeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AddTargetedSitesRequestTargetingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AddTargetedSitesRequestTargetingMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AddTargetedSitesRequestTargetingMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXCLUSIVE" => AddTargetedSitesRequestTargetingMode::Exclusive,
                "INCLUSIVE" => AddTargetedSitesRequestTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    AddTargetedSitesRequestTargetingMode::TargetingModeUnspecified
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
    impl ::google_field_selector::FieldSelector for AddTargetedSitesRequestTargetingMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddTargetedSitesRequestTargetingMode {
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
    pub struct AdvertiserAndBrand {
        #[doc = "See https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt for the list of possible values. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "advertiserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub advertiser_id: ::std::option::Option<i64>,
        #[doc = "Advertiser name. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "advertiserName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advertiser_name: ::std::option::Option<String>,
        #[doc = "Detected brand ID or zero if no brand has been detected. See https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt for the list of possible values. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "brandId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub brand_id: ::std::option::Option<i64>,
        #[doc = "Brand name. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "brandName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brand_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AdvertiserAndBrand {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdvertiserAndBrand {
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
    pub struct AppTargeting {
        #[doc = "Lists of included and excluded mobile app categories as defined in https://developers.google.com/adwords/api/docs/appendix/mobileappcategories.csv."]
        #[serde(
            rename = "mobileAppCategoryTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_app_category_targeting:
            ::std::option::Option<crate::schemas::NumericTargetingDimension>,
        #[doc = "Targeted app IDs. App IDs can refer to those found in an app store or ones that are not published in an app store. A maximum of 30,000 app IDs can be targeted."]
        #[serde(
            rename = "mobileAppTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_app_targeting: ::std::option::Option<crate::schemas::StringTargetingDimension>,
    }
    impl ::google_field_selector::FieldSelector for AppTargeting {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppTargeting {
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
    pub struct Bidder {
        #[doc = "Output only. A flag to bypass pretargeting for private auctions and preferred deals. When true, bid requests from these nonguaranteed deals will always be sent. When false, bid requests will be subject to regular pretargeting configurations. Programmatic Guaranteed deals will always be sent to the bidder, regardless of the value for this flag. Auction packages are not impacted by this value and are subject to the regular pretargeting configurations."]
        #[serde(
            rename = "bypassNonguaranteedDealsPretargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bypass_nonguaranteed_deals_pretargeting: ::std::option::Option<bool>,
        #[doc = "Output only. The buyer's network ID used for cookie matching. This ID corresponds to the `google_nid` parameter in the URL used in cookie match requests. Refer to https://developers.google.com/authorized-buyers/rtb/cookie-guide for further information."]
        #[serde(
            rename = "cookieMatchingNetworkId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cookie_matching_network_id: ::std::option::Option<String>,
        #[doc = "Output only. The base URL used in cookie match requests. Refer to https://developers.google.com/authorized-buyers/rtb/cookie-guide for further information."]
        #[serde(
            rename = "cookieMatchingUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cookie_matching_url: ::std::option::Option<String>,
        #[doc = "Output only. The billing ID for the deals pretargeting config. This billing ID is sent on the bid request for guaranteed and nonguaranteed deals matched in pretargeting."]
        #[serde(
            rename = "dealsBillingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals_billing_id: ::std::option::Option<String>,
        #[doc = "Output only. Name of the bidder resource that must follow the pattern `bidders/{bidderAccountId}`, where `{bidderAccountId}` is the account ID of the bidder whose information is to be received. One can get their account ID on the Authorized Buyers or Open Bidding UI, or by contacting their Google account manager."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Bidder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Bidder {
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
    pub struct Buyer {
        #[doc = "Output only. The number of creatives that this buyer submitted via the API or bid with in the last 30 days. This is counted against the maximum number of active creatives."]
        #[serde(
            rename = "activeCreativeCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub active_creative_count: ::std::option::Option<i64>,
        #[doc = "Output only. The name of the bidder resource that is responsible for receiving bidding traffic for this account. The bidder name must follow the pattern `bidders/{bidderAccountId}`, where `{bidderAccountId}` is the account ID of the bidder receiving traffic for this buyer."]
        #[serde(
            rename = "bidder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bidder: ::std::option::Option<String>,
        #[doc = "Output only. A list of billing IDs associated with this account. These IDs appear on: 1. A bid request, to signal which buyers are eligible to bid on a given opportunity, and which pretargeting configurations were matched for each eligible buyer. 2. The bid response, to attribute a winning impression to a specific account for billing, reporting, policy and publisher block enforcement."]
        #[serde(
            rename = "billingIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. The diplay name associated with this buyer account, as visible to sellers."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. The maximum number of active creatives that this buyer can have."]
        #[serde(
            rename = "maximumActiveCreativeCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_active_creative_count: ::std::option::Option<i64>,
        #[doc = "Output only. Name of the buyer resource that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` is the account ID of the buyer account whose information is to be received. One can get their account ID on the Authorized Buyers or Open Bidding UI, or by contacting their Google account manager."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Buyer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Buyer {
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
    pub struct CloseUserListRequest {}
    impl ::google_field_selector::FieldSelector for CloseUserListRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloseUserListRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Creative {
        #[doc = "Output only. ID of the buyer account that this creative is owned by. Can be used to filter the response of the creatives.list method with equality and inequality check."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub account_id: ::std::option::Option<i64>,
        #[doc = "The link to AdChoices destination page. This is only supported for native ads."]
        #[serde(
            rename = "adChoicesDestinationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_choices_destination_url: ::std::option::Option<String>,
        #[doc = "The name of the company being advertised in the creative. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "advertiserName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advertiser_name: ::std::option::Option<String>,
        #[doc = "The agency ID for this creative."]
        #[serde(
            rename = "agencyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: ::std::option::Option<i64>,
        #[doc = "Output only. The last update timestamp of the creative via API."]
        #[serde(
            rename = "apiUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_update_time: ::std::option::Option<String>,
        #[doc = "Output only. The format of this creative. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "creativeFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_format: ::std::option::Option<crate::schemas::CreativeCreativeFormat>,
        #[doc = "Buyer-specific creative ID that references this creative in bid responses. This field is Ignored in update operations. Can be used to filter the response of the creatives.list method. The maximum length of the creative ID is 128 bytes."]
        #[serde(
            rename = "creativeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_id: ::std::option::Option<String>,
        #[doc = "Output only. Top level status and detected attributes of a creative (for example domain, language, advertiser, product category, etc.) that affect whether (status) and where (context) a creative will be allowed to serve."]
        #[serde(
            rename = "creativeServingDecision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_serving_decision:
            ::std::option::Option<crate::schemas::CreativeServingDecision>,
        #[doc = "Output only. IDs of all of the deals with which this creative has been used in bidding. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "dealIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deal_ids: ::std::option::Option<Vec<String>>,
        #[doc = "All declared attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto\") contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction."]
        #[serde(
            rename = "declaredAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub declared_attributes:
            ::std::option::Option<Vec<crate::schemas::CreativeDeclaredAttributesItems>>,
        #[doc = "The set of declared destination URLs for the creative. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "declaredClickThroughUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub declared_click_through_urls: ::std::option::Option<Vec<String>>,
        #[doc = "All declared restricted categories for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "declaredRestrictedCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub declared_restricted_categories:
            ::std::option::Option<Vec<crate::schemas::CreativeDeclaredRestrictedCategoriesItems>>,
        #[doc = "IDs for the declared ad technology vendors that may be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "declaredVendorIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub declared_vendor_ids: ::std::option::Option<Vec<i32>>,
        #[doc = "An HTML creative."]
        #[serde(
            rename = "html",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html: ::std::option::Option<crate::schemas::HtmlContent>,
        #[doc = "The set of URLs to be called to record an impression."]
        #[serde(
            rename = "impressionTrackingUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impression_tracking_urls: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Name of the creative. Follows the pattern `buyers/{buyer}/creatives/{creative}`, where `{buyer}` represents the account ID of the buyer who owns the creative, and `{creative}` is the buyer-specific creative ID that references this creative in the bid response."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A native creative."]
        #[serde(
            rename = "native",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub native: ::std::option::Option<crate::schemas::NativeContent>,
        #[doc = "All restricted categories for the ads that may be shown from this creative."]
        #[serde(
            rename = "restrictedCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restricted_categories:
            ::std::option::Option<Vec<crate::schemas::CreativeRestrictedCategoriesItems>>,
        #[doc = "Output only. The version of this creative. Version for a new creative is 1 and it increments during subsequent creative updates."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
        #[doc = "A video creative."]
        #[serde(
            rename = "video",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video: ::std::option::Option<crate::schemas::VideoContent>,
    }
    impl ::google_field_selector::FieldSelector for Creative {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Creative {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeCreativeFormat {
        #[doc = "The format is unknown."]
        CreativeFormatUnspecified,
        #[doc = "HTML creative."]
        Html,
        #[doc = "Native creative."]
        Native,
        #[doc = "Video creative."]
        Video,
    }
    impl CreativeCreativeFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeCreativeFormat::CreativeFormatUnspecified => "CREATIVE_FORMAT_UNSPECIFIED",
                CreativeCreativeFormat::Html => "HTML",
                CreativeCreativeFormat::Native => "NATIVE",
                CreativeCreativeFormat::Video => "VIDEO",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreativeCreativeFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreativeCreativeFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreativeCreativeFormat, ()> {
            Ok(match s {
                "CREATIVE_FORMAT_UNSPECIFIED" => CreativeCreativeFormat::CreativeFormatUnspecified,
                "HTML" => CreativeCreativeFormat::Html,
                "NATIVE" => CreativeCreativeFormat::Native,
                "VIDEO" => CreativeCreativeFormat::Video,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreativeCreativeFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeCreativeFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeCreativeFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATIVE_FORMAT_UNSPECIFIED" => CreativeCreativeFormat::CreativeFormatUnspecified,
                "HTML" => CreativeCreativeFormat::Html,
                "NATIVE" => CreativeCreativeFormat::Native,
                "VIDEO" => CreativeCreativeFormat::Video,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CreativeCreativeFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeCreativeFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeDeclaredAttributesItems {
        #[doc = "The creative is of video type Adobe Flash FLV. For pretargeting."]
        AdobeFlashFlv,
        #[doc = "The creative has an interstitial size of any interstitial. For pretargeting."]
        AnyInterstitial,
        #[doc = "Do not use. This is a placeholder value only."]
        AttributeUnspecified,
        #[doc = "The creative type is HTML."]
        CreativeTypeHtml,
        #[doc = "The creative type is VAST video."]
        CreativeTypeVastVideo,
        #[doc = "The creative expands when rolled over."]
        ExpandingActionRolloverToExpand,
        #[doc = "The creative expands on any diagonal."]
        ExpandingDirectionAnyDiagonal,
        #[doc = "The creative expands down."]
        ExpandingDirectionDown,
        #[doc = "The creative expands down and left."]
        ExpandingDirectionDownLeft,
        #[doc = "The creative expands down and right."]
        ExpandingDirectionDownRight,
        #[doc = "The creative expands left."]
        ExpandingDirectionLeft,
        #[doc = "The creative expands left or right."]
        ExpandingDirectionLeftOrRight,
        #[doc = "The creative does not expand."]
        ExpandingDirectionNone,
        #[doc = "The creative expands right."]
        ExpandingDirectionRight,
        #[doc = "The creative expands up."]
        ExpandingDirectionUp,
        #[doc = "The creative expands up and left."]
        ExpandingDirectionUpLeft,
        #[doc = "The creative expands up or down."]
        ExpandingDirectionUpOrDown,
        #[doc = "The creative expands up and right."]
        ExpandingDirectionUpRight,
        #[doc = "The creative is of type image/rich media. For pretargeting."]
        ImageRichMedia,
        #[doc = "The video type is in-banner video."]
        InBannerVideo,
        #[doc = "The instream vast video type is vpaid flash."]
        InstreamVastVideoTypeVpaidFlash,
        #[doc = "The creative is cookie targeted."]
        IsCookieTargeted,
        #[doc = "The creative is tagged."]
        IsTagged,
        #[doc = "The creative is user interest targeted."]
        IsUserInterestTargeted,
        #[doc = "The creative is eligible for native."]
        NativeEligibilityEligible,
        #[doc = "The creative is not eligible for native."]
        NativeEligibilityNotEligible,
        #[doc = "The creative has an interstitial size of non interstitial. For pretargeting."]
        NonInterstitial,
        #[doc = "The creative has an instream VAST video type of non-skippable instream video. For pretargeting."]
        NonSkippableInstreamVideo,
        #[doc = "The creative has an instream VAST video type of non-VPAID. For pretargeting."]
        NonVpaid,
        #[doc = "The open measurement SDK is supported."]
        Omsdk10,
        #[doc = "The creative can dynamically resize to fill a variety of slot sizes."]
        RenderingSizelessAdx,
        #[doc = "The creative is Flash."]
        RichMediaCapabilityTypeFlash,
        #[doc = "The creative is HTML5."]
        RichMediaCapabilityTypeHtml5,
        #[doc = "The creative is an interstitial."]
        RichMediaCapabilityTypeInterstitial,
        #[doc = "The creative is MRAID."]
        RichMediaCapabilityTypeMraid,
        #[doc = "The creative is non-SSL."]
        RichMediaCapabilityTypeNonSsl,
        #[doc = "The creative is SSL."]
        RichMediaCapabilityTypeSsl,
        #[doc = "The creative has an instream VAST video type of skippable instream video. For pretargeting."]
        SkippableInstreamVideo,
    }
    impl CreativeDeclaredAttributesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeDeclaredAttributesItems::AdobeFlashFlv => "ADOBE_FLASH_FLV",
                CreativeDeclaredAttributesItems::AnyInterstitial => "ANY_INTERSTITIAL",
                CreativeDeclaredAttributesItems::AttributeUnspecified => "ATTRIBUTE_UNSPECIFIED",
                CreativeDeclaredAttributesItems::CreativeTypeHtml => "CREATIVE_TYPE_HTML",
                CreativeDeclaredAttributesItems::CreativeTypeVastVideo => {
                    "CREATIVE_TYPE_VAST_VIDEO"
                }
                CreativeDeclaredAttributesItems::ExpandingActionRolloverToExpand => {
                    "EXPANDING_ACTION_ROLLOVER_TO_EXPAND"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionAnyDiagonal => {
                    "EXPANDING_DIRECTION_ANY_DIAGONAL"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionDown => {
                    "EXPANDING_DIRECTION_DOWN"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionDownLeft => {
                    "EXPANDING_DIRECTION_DOWN_LEFT"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionDownRight => {
                    "EXPANDING_DIRECTION_DOWN_RIGHT"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionLeft => {
                    "EXPANDING_DIRECTION_LEFT"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionLeftOrRight => {
                    "EXPANDING_DIRECTION_LEFT_OR_RIGHT"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionNone => {
                    "EXPANDING_DIRECTION_NONE"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionRight => {
                    "EXPANDING_DIRECTION_RIGHT"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionUp => "EXPANDING_DIRECTION_UP",
                CreativeDeclaredAttributesItems::ExpandingDirectionUpLeft => {
                    "EXPANDING_DIRECTION_UP_LEFT"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionUpOrDown => {
                    "EXPANDING_DIRECTION_UP_OR_DOWN"
                }
                CreativeDeclaredAttributesItems::ExpandingDirectionUpRight => {
                    "EXPANDING_DIRECTION_UP_RIGHT"
                }
                CreativeDeclaredAttributesItems::ImageRichMedia => "IMAGE_RICH_MEDIA",
                CreativeDeclaredAttributesItems::InBannerVideo => "IN_BANNER_VIDEO",
                CreativeDeclaredAttributesItems::InstreamVastVideoTypeVpaidFlash => {
                    "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH"
                }
                CreativeDeclaredAttributesItems::IsCookieTargeted => "IS_COOKIE_TARGETED",
                CreativeDeclaredAttributesItems::IsTagged => "IS_TAGGED",
                CreativeDeclaredAttributesItems::IsUserInterestTargeted => {
                    "IS_USER_INTEREST_TARGETED"
                }
                CreativeDeclaredAttributesItems::NativeEligibilityEligible => {
                    "NATIVE_ELIGIBILITY_ELIGIBLE"
                }
                CreativeDeclaredAttributesItems::NativeEligibilityNotEligible => {
                    "NATIVE_ELIGIBILITY_NOT_ELIGIBLE"
                }
                CreativeDeclaredAttributesItems::NonInterstitial => "NON_INTERSTITIAL",
                CreativeDeclaredAttributesItems::NonSkippableInstreamVideo => {
                    "NON_SKIPPABLE_INSTREAM_VIDEO"
                }
                CreativeDeclaredAttributesItems::NonVpaid => "NON_VPAID",
                CreativeDeclaredAttributesItems::Omsdk10 => "OMSDK_1_0",
                CreativeDeclaredAttributesItems::RenderingSizelessAdx => "RENDERING_SIZELESS_ADX",
                CreativeDeclaredAttributesItems::RichMediaCapabilityTypeFlash => {
                    "RICH_MEDIA_CAPABILITY_TYPE_FLASH"
                }
                CreativeDeclaredAttributesItems::RichMediaCapabilityTypeHtml5 => {
                    "RICH_MEDIA_CAPABILITY_TYPE_HTML5"
                }
                CreativeDeclaredAttributesItems::RichMediaCapabilityTypeInterstitial => {
                    "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL"
                }
                CreativeDeclaredAttributesItems::RichMediaCapabilityTypeMraid => {
                    "RICH_MEDIA_CAPABILITY_TYPE_MRAID"
                }
                CreativeDeclaredAttributesItems::RichMediaCapabilityTypeNonSsl => {
                    "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL"
                }
                CreativeDeclaredAttributesItems::RichMediaCapabilityTypeSsl => {
                    "RICH_MEDIA_CAPABILITY_TYPE_SSL"
                }
                CreativeDeclaredAttributesItems::SkippableInstreamVideo => {
                    "SKIPPABLE_INSTREAM_VIDEO"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreativeDeclaredAttributesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreativeDeclaredAttributesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreativeDeclaredAttributesItems, ()> {
            Ok(match s {
                "ADOBE_FLASH_FLV" => CreativeDeclaredAttributesItems::AdobeFlashFlv,
                "ANY_INTERSTITIAL" => CreativeDeclaredAttributesItems::AnyInterstitial,
                "ATTRIBUTE_UNSPECIFIED" => CreativeDeclaredAttributesItems::AttributeUnspecified,
                "CREATIVE_TYPE_HTML" => CreativeDeclaredAttributesItems::CreativeTypeHtml,
                "CREATIVE_TYPE_VAST_VIDEO" => {
                    CreativeDeclaredAttributesItems::CreativeTypeVastVideo
                }
                "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" => {
                    CreativeDeclaredAttributesItems::ExpandingActionRolloverToExpand
                }
                "EXPANDING_DIRECTION_ANY_DIAGONAL" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionAnyDiagonal
                }
                "EXPANDING_DIRECTION_DOWN" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionDown
                }
                "EXPANDING_DIRECTION_DOWN_LEFT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionDownLeft
                }
                "EXPANDING_DIRECTION_DOWN_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionDownRight
                }
                "EXPANDING_DIRECTION_LEFT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionLeft
                }
                "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionLeftOrRight
                }
                "EXPANDING_DIRECTION_NONE" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionNone
                }
                "EXPANDING_DIRECTION_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionRight
                }
                "EXPANDING_DIRECTION_UP" => CreativeDeclaredAttributesItems::ExpandingDirectionUp,
                "EXPANDING_DIRECTION_UP_LEFT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionUpLeft
                }
                "EXPANDING_DIRECTION_UP_OR_DOWN" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionUpOrDown
                }
                "EXPANDING_DIRECTION_UP_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionUpRight
                }
                "IMAGE_RICH_MEDIA" => CreativeDeclaredAttributesItems::ImageRichMedia,
                "IN_BANNER_VIDEO" => CreativeDeclaredAttributesItems::InBannerVideo,
                "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" => {
                    CreativeDeclaredAttributesItems::InstreamVastVideoTypeVpaidFlash
                }
                "IS_COOKIE_TARGETED" => CreativeDeclaredAttributesItems::IsCookieTargeted,
                "IS_TAGGED" => CreativeDeclaredAttributesItems::IsTagged,
                "IS_USER_INTEREST_TARGETED" => {
                    CreativeDeclaredAttributesItems::IsUserInterestTargeted
                }
                "NATIVE_ELIGIBILITY_ELIGIBLE" => {
                    CreativeDeclaredAttributesItems::NativeEligibilityEligible
                }
                "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" => {
                    CreativeDeclaredAttributesItems::NativeEligibilityNotEligible
                }
                "NON_INTERSTITIAL" => CreativeDeclaredAttributesItems::NonInterstitial,
                "NON_SKIPPABLE_INSTREAM_VIDEO" => {
                    CreativeDeclaredAttributesItems::NonSkippableInstreamVideo
                }
                "NON_VPAID" => CreativeDeclaredAttributesItems::NonVpaid,
                "OMSDK_1_0" => CreativeDeclaredAttributesItems::Omsdk10,
                "RENDERING_SIZELESS_ADX" => CreativeDeclaredAttributesItems::RenderingSizelessAdx,
                "RICH_MEDIA_CAPABILITY_TYPE_FLASH" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeFlash
                }
                "RICH_MEDIA_CAPABILITY_TYPE_HTML5" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeHtml5
                }
                "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeInterstitial
                }
                "RICH_MEDIA_CAPABILITY_TYPE_MRAID" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeMraid
                }
                "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeNonSsl
                }
                "RICH_MEDIA_CAPABILITY_TYPE_SSL" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeSsl
                }
                "SKIPPABLE_INSTREAM_VIDEO" => {
                    CreativeDeclaredAttributesItems::SkippableInstreamVideo
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreativeDeclaredAttributesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeDeclaredAttributesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeDeclaredAttributesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADOBE_FLASH_FLV" => CreativeDeclaredAttributesItems::AdobeFlashFlv,
                "ANY_INTERSTITIAL" => CreativeDeclaredAttributesItems::AnyInterstitial,
                "ATTRIBUTE_UNSPECIFIED" => CreativeDeclaredAttributesItems::AttributeUnspecified,
                "CREATIVE_TYPE_HTML" => CreativeDeclaredAttributesItems::CreativeTypeHtml,
                "CREATIVE_TYPE_VAST_VIDEO" => {
                    CreativeDeclaredAttributesItems::CreativeTypeVastVideo
                }
                "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" => {
                    CreativeDeclaredAttributesItems::ExpandingActionRolloverToExpand
                }
                "EXPANDING_DIRECTION_ANY_DIAGONAL" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionAnyDiagonal
                }
                "EXPANDING_DIRECTION_DOWN" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionDown
                }
                "EXPANDING_DIRECTION_DOWN_LEFT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionDownLeft
                }
                "EXPANDING_DIRECTION_DOWN_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionDownRight
                }
                "EXPANDING_DIRECTION_LEFT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionLeft
                }
                "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionLeftOrRight
                }
                "EXPANDING_DIRECTION_NONE" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionNone
                }
                "EXPANDING_DIRECTION_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionRight
                }
                "EXPANDING_DIRECTION_UP" => CreativeDeclaredAttributesItems::ExpandingDirectionUp,
                "EXPANDING_DIRECTION_UP_LEFT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionUpLeft
                }
                "EXPANDING_DIRECTION_UP_OR_DOWN" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionUpOrDown
                }
                "EXPANDING_DIRECTION_UP_RIGHT" => {
                    CreativeDeclaredAttributesItems::ExpandingDirectionUpRight
                }
                "IMAGE_RICH_MEDIA" => CreativeDeclaredAttributesItems::ImageRichMedia,
                "IN_BANNER_VIDEO" => CreativeDeclaredAttributesItems::InBannerVideo,
                "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" => {
                    CreativeDeclaredAttributesItems::InstreamVastVideoTypeVpaidFlash
                }
                "IS_COOKIE_TARGETED" => CreativeDeclaredAttributesItems::IsCookieTargeted,
                "IS_TAGGED" => CreativeDeclaredAttributesItems::IsTagged,
                "IS_USER_INTEREST_TARGETED" => {
                    CreativeDeclaredAttributesItems::IsUserInterestTargeted
                }
                "NATIVE_ELIGIBILITY_ELIGIBLE" => {
                    CreativeDeclaredAttributesItems::NativeEligibilityEligible
                }
                "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" => {
                    CreativeDeclaredAttributesItems::NativeEligibilityNotEligible
                }
                "NON_INTERSTITIAL" => CreativeDeclaredAttributesItems::NonInterstitial,
                "NON_SKIPPABLE_INSTREAM_VIDEO" => {
                    CreativeDeclaredAttributesItems::NonSkippableInstreamVideo
                }
                "NON_VPAID" => CreativeDeclaredAttributesItems::NonVpaid,
                "OMSDK_1_0" => CreativeDeclaredAttributesItems::Omsdk10,
                "RENDERING_SIZELESS_ADX" => CreativeDeclaredAttributesItems::RenderingSizelessAdx,
                "RICH_MEDIA_CAPABILITY_TYPE_FLASH" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeFlash
                }
                "RICH_MEDIA_CAPABILITY_TYPE_HTML5" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeHtml5
                }
                "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeInterstitial
                }
                "RICH_MEDIA_CAPABILITY_TYPE_MRAID" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeMraid
                }
                "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeNonSsl
                }
                "RICH_MEDIA_CAPABILITY_TYPE_SSL" => {
                    CreativeDeclaredAttributesItems::RichMediaCapabilityTypeSsl
                }
                "SKIPPABLE_INSTREAM_VIDEO" => {
                    CreativeDeclaredAttributesItems::SkippableInstreamVideo
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
    impl ::google_field_selector::FieldSelector for CreativeDeclaredAttributesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeDeclaredAttributesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeDeclaredRestrictedCategoriesItems {
        #[doc = "The alcohol restricted category."]
        Alcohol,
        #[doc = "Default value that should never be used."]
        RestrictedCategoryUnspecified,
    }
    impl CreativeDeclaredRestrictedCategoriesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeDeclaredRestrictedCategoriesItems::Alcohol => "ALCOHOL",
                CreativeDeclaredRestrictedCategoriesItems::RestrictedCategoryUnspecified => {
                    "RESTRICTED_CATEGORY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreativeDeclaredRestrictedCategoriesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreativeDeclaredRestrictedCategoriesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CreativeDeclaredRestrictedCategoriesItems, ()> {
            Ok(match s {
                "ALCOHOL" => CreativeDeclaredRestrictedCategoriesItems::Alcohol,
                "RESTRICTED_CATEGORY_UNSPECIFIED" => {
                    CreativeDeclaredRestrictedCategoriesItems::RestrictedCategoryUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreativeDeclaredRestrictedCategoriesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeDeclaredRestrictedCategoriesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeDeclaredRestrictedCategoriesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALCOHOL" => CreativeDeclaredRestrictedCategoriesItems::Alcohol,
                "RESTRICTED_CATEGORY_UNSPECIFIED" => {
                    CreativeDeclaredRestrictedCategoriesItems::RestrictedCategoryUnspecified
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
    impl ::google_field_selector::FieldSelector for CreativeDeclaredRestrictedCategoriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeDeclaredRestrictedCategoriesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeRestrictedCategoriesItems {
        #[doc = "The alcohol restricted category."]
        Alcohol,
        #[doc = "Default value that should never be used."]
        RestrictedCategoryUnspecified,
    }
    impl CreativeRestrictedCategoriesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                CreativeRestrictedCategoriesItems::Alcohol => "ALCOHOL",
                CreativeRestrictedCategoriesItems::RestrictedCategoryUnspecified => {
                    "RESTRICTED_CATEGORY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CreativeRestrictedCategoriesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreativeRestrictedCategoriesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CreativeRestrictedCategoriesItems, ()> {
            Ok(match s {
                "ALCOHOL" => CreativeRestrictedCategoriesItems::Alcohol,
                "RESTRICTED_CATEGORY_UNSPECIFIED" => {
                    CreativeRestrictedCategoriesItems::RestrictedCategoryUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CreativeRestrictedCategoriesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeRestrictedCategoriesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeRestrictedCategoriesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALCOHOL" => CreativeRestrictedCategoriesItems::Alcohol,
                "RESTRICTED_CATEGORY_UNSPECIFIED" => {
                    CreativeRestrictedCategoriesItems::RestrictedCategoryUnspecified
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
    impl ::google_field_selector::FieldSelector for CreativeRestrictedCategoriesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeRestrictedCategoriesItems {
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
    pub struct CreativeDimensions {
        #[doc = "The height of the creative in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub height: ::std::option::Option<i64>,
        #[doc = "The width of the creative in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub width: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for CreativeDimensions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeDimensions {
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
    pub struct CreativeServingDecision {
        #[doc = "The detected ad technology providers."]
        #[serde(
            rename = "adTechnologyProviders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_technology_providers: ::std::option::Option<crate::schemas::AdTechnologyProviders>,
        #[doc = "The policy compliance of this creative in China. When approved or disapproved, this applies to both deals and open auction in China. When pending review, this creative is allowed to serve for deals but not for open auction."]
        #[serde(
            rename = "chinaPolicyCompliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub china_policy_compliance: ::std::option::Option<crate::schemas::PolicyCompliance>,
        #[doc = "Policy compliance of this creative when bidding on Programmatic Guaranteed and Preferred Deals (outside of Russia and China)."]
        #[serde(
            rename = "dealsPolicyCompliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals_policy_compliance: ::std::option::Option<crate::schemas::PolicyCompliance>,
        #[doc = "Detected advertisers and brands."]
        #[serde(
            rename = "detectedAdvertisers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_advertisers: ::std::option::Option<Vec<crate::schemas::AdvertiserAndBrand>>,
        #[doc = "Publisher-excludable attributes that were detected for this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction."]
        #[serde(
            rename = "detectedAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_attributes: ::std::option::Option<
            Vec<crate::schemas::CreativeServingDecisionDetectedAttributesItems>,
        >,
        #[doc = "The set of detected destination URLs for the creative. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "detectedClickThroughUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_click_through_urls: ::std::option::Option<Vec<String>>,
        #[doc = "The detected domains for this creative."]
        #[serde(
            rename = "detectedDomains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_domains: ::std::option::Option<Vec<String>>,
        #[doc = "The detected languages for this creative. The order is arbitrary. The codes are 2 or 5 characters and are documented at https://developers.google.com/adwords/api/docs/appendix/languagecodes. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "detectedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_languages: ::std::option::Option<Vec<String>>,
        #[doc = "Detected product categories, if any. See the ad-product-categories.txt file in the technical documentation for a list of IDs. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "detectedProductCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_product_categories: ::std::option::Option<Vec<i32>>,
        #[doc = "Detected sensitive categories, if any. Can be used to filter the response of the creatives.list method. See the ad-sensitive-categories.txt file in the technical documentation for a list of IDs. You should use these IDs along with the excluded-sensitive-category field in the bid request to filter your bids."]
        #[serde(
            rename = "detectedSensitiveCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_sensitive_categories: ::std::option::Option<Vec<i32>>,
        #[doc = "IDs of the ad technology vendors that were detected to be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method. If the `allowed_vendor_type` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) does not contain one of the vendor type IDs that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction."]
        #[serde(
            rename = "detectedVendorIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_vendor_ids: ::std::option::Option<Vec<i32>>,
        #[doc = "The last time the creative status was updated. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "lastStatusUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_status_update: ::std::option::Option<String>,
        #[doc = "Policy compliance of this creative when bidding in open auction, private auction, or auction packages (outside of Russia and China)."]
        #[serde(
            rename = "networkPolicyCompliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_policy_compliance: ::std::option::Option<crate::schemas::PolicyCompliance>,
        #[doc = "Policy compliance of this creative when bidding in Open Bidding (outside of Russia and China). For the list of platform policies, see: https://support.google.com/platformspolicy/answer/3013851."]
        #[serde(
            rename = "platformPolicyCompliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform_policy_compliance: ::std::option::Option<crate::schemas::PolicyCompliance>,
        #[doc = "The policy compliance of this creative in Russia. When approved or disapproved, this applies to both deals and open auction in Russia. When pending review, this creative is allowed to serve for deals but not for open auction."]
        #[serde(
            rename = "russiaPolicyCompliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub russia_policy_compliance: ::std::option::Option<crate::schemas::PolicyCompliance>,
    }
    impl ::google_field_selector::FieldSelector for CreativeServingDecision {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeServingDecision {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreativeServingDecisionDetectedAttributesItems {
        #[doc = "The creative is of video type Adobe Flash FLV. For pretargeting."]
        AdobeFlashFlv,
        #[doc = "The creative has an interstitial size of any interstitial. For pretargeting."]
        AnyInterstitial,
        #[doc = "Do not use. This is a placeholder value only."]
        AttributeUnspecified,
        #[doc = "The creative type is HTML."]
        CreativeTypeHtml,
        #[doc = "The creative type is VAST video."]
        CreativeTypeVastVideo,
        #[doc = "The creative expands when rolled over."]
        ExpandingActionRolloverToExpand,
        #[doc = "The creative expands on any diagonal."]
        ExpandingDirectionAnyDiagonal,
        #[doc = "The creative expands down."]
        ExpandingDirectionDown,
        #[doc = "The creative expands down and left."]
        ExpandingDirectionDownLeft,
        #[doc = "The creative expands down and right."]
        ExpandingDirectionDownRight,
        #[doc = "The creative expands left."]
        ExpandingDirectionLeft,
        #[doc = "The creative expands left or right."]
        ExpandingDirectionLeftOrRight,
        #[doc = "The creative does not expand."]
        ExpandingDirectionNone,
        #[doc = "The creative expands right."]
        ExpandingDirectionRight,
        #[doc = "The creative expands up."]
        ExpandingDirectionUp,
        #[doc = "The creative expands up and left."]
        ExpandingDirectionUpLeft,
        #[doc = "The creative expands up or down."]
        ExpandingDirectionUpOrDown,
        #[doc = "The creative expands up and right."]
        ExpandingDirectionUpRight,
        #[doc = "The creative is of type image/rich media. For pretargeting."]
        ImageRichMedia,
        #[doc = "The video type is in-banner video."]
        InBannerVideo,
        #[doc = "The instream vast video type is vpaid flash."]
        InstreamVastVideoTypeVpaidFlash,
        #[doc = "The creative is cookie targeted."]
        IsCookieTargeted,
        #[doc = "The creative is tagged."]
        IsTagged,
        #[doc = "The creative is user interest targeted."]
        IsUserInterestTargeted,
        #[doc = "The creative is eligible for native."]
        NativeEligibilityEligible,
        #[doc = "The creative is not eligible for native."]
        NativeEligibilityNotEligible,
        #[doc = "The creative has an interstitial size of non interstitial. For pretargeting."]
        NonInterstitial,
        #[doc = "The creative has an instream VAST video type of non-skippable instream video. For pretargeting."]
        NonSkippableInstreamVideo,
        #[doc = "The creative has an instream VAST video type of non-VPAID. For pretargeting."]
        NonVpaid,
        #[doc = "The open measurement SDK is supported."]
        Omsdk10,
        #[doc = "The creative can dynamically resize to fill a variety of slot sizes."]
        RenderingSizelessAdx,
        #[doc = "The creative is Flash."]
        RichMediaCapabilityTypeFlash,
        #[doc = "The creative is HTML5."]
        RichMediaCapabilityTypeHtml5,
        #[doc = "The creative is an interstitial."]
        RichMediaCapabilityTypeInterstitial,
        #[doc = "The creative is MRAID."]
        RichMediaCapabilityTypeMraid,
        #[doc = "The creative is non-SSL."]
        RichMediaCapabilityTypeNonSsl,
        #[doc = "The creative is SSL."]
        RichMediaCapabilityTypeSsl,
        #[doc = "The creative has an instream VAST video type of skippable instream video. For pretargeting."]
        SkippableInstreamVideo,
    }
    impl CreativeServingDecisionDetectedAttributesItems {
        pub fn as_str(self) -> &'static str {
            match self { CreativeServingDecisionDetectedAttributesItems :: AdobeFlashFlv => "ADOBE_FLASH_FLV" , CreativeServingDecisionDetectedAttributesItems :: AnyInterstitial => "ANY_INTERSTITIAL" , CreativeServingDecisionDetectedAttributesItems :: AttributeUnspecified => "ATTRIBUTE_UNSPECIFIED" , CreativeServingDecisionDetectedAttributesItems :: CreativeTypeHtml => "CREATIVE_TYPE_HTML" , CreativeServingDecisionDetectedAttributesItems :: CreativeTypeVastVideo => "CREATIVE_TYPE_VAST_VIDEO" , CreativeServingDecisionDetectedAttributesItems :: ExpandingActionRolloverToExpand => "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionAnyDiagonal => "EXPANDING_DIRECTION_ANY_DIAGONAL" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDown => "EXPANDING_DIRECTION_DOWN" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDownLeft => "EXPANDING_DIRECTION_DOWN_LEFT" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDownRight => "EXPANDING_DIRECTION_DOWN_RIGHT" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionLeft => "EXPANDING_DIRECTION_LEFT" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionLeftOrRight => "EXPANDING_DIRECTION_LEFT_OR_RIGHT" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionNone => "EXPANDING_DIRECTION_NONE" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionRight => "EXPANDING_DIRECTION_RIGHT" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUp => "EXPANDING_DIRECTION_UP" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpLeft => "EXPANDING_DIRECTION_UP_LEFT" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpOrDown => "EXPANDING_DIRECTION_UP_OR_DOWN" , CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpRight => "EXPANDING_DIRECTION_UP_RIGHT" , CreativeServingDecisionDetectedAttributesItems :: ImageRichMedia => "IMAGE_RICH_MEDIA" , CreativeServingDecisionDetectedAttributesItems :: InBannerVideo => "IN_BANNER_VIDEO" , CreativeServingDecisionDetectedAttributesItems :: InstreamVastVideoTypeVpaidFlash => "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" , CreativeServingDecisionDetectedAttributesItems :: IsCookieTargeted => "IS_COOKIE_TARGETED" , CreativeServingDecisionDetectedAttributesItems :: IsTagged => "IS_TAGGED" , CreativeServingDecisionDetectedAttributesItems :: IsUserInterestTargeted => "IS_USER_INTEREST_TARGETED" , CreativeServingDecisionDetectedAttributesItems :: NativeEligibilityEligible => "NATIVE_ELIGIBILITY_ELIGIBLE" , CreativeServingDecisionDetectedAttributesItems :: NativeEligibilityNotEligible => "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" , CreativeServingDecisionDetectedAttributesItems :: NonInterstitial => "NON_INTERSTITIAL" , CreativeServingDecisionDetectedAttributesItems :: NonSkippableInstreamVideo => "NON_SKIPPABLE_INSTREAM_VIDEO" , CreativeServingDecisionDetectedAttributesItems :: NonVpaid => "NON_VPAID" , CreativeServingDecisionDetectedAttributesItems :: Omsdk10 => "OMSDK_1_0" , CreativeServingDecisionDetectedAttributesItems :: RenderingSizelessAdx => "RENDERING_SIZELESS_ADX" , CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeFlash => "RICH_MEDIA_CAPABILITY_TYPE_FLASH" , CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeHtml5 => "RICH_MEDIA_CAPABILITY_TYPE_HTML5" , CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeInterstitial => "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" , CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeMraid => "RICH_MEDIA_CAPABILITY_TYPE_MRAID" , CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeNonSsl => "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" , CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeSsl => "RICH_MEDIA_CAPABILITY_TYPE_SSL" , CreativeServingDecisionDetectedAttributesItems :: SkippableInstreamVideo => "SKIPPABLE_INSTREAM_VIDEO" , }
        }
    }
    impl ::std::convert::AsRef<str> for CreativeServingDecisionDetectedAttributesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CreativeServingDecisionDetectedAttributesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CreativeServingDecisionDetectedAttributesItems, ()> {
            Ok ( match s { "ADOBE_FLASH_FLV" => CreativeServingDecisionDetectedAttributesItems :: AdobeFlashFlv , "ANY_INTERSTITIAL" => CreativeServingDecisionDetectedAttributesItems :: AnyInterstitial , "ATTRIBUTE_UNSPECIFIED" => CreativeServingDecisionDetectedAttributesItems :: AttributeUnspecified , "CREATIVE_TYPE_HTML" => CreativeServingDecisionDetectedAttributesItems :: CreativeTypeHtml , "CREATIVE_TYPE_VAST_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: CreativeTypeVastVideo , "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" => CreativeServingDecisionDetectedAttributesItems :: ExpandingActionRolloverToExpand , "EXPANDING_DIRECTION_ANY_DIAGONAL" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionAnyDiagonal , "EXPANDING_DIRECTION_DOWN" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDown , "EXPANDING_DIRECTION_DOWN_LEFT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDownLeft , "EXPANDING_DIRECTION_DOWN_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDownRight , "EXPANDING_DIRECTION_LEFT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionLeft , "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionLeftOrRight , "EXPANDING_DIRECTION_NONE" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionNone , "EXPANDING_DIRECTION_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionRight , "EXPANDING_DIRECTION_UP" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUp , "EXPANDING_DIRECTION_UP_LEFT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpLeft , "EXPANDING_DIRECTION_UP_OR_DOWN" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpOrDown , "EXPANDING_DIRECTION_UP_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpRight , "IMAGE_RICH_MEDIA" => CreativeServingDecisionDetectedAttributesItems :: ImageRichMedia , "IN_BANNER_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: InBannerVideo , "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" => CreativeServingDecisionDetectedAttributesItems :: InstreamVastVideoTypeVpaidFlash , "IS_COOKIE_TARGETED" => CreativeServingDecisionDetectedAttributesItems :: IsCookieTargeted , "IS_TAGGED" => CreativeServingDecisionDetectedAttributesItems :: IsTagged , "IS_USER_INTEREST_TARGETED" => CreativeServingDecisionDetectedAttributesItems :: IsUserInterestTargeted , "NATIVE_ELIGIBILITY_ELIGIBLE" => CreativeServingDecisionDetectedAttributesItems :: NativeEligibilityEligible , "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" => CreativeServingDecisionDetectedAttributesItems :: NativeEligibilityNotEligible , "NON_INTERSTITIAL" => CreativeServingDecisionDetectedAttributesItems :: NonInterstitial , "NON_SKIPPABLE_INSTREAM_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: NonSkippableInstreamVideo , "NON_VPAID" => CreativeServingDecisionDetectedAttributesItems :: NonVpaid , "OMSDK_1_0" => CreativeServingDecisionDetectedAttributesItems :: Omsdk10 , "RENDERING_SIZELESS_ADX" => CreativeServingDecisionDetectedAttributesItems :: RenderingSizelessAdx , "RICH_MEDIA_CAPABILITY_TYPE_FLASH" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeFlash , "RICH_MEDIA_CAPABILITY_TYPE_HTML5" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeHtml5 , "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeInterstitial , "RICH_MEDIA_CAPABILITY_TYPE_MRAID" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeMraid , "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeNonSsl , "RICH_MEDIA_CAPABILITY_TYPE_SSL" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeSsl , "SKIPPABLE_INSTREAM_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: SkippableInstreamVideo , _ => return Err ( ( ) ) , } )
        }
    }
    impl ::std::fmt::Display for CreativeServingDecisionDetectedAttributesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreativeServingDecisionDetectedAttributesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreativeServingDecisionDetectedAttributesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "ADOBE_FLASH_FLV" => CreativeServingDecisionDetectedAttributesItems :: AdobeFlashFlv , "ANY_INTERSTITIAL" => CreativeServingDecisionDetectedAttributesItems :: AnyInterstitial , "ATTRIBUTE_UNSPECIFIED" => CreativeServingDecisionDetectedAttributesItems :: AttributeUnspecified , "CREATIVE_TYPE_HTML" => CreativeServingDecisionDetectedAttributesItems :: CreativeTypeHtml , "CREATIVE_TYPE_VAST_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: CreativeTypeVastVideo , "EXPANDING_ACTION_ROLLOVER_TO_EXPAND" => CreativeServingDecisionDetectedAttributesItems :: ExpandingActionRolloverToExpand , "EXPANDING_DIRECTION_ANY_DIAGONAL" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionAnyDiagonal , "EXPANDING_DIRECTION_DOWN" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDown , "EXPANDING_DIRECTION_DOWN_LEFT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDownLeft , "EXPANDING_DIRECTION_DOWN_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionDownRight , "EXPANDING_DIRECTION_LEFT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionLeft , "EXPANDING_DIRECTION_LEFT_OR_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionLeftOrRight , "EXPANDING_DIRECTION_NONE" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionNone , "EXPANDING_DIRECTION_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionRight , "EXPANDING_DIRECTION_UP" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUp , "EXPANDING_DIRECTION_UP_LEFT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpLeft , "EXPANDING_DIRECTION_UP_OR_DOWN" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpOrDown , "EXPANDING_DIRECTION_UP_RIGHT" => CreativeServingDecisionDetectedAttributesItems :: ExpandingDirectionUpRight , "IMAGE_RICH_MEDIA" => CreativeServingDecisionDetectedAttributesItems :: ImageRichMedia , "IN_BANNER_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: InBannerVideo , "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH" => CreativeServingDecisionDetectedAttributesItems :: InstreamVastVideoTypeVpaidFlash , "IS_COOKIE_TARGETED" => CreativeServingDecisionDetectedAttributesItems :: IsCookieTargeted , "IS_TAGGED" => CreativeServingDecisionDetectedAttributesItems :: IsTagged , "IS_USER_INTEREST_TARGETED" => CreativeServingDecisionDetectedAttributesItems :: IsUserInterestTargeted , "NATIVE_ELIGIBILITY_ELIGIBLE" => CreativeServingDecisionDetectedAttributesItems :: NativeEligibilityEligible , "NATIVE_ELIGIBILITY_NOT_ELIGIBLE" => CreativeServingDecisionDetectedAttributesItems :: NativeEligibilityNotEligible , "NON_INTERSTITIAL" => CreativeServingDecisionDetectedAttributesItems :: NonInterstitial , "NON_SKIPPABLE_INSTREAM_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: NonSkippableInstreamVideo , "NON_VPAID" => CreativeServingDecisionDetectedAttributesItems :: NonVpaid , "OMSDK_1_0" => CreativeServingDecisionDetectedAttributesItems :: Omsdk10 , "RENDERING_SIZELESS_ADX" => CreativeServingDecisionDetectedAttributesItems :: RenderingSizelessAdx , "RICH_MEDIA_CAPABILITY_TYPE_FLASH" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeFlash , "RICH_MEDIA_CAPABILITY_TYPE_HTML5" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeHtml5 , "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeInterstitial , "RICH_MEDIA_CAPABILITY_TYPE_MRAID" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeMraid , "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeNonSsl , "RICH_MEDIA_CAPABILITY_TYPE_SSL" => CreativeServingDecisionDetectedAttributesItems :: RichMediaCapabilityTypeSsl , "SKIPPABLE_INSTREAM_VIDEO" => CreativeServingDecisionDetectedAttributesItems :: SkippableInstreamVideo , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    impl ::google_field_selector::FieldSelector for CreativeServingDecisionDetectedAttributesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeServingDecisionDetectedAttributesItems {
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
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
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
    pub struct DestinationNotCrawlableEvidence {
        #[doc = "Approximate time of the crawl."]
        #[serde(
            rename = "crawlTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crawl_time: ::std::option::Option<String>,
        #[doc = "Destination URL that was attempted to be crawled."]
        #[serde(
            rename = "crawledUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crawled_url: ::std::option::Option<String>,
        #[doc = "Reason of destination not crawlable."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<crate::schemas::DestinationNotCrawlableEvidenceReason>,
    }
    impl ::google_field_selector::FieldSelector for DestinationNotCrawlableEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotCrawlableEvidence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DestinationNotCrawlableEvidenceReason {
        #[doc = "Default value that should never be used."]
        ReasonUnspecified,
        #[doc = "Crawler was disallowed by the site's robots exclusion file (e.g., robots.txt)."]
        RobotedDenied,
        #[doc = "Timed out reading site's robots exclusion file (e.g., robots.txt)."]
        TimeoutRobots,
        #[doc = "Unknown reason."]
        Unknown,
        #[doc = "Site's robots exclusion file (e.g., robots.txt) was unreachable."]
        UnreachableRobots,
    }
    impl DestinationNotCrawlableEvidenceReason {
        pub fn as_str(self) -> &'static str {
            match self {
                DestinationNotCrawlableEvidenceReason::ReasonUnspecified => "REASON_UNSPECIFIED",
                DestinationNotCrawlableEvidenceReason::RobotedDenied => "ROBOTED_DENIED",
                DestinationNotCrawlableEvidenceReason::TimeoutRobots => "TIMEOUT_ROBOTS",
                DestinationNotCrawlableEvidenceReason::Unknown => "UNKNOWN",
                DestinationNotCrawlableEvidenceReason::UnreachableRobots => "UNREACHABLE_ROBOTS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DestinationNotCrawlableEvidenceReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DestinationNotCrawlableEvidenceReason {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DestinationNotCrawlableEvidenceReason, ()> {
            Ok(match s {
                "REASON_UNSPECIFIED" => DestinationNotCrawlableEvidenceReason::ReasonUnspecified,
                "ROBOTED_DENIED" => DestinationNotCrawlableEvidenceReason::RobotedDenied,
                "TIMEOUT_ROBOTS" => DestinationNotCrawlableEvidenceReason::TimeoutRobots,
                "UNKNOWN" => DestinationNotCrawlableEvidenceReason::Unknown,
                "UNREACHABLE_ROBOTS" => DestinationNotCrawlableEvidenceReason::UnreachableRobots,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DestinationNotCrawlableEvidenceReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DestinationNotCrawlableEvidenceReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DestinationNotCrawlableEvidenceReason {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "REASON_UNSPECIFIED" => DestinationNotCrawlableEvidenceReason::ReasonUnspecified,
                "ROBOTED_DENIED" => DestinationNotCrawlableEvidenceReason::RobotedDenied,
                "TIMEOUT_ROBOTS" => DestinationNotCrawlableEvidenceReason::TimeoutRobots,
                "UNKNOWN" => DestinationNotCrawlableEvidenceReason::Unknown,
                "UNREACHABLE_ROBOTS" => DestinationNotCrawlableEvidenceReason::UnreachableRobots,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DestinationNotCrawlableEvidenceReason {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotCrawlableEvidenceReason {
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
    pub struct DestinationNotWorkingEvidence {
        #[doc = "DNS lookup errors."]
        #[serde(
            rename = "dnsError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dns_error: ::std::option::Option<crate::schemas::DestinationNotWorkingEvidenceDnsError>,
        #[doc = "The full non-working URL."]
        #[serde(
            rename = "expandedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expanded_url: ::std::option::Option<String>,
        #[doc = "HTTP error code (e.g. 404 or 5xx)"]
        #[serde(
            rename = "httpError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_error: ::std::option::Option<i32>,
        #[doc = "Page was crawled successfully, but was detected as either a page with no content or an error page."]
        #[serde(
            rename = "invalidPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invalid_page:
            ::std::option::Option<crate::schemas::DestinationNotWorkingEvidenceInvalidPage>,
        #[doc = "Approximate time when the ad destination was last checked."]
        #[serde(
            rename = "lastCheckTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_check_time: ::std::option::Option<String>,
        #[doc = "Platform of the non-working URL."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<crate::schemas::DestinationNotWorkingEvidencePlatform>,
        #[doc = "HTTP redirect chain error."]
        #[serde(
            rename = "redirectionError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub redirection_error:
            ::std::option::Option<crate::schemas::DestinationNotWorkingEvidenceRedirectionError>,
        #[doc = "Rejected because of malformed URLs or invalid requests."]
        #[serde(
            rename = "urlRejected",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url_rejected:
            ::std::option::Option<crate::schemas::DestinationNotWorkingEvidenceUrlRejected>,
    }
    impl ::google_field_selector::FieldSelector for DestinationNotWorkingEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotWorkingEvidence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DestinationNotWorkingEvidenceDnsError {
        #[doc = "Default value that should never be used."]
        DnsErrorUnspecified,
        #[doc = "DNS name was not found."]
        ErrorDns,
        #[doc = "An internal issue occurred when Google's crawler tried to resolve the DNS entry. This is a Google-internal issue and may not be the result of an issue with the landing page."]
        GoogleCrawlerDnsIssue,
    }
    impl DestinationNotWorkingEvidenceDnsError {
        pub fn as_str(self) -> &'static str {
            match self {
                DestinationNotWorkingEvidenceDnsError::DnsErrorUnspecified => {
                    "DNS_ERROR_UNSPECIFIED"
                }
                DestinationNotWorkingEvidenceDnsError::ErrorDns => "ERROR_DNS",
                DestinationNotWorkingEvidenceDnsError::GoogleCrawlerDnsIssue => {
                    "GOOGLE_CRAWLER_DNS_ISSUE"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DestinationNotWorkingEvidenceDnsError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DestinationNotWorkingEvidenceDnsError {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DestinationNotWorkingEvidenceDnsError, ()> {
            Ok(match s {
                "DNS_ERROR_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceDnsError::DnsErrorUnspecified
                }
                "ERROR_DNS" => DestinationNotWorkingEvidenceDnsError::ErrorDns,
                "GOOGLE_CRAWLER_DNS_ISSUE" => {
                    DestinationNotWorkingEvidenceDnsError::GoogleCrawlerDnsIssue
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DestinationNotWorkingEvidenceDnsError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DestinationNotWorkingEvidenceDnsError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DestinationNotWorkingEvidenceDnsError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DNS_ERROR_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceDnsError::DnsErrorUnspecified
                }
                "ERROR_DNS" => DestinationNotWorkingEvidenceDnsError::ErrorDns,
                "GOOGLE_CRAWLER_DNS_ISSUE" => {
                    DestinationNotWorkingEvidenceDnsError::GoogleCrawlerDnsIssue
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
    impl ::google_field_selector::FieldSelector for DestinationNotWorkingEvidenceDnsError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotWorkingEvidenceDnsError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DestinationNotWorkingEvidenceInvalidPage {
        #[doc = "Page was empty or had an error."]
        EmptyOrErrorPage,
        #[doc = "Default value that should never be used."]
        InvalidPageUnspecified,
    }
    impl DestinationNotWorkingEvidenceInvalidPage {
        pub fn as_str(self) -> &'static str {
            match self {
                DestinationNotWorkingEvidenceInvalidPage::EmptyOrErrorPage => "EMPTY_OR_ERROR_PAGE",
                DestinationNotWorkingEvidenceInvalidPage::InvalidPageUnspecified => {
                    "INVALID_PAGE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DestinationNotWorkingEvidenceInvalidPage {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DestinationNotWorkingEvidenceInvalidPage {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<DestinationNotWorkingEvidenceInvalidPage, ()> {
            Ok(match s {
                "EMPTY_OR_ERROR_PAGE" => DestinationNotWorkingEvidenceInvalidPage::EmptyOrErrorPage,
                "INVALID_PAGE_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceInvalidPage::InvalidPageUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DestinationNotWorkingEvidenceInvalidPage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DestinationNotWorkingEvidenceInvalidPage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DestinationNotWorkingEvidenceInvalidPage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMPTY_OR_ERROR_PAGE" => DestinationNotWorkingEvidenceInvalidPage::EmptyOrErrorPage,
                "INVALID_PAGE_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceInvalidPage::InvalidPageUnspecified
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
    impl ::google_field_selector::FieldSelector for DestinationNotWorkingEvidenceInvalidPage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotWorkingEvidenceInvalidPage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DestinationNotWorkingEvidencePlatform {
        #[doc = "The Android platform."]
        Android,
        #[doc = "The iOS platform."]
        Ios,
        #[doc = "The personal computer platform."]
        PersonalComputer,
        #[doc = "Default value that should never be used."]
        PlatformUnspecified,
    }
    impl DestinationNotWorkingEvidencePlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                DestinationNotWorkingEvidencePlatform::Android => "ANDROID",
                DestinationNotWorkingEvidencePlatform::Ios => "IOS",
                DestinationNotWorkingEvidencePlatform::PersonalComputer => "PERSONAL_COMPUTER",
                DestinationNotWorkingEvidencePlatform::PlatformUnspecified => {
                    "PLATFORM_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DestinationNotWorkingEvidencePlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DestinationNotWorkingEvidencePlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DestinationNotWorkingEvidencePlatform, ()> {
            Ok(match s {
                "ANDROID" => DestinationNotWorkingEvidencePlatform::Android,
                "IOS" => DestinationNotWorkingEvidencePlatform::Ios,
                "PERSONAL_COMPUTER" => DestinationNotWorkingEvidencePlatform::PersonalComputer,
                "PLATFORM_UNSPECIFIED" => {
                    DestinationNotWorkingEvidencePlatform::PlatformUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DestinationNotWorkingEvidencePlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DestinationNotWorkingEvidencePlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DestinationNotWorkingEvidencePlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => DestinationNotWorkingEvidencePlatform::Android,
                "IOS" => DestinationNotWorkingEvidencePlatform::Ios,
                "PERSONAL_COMPUTER" => DestinationNotWorkingEvidencePlatform::PersonalComputer,
                "PLATFORM_UNSPECIFIED" => {
                    DestinationNotWorkingEvidencePlatform::PlatformUnspecified
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
    impl ::google_field_selector::FieldSelector for DestinationNotWorkingEvidencePlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotWorkingEvidencePlatform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DestinationNotWorkingEvidenceRedirectionError {
        #[doc = "Got a redirect but it was empty."]
        EmptyRedirect,
        #[doc = "Got a redirect but it was invalid."]
        InvalidRedirect,
        #[doc = "Unknown redirect error."]
        RedirectErrorUnknown,
        #[doc = "Default value that should never be used."]
        RedirectionErrorUnspecified,
        #[doc = "Too many redirect hops."]
        TooManyRedirects,
    }
    impl DestinationNotWorkingEvidenceRedirectionError {
        pub fn as_str(self) -> &'static str {
            match self {
                DestinationNotWorkingEvidenceRedirectionError::EmptyRedirect => "EMPTY_REDIRECT",
                DestinationNotWorkingEvidenceRedirectionError::InvalidRedirect => {
                    "INVALID_REDIRECT"
                }
                DestinationNotWorkingEvidenceRedirectionError::RedirectErrorUnknown => {
                    "REDIRECT_ERROR_UNKNOWN"
                }
                DestinationNotWorkingEvidenceRedirectionError::RedirectionErrorUnspecified => {
                    "REDIRECTION_ERROR_UNSPECIFIED"
                }
                DestinationNotWorkingEvidenceRedirectionError::TooManyRedirects => {
                    "TOO_MANY_REDIRECTS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DestinationNotWorkingEvidenceRedirectionError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DestinationNotWorkingEvidenceRedirectionError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<DestinationNotWorkingEvidenceRedirectionError, ()> {
            Ok(match s {
                "EMPTY_REDIRECT" => DestinationNotWorkingEvidenceRedirectionError::EmptyRedirect,
                "INVALID_REDIRECT" => {
                    DestinationNotWorkingEvidenceRedirectionError::InvalidRedirect
                }
                "REDIRECT_ERROR_UNKNOWN" => {
                    DestinationNotWorkingEvidenceRedirectionError::RedirectErrorUnknown
                }
                "REDIRECTION_ERROR_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceRedirectionError::RedirectionErrorUnspecified
                }
                "TOO_MANY_REDIRECTS" => {
                    DestinationNotWorkingEvidenceRedirectionError::TooManyRedirects
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DestinationNotWorkingEvidenceRedirectionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DestinationNotWorkingEvidenceRedirectionError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DestinationNotWorkingEvidenceRedirectionError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMPTY_REDIRECT" => DestinationNotWorkingEvidenceRedirectionError::EmptyRedirect,
                "INVALID_REDIRECT" => {
                    DestinationNotWorkingEvidenceRedirectionError::InvalidRedirect
                }
                "REDIRECT_ERROR_UNKNOWN" => {
                    DestinationNotWorkingEvidenceRedirectionError::RedirectErrorUnknown
                }
                "REDIRECTION_ERROR_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceRedirectionError::RedirectionErrorUnspecified
                }
                "TOO_MANY_REDIRECTS" => {
                    DestinationNotWorkingEvidenceRedirectionError::TooManyRedirects
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
    impl ::google_field_selector::FieldSelector for DestinationNotWorkingEvidenceRedirectionError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotWorkingEvidenceRedirectionError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DestinationNotWorkingEvidenceUrlRejected {
        #[doc = "URL rejected because of a malformed request."]
        BadRequest,
        #[doc = "URL rejected because of a malformed URL."]
        MalformedUrl,
        #[doc = "URL rejected because of unknown reason."]
        UrlRejectedUnknown,
        #[doc = "Default value that should never be used."]
        UrlRejectedUnspecified,
    }
    impl DestinationNotWorkingEvidenceUrlRejected {
        pub fn as_str(self) -> &'static str {
            match self {
                DestinationNotWorkingEvidenceUrlRejected::BadRequest => "BAD_REQUEST",
                DestinationNotWorkingEvidenceUrlRejected::MalformedUrl => "MALFORMED_URL",
                DestinationNotWorkingEvidenceUrlRejected::UrlRejectedUnknown => {
                    "URL_REJECTED_UNKNOWN"
                }
                DestinationNotWorkingEvidenceUrlRejected::UrlRejectedUnspecified => {
                    "URL_REJECTED_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for DestinationNotWorkingEvidenceUrlRejected {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DestinationNotWorkingEvidenceUrlRejected {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<DestinationNotWorkingEvidenceUrlRejected, ()> {
            Ok(match s {
                "BAD_REQUEST" => DestinationNotWorkingEvidenceUrlRejected::BadRequest,
                "MALFORMED_URL" => DestinationNotWorkingEvidenceUrlRejected::MalformedUrl,
                "URL_REJECTED_UNKNOWN" => {
                    DestinationNotWorkingEvidenceUrlRejected::UrlRejectedUnknown
                }
                "URL_REJECTED_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceUrlRejected::UrlRejectedUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DestinationNotWorkingEvidenceUrlRejected {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DestinationNotWorkingEvidenceUrlRejected {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DestinationNotWorkingEvidenceUrlRejected {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BAD_REQUEST" => DestinationNotWorkingEvidenceUrlRejected::BadRequest,
                "MALFORMED_URL" => DestinationNotWorkingEvidenceUrlRejected::MalformedUrl,
                "URL_REJECTED_UNKNOWN" => {
                    DestinationNotWorkingEvidenceUrlRejected::UrlRejectedUnknown
                }
                "URL_REJECTED_UNSPECIFIED" => {
                    DestinationNotWorkingEvidenceUrlRejected::UrlRejectedUnspecified
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
    impl ::google_field_selector::FieldSelector for DestinationNotWorkingEvidenceUrlRejected {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationNotWorkingEvidenceUrlRejected {
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
    pub struct DestinationUrlEvidence {
        #[doc = "The full landing page URL of the destination."]
        #[serde(
            rename = "destinationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DestinationUrlEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationUrlEvidence {
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
    pub struct DomainCallEvidence {
        #[doc = "Breakdown of the most frequent domains called via HTTP by the creative."]
        #[serde(
            rename = "topHttpCallDomains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_http_call_domains: ::std::option::Option<Vec<crate::schemas::DomainCalls>>,
        #[doc = "The total number of HTTP calls made by the creative, including but not limited to the number of calls in the top_http_call_domains."]
        #[serde(
            rename = "totalHttpCallCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_http_call_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DomainCallEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DomainCallEvidence {
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
    pub struct DomainCalls {
        #[doc = "The domain name."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "Number of HTTP calls made to the domain."]
        #[serde(
            rename = "httpCallCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_call_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DomainCalls {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DomainCalls {
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
    pub struct DownloadSizeEvidence {
        #[doc = "Download size broken down by URLs with the top download size."]
        #[serde(
            rename = "topUrlDownloadSizeBreakdowns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_url_download_size_breakdowns:
            ::std::option::Option<Vec<crate::schemas::UrlDownloadSize>>,
        #[doc = "Total download size (in kilobytes) for all the resources in the creative."]
        #[serde(
            rename = "totalDownloadSizeKb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_download_size_kb: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DownloadSizeEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DownloadSizeEvidence {
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
    pub struct Empty {}
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
    pub struct Endpoint {
        #[doc = "The protocol that the bidder endpoint is using."]
        #[serde(
            rename = "bidProtocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bid_protocol: ::std::option::Option<crate::schemas::EndpointBidProtocol>,
        #[doc = "The maximum number of queries per second allowed to be sent to this server."]
        #[serde(
            rename = "maximumQps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_qps: ::std::option::Option<i64>,
        #[doc = "Output only. Name of the endpoint resource that must follow the pattern `bidders/{bidderAccountId}/endpoints/{endpointId}`, where {bidderAccountId} is the account ID of the bidder who operates this endpoint, and {endpointId} is a unique ID assigned by the server."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The trading location that bid requests should be sent from. See https://developers.google.com/authorized-buyers/rtb/peer-guide#trading-locations for further information."]
        #[serde(
            rename = "tradingLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trading_location: ::std::option::Option<crate::schemas::EndpointTradingLocation>,
        #[doc = "Output only. The URL that bid requests should be sent to."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Endpoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Endpoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EndpointBidProtocol {
        #[doc = "Placeholder for undefined bid protocol. This value should not be used."]
        BidProtocolUnspecified,
        #[doc = "Google RTB protocol / Protobuf encoding."]
        GoogleRtb,
        #[doc = "OpenRTB / JSON encoding, specification version 2.2."]
        Openrtb22,
        #[doc = "OpenRTB / JSON encoding, specification version 2.3."]
        Openrtb23,
        #[doc = "OpenRTB / JSON encoding, specification version 2.4."]
        Openrtb24,
        #[doc = "OpenRTB / JSON encoding, specification version 2.5."]
        Openrtb25,
        #[doc = "OpenRTB / Protobuf encoding, specification version 2.3."]
        OpenrtbProtobuf23,
        #[doc = "OpenRTB / Protobuf encoding, specification version 2.4."]
        OpenrtbProtobuf24,
        #[doc = "OpenRTB / Protobuf encoding, specification version 2.5."]
        OpenrtbProtobuf25,
    }
    impl EndpointBidProtocol {
        pub fn as_str(self) -> &'static str {
            match self {
                EndpointBidProtocol::BidProtocolUnspecified => "BID_PROTOCOL_UNSPECIFIED",
                EndpointBidProtocol::GoogleRtb => "GOOGLE_RTB",
                EndpointBidProtocol::Openrtb22 => "OPENRTB_2_2",
                EndpointBidProtocol::Openrtb23 => "OPENRTB_2_3",
                EndpointBidProtocol::Openrtb24 => "OPENRTB_2_4",
                EndpointBidProtocol::Openrtb25 => "OPENRTB_2_5",
                EndpointBidProtocol::OpenrtbProtobuf23 => "OPENRTB_PROTOBUF_2_3",
                EndpointBidProtocol::OpenrtbProtobuf24 => "OPENRTB_PROTOBUF_2_4",
                EndpointBidProtocol::OpenrtbProtobuf25 => "OPENRTB_PROTOBUF_2_5",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EndpointBidProtocol {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EndpointBidProtocol {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EndpointBidProtocol, ()> {
            Ok(match s {
                "BID_PROTOCOL_UNSPECIFIED" => EndpointBidProtocol::BidProtocolUnspecified,
                "GOOGLE_RTB" => EndpointBidProtocol::GoogleRtb,
                "OPENRTB_2_2" => EndpointBidProtocol::Openrtb22,
                "OPENRTB_2_3" => EndpointBidProtocol::Openrtb23,
                "OPENRTB_2_4" => EndpointBidProtocol::Openrtb24,
                "OPENRTB_2_5" => EndpointBidProtocol::Openrtb25,
                "OPENRTB_PROTOBUF_2_3" => EndpointBidProtocol::OpenrtbProtobuf23,
                "OPENRTB_PROTOBUF_2_4" => EndpointBidProtocol::OpenrtbProtobuf24,
                "OPENRTB_PROTOBUF_2_5" => EndpointBidProtocol::OpenrtbProtobuf25,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EndpointBidProtocol {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EndpointBidProtocol {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EndpointBidProtocol {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BID_PROTOCOL_UNSPECIFIED" => EndpointBidProtocol::BidProtocolUnspecified,
                "GOOGLE_RTB" => EndpointBidProtocol::GoogleRtb,
                "OPENRTB_2_2" => EndpointBidProtocol::Openrtb22,
                "OPENRTB_2_3" => EndpointBidProtocol::Openrtb23,
                "OPENRTB_2_4" => EndpointBidProtocol::Openrtb24,
                "OPENRTB_2_5" => EndpointBidProtocol::Openrtb25,
                "OPENRTB_PROTOBUF_2_3" => EndpointBidProtocol::OpenrtbProtobuf23,
                "OPENRTB_PROTOBUF_2_4" => EndpointBidProtocol::OpenrtbProtobuf24,
                "OPENRTB_PROTOBUF_2_5" => EndpointBidProtocol::OpenrtbProtobuf25,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EndpointBidProtocol {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EndpointBidProtocol {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EndpointTradingLocation {
        #[doc = "The Asia trading location."]
        Asia,
        #[doc = "The European trading location."]
        Europe,
        #[doc = "A placeholder for an undefined trading region. This value should not be used."]
        TradingLocationUnspecified,
        #[doc = "The Eastern US trading location."]
        UsEast,
        #[doc = "The Western US trading location."]
        UsWest,
    }
    impl EndpointTradingLocation {
        pub fn as_str(self) -> &'static str {
            match self {
                EndpointTradingLocation::Asia => "ASIA",
                EndpointTradingLocation::Europe => "EUROPE",
                EndpointTradingLocation::TradingLocationUnspecified => {
                    "TRADING_LOCATION_UNSPECIFIED"
                }
                EndpointTradingLocation::UsEast => "US_EAST",
                EndpointTradingLocation::UsWest => "US_WEST",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EndpointTradingLocation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EndpointTradingLocation {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EndpointTradingLocation, ()> {
            Ok(match s {
                "ASIA" => EndpointTradingLocation::Asia,
                "EUROPE" => EndpointTradingLocation::Europe,
                "TRADING_LOCATION_UNSPECIFIED" => {
                    EndpointTradingLocation::TradingLocationUnspecified
                }
                "US_EAST" => EndpointTradingLocation::UsEast,
                "US_WEST" => EndpointTradingLocation::UsWest,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EndpointTradingLocation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EndpointTradingLocation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EndpointTradingLocation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASIA" => EndpointTradingLocation::Asia,
                "EUROPE" => EndpointTradingLocation::Europe,
                "TRADING_LOCATION_UNSPECIFIED" => {
                    EndpointTradingLocation::TradingLocationUnspecified
                }
                "US_EAST" => EndpointTradingLocation::UsEast,
                "US_WEST" => EndpointTradingLocation::UsWest,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EndpointTradingLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EndpointTradingLocation {
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
    pub struct GetRemarketingTagResponse {
        #[doc = "A HTML tag that can be placed on the advertiser's page to add users to a user list. For more information and code samples on using snippet on your website refer to [Tag your site for remarketing](https://support.google.com/google-ads/answer/2476688)."]
        #[serde(
            rename = "snippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snippet: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GetRemarketingTagResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetRemarketingTagResponse {
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
    pub struct HtmlContent {
        #[doc = "The height of the HTML snippet in pixels. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The HTML snippet that displays the ad when inserted in the web page."]
        #[serde(
            rename = "snippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snippet: ::std::option::Option<String>,
        #[doc = "The width of the HTML snippet in pixels. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for HtmlContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HtmlContent {
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
    pub struct HttpCallEvidence {
        #[doc = "URLs of HTTP calls made by the creative."]
        #[serde(
            rename = "urls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub urls: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for HttpCallEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HttpCallEvidence {
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
    pub struct HttpCookieEvidence {
        #[doc = "Names of cookies that violate Google policies. For TOO_MANY_COOKIES policy, this will be the cookie names of top domains with the largest number of cookies. For other policies, this will be all the cookie names that violate the policy."]
        #[serde(
            rename = "cookieNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cookie_names: ::std::option::Option<Vec<String>>,
        #[doc = "The largest number of cookies set by a creative. If this field is set, cookie_names above will be set to the cookie names of top domains with the largest number of cookies. This field will only be set for TOO_MANY_COOKIES policy."]
        #[serde(
            rename = "maxCookieCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_cookie_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for HttpCookieEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HttpCookieEvidence {
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
    pub struct Image {
        #[doc = "Image height in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The URL of the image."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[doc = "Image width in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Image {
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
    pub struct ListBiddersResponse {
        #[doc = "List of bidders."]
        #[serde(
            rename = "bidders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bidders: ::std::option::Option<Vec<crate::schemas::Bidder>>,
        #[doc = "A token which can be passed to a subsequent call to the `ListBidders` method to retrieve the next page of results in ListBiddersRequest.pageToken."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListBiddersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListBiddersResponse {
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
    pub struct ListBuyersResponse {
        #[doc = "List of buyers."]
        #[serde(
            rename = "buyers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyers: ::std::option::Option<Vec<crate::schemas::Buyer>>,
        #[doc = "A token which can be passed to a subsequent call to the `ListBuyers` method to retrieve the next page of results in ListBuyersRequest.pageToken."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListBuyersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListBuyersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListCreativesResponse {
        #[doc = "The list of creatives."]
        #[serde(
            rename = "creatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creatives: ::std::option::Option<Vec<crate::schemas::Creative>>,
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListCreativesRequest.pageToken field in the subsequent call to the `ListCreatives` method to retrieve the next page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListCreativesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCreativesResponse {
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
    pub struct ListEndpointsResponse {
        #[doc = "List of bidder endpoints."]
        #[serde(
            rename = "endpoints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub endpoints: ::std::option::Option<Vec<crate::schemas::Endpoint>>,
        #[doc = "A token which can be passed to a subsequent call to the `ListEndpoints` method to retrieve the next page of results in ListEndpointsRequest.pageToken."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListEndpointsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListEndpointsResponse {
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
    pub struct ListPretargetingConfigsResponse {
        #[doc = "A token which can be passed to a subsequent call to the `ListPretargetingConfigs` method to retrieve the next page of results in ListPretargetingConfigsRequest.pageToken."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of pretargeting configurations."]
        #[serde(
            rename = "pretargetingConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pretargeting_configs: ::std::option::Option<Vec<crate::schemas::PretargetingConfig>>,
    }
    impl ::google_field_selector::FieldSelector for ListPretargetingConfigsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPretargetingConfigsResponse {
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
    pub struct ListUserListsResponse {
        #[doc = "The continuation page token to send back to the server in a subsequent request. Due to a currently known issue, it is recommended that the caller keep invoking the list method till the time a next page token is not returned (even if the result set is empty)."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of user lists from the search."]
        #[serde(
            rename = "userLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_lists: ::std::option::Option<Vec<crate::schemas::UserList>>,
    }
    impl ::google_field_selector::FieldSelector for ListUserListsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUserListsResponse {
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
    pub struct MediaFile {
        #[doc = "Bitrate of the video file, in Kbps. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "bitrate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bitrate: ::std::option::Option<i64>,
        #[doc = "The MIME type of this media file. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<crate::schemas::MediaFileMimeType>,
    }
    impl ::google_field_selector::FieldSelector for MediaFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediaFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MediaFileMimeType {
        #[doc = "JavaScript (used for VPAID ads)."]
        MimeApplicationJavascript,
        #[doc = "DASH."]
        MimeApplicationMpegdash,
        #[doc = "HLS/M3U8."]
        MimeApplicationMpegurl,
        #[doc = "Silverlight (used for VPAID ads)."]
        MimeApplicationSilverlight,
        #[doc = "Properties of VAST served by consumer survey."]
        MimeApplicationSurvey,
        #[doc = "Shockwave Flash (used for VPAID ads)."]
        MimeApplicationSwf,
        #[doc = "MPEG-3 audio format."]
        MimeAudioMp3,
        #[doc = "MPEG-4 audio format."]
        MimeAudioMp4A,
        #[doc = "Ogg audio format"]
        MimeAudioOgg,
        #[doc = "3GPP container format used on 3G phones."]
        MimeVideo3Gpp,
        #[doc = "Quicktime container format."]
        MimeVideoMov,
        #[doc = "MPEG-4 container typically with H.264 codec."]
        MimeVideoMp4,
        #[doc = "Ogg container assuming Theora codec."]
        MimeVideoOgg,
        #[doc = "WebM container assuming VP9 codec."]
        MimeVideoWebm,
        #[doc = "Windows Media Video Codec."]
        MimeVideoXMsWmv,
        #[doc = "Flash container."]
        MimeVideoXflv,
        #[doc = "Video files hosted on YouTube."]
        MimeVideoYtHosted,
        #[doc = "Default value that should never be used."]
        VideoMimeTypeUnspecified,
    }
    impl MediaFileMimeType {
        pub fn as_str(self) -> &'static str {
            match self {
                MediaFileMimeType::MimeApplicationJavascript => "MIME_APPLICATION_JAVASCRIPT",
                MediaFileMimeType::MimeApplicationMpegdash => "MIME_APPLICATION_MPEGDASH",
                MediaFileMimeType::MimeApplicationMpegurl => "MIME_APPLICATION_MPEGURL",
                MediaFileMimeType::MimeApplicationSilverlight => "MIME_APPLICATION_SILVERLIGHT",
                MediaFileMimeType::MimeApplicationSurvey => "MIME_APPLICATION_SURVEY",
                MediaFileMimeType::MimeApplicationSwf => "MIME_APPLICATION_SWF",
                MediaFileMimeType::MimeAudioMp3 => "MIME_AUDIO_MP3",
                MediaFileMimeType::MimeAudioMp4A => "MIME_AUDIO_MP4A",
                MediaFileMimeType::MimeAudioOgg => "MIME_AUDIO_OGG",
                MediaFileMimeType::MimeVideo3Gpp => "MIME_VIDEO_3GPP",
                MediaFileMimeType::MimeVideoMov => "MIME_VIDEO_MOV",
                MediaFileMimeType::MimeVideoMp4 => "MIME_VIDEO_MP4",
                MediaFileMimeType::MimeVideoOgg => "MIME_VIDEO_OGG",
                MediaFileMimeType::MimeVideoWebm => "MIME_VIDEO_WEBM",
                MediaFileMimeType::MimeVideoXMsWmv => "MIME_VIDEO_X_MS_WMV",
                MediaFileMimeType::MimeVideoXflv => "MIME_VIDEO_XFLV",
                MediaFileMimeType::MimeVideoYtHosted => "MIME_VIDEO_YT_HOSTED",
                MediaFileMimeType::VideoMimeTypeUnspecified => "VIDEO_MIME_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MediaFileMimeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MediaFileMimeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MediaFileMimeType, ()> {
            Ok(match s {
                "MIME_APPLICATION_JAVASCRIPT" => MediaFileMimeType::MimeApplicationJavascript,
                "MIME_APPLICATION_MPEGDASH" => MediaFileMimeType::MimeApplicationMpegdash,
                "MIME_APPLICATION_MPEGURL" => MediaFileMimeType::MimeApplicationMpegurl,
                "MIME_APPLICATION_SILVERLIGHT" => MediaFileMimeType::MimeApplicationSilverlight,
                "MIME_APPLICATION_SURVEY" => MediaFileMimeType::MimeApplicationSurvey,
                "MIME_APPLICATION_SWF" => MediaFileMimeType::MimeApplicationSwf,
                "MIME_AUDIO_MP3" => MediaFileMimeType::MimeAudioMp3,
                "MIME_AUDIO_MP4A" => MediaFileMimeType::MimeAudioMp4A,
                "MIME_AUDIO_OGG" => MediaFileMimeType::MimeAudioOgg,
                "MIME_VIDEO_3GPP" => MediaFileMimeType::MimeVideo3Gpp,
                "MIME_VIDEO_MOV" => MediaFileMimeType::MimeVideoMov,
                "MIME_VIDEO_MP4" => MediaFileMimeType::MimeVideoMp4,
                "MIME_VIDEO_OGG" => MediaFileMimeType::MimeVideoOgg,
                "MIME_VIDEO_WEBM" => MediaFileMimeType::MimeVideoWebm,
                "MIME_VIDEO_X_MS_WMV" => MediaFileMimeType::MimeVideoXMsWmv,
                "MIME_VIDEO_XFLV" => MediaFileMimeType::MimeVideoXflv,
                "MIME_VIDEO_YT_HOSTED" => MediaFileMimeType::MimeVideoYtHosted,
                "VIDEO_MIME_TYPE_UNSPECIFIED" => MediaFileMimeType::VideoMimeTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MediaFileMimeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MediaFileMimeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MediaFileMimeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MIME_APPLICATION_JAVASCRIPT" => MediaFileMimeType::MimeApplicationJavascript,
                "MIME_APPLICATION_MPEGDASH" => MediaFileMimeType::MimeApplicationMpegdash,
                "MIME_APPLICATION_MPEGURL" => MediaFileMimeType::MimeApplicationMpegurl,
                "MIME_APPLICATION_SILVERLIGHT" => MediaFileMimeType::MimeApplicationSilverlight,
                "MIME_APPLICATION_SURVEY" => MediaFileMimeType::MimeApplicationSurvey,
                "MIME_APPLICATION_SWF" => MediaFileMimeType::MimeApplicationSwf,
                "MIME_AUDIO_MP3" => MediaFileMimeType::MimeAudioMp3,
                "MIME_AUDIO_MP4A" => MediaFileMimeType::MimeAudioMp4A,
                "MIME_AUDIO_OGG" => MediaFileMimeType::MimeAudioOgg,
                "MIME_VIDEO_3GPP" => MediaFileMimeType::MimeVideo3Gpp,
                "MIME_VIDEO_MOV" => MediaFileMimeType::MimeVideoMov,
                "MIME_VIDEO_MP4" => MediaFileMimeType::MimeVideoMp4,
                "MIME_VIDEO_OGG" => MediaFileMimeType::MimeVideoOgg,
                "MIME_VIDEO_WEBM" => MediaFileMimeType::MimeVideoWebm,
                "MIME_VIDEO_X_MS_WMV" => MediaFileMimeType::MimeVideoXMsWmv,
                "MIME_VIDEO_XFLV" => MediaFileMimeType::MimeVideoXflv,
                "MIME_VIDEO_YT_HOSTED" => MediaFileMimeType::MimeVideoYtHosted,
                "VIDEO_MIME_TYPE_UNSPECIFIED" => MediaFileMimeType::VideoMimeTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MediaFileMimeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediaFileMimeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NativeContent {
        #[doc = "The name of the advertiser or sponsor, to be displayed in the ad creative."]
        #[serde(
            rename = "advertiserName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advertiser_name: ::std::option::Option<String>,
        #[doc = "The app icon, for app download ads."]
        #[serde(
            rename = "appIcon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_icon: ::std::option::Option<crate::schemas::Image>,
        #[doc = "A long description of the ad."]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<String>,
        #[doc = "A label for the button that the user is supposed to click."]
        #[serde(
            rename = "callToAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub call_to_action: ::std::option::Option<String>,
        #[doc = "The URL that the browser/SDK will load when the user clicks the ad."]
        #[serde(
            rename = "clickLinkUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub click_link_url: ::std::option::Option<String>,
        #[doc = "The URL to use for click tracking."]
        #[serde(
            rename = "clickTrackingUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub click_tracking_url: ::std::option::Option<String>,
        #[doc = "A short title for the ad."]
        #[serde(
            rename = "headline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headline: ::std::option::Option<String>,
        #[doc = "A large image."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::Image>,
        #[doc = "A smaller image, for the advertiser's logo."]
        #[serde(
            rename = "logo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo: ::std::option::Option<crate::schemas::Image>,
        #[doc = "The price of the promoted app including currency info."]
        #[serde(
            rename = "priceDisplayText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price_display_text: ::std::option::Option<String>,
        #[doc = "The app rating in the app store. Must be in the range [0-5]."]
        #[serde(
            rename = "starRating",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub star_rating: ::std::option::Option<f64>,
        #[doc = "The URL to fetch a native video ad."]
        #[serde(
            rename = "videoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NativeContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NativeContent {
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
    pub struct NumericTargetingDimension {
        #[doc = "The IDs excluded in a configuration."]
        #[serde(
            rename = "excludedIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "The IDs included in a configuration."]
        #[serde(
            rename = "includedIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_ids: ::std::option::Option<Vec<i64>>,
    }
    impl ::google_field_selector::FieldSelector for NumericTargetingDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumericTargetingDimension {
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
    pub struct OpenUserListRequest {}
    impl ::google_field_selector::FieldSelector for OpenUserListRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OpenUserListRequest {
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
    pub struct PolicyCompliance {
        #[doc = "Serving status for the given transaction type (e.g., open auction, deals) or region (e.g., China, Russia). Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::PolicyComplianceStatus>,
        #[doc = "Topics related to the policy compliance for this transaction type (e.g., open auction, deals) or region (e.g., China, Russia). Topics may be present only if status is DISAPPROVED."]
        #[serde(
            rename = "topics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub topics: ::std::option::Option<Vec<crate::schemas::PolicyTopicEntry>>,
    }
    impl ::google_field_selector::FieldSelector for PolicyCompliance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyCompliance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PolicyComplianceStatus {
        #[doc = "Creative is approved."]
        Approved,
        #[doc = "Creative cannot serve."]
        Disapproved,
        #[doc = "Creative is pending review."]
        PendingReview,
        #[doc = "Default value that should never be used."]
        StatusUnspecified,
    }
    impl PolicyComplianceStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                PolicyComplianceStatus::Approved => "APPROVED",
                PolicyComplianceStatus::Disapproved => "DISAPPROVED",
                PolicyComplianceStatus::PendingReview => "PENDING_REVIEW",
                PolicyComplianceStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PolicyComplianceStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PolicyComplianceStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PolicyComplianceStatus, ()> {
            Ok(match s {
                "APPROVED" => PolicyComplianceStatus::Approved,
                "DISAPPROVED" => PolicyComplianceStatus::Disapproved,
                "PENDING_REVIEW" => PolicyComplianceStatus::PendingReview,
                "STATUS_UNSPECIFIED" => PolicyComplianceStatus::StatusUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PolicyComplianceStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PolicyComplianceStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PolicyComplianceStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPROVED" => PolicyComplianceStatus::Approved,
                "DISAPPROVED" => PolicyComplianceStatus::Disapproved,
                "PENDING_REVIEW" => PolicyComplianceStatus::PendingReview,
                "STATUS_UNSPECIFIED" => PolicyComplianceStatus::StatusUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PolicyComplianceStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyComplianceStatus {
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
    pub struct PolicyTopicEntry {
        #[doc = "Pieces of evidence associated with this policy topic entry."]
        #[serde(
            rename = "evidences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub evidences: ::std::option::Option<Vec<crate::schemas::PolicyTopicEvidence>>,
        #[doc = "URL of the help center article describing this policy topic."]
        #[serde(
            rename = "helpCenterUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub help_center_url: ::std::option::Option<String>,
        #[doc = "Policy topic this entry refers to. For example, \"ALCOHOL\", \"TRADEMARKS_IN_AD_TEXT\", or \"DESTINATION_NOT_WORKING\". The set of possible policy topics is not fixed for a particular API version and may change at any time. Can be used to filter the response of the creatives.list method"]
        #[serde(
            rename = "policyTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_topic: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PolicyTopicEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyTopicEntry {
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
    pub struct PolicyTopicEvidence {
        #[doc = "The creative's destination URL was not crawlable by Google."]
        #[serde(
            rename = "destinationNotCrawlable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_not_crawlable:
            ::std::option::Option<crate::schemas::DestinationNotCrawlableEvidence>,
        #[doc = "The creative's destination URL did not function properly or was incorrectly set up."]
        #[serde(
            rename = "destinationNotWorking",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_not_working:
            ::std::option::Option<crate::schemas::DestinationNotWorkingEvidence>,
        #[doc = "URL of the actual landing page."]
        #[serde(
            rename = "destinationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_url: ::std::option::Option<crate::schemas::DestinationUrlEvidence>,
        #[doc = "Number of HTTP calls made by the creative, broken down by domain."]
        #[serde(
            rename = "domainCall",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_call: ::std::option::Option<crate::schemas::DomainCallEvidence>,
        #[doc = "Total download size and URL-level download size breakdown for resources in a creative."]
        #[serde(
            rename = "downloadSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_size: ::std::option::Option<crate::schemas::DownloadSizeEvidence>,
        #[doc = "HTTP calls made by the creative that resulted in policy violations."]
        #[serde(
            rename = "httpCall",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_call: ::std::option::Option<crate::schemas::HttpCallEvidence>,
        #[doc = "Evidence for HTTP cookie-related policy violations."]
        #[serde(
            rename = "httpCookie",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_cookie: ::std::option::Option<crate::schemas::HttpCookieEvidence>,
    }
    impl ::google_field_selector::FieldSelector for PolicyTopicEvidence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyTopicEvidence {
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
    pub struct PretargetingConfig {
        #[doc = "Targeting modes included by this configuration. A bid request must allow all the specified targeting modes. An unset value allows all bid requests to be sent, regardless of which targeting modes they allow."]
        #[serde(
            rename = "allowedUserTargetingModes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_user_targeting_modes: ::std::option::Option<
            Vec<crate::schemas::PretargetingConfigAllowedUserTargetingModesItems>,
        >,
        #[doc = "Targeting on a subset of app inventory. If APP is listed in targeted_environments, the specified targeting is applied. A maximum of 30,000 app IDs can be targeted. An unset value for targeting allows all app-based bid requests to be sent. Apps can either be targeting positively (bid requests will be sent only if the destination app is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination app is not listed in the targeting dimension)."]
        #[serde(
            rename = "appTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_targeting: ::std::option::Option<crate::schemas::AppTargeting>,
        #[doc = "Output only. The identifier that corresponds to this pretargeting configuration that helps buyers track and attribute their spend across their own arbitrary divisions. If a bid request matches more than one configuration, the buyer chooses which billing_id to attribute each of their bids."]
        #[serde(
            rename = "billingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub billing_id: ::std::option::Option<i64>,
        #[doc = "The diplay name associated with this configuration. This name must be unique among all the pretargeting configurations a bidder has."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The sensitive content category label IDs excluded in this configuration. Bid requests for inventory with any of the specified content label IDs will not be sent. Refer to this file https://storage.googleapis.com/adx-rtb-dictionaries/content-labels.txt for category IDs."]
        #[serde(
            rename = "excludedContentLabelIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_content_label_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "The geos included or excluded in this configuration defined in https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv"]
        #[serde(
            rename = "geoTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo_targeting: ::std::option::Option<crate::schemas::NumericTargetingDimension>,
        #[doc = "Creative dimensions included by this configuration. Only bid requests eligible for at least one of the specified creative dimensions will be sent. An unset value allows all bid requests to be sent, regardless of creative dimension."]
        #[serde(
            rename = "includedCreativeDimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_creative_dimensions:
            ::std::option::Option<Vec<crate::schemas::CreativeDimensions>>,
        #[doc = "Environments that are being included. Bid requests will not be sent for a given environment if it is not included. Further restrictions can be applied to included environments to target only a subset of its inventory. An unset value includes all environments."]
        #[serde(
            rename = "includedEnvironments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_environments:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigIncludedEnvironmentsItems>>,
        #[doc = "Creative formats included by this configuration. Only bid requests eligible for at least one of the specified creative formats will be sent. An unset value will allow all bid requests to be sent, regardless of format."]
        #[serde(
            rename = "includedFormats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_formats:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigIncludedFormatsItems>>,
        #[doc = "The languages included in this configuration, represented by their language code. See https://developers.google.com/adwords/api/docs/appendix/languagecodes."]
        #[serde(
            rename = "includedLanguages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_languages: ::std::option::Option<Vec<String>>,
        #[doc = "The mobile operating systems included in this configuration as defined in https://storage.googleapis.com/adx-rtb-dictionaries/mobile-os.csv"]
        #[serde(
            rename = "includedMobileOperatingSystemIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_mobile_operating_system_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "The platforms included by this configration. Bid requests for devices with the specified platform types will be sent. An unset value allows all bid requests to be sent, regardless of platform."]
        #[serde(
            rename = "includedPlatforms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_platforms:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigIncludedPlatformsItems>>,
        #[doc = "User identifier types included in this configuration. At least one of the user identifier types specified in this list must be available for the bid request to be sent."]
        #[serde(
            rename = "includedUserIdTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_user_id_types:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigIncludedUserIdTypesItems>>,
        #[doc = "The interstitial targeting specified for this configuration. The unset value will allow bid requests to be sent regardless of whether they are for interstitials or not."]
        #[serde(
            rename = "interstitialTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interstitial_targeting:
            ::std::option::Option<crate::schemas::PretargetingConfigInterstitialTargeting>,
        #[doc = "Output only. Existing included or excluded geos that are invalid. Previously targeted geos may become invalid due to privacy restrictions."]
        #[serde(
            rename = "invalidGeoIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invalid_geo_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "The maximum QPS threshold for this configuration. The bidder should receive no more than this number of bid requests matching this configuration per second across all their bidding endpoints among all trading locations. Further information available at https://developers.google.com/authorized-buyers/rtb/peer-guide"]
        #[serde(
            rename = "maximumQps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_qps: ::std::option::Option<i64>,
        #[doc = "The targeted minimum viewability decile, ranging in values [0, 10]. A value of 5 means that the configuration will only match adslots for which we predict at least 50% viewability. Values > 10 will be rounded down to 10. An unset value or a value of 0 indicates that bid requests will be sent regardless of viewability."]
        #[serde(
            rename = "minimumViewabilityDecile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum_viewability_decile: ::std::option::Option<i32>,
        #[doc = "Output only. Name of the pretargeting configuration that must follow the pattern `bidders/{bidder_account_id}/pretargetingConfigs/{config_id}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Targeting on a subset of publisher inventory. Publishers can either be targeted positively (bid requests will be sent only if the publisher is listed in the targeting dimension) or negatively (bid requests will be sent only if the publisher is not listed in the targeting dimension). A maximum of 10,000 publisher IDs can be targeted. Publisher IDs are found in [ads.txt](https://iabtechlab.com/ads-txt/) / [app-ads.txt](https://iabtechlab.com/app-ads-txt/) and in bid requests in the `BidRequest.publisher_id` field on the [Google RTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) or the `BidRequest.site.publisher.id` / `BidRequest.app.publisher.id` field on the [OpenRTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto)."]
        #[serde(
            rename = "publisherTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_targeting: ::std::option::Option<crate::schemas::StringTargetingDimension>,
        #[doc = "Output only. The state of this pretargeting configuration."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::PretargetingConfigState>,
        #[doc = "The remarketing lists included or excluded in this configuration as defined in UserList."]
        #[serde(
            rename = "userListTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_list_targeting: ::std::option::Option<crate::schemas::NumericTargetingDimension>,
        #[doc = "The verticals included or excluded in this configuration as defined in https://developers.google.com/authorized-buyers/rtb/downloads/publisher-verticals"]
        #[serde(
            rename = "verticalTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertical_targeting: ::std::option::Option<crate::schemas::NumericTargetingDimension>,
        #[doc = "Targeting on a subset of site inventory. If WEB is listed in included_environments, the specified targeting is applied. A maximum of 50,000 site URLs can be targeted. An unset value for targeting allows all web-based bid requests to be sent. Sites can either be targeting positively (bid requests will be sent only if the destination site is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination site is not listed in the pretargeting configuration)."]
        #[serde(
            rename = "webTargeting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_targeting: ::std::option::Option<crate::schemas::StringTargetingDimension>,
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PretargetingConfigAllowedUserTargetingModesItems {
        #[doc = "Ads based on user interest category targeting are allowed to serve."]
        InterestBasedTargeting,
        #[doc = "Remarketing ads are allowed to serve."]
        RemarketingAds,
        #[doc = "Placeholder for undefined user targeting mode."]
        UserTargetingModeUnspecified,
    }
    impl PretargetingConfigAllowedUserTargetingModesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PretargetingConfigAllowedUserTargetingModesItems::InterestBasedTargeting => {
                    "INTEREST_BASED_TARGETING"
                }
                PretargetingConfigAllowedUserTargetingModesItems::RemarketingAds => {
                    "REMARKETING_ADS"
                }
                PretargetingConfigAllowedUserTargetingModesItems::UserTargetingModeUnspecified => {
                    "USER_TARGETING_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PretargetingConfigAllowedUserTargetingModesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PretargetingConfigAllowedUserTargetingModesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PretargetingConfigAllowedUserTargetingModesItems, ()> {
            Ok(match s {
                "INTEREST_BASED_TARGETING" => {
                    PretargetingConfigAllowedUserTargetingModesItems::InterestBasedTargeting
                }
                "REMARKETING_ADS" => {
                    PretargetingConfigAllowedUserTargetingModesItems::RemarketingAds
                }
                "USER_TARGETING_MODE_UNSPECIFIED" => {
                    PretargetingConfigAllowedUserTargetingModesItems::UserTargetingModeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PretargetingConfigAllowedUserTargetingModesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PretargetingConfigAllowedUserTargetingModesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PretargetingConfigAllowedUserTargetingModesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INTEREST_BASED_TARGETING" => {
                    PretargetingConfigAllowedUserTargetingModesItems::InterestBasedTargeting
                }
                "REMARKETING_ADS" => {
                    PretargetingConfigAllowedUserTargetingModesItems::RemarketingAds
                }
                "USER_TARGETING_MODE_UNSPECIFIED" => {
                    PretargetingConfigAllowedUserTargetingModesItems::UserTargetingModeUnspecified
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
    impl ::google_field_selector::FieldSelector for PretargetingConfigAllowedUserTargetingModesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigAllowedUserTargetingModesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PretargetingConfigIncludedEnvironmentsItems {
        #[doc = "App environment."]
        App,
        #[doc = "Placeholder for unspecified environment. This value should not be used."]
        EnvironmentUnspecified,
        #[doc = "Web environment."]
        Web,
    }
    impl PretargetingConfigIncludedEnvironmentsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PretargetingConfigIncludedEnvironmentsItems::App => "APP",
                PretargetingConfigIncludedEnvironmentsItems::EnvironmentUnspecified => {
                    "ENVIRONMENT_UNSPECIFIED"
                }
                PretargetingConfigIncludedEnvironmentsItems::Web => "WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PretargetingConfigIncludedEnvironmentsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PretargetingConfigIncludedEnvironmentsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PretargetingConfigIncludedEnvironmentsItems, ()> {
            Ok(match s {
                "APP" => PretargetingConfigIncludedEnvironmentsItems::App,
                "ENVIRONMENT_UNSPECIFIED" => {
                    PretargetingConfigIncludedEnvironmentsItems::EnvironmentUnspecified
                }
                "WEB" => PretargetingConfigIncludedEnvironmentsItems::Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PretargetingConfigIncludedEnvironmentsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PretargetingConfigIncludedEnvironmentsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PretargetingConfigIncludedEnvironmentsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP" => PretargetingConfigIncludedEnvironmentsItems::App,
                "ENVIRONMENT_UNSPECIFIED" => {
                    PretargetingConfigIncludedEnvironmentsItems::EnvironmentUnspecified
                }
                "WEB" => PretargetingConfigIncludedEnvironmentsItems::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigIncludedEnvironmentsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigIncludedEnvironmentsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PretargetingConfigIncludedFormatsItems {
        #[doc = "Placeholder for undefined creative format. This value should not be used."]
        CreativeFormatUnspecified,
        #[doc = "HTML and AMPHTML creatives."]
        Html,
        #[doc = "Native creative, including standard and video native ads."]
        Native,
        #[doc = "VAST video or audio creative."]
        Vast,
    }
    impl PretargetingConfigIncludedFormatsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PretargetingConfigIncludedFormatsItems::CreativeFormatUnspecified => {
                    "CREATIVE_FORMAT_UNSPECIFIED"
                }
                PretargetingConfigIncludedFormatsItems::Html => "HTML",
                PretargetingConfigIncludedFormatsItems::Native => "NATIVE",
                PretargetingConfigIncludedFormatsItems::Vast => "VAST",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PretargetingConfigIncludedFormatsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PretargetingConfigIncludedFormatsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PretargetingConfigIncludedFormatsItems, ()> {
            Ok(match s {
                "CREATIVE_FORMAT_UNSPECIFIED" => {
                    PretargetingConfigIncludedFormatsItems::CreativeFormatUnspecified
                }
                "HTML" => PretargetingConfigIncludedFormatsItems::Html,
                "NATIVE" => PretargetingConfigIncludedFormatsItems::Native,
                "VAST" => PretargetingConfigIncludedFormatsItems::Vast,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PretargetingConfigIncludedFormatsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PretargetingConfigIncludedFormatsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PretargetingConfigIncludedFormatsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATIVE_FORMAT_UNSPECIFIED" => {
                    PretargetingConfigIncludedFormatsItems::CreativeFormatUnspecified
                }
                "HTML" => PretargetingConfigIncludedFormatsItems::Html,
                "NATIVE" => PretargetingConfigIncludedFormatsItems::Native,
                "VAST" => PretargetingConfigIncludedFormatsItems::Vast,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigIncludedFormatsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigIncludedFormatsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PretargetingConfigIncludedPlatformsItems {
        #[doc = "The connected TV platform."]
        ConnectedTv,
        #[doc = "The personal computer platform."]
        PersonalComputer,
        #[doc = "The mobile platform."]
        Phone,
        #[doc = "Placeholder for an undefined platform. This value should not be used."]
        PlatformUnspecified,
        #[doc = "The tablet platform."]
        Tablet,
    }
    impl PretargetingConfigIncludedPlatformsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PretargetingConfigIncludedPlatformsItems::ConnectedTv => "CONNECTED_TV",
                PretargetingConfigIncludedPlatformsItems::PersonalComputer => "PERSONAL_COMPUTER",
                PretargetingConfigIncludedPlatformsItems::Phone => "PHONE",
                PretargetingConfigIncludedPlatformsItems::PlatformUnspecified => {
                    "PLATFORM_UNSPECIFIED"
                }
                PretargetingConfigIncludedPlatformsItems::Tablet => "TABLET",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PretargetingConfigIncludedPlatformsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PretargetingConfigIncludedPlatformsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PretargetingConfigIncludedPlatformsItems, ()> {
            Ok(match s {
                "CONNECTED_TV" => PretargetingConfigIncludedPlatformsItems::ConnectedTv,
                "PERSONAL_COMPUTER" => PretargetingConfigIncludedPlatformsItems::PersonalComputer,
                "PHONE" => PretargetingConfigIncludedPlatformsItems::Phone,
                "PLATFORM_UNSPECIFIED" => {
                    PretargetingConfigIncludedPlatformsItems::PlatformUnspecified
                }
                "TABLET" => PretargetingConfigIncludedPlatformsItems::Tablet,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PretargetingConfigIncludedPlatformsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PretargetingConfigIncludedPlatformsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PretargetingConfigIncludedPlatformsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONNECTED_TV" => PretargetingConfigIncludedPlatformsItems::ConnectedTv,
                "PERSONAL_COMPUTER" => PretargetingConfigIncludedPlatformsItems::PersonalComputer,
                "PHONE" => PretargetingConfigIncludedPlatformsItems::Phone,
                "PLATFORM_UNSPECIFIED" => {
                    PretargetingConfigIncludedPlatformsItems::PlatformUnspecified
                }
                "TABLET" => PretargetingConfigIncludedPlatformsItems::Tablet,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigIncludedPlatformsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigIncludedPlatformsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PretargetingConfigIncludedUserIdTypesItems {
        #[doc = "Mobile device advertising ID."]
        DeviceId,
        #[doc = "Google cookie, referring to google_user_id in the bid request."]
        GoogleCookie,
        #[doc = "Hosted match data, referring to hosted_match_data in the bid request."]
        HostedMatchData,
        #[doc = "Placeholder for unspecified user identifier."]
        UserIdTypeUnspecified,
    }
    impl PretargetingConfigIncludedUserIdTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PretargetingConfigIncludedUserIdTypesItems::DeviceId => "DEVICE_ID",
                PretargetingConfigIncludedUserIdTypesItems::GoogleCookie => "GOOGLE_COOKIE",
                PretargetingConfigIncludedUserIdTypesItems::HostedMatchData => "HOSTED_MATCH_DATA",
                PretargetingConfigIncludedUserIdTypesItems::UserIdTypeUnspecified => {
                    "USER_ID_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PretargetingConfigIncludedUserIdTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PretargetingConfigIncludedUserIdTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PretargetingConfigIncludedUserIdTypesItems, ()> {
            Ok(match s {
                "DEVICE_ID" => PretargetingConfigIncludedUserIdTypesItems::DeviceId,
                "GOOGLE_COOKIE" => PretargetingConfigIncludedUserIdTypesItems::GoogleCookie,
                "HOSTED_MATCH_DATA" => PretargetingConfigIncludedUserIdTypesItems::HostedMatchData,
                "USER_ID_TYPE_UNSPECIFIED" => {
                    PretargetingConfigIncludedUserIdTypesItems::UserIdTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PretargetingConfigIncludedUserIdTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PretargetingConfigIncludedUserIdTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PretargetingConfigIncludedUserIdTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_ID" => PretargetingConfigIncludedUserIdTypesItems::DeviceId,
                "GOOGLE_COOKIE" => PretargetingConfigIncludedUserIdTypesItems::GoogleCookie,
                "HOSTED_MATCH_DATA" => PretargetingConfigIncludedUserIdTypesItems::HostedMatchData,
                "USER_ID_TYPE_UNSPECIFIED" => {
                    PretargetingConfigIncludedUserIdTypesItems::UserIdTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for PretargetingConfigIncludedUserIdTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigIncludedUserIdTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PretargetingConfigInterstitialTargeting {
        #[doc = "Unspecified interstitial targeting. Represents an interstitial-agnostic selection."]
        InterstitialTargetingUnspecified,
        #[doc = "Only bid requests for interstitial inventory should be sent."]
        OnlyInterstitialRequests,
        #[doc = "Only bid requests for non-interstitial inventory should be sent."]
        OnlyNonInterstitialRequests,
    }
    impl PretargetingConfigInterstitialTargeting {
        pub fn as_str(self) -> &'static str {
            match self {
                PretargetingConfigInterstitialTargeting::InterstitialTargetingUnspecified => {
                    "INTERSTITIAL_TARGETING_UNSPECIFIED"
                }
                PretargetingConfigInterstitialTargeting::OnlyInterstitialRequests => {
                    "ONLY_INTERSTITIAL_REQUESTS"
                }
                PretargetingConfigInterstitialTargeting::OnlyNonInterstitialRequests => {
                    "ONLY_NON_INTERSTITIAL_REQUESTS"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for PretargetingConfigInterstitialTargeting {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PretargetingConfigInterstitialTargeting {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PretargetingConfigInterstitialTargeting, ()> {
            Ok(match s {
                "INTERSTITIAL_TARGETING_UNSPECIFIED" => {
                    PretargetingConfigInterstitialTargeting::InterstitialTargetingUnspecified
                }
                "ONLY_INTERSTITIAL_REQUESTS" => {
                    PretargetingConfigInterstitialTargeting::OnlyInterstitialRequests
                }
                "ONLY_NON_INTERSTITIAL_REQUESTS" => {
                    PretargetingConfigInterstitialTargeting::OnlyNonInterstitialRequests
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PretargetingConfigInterstitialTargeting {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PretargetingConfigInterstitialTargeting {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PretargetingConfigInterstitialTargeting {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INTERSTITIAL_TARGETING_UNSPECIFIED" => {
                    PretargetingConfigInterstitialTargeting::InterstitialTargetingUnspecified
                }
                "ONLY_INTERSTITIAL_REQUESTS" => {
                    PretargetingConfigInterstitialTargeting::OnlyInterstitialRequests
                }
                "ONLY_NON_INTERSTITIAL_REQUESTS" => {
                    PretargetingConfigInterstitialTargeting::OnlyNonInterstitialRequests
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
    impl ::google_field_selector::FieldSelector for PretargetingConfigInterstitialTargeting {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigInterstitialTargeting {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PretargetingConfigState {
        #[doc = "This pretargeting configuration is actively being used to filter bid requests."]
        Active,
        #[doc = "Placeholder for undefined state."]
        StateUnspecified,
        #[doc = "This pretargeting configuration is suspended and not used in serving."]
        Suspended,
    }
    impl PretargetingConfigState {
        pub fn as_str(self) -> &'static str {
            match self {
                PretargetingConfigState::Active => "ACTIVE",
                PretargetingConfigState::StateUnspecified => "STATE_UNSPECIFIED",
                PretargetingConfigState::Suspended => "SUSPENDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PretargetingConfigState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PretargetingConfigState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PretargetingConfigState, ()> {
            Ok(match s {
                "ACTIVE" => PretargetingConfigState::Active,
                "STATE_UNSPECIFIED" => PretargetingConfigState::StateUnspecified,
                "SUSPENDED" => PretargetingConfigState::Suspended,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PretargetingConfigState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PretargetingConfigState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PretargetingConfigState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => PretargetingConfigState::Active,
                "STATE_UNSPECIFIED" => PretargetingConfigState::StateUnspecified,
                "SUSPENDED" => PretargetingConfigState::Suspended,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigState {
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
    pub struct RemoveTargetedAppsRequest {
        #[doc = "A list of app IDs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted app IDs in PretargetingConfig.appTargeting.mobileAppTargeting.values."]
        #[serde(
            rename = "appIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for RemoveTargetedAppsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RemoveTargetedAppsRequest {
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
    pub struct RemoveTargetedPublishersRequest {
        #[doc = "A list of publisher IDs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted publisher IDs in PretargetingConfig.publisherTargeting.values. Publishers are identified by their publisher ID from ads.txt / app-ads.txt. See https://iabtechlab.com/ads-txt/ and https://iabtechlab.com/app-ads-txt/ for more details."]
        #[serde(
            rename = "publisherIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for RemoveTargetedPublishersRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RemoveTargetedPublishersRequest {
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
    pub struct RemoveTargetedSitesRequest {
        #[doc = "A list of site URLs to stop targeting in the pretargeting configuration. These values will be removed from the list of targeted URLs in PretargetingConfig.webTargeting.values."]
        #[serde(
            rename = "sites",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sites: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for RemoveTargetedSitesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RemoveTargetedSitesRequest {
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
    pub struct StringTargetingDimension {
        #[doc = "How the items in this list should be targeted."]
        #[serde(
            rename = "targetingMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub targeting_mode:
            ::std::option::Option<crate::schemas::StringTargetingDimensionTargetingMode>,
        #[doc = "The values specified."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for StringTargetingDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringTargetingDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StringTargetingDimensionTargetingMode {
        #[doc = "The exclusive list type. Inventory must not match any item in this list to be targeted."]
        Exclusive,
        #[doc = "The inclusive list type. Inventory must match an item in this list to be targeted."]
        Inclusive,
        #[doc = "Placeholder for undefined targeting mode."]
        TargetingModeUnspecified,
    }
    impl StringTargetingDimensionTargetingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                StringTargetingDimensionTargetingMode::Exclusive => "EXCLUSIVE",
                StringTargetingDimensionTargetingMode::Inclusive => "INCLUSIVE",
                StringTargetingDimensionTargetingMode::TargetingModeUnspecified => {
                    "TARGETING_MODE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for StringTargetingDimensionTargetingMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for StringTargetingDimensionTargetingMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<StringTargetingDimensionTargetingMode, ()> {
            Ok(match s {
                "EXCLUSIVE" => StringTargetingDimensionTargetingMode::Exclusive,
                "INCLUSIVE" => StringTargetingDimensionTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    StringTargetingDimensionTargetingMode::TargetingModeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for StringTargetingDimensionTargetingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StringTargetingDimensionTargetingMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StringTargetingDimensionTargetingMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EXCLUSIVE" => StringTargetingDimensionTargetingMode::Exclusive,
                "INCLUSIVE" => StringTargetingDimensionTargetingMode::Inclusive,
                "TARGETING_MODE_UNSPECIFIED" => {
                    StringTargetingDimensionTargetingMode::TargetingModeUnspecified
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
    impl ::google_field_selector::FieldSelector for StringTargetingDimensionTargetingMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringTargetingDimensionTargetingMode {
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
    pub struct SuspendPretargetingConfigRequest {}
    impl ::google_field_selector::FieldSelector for SuspendPretargetingConfigRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuspendPretargetingConfigRequest {
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
    pub struct UrlDownloadSize {
        #[doc = "Download size of the URL in kilobytes."]
        #[serde(
            rename = "downloadSizeKb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_size_kb: ::std::option::Option<i32>,
        #[doc = "The normalized URL with query parameters and fragment removed."]
        #[serde(
            rename = "normalizedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UrlDownloadSize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UrlDownloadSize {
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
    pub struct UrlRestriction {
        #[doc = "End date (if specified) of the URL restriction. End date should be later than the start date for the date range to be valid."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The restriction type for the specified URL."]
        #[serde(
            rename = "restrictionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restriction_type: ::std::option::Option<crate::schemas::UrlRestrictionRestrictionType>,
        #[doc = "Start date (if specified) of the URL restriction."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Required. The URL to use for applying the restriction on the user list."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UrlRestriction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UrlRestriction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UrlRestrictionRestrictionType {
        #[doc = "The tag URL (as recorded by the pixel callback) contains the specified URL."]
        Contains,
        #[doc = "The tag URL (as recorded by the pixel callback) does not contain the specified URL."]
        DoesNotContain,
        #[doc = "The tag URL (as recorded by the pixel callback) does not end with the specified URL."]
        DoesNotEndWith,
        #[doc = "The tag URL (as recorded by the pixel callback) does not equal the specified URL."]
        DoesNotEqual,
        #[doc = "The tag URL (as recorded by the pixel callback) does not start with the specified URL."]
        DoesNotStartWith,
        #[doc = "The tag URL (as recorded by the pixel callback) ends with the specified URL."]
        EndsWith,
        #[doc = "The tag URL (as recorded by the pixel callback) exactly matches the specified URL."]
        Equals,
        #[doc = "Default value that should never be used."]
        RestrictionTypeUnspecified,
        #[doc = "The tag URL (as recorded by the pixel callback) starts with the specified URL."]
        StartsWith,
    }
    impl UrlRestrictionRestrictionType {
        pub fn as_str(self) -> &'static str {
            match self {
                UrlRestrictionRestrictionType::Contains => "CONTAINS",
                UrlRestrictionRestrictionType::DoesNotContain => "DOES_NOT_CONTAIN",
                UrlRestrictionRestrictionType::DoesNotEndWith => "DOES_NOT_END_WITH",
                UrlRestrictionRestrictionType::DoesNotEqual => "DOES_NOT_EQUAL",
                UrlRestrictionRestrictionType::DoesNotStartWith => "DOES_NOT_START_WITH",
                UrlRestrictionRestrictionType::EndsWith => "ENDS_WITH",
                UrlRestrictionRestrictionType::Equals => "EQUALS",
                UrlRestrictionRestrictionType::RestrictionTypeUnspecified => {
                    "RESTRICTION_TYPE_UNSPECIFIED"
                }
                UrlRestrictionRestrictionType::StartsWith => "STARTS_WITH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UrlRestrictionRestrictionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UrlRestrictionRestrictionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UrlRestrictionRestrictionType, ()> {
            Ok(match s {
                "CONTAINS" => UrlRestrictionRestrictionType::Contains,
                "DOES_NOT_CONTAIN" => UrlRestrictionRestrictionType::DoesNotContain,
                "DOES_NOT_END_WITH" => UrlRestrictionRestrictionType::DoesNotEndWith,
                "DOES_NOT_EQUAL" => UrlRestrictionRestrictionType::DoesNotEqual,
                "DOES_NOT_START_WITH" => UrlRestrictionRestrictionType::DoesNotStartWith,
                "ENDS_WITH" => UrlRestrictionRestrictionType::EndsWith,
                "EQUALS" => UrlRestrictionRestrictionType::Equals,
                "RESTRICTION_TYPE_UNSPECIFIED" => {
                    UrlRestrictionRestrictionType::RestrictionTypeUnspecified
                }
                "STARTS_WITH" => UrlRestrictionRestrictionType::StartsWith,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UrlRestrictionRestrictionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UrlRestrictionRestrictionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UrlRestrictionRestrictionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTAINS" => UrlRestrictionRestrictionType::Contains,
                "DOES_NOT_CONTAIN" => UrlRestrictionRestrictionType::DoesNotContain,
                "DOES_NOT_END_WITH" => UrlRestrictionRestrictionType::DoesNotEndWith,
                "DOES_NOT_EQUAL" => UrlRestrictionRestrictionType::DoesNotEqual,
                "DOES_NOT_START_WITH" => UrlRestrictionRestrictionType::DoesNotStartWith,
                "ENDS_WITH" => UrlRestrictionRestrictionType::EndsWith,
                "EQUALS" => UrlRestrictionRestrictionType::Equals,
                "RESTRICTION_TYPE_UNSPECIFIED" => {
                    UrlRestrictionRestrictionType::RestrictionTypeUnspecified
                }
                "STARTS_WITH" => UrlRestrictionRestrictionType::StartsWith,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UrlRestrictionRestrictionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UrlRestrictionRestrictionType {
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
    pub struct UserList {
        #[doc = "The description for the user list."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. Display name of the user list. This must be unique across all user lists for a given account."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Required. The number of days a user's cookie stays on the user list. The field must be between 0 and 540 inclusive."]
        #[serde(
            rename = "membershipDurationDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub membership_duration_days: ::std::option::Option<i64>,
        #[doc = "Output only. Name of the user list that must follow the pattern `buyers/{buyer}/userLists/{user_list}`, where `{buyer}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyer}` represents the account ID of the child seat buyer. `{user_list}` is an int64 identifier assigned by Google to uniquely identify a user list."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The status of the user list. A new user list starts out as open."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::UserListStatus>,
        #[doc = "Required. The URL restriction for the user list."]
        #[serde(
            rename = "urlRestriction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url_restriction: ::std::option::Option<crate::schemas::UrlRestriction>,
    }
    impl ::google_field_selector::FieldSelector for UserList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UserListStatus {
        #[doc = "New users cannot be added to the user list."]
        Closed,
        #[doc = "New users can be added to the user list."]
        Open,
        #[doc = "Default value that should never be used."]
        StatusUnspecified,
    }
    impl UserListStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                UserListStatus::Closed => "CLOSED",
                UserListStatus::Open => "OPEN",
                UserListStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UserListStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UserListStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UserListStatus, ()> {
            Ok(match s {
                "CLOSED" => UserListStatus::Closed,
                "OPEN" => UserListStatus::Open,
                "STATUS_UNSPECIFIED" => UserListStatus::StatusUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UserListStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UserListStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserListStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLOSED" => UserListStatus::Closed,
                "OPEN" => UserListStatus::Open,
                "STATUS_UNSPECIFIED" => UserListStatus::StatusUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UserListStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserListStatus {
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
    pub struct VideoContent {
        #[doc = "Output only. Video metadata."]
        #[serde(
            rename = "videoMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_metadata: ::std::option::Option<crate::schemas::VideoMetadata>,
        #[doc = "The URL to fetch a video ad."]
        #[serde(
            rename = "videoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_url: ::std::option::Option<String>,
        #[doc = "The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard."]
        #[serde(
            rename = "videoVastXml",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_vast_xml: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VideoContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VideoContent {
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
    pub struct VideoMetadata {
        #[doc = "The duration of the ad. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<String>,
        #[doc = "Is this a valid VAST ad? Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "isValidVast",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_valid_vast: ::std::option::Option<bool>,
        #[doc = "Is this a VPAID ad? Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "isVpaid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_vpaid: ::std::option::Option<bool>,
        #[doc = "The list of all media files declared in the VAST. If there are multiple VASTs in a wrapper chain, this includes the media files from the deepest one in the chain."]
        #[serde(
            rename = "mediaFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub media_files: ::std::option::Option<Vec<crate::schemas::MediaFile>>,
        #[doc = "The minimum duration that the user has to watch before being able to skip this ad. If the field is not set, the ad is not skippable. If the field is set, the ad is skippable. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "skipOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_offset: ::std::option::Option<String>,
        #[doc = "The maximum VAST version across all wrapped VAST documents. Can be used to filter the response of the creatives.list method."]
        #[serde(
            rename = "vastVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vast_version: ::std::option::Option<crate::schemas::VideoMetadataVastVersion>,
    }
    impl ::google_field_selector::FieldSelector for VideoMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VideoMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VideoMetadataVastVersion {
        #[doc = "VAST 1.0"]
        VastVersion10,
        #[doc = "VAST 2.0"]
        VastVersion20,
        #[doc = "VAST 3.0"]
        VastVersion30,
        #[doc = "VAST 4.0"]
        VastVersion40,
        #[doc = "Default value that should never be used."]
        VastVersionUnspecified,
    }
    impl VideoMetadataVastVersion {
        pub fn as_str(self) -> &'static str {
            match self {
                VideoMetadataVastVersion::VastVersion10 => "VAST_VERSION_1_0",
                VideoMetadataVastVersion::VastVersion20 => "VAST_VERSION_2_0",
                VideoMetadataVastVersion::VastVersion30 => "VAST_VERSION_3_0",
                VideoMetadataVastVersion::VastVersion40 => "VAST_VERSION_4_0",
                VideoMetadataVastVersion::VastVersionUnspecified => "VAST_VERSION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VideoMetadataVastVersion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VideoMetadataVastVersion {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VideoMetadataVastVersion, ()> {
            Ok(match s {
                "VAST_VERSION_1_0" => VideoMetadataVastVersion::VastVersion10,
                "VAST_VERSION_2_0" => VideoMetadataVastVersion::VastVersion20,
                "VAST_VERSION_3_0" => VideoMetadataVastVersion::VastVersion30,
                "VAST_VERSION_4_0" => VideoMetadataVastVersion::VastVersion40,
                "VAST_VERSION_UNSPECIFIED" => VideoMetadataVastVersion::VastVersionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VideoMetadataVastVersion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VideoMetadataVastVersion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VideoMetadataVastVersion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "VAST_VERSION_1_0" => VideoMetadataVastVersion::VastVersion10,
                "VAST_VERSION_2_0" => VideoMetadataVastVersion::VastVersion20,
                "VAST_VERSION_3_0" => VideoMetadataVastVersion::VastVersion30,
                "VAST_VERSION_4_0" => VideoMetadataVastVersion::VastVersion40,
                "VAST_VERSION_UNSPECIFIED" => VideoMetadataVastVersion::VastVersionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VideoMetadataVastVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VideoMetadataVastVersion {
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
    pub struct WatchCreativesRequest {}
    impl ::google_field_selector::FieldSelector for WatchCreativesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WatchCreativesRequest {
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
    pub struct WatchCreativesResponse {
        #[doc = "The Pub/Sub subscription that can be used to pull creative status notifications. This would be of the format `projects/{project_id}/subscriptions/{subscription_id}`. Subscription is created with pull delivery. All service accounts belonging to the bidder will have read access to this subscription. Subscriptions that are inactive for more than 90 days will be disabled. Please use watchCreatives to re-enable the subscription."]
        #[serde(
            rename = "subscription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subscription: ::std::option::Option<String>,
        #[doc = "The Pub/Sub topic that will be used to publish creative serving status notifications. This would be of the format `projects/{project_id}/topics/{topic_id}`."]
        #[serde(
            rename = "topic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub topic: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WatchCreativesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WatchCreativesResponse {
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
    #[doc = "Actions that can be performed on the bidders resource"]
    pub fn bidders(&self) -> crate::resources::bidders::BiddersActions {
        crate::resources::bidders::BiddersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the buyers resource"]
    pub fn buyers(&self) -> crate::resources::buyers::BuyersActions {
        crate::resources::buyers::BuyersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod bidders {
        pub mod params {}
        pub struct BiddersActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> BiddersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets a bidder account by its name."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                    name: name.into(),
                }
            }
            #[doc = "Lists all the bidder accounts that belong to the caller."]
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
                }
            }
            #[doc = "Actions that can be performed on the creatives resource"]
            pub fn creatives(&self) -> crate::resources::bidders::creatives::CreativesActions {
                crate::resources::bidders::creatives::CreativesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the endpoints resource"]
            pub fn endpoints(&self) -> crate::resources::bidders::endpoints::EndpointsActions {
                crate::resources::bidders::endpoints::EndpointsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the pretargeting_configs resource"]
            pub fn pretargeting_configs(
                &self,
            ) -> crate::resources::bidders::pretargeting_configs::PretargetingConfigsActions
            {
                crate::resources::bidders::pretargeting_configs::PretargetingConfigsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [BiddersActions::get()](struct.BiddersActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
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
            ) -> Result<crate::schemas::Bidder, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Bidder, crate::Error> {
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
                let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [BiddersActions::list()](struct.BiddersActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "The maximum number of bidders to return. If unspecified, at most 100 bidders will be returned. The maximum value is 500; values above 500 will be coerced to 500."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results the server should return. This value is received from a previous `ListBidders` call in ListBiddersResponse.nextPageToken."]
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
            pub fn iter_bidders<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_bidders_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_bidders_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Bidder> {
                self.iter_bidders_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_bidders_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Bidder> {
                self.iter_bidders_with_fields(Some("*"))
            }
            pub fn iter_bidders_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "bidders").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "bidders")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListBiddersResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListBiddersResponse> {
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
            ) -> Result<crate::schemas::ListBiddersResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListBiddersResponse, crate::Error> {
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
                let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                output.push_str("v1/bidders");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        pub mod creatives {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListView {
                    #[doc = "Not specified, equivalent to SERVING_DECISION_ONLY."]
                    CreativeViewUnspecified,
                    #[doc = "The entire creative resource (including the declared fields and the creative content) is included in the response."]
                    Full,
                    #[doc = "Only creativeServingDecision is included in the response."]
                    ServingDecisionOnly,
                }
                impl ListView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListView::CreativeViewUnspecified => "CREATIVE_VIEW_UNSPECIFIED",
                            ListView::Full => "FULL",
                            ListView::ServingDecisionOnly => "SERVING_DECISION_ONLY",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListView {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListView {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListView, ()> {
                        Ok(match s {
                            "CREATIVE_VIEW_UNSPECIFIED" => ListView::CreativeViewUnspecified,
                            "FULL" => ListView::Full,
                            "SERVING_DECISION_ONLY" => ListView::ServingDecisionOnly,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "CREATIVE_VIEW_UNSPECIFIED" => ListView::CreativeViewUnspecified,
                            "FULL" => ListView::Full,
                            "SERVING_DECISION_ONLY" => ListView::ServingDecisionOnly,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListView {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListView {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct CreativesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> CreativesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists creatives."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                        view: None,
                    }
                }
                #[doc = "Watches all creatives pertaining to a bidder. It is sufficient to invoke this endpoint once per bidder. A Pub/Sub topic will be created and notifications will be pushed to the topic when any of the bidder's creatives change status. All of the bidder's service accounts will have access to read from the topic. Subsequent invocations of this method will return the existing Pub/Sub configuration."]
                pub fn watch(
                    &self,
                    request: crate::schemas::WatchCreativesRequest,
                    parent: impl Into<String>,
                ) -> WatchRequestBuilder {
                    WatchRequestBuilder {
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
                        parent: parent.into(),
                    }
                }
            }
            #[doc = "Created via [CreativesActions::list()](struct.CreativesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                view: Option<crate::resources::bidders::creatives::params::ListView>,
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
                #[doc = "Query string to filter creatives. If no filter is specified, all active creatives will be returned. Example: 'accountId=12345 AND (dealsStatus:DISAPPROVED AND disapprovalReason:UNACCEPTABLE_CONTENT) OR declaredAttributes:IS_COOKIE_TARGETED'"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Requested page size. The server may return fewer creatives than requested (due to timeout constraint) even if more are available via another call. If unspecified, server will pick an appropriate default. Acceptable values are 1 to 1000, inclusive."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativesResponse.nextPageToken returned from the previous call to the 'ListCreatives' method."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                pub fn view(
                    mut self,
                    value: crate::resources::bidders::creatives::params::ListView,
                ) -> Self {
                    self.view = Some(value);
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
                pub fn iter_creatives<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_creatives_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_creatives_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Creative> {
                    self.iter_creatives_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_creatives_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Creative> {
                    self.iter_creatives_with_fields(Some("*"))
                }
                pub fn iter_creatives_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "creatives").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "creatives")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCreativesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCreativesResponse>
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
                ) -> Result<crate::schemas::ListCreativesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListCreativesResponse, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/creatives");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("view", &self.view)]);
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
            #[doc = "Created via [CreativesActions::watch()](struct.CreativesActions.html#method.watch)"]
            #[derive(Debug, Clone)]
            pub struct WatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::WatchCreativesRequest,
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
            impl<'a> WatchRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::WatchCreativesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::WatchCreativesResponse, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/creatives:watch");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
        }
        pub mod endpoints {
            pub mod params {}
            pub struct EndpointsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> EndpointsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets a bidder endpoint by its name."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Lists all the bidder's endpoints."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [EndpointsActions::get()](struct.EndpointsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Endpoint, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Endpoint, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [EndpointsActions::list()](struct.EndpointsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The maximum number of endpoints to return. If unspecified, at most 100 endpoints will be returned. The maximum value is 500; values above 500 will be coerced to 500."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return. This value is received from a previous `ListEndpoints` call in ListEndpointsResponse.nextPageToken."]
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
                pub fn iter_endpoints<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_endpoints_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_endpoints_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Endpoint> {
                    self.iter_endpoints_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_endpoints_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Endpoint> {
                    self.iter_endpoints_with_fields(Some("*"))
                }
                pub fn iter_endpoints_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "endpoints").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "endpoints")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListEndpointsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListEndpointsResponse>
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
                ) -> Result<crate::schemas::ListEndpointsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListEndpointsResponse, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/endpoints");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        pub mod pretargeting_configs {
            pub mod params {}
            pub struct PretargetingConfigsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PretargetingConfigsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Activates a pretargeting configuration."]
                pub fn activate(
                    &self,
                    request: crate::schemas::ActivatePretargetingConfigRequest,
                    name: impl Into<String>,
                ) -> ActivateRequestBuilder {
                    ActivateRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Adds targeted apps to the pretargeting configuration."]
                pub fn add_targeted_apps(
                    &self,
                    request: crate::schemas::AddTargetedAppsRequest,
                    pretargeting_config: impl Into<String>,
                ) -> AddTargetedAppsRequestBuilder {
                    AddTargetedAppsRequestBuilder {
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
                        pretargeting_config: pretargeting_config.into(),
                    }
                }
                #[doc = "Adds targeted publishers to the pretargeting config."]
                pub fn add_targeted_publishers(
                    &self,
                    request: crate::schemas::AddTargetedPublishersRequest,
                    pretargeting_config: impl Into<String>,
                ) -> AddTargetedPublishersRequestBuilder {
                    AddTargetedPublishersRequestBuilder {
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
                        pretargeting_config: pretargeting_config.into(),
                    }
                }
                #[doc = "Adds targeted sites to the pretargeting configuration."]
                pub fn add_targeted_sites(
                    &self,
                    request: crate::schemas::AddTargetedSitesRequest,
                    pretargeting_config: impl Into<String>,
                ) -> AddTargetedSitesRequestBuilder {
                    AddTargetedSitesRequestBuilder {
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
                        pretargeting_config: pretargeting_config.into(),
                    }
                }
                #[doc = "Creates a pretargeting configuration. A pretargeting configuration's state (PretargetingConfig.state) is active upon creation, and it will start to affect traffic shortly after. A bidder may create a maximum of 10 pretargeting configurations. Attempts to exceed this maximum results in a 400 bad request error."]
                pub fn create(
                    &self,
                    request: crate::schemas::PretargetingConfig,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Deletes a pretargeting configuration."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets a pretargeting configuration."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Lists all pretargeting configurations for a single bidder."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a pretargeting configuration."]
                pub fn patch(
                    &self,
                    request: crate::schemas::PretargetingConfig,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder {
                    PatchRequestBuilder {
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
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Removes targeted apps from the pretargeting configuration."]
                pub fn remove_targeted_apps(
                    &self,
                    request: crate::schemas::RemoveTargetedAppsRequest,
                    pretargeting_config: impl Into<String>,
                ) -> RemoveTargetedAppsRequestBuilder {
                    RemoveTargetedAppsRequestBuilder {
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
                        pretargeting_config: pretargeting_config.into(),
                    }
                }
                #[doc = "Removes targeted publishers from the pretargeting config."]
                pub fn remove_targeted_publishers(
                    &self,
                    request: crate::schemas::RemoveTargetedPublishersRequest,
                    pretargeting_config: impl Into<String>,
                ) -> RemoveTargetedPublishersRequestBuilder {
                    RemoveTargetedPublishersRequestBuilder {
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
                        pretargeting_config: pretargeting_config.into(),
                    }
                }
                #[doc = "Removes targeted sites from the pretargeting configuration."]
                pub fn remove_targeted_sites(
                    &self,
                    request: crate::schemas::RemoveTargetedSitesRequest,
                    pretargeting_config: impl Into<String>,
                ) -> RemoveTargetedSitesRequestBuilder {
                    RemoveTargetedSitesRequestBuilder {
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
                        pretargeting_config: pretargeting_config.into(),
                    }
                }
                #[doc = "Suspends a pretargeting configuration."]
                pub fn suspend(
                    &self,
                    request: crate::schemas::SuspendPretargetingConfigRequest,
                    name: impl Into<String>,
                ) -> SuspendRequestBuilder {
                    SuspendRequestBuilder {
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
                        name: name.into(),
                    }
                }
            }
            #[doc = "Created via [PretargetingConfigsActions::activate()](struct.PretargetingConfigsActions.html#method.activate)"]
            #[derive(Debug, Clone)]
            pub struct ActivateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ActivatePretargetingConfigRequest,
                name: String,
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
            impl<'a> ActivateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":activate");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::add_targeted_apps()](struct.PretargetingConfigsActions.html#method.add_targeted_apps)"]
            #[derive(Debug, Clone)]
            pub struct AddTargetedAppsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::AddTargetedAppsRequest,
                pretargeting_config: String,
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
            impl<'a> AddTargetedAppsRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.pretargeting_config;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":addTargetedApps");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::add_targeted_publishers()](struct.PretargetingConfigsActions.html#method.add_targeted_publishers)"]
            #[derive(Debug, Clone)]
            pub struct AddTargetedPublishersRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::AddTargetedPublishersRequest,
                pretargeting_config: String,
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
            impl<'a> AddTargetedPublishersRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.pretargeting_config;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":addTargetedPublishers");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::add_targeted_sites()](struct.PretargetingConfigsActions.html#method.add_targeted_sites)"]
            #[derive(Debug, Clone)]
            pub struct AddTargetedSitesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::AddTargetedSitesRequest,
                pretargeting_config: String,
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
            impl<'a> AddTargetedSitesRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.pretargeting_config;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":addTargetedSites");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::create()](struct.PretargetingConfigsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::PretargetingConfig,
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/pretargetingConfigs");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::delete()](struct.PretargetingConfigsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
            #[doc = "Created via [PretargetingConfigsActions::get()](struct.PretargetingConfigsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [PretargetingConfigsActions::list()](struct.PretargetingConfigsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The maximum number of pretargeting configurations to return. If unspecified, at most 10 pretargeting configurations will be returned. The maximum value is 100; values above 100 will be coerced to 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return. This value is received from a previous `ListPretargetingConfigs` call in ListPretargetingConfigsResponse.nextPageToken."]
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
                pub fn iter_pretargeting_configs<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_pretargeting_configs_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_pretargeting_configs_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::PretargetingConfig>
                {
                    self.iter_pretargeting_configs_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_pretargeting_configs_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::PretargetingConfig>
                {
                    self.iter_pretargeting_configs_with_fields(Some("*"))
                }
                pub fn iter_pretargeting_configs_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector =
                            concat!("nextPageToken,", "pretargetingConfigs").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "pretargetingConfigs")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListPretargetingConfigsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListPretargetingConfigsResponse>
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
                ) -> Result<crate::schemas::ListPretargetingConfigsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListPretargetingConfigsResponse, crate::Error>
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/pretargetingConfigs");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [PretargetingConfigsActions::patch()](struct.PretargetingConfigsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::PretargetingConfig,
                name: String,
                update_mask: Option<String>,
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
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "Field mask to use for partial in-place updates."]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    req = req.query(&[("updateMask", &self.update_mask)]);
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
            #[doc = "Created via [PretargetingConfigsActions::remove_targeted_apps()](struct.PretargetingConfigsActions.html#method.remove_targeted_apps)"]
            #[derive(Debug, Clone)]
            pub struct RemoveTargetedAppsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::RemoveTargetedAppsRequest,
                pretargeting_config: String,
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
            impl<'a> RemoveTargetedAppsRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.pretargeting_config;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":removeTargetedApps");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::remove_targeted_publishers()](struct.PretargetingConfigsActions.html#method.remove_targeted_publishers)"]
            #[derive(Debug, Clone)]
            pub struct RemoveTargetedPublishersRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::RemoveTargetedPublishersRequest,
                pretargeting_config: String,
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
            impl<'a> RemoveTargetedPublishersRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.pretargeting_config;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":removeTargetedPublishers");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::remove_targeted_sites()](struct.PretargetingConfigsActions.html#method.remove_targeted_sites)"]
            #[derive(Debug, Clone)]
            pub struct RemoveTargetedSitesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::RemoveTargetedSitesRequest,
                pretargeting_config: String,
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
            impl<'a> RemoveTargetedSitesRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.pretargeting_config;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":removeTargetedSites");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [PretargetingConfigsActions::suspend()](struct.PretargetingConfigsActions.html#method.suspend)"]
            #[derive(Debug, Clone)]
            pub struct SuspendRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SuspendPretargetingConfigRequest,
                name: String,
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
            impl<'a> SuspendRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::PretargetingConfig, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":suspend");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
        }
    }
    pub mod buyers {
        pub mod params {}
        pub struct BuyersActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> BuyersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets a buyer account by its name."]
            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                    name: name.into(),
                }
            }
            #[doc = "Gets remarketing tag for a buyer. A remarketing tag is a piece of JavaScript code that can be placed on a web page. When a user visits a page containing a remarketing tag, Google adds the user to a user list."]
            pub fn get_remarketing_tag(
                &self,
                name: impl Into<String>,
            ) -> GetRemarketingTagRequestBuilder {
                GetRemarketingTagRequestBuilder {
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
                    name: name.into(),
                }
            }
            #[doc = "Lists all buyer account information the calling buyer user or service account is permissioned to manage."]
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
                }
            }
            #[doc = "Actions that can be performed on the creatives resource"]
            pub fn creatives(&self) -> crate::resources::buyers::creatives::CreativesActions {
                crate::resources::buyers::creatives::CreativesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the user_lists resource"]
            pub fn user_lists(&self) -> crate::resources::buyers::user_lists::UserListsActions {
                crate::resources::buyers::user_lists::UserListsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [BuyersActions::get()](struct.BuyersActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
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
            ) -> Result<crate::schemas::Buyer, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Buyer, crate::Error> {
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
                let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [BuyersActions::get_remarketing_tag()](struct.BuyersActions.html#method.get_remarketing_tag)"]
        #[derive(Debug, Clone)]
        pub struct GetRemarketingTagRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
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
        impl<'a> GetRemarketingTagRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GetRemarketingTagResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetRemarketingTagResponse, crate::Error> {
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
                let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":getRemarketingTag");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        #[doc = "Created via [BuyersActions::list()](struct.BuyersActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "The maximum number of buyers to return. If unspecified, at most 100 buyers will be returned. The maximum value is 500; values above 500 will be coerced to 500."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results the server should return. This value is received from a previous `ListBuyers` call in ListBuyersResponse.nextPageToken."]
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
            pub fn iter_buyers<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_buyers_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_buyers_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Buyer> {
                self.iter_buyers_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_buyers_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Buyer> {
                self.iter_buyers_with_fields(Some("*"))
            }
            pub fn iter_buyers_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "buyers").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "buyers")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListBuyersResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListBuyersResponse> {
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
            ) -> Result<crate::schemas::ListBuyersResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListBuyersResponse, crate::Error> {
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
                let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                output.push_str("v1/buyers");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
        pub mod creatives {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetView {
                    #[doc = "Not specified, equivalent to SERVING_DECISION_ONLY."]
                    CreativeViewUnspecified,
                    #[doc = "The entire creative resource (including the declared fields and the creative content) is included in the response."]
                    Full,
                    #[doc = "Only creativeServingDecision is included in the response."]
                    ServingDecisionOnly,
                }
                impl GetView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetView::CreativeViewUnspecified => "CREATIVE_VIEW_UNSPECIFIED",
                            GetView::Full => "FULL",
                            GetView::ServingDecisionOnly => "SERVING_DECISION_ONLY",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetView {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetView {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GetView, ()> {
                        Ok(match s {
                            "CREATIVE_VIEW_UNSPECIFIED" => GetView::CreativeViewUnspecified,
                            "FULL" => GetView::Full,
                            "SERVING_DECISION_ONLY" => GetView::ServingDecisionOnly,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "CREATIVE_VIEW_UNSPECIFIED" => GetView::CreativeViewUnspecified,
                            "FULL" => GetView::Full,
                            "SERVING_DECISION_ONLY" => GetView::ServingDecisionOnly,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetView {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetView {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListView {
                    #[doc = "Not specified, equivalent to SERVING_DECISION_ONLY."]
                    CreativeViewUnspecified,
                    #[doc = "The entire creative resource (including the declared fields and the creative content) is included in the response."]
                    Full,
                    #[doc = "Only creativeServingDecision is included in the response."]
                    ServingDecisionOnly,
                }
                impl ListView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListView::CreativeViewUnspecified => "CREATIVE_VIEW_UNSPECIFIED",
                            ListView::Full => "FULL",
                            ListView::ServingDecisionOnly => "SERVING_DECISION_ONLY",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListView {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListView {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListView, ()> {
                        Ok(match s {
                            "CREATIVE_VIEW_UNSPECIFIED" => ListView::CreativeViewUnspecified,
                            "FULL" => ListView::Full,
                            "SERVING_DECISION_ONLY" => ListView::ServingDecisionOnly,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "CREATIVE_VIEW_UNSPECIFIED" => ListView::CreativeViewUnspecified,
                            "FULL" => ListView::Full,
                            "SERVING_DECISION_ONLY" => ListView::ServingDecisionOnly,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListView {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListView {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct CreativesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> CreativesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a creative."]
                pub fn create(
                    &self,
                    request: crate::schemas::Creative,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Gets a creative."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                        name: name.into(),
                        view: None,
                    }
                }
                #[doc = "Lists creatives."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                        view: None,
                    }
                }
                #[doc = "Updates a creative."]
                pub fn patch(
                    &self,
                    request: crate::schemas::Creative,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder {
                    PatchRequestBuilder {
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
                        name: name.into(),
                        update_mask: None,
                    }
                }
            }
            #[doc = "Created via [CreativesActions::create()](struct.CreativesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Creative,
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
                ) -> Result<crate::schemas::Creative, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Creative, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/creatives");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [CreativesActions::get()](struct.CreativesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                view: Option<crate::resources::buyers::creatives::params::GetView>,
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
                #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                pub fn view(
                    mut self,
                    value: crate::resources::buyers::creatives::params::GetView,
                ) -> Self {
                    self.view = Some(value);
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
                ) -> Result<crate::schemas::Creative, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Creative, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("view", &self.view)]);
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
            #[doc = "Created via [CreativesActions::list()](struct.CreativesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                view: Option<crate::resources::buyers::creatives::params::ListView>,
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
                #[doc = "Query string to filter creatives. If no filter is specified, all active creatives will be returned. Example: 'accountId=12345 AND (dealsStatus:DISAPPROVED AND disapprovalReason:UNACCEPTABLE_CONTENT) OR declaredAttributes:IS_COOKIE_TARGETED'"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Requested page size. The server may return fewer creatives than requested (due to timeout constraint) even if more are available via another call. If unspecified, server will pick an appropriate default. Acceptable values are 1 to 1000, inclusive."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativesResponse.nextPageToken returned from the previous call to the 'ListCreatives' method."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Controls the amount of information included in the response. By default only creativeServingDecision is included. To retrieve the entire creative resource (including the declared fields and the creative content) specify the view as \"FULL\"."]
                pub fn view(
                    mut self,
                    value: crate::resources::buyers::creatives::params::ListView,
                ) -> Self {
                    self.view = Some(value);
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
                pub fn iter_creatives<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_creatives_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_creatives_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Creative> {
                    self.iter_creatives_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_creatives_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Creative> {
                    self.iter_creatives_with_fields(Some("*"))
                }
                pub fn iter_creatives_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "creatives").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "creatives")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCreativesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListCreativesResponse>
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
                ) -> Result<crate::schemas::ListCreativesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListCreativesResponse, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/creatives");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("view", &self.view)]);
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
            #[doc = "Created via [CreativesActions::patch()](struct.CreativesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Creative,
                name: String,
                update_mask: Option<String>,
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
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "Field mask to use for partial in-place updates."]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
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
                ) -> Result<crate::schemas::Creative, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Creative, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    req = req.query(&[("updateMask", &self.update_mask)]);
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
        }
        pub mod user_lists {
            pub mod params {}
            pub struct UserListsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> UserListsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Change the status of a user list to CLOSED. This prevents new users from being added to the user list."]
                pub fn close(
                    &self,
                    request: crate::schemas::CloseUserListRequest,
                    name: impl Into<String>,
                ) -> CloseRequestBuilder {
                    CloseRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Create a new user list."]
                pub fn create(
                    &self,
                    request: crate::schemas::UserList,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Gets a user list by its name."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets remarketing tag for a buyer. A remarketing tag is a piece of JavaScript code that can be placed on a web page. When a user visits a page containing a remarketing tag, Google adds the user to a user list."]
                pub fn get_remarketing_tag(
                    &self,
                    name: impl Into<String>,
                ) -> GetRemarketingTagRequestBuilder {
                    GetRemarketingTagRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Lists the user lists visible to the current user."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
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
                        parent: parent.into(),
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Change the status of a user list to OPEN. This allows new users to be added to the user list."]
                pub fn open(
                    &self,
                    request: crate::schemas::OpenUserListRequest,
                    name: impl Into<String>,
                ) -> OpenRequestBuilder {
                    OpenRequestBuilder {
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
                        name: name.into(),
                    }
                }
                #[doc = "Update the given user list. Only user lists with URLRestrictions can be updated."]
                pub fn update(
                    &self,
                    request: crate::schemas::UserList,
                    name: impl Into<String>,
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
                        name: name.into(),
                    }
                }
            }
            #[doc = "Created via [UserListsActions::close()](struct.UserListsActions.html#method.close)"]
            #[derive(Debug, Clone)]
            pub struct CloseRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::CloseUserListRequest,
                name: String,
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
            impl<'a> CloseRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::UserList, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::UserList, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":close");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [UserListsActions::create()](struct.UserListsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::UserList,
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
                ) -> Result<crate::schemas::UserList, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::UserList, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/userLists");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [UserListsActions::get()](struct.UserListsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::UserList, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::UserList, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [UserListsActions::get_remarketing_tag()](struct.UserListsActions.html#method.get_remarketing_tag)"]
            #[derive(Debug, Clone)]
            pub struct GetRemarketingTagRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
            impl<'a> GetRemarketingTagRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GetRemarketingTagResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GetRemarketingTagResponse, crate::Error>
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getRemarketingTag");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [UserListsActions::list()](struct.UserListsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The number of results to return per page."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Continuation page token (as received from a previous response)."]
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
                pub fn iter_user_lists<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_user_lists_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_user_lists_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::UserList> {
                    self.iter_user_lists_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_user_lists_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::UserList> {
                    self.iter_user_lists_with_fields(Some("*"))
                }
                pub fn iter_user_lists_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "userLists").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "userLists")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListUserListsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListUserListsResponse>
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
                ) -> Result<crate::schemas::ListUserListsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListUserListsResponse, crate::Error> {
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
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/userLists");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [UserListsActions::open()](struct.UserListsActions.html#method.open)"]
            #[derive(Debug, Clone)]
            pub struct OpenRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::OpenUserListRequest,
                name: String,
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
            impl<'a> OpenRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::UserList, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::UserList, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":open");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
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
            #[doc = "Created via [UserListsActions::update()](struct.UserListsActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::UserList,
                name: String,
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
                ) -> Result<crate::schemas::UserList, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::UserList, crate::Error> {
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
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://realtimebidding.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
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
