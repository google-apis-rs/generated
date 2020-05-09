#![doc = "# Resources and Methods\n    * [accounts](resources/accounts/struct.AccountsActions.html)\n      * [*get*](resources/accounts/struct.GetRequestBuilder.html), [*list*](resources/accounts/struct.ListRequestBuilder.html), [*patch*](resources/accounts/struct.PatchRequestBuilder.html), [*update*](resources/accounts/struct.UpdateRequestBuilder.html)\n    * [billing_info](resources/billing_info/struct.BillingInfoActions.html)\n      * [*get*](resources/billing_info/struct.GetRequestBuilder.html), [*list*](resources/billing_info/struct.ListRequestBuilder.html)\n    * [budget](resources/budget/struct.BudgetActions.html)\n      * [*get*](resources/budget/struct.GetRequestBuilder.html), [*patch*](resources/budget/struct.PatchRequestBuilder.html), [*update*](resources/budget/struct.UpdateRequestBuilder.html)\n    * [creatives](resources/creatives/struct.CreativesActions.html)\n      * [*addDeal*](resources/creatives/struct.AddDealRequestBuilder.html), [*get*](resources/creatives/struct.GetRequestBuilder.html), [*insert*](resources/creatives/struct.InsertRequestBuilder.html), [*list*](resources/creatives/struct.ListRequestBuilder.html), [*listDeals*](resources/creatives/struct.ListDealsRequestBuilder.html), [*removeDeal*](resources/creatives/struct.RemoveDealRequestBuilder.html)\n    * [marketplacedeals](resources/marketplacedeals/struct.MarketplacedealsActions.html)\n      * [*delete*](resources/marketplacedeals/struct.DeleteRequestBuilder.html), [*insert*](resources/marketplacedeals/struct.InsertRequestBuilder.html), [*list*](resources/marketplacedeals/struct.ListRequestBuilder.html), [*update*](resources/marketplacedeals/struct.UpdateRequestBuilder.html)\n    * [marketplacenotes](resources/marketplacenotes/struct.MarketplacenotesActions.html)\n      * [*insert*](resources/marketplacenotes/struct.InsertRequestBuilder.html), [*list*](resources/marketplacenotes/struct.ListRequestBuilder.html)\n    * [marketplaceprivateauction](resources/marketplaceprivateauction/struct.MarketplaceprivateauctionActions.html)\n      * [*updateproposal*](resources/marketplaceprivateauction/struct.UpdateproposalRequestBuilder.html)\n    * [performance_report](resources/performance_report/struct.PerformanceReportActions.html)\n      * [*list*](resources/performance_report/struct.ListRequestBuilder.html)\n    * [pretargeting_config](resources/pretargeting_config/struct.PretargetingConfigActions.html)\n      * [*delete*](resources/pretargeting_config/struct.DeleteRequestBuilder.html), [*get*](resources/pretargeting_config/struct.GetRequestBuilder.html), [*insert*](resources/pretargeting_config/struct.InsertRequestBuilder.html), [*list*](resources/pretargeting_config/struct.ListRequestBuilder.html), [*patch*](resources/pretargeting_config/struct.PatchRequestBuilder.html), [*update*](resources/pretargeting_config/struct.UpdateRequestBuilder.html)\n    * [products](resources/products/struct.ProductsActions.html)\n      * [*get*](resources/products/struct.GetRequestBuilder.html), [*search*](resources/products/struct.SearchRequestBuilder.html)\n    * [proposals](resources/proposals/struct.ProposalsActions.html)\n      * [*get*](resources/proposals/struct.GetRequestBuilder.html), [*insert*](resources/proposals/struct.InsertRequestBuilder.html), [*patch*](resources/proposals/struct.PatchRequestBuilder.html), [*search*](resources/proposals/struct.SearchRequestBuilder.html), [*setupcomplete*](resources/proposals/struct.SetupcompleteRequestBuilder.html), [*update*](resources/proposals/struct.UpdateRequestBuilder.html)\n    * [pubprofiles](resources/pubprofiles/struct.PubprofilesActions.html)\n      * [*list*](resources/pubprofiles/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Manage your Ad Exchange buyer account configuration\n\n`https://www.googleapis.com/auth/adexchange.buyer`"]
    pub const ADEXCHANGE_BUYER: &str = "https://www.googleapis.com/auth/adexchange.buyer";
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
    pub struct Account {
        #[doc = "When this is false, bid requests that include a deal ID for a private auction or preferred deal are always sent to your bidder. When true, all active pretargeting configs will be applied to private auctions and preferred deals. Programmatic Guaranteed deals (when enabled) are always sent to your bidder."]
        #[serde(
            rename = "applyPretargetingToNonGuaranteedDeals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apply_pretargeting_to_non_guaranteed_deals: ::std::option::Option<bool>,
        #[doc = "Your bidder locations that have distinct URLs."]
        #[serde(
            rename = "bidderLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bidder_location: ::std::option::Option<Vec<crate::schemas::AccountBidderLocationItems>>,
        #[doc = "The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this."]
        #[serde(
            rename = "cookieMatchingNid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cookie_matching_nid: ::std::option::Option<String>,
        #[doc = "The base URL used in cookie match requests."]
        #[serde(
            rename = "cookieMatchingUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cookie_matching_url: ::std::option::Option<String>,
        #[doc = "Account id."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this."]
        #[serde(
            rename = "maximumActiveCreatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximum_active_creatives: ::std::option::Option<i32>,
        #[doc = "The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this."]
        #[serde(
            rename = "maximumTotalQps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximum_total_qps: ::std::option::Option<i32>,
        #[doc = "The number of creatives that this account inserted or bid with in the last 30 days."]
        #[serde(
            rename = "numberActiveCreatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number_active_creatives: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Account {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Account {
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
    pub struct AccountBidderLocationItems {
        #[doc = "The protocol that the bidder endpoint is using. OpenRTB protocols with prefix PROTOCOL_OPENRTB_PROTOBUF use proto buffer, otherwise use JSON.  Allowed values:\n\n* PROTOCOL_ADX \n* PROTOCOL_OPENRTB_2_2 \n* PROTOCOL_OPENRTB_2_3 \n* PROTOCOL_OPENRTB_2_4 \n* PROTOCOL_OPENRTB_2_5 \n* PROTOCOL_OPENRTB_PROTOBUF_2_3 \n* PROTOCOL_OPENRTB_PROTOBUF_2_4 \n* PROTOCOL_OPENRTB_PROTOBUF_2_5"]
        #[serde(
            rename = "bidProtocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bid_protocol: ::std::option::Option<String>,
        #[doc = "The maximum queries per second the Ad Exchange will send."]
        #[serde(
            rename = "maximumQps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximum_qps: ::std::option::Option<i32>,
        #[doc = "The geographical region the Ad Exchange should send requests from. Only used by some quota systems, but always setting the value is recommended. Allowed values:\n\n* ASIA \n* EUROPE \n* US_EAST \n* US_WEST"]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "The URL to which the Ad Exchange will send bid requests."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AccountBidderLocationItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountBidderLocationItems {
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
    pub struct AccountsList {
        #[doc = "A list of accounts."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Account>>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AccountsList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountsList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddOrderDealsRequest {
        #[doc = "The list of deals to add"]
        #[serde(
            rename = "deals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals: ::std::option::Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The last known proposal revision number."]
        #[serde(
            rename = "proposalRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: ::std::option::Option<i64>,
        #[doc = "Indicates an optional action to take on the proposal"]
        #[serde(
            rename = "updateAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_action: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AddOrderDealsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddOrderDealsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AddOrderDealsResponse {
        #[doc = "List of deals added (in the same proposal as passed in the request)"]
        #[serde(
            rename = "deals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals: ::std::option::Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The updated revision number for the proposal."]
        #[serde(
            rename = "proposalRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for AddOrderDealsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddOrderDealsResponse {
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
    pub struct AddOrderNotesRequest {
        #[doc = "The list of notes to add."]
        #[serde(
            rename = "notes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes: ::std::option::Option<Vec<crate::schemas::MarketplaceNote>>,
    }
    impl ::google_field_selector::FieldSelector for AddOrderNotesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddOrderNotesRequest {
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
    pub struct AddOrderNotesResponse {
        #[serde(
            rename = "notes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes: ::std::option::Option<Vec<crate::schemas::MarketplaceNote>>,
    }
    impl ::google_field_selector::FieldSelector for AddOrderNotesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AddOrderNotesResponse {
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
    pub struct BillingInfo {
        #[doc = "Account id."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_id: ::std::option::Option<i32>,
        #[doc = "Account name."]
        #[serde(
            rename = "accountName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_name: ::std::option::Option<String>,
        #[doc = "A list of adgroup IDs associated with this particular account. These IDs may show up as part of a realtime bidding BidRequest, which indicates a bid request for this account."]
        #[serde(
            rename = "billingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing_id: ::std::option::Option<Vec<String>>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BillingInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BillingInfo {
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
    pub struct BillingInfoList {
        #[doc = "A list of billing info relevant for your account."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::BillingInfo>>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BillingInfoList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BillingInfoList {
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
    pub struct Budget {
        #[doc = "The id of the account. This is required for get and update requests."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub account_id: ::std::option::Option<i64>,
        #[doc = "The billing id to determine which adgroup to provide budget information for. This is required for get and update requests."]
        #[serde(
            rename = "billingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub billing_id: ::std::option::Option<i64>,
        #[doc = "The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests."]
        #[serde(
            rename = "budgetAmount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub budget_amount: ::std::option::Option<i64>,
        #[doc = "The currency code for the buyer. This cannot be altered here."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "The unique id that describes this item."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The kind of the resource, i.e. \"adexchangebuyer#budget\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Budget {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Budget {
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
        #[doc = "Adx account id of the buyer."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_id: ::std::option::Option<String>,
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
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContactInformation {
        #[doc = "Email address of the contact."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The name of the contact."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContactInformation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContactInformation {
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
    pub struct CreateOrdersRequest {
        #[doc = "The list of proposals to create."]
        #[serde(
            rename = "proposals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposals: ::std::option::Option<Vec<crate::schemas::Proposal>>,
        #[doc = "Web property id of the seller creating these orders"]
        #[serde(
            rename = "webPropertyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_property_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateOrdersRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateOrdersRequest {
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
    pub struct CreateOrdersResponse {
        #[doc = "The list of proposals successfully created."]
        #[serde(
            rename = "proposals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposals: ::std::option::Option<Vec<crate::schemas::Proposal>>,
    }
    impl ::google_field_selector::FieldSelector for CreateOrdersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateOrdersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Creative {
        #[doc = "Account id."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_id: ::std::option::Option<i32>,
        #[doc = "The link to the Ad Preferences page. This is only supported for native ads."]
        #[serde(
            rename = "adChoicesDestinationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_choices_destination_url: ::std::option::Option<String>,
        #[serde(
            rename = "adTechnologyProviders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_technology_providers:
            ::std::option::Option<crate::schemas::CreativeAdTechnologyProviders>,
        #[doc = "Detected advertiser id, if any. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "advertiserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advertiser_id: ::std::option::Option<Vec<i64>>,
        #[doc = "The name of the company being advertised in the creative. The value provided must exist in the advertisers.txt file."]
        #[serde(
            rename = "advertiserName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advertiser_name: ::std::option::Option<String>,
        #[doc = "The agency id for this creative."]
        #[serde(
            rename = "agencyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: ::std::option::Option<i64>,
        #[doc = "The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp)."]
        #[serde(
            rename = "apiUploadTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_upload_timestamp: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "List of buyer selectable attributes for the ads that may be shown from this snippet. Each attribute is represented by an integer as defined in  buyer-declarable-creative-attributes.txt."]
        #[serde(
            rename = "attribute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribute: ::std::option::Option<Vec<i32>>,
        #[doc = "A buyer-specific id identifying the creative in this ad."]
        #[serde(
            rename = "buyerCreativeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer_creative_id: ::std::option::Option<String>,
        #[doc = "The set of destination urls for the snippet."]
        #[serde(
            rename = "clickThroughUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub click_through_url: ::std::option::Option<Vec<String>>,
        #[doc = "Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "corrections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrections: ::std::option::Option<Vec<crate::schemas::CreativeCorrectionsItems>>,
        #[doc = "Creative status identity type that the creative item applies to. Ad Exchange real-time bidding is migrating to the sizeless creative verification. Originally, Ad Exchange assigned creative verification status to a unique combination of a buyer creative ID and creative dimensions. Post-migration, a single verification status will be assigned at the buyer creative ID level. This field allows to distinguish whether a given creative status applies to a unique combination of a buyer creative ID and creative dimensions, or to a buyer creative ID as a whole."]
        #[serde(
            rename = "creativeStatusIdentityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_status_identity_type: ::std::option::Option<String>,
        #[doc = "Top-level deals status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=DIRECT_DEALS (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from servingRestrictions directly."]
        #[serde(
            rename = "dealsStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals_status: ::std::option::Option<String>,
        #[doc = "Detected domains for this creative. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "detectedDomains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_domains: ::std::option::Option<Vec<String>>,
        #[doc = "The filtering reasons for the creative. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "filteringReasons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filtering_reasons: ::std::option::Option<crate::schemas::CreativeFilteringReasons>,
        #[doc = "Ad height."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The HTML snippet that displays the ad when inserted in the web page. If set, videoURL, videoVastXML, and nativeAd should not be set."]
        #[serde(
            rename = "HTMLSnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_snippet: ::std::option::Option<String>,
        #[doc = "The set of urls to be called to record an impression."]
        #[serde(
            rename = "impressionTrackingUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impression_tracking_url: ::std::option::Option<Vec<String>>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Detected languages for this creative. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "languages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub languages: ::std::option::Option<Vec<String>>,
        #[doc = "If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.)"]
        #[serde(
            rename = "nativeAd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub native_ad: ::std::option::Option<crate::schemas::CreativeNativeAd>,
        #[doc = "Top-level open auction status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=OPEN_AUCTION (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from ServingRestrictions directly."]
        #[serde(
            rename = "openAuctionStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_auction_status: ::std::option::Option<String>,
        #[doc = "Detected product categories, if any. Each category is represented by an integer as defined in  ad-product-categories.txt. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "productCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_categories: ::std::option::Option<Vec<i32>>,
        #[doc = "All restricted categories for the ads that may be shown from this snippet. Each category is represented by an integer as defined in the  ad-restricted-categories.txt."]
        #[serde(
            rename = "restrictedCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restricted_categories: ::std::option::Option<Vec<i32>>,
        #[doc = "Detected sensitive categories, if any. Each category is represented by an integer as defined in  ad-sensitive-categories.txt. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "sensitiveCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sensitive_categories: ::std::option::Option<Vec<i32>>,
        #[doc = "The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction). Read-only. This field should not be set in requests. See the examples in the Creatives guide for more details."]
        #[serde(
            rename = "servingRestrictions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serving_restrictions:
            ::std::option::Option<Vec<crate::schemas::CreativeServingRestrictionsItems>>,
        #[doc = "List of vendor types for the ads that may be shown from this snippet. Each vendor type is represented by an integer as defined in vendors.txt."]
        #[serde(
            rename = "vendorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vendor_type: ::std::option::Option<Vec<i32>>,
        #[doc = "The version for this creative. Read-only. This field should not be set in requests."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
        #[doc = "The URL to fetch a video ad. If set, HTMLSnippet, videoVastXML, and nativeAd should not be set. Note, this is different from resource.native_ad.video_url above."]
        #[serde(
            rename = "videoURL",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_url: ::std::option::Option<String>,
        #[doc = "The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard. If set, HTMLSnippet, videoURL, and nativeAd and should not be set."]
        #[serde(
            rename = "videoVastXML",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_vast_xml: ::std::option::Option<String>,
        #[doc = "Ad width."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
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
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The detected ad technology provider IDs for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider. If this creative contains provider IDs that are outside of those listed in the `BidRequest.adslot.consented_providers_settings.consented_providers` field on the  Authorized Buyers Real-Time Bidding protocol or the `BidRequest.user.ext.consented_providers_settings.consented_providers` field on the OpenRTB protocol, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the \"Third-party Ad Technology Vendors\" section of Authorized Buyers Program Guidelines."]
        #[serde(
            rename = "detectedProviderIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detected_provider_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "Whether the creative contains an unidentified ad technology provider. If true, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the \"Third-party Ad Technology Vendors\" section of Authorized Buyers Program Guidelines."]
        #[serde(
            rename = "hasUnidentifiedProvider",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_unidentified_provider: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CreativeAdTechnologyProviders {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeAdTechnologyProviders {
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
    pub struct CreativeCorrectionsItems {
        #[doc = "All known serving contexts containing serving status information."]
        #[serde(
            rename = "contexts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contexts:
            ::std::option::Option<Vec<crate::schemas::CreativeCorrectionsItemsContextsItems>>,
        #[doc = "Additional details about the correction."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<Vec<String>>,
        #[doc = "The type of correction that was applied to the creative."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreativeCorrectionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeCorrectionsItems {
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
    pub struct CreativeCorrectionsItemsContextsItems {
        #[doc = "Only set when contextType=AUCTION_TYPE. Represents the auction types this correction applies to."]
        #[serde(
            rename = "auctionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auction_type: ::std::option::Option<Vec<String>>,
        #[doc = "The type of context (e.g., location, platform, auction type, SSL-ness)."]
        #[serde(
            rename = "contextType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context_type: ::std::option::Option<String>,
        #[doc = "Only set when contextType=LOCATION. Represents the geo criterias this correction applies to."]
        #[serde(
            rename = "geoCriteriaId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo_criteria_id: ::std::option::Option<Vec<i32>>,
        #[doc = "Only set when contextType=PLATFORM. Represents the platforms this correction applies to."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for CreativeCorrectionsItemsContextsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeCorrectionsItemsContextsItems {
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
    pub struct CreativeFilteringReasons {
        #[doc = "The date in ISO 8601 format for the data. The data is collected from 00:00:00 to 23:59:59 in PST."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<String>,
        #[doc = "The filtering reasons."]
        #[serde(
            rename = "reasons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reasons:
            ::std::option::Option<Vec<crate::schemas::CreativeFilteringReasonsReasonsItems>>,
    }
    impl ::google_field_selector::FieldSelector for CreativeFilteringReasons {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeFilteringReasons {
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
    pub struct CreativeFilteringReasonsReasonsItems {
        #[doc = "The number of times the creative was filtered for the status. The count is aggregated across all publishers on the exchange."]
        #[serde(
            rename = "filteringCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub filtering_count: ::std::option::Option<i64>,
        #[doc = "The filtering status code as defined in  creative-status-codes.txt."]
        #[serde(
            rename = "filteringStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filtering_status: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for CreativeFilteringReasonsReasonsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeFilteringReasonsReasonsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreativeNativeAd {
        #[serde(
            rename = "advertiser",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub advertiser: ::std::option::Option<String>,
        #[doc = "The app icon, for app download ads."]
        #[serde(
            rename = "appIcon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_icon: ::std::option::Option<crate::schemas::CreativeNativeAdAppIcon>,
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
        pub image: ::std::option::Option<crate::schemas::CreativeNativeAdImage>,
        #[doc = "The URLs are called when the impression is rendered."]
        #[serde(
            rename = "impressionTrackingUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impression_tracking_url: ::std::option::Option<Vec<String>>,
        #[doc = "A smaller image, for the advertiser logo."]
        #[serde(
            rename = "logo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo: ::std::option::Option<crate::schemas::CreativeNativeAdLogo>,
        #[doc = "The price of the promoted app including the currency info."]
        #[serde(
            rename = "price",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price: ::std::option::Option<String>,
        #[doc = "The app rating in the app store. Must be in the range [0-5]."]
        #[serde(
            rename = "starRating",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub star_rating: ::std::option::Option<f64>,
        #[doc = "The URL of the XML VAST for a native ad. Note this is a separate field from resource.video_url."]
        #[serde(
            rename = "videoURL",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreativeNativeAd {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeNativeAd {
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
    pub struct CreativeNativeAdAppIcon {
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for CreativeNativeAdAppIcon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeNativeAdAppIcon {
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
    pub struct CreativeNativeAdImage {
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for CreativeNativeAdImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeNativeAdImage {
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
    pub struct CreativeNativeAdLogo {
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for CreativeNativeAdLogo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeNativeAdLogo {
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
    pub struct CreativeServingRestrictionsItems {
        #[doc = "All known contexts/restrictions."]
        #[serde(
            rename = "contexts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contexts: ::std::option::Option<
            Vec<crate::schemas::CreativeServingRestrictionsItemsContextsItems>,
        >,
        #[doc = "The reasons for disapproval within this restriction, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED or CONDITIONALLY_APPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue."]
        #[serde(
            rename = "disapprovalReasons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disapproval_reasons: ::std::option::Option<
            Vec<crate::schemas::CreativeServingRestrictionsItemsDisapprovalReasonsItems>,
        >,
        #[doc = "Why the creative is ineligible to serve in this context (e.g., it has been explicitly disapproved or is pending review)."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreativeServingRestrictionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeServingRestrictionsItems {
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
    pub struct CreativeServingRestrictionsItemsContextsItems {
        #[doc = "Only set when contextType=AUCTION_TYPE. Represents the auction types this restriction applies to."]
        #[serde(
            rename = "auctionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auction_type: ::std::option::Option<Vec<String>>,
        #[doc = "The type of context (e.g., location, platform, auction type, SSL-ness)."]
        #[serde(
            rename = "contextType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context_type: ::std::option::Option<String>,
        #[doc = "Only set when contextType=LOCATION. Represents the geo criterias this restriction applies to. Impressions are considered to match a context if either the user location or publisher location matches a given geoCriteriaId."]
        #[serde(
            rename = "geoCriteriaId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo_criteria_id: ::std::option::Option<Vec<i32>>,
        #[doc = "Only set when contextType=PLATFORM. Represents the platforms this restriction applies to."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for CreativeServingRestrictionsItemsContextsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeServingRestrictionsItemsContextsItems {
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
    pub struct CreativeServingRestrictionsItemsDisapprovalReasonsItems {
        #[doc = "Additional details about the reason for disapproval."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<Vec<String>>,
        #[doc = "The categorized reason for disapproval."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for CreativeServingRestrictionsItemsDisapprovalReasonsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for CreativeServingRestrictionsItemsDisapprovalReasonsItems
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
    pub struct CreativeDealIds {
        #[doc = "A list of external deal ids and ARC approval status."]
        #[serde(
            rename = "dealStatuses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deal_statuses:
            ::std::option::Option<Vec<crate::schemas::CreativeDealIdsDealStatusesItems>>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreativeDealIds {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeDealIds {
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
    pub struct CreativeDealIdsDealStatusesItems {
        #[doc = "ARC approval status."]
        #[serde(
            rename = "arcStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arc_status: ::std::option::Option<String>,
        #[doc = "External deal ID."]
        #[serde(
            rename = "dealId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub deal_id: ::std::option::Option<i64>,
        #[doc = "Publisher ID."]
        #[serde(
            rename = "webPropertyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_property_id: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for CreativeDealIdsDealStatusesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativeDealIdsDealStatusesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreativesList {
        #[doc = "A list of creatives."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Creative>>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Continuation token used to page through creatives. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreativesList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreativesList {
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
    pub struct DealServingMetadata {
        #[doc = "True if alcohol ads are allowed for this deal (read-only). This field is only populated when querying for finalized orders using the method GetFinalizedOrderDeals"]
        #[serde(
            rename = "alcoholAdsAllowed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alcohol_ads_allowed: ::std::option::Option<bool>,
        #[doc = "Tracks which parties (if any) have paused a deal. (readonly, except via PauseResumeOrderDeals action)"]
        #[serde(
            rename = "dealPauseStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deal_pause_status:
            ::std::option::Option<crate::schemas::DealServingMetadataDealPauseStatus>,
    }
    impl ::google_field_selector::FieldSelector for DealServingMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealServingMetadata {
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
    pub struct DealServingMetadataDealPauseStatus {
        #[serde(
            rename = "buyerPauseReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer_pause_reason: ::std::option::Option<String>,
        #[doc = "If the deal is paused, records which party paused the deal first."]
        #[serde(
            rename = "firstPausedBy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_paused_by: ::std::option::Option<String>,
        #[serde(
            rename = "hasBuyerPaused",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_buyer_paused: ::std::option::Option<bool>,
        #[serde(
            rename = "hasSellerPaused",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_seller_paused: ::std::option::Option<bool>,
        #[serde(
            rename = "sellerPauseReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller_pause_reason: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DealServingMetadataDealPauseStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealServingMetadataDealPauseStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DealTerms {
        #[doc = "Visibility of the URL in bid requests."]
        #[serde(
            rename = "brandingType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub branding_type: ::std::option::Option<String>,
        #[doc = "Indicates that this ExternalDealId exists under at least two different AdxInventoryDeals. Currently, the only case that the same ExternalDealId will exist is programmatic cross sell case."]
        #[serde(
            rename = "crossListedExternalDealIdType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cross_listed_external_deal_id_type: ::std::option::Option<String>,
        #[doc = "Description for the proposed terms of the deal."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Non-binding estimate of the estimated gross spend for this deal Can be set by buyer or seller."]
        #[serde(
            rename = "estimatedGrossSpend",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub estimated_gross_spend: ::std::option::Option<crate::schemas::Price>,
        #[doc = "Non-binding estimate of the impressions served per day Can be set by buyer or seller."]
        #[serde(
            rename = "estimatedImpressionsPerDay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub estimated_impressions_per_day: ::std::option::Option<i64>,
        #[doc = "The terms for guaranteed fixed price deals."]
        #[serde(
            rename = "guaranteedFixedPriceTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub guaranteed_fixed_price_terms:
            ::std::option::Option<crate::schemas::DealTermsGuaranteedFixedPriceTerms>,
        #[doc = "The terms for non-guaranteed auction deals."]
        #[serde(
            rename = "nonGuaranteedAuctionTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_guaranteed_auction_terms:
            ::std::option::Option<crate::schemas::DealTermsNonGuaranteedAuctionTerms>,
        #[doc = "The terms for non-guaranteed fixed price deals."]
        #[serde(
            rename = "nonGuaranteedFixedPriceTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_guaranteed_fixed_price_terms:
            ::std::option::Option<crate::schemas::DealTermsNonGuaranteedFixedPriceTerms>,
        #[doc = "The terms for rubicon non-guaranteed deals."]
        #[serde(
            rename = "rubiconNonGuaranteedTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rubicon_non_guaranteed_terms:
            ::std::option::Option<crate::schemas::DealTermsRubiconNonGuaranteedTerms>,
        #[doc = "For deals with Cost Per Day billing, defines the timezone used to mark the boundaries of a day (buyer-readonly)"]
        #[serde(
            rename = "sellerTimeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller_time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DealTerms {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealTerms {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DealTermsGuaranteedFixedPriceTerms {
        #[doc = "External billing info for this Deal. This field is relevant when external billing info such as price has a different currency code than DFP/AdX."]
        #[serde(
            rename = "billingInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billing_info:
            ::std::option::Option<crate::schemas::DealTermsGuaranteedFixedPriceTermsBillingInfo>,
        #[doc = "Fixed price for the specified buyer."]
        #[serde(
            rename = "fixedPrices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_prices: ::std::option::Option<Vec<crate::schemas::PricePerBuyer>>,
        #[doc = "Guaranteed impressions as a percentage. This is the percentage of guaranteed looks that the buyer is guaranteeing to buy."]
        #[serde(
            rename = "guaranteedImpressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub guaranteed_impressions: ::std::option::Option<i64>,
        #[doc = "Count of guaranteed looks. Required for deal, optional for product. For CPD deals, buyer changes to guaranteed_looks will be ignored."]
        #[serde(
            rename = "guaranteedLooks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub guaranteed_looks: ::std::option::Option<i64>,
        #[doc = "Count of minimum daily looks for a CPD deal. For CPD deals, buyer should negotiate on this field instead of guaranteed_looks."]
        #[serde(
            rename = "minimumDailyLooks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub minimum_daily_looks: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for DealTermsGuaranteedFixedPriceTerms {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealTermsGuaranteedFixedPriceTerms {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DealTermsGuaranteedFixedPriceTermsBillingInfo {
        #[doc = "The timestamp (in ms since epoch) when the original reservation price for the deal was first converted to DFP currency. This is used to convert the contracted price into buyer's currency without discrepancy."]
        #[serde(
            rename = "currencyConversionTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub currency_conversion_time_ms: ::std::option::Option<i64>,
        #[doc = "The DFP line item id associated with this deal. For features like CPD, buyers can retrieve the DFP line item for billing reconciliation."]
        #[serde(
            rename = "dfpLineItemId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub dfp_line_item_id: ::std::option::Option<i64>,
        #[doc = "The original contracted quantity (# impressions) for this deal. To ensure delivery, sometimes the publisher will book the deal with a impression buffer, such that guaranteed_looks is greater than the contracted quantity. However clients are billed using the original contracted quantity."]
        #[serde(
            rename = "originalContractedQuantity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub original_contracted_quantity: ::std::option::Option<i64>,
        #[doc = "The original reservation price for the deal, if the currency code is different from the one used in negotiation."]
        #[serde(
            rename = "price",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price: ::std::option::Option<crate::schemas::Price>,
    }
    impl ::google_field_selector::FieldSelector for DealTermsGuaranteedFixedPriceTermsBillingInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealTermsGuaranteedFixedPriceTermsBillingInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DealTermsNonGuaranteedAuctionTerms {
        #[doc = "True if open auction buyers are allowed to compete with invited buyers in this private auction (buyer-readonly)."]
        #[serde(
            rename = "autoOptimizePrivateAuction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_optimize_private_auction: ::std::option::Option<bool>,
        #[doc = "Reserve price for the specified buyer."]
        #[serde(
            rename = "reservePricePerBuyers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reserve_price_per_buyers: ::std::option::Option<Vec<crate::schemas::PricePerBuyer>>,
    }
    impl ::google_field_selector::FieldSelector for DealTermsNonGuaranteedAuctionTerms {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealTermsNonGuaranteedAuctionTerms {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DealTermsNonGuaranteedFixedPriceTerms {
        #[doc = "Fixed price for the specified buyer."]
        #[serde(
            rename = "fixedPrices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_prices: ::std::option::Option<Vec<crate::schemas::PricePerBuyer>>,
    }
    impl ::google_field_selector::FieldSelector for DealTermsNonGuaranteedFixedPriceTerms {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealTermsNonGuaranteedFixedPriceTerms {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DealTermsRubiconNonGuaranteedTerms {
        #[doc = "Optional price for Rubicon priority access in the auction."]
        #[serde(
            rename = "priorityPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub priority_price: ::std::option::Option<crate::schemas::Price>,
        #[doc = "Optional price for Rubicon standard access in the auction."]
        #[serde(
            rename = "standardPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub standard_price: ::std::option::Option<crate::schemas::Price>,
    }
    impl ::google_field_selector::FieldSelector for DealTermsRubiconNonGuaranteedTerms {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DealTermsRubiconNonGuaranteedTerms {
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
    pub struct DeleteOrderDealsRequest {
        #[doc = "List of deals to delete for a given proposal"]
        #[serde(
            rename = "dealIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deal_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The last known proposal revision number."]
        #[serde(
            rename = "proposalRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: ::std::option::Option<i64>,
        #[doc = "Indicates an optional action to take on the proposal"]
        #[serde(
            rename = "updateAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_action: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteOrderDealsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteOrderDealsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DeleteOrderDealsResponse {
        #[doc = "List of deals deleted (in the same proposal as passed in the request)"]
        #[serde(
            rename = "deals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals: ::std::option::Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The updated revision number for the proposal."]
        #[serde(
            rename = "proposalRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for DeleteOrderDealsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteOrderDealsResponse {
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
    pub struct DeliveryControl {
        #[serde(
            rename = "creativeBlockingLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_blocking_level: ::std::option::Option<String>,
        #[serde(
            rename = "deliveryRateType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delivery_rate_type: ::std::option::Option<String>,
        #[serde(
            rename = "frequencyCaps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frequency_caps: ::std::option::Option<Vec<crate::schemas::DeliveryControlFrequencyCap>>,
    }
    impl ::google_field_selector::FieldSelector for DeliveryControl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeliveryControl {
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
    pub struct DeliveryControlFrequencyCap {
        #[serde(
            rename = "maxImpressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_impressions: ::std::option::Option<i32>,
        #[serde(
            rename = "numTimeUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_time_units: ::std::option::Option<i32>,
        #[serde(
            rename = "timeUnitType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_unit_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeliveryControlFrequencyCap {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeliveryControlFrequencyCap {
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
    pub struct Dimension {
        #[serde(
            rename = "dimensionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_type: ::std::option::Option<String>,
        #[serde(
            rename = "dimensionValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_values: ::std::option::Option<Vec<crate::schemas::DimensionDimensionValue>>,
    }
    impl ::google_field_selector::FieldSelector for Dimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dimension {
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
    pub struct DimensionDimensionValue {
        #[doc = "Id of the dimension."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<i32>,
        #[doc = "Name of the dimension mainly for debugging purposes, except for the case of CREATIVE_SIZE. For CREATIVE_SIZE, strings are used instead of ids."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Percent of total impressions for a dimension type. e.g. {dimension_type: 'GENDER', [{dimension_value: {id: 1, name: 'MALE', percentage: 60}}]} Gender MALE is 60% of all impressions which have gender."]
        #[serde(
            rename = "percentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub percentage: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for DimensionDimensionValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionDimensionValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EditAllOrderDealsRequest {
        #[doc = "List of deals to edit. Service may perform 3 different operations based on comparison of deals in this list vs deals already persisted in database: 1. Add new deal to proposal If a deal in this list does not exist in the proposal, the service will create a new deal and add it to the proposal. Validation will follow AddOrderDealsRequest. 2. Update existing deal in the proposal If a deal in this list already exist in the proposal, the service will update that existing deal to this new deal in the request. Validation will follow UpdateOrderDealsRequest. 3. Delete deals from the proposal (just need the id) If a existing deal in the proposal is not present in this list, the service will delete that deal from the proposal. Validation will follow DeleteOrderDealsRequest."]
        #[serde(
            rename = "deals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals: ::std::option::Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "If specified, also updates the proposal in the batch transaction. This is useful when the proposal and the deals need to be updated in one transaction."]
        #[serde(
            rename = "proposal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposal: ::std::option::Option<crate::schemas::Proposal>,
        #[doc = "The last known revision number for the proposal."]
        #[serde(
            rename = "proposalRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: ::std::option::Option<i64>,
        #[doc = "Indicates an optional action to take on the proposal"]
        #[serde(
            rename = "updateAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_action: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EditAllOrderDealsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EditAllOrderDealsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EditAllOrderDealsResponse {
        #[doc = "List of all deals in the proposal after edit."]
        #[serde(
            rename = "deals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals: ::std::option::Option<Vec<crate::schemas::MarketplaceDeal>>,
        #[doc = "The latest revision number after the update has been applied."]
        #[serde(
            rename = "orderRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub order_revision_number: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for EditAllOrderDealsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EditAllOrderDealsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GetOffersResponse {
        #[doc = "The returned list of products."]
        #[serde(
            rename = "products",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub products: ::std::option::Option<Vec<crate::schemas::Product>>,
    }
    impl ::google_field_selector::FieldSelector for GetOffersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetOffersResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GetOrderDealsResponse {
        #[doc = "List of deals for the proposal"]
        #[serde(
            rename = "deals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deals: ::std::option::Option<Vec<crate::schemas::MarketplaceDeal>>,
    }
    impl ::google_field_selector::FieldSelector for GetOrderDealsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetOrderDealsResponse {
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
    pub struct GetOrderNotesResponse {
        #[doc = "The list of matching notes. The notes for a proposal are ordered from oldest to newest. If the notes span multiple proposals, they will be grouped by proposal, with the notes for the most recently modified proposal appearing first."]
        #[serde(
            rename = "notes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes: ::std::option::Option<Vec<crate::schemas::MarketplaceNote>>,
    }
    impl ::google_field_selector::FieldSelector for GetOrderNotesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetOrderNotesResponse {
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
    pub struct GetOrdersResponse {
        #[doc = "The list of matching proposals."]
        #[serde(
            rename = "proposals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposals: ::std::option::Option<Vec<crate::schemas::Proposal>>,
    }
    impl ::google_field_selector::FieldSelector for GetOrdersResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetOrdersResponse {
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
    pub struct GetPublisherProfilesByAccountIdResponse {
        #[doc = "Profiles for the requested publisher"]
        #[serde(
            rename = "profiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profiles: ::std::option::Option<Vec<crate::schemas::PublisherProfileApiProto>>,
    }
    impl ::google_field_selector::FieldSelector for GetPublisherProfilesByAccountIdResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetPublisherProfilesByAccountIdResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MarketplaceDeal {
        #[doc = "Buyer private data (hidden from seller)."]
        #[serde(
            rename = "buyerPrivateData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer_private_data: ::std::option::Option<crate::schemas::PrivateData>,
        #[doc = "The time (ms since epoch) of the deal creation. (readonly)"]
        #[serde(
            rename = "creationTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time_ms: ::std::option::Option<i64>,
        #[doc = "Specifies the creative pre-approval policy (buyer-readonly)"]
        #[serde(
            rename = "creativePreApprovalPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_pre_approval_policy: ::std::option::Option<String>,
        #[doc = "Specifies whether the creative is safeFrame compatible (buyer-readonly)"]
        #[serde(
            rename = "creativeSafeFrameCompatibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_safe_frame_compatibility: ::std::option::Option<String>,
        #[doc = "A unique deal-id for the deal (readonly)."]
        #[serde(
            rename = "dealId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deal_id: ::std::option::Option<String>,
        #[doc = "Metadata about the serving status of this deal (readonly, writes via custom actions)"]
        #[serde(
            rename = "dealServingMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deal_serving_metadata: ::std::option::Option<crate::schemas::DealServingMetadata>,
        #[doc = "The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension."]
        #[serde(
            rename = "deliveryControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delivery_control: ::std::option::Option<crate::schemas::DeliveryControl>,
        #[doc = "The external deal id assigned to this deal once the deal is finalized. This is the deal-id that shows up in serving/reporting etc. (readonly)"]
        #[serde(
            rename = "externalDealId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_deal_id: ::std::option::Option<String>,
        #[doc = "Proposed flight end time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)"]
        #[serde(
            rename = "flightEndTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub flight_end_time_ms: ::std::option::Option<i64>,
        #[doc = "Proposed flight start time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)"]
        #[serde(
            rename = "flightStartTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub flight_start_time_ms: ::std::option::Option<i64>,
        #[doc = "Description for the deal terms. (buyer-readonly)"]
        #[serde(
            rename = "inventoryDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventory_description: ::std::option::Option<String>,
        #[doc = "Indicates whether the current deal is a RFP template. RFP template is created by buyer and not based on seller created products."]
        #[serde(
            rename = "isRfpTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_rfp_template: ::std::option::Option<bool>,
        #[doc = "True, if the buyside inventory setup is complete for this deal. (readonly, except via OrderSetupCompleted action)"]
        #[serde(
            rename = "isSetupComplete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_setup_complete: ::std::option::Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#marketplaceDeal\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The time (ms since epoch) when the deal was last updated. (readonly)"]
        #[serde(
            rename = "lastUpdateTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_update_time_ms: ::std::option::Option<i64>,
        #[doc = "The name of the deal. (updatable)"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The product-id from which this deal was created. (readonly, except on create)"]
        #[serde(
            rename = "productId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_id: ::std::option::Option<String>,
        #[doc = "The revision number of the product that the deal was created from (readonly, except on create)"]
        #[serde(
            rename = "productRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub product_revision_number: ::std::option::Option<i64>,
        #[doc = "Specifies the creative source for programmatic deals, PUBLISHER means creative is provided by seller and ADVERTISR means creative is provided by buyer. (buyer-readonly)"]
        #[serde(
            rename = "programmaticCreativeSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub programmatic_creative_source: ::std::option::Option<String>,
        #[serde(
            rename = "proposalId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposal_id: ::std::option::Option<String>,
        #[doc = "Optional Seller contact information for the deal (buyer-readonly)"]
        #[serde(
            rename = "sellerContacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller_contacts: ::std::option::Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "The shared targeting visible to buyers and sellers. Each shared targeting entity is AND'd together. (updatable)"]
        #[serde(
            rename = "sharedTargetings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shared_targetings: ::std::option::Option<Vec<crate::schemas::SharedTargeting>>,
        #[doc = "The syndication product associated with the deal. (readonly, except on create)"]
        #[serde(
            rename = "syndicationProduct",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syndication_product: ::std::option::Option<String>,
        #[doc = "The negotiable terms of the deal. (updatable)"]
        #[serde(
            rename = "terms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub terms: ::std::option::Option<crate::schemas::DealTerms>,
        #[serde(
            rename = "webPropertyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_property_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MarketplaceDeal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MarketplaceDeal {
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
    pub struct MarketplaceDealParty {
        #[doc = "The buyer/seller associated with the deal. One of buyer/seller is specified for a deal-party."]
        #[serde(
            rename = "buyer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer: ::std::option::Option<crate::schemas::Buyer>,
        #[doc = "The buyer/seller associated with the deal. One of buyer/seller is specified for a deal party."]
        #[serde(
            rename = "seller",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller: ::std::option::Option<crate::schemas::Seller>,
    }
    impl ::google_field_selector::FieldSelector for MarketplaceDealParty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MarketplaceDealParty {
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
    pub struct MarketplaceLabel {
        #[doc = "The accountId of the party that created the label."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_id: ::std::option::Option<String>,
        #[doc = "The creation time (in ms since epoch) for the label."]
        #[serde(
            rename = "createTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub create_time_ms: ::std::option::Option<i64>,
        #[doc = "Information about the party that created the label."]
        #[serde(
            rename = "deprecatedMarketplaceDealParty",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deprecated_marketplace_deal_party:
            ::std::option::Option<crate::schemas::MarketplaceDealParty>,
        #[doc = "The label to use."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MarketplaceLabel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MarketplaceLabel {
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
    pub struct MarketplaceNote {
        #[doc = "The role of the person (buyer/seller) creating the note. (readonly)"]
        #[serde(
            rename = "creatorRole",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator_role: ::std::option::Option<String>,
        #[doc = "Notes can optionally be associated with a deal. (readonly, except on create)"]
        #[serde(
            rename = "dealId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deal_id: ::std::option::Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#marketplaceNote\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The actual note to attach. (readonly, except on create)"]
        #[serde(
            rename = "note",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub note: ::std::option::Option<String>,
        #[doc = "The unique id for the note. (readonly)"]
        #[serde(
            rename = "noteId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub note_id: ::std::option::Option<String>,
        #[doc = "The proposalId that a note is attached to. (readonly)"]
        #[serde(
            rename = "proposalId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposal_id: ::std::option::Option<String>,
        #[doc = "If the note is associated with a proposal revision number, then store that here. (readonly, except on create)"]
        #[serde(
            rename = "proposalRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: ::std::option::Option<i64>,
        #[doc = "The timestamp (ms since epoch) that this note was created. (readonly)"]
        #[serde(
            rename = "timestampMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub timestamp_ms: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for MarketplaceNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MarketplaceNote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct PerformanceReport {
        #[doc = "The number of bid responses with an ad."]
        #[serde(
            rename = "bidRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bid_rate: ::std::option::Option<f64>,
        #[doc = "The number of bid requests sent to your bidder."]
        #[serde(
            rename = "bidRequestRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bid_request_rate: ::std::option::Option<f64>,
        #[doc = "Rate of various prefiltering statuses per match. Please refer to the callout-status-codes.txt file for different statuses."]
        #[serde(
            rename = "calloutStatusRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub callout_status_rate: ::std::option::Option<Vec<::serde_json::Value>>,
        #[doc = "Average QPS for cookie matcher operations."]
        #[serde(
            rename = "cookieMatcherStatusRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cookie_matcher_status_rate: ::std::option::Option<Vec<::serde_json::Value>>,
        #[doc = "Rate of ads with a given status. Please refer to the creative-status-codes.txt file for different statuses."]
        #[serde(
            rename = "creativeStatusRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_status_rate: ::std::option::Option<Vec<::serde_json::Value>>,
        #[doc = "The number of bid responses that were filtered due to a policy violation or other errors."]
        #[serde(
            rename = "filteredBidRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filtered_bid_rate: ::std::option::Option<f64>,
        #[doc = "Average QPS for hosted match operations."]
        #[serde(
            rename = "hostedMatchStatusRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hosted_match_status_rate: ::std::option::Option<Vec<::serde_json::Value>>,
        #[doc = "The number of potential queries based on your pretargeting settings."]
        #[serde(
            rename = "inventoryMatchRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventory_match_rate: ::std::option::Option<f64>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The 50th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
        #[serde(
            rename = "latency50thPercentile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latency_5_0th_percentile: ::std::option::Option<f64>,
        #[doc = "The 85th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
        #[serde(
            rename = "latency85thPercentile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latency_8_5th_percentile: ::std::option::Option<f64>,
        #[doc = "The 95th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
        #[serde(
            rename = "latency95thPercentile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latency_9_5th_percentile: ::std::option::Option<f64>,
        #[doc = "Rate of various quota account statuses per quota check."]
        #[serde(
            rename = "noQuotaInRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub no_quota_in_region: ::std::option::Option<f64>,
        #[doc = "Rate of various quota account statuses per quota check."]
        #[serde(
            rename = "outOfQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub out_of_quota: ::std::option::Option<f64>,
        #[doc = "Average QPS for pixel match requests from clients."]
        #[serde(
            rename = "pixelMatchRequests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pixel_match_requests: ::std::option::Option<f64>,
        #[doc = "Average QPS for pixel match responses from clients."]
        #[serde(
            rename = "pixelMatchResponses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pixel_match_responses: ::std::option::Option<f64>,
        #[doc = "The configured quota limits for this account."]
        #[serde(
            rename = "quotaConfiguredLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota_configured_limit: ::std::option::Option<f64>,
        #[doc = "The throttled quota limits for this account."]
        #[serde(
            rename = "quotaThrottledLimit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota_throttled_limit: ::std::option::Option<f64>,
        #[doc = "The trading location of this data."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "The number of properly formed bid responses received by our servers within the deadline."]
        #[serde(
            rename = "successfulRequestRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub successful_request_rate: ::std::option::Option<f64>,
        #[doc = "The unix timestamp of the starting time of this performance data."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub timestamp: ::std::option::Option<i64>,
        #[doc = "The number of bid responses that were unsuccessful due to timeouts, incorrect formatting, etc."]
        #[serde(
            rename = "unsuccessfulRequestRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unsuccessful_request_rate: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for PerformanceReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerformanceReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct PerformanceReportList {
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "A list of performance reports relevant for the account."]
        #[serde(
            rename = "performanceReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub performance_report: ::std::option::Option<Vec<crate::schemas::PerformanceReport>>,
    }
    impl ::google_field_selector::FieldSelector for PerformanceReportList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PerformanceReportList {
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
        #[doc = "The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically."]
        #[serde(
            rename = "billingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub billing_id: ::std::option::Option<i64>,
        #[doc = "The config id; generated automatically. Leave this field blank for insert requests."]
        #[serde(
            rename = "configId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub config_id: ::std::option::Option<i64>,
        #[doc = "The name of the config. Must be unique. Required for all requests."]
        #[serde(
            rename = "configName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_name: ::std::option::Option<String>,
        #[doc = "List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO."]
        #[serde(
            rename = "creativeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_type: ::std::option::Option<Vec<String>>,
        #[doc = "Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigDimensionsItems>>,
        #[doc = "Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section."]
        #[serde(
            rename = "excludedContentLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_content_labels: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these geo criteria ids will not match."]
        #[serde(
            rename = "excludedGeoCriteriaIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_geo_criteria_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these placements will not match."]
        #[serde(
            rename = "excludedPlacements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_placements:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigExcludedPlacementsItems>>,
        #[doc = "Requests containing any of these users list ids will not match."]
        #[serde(
            rename = "excludedUserLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_user_lists: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section."]
        #[serde(
            rename = "excludedVerticals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_verticals: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these geo criteria ids will match."]
        #[serde(
            rename = "geoCriteriaIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo_criteria_ids: ::std::option::Option<Vec<i64>>,
        #[doc = "Whether this config is active. Required for all requests."]
        #[serde(
            rename = "isActive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_active: ::std::option::Option<bool>,
        #[doc = "The kind of the resource, i.e. \"adexchangebuyer#pretargetingConfig\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Request containing any of these language codes will match."]
        #[serde(
            rename = "languages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub languages: ::std::option::Option<Vec<String>>,
        #[doc = "The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed)."]
        #[serde(
            rename = "maximumQps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_qps: ::std::option::Option<i64>,
        #[doc = "Requests where the predicted viewability is below the specified decile will not match. E.g. if the buyer sets this value to 5, requests from slots where the predicted viewability is below 50% will not match. If the predicted viewability is unknown this field will be ignored."]
        #[serde(
            rename = "minimumViewabilityDecile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimum_viewability_decile: ::std::option::Option<i32>,
        #[doc = "Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section."]
        #[serde(
            rename = "mobileCarriers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_carriers: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section."]
        #[serde(
            rename = "mobileDevices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_devices: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section."]
        #[serde(
            rename = "mobileOperatingSystemVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_operating_system_versions: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these placements will match."]
        #[serde(
            rename = "placements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub placements:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigPlacementsItems>>,
        #[doc = "Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET."]
        #[serde(
            rename = "platforms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platforms: ::std::option::Option<Vec<String>>,
        #[doc = "Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section."]
        #[serde(
            rename = "supportedCreativeAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supported_creative_attributes: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing the specified type of user data will match. Possible values are HOSTED_MATCH_DATA, which means the request is cookie-targetable and has a match in the buyer's hosted match table, and COOKIE_OR_IDFA, which means the request has either a targetable cookie or an iOS IDFA."]
        #[serde(
            rename = "userIdentifierDataRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_identifier_data_required: ::std::option::Option<Vec<String>>,
        #[doc = "Requests containing any of these user list ids will match."]
        #[serde(
            rename = "userLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_lists: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section."]
        #[serde(
            rename = "vendorTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vendor_types: ::std::option::Option<Vec<i64>>,
        #[doc = "Requests containing any of these vertical ids will match."]
        #[serde(
            rename = "verticals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verticals: ::std::option::Option<Vec<i64>>,
        #[doc = "Video requests satisfying any of these player size constraints will match."]
        #[serde(
            rename = "videoPlayerSizes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_player_sizes:
            ::std::option::Option<Vec<crate::schemas::PretargetingConfigVideoPlayerSizesItems>>,
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
    #[derive(
        Debug,
        Clone,
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
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub height: ::std::option::Option<i64>,
        #[doc = "Width in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub width: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigDimensionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigDimensionsItems {
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
    pub struct PretargetingConfigExcludedPlacementsItems {
        #[doc = "The type of the placement."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigExcludedPlacementsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigExcludedPlacementsItems {
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
    pub struct PretargetingConfigPlacementsItems {
        #[doc = "The type of the placement."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigPlacementsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigPlacementsItems {
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
    pub struct PretargetingConfigVideoPlayerSizesItems {
        #[doc = "The type of aspect ratio. Leave this field blank to match all aspect ratios."]
        #[serde(
            rename = "aspectRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aspect_ratio: ::std::option::Option<String>,
        #[doc = "The minimum player height in pixels. Leave this field blank to match any player height."]
        #[serde(
            rename = "minHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_height: ::std::option::Option<i64>,
        #[doc = "The minimum player width in pixels. Leave this field blank to match any player width."]
        #[serde(
            rename = "minWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_width: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigVideoPlayerSizesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigVideoPlayerSizesItems {
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
    pub struct PretargetingConfigList {
        #[doc = "A list of pretargeting configs"]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::PretargetingConfig>>,
        #[doc = "Resource type."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PretargetingConfigList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PretargetingConfigList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Price {
        #[doc = "The price value in micros."]
        #[serde(
            rename = "amountMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount_micros: ::std::option::Option<f64>,
        #[doc = "The currency code for the price."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "In case of CPD deals, the expected CPM in micros."]
        #[serde(
            rename = "expectedCpmMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expected_cpm_micros: ::std::option::Option<f64>,
        #[doc = "The pricing type for the deal/product."]
        #[serde(
            rename = "pricingType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pricing_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Price {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Price {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PricePerBuyer {
        #[doc = "Optional access type for this buyer."]
        #[serde(
            rename = "auctionTier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auction_tier: ::std::option::Option<String>,
        #[doc = "Reference to the buyer that will get billed."]
        #[serde(
            rename = "billedBuyer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billed_buyer: ::std::option::Option<crate::schemas::Buyer>,
        #[doc = "The buyer who will pay this price. If unset, all buyers can pay this price (if the advertisers match, and there's no more specific rule matching the buyer)."]
        #[serde(
            rename = "buyer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer: ::std::option::Option<crate::schemas::Buyer>,
        #[doc = "The specified price"]
        #[serde(
            rename = "price",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price: ::std::option::Option<crate::schemas::Price>,
    }
    impl ::google_field_selector::FieldSelector for PricePerBuyer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PricePerBuyer {
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
    pub struct PrivateData {
        #[serde(
            rename = "referenceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reference_id: ::std::option::Option<String>,
        #[serde(
            rename = "referencePayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reference_payload: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for PrivateData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PrivateData {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Product {
        #[doc = "The billed buyer corresponding to the buyer that created the offer. (readonly, except on create)"]
        #[serde(
            rename = "billedBuyer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billed_buyer: ::std::option::Option<crate::schemas::Buyer>,
        #[doc = "The buyer that created the offer if this is a buyer initiated offer (readonly, except on create)"]
        #[serde(
            rename = "buyer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer: ::std::option::Option<crate::schemas::Buyer>,
        #[doc = "Creation time in ms. since epoch (readonly)"]
        #[serde(
            rename = "creationTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creation_time_ms: ::std::option::Option<i64>,
        #[doc = "Optional contact information for the creator of this product. (buyer-readonly)"]
        #[serde(
            rename = "creatorContacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator_contacts: ::std::option::Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "The role that created the offer. Set to BUYER for buyer initiated offers."]
        #[serde(
            rename = "creatorRole",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator_role: ::std::option::Option<String>,
        #[doc = "The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension."]
        #[serde(
            rename = "deliveryControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delivery_control: ::std::option::Option<crate::schemas::DeliveryControl>,
        #[doc = "The proposed end time for the deal (ms since epoch) (buyer-readonly)"]
        #[serde(
            rename = "flightEndTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub flight_end_time_ms: ::std::option::Option<i64>,
        #[doc = "Inventory availability dates. (times are in ms since epoch) The granularity is generally in the order of seconds. (buyer-readonly)"]
        #[serde(
            rename = "flightStartTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub flight_start_time_ms: ::std::option::Option<i64>,
        #[doc = "If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false."]
        #[serde(
            rename = "hasCreatorSignedOff",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_creator_signed_off: ::std::option::Option<bool>,
        #[doc = "What exchange will provide this inventory (readonly, except on create)."]
        #[serde(
            rename = "inventorySource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventory_source: ::std::option::Option<String>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#product\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Optional List of labels for the product (optional, buyer-readonly)."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::MarketplaceLabel>>,
        #[doc = "Time of last update in ms. since epoch (readonly)"]
        #[serde(
            rename = "lastUpdateTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_update_time_ms: ::std::option::Option<i64>,
        #[doc = "Optional legacy offer id if this offer is a preferred deal offer."]
        #[serde(
            rename = "legacyOfferId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy_offer_id: ::std::option::Option<String>,
        #[doc = "Marketplace publisher profile Id. This Id differs from the regular publisher_profile_id in that 1. This is a new id, the old Id will be deprecated in 2017. 2. This id uniquely identifies a publisher profile by itself."]
        #[serde(
            rename = "marketplacePublisherProfileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub marketplace_publisher_profile_id: ::std::option::Option<String>,
        #[doc = "The name for this product as set by the seller. (buyer-readonly)"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional private auction id if this offer is a private auction offer."]
        #[serde(
            rename = "privateAuctionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_auction_id: ::std::option::Option<String>,
        #[doc = "The unique id for the product (readonly)"]
        #[serde(
            rename = "productId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_id: ::std::option::Option<String>,
        #[doc = "Id of the publisher profile for a given seller. A (seller.account_id, publisher_profile_id) pair uniquely identifies a publisher profile. Buyers can call the PublisherProfiles::List endpoint to get a list of publisher profiles for a given seller."]
        #[serde(
            rename = "publisherProfileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_profile_id: ::std::option::Option<String>,
        #[doc = "Publisher self-provided forecast information."]
        #[serde(
            rename = "publisherProvidedForecast",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_provided_forecast:
            ::std::option::Option<crate::schemas::PublisherProvidedForecast>,
        #[doc = "The revision number of the product. (readonly)"]
        #[serde(
            rename = "revisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub revision_number: ::std::option::Option<i64>,
        #[doc = "Information about the seller that created this product (readonly, except on create)"]
        #[serde(
            rename = "seller",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller: ::std::option::Option<crate::schemas::Seller>,
        #[doc = "Targeting that is shared between the buyer and the seller. Each targeting criteria has a specified key and for each key there is a list of inclusion value or exclusion values. (buyer-readonly)"]
        #[serde(
            rename = "sharedTargetings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shared_targetings: ::std::option::Option<Vec<crate::schemas::SharedTargeting>>,
        #[doc = "The state of the product. (buyer-readonly)"]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "The syndication product associated with the deal. (readonly, except on create)"]
        #[serde(
            rename = "syndicationProduct",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub syndication_product: ::std::option::Option<String>,
        #[doc = "The negotiable terms of the deal (buyer-readonly)"]
        #[serde(
            rename = "terms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub terms: ::std::option::Option<crate::schemas::DealTerms>,
        #[doc = "The web property code for the seller. This field is meant to be copied over as is when creating deals."]
        #[serde(
            rename = "webPropertyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_property_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Product {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Product {
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
    pub struct Proposal {
        #[doc = "Reference to the buyer that will get billed for this proposal. (readonly)"]
        #[serde(
            rename = "billedBuyer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub billed_buyer: ::std::option::Option<crate::schemas::Buyer>,
        #[doc = "Reference to the buyer on the proposal. (readonly, except on create)"]
        #[serde(
            rename = "buyer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer: ::std::option::Option<crate::schemas::Buyer>,
        #[doc = "Optional contact information of the buyer. (seller-readonly)"]
        #[serde(
            rename = "buyerContacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer_contacts: ::std::option::Option<Vec<crate::schemas::ContactInformation>>,
        #[doc = "Private data for buyer. (hidden from seller)."]
        #[serde(
            rename = "buyerPrivateData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer_private_data: ::std::option::Option<crate::schemas::PrivateData>,
        #[doc = "IDs of DBM advertisers permission to this proposal."]
        #[serde(
            rename = "dbmAdvertiserIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dbm_advertiser_ids: ::std::option::Option<Vec<String>>,
        #[doc = "When an proposal is in an accepted state, indicates whether the buyer has signed off. Once both sides have signed off on a deal, the proposal can be finalized by the seller. (seller-readonly)"]
        #[serde(
            rename = "hasBuyerSignedOff",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_buyer_signed_off: ::std::option::Option<bool>,
        #[doc = "When an proposal is in an accepted state, indicates whether the buyer has signed off Once both sides have signed off on a deal, the proposal can be finalized by the seller. (buyer-readonly)"]
        #[serde(
            rename = "hasSellerSignedOff",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_seller_signed_off: ::std::option::Option<bool>,
        #[doc = "What exchange will provide this inventory (readonly, except on create)."]
        #[serde(
            rename = "inventorySource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inventory_source: ::std::option::Option<String>,
        #[doc = "True if the proposal is being renegotiated (readonly)."]
        #[serde(
            rename = "isRenegotiating",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_renegotiating: ::std::option::Option<bool>,
        #[doc = "True, if the buyside inventory setup is complete for this proposal. (readonly, except via OrderSetupCompleted action) Deprecated in favor of deal level setup complete flag."]
        #[serde(
            rename = "isSetupComplete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_setup_complete: ::std::option::Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#proposal\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "List of labels associated with the proposal. (readonly)"]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::MarketplaceLabel>>,
        #[doc = "The role of the last user that either updated the proposal or left a comment. (readonly)"]
        #[serde(
            rename = "lastUpdaterOrCommentorRole",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_updater_or_commentor_role: ::std::option::Option<String>,
        #[doc = "The name for the proposal (updatable)"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional negotiation id if this proposal is a preferred deal proposal."]
        #[serde(
            rename = "negotiationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negotiation_id: ::std::option::Option<String>,
        #[doc = "Indicates whether the buyer/seller created the proposal.(readonly)"]
        #[serde(
            rename = "originatorRole",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub originator_role: ::std::option::Option<String>,
        #[doc = "Optional private auction id if this proposal is a private auction proposal."]
        #[serde(
            rename = "privateAuctionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_auction_id: ::std::option::Option<String>,
        #[doc = "The unique id of the proposal. (readonly)."]
        #[serde(
            rename = "proposalId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposal_id: ::std::option::Option<String>,
        #[doc = "The current state of the proposal. (readonly)"]
        #[serde(
            rename = "proposalState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proposal_state: ::std::option::Option<String>,
        #[doc = "The revision number for the proposal (readonly)."]
        #[serde(
            rename = "revisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub revision_number: ::std::option::Option<i64>,
        #[doc = "The time (ms since epoch) when the proposal was last revised (readonly)."]
        #[serde(
            rename = "revisionTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub revision_time_ms: ::std::option::Option<i64>,
        #[doc = "Reference to the seller on the proposal. (readonly, except on create)"]
        #[serde(
            rename = "seller",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller: ::std::option::Option<crate::schemas::Seller>,
        #[doc = "Optional contact information of the seller (buyer-readonly)."]
        #[serde(
            rename = "sellerContacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller_contacts: ::std::option::Option<Vec<crate::schemas::ContactInformation>>,
    }
    impl ::google_field_selector::FieldSelector for Proposal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Proposal {
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
    pub struct PublisherProfileApiProto {
        #[doc = "Publisher provided info on its audience."]
        #[serde(
            rename = "audience",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audience: ::std::option::Option<String>,
        #[doc = "A pitch statement for the buyer"]
        #[serde(
            rename = "buyerPitchStatement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buyer_pitch_statement: ::std::option::Option<String>,
        #[doc = "Direct contact for the publisher profile."]
        #[serde(
            rename = "directContact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direct_contact: ::std::option::Option<String>,
        #[doc = "Exchange where this publisher profile is from. E.g. AdX, Rubicon etc..."]
        #[serde(
            rename = "exchange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exchange: ::std::option::Option<String>,
        #[doc = "Link to publisher's Google+ page."]
        #[serde(
            rename = "googlePlusLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_plus_link: ::std::option::Option<String>,
        #[doc = "True, if this is the parent profile, which represents all domains owned by the publisher."]
        #[serde(
            rename = "isParent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_parent: ::std::option::Option<bool>,
        #[doc = "True, if this profile is published. Deprecated for state."]
        #[serde(
            rename = "isPublished",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_published: ::std::option::Option<bool>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#publisherProfileApiProto\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The url to the logo for the publisher."]
        #[serde(
            rename = "logoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo_url: ::std::option::Option<String>,
        #[doc = "The url for additional marketing and sales materials."]
        #[serde(
            rename = "mediaKitLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub media_kit_link: ::std::option::Option<String>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Publisher provided overview."]
        #[serde(
            rename = "overview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overview: ::std::option::Option<String>,
        #[doc = "The pair of (seller.account_id, profile_id) uniquely identifies a publisher profile for a given publisher."]
        #[serde(
            rename = "profileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_id: ::std::option::Option<i32>,
        #[doc = "Programmatic contact for the publisher profile."]
        #[serde(
            rename = "programmaticContact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub programmatic_contact: ::std::option::Option<String>,
        #[doc = "The list of domains represented in this publisher profile. Empty if this is a parent profile."]
        #[serde(
            rename = "publisherDomains",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_domains: ::std::option::Option<Vec<String>>,
        #[doc = "Unique Id for publisher profile."]
        #[serde(
            rename = "publisherProfileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_profile_id: ::std::option::Option<String>,
        #[doc = "Publisher provided forecasting information."]
        #[serde(
            rename = "publisherProvidedForecast",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_provided_forecast:
            ::std::option::Option<crate::schemas::PublisherProvidedForecast>,
        #[doc = "Link to publisher rate card"]
        #[serde(
            rename = "rateCardInfoLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rate_card_info_link: ::std::option::Option<String>,
        #[doc = "Link for a sample content page."]
        #[serde(
            rename = "samplePageLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_page_link: ::std::option::Option<String>,
        #[doc = "Seller of the publisher profile."]
        #[serde(
            rename = "seller",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub seller: ::std::option::Option<crate::schemas::Seller>,
        #[doc = "State of the publisher profile."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "Publisher provided key metrics and rankings."]
        #[serde(
            rename = "topHeadlines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_headlines: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for PublisherProfileApiProto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PublisherProfileApiProto {
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
    pub struct PublisherProvidedForecast {
        #[doc = "Publisher provided dimensions. E.g. geo, sizes etc..."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<Vec<crate::schemas::Dimension>>,
        #[doc = "Publisher provided weekly impressions."]
        #[serde(
            rename = "weeklyImpressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub weekly_impressions: ::std::option::Option<i64>,
        #[doc = "Publisher provided weekly uniques."]
        #[serde(
            rename = "weeklyUniques",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub weekly_uniques: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for PublisherProvidedForecast {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PublisherProvidedForecast {
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
    pub struct Seller {
        #[doc = "The unique id for the seller. The seller fills in this field. The seller account id is then available to buyer in the product."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_id: ::std::option::Option<String>,
        #[doc = "Optional sub-account id for the seller."]
        #[serde(
            rename = "subAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sub_account_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Seller {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Seller {
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
    pub struct SharedTargeting {
        #[doc = "The list of values to exclude from targeting. Each value is AND'd together."]
        #[serde(
            rename = "exclusions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclusions: ::std::option::Option<Vec<crate::schemas::TargetingValue>>,
        #[doc = "The list of value to include as part of the targeting. Each value is OR'd together."]
        #[serde(
            rename = "inclusions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inclusions: ::std::option::Option<Vec<crate::schemas::TargetingValue>>,
        #[doc = "The key representing the shared targeting criterion."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SharedTargeting {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SharedTargeting {
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
    pub struct TargetingValue {
        #[doc = "The creative size value to exclude/include."]
        #[serde(
            rename = "creativeSizeValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_size_value: ::std::option::Option<crate::schemas::TargetingValueCreativeSize>,
        #[doc = "The daypart targeting to include / exclude. Filled in when the key is GOOG_DAYPART_TARGETING."]
        #[serde(
            rename = "dayPartTargetingValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_part_targeting_value:
            ::std::option::Option<crate::schemas::TargetingValueDayPartTargeting>,
        #[serde(
            rename = "demogAgeCriteriaValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub demog_age_criteria_value:
            ::std::option::Option<crate::schemas::TargetingValueDemogAgeCriteria>,
        #[serde(
            rename = "demogGenderCriteriaValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub demog_gender_criteria_value:
            ::std::option::Option<crate::schemas::TargetingValueDemogGenderCriteria>,
        #[doc = "The long value to exclude/include."]
        #[serde(
            rename = "longValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub long_value: ::std::option::Option<i64>,
        #[doc = "The string value to exclude/include."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TargetingValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetingValue {
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
    pub struct TargetingValueCreativeSize {
        #[doc = "The formats allowed by the publisher."]
        #[serde(
            rename = "allowedFormats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_formats: ::std::option::Option<Vec<String>>,
        #[doc = "For video size type, the list of companion sizes."]
        #[serde(
            rename = "companionSizes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub companion_sizes: ::std::option::Option<Vec<crate::schemas::TargetingValueSize>>,
        #[doc = "The Creative size type."]
        #[serde(
            rename = "creativeSizeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creative_size_type: ::std::option::Option<String>,
        #[doc = "The native template for native ad."]
        #[serde(
            rename = "nativeTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub native_template: ::std::option::Option<String>,
        #[doc = "For regular or video creative size type, specifies the size of the creative."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<crate::schemas::TargetingValueSize>,
        #[doc = "The skippable ad type for video size."]
        #[serde(
            rename = "skippableAdType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skippable_ad_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TargetingValueCreativeSize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetingValueCreativeSize {
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
    pub struct TargetingValueDayPartTargeting {
        #[serde(
            rename = "dayParts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_parts:
            ::std::option::Option<Vec<crate::schemas::TargetingValueDayPartTargetingDayPart>>,
        #[serde(
            rename = "timeZoneType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TargetingValueDayPartTargeting {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetingValueDayPartTargeting {
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
    pub struct TargetingValueDayPartTargetingDayPart {
        #[serde(
            rename = "dayOfWeek",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_of_week: ::std::option::Option<String>,
        #[serde(
            rename = "endHour",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_hour: ::std::option::Option<i32>,
        #[serde(
            rename = "endMinute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_minute: ::std::option::Option<i32>,
        #[serde(
            rename = "startHour",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_hour: ::std::option::Option<i32>,
        #[serde(
            rename = "startMinute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_minute: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TargetingValueDayPartTargetingDayPart {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetingValueDayPartTargetingDayPart {
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
    pub struct TargetingValueDemogAgeCriteria {
        #[serde(
            rename = "demogAgeCriteriaIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub demog_age_criteria_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TargetingValueDemogAgeCriteria {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetingValueDemogAgeCriteria {
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
    pub struct TargetingValueDemogGenderCriteria {
        #[serde(
            rename = "demogGenderCriteriaIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub demog_gender_criteria_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TargetingValueDemogGenderCriteria {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetingValueDemogGenderCriteria {
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
    pub struct TargetingValueSize {
        #[doc = "The height of the creative."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The width of the creative."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TargetingValueSize {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TargetingValueSize {
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
    pub struct UpdatePrivateAuctionProposalRequest {
        #[doc = "The externalDealId of the deal to be updated."]
        #[serde(
            rename = "externalDealId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_deal_id: ::std::option::Option<String>,
        #[doc = "Optional note to be added."]
        #[serde(
            rename = "note",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub note: ::std::option::Option<crate::schemas::MarketplaceNote>,
        #[doc = "The current revision number of the proposal to be updated."]
        #[serde(
            rename = "proposalRevisionNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub proposal_revision_number: ::std::option::Option<i64>,
        #[doc = "The proposed action on the private auction proposal."]
        #[serde(
            rename = "updateAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_action: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdatePrivateAuctionProposalRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdatePrivateAuctionProposalRequest {
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
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
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
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
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
        A: Into<Box<dyn ::google_api_auth::GetAccessToken>>,
    {
        Client {
            reqwest,
            auth: auth.into(),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the accounts resource"]
    pub fn accounts(&self) -> crate::resources::accounts::AccountsActions {
        crate::resources::accounts::AccountsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the billing_info resource"]
    pub fn billing_info(&self) -> crate::resources::billing_info::BillingInfoActions {
        crate::resources::billing_info::BillingInfoActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the budget resource"]
    pub fn budget(&self) -> crate::resources::budget::BudgetActions {
        crate::resources::budget::BudgetActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the creatives resource"]
    pub fn creatives(&self) -> crate::resources::creatives::CreativesActions {
        crate::resources::creatives::CreativesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the marketplacedeals resource"]
    pub fn marketplacedeals(&self) -> crate::resources::marketplacedeals::MarketplacedealsActions {
        crate::resources::marketplacedeals::MarketplacedealsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the marketplacenotes resource"]
    pub fn marketplacenotes(&self) -> crate::resources::marketplacenotes::MarketplacenotesActions {
        crate::resources::marketplacenotes::MarketplacenotesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the marketplaceprivateauction resource"]
    pub fn marketplaceprivateauction(
        &self,
    ) -> crate::resources::marketplaceprivateauction::MarketplaceprivateauctionActions {
        crate::resources::marketplaceprivateauction::MarketplaceprivateauctionActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the performance_report resource"]
    pub fn performance_report(
        &self,
    ) -> crate::resources::performance_report::PerformanceReportActions {
        crate::resources::performance_report::PerformanceReportActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the pretargeting_config resource"]
    pub fn pretargeting_config(
        &self,
    ) -> crate::resources::pretargeting_config::PretargetingConfigActions {
        crate::resources::pretargeting_config::PretargetingConfigActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the products resource"]
    pub fn products(&self) -> crate::resources::products::ProductsActions {
        crate::resources::products::ProductsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the proposals resource"]
    pub fn proposals(&self) -> crate::resources::proposals::ProposalsActions {
        crate::resources::proposals::ProposalsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the pubprofiles resource"]
    pub fn pubprofiles(&self) -> crate::resources::pubprofiles::PubprofilesActions {
        crate::resources::pubprofiles::PubprofilesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod accounts {
        pub mod params {}
        pub struct AccountsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AccountsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets one account by ID."]
            pub fn get(&self, id: i32) -> GetRequestBuilder {
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
                    id,
                }
            }
            #[doc = "Retrieves the authenticated user's list of accounts."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            pub fn patch(&self, request: crate::schemas::Account, id: i32) -> PatchRequestBuilder {
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
                    id,
                    confirm_unsafe_account_change: None,
                }
            }
            #[doc = "Updates an existing account."]
            pub fn update(
                &self,
                request: crate::schemas::Account,
                id: i32,
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
                    id,
                    confirm_unsafe_account_change: None,
                }
            }
        }
        #[doc = "Created via [AccountsActions::get()](struct.AccountsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            id: i32,
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
            ) -> Result<crate::schemas::Account, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Account, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("accounts/");
                {
                    let var_as_string = self.id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [AccountsActions::list()](struct.AccountsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::AccountsList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AccountsList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("accounts");
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
        #[doc = "Created via [AccountsActions::patch()](struct.AccountsActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> PatchRequestBuilder<'a> {
            #[doc = "Confirmation for erasing bidder and cookie matching urls."]
            pub fn confirm_unsafe_account_change(mut self, value: bool) -> Self {
                self.confirm_unsafe_account_change = Some(value);
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
            ) -> Result<crate::schemas::Account, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Account, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("accounts/");
                {
                    let var_as_string = self.id.to_string();
                    let var_as_str = &var_as_string;
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [AccountsActions::update()](struct.AccountsActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> UpdateRequestBuilder<'a> {
            #[doc = "Confirmation for erasing bidder and cookie matching urls."]
            pub fn confirm_unsafe_account_change(mut self, value: bool) -> Self {
                self.confirm_unsafe_account_change = Some(value);
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
            ) -> Result<crate::schemas::Account, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Account, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("accounts/");
                {
                    let var_as_string = self.id.to_string();
                    let var_as_str = &var_as_string;
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod billing_info {
        pub mod params {}
        pub struct BillingInfoActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> BillingInfoActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns the billing information for one account specified by account ID."]
            pub fn get(&self, account_id: i32) -> GetRequestBuilder {
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
                    account_id,
                }
            }
            #[doc = "Retrieves a list of billing information for all accounts of the authenticated user."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        #[doc = "Created via [BillingInfoActions::get()](struct.BillingInfoActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            account_id: i32,
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
            ) -> Result<crate::schemas::BillingInfo, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BillingInfo, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("billinginfo/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [BillingInfoActions::list()](struct.BillingInfoActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::BillingInfoList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BillingInfoList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("billinginfo");
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
    }
    pub mod budget {
        pub mod params {}
        pub struct BudgetActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> BudgetActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns the budget information for the adgroup specified by the accountId and billingId."]
            pub fn get(&self, account_id: i64, billing_id: i64) -> GetRequestBuilder {
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
                    account_id,
                    billing_id,
                }
            }
        }
        #[doc = "Created via [BudgetActions::get()](struct.BudgetActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            ) -> Result<crate::schemas::Budget, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Budget, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("billinginfo/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.billing_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [BudgetActions::patch()](struct.BudgetActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            ) -> Result<crate::schemas::Budget, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Budget, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("billinginfo/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.billing_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [BudgetActions::update()](struct.BudgetActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            ) -> Result<crate::schemas::Budget, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Budget, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("billinginfo/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.billing_id.to_string();
                    let var_as_str = &var_as_string;
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
            impl ::std::convert::AsRef<str> for ListDealsStatusFilter {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListDealsStatusFilter {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListDealsStatusFilter, ()> {
                    Ok(match s {
                        "approved" => ListDealsStatusFilter::Approved,
                        "conditionally_approved" => ListDealsStatusFilter::ConditionallyApproved,
                        "disapproved" => ListDealsStatusFilter::Disapproved,
                        "not_checked" => ListDealsStatusFilter::NotChecked,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListDealsStatusFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListDealsStatusFilter {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListDealsStatusFilter {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
            impl ::google_field_selector::FieldSelector for ListDealsStatusFilter {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListDealsStatusFilter {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
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
                        ListOpenAuctionStatusFilter::ConditionallyApproved => {
                            "conditionally_approved"
                        }
                        ListOpenAuctionStatusFilter::Disapproved => "disapproved",
                        ListOpenAuctionStatusFilter::NotChecked => "not_checked",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListOpenAuctionStatusFilter {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListOpenAuctionStatusFilter {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListOpenAuctionStatusFilter, ()> {
                    Ok(match s {
                        "approved" => ListOpenAuctionStatusFilter::Approved,
                        "conditionally_approved" => {
                            ListOpenAuctionStatusFilter::ConditionallyApproved
                        }
                        "disapproved" => ListOpenAuctionStatusFilter::Disapproved,
                        "not_checked" => ListOpenAuctionStatusFilter::NotChecked,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListOpenAuctionStatusFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListOpenAuctionStatusFilter {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListOpenAuctionStatusFilter {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "approved" => ListOpenAuctionStatusFilter::Approved,
                        "conditionally_approved" => {
                            ListOpenAuctionStatusFilter::ConditionallyApproved
                        }
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
            impl ::google_field_selector::FieldSelector for ListOpenAuctionStatusFilter {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListOpenAuctionStatusFilter {
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
            #[doc = "Add a deal id association for the creative."]
            pub fn add_deal(
                &self,
                account_id: i32,
                buyer_creative_id: impl Into<String>,
                deal_id: i64,
            ) -> AddDealRequestBuilder {
                AddDealRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            ) -> GetRequestBuilder {
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
                    account_id,
                    buyer_creative_id: buyer_creative_id.into(),
                }
            }
            #[doc = "Submit a new creative."]
            pub fn insert(&self, request: crate::schemas::Creative) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                }
            }
            #[doc = "Retrieves a list of the authenticated user's active creatives. A creative will be available 30-40 minutes after submission."]
            pub fn list(&self) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            ) -> ListDealsRequestBuilder {
                ListDealsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            ) -> RemoveDealRequestBuilder {
                RemoveDealRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        #[doc = "Created via [CreativesActions::add_deal()](struct.CreativesActions.html#method.add_deal)"]
        #[derive(Debug, Clone)]
        pub struct AddDealRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> AddDealRequestBuilder<'a> {
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("creatives/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.buyer_creative_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/addDeal/");
                {
                    let var_as_string = self.deal_id.to_string();
                    let var_as_str = &var_as_string;
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
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [CreativesActions::get()](struct.CreativesActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            ) -> Result<crate::schemas::Creative, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Creative, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("creatives/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.buyer_creative_id;
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
        #[doc = "Created via [CreativesActions::insert()](struct.CreativesActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Creative,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> InsertRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Creative, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Creative, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("creatives");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [CreativesActions::list()](struct.CreativesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            account_id: Option<Vec<i32>>,
            buyer_creative_id: Option<Vec<String>>,
            deals_status_filter: Option<crate::resources::creatives::params::ListDealsStatusFilter>,
            max_results: Option<u32>,
            open_auction_status_filter:
                Option<crate::resources::creatives::params::ListOpenAuctionStatusFilter>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "When specified, only creatives for the given account ids are returned."]
            pub fn account_id(mut self, value: impl Into<Vec<i32>>) -> Self {
                self.account_id = Some(value.into());
                self
            }
            #[doc = "When specified, only creatives for the given buyer creative ids are returned."]
            pub fn buyer_creative_id(mut self, value: impl Into<Vec<String>>) -> Self {
                self.buyer_creative_id = Some(value.into());
                self
            }
            #[doc = "When specified, only creatives having the given deals status are returned."]
            pub fn deals_status_filter(
                mut self,
                value: crate::resources::creatives::params::ListDealsStatusFilter,
            ) -> Self {
                self.deals_status_filter = Some(value);
                self
            }
            #[doc = "Maximum number of entries returned on one result page. If not set, the default is 100. Optional."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "When specified, only creatives having the given open auction status are returned."]
            pub fn open_auction_status_filter(
                mut self,
                value: crate::resources::creatives::params::ListOpenAuctionStatusFilter,
            ) -> Self {
                self.open_auction_status_filter = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through ad clients. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response. Optional."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_items<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_items_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Creative> {
                self.iter_items_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Creative> {
                self.iter_items_with_fields(Some("*"))
            }
            pub fn iter_items_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "items").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "items")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::CreativesList> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::CreativesList> {
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
            ) -> Result<crate::schemas::CreativesList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CreativesList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("creatives");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("accountId", &self.account_id)]);
                let req = req.query(&[("buyerCreativeId", &self.buyer_creative_id)]);
                let req = req.query(&[("dealsStatusFilter", &self.deals_status_filter)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req =
                    req.query(&[("openAuctionStatusFilter", &self.open_auction_status_filter)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
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
        #[doc = "Created via [CreativesActions::list_deals()](struct.CreativesActions.html#method.list_deals)"]
        #[derive(Debug, Clone)]
        pub struct ListDealsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ListDealsRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::CreativeDealIds, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CreativeDealIds, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("creatives/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.buyer_creative_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/listDeals");
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
        #[doc = "Created via [CreativesActions::remove_deal()](struct.CreativesActions.html#method.remove_deal)"]
        #[derive(Debug, Clone)]
        pub struct RemoveDealRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> RemoveDealRequestBuilder<'a> {
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("creatives/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.buyer_creative_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/removeDeal/");
                {
                    let var_as_string = self.deal_id.to_string();
                    let var_as_str = &var_as_string;
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
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
    pub mod marketplacedeals {
        pub mod params {}
        pub struct MarketplacedealsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> MarketplacedealsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Delete the specified deals from the proposal"]
            pub fn delete(
                &self,
                request: crate::schemas::DeleteOrderDealsRequest,
                proposal_id: impl Into<String>,
            ) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
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
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Add new deals for the specified proposal"]
            pub fn insert(
                &self,
                request: crate::schemas::AddOrderDealsRequest,
                proposal_id: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "List all the deals for a given proposal"]
            pub fn list(&self, proposal_id: impl Into<String>) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
                    proposal_id: proposal_id.into(),
                }
            }
        }
        #[doc = "Created via [MarketplacedealsActions::delete()](struct.MarketplacedealsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> DeleteRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::DeleteOrderDealsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DeleteOrderDealsResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/deals/delete");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [MarketplacedealsActions::insert()](struct.MarketplacedealsActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> InsertRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::AddOrderDealsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AddOrderDealsResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/deals/insert");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [MarketplacedealsActions::list()](struct.MarketplacedealsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Query string to retrieve specific deals."]
            pub fn pql_query(mut self, value: impl Into<String>) -> Self {
                self.pql_query = Some(value.into());
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
            ) -> Result<crate::schemas::GetOrderDealsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetOrderDealsResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/deals");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pqlQuery", &self.pql_query)]);
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
        #[doc = "Created via [MarketplacedealsActions::update()](struct.MarketplacedealsActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            ) -> Result<crate::schemas::EditAllOrderDealsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::EditAllOrderDealsResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/deals/update");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
    pub mod marketplacenotes {
        pub mod params {}
        pub struct MarketplacenotesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> MarketplacenotesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Add notes to the proposal"]
            pub fn insert(
                &self,
                request: crate::schemas::AddOrderNotesRequest,
                proposal_id: impl Into<String>,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Get all the notes associated with a proposal"]
            pub fn list(&self, proposal_id: impl Into<String>) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        #[doc = "Created via [MarketplacenotesActions::insert()](struct.MarketplacenotesActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> InsertRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::AddOrderNotesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AddOrderNotesResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/notes/insert");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [MarketplacenotesActions::list()](struct.MarketplacenotesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Query string to retrieve specific notes. To search the text contents of notes, please use syntax like \"WHERE note.note = \"foo\" or \"WHERE note.note LIKE \"%bar%\""]
            pub fn pql_query(mut self, value: impl Into<String>) -> Self {
                self.pql_query = Some(value.into());
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
            ) -> Result<crate::schemas::GetOrderNotesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetOrderNotesResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/notes");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pqlQuery", &self.pql_query)]);
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
    pub mod marketplaceprivateauction {
        pub mod params {}
        pub struct MarketplaceprivateauctionActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> MarketplaceprivateauctionActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Update a given private auction proposal"]
            pub fn updateproposal(
                &self,
                request: crate::schemas::UpdatePrivateAuctionProposalRequest,
                private_auction_id: impl Into<String>,
            ) -> UpdateproposalRequestBuilder {
                UpdateproposalRequestBuilder {
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
                    private_auction_id: private_auction_id.into(),
                }
            }
        }
        #[doc = "Created via [MarketplaceprivateauctionActions::updateproposal()](struct.MarketplaceprivateauctionActions.html#method.updateproposal)"]
        #[derive(Debug, Clone)]
        pub struct UpdateproposalRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> UpdateproposalRequestBuilder<'a> {
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                let req = req.json(&self.request);
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("privateauction/");
                {
                    let var_as_str = &self.private_auction_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/updateproposal");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
    pub mod performance_report {
        pub mod params {}
        pub struct PerformanceReportActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PerformanceReportActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves the authenticated user's list of performance metrics."]
            pub fn list(
                &self,
                account_id: i64,
                end_date_time: impl Into<String>,
                start_date_time: impl Into<String>,
            ) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        #[doc = "Created via [PerformanceReportActions::list()](struct.PerformanceReportActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ListRequestBuilder<'a> {
            #[doc = "Maximum number of entries returned on one result page. If not set, the default is 100. Optional."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "A continuation token, used to page through performance reports. To retrieve the next page, set this parameter to the value of \"nextPageToken\" from the previous response. Optional."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            ) -> Result<crate::schemas::PerformanceReportList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PerformanceReportList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("performancereport");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                let req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
    }
    pub mod pretargeting_config {
        pub mod params {}
        pub struct PretargetingConfigActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PretargetingConfigActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Deletes an existing pretargeting config."]
            pub fn delete(&self, account_id: i64, config_id: i64) -> DeleteRequestBuilder {
                DeleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            pub fn get(&self, account_id: i64, config_id: i64) -> GetRequestBuilder {
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
                    account_id,
                    config_id,
                }
            }
            #[doc = "Inserts a new pretargeting configuration."]
            pub fn insert(
                &self,
                request: crate::schemas::PretargetingConfig,
                account_id: i64,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                    account_id,
                }
            }
            #[doc = "Retrieves a list of the authenticated user's pretargeting configurations."]
            pub fn list(&self, account_id: i64) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
                    account_id,
                    config_id,
                }
            }
        }
        #[doc = "Created via [PretargetingConfigActions::delete()](struct.PretargetingConfigActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> DeleteRequestBuilder<'a> {
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("pretargetingconfigs/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.config_id.to_string();
                    let var_as_str = &var_as_string;
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
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
        #[doc = "Created via [PretargetingConfigActions::get()](struct.PretargetingConfigActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("pretargetingconfigs/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.config_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [PretargetingConfigActions::insert()](struct.PretargetingConfigActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> InsertRequestBuilder<'a> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("pretargetingconfigs/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
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
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [PretargetingConfigActions::list()](struct.PretargetingConfigActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            account_id: i64,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::PretargetingConfigList, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PretargetingConfigList, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("pretargetingconfigs/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [PretargetingConfigActions::patch()](struct.PretargetingConfigActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("pretargetingconfigs/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.config_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [PretargetingConfigActions::update()](struct.PretargetingConfigActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("pretargetingconfigs/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.config_id.to_string();
                    let var_as_str = &var_as_string;
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
    pub mod products {
        pub mod params {}
        pub struct ProductsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProductsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets the requested product by id."]
            pub fn get(&self, product_id: impl Into<String>) -> GetRequestBuilder {
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
                    product_id: product_id.into(),
                }
            }
            #[doc = "Gets the requested product."]
            pub fn search(&self) -> SearchRequestBuilder {
                SearchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        #[doc = "Created via [ProductsActions::get()](struct.ProductsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            product_id: String,
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
            ) -> Result<crate::schemas::Product, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Product, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("products/");
                {
                    let var_as_str = &self.product_id;
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
        #[doc = "Created via [ProductsActions::search()](struct.ProductsActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            pql_query: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> SearchRequestBuilder<'a> {
            #[doc = "The pql query used to query for products."]
            pub fn pql_query(mut self, value: impl Into<String>) -> Self {
                self.pql_query = Some(value.into());
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
            ) -> Result<crate::schemas::GetOffersResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetOffersResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("products/search");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pqlQuery", &self.pql_query)]);
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
            impl ::std::convert::AsRef<str> for PatchUpdateAction {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for PatchUpdateAction {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<PatchUpdateAction, ()> {
                    Ok(match s {
                        "accept" => PatchUpdateAction::Accept,
                        "cancel" => PatchUpdateAction::Cancel,
                        "propose" => PatchUpdateAction::Propose,
                        "proposeAndAccept" => PatchUpdateAction::ProposeAndAccept,
                        "unknownAction" => PatchUpdateAction::UnknownAction,
                        "updateNonTerms" => PatchUpdateAction::UpdateNonTerms,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for PatchUpdateAction {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for PatchUpdateAction {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for PatchUpdateAction {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
            impl ::google_field_selector::FieldSelector for PatchUpdateAction {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for PatchUpdateAction {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
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
            impl ::std::convert::AsRef<str> for UpdateUpdateAction {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for UpdateUpdateAction {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<UpdateUpdateAction, ()> {
                    Ok(match s {
                        "accept" => UpdateUpdateAction::Accept,
                        "cancel" => UpdateUpdateAction::Cancel,
                        "propose" => UpdateUpdateAction::Propose,
                        "proposeAndAccept" => UpdateUpdateAction::ProposeAndAccept,
                        "unknownAction" => UpdateUpdateAction::UnknownAction,
                        "updateNonTerms" => UpdateUpdateAction::UpdateNonTerms,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for UpdateUpdateAction {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for UpdateUpdateAction {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for UpdateUpdateAction {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
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
            impl ::google_field_selector::FieldSelector for UpdateUpdateAction {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for UpdateUpdateAction {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct ProposalsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProposalsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Get a proposal given its id"]
            pub fn get(&self, proposal_id: impl Into<String>) -> GetRequestBuilder {
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
                    proposal_id: proposal_id.into(),
                }
            }
            #[doc = "Create the given list of proposals"]
            pub fn insert(
                &self,
                request: crate::schemas::CreateOrdersRequest,
            ) -> InsertRequestBuilder {
                InsertRequestBuilder {
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
                }
            }
            #[doc = "Update the given proposal. This method supports patch semantics."]
            pub fn patch(
                &self,
                request: crate::schemas::Proposal,
                proposal_id: impl Into<String>,
                revision_number: i64,
                update_action: crate::resources::proposals::params::PatchUpdateAction,
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
                    proposal_id: proposal_id.into(),
                    revision_number,
                    update_action,
                }
            }
            #[doc = "Search for proposals using pql query"]
            pub fn search(&self) -> SearchRequestBuilder {
                SearchRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            ) -> SetupcompleteRequestBuilder {
                SetupcompleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
                update_action: crate::resources::proposals::params::UpdateUpdateAction,
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
                    proposal_id: proposal_id.into(),
                    revision_number,
                    update_action,
                }
            }
        }
        #[doc = "Created via [ProposalsActions::get()](struct.ProposalsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            proposal_id: String,
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
            ) -> Result<crate::schemas::Proposal, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Proposal, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
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
        #[doc = "Created via [ProposalsActions::insert()](struct.ProposalsActions.html#method.insert)"]
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CreateOrdersRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> InsertRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::CreateOrdersResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CreateOrdersResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/insert");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [ProposalsActions::patch()](struct.ProposalsActions.html#method.patch)"]
        #[derive(Debug, Clone)]
        pub struct PatchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Proposal,
            proposal_id: String,
            revision_number: i64,
            update_action: crate::resources::proposals::params::PatchUpdateAction,
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
            ) -> Result<crate::schemas::Proposal, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Proposal, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.revision_number.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.update_action.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [ProposalsActions::search()](struct.ProposalsActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            pql_query: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> SearchRequestBuilder<'a> {
            #[doc = "Query string to retrieve specific proposals."]
            pub fn pql_query(mut self, value: impl Into<String>) -> Self {
                self.pql_query = Some(value.into());
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
            ) -> Result<crate::schemas::GetOrdersResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetOrdersResponse, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/search");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("pqlQuery", &self.pql_query)]);
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
        #[doc = "Created via [ProposalsActions::setupcomplete()](struct.ProposalsActions.html#method.setupcomplete)"]
        #[derive(Debug, Clone)]
        pub struct SetupcompleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            proposal_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> SetupcompleteRequestBuilder<'a> {
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
            pub fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path())?;
                crate::error_from_response(req.send()?)?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/setupcomplete");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
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
        #[doc = "Created via [ProposalsActions::update()](struct.ProposalsActions.html#method.update)"]
        #[derive(Debug, Clone)]
        pub struct UpdateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Proposal,
            proposal_id: String,
            revision_number: i64,
            update_action: crate::resources::proposals::params::UpdateUpdateAction,
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
            ) -> Result<crate::schemas::Proposal, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Proposal, crate::Error> {
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
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("proposals/");
                {
                    let var_as_str = &self.proposal_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.revision_number.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.update_action.to_string();
                    let var_as_str = &var_as_string;
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
    pub mod pubprofiles {
        pub mod params {}
        pub struct PubprofilesActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PubprofilesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets the requested publisher profile(s) by publisher accountId."]
            pub fn list(&self, account_id: i32) -> ListRequestBuilder {
                ListRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
        #[doc = "Created via [PubprofilesActions::list()](struct.PubprofilesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            account_id: i32,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a> ListRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::GetPublisherProfilesByAccountIdResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetPublisherProfilesByAccountIdResponse, crate::Error>
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
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://www.googleapis.com/adexchangebuyer/v1.4/".to_owned();
                output.push_str("publisher/");
                {
                    let var_as_string = self.account_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/profiles");
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
