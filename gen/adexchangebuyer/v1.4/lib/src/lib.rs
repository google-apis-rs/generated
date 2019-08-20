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
    pub struct AccountBidderLocationItems {
        #[doc = "The protocol that the bidder endpoint is using. OpenRTB protocols with prefix PROTOCOL_OPENRTB_PROTOBUF use proto buffer, otherwise use JSON.  Allowed values:\n\n* PROTOCOL_ADX \n* PROTOCOL_OPENRTB_2_2 \n* PROTOCOL_OPENRTB_2_3 \n* PROTOCOL_OPENRTB_2_4 \n* PROTOCOL_OPENRTB_2_5 \n* PROTOCOL_OPENRTB_PROTOBUF_2_3 \n* PROTOCOL_OPENRTB_PROTOBUF_2_4 \n* PROTOCOL_OPENRTB_PROTOBUF_2_5"]
        #[serde(rename = "bidProtocol", default)]
        pub bid_protocol: Option<String>,
        #[doc = "The maximum queries per second the Ad Exchange will send."]
        #[serde(rename = "maximumQps", default)]
        pub maximum_qps: Option<i32>,
        #[doc = "The geographical region the Ad Exchange should send requests from. Only used by some quota systems, but always setting the value is recommended. Allowed values:\n\n* ASIA \n* EUROPE \n* US_EAST \n* US_WEST"]
        #[serde(rename = "region", default)]
        pub region: Option<String>,
        #[doc = "The URL to which the Ad Exchange will send bid requests."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for AccountBidderLocationItems {
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
    pub struct Account {
        #[doc = "When this is false, bid requests that include a deal ID for a private auction or preferred deal are always sent to your bidder. When true, all active pretargeting configs will be applied to private auctions and preferred deals. Programmatic Guaranteed deals (when enabled) are always sent to your bidder."]
        #[serde(rename = "applyPretargetingToNonGuaranteedDeals", default)]
        pub apply_pretargeting_to_non_guaranteed_deals: Option<bool>,
        #[doc = "Your bidder locations that have distinct URLs."]
        #[serde(rename = "bidderLocation", default)]
        pub bidder_location: Option<Vec<crate::schemas::AccountBidderLocationItems>>,
        #[doc = "The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this."]
        #[serde(rename = "cookieMatchingNid", default)]
        pub cookie_matching_nid: Option<String>,
        #[doc = "The base URL used in cookie match requests."]
        #[serde(rename = "cookieMatchingUrl", default)]
        pub cookie_matching_url: Option<String>,
        #[doc = "Account id."]
        #[serde(rename = "id", default)]
        pub id: Option<i32>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this."]
        #[serde(rename = "maximumActiveCreatives", default)]
        pub maximum_active_creatives: Option<i32>,
        #[doc = "The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this."]
        #[serde(rename = "maximumTotalQps", default)]
        pub maximum_total_qps: Option<i32>,
        #[doc = "The number of creatives that this account inserted or bid with in the last 30 days."]
        #[serde(rename = "numberActiveCreatives", default)]
        pub number_active_creatives: Option<i32>,
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
    pub struct AccountsList {
        #[doc = "A list of accounts."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Account>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for AccountsList {
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
    pub struct AddOrderDealsRequest {
        #[doc = "The list of deals to add"]
        #[serde(rename = "deals", default)]
        pub deals: Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The last known proposal revision number."]
        #[serde(rename = "proposalRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: Option<i64>,
        #[doc = "Indicates an optional action to take on the proposal"]
        #[serde(rename = "updateAction", default)]
        pub update_action: Option<String>,
    }
    impl ::field_selector::FieldSelector for AddOrderDealsRequest {
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
    pub struct AddOrderDealsResponse {
        #[doc = "List of deals added (in the same proposal as passed in the request)"]
        #[serde(rename = "deals", default)]
        pub deals: Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The updated revision number for the proposal."]
        #[serde(rename = "proposalRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: Option<i64>,
    }
    impl ::field_selector::FieldSelector for AddOrderDealsResponse {
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
    pub struct AddOrderNotesRequest {
        #[doc = "The list of notes to add."]
        #[serde(rename = "notes", default)]
        pub notes: Option<Vec<crate::schemas::MarketplaceNote>>,
    }
    impl ::field_selector::FieldSelector for AddOrderNotesRequest {
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
    pub struct AddOrderNotesResponse {
        #[serde(rename = "notes", default)]
        pub notes: Option<Vec<crate::schemas::MarketplaceNote>>,
    }
    impl ::field_selector::FieldSelector for AddOrderNotesResponse {
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
    pub struct BillingInfo {
        #[doc = "Account id."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<i32>,
        #[doc = "Account name."]
        #[serde(rename = "accountName", default)]
        pub account_name: Option<String>,
        #[doc = "A list of adgroup IDs associated with this particular account. These IDs may show up as part of a realtime bidding BidRequest, which indicates a bid request for this account."]
        #[serde(rename = "billingId", default)]
        pub billing_id: Option<Vec<String>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for BillingInfo {
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
    pub struct BillingInfoList {
        #[doc = "A list of billing info relevant for your account."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::BillingInfo>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for BillingInfoList {
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
    pub struct Budget {
        #[doc = "The id of the account. This is required for get and update requests."]
        #[serde(rename = "accountId", default)]
        #[serde(with = "crate::parsed_string")]
        pub account_id: Option<i64>,
        #[doc = "The billing id to determine which adgroup to provide budget information for. This is required for get and update requests."]
        #[serde(rename = "billingId", default)]
        #[serde(with = "crate::parsed_string")]
        pub billing_id: Option<i64>,
        #[doc = "The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests."]
        #[serde(rename = "budgetAmount", default)]
        #[serde(with = "crate::parsed_string")]
        pub budget_amount: Option<i64>,
        #[doc = "The currency code for the buyer. This cannot be altered here."]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: Option<String>,
        #[doc = "The unique id that describes this item."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The kind of the resource, i.e. \"adexchangebuyer#budget\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for Budget {
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
    pub struct Buyer {
        #[doc = "Adx account id of the buyer."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Buyer {
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
    pub struct ContactInformation {
        #[doc = "Email address of the contact."]
        #[serde(rename = "email", default)]
        pub email: Option<String>,
        #[doc = "The name of the contact."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for ContactInformation {
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
    pub struct CreateOrdersRequest {
        #[doc = "The list of proposals to create."]
        #[serde(rename = "proposals", default)]
        pub proposals: Option<Vec<crate::schemas::Proposal>>,
        #[doc = "Web property id of the seller creating these orders"]
        #[serde(rename = "webPropertyCode", default)]
        pub web_property_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateOrdersRequest {
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
    pub struct CreateOrdersResponse {
        #[doc = "The list of proposals successfully created."]
        #[serde(rename = "proposals", default)]
        pub proposals: Option<Vec<crate::schemas::Proposal>>,
    }
    impl ::field_selector::FieldSelector for CreateOrdersResponse {
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
    pub struct CreativeAdTechnologyProviders {
        #[serde(rename = "detectedProviderIds", default)]
        pub detected_provider_ids: Option<Vec<i64>>,
        #[serde(rename = "hasUnidentifiedProvider", default)]
        pub has_unidentified_provider: Option<bool>,
    }
    impl ::field_selector::FieldSelector for CreativeAdTechnologyProviders {
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
    pub struct CreativeCorrectionsItemsContextsItems {
        #[doc = "Only set when contextType=AUCTION_TYPE. Represents the auction types this correction applies to."]
        #[serde(rename = "auctionType", default)]
        pub auction_type: Option<Vec<String>>,
        #[doc = "The type of context (e.g., location, platform, auction type, SSL-ness)."]
        #[serde(rename = "contextType", default)]
        pub context_type: Option<String>,
        #[doc = "Only set when contextType=LOCATION. Represents the geo criterias this correction applies to."]
        #[serde(rename = "geoCriteriaId", default)]
        pub geo_criteria_id: Option<Vec<i32>>,
        #[doc = "Only set when contextType=PLATFORM. Represents the platforms this correction applies to."]
        #[serde(rename = "platform", default)]
        pub platform: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for CreativeCorrectionsItemsContextsItems {
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
    pub struct CreativeCorrectionsItems {
        #[doc = "All known serving contexts containing serving status information."]
        #[serde(rename = "contexts", default)]
        pub contexts: Option<Vec<crate::schemas::CreativeCorrectionsItemsContextsItems>>,
        #[doc = "Additional details about the correction."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<String>>,
        #[doc = "The type of correction that was applied to the creative."]
        #[serde(rename = "reason", default)]
        pub reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreativeCorrectionsItems {
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
    pub struct CreativeFilteringReasonsReasonsItems {
        #[doc = "The number of times the creative was filtered for the status. The count is aggregated across all publishers on the exchange."]
        #[serde(rename = "filteringCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub filtering_count: Option<i64>,
        #[doc = "The filtering status code as defined in  creative-status-codes.txt."]
        #[serde(rename = "filteringStatus", default)]
        pub filtering_status: Option<i32>,
    }
    impl ::field_selector::FieldSelector for CreativeFilteringReasonsReasonsItems {
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
    pub struct CreativeFilteringReasons {
        #[doc = "The date in ISO 8601 format for the data. The data is collected from 00:00:00 to 23:59:59 in PST."]
        #[serde(rename = "date", default)]
        pub date: Option<String>,
        #[doc = "The filtering reasons."]
        #[serde(rename = "reasons", default)]
        pub reasons: Option<Vec<crate::schemas::CreativeFilteringReasonsReasonsItems>>,
    }
    impl ::field_selector::FieldSelector for CreativeFilteringReasons {
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
    pub struct CreativeNativeAdAppIcon {
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "url", default)]
        pub url: Option<String>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for CreativeNativeAdAppIcon {
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
    pub struct CreativeNativeAdImage {
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "url", default)]
        pub url: Option<String>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for CreativeNativeAdImage {
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
    pub struct CreativeNativeAdLogo {
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[serde(rename = "url", default)]
        pub url: Option<String>,
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for CreativeNativeAdLogo {
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
    pub struct CreativeNativeAd {
        #[serde(rename = "advertiser", default)]
        pub advertiser: Option<String>,
        #[doc = "The app icon, for app download ads."]
        #[serde(rename = "appIcon", default)]
        pub app_icon: Option<crate::schemas::CreativeNativeAdAppIcon>,
        #[doc = "A long description of the ad."]
        #[serde(rename = "body", default)]
        pub body: Option<String>,
        #[doc = "A label for the button that the user is supposed to click."]
        #[serde(rename = "callToAction", default)]
        pub call_to_action: Option<String>,
        #[doc = "The URL that the browser/SDK will load when the user clicks the ad."]
        #[serde(rename = "clickLinkUrl", default)]
        pub click_link_url: Option<String>,
        #[doc = "The URL to use for click tracking."]
        #[serde(rename = "clickTrackingUrl", default)]
        pub click_tracking_url: Option<String>,
        #[doc = "A short title for the ad."]
        #[serde(rename = "headline", default)]
        pub headline: Option<String>,
        #[doc = "A large image."]
        #[serde(rename = "image", default)]
        pub image: Option<crate::schemas::CreativeNativeAdImage>,
        #[doc = "The URLs are called when the impression is rendered."]
        #[serde(rename = "impressionTrackingUrl", default)]
        pub impression_tracking_url: Option<Vec<String>>,
        #[doc = "A smaller image, for the advertiser logo."]
        #[serde(rename = "logo", default)]
        pub logo: Option<crate::schemas::CreativeNativeAdLogo>,
        #[doc = "The price of the promoted app including the currency info."]
        #[serde(rename = "price", default)]
        pub price: Option<String>,
        #[doc = "The app rating in the app store. Must be in the range [0-5]."]
        #[serde(rename = "starRating", default)]
        pub star_rating: Option<f64>,
        #[doc = "The URL of the XML VAST for a native ad. Note this is a separate field from resource.video_url."]
        #[serde(rename = "videoURL", default)]
        pub video_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreativeNativeAd {
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
    pub struct CreativeServingRestrictionsItemsContextsItems {
        #[doc = "Only set when contextType=AUCTION_TYPE. Represents the auction types this restriction applies to."]
        #[serde(rename = "auctionType", default)]
        pub auction_type: Option<Vec<String>>,
        #[doc = "The type of context (e.g., location, platform, auction type, SSL-ness)."]
        #[serde(rename = "contextType", default)]
        pub context_type: Option<String>,
        #[doc = "Only set when contextType=LOCATION. Represents the geo criterias this restriction applies to. Impressions are considered to match a context if either the user location or publisher location matches a given geoCriteriaId."]
        #[serde(rename = "geoCriteriaId", default)]
        pub geo_criteria_id: Option<Vec<i32>>,
        #[doc = "Only set when contextType=PLATFORM. Represents the platforms this restriction applies to."]
        #[serde(rename = "platform", default)]
        pub platform: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for CreativeServingRestrictionsItemsContextsItems {
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
    pub struct CreativeServingRestrictionsItemsDisapprovalReasonsItems {
        #[doc = "Additional details about the reason for disapproval."]
        #[serde(rename = "details", default)]
        pub details: Option<Vec<String>>,
        #[doc = "The categorized reason for disapproval."]
        #[serde(rename = "reason", default)]
        pub reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreativeServingRestrictionsItemsDisapprovalReasonsItems {
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
    pub struct CreativeServingRestrictionsItems {
        #[doc = "All known contexts/restrictions."]
        #[serde(rename = "contexts", default)]
        pub contexts: Option<Vec<crate::schemas::CreativeServingRestrictionsItemsContextsItems>>,
        #[doc = "The reasons for disapproval within this restriction, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED or CONDITIONALLY_APPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue."]
        #[serde(rename = "disapprovalReasons", default)]
        pub disapproval_reasons:
            Option<Vec<crate::schemas::CreativeServingRestrictionsItemsDisapprovalReasonsItems>>,
        #[doc = "Why the creative is ineligible to serve in this context (e.g., it has been explicitly disapproved or is pending review)."]
        #[serde(rename = "reason", default)]
        pub reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreativeServingRestrictionsItems {
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
    pub struct Creative {
        #[doc = "Account id."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<i32>,
        #[doc = "The link to the Ad Preferences page. This is only supported for native ads."]
        #[serde(rename = "adChoicesDestinationUrl", default)]
        pub ad_choices_destination_url: Option<String>,
        #[serde(rename = "adTechnologyProviders", default)]
        pub ad_technology_providers: Option<crate::schemas::CreativeAdTechnologyProviders>,
        #[doc = "Detected advertiser id, if any. Read-only. This field should not be set in requests."]
        #[serde(rename = "advertiserId", default)]
        pub advertiser_id: Option<Vec<i64>>,
        #[doc = "The name of the company being advertised in the creative. The value provided must exist in the advertisers.txt file."]
        #[serde(rename = "advertiserName", default)]
        pub advertiser_name: Option<String>,
        #[doc = "The agency id for this creative."]
        #[serde(rename = "agencyId", default)]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: Option<i64>,
        #[doc = "The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp)."]
        #[serde(rename = "apiUploadTimestamp", default)]
        pub api_upload_timestamp: Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "List of buyer selectable attributes for the ads that may be shown from this snippet. Each attribute is represented by an integer as defined in  buyer-declarable-creative-attributes.txt."]
        #[serde(rename = "attribute", default)]
        pub attribute: Option<Vec<i32>>,
        #[doc = "A buyer-specific id identifying the creative in this ad."]
        #[serde(rename = "buyerCreativeId", default)]
        pub buyer_creative_id: Option<String>,
        #[doc = "The set of destination urls for the snippet."]
        #[serde(rename = "clickThroughUrl", default)]
        pub click_through_url: Option<Vec<String>>,
        #[doc = "Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests."]
        #[serde(rename = "corrections", default)]
        pub corrections: Option<Vec<crate::schemas::CreativeCorrectionsItems>>,
        #[doc = "Creative status identity type that the creative item applies to. Ad Exchange real-time bidding is migrating to the sizeless creative verification. Originally, Ad Exchange assigned creative verification status to a unique combination of a buyer creative ID and creative dimensions. Post-migration, a single verification status will be assigned at the buyer creative ID level. This field allows to distinguish whether a given creative status applies to a unique combination of a buyer creative ID and creative dimensions, or to a buyer creative ID as a whole."]
        #[serde(rename = "creativeStatusIdentityType", default)]
        pub creative_status_identity_type: Option<String>,
        #[doc = "Top-level deals status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=DIRECT_DEALS (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from servingRestrictions directly."]
        #[serde(rename = "dealsStatus", default)]
        pub deals_status: Option<String>,
        #[doc = "Detected domains for this creative. Read-only. This field should not be set in requests."]
        #[serde(rename = "detectedDomains", default)]
        pub detected_domains: Option<Vec<String>>,
        #[doc = "The filtering reasons for the creative. Read-only. This field should not be set in requests."]
        #[serde(rename = "filteringReasons", default)]
        pub filtering_reasons: Option<crate::schemas::CreativeFilteringReasons>,
        #[doc = "Ad height."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The HTML snippet that displays the ad when inserted in the web page. If set, videoURL, videoVastXML, and nativeAd should not be set."]
        #[serde(rename = "HTMLSnippet", default)]
        pub html_snippet: Option<String>,
        #[doc = "The set of urls to be called to record an impression."]
        #[serde(rename = "impressionTrackingUrl", default)]
        pub impression_tracking_url: Option<Vec<String>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Detected languages for this creative. Read-only. This field should not be set in requests."]
        #[serde(rename = "languages", default)]
        pub languages: Option<Vec<String>>,
        #[doc = "If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.)"]
        #[serde(rename = "nativeAd", default)]
        pub native_ad: Option<crate::schemas::CreativeNativeAd>,
        #[doc = "Top-level open auction status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=OPEN_AUCTION (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from ServingRestrictions directly."]
        #[serde(rename = "openAuctionStatus", default)]
        pub open_auction_status: Option<String>,
        #[doc = "Detected product categories, if any. Each category is represented by an integer as defined in  ad-product-categories.txt. Read-only. This field should not be set in requests."]
        #[serde(rename = "productCategories", default)]
        pub product_categories: Option<Vec<i32>>,
        #[doc = "All restricted categories for the ads that may be shown from this snippet. Each category is represented by an integer as defined in the  ad-restricted-categories.txt."]
        #[serde(rename = "restrictedCategories", default)]
        pub restricted_categories: Option<Vec<i32>>,
        #[doc = "Detected sensitive categories, if any. Each category is represented by an integer as defined in  ad-sensitive-categories.txt. Read-only. This field should not be set in requests."]
        #[serde(rename = "sensitiveCategories", default)]
        pub sensitive_categories: Option<Vec<i32>>,
        #[doc = "The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction). Read-only. This field should not be set in requests. See the examples in the Creatives guide for more details."]
        #[serde(rename = "servingRestrictions", default)]
        pub serving_restrictions: Option<Vec<crate::schemas::CreativeServingRestrictionsItems>>,
        #[doc = "List of vendor types for the ads that may be shown from this snippet. Each vendor type is represented by an integer as defined in vendors.txt."]
        #[serde(rename = "vendorType", default)]
        pub vendor_type: Option<Vec<i32>>,
        #[doc = "The version for this creative. Read-only. This field should not be set in requests."]
        #[serde(rename = "version", default)]
        pub version: Option<i32>,
        #[doc = "The URL to fetch a video ad. If set, HTMLSnippet, videoVastXML, and nativeAd should not be set. Note, this is different from resource.native_ad.video_url above."]
        #[serde(rename = "videoURL", default)]
        pub video_url: Option<String>,
        #[doc = "The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard. If set, HTMLSnippet, videoURL, and nativeAd and should not be set."]
        #[serde(rename = "videoVastXML", default)]
        pub video_vast_xml: Option<String>,
        #[doc = "Ad width."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Creative {
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
    pub struct CreativeDealIdsDealStatusesItems {
        #[doc = "ARC approval status."]
        #[serde(rename = "arcStatus", default)]
        pub arc_status: Option<String>,
        #[doc = "External deal ID."]
        #[serde(rename = "dealId", default)]
        #[serde(with = "crate::parsed_string")]
        pub deal_id: Option<i64>,
        #[doc = "Publisher ID."]
        #[serde(rename = "webPropertyId", default)]
        pub web_property_id: Option<i32>,
    }
    impl ::field_selector::FieldSelector for CreativeDealIdsDealStatusesItems {
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
    pub struct CreativeDealIds {
        #[doc = "A list of external deal ids and ARC approval status."]
        #[serde(rename = "dealStatuses", default)]
        pub deal_statuses: Option<Vec<crate::schemas::CreativeDealIdsDealStatusesItems>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreativeDealIds {
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
    pub struct CreativesList {
        #[doc = "A list of creatives."]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::Creative>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Continuation token used to page through creatives. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreativesList {
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
    pub struct DealServingMetadata {
        #[doc = "True if alcohol ads are allowed for this deal (read-only). This field is only populated when querying for finalized orders using the method GetFinalizedOrderDeals"]
        #[serde(rename = "alcoholAdsAllowed", default)]
        pub alcohol_ads_allowed: Option<bool>,
        #[doc = "Tracks which parties (if any) have paused a deal. (readonly, except via PauseResumeOrderDeals action)"]
        #[serde(rename = "dealPauseStatus", default)]
        pub deal_pause_status: Option<crate::schemas::DealServingMetadataDealPauseStatus>,
    }
    impl ::field_selector::FieldSelector for DealServingMetadata {
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
    pub struct DealServingMetadataDealPauseStatus {
        #[serde(rename = "buyerPauseReason", default)]
        pub buyer_pause_reason: Option<String>,
        #[doc = "If the deal is paused, records which party paused the deal first."]
        #[serde(rename = "firstPausedBy", default)]
        pub first_paused_by: Option<String>,
        #[serde(rename = "hasBuyerPaused", default)]
        pub has_buyer_paused: Option<bool>,
        #[serde(rename = "hasSellerPaused", default)]
        pub has_seller_paused: Option<bool>,
        #[serde(rename = "sellerPauseReason", default)]
        pub seller_pause_reason: Option<String>,
    }
    impl ::field_selector::FieldSelector for DealServingMetadataDealPauseStatus {
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
    pub struct DealTerms {
        #[doc = "Visibilty of the URL in bid requests."]
        #[serde(rename = "brandingType", default)]
        pub branding_type: Option<String>,
        #[doc = "Indicates that this ExternalDealId exists under at least two different AdxInventoryDeals. Currently, the only case that the same ExternalDealId will exist is programmatic cross sell case."]
        #[serde(rename = "crossListedExternalDealIdType", default)]
        pub cross_listed_external_deal_id_type: Option<String>,
        #[doc = "Description for the proposed terms of the deal."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "Non-binding estimate of the estimated gross spend for this deal Can be set by buyer or seller."]
        #[serde(rename = "estimatedGrossSpend", default)]
        pub estimated_gross_spend: Option<crate::schemas::Price>,
        #[doc = "Non-binding estimate of the impressions served per day Can be set by buyer or seller."]
        #[serde(rename = "estimatedImpressionsPerDay", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_impressions_per_day: Option<i64>,
        #[doc = "The terms for guaranteed fixed price deals."]
        #[serde(rename = "guaranteedFixedPriceTerms", default)]
        pub guaranteed_fixed_price_terms:
            Option<crate::schemas::DealTermsGuaranteedFixedPriceTerms>,
        #[doc = "The terms for non-guaranteed auction deals."]
        #[serde(rename = "nonGuaranteedAuctionTerms", default)]
        pub non_guaranteed_auction_terms:
            Option<crate::schemas::DealTermsNonGuaranteedAuctionTerms>,
        #[doc = "The terms for non-guaranteed fixed price deals."]
        #[serde(rename = "nonGuaranteedFixedPriceTerms", default)]
        pub non_guaranteed_fixed_price_terms:
            Option<crate::schemas::DealTermsNonGuaranteedFixedPriceTerms>,
        #[doc = "The terms for rubicon non-guaranteed deals."]
        #[serde(rename = "rubiconNonGuaranteedTerms", default)]
        pub rubicon_non_guaranteed_terms:
            Option<crate::schemas::DealTermsRubiconNonGuaranteedTerms>,
        #[doc = "For deals with Cost Per Day billing, defines the timezone used to mark the boundaries of a day (buyer-readonly)"]
        #[serde(rename = "sellerTimeZone", default)]
        pub seller_time_zone: Option<String>,
    }
    impl ::field_selector::FieldSelector for DealTerms {
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
    pub struct DealTermsGuaranteedFixedPriceTerms {
        #[doc = "External billing info for this Deal. This field is relevant when external billing info such as price has a different currency code than DFP/AdX."]
        #[serde(rename = "billingInfo", default)]
        pub billing_info: Option<crate::schemas::DealTermsGuaranteedFixedPriceTermsBillingInfo>,
        #[doc = "Fixed price for the specified buyer."]
        #[serde(rename = "fixedPrices", default)]
        pub fixed_prices: Option<Vec<crate::schemas::PricePerBuyer>>,
        #[doc = "Guaranteed impressions as a percentage. This is the percentage of guaranteed looks that the buyer is guaranteeing to buy."]
        #[serde(rename = "guaranteedImpressions", default)]
        #[serde(with = "crate::parsed_string")]
        pub guaranteed_impressions: Option<i64>,
        #[doc = "Count of guaranteed looks. Required for deal, optional for product. For CPD deals, buyer changes to guaranteed_looks will be ignored."]
        #[serde(rename = "guaranteedLooks", default)]
        #[serde(with = "crate::parsed_string")]
        pub guaranteed_looks: Option<i64>,
        #[doc = "Count of minimum daily looks for a CPD deal. For CPD deals, buyer should negotiate on this field instead of guaranteed_looks."]
        #[serde(rename = "minimumDailyLooks", default)]
        #[serde(with = "crate::parsed_string")]
        pub minimum_daily_looks: Option<i64>,
    }
    impl ::field_selector::FieldSelector for DealTermsGuaranteedFixedPriceTerms {
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
    pub struct DealTermsGuaranteedFixedPriceTermsBillingInfo {
        #[doc = "The timestamp (in ms since epoch) when the original reservation price for the deal was first converted to DFP currency. This is used to convert the contracted price into buyer's currency without discrepancy."]
        #[serde(rename = "currencyConversionTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub currency_conversion_time_ms: Option<i64>,
        #[doc = "The DFP line item id associated with this deal. For features like CPD, buyers can retrieve the DFP line item for billing reconciliation."]
        #[serde(rename = "dfpLineItemId", default)]
        #[serde(with = "crate::parsed_string")]
        pub dfp_line_item_id: Option<i64>,
        #[doc = "The original contracted quantity (# impressions) for this deal. To ensure delivery, sometimes the publisher will book the deal with a impression buffer, such that guaranteed_looks is greater than the contracted quantity. However clients are billed using the original contracted quantity."]
        #[serde(rename = "originalContractedQuantity", default)]
        #[serde(with = "crate::parsed_string")]
        pub original_contracted_quantity: Option<i64>,
        #[doc = "The original reservation price for the deal, if the currency code is different from the one used in negotiation."]
        #[serde(rename = "price", default)]
        pub price: Option<crate::schemas::Price>,
    }
    impl ::field_selector::FieldSelector for DealTermsGuaranteedFixedPriceTermsBillingInfo {
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
    pub struct DealTermsNonGuaranteedAuctionTerms {
        #[doc = "True if open auction buyers are allowed to compete with invited buyers in this private auction (buyer-readonly)."]
        #[serde(rename = "autoOptimizePrivateAuction", default)]
        pub auto_optimize_private_auction: Option<bool>,
        #[doc = "Reserve price for the specified buyer."]
        #[serde(rename = "reservePricePerBuyers", default)]
        pub reserve_price_per_buyers: Option<Vec<crate::schemas::PricePerBuyer>>,
    }
    impl ::field_selector::FieldSelector for DealTermsNonGuaranteedAuctionTerms {
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
    pub struct DealTermsNonGuaranteedFixedPriceTerms {
        #[doc = "Fixed price for the specified buyer."]
        #[serde(rename = "fixedPrices", default)]
        pub fixed_prices: Option<Vec<crate::schemas::PricePerBuyer>>,
    }
    impl ::field_selector::FieldSelector for DealTermsNonGuaranteedFixedPriceTerms {
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
    pub struct DealTermsRubiconNonGuaranteedTerms {
        #[doc = "Optional price for Rubicon priority access in the auction."]
        #[serde(rename = "priorityPrice", default)]
        pub priority_price: Option<crate::schemas::Price>,
        #[doc = "Optional price for Rubicon standard access in the auction."]
        #[serde(rename = "standardPrice", default)]
        pub standard_price: Option<crate::schemas::Price>,
    }
    impl ::field_selector::FieldSelector for DealTermsRubiconNonGuaranteedTerms {
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
    pub struct DeleteOrderDealsRequest {
        #[doc = "List of deals to delete for a given proposal"]
        #[serde(rename = "dealIds", default)]
        pub deal_ids: Option<Vec<String>>,
        #[doc = "The last known proposal revision number."]
        #[serde(rename = "proposalRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: Option<i64>,
        #[doc = "Indicates an optional action to take on the proposal"]
        #[serde(rename = "updateAction", default)]
        pub update_action: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeleteOrderDealsRequest {
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
    pub struct DeleteOrderDealsResponse {
        #[doc = "List of deals deleted (in the same proposal as passed in the request)"]
        #[serde(rename = "deals", default)]
        pub deals: Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The updated revision number for the proposal."]
        #[serde(rename = "proposalRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: Option<i64>,
    }
    impl ::field_selector::FieldSelector for DeleteOrderDealsResponse {
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
    pub struct DeliveryControl {
        #[serde(rename = "creativeBlockingLevel", default)]
        pub creative_blocking_level: Option<String>,
        #[serde(rename = "deliveryRateType", default)]
        pub delivery_rate_type: Option<String>,
        #[serde(rename = "frequencyCaps", default)]
        pub frequency_caps: Option<Vec<crate::schemas::DeliveryControlFrequencyCap>>,
    }
    impl ::field_selector::FieldSelector for DeliveryControl {
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
    pub struct DeliveryControlFrequencyCap {
        #[serde(rename = "maxImpressions", default)]
        pub max_impressions: Option<i32>,
        #[serde(rename = "numTimeUnits", default)]
        pub num_time_units: Option<i32>,
        #[serde(rename = "timeUnitType", default)]
        pub time_unit_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeliveryControlFrequencyCap {
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
    pub struct Dimension {
        #[serde(rename = "dimensionType", default)]
        pub dimension_type: Option<String>,
        #[serde(rename = "dimensionValues", default)]
        pub dimension_values: Option<Vec<crate::schemas::DimensionDimensionValue>>,
    }
    impl ::field_selector::FieldSelector for Dimension {
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
    pub struct DimensionDimensionValue {
        #[doc = "Id of the dimension."]
        #[serde(rename = "id", default)]
        pub id: Option<i32>,
        #[doc = "Name of the dimension mainly for debugging purposes, except for the case of CREATIVE_SIZE. For CREATIVE_SIZE, strings are used instead of ids."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Percent of total impressions for a dimension type. e.g. {dimension_type: 'GENDER', [{dimension_value: {id: 1, name: 'MALE', percentage: 60}}]} Gender MALE is 60% of all impressions which have gender."]
        #[serde(rename = "percentage", default)]
        pub percentage: Option<i32>,
    }
    impl ::field_selector::FieldSelector for DimensionDimensionValue {
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
    pub struct EditAllOrderDealsRequest {
        #[doc = "List of deals to edit. Service may perform 3 different operations based on comparison of deals in this list vs deals already persisted in database: 1. Add new deal to proposal If a deal in this list does not exist in the proposal, the service will create a new deal and add it to the proposal. Validation will follow AddOrderDealsRequest. 2. Update existing deal in the proposal If a deal in this list already exist in the proposal, the service will update that existing deal to this new deal in the request. Validation will follow UpdateOrderDealsRequest. 3. Delete deals from the proposal (just need the id) If a existing deal in the proposal is not present in this list, the service will delete that deal from the proposal. Validation will follow DeleteOrderDealsRequest."]
        #[serde(rename = "deals", default)]
        pub deals: Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "If specified, also updates the proposal in the batch transaction. This is useful when the proposal and the deals need to be updated in one transaction."]
        #[serde(rename = "proposal", default)]
        pub proposal: Option<crate::schemas::Proposal>,
        #[doc = "The last known revision number for the proposal."]
        #[serde(rename = "proposalRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: Option<i64>,
        #[doc = "Indicates an optional action to take on the proposal"]
        #[serde(rename = "updateAction", default)]
        pub update_action: Option<String>,
    }
    impl ::field_selector::FieldSelector for EditAllOrderDealsRequest {
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
    pub struct EditAllOrderDealsResponse {
        #[doc = "List of all deals in the proposal after edit."]
        #[serde(rename = "deals", default)]
        pub deals: Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The latest revision number after the update has been applied."]
        #[serde(rename = "orderRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub order_revision_number: Option<i64>,
    }
    impl ::field_selector::FieldSelector for EditAllOrderDealsResponse {
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
    pub struct GetOffersResponse {
        #[doc = "The returned list of products."]
        #[serde(rename = "products", default)]
        pub products: Option<Vec<crate::schemas::Product>>,
    }
    impl ::field_selector::FieldSelector for GetOffersResponse {
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
    pub struct GetOrderDealsResponse {
        #[doc = "List of deals for the proposal"]
        #[serde(rename = "deals", default)]
        pub deals: Option<Vec<crate::schemas::MarketplaceDeal>>,
    }
    impl ::field_selector::FieldSelector for GetOrderDealsResponse {
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
    pub struct GetOrderNotesResponse {
        #[doc = "The list of matching notes. The notes for a proposal are ordered from oldest to newest. If the notes span multiple proposals, they will be grouped by proposal, with the notes for the most recently modified proposal appearing first."]
        #[serde(rename = "notes", default)]
        pub notes: Option<Vec<crate::schemas::MarketplaceNote>>,
    }
    impl ::field_selector::FieldSelector for GetOrderNotesResponse {
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
    pub struct GetOrdersResponse {
        #[doc = "The list of matching proposals."]
        #[serde(rename = "proposals", default)]
        pub proposals: Option<Vec<crate::schemas::Proposal>>,
    }
    impl ::field_selector::FieldSelector for GetOrdersResponse {
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
    pub struct GetPublisherProfilesByAccountIdResponse {
        #[doc = "Profiles for the requested publisher"]
        #[serde(rename = "profiles", default)]
        pub profiles: Option<Vec<crate::schemas::PublisherProfileApiProto>>,
    }
    impl ::field_selector::FieldSelector for GetPublisherProfilesByAccountIdResponse {
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
    pub struct MarketplaceDeal {
        #[doc = "Buyer private data (hidden from seller)."]
        #[serde(rename = "buyerPrivateData", default)]
        pub buyer_private_data: Option<crate::schemas::PrivateData>,
        #[doc = "The time (ms since epoch) of the deal creation. (readonly)"]
        #[serde(rename = "creationTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time_ms: Option<i64>,
        #[doc = "Specifies the creative pre-approval policy (buyer-readonly)"]
        #[serde(rename = "creativePreApprovalPolicy", default)]
        pub creative_pre_approval_policy: Option<String>,
        #[doc = "Specifies whether the creative is safeFrame compatible (buyer-readonly)"]
        #[serde(rename = "creativeSafeFrameCompatibility", default)]
        pub creative_safe_frame_compatibility: Option<String>,
        #[doc = "A unique deal-id for the deal (readonly)."]
        #[serde(rename = "dealId", default)]
        pub deal_id: Option<String>,
        #[doc = "Metadata about the serving status of this deal (readonly, writes via custom actions)"]
        #[serde(rename = "dealServingMetadata", default)]
        pub deal_serving_metadata: Option<crate::schemas::DealServingMetadata>,
        #[doc = "The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension."]
        #[serde(rename = "deliveryControl", default)]
        pub delivery_control: Option<crate::schemas::DeliveryControl>,
        #[doc = "The external deal id assigned to this deal once the deal is finalized. This is the deal-id that shows up in serving/reporting etc. (readonly)"]
        #[serde(rename = "externalDealId", default)]
        pub external_deal_id: Option<String>,
        #[doc = "Proposed flight end time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)"]
        #[serde(rename = "flightEndTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub flight_end_time_ms: Option<i64>,
        #[doc = "Proposed flight start time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)"]
        #[serde(rename = "flightStartTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub flight_start_time_ms: Option<i64>,
        #[doc = "Description for the deal terms. (buyer-readonly)"]
        #[serde(rename = "inventoryDescription", default)]
        pub inventory_description: Option<String>,
        #[doc = "Indicates whether the current deal is a RFP template. RFP template is created by buyer and not based on seller created products."]
        #[serde(rename = "isRfpTemplate", default)]
        pub is_rfp_template: Option<bool>,
        #[doc = "True, if the buyside inventory setup is complete for this deal. (readonly, except via OrderSetupCompleted action)"]
        #[serde(rename = "isSetupComplete", default)]
        pub is_setup_complete: Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#marketplaceDeal\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The time (ms since epoch) when the deal was last updated. (readonly)"]
        #[serde(rename = "lastUpdateTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_update_time_ms: Option<i64>,
        #[doc = "The name of the deal. (updatable)"]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "The product-id from which this deal was created. (readonly, except on create)"]
        #[serde(rename = "productId", default)]
        pub product_id: Option<String>,
        #[doc = "The revision number of the product that the deal was created from (readonly, except on create)"]
        #[serde(rename = "productRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub product_revision_number: Option<i64>,
        #[doc = "Specifies the creative source for programmatic deals, PUBLISHER means creative is provided by seller and ADVERTISR means creative is provided by buyer. (buyer-readonly)"]
        #[serde(rename = "programmaticCreativeSource", default)]
        pub programmatic_creative_source: Option<String>,
        #[serde(rename = "proposalId", default)]
        pub proposal_id: Option<String>,
        #[doc = "Optional Seller contact information for the deal (buyer-readonly)"]
        #[serde(rename = "sellerContacts", default)]
        pub seller_contacts: Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "The shared targeting visible to buyers and sellers. Each shared targeting entity is AND'd together. (updatable)"]
        #[serde(rename = "sharedTargetings", default)]
        pub shared_targetings: Option<Vec<crate::schemas::SharedTargeting>>,
        #[doc = "The syndication product associated with the deal. (readonly, except on create)"]
        #[serde(rename = "syndicationProduct", default)]
        pub syndication_product: Option<String>,
        #[doc = "The negotiable terms of the deal. (updatable)"]
        #[serde(rename = "terms", default)]
        pub terms: Option<crate::schemas::DealTerms>,
        #[serde(rename = "webPropertyCode", default)]
        pub web_property_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for MarketplaceDeal {
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
    pub struct MarketplaceDealParty {
        #[doc = "The buyer/seller associated with the deal. One of buyer/seller is specified for a deal-party."]
        #[serde(rename = "buyer", default)]
        pub buyer: Option<crate::schemas::Buyer>,
        #[doc = "The buyer/seller associated with the deal. One of buyer/seller is specified for a deal party."]
        #[serde(rename = "seller", default)]
        pub seller: Option<crate::schemas::Seller>,
    }
    impl ::field_selector::FieldSelector for MarketplaceDealParty {
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
    pub struct MarketplaceLabel {
        #[doc = "The accountId of the party that created the label."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "The creation time (in ms since epoch) for the label."]
        #[serde(rename = "createTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub create_time_ms: Option<i64>,
        #[doc = "Information about the party that created the label."]
        #[serde(rename = "deprecatedMarketplaceDealParty", default)]
        pub deprecated_marketplace_deal_party: Option<crate::schemas::MarketplaceDealParty>,
        #[doc = "The label to use."]
        #[serde(rename = "label", default)]
        pub label: Option<String>,
    }
    impl ::field_selector::FieldSelector for MarketplaceLabel {
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
    pub struct MarketplaceNote {
        #[doc = "The role of the person (buyer/seller) creating the note. (readonly)"]
        #[serde(rename = "creatorRole", default)]
        pub creator_role: Option<String>,
        #[doc = "Notes can optionally be associated with a deal. (readonly, except on create)"]
        #[serde(rename = "dealId", default)]
        pub deal_id: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#marketplaceNote\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The actual note to attach. (readonly, except on create)"]
        #[serde(rename = "note", default)]
        pub note: Option<String>,
        #[doc = "The unique id for the note. (readonly)"]
        #[serde(rename = "noteId", default)]
        pub note_id: Option<String>,
        #[doc = "The proposalId that a note is attached to. (readonly)"]
        #[serde(rename = "proposalId", default)]
        pub proposal_id: Option<String>,
        #[doc = "If the note is associated with a proposal revision number, then store that here. (readonly, except on create)"]
        #[serde(rename = "proposalRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: Option<i64>,
        #[doc = "The timestamp (ms since epoch) that this note was created. (readonly)"]
        #[serde(rename = "timestampMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub timestamp_ms: Option<i64>,
    }
    impl ::field_selector::FieldSelector for MarketplaceNote {
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
    pub struct PerformanceReport {
        #[doc = "The number of bid responses with an ad."]
        #[serde(rename = "bidRate", default)]
        pub bid_rate: Option<f64>,
        #[doc = "The number of bid requests sent to your bidder."]
        #[serde(rename = "bidRequestRate", default)]
        pub bid_request_rate: Option<f64>,
        #[doc = "Rate of various prefiltering statuses per match. Please refer to the callout-status-codes.txt file for different statuses."]
        #[serde(rename = "calloutStatusRate", default)]
        pub callout_status_rate: Option<Vec<::serde_json::Value>>,
        #[doc = "Average QPS for cookie matcher operations."]
        #[serde(rename = "cookieMatcherStatusRate", default)]
        pub cookie_matcher_status_rate: Option<Vec<::serde_json::Value>>,
        #[doc = "Rate of ads with a given status. Please refer to the creative-status-codes.txt file for different statuses."]
        #[serde(rename = "creativeStatusRate", default)]
        pub creative_status_rate: Option<Vec<::serde_json::Value>>,
        #[doc = "The number of bid responses that were filtered due to a policy violation or other errors."]
        #[serde(rename = "filteredBidRate", default)]
        pub filtered_bid_rate: Option<f64>,
        #[doc = "Average QPS for hosted match operations."]
        #[serde(rename = "hostedMatchStatusRate", default)]
        pub hosted_match_status_rate: Option<Vec<::serde_json::Value>>,
        #[doc = "The number of potential queries based on your pretargeting settings."]
        #[serde(rename = "inventoryMatchRate", default)]
        pub inventory_match_rate: Option<f64>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The 50th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
        #[serde(rename = "latency50thPercentile", default)]
        pub latency_5_0th_percentile: Option<f64>,
        #[doc = "The 85th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
        #[serde(rename = "latency85thPercentile", default)]
        pub latency_8_5th_percentile: Option<f64>,
        #[doc = "The 95th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
        #[serde(rename = "latency95thPercentile", default)]
        pub latency_9_5th_percentile: Option<f64>,
        #[doc = "Rate of various quota account statuses per quota check."]
        #[serde(rename = "noQuotaInRegion", default)]
        pub no_quota_in_region: Option<f64>,
        #[doc = "Rate of various quota account statuses per quota check."]
        #[serde(rename = "outOfQuota", default)]
        pub out_of_quota: Option<f64>,
        #[doc = "Average QPS for pixel match requests from clients."]
        #[serde(rename = "pixelMatchRequests", default)]
        pub pixel_match_requests: Option<f64>,
        #[doc = "Average QPS for pixel match responses from clients."]
        #[serde(rename = "pixelMatchResponses", default)]
        pub pixel_match_responses: Option<f64>,
        #[doc = "The configured quota limits for this account."]
        #[serde(rename = "quotaConfiguredLimit", default)]
        pub quota_configured_limit: Option<f64>,
        #[doc = "The throttled quota limits for this account."]
        #[serde(rename = "quotaThrottledLimit", default)]
        pub quota_throttled_limit: Option<f64>,
        #[doc = "The trading location of this data."]
        #[serde(rename = "region", default)]
        pub region: Option<String>,
        #[doc = "The number of properly formed bid responses received by our servers within the deadline."]
        #[serde(rename = "successfulRequestRate", default)]
        pub successful_request_rate: Option<f64>,
        #[doc = "The unix timestamp of the starting time of this performance data."]
        #[serde(rename = "timestamp", default)]
        #[serde(with = "crate::parsed_string")]
        pub timestamp: Option<i64>,
        #[doc = "The number of bid responses that were unsuccessful due to timeouts, incorrect formatting, etc."]
        #[serde(rename = "unsuccessfulRequestRate", default)]
        pub unsuccessful_request_rate: Option<f64>,
    }
    impl ::field_selector::FieldSelector for PerformanceReport {
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
    pub struct PerformanceReportList {
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "A list of performance reports relevant for the account."]
        #[serde(rename = "performanceReport", default)]
        pub performance_report: Option<Vec<crate::schemas::PerformanceReport>>,
    }
    impl ::field_selector::FieldSelector for PerformanceReportList {
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
    pub struct PretargetingConfigDimensionsItems {
        #[doc = "Height in pixels."]
        #[serde(rename = "height", default)]
        #[serde(with = "crate::parsed_string")]
        pub height: Option<i64>,
        #[doc = "Width in pixels."]
        #[serde(rename = "width", default)]
        #[serde(with = "crate::parsed_string")]
        pub width: Option<i64>,
    }
    impl ::field_selector::FieldSelector for PretargetingConfigDimensionsItems {
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
    pub struct PretargetingConfigExcludedPlacementsItems {
        #[doc = "The type of the placement."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement."]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
    }
    impl ::field_selector::FieldSelector for PretargetingConfigExcludedPlacementsItems {
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
    pub struct PretargetingConfigPlacementsItems {
        #[doc = "The type of the placement."]
        #[serde(rename = "type", default)]
        pub r#type: Option<String>,
        #[doc = "The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement."]
        #[serde(rename = "token", default)]
        pub token: Option<String>,
    }
    impl ::field_selector::FieldSelector for PretargetingConfigPlacementsItems {
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
    pub struct PretargetingConfigVideoPlayerSizesItems {
        #[doc = "The type of aspect ratio. Leave this field blank to match all aspect ratios."]
        #[serde(rename = "aspectRatio", default)]
        pub aspect_ratio: Option<String>,
        #[doc = "The minimum player height in pixels. Leave this field blank to match any player height."]
        #[serde(rename = "minHeight", default)]
        #[serde(with = "crate::parsed_string")]
        pub min_height: Option<i64>,
        #[doc = "The minimum player width in pixels. Leave this field blank to match any player width."]
        #[serde(rename = "minWidth", default)]
        #[serde(with = "crate::parsed_string")]
        pub min_width: Option<i64>,
    }
    impl ::field_selector::FieldSelector for PretargetingConfigVideoPlayerSizesItems {
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
    pub struct PretargetingConfig {
        #[doc = "The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically."]
        #[serde(rename = "billingId", default)]
        #[serde(with = "crate::parsed_string")]
        pub billing_id: Option<i64>,
        #[doc = "The config id; generated automatically. Leave this field blank for insert requests."]
        #[serde(rename = "configId", default)]
        #[serde(with = "crate::parsed_string")]
        pub config_id: Option<i64>,
        #[doc = "The name of the config. Must be unique. Required for all requests."]
        #[serde(rename = "configName", default)]
        pub config_name: Option<String>,
        #[doc = "List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO."]
        #[serde(rename = "creativeType", default)]
        pub creative_type: Option<Vec<String>>,
        #[doc = "Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: Option<Vec<crate::schemas::PretargetingConfigDimensionsItems>>,
        #[doc = "Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section."]
        #[serde(rename = "excludedContentLabels", default)]
        pub excluded_content_labels: Option<Vec<i64>>,
        #[doc = "Requests containing any of these geo criteria ids will not match."]
        #[serde(rename = "excludedGeoCriteriaIds", default)]
        pub excluded_geo_criteria_ids: Option<Vec<i64>>,
        #[doc = "Requests containing any of these placements will not match."]
        #[serde(rename = "excludedPlacements", default)]
        pub excluded_placements:
            Option<Vec<crate::schemas::PretargetingConfigExcludedPlacementsItems>>,
        #[doc = "Requests containing any of these users list ids will not match."]
        #[serde(rename = "excludedUserLists", default)]
        pub excluded_user_lists: Option<Vec<i64>>,
        #[doc = "Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section."]
        #[serde(rename = "excludedVerticals", default)]
        pub excluded_verticals: Option<Vec<i64>>,
        #[doc = "Requests containing any of these geo criteria ids will match."]
        #[serde(rename = "geoCriteriaIds", default)]
        pub geo_criteria_ids: Option<Vec<i64>>,
        #[doc = "Whether this config is active. Required for all requests."]
        #[serde(rename = "isActive", default)]
        pub is_active: Option<bool>,
        #[doc = "The kind of the resource, i.e. \"adexchangebuyer#pretargetingConfig\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Request containing any of these language codes will match."]
        #[serde(rename = "languages", default)]
        pub languages: Option<Vec<String>>,
        #[doc = "Requests where the predicted viewability is below the specified decile will not match. E.g. if the buyer sets this value to 5, requests from slots where the predicted viewability is below 50% will not match. If the predicted viewability is unknown this field will be ignored."]
        #[serde(rename = "minimumViewabilityDecile", default)]
        pub minimum_viewability_decile: Option<i32>,
        #[doc = "Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section."]
        #[serde(rename = "mobileCarriers", default)]
        pub mobile_carriers: Option<Vec<i64>>,
        #[doc = "Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section."]
        #[serde(rename = "mobileDevices", default)]
        pub mobile_devices: Option<Vec<i64>>,
        #[doc = "Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section."]
        #[serde(rename = "mobileOperatingSystemVersions", default)]
        pub mobile_operating_system_versions: Option<Vec<i64>>,
        #[doc = "Requests containing any of these placements will match."]
        #[serde(rename = "placements", default)]
        pub placements: Option<Vec<crate::schemas::PretargetingConfigPlacementsItems>>,
        #[doc = "Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET."]
        #[serde(rename = "platforms", default)]
        pub platforms: Option<Vec<String>>,
        #[doc = "Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section."]
        #[serde(rename = "supportedCreativeAttributes", default)]
        pub supported_creative_attributes: Option<Vec<i64>>,
        #[doc = "Requests containing the specified type of user data will match. Possible values are HOSTED_MATCH_DATA, which means the request is cookie-targetable and has a match in the buyer's hosted match table, and COOKIE_OR_IDFA, which means the request has either a targetable cookie or an iOS IDFA."]
        #[serde(rename = "userIdentifierDataRequired", default)]
        pub user_identifier_data_required: Option<Vec<String>>,
        #[doc = "Requests containing any of these user list ids will match."]
        #[serde(rename = "userLists", default)]
        pub user_lists: Option<Vec<i64>>,
        #[doc = "Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section."]
        #[serde(rename = "vendorTypes", default)]
        pub vendor_types: Option<Vec<i64>>,
        #[doc = "Requests containing any of these vertical ids will match."]
        #[serde(rename = "verticals", default)]
        pub verticals: Option<Vec<i64>>,
        #[doc = "Video requests satisfying any of these player size constraints will match."]
        #[serde(rename = "videoPlayerSizes", default)]
        pub video_player_sizes:
            Option<Vec<crate::schemas::PretargetingConfigVideoPlayerSizesItems>>,
    }
    impl ::field_selector::FieldSelector for PretargetingConfig {
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
    pub struct PretargetingConfigList {
        #[doc = "A list of pretargeting configs"]
        #[serde(rename = "items", default)]
        pub items: Option<Vec<crate::schemas::PretargetingConfig>>,
        #[doc = "Resource type."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
    }
    impl ::field_selector::FieldSelector for PretargetingConfigList {
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
    pub struct Price {
        #[doc = "The price value in micros."]
        #[serde(rename = "amountMicros", default)]
        pub amount_micros: Option<f64>,
        #[doc = "The currency code for the price."]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: Option<String>,
        #[doc = "In case of CPD deals, the expected CPM in micros."]
        #[serde(rename = "expectedCpmMicros", default)]
        pub expected_cpm_micros: Option<f64>,
        #[doc = "The pricing type for the deal/product."]
        #[serde(rename = "pricingType", default)]
        pub pricing_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for Price {
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
    pub struct PricePerBuyer {
        #[doc = "Optional access type for this buyer."]
        #[serde(rename = "auctionTier", default)]
        pub auction_tier: Option<String>,
        #[doc = "Reference to the buyer that will get billed."]
        #[serde(rename = "billedBuyer", default)]
        pub billed_buyer: Option<crate::schemas::Buyer>,
        #[doc = "The buyer who will pay this price. If unset, all buyers can pay this price (if the advertisers match, and there's no more specific rule matching the buyer)."]
        #[serde(rename = "buyer", default)]
        pub buyer: Option<crate::schemas::Buyer>,
        #[doc = "The specified price"]
        #[serde(rename = "price", default)]
        pub price: Option<crate::schemas::Price>,
    }
    impl ::field_selector::FieldSelector for PricePerBuyer {
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
    pub struct PrivateData {
        #[serde(rename = "referenceId", default)]
        pub reference_id: Option<String>,
        #[serde(rename = "referencePayload", default)]
        pub reference_payload: Option<Vec<u8>>,
    }
    impl ::field_selector::FieldSelector for PrivateData {
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
    pub struct Product {
        #[doc = "The billed buyer corresponding to the buyer that created the offer. (readonly, except on create)"]
        #[serde(rename = "billedBuyer", default)]
        pub billed_buyer: Option<crate::schemas::Buyer>,
        #[doc = "The buyer that created the offer if this is a buyer initiated offer (readonly, except on create)"]
        #[serde(rename = "buyer", default)]
        pub buyer: Option<crate::schemas::Buyer>,
        #[doc = "Creation time in ms. since epoch (readonly)"]
        #[serde(rename = "creationTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time_ms: Option<i64>,
        #[doc = "Optional contact information for the creator of this product. (buyer-readonly)"]
        #[serde(rename = "creatorContacts", default)]
        pub creator_contacts: Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "The role that created the offer. Set to BUYER for buyer initiated offers."]
        #[serde(rename = "creatorRole", default)]
        pub creator_role: Option<String>,
        #[doc = "The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension."]
        #[serde(rename = "deliveryControl", default)]
        pub delivery_control: Option<crate::schemas::DeliveryControl>,
        #[doc = "The proposed end time for the deal (ms since epoch) (buyer-readonly)"]
        #[serde(rename = "flightEndTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub flight_end_time_ms: Option<i64>,
        #[doc = "Inventory availability dates. (times are in ms since epoch) The granularity is generally in the order of seconds. (buyer-readonly)"]
        #[serde(rename = "flightStartTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub flight_start_time_ms: Option<i64>,
        #[doc = "If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false."]
        #[serde(rename = "hasCreatorSignedOff", default)]
        pub has_creator_signed_off: Option<bool>,
        #[doc = "What exchange will provide this inventory (readonly, except on create)."]
        #[serde(rename = "inventorySource", default)]
        pub inventory_source: Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#product\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "Optional List of labels for the product (optional, buyer-readonly)."]
        #[serde(rename = "labels", default)]
        pub labels: Option<Vec<crate::schemas::MarketplaceLabel>>,
        #[doc = "Time of last update in ms. since epoch (readonly)"]
        #[serde(rename = "lastUpdateTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub last_update_time_ms: Option<i64>,
        #[doc = "Optional legacy offer id if this offer is a preferred deal offer."]
        #[serde(rename = "legacyOfferId", default)]
        pub legacy_offer_id: Option<String>,
        #[doc = "Marketplace publisher profile Id. This Id differs from the regular publisher_profile_id in that 1. This is a new id, the old Id will be deprecated in 2017. 2. This id uniquely identifies a publisher profile by itself."]
        #[serde(rename = "marketplacePublisherProfileId", default)]
        pub marketplace_publisher_profile_id: Option<String>,
        #[doc = "The name for this product as set by the seller. (buyer-readonly)"]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Optional private auction id if this offer is a private auction offer."]
        #[serde(rename = "privateAuctionId", default)]
        pub private_auction_id: Option<String>,
        #[doc = "The unique id for the product (readonly)"]
        #[serde(rename = "productId", default)]
        pub product_id: Option<String>,
        #[doc = "Id of the publisher profile for a given seller. A (seller.account_id, publisher_profile_id) pair uniquely identifies a publisher profile. Buyers can call the PublisherProfiles::List endpoint to get a list of publisher profiles for a given seller."]
        #[serde(rename = "publisherProfileId", default)]
        pub publisher_profile_id: Option<String>,
        #[doc = "Publisher self-provided forecast information."]
        #[serde(rename = "publisherProvidedForecast", default)]
        pub publisher_provided_forecast: Option<crate::schemas::PublisherProvidedForecast>,
        #[doc = "The revision number of the product. (readonly)"]
        #[serde(rename = "revisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub revision_number: Option<i64>,
        #[doc = "Information about the seller that created this product (readonly, except on create)"]
        #[serde(rename = "seller", default)]
        pub seller: Option<crate::schemas::Seller>,
        #[doc = "Targeting that is shared between the buyer and the seller. Each targeting criteria has a specified key and for each key there is a list of inclusion value or exclusion values. (buyer-readonly)"]
        #[serde(rename = "sharedTargetings", default)]
        pub shared_targetings: Option<Vec<crate::schemas::SharedTargeting>>,
        #[doc = "The state of the product. (buyer-readonly)"]
        #[serde(rename = "state", default)]
        pub state: Option<String>,
        #[doc = "The syndication product associated with the deal. (readonly, except on create)"]
        #[serde(rename = "syndicationProduct", default)]
        pub syndication_product: Option<String>,
        #[doc = "The negotiable terms of the deal (buyer-readonly)"]
        #[serde(rename = "terms", default)]
        pub terms: Option<crate::schemas::DealTerms>,
        #[doc = "The web property code for the seller. This field is meant to be copied over as is when creating deals."]
        #[serde(rename = "webPropertyCode", default)]
        pub web_property_code: Option<String>,
    }
    impl ::field_selector::FieldSelector for Product {
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
    pub struct Proposal {
        #[doc = "Reference to the buyer that will get billed for this proposal. (readonly)"]
        #[serde(rename = "billedBuyer", default)]
        pub billed_buyer: Option<crate::schemas::Buyer>,
        #[doc = "Reference to the buyer on the proposal. (readonly, except on create)"]
        #[serde(rename = "buyer", default)]
        pub buyer: Option<crate::schemas::Buyer>,
        #[doc = "Optional contact information of the buyer. (seller-readonly)"]
        #[serde(rename = "buyerContacts", default)]
        pub buyer_contacts: Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "Private data for buyer. (hidden from seller)."]
        #[serde(rename = "buyerPrivateData", default)]
        pub buyer_private_data: Option<crate::schemas::PrivateData>,
        #[doc = "IDs of DBM advertisers permission to this proposal."]
        #[serde(rename = "dbmAdvertiserIds", default)]
        pub dbm_advertiser_ids: Option<Vec<String>>,
        #[doc = "When an proposal is in an accepted state, indicates whether the buyer has signed off. Once both sides have signed off on a deal, the proposal can be finalized by the seller. (seller-readonly)"]
        #[serde(rename = "hasBuyerSignedOff", default)]
        pub has_buyer_signed_off: Option<bool>,
        #[doc = "When an proposal is in an accepted state, indicates whether the buyer has signed off Once both sides have signed off on a deal, the proposal can be finalized by the seller. (buyer-readonly)"]
        #[serde(rename = "hasSellerSignedOff", default)]
        pub has_seller_signed_off: Option<bool>,
        #[doc = "What exchange will provide this inventory (readonly, except on create)."]
        #[serde(rename = "inventorySource", default)]
        pub inventory_source: Option<String>,
        #[doc = "True if the proposal is being renegotiated (readonly)."]
        #[serde(rename = "isRenegotiating", default)]
        pub is_renegotiating: Option<bool>,
        #[doc = "True, if the buyside inventory setup is complete for this proposal. (readonly, except via OrderSetupCompleted action) Deprecated in favor of deal level setup complete flag."]
        #[serde(rename = "isSetupComplete", default)]
        pub is_setup_complete: Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#proposal\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "List of labels associated with the proposal. (readonly)"]
        #[serde(rename = "labels", default)]
        pub labels: Option<Vec<crate::schemas::MarketplaceLabel>>,
        #[doc = "The role of the last user that either updated the proposal or left a comment. (readonly)"]
        #[serde(rename = "lastUpdaterOrCommentorRole", default)]
        pub last_updater_or_commentor_role: Option<String>,
        #[doc = "The name for the proposal (updatable)"]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Optional negotiation id if this proposal is a preferred deal proposal."]
        #[serde(rename = "negotiationId", default)]
        pub negotiation_id: Option<String>,
        #[doc = "Indicates whether the buyer/seller created the proposal.(readonly)"]
        #[serde(rename = "originatorRole", default)]
        pub originator_role: Option<String>,
        #[doc = "Optional private auction id if this proposal is a private auction proposal."]
        #[serde(rename = "privateAuctionId", default)]
        pub private_auction_id: Option<String>,
        #[doc = "The unique id of the proposal. (readonly)."]
        #[serde(rename = "proposalId", default)]
        pub proposal_id: Option<String>,
        #[doc = "The current state of the proposal. (readonly)"]
        #[serde(rename = "proposalState", default)]
        pub proposal_state: Option<String>,
        #[doc = "The revision number for the proposal (readonly)."]
        #[serde(rename = "revisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub revision_number: Option<i64>,
        #[doc = "The time (ms since epoch) when the proposal was last revised (readonly)."]
        #[serde(rename = "revisionTimeMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub revision_time_ms: Option<i64>,
        #[doc = "Reference to the seller on the proposal. (readonly, except on create)"]
        #[serde(rename = "seller", default)]
        pub seller: Option<crate::schemas::Seller>,
        #[doc = "Optional contact information of the seller (buyer-readonly)."]
        #[serde(rename = "sellerContacts", default)]
        pub seller_contacts: Option<Vec<crate::schemas::ContactInformation>>,
    }
    impl ::field_selector::FieldSelector for Proposal {
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
    pub struct PublisherProfileApiProto {
        #[doc = "Publisher provided info on its audience."]
        #[serde(rename = "audience", default)]
        pub audience: Option<String>,
        #[doc = "A pitch statement for the buyer"]
        #[serde(rename = "buyerPitchStatement", default)]
        pub buyer_pitch_statement: Option<String>,
        #[doc = "Direct contact for the publisher profile."]
        #[serde(rename = "directContact", default)]
        pub direct_contact: Option<String>,
        #[doc = "Exchange where this publisher profile is from. E.g. AdX, Rubicon etc..."]
        #[serde(rename = "exchange", default)]
        pub exchange: Option<String>,
        #[doc = "Link to publisher's Google+ page."]
        #[serde(rename = "googlePlusLink", default)]
        pub google_plus_link: Option<String>,
        #[doc = "True, if this is the parent profile, which represents all domains owned by the publisher."]
        #[serde(rename = "isParent", default)]
        pub is_parent: Option<bool>,
        #[doc = "True, if this profile is published. Deprecated for state."]
        #[serde(rename = "isPublished", default)]
        pub is_published: Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#publisherProfileApiProto\"."]
        #[serde(rename = "kind", default)]
        pub kind: Option<String>,
        #[doc = "The url to the logo for the publisher."]
        #[serde(rename = "logoUrl", default)]
        pub logo_url: Option<String>,
        #[doc = "The url for additional marketing and sales materials."]
        #[serde(rename = "mediaKitLink", default)]
        pub media_kit_link: Option<String>,
        #[serde(rename = "name", default)]
        pub name: Option<String>,
        #[doc = "Publisher provided overview."]
        #[serde(rename = "overview", default)]
        pub overview: Option<String>,
        #[doc = "The pair of (seller.account_id, profile_id) uniquely identifies a publisher profile for a given publisher."]
        #[serde(rename = "profileId", default)]
        pub profile_id: Option<i32>,
        #[doc = "Programmatic contact for the publisher profile."]
        #[serde(rename = "programmaticContact", default)]
        pub programmatic_contact: Option<String>,
        #[doc = "The list of domains represented in this publisher profile. Empty if this is a parent profile."]
        #[serde(rename = "publisherDomains", default)]
        pub publisher_domains: Option<Vec<String>>,
        #[doc = "Unique Id for publisher profile."]
        #[serde(rename = "publisherProfileId", default)]
        pub publisher_profile_id: Option<String>,
        #[doc = "Publisher provided forecasting information."]
        #[serde(rename = "publisherProvidedForecast", default)]
        pub publisher_provided_forecast: Option<crate::schemas::PublisherProvidedForecast>,
        #[doc = "Link to publisher rate card"]
        #[serde(rename = "rateCardInfoLink", default)]
        pub rate_card_info_link: Option<String>,
        #[doc = "Link for a sample content page."]
        #[serde(rename = "samplePageLink", default)]
        pub sample_page_link: Option<String>,
        #[doc = "Seller of the publisher profile."]
        #[serde(rename = "seller", default)]
        pub seller: Option<crate::schemas::Seller>,
        #[doc = "State of the publisher profile."]
        #[serde(rename = "state", default)]
        pub state: Option<String>,
        #[doc = "Publisher provided key metrics and rankings."]
        #[serde(rename = "topHeadlines", default)]
        pub top_headlines: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for PublisherProfileApiProto {
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
    pub struct PublisherProvidedForecast {
        #[doc = "Publisher provided dimensions. E.g. geo, sizes etc..."]
        #[serde(rename = "dimensions", default)]
        pub dimensions: Option<Vec<crate::schemas::Dimension>>,
        #[doc = "Publisher provided weekly impressions."]
        #[serde(rename = "weeklyImpressions", default)]
        #[serde(with = "crate::parsed_string")]
        pub weekly_impressions: Option<i64>,
        #[doc = "Publisher provided weekly uniques."]
        #[serde(rename = "weeklyUniques", default)]
        #[serde(with = "crate::parsed_string")]
        pub weekly_uniques: Option<i64>,
    }
    impl ::field_selector::FieldSelector for PublisherProvidedForecast {
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
    pub struct Seller {
        #[doc = "The unique id for the seller. The seller fills in this field. The seller account id is then available to buyer in the product."]
        #[serde(rename = "accountId", default)]
        pub account_id: Option<String>,
        #[doc = "Optional sub-account id for the seller."]
        #[serde(rename = "subAccountId", default)]
        pub sub_account_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for Seller {
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
    pub struct SharedTargeting {
        #[doc = "The list of values to exclude from targeting. Each value is AND'd together."]
        #[serde(rename = "exclusions", default)]
        pub exclusions: Option<Vec<crate::schemas::TargetingValue>>,
        #[doc = "The list of value to include as part of the targeting. Each value is OR'd together."]
        #[serde(rename = "inclusions", default)]
        pub inclusions: Option<Vec<crate::schemas::TargetingValue>>,
        #[doc = "The key representing the shared targeting criterion."]
        #[serde(rename = "key", default)]
        pub key: Option<String>,
    }
    impl ::field_selector::FieldSelector for SharedTargeting {
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
    pub struct TargetingValue {
        #[doc = "The creative size value to exclude/include."]
        #[serde(rename = "creativeSizeValue", default)]
        pub creative_size_value: Option<crate::schemas::TargetingValueCreativeSize>,
        #[doc = "The daypart targeting to include / exclude. Filled in when the key is GOOG_DAYPART_TARGETING."]
        #[serde(rename = "dayPartTargetingValue", default)]
        pub day_part_targeting_value: Option<crate::schemas::TargetingValueDayPartTargeting>,
        #[serde(rename = "demogAgeCriteriaValue", default)]
        pub demog_age_criteria_value: Option<crate::schemas::TargetingValueDemogAgeCriteria>,
        #[serde(rename = "demogGenderCriteriaValue", default)]
        pub demog_gender_criteria_value: Option<crate::schemas::TargetingValueDemogGenderCriteria>,
        #[doc = "The long value to exclude/include."]
        #[serde(rename = "longValue", default)]
        #[serde(with = "crate::parsed_string")]
        pub long_value: Option<i64>,
        #[doc = "The string value to exclude/include."]
        #[serde(rename = "stringValue", default)]
        pub string_value: Option<String>,
    }
    impl ::field_selector::FieldSelector for TargetingValue {
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
    pub struct TargetingValueCreativeSize {
        #[doc = "The formats allowed by the publisher."]
        #[serde(rename = "allowedFormats", default)]
        pub allowed_formats: Option<Vec<String>>,
        #[doc = "For video size type, the list of companion sizes."]
        #[serde(rename = "companionSizes", default)]
        pub companion_sizes: Option<Vec<crate::schemas::TargetingValueSize>>,
        #[doc = "The Creative size type."]
        #[serde(rename = "creativeSizeType", default)]
        pub creative_size_type: Option<String>,
        #[doc = "The native template for native ad."]
        #[serde(rename = "nativeTemplate", default)]
        pub native_template: Option<String>,
        #[doc = "For regular or video creative size type, specifies the size of the creative."]
        #[serde(rename = "size", default)]
        pub size: Option<crate::schemas::TargetingValueSize>,
        #[doc = "The skippable ad type for video size."]
        #[serde(rename = "skippableAdType", default)]
        pub skippable_ad_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for TargetingValueCreativeSize {
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
    pub struct TargetingValueDayPartTargeting {
        #[serde(rename = "dayParts", default)]
        pub day_parts: Option<Vec<crate::schemas::TargetingValueDayPartTargetingDayPart>>,
        #[serde(rename = "timeZoneType", default)]
        pub time_zone_type: Option<String>,
    }
    impl ::field_selector::FieldSelector for TargetingValueDayPartTargeting {
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
    pub struct TargetingValueDayPartTargetingDayPart {
        #[serde(rename = "dayOfWeek", default)]
        pub day_of_week: Option<String>,
        #[serde(rename = "endHour", default)]
        pub end_hour: Option<i32>,
        #[serde(rename = "endMinute", default)]
        pub end_minute: Option<i32>,
        #[serde(rename = "startHour", default)]
        pub start_hour: Option<i32>,
        #[serde(rename = "startMinute", default)]
        pub start_minute: Option<i32>,
    }
    impl ::field_selector::FieldSelector for TargetingValueDayPartTargetingDayPart {
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
    pub struct TargetingValueDemogAgeCriteria {
        #[serde(rename = "demogAgeCriteriaIds", default)]
        pub demog_age_criteria_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TargetingValueDemogAgeCriteria {
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
    pub struct TargetingValueDemogGenderCriteria {
        #[serde(rename = "demogGenderCriteriaIds", default)]
        pub demog_gender_criteria_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TargetingValueDemogGenderCriteria {
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
    pub struct TargetingValueSize {
        #[doc = "The height of the creative."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The width of the creative."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for TargetingValueSize {
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
    pub struct UpdatePrivateAuctionProposalRequest {
        #[doc = "The externalDealId of the deal to be updated."]
        #[serde(rename = "externalDealId", default)]
        pub external_deal_id: Option<String>,
        #[doc = "Optional note to be added."]
        #[serde(rename = "note", default)]
        pub note: Option<crate::schemas::MarketplaceNote>,
        #[doc = "The current revision number of the proposal to be updated."]
        #[serde(rename = "proposalRevisionNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: Option<i64>,
        #[doc = "The proposed action on the private auction proposal."]
        #[serde(rename = "updateAction", default)]
        pub update_action: Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdatePrivateAuctionProposalRequest {
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
    #[doc = "Actions that can be performed on the billing_info resource"]
    pub fn billing_info(&self) -> crate::billing_info::BillingInfoActions<A> {
        crate::billing_info::BillingInfoActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the budget resource"]
    pub fn budget(&self) -> crate::budget::BudgetActions<A> {
        crate::budget::BudgetActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the creatives resource"]
    pub fn creatives(&self) -> crate::creatives::CreativesActions<A> {
        crate::creatives::CreativesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the marketplacedeals resource"]
    pub fn marketplacedeals(&self) -> crate::marketplacedeals::MarketplacedealsActions<A> {
        crate::marketplacedeals::MarketplacedealsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the marketplacenotes resource"]
    pub fn marketplacenotes(&self) -> crate::marketplacenotes::MarketplacenotesActions<A> {
        crate::marketplacenotes::MarketplacenotesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the marketplaceprivateauction resource"]
    pub fn marketplaceprivateauction(
        &self,
    ) -> crate::marketplaceprivateauction::MarketplaceprivateauctionActions<A> {
        crate::marketplaceprivateauction::MarketplaceprivateauctionActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the performance_report resource"]
    pub fn performance_report(&self) -> crate::performance_report::PerformanceReportActions<A> {
        crate::performance_report::PerformanceReportActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the pretargeting_config resource"]
    pub fn pretargeting_config(&self) -> crate::pretargeting_config::PretargetingConfigActions<A> {
        crate::pretargeting_config::PretargetingConfigActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the products resource"]
    pub fn products(&self) -> crate::products::ProductsActions<A> {
        crate::products::ProductsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the proposals resource"]
    pub fn proposals(&self) -> crate::proposals::ProposalsActions<A> {
        crate::proposals::ProposalsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the pubprofiles resource"]
    pub fn pubprofiles(&self) -> crate::pubprofiles::PubprofilesActions<A> {
        crate::pubprofiles::PubprofilesActions {
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
        #[doc = "Gets one account by ID."]
        pub fn get(&self, id: i32) -> GetRequestBuilder<A> {
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
                id,
            }
        }
        #[doc = "Retrieves the authenticated user's list of accounts."]
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
        #[doc = "Updates an existing account. This method supports patch semantics."]
        pub fn patch(&self, request: crate::schemas::Account, id: i32) -> PatchRequestBuilder<A> {
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
                id,
                confirm_unsafe_account_change: None,
            }
        }
        #[doc = "Updates an existing account."]
        pub fn update(&self, request: crate::schemas::Account, id: i32) -> UpdateRequestBuilder<A> {
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
                id,
                confirm_unsafe_account_change: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        id: i32,
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("accounts/");
            {
                let str_value = self.id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        ) -> Result<crate::schemas::AccountsList, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("accounts");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Account,
        id: i32,
        confirm_unsafe_account_change: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
        #[doc = "Confirmation for erasing bidder and cookie matching urls."]
        pub fn confirm_unsafe_account_change(&mut self, value: bool) -> &mut Self {
            self.confirm_unsafe_account_change = Some(value);
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("accounts/");
            {
                let str_value = self.id.to_string();
                output.push_str(&str_value);
            }
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PATCH, path);
            let req = req.query(&[(
                "confirmUnsafeAccountChange",
                &self.confirm_unsafe_account_change,
            )]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        request: crate::schemas::Account,
        id: i32,
        confirm_unsafe_account_change: Option<bool>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
        #[doc = "Confirmation for erasing bidder and cookie matching urls."]
        pub fn confirm_unsafe_account_change(&mut self, value: bool) -> &mut Self {
            self.confirm_unsafe_account_change = Some(value);
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("accounts/");
            {
                let str_value = self.id.to_string();
                output.push_str(&str_value);
            }
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::PUT, path);
            let req = req.query(&[(
                "confirmUnsafeAccountChange",
                &self.confirm_unsafe_account_change,
            )]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod billing_info {
    pub mod params {}
    pub struct BillingInfoActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> BillingInfoActions<'a, A> {
        #[doc = "Returns the billing information for one account specified by account ID."]
        pub fn get(&self, account_id: i32) -> GetRequestBuilder<A> {
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
                account_id,
            }
        }
        #[doc = "Retrieves a list of billing information for all accounts of the authenticated user."]
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
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i32,
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
        ) -> Result<crate::schemas::BillingInfo, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("billinginfo/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        ) -> Result<crate::schemas::BillingInfoList, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("billinginfo");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod budget {
    pub mod params {}
    pub struct BudgetActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> BudgetActions<'a, A> {
        #[doc = "Returns the budget information for the adgroup specified by the accountId and billingId."]
        pub fn get(&self, account_id: i64, billing_id: i64) -> GetRequestBuilder<A> {
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
                account_id,
                billing_id,
            }
        }
        #[doc = "Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request. This method supports patch semantics."]
        pub fn patch(
            &self,
            request: crate::schemas::Budget,
            account_id: i64,
            billing_id: i64,
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
                account_id,
                billing_id,
            }
        }
        #[doc = "Updates the budget amount for the budget of the adgroup specified by the accountId and billingId, with the budget amount in the request."]
        pub fn update(
            &self,
            request: crate::schemas::Budget,
            account_id: i64,
            billing_id: i64,
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
                account_id,
                billing_id,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i64,
        billing_id: i64,
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
        pub fn execute_debug(self) -> Result<crate::schemas::Budget, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("billinginfo/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.billing_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Budget,
        account_id: i64,
        billing_id: i64,
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
        pub fn execute_debug(self) -> Result<crate::schemas::Budget, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("billinginfo/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.billing_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        request: crate::schemas::Budget,
        account_id: i64,
        billing_id: i64,
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
        pub fn execute_debug(self) -> Result<crate::schemas::Budget, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("billinginfo/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.billing_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod creatives {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum ListDealsStatusFilter {
            #[doc = "Creatives which have been approved for serving on deals."]
            Approved,
            #[doc = "Creatives which have been conditionally approved for serving on deals."]
            ConditionallyApproved,
            #[doc = "Creatives which have been disapproved for serving on deals."]
            Disapproved,
            #[doc = "Creatives whose deals status is not yet checked."]
            NotChecked,
        }
        impl ListDealsStatusFilter {
            pub fn as_str(self) -> &'static str {
                match self {
                    ListDealsStatusFilter::Approved => "approved",
                    ListDealsStatusFilter::ConditionallyApproved => "conditionally_approved",
                    ListDealsStatusFilter::Disapproved => "disapproved",
                    ListDealsStatusFilter::NotChecked => "not_checked",
                }
            }
        }
        impl ::std::fmt::Display for ListDealsStatusFilter {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListDealsStatusFilter {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListDealsStatusFilter {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "approved" => ListDealsStatusFilter::Approved,
                    "conditionally_approved" => ListDealsStatusFilter::ConditionallyApproved,
                    "disapproved" => ListDealsStatusFilter::Disapproved,
                    "not_checked" => ListDealsStatusFilter::NotChecked,
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
        pub enum ListOpenAuctionStatusFilter {
            #[doc = "Creatives which have been approved for serving on the open auction."]
            Approved,
            #[doc = "Creatives which have been conditionally approved for serving on the open auction."]
            ConditionallyApproved,
            #[doc = "Creatives which have been disapproved for serving on the open auction."]
            Disapproved,
            #[doc = "Creatives whose open auction status is not yet checked."]
            NotChecked,
        }
        impl ListOpenAuctionStatusFilter {
            pub fn as_str(self) -> &'static str {
                match self {
                    ListOpenAuctionStatusFilter::Approved => "approved",
                    ListOpenAuctionStatusFilter::ConditionallyApproved => "conditionally_approved",
                    ListOpenAuctionStatusFilter::Disapproved => "disapproved",
                    ListOpenAuctionStatusFilter::NotChecked => "not_checked",
                }
            }
        }
        impl ::std::fmt::Display for ListOpenAuctionStatusFilter {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for ListOpenAuctionStatusFilter {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for ListOpenAuctionStatusFilter {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "approved" => ListOpenAuctionStatusFilter::Approved,
                    "conditionally_approved" => ListOpenAuctionStatusFilter::ConditionallyApproved,
                    "disapproved" => ListOpenAuctionStatusFilter::Disapproved,
                    "not_checked" => ListOpenAuctionStatusFilter::NotChecked,
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
    pub struct CreativesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> CreativesActions<'a, A> {
        #[doc = "Add a deal id association for the creative."]
        pub fn add_deal(
            &self,
            account_id: i32,
            buyer_creative_id: impl Into<String>,
            deal_id: i64,
        ) -> AddDealRequestBuilder<A> {
            AddDealRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                account_id,
                buyer_creative_id: buyer_creative_id.into(),
                deal_id,
            }
        }
        #[doc = "Gets the status for a single creative. A creative will be available 30-40 minutes after submission."]
        pub fn get(
            &self,
            account_id: i32,
            buyer_creative_id: impl Into<String>,
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
                account_id,
                buyer_creative_id: buyer_creative_id.into(),
            }
        }
        #[doc = "Submit a new creative."]
        pub fn insert(&self, request: crate::schemas::Creative) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
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
            }
        }
        #[doc = "Retrieves a list of the authenticated user's active creatives. A creative will be available 30-40 minutes after submission."]
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
                account_id: None,
                buyer_creative_id: None,
                deals_status_filter: None,
                max_results: None,
                open_auction_status_filter: None,
                page_token: None,
            }
        }
        #[doc = "Lists the external deal ids associated with the creative."]
        pub fn list_deals(
            &self,
            account_id: i32,
            buyer_creative_id: impl Into<String>,
        ) -> ListDealsRequestBuilder<A> {
            ListDealsRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                account_id,
                buyer_creative_id: buyer_creative_id.into(),
            }
        }
        #[doc = "Remove a deal id associated with the creative."]
        pub fn remove_deal(
            &self,
            account_id: i32,
            buyer_creative_id: impl Into<String>,
            deal_id: i64,
        ) -> RemoveDealRequestBuilder<A> {
            RemoveDealRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                account_id,
                buyer_creative_id: buyer_creative_id.into(),
                deal_id,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct AddDealRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i32,
        buyer_creative_id: String,
        deal_id: i64,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> AddDealRequestBuilder<'a, A> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("creatives/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            output.push_str(&self.buyer_creative_id);
            output.push_str("/addDeal/");
            {
                let str_value = self.deal_id.to_string();
                output.push_str(&str_value);
            }
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        account_id: i32,
        buyer_creative_id: String,
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
        ) -> Result<crate::schemas::Creative, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("creatives/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            output.push_str(&self.buyer_creative_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Creative,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::Creative, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("creatives");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        account_id: Option<i32>,
        buyer_creative_id: Option<String>,
        deals_status_filter: Option<crate::creatives::params::ListDealsStatusFilter>,
        max_results: Option<u32>,
        open_auction_status_filter: Option<crate::creatives::params::ListOpenAuctionStatusFilter>,
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
        #[doc = "When specified, only creatives for the given account ids are returned."]
        pub fn account_id(&mut self, value: i32) -> &mut Self {
            self.account_id = Some(value);
            self
        }
        #[doc = "When specified, only creatives for the given buyer creative ids are returned."]
        pub fn buyer_creative_id(&mut self, value: impl Into<String>) -> &mut Self {
            self.buyer_creative_id = Some(value.into());
            self
        }
        #[doc = "When specified, only creatives having the given deals status are returned."]
        pub fn deals_status_filter(
            &mut self,
            value: crate::creatives::params::ListDealsStatusFilter,
        ) -> &mut Self {
            self.deals_status_filter = Some(value);
            self
        }
        #[doc = "Maximum number of entries returned on one result page. If not set, the default is 100. Optional."]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "When specified, only creatives having the given open auction status are returned."]
        pub fn open_auction_status_filter(
            &mut self,
            value: crate::creatives::params::ListOpenAuctionStatusFilter,
        ) -> &mut Self {
            self.open_auction_status_filter = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through ad clients. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response. Optional."]
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
        ) -> Result<crate::schemas::CreativesList, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("creatives");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("accountId", &self.account_id)]);
            let req = req.query(&[("buyerCreativeId", &self.buyer_creative_id)]);
            let req = req.query(&[("dealsStatusFilter", &self.deals_status_filter)]);
            let req = req.query(&[("maxResults", &self.max_results)]);
            let req = req.query(&[("openAuctionStatusFilter", &self.open_auction_status_filter)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
    pub struct ListDealsRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i32,
        buyer_creative_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListDealsRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::CreativeDealIds, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("creatives/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            output.push_str(&self.buyer_creative_id);
            output.push_str("/listDeals");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct RemoveDealRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i32,
        buyer_creative_id: String,
        deal_id: i64,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> RemoveDealRequestBuilder<'a, A> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("creatives/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            output.push_str(&self.buyer_creative_id);
            output.push_str("/removeDeal/");
            {
                let str_value = self.deal_id.to_string();
                output.push_str(&str_value);
            }
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod marketplacedeals {
    pub mod params {}
    pub struct MarketplacedealsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> MarketplacedealsActions<'a, A> {
        #[doc = "Delete the specified deals from the proposal"]
        pub fn delete(
            &self,
            request: crate::schemas::DeleteOrderDealsRequest,
            proposal_id: impl Into<String>,
        ) -> DeleteRequestBuilder<A> {
            DeleteRequestBuilder {
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
                proposal_id: proposal_id.into(),
            }
        }
        #[doc = "Add new deals for the specified proposal"]
        pub fn insert(
            &self,
            request: crate::schemas::AddOrderDealsRequest,
            proposal_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
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
                proposal_id: proposal_id.into(),
            }
        }
        #[doc = "List all the deals for a given proposal"]
        pub fn list(&self, proposal_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                proposal_id: proposal_id.into(),
                pql_query: None,
            }
        }
        #[doc = "Replaces all the deals in the proposal with the passed in deals"]
        pub fn update(
            &self,
            request: crate::schemas::EditAllOrderDealsRequest,
            proposal_id: impl Into<String>,
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
                proposal_id: proposal_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::DeleteOrderDealsRequest,
        proposal_id: String,
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
        ) -> Result<crate::schemas::DeleteOrderDealsResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/deals/delete");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AddOrderDealsRequest,
        proposal_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AddOrderDealsResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/deals/insert");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        proposal_id: String,
        pql_query: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Query string to retrieve specific deals."]
        pub fn pql_query(&mut self, value: impl Into<String>) -> &mut Self {
            self.pql_query = Some(value.into());
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
        ) -> Result<crate::schemas::GetOrderDealsResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/deals");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pqlQuery", &self.pql_query)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        request: crate::schemas::EditAllOrderDealsRequest,
        proposal_id: String,
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
        ) -> Result<crate::schemas::EditAllOrderDealsResponse, Box<dyn ::std::error::Error>>
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/deals/update");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod marketplacenotes {
    pub mod params {}
    pub struct MarketplacenotesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> MarketplacenotesActions<'a, A> {
        #[doc = "Add notes to the proposal"]
        pub fn insert(
            &self,
            request: crate::schemas::AddOrderNotesRequest,
            proposal_id: impl Into<String>,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
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
                proposal_id: proposal_id.into(),
            }
        }
        #[doc = "Get all the notes associated with a proposal"]
        pub fn list(&self, proposal_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                proposal_id: proposal_id.into(),
                pql_query: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::AddOrderNotesRequest,
        proposal_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::AddOrderNotesResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/notes/insert");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        proposal_id: String,
        pql_query: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
        #[doc = "Query string to retrieve specific notes. To search the text contents of notes, please use syntax like \"WHERE note.note = \"foo\" or \"WHERE note.note LIKE \"%bar%\""]
        pub fn pql_query(&mut self, value: impl Into<String>) -> &mut Self {
            self.pql_query = Some(value.into());
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
        ) -> Result<crate::schemas::GetOrderNotesResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/notes");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pqlQuery", &self.pql_query)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod marketplaceprivateauction {
    pub mod params {}
    pub struct MarketplaceprivateauctionActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> MarketplaceprivateauctionActions<'a, A> {
        #[doc = "Update a given private auction proposal"]
        pub fn updateproposal(
            &self,
            request: crate::schemas::UpdatePrivateAuctionProposalRequest,
            private_auction_id: impl Into<String>,
        ) -> UpdateproposalRequestBuilder<A> {
            UpdateproposalRequestBuilder {
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
                private_auction_id: private_auction_id.into(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct UpdateproposalRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::UpdatePrivateAuctionProposalRequest,
        private_auction_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> UpdateproposalRequestBuilder<'a, A> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("privateauction/");
            output.push_str(&self.private_auction_id);
            output.push_str("/updateproposal");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod performance_report {
    pub mod params {}
    pub struct PerformanceReportActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PerformanceReportActions<'a, A> {
        #[doc = "Retrieves the authenticated user's list of performance metrics."]
        pub fn list(
            &self,
            account_id: i64,
            end_date_time: impl Into<String>,
            start_date_time: impl Into<String>,
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
                account_id,
                end_date_time: end_date_time.into(),
                start_date_time: start_date_time.into(),
                max_results: None,
                page_token: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i64,
        end_date_time: String,
        start_date_time: String,
        max_results: Option<u32>,
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
        #[doc = "Maximum number of entries returned on one result page. If not set, the default is 100. Optional."]
        pub fn max_results(&mut self, value: u32) -> &mut Self {
            self.max_results = Some(value);
            self
        }
        #[doc = "A continuation token, used to page through performance reports. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response. Optional."]
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
        ) -> Result<crate::schemas::PerformanceReportList, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("performancereport");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("accountId", &self.account_id)]);
            let req = req.query(&[("endDateTime", &self.end_date_time)]);
            let req = req.query(&[("startDateTime", &self.start_date_time)]);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod pretargeting_config {
    pub mod params {}
    pub struct PretargetingConfigActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PretargetingConfigActions<'a, A> {
        #[doc = "Deletes an existing pretargeting config."]
        pub fn delete(&self, account_id: i64, config_id: i64) -> DeleteRequestBuilder<A> {
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
                account_id,
                config_id,
            }
        }
        #[doc = "Gets a specific pretargeting configuration"]
        pub fn get(&self, account_id: i64, config_id: i64) -> GetRequestBuilder<A> {
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
                account_id,
                config_id,
            }
        }
        #[doc = "Inserts a new pretargeting configuration."]
        pub fn insert(
            &self,
            request: crate::schemas::PretargetingConfig,
            account_id: i64,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
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
                account_id,
            }
        }
        #[doc = "Retrieves a list of the authenticated user's pretargeting configurations."]
        pub fn list(&self, account_id: i64) -> ListRequestBuilder<A> {
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
                account_id,
            }
        }
        #[doc = "Updates an existing pretargeting config. This method supports patch semantics."]
        pub fn patch(
            &self,
            request: crate::schemas::PretargetingConfig,
            account_id: i64,
            config_id: i64,
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
                account_id,
                config_id,
            }
        }
        #[doc = "Updates an existing pretargeting config."]
        pub fn update(
            &self,
            request: crate::schemas::PretargetingConfig,
            account_id: i64,
            config_id: i64,
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
                account_id,
                config_id,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct DeleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i64,
        config_id: i64,
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("pretargetingconfigs/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.config_id.to_string();
                output.push_str(&str_value);
            }
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        account_id: i64,
        config_id: i64,
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
        ) -> Result<crate::schemas::PretargetingConfig, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("pretargetingconfigs/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.config_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::PretargetingConfig,
        account_id: i64,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::PretargetingConfig, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("pretargetingconfigs/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        account_id: i64,
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
        ) -> Result<crate::schemas::PretargetingConfigList, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("pretargetingconfigs/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::PretargetingConfig,
        account_id: i64,
        config_id: i64,
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
        ) -> Result<crate::schemas::PretargetingConfig, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("pretargetingconfigs/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.config_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        request: crate::schemas::PretargetingConfig,
        account_id: i64,
        config_id: i64,
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
        ) -> Result<crate::schemas::PretargetingConfig, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("pretargetingconfigs/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.config_id.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod products {
    pub mod params {}
    pub struct ProductsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProductsActions<'a, A> {
        #[doc = "Gets the requested product by id."]
        pub fn get(&self, product_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                product_id: product_id.into(),
            }
        }
        #[doc = "Gets the requested product."]
        pub fn search(&self) -> SearchRequestBuilder<A> {
            SearchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                pql_query: None,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        product_id: String,
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
        ) -> Result<crate::schemas::Product, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("products/");
            output.push_str(&self.product_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SearchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        pql_query: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SearchRequestBuilder<'a, A> {
        #[doc = "The pql query used to query for products."]
        pub fn pql_query(&mut self, value: impl Into<String>) -> &mut Self {
            self.pql_query = Some(value.into());
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
        ) -> Result<crate::schemas::GetOffersResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("products/search");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pqlQuery", &self.pql_query)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod proposals {
    pub mod params {
        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
        pub enum PatchUpdateAction {
            Accept,
            Cancel,
            Propose,
            ProposeAndAccept,
            UnknownAction,
            UpdateNonTerms,
        }
        impl PatchUpdateAction {
            pub fn as_str(self) -> &'static str {
                match self {
                    PatchUpdateAction::Accept => "accept",
                    PatchUpdateAction::Cancel => "cancel",
                    PatchUpdateAction::Propose => "propose",
                    PatchUpdateAction::ProposeAndAccept => "proposeAndAccept",
                    PatchUpdateAction::UnknownAction => "unknownAction",
                    PatchUpdateAction::UpdateNonTerms => "updateNonTerms",
                }
            }
        }
        impl ::std::fmt::Display for PatchUpdateAction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for PatchUpdateAction {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for PatchUpdateAction {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "accept" => PatchUpdateAction::Accept,
                    "cancel" => PatchUpdateAction::Cancel,
                    "propose" => PatchUpdateAction::Propose,
                    "proposeAndAccept" => PatchUpdateAction::ProposeAndAccept,
                    "unknownAction" => PatchUpdateAction::UnknownAction,
                    "updateNonTerms" => PatchUpdateAction::UpdateNonTerms,
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
        pub enum UpdateUpdateAction {
            Accept,
            Cancel,
            Propose,
            ProposeAndAccept,
            UnknownAction,
            UpdateNonTerms,
        }
        impl UpdateUpdateAction {
            pub fn as_str(self) -> &'static str {
                match self {
                    UpdateUpdateAction::Accept => "accept",
                    UpdateUpdateAction::Cancel => "cancel",
                    UpdateUpdateAction::Propose => "propose",
                    UpdateUpdateAction::ProposeAndAccept => "proposeAndAccept",
                    UpdateUpdateAction::UnknownAction => "unknownAction",
                    UpdateUpdateAction::UpdateNonTerms => "updateNonTerms",
                }
            }
        }
        impl ::std::fmt::Display for UpdateUpdateAction {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl ::serde::Serialize for UpdateUpdateAction {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> ::serde::Deserialize<'de> for UpdateUpdateAction {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let value: &'de str = <&str>::deserialize(deserializer)?;
                Ok(match value {
                    "accept" => UpdateUpdateAction::Accept,
                    "cancel" => UpdateUpdateAction::Cancel,
                    "propose" => UpdateUpdateAction::Propose,
                    "proposeAndAccept" => UpdateUpdateAction::ProposeAndAccept,
                    "unknownAction" => UpdateUpdateAction::UnknownAction,
                    "updateNonTerms" => UpdateUpdateAction::UpdateNonTerms,
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
    pub struct ProposalsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> ProposalsActions<'a, A> {
        #[doc = "Get a proposal given its id"]
        pub fn get(&self, proposal_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                proposal_id: proposal_id.into(),
            }
        }
        #[doc = "Create the given list of proposals"]
        pub fn insert(
            &self,
            request: crate::schemas::CreateOrdersRequest,
        ) -> InsertRequestBuilder<A> {
            InsertRequestBuilder {
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
            }
        }
        #[doc = "Update the given proposal. This method supports patch semantics."]
        pub fn patch(
            &self,
            request: crate::schemas::Proposal,
            proposal_id: impl Into<String>,
            revision_number: i64,
            update_action: crate::proposals::params::PatchUpdateAction,
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
                proposal_id: proposal_id.into(),
                revision_number,
                update_action,
            }
        }
        #[doc = "Search for proposals using pql query"]
        pub fn search(&self) -> SearchRequestBuilder<A> {
            SearchRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                pql_query: None,
            }
        }
        #[doc = "Update the given proposal to indicate that setup has been completed."]
        pub fn setupcomplete(
            &self,
            proposal_id: impl Into<String>,
        ) -> SetupcompleteRequestBuilder<A> {
            SetupcompleteRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
                alt: None,
                fields: None,
                key: None,
                oauth_token: None,
                pretty_print: None,
                quota_user: None,
                user_ip: None,
                proposal_id: proposal_id.into(),
            }
        }
        #[doc = "Update the given proposal"]
        pub fn update(
            &self,
            request: crate::schemas::Proposal,
            proposal_id: impl Into<String>,
            revision_number: i64,
            update_action: crate::proposals::params::UpdateUpdateAction,
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
                proposal_id: proposal_id.into(),
                revision_number,
                update_action,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        proposal_id: String,
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
        ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct InsertRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::CreateOrdersRequest,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
        ) -> Result<crate::schemas::CreateOrdersResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/insert");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct PatchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Proposal,
        proposal_id: String,
        revision_number: i64,
        update_action: crate::proposals::params::PatchUpdateAction,
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
        ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/");
            {
                let str_value = self.revision_number.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.update_action.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SearchRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        pql_query: Option<String>,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SearchRequestBuilder<'a, A> {
        #[doc = "Query string to retrieve specific proposals."]
        pub fn pql_query(&mut self, value: impl Into<String>) -> &mut Self {
            self.pql_query = Some(value.into());
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
        ) -> Result<crate::schemas::GetOrdersResponse, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/search");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("pqlQuery", &self.pql_query)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("userIp", &self.user_ip)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct SetupcompleteRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        proposal_id: String,
        alt: Option<crate::params::Alt>,
        fields: Option<String>,
        key: Option<String>,
        oauth_token: Option<String>,
        pretty_print: Option<bool>,
        quota_user: Option<String>,
        user_ip: Option<String>,
    }
    impl<'a, A: yup_oauth2::GetToken> SetupcompleteRequestBuilder<'a, A> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/setupcomplete");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
        request: crate::schemas::Proposal,
        proposal_id: String,
        revision_number: i64,
        update_action: crate::proposals::params::UpdateUpdateAction,
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
        ) -> Result<crate::schemas::Proposal, Box<dyn ::std::error::Error>> {
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("proposals/");
            output.push_str(&self.proposal_id);
            output.push_str("/");
            {
                let str_value = self.revision_number.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/");
            {
                let str_value = self.update_action.to_string();
                output.push_str(&str_value);
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
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
}
pub mod pubprofiles {
    pub mod params {}
    pub struct PubprofilesActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PubprofilesActions<'a, A> {
        #[doc = "Gets the requested publisher profile(s) by publisher accountId."]
        pub fn list(&self, account_id: i32) -> ListRequestBuilder<A> {
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
                account_id,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct ListRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        account_id: i32,
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
        ) -> Result<
            crate::schemas::GetPublisherProfilesByAccountIdResponse,
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
            let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
            output.push_str("publisher/");
            {
                let str_value = self.account_id.to_string();
                output.push_str(&str_value);
            }
            output.push_str("/profiles");
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
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/adexchange.buyer"])
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
